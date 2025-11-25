use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
    kind: ParserErrorKind,
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug)]
pub(crate) enum ParserErrorKind {
    TokenStreamParse(TokenStreamParseError),
}

#[derive(Debug)]
pub enum TokenStreamParseError {
    UnexpectedToken { position: usize },
    UnmatchedToken { position: usize },
    UnexpectedEOF,
    UnusableCharacters { position: usize },
    CannotConvertTextToNumbers,
    NotKeyword,
}
