import Expression from "./base.ts";
import Token from "../token.ts";

export class Literal extends Expression {
  value: Token; // should be one of "NUMBER", "STRING", "BOOLEAN", "NULL"

  constructor(value: Token) {
    super();
    this.value = value;
  }
}

export class Unary extends Expression {
  operator: Token; // Should be one of "MINUS", "BANG"
  right: Expression;

  constructor(modifier: Token, right: Expression) {
    super();
    this.operator = modifier;
    this.right = right;
  }
}

export class Binary extends Expression {
  left: Expression;
  operator: Token;
  right: Expression;

  constructor(left: Expression, operator: Token, right: Expression) {
    super();
    this.left = left;
    this.operator = operator;
    this.right = right;
  }
}


export class Grouping extends Expression {
  expression: Expression;

  constructor(expression: Expression) {
    super();
    this.expression = expression;
  }
}
