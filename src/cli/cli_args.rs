use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "fs-tools")]
#[command(about = "File system utility tools", long_about = None)]
pub struct Cli {
    /// Directory path to start from
    pub path: PathBuf,
}