use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_qualified_path_in_expression_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            QualifiedPathInExpression ::= QualifiedPathType (`::` PathExprSegment)+
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            QualifiedPathType ::= `<` Type (`as` TypePath)? `>`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            QualifiedPathInType ::= QualifiedPathType (`::` TypePathSegment)+
            ",
            ASTBuilder::None,
        ),
    ]
}
