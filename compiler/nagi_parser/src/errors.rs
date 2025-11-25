use nagi_lexer::token::TokenKind;

#[derive(Debug)]
pub struct ParserError {
    pub message: String,
}

#[derive(Debug)]
pub(crate) enum ParserErrorKind {}

#[derive(Debug)]
pub enum TokenizeError<'a> {
    UnexpectedToken {
        expect_token: TokenKind<'a>,
        unexpected_token: TokenKind<'a>,
        position: usize,
    },
    UnmatchToken {
        position: usize,
    },
    UnexpectedEOF,
    UnusableCharacters {
        position: usize,
    },
    CannotConvertTextToNumbers,
}
