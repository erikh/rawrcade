import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [system, setSystem] = useState("");

  async function getSystem() {
    setSystem(await invoke("current_system"));
  }

  useEffect(() => {
    const id = setInterval(() => nextEvent(), 50);
    return () => clearInterval(id);
  }, []);

  useEffect(() => {
    const id = setInterval(() => getSystem(), 50);
    return () => clearInterval(id);
  }, []);

  return (
    <main className="container">
      <div>{system.name}</div>
    </main>
  );
}

export default App;
