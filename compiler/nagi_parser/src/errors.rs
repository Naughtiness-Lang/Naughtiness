use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ParserError {
    TokenStreamParse(TokenStreamParseError),
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::TokenStreamParse(e) => e.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum TokenStreamParseError {
    UnexpectedToken { position: usize },
    UnmatchedToken { position: usize },
    UnexpectedEOF,
    UnusableCharacters { position: usize },
    CannotConvertTextToNumbers { position: usize },
    NotKeyword,
}

impl Display for TokenStreamParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenStreamParseError::UnexpectedToken { position } => {
                write!(f, "Unexpected token at position {position}")
            }
            TokenStreamParseError::UnmatchedToken { position } => {
                write!(f, "Unmatched token at position {position}")
            }
            TokenStreamParseError::UnexpectedEOF => write!(f, "Unexpected end of file"),
            TokenStreamParseError::UnusableCharacters { position } => {
                write!(f, "Unusable characters at position {position}")
            }
            TokenStreamParseError::CannotConvertTextToNumbers { position } => {
                write!(f, "Cannot convert text to numbers. position: {position}")
            }
            TokenStreamParseError::NotKeyword => write!(f, "Not a keyword"),
        }
    }
}
