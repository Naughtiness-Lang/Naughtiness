use std::{error::Error, fmt::Display, io};

use nagi_command_option::errors::CommandOptionError;

#[derive(Debug)]
pub(crate) enum CompileError {
    IOError(io::Error),
    CommandOptionError(CommandOptionError),
    Other(String), // 専用のエラーができるまで
}

impl CompileError {}

impl Error for CompileError {}

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::IOError(e) => write!(f, "{e}"),
            CompileError::CommandOptionError(e) => write!(f, "{}", e.message),
            CompileError::Other(e) => write!(f, "{e}"),
        }
    }
}

impl From<io::Error> for CompileError {
    fn from(value: io::Error) -> Self {
        CompileError::IOError(value)
    }
}

impl From<CommandOptionError> for CompileError {
    fn from(value: CommandOptionError) -> Self {
        CompileError::CommandOptionError(value)
    }
}

// TODO remove
impl From<String> for CompileError {
    fn from(value: String) -> Self {
        CompileError::Other(value)
    }
}
