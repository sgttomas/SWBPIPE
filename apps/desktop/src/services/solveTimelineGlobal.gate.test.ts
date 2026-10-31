// RT-U4: the solve timeline's global is installed under the very condition that installs
// `__openPipeStressUiDiagnosticsV1` (uiDiagnostics.ts): where that one is not installed by
// the module, neither is this one. Its own file, so this realm's globals start empty.
import { describe, expect, it } from "vitest";

describe("the solve timeline global's gating", () => {
  it("is not installed when the diagnostics global's install condition is false", async () => {
    const preexisting = Object.freeze({ schema: "preexisting" });
    Object.defineProperty(globalThis, "__openPipeStressUiDiagnosticsV1", { value: preexisting, writable: false, configurable: false, enumerable: false });
    await import("../features/workspace/uiDiagnostics");
    expect(globalThis.__openPipeStressUiDiagnosticsV1).toBe(preexisting);
    expect(Object.prototype.hasOwnProperty.call(globalThis, "__openPipeStressSolveTimelineV1")).toBe(false);
  });
});
