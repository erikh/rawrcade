import React from "react";
import Box from "@mui/system/Box";
import Container from "@mui/system/Container";
import Stack from "@mui/system/Stack";
import Grid from "@mui/system/Grid";
import Popover from "@mui/material/Popover";
import "./Theme.css";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";

let CURRENT_MENU = [];
let CURRENT_GAMELIST_ASSETS = null;

async function getAsset(t) {
  return await invoke("current_asset", { assetType: t });
}

async function getText(t) {
  let res = await invoke("current_text", { textType: t });

  if (res && res.length >= 100) {
    res = res.substr(0, 97) + "...";
  }

  return res;
}

function NoGameList() {
  return <div>No Game List Provided</div>;
}

function populateGameListAssets(current) {
  let res = getAsset("image").then((filename) => {
    if (filename) {
      return (
        <div>
          <img class="game-image" src={convertFileSrc(filename)} />
        </div>
      );
    } else {
      return null;
    }
  });

  let image = <div> </div>;

  if (res) {
    image = res;
  }

  let description = getText("description").then((desc) =>
    desc ? <div>{desc}</div> : <div> </div>
  );

  CURRENT_GAMELIST_ASSETS = {
    index: current,
    image: image,
    description: description,
  };
}

function GameList(props) {
  const list = props.list;
  const current = props.current;

  React.useEffect(() => {
    populateGameListAssets(current || 0);
  }, [current]);

  if (!list || list.length == 0) {
    return <NoGameList />;
  }

  return (
    <div>
      {list.map((x, i) => {
        return current == i ? (
          <div key={i} id="selected" class="game">
            <span class="arrow">►</span>
            {"  "}
            {x.name}
          </div>
        ) : (
          <div key={i} class="game not-selected">
            {x.name}
          </div>
        );
      })}
    </div>
  );
}

function Theme(props) {
  const orientation = props.orientation;
  const systems = props.systems;
  const current_system =
    orientation && systems && systems.length > 0
      ? systems[orientation.system_index]
      : {};

  React.useEffect(() => {
    invoke("menu").then((x) => (CURRENT_MENU = x));
  }, [orientation && orientation.menu_active]);

  return (
    <React.Fragment>
      <Container maxWidth="100%">
        <Stack spacing={2}>
          <div className="section system-info">
            <Grid container spacing={2}>
              <Grid size={4}>
                <div>
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
                </div>
              </Grid>
              <Grid size={8}>
                <div className="system-title">
                  <div className="vertical-spacer" />
                  {orientation && systems.length > 0
                    ? systems[orientation.system_index].name
                    : "No Systems Loaded"}
                  <div className="vertical-spacer" />
                </div>
              </Grid>
            </Grid>
          </div>
          <div className="section">
            <Grid container spacing={2}>
              <Grid size={6} className="game-list">
                <div>
                  {current_system ? (
                    <GameList
                      list={current_system.gamelist}
                      current={orientation ? orientation.gamelist_index : 0}
                    />
                  ) : (
                    <NoGameList />
                  )}
                </div>
              </Grid>
              <Grid size={6} className="current-game">
                {CURRENT_GAMELIST_ASSETS ? (
                  CURRENT_GAMELIST_ASSETS.image
                ) : (
                  <div />
                )}
                {CURRENT_GAMELIST_ASSETS ? (
                  CURRENT_GAMELIST_ASSETS.description
                ) : (
                  <div />
                )}
              </Grid>
            </Grid>
          </div>
        </Stack>
      </Container>
      <Popover
        className="menu-popover"
        open={CURRENT_MENU.length > 0 && orientation && orientation.menu_active}
      >
        <div className="menu-root">
          {CURRENT_MENU.map((item, i) =>
            orientation && orientation.menu_index == i ? (
              <div className="menu-item menu-selected">{item}</div>
            ) : i == orientation.menu_index - 1 && i == 0 ? (
              <div className="menu-item menu-not-selected-previous-first-item">
                {item}
              </div>
            ) : i == orientation.menu_index - 1 ? (
              <div className="menu-item menu-not-selected-previous-item">
                {item}
              </div>
            ) : i == orientation.menu_index + 1 ? (
              <div className="menu-item menu-not-selected-next-item">
                {item}
              </div>
            ) : i == 0 ? (
              <div className="menu-item menu-not-selected-first-item">
                {item}
              </div>
            ) : (
              <div className="menu-item menu-not-selected">{item}</div>
            )
          )}
        </div>
      </Popover>
    </React.Fragment>
  );
}

export default Theme;
