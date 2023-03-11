import "./App.css";
import { useState } from "react";
import { Grid, Typography, TextField, Button } from "@mui/material";
import * as R from "../node_modules/ramda/";

const evaluateExpr: string = (expr: string[], stck: string[]): string[] => {
  console.log({expr, stck});

  const out_st: string[] = R.reduce((interimStack: string[], op: string): string => {
      let rtn: string[] = [];

      switch (op) {
        case "+": {
          const [a, b, ...rest] = interimStack;
          console.log({a, b});
          rtn = [(parseFloat(a) + parseFloat(b)).toString(), ...rest];
          console.log({rtn});
          break;
        }
        case "-": {
          const [a, b, ...rest] = interimStack;
          rtn = [(parseFloat(a) - parseFloat(b)).toString(), ...rest];
          break;
        }
        case "x": {
          const [a, b, ...rest] = interimStack;
          rtn = [(parseFloat(a) * parseFloat(b)).toString(), ...rest];
          break;
        }
        case "/": {
          const [a, b, ...rest] = interimStack;
          rtn = [(parseFloat(a) / parseFloat(b)).toString(), ...rest];
          break;
        }
        case "sqrt": {
          const [a, ...rest] = interimStack;
          rtn = [Math.sqrt(parseFloat(a)).toString(), ...rest];
          break;
        }
        default: rtn = [...interimStack, op]; // add to stack
      }

      return rtn;
    },
    stck,
    expr,
  );

  return out_st;
};

function App() {
  const [stack, setStack] = useState([]);
  const [input, setInput] = useState("");

  const onEnter = (expr) => {
    console.log("evaluating expression: ", expr);
    setStack(evaluateExpr(expr.split(' '), []));
    setInput("");
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
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => {
            e.key == "Enter" ? onEnter(e.target.value) : {};
          }}
        />
      </Grid>
      <Grid item xs={12}>
        {[...stack].reverse().map((entry, i) => (
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
