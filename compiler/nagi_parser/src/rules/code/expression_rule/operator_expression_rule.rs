use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_operator_expression_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            OperatorExpression ::= ArithmeticOrLogicalExpression
                                 | ComparisonExpression 
                                 | AssignmentExpression 
                                 | CompoundAssignmentExpression 
                                 | LazyBooleanExpression
                                 | NegationExpression
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            ArithmeticOrLogicalExpression ::= Expression `+` Expression
                                            | Expression `-` Expression
                                            | Expression `*` Expression
                                            | Expression `/` Expression
                                            | Expression `%` Expression
                                            | Expression `&` Expression
                                            | Expression `|` Expression
                                            | Expression `^` Expression
                                            | Expression `<<` Expression
                                            | Expression `>>` Expression
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            ComparisonExpression ::= Expression `==` Expression
                                   | Expression `!=` Expression
                                   | Expression `>` Expression
                                   | Expression `<` Expression
                                   | Expression `>=` Expression
                                   | Expression `<=` Expression
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            AssignmentExpression ::= Expression `=` Expression
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            CompoundAssignmentExpression ::= Expression `+=` Expression
                                           | Expression `-=` Expression
                                           | Expression `*=` Expression
                                           | Expression `/=` Expression
                                           | Expression `%=` Expression
                                           | Expression `&=` Expression
                                           | Expression `|=` Expression
                                           | Expression `^=` Expression
                                           | Expression `<<=` Expression
                                           | Expression `>>=` Expression
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            LazyBooleanExpression ::= Expression `||` Expression 
                                    | Expression `&&` Expression
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            NegationExpression ::= `-` Expression | `!` Expression
            ",
            ASTBuilder::None,
        ),
    ]
}
