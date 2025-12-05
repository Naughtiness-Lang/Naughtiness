use crate::packrat::{ASTBuilder, EBNFTable};

use super::NagiCodeRules;

pub(super) fn make_statement_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            Statements ::= Statement+ 
                         | Statement+ ExpressionWithoutBlock 
                         | ExpressionWithoutBlock
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            Statement ::= `;` | Item | LetStatement | ExpressionStatement
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            LetStatement ::= `let` PatternNoTopAlt ( `:` Type )? (`=` Expression )? `;`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            ExpressionStatement ::= ExpressionWithoutBlock `;` 
                                  | ExpressionWithBlock `;`?
            ",
            ASTBuilder::None,
        ),
    ]
}
