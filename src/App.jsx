import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [systems, setSystems] = useState([]);
  const [orientation, setOrientation] = useState(null);

  async function getSystems() {
    setSystems(await invoke("all_systems"));
  }

  async function getOrientation() {
    setOrientation(await invoke("current_orientation"));
  }

  useEffect(() => {
    getSystems();
    const id = setInterval(() => getSystems(), 1000);
    return () => clearInterval(id);
  }, []);

  useEffect(() => {
    getOrientation();
    const id = setInterval(() => getOrientation(), 50);
    return () => clearInterval(id);
  }, []);

  return (
    <main className="container">
      <div>
        {orientation && systems.length > 0
          ? systems[orientation.system_index].name
          : "No Systems Loaded"}
      </div>
    </main>
  );
}

export default App;
