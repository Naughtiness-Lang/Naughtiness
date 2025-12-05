use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_path_expression_table() -> NagiCodeRules {
    vec![EBNFTable::new(
        "
        PathExpression ::= PathInExpression | QualifiedPathInExpression
        ",
        ASTBuilder::None,
    )]
}
