pub mod cli_args;

use std::path::PathBuf;

use clap::Parser;

use cli_args::Cli;
use crate::tool::{read_json_pairs, process_files};

pub fn entry_point() {
    // parse the console args
    let args = Cli::parse();

    // Build the path of cuttercookie.json
    let path = PathBuf::new().join(&args.path).join("cuttercookie.json");
    let path_str = path.to_string_lossy().into_owned();

    // Build the replacer
    let replacer = read_json_pairs(&path_str);

    // process files
    match process_files(args.path.as_str(), args.excluded_dirs, replacer) {
        Ok(_) => println!("Finished with success"),
        Err(err) => panic!("Proccess had error : {}", err)
    }
    println!("Hello, {:?} !", args.path);
}