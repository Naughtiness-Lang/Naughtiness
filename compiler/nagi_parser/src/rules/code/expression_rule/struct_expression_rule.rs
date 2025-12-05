use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_struct_expression_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            StructExpression ::= StructExprStruct | StructExprTuple | StructExprUnit
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            StructExprStruct ::= PathInExpression `{` ( StructExprFields | StructBase )? `}`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            StructExprFields ::= StructExprField 
                                 ( `,` StructExprField )* 
                                 ( `,` StructBase | `,`? )
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            StructExprField ::= IDENTIFIER `:` Expression
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            StructBase ::= `..` Expression
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            StructExprTuple ::= PathInExpression `(` ( Expression ( `,` Expression)* `,`? )? `)`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            StructExprUnit ::= PathInExpression
            ",
            ASTBuilder::None,
        ),
    ]
}
