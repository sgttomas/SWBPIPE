import { defineConfig } from "@playwright/test";
import base from "./playwright.performance.config";

export default defineConfig({
  ...base,
  testMatch: "candidate-camera-preflight.benchmark.ts",
  workers: 1,
  fullyParallel: false,
  timeout: 10 * 60_000,
  reporter: [["list"]],
  use: {
    ...base.use,
    trace: "off",
    video: "off",
    screenshot: "off"
  },
  webServer: {
    command: "node e2e/ui-foundation/serve-bound-candidate-output.mjs",
    cwd: "../..",
    reuseExistingServer: false,
    timeout: 30_000,
    url: "http://127.0.0.1:5176"
  },
  projects: [{ name: "chromium-ui-foundation-candidate-camera-preflight" }]
});
