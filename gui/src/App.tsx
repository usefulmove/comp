import "./App.css";
import { useState } from "react";
import { Grid, Typography, TextField, Button } from "@mui/material";
import * as R from "../node_modules/ramda/";
import { Commands } from "./commands";

type Stack = string[];
const C = new Commands();

const evaluateOps = (ops: Stack, stck: Stack): Stack => {
  console.log({ ops, stck });

  const out_st: Stack = R.reduce(
    (interimStack: Stack, op: string): Stack =>
      C.cmds.has(op) ? C.cmds.get(op)(interimStack) : [...interimStack, op],
    stck,
    ops
  );

  return out_st;
};

function App() {
  const [outputStack, setOutputStack] = useState([]);
  const [inputField, setInputField] = useState("");

  const exprToOps = (expr: string): Stack =>
    expr.split(" ").filter((op: string) => op.length > 0);

  const onEnter = (expr) => {
    console.log("evaluating expression: ", expr);
    setOutputStack(evaluateOps(exprToOps(expr), [])); // evaluate expression and set output stack to result
    setInputField(""); // clear input field
  };

  return (
    <Grid container padding={4} spacing={3}>
      <Grid item xs={12}>
        <Typography variant="h4" className="title" sx={{ color: "#c8c8c8" }}>
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
          <div key={i}>
            <Typography
              component="span"
              color={"#000000"}
              align="left"
              sx={{ fontSize: "12px", fontFamily: "Monospace" }}
            >
              {i + 1}.{"  "}
            </Typography>
            <Typography
              variant="h6"
              component="span"
              color={i === 0 ? "#00b0ff" : "#0080ff"}
              sx={{ fontFamily: "Monospace" }}
              align="left"
            >
              {entry}
            </Typography>
          </div>
        ))}
        <br />
        <Typography variant="body2" color="#000000">
          ( ver. 0.0.2 )
        </Typography>
      </Grid>
    </Grid>
  );
}

export default App;
