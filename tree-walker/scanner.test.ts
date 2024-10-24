import { assertEquals, assertThrows } from "@std/assert";
import Scanner from "./scanner.ts";
import Token, { TokenType } from "./token.ts";

Deno.test("Scanner can scan tokens with newline / tab / carriage return", () => {
  const source = `(\n\n\r\t)`;
  const scanner = new Scanner(source);
  scanner.scanTokens();
  assertEquals(
    scanner.tokens[0],
    new Token({
      type: TokenType.LEFT_PAREN,
      lexeme: "(",
      literal: null,
      coordinate: { line: 1, offset: 1 },
    }),
  );

  assertEquals(
    scanner.tokens[1],
    new Token({
      type: TokenType.RIGHT_PAREN,
      lexeme: ")",
      literal: null,
      coordinate: { line: 3, offset: 3 },
    }),
  );
});

Deno.test("Scanner can scan tokens with comments", () => {
  const source = `// This is a comment\n( // Another comment\n)`;
  const scanner = new Scanner(source);
  scanner.scanTokens();
  assertEquals(
    scanner.tokens[0],
    new Token({
      type: TokenType.LEFT_PAREN,
      lexeme: "(",
      literal: null,
      coordinate: { line: 2, offset: 1 },
    }),
  );

  assertEquals(
    scanner.tokens[1],
    new Token({
      type: TokenType.RIGHT_PAREN,
      lexeme: ")",
      literal: null,
      coordinate: { line: 3, offset: 1 },
    }),
  );
});

Deno.test("Scanner can handle operators", () => {
  const source = `! != = == > >= < <=`;
  const scanner = new Scanner(source);
  scanner.scanTokens();
  assertEquals(
    scanner.tokens[0],
    new Token({
      type: TokenType.BANG,
      lexeme: "!",
      literal: null,
      coordinate: { line: 1, offset: 1 },
    }),
  );

  assertEquals(
    scanner.tokens[1],
    new Token({
      type: TokenType.BANG_EQUAL,
      lexeme: "!=",
      literal: null,
      coordinate: { line: 1, offset: 3 },
    }),
  );

  assertEquals(
    scanner.tokens[2],
    new Token({
      type: TokenType.EQUAL,
      lexeme: "=",
      literal: null,
      coordinate: { line: 1, offset: 6 },
    }),
  );

  assertEquals(
    scanner.tokens[3],
    new Token({
      type: TokenType.EQUAL_EQUAL,
      lexeme: "==",
      literal: null,
      coordinate: { line: 1, offset: 8 },
    }),
  );

  assertEquals(
    scanner.tokens[4],
    new Token({
      type: TokenType.GREATER,
      lexeme: ">",
      literal: null,
      coordinate: { line: 1, offset: 11 },
    }),
  );

  assertEquals(
    scanner.tokens[5],
    new Token({
      type: TokenType.GREATER_EQUAL,
      lexeme: ">=",
      literal: null,
      coordinate: { line: 1, offset: 13 },
    }),
  );

  assertEquals(
    scanner.tokens[6],
    new Token({
      type: TokenType.LESS,
      lexeme: "<",
      literal: null,
      coordinate: { line: 1, offset: 16 },
    }),
  );

  assertEquals(
    scanner.tokens[7],
    new Token({
      type: TokenType.LESS_EQUAL,
      lexeme: "<=",
      literal: null,
      coordinate: { line: 1, offset: 18 },
    }),
  );
});

Deno.test("Scanner can handle strings", () => {
  const src = `"Hello, World!"`;
  const scanner = new Scanner(src);
  scanner.scanTokens();
  assertEquals(
    scanner.tokens[0],
    new Token({
      type: TokenType.STRING,
      lexeme: `"Hello, World!"`,
      literal: "Hello, World!",
      coordinate: { line: 1, offset: 1 },
    }),
  );
});

Deno.test("Scanner throws is string not terminated", () => {
  const src = `"Hello, World!`;
  const scanner = new Scanner(src);
  assertThrows(() => {
    scanner.scanTokens();
  });
});

Deno.test("Scanner handles compound string sequence", () => {
  const src = `"Hello, " + "World!"`;
  const scanner = new Scanner(src);
  scanner.scanTokens();
  assertEquals(
    scanner.tokens[0],
    new Token({
      type: TokenType.STRING,
      lexeme: `"Hello, "`,
      literal: "Hello, ",
      coordinate: { line: 1, offset: 1 },
    }),
  );

  assertEquals(
    scanner.tokens[1],
    new Token({
      type: TokenType.PLUS,
      lexeme: "+",
      literal: null,
      coordinate: { line: 1, offset: 11 },
    }),
  );

  assertEquals(
    scanner.tokens[2],
    new Token({
      type: TokenType.STRING,
      lexeme: `"World!"`,
      literal: "World!",
      coordinate: { line: 1, offset: 13 },
    }),
  );
});

Deno.test("Scanner can handle integers", () => {
  const src = "1234";
  const scanner = new Scanner(src);
  scanner.scanTokens();
  assertEquals(
    scanner.tokens[0],
    new Token({
      type: TokenType.NUMBER,
      lexeme: "1234",
      literal: 1234,
      coordinate: { line: 1, offset: 1 },
    }),
  );
});

Deno.test("Scanner can handle floats", () => {
  const src = "1234.1234";
  const scanner = new Scanner(src);
  scanner.scanTokens();
  assertEquals(
    scanner.tokens[0],
    new Token({
      type: TokenType.NUMBER,
      lexeme: "1234.1234",
      literal: 1234.1234,
      coordinate: { line: 1, offset: 1 },
    }),
  );
});

Deno.test("Scanner can handle decimal values", () => {
  const src = ".1234";
  const scanner = new Scanner(src);
  scanner.scanTokens();
  assertEquals(
    scanner.tokens[0],
    new Token({
      type: TokenType.NUMBER,
      lexeme: ".1234",
      literal: .1234,
      coordinate: { line: 1, offset: 1 },
    }),
  );
});

Deno.test("Scanner can handle compound number sequence", () => {
  const src = "1234.1234 + .1234";
  const scanner = new Scanner(src);
  scanner.scanTokens();
  assertEquals(
    scanner.tokens[0],
    new Token({
      type: TokenType.NUMBER,
      lexeme: "1234.1234",
      literal: 1234.1234,
      coordinate: { line: 1, offset: 1 },
    }),
  );

  assertEquals(
    scanner.tokens[1],
    new Token({
      type: TokenType.PLUS,
      lexeme: "+",
      literal: null,
      coordinate: { line: 1, offset: 11 },
    }),
  );

  assertEquals(
    scanner.tokens[2],
    new Token({
      type: TokenType.NUMBER,
      lexeme: ".1234",
      literal: .1234,
      coordinate: { line: 1, offset: 13 },
    }),
  );
});

Deno.test("Scanner can handle identifiers", () => {
  const src = "identifier";
  const scanner = new Scanner(src);
  scanner.scanTokens();
  assertEquals(
    scanner.tokens[0],
    new Token({
      type: TokenType.IDENTIFIER,
      lexeme: "identifier",
      literal: null,
      coordinate: { line: 1, offset: 1 },
    }),
  );
});

Deno.test("Scanner can handle compound identifier sequence", () => {
  const src = "identifier + identifier";
  const scanner = new Scanner(src);
  scanner.scanTokens();
  assertEquals(
    scanner.tokens[0],
    new Token({
      type: TokenType.IDENTIFIER,
      lexeme: "identifier",
      literal: null,
      coordinate: { line: 1, offset: 1 },
    }),
  );

  assertEquals(
    scanner.tokens[1],
    new Token({
      type: TokenType.PLUS,
      lexeme: "+",
      literal: null,
      coordinate: { line: 1, offset: 12 },
    }),
  );

  assertEquals(
    scanner.tokens[2],
    new Token({
      type: TokenType.IDENTIFIER,
      lexeme: "identifier",
      literal: null,
      coordinate: { line: 1, offset: 14 },
    }),
  );
});

Deno.test("Scanner can handle keywords", () => {
  const src = "and or not";
  const scanner = new Scanner(src);
  scanner.scanTokens();
  assertEquals(
    scanner.tokens[0],
    new Token({
      type: TokenType.AND,
      lexeme: "and",
      literal: null,
      coordinate: { line: 1, offset: 1 },
    }),
  );

  assertEquals(
    scanner.tokens[1],
    new Token({
      type: TokenType.OR,
      lexeme: "or",
      literal: null,
      coordinate: { line: 1, offset: 5 },
    }),
  );

  assertEquals(
    scanner.tokens[2],
    new Token({
      type: TokenType.IDENTIFIER,
      lexeme: "not",
      literal: null,
      coordinate: { line: 1, offset: 8 },
    }),
  );
});

Deno.test("Scanner can handle booleans", () => {
  const src = "true false";
  const scanner = new Scanner(src);
  scanner.scanTokens();
  assertEquals(
    scanner.tokens[0],
    new Token({
      type: TokenType.TRUE,
      lexeme: "true",
      literal: true,
      coordinate: { line: 1, offset: 1 },
    }),
  );

  assertEquals(
    scanner.tokens[1],
    new Token({
      type: TokenType.FALSE,
      lexeme: "false",
      literal: false,
      coordinate: { line: 1, offset: 6 },
    }),
  );
});
