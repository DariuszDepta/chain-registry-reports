use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct Chain {
    pub chain_name: String,
    pub codebase: Option<Codebase>,
}

#[derive(Default, Deserialize)]
pub struct Codebase {
    pub cosmwasm_version: Option<String>,
    pub cosmwasm_enabled: Option<bool>,
    pub cosmos_sdk_version: Option<String>,
    pub git_repo: Option<String>,
    pub recommended_version: Option<String>,
}
