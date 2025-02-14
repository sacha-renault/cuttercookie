pub mod cli_args;

use std::path::PathBuf;
use std::fs;

use clap::Parser;

use cli_args::Cli;
use crate::tool::{build_replacer, process_files};

/// Application entry point that handles command-line arguments and file processing
///
/// # Functionality
/// - Parses command-line arguments
/// - Validates working directory state
/// - Configures and executes file processing based on provided arguments
///
/// # Process Flow
/// 1. Parses command-line arguments using `clap`
/// 2. Constructs path to `cuttercookie.json` configuration file
/// 3. Initializes regex replacer with patterns from configuration
/// 4. Validates that current directory is empty
/// 5. Processes files according to specified parameters
///
/// # Returns
/// * `Result<(), String>` - Success (`Ok(())`) if processing completes,
///   or `Err` with error description if any step fails
///
/// # Errors
/// Returns error in following cases:
/// - Current directory is not empty
/// - Configuration file cannot be read or parsed
/// - File processing encounters errors
/// - Directory access is restricted
pub fn entry_point() -> Result<(), String> {
    // parse the console args
    let args = Cli::parse();

    // Build the path of cuttercookie.json
    let dest_path = PathBuf::new();
    let json_path = dest_path
        .join("cookiecutter.json")
        .to_string_lossy()
        .into_owned();

    // Build the replacer
    let replacer = build_replacer(&json_path)
        .map_err(|err| err.to_string())?;

    // Check if current dir is empty
    let entries = fs::read_dir(".")
        .expect("Failed to read current directory")
        .filter_map(|e| e.ok())
        .filter(|f| f.file_name() != "cookiecutter.json")
        .count();

    if entries > 0 {
        return Err("Current directory is not empty. Please run in an empty directory".to_string());
    }

    // process files
    match process_files(args.path.as_str(), &dest_path.display().to_string(), args.excluded_items, replacer) {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Proccess had error : {}", err))
    }
}