use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_array_expression_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            ArrayExpression ::= `[` ArrayElements? `]`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            ArrayElements ::= Expression ( `,` Expression )* `,`? 
                            | Expression `;` Expression
            ",
            ASTBuilder::None,
        ),
    ]
}
