//! # Xorion SDK
//!
//! Multi-chain wallet SDK for the Xorion Web3 OS. Provides HD wallet
//! creation, blockchain RPC, and smart contract interaction.
//!
//! ## Quick Start
//!
//! ```rust
//! use xorion_sdk::Wallet;
//!
//! let wallet = Wallet::from_mnemonic(
//!     "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
//! ).unwrap();
//! let eth = wallet.derive_eth_address().unwrap();
//! assert!(eth.starts_with("0x"));
//! ```

pub mod contract;
pub mod error;
pub mod kernel;
pub mod rpc;
pub mod wallet;

/// ERC-20 token interface for querying balances, allowances, and transfers.
pub use contract::Erc20;
/// Uniswap V2 pair interface for reading reserves and swap data.
pub use contract::UniswapV2Pair;
/// Uniswap V2 router interface for encoding swap calls.
pub use contract::UniswapV2Router;
/// Top-level error type for wallet and derivation operations.
pub use error::WalletError;
/// Unified wallet client providing a high-level API over the kernel layer.
pub use kernel::WalletClient;
/// Blockchain chain identifier (Ethereum, Solana).
pub use rpc::Chain;
/// Trait for async blockchain RPC providers.
pub use rpc::ChainProvider;
/// Parsed transaction receipt from RPC responses.
pub use rpc::TransactionReceipt;
/// BIP-39/BIP-44 HD wallet with multi-chain address derivation.
pub use wallet::Wallet;
