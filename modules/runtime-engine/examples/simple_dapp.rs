//! Example: Xorion WASM dApp Runtime demo.

use xorion_core::{Permission, WasmRuntime};

fn main() -> anyhow::Result<()> {
    println!("=== Xorion WASM Runtime Demo ===\n");

    let mut runtime = WasmRuntime::new()?;
    println!("[+] Runtime created");

    let mnemonic =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    runtime.init_wallet(mnemonic)?;
    println!("[+] Wallet initialized");
    println!("    ETH: {}", runtime.bridge().eth_address());
    println!("    SOL: {}", runtime.bridge().sol_address());

    runtime.sandbox_mut().grant_permission(Permission::SignTransaction);
    runtime.sandbox_mut().grant_permission(Permission::Network);
    println!("[+] Sandbox permissions: {:?}", runtime.sandbox_mut().permissions());

    // Minimal WASM: (module (func (export "_start")))
    let minimal_wasm: &[u8] = &[
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00, 0x01, 0x04, 0x01,
        0x60, 0x00, 0x00, 0x03, 0x02, 0x01, 0x00, 0x07, 0x0a, 0x01, 0x06,
        0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x00, 0x00, 0x0a, 0x04, 0x01,
        0x02, 0x00, 0x0b,
    ];
    runtime.load_from_bytes(minimal_wasm)?;
    println!("[+] WASM module loaded ({} bytes)", minimal_wasm.len());

    runtime.run()?;
    println!("[+] dApp executed successfully");

    println!("\nDone.");
    Ok(())
}
