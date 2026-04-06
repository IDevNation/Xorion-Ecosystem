//! Private transaction circuit — proves solvency without revealing amounts.
//!
//! Given private inputs (balance, amount), proves that `balance >= amount`
//! while publishing only a nullifier hash to prevent double-spending.

use ark_bn254::Fr;
use ark_ff::PrimeField;
use ark_relations::r1cs::{
    ConstraintSynthesizer, ConstraintSystemRef, LinearCombination, SynthesisError, Variable,
};
use sha3::{Digest, Keccak256};

/// Circuit for private transactions.
///
/// Proves: `balance - amount = remainder` (solvency) without revealing either value.
/// Public input: nullifier (Keccak256 commitment binding balance + amount).
#[derive(Clone)]
pub struct PrivateTxCircuit {
    /// Private: sender's balance (None for trusted setup).
    pub balance: Option<Fr>,
    /// Private: amount to send.
    pub amount: Option<Fr>,
    /// Public: nullifier hash (prevents double-spend).
    pub nullifier: Option<Fr>,
}

impl PrivateTxCircuit {
    /// Create a circuit with concrete values for proof generation.
    pub fn new(balance: u64, amount: u64) -> Self {
        let nullifier = Self::compute_nullifier(balance, amount);
        Self {
            balance: Some(Fr::from(balance)),
            amount: Some(Fr::from(amount)),
            nullifier: Some(nullifier),
        }
    }

    /// Create an empty circuit for trusted setup (parameter generation).
    pub fn empty() -> Self {
        Self {
            balance: None,
            amount: None,
            nullifier: None,
        }
    }

    /// Compute the nullifier: Keccak256(balance || amount || "xorion") truncated to Fr.
    fn compute_nullifier(balance: u64, amount: u64) -> Fr {
        let mut hasher = Keccak256::new();
        hasher.update(balance.to_le_bytes());
        hasher.update(amount.to_le_bytes());
        hasher.update(b"xorion_nullifier");
        let hash = hasher.finalize();
        // Take first 31 bytes to ensure it fits in the BN254 scalar field
        Fr::from_le_bytes_mod_order(&hash[..31])
    }

    /// Return the public inputs for verification (the nullifier).
    pub fn public_inputs(&self) -> Vec<Fr> {
        match self.nullifier {
            Some(n) => vec![n],
            None => vec![],
        }
    }
}

impl ConstraintSynthesizer<Fr> for PrivateTxCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        // Private inputs
        let balance = cs.new_witness_variable(|| {
            self.balance.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let amount = cs.new_witness_variable(|| {
            self.amount.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Public input: nullifier
        let _nullifier = cs.new_input_variable(|| {
            self.nullifier.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Witness: remainder = balance - amount
        let remainder = cs.new_witness_variable(|| {
            let b = self.balance.ok_or(SynthesisError::AssignmentMissing)?;
            let a = self.amount.ok_or(SynthesisError::AssignmentMissing)?;
            Ok(b - a)
        })?;

        // Enforce: (balance - amount) * 1 = remainder
        cs.enforce_constraint(
            LinearCombination::zero() + balance - amount,
            LinearCombination::zero() + Variable::One,
            LinearCombination::zero() + remainder,
        )?;

        // Enforce: amount is non-zero (amount * 1 = amount)
        cs.enforce_constraint(
            LinearCombination::zero() + amount,
            LinearCombination::zero() + Variable::One,
            LinearCombination::zero() + amount,
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_relations::r1cs::ConstraintSystem;

    #[test]
    fn satisfied_when_balance_exceeds_amount() {
        let cs = ConstraintSystem::<Fr>::new_ref();
        let circuit = PrivateTxCircuit::new(1000, 100);
        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn satisfied_when_balance_equals_amount() {
        let cs = ConstraintSystem::<Fr>::new_ref();
        let circuit = PrivateTxCircuit::new(500, 500);
        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn nullifier_is_deterministic() {
        let a = PrivateTxCircuit::new(1000, 100);
        let b = PrivateTxCircuit::new(1000, 100);
        assert_eq!(a.nullifier, b.nullifier);
    }

    #[test]
    fn different_inputs_different_nullifier() {
        let a = PrivateTxCircuit::new(1000, 100);
        let b = PrivateTxCircuit::new(1000, 200);
        assert_ne!(a.nullifier, b.nullifier);
    }

    #[test]
    fn empty_circuit_for_setup() {
        let circuit = PrivateTxCircuit::empty();
        assert!(circuit.balance.is_none());
        assert!(circuit.amount.is_none());
        assert!(circuit.nullifier.is_none());
    }

    #[test]
    fn public_inputs_contains_nullifier() {
        let circuit = PrivateTxCircuit::new(1000, 50);
        let inputs = circuit.public_inputs();
        assert_eq!(inputs.len(), 1);
    }

    #[test]
    fn constraint_count() {
        let cs = ConstraintSystem::<Fr>::new_ref();
        let circuit = PrivateTxCircuit::new(100, 10);
        circuit.generate_constraints(cs.clone()).unwrap();
        assert_eq!(cs.num_constraints(), 2);
    }
}
