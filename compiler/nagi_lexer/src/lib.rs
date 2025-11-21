use std::iter::from_fn;
use std::iter::Peekable;
use std::str::CharIndices;
use token::{LineBreak, Space, Symbol, Token, TokenKind};

pub mod token;

type Iter<'a> = Peekable<CharIndices<'a>>;

// 単純なトークンに切り分け, 切り分けた結果のトークン列を返す
// 識別子, 数字, 記号(1文字), ホワイトスペースの単純なトークンに切り分けるだけなので
// 浮動小数や_を含む識別子などは別で処理を行う必要がある
// 作りたい言語の仕様上パーサーを2つ書くのでここで固定のルールにすると,
// パーサー側で扱いにくくなるため一旦特定の文字の塊だけにして
// パーサーに渡す前にそのパーサーに適したトークンに変換する
pub fn tokenize<'a>(source_code: &'a str) -> Result<Vec<Token<'a>>, String> {
    let mut iter = source_code.char_indices().peekable();
    let mut token_list = vec![];

    while let Some(&(_, c)) = iter.peek() {
        let token = if c.is_ascii_digit() {
            eat_number(source_code, &mut iter)? // 0-9で始まるものは数値として扱う
        } else if c.is_ascii_whitespace() {
            eat_whitespace(&mut iter)?
        } else if c.is_ascii_punctuation() {
            eat_symbol(&mut iter)? // ASCIIの記号
        } else if c.is_alphabetic() {
            eat_identifier(source_code, &mut iter)? // 日本語などを使用するのでasciiに限定しない
        } else {
            return Err(format!("Invalid characters were used: {c}"));
        };

        token_list.push(token);
    }

    Ok(token_list)
}

fn eat_identifier<'a>(source_code: &'a str, iter: &mut Iter) -> Result<Token<'a>, String> {
    let Some(&(position, _)) = iter.peek() else {
        unreachable!();
    };

    let code = slice_code(source_code, iter, |c| c.is_alphabetic())?;

    Ok(Token {
        token_kind: TokenKind::Identifier(code),
        token_pos: position,
    })
}

fn eat_number<'a>(source_code: &'a str, iter: &mut Iter) -> Result<Token<'a>, String> {
    let Some(&(position, _)) = iter.peek() else {
        unreachable!();
    };

    let code = slice_code(source_code, iter, |c| c.is_ascii_digit())?;

    Ok(Token {
        token_kind: TokenKind::Number(code),
        token_pos: position,
    })
}

fn eat_symbol<'a>(iter: &mut Iter) -> Result<Token<'a>, String> {
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

fn eat_whitespace<'a>(iter: &mut Iter) -> Result<Token<'a>, String> {
    let Some(&(_, c)) = iter.peek() else {
        unreachable!();
    };

    match c {
        ' ' | '\t' => eat_space(iter),
        '\r' | '\n' => eat_line_break(iter),
        _ => Err(format!("Unusable whitespace: {c}")),
    }
}

fn eat_space<'a>(iter: &mut Iter) -> Result<Token<'a>, String> {
    let Some(&(position, _)) = iter.peek() else {
        unreachable!();
    };

    let code = from_fn(|| iter.next_if(|c| matches!(c.1, ' ' | '\t')))
        .map(|c| match c.1 {
            ' ' => Space::Space,
            '\t' => Space::Tab,
            _ => unreachable!(),
        })
        .collect();

    Ok(Token {
        token_kind: TokenKind::WhiteSpace(code),
        token_pos: position,
    })
}

fn eat_line_break<'a>(iter: &mut Iter) -> Result<Token<'a>, String> {
    let Some(&(position, _)) = iter.peek() else {
        unreachable!();
    };

    let code = from_fn(|| iter.next_if(|c| matches!(c.1, '\n' | '\r')))
        .map(|c| match c.1 {
            '\r' => LineBreak::CR,
            '\n' => LineBreak::LF,
            _ => unreachable!(),
        })
        .collect();

    Ok(Token {
        token_kind: TokenKind::LineBreak(code),
        token_pos: position,
    })
}

fn slice_code<'a>(
    source_code: &'a str,
    iter: &mut Iter,
    condition: impl Fn(char) -> bool,
) -> Result<&'a str, String> {
    let Some(&(start, _)) = iter.peek() else {
        unreachable!();
    };

    let _ = from_fn(|| iter.next_if(|&(_, c)| condition(c))).count(); // count()によりイテレータを消費
    let end = iter.peek().map(|e| e.0).unwrap_or(source_code.len());

    if start == end {
        unreachable!("slice_code was called without a matching character");
    }

    Ok(&source_code[start..end])
}
