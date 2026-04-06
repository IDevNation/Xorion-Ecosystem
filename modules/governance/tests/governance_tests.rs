use xos_governance::{
    DelegationRegistry, GovernorAbi, Proposal, ProposalParams, ProposalState, Timelock, Treasury,
    VoteType, VotingRecord,
};

// ── Full proposal lifecycle ───────────────────────────────

fn default_params() -> ProposalParams {
    ProposalParams {
        title: "Fund marketing".into(),
        description: "Allocate 5000 tokens to marketing team".into(),
        proposer: "0xProposer".into(),
        voting_duration: 86400, // 1 day
        quorum: 1000,
    }
}

#[test]
fn full_proposal_lifecycle_succeeded() {
    let mut p = Proposal::new(1, default_params(), 1_000_000);
    assert_eq!(p.state, ProposalState::Pending);

    p.activate().unwrap();
    assert_eq!(p.state, ProposalState::Active);
    assert!(p.is_voting_open(1_050_000));

    p.votes.cast_vote("0xA", VoteType::For, 700).unwrap();
    p.votes.cast_vote("0xB", VoteType::For, 500).unwrap();
    p.votes.cast_vote("0xC", VoteType::Against, 200).unwrap();

    p.finalize().unwrap();
    assert_eq!(p.state, ProposalState::Succeeded);

    let tl = Timelock::new(3600);
    p.queue(2_000_000).unwrap();
    assert!(tl.can_execute(p.queued_at, 2_001_000).is_err());
    assert!(tl.can_execute(p.queued_at, 2_004_000).is_ok());

    p.execute().unwrap();
    assert_eq!(p.state, ProposalState::Executed);
}

#[test]
fn full_proposal_lifecycle_defeated() {
    let mut p = Proposal::new(2, default_params(), 1_000_000);
    p.activate().unwrap();

    p.votes.cast_vote("0xA", VoteType::Against, 800).unwrap();
    p.votes.cast_vote("0xB", VoteType::For, 300).unwrap();
    p.votes.cast_vote("0xC", VoteType::Abstain, 100).unwrap();

    p.finalize().unwrap();
    assert_eq!(p.state, ProposalState::Defeated);
}

#[test]
fn proposal_defeated_no_quorum() {
    let mut p = Proposal::new(3, default_params(), 0);
    p.activate().unwrap();
    p.votes.cast_vote("0xA", VoteType::For, 500).unwrap();
    // total 500 < quorum 1000
    p.finalize().unwrap();
    assert_eq!(p.state, ProposalState::Defeated);
}

#[test]
fn proposal_cancellation() {
    let mut p = Proposal::new(4, default_params(), 0);
    p.activate().unwrap();
    p.cancel().unwrap();
    assert_eq!(p.state, ProposalState::Cancelled);
}

// ── Delegation + voting integration ───────────────────────

#[test]
fn delegation_affects_voting_power() {
    let mut reg = DelegationRegistry::new();
    reg.set_balance("0xAlice", 1000);
    reg.set_balance("0xBob", 500);
    reg.delegate("0xAlice", "0xBob").unwrap();

    // Bob votes with delegated power
    let mut p = Proposal::new(5, default_params(), 0);
    p.activate().unwrap();
    let bob_power = reg.voting_power("0xBob");
    assert_eq!(bob_power, 1500);
    p.votes.cast_vote("0xBob", VoteType::For, bob_power).unwrap();
    p.finalize().unwrap();
    assert_eq!(p.state, ProposalState::Succeeded);
}

// ── Treasury + proposal integration ───────────────────────

#[test]
fn treasury_withdrawal_via_proposal() {
    let mut treasury = Treasury::new();
    treasury.deposit(10_000, "Initial funding");

    let mut p = Proposal::new(6, ProposalParams {
        title: "Withdraw 2000 for dev".into(),
        description: "Dev team grant".into(),
        proposer: "0xDev".into(),
        voting_duration: 3600,
        quorum: 100,
    }, 0);

    p.activate().unwrap();
    p.votes.cast_vote("0xVoter", VoteType::For, 500).unwrap();
    p.finalize().unwrap();
    assert_eq!(p.state, ProposalState::Succeeded);
    p.queue(1000).unwrap();
    p.execute().unwrap();

    treasury.withdraw(2000, p.id, "Dev team grant").unwrap();
    assert_eq!(treasury.balance(), 8000);
    assert_eq!(treasury.history().len(), 2);
}

// ── Governor ABI encoding ─────────────────────────────────

#[test]
fn governor_propose_encoding() {
    let data = GovernorAbi::encode_propose("Fund marketing team");
    assert!(!data.is_empty());
    assert_eq!(&data[..4], &GovernorAbi::selector("propose(address[],uint256[],bytes[],string)"));
}

#[test]
fn governor_cast_vote_encoding() {
    let data = GovernorAbi::encode_cast_vote(42, 1); // proposal 42, vote For
    assert_eq!(data.len(), 68);
    // Verify proposal ID is at offset 4..36
    assert_eq!(data[35], 42);
    // Verify support byte
    assert_eq!(data[67], 1);
}

#[test]
fn governor_delegate_encoding() {
    let data = GovernorAbi::encode_delegate("0xd8da6bf26964af9d7eed9e03e53415d37aa96045");
    assert_eq!(data.len(), 36);
}

// ── Voting edge cases ─────────────────────────────────────

#[test]
fn large_token_weighted_votes() {
    let mut record = VotingRecord::new();
    record.cast_vote("0xWhale", VoteType::For, 1_000_000_000).unwrap();
    record.cast_vote("0xShrimp", VoteType::Against, 1).unwrap();
    assert!(record.passed(100));
    assert_eq!(record.total_votes(), 1_000_000_001);
}

#[test]
fn abstain_counts_toward_quorum() {
    let mut record = VotingRecord::new();
    record.cast_vote("0xA", VoteType::For, 30).unwrap();
    record.cast_vote("0xB", VoteType::Abstain, 80).unwrap();
    assert!(record.quorum_reached(100)); // 30 + 80 = 110 >= 100
    assert!(record.passed(100)); // For(30) > Against(0)
}

#[test]
fn tie_vote_does_not_pass() {
    let mut record = VotingRecord::new();
    record.cast_vote("0xA", VoteType::For, 50).unwrap();
    record.cast_vote("0xB", VoteType::Against, 50).unwrap();
    assert!(!record.passed(50)); // tie: For is NOT > Against
}

// ── Timelock integration ──────────────────────────────────

#[test]
fn timelock_remaining_seconds_in_error() {
    let tl = Timelock::new(7200);
    let err = tl.can_execute(1000, 3000).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("5200")); // 7200 - (3000 - 1000) = 5200
}
