use crate::language::token::{Coordinate, TokenType};
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum ParseError {
    #[error(
        "ParseError: Expected token of type \"{expected:?}\" but got token \"{found:?}\", {msg} {coordinate}"
    )]
    TokenAssertionFailure {
        msg: &'static str,
        expected: TokenType,
        found: TokenType,
        coordinate: Coordinate,
    },
    #[error("ParseError: Unexpected token {token_lexeme}, {msg} {coordinate}")]
    UnexpectedToken {
        msg: &'static str,
        token_lexeme: String,
        coordinate: Coordinate,
    },
    #[error("ParseError: Unexpected end of file {after_token}")]
    UnexpectedEndOfFile { after_token: String },
    #[error("Invalid assignment target: {token_lexeme} {coordinate}")]
    InvalidAssignmentTarget {
        token_lexeme: String,
        coordinate: Coordinate,
    },
    #[error("ParseError: Likely logical error with your parser...")]
    LikelyLogicalError,
}
