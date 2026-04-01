//! # Xorion ZK — zk-SNARKs Privacy Layer
//!
//! Provides Groth16 zero-knowledge proofs on the BN254 curve for:
//! - **Private transactions** — prove solvency without revealing balance or amount
//! - **Age verification** — prove age >= 18 without revealing date of birth
//! - **Balance proofs** — prove sufficient funds without revealing exact balance
//!
//! ## Example
//!
//! ```rust,no_run
//! use xorion_zk::ProofSystem;
//!
//! let (pk, vk) = ProofSystem::setup_private_tx().unwrap();
//! let (proof, inputs) = ProofSystem::prove_private_tx(&pk, 1000, 100).unwrap();
//! assert!(ProofSystem::verify(&vk, &inputs, &proof).unwrap());
//! ```

pub mod cache;
pub mod circuits;
pub mod proof;

/// Thread-safe cache for generated proofs with TTL-based expiration.
pub use cache::ProofCache;
/// R1CS circuit definitions for Groth16 proving.
pub use circuits::{AgeVerificationCircuit, BalanceProofCircuit, PrivateTxCircuit};
/// High-level API for proof generation, verification, and serialization.
pub use proof::ProofSystem;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrivacyError {
    #[error("proof generation failed: {0}")]
    GenerationFailed(String),

    #[error("proof verification failed: {0}")]
    VerificationFailed(String),

    #[error("invalid parameters: {0}")]
    InvalidParams(String),

    #[error("serialization error: {0}")]
    SerializationError(String),

    #[error("cache error: {0}")]
    CacheError(String),
}

pub type Result<T> = std::result::Result<T, PrivacyError>;
