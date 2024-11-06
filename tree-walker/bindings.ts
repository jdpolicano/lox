import { Literal } from "./token.ts"; // todo: this should become something on its own.
import { RuntimeError } from "./errors.ts";

export default class Bindings {
  // keeps track of variables that have been assigned values
  readonly bindings: Map<string, Literal> = new Map();

  constructor() {}

  declare(name: string, value: Literal) {
    this.bindings.set(name, value);
  }

  get(name: string): Literal {
    if (!this.bindings.has(name)) {
      throw new RuntimeError(`Undefined variable '${name}'`);
    }

    return this.bindings.get(name)!;
  }
}
