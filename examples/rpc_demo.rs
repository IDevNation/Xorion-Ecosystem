use xorion_sdk::rpc::ethereum::EthereumProvider;
use xorion_sdk::rpc::solana::SolanaProvider;
use xorion_sdk::ChainProvider;

#[tokio::main]
async fn main() {
    println!("=== Xorion RPC Demo ===\n");

    // ── Ethereum (public RPC) ───────────────────────────────────
    let eth = EthereumProvider::new("https://eth.llamarpc.com");
    println!("[{}]", eth.chain());

    match eth.get_block_number().await {
        Ok(block) => println!("  Latest block : {block}"),
        Err(e) => eprintln!("  Block error   : {e}"),
    }

    match eth.get_fee_estimate().await {
        Ok(gas) => println!("  Gas price     : {gas}"),
        Err(e) => eprintln!("  Gas error     : {e}"),
    }

    // Vitalik's address — always has a balance
    let vitalik = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";
    match eth.get_balance(vitalik).await {
        Ok(bal) => println!("  Balance ({vitalik}): {bal}"),
        Err(e) => eprintln!("  Balance error : {e}"),
    }

    // ── Solana (devnet) ─────────────────────────────────────────
    println!();
    let sol = SolanaProvider::new("https://api.devnet.solana.com");
    println!("[{}]", sol.chain());

    match sol.get_block_number().await {
        Ok(slot) => println!("  Latest slot   : {slot}"),
        Err(e) => eprintln!("  Slot error    : {e}"),
    }

    match sol.get_fee_estimate().await {
        Ok(hash) => println!("  Blockhash     : {hash}"),
        Err(e) => eprintln!("  Hash error    : {e}"),
    }

    // System program — always exists
    let sys_program = "11111111111111111111111111111111";
    match sol.get_balance(sys_program).await {
        Ok(bal) => println!("  Balance ({sys_program}): {bal} lamports"),
        Err(e) => eprintln!("  Balance error : {e}"),
    }

    println!("\nDone.");
}
