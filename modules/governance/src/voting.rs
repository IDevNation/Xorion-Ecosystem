//! Token-weighted voting with quorum enforcement.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{GovernanceError, Result};

/// Type of vote cast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VoteType {
    For,
    Against,
    Abstain,
}

/// A single vote with token weight.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub voter: String,
    pub vote_type: VoteType,
    pub weight: u64,
}

/// Aggregated voting record for a proposal.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VotingRecord {
    pub for_votes: u64,
    pub against_votes: u64,
    pub abstain_votes: u64,
    /// Individual votes by address.
    pub voters: HashMap<String, Vote>,
}

impl VotingRecord {
    pub fn new() -> Self {
        Self::default()
    }

    /// Cast a token-weighted vote.
    pub fn cast_vote(&mut self, voter: &str, vote_type: VoteType, weight: u64) -> Result<()> {
        if self.voters.contains_key(voter) {
            return Err(GovernanceError::AlreadyVoted(voter.to_string()));
        }

        if weight == 0 {
            return Err(GovernanceError::InsufficientPower { have: 0, need: 1 });
        }

        match vote_type {
            VoteType::For => self.for_votes += weight,
            VoteType::Against => self.against_votes += weight,
            VoteType::Abstain => self.abstain_votes += weight,
        }

        self.voters.insert(
            voter.to_string(),
            Vote {
                voter: voter.to_string(),
                vote_type,
                weight,
            },
        );

        Ok(())
    }

    /// Total votes cast (For + Against + Abstain).
    pub fn total_votes(&self) -> u64 {
        self.for_votes + self.against_votes + self.abstain_votes
    }

    /// Whether quorum is met.
    pub fn quorum_reached(&self, quorum: u64) -> bool {
        self.total_votes() >= quorum
    }

    /// Whether the proposal passed (For > Against, quorum met).
    pub fn passed(&self, quorum: u64) -> bool {
        self.quorum_reached(quorum) && self.for_votes > self.against_votes
    }

    /// Number of unique voters.
    pub fn voter_count(&self) -> usize {
        self.voters.len()
    }

    /// Get a voter's vote.
    pub fn get_vote(&self, voter: &str) -> Option<&Vote> {
        self.voters.get(voter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cast_for_vote() {
        let mut record = VotingRecord::new();
        record.cast_vote("0xAlice", VoteType::For, 100).unwrap();
        assert_eq!(record.for_votes, 100);
        assert_eq!(record.total_votes(), 100);
    }

    #[test]
    fn cast_all_types() {
        let mut record = VotingRecord::new();
        record.cast_vote("0xA", VoteType::For, 50).unwrap();
        record.cast_vote("0xB", VoteType::Against, 30).unwrap();
        record.cast_vote("0xC", VoteType::Abstain, 20).unwrap();
        assert_eq!(record.for_votes, 50);
        assert_eq!(record.against_votes, 30);
        assert_eq!(record.abstain_votes, 20);
        assert_eq!(record.total_votes(), 100);
        assert_eq!(record.voter_count(), 3);
    }

    #[test]
    fn double_vote_rejected() {
        let mut record = VotingRecord::new();
        record.cast_vote("0xA", VoteType::For, 10).unwrap();
        assert!(record.cast_vote("0xA", VoteType::Against, 10).is_err());
    }

    #[test]
    fn zero_weight_rejected() {
        let mut record = VotingRecord::new();
        assert!(record.cast_vote("0xA", VoteType::For, 0).is_err());
    }

    #[test]
    fn quorum_check() {
        let mut record = VotingRecord::new();
        record.cast_vote("0xA", VoteType::For, 60).unwrap();
        assert!(!record.quorum_reached(100));
        record.cast_vote("0xB", VoteType::Against, 50).unwrap();
        assert!(record.quorum_reached(100));
    }

    #[test]
    fn passed_check() {
        let mut record = VotingRecord::new();
        record.cast_vote("0xA", VoteType::For, 70).unwrap();
        record.cast_vote("0xB", VoteType::Against, 30).unwrap();
        assert!(record.passed(50));
        assert!(record.passed(100));
    }

    #[test]
    fn not_passed_against_majority() {
        let mut record = VotingRecord::new();
        record.cast_vote("0xA", VoteType::For, 30).unwrap();
        record.cast_vote("0xB", VoteType::Against, 70).unwrap();
        assert!(!record.passed(50));
    }

    #[test]
    fn get_individual_vote() {
        let mut record = VotingRecord::new();
        record.cast_vote("0xA", VoteType::Abstain, 25).unwrap();
        let vote = record.get_vote("0xA").unwrap();
        assert_eq!(vote.vote_type, VoteType::Abstain);
        assert_eq!(vote.weight, 25);
    }
}
