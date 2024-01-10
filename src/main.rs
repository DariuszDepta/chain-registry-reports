mod columns;
mod model;

use crate::columns::{Column, Columns, Widths};
use crate::model::Chain;
use std::collections::{BTreeMap, HashSet};
use std::fs::{read_dir, read_to_string};
use std::path::Path;

const CHAIN_REGISTRY_REPO: &str = "../chain-registry";

type DataFiles = (Option<String>, Option<String>);

///
fn search_files(path: &Path, files: &mut BTreeMap<String, DataFiles>) {
    if let Ok(entries) = read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                search_files(&path, files);
            } else {
                let parent_dir = path
                    .canonicalize()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                let file_name = path.file_name().unwrap().to_string_lossy();
                if file_name == "chain.json" {
                    files
                        .entry(parent_dir.clone())
                        .and_modify(|data| data.0 = Some(file_name.to_string()))
                        .or_insert((Some(file_name.to_string()), None));
                }
                if file_name == "assetlist.json" {
                    files
                        .entry(parent_dir.clone())
                        .and_modify(|data| data.1 = Some(file_name.to_string()))
                        .or_insert((None, Some(file_name.to_string())));
                }
            }
        }
    }
}

fn chains(path: &Path) -> Vec<Chain> {
    let mut chains: Vec<Chain> = vec![];
    let mut data_files = BTreeMap::new();
    search_files(path, &mut data_files);
    for (dir, (chain, _asset_list)) in data_files {
        if let Some(chain) = chain {
            let file_name = format!("{}/{}", dir, chain);
            let content = read_to_string(file_name).expect("loading chain.json failed");
            chains.push(serde_json::from_str(&content).expect("parsing chain.json failed"));
        }
    }
    chains.sort_by_key(|chain| chain.chain_name.clone());
    chains
}

fn widths() -> Widths {
    let mut widths = BTreeMap::new();
    widths.insert(Column::ChainName, 30);
    widths.insert(Column::CosmWasmEnabled, 8);
    widths.insert(Column::CosmWasmVersion, 10);
    widths.insert(Column::CosmosSdkVersion, 10);
    widths.insert(Column::GitRepository, 40);
    widths.insert(Column::RecommendedVersion, 11);
    widths
}

fn displayed(displayed: &[&str]) -> HashSet<String> {
    displayed.iter().map(|s| s.to_string()).collect()
}

#[rustfmt::skip]
fn print_header() {
    println!("┌────────────────────────────────┬───────────────────────┬────────────┬──────────────────────────────────────────┬─────────────┐");
    println!("│                                │       CosmWasm        │            │                                          │ Chain       │");
    println!("│     Chain name                 ├──────────┬────────────┤ Cosmos SDK │                                          │ recommended │");
    println!("│                                │ enabled  │  version   │   version  │ Git repository                           │ version     │");
    println!("├────────────────────────────────┼──────────┼────────────┼────────────┼──────────────────────────────────────────┼─────────────┤");
}

#[rustfmt::skip]
fn print_footer() {
    println!("└────────────────────────────────┴──────────┴────────────┴────────────┴──────────────────────────────────────────┴─────────────┘");
}

fn main() {
    let path = Path::new(CHAIN_REGISTRY_REPO);
    let widths = widths();
    let displayed = displayed(&["neutron", "terra", "terra2"]);
    print_header();
    chains(path)
        .drain(..)
        .filter(|chain| !chain.chain_name.is_empty())
        .filter(|chain| displayed.contains(&chain.chain_name))
        .for_each(|chain| {
            let columns: Columns = chain.into();
            println!("│{}", columns.report_line(&widths));
        });
    print_footer();
}
