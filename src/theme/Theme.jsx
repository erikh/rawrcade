import React from "react";
import Box from "@mui/system/Box";
import Container from "@mui/system/Container";
import Stack from "@mui/system/Stack";
import './Theme.css';

function Theme(props){
  const orientation = props.orientation;
  const systems = props.systems;

  console.log(props);

  return (
    <Container maxWidth="sm">
      <Stack spacing={2}>
        <div class="section">
          <Box>
            {orientation && systems.length > 0
              ? systems[orientation.system_index].name
              : "No Systems Loaded"}
          </Box>
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
