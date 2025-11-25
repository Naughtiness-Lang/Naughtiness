use super::{
    keywords::NagiCodeKeyword, operators::OPERATOR_PATTERN_MAP, symbols::SYMBOL_PATTERN_MAP,
};
use crate::{errors::TokenStreamParseError, lexer::Lexer};
use nagi_lexer::token::{Symbol, Token, TokenKind};
use std::{
    iter::{from_fn, Peekable},
    slice::Iter,
};

// コード ナギ自体のコード
#[derive(Debug)]
pub struct NagiProgramToken {
    pub token_kind: NagiProgramTokenKind,
    pub position: usize,
}

#[derive(Debug)]
pub enum NagiProgramTokenKind {
    Identifier(NagiIdentifier),
    Literal(NagiLiteral),
    Operator(NagiOperator),
    Symbol(NagiSymbol),
}

#[derive(Debug)]
pub enum NagiIdentifier {
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

#[derive(Debug, Clone)]
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

    BitwiseAnd,
    BitwiseOr,
    BitwiseNot,
    BitwiseXor,
    LeftShift,
    RightShift,

    Question,
    Dot,
}

#[derive(Debug, Clone)]
pub enum NagiSymbol {
    LeftParenthesis,  // (
    RightParenthesis, // )
    LeftBrackets,     // [
    RightBrackets,    // ]
    LeftBrace,        // {
    RightBrace,       // }
    Semicolon,
    Comma,
}

type ParseIter<'a> = Peekable<Iter<'a, Token<'a>>>;

pub fn tokenize_program(
    token_list: &[Token],
) -> Result<Lexer<NagiProgramToken>, TokenStreamParseError> {
    let mut iter = token_list.iter().peekable();
    let mut token_list = vec![];
    while iter.peek().is_some() {
        glue_comment(&mut iter); // 先にコメント処理

        let Some(token) = iter.peek() else {
            break;
        };

        let position = token.token_pos;
        let token_kind = &token.token_kind;

        match token_kind {
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
                token_list.push(NagiProgramToken {
                    token_kind: glue_symbol_or_operator(&mut iter)?,
                    position,
                });
            }
            TokenKind::WhiteSpace(_) | TokenKind::LineBreak(_) => {
                skip_white_space(&mut iter);
            }
        }
    }

    Ok(Lexer::new(token_list))
}

/// キーワードもしくは識別子に変換する
fn glue_identifier<'a>(
    iter: &mut ParseIter<'a>,
) -> Result<NagiProgramTokenKind, TokenStreamParseError> {
    expect_token(iter, |t| matches!(t.token_kind, TokenKind::Identifier(_)))?;

    let ident = glue_text_with_underscore(iter)?;

    if let Some(keyword) = NagiCodeKeyword::from(&ident) {
        return Ok(NagiProgramTokenKind::Identifier(NagiIdentifier::Keyword(
            keyword,
        )));
    }

    Ok(NagiProgramTokenKind::Identifier(
        NagiIdentifier::Identifier(ident),
    ))
}

fn glue_literal<'a>(
    iter: &mut ParseIter<'a>,
) -> Result<NagiProgramTokenKind, TokenStreamParseError> {
    let token = expect_token(iter, |t| matches!(t.token_kind, TokenKind::Number(_)))?;
    let TokenKind::Number(num) = token.token_kind else {
        unreachable!()
    };

    // 0始まり
    if matches!(num, "0") {
        return eat_literal_with_prefix(iter, true);
    }

    let value = eat_dec_literal(iter)?;

    if iter
        .next_if(|t| t.token_kind == TokenKind::Symbol(Symbol::Dot))
        .is_some()
    {
        return eat_float_literal(iter, value);
    }

    let suffix = eat_suffix(iter);

    Ok(NagiProgramTokenKind::Literal(NagiLiteral::Integer {
        signed: true,
        value,
        suffix,
    }))
}

fn glue_symbol_or_operator<'a>(
    iter: &mut ParseIter<'a>,
) -> Result<NagiProgramTokenKind, TokenStreamParseError> {
    let token = expect_token(iter, |t| matches!(t.token_kind, TokenKind::Symbol(_)))?;

    if let Ok(token_kind) = glue_symbol(iter) {
        return Ok(token_kind);
    }

    if let Ok(token_kind) = glue_operator(iter) {
        return Ok(token_kind);
    }

    Err(TokenStreamParseError::UnmatchToken {
        position: token.token_pos,
    })
}

fn glue_operator<'a>(
    iter: &mut ParseIter<'a>,
) -> Result<NagiProgramTokenKind, TokenStreamParseError> {
    let token = expect_token(iter, |t| matches!(t.token_kind, TokenKind::Symbol(_)))?;
    let TokenKind::Symbol(symbol) = &token.token_kind else {
        unreachable!()
    };

    let Some(patterns) = OPERATOR_PATTERN_MAP.get(symbol) else {
        return Err(TokenStreamParseError::UnmatchToken {
            position: token.token_pos,
        });
    };

    for (pattern, operator) in patterns {
        let result = match_token(iter, pattern, |token, symbol| {
            let TokenKind::Symbol(target_symbol) = &token.token_kind else {
                return false;
            };

            target_symbol == symbol
        });

        if !result {
            continue;
        }

        return Ok(NagiProgramTokenKind::Operator(operator.clone()));
    }

    Err(TokenStreamParseError::UnmatchToken {
        position: token.token_pos,
    })
}

fn glue_symbol<'a>(
    iter: &mut ParseIter<'a>,
) -> Result<NagiProgramTokenKind, TokenStreamParseError> {
    let token = expect_token(iter, |t| matches!(t.token_kind, TokenKind::Symbol(_)))?;
    let TokenKind::Symbol(symbol) = &token.token_kind else {
        unreachable!()
    };

    let Some(patterns) = SYMBOL_PATTERN_MAP.get(symbol) else {
        return Err(TokenStreamParseError::UnmatchToken {
            position: token.token_pos,
        });
    };

    for (pattern, symbol) in patterns {
        let result = match_token(iter, pattern, |token, symbol| {
            let TokenKind::Symbol(target_symbol) = &token.token_kind else {
                return false;
            };

            target_symbol == symbol
        });

        if !result {
            continue;
        }

        return Ok(NagiProgramTokenKind::Symbol(symbol.clone()));
    }

    Err(TokenStreamParseError::UnmatchToken {
        position: token.token_pos,
    })
}

fn glue_comment<'a>(iter: &mut ParseIter<'a>) {
    let result = match_token(iter, &[Symbol::Slash, Symbol::Slash], |token, symbol| {
        let TokenKind::Symbol(target_symbol) = &token.token_kind else {
            return false;
        };

        target_symbol == symbol
    });

    if !result {
        return;
    }

    // コメントは何もしない
    // 今は1行コメントのみの対応
    eat_line_comment(iter);
}

fn match_token<'a, T, F>(iter: &mut ParseIter<'a>, list: &[T], condition: F) -> bool
where
    F: Fn(&Token, &T) -> bool,
{
    let mut clone_iter = iter.clone();
    for element in list {
        let Some(token) = clone_iter.next() else {
            return false;
        };

        if !condition(token, element) {
            return false;
        }
    }

    for _ in 0..list.len() {
        iter.next();
    }

    true
}

/// 数値と文字列と_が続くトークンをすべて接着
fn glue_text_with_underscore<'a>(
    iter: &mut ParseIter<'a>,
) -> Result<String, TokenStreamParseError> {
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
fn eat_literal_with_prefix<'a>(
    iter: &mut ParseIter<'a>,
    signed: bool,
) -> Result<NagiProgramTokenKind, TokenStreamParseError> {
    expect_token(iter, |t| t.token_kind == TokenKind::Number("0"))?;
    iter.next(); // 0は確定しているので次のトークンへ

    // 次が終端かつ0のみの場合
    let Some(token) = iter.next() else {
        return Ok(NagiProgramTokenKind::Literal(NagiLiteral::Integer {
            signed,
            value: 0,
            suffix: None,
        }));
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

/// BIN_LITERAL ::= 0b ( BIN_DIGIT | "_" )*BIN_DIGIT ( BIN_DIGIT | "_" )*
/// BIN_DIGIT   ::= [0-1]
fn eat_bin_literal<'a>(iter: &mut ParseIter<'a>) -> Result<u64, TokenStreamParseError> {
    expect_token(iter, |t| {
        matches!(
            t.token_kind,
            TokenKind::Number(_) | TokenKind::Symbol(Symbol::Underscore)
        )
    })?;

    convert_to_number(iter, 2, |c| matches!(c, '0' | '1'))
}

/// OCT_LITERAL ::= 0o ( OCT_DIGIT | "_" )*OCT_DIGIT (OCT_DIGIT | "_" )*
/// OCT_DIGIT   ::= [0-7]
fn eat_oct_literal<'a>(iter: &mut ParseIter<'a>) -> Result<u64, TokenStreamParseError> {
    expect_token(iter, |t| {
        matches!(
            t.token_kind,
            TokenKind::Number(_) | TokenKind::Symbol(Symbol::Underscore)
        )
    })?;

    convert_to_number(iter, 8, |c| matches!(c, '0'..='7'))
}

/// DEC_LITERAL ::= DEC_DIGIT ( DEC_DIGIT | "_" )*
/// DEC_DIGIT   ::= [0-9]
fn eat_dec_literal<'a>(iter: &mut ParseIter<'a>) -> Result<u64, TokenStreamParseError> {
    expect_token(iter, |t| {
        matches!(
            t.token_kind,
            TokenKind::Number(_) | TokenKind::Symbol(Symbol::Underscore)
        )
    })?;

    convert_to_number(iter, 10, |c| c.is_ascii_digit())
}

/// HEX_LITERAL ::= 0x ( HEX_DIGIT | "_" )* HEX_DIGIT ( HEX_DIGIT | "_" )*
/// HEX_DIGIT   ::= [0-9a-fA-F]
///
/// 0x は解析済み前提
fn eat_hex_literal<'a>(iter: &mut ParseIter<'a>) -> Result<u64, TokenStreamParseError> {
    expect_token(iter, |t| {
        matches!(
            t.token_kind,
            TokenKind::Identifier(_) | TokenKind::Number(_) | TokenKind::Symbol(Symbol::Underscore)
        )
    })?;

    convert_to_number(iter, 16, |c| c.is_ascii_hexdigit())
}

/// FLOAT_LITERAL ::= DEC_LITERAL "."
///                 | DEC_LITERAL "." DEC_LITERAL
///
/// 先頭の DEC_LITERAL "." は解析済み前提
fn eat_float_literal<'a>(
    iter: &mut ParseIter<'a>,
    front_dec: u64,
) -> Result<NagiProgramTokenKind, TokenStreamParseError> {
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
        .map_err(|_| TokenStreamParseError::CannotConvertTextToNumbers)?;

    Ok(NagiProgramTokenKind::Literal(NagiLiteral::Float {
        value,
        suffix: eat_suffix(iter),
    }))
}

fn eat_suffix<'a>(iter: &mut ParseIter<'a>) -> Option<String> {
    iter.next_if(|t| matches!(t.token_kind, TokenKind::Identifier(_)))
        .map(|t| match t.token_kind {
            TokenKind::Identifier(ident) => ident.to_string(),
            _ => unreachable!(),
        })
}

fn eat_line_comment<'a>(iter: &mut ParseIter<'a>) {
    // 今はcountによるイテレータの消費のみ
    from_fn(|| iter.next_if(|t| !matches!(t.token_kind, TokenKind::LineBreak(_)))).count();
}

fn convert_to_number<'a>(
    iter: &mut ParseIter<'a>,
    radix: u32,
    condition: impl Fn(&char) -> bool,
) -> Result<u64, TokenStreamParseError> {
    let Some(token) = iter.peek() else {
        return Err(TokenStreamParseError::UnexpectedEOF);
    };
    let token_pos = token.token_pos;

    let num_text = glue_text_with_underscore(iter)?;
    if num_text.is_empty() {
        return Err(TokenStreamParseError::UnmatchToken {
            position: token_pos,
        });
    }

    for (pos, c) in num_text.char_indices() {
        if condition(&c) || matches!(c, '_') {
            continue;
        }

        return Err(TokenStreamParseError::UnusableCharacters {
            position: token_pos + pos,
        });
    }

    let src: String = num_text.chars().filter(condition).collect();
    u64::from_str_radix(&src, radix).map_err(|_| TokenStreamParseError::CannotConvertTextToNumbers)
}

fn expect_token<'a, F>(
    iter: &mut ParseIter<'a>,
    condition: F,
) -> Result<&'a Token<'a>, TokenStreamParseError>
where
    F: Fn(&Token) -> bool,
{
    let Some(token) = iter.peek() else {
        return Err(TokenStreamParseError::UnexpectedEOF);
    };

    if !condition(token) {
        return Err(TokenStreamParseError::UnexpectedToken {
            position: token.token_pos,
        });
    }

    Ok(*token)
}

fn skip_white_space<'a>(iter: &mut ParseIter<'a>) {
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
