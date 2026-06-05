use clap::Parser;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Parser)]
pub struct InputArgs {
    #[clap(short, long)]
    pub file: PathBuf,
}

impl InputArgs {
    pub fn numbers(&self) -> Result<Vec<String>, InputError> {
        let content =
            std::fs::read_to_string(&self.file).map_err(InputError::FileRead)?;

        let mut numbers = Vec::new();
        for line in content.lines() {
            let line = line.trim().to_string();
            // Validating
            let _: u32 = line.trim().parse()?;
            numbers.push(line);
        }
        Ok(numbers)
    }
}

#[derive(Debug, Error)]
pub enum InputError {
    #[error("File read error: {0}")]
    FileRead(std::io::Error),

    #[error("Failed to parse number: {0}")]
    Parse(#[from] std::num::ParseIntError),
}
