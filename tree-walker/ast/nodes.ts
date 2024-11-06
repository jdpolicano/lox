import { Expr, Stmt } from './base.ts';
import type Token from '../token.ts';
/**
 * this file is auto generated and should not be altered by hand.
 */

export interface AstVisitor<T> {
  visitLiteral(node: Literal): T;
  visitGrouping(node: Grouping): T;
  visitUrnary(node: Urnary): T;
  visitBinary(node: Binary): T;
  visitVariable(node: Variable): T;
  visitExpression(node: Expression): T;
  visitPrint(node: Print): T;
  visitVar(node: Var): T;
}

export class Literal extends Expr {
  value: Token;
  
  constructor(value: Token) {
    super();
    this.value = value;
  }

  accept<T>(visitor: AstVisitor<T>): T {
    return visitor.visitLiteral(this);
  }
}

export class Grouping extends Expr {
  expression: Expr;
  
  constructor(expression: Expr) {
    super();
    this.expression = expression;
  }

  accept<T>(visitor: AstVisitor<T>): T {
    return visitor.visitGrouping(this);
  }
}

export class Urnary extends Expr {
  operator: Token;
  right: Expr;
    
  constructor(operator: Token, right: Expr) {
    super();
    this.operator = operator;
    this.right = right;
  }

  accept<T>(visitor: AstVisitor<T>): T {
    return visitor.visitUrnary(this);
  }
}

export class Binary extends Expr {
  left: Expr;
  operator: Token;
  right: Expr;
      
  constructor(left: Expr, operator: Token, right: Expr) {
    super();
    this.left = left;
    this.operator = operator;
    this.right = right;
  }

  accept<T>(visitor: AstVisitor<T>): T {
    return visitor.visitBinary(this);
  }
}

export class Variable extends Expr {
  name: Token;
  
  constructor(name: Token) {
    super();
    this.name = name;
  }

  accept<T>(visitor: AstVisitor<T>): T {
    return visitor.visitVariable(this);
  }
}

export class Expression extends Stmt {
  expression: Expr;
  
  constructor(expression: Expr) {
    super();
    this.expression = expression;
  }

  accept<T>(visitor: AstVisitor<T>): T {
    return visitor.visitExpression(this);
  }
}

export class Print extends Stmt {
  expression: Expr;
  
  constructor(expression: Expr) {
    super();
    this.expression = expression;
  }

  accept<T>(visitor: AstVisitor<T>): T {
    return visitor.visitPrint(this);
  }
}

export class Var extends Stmt {
  name: Token;
  initializer?: Expr;
    
  constructor(name: Token, initializer?: Expr) {
    super();
    this.name = name;
    this.initializer = initializer;
  }

  accept<T>(visitor: AstVisitor<T>): T {
    return visitor.visitVar(this);
  }
}

