use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_field_expression_table() -> NagiCodeRules {
    vec![EBNFTable::new(
        "
        FieldExpression ::= Expression `.` IDENTIFIER
        ",
        ASTBuilder::None,
    )]
}
