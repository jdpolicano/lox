use crate::token::{Coordinate, Literal, Token, TokenType};

use std::iter::{Iterator, Peekable};
use std::str::CharIndices;

#[derive(Debug, Clone)]
struct CharWithCoordinate<'a> {
    chars: CharIndices<'a>,
    line: usize,
    column: usize,
}

impl<'a> CharWithCoordinate<'a> {
    pub fn new(src: &'a str) -> CharWithCoordinate<'a> {
        CharWithCoordinate {
            chars: src.char_indices(),
            line: 1,
            column: 1,
        }
    }
}

impl Iterator for CharWithCoordinate<'_> {
    type Item = (char, Coordinate);

    fn next(&mut self) -> Option<(char, Coordinate)> {
        let (index, ch) = self.chars.next()?;
        let coordinate = Coordinate::new(index, self.line, self.column);

        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        Some((ch, coordinate))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LexicalError {
    InvalidCharacter(char, Coordinate),
    InvalidNumber(Coordinate),
    UnterminatedString(Coordinate),
    UnexpectedEndOfFile,
}

#[derive(Debug, Clone)]
pub struct Scanner<'a> {
    chars: Peekable<CharWithCoordinate<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(src: &'a str) -> Scanner<'a> {
        Scanner {
            chars: CharWithCoordinate::new(src).peekable(),
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, LexicalError> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            let token = self.scan_token()?;
            if let Some(toke) = token {
                tokens.push(toke);
            }
        }

        if let Some(t) = tokens.last() {
            if t.token_type != TokenType::Eof {
                tokens.push(Token::new(
                    TokenType::Eof,
                    None,
                    Literal::Nil,
                    t.coordinate.clone(),
                ));
            }
        }

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Option<Token>, LexicalError> {
        let (ch, coordinate) = self.take()?;

        match ch {
            '(' => Ok(self.simple_token(TokenType::LeftParen, (ch, coordinate))),
            ')' => Ok(self.simple_token(TokenType::RightParen, (ch, coordinate))),
            '{' => Ok(self.simple_token(TokenType::LeftBrace, (ch, coordinate))),
            '}' => Ok(self.simple_token(TokenType::RightBrace, (ch, coordinate))),
            ',' => Ok(self.simple_token(TokenType::Comma, (ch, coordinate))),
            '.' => {
                if self.next_is_digit() {
                    return self.number(String::from('.'), coordinate);
                }

                Ok(self.simple_token(TokenType::Dot, (ch, coordinate)))
            }
            '-' => {
                let toke = if self.match_char('=') {
                    self.multi_char_token(TokenType::MinusEqual, "-=".to_string(), coordinate)
                } else {
                    self.simple_token(TokenType::Minus, (ch, coordinate))
                };
                Ok(toke)
            }
            '+' => {
                let toke = if self.match_char('=') {
                    self.multi_char_token(TokenType::PlusEqual, "+=".to_string(), coordinate)
                } else {
                    self.simple_token(TokenType::Plus, (ch, coordinate))
                };
                Ok(toke)
            }
            ';' => Ok(self.simple_token(TokenType::Semicolon, (ch, coordinate))),
            '*' => {
                let toke = if self.match_char('=') {
                    self.multi_char_token(TokenType::StarEqual, "*=".to_string(), coordinate)
                } else {
                    self.simple_token(TokenType::Plus, (ch, coordinate))
                };
                Ok(toke)
            }
            '!' => {
                let toke = if self.match_char('=') {
                    self.multi_char_token(TokenType::BangEqual, "!=".to_string(), coordinate)
                } else {
                    self.simple_token(TokenType::Bang, (ch, coordinate))
                };
                Ok(toke)
            }
            '=' => {
                let toke = if self.match_char('=') {
                    self.multi_char_token(TokenType::EqualEqual, "==".to_string(), coordinate)
                } else {
                    self.simple_token(TokenType::Equal, (ch, coordinate))
                };
                Ok(toke)
            }
            '<' => {
                let toke = if self.match_char('=') {
                    self.multi_char_token(TokenType::LessEqual, "<=".to_string(), coordinate)
                } else {
                    self.simple_token(TokenType::Less, (ch, coordinate))
                };
                Ok(toke)
            }
            '>' => {
                let toke = if self.match_char('=') {
                    self.multi_char_token(TokenType::GreaterEqual, ">=".to_string(), coordinate)
                } else {
                    self.simple_token(TokenType::Greater, (ch, coordinate))
                };
                Ok(toke)
            }
            '/' => {
                if self.match_char('/') {
                    let _ = self.skip_comment();
                    if self.is_at_end() {
                        return Ok(None);
                    }
                    Ok(self.scan_token()?)
                } else if self.match_char('=') {
                    Ok(self.multi_char_token(TokenType::SlashEqual, "/=".to_string(), coordinate))
                } else {
                    Ok(self.simple_token(TokenType::Slash, (ch, coordinate)))
                }
            }
            ' ' | '\r' | '\t' | '\n' => {
                if self.is_at_end() {
                    return Ok(None);
                } else {
                    return Ok(self.scan_token()?);
                }
            }
            '"' => self.string(String::from('"'), coordinate),
            _ => {
                if ch.is_digit(10) {
                    self.number(ch.to_string(), coordinate)
                } else if ch.is_alphabetic() || ch == '_' {
                    self.identifier(ch.to_string(), coordinate)
                } else {
                    Err(LexicalError::InvalidCharacter(ch, coordinate))
                }
            }
        }
    }

    fn string(
        &mut self,
        mut lexeme: String,
        start_corrdinate: Coordinate,
    ) -> Result<Option<Token>, LexicalError> {
        while let Ok((ch, _)) = self.take() {
            lexeme.push(ch);
            if ch == '"' {
                let literal = Literal::String(String::from(&lexeme[1..lexeme.len() - 1]));
                return Ok(Some(Token::new(
                    TokenType::String,
                    Some(lexeme),
                    literal,
                    start_corrdinate,
                )));
            }
        }
        Err(LexicalError::UnterminatedString(start_corrdinate))
    }

    fn number(
        &mut self,
        mut lexeme: String,
        start_coordinate: Coordinate,
    ) -> Result<Option<Token>, LexicalError> {
        while self.next_is_digit() {
            let (ch, _) = self.take()?;
            lexeme.push(ch);
        }

        if self.match_char('.') {
            if !self.next_is_digit() {
                return Err(LexicalError::InvalidNumber(start_coordinate));
            }

            lexeme.push('.');
            while self.next_is_digit() {
                let (ch, _) = self.take()?;
                lexeme.push(ch);
            }
        }

        let literal = Literal::Number(self.parse_number(&lexeme, start_coordinate.clone())?);

        Ok(Some(Token::new(
            TokenType::Number,
            Some(lexeme),
            literal,
            start_coordinate,
        )))
    }

    fn identifier(
        &mut self,
        identifier: String,
        start_coordinate: Coordinate,
    ) -> Result<Option<Token>, LexicalError> {
        let mut lexeme = identifier;

        while self.next_is_alphanumeric() {
            let (ch, _) = self.take()?;
            lexeme.push(ch);
        }

        let token_type = match lexeme.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            "break" => TokenType::Break,
            _ => TokenType::Identifier,
        };

        let literal = match token_type {
            TokenType::True => Literal::Boolean(true),
            TokenType::False => Literal::Boolean(false),
            _ => Literal::Nil,
        };

        Ok(Some(Token::new(
            token_type,
            Some(lexeme),
            literal,
            start_coordinate,
        )))
    }

    fn parse_number(&self, num: &str, coordinate: Coordinate) -> Result<f64, LexicalError> {
        num.parse::<f64>()
            .map_err(|_| LexicalError::InvalidNumber(coordinate))
    }

    fn skip_comment(&mut self) {
        while let Ok((ch, _)) = self.take() {
            if ch == '\n' || self.is_at_end() {
                return;
            }
        }
    }

    fn multi_char_token(
        &mut self,
        token_type: TokenType,
        s: String,
        coordinate: Coordinate,
    ) -> Option<Token> {
        Some(Token::new(token_type, Some(s), Literal::Nil, coordinate))
    }

    fn simple_token(&mut self, token_type: TokenType, ch: (char, Coordinate)) -> Option<Token> {
        Some(Token::new(
            token_type,
            Some(ch.0.to_string()),
            Literal::Nil,
            ch.1,
        ))
    }

    fn is_at_end(&mut self) -> bool {
        self.chars.peek().is_none()
    }

    fn next_is_digit(&mut self) -> bool {
        if let Some(&(ch, _)) = self.peek() {
            return ch.is_digit(10);
        }
        false
    }

    fn next_is_alphanumeric(&mut self) -> bool {
        if let Some(&(ch, _)) = self.peek() {
            return ch.is_alphanumeric() || ch == '_';
        }
        false
    }

    /// will match a certain token and consume it if it is the next token returning true
    /// else it will return false
    fn match_char(&mut self, expected: char) -> bool {
        if let Some(&(ch, _)) = self.peek() {
            if ch == expected {
                self.chars.next();
                return true;
            }
        }
        false
    }

    /// takes a character from the iterator and returns it
    /// This assumes a character is available and returns and error if it is not
    fn take(&mut self) -> Result<(char, Coordinate), LexicalError> {
        self.chars.next().ok_or(LexicalError::UnexpectedEndOfFile)
    }

    /// peeks at the next character in the iterator
    fn peek(&mut self) -> Option<&(char, Coordinate)> {
        self.chars.peek()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn make_token(
        token_type: TokenType,
        lexeme: &str,
        literal: Literal,
        index: usize,
        line: usize,
        column: usize,
    ) -> Token {
        Token::new(
            token_type,
            Some(lexeme.to_string()),
            literal,
            Coordinate::new(index, line, column),
        )
    }

    #[test]
    fn test_empty_input() {
        let scanner = Scanner::new("");
        let tokens = scanner.scan_tokens().unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_whitespace_only() {
        let scanner = Scanner::new("   \t\n  ");
        let tokens = scanner.scan_tokens().unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_single_character_tokens() {
        let input = "(){},.-+;*!";
        let scanner = Scanner::new(input);
        let tokens = scanner.scan_tokens().unwrap();

        let expected_tokens = vec![
            make_token(TokenType::LeftParen, "(", Literal::Nil, 0, 1, 1),
            make_token(TokenType::RightParen, ")", Literal::Nil, 1, 1, 2),
            make_token(TokenType::LeftBrace, "{", Literal::Nil, 2, 1, 3),
            make_token(TokenType::RightBrace, "}", Literal::Nil, 3, 1, 4),
            make_token(TokenType::Comma, ",", Literal::Nil, 4, 1, 5),
            make_token(TokenType::Dot, ".", Literal::Nil, 5, 1, 6),
            make_token(TokenType::Minus, "-", Literal::Nil, 6, 1, 7),
            make_token(TokenType::Plus, "+", Literal::Nil, 7, 1, 8),
            make_token(TokenType::Semicolon, ";", Literal::Nil, 8, 1, 9),
            make_token(TokenType::Star, "*", Literal::Nil, 9, 1, 10),
            make_token(TokenType::Bang, "!", Literal::Nil, 10, 1, 11),
            make_token(TokenType::Eof, "", Literal::Nil, 10, 1, 11),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_multi_character_tokens() {
        let input = "!= == <= >= //";
        let scanner = Scanner::new(input);
        let tokens = scanner.scan_tokens().unwrap();

        let expected_tokens = vec![
            make_token(TokenType::BangEqual, "!=", Literal::Nil, 0, 1, 1),
            make_token(TokenType::EqualEqual, "==", Literal::Nil, 3, 1, 4),
            make_token(TokenType::LessEqual, "<=", Literal::Nil, 6, 1, 7),
            make_token(TokenType::GreaterEqual, ">=", Literal::Nil, 9, 1, 10),
            make_token(TokenType::Eof, "", Literal::Nil, 9, 1, 10),
        ];

        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn test_number_parsing() {
        let inputs = vec![
            ("123", 0, 1, 1),
            ("123.456", 0, 1, 1),
            (".789", 0, 1, 1),
            ("0.001", 0, 1, 1),
            ("42.", 0, 1, 1), // Invalid number: ends with dot
        ];

        for (input, index, line, column) in inputs {
            let scanner = Scanner::new(input);
            let result = scanner.scan_tokens();

            if input.ends_with('.') {
                // Expect an error
                assert!(result.is_err());
                assert_eq!(
                    result.unwrap_err(),
                    LexicalError::InvalidNumber(Coordinate::new(index, line, column))
                );
            } else {
                let tokens = result.unwrap();
                assert_eq!(tokens.len(), 2);
                let expected_literal = Literal::Number(input.parse::<f64>().unwrap());
                let expected_token = make_token(
                    TokenType::Number,
                    input,
                    expected_literal,
                    index,
                    line,
                    column,
                );
                assert_eq!(tokens[0], expected_token);
            }
        }
    }

    #[test]
    fn test_identifier_and_keyword_parsing() {
        let input = "and class else false for fun if nil or print return super this true var while identifier _identifier ident123";
        let scanner = Scanner::new(input);
        let tokens = scanner.scan_tokens().unwrap();

        let expected_token_types = vec![
            TokenType::And,
            TokenType::Class,
            TokenType::Else,
            TokenType::False,
            TokenType::For,
            TokenType::Fun,
            TokenType::If,
            TokenType::Nil,
            TokenType::Or,
            TokenType::Print,
            TokenType::Return,
            TokenType::Super,
            TokenType::This,
            TokenType::True,
            TokenType::Var,
            TokenType::While,
            TokenType::Identifier,
            TokenType::Identifier,
            TokenType::Identifier,
            TokenType::Eof,
        ];

        assert_eq!(tokens.len(), expected_token_types.len());

        for (token, expected_type) in tokens.iter().zip(expected_token_types.iter()) {
            assert_eq!(token.token_type, *expected_type);
        }
    }

    #[test]
    fn test_comment_handling() {
        let input = "var x = 42; // This is a comment\nvar y = x;";
        let scanner = Scanner::new(input);
        let tokens = scanner.scan_tokens().unwrap();

        let expected_token_types = vec![
            TokenType::Var,
            TokenType::Identifier,
            TokenType::Equal,
            TokenType::Number,
            TokenType::Semicolon,
            TokenType::Var,
            TokenType::Identifier,
            TokenType::Equal,
            TokenType::Identifier,
            TokenType::Semicolon,
            TokenType::Eof,
        ];

        assert_eq!(tokens.len(), expected_token_types.len());

        for (token, expected_type) in tokens.iter().zip(expected_token_types.iter()) {
            assert_eq!(token.token_type, *expected_type);
        }
    }

    #[test]
    fn test_error_handling_invalid_character() {
        let input = "@";
        let scanner = Scanner::new(input);
        let result = scanner.scan_tokens();
        assert!(result.is_err());

        let expected_error = LexicalError::InvalidCharacter('@', Coordinate::default());
        assert_eq!(result.unwrap_err(), expected_error);
    }

    #[test]
    fn test_error_handling_unterminated_string() {
        let input = "\"This string never ends...";
        let scanner = Scanner::new(input);
        let result = scanner.scan_tokens();
        assert!(result.is_err());

        let expected_error = LexicalError::UnterminatedString(Coordinate::default());
        assert_eq!(result.unwrap_err(), expected_error);
    }

    #[test]
    fn test_error_handling_invalid_number() {
        let input = "123abc";
        let scanner = Scanner::new(input);
        let result = scanner.scan_tokens();
        assert!(result.is_ok());

        let tokens = result.unwrap();
        // Should parse '123' as a number and 'abc' as an identifier
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[1].token_type, TokenType::Identifier);
    }

    #[test]
    fn test_complex_input() {
        let input = r#"
        // Sample code
        class Point {
            var x;
            var y;

            fun init(x, y) {
                this.x = x;
                this.y = y;
            }

            fun distance(other) {
                return sqrt((this.x - other.x) * (this.x - other.x) +
                            (this.y - other.y) * (this.y - other.y));
            }
        }
        "#;

        let scanner = Scanner::new(input);
        let result = scanner.scan_tokens();
        assert!(result.is_ok());

        let tokens = result.unwrap();

        // For brevity, we'll just check that certain tokens are present
        let token_types: Vec<TokenType> = tokens.iter().map(|t| t.token_type.clone()).collect();

        let expected_token_types = vec![
            TokenType::Class,
            TokenType::Identifier,
            TokenType::LeftBrace,
            TokenType::Var,
            TokenType::Identifier,
            TokenType::Semicolon,
            TokenType::Var,
            TokenType::Identifier,
            TokenType::Semicolon,
            TokenType::Fun,
            TokenType::Identifier,
            TokenType::LeftParen,
            TokenType::Identifier,
            TokenType::Comma,
            TokenType::Identifier,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::This,
            TokenType::Dot,
            TokenType::Identifier,
            TokenType::Equal,
            TokenType::Identifier,
            TokenType::Semicolon,
            TokenType::This,
            TokenType::Dot,
            TokenType::Identifier,
            TokenType::Equal,
            TokenType::Identifier,
            TokenType::Semicolon,
            TokenType::RightBrace,
            TokenType::Fun,
            TokenType::Identifier,
            TokenType::LeftParen,
            TokenType::Identifier,
            TokenType::RightParen,
            TokenType::LeftBrace,
            TokenType::Return,
            TokenType::Identifier, // sqrt
            TokenType::LeftParen,
            TokenType::LeftParen,
            TokenType::This,
            TokenType::Dot,
            TokenType::Identifier,
            TokenType::Minus,
            TokenType::Identifier,
            TokenType::Dot,
            TokenType::Identifier,
            TokenType::RightParen,
            TokenType::Star,
            TokenType::LeftParen,
            TokenType::This,
            TokenType::Dot,
            TokenType::Identifier,
            TokenType::Minus,
            TokenType::Identifier,
            TokenType::Dot,
            TokenType::Identifier,
            TokenType::RightParen,
            TokenType::Plus,
            TokenType::LeftParen,
            TokenType::This,
            TokenType::Dot,
            TokenType::Identifier,
            TokenType::Minus,
            TokenType::Identifier,
            TokenType::Dot,
            TokenType::Identifier,
            TokenType::RightParen,
            TokenType::Star,
            TokenType::LeftParen,
            TokenType::This,
            TokenType::Dot,
            TokenType::Identifier,
            TokenType::Minus,
            TokenType::Identifier,
            TokenType::Dot,
            TokenType::Identifier,
            TokenType::RightParen,
            TokenType::RightParen,
            TokenType::Semicolon,
            TokenType::RightBrace,
            TokenType::RightBrace,
            TokenType::Eof,
        ];

        assert_eq!(token_types, expected_token_types);
    }
}
