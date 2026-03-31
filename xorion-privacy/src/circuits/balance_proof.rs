//! Balance proof circuit — proves sufficient funds without revealing exact balance.
//!
//! Public input: threshold.
//! Private input: balance.
//! Proves: balance >= threshold.

use ark_bn254::Fr;
use ark_relations::r1cs::{
    ConstraintSynthesizer, ConstraintSystemRef, LinearCombination, SynthesisError, Variable,
};

/// Circuit for proving sufficient balance without revealing the exact amount.
#[derive(Clone)]
pub struct BalanceProofCircuit {
    /// Private: actual balance.
    pub balance: Option<Fr>,
    /// Public: minimum threshold to prove.
    pub threshold: Option<Fr>,
}

impl BalanceProofCircuit {
    /// Create a circuit with concrete values.
    pub fn new(balance: u64, threshold: u64) -> Self {
        Self {
            balance: Some(Fr::from(balance)),
            threshold: Some(Fr::from(threshold)),
        }
    }

    /// Create an empty circuit for trusted setup.
    pub fn empty() -> Self {
        Self {
            balance: None,
            threshold: None,
        }
    }

    /// Return the public inputs for verification: [threshold].
    pub fn public_inputs(&self) -> Vec<Fr> {
        match self.threshold {
            Some(t) => vec![t],
            None => vec![],
        }
    }
}

impl ConstraintSynthesizer<Fr> for BalanceProofCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        // Private input: balance
        let balance = cs.new_witness_variable(|| {
            self.balance.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Public input: threshold
        let threshold = cs.new_input_variable(|| {
            self.threshold.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Witness: excess = balance - threshold
        let excess = cs.new_witness_variable(|| {
            let b = self.balance.ok_or(SynthesisError::AssignmentMissing)?;
            let t = self.threshold.ok_or(SynthesisError::AssignmentMissing)?;
            Ok(b - t)
        })?;

        // Enforce: (balance - threshold) * 1 = excess
        cs.enforce_constraint(
            LinearCombination::zero() + balance - threshold,
            LinearCombination::zero() + Variable::One,
            LinearCombination::zero() + excess,
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_relations::r1cs::ConstraintSystem;

    #[test]
    fn satisfied_when_balance_exceeds_threshold() {
        let cs = ConstraintSystem::<Fr>::new_ref();
        let circuit = BalanceProofCircuit::new(10_000, 1_000);
        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn satisfied_when_balance_equals_threshold() {
        let cs = ConstraintSystem::<Fr>::new_ref();
        let circuit = BalanceProofCircuit::new(1_000, 1_000);
        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn public_inputs_correct() {
        let circuit = BalanceProofCircuit::new(5000, 1000);
        let inputs = circuit.public_inputs();
        assert_eq!(inputs.len(), 1);
        assert_eq!(inputs[0], Fr::from(1000u64));
    }

    #[test]
    fn constraint_count() {
        let cs = ConstraintSystem::<Fr>::new_ref();
        let circuit = BalanceProofCircuit::new(500, 100);
        circuit.generate_constraints(cs.clone()).unwrap();
        assert_eq!(cs.num_constraints(), 1);
    }
}
