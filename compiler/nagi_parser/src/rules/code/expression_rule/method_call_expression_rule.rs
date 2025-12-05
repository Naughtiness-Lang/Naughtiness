use crate::{
    packrat::{ASTBuilder, EBNFTable},
    rules::code::NagiCodeRules,
};

pub(super) fn make_method_call_expression_table() -> NagiCodeRules {
    vec![EBNFTable::new(
        "
        MethodCallExpression ::= Expression `.` PathExprSegment `(` CallParams? `)`
        ",
        ASTBuilder::None,
    )]
}
