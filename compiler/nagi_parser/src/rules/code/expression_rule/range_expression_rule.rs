use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_range_expression_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            RangeExpression ::= RangeExpr
                              | RangeFromExpr
                              | RangeToExpr
                              | RangeFullExpr
                              | RangeInclusiveExpr
                              | RangeToInclusiveExpr 
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            RangeExpr ::= Expression `..` Expression
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            RangeFromExpr ::= Expression `..`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            RangeToExpr ::= `..` Expression
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            RangeFullExpr ::= `..`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            RangeInclusiveExpr ::= Expression `..=` Expression
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            RangeToInclusiveExpr ::= `..=` Expression
            ",
            ASTBuilder::None,
        ),
    ]
}
