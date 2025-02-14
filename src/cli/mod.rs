pub mod cli_args;

use clap::Parser;

use cli_args::Cli;
use crate::tool::read_json_pairs;

pub fn entry_point() {
    let args = Cli::parse();
    let pairs = read_json_pairs("cuttercookie.json");
    println!("Hello, {:?} / {:?} !", args.path, pairs);
}