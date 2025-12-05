use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_if_expression_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            IfExpression ::= `if` Expression BlockExpression 
                         ( `else` ( BlockExpression | IfExpression | IfLetExpression ) )?
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            IfLetExpression ::= `if` `let` Pattern `=` Scrutinee BlockExpression 
                                 ( `else` ( BlockExpression | IfExpression | IfLetExpression ) )?
            ",
            ASTBuilder::None,
        ),
    ]
}
