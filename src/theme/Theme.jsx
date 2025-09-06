import React from "react";
import Box from "@mui/system/Box";
import Container from "@mui/system/Container";
import Stack from "@mui/system/Stack";
import Grid from "@mui/system/Grid";
import './Theme.css';

function Theme(props){
  const orientation = props.orientation;
  const systems = props.systems;
  const current_system = orientation && systems.length > 0 ? systems[orientation.system_index] : null;

  return (
    <Container maxWidth="sm">
      <Stack spacing={2}>
        <div class="section system-info">
          <Grid container spacing={2}>
            <Grid size={4}>
              <Box className="system-banner">
                {current_system ?
                  <img class="system-banner" src={`theme/${current_system.tag}.png`} />
                  : <React.Fragment />
                }
              </Box>
            </Grid>
            <Grid size={8}>
              <Box className="system-title">
                {orientation && systems.length > 0
                  ? systems[orientation.system_index].name
                  : "No Systems Loaded"}
              </Box>
            </Grid>
          </Grid>
        </div>
        <div class="section">
            {current_system
              ? (
                <Stack spacing={2}>
                  {current_system.gamelist.map((x, i) => {
                    orientation.gamelist_index == i ?
                      <div class="game selected">{x.name}</div>
                      : <div class="game not-selected">{x.name}</div>
                  })}
                </Stack>
              )
              : "No Game List Loaded"}
        </div>
      </Stack>
    </Container>
  )
}

export default Theme;
