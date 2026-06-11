import { defineConfig, devices } from "@playwright/test";
import { existsSync } from "node:fs";

const configuredChrome = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH;
const macChrome = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
const executablePath =
  configuredChrome && existsSync(configuredChrome)
    ? configuredChrome
    : existsSync(macChrome)
      ? macChrome
      : undefined;

export default defineConfig({
  testDir: "./e2e",
  timeout: 30_000,
  expect: {
    timeout: 10_000
  },
  reporter: [["list"]],
  use: {
    baseURL: "http://127.0.0.1:5174",
    trace: "retain-on-failure",
    launchOptions: executablePath ? { executablePath } : undefined
  },
  webServer: {
    command: "npm run dev -- --port 5174",
    reuseExistingServer: !process.env.CI,
    timeout: 30_000,
    url: "http://127.0.0.1:5174"
  },
  projects: [
    {
      name: "chromium-desktop",
      use: {
        ...devices["Desktop Chrome"],
        viewport: { width: 1280, height: 900 }
      }
    }
  ]
});
