use crate::ast::Stmt;
use crate::environment::Environment;
use crate::interpreter::{Interpreter, RuntimeError};
use crate::primitive::{Callable, LoxObject};
use crate::token::Token;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

// the function type.
#[derive(Debug)]
pub struct LoxFunction {
    _name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
    closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn new(
        name: Token,
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
        interpreter: &mut Interpreter,
        args: &[LoxObject],
    ) -> Result<LoxObject, RuntimeError> {
        let fresh_env = self.closure.clone();

        for (param, value) in self.params.iter().zip(args.iter()) {
            fresh_env
                .borrow_mut()
                .define(param.with_lexeme(|l| l.to_string()), value.clone());
        }

        interpreter.execute_block(fresh_env, &self.body)
    }
}

#[derive(Debug)]
pub struct Clock;

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _: &mut Interpreter, _: &[LoxObject]) -> Result<LoxObject, RuntimeError> {
        let t = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| RuntimeError::Native(format!("{e}")))?;
        Ok(LoxObject::Number(t.as_secs_f64()))
    }
}
