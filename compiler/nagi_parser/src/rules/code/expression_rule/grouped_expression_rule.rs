use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_grouped_expression_table() -> NagiCodeRules {
    vec![EBNFTable::new(
        "
        GroupedExpression ::= `(` Expression `)`
        ",
        ASTBuilder::None,
    )]
}
