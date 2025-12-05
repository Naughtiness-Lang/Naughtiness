use super::{NagiCodeParseResult, NagiCodeRules};
use crate::{
    errors::ParsingError,
    packrat::{ASTBuilder, EBNFTable},
};

pub(super) fn make_function_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            Function ::= `fn` IDENTIFIER `(` FunctionParameters? `)` 
                          FunctionReturnType? ( BlockExpression | `;` )
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            FunctionParameters ::= FunctionParam (`,` FunctionParam)* `,`?
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            FunctionParam ::= ( FunctionParamPattern | `...` | Type )
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            FunctionParamPattern ::= PatternNoTopAlt `:` ( Type | `...` )
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            FunctionReturnType ::= `->` Type
            ",
            ASTBuilder::None,
        ),
    ]
}

fn make_function_ast() -> NagiCodeParseResult {
    Err(ParsingError::UnexpectedToken)
}
