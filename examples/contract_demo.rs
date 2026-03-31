use xorion_sdk::contract::abi;
use xorion_sdk::contract::erc20::Erc20;
use xorion_sdk::contract::defi::UniswapV2Router;
use xorion_sdk::rpc::ethereum::EthereumProvider;

// Well-known mainnet addresses
const USDC: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
const WETH: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
const UNISWAP_V2_ROUTER: &str = "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D";
const VITALIK: &str = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";

#[tokio::main]
async fn main() {
    println!("=== Xorion Smart Contract Demo ===\n");

    // ── ABI encoding basics ────────────────────────────────────
    println!("--- ABI Encoding ---");
    let sel = abi::function_selector("transfer(address,uint256)");
    println!("transfer selector : 0x{}", hex::encode(sel));

    let sel2 = abi::function_selector("balanceOf(address)");
    println!("balanceOf selector: 0x{}", hex::encode(sel2));

    let calldata = abi::encode_call_hex(
        "balanceOf(address)",
        &[abi::hex_to_address(VITALIK).unwrap()],
    )
    .unwrap();
    println!("balanceOf calldata: {}...{}", &calldata[..18], &calldata[calldata.len()-8..]);

    // ── ERC-20 token queries (live RPC) ────────────────────────
    println!("\n--- ERC-20: USDC ---");
    let provider = EthereumProvider::new("https://eth.llamarpc.com");
    let usdc = Erc20::new(USDC, provider);

    match usdc.name().await {
        Ok(name) => println!("Name    : {name}"),
        Err(e) => eprintln!("Name err: {e}"),
    }
    match usdc.symbol().await {
        Ok(sym) => println!("Symbol  : {sym}"),
        Err(e) => eprintln!("Sym err : {e}"),
    }
    match usdc.decimals().await {
        Ok(dec) => println!("Decimals: {dec}"),
        Err(e) => eprintln!("Dec err : {e}"),
    }
    match usdc.total_supply().await {
        Ok(supply) => println!("Supply  : {supply}"),
        Err(e) => eprintln!("Sup err : {e}"),
    }
    match usdc.balance_of(VITALIK).await {
        Ok(bal) => println!("Vitalik : {bal} USDC (raw)"),
        Err(e) => eprintln!("Bal err : {e}"),
    }

    // ── Encode a transfer (offline) ────────────────────────────
    println!("\n--- Encode Transfer ---");
    let transfer_data = usdc
        .encode_transfer(VITALIK, "0x3B9ACA00") // 1 USDC (6 decimals)
        .unwrap();
    println!("transfer calldata : {}...{}", &transfer_data[..18], &transfer_data[transfer_data.len()-8..]);

    // ── Uniswap V2 Router (encode swap) ────────────────────────
    println!("\n--- Uniswap V2 Router ---");
    let router_provider = EthereumProvider::new("https://eth.llamarpc.com");
    let router = UniswapV2Router::new(UNISWAP_V2_ROUTER, router_provider);

    match router.factory().await {
        Ok(factory) => println!("Factory : {factory}"),
        Err(e) => eprintln!("Fact err: {e}"),
    }
    match router.weth().await {
        Ok(weth) => println!("WETH    : {weth}"),
        Err(e) => eprintln!("WETH err: {e}"),
    }

    // Quote: 1 ETH → USDC
    match router.get_amounts_out("0xDE0B6B3A7640000", &[WETH, USDC]).await {
        Ok(amounts) => {
            println!("Swap quote (1 ETH → USDC):");
            for (i, amt) in amounts.iter().enumerate() {
                println!("  step {i}: {amt}");
            }
        }
        Err(e) => eprintln!("Quote err: {e}"),
    }

    // Encode a swap (offline)
    let swap_data = router
        .encode_swap_exact_tokens(
            "0xDE0B6B3A7640000",
            "0x1",
            &[WETH, USDC],
            VITALIK,
            "0xFFFFFFFF",
        )
        .unwrap();
    println!("swap calldata     : {}...", &swap_data[..18]);

    println!("\nDone.");
}
