use crate::cli::InputArgs;
use clap::Parser;

fn main() {
    let args = InputArgs::parse();

    let numbers = match args.numbers() {
        Ok(numbers) => numbers,
        Err(error) => {
            eprintln!("Error occurred. {}", error);
            std::process::exit(1);
        },
    };
}

mod cli;
