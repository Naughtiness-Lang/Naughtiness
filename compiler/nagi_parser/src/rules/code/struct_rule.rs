use crate::packrat::{ASTBuilder, EBNFTable};

use super::NagiCodeRules;

pub(super) fn make_struct_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            Struct ::= StructStruct | TupleStruct
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            StructStruct ::= `struct` IDENTIFIER GenericParams?
                              ( `{` StructFields? `}` | `;` )
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            StructFields ::= StructField (`,` StructField)* `,`?
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            StructField ::= Visibility? IDENTIFIER `:` Type
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            TupleStruct ::= `struct` IDENTIFIER GenericParams?
                             `(` TupleFields? `)` `;`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            TupleFields ::= TupleField (`,` TupleField)* `,`?
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            TupleField ::= Visibility? Type
            ",
            ASTBuilder::None,
        ),
    ]
}
