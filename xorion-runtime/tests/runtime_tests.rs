use std::time::Duration;
use xorion_runtime::{IpfsLoader, Permission, Sandbox, SandboxError, WalletBridge, WasmRuntime};

const TEST_MNEMONIC: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

// Minimal valid WASM: (module (func (export "_start")))
const MINIMAL_WASM: &[u8] = &[
    0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x04, 0x01, 0x60,
    0x00, 0x00, 0x03, 0x02, 0x01, 0x00, 0x07, 0x0a, 0x01, 0x06, 0x5f, 0x73,
    0x74, 0x61, 0x72, 0x74, 0x00, 0x00, 0x0a, 0x04, 0x01, 0x02, 0x00, 0x0b,
];

// ── Sandbox tests ──────────────────────────────────────────

#[test]
fn sandbox_default_permissions() {
    let sb = Sandbox::new();
    assert!(sb.has_permission(&Permission::ReadWallet));
    assert!(sb.has_permission(&Permission::ReadBalance));
    assert!(!sb.has_permission(&Permission::SignTransaction));
}

#[test]
fn sandbox_grant_revoke() {
    let mut sb = Sandbox::new();
    sb.grant_permission(Permission::Network);
    assert!(sb.has_permission(&Permission::Network));
    sb.revoke_permission(&Permission::Network);
    assert!(!sb.has_permission(&Permission::Network));
}

#[test]
fn sandbox_no_instance_error() {
    let sb = Sandbox::new();
    let engine = wasmtime::Engine::default();
    let mut store = wasmtime::Store::new(&engine, ());
    let err = sb.run(&mut store).unwrap_err();
    assert!(matches!(err, SandboxError::NoInstance));
}

#[test]
fn sandbox_custom_limits() {
    let sb = Sandbox::new()
        .with_memory_limit(16 * 1024 * 1024)
        .with_time_limit(Duration::from_secs(5));
    assert_eq!(sb.memory_limit(), 16 * 1024 * 1024);
    assert_eq!(sb.time_limit(), Duration::from_secs(5));
}

// ── WalletBridge tests ─────────────────────────────────────

#[test]
fn bridge_uninitialized() {
    let bridge = WalletBridge::new();
    assert!(!bridge.is_initialized());
    assert!(bridge.eth_address().is_empty());
    assert!(bridge.sol_address().is_empty());
}

#[test]
fn bridge_init_valid_mnemonic() {
    let bridge = WalletBridge::new();
    bridge.init_wallet(TEST_MNEMONIC).unwrap();
    assert!(bridge.is_initialized());
    assert!(bridge.eth_address().starts_with("0x"));
    assert_eq!(bridge.eth_address().len(), 42);
    assert!(!bridge.sol_address().is_empty());
}

#[test]
fn bridge_init_invalid_mnemonic() {
    let bridge = WalletBridge::new();
    assert!(bridge.init_wallet("not valid words").is_err());
    assert!(!bridge.is_initialized());
}

#[test]
fn bridge_registers_host_functions() {
    let engine = wasmtime::Engine::default();
    let mut linker = wasmtime::Linker::new(&engine);
    WalletBridge::register_host_functions(&mut linker).unwrap();
}

// ── WasmRuntime tests ──────────────────────────────────────

#[test]
fn runtime_creates_successfully() {
    assert!(WasmRuntime::new().is_ok());
}

#[test]
fn runtime_init_wallet() {
    let rt = WasmRuntime::new().unwrap();
    rt.init_wallet(TEST_MNEMONIC).unwrap();
    assert!(rt.bridge().eth_address().starts_with("0x"));
}

#[test]
fn runtime_load_and_run_minimal_wasm() {
    let mut rt = WasmRuntime::new().unwrap();
    rt.load_from_bytes(MINIMAL_WASM).unwrap();
    rt.run().unwrap();
}

#[test]
fn runtime_load_invalid_wasm() {
    let mut rt = WasmRuntime::new().unwrap();
    assert!(rt.load_from_bytes(b"garbage data").is_err());
}

#[test]
fn runtime_sandbox_permissions() {
    let mut rt = WasmRuntime::new().unwrap();
    assert!(!rt.sandbox_mut().has_permission(&Permission::SignTransaction));
    rt.sandbox_mut().grant_permission(Permission::SignTransaction);
    assert!(rt.sandbox_mut().has_permission(&Permission::SignTransaction));
}

#[test]
fn runtime_run_without_load_fails() {
    let mut rt = WasmRuntime::new().unwrap();
    assert!(rt.run().is_err());
}

// ── IpfsLoader tests ───────────────────────────────────────

#[test]
fn ipfs_loader_defaults() {
    let _loader = IpfsLoader::new();
}

#[test]
fn ipfs_loader_custom_gateway() {
    let loader = IpfsLoader::new().with_gateway("https://gateway.pinata.cloud/ipfs");
    let _ = loader;
}
