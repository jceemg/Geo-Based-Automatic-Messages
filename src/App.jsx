import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [url, setUrl] = useState("");
  const [format, setFormat] = useState("mp3");
  const [output, setOutput] = useState("");
  const [status, setStatus] = useState("");
  const [progress, setProgress] = useState("");

  async function convert() {
    setStatus("Starting conversion...");
    try {
      const result = await invoke("convert", { url, format });
      setOutput(result);
      setStatus("Done");
    } catch (e) {
      setStatus("Error: " + e);
    }
  }

  return (
    <main className="container">
      <h1>YouTube MP4 & MP3 Converter</h1>
      <div className="row" style={{ flexDirection: "column", gap: 12, alignItems: "center" }}>
        <input
          type="text"
          value={url}
          onChange={(e) => setUrl(e.currentTarget.value)}
          placeholder="Enter YouTube URL..."
          style={{ width: "80%" }}
        />
        <div className="row" style={{ gap: 8 }}>
          <label>
            <input
              type="radio"
              value="mp3"
              checked={format === "mp3"}
              onChange={() => setFormat("mp3")}
            />
            MP3
          </label>
          <label>
            <input
              type="radio"
              value="mp4"
              checked={format === "mp4"}
              onChange={() => setFormat("mp4")}
            />
            MP4
          </label>
        </div>
        <button onClick={convert} disabled={!url}>
          Convert
        </button>
        {status && <p>{status}</p>}
        {progress && <p>{progress}</p>}
        {output && <p>Saved to: {output}</p>}
      </div>
    </main>
  );
}

export default App;
