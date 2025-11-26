use std::{error::Error, fmt::Display, io};

use nagi_command_option::errors::CommandOptionError;
use nagi_lexer::errors::TokenizeError;
use nagi_parser::errors::ParserError;

#[derive(Debug)]
pub(crate) enum CompileError {
    IO(io::Error),
    WalkDir(walkdir::Error),
    CommandOption(CommandOptionError),
    TokenizeError(TokenizeError),
    ParserError(ParserError),
}

impl Error for CompileError {}

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::IO(e) => write!(f, "{e}"),
            CompileError::WalkDir(e) => write!(f, "{e}"),
            CompileError::CommandOption(e) => write!(f, "{}", e.message),
            CompileError::TokenizeError(e) => write!(f, "{e}"),
            CompileError::ParserError(e) => write!(f, "{e}"),
        }
    }
}

impl From<io::Error> for CompileError {
    fn from(value: io::Error) -> Self {
        CompileError::IO(value)
    }
}

impl From<CommandOptionError> for CompileError {
    fn from(value: CommandOptionError) -> Self {
        CompileError::CommandOption(value)
    }
}

impl From<walkdir::Error> for CompileError {
    fn from(value: walkdir::Error) -> Self {
        CompileError::WalkDir(value)
    }
}

impl From<TokenizeError> for CompileError {
    fn from(value: TokenizeError) -> Self {
        CompileError::TokenizeError(value)
    }
}

impl From<ParserError> for CompileError {
    fn from(value: ParserError) -> Self {
        CompileError::ParserError(value)
    }
}
