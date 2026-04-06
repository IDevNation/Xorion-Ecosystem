//! Core WASM runtime for loading and executing dApps.

use anyhow::{Context, Result};
use tracing::info;
use wasmtime::{Engine, Linker, Module, Store};

use crate::api::{BridgeState, WalletBridge};
use crate::sandbox::Sandbox;

/// Main WASM runtime for executing Xorion dApps.
pub struct WasmRuntime {
    engine: Engine,
    store: Store<BridgeState>,
    linker: Linker<BridgeState>,
    sandbox: Sandbox,
    bridge: WalletBridge,
}

impl WasmRuntime {
    /// Create a new runtime with default settings.
    pub fn new() -> Result<Self> {
        info!("Initializing Xorion WASM runtime");
        let engine = Engine::default();
        let bridge = WalletBridge::new();
        let store = Store::new(&engine, bridge.state());

        let mut linker = Linker::new(&engine);
        WalletBridge::register_host_functions(&mut linker)?;

        Ok(Self {
            engine,
            store,
            linker,
            sandbox: Sandbox::new(),
            bridge,
        })
    }

    /// Create a runtime with a custom sandbox configuration.
    pub fn with_sandbox(sandbox: Sandbox) -> Result<Self> {
        let engine = Engine::default();
        let bridge = WalletBridge::new();
        let store = Store::new(&engine, bridge.state());
        let mut linker = Linker::new(&engine);
        WalletBridge::register_host_functions(&mut linker)?;

        Ok(Self {
            engine,
            store,
            linker,
            sandbox,
            bridge,
        })
    }

    /// Initialize the wallet bridge from a mnemonic.
    pub fn init_wallet(&self, mnemonic: &str) -> Result<()> {
        self.bridge
            .init_wallet(mnemonic)
            .map_err(|e| anyhow::anyhow!(e))
    }

    /// Access the sandbox (e.g. to grant permissions).
    pub fn sandbox_mut(&mut self) -> &mut Sandbox {
        &mut self.sandbox
    }

    /// Access the wallet bridge.
    pub fn bridge(&self) -> &WalletBridge {
        &self.bridge
    }

    /// Load a WASM module from a file path.
    pub fn load_from_file(&mut self, path: &str) -> Result<()> {
        let wasm_bytes = std::fs::read(path)
            .with_context(|| format!("failed to read WASM file: {path}"))?;
        self.load_from_bytes(&wasm_bytes)
    }

    /// Load a WASM module from raw bytes.
    pub fn load_from_bytes(&mut self, wasm_bytes: &[u8]) -> Result<()> {
        info!("Compiling WASM module ({} bytes)", wasm_bytes.len());

        let module = Module::new(&self.engine, wasm_bytes)
            .context("failed to compile WASM module")?;

        let instance = self
            .linker
            .instantiate(&mut self.store, &module)
            .context("failed to instantiate WASM module")?;

        self.sandbox.set_instance(instance);
        info!("WASM module loaded successfully");
        Ok(())
    }

    /// Execute the loaded dApp (calls `_start`).
    pub fn run(&mut self) -> Result<()> {
        info!("Executing dApp...");
        self.sandbox
            .run(&mut self.store)
            .map_err(|e| anyhow::anyhow!(e))?;
        info!("dApp execution completed");
        Ok(())
    }
}

impl Default for WasmRuntime {
    fn default() -> Self {
        Self::new().expect("failed to create default WasmRuntime")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sandbox::Permission;

    #[test]
    fn runtime_creation() {
        let runtime = WasmRuntime::new();
        assert!(runtime.is_ok());
    }

    #[test]
    fn runtime_with_wallet() {
        let runtime = WasmRuntime::new().unwrap();
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        runtime.init_wallet(mnemonic).unwrap();
        assert!(runtime.bridge().is_initialized());
        assert!(runtime.bridge().eth_address().starts_with("0x"));
    }

    #[test]
    fn runtime_sandbox_access() {
        let mut runtime = WasmRuntime::new().unwrap();
        assert!(!runtime.sandbox_mut().has_permission(&Permission::SignTransaction));
        runtime.sandbox_mut().grant_permission(Permission::SignTransaction);
        assert!(runtime.sandbox_mut().has_permission(&Permission::SignTransaction));
    }

    // Minimal valid WASM: (module (func (export "_start")))
    const MINIMAL_WASM: &[u8] = &[
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
        0x01, 0x04, 0x01, 0x60, 0x00, 0x00,
        0x03, 0x02, 0x01, 0x00,
        0x07, 0x0a, 0x01, 0x06, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x00, 0x00,
        0x0a, 0x04, 0x01, 0x02, 0x00, 0x0b,
    ];

    #[test]
    fn load_and_run_minimal_wasm() {
        let mut runtime = WasmRuntime::new().unwrap();
        runtime.load_from_bytes(MINIMAL_WASM).unwrap();
        runtime.run().unwrap();
    }

    #[test]
    fn load_invalid_wasm_fails() {
        let mut runtime = WasmRuntime::new().unwrap();
        let result = runtime.load_from_bytes(b"not wasm at all");
        assert!(result.is_err());
    }
}
