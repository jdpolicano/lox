use crate::interpreter::environment::Environment;
use crate::interpreter::errors::RuntimeError;
use crate::interpreter::primitive::{Callable, LoxObject};
use crate::interpreter::visitor::LoxVisitor;
use crate::language::ast::Stmt;
use crate::language::token::Token;
use std::cell::RefCell;
use std::fs::read_to_string;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

// the function type.
#[derive(Debug)]
pub struct LoxFunction {
    _name: Option<Token>,
    params: Vec<Token>,
    body: Vec<Stmt>,
    closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn new(
        name: Option<Token>,
        params: Vec<Token>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    ) -> Self {
        Self {
            _name: name,
            params,
            body,
            closure,
        }
    }
}

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &self,
        interpreter: &mut LoxVisitor,
        args: &[LoxObject],
    ) -> Result<LoxObject, RuntimeError> {
        let fresh_env = Environment::new_rc(Some(self.closure.clone()));

        for (param, value) in self.params.iter().zip(args.iter()) {
            fresh_env
                .borrow_mut()
                .define(param.lexeme_or_empty(), value.clone());
        }

        let v = interpreter.execute_block(fresh_env, &self.body)?;
        match v {
            LoxObject::Exit(v) => Ok(*v),
            _ => Ok(v),
        }
    }
}

#[derive(Debug)]
pub struct Clock;

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _: &mut LoxVisitor, _: &[LoxObject]) -> Result<LoxObject, RuntimeError> {
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| RuntimeError::Native(format!("{e}")))?;
        Ok(LoxObject::Number(t.as_secs_f64()))
    }
}
