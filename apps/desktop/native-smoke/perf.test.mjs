// The desktop perf runner's plan, sampling summary and classification, without the app.
//
//   node --test apps/desktop/native-smoke/perf.test.mjs
import { test } from "node:test";
import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import { existsSync, mkdirSync, mkdtempSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { spawn } from "node:child_process";
import { once } from "node:events";
import { classify, containedPath, finishScratch, formatPlan, loadPlan, planName, sampleStopReason, selectRuns, summarizeSamples, validatePlan, watchWindow } from "./perf.mjs";

const plan = loadPlan(fileURLToPath(new URL("./perf-plan.json", import.meta.url)));

// Ordinary process-control regressions: only tiny Node fixtures, no App,
// model solve, sampler, host readings, RSS or timing observation.
const fixture = () => spawn(process.execPath, ["--input-type=module", "-e", `
  import {spawn} from 'node:child_process';
  const worker=spawn(process.execPath,['-e',"process.on('SIGTERM',()=>{});process.send('ready');setInterval(()=>{},1000)"],
    {stdio:['ignore','ignore','ignore','ipc'],detached:true});
  process.on('SIGTERM',()=>{});
  process.on('message', m=>{
    if(m==='block') { for(;;) {} }
    if(m==='exit') process.exit(7);
    if(m==='stop') process.send({type:'stop',reason:'fixture_resource_stop'});
  });
  worker.once('message',()=>process.send({type:'ready',worker:worker.pid}));
`], { detached: true, stdio: ["ignore", "ignore", "inherit", "ipc"] });

test("a blocked runner cannot hold its fake worker past the independent deadline", async (t) => {
  const child = fixture();
  t.after(() => { try { process.kill(-child.pid, "SIGKILL"); } catch { /* gone */ } });
  const [ready] = await once(child, "message");
  t.after(() => { try { process.kill(-ready.worker, "SIGKILL"); } catch { /* gone */ } });
  const result = watchWindow(child, { budgetMs: 100, killGraceMs: 20, finalGraceMs: 1000, listOwned: async () => [{pid:ready.worker,group:ready.worker}] });
  child.send("block");
  const stopped = await result;
  assert.equal(stopped.stop_reason, "window_deadline");
  assert.equal(stopped.exit.reaped, true);
  assert.equal(stopped.owned_group_gone, true);
  assert.equal(stopped.owned_executable_pids_gone, true);
  assert.deepEqual(stopped.group_errors, []);
});

test("an early runner exit still ends and checks its surviving fake worker", async (t) => {
  const child = fixture();
  t.after(() => { try { process.kill(-child.pid, "SIGKILL"); } catch { /* gone */ } });
  const [ready] = await once(child, "message");
  t.after(() => { try { process.kill(-ready.worker, "SIGKILL"); } catch { /* gone */ } });
  const result = watchWindow(child, { budgetMs: 5000, listOwned: async () => [{pid:ready.worker,group:ready.worker}] });
  child.send("exit");
  const stopped = await result;
  assert.equal(stopped.exit.code, 7);
  assert.equal(stopped.owned_executable_pids_gone, true);
  assert.equal(stopped.owned_group_gone, true);
});

test("a sampled resource refusal stops only the owned window group", async (t) => {
  const child = fixture();
  t.after(() => { try { process.kill(-child.pid, "SIGKILL"); } catch { /* gone */ } });
  const [ready] = await once(child, "message");
  t.after(() => { try { process.kill(-ready.worker, "SIGKILL"); } catch { /* gone */ } });
  const result = watchWindow(child, { budgetMs: 5000, killGraceMs: 20, listOwned: async () => [{pid:ready.worker,group:ready.worker}] });
  child.send("stop");
  const stopped = await result;
  assert.equal(stopped.stop_reason, "fixture_resource_stop");
  assert.equal(stopped.owned_group_gone, true);
  assert.equal(stopped.owned_executable_pids_gone, true);
});

test("repeated interruption during worker lookup does not remove the teardown handlers", async (t) => {
  const child = fixture();
  t.after(() => { try { process.kill(-child.pid, "SIGKILL"); } catch { /* gone */ } });
  const [ready] = await once(child, "message");
  t.after(() => { try { process.kill(-ready.worker, "SIGKILL"); } catch { /* gone */ } });
  let entered, release;
  const started = new Promise((resolve) => { entered = resolve; });
  const lookup = new Promise((resolve) => { release = resolve; });
  const before = process.listenerCount("SIGINT");
  const result = watchWindow(child, { budgetMs: 5000, listOwned: () => { entered(); return lookup; } });
  child.send("exit");
  await started;
  process.emit("SIGINT");
  process.emit("SIGINT");
  assert.equal(process.listenerCount("SIGINT"), before + 1);
  release([{pid:ready.worker,group:ready.worker}]);
  const stopped = await result;
  assert.equal(stopped.stop_reason, "runner_SIGINT");
  assert.equal(stopped.owned_executable_pids_gone, true);
  assert.equal(process.listenerCount("SIGINT"), before);
});

test("a failed owned-worker lookup is reported unknown, never clean", async (t) => {
  const child = fixture();
  t.after(() => { try { process.kill(-child.pid, "SIGKILL"); } catch { /* gone */ } });
  const [ready] = await once(child, "message");
  t.after(() => { try { process.kill(-ready.worker, "SIGKILL"); } catch { /* gone */ } });
  const result = watchWindow(child, { budgetMs: 5000, listOwned: async () => { throw new Error("fixture lookup failed"); } });
  child.send("exit");
  const stopped = await result;
  assert.equal(stopped.owned_group_gone, true);
  assert.equal(stopped.owned_executable_pids_gone, false);
  assert.equal(stopped.group_errors[0].signal, "enumerate");
});

test("a stalled worker lookup cannot make teardown wait indefinitely", async (t) => {
  const child = fixture();
  t.after(() => { try { process.kill(-child.pid, "SIGKILL"); } catch { /* gone */ } });
  const [ready] = await once(child, "message");
  t.after(() => { try { process.kill(-ready.worker, "SIGKILL"); } catch { /* gone */ } });
  const result = watchWindow(child, { budgetMs: 5000, listOwned: () => new Promise(() => {}) });
  child.send("exit");
  const stopped = await result;
  assert.equal(stopped.owned_group_gone, true);
  assert.equal(stopped.owned_executable_pids_gone, false);
  assert.match(stopped.group_errors[0].error, /timed out/);
});

test("run-only resource stops use aligned samples and refuse unknown readings", () => {
  const g = 1024 ** 3;
  const sample = { host: { available_percent: 30 }, p: [[1, "app", g, 4*g, 4*g], [2, "webcontent", g, 4*g, 4*g], [3, "worker", g, 4*g, 4*g]] };
  assert.equal(sampleStopReason(sample), null);
  assert.equal(sampleStopReason({ ...sample, host: {} }), "host_headroom_unknown");
  assert.equal(sampleStopReason({ ...sample, host: { available_percent: 29 } }), "host_headroom_below_30_percent");
  assert.equal(sampleStopReason({ ...sample, p: [...sample.p, [4,"webcontent",1,1,1]] }), "webcontent_attribution_ambiguous");
  assert.equal(sampleStopReason({ ...sample, p: [[1,"app",g,12*g+1,12*g+1]] }), "aligned_footprint_above_12_gib");
  assert.equal(sampleStopReason({ ...sample, p: [[1,"app",16*g+1,g,g]] }), "owned_pid_rss_above_16_gib");
  assert.equal(sampleStopReason({ ...sample, p: [[1,"app",null,null,null]] }), "process_memory_unknown");
});

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

test("plan names are bare: traversal, separators, absolute paths and dot names are refused", () => {
  for (const bad of ["../victim.json", "../../../victim.json", "a/b.json", "/etc/passwd", "..", ".", ".hidden", "a\\b", "", "x".repeat(200), 7, null]) {
    assert.throws(() => planName(bad, "name"), /not a bare name/, String(bad));
  }
  for (const good of ["linear-ex-n0250.json", "injected-linear-ex-n2000", "a_b.result.json"]) assert.equal(planName(good, "name"), good);
  const run = { id: "r", mode: "injected", model: "m.json", result: "r.json", estimate_s: 1, timeout_s: 1, paint_ms: 1 };
  assert.doesNotThrow(() => validatePlan({ runs: [run], subsets: { s: ["r"] } }));
  assert.throws(() => validatePlan({ runs: [{ ...run, model: "../../victim.json" }], subsets: {} }), /model/);
  assert.throws(() => validatePlan({ runs: [{ ...run, result: "/tmp/x.json" }], subsets: {} }), /result/);
  assert.throws(() => validatePlan({ runs: [{ ...run, id: "../x" }], subsets: {} }), /run id/);
  assert.throws(() => validatePlan({ runs: [{ ...run, mode: "other" }], subsets: {} }), /mode/);
  assert.throws(() => validatePlan({ runs: [run, run], subsets: {} }), /repeated/);
  assert.throws(() => validatePlan({ runs: [run], subsets: { s: ["missing"] } }), /unknown run/);
  assert.throws(() => validatePlan({ runs: [{ ...run, mode: "native" }], subsets: {} }), /native run r names a result/);
});

test("contained paths stay inside their root, also through symbolic links", (t) => {
  const base = mkdtempSync(path.join(os.tmpdir(), "perf-contained-"));
  t.after(() => rmSync(base, { recursive: true, force: true }));
  const root = path.join(base, "root");
  const outside = path.join(base, "outside");
  mkdirSync(root);
  mkdirSync(outside);
  writeFileSync(path.join(root, "model.json"), "{}");
  writeFileSync(path.join(outside, "victim.json"), "{}");
  mkdirSync(path.join(root, "dir.json"));
  symlinkSync(path.join(outside, "victim.json"), path.join(root, "escape.json"));
  symlinkSync(path.join(root, "model.json"), path.join(root, "inner.json"));
  symlinkSync(path.join(base, "nowhere.json"), path.join(root, "dangling.json"));
  assert.equal(containedPath(root, "model.json", { mustExist: true }), path.join(root, "model.json"));
  assert.equal(containedPath(root, "inner.json", { mustExist: true }), path.join(root, "inner.json"));
  assert.equal(containedPath(root, "new.json"), path.join(root, "new.json"));
  assert.throws(() => containedPath(root, "escape.json", { mustExist: true }), /outside/);
  assert.throws(() => containedPath(root, "escape.json"), /outside/, "a destination that is a link out is refused too");
  assert.throws(() => containedPath(root, "dangling.json"), /dangling/);
  assert.throws(() => containedPath(root, "dir.json", { mustExist: true }), /not a regular file/);
  assert.throws(() => containedPath(root, "absent.json", { mustExist: true }), /does not exist/);
  assert.throws(() => containedPath(root, "../outside/victim.json"), /not a bare name/);
  assert.throws(() => containedPath(root, path.join(outside, "victim.json")), /not a bare name/);
  // A root that is itself reached through a link still contains its own files.
  symlinkSync(root, path.join(base, "root-link"));
  assert.equal(containedPath(path.join(base, "root-link"), "model.json", { mustExist: true }), path.join(base, "root-link", "model.json"));
});

test("--keep keeps each run's directory; only the app copy is removed", async (t) => {
  const base = mkdtempSync(path.join(os.tmpdir(), "perf-keep-"));
  t.after(() => rmSync(base, { recursive: true, force: true }));
  const make = (name) => {
    const scratch = path.join(base, name);
    mkdirSync(path.join(scratch, "run-a-1"), { recursive: true });
    writeFileSync(path.join(scratch, "run-a-1", "report.json"), "{}");
    writeFileSync(path.join(scratch, "openpipestress-desktop"), "app");
    return { scratch, binary: path.join(scratch, "openpipestress-desktop") };
  };
  const kept = make("kept");
  await finishScratch({ ...kept, keep: true });
  assert.ok(existsSync(path.join(kept.scratch, "run-a-1", "report.json")));
  assert.ok(!existsSync(kept.binary));
  const removed = make("removed");
  await finishScratch({ ...removed, keep: false });
  assert.ok(!existsSync(removed.scratch));
});
