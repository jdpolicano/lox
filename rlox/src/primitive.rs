use crate::interpreter::{Interpreter, RuntimeError};
use crate::token::Literal;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum LoxObject {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Function(Rc<RefCell<dyn Callable>>),
}

impl fmt::Display for LoxObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoxObject::Number(value) => write!(f, "{}", value),
            LoxObject::String(value) => write!(f, "{}", value),
            LoxObject::Boolean(value) => write!(f, "{}", value),
            LoxObject::Nil => write!(f, "nil"),
            LoxObject::Function(_) => write!(f, "[__object__]"),
        }
    }
}

impl PartialEq for LoxObject {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LoxObject::Number(n1), LoxObject::Number(n2)) => n1 == n2,
            (LoxObject::String(s1), LoxObject::String(s2)) => s1 == s2,
            (LoxObject::Boolean(b1), LoxObject::Boolean(b2)) => b1 == b2,
            (LoxObject::Nil, LoxObject::Nil) => true,
            _ => false,
        }
    }
}

impl From<Literal> for LoxObject {
    fn from(v: Literal) -> Self {
        match v {
            Literal::Number(f) => LoxObject::Number(f),
            Literal::String(s) => LoxObject::String(s),
            Literal::Boolean(b) => LoxObject::Boolean(b),
            Literal::Nil => LoxObject::Nil,
        }
    }
}

pub trait Callable
where
    Self: fmt::Debug,
{
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: &[LoxObject],
    ) -> Result<LoxObject, RuntimeError>;

    fn arity(&self) -> usize;
}
