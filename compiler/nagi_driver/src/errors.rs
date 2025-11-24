use std::{error::Error, fmt::Display, io};

#[derive(Debug)]
pub(crate) enum CompileError {
    IOError(io::Error),
    Other(String),
}

impl CompileError {}

impl Error for CompileError {}

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::IOError(e) => write!(f, "{}", e),
            CompileError::Other(e) => write!(f, "{}", e),
        }
    }
}

impl From<io::Error> for CompileError {
    fn from(value: io::Error) -> Self {
        CompileError::IOError(value)
    }
}

impl From<String> for CompileError {
    fn from(value: String) -> Self {
        CompileError::Other(value)
    }
}
