#[derive(Debug)]
pub struct ParserError {
    pub message: String,
}

#[derive(Debug)]
pub(crate) enum ParserErrorKind {}
