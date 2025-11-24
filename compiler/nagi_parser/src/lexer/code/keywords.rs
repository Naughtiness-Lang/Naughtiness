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

impl NagiCodeKeyword {
    pub fn from(token: &str) -> Option<NagiCodeKeyword> {
        let keyword = match token {
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
            _ => return None,
        };

        Some(keyword)
    }
}
