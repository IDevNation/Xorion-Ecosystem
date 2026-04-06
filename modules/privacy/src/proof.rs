//! Groth16 proof generation and verification.
//!
//! Wraps arkworks' Groth16 implementation with ergonomic methods
//! for each circuit type.

use ark_bn254::{Bn254, Fr};
use ark_groth16::{Groth16, Proof, ProvingKey, VerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_snark::SNARK;
use ark_std::rand::rngs::StdRng;
use ark_std::rand::SeedableRng;
use tracing::info;

use crate::circuits::{AgeVerificationCircuit, BalanceProofCircuit, PrivateTxCircuit};
use crate::{PrivacyError, Result};

/// Serialized proof bytes for transport/storage.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SerializedProof {
    pub proof_bytes: Vec<u8>,
    pub public_inputs: Vec<Vec<u8>>,
}

/// High-level interface for Groth16 proof operations.
pub struct ProofSystem;

impl ProofSystem {
    /// Run trusted setup for the private transaction circuit.
    pub fn setup_private_tx() -> Result<(ProvingKey<Bn254>, VerifyingKey<Bn254>)> {
        info!("Running trusted setup for PrivateTxCircuit");
        let mut rng = StdRng::from_entropy();
        Groth16::<Bn254>::circuit_specific_setup(PrivateTxCircuit::empty(), &mut rng)
            .map_err(|e| PrivacyError::GenerationFailed(format!("setup failed: {e}")))
    }

    /// Run trusted setup for the age verification circuit.
    pub fn setup_age_verification() -> Result<(ProvingKey<Bn254>, VerifyingKey<Bn254>)> {
        info!("Running trusted setup for AgeVerificationCircuit");
        let mut rng = StdRng::from_entropy();
        Groth16::<Bn254>::circuit_specific_setup(AgeVerificationCircuit::empty(), &mut rng)
            .map_err(|e| PrivacyError::GenerationFailed(format!("setup failed: {e}")))
    }

    /// Run trusted setup for the balance proof circuit.
    pub fn setup_balance_proof() -> Result<(ProvingKey<Bn254>, VerifyingKey<Bn254>)> {
        info!("Running trusted setup for BalanceProofCircuit");
        let mut rng = StdRng::from_entropy();
        Groth16::<Bn254>::circuit_specific_setup(BalanceProofCircuit::empty(), &mut rng)
            .map_err(|e| PrivacyError::GenerationFailed(format!("setup failed: {e}")))
    }

    /// Generate a Groth16 proof for a private transaction.
    pub fn prove_private_tx(
        pk: &ProvingKey<Bn254>,
        balance: u64,
        amount: u64,
    ) -> Result<(Proof<Bn254>, Vec<Fr>)> {
        info!("Generating private transaction proof");
        let circuit = PrivateTxCircuit::new(balance, amount);
        let public_inputs = circuit.public_inputs();
        let mut rng = StdRng::from_entropy();

        let proof = Groth16::<Bn254>::prove(pk, circuit, &mut rng)
            .map_err(|e| PrivacyError::GenerationFailed(e.to_string()))?;

        Ok((proof, public_inputs))
    }

    /// Generate a Groth16 proof for age verification.
    pub fn prove_age(
        pk: &ProvingKey<Bn254>,
        birth_year: u32,
        current_year: u32,
        minimum_age: u32,
    ) -> Result<(Proof<Bn254>, Vec<Fr>)> {
        info!("Generating age verification proof");
        let circuit = AgeVerificationCircuit::new(birth_year, current_year, minimum_age);
        let public_inputs = circuit.public_inputs();
        let mut rng = StdRng::from_entropy();

        let proof = Groth16::<Bn254>::prove(pk, circuit, &mut rng)
            .map_err(|e| PrivacyError::GenerationFailed(e.to_string()))?;

        Ok((proof, public_inputs))
    }

    /// Generate a Groth16 proof for balance sufficiency.
    pub fn prove_balance(
        pk: &ProvingKey<Bn254>,
        balance: u64,
        threshold: u64,
    ) -> Result<(Proof<Bn254>, Vec<Fr>)> {
        info!("Generating balance proof");
        let circuit = BalanceProofCircuit::new(balance, threshold);
        let public_inputs = circuit.public_inputs();
        let mut rng = StdRng::from_entropy();

        let proof = Groth16::<Bn254>::prove(pk, circuit, &mut rng)
            .map_err(|e| PrivacyError::GenerationFailed(e.to_string()))?;

        Ok((proof, public_inputs))
    }

    /// Verify a Groth16 proof against the verifying key and public inputs.
    pub fn verify(
        vk: &VerifyingKey<Bn254>,
        public_inputs: &[Fr],
        proof: &Proof<Bn254>,
    ) -> Result<bool> {
        Groth16::<Bn254>::verify(vk, public_inputs, proof)
            .map_err(|e| PrivacyError::VerificationFailed(e.to_string()))
    }

    /// Serialize a proof to bytes for storage or transport.
    pub fn serialize_proof(proof: &Proof<Bn254>) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();
        proof
            .serialize_compressed(&mut bytes)
            .map_err(|e| PrivacyError::SerializationError(e.to_string()))?;
        Ok(bytes)
    }

    /// Deserialize a proof from bytes.
    pub fn deserialize_proof(bytes: &[u8]) -> Result<Proof<Bn254>> {
        Proof::deserialize_compressed(bytes)
            .map_err(|e| PrivacyError::SerializationError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_private_tx() {
        let result = ProofSystem::setup_private_tx();
        assert!(result.is_ok());
    }

    #[test]
    fn setup_age_verification() {
        let result = ProofSystem::setup_age_verification();
        assert!(result.is_ok());
    }

    #[test]
    fn setup_balance_proof() {
        let result = ProofSystem::setup_balance_proof();
        assert!(result.is_ok());
    }

    #[test]
    fn full_groth16_private_tx() {
        let (pk, vk) = ProofSystem::setup_private_tx().unwrap();
        let (proof, inputs) = ProofSystem::prove_private_tx(&pk, 1000, 100).unwrap();
        assert!(ProofSystem::verify(&vk, &inputs, &proof).unwrap());
    }

    #[test]
    fn full_groth16_age_verification() {
        let (pk, vk) = ProofSystem::setup_age_verification().unwrap();
        let (proof, inputs) = ProofSystem::prove_age(&pk, 1990, 2026, 18).unwrap();
        assert!(ProofSystem::verify(&vk, &inputs, &proof).unwrap());
    }

    #[test]
    fn full_groth16_balance_proof() {
        let (pk, vk) = ProofSystem::setup_balance_proof().unwrap();
        let (proof, inputs) = ProofSystem::prove_balance(&pk, 5000, 1000).unwrap();
        assert!(ProofSystem::verify(&vk, &inputs, &proof).unwrap());
    }

    #[test]
    fn verification_fails_with_wrong_inputs() {
        let (pk, vk) = ProofSystem::setup_balance_proof().unwrap();
        let (proof, _) = ProofSystem::prove_balance(&pk, 5000, 1000).unwrap();
        // Use wrong public input
        let wrong_inputs = vec![Fr::from(9999u64)];
        let result = ProofSystem::verify(&vk, &wrong_inputs, &proof).unwrap();
        assert!(!result);
    }

    #[test]
    fn proof_serialization_roundtrip() {
        let (pk, _) = ProofSystem::setup_balance_proof().unwrap();
        let (proof, _) = ProofSystem::prove_balance(&pk, 5000, 1000).unwrap();
        let bytes = ProofSystem::serialize_proof(&proof).unwrap();
        let deserialized = ProofSystem::deserialize_proof(&bytes).unwrap();
        let re_bytes = ProofSystem::serialize_proof(&deserialized).unwrap();
        assert_eq!(bytes, re_bytes);
    }
}
