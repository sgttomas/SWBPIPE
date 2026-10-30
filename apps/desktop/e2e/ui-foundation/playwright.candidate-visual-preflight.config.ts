import { defineConfig } from "@playwright/test";
import candidateBase from "./playwright.candidate-performance.config";

export default defineConfig({
  ...candidateBase,
  testMatch: "candidate-visual-point-preflight.benchmark.ts",
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
  projects: [{ name: "chromium-ui-foundation-candidate-visual-preflight" }]
});
