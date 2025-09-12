import React from "react";
import Box from "@mui/system/Box";
import Container from "@mui/system/Container";
import Stack from "@mui/system/Stack";
import { Grid, Item } from "@mui/system/Grid";
import "./Theme.css";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";

let CURRENT_GAMELIST_ASSETS = null;

async function getAsset(t) {
  return await invoke("current_asset", { assetType: t });
}

const NoGameList = () => {
  <div>No Game List Provided</div>;
};

function GameList(props) {
  const list = props.list;
  const current = props.current;

  if (!CURRENT_GAMELIST_ASSETS || current != CURRENT_GAMELIST_ASSETS.index) {
    let res = getAsset("image").then((filename) => {
      if (filename) {
        return <img class="game-image" src={convertFileSrc(filename)} />;
      } else {
        return null;
      }
    });

    let image = <div> </div>;

    if (res) {
      image = res;
    }

    CURRENT_GAMELIST_ASSETS = {
      index: current,
      image: image,
    };
  }

  if (list.length == 0) {
    return <NoGameList />;
  }

  return (
    <React.Fragment>
      {list.map((x, i) => {
        return current == i ? (
          <div key={i} id="selected" class="game">
            <span class="arrow">►</span>
            {"  "}
            {x.name}
          </div>
        ) : (
          <div class="game not-selected">{x.name}</div>
        );
      })}
    </React.Fragment>
  );
}

function Theme(props) {
  const orientation = props.orientation;
  const systems = props.systems;
  const current_system =
    systems.length > 0 && systems[orientation.system_index];

  return (
    <Container maxWidth="100%">
      <Stack spacing={2}>
        <Item className="section system-info">
          <Grid container spacing={2}>
            <Grid size={4}>
              <Item>
                <Box className="system-banner">
                  {current_system ? (
                    <img
                      class="system-banner"
                      src={`theme/${current_system.tag}.png`}
                    />
                  ) : (
                    <React.Fragment />
                  )}
                </Box>
              </Item>
            </Grid>
            <Grid size={8}>
              <Item className="system-title">
                <div className="vertical-spacer" />
                {orientation && systems.length > 0
                  ? systems[orientation.system_index].name
                  : "No Systems Loaded"}
                <div className="vertical-spacer" />
              </Item>
            </Grid>
          </Grid>
        </Item>
        <Item className="section">
          <Grid container spacing={2}>
            <Grid size={6} className="game-list">
              <Item>
                {current_system ? (
                  <GameList
                    list={current_system.gamelist}
                    current={orientation.gamelist_index}
                  />
                ) : (
                  <NoGameList />
                )}
              </Item>
            </Grid>
            <Grid size={6} className="current-game">
              <Item>
                {CURRENT_GAMELIST_ASSETS ? (
                  CURRENT_GAMELIST_ASSETS.image
                ) : (
                  <React.Fragment />
                )}
              </Item>
            </Grid>
          </Grid>
        </Item>
      </Stack>
    </Container>
  );
}

export default Theme;
