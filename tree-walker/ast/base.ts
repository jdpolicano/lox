import type { AstVisitor } from "./node.ts";
// basically just an interface at this point. I'm not sure if I should keep it or not.
export abstract class Expr {
  abstract accept<T>(visitor: AstVisitor<T>): T;
}

export abstract class Stmt {
  abstract accept<T>(visitor: AstVisitor<T>): T;
}
