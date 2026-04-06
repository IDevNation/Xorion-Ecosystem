//! DAO treasury management.
//!
//! Tracks deposited funds and allows withdrawals only through
//! executed governance proposals.

use serde::{Deserialize, Serialize};

use crate::{GovernanceError, Result};

/// A treasury transaction record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryTx {
    pub tx_type: TreasuryTxType,
    pub amount: u64,
    pub description: String,
    pub proposal_id: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TreasuryTxType {
    Deposit,
    Withdrawal,
}

/// DAO treasury with balance tracking and audit log.
pub struct Treasury {
    balance: u64,
    history: Vec<TreasuryTx>,
}

impl Treasury {
    pub fn new() -> Self {
        Self {
            balance: 0,
            history: Vec::new(),
        }
    }

    /// Deposit funds into the treasury.
    pub fn deposit(&mut self, amount: u64, description: &str) {
        self.balance += amount;
        self.history.push(TreasuryTx {
            tx_type: TreasuryTxType::Deposit,
            amount,
            description: description.to_string(),
            proposal_id: None,
        });
    }

    /// Withdraw funds (must be linked to an executed proposal).
    pub fn withdraw(&mut self, amount: u64, proposal_id: u64, description: &str) -> Result<()> {
        if amount > self.balance {
            return Err(GovernanceError::Treasury(format!(
                "insufficient funds: have {}, need {amount}",
                self.balance
            )));
        }

        self.balance -= amount;
        self.history.push(TreasuryTx {
            tx_type: TreasuryTxType::Withdrawal,
            amount,
            description: description.to_string(),
            proposal_id: Some(proposal_id),
        });

        Ok(())
    }

    /// Current treasury balance.
    pub fn balance(&self) -> u64 {
        self.balance
    }

    /// Full transaction history.
    pub fn history(&self) -> &[TreasuryTx] {
        &self.history
    }

    /// Total deposited over all time.
    pub fn total_deposited(&self) -> u64 {
        self.history
            .iter()
            .filter(|tx| tx.tx_type == TreasuryTxType::Deposit)
            .map(|tx| tx.amount)
            .sum()
    }

    /// Total withdrawn over all time.
    pub fn total_withdrawn(&self) -> u64 {
        self.history
            .iter()
            .filter(|tx| tx.tx_type == TreasuryTxType::Withdrawal)
            .map(|tx| tx.amount)
            .sum()
    }
}

impl Default for Treasury {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_increases_balance() {
        let mut t = Treasury::new();
        t.deposit(1000, "initial funding");
        assert_eq!(t.balance(), 1000);
    }

    #[test]
    fn withdraw_decreases_balance() {
        let mut t = Treasury::new();
        t.deposit(1000, "seed");
        t.withdraw(400, 1, "dev grant").unwrap();
        assert_eq!(t.balance(), 600);
    }

    #[test]
    fn withdraw_exceeding_balance_fails() {
        let mut t = Treasury::new();
        t.deposit(100, "seed");
        assert!(t.withdraw(200, 1, "too much").is_err());
    }

    #[test]
    fn history_tracked() {
        let mut t = Treasury::new();
        t.deposit(500, "a");
        t.withdraw(100, 1, "b").unwrap();
        assert_eq!(t.history().len(), 2);
        assert_eq!(t.total_deposited(), 500);
        assert_eq!(t.total_withdrawn(), 100);
    }

    #[test]
    fn withdrawal_linked_to_proposal() {
        let mut t = Treasury::new();
        t.deposit(1000, "seed");
        t.withdraw(200, 42, "grant").unwrap();
        let last = t.history().last().unwrap();
        assert_eq!(last.proposal_id, Some(42));
    }
}
