use crate::compare::same;
use clap::{Parser,Subcommand};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::{self, Write};
pub fn quarantine_duplicate(path: &Path) {
    let duplicates = same(path);
    if let Some((_, group)) = duplicates.into_iter().next() {
        if let Some(duplicate) = group.get(0) {
            let quarantine_dir = path.join(".quarantine_files");
            if !quarantine_dir.exists() {
                fs::create_dir_all(&quarantine_dir).unwrap();
            }

            let destination = quarantine_dir.join(duplicate.file_name().unwrap());
            if let Err(e) = fs::rename(duplicate, &destination) {
                eprintln!("Failed to quarantine file: {}", e);
            } else {
                println!("File quarantined: {}", destination.display());
            }
        }
    }
}
pub fn delete_quarantine() {
    let quarantine_dir = Path::new(".quarantine_files");
    if quarantine_dir.exists() {
        if let Err(e) = fs::remove_dir_all(quarantine_dir) {
            eprintln!("Failed to delete quarantine folder: {}", e);
        } else {
            println!("Quarantine folder deleted.");
        }
    } else {
        println!("No quarantine folder found.");
    }
}
pub fn restore_from_quarantine() {
    let quarantine_dir = Path::new(".quarantine_files");

    if quarantine_dir.exists() {
        match fs::read_dir(quarantine_dir) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let quarantine_path = entry.path();
                            let original_path = Path::new(".").join(quarantine_path.file_name().unwrap());

                            if let Err(e) = fs::rename(&quarantine_path, &original_path) {
                                eprintln!("Failed to restore file '{}': {}", quarantine_path.display(), e);
                            } else {
                                println!("File restored: {}", original_path.display());
                            }
                        }
                        Err(e) => eprintln!("Error accessing file in quarantine: {}", e),
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading the quarantine directory: {}", e);
            }
        }
    } else {
        println!("No quarantine folder found.");
    }
}