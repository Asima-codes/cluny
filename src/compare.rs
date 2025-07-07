use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use sha2::{Sha256,Digest};
struct FileInfo{
    hash: String,
    path: PathBuf
}

pub fn hash(path: &Path) -> String {
    let mut file = File::open(path).unwrap();
    let mut hasher = Sha256::new();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer);
    hasher.update(buffer);
    format!("{:x}",hasher.finalize())
} 

pub fn same(root: &Path) -> HashMap<String,Vec<PathBuf>> {
    let mut files:HashMap<String, Vec<PathBuf>> = HashMap::new();
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            let hashed = hash(path);
            files.entry(hashed).or_default().push(path.to_path_buf());

        }
    }

    let mut duplicates = HashMap::new();
    for(hashed,paths) in files {
        if paths.len() > 1 {
            duplicates.insert(hashed,paths);
        }
    }
    duplicates
}

