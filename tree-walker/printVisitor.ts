import * as ast from "./ast/node.ts";

export default class PrintVisitor implements ast.AstVisitor<string> {
  visitLiteral(literal: ast.Literal) {
    return `${literal.value.toLogicalString()}`;
  }

  visitGrouping(grouping: ast.Grouping): string {
    return `(grouping ${grouping.expression.accept(this)})`;
  }

  visitUrnary(urnary: ast.Urnary): string {
    return `(${urnary.operator.toLogicalString()} ${
      urnary.right.accept(this)
    })`;
  }

  visitBinary(binary: ast.Binary): string {
    return `(${binary.operator.toLogicalString()} ${binary.left.accept(this)} ${
      binary.right.accept(this)
    })`;
  }
}
