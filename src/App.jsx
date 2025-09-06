import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import Theme from "./theme/Theme";

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

    // FIXME: play with this interval value, could be happening way too often
    //        for big game lists
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
      <Theme systems={systems} orientation={orientation} />
    </main>
  );
}

export default App;
