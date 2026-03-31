use xorion_sdk::kernel::{WalletClient, CHAIN_ETHEREUM, CHAIN_SOLANA};

/// Demonstrates the kernel-style wallet API.
///
/// To run this demo:
/// 1. Start the scheme daemon:  cargo run -p xorion-scheme
/// 2. In another terminal:      cargo run --example kernel_demo
fn main() {
    println!("=== Xorion Kernel Integration Demo ===\n");

    // Connect to the wallet scheme daemon
    let mut client = match WalletClient::connect() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Cannot connect to wallet daemon: {e}");
            eprintln!("Start the daemon first:  cargo run -p xorion-scheme");
            std::process::exit(1);
        }
    };
    println!("[+] Connected to wallet daemon");

    // Check status before init
    match client.wallet_status() {
        Ok(status) => println!("[status] {status}"),
        Err(e) => eprintln!("[error]  {e}"),
    }

    // Initialize wallet (syscall: init_wallet)
    let mnemonic =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    match client.wallet_init(mnemonic) {
        Ok(()) => println!("[+] Wallet initialized"),
        Err(e) => {
            eprintln!("[-] Init failed: {e}");
            return;
        }
    }

    // Get Ethereum address (syscall: wallet_eth_address)
    let mut eth_buf = [0u8; 128];
    match client.wallet_eth_address(&mut eth_buf) {
        Ok(len) => {
            let addr = std::str::from_utf8(&eth_buf[..len]).unwrap();
            println!("[ETH]    address = {addr}");
        }
        Err(e) => eprintln!("[error]  ETH address: {e}"),
    }

    // Get Solana address (syscall: wallet_solana_address)
    let mut sol_buf = [0u8; 128];
    match client.wallet_solana_address(&mut sol_buf) {
        Ok(len) => {
            let addr = std::str::from_utf8(&sol_buf[..len]).unwrap();
            println!("[SOL]    address = {addr}");
        }
        Err(e) => eprintln!("[error]  SOL address: {e}"),
    }

    // Sign a transaction (syscall: wallet_sign_transaction)
    let tx_data = b"sample transaction payload";
    let mut sig_buf = [0u8; 512];
    match client.wallet_sign_transaction(CHAIN_ETHEREUM, tx_data, &mut sig_buf) {
        Ok(len) => {
            let sig = std::str::from_utf8(&sig_buf[..len]).unwrap();
            println!("[ETH]    signed  = {sig}");
        }
        Err(e) => eprintln!("[error]  sign: {e}"),
    }

    // Get balance (syscall: wallet_get_balance)
    let mut bal_buf = [0u8; 256];
    match client.wallet_get_balance(
        CHAIN_SOLANA,
        "11111111111111111111111111111111",
        &mut bal_buf,
    ) {
        Ok(len) => {
            let balance = std::str::from_utf8(&bal_buf[..len]).unwrap();
            println!("[SOL]    balance = {balance}");
        }
        Err(e) => eprintln!("[error]  balance: {e}"),
    }

    // Final status
    match client.wallet_status() {
        Ok(status) => println!("[status] {status}"),
        Err(e) => eprintln!("[error]  {e}"),
    }

    println!("\nDone.");
}
