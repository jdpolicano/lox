use crate::ast::{Expr, Stmt};
use crate::token::{Coordinate, Literal, Token, TokenType};
use std::fmt::{self, Display};

const EQUALITIES: [TokenType; 2] = [TokenType::BangEqual, TokenType::EqualEqual];
const COMPARISONS: [TokenType; 4] = [
    TokenType::Greater,
    TokenType::GreaterEqual,
    TokenType::Less,
    TokenType::LessEqual,
];
const ADDITIONS: [TokenType; 2] = [TokenType::Minus, TokenType::Plus];
const MULTIPLICATIONS: [TokenType; 2] = [TokenType::Slash, TokenType::Star];
const URNARIES: [TokenType; 2] = [TokenType::Bang, TokenType::Minus];
const LITERALS: [TokenType; 5] = [
    TokenType::Number,
    TokenType::String,
    TokenType::True,
    TokenType::False,
    TokenType::Nil,
];

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    TokenAssertionFailure(&'static str, TokenType, Token),
    UnexpectedToken(&'static str, Token),
    UnexpectedEndOfFile(Option<Token>),
    InvalidAssignmentTarget(Token),
    LikelyLogicalError, // This is a catch-all for errors that are likely to be logical errors in the parser
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError: ")?;
        match self {
            ParseError::TokenAssertionFailure(msg, expected, token) => {
                write!(
                    f,
                    "Expected token of type \"{:?}\" but got token \"{:?}\" {}",
                    expected, token.token_type, token.coordinate
                )?;
                write!(f, "\n{}", msg)
            }
            ParseError::UnexpectedToken(msg, token) => {
                write!(f, "Unexpected token {} {}", token.lexeme, token.coordinate)?;
                write!(f, "\n{}", msg)
            }
            ParseError::UnexpectedEndOfFile(token) => {
                write!(f, "Unexpected end of file")?;
                if let Some(token) = token {
                    write!(f, " after token {} {}", token.lexeme, token.coordinate)?;
                }
                Ok(())
            }
            ParseError::LikelyLogicalError => write!(f, "Likely logical error with your parser..."),

            ParseError::InvalidAssignmentTarget(token) => {
                write!(
                    f,
                    "Invalid assignment target: {} {}",
                    token.lexeme, token.coordinate
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct TokenStream {
    tokens: Vec<Token>,
    current: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    pub fn next(&mut self) -> Option<&Token> {
        if self.current >= self.tokens.len() {
            return None;
        }
        self.current += 1;
        self.tokens.get(self.current - 1)
    }

    pub fn previous(&self) -> Option<&Token> {
        if self.current == 0 {
            return None;
        }
        self.tokens.get(self.current - 1)
    }

    pub fn take_if(&mut self, f: impl Fn(&Token) -> bool) -> Option<&Token> {
        if let Some(toke) = self.peek() {
            if f(toke) {
                return self.next();
            }
        }
        None
    }
}

pub struct Parser {
    stream: TokenStream,
    is_in_loop: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            stream: TokenStream::new(tokens),
            is_in_loop: false,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<ParseError>> {
        let mut stmts = Vec::new();
        let mut errors = Vec::new();
        while !self.is_done() {
            match self.declaration() {
                Ok(s) => stmts.push(s),
                Err(e) => {
                    errors.push(e);
                    self.syncronize();
                }
            }
        }

        if errors.is_empty() {
            Ok(stmts)
        } else {
            Err(errors)
        }
    }

    pub fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_exact(TokenType::Var).is_some() {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    pub fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self
            .expect("var statment missing identifier", TokenType::Identifier)?
            .clone();
        let initializer = if self.match_exact(TokenType::Equal).is_some() {
            Some(self.expression()?)
        } else {
            None
        };
        self.expect("unterminated var statement", TokenType::Semicolon)?;
        Ok(Stmt::Var { name, initializer })
    }

    pub fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_exact(TokenType::Print).is_some() {
            self.print_statement()
        } else if self.match_exact(TokenType::LeftBrace).is_some() {
            self.block()
        } else if self.match_exact(TokenType::If).is_some() {
            self.if_statement()
        } else if self.next_is(TokenType::While) || self.next_is(TokenType::For) {
            self.loop_statment()
        } else if self.next_is(TokenType::Break) {
            self.break_statement()
        } else {
            self.expression_statement()
        }
    }

    fn break_statement(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.take_token()?.clone();
        if !self.is_in_loop {
            return Err(ParseError::UnexpectedToken(
                "\"break\" can only occur inside a loop",
                keyword,
            ));
        }
        self.expect("unterminated \"break\"", TokenType::Semicolon)?;
        Ok(Stmt::Break { keyword })
    }

    fn loop_statment(&mut self) -> Result<Stmt, ParseError> {
        self.toggle_loop();
        let res = match self.take_token()? {
            t if t.token_type == TokenType::For => self.for_statement(),
            t if t.token_type == TokenType::While => self.while_statement(),
            _ => {
                unreachable!(
                    "loop_statement() should only have been called after a forward scan..."
                )
            }
        };
        self.toggle_loop();
        res
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.expect("for loop requires \"(...\'", TokenType::LeftParen)?;

        let intializer = if self.match_exact(TokenType::Semicolon).is_some() {
            None
        } else if self.match_exact(TokenType::Var).is_some() {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if self.match_exact(TokenType::Semicolon).is_some() {
            None
        } else {
            let cond = Some(self.expression()?);
            self.expect(
                "for loop condition missing termating \";\"",
                TokenType::Semicolon,
            )?;
            cond
        };

        let increment = if self.match_exact(TokenType::RightParen).is_some() {
            None
        } else {
            let inc = Some(Stmt::Expression {
                expression: self.expression()?,
            });
            self.expect("for loop unclosed parens", TokenType::RightParen)?;
            inc
        };

        let body = self.statement()?;
        Ok(desugar_for_loop(intializer, condition, increment, body))
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.expect("while statement requires \"(...\"", TokenType::LeftParen)?;
        let condition = self.expression()?;
        self.expect("while statement unclosed parens", TokenType::RightParen)?;
        let body = Box::new(self.statement()?);
        Ok(Stmt::While { condition, body })
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.expect("if statement requires \"(...\"", TokenType::LeftParen)?;
        let condition = self.expression()?;
        self.expect("if statement unclosed parens", TokenType::RightParen)?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_exact(TokenType::Else).is_some() {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn block(&mut self) -> Result<Stmt, ParseError> {
        let mut statements = Vec::new();
        while !(self.next_is(TokenType::RightBrace) || self.is_done()) {
            statements.push(self.declaration()?);
        }
        self.expect("unterminated block scope", TokenType::RightBrace)?;
        Ok(Stmt::Block { statements })
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let expression = self.expression()?;
        self.expect("unterminated \"print\" statement", TokenType::Semicolon)?;
        Ok(Stmt::Print { expression })
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expression = self.expression()?;
        self.expect("unterminated statement", TokenType::Semicolon)?;
        Ok(Stmt::Expression { expression })
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.logical_or()?;

        if let Some(tok) = self.match_exact(TokenType::Equal) {
            if let Expr::Variable { name } = expr {
                let value = self.assignment()?;
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            } else {
                return Err(ParseError::InvalidAssignmentTarget(tok.clone()));
            }
        }

        Ok(expr)
    }

    fn logical_or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.logical_and()?;

        while let Some(tok) = self.match_exact(TokenType::Or) {
            let operator = tok.clone();
            let right = self.logical_and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn logical_and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while let Some(tok) = self.match_exact(TokenType::And) {
            let operator = tok.clone();
            let right = self.equality()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while let Some(tok) = self.match_one_of(&EQUALITIES) {
            let operator = tok.clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut term = self.term()?;

        while let Some(tok) = self.match_one_of(&COMPARISONS) {
            let operator = tok.clone();
            let right = self.term()?;
            term = Expr::Binary {
                left: Box::new(term),
                operator,
                right: Box::new(right),
            };
        }

        Ok(term)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut factor = self.factor()?;

        while let Some(tok) = self.match_one_of(&ADDITIONS) {
            let operator = tok.clone();
            let right = self.factor()?;
            factor = Expr::Binary {
                left: Box::new(factor),
                operator,
                right: Box::new(right),
            };
        }

        Ok(factor)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut unary = self.unary()?;

        while let Some(tok) = self.match_one_of(&MULTIPLICATIONS) {
            let operator = tok.clone();
            let right = self.unary()?;
            unary = Expr::Binary {
                left: Box::new(unary),
                operator,
                right: Box::new(right),
            };
        }

        Ok(unary)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if let Some(tok) = self.match_one_of(&URNARIES) {
            let operator = tok.clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let tok = self.take_token()?.clone();

        if LITERALS.contains(&tok.token_type) {
            return Ok(Expr::Literal { value: tok.clone() });
        }

        if tok.token_type == TokenType::Identifier {
            return Ok(Expr::Variable { name: tok });
        }

        if tok.token_type == TokenType::LeftParen {
            let expr = self.expression()?;
            self.expect("unterminated left parens", TokenType::RightParen)?;
            return Ok(Expr::Grouping {
                expression: Box::new(expr),
            });
        }

        Err(ParseError::UnexpectedToken("unexpected primary", tok))
    }

    fn syncronize(&mut self) {
        while let Some(tok) = self.stream.next() {
            if tok.token_type == TokenType::Semicolon {
                return;
            }
            match tok.token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }
        }
    }

    fn toggle_loop(&mut self) {
        self.is_in_loop = !self.is_in_loop;
    }

    fn expect(&mut self, msg: &'static str, t: TokenType) -> Result<&Token, ParseError> {
        self.take_token().and_then(|tok| {
            if tok.token_type == t {
                Ok(tok)
            } else {
                Err(ParseError::TokenAssertionFailure(msg, t, tok.clone()))
            }
        })
    }

    fn next_is(&mut self, t: TokenType) -> bool {
        self.stream.peek().map_or(false, |tok| tok.token_type == t)
    }

    fn match_exact(&mut self, t: TokenType) -> Option<&Token> {
        self.stream.take_if(|tok| tok.token_type == t)
    }

    fn match_one_of(&mut self, types: &[TokenType]) -> Option<&Token> {
        self.stream.take_if(|tok| types.contains(&tok.token_type))
    }

    fn take_token(&mut self) -> Result<&Token, ParseError> {
        Ok(self.stream.next().unwrap())
    }

    fn is_done(&self) -> bool {
        if let Some(tok) = self.stream.peek() {
            tok.token_type == TokenType::Eof
        } else {
            true
        }
    }
}

// This is so the parser can inject tokens into the tree
// that aren't actually reflected in the source token stream.
fn artificial_token(t: TokenType, v: Literal) -> Token {
    Token::new(
        t,
        String::new(), // doesn't allocate
        v,
        Coordinate::default(),
    )
}

fn literal_true() -> Expr {
    Expr::Literal {
        value: artificial_token(TokenType::True, Literal::Boolean(true)),
    }
}

fn while_loop_with_increment(condition: Expr, body: Stmt, increment: Option<Stmt>) -> Stmt {
    match body {
        Stmt::Block { mut statements } => {
            increment.map(|i| statements.push(i));
            Stmt::While {
                condition,
                body: Box::new(Stmt::Block { statements }),
            }
        }

        other => {
            let mut statements = vec![other];
            increment.map(|i| statements.push(i));
            Stmt::While {
                condition,
                body: Box::new(Stmt::Block { statements }),
            }
        }
    }
}

fn desugar_for_loop(
    init: Option<Stmt>,
    condition: Option<Expr>,
    increment: Option<Stmt>,
    body: Stmt,
) -> Stmt {
    let mut statements = match init {
        Some(s) => vec![s],
        _ => vec![],
    };

    statements.push(while_loop_with_increment(
        condition.unwrap_or(literal_true()),
        body,
        increment,
    ));

    Stmt::Block { statements }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::scanner::Scanner;
    use crate::token::{Coordinate, Literal};

    fn expression_stmt(expr: Expr) -> Stmt {
        Stmt::Expression { expression: expr }
    }

    fn binary(left: Expr, operator: Token, right: Expr) -> Expr {
        Expr::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    fn unary(operator: Token, right: Expr) -> Expr {
        Expr::Unary {
            operator,
            right: Box::new(right),
        }
    }

    fn literal(value: Token) -> Expr {
        Expr::Literal { value }
    }

    fn grouping(expression: Expr) -> Expr {
        Expr::Grouping {
            expression: Box::new(expression),
        }
    }

    fn number(v: f64, i: usize, r: usize, c: usize) -> Token {
        Token::new(
            TokenType::Number,
            v.to_string(),
            Literal::Number(v),
            coordinate(i, r, c),
        )
    }

    fn string(v: String, i: usize, r: usize, c: usize) -> Token {
        Token::new(
            TokenType::String,
            format!("\"{}\"", v),
            Literal::String(v),
            coordinate(i, r, c),
        )
    }

    fn operator(t: TokenType, i: usize, r: usize, c: usize) -> Token {
        let op = match t {
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Star => "*",
            TokenType::Slash => "/",
            TokenType::Bang => "!",
            TokenType::Equal => "=",
            TokenType::EqualEqual => "==",
            TokenType::BangEqual => "!=",
            TokenType::Greater => ">",
            TokenType::GreaterEqual => ">=",
            TokenType::Less => "<",
            TokenType::LessEqual => "<=",
            _ => panic!("Invalid operator"),
        };

        Token::new(t, op.to_string(), Literal::Nil, coordinate(i, r, c))
    }

    fn coordinate(i: usize, r: usize, c: usize) -> Coordinate {
        Coordinate {
            index: i,
            line: r,
            column: c,
        }
    }

    #[test]
    fn test_basic_integration() {
        let input = "1 + 2 * 3;";
        let tokens = Scanner::new(&input).scan_tokens().unwrap();
        let tree = Parser::new(tokens).parse().unwrap();

        let expected = vec![expression_stmt(binary(
            literal(number(1.0, 0, 1, 1)),
            operator(TokenType::Plus, 2, 1, 3),
            binary(
                literal(number(2.0, 4, 1, 5)),
                operator(TokenType::Star, 6, 1, 7),
                literal(number(3.0, 8, 1, 9)),
            ),
        ))];

        assert_eq!(tree, expected);
    }

    #[test]
    fn test_unary_operator() {
        let input = "-1;";
        let tokens = Scanner::new(&input).scan_tokens().unwrap();
        let tree = Parser::new(tokens).parse().unwrap();

        let expected = vec![expression_stmt(unary(
            operator(TokenType::Minus, 0, 1, 1),
            literal(number(1.0, 1, 1, 2)),
        ))];

        assert_eq!(tree, expected);
    }

    #[test]
    fn test_grouping() {
        let input = "(1 + 2) * 3;";
        let tokens = Scanner::new(&input).scan_tokens().unwrap();
        let tree = Parser::new(tokens).parse().unwrap();

        let expected = vec![expression_stmt(binary(
            grouping(binary(
                literal(number(1.0, 1, 1, 2)),
                operator(TokenType::Plus, 3, 1, 4),
                literal(number(2.0, 5, 1, 6)),
            )),
            operator(TokenType::Star, 8, 1, 9),
            literal(number(3.0, 10, 1, 11)),
        ))];

        assert_eq!(tree, expected);
    }

    #[test]
    fn test_invalid_expression() {
        let input = "1 +;";
        let tokens = Scanner::new(&input).scan_tokens().unwrap();
        let tree = Parser::new(tokens).parse();
        assert!(tree.is_err());

        if let Err(err) = tree {
            println!("{:?}", err);
        }
    }

    #[test]
    fn test_concate_strings() {
        let input = "\"hello\" + \"world\";";
        let tokens = Scanner::new(&input).scan_tokens().unwrap();
        let tree = Parser::new(tokens).parse().unwrap();

        let expected = vec![expression_stmt(binary(
            literal(string("hello".to_string(), 0, 1, 1)),
            operator(TokenType::Plus, 8, 1, 9),
            literal(string("world".to_string(), 10, 1, 11)),
        ))];

        assert_eq!(tree, expected);
    }

    #[test]
    fn multi_line() {
        let input = "1 + \n2;";
        let tokens = Scanner::new(input).scan_tokens().unwrap();
        let tree = Parser::new(tokens).parse().unwrap();

        let expected = vec![expression_stmt(binary(
            literal(number(1.0, 0, 1, 1)),
            operator(TokenType::Plus, 2, 1, 3),
            literal(number(2.0, 5, 2, 1)),
        ))];

        assert_eq!(tree, expected)
    }
}