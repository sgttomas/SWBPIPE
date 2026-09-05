import { execFileSync } from "node:child_process";
import path from "node:path";
const root = execFileSync("git", ["rev-parse", "--show-toplevel"], { encoding: "utf8" }).trim();
const desktop = path.join(root, "projects/chirality-piping/apps/desktop");
const source = require(path.join(desktop, "playwright.config.ts")).default;
const dist = require(path.join(desktop, "playwright.dist.config.ts")).default;
const base = source;
export default {
  ...base,
  testDir: path.join(desktop, "e2e"),
  testMatch: ["**/r2-smoke.spec.ts"],
  timeout: source.timeout,
  captureGitInfo: { commit: false, diff: false },
  projects: source.projects,
  reporter: [["list"], ["json", { outputFile: path.join(__dirname, "_run_records/source-results.json") }]],
  outputDir: path.join(__dirname, "_run_records/source-artifacts"),
  webServer: { ...base.webServer, cwd: desktop, reuseExistingServer: false },
};
