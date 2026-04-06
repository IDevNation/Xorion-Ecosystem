#[derive(Clone, Default, PartialEq)]
pub struct ExplorerLinks {
    pub home: Option<String>,
    pub address: Option<String>,
    pub block: Option<String>,
}

pub fn build_explorer_links(base_url: &str, address: Option<&str>, latest_block: Option<&str>) -> ExplorerLinks {
    let base = normalize_base_url(base_url);

    ExplorerLinks {
        home: base.clone(),
        address: match (base.as_deref(), address) {
            (Some(base), Some(address)) if is_valid_evm_address(address) => {
                Some(format!("{}/address/{}", base, address.trim()))
            }
            _ => None,
        },
        block: match (base.as_deref(), latest_block) {
            (Some(base), Some(block)) if is_valid_block_value(block) => {
                Some(format!("{}/block/{}", base, block.trim()))
            }
            _ => None,
        },
    }
}

fn normalize_base_url(value: &str) -> Option<String> {
    let trimmed = value.trim().trim_end_matches('/');
    if (trimmed.starts_with("http://") || trimmed.starts_with("https://"))
        && !trimmed.contains(' ')
        && !trimmed.contains("/address/")
        && !trimmed.contains("/block/")
    {
        Some(trimmed.to_string())
    } else {
        None
    }
}

fn is_valid_block_value(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty() && trimmed != "--" && trimmed.chars().all(|ch| ch.is_ascii_digit())
}

fn is_valid_evm_address(value: &str) -> bool {
    let trimmed = value.trim();
    trimmed.len() == 42
        && trimmed.starts_with("0x")
        && trimmed.chars().skip(2).all(|ch| ch.is_ascii_hexdigit())
}
