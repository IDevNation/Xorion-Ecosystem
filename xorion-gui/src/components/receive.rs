#![allow(non_snake_case)]

use dioxus::prelude::*;
use qrcode::QrCode;

#[component]
pub fn Receive(eth_address: String, sol_address: String) -> Element {
    let mut selected_chain = use_signal(|| "ethereum".to_string());
    let mut copied = use_signal(|| false);

    let address = if *selected_chain.read() == "ethereum" {
        eth_address.clone()
    } else {
        sol_address.clone()
    };

    // Generate QR code as SVG
    let qr_svg = generate_qr_svg(&address);

    rsx! {
        h1 { class: "page-title", "Receive" }

        // ── Chain selector ──
        div { class: "chain-tabs",
            div {
                class: if *selected_chain.read() == "ethereum" { "chain-tab active" } else { "chain-tab" },
                onclick: move |_| { selected_chain.set("ethereum".into()); copied.set(false); },
                "Ethereum"
            }
            div {
                class: if *selected_chain.read() == "solana" { "chain-tab active" } else { "chain-tab" },
                onclick: move |_| { selected_chain.set("solana".into()); copied.set(false); },
                "Solana"
            }
        }

        div { class: "card",
            div { class: "qr-container",
                // QR code
                div { class: "qr-box",
                    div {
                        dangerous_inner_html: "{qr_svg}",
                    }
                }

                // Chain label
                div {
                    style: "font-size: 14px; font-weight: 600; margin-bottom: 12px;",
                    if *selected_chain.read() == "ethereum" {
                        "Ethereum Address"
                    } else {
                        "Solana Address"
                    }
                }

                // Address display
                div { class: "address-box", "{address}" }

                // Copy button
                button {
                    class: "copy-btn",
                    onclick: move |_| {
                        copied.set(true);
                    },
                    if *copied.read() { "Copied!" } else { "Copy Address" }
                }
            }
        }

        // Warning
        div { class: "card", style: "border-color: #d29922;",
            div { style: "display: flex; align-items: center; gap: 8px;",
                span { style: "font-size: 18px;", "\u{26A0}" }
                div {
                    div { style: "font-size: 13px; font-weight: 600; color: #d29922;", "Important" }
                    div {
                        style: "font-size: 12px; color: #8b949e; margin-top: 2px;",
                        if *selected_chain.read() == "ethereum" {
                            "Only send Ethereum (ETH) and ERC-20 tokens to this address."
                        } else {
                            "Only send Solana (SOL) and SPL tokens to this address."
                        }
                    }
                }
            }
        }
    }
}

/// Generate a QR code as an SVG string.
fn generate_qr_svg(data: &str) -> String {
    if data.is_empty() {
        return r##"<svg width="180" height="180" xmlns="http://www.w3.org/2000/svg"><rect width="180" height="180" fill="#fff"/><text x="90" y="95" text-anchor="middle" font-size="12" fill="#666">No Address</text></svg>"##.to_string();
    }

    let code = match QrCode::new(data.as_bytes()) {
        Ok(c) => c,
        Err(_) => return String::from("<svg></svg>"),
    };

    let modules = code.to_colors();
    let size = code.width();
    let scale = 4;
    let margin = 8;
    let total = size * scale + margin * 2;

    let mut svg = format!(
        r##"<svg width="{total}" height="{total}" xmlns="http://www.w3.org/2000/svg"><rect width="{total}" height="{total}" fill="#fff"/>"##
    );

    for y in 0..size {
        for x in 0..size {
            let idx = y * size + x;
            if idx < modules.len() && modules[idx] == qrcode::Color::Dark {
                let px = x * scale + margin;
                let py = y * scale + margin;
                svg.push_str(&format!(
                    r##"<rect x="{px}" y="{py}" width="{scale}" height="{scale}" fill="#0d1117"/>"##
                ));
            }
        }
    }

    svg.push_str("</svg>");
    svg
}
