use crate::model::Chain;
use std::collections::BTreeMap;
use std::fmt::Write;

const NA: &str = "-";

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Column {
    ChainName = 1,
    CosmWasmEnabled = 2,
    CosmWasmVersion = 3,
    CosmosSdkVersion = 4,
    GitRepository = 5,
    RecommendedVersion = 6,
}

#[derive(Default)]
pub struct Columns(BTreeMap<Column, String>);

pub type Widths = BTreeMap<Column, usize>;

impl Columns {
    ///
    pub fn na(&mut self, column: Column) {
        self.0.insert(column, NA.to_string());
    }

    ///
    pub fn set<T: ToString>(&mut self, column: Column, value: &Option<T>) {
        if let Some(value) = value {
            self.0.insert(column, value.to_string());
        } else {
            self.0.insert(column, NA.to_string());
        }
    }

    pub fn report_line(&self, widths: &Widths) -> String {
        let mut line = String::new();
        for (column, value) in &self.0 {
            let width = *widths.get(column).unwrap();
            let _ = write!(&mut line, " {:w$} │", value, w = width);
        }
        line
    }
}

impl From<Chain> for Columns {
    ///
    fn from(chain: Chain) -> Self {
        let mut columns = Self::default();
        columns.set(Column::ChainName, &chain.chain_name.into());
        if let Some(codebase) = &chain.codebase {
            columns.set(Column::CosmWasmVersion, &codebase.cosmwasm_version);
            columns.set(Column::CosmWasmEnabled, &codebase.cosmwasm_enabled);
            columns.set(Column::CosmosSdkVersion, &codebase.cosmos_sdk_version);
            columns.set(Column::GitRepository, &codebase.git_repo);
            columns.set(Column::RecommendedVersion, &codebase.recommended_version);
        } else {
            columns.na(Column::CosmWasmVersion);
            columns.na(Column::CosmWasmEnabled);
            columns.na(Column::CosmosSdkVersion);
            columns.na(Column::GitRepository);
            columns.na(Column::RecommendedVersion);
        }
        columns
    }
}
