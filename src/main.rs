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
                let parent_dir = path.parent().unwrap().to_string_lossy();
                let file_name = path.file_name().unwrap().to_string_lossy();
                if file_name == "chain.json" {
                    files
                        .entry(parent_dir.to_string())
                        .and_modify(|data| (*data).0 = Some(file_name.to_string()))
                        .or_insert((Some(file_name.to_string()), None));
                }
                if file_name == "assetlist.json" {
                    files
                        .entry(parent_dir.to_string())
                        .and_modify(|data| (*data).1 = Some(file_name.to_string()))
                        .or_insert((None, Some(file_name.to_string())));
                }
            }
        }
    }
}

fn main() {
    let mut data_files = BTreeMap::new();
    search_files(Path::new(CHAIN_REGISTRY_REPO), &mut data_files);
    for (a, b) in data_files {
        println!("{} {:?}", a, b);
    }
}
