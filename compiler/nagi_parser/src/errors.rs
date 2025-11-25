#[derive(Debug)]
pub struct ParserError {
    pub message: String,
    kind: ParserErrorKind,
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
