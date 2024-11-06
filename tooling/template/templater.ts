import { Eta } from "eta";
import { parseArgs } from "@std/cli";
import { z } from "zod";
import { basename, dirname, join } from "@std/path";

interface Config {
  classes: Class[];
}

interface Class {
  name: string;
  properties: Record<string, string>;
}

interface CliArgs {
  configPath: string;
  src: string;
  outName?: string;
}

const argsSchema = z.object({
  configPath: z.string(),
  src: z.string(),
  outName: z.string().optional(),
});

const configSchema = z.object({
  classes: z.array(
    z.object({
      name: z.string(),
      extends: z.string(),
      properties: z.record(z.string()),
    }),
  ),
});

function parseCliArgs(): CliArgs {
  const args = parseArgs(Deno.args, {
    default: {
      configPath: "./template.config.json",
    },
  });

  return argsSchema.parse(args);
}

function tryRead<T>(path: string, callback: (src: string) => T): T;
function tryRead(path: string): string;

function tryRead<T>(path: string, callback?: (src: string) => T) {
  try {
    const file = Deno.readTextFileSync(path);
    if (callback) {
      return callback(file);
    }
    return file;
  } catch (e) {
    console.error("Error reading config file: ", e);
    throw e;
  }
}

function tryReadConfig(path: string): Config {
  return tryRead(path, (src) => {
    try {
      return configSchema.parse(JSON.parse(src));
    } catch (e) {
      console.error("Error parsing config file: ", e);
      Deno.exit(1);
    }
  });
}

function tryReadTemplate(path: string) {
  return tryRead(path);
}

function buildPath(srcPath: string, outputFileName: string): string {
  const srcDir = dirname(srcPath);
  return join(srcDir, `${outputFileName}.ts`);
}

function run() {
  const args = parseCliArgs();
  const config = tryReadConfig(args.configPath);
  const template = tryReadTemplate(args.src);
  const outputPath = buildPath(
    args.src,
    args.outName ?? basename(args.src, ".eta"),
  );
  const eta = new Eta();
  const res = eta.renderString(template, config);
  Deno.writeTextFileSync(outputPath, res);
}

run();
