use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_index_expression_table() -> NagiCodeRules {
    vec![EBNFTable::new(
        "
        IndexExpression ::= Expression `[` Expression `]`
        ",
        ASTBuilder::None,
    )]
}
