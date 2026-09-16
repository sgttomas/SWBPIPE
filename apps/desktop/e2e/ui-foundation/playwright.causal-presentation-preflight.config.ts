import { defineConfig } from "@playwright/test";
import base from "./playwright.performance.config";
import { bindRequiredChromiumExecutable, REQUIRED_CHROMIUM_RUNTIME_BINDING } from "./candidate-server-response";

const chromiumExecutable = bindRequiredChromiumExecutable();

// This config runs only the bounded source-method validation. Playwright trace,
// video, and automatic screenshots remain off; the spec owns one explicit,
// lossless ReturnAsStream CDP capture and its declared before/after images.
export default defineConfig({
  ...base,
  metadata: { ...(base.metadata ?? {}), chromiumExecutable,
    chromiumSourceBinding: REQUIRED_CHROMIUM_RUNTIME_BINDING },
  testMatch: "causal-presentation-preflight.benchmark.ts",
  workers: 1,
  fullyParallel: false,
  timeout: 5 * 60_000,
  reporter: [["list"]],
  use: {
    ...base.use,
    launchOptions: { executablePath: chromiumExecutable.executablePath, args: ["--enable-automation"] },
    trace: "off",
    video: "off",
    screenshot: "off"
  },
  projects: [{ name: "chromium-ui-foundation-causal-presentation-preflight" }]
});
