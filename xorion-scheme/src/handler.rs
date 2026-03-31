use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use xorion_sdk::Wallet;

use crate::protocol::{WalletRequest, WalletResponse};

/// Per-process wallet state.
struct ProcessWallet {
    wallet: Wallet,
}

/// The core handler that processes wallet requests.
///
/// Each process (identified by an opaque handle ID) can have its own wallet
/// instance, isolated from other processes.
pub struct WalletHandler {
    wallets: Arc<Mutex<HashMap<u64, ProcessWallet>>>,
    next_id: Arc<Mutex<u64>>,
}

impl WalletHandler {
    pub fn new() -> Self {
        Self {
            wallets: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Allocate a new handle (called on open).
    pub fn open(&self) -> u64 {
        let mut id = self.next_id.lock().unwrap();
        let handle = *id;
        *id += 1;
        handle
    }

    /// Release a handle (called on close).
    pub fn close(&self, handle: u64) {
        self.wallets.lock().unwrap().remove(&handle);
    }

    /// Process a single request for the given handle.
    pub fn handle_request(&self, handle: u64, request: &WalletRequest) -> WalletResponse {
        match request {
            WalletRequest::InitWallet { mnemonic } => self.init_wallet(handle, mnemonic),
            WalletRequest::EthAddress => self.eth_address(handle),
            WalletRequest::SolanaAddress => self.solana_address(handle),
            WalletRequest::SignTransaction { chain, tx_data } => {
                self.sign_transaction(handle, *chain, tx_data)
            }
            WalletRequest::GetBalance { chain, address } => {
                self.get_balance(*chain, address)
            }
            WalletRequest::Status => self.status(handle),
        }
    }

    fn init_wallet(&self, handle: u64, mnemonic: &str) -> WalletResponse {
        match Wallet::from_mnemonic(mnemonic) {
            Ok(wallet) => {
                self.wallets
                    .lock()
                    .unwrap()
                    .insert(handle, ProcessWallet { wallet });
                WalletResponse::ok("wallet initialized")
            }
            Err(e) => WalletResponse::error(e.to_string()),
        }
    }

    fn eth_address(&self, handle: u64) -> WalletResponse {
        let wallets = self.wallets.lock().unwrap();
        match wallets.get(&handle) {
            Some(pw) => match pw.wallet.derive_eth_address() {
                Ok(addr) => WalletResponse::ok(addr),
                Err(e) => WalletResponse::error(e.to_string()),
            },
            None => WalletResponse::error("wallet not initialized — send init_wallet first"),
        }
    }

    fn solana_address(&self, handle: u64) -> WalletResponse {
        let wallets = self.wallets.lock().unwrap();
        match wallets.get(&handle) {
            Some(pw) => match pw.wallet.derive_solana_address() {
                Ok(addr) => WalletResponse::ok(addr),
                Err(e) => WalletResponse::error(e.to_string()),
            },
            None => WalletResponse::error("wallet not initialized — send init_wallet first"),
        }
    }

    fn sign_transaction(&self, handle: u64, chain: u32, tx_data: &str) -> WalletResponse {
        let wallets = self.wallets.lock().unwrap();
        match wallets.get(&handle) {
            Some(_pw) => {
                // Phase 4 stub — actual signing will use the secp256k1 key
                // derived from the wallet seed + the chain-specific
                // transaction serialization format.
                let chain_name = match chain {
                    0 => "ethereum",
                    1 => "solana",
                    _ => return WalletResponse::error(format!("unknown chain ID: {chain}")),
                };
                let tx_hash = {
                    use sha3::{Digest, Keccak256};
                    let mut hasher = Keccak256::new();
                    hasher.update(tx_data.as_bytes());
                    hex::encode(hasher.finalize())
                };
                WalletResponse::ok(format!(
                    "{{\"chain\":\"{chain_name}\",\"tx_hash\":\"0x{tx_hash}\",\"signed\":true}}"
                ))
            }
            None => WalletResponse::error("wallet not initialized — send init_wallet first"),
        }
    }

    fn get_balance(&self, chain: u32, address: &str) -> WalletResponse {
        // Balance queries go through RPC (Phase 2). In the scheme daemon this
        // would spawn an async task. For now we return a placeholder showing
        // the daemon understood the request.
        let chain_name = match chain {
            0 => "ethereum",
            1 => "solana",
            _ => return WalletResponse::error(format!("unknown chain ID: {chain}")),
        };
        WalletResponse::ok(format!(
            "{{\"chain\":\"{chain_name}\",\"address\":\"{address}\",\"balance\":\"query_pending\"}}"
        ))
    }

    fn status(&self, handle: u64) -> WalletResponse {
        let wallets = self.wallets.lock().unwrap();
        let initialized = wallets.contains_key(&handle);
        let total = wallets.len();
        WalletResponse::ok(format!(
            "{{\"initialized\":{initialized},\"active_wallets\":{total}}}"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MNEMONIC: &str =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    #[test]
    fn open_and_close() {
        let handler = WalletHandler::new();
        let h1 = handler.open();
        let h2 = handler.open();
        assert_ne!(h1, h2);
        handler.close(h1);
        handler.close(h2);
    }

    #[test]
    fn init_and_get_eth_address() {
        let handler = WalletHandler::new();
        let h = handler.open();

        let req = WalletRequest::InitWallet {
            mnemonic: TEST_MNEMONIC.to_string(),
        };
        let resp = handler.handle_request(h, &req);
        assert!(matches!(resp, WalletResponse::Ok { .. }));

        let resp = handler.handle_request(h, &WalletRequest::EthAddress);
        match resp {
            WalletResponse::Ok { data } => {
                assert!(data.starts_with("0x"));
                assert_eq!(data.len(), 42);
            }
            WalletResponse::Error { message } => panic!("unexpected error: {message}"),
        }
    }

    #[test]
    fn init_and_get_solana_address() {
        let handler = WalletHandler::new();
        let h = handler.open();

        handler.handle_request(
            h,
            &WalletRequest::InitWallet {
                mnemonic: TEST_MNEMONIC.to_string(),
            },
        );

        let resp = handler.handle_request(h, &WalletRequest::SolanaAddress);
        match resp {
            WalletResponse::Ok { data } => assert!(!data.is_empty()),
            WalletResponse::Error { message } => panic!("unexpected error: {message}"),
        }
    }

    #[test]
    fn uninitialized_wallet_returns_error() {
        let handler = WalletHandler::new();
        let h = handler.open();
        let resp = handler.handle_request(h, &WalletRequest::EthAddress);
        assert!(matches!(resp, WalletResponse::Error { .. }));
    }

    #[test]
    fn sign_transaction() {
        let handler = WalletHandler::new();
        let h = handler.open();
        handler.handle_request(
            h,
            &WalletRequest::InitWallet {
                mnemonic: TEST_MNEMONIC.to_string(),
            },
        );

        let resp = handler.handle_request(
            h,
            &WalletRequest::SignTransaction {
                chain: 0,
                tx_data: "deadbeef".to_string(),
            },
        );
        match resp {
            WalletResponse::Ok { data } => {
                assert!(data.contains("ethereum"));
                assert!(data.contains("signed"));
            }
            WalletResponse::Error { message } => panic!("unexpected error: {message}"),
        }
    }

    #[test]
    fn invalid_chain_returns_error() {
        let handler = WalletHandler::new();
        let h = handler.open();
        handler.handle_request(
            h,
            &WalletRequest::InitWallet {
                mnemonic: TEST_MNEMONIC.to_string(),
            },
        );

        let resp = handler.handle_request(
            h,
            &WalletRequest::SignTransaction {
                chain: 99,
                tx_data: "aabb".to_string(),
            },
        );
        assert!(matches!(resp, WalletResponse::Error { .. }));
    }

    #[test]
    fn get_balance_responds() {
        let handler = WalletHandler::new();
        let h = handler.open();
        let resp = handler.handle_request(
            h,
            &WalletRequest::GetBalance {
                chain: 0,
                address: "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
            },
        );
        match resp {
            WalletResponse::Ok { data } => assert!(data.contains("ethereum")),
            WalletResponse::Error { message } => panic!("unexpected error: {message}"),
        }
    }

    #[test]
    fn status_before_and_after_init() {
        let handler = WalletHandler::new();
        let h = handler.open();

        let resp = handler.handle_request(h, &WalletRequest::Status);
        match &resp {
            WalletResponse::Ok { data } => assert!(data.contains("\"initialized\":false")),
            _ => panic!("expected ok"),
        }

        handler.handle_request(
            h,
            &WalletRequest::InitWallet {
                mnemonic: TEST_MNEMONIC.to_string(),
            },
        );

        let resp = handler.handle_request(h, &WalletRequest::Status);
        match &resp {
            WalletResponse::Ok { data } => assert!(data.contains("\"initialized\":true")),
            _ => panic!("expected ok"),
        }
    }

    #[test]
    fn invalid_mnemonic_returns_error() {
        let handler = WalletHandler::new();
        let h = handler.open();
        let resp = handler.handle_request(
            h,
            &WalletRequest::InitWallet {
                mnemonic: "not a valid mnemonic phrase at all".to_string(),
            },
        );
        assert!(matches!(resp, WalletResponse::Error { .. }));
    }

    #[test]
    fn process_isolation() {
        let handler = WalletHandler::new();
        let h1 = handler.open();
        let h2 = handler.open();

        // Init wallet on h1 only
        handler.handle_request(
            h1,
            &WalletRequest::InitWallet {
                mnemonic: TEST_MNEMONIC.to_string(),
            },
        );

        // h1 should work
        let resp = handler.handle_request(h1, &WalletRequest::EthAddress);
        assert!(matches!(resp, WalletResponse::Ok { .. }));

        // h2 should fail — separate process, no wallet
        let resp = handler.handle_request(h2, &WalletRequest::EthAddress);
        assert!(matches!(resp, WalletResponse::Error { .. }));
    }
}
