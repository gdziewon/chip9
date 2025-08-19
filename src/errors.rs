use std::{fmt, error};

#[derive(Debug)]
pub enum Chip9Error {
    FileReadError(String),
    MissingFilePath,
    TooManyLines(usize, usize),
    UnrecognizedOpcode(u16),
    WindowCreationError(minifb::Error),
    WindowUpdateError(minifb::Error),
}

impl fmt::Display for Chip9Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Chip9Error::FileReadError(file_path) => write!(f, "Failed to read file: {}", file_path),
            Chip9Error::MissingFilePath => write!(f, "Expected a file path as the argument"),
            Chip9Error::TooManyLines(lines, available) => write!(f, "File has too many lines: {}. Maximum memory available for a program is {}.", lines, available),
            Chip9Error::UnrecognizedOpcode(op) => write!(f, "Unrecognized opcode: {:#X}", op),
            Chip9Error::WindowCreationError(e) => write!(f, "Window creation error: {}", e),
            Chip9Error::WindowUpdateError(e) => write!(f, "Window update error: {}", e),
        }
    }
}

impl error::Error for Chip9Error {}
