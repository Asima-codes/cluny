mod compare;
mod quarantine;
use clap::{Parser,Subcommand};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::{self, Write};
#[derive(Parser, Debug)]
#[command(name = "cluny")]
#[command(about = "Detects duplicate files in a given directory", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Same {
        #[arg(short, long)]
        path: PathBuf,
    },
    Quarantine {
        #[arg(short, long)]
        path: PathBuf,
    },
    Delete,
    Restore,
}
fn main() {
    let args = Args::parse();
    match args.command {
        Command::Same { path } => show_duplicates(&path),
        Command::Quarantine { path } => quarantine::quarantine_duplicate(&path),
        Command::Delete => quarantine::delete_quarantine(),
        Command::Restore => quarantine::restore_from_quarantine(),
    }
}
fn show_duplicates(path: &Path) {
    let duplicates = compare::same(path);
    if duplicates.is_empty() {
        println!("No duplicate files found.");
    } else {
        println!("Duplicate files found:");
        for (hash, group) in duplicates {
            println!("Hash: {}", hash);
            for path in group {
                println!("  {}", path.display());
            }
        }
    }
}   



