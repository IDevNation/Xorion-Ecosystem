use serde::{Deserialize, Serialize};

/// Commands that clients send to the wallet scheme daemon.
///
/// Clients communicate by writing JSON to the scheme file descriptor.
/// The daemon processes the command and the client reads back the response.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum WalletRequest {
    /// Initialize wallet from a mnemonic phrase.
    InitWallet {
        mnemonic: String,
    },

    /// Get the Ethereum address.
    EthAddress,

    /// Get the Solana address.
    SolanaAddress,

    /// Sign a transaction.
    SignTransaction {
        /// 0 = Ethereum, 1 = Solana
        chain: u32,
        /// Hex-encoded transaction data.
        tx_data: String,
    },

    /// Get native token balance (requires RPC — delegated to SDK).
    GetBalance {
        /// 0 = Ethereum, 1 = Solana
        chain: u32,
        address: String,
    },

    /// Get the wallet status.
    Status,
}

/// Responses the daemon sends back to clients.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum WalletResponse {
    Ok {
        data: String,
    },
    Error {
        message: String,
    },
}

impl WalletResponse {
    pub fn ok(data: impl Into<String>) -> Self {
        Self::Ok { data: data.into() }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::Error {
            message: message.into(),
        }
    }
}
