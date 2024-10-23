import Token, { Coordinate, TokenType } from "./token.ts";

export default class Scanner {
  private source: string;
  private coordinates: Coordinate;
  tokens: Token<any>[] = [];

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
    const coordinates = this.copyCoordinates();

    if (this.matchCurr("!")) {
      this.addToken(TokenType.LEFT_PAREN);
      return;
    }

    if (currChar === ")") {
      this.addToken(TokenType.RIGHT_PAREN);
      return;
    }

    if (currChar === "{") {
      this.addToken(TokenType.LEFT_BRACE);
      return;
    }

    if (currChar === "}") {
      this.addToken(TokenType.RIGHT_BRACE);
      return;
    }

    if (currChar === ",") {
      this.addToken(TokenType.COMMA);
      return;
    }

    if (currChar === ".") {
      this.addToken(TokenType.DOT);
      return;
    }

    if (currChar === "-") {
      this.addToken(TokenType.MINUS);
      return;
    }

    if (currChar === "+") {
      this.addToken(TokenType.PLUS);
      return;
    }

    if (currChar === ";") {
      this.addToken(TokenType.SEMICOLON);
      return;
    }

    if (currChar === "*") {
      this.addToken(TokenType.STAR);
      return;
    }

    if (currChar) {
      this.safeAdvance();
    }
  }

  private addToken(
    type: TokenType,
    literal: any = null,
    coordinate: Coordinate = this.copyCoordinates(),
  ) {
    this.tokens.push(
      new Token({
        type,
        lexeme: this.source[0],
        literal,
        coordinate,
      }),
    );
  }

  private copyCoordinates(): Coordinate {
    return structuredClone(this.coordinates);
  }

  private matchCurr(matcher: string) {
    if (this.currIs(matcher)) {
      this.advance();
      return true;
    }
    return false;
  }

  private currIs(matcher: string) {
    if (this.isEof()) return false;
    return this.source[0] === matcher;
  }

  private peek() {
    return this.source[0] || "";
  }

  private advance() {
    if (this.isEof()) return;

    if (this.source[0] === "\n") {
      this.coordinates.line++;
      this.coordinates.offset = 0;
    } else {
      this.coordinates.offset++;
    }

    this.source = this.source.slice(1);
  }

  isEof() {
    return this.source.length === 0;
  }

  private safeAdvance() {
    const garbage = this.peek();

    if (this.isOkGarbage(garbage)) {
      this.advance();
      return;
    }

    this.throwLoxLexicalError(
      `Unexpected character '${garbage}' at ${this.fromatRowCol()}`,
    );
  }

  private isOkGarbage(garbage: string) {
    return garbage === " " || garbage === "\t" || garbage === "\r" ||
      garbage === "\n";
  }

  private throwLoxLexicalError(message: string) {
    throw new Error(`Lexical error: ${message}`);
  }

  private fromatRowCol() {
    return `(${this.coordinates.line}:${this.coordinates.offset})`;
  }
}
