use crate::packrat::{ASTBuilder, EBNFTable};

use super::NagiCodeRules;

pub(super) fn make_pattern_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "
            Pattern ::= `|`? PatternNoTopAlt ( `|` PatternNoTopAlt )*
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            PatternNoTopAlt ::= PatternWithoutRange | RangePattern
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            PatternWithoutRange ::= LiteralPattern
                                  | IDENTIFIERPattern
                                  | WildcardPattern
   　                             | RestPattern
   　                             | ReferencePattern
                                  | StructPattern
                                  | TupleStructPattern
                                  | TuplePattern
                                  | GroupedPattern
                                  | SlicePattern
                                  | PathPattern
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            LiteralPattern ::= `true` 
                             | `false`
                             | `-`? INTEGER_LITERAL
                             | `-`? FLOAT_LITERAL
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            IDENTIFIERPattern ::= `ref`? `mut`? IDENTIFIER (`@` PatternNoTopAlt )?
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            WildcardPattern ::= `_`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            RestPattern ::= `..`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            ReferencePattern ::= (`&`|`&&`) `mut`? PatternWithoutRange
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            StructPattern ::= PathInExpression `{` StructPatternElements? `}`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            StructPatternElements ::= StructPatternFields 
                                      (`,` | `,` StructPatternEtCetera)? 
                                      | StructPatternEtCetera
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            StructPatternFields ::= StructPatternField (`,` StructPatternField)*
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            StructPatternField ::= IDENTIFIER `:` Pattern
                                 | `ref`? `mut`? IDENTIFIER
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            StructPatternEtCetera  ::= `..`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            TupleStructPattern ::= PathInExpression `(` TupleStructItems? `)`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            TupleStructItems ::= Pattern ( `,` Pattern )* `,`?
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            TuplePattern ::= `(` TuplePatternItems? `)`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            TuplePatternItems ::= Pattern `,`
                                | RestPattern
                                | Pattern (`,` Pattern)+ `,`?
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            GroupedPattern ::= `(` Pattern `)`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            SlicePattern ::= `[` SlicePatternItems? `]`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            SlicePatternItems ::= Pattern (`,` Pattern)* `,`?
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            PathPattern ::= PathExpression
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            RangePattern ::= RangeInclusivePattern
                           | RangeFromPattern
                           | RangeToInclusivePattern
                           | ObsoleteRangePattern
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            RangeExclusivePattern ::= RangePatternBound `..` RangePatternBound
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            RangeInclusivePattern ::= RangePatternBound `..=` RangePatternBound
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            RangeFromPattern ::= RangePatternBound `..`
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            RangeToInclusivePattern ::= `..=` RangePatternBound
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            ObsoleteRangePattern ::= RangePatternBound `...` RangePatternBound
            ",
            ASTBuilder::None,
        ),
        EBNFTable::new(
            "
            RangePatternBound ::= `-`? INTEGER_LITERAL
                                | `-`? FLOAT_LITERAL
                                | PathExpression
            ",
            ASTBuilder::None,
        ),
    ]
}
