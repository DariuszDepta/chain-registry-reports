mod model;

use crate::model::{report, Chain};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const CHAIN_REGISTRY_REPO: &str = "../chain-registry";

type DataFiles = (Option<String>, Option<String>);

///
fn search_files(path: &Path, files: &mut BTreeMap<String, DataFiles>) {
    if let Ok(entries) = fs::read_dir(path) {
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

fn main() {
    let mut data_files = BTreeMap::new();
    search_files(Path::new(CHAIN_REGISTRY_REPO), &mut data_files);
    for (dir, (chain, _asset_list)) in data_files {
        if let Some(chain) = chain {
            let content = fs::read_to_string(format!("{}/{}", dir, chain))
                .expect("loading chain.json failed");
            let chain: Chain = serde_json::from_str(&content).expect("parsing chain.json failed");
            println!("{:80} {}", dir, report(&chain));
        }
    }
}
