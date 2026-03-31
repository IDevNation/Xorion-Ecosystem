//! Wallet bridge — host functions exposed to WASM dApps.
//!
//! WASM guest code can only exchange numeric types (i32/i64/f32/f64) with
//! the host. Complex data (addresses, signatures) is passed through shared
//! WASM linear memory.

use std::sync::{Arc, Mutex};
use tracing::debug;
use wasmtime::{Caller, Linker};
use xorion_wallet_sdk::Wallet;

/// Shared state passed to host functions via the wasmtime Store.
#[derive(Clone)]
pub struct BridgeState {
    inner: Arc<Mutex<WalletState>>,
}

struct WalletState {
    eth_address: String,
    sol_address: String,
    initialized: bool,
}

/// Bridge between WASM dApps and the Xorion wallet.
pub struct WalletBridge {
    state: BridgeState,
}

impl WalletBridge {
    /// Create a new bridge (no wallet loaded yet).
    pub fn new() -> Self {
        Self {
            state: BridgeState {
                inner: Arc::new(Mutex::new(WalletState {
                    eth_address: String::new(),
                    sol_address: String::new(),
                    initialized: false,
                })),
            },
        }
    }

    /// Initialize the bridge with a wallet derived from a mnemonic.
    pub fn init_wallet(&self, mnemonic: &str) -> Result<(), String> {
        let wallet =
            Wallet::from_mnemonic(mnemonic).map_err(|e| format!("wallet init failed: {e}"))?;

        let eth = wallet
            .derive_eth_address()
            .map_err(|e| format!("ETH derivation failed: {e}"))?;
        let sol = wallet
            .derive_solana_address()
            .map_err(|e| format!("SOL derivation failed: {e}"))?;

        let mut state = self.state.inner.lock().unwrap();
        state.eth_address = eth;
        state.sol_address = sol;
        state.initialized = true;
        Ok(())
    }

    /// Whether a wallet has been loaded.
    pub fn is_initialized(&self) -> bool {
        self.state.inner.lock().unwrap().initialized
    }

    /// Read the cached ETH address.
    pub fn eth_address(&self) -> String {
        self.state.inner.lock().unwrap().eth_address.clone()
    }

    /// Read the cached SOL address.
    pub fn sol_address(&self) -> String {
        self.state.inner.lock().unwrap().sol_address.clone()
    }

    /// Return the bridge state for embedding in a wasmtime `Store`.
    pub fn state(&self) -> BridgeState {
        self.state.clone()
    }

    /// Register all host functions into a wasmtime `Linker`.
    pub fn register_host_functions(
        linker: &mut Linker<BridgeState>,
    ) -> anyhow::Result<()> {
        // wallet_get_chain_address(chain: i32) -> i32
        //   Returns address length, or negative on error.
        linker.func_wrap(
            "env",
            "wallet_get_chain_address",
            |caller: Caller<'_, BridgeState>, chain: i32| -> i32 {
                let state = caller.data().inner.lock().unwrap();
                if !state.initialized {
                    debug!("wallet_get_chain_address: not initialized");
                    return -1;
                }
                let addr = match chain {
                    0 => &state.eth_address,
                    1 => &state.sol_address,
                    _ => return -2,
                };
                debug!("wallet_get_chain_address(chain={chain}) -> {} bytes", addr.len());
                addr.len() as i32
            },
        )?;

        // wallet_sign_hash(chain: i32, hash_hi: i64, hash_lo: i64) -> i32
        //   Stub: returns 0 on success, negative on error.
        linker.func_wrap(
            "env",
            "wallet_sign_hash",
            |caller: Caller<'_, BridgeState>, chain: i32, _hash_hi: i64, _hash_lo: i64| -> i32 {
                let state = caller.data().inner.lock().unwrap();
                if !state.initialized {
                    return -1;
                }
                if chain != 0 && chain != 1 {
                    return -2;
                }
                debug!("wallet_sign_hash(chain={chain}) -> stub OK");
                0
            },
        )?;

        // wallet_status() -> i32
        //   Returns 1 if initialized, 0 otherwise.
        linker.func_wrap(
            "env",
            "wallet_status",
            |caller: Caller<'_, BridgeState>| -> i32 {
                let state = caller.data().inner.lock().unwrap();
                i32::from(state.initialized)
            },
        )?;

        Ok(())
    }
}

impl Default for WalletBridge {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasmtime::Engine;

    const TEST_MNEMONIC: &str =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    #[test]
    fn bridge_starts_uninitialized() {
        let bridge = WalletBridge::new();
        assert!(!bridge.is_initialized());
        assert!(bridge.eth_address().is_empty());
    }

    #[test]
    fn bridge_init_wallet() {
        let bridge = WalletBridge::new();
        bridge.init_wallet(TEST_MNEMONIC).unwrap();
        assert!(bridge.is_initialized());
        assert!(bridge.eth_address().starts_with("0x"));
        assert!(!bridge.sol_address().is_empty());
    }

    #[test]
    fn bridge_invalid_mnemonic() {
        let bridge = WalletBridge::new();
        let result = bridge.init_wallet("bad mnemonic");
        assert!(result.is_err());
        assert!(!bridge.is_initialized());
    }

    #[test]
    fn host_functions_register() {
        let engine = Engine::default();
        let mut linker = Linker::new(&engine);
        WalletBridge::register_host_functions(&mut linker).unwrap();
    }
}
