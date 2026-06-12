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
  // `*-dist.spec.ts` specs run against the built dist via
  // playwright.dist.config.ts (`npm run test:e2e:dist`), not against this
  // dev-server lane. (A `dist/` subfolder would match the repo .gitignore.)
  testIgnore: ["**/*-dist.spec.ts"],
  // 120s budget: with `trace: "retain-on-failure"` the recorder snapshots the
  // DOM after every action, and the smoke spec's late steps run against the
  // heaviest DOM state (solved 647-row results). Measured 2026-06-11: ~9s with
  // trace off, ~22s warm / ~37s cold with trace on. 30s was a flaky margin.
  timeout: 120_000,
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
