use super::nagi_code_lexer::NagiSymbol;
use crate::lexer::{make_pattern_map, PatternHashMap};
use nagi_lexer::token::Symbol;
use std::sync::LazyLock;

pub(crate) static SYMBOL_PATTERN_MAP: LazyLock<PatternHashMap<Symbol, NagiSymbol>> =
    LazyLock::new(|| {
        let list = vec![
            (vec![Symbol::LeftBrace], NagiSymbol::LeftBrace),
            (vec![Symbol::RightBrace], NagiSymbol::RightBrace),
            (vec![Symbol::LeftBrackets], NagiSymbol::LeftBrackets),
            (vec![Symbol::RightBrackets], NagiSymbol::RightBrackets),
            (vec![Symbol::LeftParenthesis], NagiSymbol::LeftParenthesis),
            (vec![Symbol::RightParenthesis], NagiSymbol::RightParenthesis),
            (vec![Symbol::Semicolon], NagiSymbol::Semicolon),
            (vec![Symbol::Comma], NagiSymbol::Comma),
        ];

        make_pattern_map(list)
    });
