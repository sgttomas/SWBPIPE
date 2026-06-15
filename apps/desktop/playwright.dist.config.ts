import { defineConfig, devices } from "@playwright/test";
import { existsSync } from "node:fs";

// TP-APP-R2-WASMPKG-001 production-dist e2e lane: runs against the BUILT
// `dist/` via `vite preview` (not the dev server), so it exercises the same
// static-asset layout the packaged app serves — the lane that would have
// caught the wasm-engine assets never reaching `dist`. The tauri:// asset
// protocol itself is not reachable from a browser; that last hop stays on
// the packaged manual pass (TP-MAC-141).
const configuredChrome = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH;
const macChrome = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
const executablePath =
  configuredChrome && existsSync(configuredChrome)
    ? configuredChrome
    : existsSync(macChrome)
      ? macChrome
      : undefined;

// Worker cap: identical rationale to playwright.config.ts — run e2e workers
// serially under the DEC-025 sweep's machine load so slow Chromium teardown
// does not trip Playwright's 300s worker-stop grace ("worker process did not
// exit within 300000ms after stop, force-killed it"). The sweep sets
// PLAYWRIGHT_WORKERS=1; standalone local runs keep Playwright's smart default.
// CI runs serial too. An explicit PLAYWRIGHT_WORKERS value wins.
const parsedWorkers = Number(process.env.PLAYWRIGHT_WORKERS);
const workers =
  Number.isInteger(parsedWorkers) && parsedWorkers > 0
    ? parsedWorkers
    : process.env.CI
      ? 1
      : undefined;

export default defineConfig({
  testDir: "./e2e",
  workers,
  // Only the production-dist specs; the dev-server lane (playwright.config.ts)
  // ignores this same pattern, so each spec runs in exactly one lane.
  testMatch: ["**/*-dist.spec.ts"],
  timeout: 60_000,
  expect: {
    timeout: 10_000
  },
  reporter: [["list"]],
  use: {
    baseURL: "http://127.0.0.1:5175",
    trace: "retain-on-failure",
    launchOptions: executablePath ? { executablePath } : undefined
  },
  webServer: {
    // `vite preview` serves the prebuilt dist verbatim; the `test:e2e:dist`
    // script builds wasm + dist first, so the preview never falls back to a
    // stale bundle silently — the vite.config.ts guard fails the build.
    command: "npm run preview -- --host 127.0.0.1 --port 5175 --strictPort",
    reuseExistingServer: !process.env.CI,
    timeout: 30_000,
    url: "http://127.0.0.1:5175"
  },
  projects: [
    {
      name: "chromium-desktop-dist",
      use: {
        ...devices["Desktop Chrome"],
        viewport: { width: 1280, height: 900 }
      }
    }
  ]
});
