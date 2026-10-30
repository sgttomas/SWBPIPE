import React from "react";
import { createRoot, type Root } from "react-dom/client";
import { App } from "../../src/App";
import * as viewportResources from "../../src/features/viewport/viewportResource";
import "../../src/styles.css";

const mountButton = document.querySelector<HTMLButtonElement>("#mount-actual-app")!;
const unmountButton = document.querySelector<HTMLButtonElement>("#unmount-actual-app")!;
const status = document.querySelector<HTMLElement>("#lifecycle-status")!;
const host = document.querySelector<HTMLElement>("#actual-app-host")!;
let root: Root | null = null;

function readOwnerSnapshot(): unknown {
  const reader = (viewportResources as any).currentOwnedViewportResourceSnapshot;
  if (typeof reader !== "function") throw new Error("source-only currentOwnedViewportResourceSnapshot export is unavailable");
  return reader();
}

function mountActualApp(): void {
  if (root) return;
  root = createRoot(host);
  root.render(<React.StrictMode><App /></React.StrictMode>);
  mountButton.disabled = true;
  unmountButton.disabled = false;
  status.textContent = "Actual App is mounted.";
}

function unmountActualApp(): void {
  if (!root) return;
  root.unmount();
  root = null;
  mountButton.disabled = false;
  unmountButton.disabled = true;
  status.textContent = "Actual App is unmounted.";
}

mountButton.addEventListener("click", mountActualApp);
unmountButton.addEventListener("click", unmountActualApp);

Object.defineProperty(globalThis, "__uifActualAppLifecycleWitnessV1", {
  configurable: false,
  enumerable: false,
  writable: false,
  value: Object.freeze({ readOwnerSnapshot })
});
