# X-OS Governance (xos-governance)

[![Crates.io](https://img.shields.io/crates/v/xorion-governance.svg)](https://crates.io/crates/xorion-governance)
[![Documentation](https://docs.rs/xorion-governance/badge.svg)](https://docs.rs/xorion-governance)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Overview

X-OS Governance provides a complete DAO governance framework for decentralized decision-making, proposal management, voting mechanisms, and treasury control within the Xorion ecosystem.

## Features

| Feature | Description | Status |
|---------|-------------|--------|
| 🗳️ Proposal Creation | Create and manage governance proposals | ✅ Complete |
| ✅ Voting Mechanisms | Multiple voting strategies (token-weighted, quadratic) | ✅ Complete |
| 💰 Treasury Management | Secure multi-sig treasury control | ✅ Complete |
| ⏱️ Timelock | Configurable execution delays | ✅ Complete |
| 📊 Delegation | Vote delegation and power tracking | ✅ Complete |
| 🔍 Transparency | Full on-chain governance history | ✅ Complete |

## Installation

\\\	oml
[dependencies]
xorion-governance = "2.0"
xorion-core = "2.0"
tokio = { version = "1.35", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
\\\

## Quick Start

\\\ust
use xorion_governance::{DaoGovernance, Proposal, VoteType};
use xorion_core::wallet::Wallet;

// Initialize governance
let mut governance = DaoGovernance::new(dao_address);

// Create proposal
let proposal = Proposal::new()
    .title("Upgrade Protocol")
    .description("Implement v3.0 changes")
    .voting_period(7 * 24 * 60 * 60); // 7 days

let proposal_id = governance.create_proposal(proposal, &wallet).await?;

// Cast vote
governance.vote(proposal_id, VoteType::Yes, voting_power, &wallet).await?;

// Execute proposal (after voting period)
governance.execute_proposal(proposal_id).await?;
\\\

## Voting Strategies

### Token-Weighted Voting
\\\ust
let strategy = VotingStrategy::TokenWeighted {
    token_address: token_contract,
    snapshot_block: Some(block_number),
};
\\\

### Quadratic Voting
\\\ust
let strategy = VotingStrategy::Quadratic {
    max_votes_per_user: 100,
};
\\\

## Governance Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| voting_period | 7 days | Time allowed for voting |
| quorum | 10% | Minimum participation required |
| threshold | 50% | Approval threshold for passing |
| timelock | 2 days | Delay before execution |
| proposal_deposit | 100 tokens | Required deposit to create proposal |

## Architecture

\\\
┌─────────────────────────────────────────┐
│         X-OS Governance                 │
├─────────────────────────────────────────┤
│  ┌─────────────┐  ┌──────────────────┐  │
│  │  Proposal   │  │    Voting        │  │
│  │  Manager    │  │    Engine        │  │
│  └─────────────┘  └──────────────────┘  │
│  ┌─────────────┐  ┌──────────────────┐  │
│  │  Treasury   │  │    Timelock      │  │
│  │  Controller │  │    Manager       │  │
│  └─────────────┘  └──────────────────┘  │
│  ┌─────────────┐  ┌──────────────────┐  │
│  │  Delegation │  │    Analytics     │  │
│  │  Tracker    │  │    Dashboard     │  │
│  └─────────────┘  └──────────────────┘  │
└─────────────────────────────────────────┘
         ▲
         │
    On-Chain Storage
\\\

## Events

\\\ust
pub enum GovernanceEvent {
    ProposalCreated(u64, Address),
    VoteCast(u64, Address, VoteType, u64),
    ProposalExecuted(u64),
    ProposalCancelled(u64),
    TreasuryTransfer(Address, Address, u64),
}
\\\

## Development

\\\ash
# Build
cargo build --release

# Test
cargo test

# Lint
cargo clippy

# Generate docs
cargo doc --open
\\\

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| xorion-core | 2.0 | Core utilities |
| xorion-storage | 2.0 | IPFS storage |
| tokio | 1.35 | Async runtime |
| serde | 1.0 | Serialization |
| sha3 | 0.10 | Hashing |

## Security Considerations

- **Multi-sig**: Treasury requires multiple signatures
- **Timelock**: Prevents rushed executions
- **Snapshot**: Voting power captured at proposal creation
- **Reentrancy Guards**: Protection against reentrancy attacks

## Troubleshooting

**Issue**: Vote not counted  
**Solution**: Ensure wallet had tokens at snapshot block

**Issue**: Proposal won't execute  
**Solution**: Check quorum and threshold requirements met

## License

MIT License - see [LICENSE](../LICENSE) for details.

## Links

- [Main Documentation](../README.md)
- [Contributing Guide](../CONTRIBUTING.md)
- [Change Log](../CHANGELOG.md)
