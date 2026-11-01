import { afterEach, describe, expect, it, vi } from "vitest";
import {
  SOLVE_TIMELINE_GLOBAL,
  SOLVE_TIMELINE_SCHEMA,
  beginSolveTimeline,
  codeOrNull,
  errorCode,
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
  it("is absent before any solve", () => {
    expect(readSolveTimeline()).toBeNull();
    expect(globalThis.__openPipeStressSolveTimelineV1?.readCurrent()).toBeNull();
    markSolveStage("job_started");
    expect(timedSolvePass("checked_json_text", () => "x", (text) => text.length)).toBe("x");
    expect(readSolveTimeline()).toBeNull();
  });

  it("records each stage once in order, with monotonic times since the request", () => {
    let clock = 100;
    vi.spyOn(performance, "now").mockImplementation(() => clock);
    const run = beginSolveTimeline({ solver_mode: "sparse_interactive" });
    clock = 110; run.jobStarted("backend-solve-job-1");
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
    expect(timeline).toMatchObject({ schema: SOLVE_TIMELINE_SCHEMA, sequence: expect.any(Number), outcome: "completed", failure: null, dropped_marks: 0 });
    expect(timeline.marks[1].detail).toBeNull();
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

  it("records a failure with the last stage reached, once, as its code and error class only", () => {
    const run = beginSolveTimeline();
    run.jobStarted("j");
    markSolveStage("take_end", { bytes: 7 });
    run.end("failed", new RangeError("Out of memory at /Users/someone/model.json"));
    run.end("failed", new Error("SOLVE-JOB-IDENTITY-MISMATCH: later"));
    const timeline = readSolveTimeline()!;
    expect(timeline.outcome).toBe("failed");
    expect(timeline.failure).toEqual({ stage: "take_end", code: null, error_name: "RangeError", at_ms: expect.any(Number) });
    const second = beginSolveTimeline();
    second.end("failed", new Error("SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED: the received result is not bound to project:RK-1."));
    expect(readSolveTimeline()!.failure).toMatchObject({ code: "SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED", error_name: "Error" });
  });

  it("keeps a detail string only when it is a code, under a measurement field only: no path, identifier or message text", () => {
    beginSolveTimeline({ solver_mode: "sparse_interactive" });
    markSolveStage("reader_verdict_end", { registered: false, reason_code: "SOURCE_PHYSICS_RHS_DIRECTION" });
    markSolveStage("job_done", { state: "completed", result_bytes: 5, path: "/Users/someone/rack.json" });
    markSolveStage("take_end", { job: "backend-solve-job-1", project: "project:RK-linear-ex-82", note: "Opened model document a.json" });
    const details = readSolveTimeline()!.marks.map((mark) => mark.detail);
    expect(details).toEqual([
      { solver_mode: "sparse_interactive" },
      { registered: false, reason_code: "SOURCE_PHYSICS_RHS_DIRECTION" },
      { state: "completed", result_bytes: 5 },
      {}
    ]);
    expect(codeOrNull("PREVIEW-SOLVE-RESULT-UTF8-INVALID")).toBe("PREVIEW-SOLVE-RESULT-UTF8-INVALID");
    expect(errorCode(new Error("PREVIEW-SOLVE-RESULT-UTF8-INVALID: the bytes"))).toBe("PREVIEW-SOLVE-RESULT-UTF8-INVALID");
    expect(errorCode(new Error("unknown solve job: backend-solve-job-1"))).toBeNull();
  });

  it("drops a detail key that is not a measurement field and an error class it does not know", () => {
    const run = beginSolveTimeline({ EXAMPLE_CUSTOMER: 7, solver_mode: "dense_scrutiny" });
    markSolveStage("take_end", { bytes: 3, SOURCE_CUSTOMER_ACME: true, "/Users/someone/rack.json": 1 });
    class ExampleCustomerError extends Error { override name = "ExampleCustomerError"; }
    run.end("failed", new ExampleCustomerError("SOLVE-JOB-IDENTITY-MISMATCH: later"));
    const timeline = readSolveTimeline()!;
    expect(timeline.marks.map((mark) => mark.detail)).toEqual([{ solver_mode: "dense_scrutiny" }, { bytes: 3 }]);
    expect(timeline.failure).toMatchObject({ code: "SOLVE-JOB-IDENTITY-MISMATCH", error_name: null });
    const known = beginSolveTimeline();
    const refused = new Error("RESOURCE_UNAVAILABLE: the pool is full");
    refused.name = "ResourceUnavailableError";
    known.end("failed", refused);
    expect(readSolveTimeline()!.failure).toMatchObject({ error_name: "ResourceUnavailableError" });
  });

  it("ignores a superseded run's marks and end", () => {
    const first = beginSolveTimeline();
    const second = beginSolveTimeline();
    first.jobStarted("old");
    first.end("failed", new Error("old"));
    second.jobStarted("new");
    const timeline = readSolveTimeline()!;
    expect(isSolveJob("new")).toBe(true);
    expect(isSolveJob("old")).toBe(false);
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

  it("is bounded, and its global reads a recursively frozen copy", () => {
    const run = beginSolveTimeline();
    for (let index = 0; index < 100; index += 1) run.mark("job_done");
    const read = globalThis.__openPipeStressSolveTimelineV1!.readCurrent()!;
    expect(read.marks).toHaveLength(64);
    expect(read.dropped_marks).toBe(37);
    expect(Object.isFrozen(read) && Object.isFrozen(read.marks) && Object.isFrozen(read.marks[0])).toBe(true);
    expect(Object.isFrozen(readSolveTimeline())).toBe(false);
  });
});

describe("the codes the timeline keeps", () => {
  it("keeps its own codes and drops identifiers, file names and message text", () => {
    for (const kept of ["sparse_interactive", "dense_scrutiny", "completed", "refused", "SOURCE_PHYSICS_RHS_DIRECTION",
      "SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED", "PREVIEW-SOLVE-RESULT-UTF8-INVALID", "CAPTURE_MISMATCH", "RESOURCE_UNAVAILABLE"]) {
      expect(codeOrNull(kept)).toBe(kept);
    }
    for (const dropped of ["SOURCE_CUSTOMER_ACME", "ANALYSIS_PROJECT_SECRET", "SOURCE_PHYSICS_ACME_CORP", "PREVIEW-SOLVE-ACME",
      "SOLVE_JOB_FOR_CUSTOMER_X", "secret_project", "ALICE", "SECRET_PROJECT", "RK-H0012", "RK-linear-ex-82", "linear-ex-n0250.json",
      "rack.json", "model", "project", "sparse", "Completed", "SOURCE_", "SOURCE", "source_physics_rhs_direction",
      "backend-solve-job-1", "project:RK-linear-ex-82", "/Users/someone/rack.json", "SOURCE_PHYSICS RHS", `SOURCE_${"A".repeat(100)}`]) {
      expect(codeOrNull(dropped), dropped).toBeNull();
    }
    expect(errorCode(new Error("ALICE: something"))).toBeNull();
    expect(errorCode(new Error("RK-H0012 refused"))).toBeNull();
  });
});

describe("the solve timeline's own global, beside the unchanged diagnostics global", () => {
  it("is frozen through and through: its functions and every object it returns", () => {
    "use strict";
    const api = globalThis.__openPipeStressSolveTimelineV1!;
    for (const key of ["schema", "readCurrent"] as const) {
      expect(Object.getOwnPropertyDescriptor(api, key)).toMatchObject({ writable: false, configurable: false });
    }
    expect(Object.isFrozen(api.readCurrent)).toBe(true);
    expect(() => { (api.readCurrent as unknown as Record<string, unknown>).extra = 1; }).toThrow(TypeError);
    expect((api.readCurrent as unknown as Record<string, unknown>).extra).toBeUndefined();
    for (const key of Reflect.ownKeys(api.readCurrent)) {
      expect(Object.getOwnPropertyDescriptor(api.readCurrent, key)).toMatchObject({ configurable: false });
    }
    expect(() => { (api as unknown as Record<string, unknown>).readCurrent = () => null; }).toThrow(TypeError);
    const run = beginSolveTimeline({ solver_mode: "sparse_interactive" });
    run.mark("job_done", { state: "completed", result_bytes: 1 });
    recordSolvePass("checked_json_text", performance.now(), 1);
    const read = api.readCurrent()!;
    const objects: unknown[] = [read, read.marks, read.passes, ...read.marks, ...read.marks.map((mark) => mark.detail), ...read.passes];
    for (const object of objects.filter(Boolean)) {
      expect(Object.isFrozen(object)).toBe(true);
      expect(() => { (object as Record<string, unknown>).extra = 1; }).toThrow(TypeError);
    }
    expect(() => { (read.marks[1].detail as Record<string, unknown>).state = "x"; }).toThrow(TypeError);
  });

  it("is a frozen, non-writable, non-configurable data property with schema and readCurrent only", () => {
    const descriptor = Object.getOwnPropertyDescriptor(globalThis, SOLVE_TIMELINE_GLOBAL)!;
    expect(descriptor).toMatchObject({ writable: false, configurable: false, enumerable: false });
    expect("get" in descriptor || "set" in descriptor).toBe(false);
    expect(Object.isFrozen(descriptor.value)).toBe(true);
    expect(Object.keys(descriptor.value).sort()).toEqual(["readCurrent", "schema"]);
    expect(descriptor.value.schema).toBe(SOLVE_TIMELINE_SCHEMA);
    expect(() => { (globalThis as Record<string, unknown>)[SOLVE_TIMELINE_GLOBAL] = {}; }).toThrow();
  });

  it("leaves the diagnostics global's members and every snapshot key as they were, with no solveTimeline", () => {
    const api = globalThis.__openPipeStressUiDiagnosticsV1;
    expect(Object.keys(api).sort()).toEqual(["projectAuthoredPoint", "readCurrent", "schema"]);
    const idle = api.readCurrent();
    expect(Object.keys(idle).sort()).toEqual(["capturedAt", "model", "schema", "snapshotSequence", "tree", "viewport"]);
    expect(Object.keys(idle.model).sort()).toEqual(["assignment", "generation", "identityHash", "indexGeneration", "projectId", "projectSessionGeneration"]);
    const run = beginSolveTimeline();
    run.jobStarted("j");
    noteSolveResult({}, 1);
    const during = api.readCurrent();
    expect(Object.keys(during).sort()).toEqual(["capturedAt", "model", "schema", "snapshotSequence", "tree", "viewport"]);
    expect("solveTimeline" in during).toBe(false);
  });
});
