//! Proposal lifecycle management.

use serde::{Deserialize, Serialize};

use crate::voting::VotingRecord;
use crate::{GovernanceError, Result};

/// Parameters for creating a new proposal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalParams {
    pub title: String,
    pub description: String,
    /// Proposer's address (hex).
    pub proposer: String,
    /// Voting duration in seconds.
    pub voting_duration: u64,
    /// Minimum quorum (total votes required).
    pub quorum: u64,
}

/// State machine for proposal lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalState {
    /// Created, waiting for voting period to start.
    Pending,
    /// Voting is currently open.
    Active,
    /// Voting ended, quorum met, majority voted For.
    Succeeded,
    /// Voting ended, defeated (quorum not met or majority Against).
    Defeated,
    /// Succeeded and queued for timelock execution.
    Queued,
    /// Executed after timelock.
    Executed,
    /// Cancelled by proposer or guardian.
    Cancelled,
}

/// A governance proposal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub state: ProposalState,
    /// Unix timestamp when voting starts.
    pub start_time: u64,
    /// Unix timestamp when voting ends.
    pub end_time: u64,
    /// Minimum total votes for quorum.
    pub quorum: u64,
    /// Voting record.
    pub votes: VotingRecord,
    /// Timestamp when queued for execution (0 if not queued).
    pub queued_at: u64,
}

impl Proposal {
    /// Create a new proposal in Pending state.
    pub fn new(id: u64, params: ProposalParams, now: u64) -> Self {
        Self {
            id,
            title: params.title,
            description: params.description,
            proposer: params.proposer,
            state: ProposalState::Pending,
            start_time: now,
            end_time: now + params.voting_duration,
            quorum: params.quorum,
            votes: VotingRecord::new(),
            queued_at: 0,
        }
    }

    /// Activate the proposal (Pending -> Active).
    pub fn activate(&mut self) -> Result<()> {
        self.transition(ProposalState::Pending, ProposalState::Active)
    }

    /// Finalize voting. Transitions to Succeeded or Defeated based on results.
    pub fn finalize(&mut self) -> Result<()> {
        if self.state != ProposalState::Active {
            return Err(GovernanceError::InvalidTransition {
                from: self.state,
                to: ProposalState::Succeeded,
            });
        }

        let total = self.votes.total_votes();
        if total < self.quorum {
            self.state = ProposalState::Defeated;
            return Ok(());
        }

        if self.votes.for_votes > self.votes.against_votes {
            self.state = ProposalState::Succeeded;
        } else {
            self.state = ProposalState::Defeated;
        }

        Ok(())
    }

    /// Queue for timelock execution (Succeeded -> Queued).
    pub fn queue(&mut self, now: u64) -> Result<()> {
        self.transition(ProposalState::Succeeded, ProposalState::Queued)?;
        self.queued_at = now;
        Ok(())
    }

    /// Mark as executed (Queued -> Executed).
    pub fn execute(&mut self) -> Result<()> {
        self.transition(ProposalState::Queued, ProposalState::Executed)
    }

    /// Cancel the proposal (from Pending or Active).
    pub fn cancel(&mut self) -> Result<()> {
        if self.state != ProposalState::Pending && self.state != ProposalState::Active {
            return Err(GovernanceError::InvalidTransition {
                from: self.state,
                to: ProposalState::Cancelled,
            });
        }
        self.state = ProposalState::Cancelled;
        Ok(())
    }

    /// Whether the voting period is currently active given the current time.
    pub fn is_voting_open(&self, now: u64) -> bool {
        self.state == ProposalState::Active && now >= self.start_time && now < self.end_time
    }

    fn transition(&mut self, expected: ProposalState, next: ProposalState) -> Result<()> {
        if self.state != expected {
            return Err(GovernanceError::InvalidTransition {
                from: self.state,
                to: next,
            });
        }
        self.state = next;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_params() -> ProposalParams {
        ProposalParams {
            title: "Fund dev team".into(),
            description: "Allocate 1000 tokens to dev".into(),
            proposer: "0xabc".into(),
            voting_duration: 3600,
            quorum: 100,
        }
    }

    #[test]
    fn new_proposal_is_pending() {
        let p = Proposal::new(1, sample_params(), 1000);
        assert_eq!(p.state, ProposalState::Pending);
        assert_eq!(p.start_time, 1000);
        assert_eq!(p.end_time, 4600);
    }

    #[test]
    fn activate_from_pending() {
        let mut p = Proposal::new(1, sample_params(), 1000);
        p.activate().unwrap();
        assert_eq!(p.state, ProposalState::Active);
    }

    #[test]
    fn activate_from_active_fails() {
        let mut p = Proposal::new(1, sample_params(), 1000);
        p.activate().unwrap();
        assert!(p.activate().is_err());
    }

    #[test]
    fn finalize_succeeded() {
        let mut p = Proposal::new(1, sample_params(), 1000);
        p.activate().unwrap();
        p.votes.for_votes = 80;
        p.votes.against_votes = 30;
        p.votes.abstain_votes = 10;
        p.finalize().unwrap();
        assert_eq!(p.state, ProposalState::Succeeded);
    }

    #[test]
    fn finalize_defeated_by_majority() {
        let mut p = Proposal::new(1, sample_params(), 1000);
        p.activate().unwrap();
        p.votes.for_votes = 30;
        p.votes.against_votes = 80;
        p.votes.abstain_votes = 10;
        p.finalize().unwrap();
        assert_eq!(p.state, ProposalState::Defeated);
    }

    #[test]
    fn finalize_defeated_no_quorum() {
        let mut p = Proposal::new(1, sample_params(), 1000);
        p.activate().unwrap();
        p.votes.for_votes = 50;
        p.votes.against_votes = 10;
        // total = 60 < quorum 100
        p.finalize().unwrap();
        assert_eq!(p.state, ProposalState::Defeated);
    }

    #[test]
    fn queue_and_execute() {
        let mut p = Proposal::new(1, sample_params(), 1000);
        p.activate().unwrap();
        p.votes.for_votes = 200;
        p.finalize().unwrap();
        p.queue(5000).unwrap();
        assert_eq!(p.state, ProposalState::Queued);
        assert_eq!(p.queued_at, 5000);
        p.execute().unwrap();
        assert_eq!(p.state, ProposalState::Executed);
    }

    #[test]
    fn cancel_from_pending() {
        let mut p = Proposal::new(1, sample_params(), 1000);
        p.cancel().unwrap();
        assert_eq!(p.state, ProposalState::Cancelled);
    }

    #[test]
    fn cancel_from_executed_fails() {
        let mut p = Proposal::new(1, sample_params(), 1000);
        p.activate().unwrap();
        p.votes.for_votes = 200;
        p.finalize().unwrap();
        p.queue(5000).unwrap();
        p.execute().unwrap();
        assert!(p.cancel().is_err());
    }

    #[test]
    fn is_voting_open() {
        let mut p = Proposal::new(1, sample_params(), 1000);
        assert!(!p.is_voting_open(1000));
        p.activate().unwrap();
        assert!(p.is_voting_open(2000));
        assert!(!p.is_voting_open(5000)); // after end_time
    }
}
