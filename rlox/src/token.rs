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

    // One or two character tokens.
    Minus,
    MinusEqual,
    Plus,
    PlusEqual,
    Semicolon,
    Slash,
    SlashEqual,
    Star,
    StarEqual,
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
    is_synthetic: bool,
}

impl Coordinate {
    pub fn new(index: usize, line: usize, column: usize) -> Coordinate {
        Coordinate {
            index,
            line,
            column,
            is_synthetic: false,
        }
    }

    pub fn synthetic() -> Self {
        Coordinate {
            index: 0,
            line: 0,
            column: 0,
            is_synthetic: true,
        }
    }

    pub fn is_synthetic(&self) -> bool {
        self.is_synthetic
    }
}

impl Default for Coordinate {
    fn default() -> Self {
        Coordinate::new(0, 1, 1)
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
    pub lexeme: Option<String>,
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
        lexeme: Option<String>,
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

    pub fn synthetic(t: TokenType, literal: Literal) -> Self {
        Token::new(t, None, literal, Coordinate::synthetic())
    }

    pub fn with_lexeme<T, R>(&self, f: T) -> R
    where
        T: FnOnce(&str) -> R,
    {
        if let Some(ref lex) = self.lexeme {
            f(lex)
        } else {
            f("")
        }
    }
}
