use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_tuple_expression_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            TupleExpression ::= `(` TupleElements? `)`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            TupleElements ::= ( Expression `,` )+ Expression?
            ",
            ASTBuilder::None,
        ),
    ]
}
