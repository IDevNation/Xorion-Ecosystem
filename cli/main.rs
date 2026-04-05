fn main() {
    println!("Xorion Wallet CLI");

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  xorion wallet create");
        println!("  xorion wallet address");
        return;
    }

    match args[1].as_str() {
        "wallet" => {
            println!("Wallet command coming soon...");
        }
        _ => {
            println!("Unknown command");
        }
    }
}
