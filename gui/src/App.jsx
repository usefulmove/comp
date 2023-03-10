import { Grid, Typography, TextField, Button } from "@mui/material";
import "./App.css";

function App() {
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
        />
      </Grid>
      <Grid item xs={12}>
        <Typography
          variant="h6"
          color="#ffffff"
          sx={{ fontFamily: "Monospace" }}
          align="left"
        >
          0.6180340
        </Typography>
        <Typography
          variant="h6"
          color="#0080ff"
          sx={{ fontFamily: "Monospace" }}
          align="left"
        >
          1.6180340
          <br />
          1.2345678
          <br />
          512
        </Typography>
      </Grid>
    </Grid>
  );
}

export default App;
