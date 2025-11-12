use std::iter::from_fn;
use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};
use token::{Symbol, Token, TokenKind};

pub mod token;

type Iter<'a> = Peekable<Enumerate<Chars<'a>>>;

// 単純なトークンに切り分け, 切り分けた結果のトークン列を返す
// 識別子, 数字, 記号(1文字), ホワイトスペースの単純なトークンに切り分けるだけなので
// 浮動小数や_を含む識別子などは別で処理を行う必要がある
// 作りたい言語の仕様上パーサーを2つ書くのでここで固定のルールにすると,
// パーサー側で扱いにくくなるため一旦特定の文字の塊だけにして
// パーサーに渡す前にそのパーサーに適したトークンに変換する
pub fn tokenize(source_code: &str) -> Result<Vec<Token>, String> {
    let mut iter = source_code.chars().enumerate().peekable();
    let mut token_list = vec![];

    while let Some(&(_, c)) = iter.peek() {
        let token = if c.is_ascii_digit() {
            eat_number(&mut iter)? // 0-9で始まるものは数値として扱う
        } else if c.is_ascii_whitespace() {
            eat_whitespace(&mut iter)?
        } else if c.is_ascii_punctuation() {
            eat_symbol(&mut iter)? // ASCIIの記号
        } else if c.is_alphabetic() {
            eat_identifier(&mut iter)? // 日本語などを使用するのでasciiに限定しない
        } else {
            return Err(format!("Invalid characters were used: {c}"));
        };

        token_list.push(token);
    }

    Ok(token_list)
}

fn eat_identifier(iter: &mut Iter) -> Result<Token, String> {
    let Some(&(position, _)) = iter.peek() else {
        unreachable!();
    };

    let code = from_fn(|| iter.next_if(|c| c.1.is_alphabetic()))
        .map(|c| c.1)
        .collect();

    Ok(Token {
        token_kind: TokenKind::Identifier(code),
        token_pos: position,
    })
}

fn eat_number(iter: &mut Iter) -> Result<Token, String> {
    let Some(&(position, _)) = iter.peek() else {
        unreachable!();
    };

    let code = from_fn(|| iter.next_if(|c| c.1.is_ascii_digit()))
        .map(|c| c.1)
        .collect();

    Ok(Token {
        token_kind: TokenKind::Number(code),
        token_pos: position,
    })
}

fn eat_symbol(iter: &mut Iter) -> Result<Token, String> {
    let Some(&(position, c)) = iter.peek() else {
        unreachable!();
    };

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
        _ => unreachable!(),
    };

    iter.next();

    Ok(Token {
        token_kind: TokenKind::Symbol(symbol),
        token_pos: position,
    })
}

fn eat_whitespace(iter: &mut Iter) -> Result<Token, String> {
    let Some(&(position, _)) = iter.peek() else {
        unreachable!();
    };

    let code = from_fn(|| iter.next_if(|c| c.1.is_whitespace()))
        .map(|c| c.1)
        .collect();

    Ok(Token {
        token_kind: TokenKind::WhiteSpace(code),
        token_pos: position,
    })
}
