import React from "react";
import Box from "@mui/system/Box";
import Container from "@mui/system/Container";
import './Theme.css';

function Theme(...props){
  const orientation = props.orientation;
  const systems = props.systems;

  return (
    <Container maxWidth="sm">
      <Box>
        {orientation && systems.length > 0
          ? systems[orientation.system_index].name
          : "No Systems Loaded"}
      </Box>
      <Box>
        {orientation && systems.length > 0
          ? systems[orientation.system_index].gamelist[orientation.gamelist_index].name
          : "No Game List Loaded"}
      </Box>
    </Container>
  )
}

export default Theme;
