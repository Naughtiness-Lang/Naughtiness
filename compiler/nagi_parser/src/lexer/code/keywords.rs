use std::str::FromStr;

use crate::errors::TokenStreamParseError;

#[derive(Debug)]
pub(crate) enum NagiCodeKeyword {
    Fn,
    Let,
    Ref,
    Mut,
    Const,
    Loop,
    For,
    While,
    If,
    Else,
    In,
    Impl,
    Return,
    Break,
    Continue,
    Struct,
    Union,
    Enum,
    Pub,
    Type,
    Match,
    Static,
    Extern,
}

impl FromStr for NagiCodeKeyword {
    type Err = TokenStreamParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let keyword = match s {
            "let" => NagiCodeKeyword::Let,
            "ref" => NagiCodeKeyword::Ref,
            "mut" => NagiCodeKeyword::Mut,
            "pub" => NagiCodeKeyword::Pub,
            "fn" => NagiCodeKeyword::Fn,
            "struct" => NagiCodeKeyword::Struct,
            "enum" => NagiCodeKeyword::Enum,
            "union" => NagiCodeKeyword::Union,
            "const" => NagiCodeKeyword::Const,
            "if" => NagiCodeKeyword::If,
            "else" => NagiCodeKeyword::Else,
            "match" => NagiCodeKeyword::Match,
            "in" => NagiCodeKeyword::In,
            "for" => NagiCodeKeyword::For,
            "while" => NagiCodeKeyword::While,
            "loop" => NagiCodeKeyword::Loop,
            "break" => NagiCodeKeyword::Break,
            "continue" => NagiCodeKeyword::Continue,
            "return" => NagiCodeKeyword::Return,
            "type" => NagiCodeKeyword::Type,
            "static" => NagiCodeKeyword::Static,
            "extern" => NagiCodeKeyword::Extern,
            "impl" => NagiCodeKeyword::Impl,
            _ => return Err(TokenStreamParseError::NotKeyword),
        };

        Ok(keyword)
    }
}
