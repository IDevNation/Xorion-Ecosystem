//! Xorion WASM dApp Runtime
//!
//! Provides a secure sandboxed runtime for WebAssembly dApps within the
//! Xorion Web3 OS. dApps can be loaded from the local filesystem or IPFS,
//! and interact with the wallet through a host-function API.

pub mod api;
pub mod ipfs_loader;
pub mod runtime;
pub mod sandbox;

pub use api::WalletBridge;
pub use ipfs_loader::IpfsLoader;
pub use runtime::WasmRuntime;
pub use sandbox::{Permission, Sandbox, SandboxError};
