use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_call_expression_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            CallExpression ::= Expression `(` CallParams? `)`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            CallParams ::= Expression ( `,` Expression )* `,`?
            ",
            ASTBuilder::None,
        ),
    ]
}
