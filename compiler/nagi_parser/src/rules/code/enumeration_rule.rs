use crate::packrat::{ASTBuilder, EBNFTable};

use super::NagiCodeRules;

pub(super) fn make_enum_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            Enumeration ::= `enum` IDENTIFIER GenericParams?
                             `{` EnumItems? `}`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            EnumItems ::= EnumItem ( `,` EnumItem )* `,`?
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            EnumItem ::= Visibility? IDENTIFIER 
                         ( EnumItemTuple | EnumItemStruct )?
                         EnumItemDiscriminant?
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            EnumItemTuple ::= `(` TupleFields? `)`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            EnumItemStruct ::= `{` StructFields? `}`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            EnumItemDiscriminant ::= `=` Expression
            ",
            ASTBuilder::None,
        ),
    ]
}
