import * as R from "../node_modules/ramda/";

type Stack = string[];
type Op = string;

const getNumber = (stck: Stack): [number, Stack] => {
  const n: number = parseFloat(stck.at(-1));
  const rest: Stack = stck.slice(0, -1);
  return [n, rest];
};

const getNumber2 = (stck: Stack): [number, number, Stack] => {
  const [b, restb] = getNumber(stck);
  const [a, rest] = getNumber(restb);
  return [a, b, rest];
};

export class Commands {
  public evaluateOps =
    (ops: Ops) =>
    (stck: Stack): Stack => {
      console.log({ ops, stck });

      const out_st: Stack = R.reduce(
        (interimStack: Stack, op: Op): Stack =>
          this.cmds.has(op)
            ? this.cmds.get(op)(interimStack)
            : [...interimStack, op],
        stck,
        ops
      );

      return out_st;
    };

  cmds: Map<string, any> = new Map<string, any>();

  constructor() {
    this.cmds.set("cls", (stck: Stack): Stack => []);

    this.cmds.set("+", (stck: Stack): Stack => {
      const [a, b, rest] = getNumber2(stck);
      return [...rest, (parseFloat(a) + parseFloat(b)).toString()];
    });

    this.cmds.set("-", (stck: Stack): Stack => {
      const [a, b, rest] = getNumber2(stck);
      return [...rest, (parseFloat(a) - parseFloat(b)).toString()];
    });

    this.cmds.set("x", (stck: Stack): Stack => {
      const [a, b, rest] = getNumber2(stck);
      return [...rest, (parseFloat(a) * parseFloat(b)).toString()];
    });

    this.cmds.set("/", (stck: Stack): Stack => {
      const [a, b, rest] = getNumber2(stck);
      return [...rest, (a / b).toString()];
    });

    this.cmds.set("^", (stck: Stack): Stack => {
      const [a, b, rest] = getNumber2(stck);
      return [...rest, Math.pow(a, b).toString()];
    });

    this.cmds.set("pi", (stck: Stack): Stack => [...stck, Math.PI.toString()]);

    this.cmds.set("dup", (stck: Stack): Stack => [...stck, stck.at(-1)]);

    this.cmds.set("drop", (stck: Stack): Stack => [...stck.slice(0, -1)]);
    this.cmds.set("dropn", (stck: Stack): Stack => {
      const [a, rest] = getNumber(stck);
      return [...rest.slice(0, -a)];
    });

    this.cmds.set("sqrt", (stck: Stack): Stack => {
      const [a, rest] = getNumber(stck);
      return [...rest, Math.sqrt(a).toString()];
    });

    this.cmds.set("swap", (stck: Stack): Stack => {
      const [a, b, rest] = getNumber2(stck);
      return [...rest, b, a];
    });

    this.cmds.set("inv", (stck: Stack): Stack => {
      const [a, rest] = getNumber(stck);
      return [...rest, (1 / a).toString()];
    });
  }
}
