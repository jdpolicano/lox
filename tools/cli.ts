import { generateNodes } from "./nodeBuilder.ts";
import { z } from "zod";
import { parseArgs } from "@std/cli";

const baseConfigSchema = z.object({
  baseClassName: z.string(),
  location: z.string(),
  namedExport: z.boolean(),
});

const propSchema = z.object({
  name: z.string(),
  type: z.string(),
});

const nodeConfigSchema = z.object({
  className: z.string(),
  base: z.string(),
  props: propSchema.array(),
});

const builderConfigSchema = z.object({
  baseClasses: baseConfigSchema.array(),
  nodes: nodeConfigSchema.array(),
});

const cliArgsSchema = z.object({
  configPath: z.string(),
  output: z.string(),
});

const args = parseArgs(Deno.args, {
  default: {
    configPath: "./nodeBuilder.config.json",
  },
});

const cliArgs = cliArgsSchema.parse(args);

const config = JSON.parse(await Deno.readTextFile(cliArgs.configPath));

const finalConfig = builderConfigSchema.parse(config);

using file = await Deno.open(cliArgs.output, { write: true, create: true });
const encoder = new TextEncoder();
const writer = file.writable.getWriter();

// write the file imports that are defined in the base classes
for (const base of finalConfig.baseClasses) {
  const namedExport = base.namedExport
    ? `{ ${base.baseClassName} "`
    : `${base.baseClassName}`;

  const importString = `import ${namedExport} from "${base.location}";\n`;
  await writer.write(encoder.encode(importString));
}

// append extra newline
await writer.write(encoder.encode("\n"));

// write the class definitions
const generated = generateNodes(finalConfig);

await writer.write(encoder.encode(generated));

await writer.close();
