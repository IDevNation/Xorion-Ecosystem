//! Age verification circuit — proves age >= minimum without revealing birth year.
//!
//! Public inputs: current_year, minimum_age.
//! Private input: birth_year.
//! Proves: current_year - birth_year >= minimum_age.

use ark_bn254::Fr;
use ark_relations::r1cs::{
    ConstraintSynthesizer, ConstraintSystemRef, LinearCombination, SynthesisError, Variable,
};

/// Circuit for age verification without revealing date of birth.
#[derive(Clone)]
pub struct AgeVerificationCircuit {
    /// Private: user's birth year.
    pub birth_year: Option<Fr>,
    /// Public: current year.
    pub current_year: Option<Fr>,
    /// Public: minimum required age (e.g. 18).
    pub minimum_age: Option<Fr>,
}

impl AgeVerificationCircuit {
    /// Create a circuit with concrete values.
    pub fn new(birth_year: u32, current_year: u32, minimum_age: u32) -> Self {
        Self {
            birth_year: Some(Fr::from(birth_year as u64)),
            current_year: Some(Fr::from(current_year as u64)),
            minimum_age: Some(Fr::from(minimum_age as u64)),
        }
    }

    /// Create an empty circuit for trusted setup.
    pub fn empty() -> Self {
        Self {
            birth_year: None,
            current_year: None,
            minimum_age: None,
        }
    }

    /// Return the public inputs for verification: [current_year, minimum_age].
    pub fn public_inputs(&self) -> Vec<Fr> {
        match (self.current_year, self.minimum_age) {
            (Some(cy), Some(ma)) => vec![cy, ma],
            _ => vec![],
        }
    }
}

impl ConstraintSynthesizer<Fr> for AgeVerificationCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        // Private input: birth year
        let birth_year = cs.new_witness_variable(|| {
            self.birth_year.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Public inputs
        let current_year = cs.new_input_variable(|| {
            self.current_year.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let minimum_age = cs.new_input_variable(|| {
            self.minimum_age.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Witness: age = current_year - birth_year
        let age = cs.new_witness_variable(|| {
            let cy = self.current_year.ok_or(SynthesisError::AssignmentMissing)?;
            let by = self.birth_year.ok_or(SynthesisError::AssignmentMissing)?;
            Ok(cy - by)
        })?;

        // Enforce: (current_year - birth_year) * 1 = age
        cs.enforce_constraint(
            LinearCombination::zero() + current_year - birth_year,
            LinearCombination::zero() + Variable::One,
            LinearCombination::zero() + age,
        )?;

        // Witness: excess = age - minimum_age (must be >= 0)
        let excess = cs.new_witness_variable(|| {
            let cy = self.current_year.ok_or(SynthesisError::AssignmentMissing)?;
            let by = self.birth_year.ok_or(SynthesisError::AssignmentMissing)?;
            let ma = self.minimum_age.ok_or(SynthesisError::AssignmentMissing)?;
            Ok((cy - by) - ma)
        })?;

        // Enforce: (age - minimum_age) * 1 = excess
        cs.enforce_constraint(
            LinearCombination::zero() + age - minimum_age,
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
    fn satisfied_when_old_enough() {
        let cs = ConstraintSystem::<Fr>::new_ref();
        let circuit = AgeVerificationCircuit::new(1990, 2026, 18);
        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn satisfied_when_exactly_minimum_age() {
        let cs = ConstraintSystem::<Fr>::new_ref();
        let circuit = AgeVerificationCircuit::new(2008, 2026, 18);
        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn public_inputs_correct() {
        let circuit = AgeVerificationCircuit::new(1990, 2026, 18);
        let inputs = circuit.public_inputs();
        assert_eq!(inputs.len(), 2);
        assert_eq!(inputs[0], Fr::from(2026u64));
        assert_eq!(inputs[1], Fr::from(18u64));
    }

    #[test]
    fn constraint_count() {
        let cs = ConstraintSystem::<Fr>::new_ref();
        let circuit = AgeVerificationCircuit::new(2000, 2026, 18);
        circuit.generate_constraints(cs.clone()).unwrap();
        assert_eq!(cs.num_constraints(), 2);
    }
}
