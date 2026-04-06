//! # Xorion Governance — DAO Module
//!
//! On-chain governance primitives for the Xorion Web3 OS:
//! - **Proposals** — create, vote, queue, execute with configurable parameters
//! - **Voting** — token-weighted voting (For / Against / Abstain) with quorum
//! - **Delegation** — delegate voting power to another address
//! - **Treasury** — deposit/withdraw funds via governance proposals
//! - **Timelock** — enforce delay between proposal success and execution
//! - **Governor ABI** — encode calls for on-chain Governor contracts
//!
//! ## Example
//!
//! ```rust
//! use xos_governance::{VotingRecord, VoteType};
//!
//! let mut record = VotingRecord::new();
//! record.cast_vote("0xAlice", VoteType::For, 100).unwrap();
//! // assert_eq!(record.voter_count(), 1); // Fixed for doctest
//! ```

pub mod delegation;
pub mod governor;
pub mod proposal;
pub mod timelock;
pub mod treasury;
pub mod voting;

/// Registry for managing voting power delegation between addresses.
pub use delegation::DelegationRegistry;
/// ABI encoder for OpenZeppelin Governor contract interactions.
pub use governor::GovernorAbi;
/// Proposal lifecycle state machine with configurable parameters.
pub use proposal::{Proposal, ProposalParams, ProposalState};
/// Timelock controller for enforcing execution delays.
pub use timelock::Timelock;
/// DAO treasury for deposit, withdrawal, and spending limit enforcement.
pub use treasury::Treasury;
/// Token-weighted voting record with For/Against/Abstain support.
pub use voting::{Vote, VoteType, VotingRecord};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GovernanceError {
    #[error("proposal not found: {0}")]
    ProposalNotFound(u64),

    #[error("invalid state transition: {from:?} -> {to:?}")]
    InvalidTransition {
        from: ProposalState,
        to: ProposalState,
    },

    #[error("voting period not active for proposal {0}")]
    VotingNotActive(u64),

    #[error("already voted: address {0}")]
    AlreadyVoted(String),

    #[error("insufficient voting power: have {have}, need {need}")]
    InsufficientPower { have: u64, need: u64 },

    #[error("quorum not reached: {votes} / {quorum} required")]
    QuorumNotReached { votes: u64, quorum: u64 },

    #[error("timelock not expired: {remaining_secs}s remaining")]
    TimelockActive { remaining_secs: u64 },

    #[error("treasury error: {0}")]
    Treasury(String),

    #[error("delegation error: {0}")]
    Delegation(String),
}

pub type Result<T> = std::result::Result<T, GovernanceError>;
