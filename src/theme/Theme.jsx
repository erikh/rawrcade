import React from "react";
import Box from "@mui/system/Box";
import Container from "@mui/system/Container";
import Stack from "@mui/system/Stack";
import Grid from "@mui/system/Grid";
import './Theme.css';

const NO_GAME_LIST = () => { <div>No Game List Provided</div> };

function GameList(props) {
  const list = props.list;
  const current = props.current;

  if (list.length == 0) {
    return <NO_GAME_LIST />;
  }

  return (
    <React.Fragment>
      {
        list.map((x, i) => {
          return current == i ?
            <div class="game selected"><span class="arrow">►</span>{"  "}{x.name}</div>
            : <div class="game not-selected">{x.name}</div>
      })
      }
    </React.Fragment>
  )
}

function Theme(props){
  const orientation = props.orientation;
  const systems = props.systems;
  const current_system = systems.length > 0 && systems[orientation.system_index];

  return (
    <Container maxWidth="100%">
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
                <div class="system-title">
                    <div class="vertical-spacer" />
                      {orientation && systems.length > 0
                        ? systems[orientation.system_index].name
                        : "No Systems Loaded"}
                    <div class="vertical-spacer" />
                </div>
            </Grid>
          </Grid>
        </div>
        <div class="section">
          {
            current_system ?
            <GameList list={current_system.gamelist} current={orientation.gamelist_index} />
            :
            <NO_GAME_LIST />
          }
        </div>
      </Stack>
    </Container>
  )
}

export default Theme;
