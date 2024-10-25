import Expression from "./tree-walker/ast/base.ts";
import Token from "./tree-walker/token.ts";

// begin autogen for "Literal"
export class Literal extends Expression {
  value: Token;

  constructor(value: Token) {
    super();
    this.value = value;
  }
}
// end autogen for "Literal"
// begin autogen for "Binary"
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
// end autogen for "Binary"
