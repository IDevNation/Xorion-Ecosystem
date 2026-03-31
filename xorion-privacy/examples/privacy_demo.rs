//! Example: Xorion zk-SNARKs Privacy Layer demo.

use xorion_zk::ProofSystem;

fn main() {
    println!("=== Xorion zk-SNARKs Privacy Demo ===\n");

    // ── Private Transaction ────────────────────────────
    println!("[1] Private Transaction Proof");
    println!("    Proving: balance(10000) >= amount(500) without revealing either");
    let (pk, vk) = ProofSystem::setup_private_tx().expect("setup failed");
    let (proof, inputs) = ProofSystem::prove_private_tx(&pk, 10_000, 500).expect("prove failed");
    let valid = ProofSystem::verify(&vk, &inputs, &proof).expect("verify failed");
    println!("    Proof valid: {valid}");
    let bytes = ProofSystem::serialize_proof(&proof).unwrap();
    println!("    Proof size: {} bytes\n", bytes.len());

    // ── Age Verification ───────────────────────────────
    println!("[2] Age Verification Proof");
    println!("    Proving: age >= 18 without revealing birth year");
    let (pk, vk) = ProofSystem::setup_age_verification().expect("setup failed");
    let (proof, inputs) = ProofSystem::prove_age(&pk, 1995, 2026, 18).expect("prove failed");
    let valid = ProofSystem::verify(&vk, &inputs, &proof).expect("verify failed");
    println!("    Proof valid: {valid}");
    let bytes = ProofSystem::serialize_proof(&proof).unwrap();
    println!("    Proof size: {} bytes\n", bytes.len());

    // ── Balance Proof ──────────────────────────────────
    println!("[3] Balance Sufficiency Proof");
    println!("    Proving: balance >= 1000 without revealing exact amount");
    let (pk, vk) = ProofSystem::setup_balance_proof().expect("setup failed");
    let (proof, inputs) = ProofSystem::prove_balance(&pk, 50_000, 1_000).expect("prove failed");
    let valid = ProofSystem::verify(&vk, &inputs, &proof).expect("verify failed");
    println!("    Proof valid: {valid}");
    let bytes = ProofSystem::serialize_proof(&proof).unwrap();
    println!("    Proof size: {} bytes\n", bytes.len());

    println!("Done. All three ZK proof types generated and verified.");
}
