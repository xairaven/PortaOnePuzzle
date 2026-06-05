use crate::cli::InputArgs;
use clap::Parser;

fn main() {
    // Parse command line arguments
    let args = InputArgs::parse();

    // Attempt to read and validate numbers from the file
    let numbers = match args.read_numbers() {
        Ok(numbers) => numbers,
        Err(error) => {
            eprintln!("Error occurred: {}", error);
            std::process::exit(1);
        },
    };
}

mod cli;
mod errors;
