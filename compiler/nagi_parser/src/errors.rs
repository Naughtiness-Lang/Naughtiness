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
pub(crate) enum TokenStreamParseError {
    UnexpectedToken { position: usize },
    UnmatchedToken { position: usize },
    UnexpectedEOF,
    UnusableCharacters { position: usize },
    CannotConvertTextToNumbers,
    NotKeyword,
}

impl Display for TokenStreamParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}") // エラー内容は別ブランチで対応する
    }
}
