// The desktop perf runner's plan, sampling summary and classification, without the app.
//
//   node --test apps/desktop/native-smoke/perf.test.mjs
import { test } from "node:test";
import assert from "node:assert/strict";
import { fileURLToPath, pathToFileURL } from "node:url";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { execFile, execFileSync, spawn } from "node:child_process";
import { once } from "node:events";
import { c1PhysicalPhases, checkpointRun, classify, commitRun, readStartupHost, containedPath, createSampleGuard, finishScratch, formatPlan, inspectCsv, loadPlan, planName, sampleStopReason, selectRuns, summarizeSamples, validatePlan, watchWindow } from "./perf.mjs";

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
    if(m==='stop') {
      process.send({type:'completed_run',path:'fixture-run.json'});
      process.send({type:'stop',reason:'fixture_resource_stop',sample:{t:123,p:[[11,'app',1,2,2]],host:{available_percent:29,pressure_level:1}}});
    }
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
  assert.equal(stopped.trigger_sample.t, 123);
  assert.equal(stopped.evidence_complete, false);
  assert.deepEqual(stopped.completed_run_files,["fixture-run.json"]);
});

test("supervisor death ends a blocked runner and its fake App group", { timeout: 10000 }, async (t) => {
  const moduleUrl = pathToFileURL(fileURLToPath(new URL("./perf.mjs", import.meta.url))).href;
  const runnerCode = `
    import {guardSupervisor} from ${JSON.stringify(moduleUrl)};
    import {spawn} from 'node:child_process';
    import {once} from 'node:events';
    await guardSupervisor(Number(process.argv[1]));
    const app=spawn(process.execPath,['-e',"process.send('ready');setInterval(()=>{},1000)"],{stdio:['ignore','ignore','inherit','ipc']});
    await once(app,'message');
    process.send({runner:process.pid,app:app.pid});
    for(;;) {}
  `;
  const supervisor = spawn(process.execPath, ["--input-type=module", "-e", `
    import {spawn} from 'node:child_process';
    const runner=spawn(process.execPath,['--input-type=module','-e',${JSON.stringify(runnerCode)},String(process.pid)],{detached:true,stdio:['ignore','ignore','inherit','ipc']});
    process.send({started:runner.pid});
    runner.once('message',value=>process.send(value));
    setInterval(()=>{},1000);
  `], { stdio: ["ignore", "ignore", "inherit", "ipc"] });
  t.after(() => supervisor.kill("SIGKILL"));
  const [started] = await once(supervisor, "message");
  t.after(() => { try { process.kill(-started.started, "SIGKILL"); } catch { /* gone */ } });
  const [ready] = await once(supervisor, "message");
  t.after(() => { try { process.kill(-ready.runner, "SIGKILL"); } catch { /* gone */ } });
  process.kill(ready.app,"SIGSTOP"); // teardown must not freeze the guard thread
  supervisor.kill("SIGKILL");
  await once(supervisor, "exit");
  const alive = pid => { try { process.kill(pid,0); return true; } catch(error) { return error.code !== "ESRCH"; } };
  const until = performance.now() + 3000;
  while ((alive(ready.runner) || alive(ready.app)) && performance.now() < until) await new Promise(resolve=>setTimeout(resolve,20));
  assert.equal(alive(ready.runner), false);
  assert.equal(alive(ready.app), false);
});

test("SIGHUP takes the bounded supervised-stop path", async (t) => {
  const child = fixture();
  t.after(() => { try { process.kill(-child.pid,"SIGKILL"); } catch { /* gone */ } });
  const [ready] = await once(child,"message");
  t.after(() => { try { process.kill(-ready.worker,"SIGKILL"); } catch { /* gone */ } });
  const result = watchWindow(child,{budgetMs:5000,killGraceMs:20,listOwned:async()=>[{pid:ready.worker,group:ready.worker}]});
  process.emit("SIGHUP");
  const stopped = await result;
  assert.equal(stopped.stop_reason,"runner_SIGHUP");
  assert.equal(stopped.owned_executable_pids_gone,true);
});

test("normal guard handover neither kills nor keeps its runner alive", {timeout:5000}, async (t) => {
  const moduleUrl = new URL("./perf.mjs",import.meta.url).href;
  const child = spawn(process.execPath,["--input-type=module","-e",`
    import {guardSupervisor} from ${JSON.stringify(moduleUrl)};
    const finish=await guardSupervisor(process.ppid);
    await finish();
    process.disconnect();
  `],{detached:true,stdio:["ignore","ignore","inherit","ipc"]});
  t.after(()=>{try{process.kill(-child.pid,"SIGKILL");}catch{/* gone */}});
  const [code,signal] = await once(child,"exit");
  assert.equal(code,0);
  assert.equal(signal,null);
});

test("an internal runner error is reported before self-group teardown", {timeout:5000}, async (t) => {
  const moduleUrl = new URL("./perf.mjs",import.meta.url).href;
  const child = spawn(process.execPath,["--input-type=module","-e",`
    import {runChild} from ${JSON.stringify(moduleUrl)};
    await runChild(async()=>{throw new Error('fixture internal runner failure');},process.ppid);
  `],{detached:true,stdio:["ignore","ignore","pipe","ipc"]});
  t.after(()=>{try{process.kill(-child.pid,"SIGKILL");}catch{/* gone */}});
  let stderr = "";
  child.stderr.on("data",data=>{stderr+=data;});
  const closed = once(child,"close");
  const stopped = await watchWindow(child,{budgetMs:4000,killGraceMs:20});
  await closed;
  assert.match(stderr,/perf runner: fixture internal runner failure\n/);
  assert.equal(stopped.exit.reaped,true);
  assert.equal(stopped.owned_group_gone,true);
  assert.equal(stopped.evidence_complete,false);
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
  const sample = { host: { available_percent: 30, pressure_level: 1 }, p: [[1, "app", g, 4*g, 4*g], [2, "webcontent", g, 4*g, 4*g], [3, "worker", g, 4*g, 4*g]] };
  assert.equal(sampleStopReason(sample), null);
  assert.equal(sampleStopReason({ ...sample, host: {} }), "host_headroom_unknown");
  assert.equal(sampleStopReason({ ...sample, host: { available_percent: 29 } }), "host_headroom_below_30_percent");
  assert.equal(sampleStopReason({ ...sample, p: [...sample.p, [4,"webcontent",1,1,1]] }), "webcontent_attribution_ambiguous");
  assert.equal(sampleStopReason({ ...sample, p: [[1,"app",g,12*g+1,12*g+1]] }), "aligned_footprint_above_12_gib");
  assert.equal(sampleStopReason({ ...sample, p: [[1,"app",16*g+1,g,g]] }), "owned_pid_rss_above_16_gib");
  assert.equal(sampleStopReason({ ...sample, p: [[1,"app",null,null,null]] }), "process_memory_unknown");
  assert.equal(sampleStopReason({ ...sample, host: {available_percent:40,pressure_level:2} }), "host_pressure_elevated");
});

test("one unknown PID or cached host failure does not stop a healthy run", () => {
  const sample = {t:1,host:{available_percent:40,pressure_level:1,sampled_at_ms:1},p:[[1,"app",10,10,10]]};
  const guard = createSampleGuard();
  assert.equal(guard({...sample,p:[...sample.p,[2,"worker",null,null,null]]}),null);
  assert.equal(guard(sample),null); // exited worker no longer listed
  assert.equal(guard({...sample,p:[...sample.p,[2,"worker",null,null,null]]}),null);
  assert.equal(guard({...sample,p:[...sample.p,[2,"worker",null,null,null]]}),"process_memory_unknown");
  const hostGuard = createSampleGuard();
  const unknown = {...sample,host:{available_percent:null,pressure_level:1,sampled_at_ms:2}};
  assert.equal(hostGuard(unknown),null);
  assert.equal(hostGuard(unknown),null); // same cached 1Hz reading
  assert.equal(hostGuard({...unknown,host:{...unknown.host,sampled_at_ms:3}}),"host_headroom_unknown");
  assert.equal(createSampleGuard()({...unknown,p:[[1,"app",17*1024**3,10,10]]}),"owned_pid_rss_above_16_gib");
});

test("completed records and raw samples survive private store cleanup", async (t) => {
  const out = mkdtempSync(path.join(os.tmpdir(), "perf-evidence-"));
  t.after(() => rmSync(out,{recursive:true,force:true}));
  const scratch = path.join(out,"perf-window");
  const run = path.join(scratch,"run-one");
  mkdirSync(path.join(run,"home"),{recursive:true});
  writeFileSync(path.join(run,"home","private.sqlite"),"fixture");
  const samples = path.join(out,"samples.jsonl");
  writeFileSync(samples,'{"t":123,"p":[]}\n');
  const record = {id:"one",class:"ok",kept:run,samples_file:samples,memory:{timeline:[[123,10,10]]}};
  await commitRun(record,{out,scratch,keep:false});
  const persisted = JSON.parse(readFileSync(record.record_file,"utf8"));
  assert.deepEqual(persisted.memory,record.memory);
  assert.equal(persisted.home_cleanup,"removed");
  assert.equal(existsSync(path.join(run,"home")),false);
  assert.equal(readFileSync(samples,"utf8"),'{"t":123,"p":[]}\n');
  await finishScratch({scratch,binary:path.join(scratch,"openpipestress-desktop"),keep:false});
  assert.equal(existsSync(record.record_file),true);
  assert.equal(existsSync(samples),true);
});

test("a bad completed-run root cannot remove another directory's store", async (t) => {
  const out = mkdtempSync(path.join(os.tmpdir(),"perf-cleanup-boundary-"));
  t.after(()=>rmSync(out,{recursive:true,force:true}));
  const scratch = path.join(out,"perf-window");
  mkdirSync(scratch);
  const foreign = path.join(out,"run-foreign");
  mkdirSync(path.join(foreign,"home"),{recursive:true});
  const marker = path.join(foreign,"home","keep.sqlite");
  writeFileSync(marker,"fixture outside window");
  await assert.rejects(commitRun({id:"bad",class:"ok",kept:foreign},{out,scratch,keep:false}),/not this window/);
  assert.equal(readFileSync(marker,"utf8"),"fixture outside window");
});

test("the sampler writer flushes raw evidence before a consumer can abort it", (t) => {
  const out = mkdtempSync(path.join(os.tmpdir(),"perf-writer-"));
  t.after(() => rmSync(out,{recursive:true,force:true}));
  const file = path.join(out,"samples.jsonl");
  // Execute only the production pure writer's AST, not the module, ctypes,
  // sampler main, sysctl or any host/process/memory probe.
  execFileSync("python3",["-c",`
import ast,json,pathlib,sys
source=pathlib.Path(sys.argv[1])
function=next(n for n in ast.parse(source.read_text()).body if isinstance(n,ast.FunctionDef) and n.name=='emit_sample')
scope={'json':json}
exec(compile(ast.Module(body=[function],type_ignores=[]),str(source),'exec'),scope)
value={'t':123,'p':[[11,'app',1,2,2]]}
class Consumer:
 def write(self,line):
  assert json.loads(pathlib.Path(sys.argv[2]).read_text())==value
  raise RuntimeError('fixture consumer stop')
 def flush(self): raise AssertionError('not reached')
with open(sys.argv[2],'x',encoding='utf-8') as evidence:
 try: scope['emit_sample'](value,evidence,Consumer())
 except RuntimeError as error: assert str(error)=='fixture consumer stop'
 else: raise AssertionError('consumer did not stop')
`,fileURLToPath(new URL("./perf-sampler.py",import.meta.url)),file]);
  assert.equal(JSON.parse(readFileSync(file,"utf8")).t,123);
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

test("the C1 plan binds maintained 500/1000 fixtures and forbids injected or larger cases", () => {
  const c1=loadPlan(fileURLToPath(new URL("./perf-plan-c1.json",import.meta.url)));
  const ledger=readFileSync(new URL("../../../core/solver/performance_harness/observations/rack/models_sha256.txt",import.meta.url),"utf8");
  for(const run of c1.runs) assert.match(ledger,new RegExp(`^${run.model_sha256}  linear-ex  ${run.nodes}$`,"m"));
  assert.deepEqual(selectRuns(c1,{subset:"window"}).map(run=>run.nodes),[500]);
  c1.runs[0].nodes=2000;
  assert.throws(()=>validatePlan(c1),/no size escalation/);
  c1.runs[0].nodes=500;c1.runs[0].mode="injected";
  assert.throws(()=>validatePlan(c1),/require a native solve/);
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
  for (const name of ["recordRun", "saveProject", "solve", "waitFor"]) assert.equal(typeof hooks[name], "function");
  await assert.rejects(hooks.waitFor("nothing", () => null, 30, 5), /timed out after 30 ms waiting for nothing/);
});

async function driverDom(t, html, invoke) {
  const { JSDOM } = await import("jsdom");
  const vm = await import("node:vm");
  const dom = new JSDOM(html);
  t.after(() => dom.window.close());
  const hooks = {};
  dom.window.__TAURI_INTERNALS__ = { invoke };
  const context = { window: dom.window, document: dom.window.document, navigator: dom.window.navigator,
    // Hex escapes are literal in jsdom as well as WebKit, including encoded IDs.
    CSS: { escape: value => String(value).replace(/[^a-zA-Z0-9_-]/gu, char => `\\${char.codePointAt(0).toString(16)} `) }, performance,
    Event: dom.window.Event, HTMLInputElement: dom.window.HTMLInputElement, HTMLTextAreaElement: dom.window.HTMLTextAreaElement,
    setTimeout, clearTimeout, setInterval, clearInterval, requestAnimationFrame: callback => setTimeout(() => callback(performance.now()), 0),
    TextEncoder, TextDecoder, ArrayBuffer, Uint8Array, DataView,
    __SWBPIPE_PERF_DRIVER_TEST__: hooks };
  vm.runInNewContext(readFileSync(new URL("./perf-driver.js",import.meta.url),"utf8"), context);
  return { dom, hooks, context, call: (...args) => dom.window.__TAURI_INTERNALS__.invoke(...args) };
}

function rowPacket(total) {
  const header = new TextEncoder().encode(JSON.stringify({ document_kind:"swbpipe.desktop.result_detail_rows",version:1,total,offset:0,returned:0,ordinals:[],lengths:[],truncated:false }));
  const bytes = new Uint8Array(16+header.length);
  bytes.set(new TextEncoder().encode("SWBDETv1"));
  new DataView(bytes.buffer).setBigUint64(8,BigInt(header.length),true);
  bytes.set(header,16);
  return bytes.buffer;
}

test("C1 command observation retains only headers/receipts and blocks full-copy routes", async (t) => {
  let sent = 0;
  const ui = await driverDom(t,"",async command => {
    sent++;
    if(command==="query_preview_mechanics_job_rows") return rowPacket(27);
    if(command==="finish_local_csv_stream") return {file_name:"swbpipe-results-force.csv",byte_count:123};
    if(command==="save_preview_mechanics_job_local_project") return {result:{outcome:"not_saved",code:"PROJECT-SAVE-RESULT-TOO-LARGE",get rows(){throw new Error("row payload read");}}};
    return null;
  });
  const trace = ui.hooks.beginC1Trace();
  trace.data.csv={family:"force",pages:0,expected_rows:null,receipt:null};
  await ui.call("query_preview_mechanics_job_rows",new TextEncoder().encode(JSON.stringify({filter:{family:"force"}})));
  await ui.call("finish_local_csv_stream",{});
  await ui.call("save_preview_mechanics_job_local_project",{});
  assert.equal(trace.data.csv.expected_rows,27);
  assert.equal(trace.data.csv.receipt.byte_count,123);
  assert.equal(trace.data.save.result.outcome,"not_saved");
  assert.equal("rows" in trace.data.save.result,false);
  assert.equal(trace.data.errors.length,0);
  await assert.rejects(ui.call("read_preview_mechanics_job_rows_all",{}),/unexpected whole-value/);
  assert.equal(sent,3);
  assert.equal(trace.data.violations[0],"read_preview_mechanics_job_rows_all");
  trace.restore();
});

test("C1 preflight refuses changed budgets and pre-existing custody", async (t) => {
  const pool={headroom:{pressure:"normal",available_bytes:30*1024**3},invocation_budget_bytes:12*1024**3,retained_results:0,running_workers:0,queued_jobs:0,running_jobs:0,webview_custody_results:0,reserved_bytes:0};
  const ui=await driverDom(t,"",async()=>pool);
  await ui.hooks.preflight({});
  pool.retained_results=1;
  await assert.rejects(ui.hooks.preflight({}),/prior-job state/);
  pool.retained_results=0;pool.invocation_budget_bytes=13*1024**3;
  await assert.rejects(ui.hooks.preflight({}),/budget/);
  pool.invocation_budget_bytes=12*1024**3;pool.headroom.pressure="warning";
  await assert.rejects(ui.hooks.preflight({}),/pressure\/headroom/);
});

test("C1 pool monitoring tolerates one unknown and records cancellation on persistent failure", async (t) => {
  let clock=0,tick;
  const timeline={outcome:"running"};
  const pool={headroom:{pressure:"unknown",available_bytes:null}};
  const ui=await driverDom(t,"",async(command,args)=>{
    if(command==="get_memory_pool_status") return pool;
    if(command==="native_smoke_menu" && args.commandId==="analyze.cancel") timeline.outcome="cancelled";
    return null;
  });
  ui.context.performance={now:()=>clock};
  ui.context.setInterval=callback=>{tick=callback;return 1;};
  ui.context.clearInterval=()=>{tick=null;};
  ui.context.__openPipeStressSolveTimelineV1={readCurrent:()=>timeline};
  const report={};
  const stop=ui.hooks.startPoolMonitor(report);
  clock=1000;tick();await new Promise(resolve=>setImmediate(resolve));
  assert.equal(await ui.hooks.waitFor("one unknown",()=>true,10),true);
  clock=2000;tick();await new Promise(resolve=>setImmediate(resolve));
  await assert.rejects(ui.hooks.waitFor("persistent unknown",()=>true,10),/stayed unknown/);
  await ui.hooks.cancelIfRunning(report);
  assert.equal(report.cancellation.attempted,true);
  assert.equal(report.cancellation.receipt,"timeline cancelled");
  stop();assert.equal(tick,null);
});

function installC1ResultControls(ui, root) {
  const state={kind:0,case:0,entity:0,page:0,filter:"",selected:null};
  const render=()=>{
    const ids=state.filter?[]:state.page?["row-C","row-D"]:["row-A","row-B"];
    root.innerHTML=`<div data-testid="result-page-summary">page ${state.page+1} of 2</div>
      <input data-testid="result-filter-input" value="${state.filter}"><button data-testid="clear-result-filter">Clear</button>
      <button data-testid="next-result-page" ${state.page?"disabled":""}>Next</button><button data-testid="previous-result-page" ${state.page?"":"disabled"}>Previous</button>
      ${["kind","case","entity"].map(axis=>`<span data-testid="results-navigator-${axis}-position">${state[axis]}</span><button data-testid="results-navigator-${axis}-next" aria-disabled="${state[axis]===1}">Next</button><button data-testid="results-navigator-${axis}-previous" aria-disabled="${state[axis]===0}">Previous</button>`).join("")}
      ${state.filter?'<p data-testid="result-filter-empty">No computed preview result rows match this filter.</p>':""}
      <table><tbody>${ids.map(id=>`<tr data-testid="result-row-${id}" aria-selected="${state.selected===id}"><td>${id}</td></tr>`).join("")}</tbody></table>
      ${state.selected?`<p data-testid="result-detail-id">${state.selected}</p>`:""}
      <button data-testid="result-group-displacement-export-csv">Export CSV</button><span data-testid="result-group-displacement-export-csv-status"></span>`;
    for(const row of root.querySelectorAll("tr")) row.onclick=()=>{state.selected=row.getAttribute("data-testid").slice(11);render();};
    for(const axis of ["kind","case","entity"]) for(const direction of ["next","previous"]) root.querySelector(`[data-testid="results-navigator-${axis}-${direction}"]`).onclick=()=>{
      state[axis]=direction==="next"?1:0;
      // Case/kind steps may legitimately keep the same first published ID.
      state.selected=state.entity?"row-B":"row-A";render();
    };
    for(const direction of ["next","previous"]) root.querySelector(`[data-testid="${direction}-result-page"]`).onclick=()=>{state.page=direction==="next"?1:0;render();};
    root.querySelector("input").oninput=event=>{state.filter=event.target.value;state.page=0;render();};
    root.querySelector('[data-testid="clear-result-filter"]').onclick=()=>{state.filter="";render();};
    root.querySelector('[data-testid="result-group-displacement-export-csv"]').onclick=()=>{void(async()=>{
      await ui.call("query_preview_mechanics_job_rows",{filter:{family:"displacement"}});
      await ui.call("query_preview_mechanics_job_rows",{filter:{family:"displacement"}});
      await ui.call("finish_local_csv_stream",{});
      root.querySelector('[data-testid="result-group-displacement-export-csv-status"]').textContent="Saved swbpipe-results-displacement.csv (123 bytes).";
    })();};
  };
  render();
  return state;
}

test("C1 DOM probe covers all navigator axes, pages, no-match and streamed CSV", async (t) => {
  let ui;
  ui=await driverDom(t,'<section data-testid="workspace-section-results"><section data-testid="results-panel" data-result-custody="native"></section></section>',async command=>{
    if(command==="query_preview_mechanics_job_rows") return rowPacket(2);
    if(command==="finish_local_csv_stream") return {file_name:"swbpipe-results-displacement.csv",byte_count:123};
    return null;
  });
  const doc=ui.dom.window.document;
  const section=doc.querySelector('[data-testid="workspace-section-results"]');
  Object.defineProperty(section,"offsetParent",{get:()=>doc.body});
  const root=doc.querySelector('[data-testid="results-panel"]');
  installC1ResultControls(ui,root);
  const trace=ui.hooks.beginC1Trace();
  const report={};
  await ui.hooks.probeResults({timeouts:{open_ms:1000,interaction_ms:1000,csv_ms:1000}},report,trace,{csv:true});
  assert.equal(report.actions.length,11);
  assert.equal(report.csv.expected_rows,2);
  assert.equal(report.csv.family,"displacement");
  assert.equal(report.csv.receipt.byte_count,123);
  assert.equal(report.actions.find(action=>action.name==="case next").before.selected,report.actions.find(action=>action.name==="case next").after.selected);
  trace.restore();
});

test("C1 persistence probe connects actual Rust reply shapes through record/reopen/re-save/check/release", async (t) => {
  // These committed Rust replies establish DTO shape, not live App behaviour.
  const persistence=JSON.parse(readFileSync(new URL("../src/test-support/native-replay/native_persistence_replies.json",import.meta.url),"utf8"));
  const reopen=JSON.parse(readFileSync(new URL("../src/test-support/native-reopen/native_reopen_replies.json",import.meta.url),"utf8"));
  const save=persistence.scenarios[0].reply.value;
  const recorded=structuredClone(persistence.scenarios[1].reply.value);
  const opened=reopen.open.reply.value;
  const resave=reopen.requests.find(item=>item.command==="save_restored_result_local_project").reply.value;
  const read=reopen.read.reply.value;
  const handle=opened.stored_run.handle;
  const readHandle="restored-stored-result-record-check";
  const pool={headroom:{pressure:"normal",available_bytes:30*1024**3},invocation_budget_bytes:12*1024**3,retained_results:0,running_workers:0,queued_jobs:0,running_jobs:0,webview_custody_results:0,reserved_bytes:0};
  let ui, historical=false;
  const projectId=opened.model.project.id;
  const plan={run_id:"c1-fixture",project_id:projectId,model_path:"/invented/c1-fixture.json",timeouts:{open_ms:2000,record_ms:2000,save_ms:2000,interaction_ms:2000,csv_ms:2000}};
  const html=`<p data-testid="local-project-message">Initial</p><div data-testid="tree-row-project-${encodeURIComponent(projectId)}"></div>
    <section data-testid="workspace-section-results"><section data-testid="results-panel" data-result-custody="native"></section></section>
    <section data-testid="workspace-section-project"><div data-testid="model-history-panel"><p data-testid="model-history-message"></p><button data-testid="record-run-open">Record</button><div data-testid="forms"></div><div data-testid="runs"></div></div><p data-testid="project-validation-envelope-hash">integrity=verified_match</p></section>`;
  ui=await driverDom(t,html,async(command,args)=>{
    const doc=ui?.dom.window.document;
    const setMessage=value=>{doc.querySelector('[data-testid="local-project-message"]').textContent=value;};
    if(command==="get_memory_pool_status") return pool;
    if(command==="save_preview_mechanics_job_local_project") return save;
    if(command==="record_preview_mechanics_job_analysis_run") return recorded;
    if(command==="open_local_project_in_native_custody") return opened;
    if(command==="save_restored_result_local_project") return resave;
    if(command==="read_analysis_run_in_native_custody") return {...read,restored:{...read.restored,handle:readHandle}};
    if(command==="dispose_preview_mechanics_result") return {job_id:args.jobId,released:true};
    if(command==="native_smoke_menu") {
      if(args.commandId.startsWith("view.section.")) return null;
      if(args.commandId==="file.save-local") {
        await ui.call(historical?"save_restored_result_local_project":"save_preview_mechanics_job_local_project",{});
        setMessage(historical?"Saved restored SQLite project snapshot.":"Saved current project model snapshot.");
      }
      if(args.commandId==="file.open-local") {
        await ui.call("open_local_project_in_native_custody",{});historical=true;pool.retained_results=1;
        const container=doc.createElement("div");container.setAttribute("data-testid","historical-run-context");
        const section=doc.querySelector('[data-testid="workspace-section-results"]');
        const panel=section.querySelector('[data-testid="results-panel"]');section.appendChild(container);container.appendChild(panel);
        const unavailable=doc.createElement("p");unavailable.setAttribute("data-testid","record-run-unavailable");unavailable.textContent="Record Run: historical saved run";doc.body.appendChild(unavailable);
        setMessage("Opened local SQLite project snapshot.");
      }
      if(args.commandId==="file.open-model") {
        await ui.call("dispose_preview_mechanics_result",{jobId:handle});pool.retained_results=0;
        doc.querySelector('[data-testid="historical-run-context"]').remove();
        setMessage("Opened model document c1-fixture.json (fixture).");
      }
    }
    return null;
  });
  const doc=ui.dom.window.document;
  for(const section of doc.querySelectorAll('[data-testid^="workspace-section-"]')) Object.defineProperty(section,"offsetParent",{get:()=>doc.body});
  installC1ResultControls(ui,doc.querySelector('[data-testid="results-panel"]'));
  doc.querySelector('[data-testid="record-run-open"]').onclick=()=>{
    const forms=doc.querySelector('[data-testid="forms"]');
    forms.innerHTML='<div data-testid="record-run-form"><input data-testid="record-run-name"><input data-testid="record-run-snapshot-name"><button type="button" data-testid="record-run-submit">Submit</button></div>';
    forms.querySelector("button").onclick=()=>{void(async()=>{
      recorded.run.run_name=forms.querySelector('[data-testid="record-run-name"]').value;
      await ui.call("save_preview_mechanics_job_local_project",{});
      await ui.call("record_preview_mechanics_job_analysis_run",{});
      forms.innerHTML="";
      doc.querySelector('[data-testid="model-history-message"]').textContent="Recorded run perf c1-fixture.";
      const id=recorded.run.recording_id;
      const runs=doc.querySelector('[data-testid="runs"]');
      runs.innerHTML=`<button data-testid="recorded-run-${id}-check-run">Check</button><div data-testid="recorded-run-${id}-check"></div>`;
      runs.querySelector("button").onclick=()=>{void(async()=>{
        await ui.call("read_analysis_run_in_native_custody",{});
        await ui.call("dispose_preview_mechanics_result",{jobId:readHandle});
        runs.querySelector("div").textContent="The record, its stored result and input manifest agree.";
      })();};
    })();};
  };
  const trace=ui.hooks.beginC1Trace();
  const report={c1:{preflight:{...pool}}};
  await ui.hooks.probePersistence(plan,report,trace);
  assert.equal(report.c1.resave.kept,true);
  assert.equal(report.c1.reopen.origin,"restored_saved_run");
  assert.equal(report.c1.recorded_check.origin,"restored_recorded_run");
  assert.equal(report.c1.cleanup.retained_results,0);
  assert.equal(trace.data.counts.save_restored_result_local_project,1);
  assert.equal(trace.data.violations.length,0);
  trace.restore();
});

test("C1 CSV file inspection counts quoted rows and checks receipt and containment", async (t) => {
  const home=mkdtempSync(path.join(os.tmpdir(),"perf-csv-"));
  t.after(()=>rmSync(home,{recursive:true,force:true}));
  mkdirSync(path.join(home,"Downloads"));
  const text='\ufeffexport_context,local_private\r\nreleased_report,false\r\n\r\nID,Value\r\n"row\nA",1\r\nrow-B,2\r\n';
  const file=path.join(home,"Downloads","swbpipe-results-force.csv");
  writeFileSync(file,text);
  const exported={expected_rows:2,receipt:{file_name:path.basename(file),byte_count:Buffer.byteLength(text)}};
  const good=await inspectCsv(home,exported);
  assert.equal(good.verified,true);assert.equal(good.rows,2);
  assert.equal((await inspectCsv(home,{...exported,expected_rows:3})).verified,false);
  assert.match((await inspectCsv(home,{...exported,receipt:{...exported.receipt,byte_count:1}})).error,/byte count/);
  assert.match((await inspectCsv(home,{...exported,receipt:{...exported.receipt,file_name:"../foreign.csv"}})).error,/bare CSV name/);
});

test("C1 incomplete probes and deliberate model-only saves cannot pass usability", () => {
  const input=base();input.run.c1_probes=true;input.report.c1={completed:false,trace:{save:{result:{outcome:"not_saved",code:"PROJECT-SAVE-RESULT-TOO-LARGE"}}}};
  input.store.projects[0].result_chars=4;
  const refused=classify(input);
  assert.equal(refused.class,"refused_with_message");
  assert.equal(refused.findings.some(f=>f.class==="half_written_save"),false);
  input.store.projects[0].result_chars=1000;
  input.report.c1={completed:true,csv_file:{verified:false,error:"inspection timed out"}};
  assert.equal(classify(input).class,"refused_with_message");
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

test("C1 reader probe sends only the bound handle and keeps phase metrics separate", async t => {
  const proof="identity=match; rows=1001; job=solve-123; model_sha256=sha256:model";
  let sent;
  const phase={allocation:{peak_requested:32,peak_priced:64,live_requested:16,live_priced:32,allocations:2},elapsed_us:1};
  const metrics={schema:"swbpipe.c1-reader-allocations/v1",job_id:"solve-123",rows:1001,source_parse:phase,dispatch_working:phase,registration_working:phase,semantics_working_max:phase,summary_working_and_response:phase,semantics_batches:2,semantics_batch_limit:1000};
  for(const key of ["model_parse","invocation_parse","health_parse_and_working","health_response_parse","health_view_working","summary_source_parse"]) metrics[key]=phase;
  Object.assign(metrics,{started_ms:1,finished_ms:2,route:"pressure",reader:"pressure",source_bytes:10,publication_sha256:`sha256:${"a".repeat(64)}`,intake_sha256:`sha256:${"b".repeat(64)}`,source_parse_bound_bytes:32,model_parse_bound_bytes:32,invocation_parse_bound_bytes:32,health_working_upper_bound_requested:32,health_working_upper_bound_priced:64,summary_response_bytes:10,coverage:"fixture metrics only"});
  const ui=await driverDom(t,`<div data-testid="status-pill-solve-proof"><code>${proof}</code></div>`,async(command,args,options)=>{
    if(command==="native_smoke_profile_readers") { sent={args:JSON.parse(new TextDecoder().decode(args)),options};return metrics; }
    return null;
  });
  const report={};
  await ui.hooks.profileReaders({model_jcs_sha256:"sha256:model",timeouts:{profile_ms:1000}},report);
  assert.deepEqual(sent.args,{jobId:"solve-123"});
  assert.equal(sent.options.headers["x-swbpipe-ingress"],"args-json-v1");
  assert.equal(report.reader_allocations.source_parse.allocation.peak_requested,32);
  metrics.result={get rows(){throw new Error("a result payload must not be retained");}};
  await assert.rejects(ui.hooks.profileReaders({model_jcs_sha256:"sha256:model",timeouts:{profile_ms:1000}},{}),/not bound/);
  delete metrics.result;
  metrics.job_id="wrong";
  await assert.rejects(ui.hooks.profileReaders({model_jcs_sha256:"sha256:model",timeouts:{profile_ms:1000}},{}),/not bound/);
});


test("C1 phase physical peaks align readings and mark missing-worker inference", () => {
  const sample=(t,app,worker,web=20)=>({t,p:[[1,"app",app,app,app],[2,"webcontent",web,web,web],...(worker===undefined?[]:[[3,"worker",worker,worker,worker]])]});
  const out=c1PhysicalPhases([sample(1,10,80),sample(2,100,undefined),sample(3,200,undefined),sample(4,999,undefined,null)],{driver_started_ms:2,driver_finished_ms:2},3);
  assert.equal(out.all.aligned_sampled_peak.bytes,220);
  assert.equal(out.all.unknown_samples,2);
  assert.equal(out.before_reader_probe.aligned_sampled_peak.bytes,110);
  assert.equal(out.reader_probe.aligned_sampled_peak,null);
  assert.equal(out.after_reader_probe.aligned_sampled_peak.worker_zero_inferred,true);
  assert.equal(out.all.completed_worker_zero_samples,1);
});


test("C1 pool monitoring starts staleness at dispatch and tolerates a delayed tick", async t => {
  let clock=0,tick,resolve;
  const pool={headroom:{pressure:"normal",available_bytes:30*1024**3}};
  let delayed=false;
  const ui=await driverDom(t,"",async command => command==="get_memory_pool_status" ? delayed ? new Promise(done=>{resolve=()=>done(pool);}) : pool : null);
  ui.context.performance={now:()=>clock};
  ui.context.setInterval=callback=>{tick=callback;return 1;};
  ui.context.clearInterval=()=>{tick=null;};
  const stop=ui.hooks.startPoolMonitor({});
  clock=10000;tick();await new Promise(done=>setImmediate(done));
  assert.equal(await ui.hooks.waitFor("fresh request after UI stall",()=>true,10),true);
  delayed=true;clock=11000;tick();
  clock=15000;tick();
  assert.equal(await ui.hooks.waitFor("one stale pending tick",()=>true,10),true);
  clock=16000;tick();
  await assert.rejects(ui.hooks.waitFor("pending request remained stale",()=>true,10),/request stayed stale/);
  resolve();await new Promise(done=>setImmediate(done));stop();
});

test("post-App inspection checkpoint survives an unfinished inspection and cannot pass", async t => {
  const out=mkdtempSync(path.join(os.tmpdir(),"perf-partial-"));
  t.after(()=>rmSync(out,{recursive:true,force:true}));
  const scratch=path.join(out,"perf-window");
  mkdirSync(scratch);
  const ctx={out,scratch,keep:true};
  const report={c1:{completed:true},solve:{ended:"first_paint"}};
  const record={id:"one",report,memory:{timeline:[[123,10,10]]},store:{inspection:"pending"}};
  const checkpoint=checkpointRun(record,ctx);
  const partial=JSON.parse(readFileSync(checkpoint,"utf8"));
  assert.equal(partial.evidence_complete,false);
  assert.equal(partial.class,"refused_with_message");
  assert.deepEqual(partial.memory,record.memory);
  assert.deepEqual(partial.report,report);
  assert.match(partial.findings[0].message,/not a successful run/);
  await commitRun({...record,class:"ok",store:{quick_check:["ok"]}},ctx);
  assert.equal(existsSync(checkpoint),false);
});

// Execute only extracted Python functions against deterministic fakes: no
// sampler import, ctypes/libproc, real sysctl, App, solver or memory reading.
function simulatedSampler() {
  return JSON.parse(execFileSync("python3", ["-I", "-c", `
import argparse,ast,io,json,os,pathlib,subprocess,sys,types
source=pathlib.Path(sys.argv[1])
functions=[n for n in ast.parse(source.read_text()).body if isinstance(n,ast.FunctionDef) and n.name in ('main','emit_sample','read_host_status')]
def scenario(mode,host_only=False):
 clock=[0.015]
 queries=[]
 handlers={}
 output=io.StringIO()
 def read(*args,**kwargs):
  queries.append(clock[0])
  assert kwargs['timeout']==0.5
  if mode=='stalled':
   clock[0]+=0.5
   raise subprocess.TimeoutExpired('fake sysctl',0.5)
  if mode=='failed': raise subprocess.CalledProcessError(1,'fake sysctl')
  return types.SimpleNamespace(stdout='70\\n1\\n')
 def sleep(dt):
  clock[0]+=dt
  if clock[0]>=1.401: handlers[15](15,None)
 scope={'argparse':argparse,'json':json,'os':os,'sys':types.SimpleNamespace(stdout=output),
  'time':types.SimpleNamespace(monotonic=lambda:clock[0],time=lambda:1700000000+clock[0],sleep=sleep),
  'signal':types.SimpleNamespace(SIGTERM=15,signal=lambda signum,callback:handlers.update({signum:callback})),
  'print':lambda text:output.write(str(text)+'\\n'),
  'subprocess':types.SimpleNamespace(run=read,SubprocessError=subprocess.SubprocessError),
  'all_pids':lambda:(_ for _ in ()).throw(AssertionError('host admission read PID counters')) if host_only else [11],'pid_path':lambda pid:'/fixture/not-an-app','kind_of':lambda *args:'app','rusage':lambda pid:(1,2,3)}
 exec(compile(ast.Module(body=functions,type_ignores=[]),str(source),'exec'),scope)
 scope['main'](['--host-status','--app-path','/fixture/not-an-app'] if host_only else ['--app-pid','11','--app-path','/fixture/not-an-app','--interval-ms','200'])
 return {'queries':queries,'samples':[json.loads(line) for line in output.getvalue().splitlines() if '"host"' in line], 'control':json.loads(output.getvalue()) if host_only else None}
result={mode:scenario(mode) for mode in ['known','failed','stalled']}
result['admission']=scenario('known',True)
print(json.dumps(result))
`, fileURLToPath(new URL("./perf-sampler.py", import.meta.url))], { encoding: "utf8" }));
}

test("sampler refreshes before its first sample with a process-relative clock near zero", () => {
  const cases=simulatedSampler();
  assert.equal(cases.admission.queries.length,1);
  assert.equal(cases.admission.queries[0],0.015);
  assert.equal(cases.admission.control.available_percent,70);
  assert.equal(cases.admission.control.pressure_level,1);
  assert.equal(cases.known.queries[0],0.015);
  assert.equal(cases.known.samples[0].host.available_percent,70);
  assert.equal(cases.known.samples[0].host.pressure_level,1);
  assert.ok(Number.isFinite(cases.known.samples[0].host.sampled_at_ms));
  assert.equal(cases.known.queries.length,2);
  const guard=createSampleGuard();
  for(const sample of cases.known.samples) assert.equal(guard(sample),null);
  for(const name of ['failed','stalled']) {
    const data=cases[name], guarded=createSampleGuard();
    assert.equal(data.queries[0],0.015);
    assert.equal(data.queries.length,2);
    const stamps=new Set();
    for(const sample of data.samples) {
      assert.equal(sample.host.available_percent,null);
      assert.equal(sample.host.pressure_level,null);
      assert.ok(Number.isFinite(sample.host.sampled_at_ms));
      stamps.add(sample.host.sampled_at_ms);
      assert.equal(guarded(sample),stamps.size<2?null:'host_headroom_unknown');
    }
    assert.equal(stamps.size,2);
  }
});


test("startup host admission needs one fresh known-normal reading before an App is allowed", async () => {
  const known={available_percent:70,pressure_level:1,sampled_at_ms:1001};
  const request=sample=>{let calls=0;return readStartupHost('/fixture/not-an-app',(command,args,options,callback)=>{
    assert.equal(command,'python3');assert.ok(args.includes('--host-status'));
    assert.equal(options.timeout,1500);assert.equal(options.killSignal,'SIGKILL');
    callback(null,JSON.stringify(sample));
  },()=>calls++?1002:1000);};
  assert.equal((await request(known)).available_percent,70);
  for(const sample of [null,{...known,available_percent:null},{...known,available_percent:29},{...known,pressure_level:2},{...known,pressure_level:4},{...known,pressure_level:0},{...known,sampled_at_ms:900},{...known,sampled_at_ms:1010}]) {
    await assert.rejects(request(sample),/HOST-ADMISSION-REFUSED.*no App launched/);
  }
  await assert.rejects(readStartupHost('/fixture/not-an-app',(_c,_a,_o,callback)=>callback(new Error('fixture failure'))),/failed or stalled/);
  await assert.rejects(readStartupHost('/fixture/not-an-app',(_c,_a,_o,callback)=>callback(null,'not-json')),/malformed/);
});

test("a stalled startup telemetry process is killed and refuses admission", async () => {
  let childPid;
  const execute=(_command,_args,options,callback)=>{
    const child=execFile(process.execPath,['-e','setInterval(()=>{},1000)'],options,callback);
    childPid=child.pid;
  };
  await assert.rejects(readStartupHost('/fixture/not-an-app',execute),/failed or stalled/);
  assert.throws(()=>process.kill(childPid,0),/ESRCH/);
});
