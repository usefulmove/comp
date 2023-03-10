import "./App.css";
import { useState } from "react";
import { Grid, Typography, TextField, Button } from "@mui/material";
import * as R from "../node_modules/ramda/";

function App() {
  const [stack, setStack] = useState([]);

  const onEnter = (exp) => {
    console.log("executing expression: ", exp);
    setStack(exp.split(" "));
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
          onKeyDown={(e) => {
            e.key == "Enter" ? onEnter(e.target.value) : {};
          }}
        />
      </Grid>
      <Grid item xs={12}>
        {[...stack].reverse().map((entry, i) => (
          <Typography
            variant="h6"
            color={i === 0 ? "#ffffff" : "#0080ff"}
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
