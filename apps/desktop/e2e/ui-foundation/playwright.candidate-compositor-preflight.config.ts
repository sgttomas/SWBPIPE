import { defineConfig } from "@playwright/test";
import candidateBase from "./playwright.candidate-performance.config";

// Keep the compositor inventory on the same externally hash-bound, no-store
// candidate server as the camera and full candidate runs. The test itself owns
// the explicit CDP trace; Playwright tracing and recording stay disabled.
export default defineConfig({
  ...candidateBase,
  testMatch: "compositor-trace-preflight.benchmark.ts",
  workers: 1,
  fullyParallel: false,
  timeout: 10 * 60_000,
  reporter: [["list"]],
  use: {
    ...candidateBase.use,
    trace: "off",
    video: "off",
    screenshot: "off"
  },
  projects: [{ name: "chromium-ui-foundation-candidate-compositor-preflight" }]
});
