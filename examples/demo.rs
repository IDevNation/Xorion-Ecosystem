use xorion_sdk::Wallet;

fn main() {
    // Well-known BIP-39 test mnemonic — NEVER use this for real funds.
    let mnemonic =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    println!("Creating wallet from mnemonic...\n");

    let wallet = Wallet::from_mnemonic(mnemonic).expect("valid mnemonic");

    // Display both chain addresses
    println!("{wallet}");

    // Access individual addresses
    match wallet.derive_eth_address() {
        Ok(addr) => println!("ETH address: {addr}"),
        Err(e) => eprintln!("ETH error: {e}"),
    }

    match wallet.derive_solana_address() {
        Ok(addr) => println!("SOL address: {addr}"),
        Err(e) => eprintln!("SOL error: {e}"),
    }
}
