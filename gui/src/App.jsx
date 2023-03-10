import { useState } from "react";
import { Grid, Typography, TextField, Button } from "@mui/material";
import "./App.css";

function App() {
  const [stack, setStack] = useState([]);

  return (
    <Grid container padding={4} spacing={3}>
      <Grid item xs={12}>
        <Typography variant="h5" className="title">
          Corbin
        </Typography>
      </Grid>
      <Grid item xs={12} container>
        <TextField
          id="expression"
          label="expression"
          variant="outlined"
          color="primary"
          sx={{
            input: { color: "white", fontFamily: "Monospace" },
            width: "100%",
          }}
          focused
          onKeyDown={(e) => {
            console.log(e);
            e.key == "Enter" ? setStack(e.target.value.split(" ")) : {};
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
      </Grid>
    </Grid>
  );
}

export default App;
