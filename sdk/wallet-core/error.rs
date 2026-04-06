use thiserror::Error;

/// Errors that can occur in the Xorion wallet SDK.
#[derive(Debug, Error)]
pub enum WalletError {
    #[error("invalid mnemonic phrase: {0}")]
    InvalidMnemonic(String),

    #[error("invalid seed: {0}")]
    InvalidSeed(String),

    #[error("key derivation failed: {0}")]
    DerivationError(String),

    #[error("cryptographic operation failed: {0}")]
    CryptoError(String),

    #[error("RPC request failed: {0}")]
    RpcError(String),

    #[error("transaction error: {0}")]
    TransactionError(String),

    #[error("invalid response from node: {0}")]
    InvalidResponse(String),

    #[error("network error: {0}")]
    NetworkError(String),

    #[error("ABI encoding error: {0}")]
    AbiError(String),

    #[error("contract error: {0}")]
    ContractError(String),
}

impl From<reqwest::Error> for WalletError {
    fn from(e: reqwest::Error) -> Self {
        WalletError::NetworkError(e.to_string())
    }
}

impl From<serde_json::Error> for WalletError {
    fn from(e: serde_json::Error) -> Self {
        WalletError::InvalidResponse(e.to_string())
    }
}

impl From<ethabi::Error> for WalletError {
    fn from(e: ethabi::Error) -> Self {
        WalletError::AbiError(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, WalletError>;
