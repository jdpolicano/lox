import Token, { TokenType } from "./token.ts";
import * as ast from "./ast/nodes.ts";
import { Expr, Stmt } from "./ast/base.ts";
import { ParseError } from "./errors.ts";

const equalities = [
  TokenType.EQUAL_EQUAL,
  TokenType.BANG_EQUAL,
];

const comparisons = [
  TokenType.LESS,
  TokenType.LESS_EQUAL,
  TokenType.GREATER,
  TokenType.GREATER_EQUAL,
];

const terms = [
  TokenType.PLUS,
  TokenType.MINUS,
];

const factors = [
  TokenType.STAR,
  TokenType.SLASH,
];

const urnaries = [
  TokenType.BANG,
  TokenType.MINUS,
];

const literals = [
  TokenType.STRING,
  TokenType.NUMBER,
  TokenType.TRUE,
  TokenType.FALSE,
  TokenType.NIL,
];

export interface ParseSuccess {
  ok: true;
  statements: Stmt[];
}

export interface ParseFailure {
  ok: false;
  errors: ParseError[];
}

export type ParseResult = ParseSuccess | ParseFailure;

export default class Parser {
  private idx: number = 0;
  private tokens: Token[];

  constructor(tokens: Token[]) {
    this.idx = 0;
    this.tokens = tokens;
  }

  parse(): ParseResult {
    const statements: (Stmt | ParseError)[] = [];

    while (!this.isAtEnd()) {
      const dec = this.declaration();
      statements.push(dec);
    }

    if (statements.some((stmt) => stmt instanceof ParseError)) {
      return {
        ok: false,
        errors: statements.filter((stmt) =>
          stmt instanceof ParseError
        ) as ParseError[],
      };
    }

    return {
      ok: true,
      statements: statements as Stmt[],
    };
  }

  private declaration(): Stmt | ParseError {
    try {
      if (this.match(TokenType.VAR)) {
        return this.varDeclaration();
      }
      return this.statement();
    } catch (e) {
      this.synchronize();
      return ParseError.fromError(e);
    }
  }

  private varDeclaration(): Stmt {
    const name = this.expect(TokenType.IDENTIFIER);

    if (name instanceof ParseError) {
      throw name;
    }

    let initializer;
    if (this.match(TokenType.EQUAL)) {
      initializer = this.expression();
    }

    const tail = this.expect(TokenType.SEMICOLON);

    if (tail instanceof ParseError) {
      throw tail;
    }

    return new ast.Var(name, initializer);
  }

  private statement(): Stmt {
    if (this.match(TokenType.PRINT)) {
      return this.printStatement();
    }

    return this.expressionStatement();
  }

  private printStatement(): Stmt {
    const value = this.expression();
    const tail = this.expect(TokenType.SEMICOLON);
    if (tail instanceof ParseError) {
      throw tail;
    }
    return new ast.Print(value);
  }

  private expressionStatement(): Stmt {
    const value = this.expression();
    const tail = this.expect(TokenType.SEMICOLON);
    if (tail instanceof ParseError) {
      throw tail;
    }
    return new ast.Expression(value);
  }

  private expression(): Expr {
    return this.equality();
  }

  private equality(): Expr {
    let expr = this.comparison();

    while (this.match(...equalities)) {
      const operator = this.previous();
      const right = this.comparison();
      expr = new ast.Binary(expr, operator, right);
    }

    return expr;
  }

  private comparison(): Expr {
    let expr = this.term();

    while (this.match(...comparisons)) {
      const operator = this.previous();
      const right = this.term();
      expr = new ast.Binary(expr, operator, right);
    }

    return expr;
  }

  private term(): Expr {
    let expr = this.factor();

    while (this.match(...terms)) {
      const operator = this.previous();
      const right = this.factor();
      expr = new ast.Binary(expr, operator, right);
    }

    return expr;
  }

  private factor(): Expr {
    let expr = this.unary();

    while (this.match(...factors)) {
      const operator = this.previous();
      const right = this.unary();
      expr = new ast.Binary(expr, operator, right);
    }

    return expr;
  }

  private unary(): Expr {
    if (this.match(...urnaries)) {
      const operator = this.previous();
      const right = this.unary();
      return new ast.Urnary(operator, right);
    }

    return this.primary();
  }

  private primary(): Expr {
    if (this.match(...literals)) {
      return new ast.Literal(this.previous());
    }

    if (this.match(TokenType.IDENTIFIER)) {
      return new ast.Variable(this.previous());
    }

    if (this.match(TokenType.LEFT_PAREN)) {
      const expr = this.expression();
      const next = this.expect(TokenType.RIGHT_PAREN);
      if (next instanceof ParseError) {
        throw next;
      }
      return new ast.Grouping(expr);
    }

    const error = this.error(
      "Unexpected fallthrow error - you should fix this later jake",
      this.previous(),
    );
    throw error;
  }

  private match(...types: TokenType[]) {
    const toke = this.take();
    if (!toke) return false;
    if (types.includes(toke.type)) {
      return true;
    }
    this.rewind();
    return false;
  }

  private take(): Token | undefined {
    if (this.idx === this.tokens.length) return;
    return this.tokens[this.idx++];
  }

  private previous(): Token {
    return this.tokens[this.idx - 1];
  }

  private rewind() {
    this.idx--;
  }

  private expect(t: TokenType): Token | ParseError {
    if (!this.match(t)) {
      return this.error(
        `Expected token: ${this.getTokenName(t)}`,
        this.previous(),
      );
    }
    return this.previous() as Token; // it must be a token because match returned true
  }

  private error(msg: string, token: Token): ParseError {
    if (this.idx === this.tokens.length) {
      return new ParseError(
        `Unexpected end of input ${token.toLogicalString()}`,
        token,
      );
    }

    return new ParseError(msg, token);
  }

  private getTokenName(t: TokenType): string {
    return TokenType[t];
  }

  private isAtEnd(): boolean {
    return this.idx === this.tokens.length;
  }

  private synchronize() {
    this.take();

    while (!this.isAtEnd()) {
      if (this.previous().type === TokenType.SEMICOLON) return;
      switch (this.take()?.type) {
        case TokenType.CLASS:
        case TokenType.FUN:
        case TokenType.VAR:
        case TokenType.FOR:
        case TokenType.IF:
        case TokenType.WHILE:
        case TokenType.PRINT:
        case TokenType.RETURN:
          return;
      }
    }
  }
}
