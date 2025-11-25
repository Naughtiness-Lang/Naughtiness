#[derive(Debug)]
pub struct ParserError {
    pub message: String,
}

#[derive(Debug)]
pub(crate) enum ParserErrorKind {}

#[derive(Debug)]
pub enum TokenStreamParseError {
    UnexpectedToken { position: usize },
    UnmatchToken { position: usize },
    UnexpectedEOF,
    UnusableCharacters { position: usize },
    CannotConvertTextToNumbers,
    NotKeyword,
}
