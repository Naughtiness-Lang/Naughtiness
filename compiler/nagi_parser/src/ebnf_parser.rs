use std::iter::from_fn;
use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub(crate) struct EBNF {
    pub name: String,   // 定義したルール名
    pub expr: EBNFNode, // ツリー構造(ルールの中身)
}

#[derive(Debug)]
pub(crate) enum EBNFNode {
    Expansion(String),                       // Hoge
    Concat(Vec<EBNFNode>),                   // Hoge Fuga
    Or(Vec<EBNFNode>),                       // Hoge | Fuga
    Repeat(Box<EBNFNode>, u64, Option<u64>), // Hoge? Hoge* Hoge+ Hoge{3} Hoge{7,} Hoge{2, 5}
    Group(Box<EBNFNode>),                    // (Hoge)
    Literal(String),                         // "hogefuga"
}

#[derive(Debug)]
enum Quantifier {
    Question,                 // ?
    Plus,                     // +
    Star,                     // *
    Braces(u64, Option<u64>), // {
}

pub fn parse_ebnf(ebnf: &str) -> Result<EBNF, String> {
    let mut iter = ebnf.chars().peekable();
    parse_define(&mut iter)
}

fn parse_define(iter: &mut Peekable<Chars>) -> Result<EBNF, String> {
    skip_space(iter);
    let name = from_fn(|| iter.next_if(|c| c.is_alphabetic())).collect::<String>();

    skip_space(iter);
    if !matches!(iter.next(), Some(':')) {
        return Err(error_message(iter, "Define"));
    }
    if !matches!(iter.next(), Some(':')) {
        return Err(error_message(iter, "Define"));
    }
    if !matches!(iter.next(), Some('=')) {
        return Err(error_message(iter, "Define"));
    }

    let expr = parse_expression(iter)?;

    Ok(EBNF { name, expr })
}

// Expression ::= Or ;
fn parse_expression(iter: &mut Peekable<Chars>) -> Result<EBNFNode, String> {
    skip_space(iter);
    parse_or(iter)
}

// Or ::= Concat { "|" Concat } ;
fn parse_or(iter: &mut Peekable<Chars>) -> Result<EBNFNode, String> {
    // Concat
    skip_space(iter);
    let left = parse_concat(iter)?;

    //
    skip_space(iter);
    let mut right = vec![];
    while iter.next_if(|t| matches!(t, '|')).is_some() {
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
fn parse_concat(iter: &mut Peekable<Chars>) -> Result<EBNFNode, String> {
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
fn parse_repeat(iter: &mut Peekable<Chars>) -> Result<EBNFNode, String> {
    let node = parse_primary(iter)?;

    if let Ok(quantifier) = parse_quantifier(iter) {
        let node = Box::new(node);
        return Ok(match quantifier {
            Quantifier::Plus => EBNFNode::Repeat(node, 1, None),
            Quantifier::Star => EBNFNode::Repeat(node, 0, None),
            Quantifier::Question => EBNFNode::Repeat(node, 0, Some(1)),
            Quantifier::Braces(start, end) => EBNFNode::Repeat(node, start, end),
        });
    }

    Ok(node)
}

// Quantifier ::= "?" | "*" | "+" | "{" Integer [ "," [ Integer ] ] "}" ;
fn parse_quantifier(iter: &mut Peekable<Chars>) -> Result<Quantifier, String> {
    skip_space(iter);
    let t = iter.peek().ok_or("".to_string())?;
    let res = match t {
        '?' => Quantifier::Question,
        '*' => Quantifier::Star,
        '+' => Quantifier::Plus,
        '{' => {
            iter.next();

            let start = parse_integer(iter)?;
            let mut end = None;

            skip_space(iter);
            if iter.next_if(|t| matches!(t, ',')).is_some() {
                end = parse_integer(iter).ok();
            }

            skip_space(iter);
            if iter.next_if(|t| matches!(t, '}')).is_none() {
                return Err("} ".to_string());
            }

            Quantifier::Braces(start, end)
        }

        _ => return Err("".to_string()),
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
fn parse_primary(iter: &mut Peekable<Chars>) -> Result<EBNFNode, String> {
    if let Ok(node) = parse_literal(iter) {
        return Ok(node);
    }

    if let Ok(node) = parse_group(iter) {
        return Ok(node);
    }

    if let Ok(node) = parse_expansion(iter) {
        return Ok(node);
    }

    Err(error_message(iter, "Primary"))
}

// Group ::= "(" Or ")" ;
fn parse_group(iter: &mut Peekable<Chars>) -> Result<EBNFNode, String> {
    skip_space(iter);
    if iter.next_if(|c| matches!(c, '(')).is_none() {
        return Err(error_message(iter, "Group"));
    }

    let node = parse_or(iter)?;

    skip_space(iter);
    if iter.next_if(|c| matches!(c, ')')).is_none() {
        return Err(error_message(iter, "Group"));
    }

    Ok(EBNFNode::Group(Box::new(node)))
}

fn parse_expansion(iter: &mut Peekable<Chars>) -> Result<EBNFNode, String> {
    let name = from_fn(|| iter.next_if(|c| c.is_alphabetic())).collect::<String>();

    if name.is_empty() {
        return Err(error_message(iter, "Expansion"));
    }

    Ok(EBNFNode::Expansion(name))
}

// Literal ::= "\"" { any-char-except-quote } "\"" ;
fn parse_literal(iter: &mut Peekable<Chars>) -> Result<EBNFNode, String> {
    skip_space(iter);
    if iter.next_if(|c| matches!(c, '"')).is_none() {
        return Err(error_message(iter, "Literal"));
    }

    skip_space(iter);
    let literal = from_fn(|| iter.next_if(|c| !matches!(c, '"'))).collect();

    skip_space(iter);
    if iter.next_if(|c| matches!(c, '"')).is_none() {
        return Err(error_message(iter, "Literal"));
    }

    Ok(EBNFNode::Literal(literal))
}

// Integer ::= Digit { Digit } ;
// Digit ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
fn parse_integer(iter: &mut Peekable<Chars>) -> Result<u64, String> {
    skip_space(iter);
    from_fn(|| iter.next_if(|c| c.is_numeric()))
        .collect::<String>()
        .parse()
        .map_err(|e| format!("{e:?}"))
}

fn skip_space(iter: &mut Peekable<Chars>) {
    while iter.next_if(|c| c.is_whitespace()).is_some() {}
}

fn error_message(iter: &mut Peekable<Chars>, rule: &str) -> String {
    format!("rule: {}, unexcept token: {:?} ", rule, iter.peek())
}
