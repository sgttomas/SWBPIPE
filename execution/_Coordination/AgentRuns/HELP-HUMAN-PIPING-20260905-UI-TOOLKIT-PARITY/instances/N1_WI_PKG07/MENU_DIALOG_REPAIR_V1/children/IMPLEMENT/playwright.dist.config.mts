import { defineConfig } from "@playwright/test";
import base from "../../../../../../../../../apps/desktop/playwright.config.ts";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
const desktop = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../../../apps/desktop");
export default defineConfig({
  ...base,
  testDir: resolve(desktop, "e2e"),
  outputDir: process.env.MENU_REPAIR_OUTPUT || "/tmp/menu-repair-dist",
  workers: 1,
  retries: 0,
  use: { ...base.use, baseURL: "http://127.0.0.1:4186", trace: "off" },
  webServer: { command: "npm run preview -- --host 127.0.0.1 --port 4186", cwd: desktop, url: "http://127.0.0.1:4186", reuseExistingServer: false, timeout: 30000 }
});
