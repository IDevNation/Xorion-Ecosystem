//! ZK circuits for privacy operations.
//!
//! Each circuit defines R1CS constraints for a specific privacy use case.
//! Circuits implement `ConstraintSynthesizer` for use with Groth16.

mod age_verify;
mod balance_proof;
mod private_tx;

pub use age_verify::AgeVerificationCircuit;
pub use balance_proof::BalanceProofCircuit;
pub use private_tx::PrivateTxCircuit;
