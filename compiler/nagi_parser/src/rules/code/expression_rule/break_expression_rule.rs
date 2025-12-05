use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_break_expression_table() -> NagiCodeRules {
    vec![EBNFTable::new(
        "
        BreakExpression ::= `break` Expression?
        ",
        ASTBuilder::None,
    )]
}
