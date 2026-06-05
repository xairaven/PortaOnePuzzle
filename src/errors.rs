use std::io;
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("File read. {0}")]
    FileRead(#[from] io::Error),

    #[error("Failed to parse number. {0}")]
    Parse(#[from] ParseIntError),
}
