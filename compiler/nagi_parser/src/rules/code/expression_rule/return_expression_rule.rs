use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_return_expression_table() -> NagiCodeRules {
    vec![EBNFTable::new(
        "
        ReturnExpression ::= `return` Expression?
        ",
        ASTBuilder::None,
    )]
}
