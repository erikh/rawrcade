import React from "react";

function Theme(...props){
  const orientation = props.orientation;
  const systems = props.systems;

  return (
    <React.Fragment>
      <div>
        {orientation && systems.length > 0
          ? systems[orientation.system_index].name
          : "No Systems Loaded"}
      </div>
      <div>
        {orientation && systems.length > 0
          ? systems[orientation.system_index].gamelist[orientation.gamelist_index].name
          : "No Game List Loaded"}
      </div>
    </React.Fragment>
  )
}

export default Theme;
