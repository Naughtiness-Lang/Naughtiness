use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_loop_expression_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            LoopExpression ::=  
                              (
                                InfiniteLoopExpression
                              | PredicateLoopExpression
                              | PredicatePatternLoopExpression
                              | IteratorLoopExpression
                              )
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            InfiniteLoopExpression ::= `loop` BlockExpression
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            PredicateLoopExpression ::= `while` Expression BlockExpression
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            PredicatePatternLoopExpression ::= `while` `let` Pattern `=` 
                                                Scrutinee BlockExpression
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            IteratorLoopExpression ::= `for` Pattern `in` Expression BlockExpression
            ",
            ASTBuilder::None,
        ),
    ]
}
