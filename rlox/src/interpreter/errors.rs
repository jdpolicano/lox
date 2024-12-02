use crate::interpreter::primitive::LoxObject;
use crate::language::token::Token;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum RuntimeError {
    InvalidMathOp(String, Token, String),
    InvalidComparisonOp(String, Token, String),
    InvalidUnaryOp(Token, String),
    InvalidLogicalOp(Token),
    UndefinedVariable(Token),
    Uncallable(LoxObject, Token),
    Native(String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RuntimeError: ")?;
        match self {
            RuntimeError::InvalidMathOp(left, op, right) => {
                write!(
                    f,
                    "Invalid math operation \"{} {} {}\" {}",
                    left,
                    op.with_lexeme(|lex| lex.to_string()),
                    right,
                    op.coordinate
                )
            }
            RuntimeError::InvalidComparisonOp(left, op, right) => {
                write!(
                    f,
                    "Invalid comparison operation \"{} {} {}\" {}",
                    left,
                    op.with_lexeme(|lex| lex.to_string()),
                    right,
                    op.coordinate
                )
            }
            RuntimeError::InvalidUnaryOp(op, right) => {
                write!(
                    f,
                    "Invalid unary operation \"{} {}\" {}",
                    op.with_lexeme(|lex| lex.to_string()),
                    right,
                    op.coordinate
                )
            }
            RuntimeError::UndefinedVariable(name) => {
                write!(
                    f,
                    "Undefined variable \"{}\" {}",
                    name.with_lexeme(|lex| lex.to_string()),
                    name.coordinate
                )
            }
            RuntimeError::InvalidLogicalOp(op) => {
                write!(
                    f,
                    "Invalid logical operation \"{}\" {}",
                    op.with_lexeme(|lex| lex.to_string()),
                    op.coordinate
                )
            }
            RuntimeError::Uncallable(obj, tok) => {
                write!(
                    f,
                    "Invalid call expression trying to call literal value -> {} {}",
                    obj, tok.coordinate
                )
            }
            RuntimeError::Native(s) => {
                write!(f, "{}", s)
            }
        }
    }
}
