import { defineConfig } from "@playwright/test";
import path from "node:path";
import {
  bindRequiredChromiumExecutable,
  REQUIRED_CHROMIUM_RUNTIME_BINDING
} from "./candidate-server-response";
import candidateBase from "./playwright.candidate-performance.config";

const evidenceDir = process.env.UI_FOUNDATION_EVIDENCE_DIR;
if (!evidenceDir || !path.isAbsolute(evidenceDir)) {
  throw new Error("UI_FOUNDATION_EVIDENCE_DIR must be an absolute run-owned candidate resource evidence path");
}
const chromiumExecutable = bindRequiredChromiumExecutable();

export default defineConfig({
  ...candidateBase,
  testMatch: "candidate-resource-qualification.benchmark.ts",
  timeout: 20 * 60_000,
  workers: 1,
  fullyParallel: false,
  reporter: [["list"], ["json", { outputFile: path.join(evidenceDir, "playwright-results.json") }]],
  metadata: {
    ...(candidateBase.metadata ?? {}),
    chromiumExecutable,
    chromiumSourceBinding: REQUIRED_CHROMIUM_RUNTIME_BINDING
  },
  use: {
    ...candidateBase.use,
    trace: "off",
    video: "off",
    screenshot: "off",
    launchOptions: { executablePath: chromiumExecutable.executablePath, args: ["--enable-automation"] }
  },
  projects: [{ name: "chromium-ui-foundation-candidate-resource" }]
});
