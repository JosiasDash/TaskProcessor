mod cli;
use dotenv::dotenv;
mod tasks;
mod data;
mod process;
mod utils;

fn main() {
    dotenv().ok();
    cli::prompt();
}
