use crate::interpreter::environment::Environment;
use crate::interpreter::errors::RuntimeError;
use crate::interpreter::native::{Clock, LoxFunction};
use crate::interpreter::primitive::LoxObject;
use crate::language::ast::{Expr, ExprVisitor, Stmt, StmtVisitor};
use crate::language::token::{Token, TokenType};
use std::cell::RefCell;
use std::rc::Rc;

type InterpreterResult = Result<LoxObject, RuntimeError>;

// this will eventually have state;
pub struct LoxVisitor {
    globals: Rc<RefCell<Environment>>,
    environment: Rc<RefCell<Environment>>,
}

impl LoxVisitor {
    pub fn new() -> LoxVisitor {
        let globals = Self::get_global_env();
        let environment = Environment::new_rc(Some(globals.clone()));

        LoxVisitor {
            globals,
            environment,
        }
    }

    fn get_global_env() -> Rc<RefCell<Environment>> {
        let mut env = Environment::new(None);
        env.define("clock".to_string(), LoxObject::Function(Rc::new(Clock)));
        // todo - add the rest of the native apis...
        Rc::new(RefCell::new(env))
    }

    pub fn interpret(&mut self, stmts: &[Stmt]) -> Result<(), RuntimeError> {
        for stmt in stmts {
            stmt.accept(self)?;
        }
        Ok(())
    }

    pub fn create_new_environment(&mut self) -> Rc<RefCell<Environment>> {
        Environment::new_rc(Some(self.environment.clone())) // Parent is set to original environment
    }

    pub fn set_env(&mut self, new_env: Rc<RefCell<Environment>>) {
        self.environment = new_env;
    }

    pub fn execute_block(
        &mut self,
        new_env: Rc<RefCell<Environment>>,
        statements: &[Stmt],
    ) -> InterpreterResult {
        let origin = self.environment.clone();
        self.environment = new_env;

        for stmt in statements {
            match stmt.accept(self) {
                // handle a potential break statement.
                Ok(LoxObject::Exit(v)) => {
                    self.environment = origin; // Restore original environment
                    return Ok(LoxObject::Exit(v));
                }
                Err(e) => {
                    self.environment = origin; // Restore original environment
                    return Err(e);
                }
                _ => {}
            }
        }

        self.environment = origin; // Restore original environment after block execution
        Ok(LoxObject::Nil)
    }
}

impl ExprVisitor<InterpreterResult> for LoxVisitor {
    fn visit_binary(
        &mut self,
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    ) -> InterpreterResult {
        let left = left.accept(self)?;
        let right = right.accept(self)?;
        apply_binary(left, operator, right)
    }

    fn visit_grouping(&mut self, expression: Box<Expr>) -> InterpreterResult {
        expression.accept(self)
    }

    fn visit_literal(&mut self, literal: Token) -> InterpreterResult {
        Ok(literal.literal.into())
    }

    fn visit_unary(&mut self, operator: Token, right: Box<Expr>) -> InterpreterResult {
        let right = right.accept(self)?;
        apply_unary(operator, right)
    }

    fn visit_variable(&mut self, name: Token) -> InterpreterResult {
        name.with_lexeme(|word| match self.environment.borrow_mut().get(word) {
            Some(value) => Ok(value.clone()),
            None => Err(RuntimeError::UndefinedVariable(name.clone())),
        })
    }

    fn visit_assign(&mut self, name: Token, value: Box<Expr>) -> InterpreterResult {
        let v = value.accept(self)?;
        self.environment
            .borrow_mut()
            .assign(name.lexeme.clone().unwrap(), v.clone())
            .map_err(|_| RuntimeError::UndefinedVariable(name))?;
        Ok(v)
    }

    fn visit_logical(
        &mut self,
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    ) -> InterpreterResult {
        let left = left.accept(self)?;
        let left_is_truthy = is_truthy(&left);
        match operator.token_type {
            TokenType::Or if left_is_truthy => Ok(left),
            TokenType::Or => right.accept(self),
            TokenType::And if left_is_truthy => right.accept(self),
            TokenType::And => Ok(left),
            _ => Err(RuntimeError::InvalidLogicalOp(operator)),
        }
    }

    fn visit_call(
        &mut self,
        callee: Box<Expr>,
        paren: Token,    // to do, use these...
        args: Vec<Expr>, // to do, use these...
    ) -> InterpreterResult {
        let mut eval_args = Vec::with_capacity(args.len());

        for arg in args {
            eval_args.push(arg.accept(self)?);
        }

        match callee.accept(self)? {
            LoxObject::Function(f) => f.call(self, &eval_args),
            other => Err(RuntimeError::Uncallable(other, paren)),
        }
    }

    fn visit_function(&mut self, params: Vec<Token>, body: Vec<Stmt>) -> InterpreterResult {
        let func = LoxFunction::new(None, params, body, self.environment.clone());
        Ok(LoxObject::Function(Rc::new(func)))
    }
}

impl StmtVisitor<InterpreterResult> for LoxVisitor {
    fn visit_expression(&mut self, expression: Expr) -> InterpreterResult {
        expression.accept(self)?;
        Ok(LoxObject::Nil)
    }

    fn visit_print(&mut self, expression: Expr) -> InterpreterResult {
        let value = expression.accept(self)?;
        println!("{}", value);
        Ok(LoxObject::Nil)
    }

    fn visit_var(&mut self, name: Token, initializer: Option<Expr>) -> InterpreterResult {
        let value = initializer
            .map(|e| e.accept(self))
            .unwrap_or(Ok(LoxObject::Nil))?;

        //TODO - should check that the variable isn't declared already.
        self.environment
            .borrow_mut()
            .define(name.with_lexeme(|lex| lex.to_string()), value);

        Ok(LoxObject::Nil)
    }

    fn visit_block(&mut self, statements: Vec<Stmt>) -> InterpreterResult {
        let new = self.create_new_environment();
        self.execute_block(new, &statements)
    }

    fn visit_if(
        &mut self,
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    ) -> InterpreterResult {
        let condition = condition.accept(self)?;
        if is_truthy(&condition) {
            then_branch.accept(self)
        } else if let Some(else_branch) = else_branch {
            else_branch.accept(self)
        } else {
            Ok(LoxObject::Nil)
        }
    }

    fn visit_while(&mut self, condition: Expr, body: Box<Stmt>) -> InterpreterResult {
        while is_truthy(&condition.accept(self)?) {
            if is_truthy(&body.accept(self)?) {
                break;
            }
        }
        Ok(LoxObject::Nil)
    }

    fn visit_break(&mut self, _: Token) -> InterpreterResult {
        Ok(LoxObject::Exit(Box::new(LoxObject::Nil)))
    }

    fn visit_return(&mut self, _keyword: Token, value: Option<Expr>) -> InterpreterResult {
        Ok(LoxObject::Exit(Box::new(
            value
                .map(|v| v.accept(self))
                .unwrap_or(Ok(LoxObject::Nil))?,
        )))
    }

    fn visit_function(
        &mut self,
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    ) -> InterpreterResult {
        let map_key_name = name.lexeme_or_empty();
        let func = LoxFunction::new(Some(name), params, body, self.environment.clone());
        self.environment
            .borrow_mut()
            .define(map_key_name, LoxObject::Function(Rc::new(func)));
        Ok(LoxObject::Nil)
    }
}

fn either_is_string(left: &LoxObject, right: &LoxObject) -> bool {
    match (left, right) {
        (LoxObject::String(_), _) => true,
        (_, LoxObject::String(_)) => true,
        _ => false,
    }
}

fn concatenate(left: LoxObject, right: LoxObject) -> String {
    format!("{}{}", left, right)
}

fn math_op_with_check(
    left: &LoxObject,
    right: &LoxObject,
    f: fn(f64, f64) -> f64,
) -> Result<LoxObject, ()> {
    match (left, right) {
        (LoxObject::Number(a), LoxObject::Number(b)) => Ok(LoxObject::Number(f(*a, *b))),
        _ => Err(()),
    }
}

fn math_compare_with_check(
    left: &LoxObject,
    right: &LoxObject,
    f: fn(f64, f64) -> bool,
) -> Result<LoxObject, ()> {
    match (left, right) {
        (LoxObject::Number(a), LoxObject::Number(b)) => Ok(LoxObject::Boolean(f(*a, *b))),
        _ => Err(()),
    }
}

fn apply_binary(left: LoxObject, operator: Token, right: LoxObject) -> InterpreterResult {
    let result = match operator.token_type {
        TokenType::Plus => {
            if either_is_string(&left, &right) {
                return Ok(LoxObject::String(concatenate(left, right)));
            }
            math_op_with_check(&left, &right, |a, b| a + b)
        }
        TokenType::Minus => math_op_with_check(&left, &right, |a, b| a - b),
        TokenType::Star => math_op_with_check(&left, &right, |a, b| a * b),
        TokenType::Slash => math_op_with_check(&left, &right, |a, b| a / b),
        TokenType::Greater => math_compare_with_check(&left, &right, |a, b| a > b),
        TokenType::GreaterEqual => math_compare_with_check(&left, &right, |a, b| a >= b),
        TokenType::Less => math_compare_with_check(&left, &right, |a, b| a < b),
        TokenType::LessEqual => math_compare_with_check(&left, &right, |a, b| a <= b),
        TokenType::BangEqual => Ok(LoxObject::Boolean(left != right)),
        TokenType::EqualEqual => Ok(LoxObject::Boolean(left == right)),
        _ => panic!("Unrecoverable error: invalid operator in binary expression."),
    };

    if result.is_err() {
        return get_binary_error(left, operator, right);
    }

    Ok(result.unwrap().into())
}

fn get_binary_error(left: LoxObject, operator: Token, right: LoxObject) -> InterpreterResult {
    match operator.token_type {
        TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash => Err(
            RuntimeError::InvalidMathOp(format!("{}", left), operator, format!("{}", right)),
        ),
        TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual => {
            Err(RuntimeError::InvalidComparisonOp(
                format!("{}", left),
                operator,
                format!("{}", right),
            ))
        }
        _ => panic!("Unrecoverable error: invalid operator in binary expression."),
    }
}

fn apply_unary(operator: Token, right: LoxObject) -> InterpreterResult {
    match operator.token_type {
        TokenType::Minus => match right {
            LoxObject::Number(n) => Ok(LoxObject::Number(-n)),
            _ => Err(RuntimeError::InvalidUnaryOp(operator, format!("{}", right))),
        },

        TokenType::Bang => Ok(LoxObject::Boolean(!is_truthy(&right.into()))),
        _ => panic!("Unrecoverable error: invalid operator in unary expression."),
    }
}

fn is_truthy(literal: &LoxObject) -> bool {
    match literal {
        LoxObject::Boolean(b) => *b,
        LoxObject::Nil => false,
        _ => true,
    }
}
