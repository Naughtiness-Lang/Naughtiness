use super::nagi_code_lexer::NagiOperator;
use crate::lexer::{make_pattern_map, PatternHashMap};
use nagi_lexer::token::Symbol;
use std::sync::LazyLock;

pub(crate) static OPERATOR_PATTERN_MAP: LazyLock<PatternHashMap<Symbol, NagiOperator>> =
    LazyLock::new(|| {
        let list = vec![
            // 算術
            (vec![Symbol::Plus], NagiOperator::Add),
            (vec![Symbol::Minus], NagiOperator::Sub),
            (vec![Symbol::Star], NagiOperator::Mul),
            (vec![Symbol::Slash], NagiOperator::Div),
            (vec![Symbol::Percent], NagiOperator::Mod),
            // 比較
            (vec![Symbol::Equal, Symbol::Equal], NagiOperator::Equal),
            (vec![Symbol::Not, Symbol::Equal], NagiOperator::NotEqual),
            (vec![Symbol::GreaterThan], NagiOperator::Greater),
            (vec![Symbol::LessThan], NagiOperator::Less),
            (
                vec![Symbol::GreaterThan, Symbol::Equal],
                NagiOperator::GreaterEqual,
            ),
            (
                vec![Symbol::LessThan, Symbol::Equal],
                NagiOperator::LessEqual,
            ),
            // 論理
            (vec![Symbol::And, Symbol::And], NagiOperator::And),
            (vec![Symbol::Or, Symbol::Or], NagiOperator::Or),
            (vec![Symbol::Not], NagiOperator::Not),
            // ビット演算
            (vec![Symbol::And], NagiOperator::BitwiseAnd),
            (vec![Symbol::Or], NagiOperator::BitwiseOr),
            (vec![Symbol::Tilde], NagiOperator::BitwiseNot),
            (vec![Symbol::Caret], NagiOperator::BitwiseXor),
            (
                vec![Symbol::LessThan, Symbol::LessThan],
                NagiOperator::LeftShift,
            ),
            (
                vec![Symbol::GreaterThan, Symbol::GreaterThan],
                NagiOperator::RightShift,
            ),
            // 代入
            (vec![Symbol::Equal], NagiOperator::Assign),
            (vec![Symbol::Plus, Symbol::Equal], NagiOperator::AddAssign),
            (vec![Symbol::Minus, Symbol::Equal], NagiOperator::SubAssign),
            (vec![Symbol::Star, Symbol::Equal], NagiOperator::MulAssign),
            (vec![Symbol::Slash, Symbol::Equal], NagiOperator::DivAssign),
            (
                vec![Symbol::Percent, Symbol::Equal],
                NagiOperator::ModAssign,
            ),
            (
                vec![Symbol::And, Symbol::Equal],
                NagiOperator::BitwiseAndAssign,
            ),
            (
                vec![Symbol::Or, Symbol::Equal],
                NagiOperator::BitwiseOrAssign,
            ),
            (
                vec![Symbol::Caret, Symbol::Equal],
                NagiOperator::BitwiseXorAssign,
            ),
            (
                vec![Symbol::LessThan, Symbol::LessThan, Symbol::Equal],
                NagiOperator::LeftShiftAssign,
            ),
            (
                vec![Symbol::GreaterThan, Symbol::GreaterThan, Symbol::Equal],
                NagiOperator::RightShiftAssign,
            ),
            //
            (vec![Symbol::Question], NagiOperator::Question),
            (vec![Symbol::Dot], NagiOperator::Dot),
        ];

        make_pattern_map(list)
    });
