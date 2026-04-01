use dioxus::prelude::*;

#[component]
pub fn Dashboard() -> Element {
    let mut wallet_created = use_signal(|| false);
    let mut eth_balance = use_signal(|| String::from("0.0"));
    let mut sol_balance = use_signal(|| String::from("0.0"));
    let mut eth_address = use_signal(|| String::new());
    let mut sol_address = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 2rem; max-width: 1200px; margin: 0 auto;",
            h2 { style: "margin-bottom: 1.5rem;", "Dashboard" }
            
            if !*wallet_created.read() {
                div {
                    class: "card";
                    style: "text-align: center; padding: 3rem;",
                    h3 { "Welcome to Xorion Wallet" }
                    p { style: "color: #a0a0a0; margin: 1rem 0 2rem;", 
                        "Create a new wallet or import an existing one to get started"
                    }
                    button {
                        class: "btn-primary";
                        onclick: move |_| {
                            wallet_created.set(true);
                            eth_balance.set(String::from("1.234"));
                            sol_balance.set(String::from("5.678"));
                            eth_address.set(String::from("0x742d35Cc6634C0532925a3b844Bc9e7595f8bE"));
                            sol_address.set(String::from("8ZqJzKxVvN9XqPvMxR3hYnFqKjWvLmPsT4uQwEr"));
                        };
                        "Create New Wallet"
                    }
                }
            } else {
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1.5rem;",
                    
                    // Ethereum Card
                    div {
                        class: "card";
                        h3 { style: "color: #6c47ff; margin-bottom: 1rem;", "⬡ Ethereum" }
                        p { style: "font-size: 2rem; font-weight: bold;", "{eth_balance} ETH" }
                        p { style: "color: #a0a0a0; font-size: 0.9rem; margin-top: 0.5rem;", 
                            "{eth_address}"
                        }
                        div { style: "display: flex; gap: 0.5rem; margin-top: 1rem;",
                            button { class: "btn-primary"; "Send" }
                            button { class: "btn-primary"; style: "background: #2a2a2a;", "Receive" }
                        }
                    }

                    // Solana Card
                    div {
                        class: "card";
                        h3 { style: "color: #00ffa3; margin-bottom: 1rem;", "◎ Solana" }
                        p { style: "font-size: 2rem; font-weight: bold;", "{sol_balance} SOL" }
                        p { style: "color: #a0a0a0; font-size: 0.9rem; margin-top: 0.5rem;", 
                            "{sol_address}"
                        }
                        div { style: "display: flex; gap: 0.5rem; margin-top: 1rem;",
                            button { class: "btn-primary"; "Send" }
                            button { class: "btn-primary"; style: "background: #2a2a2a;", "Receive" }
                        }
                    }

                    // Quick Actions
                    div {
                        class: "card";
                        h3 { style: "margin-bottom: 1rem;", "Quick Actions" }
                        div { style: "display: flex; flex-direction: column; gap: 0.75rem;",
                            button { class: "btn-primary"; style: "text-align: left;", "🔒 Enable Private Mode" }
                            button { class: "btn-primary"; style: "text-align: left; background: #2a2a2a;", "📦 Upload to IPFS" }
                            button { class: "btn-primary"; style: "text-align: left;", "🔄 Swap Tokens" }
                            button { class: "btn-primary"; style: "text-align: left; background: #2a2a2a;", "📊 View Analytics" }
                        }
                    }

                    // Recent Transactions
                    div {
                        class: "card";
                        h3 { style: "margin-bottom: 1rem;", "Recent Transactions" }
                        div { style: "color: #a0a0a0; text-align: center; padding: 2rem;",
                            "No recent transactions"
                        }
                    }
                }
            }
        }
    }
}
