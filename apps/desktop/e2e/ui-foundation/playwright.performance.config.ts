import { defineConfig, devices } from "@playwright/test";
import { existsSync } from "node:fs";
import path from "node:path";

const configuredChrome = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH;
const macChrome = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome";
const executablePath = configuredChrome && existsSync(configuredChrome)
  ? configuredChrome
  : existsSync(macChrome) ? macChrome : undefined;
const evidenceDir = process.env.UI_FOUNDATION_EVIDENCE_DIR ?? "/private/tmp/ui-foundation-missing-evidence-dir";

export default defineConfig({
  testDir: ".",
  testMatch: "ui-foundation-performance.benchmark.ts",
  workers: 1,
  fullyParallel: false,
  captureGitInfo: { commit: false, diff: false },
  timeout: 20 * 60_000,
  expect: { timeout: 30_000 },
  outputDir: path.join(evidenceDir, "playwright-artifacts"),
  reporter: [["list"]],
  use: {
    baseURL: "http://127.0.0.1:5176",
    trace: "off",
    video: "off",
    screenshot: "off",
    launchOptions: executablePath ? { executablePath } : undefined,
    ...devices["Desktop Chrome"],
    viewport: { width: 1440, height: 920 },
    deviceScaleFactor: 2
  },
  webServer: {
    command: "npm run preview -- --host 127.0.0.1 --port 5176 --strictPort",
    reuseExistingServer: false,
    timeout: 30_000,
    url: "http://127.0.0.1:5176"
  },
  projects: [{ name: "chromium-ui-foundation-production" }]
});
