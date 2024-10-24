export type Literal = string | number | null | boolean;

export interface Coordinate {
  line: number;
  offset: number;
}

export enum TokenType {
  // Single-character tokens.
  LEFT_PAREN,
  RIGHT_PAREN,
  LEFT_BRACE,
  RIGHT_BRACE,
  COMMA,
  DOT,
  MINUS,
  PLUS,
  SEMICOLON,
  SLASH,
  STAR,

  // One or two character tokens.
  BANG,
  BANG_EQUAL,
  EQUAL,
  EQUAL_EQUAL,
  GREATER,
  GREATER_EQUAL,
  LESS,
  LESS_EQUAL,

  // Literals.
  IDENTIFIER,
  STRING,
  NUMBER,

  // Keywords.
  AND,
  CLASS,
  ELSE,
  FALSE,
  FUN,
  FOR,
  IF,
  NIL,
  OR,
  PRINT,
  RETURN,
  SUPER,
  THIS,
  TRUE,
  VAR,
  WHILE,

  EOF,
}

export interface TokenOptions {
  type: TokenType;
  lexeme: string;
  literal: Literal;
  coordinate: Coordinate;
}

export default class Token {
  private readonly type: TokenType;
  private readonly lexeme: string;
  private readonly literal: Literal;
  private readonly coordinate: Coordinate;

  constructor(opts: TokenOptions) {
    this.type = opts.type;
    this.lexeme = opts.lexeme;
    this.literal = opts.literal;
    this.coordinate = opts.coordinate;
  }

  public toString(): string {
    return `(${this.coordinate.line}:${this.coordinate.offset}) ${
      TokenType[this.type]
    } "${this.lexeme}" ${this.literal}`;
  }
}
