use clap::Parser;

#[derive(Parser)]
#[command(name = "fs-tools")]
#[command(about = "File system utility tools", long_about = None)]
pub struct Cli {
    /// Directory path to start from
    pub path: String,

    /// Directory that will not be included
    #[arg(long, short, value_delimiter = ',', default_value = "Vec::new()")]
    pub excluded_items: Vec<String>
}