mod array_expression_rule;
mod break_expression_rule;
mod call_expression_rule;
mod continue_expression_rule;
mod field_expression_rule;
mod grouped_expression_rule;
mod if_expression_rule;
mod index_expression_rule;
mod literal_expression_rule;
mod loop_expression_rule;
mod match_expression_rule;
mod method_call_expression_rule;
mod operator_expression_rule;
mod path_expression_rule;
mod path_in_expression_rule;
mod qualified_path_in_expression_rule;
mod range_expression_rule;
mod return_expression_rule;
mod struct_expression_rule;
mod tuple_expression_rule;

use array_expression_rule::make_array_expression_table;
use break_expression_rule::make_break_expression_table;
use call_expression_rule::make_call_expression_table;
use continue_expression_rule::make_continue_expression_table;
use field_expression_rule::make_field_expression_table;
use grouped_expression_rule::make_grouped_expression_table;
use if_expression_rule::make_if_expression_table;
use index_expression_rule::make_index_expression_table;
use literal_expression_rule::make_literal_expression_table;
use loop_expression_rule::make_loop_expression_table;
use match_expression_rule::make_match_expression_table;
use method_call_expression_rule::make_method_call_expression_table;
use operator_expression_rule::make_operator_expression_table;
use path_expression_rule::make_path_expression_table;
use path_in_expression_rule::make_path_in_expression_table;
use qualified_path_in_expression_rule::make_qualified_path_in_expression_table;
use range_expression_rule::make_range_expression_table;
use return_expression_rule::make_return_expression_table;
use struct_expression_rule::make_struct_expression_table;
use tuple_expression_rule::make_tuple_expression_table;

use crate::packrat::{ASTBuilder, EBNFTable};

use super::NagiCodeRules;

pub(super) fn make_expression_table() -> NagiCodeRules {
    vec![
        make_expression_rule_table(),
        make_array_expression_table(),
        make_break_expression_table(),
        make_call_expression_table(),
        make_continue_expression_table(),
        make_field_expression_table(),
        make_grouped_expression_table(),
        make_if_expression_table(),
        make_index_expression_table(),
        make_literal_expression_table(),
        make_loop_expression_table(),
        make_match_expression_table(),
        make_method_call_expression_table(),
        make_operator_expression_table(),
        make_path_expression_table(),
        make_path_in_expression_table(),
        make_qualified_path_in_expression_table(),
        make_range_expression_table(),
        make_return_expression_table(),
        make_struct_expression_table(),
        make_tuple_expression_table(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn make_expression_rule_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            Expression ::= ExpressionWithoutBlock | ExpressionWithBlock
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            ExpressionWithoutBlock ::= OperatorExpression
                                     | LiteralExpression
                                     | PathExpression
                                     | GroupedExpression
                                     | ArrayExpression
                                     | IndexExpression
                                     | TupleExpression
                                     | StructExpression
                                     | CallExpression
                                     | MethodCallExpression
                                     | FieldExpression
                                     | ContinueExpression
                                     | BreakExpression
                                     | RangeExpression
                                     | ReturnExpression
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            ExpressionWithBlock ::= BlockExpression
                                  | LoopExpression
                                  | IfExpression
                                  | IfLetExpression
                                  | MatchExpression
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            BlockExpression ::= `{` Statements? `}`
            ",
            ASTBuilder::None,
        ),
    ]
}
