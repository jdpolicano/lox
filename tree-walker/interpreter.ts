import * as ast from "./ast/nodes.ts";
import Token, { Literal, TokenType } from "./token.ts";
import { RuntimeError } from "./errors.ts";
import { Stmt } from "./ast/base.ts";
import Bindings from "./bindings.ts";

type NumberCallback = (...nums: number[]) => Literal;

export default class Interpreter implements ast.AstVisitor<Literal> {
  readonly bindings: Bindings = new Bindings();

  interpret(expressions: Stmt[]) {
    for (const expr of expressions) {
      try {
        expr.accept(this);
      } catch (e) {
        if (e instanceof RuntimeError) {
          throw e;
        } else if (e instanceof Error) {
          throw RuntimeError.fromError(e);
        } else {
          throw new RuntimeError(`Interpreter unknown error: ${e}`);
        }
      }
    }
  }

  visitVariable(node: ast.Variable): Literal {
    try {
      return this.bindings.get(node.name.lexeme);
    } catch (e) {
      throw RuntimeError.fromError(e, node.name);
    }
  }

  visitVar(node: ast.Var): Literal {
    let value: Literal = null;

    if (node.initializer) {
      value = node.initializer.accept(this);
    }

    this.bindings.declare(node.name.lexeme, value);
    return null;
  }

  visitExpression(node: ast.Expression): Literal {
    node.expression.accept(this);
    return null;
  }

  visitPrint(node: ast.Print): Literal {
    const value = node.expression.accept(this);
    console.log(value);
    return null;
  }

  visitLiteral(node: ast.Literal): Literal {
    return node.value.literal;
  }

  visitGrouping(node: ast.Grouping): Literal {
    return node.expression.accept(this);
  }

  visitUrnary(node: ast.Urnary): Literal {
    const right: Literal = node.right.accept(this);

    switch (node.operator.type) {
      case TokenType.BANG:
        return !this.isTruthy(right);
      case TokenType.MINUS:
        return this.wrapCallFallback(
          node.operator,
          () => this.withNumber((a) => -a, right),
        );
      default:
        // unreachable
        throw new Error("Invalid urnary operator");
    }
  }

  visitBinary(node: ast.Binary): Literal {
    const left = node.left.accept(this);
    const right = node.right.accept(this);

    switch (node.operator.type) {
      case TokenType.PLUS: {
        if (this.eitherIsString(left, right)) {
          return this.stringConcat(left, right);
        }
        return this.wrapCallFallback(
          node.operator,
          () => this.withNumber((a, b) => a + b, left, right),
        );
      }
      case TokenType.MINUS:
        return this.wrapCallFallback(
          node.operator,
          () => this.withNumber((a, b) => a - b, left, right),
        );
      case TokenType.STAR:
        return this.wrapCallFallback(
          node.operator,
          () => this.withNumber((a, b) => a * b, left, right),
        );
      case TokenType.SLASH:
        return this.wrapCallFallback(
          node.operator,
          () => this.withNumber((a, b) => a / b, left, right),
        );
      case TokenType.GREATER:
        return this.wrapCallFallback(
          node.operator,
          () => this.withNumber((a, b) => a > b, left, right),
        );
      case TokenType.GREATER_EQUAL:
        return this.wrapCallFallback(
          node.operator,
          () => this.withNumber((a, b) => a >= b, left, right),
        );
      case TokenType.LESS:
        return this.wrapCallFallback(
          node.operator,
          () => this.withNumber((a, b) => a < b, left, right),
        );
      case TokenType.LESS_EQUAL:
        return this.wrapCallFallback(
          node.operator,
          () => this.withNumber((a, b) => a <= b, left, right),
        );
      case TokenType.BANG_EQUAL:
        return this.wrapCallFallback(
          node.operator,
          () => left !== right,
        );
      case TokenType.EQUAL_EQUAL:
        return this.wrapCallFallback(
          node.operator,
          () => left === right,
        );
      default:
        // unreachable
        throw new RuntimeError("Invalid binary operator", node.operator);
    }
  }

  private isTruthy(value: Literal): boolean {
    if (value === null) return false;
    if (value === false) return false;
    return true;
  }

  private withNumber(cb: NumberCallback, ...nums: unknown[]): Literal {
    if (
      nums.some((n) => typeof n !== "number" || Number.isNaN(n))
    ) {
      throw new RuntimeError("Operands must be numbers");
    }

    return cb(...nums as number[]);
  }

  private wrapCallFallback<T>(token: Token, call: (...args: unknown[]) => T) {
    try {
      return call();
    } catch (error) {
      if (error instanceof Error) {
        throw RuntimeError.fromError(error, token);
      }
      throw error;
    }
  }

  private eitherIsString(a: unknown, b: unknown) {
    return typeof a === "string" || typeof b === "string";
  }

  private stringConcat(a: unknown, b: unknown): Literal {
    return `${a}${b}`;
  }
}
