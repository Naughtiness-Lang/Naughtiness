use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_match_expression_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            MatchExpression ::= `match` Scrutinee `{` 
                                MatchArms? 
                                `}`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            MatchArms ::= ( MatchArm `=>` ( 
                            ExpressionWithoutBlock `,` | ExpressionWithBlock `,`? 
                           ) )* 
                          MatchArm `=>` Expression `,`?
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            MatchArm ::= Pattern MatchArmGuard?
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            MatchArmGuard ::= `if` Expression
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            Scrutinee ::= Expression
            ",
            ASTBuilder::None,
        ),
    ]
}
