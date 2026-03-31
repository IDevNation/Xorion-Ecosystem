//! Cross-crate integration tests — verifies all 9 phases work together.
//!
//! This test suite exercises the full Xorion Web3 OS stack:
//! Phase 1 (Wallet) -> Phase 2 (RPC) -> Phase 3 (Contracts) ->
//! Phase 6 (WASM Runtime) -> Phase 7 (ZK Privacy) ->
//! Phase 8 (IPFS Storage) -> Phase 9 (DAO Governance)

use xorion_sdk::Wallet;

const MNEMONIC: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

// ── Phase 1: Wallet SDK ───────────────────────────────────

#[test]
fn wallet_creation_and_derivation() {
    let wallet = Wallet::from_mnemonic(MNEMONIC).unwrap();
    let eth = wallet.derive_eth_address().unwrap();
    let sol = wallet.derive_solana_address().unwrap();

    assert!(eth.starts_with("0x"));
    assert_eq!(eth.len(), 42);
    assert!(!sol.is_empty());
}

#[test]
fn wallet_derives_both_chains() {
    let wallet = Wallet::from_mnemonic(MNEMONIC).unwrap();
    let eth = wallet.derive_eth_address().unwrap();
    let sol = wallet.derive_solana_address().unwrap();
    // Both addresses are deterministic from the same mnemonic
    let wallet2 = Wallet::from_mnemonic(MNEMONIC).unwrap();
    assert_eq!(eth, wallet2.derive_eth_address().unwrap());
    assert_eq!(sol, wallet2.derive_solana_address().unwrap());
}

// ── Phase 2: RPC types ───────────────────────────────────

#[test]
fn rpc_providers_construct() {
    use xorion_sdk::rpc::ethereum::EthereumProvider;
    use xorion_sdk::rpc::solana::SolanaProvider;

    let _eth = EthereumProvider::new("https://eth.llamarpc.com");
    let _sol = SolanaProvider::new("https://api.mainnet-beta.solana.com");
}

// ── Phase 3: Contract ABI ─────────────────────────────────

#[test]
fn contract_abi_encoding() {
    use xorion_sdk::contract::abi;

    let selector = abi::function_selector("transfer(address,uint256)");
    assert_eq!(selector.len(), 4);

    // Verify known selector for ERC-20 transfer
    assert_eq!(hex::encode(selector), "a9059cbb");
}

// ── Phase 6: WASM Runtime ─────────────────────────────────

#[test]
fn wasm_runtime_with_wallet() {
    use xorion_core::{Permission, WasmRuntime};

    let mut rt = WasmRuntime::new().unwrap();
    rt.init_wallet(MNEMONIC).unwrap();
    assert!(rt.bridge().eth_address().starts_with("0x"));

    rt.sandbox_mut().grant_permission(Permission::SignTransaction);
    assert!(rt.sandbox_mut().has_permission(&Permission::SignTransaction));
}

#[test]
fn wasm_runtime_load_and_execute() {
    use xorion_core::WasmRuntime;

    // Minimal WASM: (module (func (export "_start")))
    const MINIMAL_WASM: &[u8] = &[
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
        0x01, 0x04, 0x01, 0x60, 0x00, 0x00,
        0x03, 0x02, 0x01, 0x00,
        0x07, 0x0a, 0x01, 0x06, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x00, 0x00,
        0x0a, 0x04, 0x01, 0x02, 0x00, 0x0b,
    ];

    let mut rt = WasmRuntime::new().unwrap();
    rt.load_from_bytes(MINIMAL_WASM).unwrap();
    rt.run().unwrap();
}

// ── Phase 7: ZK Privacy ──────────────────────────────────

#[test]
fn zk_private_transaction_proof() {
    use xorion_zk::ProofSystem;

    let (pk, vk) = ProofSystem::setup_private_tx().unwrap();
    let (proof, inputs) = ProofSystem::prove_private_tx(&pk, 10_000, 500).unwrap();
    assert!(ProofSystem::verify(&vk, &inputs, &proof).unwrap());
}

#[test]
fn zk_age_verification_proof() {
    use xorion_zk::ProofSystem;

    let (pk, vk) = ProofSystem::setup_age_verification().unwrap();
    let (proof, inputs) = ProofSystem::prove_age(&pk, 1990, 2026, 18).unwrap();
    assert!(ProofSystem::verify(&vk, &inputs, &proof).unwrap());
}

#[test]
fn zk_balance_proof() {
    use xorion_zk::ProofSystem;

    let (pk, vk) = ProofSystem::setup_balance_proof().unwrap();
    let (proof, inputs) = ProofSystem::prove_balance(&pk, 50_000, 1_000).unwrap();
    assert!(ProofSystem::verify(&vk, &inputs, &proof).unwrap());
}

// ── Phase 8: IPFS Storage ─────────────────────────────────

#[test]
fn ipfs_encrypted_storage_roundtrip() {
    use xorion_ipfs::{Encryption, VirtualFs};

    let enc = Encryption::from_password("xorion_beta", b"betasalt1234salt").unwrap();

    let data = b"wallet backup data for beta release";
    let encrypted = enc.encrypt(data).unwrap();
    let decrypted = enc.decrypt(&encrypted).unwrap();
    assert_eq!(&decrypted, data);

    let mut vfs = VirtualFs::new();
    vfs.mkdir("/backups").unwrap();
    vfs.add_file("/backups/wallet.enc", "QmBetaBackup", encrypted.len() as u64, true)
        .unwrap();
    assert!(vfs.get("/backups/wallet.enc").unwrap().encrypted);
}

#[test]
fn ipfs_cache_and_pinning() {
    use xorion_ipfs::{FileCache, PinningService};

    let tmp = tempfile::TempDir::new().unwrap();
    let mut cache = FileCache::new(tmp.path(), 1_000_000).unwrap();
    cache.put("QmBeta1", b"cached content").unwrap();
    assert_eq!(cache.get("QmBeta1").unwrap(), b"cached content");

    let mut pins = PinningService::new();
    pins.pin("QmBeta1", "beta_asset", 14);
    assert!(pins.is_pinned("QmBeta1"));
}

// ── Phase 9: DAO Governance ───────────────────────────────

#[test]
fn dao_full_governance_flow() {
    use xorion_governance::{
        DelegationRegistry, Proposal, ProposalParams, ProposalState, Timelock, Treasury, VoteType,
    };

    // Setup delegation
    let mut reg = DelegationRegistry::new();
    reg.set_balance("0xAlice", 5000);
    reg.set_balance("0xBob", 3000);
    reg.delegate("0xAlice", "0xBob").unwrap();
    assert_eq!(reg.voting_power("0xBob"), 8000);

    // Create and vote on proposal
    let mut proposal = Proposal::new(
        1,
        ProposalParams {
            title: "Beta release funding".into(),
            description: "Fund the v1.0.0-beta release".into(),
            proposer: "0xAlice".into(),
            voting_duration: 86400,
            quorum: 5000,
        },
        1_000_000,
    );
    proposal.activate().unwrap();
    proposal
        .votes
        .cast_vote("0xBob", VoteType::For, reg.voting_power("0xBob"))
        .unwrap();
    proposal.finalize().unwrap();
    assert_eq!(proposal.state, ProposalState::Succeeded);

    // Timelock + execute
    let tl = Timelock::new(3600);
    proposal.queue(2_000_000).unwrap();
    tl.can_execute(proposal.queued_at, 2_004_000).unwrap();
    proposal.execute().unwrap();
    assert_eq!(proposal.state, ProposalState::Executed);

    // Treasury withdrawal
    let mut treasury = Treasury::new();
    treasury.deposit(100_000, "DAO funds");
    treasury.withdraw(10_000, proposal.id, "Beta release").unwrap();
    assert_eq!(treasury.balance(), 90_000);
}

// ── Cross-phase: Wallet + WASM + ZK ──────────────────────

#[test]
fn wallet_powers_wasm_and_zk() {
    use xorion_zk::ProofSystem;
    use xorion_core::WasmRuntime;

    // Wallet creates addresses
    let wallet = Wallet::from_mnemonic(MNEMONIC).unwrap();
    let eth_addr = wallet.derive_eth_address().unwrap();

    // WASM runtime uses same wallet
    let rt = WasmRuntime::new().unwrap();
    rt.init_wallet(MNEMONIC).unwrap();
    assert_eq!(rt.bridge().eth_address(), eth_addr);

    // ZK proves balance without revealing it
    let (pk, vk) = ProofSystem::setup_balance_proof().unwrap();
    let (proof, inputs) = ProofSystem::prove_balance(&pk, 100_000, 1_000).unwrap();
    assert!(ProofSystem::verify(&vk, &inputs, &proof).unwrap());
}

// ── Cross-phase: Storage + Governance ─────────────────────

#[test]
fn governance_controls_storage() {
    use xorion_governance::{Proposal, ProposalParams, ProposalState, VoteType};
    use xorion_ipfs::VirtualFs;

    // Governance approves file storage
    let mut proposal = Proposal::new(
        99,
        ProposalParams {
            title: "Store DAO documents on IPFS".into(),
            description: "Upload governance docs".into(),
            proposer: "0xAdmin".into(),
            voting_duration: 3600,
            quorum: 10,
        },
        0,
    );
    proposal.activate().unwrap();
    proposal.votes.cast_vote("0xVoter", VoteType::For, 100).unwrap();
    proposal.finalize().unwrap();
    assert_eq!(proposal.state, ProposalState::Succeeded);

    // After approval, store files
    let mut vfs = VirtualFs::new();
    vfs.mkdir("/dao").unwrap();
    vfs.add_file("/dao/charter.pdf", "QmCharter", 4096, false).unwrap();
    assert!(vfs.exists("/dao/charter.pdf"));
}
