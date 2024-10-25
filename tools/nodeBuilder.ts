export interface BaseConfig {
  baseClassName: string;
  location: string;
  namedExport: boolean;
}

export interface Prop {
  name: string;
  type: string;
}

export interface NodeConfig {
  className: string;
  base: string;
  props: Prop[];
}

export interface BuilderConfig {
  baseClasses: BaseConfig[];
  nodes: NodeConfig[];
}

function getBaseClassMap(baseConfigs: BaseConfig[]): Map<string, BaseConfig> {
  const baseClasses: Map<string, BaseConfig> = new Map();

  for (const base of baseConfigs) {
    baseClasses.set(base.baseClassName, base);
  }

  return baseClasses;
}

function classDec(name: string): string {
  return `export class ${name} extends Expression {\n`;
}

function propDec(name: string, type: string): string {
  return `  ${name}: ${type};\n`;
}

function constructorDec(props: Prop[]): string {
  let dec = "  constructor(";

  for (let i = 0; i < props.length; i++) {
    dec += `${props[i].name}: ${props[i].type}`;
    if (i !== props.length - 1) {
      dec += ", ";
    }
  }

  dec += ") {\n";

  dec += "    super();\n";
  for (const prop of props) {
    dec += `    this.${prop.name} = ${prop.name};\n`;
  }

  dec += "  }\n";

  return dec;
}

export function generateNodes(config: BuilderConfig): string {
  const baseClasses = getBaseClassMap(config.baseClasses);
  let output = "";

  for (const node of config.nodes) {
    const base = baseClasses.get(node.base);
    if (!base) {
      throw new Error(`Build Error: Base class ${node.base} not found`);
    }

    output += `// begin autogen for "${node.className}"\n`;
    output += classDec(node.className);
    for (const prop of node.props) {
      output += propDec(prop.name, prop.type);
    }

    output += "\n";
    output += constructorDec(node.props);
    output += "}\n";
    output += `// end autogen for "${node.className}"\n`;
  }

  return output;
}
