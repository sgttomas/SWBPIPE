import path from "node:path";
import { defineConfig, devices } from "@playwright/test";

// The screenshot walkthrough (`npm run walkthrough`, README.md). Its files end
// in `.walk.ts`, so the test lanes (playwright.config.ts and the dist config,
// both of which match `*.spec.ts` under e2e/) never pick them up, and this
// config never picks up a test.
//
// One window size: the product's own default window (src-tauri/tauri.conf.json),
// so a picture shows what a person has in front of them at first launch.
// SWBPIPE_WALKTHROUGH_SIZE=WIDTHxHEIGHT takes pictures at another size.
const size = /^(\d{3,4})x(\d{3,4})$/.exec(process.env.SWBPIPE_WALKTHROUGH_SIZE ?? "");
const viewport = size ? { width: Number(size[1]), height: Number(size[2]) } : { width: 1440, height: 920 };
const executablePath = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH || undefined;
const desktopRoot = path.resolve(import.meta.dirname, "..");

export default defineConfig({
  testDir: ".",
  testMatch: ["**/*.walk.ts"],
  // One browser at a time: the journeys share the output folder's numbering.
  workers: 1,
  fullyParallel: false,
  captureGitInfo: { commit: false, diff: false },
  timeout: 300_000,
  expect: { timeout: 8_000 },
  reporter: [["list"]],
  outputDir: path.join(desktopRoot, "test-results", "walkthrough"),
  use: {
    baseURL: "http://127.0.0.1:5176",
    actionTimeout: 8_000,
    colorScheme: "light",
    launchOptions: executablePath ? { executablePath } : undefined
  },
  webServer: {
    command: "npm run dev -- --port 5176 --strictPort",
    cwd: desktopRoot,
    reuseExistingServer: false,
    timeout: 60_000,
    url: "http://127.0.0.1:5176"
  },
  projects: [{ name: "walkthrough", use: { ...devices["Desktop Chrome"], viewport } }]
});
