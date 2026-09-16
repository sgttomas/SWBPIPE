import { defineConfig } from "@playwright/test";
import base from "./playwright.performance.config";

export default defineConfig({
  ...base,
  testMatch: "baseline-canvas-point.benchmark.ts",
  timeout: 30 * 60_000,
  projects: [{ name: "chromium-ui-foundation-baseline-canvas-points" }]
});
