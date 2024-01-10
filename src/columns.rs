use crate::model::Chain;
use std::collections::BTreeMap;
use std::fmt::Write;

const NA: &str = "-";

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Column {
    ChainName = 1,
    CosmWasmEnabled = 2,
    CosmWasmVersion = 3,
}

#[derive(Default)]
pub struct Columns(BTreeMap<Column, String>);

pub type Widths = BTreeMap<Column, usize>;

impl Columns {
    ///
    pub fn na(&mut self, column: Column) {
        self.set(column, NA.to_string());
    }

    ///
    pub fn set(&mut self, column: Column, value: String) {
        self.0.insert(column, value);
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
        columns.set(Column::ChainName, chain.chain_name);
        if let Some(codebase) = &chain.codebase {
            if let Some(cosmwasm_version) = &codebase.cosmwasm_version {
                columns.set(Column::CosmWasmVersion, cosmwasm_version.clone());
            } else {
                columns.na(Column::CosmWasmVersion);
            }
            if let Some(cosmwasm_enabled) = &codebase.cosmwasm_enabled {
                columns.set(Column::CosmWasmEnabled, cosmwasm_enabled.to_string());
            } else {
                columns.na(Column::CosmWasmEnabled);
            }
        } else {
            columns.na(Column::CosmWasmVersion);
            columns.na(Column::CosmWasmEnabled);
        }
        columns
    }
}
