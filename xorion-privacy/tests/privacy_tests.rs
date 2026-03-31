use ark_bn254::Fr;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem};
use xorion_privacy::{
    AgeVerificationCircuit, BalanceProofCircuit, PrivateTxCircuit, ProofCache, ProofSystem,
};

// ── Circuit constraint satisfaction tests ──────────────────

#[test]
fn private_tx_satisfied() {
    let cs = ConstraintSystem::<Fr>::new_ref();
    let circuit = PrivateTxCircuit::new(10_000, 500);
    circuit
        .generate_constraints(cs.clone())
        .expect("constraint generation failed");
    assert!(cs.is_satisfied().unwrap());
}

#[test]
fn age_verify_satisfied() {
    let cs = ConstraintSystem::<Fr>::new_ref();
    let circuit = AgeVerificationCircuit::new(1995, 2026, 18);
    circuit
        .generate_constraints(cs.clone())
        .expect("constraint generation failed");
    assert!(cs.is_satisfied().unwrap());
}

#[test]
fn balance_proof_satisfied() {
    let cs = ConstraintSystem::<Fr>::new_ref();
    let circuit = BalanceProofCircuit::new(50_000, 10_000);
    circuit
        .generate_constraints(cs.clone())
        .expect("constraint generation failed");
    assert!(cs.is_satisfied().unwrap());
}

// ── Groth16 end-to-end tests ──────────────────────────────

#[test]
fn groth16_private_tx_end_to_end() {
    let (pk, vk) = ProofSystem::setup_private_tx().unwrap();
    let (proof, inputs) = ProofSystem::prove_private_tx(&pk, 1_000_000, 50_000).unwrap();
    assert!(ProofSystem::verify(&vk, &inputs, &proof).unwrap());
}

#[test]
fn groth16_age_verify_end_to_end() {
    let (pk, vk) = ProofSystem::setup_age_verification().unwrap();
    let (proof, inputs) = ProofSystem::prove_age(&pk, 2000, 2026, 21).unwrap();
    assert!(ProofSystem::verify(&vk, &inputs, &proof).unwrap());
}

#[test]
fn groth16_balance_proof_end_to_end() {
    let (pk, vk) = ProofSystem::setup_balance_proof().unwrap();
    let (proof, inputs) = ProofSystem::prove_balance(&pk, 100_000, 25_000).unwrap();
    assert!(ProofSystem::verify(&vk, &inputs, &proof).unwrap());
}

#[test]
fn groth16_wrong_inputs_rejected() {
    let (pk, vk) = ProofSystem::setup_private_tx().unwrap();
    let (proof, _) = ProofSystem::prove_private_tx(&pk, 1000, 100).unwrap();
    // Tamper with the public input
    let fake_inputs = vec![Fr::from(42u64)];
    let valid = ProofSystem::verify(&vk, &fake_inputs, &proof).unwrap();
    assert!(!valid);
}

#[test]
fn proof_serialization_roundtrip() {
    let (pk, vk) = ProofSystem::setup_balance_proof().unwrap();
    let (proof, inputs) = ProofSystem::prove_balance(&pk, 9000, 100).unwrap();

    let bytes = ProofSystem::serialize_proof(&proof).unwrap();
    assert!(!bytes.is_empty());

    let restored = ProofSystem::deserialize_proof(&bytes).unwrap();
    assert!(ProofSystem::verify(&vk, &inputs, &restored).unwrap());
}

// ── Cache tests ───────────────────────────────────────────

#[test]
fn cache_stores_and_retrieves_proofs() {
    let cache = ProofCache::new(300);
    let (pk, _) = ProofSystem::setup_balance_proof().unwrap();
    let (proof, _) = ProofSystem::prove_balance(&pk, 5000, 100).unwrap();
    let bytes = ProofSystem::serialize_proof(&proof).unwrap();

    cache.insert("balance:5000:100".into(), bytes.clone());
    assert_eq!(cache.get("balance:5000:100").unwrap(), bytes);
}

#[test]
fn cache_miss_returns_none() {
    let cache = ProofCache::new(300);
    assert!(cache.get("nonexistent").is_none());
}
