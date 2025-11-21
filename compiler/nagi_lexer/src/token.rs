#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub token_kind: TokenKind,
    pub token_pos: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Identifier(String),
    Number(String),
    Symbol(Symbol),
    LineBreak(Vec<LineBreak>),
    WhiteSpace(Vec<Space>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LineBreak {
    CR,
    LF,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Space {
    Space,
    Tab,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    LeftParenthesis,  // (
    RightParenthesis, // )
    LeftBrackets,     // [
    RightBrackets,    // ]
    LeftBrace,        // {
    RightBrace,       // }

    Plus,            // +
    Minus,           // -
    Star,            // *
    Slash,           // /
    Percent,         // %
    Equal,           // =
    Caret,           // ^
    Not,             // !
    And,             // &
    Or,              // |
    GreaterThan,     //  >
    LessThan,        // <
    At,              // @
    Dot,             // .
    Comma,           // ,
    Colon,           // :
    Semicolon,       // ;
    Underscore,      // _
    Pound,           // #
    Dollar,          // $
    Question,        // ?
    Tilde,           // ~
    SingleQuotation, // '
    DoubleQuotation, // "
    BackSlash,       // \
    Backtick,        // `
}
