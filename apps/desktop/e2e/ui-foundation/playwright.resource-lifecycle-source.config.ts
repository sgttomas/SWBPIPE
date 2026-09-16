import { defineConfig, devices } from "@playwright/test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import path from "node:path";
import {
  bindCandidateDriverEntry,
  bindRequiredChromiumExecutable,
  REQUIRED_CHROMIUM_RUNTIME_BINDING
} from "./candidate-server-response";

const evidenceDir = process.env.UI_FOUNDATION_EVIDENCE_DIR;
if (!evidenceDir || !path.isAbsolute(evidenceDir)) {
  throw new Error("UI_FOUNDATION_EVIDENCE_DIR must be an absolute run-owned source lifecycle evidence path");
}
const candidateDriverBinding = bindCandidateDriverEntry("candidate-resource-lifecycle-source");
const chromiumExecutable = bindRequiredChromiumExecutable();
const verifierDesktopRoot = path.resolve(import.meta.dirname, "../..");
const candidateDesktopRoot = path.join(candidateDriverBinding.candidateSourceRoot, "apps/desktop");
const transferredPageFiles = ["resource-lifecycle-page.html", "resource-lifecycle-page.tsx"].map((name) => {
  const relative = path.join("e2e/ui-foundation", name);
  const maintained = readFileSync(path.join(verifierDesktopRoot, relative));
  const transferred = readFileSync(path.join(candidateDesktopRoot, relative));
  const digest = (bytes: Buffer): string => createHash("sha256").update(bytes).digest("hex");
  const maintainedSha256 = digest(maintained);
  const transferredSha256 = digest(transferred);
  if (maintained.length !== transferred.length || maintainedSha256 !== transferredSha256) {
    throw new Error(`candidate source lifecycle maintained-file transfer mismatch: ${relative}`);
  }
  return Object.freeze({ relative, bytes: maintained.length, sha256: maintainedSha256 });
});

export default defineConfig({
  testDir: ".",
  testMatch: "resource-lifecycle-source.benchmark.ts",
  workers: 1,
  fullyParallel: false,
  captureGitInfo: { commit: false, diff: false },
  timeout: 20 * 60_000,
  outputDir: path.join(evidenceDir, "playwright-source-lifecycle-artifacts"),
  reporter: [["list"], ["json", { outputFile: path.join(evidenceDir, "playwright-results.json") }]],
  metadata: {
    candidateDriverBinding,
    chromiumExecutable,
    chromiumSourceBinding: REQUIRED_CHROMIUM_RUNTIME_BINDING,
    transferredPageFiles
  },
  use: {
    baseURL: "http://127.0.0.1:5177",
    ...devices["Desktop Chrome"],
    trace: "retain-on-failure",
    video: "off",
    screenshot: "off",
    launchOptions: { executablePath: chromiumExecutable.executablePath, args: ["--enable-automation"] },
    viewport: { width: 1440, height: 920 },
    deviceScaleFactor: 2
  },
  webServer: {
    command: "npm run dev -- --host 127.0.0.1 --port 5177 --strictPort",
    cwd: candidateDesktopRoot,
    reuseExistingServer: false,
    timeout: 30_000,
    url: "http://127.0.0.1:5177/e2e/ui-foundation/resource-lifecycle-page.html"
  },
  projects: [{ name: "chromium-ui-foundation-source-actual-app-lifecycle" }]
});
