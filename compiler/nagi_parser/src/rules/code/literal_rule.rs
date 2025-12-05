use std::rc::Rc;

use crate::{
    errors::ParsingError,
    lexer::code::nagi_code_lexer::{
        NagiIdentifier, NagiLiteral, NagiProgramToken, NagiProgramTokenKind,
    },
    packrat::{ASTAssembly, ASTBuilder, EBNFTable},
};

use super::{NagiCodeASTParts, NagiCodeParseResult, NagiCodeRules};

pub(super) fn make_literal_table() -> NagiCodeRules {
    vec![
        EBNFTable::new(
            "IDENTIFIER ::= IDENTIFIER_OR_KEYWORD",
            ASTBuilder::Parsed(parse_identifier),
        ),
        EBNFTable::new(
            "INTEGER_LITERAL ::= ( 
                                 BIN_LITERAL 
                               | OCT_LITERAL 
                               | DEC_LITERAL 
                               | HEX_LITERAL 
                               ) 
                               SUFFIX_No_E?",
            ASTBuilder::Parsed(parse_integer_literal),
        ),
        EBNFTable::new(
            "FLOAT_LITERAL ::= DEC_LITERAL `.` 
                             | DEC_LITERAL `.` DEC_LITERAL (SUFFIX_NO_E)? ",
            ASTBuilder::Parsed(parse_float_literal),
        ),
        EBNFTable::new(
            "
            IDENTIFIER_OR_KEYWORD ::= XID_Start XID_Continue* | `_` XID_Continue+
            ",
            ASTBuilder::Parsed(parse_identifier_or_keyword),
        ),
    ]
}

fn parse_identifier_or_keyword(token: &NagiProgramToken) -> NagiCodeParseResult {
    let NagiProgramTokenKind::Identifier(_) = &token.token_kind else {
        return Err(ParsingError::UnexpectedToken);
    };

    Ok(Rc::new(ASTAssembly::Parts(Rc::new(NagiCodeASTParts::None))))
}

fn parse_identifier(token: &NagiProgramToken) -> NagiCodeParseResult {
    let NagiProgramTokenKind::Identifier(NagiIdentifier::Identifier(_)) = &token.token_kind else {
        return Err(ParsingError::UnexpectedToken);
    };

    Ok(Rc::new(ASTAssembly::Parts(Rc::new(NagiCodeASTParts::None))))
}

fn parse_integer_literal(token: &NagiProgramToken) -> NagiCodeParseResult {
    let NagiProgramTokenKind::Literal(NagiLiteral::Integer { value, suffix }) = &token.token_kind
    else {
        return Err(ParsingError::UnexpectedToken);
    };

    Ok(Rc::new(ASTAssembly::Parts(Rc::new(NagiCodeASTParts::None))))
}

fn parse_float_literal(token: &NagiProgramToken) -> NagiCodeParseResult {
    let NagiProgramTokenKind::Literal(NagiLiteral::Float { value, suffix }) = &token.token_kind
    else {
        return Err(ParsingError::UnexpectedToken);
    };

    Ok(Rc::new(ASTAssembly::Parts(Rc::new(NagiCodeASTParts::None))))
}
