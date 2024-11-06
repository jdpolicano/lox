import type Token from "./token.ts";

export class ParseError extends Error {
  #token?: Token;
  raw_message: string;

  constructor(message: string, token?: Token) {
    super(ParseError.getMessage(message, token));
    this.name = "ParseError";
    this.raw_message = message;
    this.#token = token;
  }

  static getMessage(message: string, token?: Token) {
    const location = token?.toCoordinateString();
    if (location) {
      return `${message} at ${location}`;
    } else {
      return message;
    }
  }

  static fromError(e: unknown, token?: Token) {
    if (e instanceof ParseError) {
      return e;
    } else if (e instanceof Error) {
      return new ParseError(e.message, token);
    } else {
      throw new ParseError(`Interpreter unknown error: ${e}`);
    }
  }
}

export class RuntimeError extends Error {
  #token?: Token;
  raw_message: string;

  constructor(message: string, token?: Token) {
    super(RuntimeError.getMessage(message, token));
    this.raw_message = message;
    this.name = "RuntimeError";
    this.#token = token;
  }

  static getMessage(message: string, token?: Token) {
    const location = token?.toCoordinateString();
    if (location) {
      return `${message} at ${location}`;
    } else {
      return message;
    }
  }

  static fromError(error: unknown, token?: Token) {
    if (error instanceof RuntimeError) {
      return new RuntimeError(error.raw_message, token);
    }
    if (error instanceof Error) {
      return new RuntimeError(error.message, token);
    }
    return new RuntimeError("Unknown runtime error type ${e}", token);
  }
}
