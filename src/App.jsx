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
    const element = document.getElementById("selected");
    if (element) {
      element.scrollIntoView();
    }
  }

  useEffect(() => {
    const effect = async () => {
      await getSystems();
    };
    effect();
  }, []);

  useEffect(() => {
    const effect = async () => {
      await getOrientation();
      const id = setInterval(async () => await getOrientation(), 50);
      return () => clearInterval(id);
    };
    effect();
  }, []);

  return <Theme systems={systems} orientation={orientation} />;
}

export default App;
