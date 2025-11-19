use crate::ebnf::*;
use std::iter::from_fn;
use std::iter::Peekable;
use std::rc::Rc;
use std::str::CharIndices;

type ParserIterator<'a> = Peekable<CharIndices<'a>>;

const EOF: &str = "EOF";

pub fn parse_ebnf<'a>(source: &'a str) -> Result<EBNF<'a>, String> {
    let mut iter = source.char_indices().peekable();
    parse_define(source, &mut iter).map_err(|e| e.error_message(source))
}

fn parse_define<'a>(
    source: &'a str,
    iter: &mut ParserIterator,
) -> Result<EBNF<'a>, EBNFParseError> {
    skip_space(iter);
    let Some(c) = iter.peek() else {
        return Err(EBNFParseError::UnexpectedEOF);
    };
    if !c.1.is_alphabetic() {
        return Err(EBNFParseError::ParseDefineError {
            position: get_position(iter, source.len()),
        });
    }

    let name = parse_and_slice(source, iter, |c| c.is_alphabetic() || c.is_ascii_digit())?;

    skip_space(iter);
    for expected_char in "::=".chars() {
        if iter.next_if(|c| c.1 == expected_char).is_none() {
            return Err(EBNFParseError::UnexpectedToken {
                expect_token: expected_char,
                unexpected_token: get_token(iter),
                position: get_position(iter, source.len()),
            });
        }
    }

    let expr = parse_expression(source, iter)?;

    // ルールの穴により末端まで解析できなかった場合
    if iter.peek().is_some() {
        return Err(EBNFParseError::UnmatchToken {
            current_token: get_token(iter),
            position: get_position(iter, source.len()),
        });
    }

    Ok(EBNF::new(name, expr))
}

// Expression ::= Or ;
fn parse_expression<'a>(
    source: &'a str,
    iter: &mut ParserIterator,
) -> Result<EBNFNode<'a>, EBNFParseError> {
    skip_space(iter);
    parse_or(source, iter)
}

// Or ::= Concat { "|" Concat } ;
fn parse_or<'a>(
    source: &'a str,
    iter: &mut ParserIterator,
) -> Result<EBNFNode<'a>, EBNFParseError> {
    // Concat
    skip_space(iter);
    let mut nodes = vec![parse_concat(source, iter)?];

    //
    skip_space(iter);
    while iter.next_if(|c| matches!(c.1, '|')).is_some() {
        nodes.push(parse_concat(source, iter)?);
        skip_space(iter);
    }

    if nodes.len() == 1 {
        return Ok(nodes.pop().unwrap());
    }

    Ok(EBNFNode::Or(nodes.into_iter().map(Rc::new).collect()))
}

// Concat ::= Repeat { Repeat } ;
fn parse_concat<'a>(
    source: &'a str,
    iter: &mut ParserIterator,
) -> Result<EBNFNode<'a>, EBNFParseError> {
    let mut nodes = vec![parse_repeat(source, iter)?];

    loop {
        skip_space(iter);
        let Some((_, c)) = iter.peek() else {
            break;
        };

        if matches!(c, '"' | '(') || c.is_alphabetic() {
            nodes.push(parse_repeat(source, iter)?);
        } else {
            break;
        }
    }

    if nodes.len() == 1 {
        return Ok(nodes.pop().unwrap());
    }

    Ok(EBNFNode::Concat(nodes.into_iter().map(Rc::new).collect()))
}

// Repeat ::= Primary [ Quantifier ] ;
fn parse_repeat<'a>(
    source: &'a str,
    iter: &mut ParserIterator,
) -> Result<EBNFNode<'a>, EBNFParseError> {
    let node = parse_primary(source, iter)?;

    skip_space(iter);
    let Some(c) = iter.peek() else {
        return Ok(node);
    };

    if !matches!(c.1, '+' | '*' | '?' | '{') {
        return Ok(node);
    }

    let quantifier = parse_quantifier(iter, source)?;
    let node = Rc::new(node);
    Ok(match quantifier {
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
    })
}

// Quantifier ::= "?" | "*" | "+" | "{" Integer [ "," [ Integer ] ] "}" ;
fn parse_quantifier(iter: &mut ParserIterator, source: &str) -> Result<Quantifier, EBNFParseError> {
    skip_space(iter);
    let t = iter.peek().ok_or(EBNFParseError::UnexpectedEOF)?;
    let res = match t.1 {
        '?' => {
            iter.next();
            Quantifier::Question
        }
        '*' => {
            iter.next();
            Quantifier::Star
        }
        '+' => {
            iter.next();
            Quantifier::Plus
        }
        '{' => {
            iter.next();

            let start = parse_integer(iter, source)?;
            let mut end = Some(start);

            skip_space(iter);
            if iter.next_if(|t| matches!(t.1, ',')).is_some() {
                skip_space(iter);
                let Some((_, c)) = iter.peek() else {
                    return Err(EBNFParseError::UnexpectedEOF);
                };

                end = if c.is_ascii_digit() {
                    Some(parse_integer(iter, source)?)
                } else {
                    None
                };
            }

            skip_space(iter);
            if iter.next_if(|t| matches!(t.1, '}')).is_none() {
                return Err(EBNFParseError::UnexpectedToken {
                    expect_token: '}',
                    unexpected_token: get_token(iter),
                    position: get_position(iter, source.len()),
                });
            }

            Quantifier::Braces(start, end)
        }

        _ => {
            return Err(EBNFParseError::UnmatchToken {
                current_token: get_token(iter),
                position: get_position(iter, source.len()),
            })
        }
    };

    Ok(res)
}

// Primary ::= Literal | Group | Expansion;
fn parse_primary<'a>(
    source: &'a str,
    iter: &mut ParserIterator,
) -> Result<EBNFNode<'a>, EBNFParseError> {
    skip_space(iter);
    let Some(&(_, c)) = iter.peek() else {
        return Err(EBNFParseError::UnexpectedEOF);
    };

    match c {
        '"' => parse_literal(source, iter),
        '(' => parse_group(source, iter),
        _ if c.is_alphabetic() => parse_expansion(source, iter),
        _ => Err(EBNFParseError::UnmatchToken {
            current_token: get_token(iter),
            position: get_position(iter, source.len()),
        }),
    }
}

// Group ::= "(" Or ")" ;
fn parse_group<'a>(
    source: &'a str,
    iter: &mut ParserIterator,
) -> Result<EBNFNode<'a>, EBNFParseError> {
    skip_space(iter);
    if iter.next_if(|c| matches!(c.1, '(')).is_none() {
        return Err(EBNFParseError::UnexpectedToken {
            expect_token: '(',
            unexpected_token: get_token(iter),
            position: get_position(iter, source.len()),
        });
    }

    let node = parse_or(source, iter)?;

    skip_space(iter);
    if iter.next_if(|c| matches!(c.1, ')')).is_none() {
        return Err(EBNFParseError::UnexpectedToken {
            expect_token: ')',
            unexpected_token: get_token(iter),
            position: get_position(iter, source.len()),
        });
    }

    Ok(EBNFNode::Group(Rc::new(node)))
}

fn parse_expansion<'a>(
    source: &'a str,
    iter: &mut ParserIterator,
) -> Result<EBNFNode<'a>, EBNFParseError> {
    skip_space(iter);

    let Some(c) = iter.peek() else {
        return Err(EBNFParseError::UnexpectedEOF);
    };
    if !c.1.is_alphabetic() {
        return Err(EBNFParseError::ParseExpansionError {
            position: get_position(iter, source.len()),
        });
    }

    let name = parse_and_slice(source, iter, |c| c.is_alphabetic() || c.is_ascii_digit())?;

    Ok(EBNFNode::Expansion(name))
}

// Literal ::= "\"" { any-char-except-quote } "\"" ;
fn parse_literal<'a>(
    source: &'a str,
    iter: &mut ParserIterator,
) -> Result<EBNFNode<'a>, EBNFParseError> {
    skip_space(iter);
    if iter.next_if(|c| matches!(c.1, '"')).is_none() {
        return Err(EBNFParseError::UnexpectedToken {
            expect_token: '"',
            unexpected_token: get_token(iter),
            position: get_position(iter, source.len()),
        });
    }

    let literal = parse_and_slice(source, iter, |c| !matches!(c, '"'))?;

    if iter.next_if(|c| matches!(c.1, '"')).is_none() {
        return Err(EBNFParseError::UnexpectedToken {
            expect_token: '"',
            unexpected_token: get_token(iter),
            position: get_position(iter, source.len()),
        });
    }

    Ok(EBNFNode::Literal(literal))
}

// Integer ::= Digit { Digit } ;
// Digit ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
fn parse_integer(iter: &mut ParserIterator, source: &str) -> Result<u64, EBNFParseError> {
    skip_space(iter);
    let position = get_position(iter, source.len());
    let token = from_fn(|| iter.next_if(|c| c.1.is_ascii_digit()))
        .map(|c| c.1)
        .collect::<String>();

    if token.is_empty() {
        return Err(EBNFParseError::UnmatchToken {
            current_token: get_token(iter),
            position: get_position(iter, source.len()),
        });
    }

    token
        .parse()
        .map_err(|_| EBNFParseError::ParseIntError { position })
}

fn parse_and_slice<'a>(
    source: &'a str,
    iter: &mut ParserIterator,
    condition: fn(char) -> bool,
) -> Result<&'a str, EBNFParseError> {
    let Some(&(start, _)) = iter.peek() else {
        return Err(EBNFParseError::UnexpectedEOF);
    };

    let _ = from_fn(|| iter.next_if(|&(_, c)| condition(c))).count();
    let end = iter.peek().map(|e| e.0).unwrap_or(source.len());

    if start == end {
        return Err(EBNFParseError::UnmatchToken {
            current_token: get_token(iter),
            position: get_position(iter, source.len()),
        });
    }

    Ok(&source[start..end])
}

fn skip_space(iter: &mut ParserIterator) {
    while iter.next_if(|c| c.1.is_whitespace()).is_some() {}
}

fn get_token(iter: &mut ParserIterator) -> String {
    iter.peek().map_or(EOF.to_string(), |c| c.1.to_string())
}

fn get_position(iter: &mut ParserIterator, source_len: usize) -> usize {
    iter.peek().map_or(source_len, |c| c.0)
}

#[derive(Debug)]
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
    ParseDefineError {
        position: usize,
    },
    ParseExpansionError {
        position: usize,
    },
}

impl EBNFParseError {
    fn error_message(&self, input: &str) -> String {
        match self {
            EBNFParseError::UnexpectedToken {
                expect_token,
                unexpected_token,
                position,
            } => [
                format!("unexpected token: {unexpected_token}"),
                input.to_string(),
                format!("{}^", " ".repeat(*position)),
                format!("expect token: {expect_token}"),
            ]
            .join("\n"),

            EBNFParseError::UnmatchToken {
                current_token,
                position,
            } => [
                format!("unmatch token: {current_token}"),
                input.to_string(),
                format!("{}^", " ".repeat(*position)),
            ]
            .join("\n"),

            EBNFParseError::UnexpectedEOF => "unexpected EOF".to_string(),
            EBNFParseError::ParseIntError { position } => [
                "can not parse integer".to_string(),
                input.to_string(),
                format!("{}^", " ".repeat(*position)),
            ]
            .join("\n"),

            EBNFParseError::ParseExpansionError { position } => [
                "can not parse expansion".to_string(),
                input.to_string(),
                format!("{}^", " ".repeat(*position)),
            ]
            .join("\n"),

            EBNFParseError::ParseDefineError { position } => [
                "can not parse define".to_string(),
                input.to_string(),
                format!("{}^", " ".repeat(*position)),
            ]
            .join("\n"),
        }
    }
}
