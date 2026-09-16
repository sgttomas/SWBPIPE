import { defineConfig } from "@playwright/test";
import path from "node:path";
import { bindRequiredChromiumExecutable, REQUIRED_CHROMIUM_RUNTIME_BINDING } from "./candidate-server-response";

const configuredOutputDir = process.env.UI_FOUNDATION_CAUSAL_METHOD_OUTPUT_DIR;
if (configuredOutputDir && !path.isAbsolute(configuredOutputDir)) {
  throw new Error("UI_FOUNDATION_CAUSAL_METHOD_OUTPUT_DIR must be an absolute run-owned evidence path");
}
const outputDir = configuredOutputDir ?? path.join(process.cwd(), "test-results", "ui-foundation-causal-method-contract");
const chromiumExecutable = bindRequiredChromiumExecutable();

export default defineConfig({
  testDir: ".",
  testMatch: "causal-method-contract.spec.ts",
  workers: 1,
  fullyParallel: false,
  timeout: 30_000,
  reporter: [["list"], ["json", { outputFile: path.join(outputDir, "results.json") }]],
  metadata: { chromiumExecutable, chromiumSourceBinding: REQUIRED_CHROMIUM_RUNTIME_BINDING },
  use: { browserName: "chromium", headless: true, trace: "off", video: "off", screenshot: "off",
    launchOptions: { executablePath: chromiumExecutable.executablePath, args: ["--enable-automation"] } },
  outputDir
});
