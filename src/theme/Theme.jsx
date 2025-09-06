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
              <Box class="system-banner">
                {current_system ?
                  <img class="system-banner" src={`theme/${current_system.tag}.png`} />
                  : <React.Fragment />
                }
              </Box>
            </Grid>
            <Grid size={8}>
              <Box class="system-title">
                {orientation && systems.length > 0
                  ? systems[orientation.system_index].name
                  : "No Systems Loaded"}
              </Box>
            </Grid>
          </Grid>
        </div>
        <div class="section">
          <Box>
            {orientation && systems.length > 0
              ? systems[orientation.system_index].gamelist[orientation.gamelist_index].name
              : "No Game List Loaded"}
          </Box>
        </div>
      </Stack>
    </Container>
  )
}

export default Theme;
