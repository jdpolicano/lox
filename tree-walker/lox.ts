import Scanner from "./scanner.ts";
import Parser from "./parser.ts";
import Interpreter from "./interpreter.ts";
import { RuntimeError } from "./errors.ts";
import { ParseError } from "./errors.ts";

export default class Lox {
  hadError: boolean;
  hadRuntimeError: boolean;
  constructor() {
    this.hadError = false;
    this.hadRuntimeError = false;
  }

  run(src: string) {
    try {
      const tokens = new Scanner(src).scanTokens();
      const parseResult = new Parser(tokens).parse();
      if (!parseResult.ok) {
        this.printParseErrors(parseResult.errors);
        return;
      }
      const result = new Interpreter().interpret(parseResult.statements);
      return result;
    } catch (e) {
      if (e instanceof RuntimeError) {
        this.hadRuntimeError = true;
        console.error(e);
      } else {
        this.hadError = true;
        console.error(e);
      }
    }
  }

  private printParseErrors(errors: ParseError[]) {
    this.hadError = true;
    for (const error of errors) {
      console.error(error);
    }
  }
}
