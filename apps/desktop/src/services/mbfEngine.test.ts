import { expect, it, vi } from "vitest";
import { loadMbfEngine } from "./mbfEngine";

// An absent browser artifact is a named failure with the build command, and a later call tries again.
it("missing browser artifact gives an actionable error and retries", async () => {
  const runtime = globalThis.process;
  const errors: unknown[] = [];
  try {
    vi.stubGlobal("process", { ...runtime, versions: {} });
    for (let attempt = 0; attempt < 2; attempt++) {
      try {
        await loadMbfEngine();
      } catch (error) {
        errors.push(error);
      }
    }
  } finally {
    vi.stubGlobal("process", runtime);
  }
  expect(errors).toHaveLength(2);
  for (const error of errors) expect(String(error)).toMatch(/MBF-ENGINE-ASSET-ABSENT.*npm run build:wasm/);
});
