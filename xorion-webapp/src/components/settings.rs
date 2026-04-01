use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    let mut eth_rpc = use_signal(|| String::from("https://eth.llamarpc.com"));
    let mut sol_rpc = use_signal(|| String::from("https://api.mainnet-beta.solana.com"));
    let mut ipfs_gateway = use_signal(|| String::from("https://ipfs.io"));
    let mut dark_mode = use_signal(|| true);
    let mut notifications = use_signal(|| true);
    let mut saved = use_signal(|| Option::<String>::None);

    rsx! {
        div {
            style: "padding: 2rem; max-width: 600px; margin: 0 auto;",
            h2 { style: "margin-bottom: 1.5rem;", "Settings" }
            
            if let Some(msg) = &*saved.read() {
                div {
                    style: "background: rgba(0, 212, 170, 0.1); border: 1px solid #00d4aa; padding: 1rem; border-radius: 8px; margin-bottom: 1rem;",
                    p { style: "color: #00d4aa;", "✅ {msg}" }
                }
            }

            div {
                class: "card";
                
                h3 { style: "margin-bottom: 1rem; color: #6c47ff;", "🔗 RPC Configuration" }
                
                label { "Ethereum RPC URL" }
                input {
                    r#type: "text",
                    value: "{eth_rpc}",
                    oninput: move |e| eth_rpc.set(e.value.clone()),
                    placeholder: "https://...",
                }

                label { style: "margin-top: 1rem;", "Solana RPC URL" }
                input {
                    r#type: "text",
                    value: "{sol_rpc}",
                    oninput: move |e| sol_rpc.set(e.value.clone()),
                    placeholder: "https://...",
                }

                label { style: "margin-top: 1rem;", "IPFS Gateway" }
                input {
                    r#type: "text",
                    value: "{ipfs_gateway}",
                    oninput: move |e| ipfs_gateway.set(e.value.clone()),
                    placeholder: "https://...",
                }

                h3 { style: "margin: 2rem 0 1rem; color: #6c47ff;", "⚙️ Preferences" }
                
                div {
                    style: "display: flex; align-items: center; justify-content: space-between; margin: 1rem 0;",
                    div {
                        label { style: "margin: 0;", "🌙 Dark Mode" }
                    }
                    input {
                        r#type: "checkbox",
                        checked: *dark_mode.read(),
                        onchange: move |e| dark_mode.set(e.checked),
                        style: "width: auto; margin: 0;",
                    }
                }

                div {
                    style: "display: flex; align-items: center; justify-content: space-between; margin: 1rem 0;",
                    div {
                        label { style: "margin: 0;", "🔔 Notifications" }
                    }
                    input {
                        r#type: "checkbox",
                        checked: *notifications.read(),
                        onchange: move |e| notifications.set(e.checked),
                        style: "width: auto; margin: 0;",
                    }
                }

                h3 { style: "margin: 2rem 0 1rem; color: #6c47ff;", "🔐 Security" }
                
                button {
                    class: "btn-primary";
                    style: "width: 100%; margin: 0.5rem 0; background: #2a2a2a;";
                    "🔑 Export Private Keys"
                }
                
                button {
                    class: "btn-primary";
                    style: "width: 100%; margin: 0.5rem 0; background: #2a2a2a;";
                    "📄 View Seed Phrase"
                }

                button {
                    class: "btn-primary";
                    style: "width: 100%; margin: 0.5rem 0; background: #ff4757;";
                    "🗑️ Clear Wallet Data"
                }

                button {
                    class: "btn-primary";
                    style: "width: 100%; margin-top: 1.5rem;";
                    onclick: move |_| {
                        saved.set(Some("Settings saved successfully!"));
                        setTimeout(move || {
                            saved.set(None);
                        }, 3000);
                    };
                    "💾 Save Settings"
                }
            }

            div {
                class: "card";
                style: "margin-top: 1.5rem;",
                h3 { style: "margin-bottom: 1rem;", "ℹ️ About" }
                p { style: "color: #a0a0a0;", "Xorion Wallet v1.0.0" }
                p { style: "color: #a0a0a0; font-size: 0.9rem; margin-top: 0.5rem;", 
                    "The Web3-Native Operating System"
                }
                div { style: "margin-top: 1rem; display: flex; gap: 0.5rem;",
                    a {
                        href: "https://github.com/IDevNation/Xorion-Web3-Beta-Multi-Chain-Wallet-SDK";
                        target: "_blank";
                        button { class: "btn-primary"; style: "background: #2a2a2a; font-size: 0.9rem;", "GitHub" }
                    }
                    a {
                        href: "https://xorion.io";
                        target: "_blank";
                        button { class: "btn-primary"; style: "background: #2a2a2a; font-size: 0.9rem;", "Website" }
                    }
                }
            }
        }
    }
}
