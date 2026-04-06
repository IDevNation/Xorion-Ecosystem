#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Dashboard(
    eth_address: String,
    sol_address: String,
    eth_balance: String,
    sol_balance: String,
) -> Element {
    let short_eth = if eth_address.len() > 10 {
        format!("{}...{}", &eth_address[..6], &eth_address[eth_address.len()-4..])
    } else {
        eth_address.clone()
    };

    let short_sol = if sol_address.len() > 10 {
        format!("{}...{}", &sol_address[..6], &sol_address[sol_address.len()-4..])
    } else {
        sol_address.clone()
    };

    rsx! {
        h1 { class: "page-title", "Dashboard" }

        // ── Balance cards ──
        div { class: "balance-grid",
            div { class: "card",
                div { class: "card-title", "Ethereum Balance" }
                div { class: "card-value blue", "{eth_balance} ETH" }
                div {
                    style: "margin-top: 8px; font-size: 12px; color: #8b949e; font-family: monospace;",
                    "{short_eth}"
                }
            }
            div { class: "card",
                div { class: "card-title", "Solana Balance" }
                div { class: "card-value green", "{sol_balance} SOL" }
                div {
                    style: "margin-top: 8px; font-size: 12px; color: #8b949e; font-family: monospace;",
                    "{short_sol}"
                }
            }
        }

        // ── Portfolio summary ──
        div { class: "card",
            div { class: "card-title", "Portfolio Overview" }
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-top: 8px;",
                div {
                    div { style: "font-size: 32px; font-weight: 700;", "$0.00" }
                    div { style: "font-size: 13px; color: #8b949e; margin-top: 4px;", "Total Value (USD)" }
                }
                div {
                    span { class: "badge badge-green", "Mainnet" }
                }
            }
        }

        // ── Recent transactions ──
        h2 { style: "font-size: 16px; font-weight: 600; margin-bottom: 12px;", "Recent Transactions" }
        div { class: "tx-list",
            // Sample transaction rows
            div { class: "tx-row",
                div {
                    div { class: "tx-direction receive", "RECEIVED" }
                    div { class: "tx-hash", "0x7a3f...e4b2" }
                }
                div { class: "tx-amount", style: "color: #3fb950;", "+0.05 ETH" }
            }
            div { class: "tx-row",
                div {
                    div { class: "tx-direction send", "SENT" }
                    div { class: "tx-hash", "0xb1c8...3d9f" }
                }
                div { class: "tx-amount", style: "color: #f85149;", "-0.02 ETH" }
            }
            div { class: "tx-row",
                div {
                    div { class: "tx-direction receive", "RECEIVED" }
                    div { class: "tx-hash", "5Kj2p...Rm7x" }
                }
                div { class: "tx-amount", style: "color: #3fb950;", "+1.5 SOL" }
            }
            div { class: "tx-row",
                div {
                    div { class: "tx-direction send", "SENT" }
                    div { class: "tx-hash", "3nVq8...Yt2k" }
                }
                div { class: "tx-amount", style: "color: #f85149;", "-50 USDC" }
            }
        }

        // ── Network status ──
        div { class: "card", style: "margin-top: 16px;",
            div { class: "card-title", "Network Status" }
            div { style: "display: flex; gap: 24px; margin-top: 8px;",
                div { style: "display: flex; align-items: center; gap: 6px;",
                    div { style: "width: 8px; height: 8px; border-radius: 50%; background: #3fb950;" }
                    span { style: "font-size: 13px;", "Ethereum Mainnet" }
                }
                div { style: "display: flex; align-items: center; gap: 6px;",
                    div { style: "width: 8px; height: 8px; border-radius: 50%; background: #3fb950;" }
                    span { style: "font-size: 13px;", "Solana Mainnet" }
                }
            }
        }
    }
}
