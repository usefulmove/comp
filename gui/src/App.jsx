import { Grid, Typography, TextField } from "@mui/material";
import "./App.css";

function App() {
  return (
    <Grid container padding={4} spacing={3}>
      <Grid item xs={12}>
        <Typography variant="h5" className="title">
          comp
        </Typography>
      </Grid>
      <Grid item xs={12}>
        <TextField
          id="expression"
          label="expression"
          variant="outlined"
          color="primary"
          sx={{ input: { color: "white" } }}
        />
      </Grid>
      <Grid item xs={12}>
        <Typography sx={{ fontFamily: "Monospace" }}>
          0.618033
          <br />
          1.618033
        </Typography>
      </Grid>
    </Grid>
  );
}

export default App;
