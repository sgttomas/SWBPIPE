import { afterEach, describe, expect, it, vi } from "vitest";
import {
  SOLVE_TIMELINE_SCHEMA,
  beginSolveTimeline,
  isSolveJob,
  isSolveResult,
  markSolveCommittedAndPaint,
  markSolveStage,
  noteSolveResult,
  readSolveTimeline,
  recordSolvePass,
  resetSolveTimelineForTests,
  timedSolvePass
} from "./solveTimeline";
import "../features/workspace/uiDiagnostics";

afterEach(() => {
  resetSolveTimelineForTests();
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});

describe("the latest solve's timeline (RT-U4)", () => {
  it("is absent before any solve, and readCurrent() then carries no solveTimeline", () => {
    expect(readSolveTimeline()).toBeNull();
    markSolveStage("job_started", { job_id: "j" });
    expect(timedSolvePass("checked_json_text", () => "x", (text) => text.length)).toBe("x");
    expect(readSolveTimeline()).toBeNull();
    expect("solveTimeline" in globalThis.__openPipeStressUiDiagnosticsV1.readCurrent()).toBe(false);
  });

  it("records each stage once in order, with monotonic times since the request", () => {
    let clock = 100;
    vi.spyOn(performance, "now").mockImplementation(() => clock);
    const run = beginSolveTimeline({ solver_mode: "sparse_interactive" });
    clock = 110; run.mark("job_started", { job_id: "backend-solve-job-1" });
    expect(isSolveJob("backend-solve-job-1")).toBe(true);
    expect(isSolveJob("backend-solve-job-2")).toBe(false);
    clock = 150; run.mark("job_done", { state: "completed", result_bytes: 42, worker_peak_footprint_bytes: null });
    clock = 151; markSolveStage("take_start");
    clock = 160; markSolveStage("take_end", { bytes: 42 });
    const result = { results: [] };
    clock = 170; markSolveStage("parse_end"); noteSolveResult(result, 42);
    expect(isSolveResult(result)).toBe(true);
    expect(isSolveResult({ results: [] })).toBe(false);
    run.end("completed");
    expect(isSolveJob("backend-solve-job-1")).toBe(false);
    const timeline = readSolveTimeline()!;
    expect(timeline).toMatchObject({ schema: SOLVE_TIMELINE_SCHEMA, sequence: expect.any(Number), outcome: "completed", job_id: "backend-solve-job-1", failure: null, dropped_marks: 0 });
    expect(timeline.marks.map((mark) => [mark.stage, mark.since_start_ms])).toEqual([
      ["solve_requested", 0], ["job_started", 10], ["job_done", 50], ["take_start", 51], ["take_end", 60], ["parse_end", 70]
    ]);
    expect(timeline.marks[2].detail).toEqual({ state: "completed", result_bytes: 42, worker_peak_footprint_bytes: null });
    // The copy is the reader's own: the timeline is unchanged by edits to it.
    (timeline.marks as unknown as unknown[]).length = 0;
    expect(readSolveTimeline()!.marks).toHaveLength(6);
  });

  it("aggregates passes by kind and stage, counting full-envelope passes against the received result", () => {
    let clock = 0;
    vi.spyOn(performance, "now").mockImplementation(() => clock);
    beginSolveTimeline();
    recordSolvePass("checked_json_text", 0, 10); // before the result is known: under 1 MiB is not full
    markSolveStage("parse_end");
    noteSolveResult({}, 4 << 20);
    clock = 5; recordSolvePass("native_fingerprint", 0, 4 << 20);
    clock = 7; recordSolvePass("native_fingerprint", 5, 1 << 20);
    clock = 20; recordSolvePass("native_fingerprint", 7, 3 << 20);
    const passes = readSolveTimeline()!.passes;
    expect(passes).toEqual([
      { kind: "checked_json_text", after_stage: "solve_requested", count: 1, total_ms: 0, max_ms: 0, full_count: 0, full_ms: 0, max_chars: 10 },
      { kind: "native_fingerprint", after_stage: "parse_end", count: 3, total_ms: 20, max_ms: 13, full_count: 2, full_ms: 18, max_chars: 4 << 20 }
    ]);
  });

  it("records a failure with the last stage reached, once", () => {
    const run = beginSolveTimeline();
    run.mark("job_started", { job_id: "j" });
    markSolveStage("take_end", { bytes: 7 });
    run.end("failed", new RangeError("Out of memory"));
    run.end("failed", new Error("later"));
    const timeline = readSolveTimeline()!;
    expect(timeline.outcome).toBe("failed");
    expect(timeline.failure).toMatchObject({ stage: "take_end", message: "Out of memory" });
  });

  it("ignores a superseded run's marks and end", () => {
    const first = beginSolveTimeline();
    const second = beginSolveTimeline();
    first.mark("job_started", { job_id: "old" });
    first.end("failed", new Error("old"));
    second.mark("job_started", { job_id: "new" });
    const timeline = readSolveTimeline()!;
    expect(timeline.job_id).toBe("new");
    expect(timeline.outcome).toBe("running");
    expect(timeline.marks.map((mark) => mark.stage)).toEqual(["solve_requested", "job_started"]);
  });

  it("marks the commit of the solve's result, then its first paint after two frames", () => {
    const frames: FrameRequestCallback[] = [];
    vi.stubGlobal("requestAnimationFrame", (callback: FrameRequestCallback) => { frames.push(callback); return frames.length; });
    beginSolveTimeline();
    const result = {};
    noteSolveResult(result, 2);
    markSolveCommittedAndPaint({});
    markSolveCommittedAndPaint(null);
    expect(readSolveTimeline()!.marks.map((mark) => mark.stage)).toEqual(["solve_requested"]);
    markSolveCommittedAndPaint(result);
    markSolveCommittedAndPaint(result);
    frames.shift()!(0);
    expect(readSolveTimeline()!.marks.map((mark) => mark.stage)).toEqual(["solve_requested", "result_committed"]);
    frames.shift()!(0);
    expect(readSolveTimeline()!.marks.map((mark) => mark.stage)).toEqual(["solve_requested", "result_committed", "first_paint"]);
    expect(frames).toHaveLength(0);
  });

  it("is bounded, and readCurrent() exposes a frozen copy", () => {
    const run = beginSolveTimeline();
    for (let index = 0; index < 100; index += 1) run.mark("job_done");
    const snapshot = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
    expect(snapshot.solveTimeline?.marks).toHaveLength(64);
    expect(snapshot.solveTimeline?.dropped_marks).toBe(37);
    expect(Object.isFrozen(snapshot.solveTimeline)).toBe(true);
    expect(Object.isFrozen(readSolveTimeline())).toBe(false);
  });
});
