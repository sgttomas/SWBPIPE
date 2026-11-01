// RT-U4: evaluating uiDiagnostics.ts installs both read-only globals together, and
// evaluating the timeline module alone installs neither.
import { describe, expect, it } from "vitest";

describe("the solve timeline global's installation", () => {
  it("comes with the diagnostics global, not with the timeline module", async () => {
    await import("./solveTimeline");
    expect(Object.prototype.hasOwnProperty.call(globalThis, "__openPipeStressSolveTimelineV1")).toBe(false);
    expect(Object.prototype.hasOwnProperty.call(globalThis, "__openPipeStressUiDiagnosticsV1")).toBe(false);
    await import("../features/workspace/uiDiagnostics");
    expect(Object.prototype.hasOwnProperty.call(globalThis, "__openPipeStressUiDiagnosticsV1")).toBe(true);
    expect(Object.prototype.hasOwnProperty.call(globalThis, "__openPipeStressSolveTimelineV1")).toBe(true);
  });
});
