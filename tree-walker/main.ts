import Lox from "./lox.ts";

const lox = new Lox();

const src = `var a = "Jake";\nprint "a = " + a;\nprint(a + " loves xtina");`;

const result = lox.run(src);

if (lox.hadError) {
  Deno.exit(65);
}

if (lox.hadRuntimeError) {
  Deno.exit(70);
}

console.log(result);

Deno.exit(0);
