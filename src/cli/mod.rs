pub mod cli_args;

use clap::Parser;

use cli_args::Cli;

pub fn entry_point() {
    let args = Cli::parse();
    println!("Hello, {:?} !", args.path);
}