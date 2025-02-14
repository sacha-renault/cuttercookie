mod cli;
mod tool;

use cli::entry_point;

fn main() {
    match entry_point() {
        Ok(_) => println!("Process ran with success"),
        Err(e) => eprintln!("Error: {}", e)
    }
}
