use dioxus::prelude::*;

#[component]
pub fn SendScreen() -> Element {
    let mut chain = use_signal(|| String::from("ethereum"));
    let mut recipient = use_signal(|| String::new());
    let mut amount = use_signal(|| String::new());
    let mut private_mode = use_signal(|| false);
    let mut sending = use_signal(|| false);
    let mut success = use_signal(|| Option::<String>::None);

    rsx! {
        div {
            style: "padding: 2rem; max-width: 600px; margin: 0 auto;",
            h2 { style: "margin-bottom: 1.5rem;", "Send Crypto" }
            
            div {
                class: "card";
                
                label { "Select Chain" }
                select {
                    value: "{chain}",
                    oninput: move |e| chain.set(e.value.clone()),
                    option { value: "ethereum", "Ethereum (ETH)" }
                    option { value: "solana", "Solana (SOL)" }
                    option { value: "polygon", "Polygon (MATIC)" }
                    option { value: "bsc", "Binance Smart Chain (BNB)" }
                }

                label { style: "margin-top: 1rem;", "Recipient Address" }
                input {
                    r#type: "text",
                    placeholder: "Enter wallet address",
                    value: "{recipient}",
                    oninput: move |e| recipient.set(e.value.clone()),
                }

                label { style: "margin-top: 1rem;", "Amount" }
                input {
                    r#type: "number",
                    placeholder: "0.00",
                    value: "{amount}",
                    oninput: move |e| amount.set(e.value.clone()),
                }

                div {
                    style: "display: flex; align-items: center; gap: 0.75rem; margin: 1.5rem 0;",
                    input {
                        r#type: "checkbox",
                        checked: *private_mode.read(),
                        onchange: move |e| private_mode.set(e.checked),
                        style: "width: auto; margin: 0;",
                    }
                    label { 
                        style: "margin: 0; display: flex; align-items: center; gap: 0.5rem;",
                        "🔒 Enable Private Mode (zk-SNARKs)"
                    }
                }

                if let Some(msg) = &*success.read() {
                    div {
                        style: "background: rgba(0, 212, 170, 0.1); border: 1px solid #00d4aa; padding: 1rem; border-radius: 8px; margin: 1rem 0;",
                        p { style: "color: #00d4aa;", "✅ {msg}" }
                    }
                }

                button {
                    class: "btn-primary";
                    style: "width: 100%; margin-top: 1rem;";
                    disabled: *sending.read();
                    onclick: move |_| {
                        sending.set(true);
                        // Simulate transaction
                        setTimeout(move || {
                            sending.set(false);
                            success.set(Some(format!("Sent {} {} to {}", amount(), chain(), recipient())));
                        }, 2000);
                    };
                    if *sending.read() {
                        "Sending..."
                    } else {
                        "Send Transaction"
                    }
                }

                p {
                    style: "color: #a0a0a0; font-size: 0.85rem; margin-top: 1rem; text-align: center;",
                    if *private_mode.read() {
                        "🔐 This transaction will be processed using zero-knowledge proofs for maximum privacy."
                    } else {
                        "⚠️ This is a public transaction visible on the blockchain."
                    }
                }
            }
        }
    }
}
