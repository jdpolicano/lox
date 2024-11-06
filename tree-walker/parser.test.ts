import Parser from "./parser.ts";
import Scanner from "./scanner.ts";
import PrintVisitor from "./ast/printVisitor.ts";

Deno.test("Scanner can scan tokens", () => {
  const scanner = new Scanner("1 + 2 - 1 /");
  scanner.scanTokens();

  const parser = new Parser(scanner.tokens);
  const ast = parser.expression();
  const visitor = new PrintVisitor();
  console.log(ast.accept(visitor));
});
