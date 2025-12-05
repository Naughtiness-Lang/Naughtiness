use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_continue_expression_table() -> NagiCodeRules {
    vec![EBNFTable::new(
        "
        ContinueExpression ::= `continue`
        ",
        ASTBuilder::None,
    )]
}
