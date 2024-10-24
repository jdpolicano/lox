import Token, { Coordinate, Literal, TokenType } from "./token.ts";

export default class Scanner {
  private source: string;
  private coordinates: Coordinate;
  tokens: Token[] = [];

  constructor(source: string) {
    this.source = source;
    this.coordinates = {
      line: 1,
      offset: 0,
    };
  }

  scanTokens() {
    while (!this.isEof()) {
      this.scanToken();
    }
  }

  private scanToken() {
    const nextChar = this.take();

    switch (nextChar) {
      case "(":
        this.addToken(TokenType.LEFT_PAREN, nextChar);
        break;
      case ")":
        this.addToken(TokenType.RIGHT_PAREN, nextChar);
        break;
      case "{":
        this.addToken(TokenType.LEFT_BRACE, nextChar);
        break;
      case "}":
        this.addToken(TokenType.RIGHT_BRACE, nextChar);
        break;
      case ",":
        this.addToken(TokenType.COMMA, nextChar);
        break;
      case ".":
        if (this.isNumber(this.peek())) {
          this.numberToken(nextChar);
        } else {
          this.addToken(TokenType.DOT, nextChar);
        }
        break;
      case "-":
        this.addToken(TokenType.MINUS, nextChar);
        break;
      case "+":
        this.addToken(TokenType.PLUS, nextChar);
        break;
      case ";":
        this.addToken(TokenType.SEMICOLON, nextChar);
        break;
      case "*":
        this.addToken(TokenType.STAR, nextChar);
        break;
      case "!":
        if (this.peek() === "=") {
          this.addToken(TokenType.BANG_EQUAL, "!=");
          this.take();
        } else {
          this.addToken(TokenType.BANG, nextChar);
        }
        break;
      case "=":
        if (this.peek() === "=") {
          this.addToken(TokenType.EQUAL_EQUAL, "==");
          this.take();
        } else {
          this.addToken(TokenType.EQUAL, nextChar);
        }
        break;
      case "<":
        if (this.peek() === "=") {
          this.addToken(TokenType.LESS_EQUAL, "<=");
          this.take();
        } else {
          this.addToken(TokenType.LESS, nextChar);
        }
        break;
      case ">":
        if (this.peek() === "=") {
          this.addToken(TokenType.GREATER_EQUAL, ">=");
          this.take();
        } else {
          this.addToken(TokenType.GREATER, nextChar);
        }
        break;
      case "/":
        if (this.peek() === "/") {
          while (this.peek() !== "\n" && !this.isEof()) {
            this.take();
          }
        } else {
          this.addToken(TokenType.SLASH, nextChar);
        }
        break;
      case " ":
      case "\t":
      case "\r":
        break;
      case "\n":
        this.incrementLine();
        break;
      case '"':
        this.stringToken();
        break;
      default:
        if (this.isNumber(nextChar)) {
          this.numberToken(nextChar);
        } else if (this.isAlpha(nextChar)) {
          this.identifierToken(nextChar);
        } else {
          this.throwLoxLexicalError(
            `Unexpected character ${nextChar} at ${this.fromatRowCol()}`,
          );
        }
        break;
    }
  }

  private numberToken(start: string) {
    let value = start;
    const numberStartLocation = this.copyCoordinates();

    while (this.isNumber(this.peek()) || this.peek() === ".") {
      value += this.take();
    }

    this.addToken(
      TokenType.NUMBER,
      value,
      parseFloat(value),
      numberStartLocation,
    );
  }

  private stringToken() {
    const stringStartLoc = this.copyCoordinates();
    let value = "";

    while (this.peek() !== '"' && !this.isEof()) {
      if (this.peek() === "\n") {
        this.incrementLine();
      }
      value += this.take();
    }

    if (this.isEof()) {
      this.throwLoxLexicalError(
        `Unterminated string at ${this.fromatRowCol()}`,
      );
    }

    this.take();
    this.addToken(
      TokenType.STRING,
      `"${value}"`,
      value,
      stringStartLoc,
    );
  }

  private identifierToken(start: string) {
    const reservedWords: Record<string, TokenType> = {
      and: TokenType.AND,
      class: TokenType.CLASS,
      else: TokenType.ELSE,
      for: TokenType.FOR,
      fun: TokenType.FUN,
      if: TokenType.IF,
      nil: TokenType.NIL,
      or: TokenType.OR,
      print: TokenType.PRINT,
      return: TokenType.RETURN,
      super: TokenType.SUPER,
      this: TokenType.THIS,
      var: TokenType.VAR,
      while: TokenType.WHILE,
    };

    const booleans: Record<string, TokenType> = {
      true: TokenType.TRUE,
      false: TokenType.FALSE,
    };

    const identifierStartLocation = this.copyCoordinates();
    let identifier = start;

    while (this.isAlpha(this.peek()) || this.isNumber(this.peek())) {
      identifier += this.take();
    }

    if (reservedWords[identifier]) {
      this.addToken(
        reservedWords[identifier],
        identifier,
        null,
        identifierStartLocation,
      );
    } else if (booleans[identifier]) {
      this.addToken(
        booleans[identifier],
        identifier,
        booleans[identifier] === TokenType.TRUE ? true : false,
        identifierStartLocation,
      );
    } else {
      this.addToken(
        TokenType.IDENTIFIER,
        identifier,
        null,
        identifierStartLocation,
      );
    }
  }

  private isNumber(char: string) {
    return (char >= "0" && char <= "9");
  }

  private isAlpha(char: string) {
    return /[a-zA-Z_]+/.test(char);
  }

  private addToken(
    type: TokenType,
    lexeme: string,
    literal: Literal = null,
    coordinate: Coordinate = this.copyCoordinates(),
  ) {
    this.tokens.push(
      new Token({
        type,
        lexeme,
        literal,
        coordinate,
      }),
    );
  }

  private copyCoordinates(): Coordinate {
    return structuredClone(this.coordinates);
  }

  private peek() {
    if (this.isEof()) return "";
    return this.source[0];
  }

  private take() {
    const nextChar = this.peek();
    this.advance();
    this.incrementOffset();
    return nextChar;
  }

  private advance() {
    if (this.isEof()) return;
    this.source = this.source.slice(1);
  }

  private incrementOffset() {
    this.coordinates.offset++;
  }

  private incrementLine() {
    this.coordinates.line++;
    this.coordinates.offset = 0;
  }

  isEof() {
    return this.source.length === 0;
  }

  private throwLoxLexicalError(message: string) {
    throw new Error(`Lexical error: ${message}`);
  }

  private fromatRowCol() {
    return `(${this.coordinates.line}:${this.coordinates.offset})`;
  }
}
