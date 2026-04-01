use dioxus::prelude::*;

#[component]
pub fn ReceiveScreen() -> Element {
    let eth_address = "0x742d35Cc6634C0532925a3b844Bc9e7595f8bE";
    let sol_address = "8ZqJzKxVvN9XqPvMxR3hYnFqKjWvLmPsT4uQwEr";
    let mut selected_chain = use_signal(|| String::from("ethereum"));

    rsx! {
        div {
            style: "padding: 2rem; max-width: 600px; margin: 0 auto;",
            h2 { style: "margin-bottom: 1.5rem;", "Receive Crypto" }
            
            div {
                class: "card";
                style: "text-align: center;",
                
                label { "Select Chain" }
                select {
                    value: "{selected_chain}",
                    oninput: move |e| selected_chain.set(e.value.clone()),
                    style: "max-width: 300px;",
                    option { value: "ethereum", "Ethereum (ETH)" }
                    option { value: "solana", "Solana (SOL)" }
                    option { value: "polygon", "Polygon (MATIC)" }
                    option { value: "bsc", "Binance Smart Chain (BNB)" }
                }

                div {
                    style: "margin: 2rem 0; padding: 2rem; background: #ffffff; border-radius: 12px; display: inline-block;",
                    // QR Code placeholder - in production, use qrcode-generator crate
                    div {
                        style: "width: 200px; height: 200px; background: #000; margin: 0 auto; display: flex; align-items: center; justify-content: center;",
                        span { style: "color: white; font-size: 3rem;", "QR" }
                    }
                }

                label { style: "margin-top: 1rem;", "Your Address" }
                div {
                    style: "display: flex; gap: 0.5rem; margin-top: 0.5rem;",
                    input {
                        r#type: "text",
                        readonly: true,
                        value: if *selected_chain.read() == "ethereum" { eth_address } else { sol_address },
                        style: "flex: 1;",
                    }
                    button {
                        class: "btn-primary";
                        style: "white-space: nowrap;";
                        onclick: move |_| {
                            // Copy to clipboard
                            let address = if *selected_chain.read() == "ethereum" { eth_address } else { sol_address };
                            web_sys::window().unwrap().navigator().clipboard().unwrap().write_text(address);
                        };
                        "📋 Copy"
                    }
                }

                p {
                    style: "color: #a0a0a0; font-size: 0.85rem; margin-top: 1.5rem;",
                    "⚠️ Only send {selected_chain} tokens to this address. Sending other tokens may result in permanent loss."
                }

                div {
                    style: "margin-top: 2rem; padding: 1rem; background: rgba(108, 71, 255, 0.1); border-radius: 8px;",
                    p { style: "color: #6c47ff; font-weight: 600;", "💡 Pro Tip" }
                    p { style: "color: #a0a0a0; font-size: 0.9rem; margin-top: 0.5rem;", 
                        "Always verify the first and last 4 characters of the address before sending funds."
                    }
                }
            }
        }
    }
}
