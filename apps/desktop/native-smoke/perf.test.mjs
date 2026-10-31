// The desktop perf runner's plan, sampling summary and classification, without the app.
//
//   node --test apps/desktop/native-smoke/perf.test.mjs
import { test } from "node:test";
import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import { classify, formatPlan, loadPlan, selectRuns, summarizeSamples } from "./perf.mjs";

const plan = loadPlan(fileURLToPath(new URL("./perf-plan.json", import.meta.url)));

test("the window subset is the brief's runs and fits its budget by estimate", () => {
  const runs = selectRuns(plan, { subset: "window" });
  assert.deepEqual(runs.map((run) => `${run.mode}:${run.model}`), [
    "injected:linear-ex-n0250.json", "injected:linear-ex-n0500.json", "injected:linear-ex-n1000.json", "injected:linear-ex-n2000.json",
    "native:linear-ex-n0250.json", "native:linear-ex-n0500.json"
  ]);
  assert.ok(runs.reduce((sum, run) => sum + run.estimate_s, 0) <= plan.budget_s);
  assert.ok(plan.budget_s <= 900, "one exclusive window is at most 15 minutes");
  for (const run of plan.runs) assert.equal(Boolean(run.result), run.mode === "injected", run.id);
  assert.throws(() => selectRuns(plan, { runs: "nope" }), /unknown run ids/);
  assert.match(formatPlan(runs, plan.budget_s), /total estimate: 840 s \(14.0 min\); budget 880 s/);
});

test("samples sum per kind, keep per-process peaks and a peak since a stage", () => {
  const samples = [
    { t: 1000, p: [[1, "app", 10, 100, 100], [2, "webcontent", 20, 200, 200]] },
    { t: 1200, p: [[1, "app", 30, 300, 300], [2, "webcontent", 25, 150, 200], [3, "webcontent", 5, 60, 60]] },
    { t: 1400, p: [[1, "app", 15, 120, 300], [2, "webcontent", null, null, null]] }
  ];
  const summary = summarizeSamples(samples, { since: 1200 });
  assert.deepEqual(summary.kinds.app, { peak_rss: 30, peak_footprint: 300, peak_footprint_since: 300 });
  assert.deepEqual(summary.kinds.webcontent, { peak_rss: 30, peak_footprint: 210, peak_footprint_since: 210 });
  assert.equal(summary.pids[2].last_ms, 1400);
  assert.equal(summary.pids[3].last_ms, 1200);
  assert.deepEqual(summary.timeline[1], [1200, 300, 30, 210, 30, null]);
});

const base = (over = {}) => ({
  exit: { status: 0, signal: null, ended_ms: 10_000 },
  timedOut: false,
  progress: [],
  memory: { pids: { 1: { kind: "app", last_ms: 10_000 }, 2: { kind: "webcontent", last_ms: 9_900 } } },
  store: { quick_check: ["ok"], files: [{ name: "openpipestress-projects.sqlite3", bytes: 4096 }], projects: [{ project_id: "p", result_type: "text", result_chars: 1000 }] },
  run: { id: "r", project_id: "p", timeout_s: 60, paint_ms: 30_000 },
  resultBytes: 1000,
  report: {
    solve: { ended: "first_paint", timeline: { marks: [] }, shown: {} },
    record_run: { attempted: true, message: "Recorded run perf r." },
    save_project: { message: "Saved current project model snapshot." }
  },
  ...over
});

test("a painted, recorded and stored run is ok", () => {
  assert.equal(classify(base()).class, "ok");
});

test("a failed solve is a refusal quoting the window's message, with the timeline's code and stage", () => {
  const result = classify(base({ report: { solve: { ended: "failed", timeline: { failure: { stage: "reader_verdict_end", code: "SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED", error_name: "Error" } }, shown: { error: "SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED: the received result is not bound." } } } }));
  assert.equal(result.class, "refused_with_message");
  assert.deepEqual(result.findings[0], { class: "refused_with_message", step: "solve", message: "SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED: the received result is not bound.", code: "SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED", stage: "reader_verdict_end", needs_review: true });
});

test("no report: a timeout is a hang, an early exit an app crash, a vanished WebContent a WebContent crash", () => {
  assert.equal(classify(base({ report: null, timedOut: true })).class, "hang");
  assert.equal(classify(base({ report: null, exit: { status: null, signal: "SIGSEGV", ended_ms: 10_000 } })).findings[0].process, "app");
  const vanished = classify(base({ report: null, timedOut: true, memory: { pids: { 1: { kind: "app", last_ms: 10_000 }, 2: { kind: "webcontent", last_ms: 4_000 } } } }));
  assert.equal(vanished.class, "crash");
  assert.equal(vanished.findings[0].process, "webcontent");
});

test("a reported save the store does not hold is misleading; a short or unchecked store is half-written", () => {
  assert.equal(classify(base({ store: { quick_check: ["ok"], files: [], projects: [] } })).class, "misleading_message");
  assert.equal(classify(base({ store: { quick_check: ["ok"], files: [], projects: [{ project_id: "p", result_type: "text", result_chars: 10 }] } })).class, "half_written_save");
  // A failed solve holds no result: the stored `null` is not a half-written save.
  const unsolved = base({ store: { quick_check: ["ok"], files: [], projects: [{ project_id: "p", result_type: "text", result_chars: 4 }] } });
  unsolved.report = { ...unsolved.report, solve: { ended: "failed", timeline: { failure: { stage: "reader_verdict_end", code: "SOLVE_NATIVE_INVOCATION_BINDING_REQUIRED", error_name: null } }, shown: {} }, record_run: { attempted: false, message: "Record Run: RUN-RECORD-NO-RUN: solve the model to record a run." } };
  assert.deepEqual(classify(unsolved).findings.map((finding) => finding.class), ["refused_with_message", "refused_with_message"]);
  assert.equal(classify(base({ store: { quick_check: ["*** in database main ***"], files: [], projects: [{ project_id: "p", result_type: "text", result_chars: 1000 }] } })).class, "half_written_save");
  assert.equal(classify(base({ store: { quick_check: ["ok"], files: [{ name: "x.sqlite3-journal", bytes: 512 }], projects: [{ project_id: "p", result_type: "text", result_chars: 1000 }] } })).class, "half_written_save");
});

test("a refused Record Run or Save Project is quoted", () => {
  const result = classify(base({ report: { solve: { ended: "first_paint", timeline: {}, shown: {} }, record_run: { attempted: true, message: "Not recorded: RESOURCE_UNAVAILABLE" }, save_project: { message: "Save failed: string or blob too big" } } }));
  assert.equal(result.class, "refused_with_message");
  assert.deepEqual(result.findings.map((finding) => finding.message), ["Not recorded: RESOURCE_UNAVAILABLE", "Save failed: string or blob too big"]);
});

test("the driver loads in a webview-like context and hands its steps to a test", async () => {
  const { readFileSync } = await import("node:fs");
  const vm = await import("node:vm");
  const hooks = {};
  const context = { window: { __TAURI_INTERNALS__: { invoke: async () => null } }, document: { querySelector: () => null }, CSS: { escape: (value) => value }, performance, setTimeout, navigator: { userAgent: "test" }, __SWBPIPE_PERF_DRIVER_TEST__: hooks };
  context.globalThis = context;
  vm.runInNewContext(readFileSync(new URL("./perf-driver.js", import.meta.url), "utf8"), context);
  assert.deepEqual(Object.keys(hooks).sort(), ["recordRun", "saveProject", "solve", "waitFor"]);
  await assert.rejects(hooks.waitFor("nothing", () => null, 30, 5), /timed out after 30 ms waiting for nothing/);
});
