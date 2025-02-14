pub mod cli_args;

use std::path::PathBuf;
use std::fs;

use clap::Parser;

use cli_args::Cli;
use crate::tool::{read_json_pairs, process_files};

pub fn entry_point() -> Result<(), String> {
    // parse the console args
    let args = Cli::parse();

    // Build the path of cuttercookie.json
    let path = PathBuf::new().join(&args.path).join("cuttercookie.json");
    let path_str = path.to_string_lossy().into_owned();

    // Build the replacer
    let replacer = read_json_pairs(&path_str);

    // Check if current dir is empty
    let entries = fs::read_dir(".")
        .expect("Failed to read current directory")
        .filter_map(|e| e.ok())
        .count();
    if entries > 0 {
        return Err("Current directory is not empty. Please run in an empty directory".to_string());
    }

    // process files
    match process_files(args.path.as_str(), ".", args.excluded_items, replacer) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Proccess had error : {}", err))
    }
}