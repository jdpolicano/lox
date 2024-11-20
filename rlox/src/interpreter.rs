use crate::ast::{Expr, ExprVisitor, Stmt, StmtVisitor};
use crate::environment::Environment;
use crate::token::{Literal, Token, TokenType};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

type InterpreterResult = Result<Literal, RuntimeError>;

// this will eventually have state;
pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Rc::new(RefCell::new(Environment::new(None))),
        }
    }

    pub fn interpret(&mut self, stmts: &[Stmt]) -> Result<(), RuntimeError> {
        for stmt in stmts {
            stmt.accept(self)?;
        }
        Ok(())
    }
}

impl ExprVisitor<InterpreterResult> for Interpreter {
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
        Ok(literal.literal)
    }

    fn visit_unary(&mut self, operator: Token, right: Box<Expr>) -> InterpreterResult {
        let right = right.accept(self)?;
        apply_unary(operator, right)
    }

    fn visit_variable(&mut self, name: Token) -> InterpreterResult {
        match self.environment.borrow_mut().get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => Err(RuntimeError::UndefinedVariable(name)),
        }
    }

    fn visit_assign(&mut self, name: Token, value: Box<Expr>) -> InterpreterResult {
        let v = value.accept(self)?;
        self.environment
            .borrow_mut()
            .assign(name.lexeme.clone(), v.clone())
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

        match operator.token_type {
            TokenType::Or => {
                if is_truthy(&left) {
                    Ok(left)
                } else {
                    Ok(right.accept(self)?)
                }
            }

            TokenType::And => {
                if is_truthy(&left) {
                    Ok(right.accept(self)?)
                } else {
                    Ok(left)
                }
            }
            _ => Err(RuntimeError::InvalidLogicalOp(operator)),
        }
    }
}

impl StmtVisitor<InterpreterResult> for Interpreter {
    fn visit_expression(&mut self, expression: Expr) -> InterpreterResult {
        expression.accept(self)?;
        Ok(Literal::Nil)
    }

    fn visit_print(&mut self, expression: Expr) -> InterpreterResult {
        let value = expression.accept(self)?;
        println!("{}", value);
        Ok(value)
    }

    fn visit_var(&mut self, name: Token, initializer: Option<Expr>) -> InterpreterResult {
        let value = if let Some(expr) = initializer {
            expr.accept(self)?
        } else {
            Literal::Nil
        };

        self.environment.borrow_mut().define(name.lexeme, value);
        Ok(Literal::Nil)
    }

    fn visit_block(&mut self, statements: Vec<Stmt>) -> InterpreterResult {
        let origin = self.environment.clone(); // Capture the original environment
        let new_env = Environment::new(Some(origin.clone())); // Parent is set to original environment
        self.environment = Rc::new(RefCell::new(new_env));

        for stmt in statements {
            match stmt.accept(self) {
                Ok(_) => {}
                Err(e) => {
                    self.environment = origin; // Restore original environment
                    return Err(e);
                }
            }
        }

        self.environment = origin; // Restore original environment after block execution
        Ok(Literal::Nil)
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
            Ok(Literal::Nil)
        }
    }

    fn visit_while(&mut self, condition: Expr, body: Box<Stmt>) -> InterpreterResult {
        while is_truthy(&condition.accept(self)?) {
            body.accept(self)?;
        }
        Ok(Literal::Nil)
    }
}

fn either_is_string(left: &Literal, right: &Literal) -> bool {
    match (left, right) {
        (Literal::String(_), _) => true,
        (_, Literal::String(_)) => true,
        _ => false,
    }
}

fn concatenate(left: Literal, right: Literal) -> String {
    format!("{}{}", left, right)
}

fn math_op_with_check(
    left: &Literal,
    right: &Literal,
    f: fn(f64, f64) -> f64,
) -> Result<Literal, ()> {
    match (left, right) {
        (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Number(f(*a, *b))),
        _ => Err(()),
    }
}

fn math_compare_with_check(
    left: &Literal,
    right: &Literal,
    f: fn(f64, f64) -> bool,
) -> Result<Literal, ()> {
    match (left, right) {
        (Literal::Number(a), Literal::Number(b)) => Ok(Literal::Boolean(f(*a, *b))),
        _ => Err(()),
    }
}

fn apply_binary(left: Literal, operator: Token, right: Literal) -> InterpreterResult {
    let result = match operator.token_type {
        TokenType::Plus => {
            if either_is_string(&left, &right) {
                return Ok(Literal::String(concatenate(left, right)));
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
        TokenType::BangEqual => Ok(Literal::Boolean(left != right)),
        TokenType::EqualEqual => Ok(Literal::Boolean(left == right)),
        _ => panic!("Unrecoverable error: invalid operator in binary expression."),
    };

    if result.is_err() {
        return get_binary_error(left, operator, right);
    }

    Ok(result.unwrap())
}

fn get_binary_error(left: Literal, operator: Token, right: Literal) -> InterpreterResult {
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

fn apply_unary(operator: Token, right: Literal) -> InterpreterResult {
    match operator.token_type {
        TokenType::Minus => match right {
            Literal::Number(n) => Ok(Literal::Number(-n)),
            _ => Err(RuntimeError::InvalidUnaryOp(operator, format!("{}", right))),
        },

        TokenType::Bang => Ok(Literal::Boolean(!is_truthy(&right))),
        _ => panic!("Unrecoverable error: invalid operator in unary expression."),
    }
}

fn is_truthy(literal: &Literal) -> bool {
    match literal {
        Literal::Boolean(b) => *b,
        Literal::Nil => false,
        _ => true,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum RuntimeError {
    InvalidMathOp(String, Token, String),
    InvalidComparisonOp(String, Token, String),
    InvalidUnaryOp(Token, String),
    InvalidLogicalOp(Token),
    UndefinedVariable(Token),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RuntimeError: ")?;
        match self {
            RuntimeError::InvalidMathOp(left, op, right) => {
                write!(
                    f,
                    "Invalid math operation \"{} {} {}\" {}",
                    left, op.lexeme, right, op.coordinate
                )
            }
            RuntimeError::InvalidComparisonOp(left, op, right) => {
                write!(
                    f,
                    "Invalid comparison operation \"{} {} {}\" {}",
                    left, op.lexeme, right, op.coordinate
                )
            }
            RuntimeError::InvalidUnaryOp(op, right) => {
                write!(
                    f,
                    "Invalid unary operation \"{} {}\" {}",
                    op.lexeme, right, op.coordinate
                )
            }

            RuntimeError::UndefinedVariable(name) => {
                write!(
                    f,
                    "Undefined variable \"{}\" {}",
                    name.lexeme, name.coordinate
                )
            }

            RuntimeError::InvalidLogicalOp(op) => {
                write!(
                    f,
                    "Invalid logical operation \"{}\" {}",
                    op.lexeme, op.coordinate
                )
            }
        }
    }
}
