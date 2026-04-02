//! Example: Xorion DAO Governance demo.

use xos_governance::{
    DelegationRegistry, GovernorAbi, Proposal, ProposalParams, Timelock, Treasury, VoteType,
};

fn main() {
    println!("=== Xorion DAO Governance Demo ===\n");

    // ── Setup delegation ───────────────────────────────
    println!("[1] Delegation");
    let mut reg = DelegationRegistry::new();
    reg.set_balance("0xAlice", 5000);
    reg.set_balance("0xBob", 3000);
    reg.set_balance("0xCharlie", 1000);
    reg.delegate("0xAlice", "0xBob").unwrap();
    println!("    Alice (5000 tokens) delegates to Bob");
    println!("    Bob voting power: {} (own 3000 + delegated 5000)", reg.voting_power("0xBob"));
    println!("    Charlie voting power: {}\n", reg.voting_power("0xCharlie"));

    // ── Create proposal ────────────────────────────────
    println!("[2] Create Proposal");
    let params = ProposalParams {
        title: "Fund development team".into(),
        description: "Allocate 2000 tokens from treasury to dev team".into(),
        proposer: "0xAlice".into(),
        voting_duration: 86400,
        quorum: 5000,
    };
    let mut proposal = Proposal::new(1, params, 1_000_000);
    proposal.activate().unwrap();
    println!("    Proposal #{}: {}", proposal.id, proposal.title);
    println!("    State: {:?}", proposal.state);
    println!("    Quorum required: {}\n", proposal.quorum);

    // ── Voting ─────────────────────────────────────────
    println!("[3] Voting");
    let bob_power = reg.voting_power("0xBob");
    proposal.votes.cast_vote("0xBob", VoteType::For, bob_power).unwrap();
    println!("    Bob votes FOR with {} power", bob_power);

    let charlie_power = reg.voting_power("0xCharlie");
    proposal.votes.cast_vote("0xCharlie", VoteType::Against, charlie_power).unwrap();
    println!("    Charlie votes AGAINST with {} power", charlie_power);

    println!("    Results: For={}, Against={}, Abstain={}",
        proposal.votes.for_votes, proposal.votes.against_votes, proposal.votes.abstain_votes);
    println!("    Total votes: {} (quorum: {})\n", proposal.votes.total_votes(), proposal.quorum);

    // ── Finalize ───────────────────────────────────────
    println!("[4] Finalize");
    proposal.finalize().unwrap();
    println!("    State: {:?}\n", proposal.state);

    // ── Timelock + Execute ─────────────────────────────
    println!("[5] Queue & Execute");
    let timelock = Timelock::new(3600);
    proposal.queue(2_000_000).unwrap();
    println!("    Queued at t=2000000, delay={}s", timelock.delay());
    println!("    Earliest execution: t={}", timelock.execution_time(proposal.queued_at));
    timelock.can_execute(proposal.queued_at, 2_004_000).unwrap();
    proposal.execute().unwrap();
    println!("    State: {:?}\n", proposal.state);

    // ── Treasury ───────────────────────────────────────
    println!("[6] Treasury");
    let mut treasury = Treasury::new();
    treasury.deposit(10_000, "Initial DAO funding");
    println!("    Balance after deposit: {}", treasury.balance());
    treasury.withdraw(2000, proposal.id, "Dev team grant").unwrap();
    println!("    Balance after withdrawal: {}", treasury.balance());
    println!("    Transactions: {}\n", treasury.history().len());

    // ── ABI encoding ───────────────────────────────────
    println!("[7] Governor ABI Encoding");
    println!("    propose() selector: {}", GovernorAbi::selector_hex("propose(address[],uint256[],bytes[],string)"));
    println!("    castVote() selector: {}", GovernorAbi::selector_hex("castVote(uint256,uint8)"));
    println!("    delegate() selector: {}", GovernorAbi::selector_hex("delegate(address)"));
    let calldata = GovernorAbi::encode_cast_vote(1, 1);
    println!("    castVote(1, For) calldata: 0x{} ({} bytes)", hex::encode(&calldata), calldata.len());

    println!("\nDone. DAO governance fully operational.");
}
