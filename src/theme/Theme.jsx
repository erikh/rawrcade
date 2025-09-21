import React from "react";
import Box from "@mui/system/Box";
import Container from "@mui/system/Container";
import Stack from "@mui/system/Stack";
import Grid from "@mui/system/Grid";
import Popover from "@mui/material/Popover";
import Switch from "@mui/material/Switch";
import "./Theme.css";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";

let CURRENT_MENU_VALUES = [];
let CURRENT_MENU_TYPES = [];
let CURRENT_MENU = [];
let CURRENT_MENU_INDEX = [];
let CURRENT_GAMELIST_ASSETS = null;

function pickSelector(i) {
  switch (CURRENT_MENU_TYPES[i]) {
    case "boolean":
      return (
        <Switch
          checked={JSON.parse(CURRENT_MENU_VALUES[i])}
          sx={{
            display: "flex",
            marginLeft: "auto",
            justifyContent: "flex-end",
          }}
        >
          {CURRENT_MENU_VALUES[i]}
        </Switch>
      );
    case "string":
      return (
        <div
          style={{
            display: "flex",
            marginLeft: "auto",
            justifyContent: "flex-end",
          }}
        >
          {JSON.parse(CURRENT_MENU_VALUES[i]) || "<None>"}
        </div>
      );
  }
}

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
    if (orientation && orientation.menu_item_index === null) {
      console.log("fetching menu");
      invoke("menu").then((x) => {
        CURRENT_MENU = x;
        CURRENT_MENU_VALUES = [];
        CURRENT_MENU_TYPES = [];
      });
    }
  }, [
    orientation && orientation.menu_item_index === null
      ? orientation.menu_active
      : null,
  ]);

  React.useEffect(() => {
    if (orientation && orientation.menu_item_index === null) {
      console.log("setting menu index");
      CURRENT_MENU_INDEX = orientation.menu_index;
    }
  }, [
    orientation &&
    orientation.menu_active &&
    orientation.menu_item_index === null
      ? orientation.menu_index
      : null,
  ]);

  React.useEffect(() => {
    if (
      orientation &&
      orientation.menu_active &&
      orientation.menu_item_index !== null
    ) {
      console.log("setting menu item index");
      CURRENT_MENU_INDEX = orientation.menu_item_index;
    }
  }, [
    orientation &&
    orientation.menu_active &&
    orientation.menu_item_index !== null
      ? orientation.menu_item_index
      : null,
  ]);

  React.useEffect(() => {
    console.log("submenu trigger");
    let interval = null;
    if (orientation && orientation.menu_item_index !== null) {
      console.log("finding submenu");
      switch (orientation.menu_index) {
        case 0: {
          console.log("fetching settings submenu & types");
          invoke("settings_menu").then((x) => {
            CURRENT_MENU = x;
            invoke("setting_types").then((x) => {
              CURRENT_MENU_TYPES = x;
              CURRENT_MENU_VALUES = [];

              CURRENT_MENU.forEach((_, x) => {
                invoke("setting_value", { setting: x }).then((value) => {
                  CURRENT_MENU_VALUES[x] = value;
                });
              });

              interval = setInterval(() => {
                CURRENT_MENU_VALUES = [];

                CURRENT_MENU.forEach((_, x) => {
                  invoke("setting_value", { setting: x }).then((value) => {
                    CURRENT_MENU_VALUES[x] = value;
                  });
                });
              }, 200);

              console.log(CURRENT_MENU_VALUES);
            });
          });
        }
      }
    }

    return () => {
      interval && clearInterval(interval);
    };
  }, [
    orientation &&
      orientation.menu_active &&
      orientation.menu_item_index !== null,
  ]);

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
                        src={`theme/${current_system.platform}.png`}
                      />
                    ) : (
                      <React.Fragment />
                    )}
                  </Box>
                </div>
              </Grid>
              <Grid size={8}>
                <div className="system-title">
                  {orientation && systems.length > 0
                    ? systems[orientation.system_index].fullname
                    : "No Systems Loaded"}
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
            CURRENT_MENU_INDEX == i ? (
              <div className="menu-item menu-selected">
                {CURRENT_MENU_TYPES[i] && CURRENT_MENU_VALUES[i] ? (
                  <React.Fragment>
                    <div>{item}</div>
                    {pickSelector(i)}
                  </React.Fragment>
                ) : (
                  <React.Fragment>{item}</React.Fragment>
                )}
              </div>
            ) : i == CURRENT_MENU_INDEX - 1 && i == 0 ? (
              <div className="menu-item menu-not-selected-previous-first-item">
                {CURRENT_MENU_TYPES[i] && CURRENT_MENU_VALUES[i] ? (
                  <React.Fragment>
                    <div>{item}</div>
                    {pickSelector(i)}
                  </React.Fragment>
                ) : (
                  <React.Fragment>{item}</React.Fragment>
                )}
              </div>
            ) : i == CURRENT_MENU_INDEX - 1 ? (
              <div className="menu-item menu-not-selected-previous-item">
                {CURRENT_MENU_TYPES[i] && CURRENT_MENU_VALUES[i] ? (
                  <React.Fragment>
                    <div>{item}</div>
                    {pickSelector(i)}
                  </React.Fragment>
                ) : (
                  <React.Fragment>{item}</React.Fragment>
                )}
              </div>
            ) : i == CURRENT_MENU_INDEX + 1 ? (
              <div className="menu-item menu-not-selected-next-item">
                {CURRENT_MENU_TYPES[i] && CURRENT_MENU_VALUES[i] ? (
                  <React.Fragment>
                    <div>{item}</div>
                    {pickSelector(i)}
                  </React.Fragment>
                ) : (
                  <React.Fragment>{item}</React.Fragment>
                )}
              </div>
            ) : i == 0 ? (
              <div className="menu-item menu-not-selected-first-item">
                {CURRENT_MENU_TYPES[i] && CURRENT_MENU_VALUES[i] ? (
                  <React.Fragment>
                    <div>{item}</div>
                    {pickSelector(i)}
                  </React.Fragment>
                ) : (
                  <React.Fragment />
                )}
              </div>
            ) : (
              <div className="menu-item menu-not-selected">
                {CURRENT_MENU_TYPES[i] && CURRENT_MENU_VALUES[i] ? (
                  <React.Fragment>
                    <div>{item}</div>
                    {pickSelector(i)}
                  </React.Fragment>
                ) : (
                  <React.Fragment>{item}</React.Fragment>
                )}
              </div>
            )
          )}
        </div>
      </Popover>
    </React.Fragment>
  );
}

export default Theme;
