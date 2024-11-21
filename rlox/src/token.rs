use std::fmt;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    False,
    Fun,
    For,
    If,
    Else,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Break,

    // End of file
    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Number(value) => write!(f, "{}", value),
            Literal::String(value) => write!(f, "{}", value),
            Literal::Boolean(value) => write!(f, "{}", value),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

impl Literal {
    pub fn is_truthy(&self) -> bool {
        match self {
            Literal::Boolean(value) => *value,
            Literal::Nil => false,
            _ => true,
        }
    }

    pub fn to_number(&self) -> Result<f64, &'static str> {
        match self {
            Literal::Number(value) => Ok(*value),
            _ => Err("Cannot convert to number"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Coordinate {
    pub index: usize,  // index into the flat buffer
    pub line: usize,   // logical line number
    pub column: usize, // logical column number
}

impl Coordinate {
    pub fn new(index: usize, line: usize, column: usize) -> Coordinate {
        Coordinate {
            index,
            line,
            column,
        }
    }
}

impl Default for Coordinate {
    fn default() -> Self {
        Coordinate::new(0, 0, 0)
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@({}:{})", self.line, self.column)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub coordinate: Coordinate,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.literal)
    }
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Literal,
        coordinate: Coordinate,
    ) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            coordinate,
        }
    }
}
