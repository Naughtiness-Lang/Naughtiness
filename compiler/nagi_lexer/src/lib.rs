use std::iter::from_fn;
use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};
use token::{Symbol, Token, TokenKind};

pub mod token;

type Iter<'a> = Peekable<Enumerate<Chars<'a>>>;

// 単純なトークンに切り分け, 切り分けた結果のトークン列を返す
// 識別子, 数字, 記号, ホワイトスペースの単純なトークンに切り分けるだけなので
// 浮動小数や_を含む識別子などは別で処理を行う必要がある
pub fn tokenize(source_code: &str) -> Result<Vec<Token>, String> {
    let mut iter = source_code.chars().enumerate().peekable();
    let mut token_list = vec![];

    while let Some((_, c)) = iter.peek() {
        let token = match c {
            _ if c.is_ascii_digit() => eat_number(&mut iter)?, // 0-9で始まるものは数値として扱う
            _ if c.is_ascii_whitespace() => eat_whitespace(&mut iter)?, //
            _ if c.is_ascii_punctuation() => eat_symbol(&mut iter)?, // ASCIIの記号
            _ if c.is_alphabetic() => eat_identifier(&mut iter)?, // 上記の条件に当てはまらない文字を識別子とする
            _ => return Err(format!("Invalid characters were used: {c}")),
        };

        token_list.push(token);
    }

    Ok(token_list)
}

fn eat_identifier(iter: &mut Iter) -> Result<Token, String> {
    let Some((position, _)) = iter.peek() else {
        return Err("Implementation error".to_string());
    };
    let position = *position;

    let code = from_fn(|| iter.next_if(|c| c.1.is_alphabetic()))
        .map(|c| c.1)
        .collect();

    Ok(Token {
        token_kind: TokenKind::Identifier(code),
        token_pos: position,
    })
}

fn eat_number(iter: &mut Iter) -> Result<Token, String> {
    let Some((position, _)) = iter.peek() else {
        return Err("Implementation error".to_string());
    };
    let position = *position;

    let code = from_fn(|| iter.next_if(|c| c.1.is_ascii_digit()))
        .map(|c| c.1)
        .collect();

    Ok(Token {
        token_kind: TokenKind::Number(code),
        token_pos: position,
    })
}

fn eat_symbol(iter: &mut Iter) -> Result<Token, String> {
    let Some((position, c)) = iter.peek() else {
        return Err("Implementation error".to_string());
    };
    let position = *position;

    let symbol = match c {
        '+' => Symbol::Plus,
        '-' => Symbol::Minus,
        '*' => Symbol::Star,
        '/' => Symbol::Slash,
        '%' => Symbol::Percent,
        '=' => Symbol::Equal,
        '^' => Symbol::Caret,
        '!' => Symbol::Not,
        '&' => Symbol::And,
        '|' => Symbol::Or,
        '>' => Symbol::GreaterThan,
        '<' => Symbol::LessThan,
        '@' => Symbol::At,
        '.' => Symbol::Dot,
        ',' => Symbol::Comma,
        ':' => Symbol::Colon,
        ';' => Symbol::Semicolon,
        '#' => Symbol::Pound,
        '$' => Symbol::Dollar,
        '?' => Symbol::Question,
        '~' => Symbol::Tilde,
        '(' => Symbol::LeftParenthesis,
        ')' => Symbol::RightParenthesis,
        '[' => Symbol::LeftBrackets,
        ']' => Symbol::RightBrackets,
        '{' => Symbol::LeftBrace,
        '}' => Symbol::RightBrace,
        '\'' => Symbol::SingleQuotation,
        '"' => Symbol::DoubleQuotation,
        '\\' => Symbol::BackSlash,
        '_' => Symbol::Underscore,
        '`' => Symbol::Backtick,
        _ => return Err("Implementation error".to_string()),
    };

    iter.next();

    Ok(Token {
        token_kind: TokenKind::Symbol(symbol),
        token_pos: position,
    })
}

fn eat_whitespace(iter: &mut Iter) -> Result<Token, String> {
    let Some((position, _)) = iter.peek() else {
        return Err("Implementation error".to_string());
    };
    let position = *position;

    let code = from_fn(|| iter.next_if(|c| c.1.is_whitespace()))
        .map(|c| c.1)
        .collect();

    Ok(Token {
        token_kind: TokenKind::WhiteSpace(code),
        token_pos: position,
    })
}
