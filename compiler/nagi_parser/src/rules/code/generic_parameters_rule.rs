use super::NagiCodeRules;
use crate::packrat::{ASTBuilder, EBNFTable};

pub(super) fn make_generic_parameters_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            GenericParams ::= `<` ( GenericParam (`,` GenericParam)* `,`? )? `>`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            GenericParam ::= ( TypeParam | ConstParam )
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            TypeParam ::= IDENTIFIER ( `=` Type )?
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            ConstParam ::= `const` IDENTIFIER `:` Type
                           ( `=` ( BlockExpression | IDENTIFIER | `-`?LiteralExpression ) )?
            ",
            ASTBuilder::None,
        ),
    ]
}
