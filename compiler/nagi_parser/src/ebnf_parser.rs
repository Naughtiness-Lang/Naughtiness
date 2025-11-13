use std::iter::{from_fn, Enumerate};
use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub(crate) struct EBNF {
    pub name: String,   // 定義したルール名
    pub expr: EBNFNode, // ツリー構造(ルールの中身)
}

#[derive(Debug)]
pub(crate) enum EBNFNode {
    Expansion(String),     // Hoge
    Concat(Vec<EBNFNode>), // Hoge Fuga
    Or(Vec<EBNFNode>),     // Hoge | Fuga
    // Hoge? Hoge* Hoge+ Hoge{3} Hoge{7,} Hoge{2, 5}
    Repeat {
        node: Box<EBNFNode>,
        min: u64,
        max: Option<u64>,
    },
    Group(Box<EBNFNode>), // (Hoge)
    Literal(String),      // "hogefuga"
}

#[derive(Debug)]
enum Quantifier {
    Question,                 // ?
    Plus,                     // +
    Star,                     // *
    Braces(u64, Option<u64>), // {
}

type ParserIterator<'a> = Peekable<Enumerate<Chars<'a>>>;

const EOF: &str = "EOF";

pub fn parse_ebnf(ebnf: &str) -> Result<EBNF, String> {
    let mut iter = ebnf.chars().enumerate().peekable();

    parse_define(&mut iter).map_err(|e| e.error_message(ebnf))
}

fn parse_define(iter: &mut ParserIterator) -> Result<EBNF, EBNFParseError> {
    skip_space(iter);
    let name = from_fn(|| iter.next_if(|c| c.1.is_alphabetic()))
        .map(|c| c.1)
        .collect::<String>();

    skip_space(iter);
    for expected_char in "::=".chars() {
        if iter.next_if(|c| c.1 == expected_char).is_none() {
            return Err(EBNFParseError::UnexpectedToken {
                expect_token: ':',
                unexpected_token: get_token(iter),
                position: get_position(iter),
            });
        }
    }

    let expr = parse_expression(iter)?;

    Ok(EBNF { name, expr })
}

// Expression ::= Or ;
fn parse_expression(iter: &mut ParserIterator) -> Result<EBNFNode, EBNFParseError> {
    skip_space(iter);
    parse_or(iter)
}

// Or ::= Concat { "|" Concat } ;
fn parse_or(iter: &mut ParserIterator) -> Result<EBNFNode, EBNFParseError> {
    // Concat
    skip_space(iter);
    let left = parse_concat(iter)?;

    //
    skip_space(iter);
    let mut right = vec![];
    while iter.next_if(|c| matches!(c.1, '|')).is_some() {
        right.push(parse_concat(iter)?);
        skip_space(iter);
    }

    if right.is_empty() {
        return Ok(left);
    }

    right.insert(0, left);
    Ok(EBNFNode::Or(right))
}

// Concat ::= Repeat { Repeat } ;
fn parse_concat(iter: &mut ParserIterator) -> Result<EBNFNode, EBNFParseError> {
    let left = parse_repeat(iter)?;
    let mut vec = vec![];

    while let Ok(node) = parse_repeat(iter) {
        vec.push(node);
    }

    if vec.is_empty() {
        return Ok(left);
    }
    vec.insert(0, left);

    Ok(EBNFNode::Concat(vec))
}

// Repeat ::= Primary [ Quantifier ] ;
fn parse_repeat(iter: &mut ParserIterator) -> Result<EBNFNode, EBNFParseError> {
    let node = parse_primary(iter)?;

    if let Ok(quantifier) = parse_quantifier(iter) {
        let node = Box::new(node);
        return Ok(match quantifier {
            Quantifier::Plus => EBNFNode::Repeat {
                node,
                min: 1,
                max: None,
            },
            Quantifier::Star => EBNFNode::Repeat {
                node,
                min: 0,
                max: None,
            },
            Quantifier::Question => EBNFNode::Repeat {
                node,
                min: 0,
                max: Some(1),
            },
            Quantifier::Braces(min, max) => EBNFNode::Repeat { node, min, max },
        });
    }

    Ok(node)
}

// Quantifier ::= "?" | "*" | "+" | "{" Integer [ "," [ Integer ] ] "}" ;
fn parse_quantifier(iter: &mut ParserIterator) -> Result<Quantifier, EBNFParseError> {
    skip_space(iter);
    let t = iter.peek().ok_or(EBNFParseError::UnexpectedEOF)?;
    let res = match t.1 {
        '?' => Quantifier::Question,
        '*' => Quantifier::Star,
        '+' => Quantifier::Plus,
        '{' => {
            iter.next();

            let start = parse_integer(iter)?;
            let mut end = None;

            skip_space(iter);
            if iter.next_if(|t| matches!(t.1, ',')).is_some() {
                end = parse_integer(iter).ok();
            }

            skip_space(iter);
            if iter.next_if(|t| matches!(t.1, '}')).is_none() {
                return Err(EBNFParseError::UnexpectedToken {
                    expect_token: '}',
                    unexpected_token: get_token(iter),
                    position: get_position(iter),
                });
            }

            Quantifier::Braces(start, end)
        }

        _ => {
            return Err(EBNFParseError::UnmatchToken {
                current_token: get_token(iter),
                position: get_position(iter),
            })
        }
    };

    if matches!(
        res,
        Quantifier::Plus | Quantifier::Star | Quantifier::Question
    ) {
        iter.next();
    }

    Ok(res)
}

// Primary ::= Literal | Group | Expansion;
fn parse_primary(iter: &mut ParserIterator) -> Result<EBNFNode, EBNFParseError> {
    if let Ok(node) = parse_literal(iter) {
        return Ok(node);
    }

    if let Ok(node) = parse_group(iter) {
        return Ok(node);
    }

    if let Ok(node) = parse_expansion(iter) {
        return Ok(node);
    }

    Err(EBNFParseError::UnmatchToken {
        current_token: get_token(iter),
        position: get_position(iter),
    })
}

// Group ::= "(" Or ")" ;
fn parse_group(iter: &mut ParserIterator) -> Result<EBNFNode, EBNFParseError> {
    skip_space(iter);
    if iter.next_if(|c| matches!(c.1, '(')).is_none() {
        return Err(EBNFParseError::UnexpectedToken {
            expect_token: '(',
            unexpected_token: get_token(iter),
            position: get_position(iter),
        });
    }

    let node = parse_or(iter)?;

    skip_space(iter);
    if iter.next_if(|c| matches!(c.1, ')')).is_none() {
        return Err(EBNFParseError::UnexpectedToken {
            expect_token: ')',
            unexpected_token: get_token(iter),
            position: get_position(iter),
        });
    }

    Ok(EBNFNode::Group(Box::new(node)))
}

fn parse_expansion(iter: &mut ParserIterator) -> Result<EBNFNode, EBNFParseError> {
    let name = from_fn(|| iter.next_if(|c| c.1.is_alphabetic()))
        .map(|c| c.1)
        .collect::<String>();

    if name.is_empty() {
        return Err(EBNFParseError::ParseExpansionError {
            position: get_position(iter),
        });
    }

    Ok(EBNFNode::Expansion(name))
}

// Literal ::= "\"" { any-char-except-quote } "\"" ;
fn parse_literal(iter: &mut ParserIterator) -> Result<EBNFNode, EBNFParseError> {
    skip_space(iter);
    if iter.next_if(|c| matches!(c.1, '"')).is_none() {
        return Err(EBNFParseError::UnexpectedToken {
            expect_token: '"',
            unexpected_token: get_token(iter),
            position: get_position(iter),
        });
    }

    skip_space(iter);
    let literal = from_fn(|| iter.next_if(|c| !matches!(c.1, '"')))
        .map(|c| c.1)
        .collect();

    skip_space(iter);
    if iter.next_if(|c| matches!(c.1, '"')).is_none() {
        return Err(EBNFParseError::UnexpectedToken {
            expect_token: '"',
            unexpected_token: get_token(iter),
            position: get_position(iter),
        });
    }

    Ok(EBNFNode::Literal(literal))
}

// Integer ::= Digit { Digit } ;
// Digit ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
fn parse_integer(iter: &mut ParserIterator) -> Result<u64, EBNFParseError> {
    skip_space(iter);
    let token = from_fn(|| iter.next_if(|c| c.1.is_ascii_digit()))
        .map(|c| c.1)
        .collect::<String>();

    if token.is_empty() {
        return Err(EBNFParseError::UnmatchToken {
            current_token: get_token(iter),
            position: get_position(iter),
        });
    }

    token.parse().map_err(|_| EBNFParseError::ParseIntError {
        position: get_position(iter),
    })
}

fn skip_space(iter: &mut ParserIterator) {
    while iter.next_if(|c| c.1.is_whitespace()).is_some() {}
}

fn get_token(iter: &mut ParserIterator) -> String {
    iter.peek().map_or(EOF.to_string(), |c| c.1.to_string())
}

fn get_position(iter: &mut ParserIterator) -> usize {
    iter.peek().map_or(0, |c| c.0)
}

enum EBNFParseError {
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
    ParseExpansionError {
        position: usize,
    },
}

impl EBNFParseError {
    fn error_message(&self, rule: &str) -> String {
        match self {
            EBNFParseError::UnexpectedToken {
                expect_token,
                unexpected_token,
                position,
            } => [
                format!("unexpected token: {unexpected_token}"),
                rule.to_string(),
                format!("{}^", " ".repeat(*position)),
                format!("expect token: {expect_token}"),
            ]
            .join("\n"),

            EBNFParseError::UnmatchToken {
                current_token,
                position,
            } => {
                let position = if current_token == EOF {
                    rule.len()
                } else {
                    *position
                };

                [
                    format!("unmatch token: {current_token}"),
                    rule.to_string(),
                    format!("{}^", " ".repeat(position)),
                ]
                .join("\n")
            }

            EBNFParseError::UnexpectedEOF => "unexpected EOF".to_string(),
            EBNFParseError::ParseIntError { position } => [
                "can not parse integer".to_string(),
                rule.to_string(),
                format!("{}^", " ".repeat(*position)),
            ]
            .join("\n"),

            EBNFParseError::ParseExpansionError { position } => [
                "can not parse expansion".to_string(),
                rule.to_string(),
                format!("{}^", " ".repeat(*position)),
            ]
            .join("\n"),
        }
    }
}
