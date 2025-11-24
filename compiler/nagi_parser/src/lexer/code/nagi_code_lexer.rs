use nagi_lexer::token::*;
use std::{
    iter::{from_fn, Peekable},
    slice::Iter,
};

use super::keywords::NagiCodeKeyword;
use crate::lexer::Lexer;

// コード ナギ自体のコード
#[derive(Debug)]
pub struct NagiProgramToken {
    pub token_kind: NagiProgramTokenKind,
    pub position: usize,
}

#[derive(Debug)]
pub enum NagiProgramTokenKind {
    Identifier(NagiIdentifir),
    Literal(NagiLiteral),
    Operator(NagiOperator),
}

#[derive(Debug)]
pub enum NagiIdentifir {
    Identifier(String),
    Keyword(NagiCodeKeyword),
}

#[derive(Debug)]
pub enum NagiLiteral {
    Integer {
        signed: bool,
        value: u64,
        suffix: Option<String>,
    },
    Float {
        value: f64,
        suffix: Option<String>,
    },
    String,
}

#[derive(Debug)]
pub enum NagiOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    LeftShiftAssign,
    RightShiftAssign,

    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,

    And,
    Or,
    Not,
    Xor,

    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
    BitwiseXor,
    LeftShift,
    RightShift,
}

#[derive(Debug)]
pub enum TokenizeError {
    UnexpectedToken { position: usize },
    UnexpectedEOF,
    UnusableCharacters { position: usize },
    CannotConvertTextToNumbers,
}

pub fn tokenize_program(token_list: &[Token]) -> Result<Lexer<NagiProgramToken>, TokenizeError> {
    let mut iter = token_list.iter().peekable();
    let mut token_list = vec![];
    while let Some(token) = iter.peek() {
        let position = token.token_pos;
        match &token.token_kind {
            TokenKind::Identifier(_) => {
                token_list.push(NagiProgramToken {
                    token_kind: glue_identifier(&mut iter)?,
                    position,
                });
            }
            TokenKind::Number(_) => {
                token_list.push(NagiProgramToken {
                    token_kind: glue_literal(&mut iter)?,
                    position,
                });
            }
            TokenKind::Symbol(_) => {
                glue_operator(&mut iter);
            }
            TokenKind::WhiteSpace(_) | TokenKind::LineBreak(_) => {
                skip_white_space(&mut iter);
            }
        }
    }

    Ok(Lexer::new(token_list))
}

/// キーワードもしくは識別子に変換する
fn glue_identifier(
    iter: &mut Peekable<Iter<'_, Token>>,
) -> Result<NagiProgramTokenKind, TokenizeError> {
    let Some(Token {
        token_kind: TokenKind::Identifier(_),
        ..
    }) = iter.peek()
    else {
        unreachable!()
    };

    let ident = glue_text_with_underscore(iter)?;

    if let Some(keyword) = NagiCodeKeyword::from(&ident) {
        return Ok(NagiProgramTokenKind::Identifier(NagiIdentifir::Keyword(
            keyword,
        )));
    }

    Ok(NagiProgramTokenKind::Identifier(NagiIdentifir::Identifier(
        ident,
    )))
}

fn glue_literal(
    iter: &mut Peekable<Iter<'_, Token>>,
) -> Result<NagiProgramTokenKind, TokenizeError> {
    let Some(Token {
        token_kind: TokenKind::Number(num),
        ..
    }) = iter.peek()
    else {
        unreachable!()
    };

    // 0始まり
    if matches!(*num, "0") {
        return eat_literal_with_prefix(iter, true);
    }

    let front = eat_dec_literal(iter)?;

    Err(TokenizeError::UnexpectedEOF)
}

fn glue_operator(iter: &mut Peekable<Iter<'_, Token>>) {
    let Some(Token {
        token_kind: TokenKind::Symbol(symbol),
        ..
    }) = iter.peek()
    else {
        unreachable!()
    };
}

// 数値と文字列と_が続くトークンをすべて接着
fn glue_text_with_underscore(
    iter: &mut Peekable<Iter<'_, Token>>,
) -> Result<String, TokenizeError> {
    let num_text: String = from_fn(|| {
        iter.next_if(|t| {
            matches!(
                t.token_kind,
                TokenKind::Identifier(_)
                    | TokenKind::Number(_)
                    | TokenKind::Symbol(Symbol::Underscore)
            )
        })
    })
    .map(|t| match t.token_kind {
        TokenKind::Identifier(ident) => ident,
        TokenKind::Number(num) => num,
        TokenKind::Symbol(Symbol::Underscore) => "_",
        _ => unreachable!(),
    })
    .collect();

    Ok(num_text)
}

/// 0始まりのトークンを解析
fn eat_literal_with_prefix(
    iter: &mut Peekable<Iter<'_, Token>>,
    signed: bool,
) -> Result<NagiProgramTokenKind, TokenizeError> {
    let Some(Token {
        token_kind: TokenKind::Number("0"),
        ..
    }) = iter.next()
    else {
        unreachable!()
    };
    let Some(token) = iter.next() else {
        return Err(TokenizeError::UnexpectedEOF);
    };

    match &token.token_kind {
        // 0 のみの場合
        TokenKind::LineBreak(_) | TokenKind::WhiteSpace(_) => {
            Ok(NagiProgramTokenKind::Literal(NagiLiteral::Integer {
                signed,
                value: 0,
                suffix: None,
            }))
        }
        // 0と記号始まりの場合
        TokenKind::Symbol(symbol) => match symbol {
            Symbol::Dot => eat_float_literal(iter, 0),
            Symbol::Underscore => {
                let value = eat_dec_literal(iter)?;
                let suffix = eat_suffix(iter);
                Ok(NagiProgramTokenKind::Literal(NagiLiteral::Integer {
                    signed,
                    value,
                    suffix,
                }))
            }
            _ => Ok(NagiProgramTokenKind::Literal(NagiLiteral::Integer {
                signed,
                value: 0,
                suffix: None,
            })),
        },
        TokenKind::Identifier(ident) => {
            let value = match *ident {
                "b" => eat_bin_literal(iter)?,
                "o" => eat_oct_literal(iter)?,
                "x" => eat_hex_literal(iter)?,
                _ => {
                    return Ok(NagiProgramTokenKind::Literal(NagiLiteral::Integer {
                        signed,
                        value: 0,
                        suffix: Some(ident.to_string()),
                    }))
                }
            };

            Ok(NagiProgramTokenKind::Literal(NagiLiteral::Integer {
                signed,
                value,
                suffix: eat_suffix(iter),
            }))
        }
        TokenKind::Number(_) => unreachable!(),
    }
}

/// BIN_LITERAL ::= 0b ( BIN_DIGIT | `_` )*BIN_DIGIT ( BIN_DIGIT | `_` )*
/// BIN_DIGIT   ::= [0-1]
fn eat_bin_literal(iter: &mut Peekable<Iter<'_, Token>>) -> Result<u64, TokenizeError> {
    let Some(token) = iter.peek() else {
        unreachable!()
    };
    if !matches!(
        token.token_kind,
        TokenKind::Number(_) | TokenKind::Symbol(Symbol::Underscore)
    ) {
        unreachable!()
    }

    convert_to_number(iter, 2, |c| matches!(c, '0' | '1'))
}

/// OCT_LITERAL ::= 0o ( OCT_DIGIT | `_` )*OCT_DIGIT (OCT_DIGIT | `_` )*
/// OCT_DIGIT   ::= [0-7]
fn eat_oct_literal(iter: &mut Peekable<Iter<'_, Token>>) -> Result<u64, TokenizeError> {
    let Some(token) = iter.peek() else {
        unreachable!()
    };
    if !matches!(
        token.token_kind,
        TokenKind::Number(_) | TokenKind::Symbol(Symbol::Underscore)
    ) {
        unreachable!()
    }

    convert_to_number(iter, 8, |c| matches!(c, '0'..='7'))
}

/// DEC_LITERAL ::= DEC_DIGIT ( DEC_DIGIT | `_` )*
/// DEC_DIGIT   ::= [0-9]
fn eat_dec_literal(iter: &mut Peekable<Iter<'_, Token>>) -> Result<u64, TokenizeError> {
    let Some(token) = iter.peek() else {
        unreachable!()
    };
    if !matches!(token.token_kind, TokenKind::Number(_)) {
        unreachable!()
    }

    convert_to_number(iter, 10, |c| c.is_ascii_digit())
}

/// HEX_LITERAL ::= 0x ( HEX_DIGIT | "_" )* HEX_DIGIT ( HEX_DIGIT | "_" )*
/// HEX_DIGIT   ::= [0-9a-fA-F]
///
/// 0x は解析済み前提
fn eat_hex_literal(iter: &mut Peekable<Iter<'_, Token>>) -> Result<u64, TokenizeError> {
    let Some(token) = iter.peek() else {
        unreachable!()
    };
    if !matches!(
        token.token_kind,
        TokenKind::Number(_) | TokenKind::Symbol(Symbol::Underscore)
    ) {
        unreachable!()
    }

    convert_to_number(iter, 16, |c| c.is_ascii_hexdigit())
}

/// FLOAT_LITERAL ::= DEC_LITERAL `.`
///                 | DEC_LITERAL `.` DEC_LITERAL
///
/// 先頭の DEC_LITERAL . は解析済み前提
fn eat_float_literal(
    iter: &mut Peekable<Iter<'_, Token>>,
    front_dec: i64,
) -> Result<NagiProgramTokenKind, TokenizeError> {
    let Some(token) = iter.next() else {
        return Ok(NagiProgramTokenKind::Literal(NagiLiteral::Float {
            value: front_dec as f64,
            suffix: None,
        }));
    };

    let TokenKind::Number(_) = token.token_kind else {
        return Ok(NagiProgramTokenKind::Literal(NagiLiteral::Float {
            value: front_dec as f64,
            suffix: None,
        }));
    };

    let value = format!("{front_dec}.{}", eat_dec_literal(iter)?)
        .parse()
        .map_err(|_| TokenizeError::CannotConvertTextToNumbers)?;

    Ok(NagiProgramTokenKind::Literal(NagiLiteral::Float {
        value,
        suffix: None,
    }))
}

fn eat_suffix(iter: &mut Peekable<Iter<'_, Token>>) -> Option<String> {
    iter.next_if(|t| matches!(t.token_kind, TokenKind::Identifier(_)))
        .map(|t| match t.token_kind {
            TokenKind::Identifier(ident) => ident.to_string(),
            _ => unreachable!(),
        })
}

fn eat_comment(iter: &mut Peekable<Iter<'_, Token>>) {}

fn convert_to_number(
    iter: &mut Peekable<Iter<'_, Token>>,
    radix: u32,
    condition: impl Fn(&char) -> bool,
) -> Result<u64, TokenizeError> {
    let Some(token) = iter.peek() else {
        return Err(TokenizeError::UnexpectedEOF);
    };
    let token_pos = token.token_pos;

    let num_text = glue_text_with_underscore(iter)?;
    if num_text.is_empty() {
        unreachable!();
    }

    for (pos, c) in num_text.char_indices() {
        if !condition(&c) || !matches!(c, '_') {
            return Err(TokenizeError::UnusableCharacters {
                position: token_pos + pos,
            });
        }
    }

    let src: String = num_text.chars().filter(condition).collect();
    u64::from_str_radix(&src, radix).map_err(|_| TokenizeError::CannotConvertTextToNumbers)
}

fn skip_white_space(iter: &mut Peekable<Iter<'_, Token>>) {
    while iter
        .next_if(|t| {
            matches!(
                t.token_kind,
                TokenKind::LineBreak(_) | TokenKind::WhiteSpace(_)
            )
        })
        .is_some()
    {}
}
