use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_literal_expression_table() -> NagiCodeRules {
    vec![EBNFTable::new(
        "
        LiteralExpression ::= INTEGER_LITERAL
                            | FLOAT_LITERAL
                            | `true`
                            | `false`
        ",
        ASTBuilder::None,
    )]
}
