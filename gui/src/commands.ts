export class Commands {
  public cmds: Map<string, any> = new Map<string, any>();

  constructor() {
    type Stack = string[];

    this.cmds.set("+", (stck: Stack): Stack => {
      const b = stck.at(-1);
      const a = stck.at(-2);
      const rest = stck.slice(0, -2);
      return [...rest, (parseFloat(a) + parseFloat(b)).toString()];
    });

    this.cmds.set("-", (stck: Stack): Stack => {
      const b = stck.at(-1);
      const a = stck.at(-2);
      const rest = stck.slice(0, -2);
      return [...rest, (parseFloat(a) - parseFloat(b)).toString()];
    });

    this.cmds.set("x", (stck: Stack): Stack => {
      const b = stck.at(-1);
      const a = stck.at(-2);
      const rest = stck.slice(0, -2);
      return [...rest, (parseFloat(a) * parseFloat(b)).toString()];
    });

    this.cmds.set("/", (stck: Stack): Stack => {
      const b = stck.at(-1);
      const a = stck.at(-2);
      const rest = stck.slice(0, -2);
      return [...rest, (parseFloat(a) / parseFloat(b)).toString()];
    });

    this.cmds.set("dup", (stck: Stack): Stack => [...stck, stck.at(-1)]);

    this.cmds.set("sqrt", (stck: Stack): Stack => {
      const a = stck.at(-1);
      const rest = stck.slice(0, -1);
      return [...rest, Math.sqrt(parseFloat(a)).toString()];
    });

    this.cmds.set("swap", (stck: Stack): Stack => {
      const b = stck.at(-1);
      const a = stck.at(-2);
      const rest = stck.slice(0, -2);
      return [...rest, b, a];
    });

    this.cmds.set("inv", (stck: Stack): Stack => {
      const a = stck.at(-1);
      const rest = stck.slice(0, -1);
      return [...rest, (1 / parseFloat(a)).toString()];
    });
  }
}
