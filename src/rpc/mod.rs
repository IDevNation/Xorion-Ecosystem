#![cfg(not(target_arch = "wasm32"))]
pub mod ethereum;
pub mod solana;
pub mod types;

use async_trait::async_trait;
pub use types::{Chain, TransactionReceipt};

use crate::error::Result;

/// Trait that every chain-specific RPC provider must implement.
#[async_trait]
pub trait ChainProvider: Send + Sync {
    /// Which chain this provider targets.
    fn chain(&self) -> Chain;

    /// Return the RPC endpoint URL.
    fn endpoint(&self) -> &str;

    /// Get the latest block number (or slot for Solana).
    async fn get_block_number(&self) -> Result<u64>;

    /// Get the native-token balance (in the smallest unit) for `address`.
    async fn get_balance(&self, address: &str) -> Result<String>;

    /// Broadcast a signed, serialized transaction.
    /// Returns the transaction hash on success.
    async fn send_raw_transaction(&self, signed_tx_hex: &str) -> Result<TransactionReceipt>;

    /// Fetch the current gas price (Ethereum) or recent blockhash (Solana).
    async fn get_fee_estimate(&self) -> Result<String>;
}
