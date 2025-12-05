use super::NagiCodeRules;
use crate::packrat::{ASTBuilder, EBNFTable};

pub(crate) fn make_item_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            Item ::= VisItem
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            VisItem ::= (
                         Function 
                       | Struct 
                       | Enumeration 
                       )
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            Visibility ::= `pub`
            ",
            ASTBuilder::None,
        ),
    ]
}
