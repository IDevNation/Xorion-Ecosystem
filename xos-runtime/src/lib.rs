//! # Xorion Core — WASM dApp Runtime
//!
//! Provides a secure sandboxed runtime for WebAssembly dApps within the
//! Xorion Web3 OS. dApps can be loaded from the local filesystem or IPFS,
//! and interact with the wallet through a host-function API.
//!
//! ## Example
//!
//! ```rust,no_run
//! use xorion_core::WasmRuntime;
//!
//! let mut runtime = WasmRuntime::new().unwrap();
//! runtime.init_wallet("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about").unwrap();
//! ```

pub mod api;
pub mod ipfs_loader;
pub mod runtime;
pub mod sandbox;

/// Bridge between WASM dApps and the Xorion wallet via host functions.
pub use api::WalletBridge;
/// Loader that fetches WASM dApps from IPFS by CID.
pub use ipfs_loader::IpfsLoader;
/// The main WASM execution engine powered by Wasmtime.
pub use runtime::WasmRuntime;
/// Permission model and sandbox configuration for dApp isolation.
pub use sandbox::{Permission, Sandbox, SandboxError};
