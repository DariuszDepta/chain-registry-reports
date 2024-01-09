use serde::Deserialize;
use std::fmt::Write;

#[derive(Default, Deserialize)]
pub struct Chain {
    pub chain_name: String,
    pub codebase: Option<Codebase>,
}

#[derive(Default, Deserialize)]
pub struct Codebase {
    pub cosmwasm_version: Option<String>,
    pub cosmwasm_enabled: Option<bool>,
}

pub fn report(chain: &Chain) -> String {
    let mut line = String::new();
    let _ = write!(&mut line, "{}", chain.chain_name);
    if let Some(codebase) = &chain.codebase {
        if let Some(cosmwasm_version) = &codebase.cosmwasm_version {
            let _ = write!(&mut line, " {}", cosmwasm_version);
        }
        if let Some(cosmwasm_enabled) = &codebase.cosmwasm_enabled {
            let _ = write!(&mut line, " {}", cosmwasm_enabled);
        } else {
            let _ = write!(&mut line, " (none)");
        }
    } else {
        let _ = write!(&mut line, " (no codebase)");
    }
    line
}
