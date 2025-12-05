use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_path_in_expression_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            PathInExpression ::= `::`? PathExprSegment ( `::` PathExprSegment )*
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            PathExprSegment ::= PathIdentSegment ( `::` GenericArgs )?
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            PathIdentSegment ::= IDENTIFIER
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            GenericArgs ::= `<` `>` 
                          | `<` ( GenericArg `,` )* GenericArg `,`? `>`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            GenericArg ::= Type 
                         | GenericArgsConst 
                         | GenericArgsBinding 
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            GenericArgsConst ::= BlockExpression 
                               | LiteralExpression 
                               | `-` LiteralExpression 
                               | SimplePathSegment
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            GenericArgsBinding ::= IDENTIFIER GenericArgs? `=` Type
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            GenericArgsBounds ::= IDENTIFIER GenericArgs? `:` TypeParamBounds
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            SimplePath ::= `::`? SimplePathSegment ( `::` SimplePathSegment )*
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            SimplePathSegment ::= IDENTIFIER 
                                | `super` 
                                | `self` 
                                | `crate` 
                                | `$crate`
            ",
            ASTBuilder::None,
        ),
    ]
}
