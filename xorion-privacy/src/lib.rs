//! Xorion Privacy Layer — zk-SNARKs for the Xorion Web3 OS
//!
//! Provides Groth16 zero-knowledge proofs for:
//! - **Private transactions** — prove solvency without revealing balance or amount
//! - **Age verification** — prove age >= 18 without revealing date of birth
//! - **Balance proofs** — prove sufficient funds without revealing exact balance

pub mod cache;
pub mod circuits;
pub mod proof;

pub use cache::ProofCache;
pub use circuits::{AgeVerificationCircuit, BalanceProofCircuit, PrivateTxCircuit};
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
