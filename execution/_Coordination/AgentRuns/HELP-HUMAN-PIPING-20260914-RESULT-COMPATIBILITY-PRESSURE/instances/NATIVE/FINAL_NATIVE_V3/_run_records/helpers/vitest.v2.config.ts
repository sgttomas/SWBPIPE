import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const workingRoot = process.env.FINAL_WORKING_ROOT;
const preparationRoot = path.dirname(fileURLToPath(import.meta.url));
if (!workingRoot || !path.isAbsolute(workingRoot)) {
  throw new Error("FINAL_WORKING_ROOT must be the absolute final-candidate projects/chirality-piping root");
}

export default defineConfig({
  resolve: {
    alias: {
      "@final-product": path.join(workingRoot, "apps/desktop/src"),
    },
  },
  test: {
    environment: "node",
    include: [path.join(preparationRoot, "compose_captured_outputs_v2.test.ts")],
    testTimeout: 120_000,
    hookTimeout: 120_000,
    pool: "forks",
    maxWorkers: 1,
  },
});
