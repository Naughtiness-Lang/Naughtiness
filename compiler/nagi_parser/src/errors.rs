use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParserError {
    TokenStreamParse(TokenStreamParseError),
    PackratError(PackratError),
    EBNFParseError(EBNFParseError),
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::TokenStreamParse(e) => e.fmt(f),
            ParserError::PackratError(e) => e.fmt(f),
            ParserError::EBNFParseError(e) => e.fmt(f),
        }
    }
}

impl From<TokenStreamParseError> for ParserError {
    fn from(value: TokenStreamParseError) -> Self {
        ParserError::TokenStreamParse(value)
    }
}

impl From<PackratError> for ParserError {
    fn from(value: PackratError) -> Self {
        ParserError::PackratError(value)
    }
}

impl From<EBNFParseError> for ParserError {
    fn from(value: EBNFParseError) -> Self {
        ParserError::EBNFParseError(value)
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

impl fmt::Display for TokenStreamParseError {
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

#[derive(Debug)]
pub enum PackratError {
    InvalidState,
    UnknownRule(String),
    UnexpectedNode,
    UnexpectedEOF,
}

impl fmt::Display for PackratError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug)]
pub enum EBNFParseError {
    UnexpectedToken {
        expect_token: char,
        unexpected_token: String,
        position: usize,
    },
    UnmatchToken {
        current_token: String,
        position: usize,
    },
    UnexpectedEOF,
    ParseIntError {
        position: usize,
    },
    ParseDefineError {
        position: usize,
    },
    ParseExpansionError {
        position: usize,
    },
}

impl fmt::Display for EBNFParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
