import { defineConfig } from "vitest/config";
export default defineConfig({
  root: "/Users/ryan/.codex/worktrees/8728/chirality-compatibility-20260914/projects/chirality-piping",
  test: {
    environment: "jsdom",
    globals: true,
    include: ["**/reuse_v15_current_constructor.test.ts"],
    setupFiles: "/Users/ryan/.codex/worktrees/8728/chirality-compatibility-20260914/projects/chirality-piping/apps/desktop/src/test/setup.ts",
    testTimeout: 30000,
    hookTimeout: 30000,
  },
});
