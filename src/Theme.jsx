function Theme(...props){
  const orientation = props.orientation;
  const systems = props.systems;

  return (
    <div>
      {orientation && systems.length > 0
        ? systems[orientation.system_index].name
        : "No Systems Loaded"}
    </div>
  )
}

export default Theme;
