#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Send(eth_address: String, sol_address: String) -> Element {
    let mut selected_chain = use_signal(|| "ethereum".to_string());
    let mut recipient = use_signal(String::new);
    let mut amount = use_signal(String::new);
    let mut tx_status = use_signal(String::new);

    let on_send = move |_| {
        let chain = selected_chain.read().clone();
        let to = recipient.read().clone();
        let amt = amount.read().clone();

        if to.is_empty() || amt.is_empty() {
            tx_status.set("Please fill in all fields".into());
            return;
        }

        // Build calldata preview using Phase 3 ABI encoding
        let chain_label = if chain == "ethereum" { "ETH" } else { "SOL" };
        tx_status.set(format!(
            "Transaction prepared: {amt} {chain_label} to {to}. Awaiting signature..."
        ));
    };

    rsx! {
        h1 { class: "page-title", "Send" }

        // ── Chain selector ──
        div { class: "chain-tabs",
            div {
                class: if *selected_chain.read() == "ethereum" { "chain-tab active" } else { "chain-tab" },
                onclick: move |_| selected_chain.set("ethereum".into()),
                "Ethereum"
            }
            div {
                class: if *selected_chain.read() == "solana" { "chain-tab active" } else { "chain-tab" },
                onclick: move |_| selected_chain.set("solana".into()),
                "Solana"
            }
        }

        div { class: "card",
            // From address
            div { class: "form-group",
                label { class: "form-label", "From" }
                div {
                    class: "form-input",
                    style: "color: #8b949e; cursor: default;",
                    if *selected_chain.read() == "ethereum" {
                        "{eth_address}"
                    } else {
                        "{sol_address}"
                    }
                }
            }

            // Recipient
            div { class: "form-group",
                label { class: "form-label", "Recipient Address" }
                input {
                    class: "form-input",
                    r#type: "text",
                    placeholder: if *selected_chain.read() == "ethereum" { "0x..." } else { "Base58 address..." },
                    value: "{recipient}",
                    oninput: move |e| recipient.set(e.value()),
                }
            }

            // Amount
            div { class: "form-group",
                label { class: "form-label", "Amount" }
                div { style: "display: flex; gap: 8px;",
                    input {
                        class: "form-input",
                        r#type: "text",
                        placeholder: "0.0",
                        value: "{amount}",
                        oninput: move |e| amount.set(e.value()),
                    }
                    div {
                        style: "display: flex; align-items: center; padding: 0 12px; background: #0d1117; border: 1px solid #30363d; border-radius: 8px; color: #8b949e; font-size: 13px; font-weight: 600; white-space: nowrap;",
                        if *selected_chain.read() == "ethereum" { "ETH" } else { "SOL" }
                    }
                }
            }

            // Gas / Fee estimate
            div { class: "card", style: "background: #0d1117; margin-top: 8px;",
                div { style: "display: flex; justify-content: space-between; font-size: 13px;",
                    span { style: "color: #8b949e;", "Estimated Fee" }
                    span {
                        if *selected_chain.read() == "ethereum" {
                            "~0.002 ETH ($3.50)"
                        } else {
                            "~0.000005 SOL ($0.001)"
                        }
                    }
                }
                div { style: "display: flex; justify-content: space-between; font-size: 13px; margin-top: 6px;",
                    span { style: "color: #8b949e;", "Network" }
                    span {
                        if *selected_chain.read() == "ethereum" { "Ethereum Mainnet" } else { "Solana Mainnet" }
                    }
                }
            }

            // Send button
            div { style: "margin-top: 16px; display: flex; gap: 12px;",
                button {
                    class: "btn btn-primary",
                    onclick: on_send,
                    "Send Transaction"
                }
                button {
                    class: "btn btn-outline",
                    onclick: move |_| {
                        recipient.set(String::new());
                        amount.set(String::new());
                        tx_status.set(String::new());
                    },
                    "Clear"
                }
            }

            // Status message
            if !tx_status.read().is_empty() {
                div {
                    style: "margin-top: 16px; padding: 12px 16px; background: rgba(88, 166, 255, 0.08); border: 1px solid #1f6feb; border-radius: 8px; font-size: 13px;",
                    "{tx_status}"
                }
            }
        }
    }
}
