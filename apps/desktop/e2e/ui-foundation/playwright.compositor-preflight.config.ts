import { defineConfig } from "@playwright/test";
import base from "./playwright.performance.config";

export default defineConfig({
  ...base,
  testMatch: "compositor-trace-preflight.benchmark.ts",
  timeout: 10 * 60_000,
  use: {
    ...base.use,
    trace: "off",
    video: "off",
    screenshot: "off"
  },
  projects: [{ name: "chromium-ui-foundation-compositor-preflight" }]
});
