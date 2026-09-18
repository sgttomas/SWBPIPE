import { defineConfig } from "@playwright/test";
import { bindRequiredChromiumExecutable } from "./candidate-server-response";
import base from "./playwright.performance.config";

const chromium = bindRequiredChromiumExecutable();
export default defineConfig({
  ...base,
  retries: 0,
  repeatEach: 1,
  testMatch: "ui-foundation-performance.benchmark.ts",
  use: {
    ...base.use,
    launchOptions: { executablePath: chromium.executablePath, args: ["--enable-automation"] },
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
  projects: [{ name: "chromium-ui-foundation-candidate-production" }]
});
