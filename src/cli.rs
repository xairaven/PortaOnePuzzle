use crate::errors::AppError;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct InputArgs {
    #[arg(short, long)]
    pub file: PathBuf,
}

impl InputArgs {
    pub fn read_numbers(&self) -> Result<Vec<String>, AppError> {
        let content = std::fs::read_to_string(&self.file)?;
        let mut numbers = Vec::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Validate that the line is a valid integer
            let _: u32 = trimmed.parse()?;
            numbers.push(trimmed.to_string());
        }

        Ok(numbers)
    }
}
