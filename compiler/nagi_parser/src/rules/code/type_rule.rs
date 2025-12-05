use crate::packrat::{ASTBuilder, EBNFTable};

use super::NagiCodeRules;

pub(super) fn make_type_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            Type ::= TypeNoBounds
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            TypeNoBounds ::= TypePath
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            TypePath ::= `::`? TypePathSegment (`::` TypePathSegment)*
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            TypePathSegment ::= PathIdentSegment (`::`? (GenericArgs | TypePathFn))?
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            TypePathFn ::= `(` TypePathFnInputs? `)` (`->` TypeNoBounds)?
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            TypePathFnInputs ::= Type (`,` Type)* `,`?
            ",
            ASTBuilder::None,
        ),
    ]
}
