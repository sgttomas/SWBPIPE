// The after-worker reply check's own vitest config (afterWorkerReplies.test.ts): this one file,
// in node, without the suite's setup (its wasm engine pre-warm; the decoders judged here use none).
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

export default defineConfig({
  root: fileURLToPath(new URL("../../..", import.meta.url)),
  test: {
    environment: "node",
    include: ["src/test-support/after-worker/afterWorkerReplies.test.ts"],
    testTimeout: 600000
  }
});
