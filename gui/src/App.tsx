import "./App.css";
import { useState } from "react";
import { Grid, Typography, TextField, Button } from "@mui/material";
import * as R from "../node_modules/ramda/";

const evaluateOps = (ops: string[], stck: string[]): string[] => {
  console.log({ ops, stck });

  const out_st: string[] = R.reduce(
    (interimStack: string[], op: string): string[] => {
      let rtn: string[] = [];

      switch (op) {
        case "+": {
          const b = interimStack.at(-1);
          const a = interimStack.at(-2);
          const rest = interimStack.slice(0, -2);
          rtn = [...rest, (parseFloat(a) + parseFloat(b)).toString()];
          break;
        }
        case "-": {
          const b = interimStack.at(-1);
          const a = interimStack.at(-2);
          const rest = interimStack.slice(0, -2);
          rtn = [(parseFloat(a) - parseFloat(b)).toString(), ...rest];
          break;
        }
        case "x": {
          const b = interimStack.at(-1);
          const a = interimStack.at(-2);
          const rest = interimStack.slice(0, -2);
          rtn = [(parseFloat(a) * parseFloat(b)).toString(), ...rest];
          break;
        }
        case "/": {
          const b = interimStack.at(-1);
          const a = interimStack.at(-2);
          const rest = interimStack.slice(0, -2);
          rtn = [(parseFloat(a) / parseFloat(b)).toString(), ...rest];
          break;
        }
        case "dup": {
          rtn = [...interimStack, interimStack.at(-1)];
          break;
        }
        case "sqrt": {
          console.log({interimStack});
          const a = interimStack.at(-1);
          const rest = interimStack.slice(0, -1);
          console.log({a, rest});
          rtn = [...rest, Math.sqrt(parseFloat(a)).toString()];
          console.log({rtn});
          break;
        }
        default:
          rtn = [...interimStack, op]; // add to stack
      }

      return rtn;
    },
    stck,
    ops
  );

  return out_st;
};

function App() {
  const [outputStack, setOutputStack] = useState([]);
  const [inputField, setInputField] = useState("");

  const exprToOps = (expr: string): string[] =>
    expr.split(" ").filter((op: string) => op.length > 0);

  const onEnter = (expr) => {
    console.log("evaluating expression: ", expr);
    setOutputStack(evaluateOps(exprToOps(expr), [])); // evaluate expression and set output stack to result
    setInputField(""); // clear input field
  };

  return (
    <Grid container padding={4} spacing={3}>
      <Grid item xs={12}>
        <Typography variant="h4" className="title" sx={{ color: "#000000" }}>
          Corbin
        </Typography>
      </Grid>
      <Grid item xs={12} container>
        <TextField
          id="expression"
          label="expression"
          variant="outlined"
          color="primary"
          placeholder="Enter an expression"
          sx={{
            input: { color: "#c8c8c8", fontFamily: "Monospace" },
            width: "100%",
          }}
          focused
          value={inputField}
          onChange={(e) => setInputField(e.target.value)}
          onKeyDown={(e) => {
            e.key == "Enter" ? onEnter(e.target.value) : {};
          }}
        />
      </Grid>
      <Grid item xs={12}>
        {[...outputStack].reverse().map((entry, i) => (
          <Typography
            variant="h6"
            color={i === 0 ? "#00c0ff" : "#0080ff"}
            sx={{ fontFamily: "Monospace" }}
            align="left"
            key={i}
          >
            {entry}
          </Typography>
        ))}
        <Typography variant="body2" color="#000000">
          ( ver. 0.0.1 )
        </Typography>
      </Grid>
    </Grid>
  );
}

export default App;
