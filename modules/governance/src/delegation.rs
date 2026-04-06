//! Voting power delegation.
//!
//! Allows token holders to delegate their voting power to another address.
//! A delegate's effective power = own tokens + all delegated tokens.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{GovernanceError, Result};

/// A single delegation record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delegation {
    pub from: String,
    pub to: String,
    pub power: u64,
}

/// Registry tracking all active delegations.
pub struct DelegationRegistry {
    /// from_address -> Delegation
    delegations: HashMap<String, Delegation>,
    /// Token balance per address (base voting power).
    balances: HashMap<String, u64>,
}

impl DelegationRegistry {
    pub fn new() -> Self {
        Self {
            delegations: HashMap::new(),
            balances: HashMap::new(),
        }
    }

    /// Set the token balance (base voting power) for an address.
    pub fn set_balance(&mut self, address: &str, balance: u64) {
        self.balances.insert(address.to_string(), balance);
    }

    /// Get the raw token balance for an address.
    pub fn balance(&self, address: &str) -> u64 {
        self.balances.get(address).copied().unwrap_or(0)
    }

    /// Delegate voting power to another address.
    /// Replaces any existing delegation from this address.
    pub fn delegate(&mut self, from: &str, to: &str) -> Result<()> {
        if from == to {
            return Err(GovernanceError::Delegation(
                "cannot delegate to self".into(),
            ));
        }

        let power = self.balance(from);
        if power == 0 {
            return Err(GovernanceError::InsufficientPower { have: 0, need: 1 });
        }

        self.delegations.insert(
            from.to_string(),
            Delegation {
                from: from.to_string(),
                to: to.to_string(),
                power,
            },
        );

        Ok(())
    }

    /// Remove delegation (reclaim own voting power).
    pub fn undelegate(&mut self, from: &str) -> Result<()> {
        self.delegations
            .remove(from)
            .map(|_| ())
            .ok_or_else(|| GovernanceError::Delegation("no active delegation".into()))
    }

    /// Get the effective voting power for an address:
    /// own balance (if not delegated away) + received delegations.
    pub fn voting_power(&self, address: &str) -> u64 {
        // Own power (only if not delegated away)
        let own = if self.delegations.contains_key(address) {
            0 // delegated away
        } else {
            self.balance(address)
        };

        // Received delegations
        let received: u64 = self
            .delegations
            .values()
            .filter(|d| d.to == address)
            .map(|d| d.power)
            .sum();

        own + received
    }

    /// Get the delegate for an address (if any).
    pub fn delegate_of(&self, address: &str) -> Option<&str> {
        self.delegations.get(address).map(|d| d.to.as_str())
    }

    /// List all delegations to a given address.
    pub fn delegations_to(&self, address: &str) -> Vec<&Delegation> {
        self.delegations
            .values()
            .filter(|d| d.to == address)
            .collect()
    }

    /// Total number of active delegations.
    pub fn delegation_count(&self) -> usize {
        self.delegations.len()
    }
}

impl Default for DelegationRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voting_power_equals_balance() {
        let mut reg = DelegationRegistry::new();
        reg.set_balance("0xA", 1000);
        assert_eq!(reg.voting_power("0xA"), 1000);
    }

    #[test]
    fn delegate_transfers_power() {
        let mut reg = DelegationRegistry::new();
        reg.set_balance("0xA", 500);
        reg.set_balance("0xB", 300);
        reg.delegate("0xA", "0xB").unwrap();
        assert_eq!(reg.voting_power("0xA"), 0); // delegated away
        assert_eq!(reg.voting_power("0xB"), 800); // own 300 + received 500
    }

    #[test]
    fn undelegate_restores_power() {
        let mut reg = DelegationRegistry::new();
        reg.set_balance("0xA", 500);
        reg.delegate("0xA", "0xB").unwrap();
        reg.undelegate("0xA").unwrap();
        assert_eq!(reg.voting_power("0xA"), 500);
        assert_eq!(reg.voting_power("0xB"), 0);
    }

    #[test]
    fn self_delegation_rejected() {
        let mut reg = DelegationRegistry::new();
        reg.set_balance("0xA", 100);
        assert!(reg.delegate("0xA", "0xA").is_err());
    }

    #[test]
    fn zero_balance_delegation_rejected() {
        let mut reg = DelegationRegistry::new();
        assert!(reg.delegate("0xEmpty", "0xB").is_err());
    }

    #[test]
    fn multiple_delegations_to_one() {
        let mut reg = DelegationRegistry::new();
        reg.set_balance("0xA", 100);
        reg.set_balance("0xB", 200);
        reg.set_balance("0xC", 50);
        reg.delegate("0xA", "0xC").unwrap();
        reg.delegate("0xB", "0xC").unwrap();
        assert_eq!(reg.voting_power("0xC"), 350); // own 50 + 100 + 200
        assert_eq!(reg.delegations_to("0xC").len(), 2);
    }

    #[test]
    fn delegate_of_returns_target() {
        let mut reg = DelegationRegistry::new();
        reg.set_balance("0xA", 100);
        reg.delegate("0xA", "0xB").unwrap();
        assert_eq!(reg.delegate_of("0xA"), Some("0xB"));
        assert_eq!(reg.delegate_of("0xB"), None);
    }
}
