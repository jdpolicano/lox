import { generateNodes } from "./nodeBuilder.ts";
import { assertEquals } from "@std/assert";

const exampleOutput = await Deno.readTextFile("./tools/nodeBuilderOutput.txt");

const mockConfig = {
  baseClasses: [
    {
      baseClassName: "Expression",
      location: "./base.ts",
      namedExport: false,
    },
  ],

  nodes: [
    {
      className: "Literal",
      base: "Expression",
      props: [
        {
          name: "value",
          type: "Token",
        },
      ],
    },
  ],
};

Deno.test("generateNodes should return a string", () => {
  const generated = generateNodes(mockConfig);
  // we do this to ignore any extra trailing newlines the editor might add
  for (let i = 0; i < generated.length; i++) {
    assertEquals(generated[i], exampleOutput[i]);
  }
});
