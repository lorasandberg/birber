import { useState } from "react";
import "./App.css";
import Thumbnail from "./components/Thumbnail";
import { syncAll } from "./db/sync";
import Landing from "./Landing";

function App() {
  const [text, setText] = useState<string[]>([]);

  async function runSync() {
    await syncAll();
  }

  return <Landing />;

  return (
    <main className="container">
      <button onClick={runSync}>Sync</button>
      <p style={{ display: "flex", gap: "20px", flexWrap: "wrap" }}>
        {text.map((t) => (
          <Thumbnail path={t} />
        ))}
      </p>
    </main>
  );
}

export default App;
