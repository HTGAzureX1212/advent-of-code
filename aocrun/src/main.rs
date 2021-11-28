use std::process;

pub mod cli;

pub fn main() {
    process::exit(cli::run(cli::RunOptions {
        year: None,
        day: None,
        bench: None
    }).unwrap().0)
}
