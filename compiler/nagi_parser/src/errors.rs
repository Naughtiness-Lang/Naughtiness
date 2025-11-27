use std::error::Error;
use std::fmt::{self, write};

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
            ParserError::TokenStreamParse(e) => write!(f, "{e:?}"),
            ParserError::PackratError(e) => write!(f, "{e:?}"),
            ParserError::EBNFParseError(e) => write!(f, "{e:?}"),
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
    NotOperator,
    NotSymbol,
    NotLiteral,
    MutexLockError,
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
            TokenStreamParseError::NotSymbol => write!(f, "Not a symbol"),
            TokenStreamParseError::NotLiteral => write!(f, "Not a literal"),
            TokenStreamParseError::NotOperator => write!(f, "Not a operator"),
            TokenStreamParseError::MutexLockError => write!(f, "Can not lock"),
        }
    }
}

// パーサー自体のエラー
#[derive(Debug)]
pub enum PackratError {
    InvalidState,
    UnknownRule(String),
    UnexpectedNode,
    UnexpectedEOF,
    FailedConstructAST,
    FailedParseRule(EBNFParseError),
}

impl fmt::Display for PackratError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match slef {}
    }
}

impl From<EBNFParseError> for PackratError {
    fn from(value: EBNFParseError) -> Self {
        PackratError::FailedParseRule(value)
    }
}

// 解析中のエラー
#[derive(Debug)]
pub enum ParsingError {
    UnexpectedToken, //
    UnexpectedLiteral,
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
        match self {
            EBNFParseError::UnexpectedToken {
                expect_token,
                unexpected_token,
                position,
            } => {
                write!(f, "except token: {expect_token} unexpected token: {unexpected_token} position: {position}")
            }
            EBNFParseError::UnexpectedEOF => write!(f, "unexpected EOF"),
            EBNFParseError::UnmatchToken {
                current_token,
                position,
            } => write!(f, "token: {current_token} position: {position}"),
            EBNFParseError::ParseIntError { position } => write!(f, "position: {position}"),
            EBNFParseError::ParseExpansionError { position } => write!(f, "position: {position}"),
            EBNFParseError::ParseDefineError { position } => write!(f, "position: {position}"),
        }
    }
}
