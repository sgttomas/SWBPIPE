import React from "react";
import ReactDOM from "react-dom/client";
import { App } from "./App";
import { loadWasmEngine } from "./services/wasmEngine/loadWasmEngine";
import "./styles.css";

// R2 (b7-plan Unit R): the result readers are the Rust readers in the wasm engine,
// read synchronously by the panels, so the engine is loaded before the first render.
// A failed load renders anyway: every result then reads as unsupported
// (RESULT_READER_UNAVAILABLE), and the edit loop reports the absent engine by name.
void loadWasmEngine().catch(() => undefined).finally(() => {
  ReactDOM.createRoot(document.getElementById("root")!).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );
});
