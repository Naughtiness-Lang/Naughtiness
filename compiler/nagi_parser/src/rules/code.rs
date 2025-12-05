mod enumeration_rule;
mod expression_rule;
mod function_rule;
mod generic_parameters_rule;
mod item_rule;
mod literal_rule;
mod pattern_rule;
mod statement_rule;
mod struct_rule;
mod type_rule;

use std::{rc::Rc, str::FromStr};

use crate::{
    errors::ParsingError,
    lexer::code::{
        keywords::NagiCodeKeyword,
        nagi_code_lexer::{
            NagiIdentifier, NagiOperator, NagiProgramToken, NagiProgramTokenKind, NagiSymbol,
        },
    },
    packrat::{ASTAssembly, ParseResult, Rules},
};
use enumeration_rule::make_enum_table;
use expression_rule::make_expression_table;
use function_rule::make_function_table;
use generic_parameters_rule::make_generic_parameters_table;
use item_rule::make_item_table;
use literal_rule::make_literal_table;
use nagi_ast::ASTNode;
use pattern_rule::make_pattern_table;
use statement_rule::make_statement_table;
use struct_rule::make_struct_table;
use type_rule::make_type_table;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum NagiCodeASTParts {
    None,
}

pub(crate) type NagiCodeParseResult = ParseResult<ASTNode, NagiCodeASTParts>;

type NagiCodeRules = Rules<'static, NagiProgramToken, ASTNode, NagiCodeASTParts>;

pub(super) fn make_nagi_code_rule_table() -> NagiCodeRules {
    vec![
        make_enum_table(),
        make_expression_table(),
        make_function_table(),
        make_generic_parameters_table(),
        make_item_table(),
        make_literal_table(),
        make_pattern_table(),
        make_statement_table(),
        make_struct_table(),
        make_type_table(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

pub(super) fn parse_nagi_code_literal(
    literal: &str,
    token: &NagiProgramToken,
) -> NagiCodeParseResult {
    match &token.token_kind {
        NagiProgramTokenKind::Symbol(symbol) => {
            let literal =
                NagiSymbol::from_str(literal).map_err(|_| ParsingError::UnexpectedLiteral)?;
            if literal != *symbol {
                return Err(ParsingError::UnexpectedLiteral);
            }
        }
        NagiProgramTokenKind::Operator(operator) => {
            let literal =
                NagiOperator::from_str(literal).map_err(|_| ParsingError::UnexpectedLiteral)?;
            if literal != *operator {
                return Err(ParsingError::UnexpectedLiteral);
            }
        }
        NagiProgramTokenKind::Identifier(NagiIdentifier::Keyword(keyword)) => {
            let literal =
                NagiCodeKeyword::from_str(literal).map_err(|_| ParsingError::UnexpectedLiteral)?;
            if literal != *keyword {
                return Err(ParsingError::UnexpectedLiteral);
            }
        }
        _ => return Err(ParsingError::UnexpectedLiteral),
    }

    Ok(Rc::new(ASTAssembly::None))
}
