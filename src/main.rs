use crate::cli::InputArgs;
use crate::solver::PuzzleSolver;
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

    // Convert owned Strings to string slices for the solver
    let pieces: Vec<&str> = numbers.iter().map(|s| s.as_str()).collect();

    // Initialize the solver and find the solution
    let solver = PuzzleSolver::new(pieces);
    let result = solver.solve();

    println!("Longest sequence: {}", result);
}

mod cli;
mod errors;
mod solver;
