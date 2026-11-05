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
import { c1PhysicalPhases, createReaderPhaseEvidence, checkpointRun, classify, commitRun, readStartupHost, containedPath, createSampleGuard, finishScratch, formatPlan, inspectCsv, loadPlan, planName, sampleStopReason, selectRuns, summarizeSamples, validatePlan, watchWindow } from "./perf.mjs";

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

test("the small C1 plan binds the generator's 82-node rack and the 500-node gate's timeouts", async () => {
  const small=loadPlan(fileURLToPath(new URL("./perf-plan-c1-small.json",import.meta.url)));
  const c1=loadPlan(fileURLToPath(new URL("./perf-plan-c1.json",import.meta.url)));
  assert.deepEqual(small.runs.map(run=>[run.nodes,run.mode,run.paint_ms,run.timeout_s]),[[82,"native",300000,720]]);
  for(const key of ["c1_probes","c1_reader_diagnostics","c1_profile_readers","timeouts","budget_s","sample_interval_ms"]) assert.deepEqual(small[key],c1[key]);
  // The rack generator's deterministic bytes are the bound model (perf.mjs checks the file's SHA-256 at launch).
  const generator=fileURLToPath(new URL("../../../core/solver/performance_harness/rack/rack_model.py",import.meta.url));
  const bytes=execFileSync("python3",[generator,"--profile","linear-ex","--nodes","82"],{stdio:["ignore","pipe","ignore"],maxBuffer:1<<24});
  const { createHash }=await import("node:crypto");
  assert.equal(createHash("sha256").update(bytes).digest("hex"),small.runs[0].model_sha256);
  assert.deepEqual(selectRuns(small,{subset:"window"}).map(run=>run.id),["native-linear-ex-n0082"]);
  // Bound to the small rack manifest (checked against the generator by tests/test_performance_rack_model.py),
  // which perf.mjs requires of every C1 plan's model, as models_sha256.txt for the 500/1000-node plan.
  const smallLedger=readFileSync(new URL("../../../core/solver/performance_harness/observations/rack/models_sha256_small.txt",import.meta.url),"utf8");
  assert.match(smallLedger,new RegExp(`^${small.runs[0].model_sha256}  linear-ex  82$`,"m"));
  const { requireRackBinding } = await import("./perf.mjs");
  for (const bad of [{ model_sha256: "0".repeat(64) }, { model: "linear-ex-n0250.json" }, { model: "nonlinear-ex-n0082.json" }]) {
    const unbound = JSON.parse(JSON.stringify(small));
    Object.assign(unbound.runs[0], bad);
    assert.throws(() => requireRackBinding(unbound), /not a recorded reference rack/, JSON.stringify(bad));
  }
  requireRackBinding(c1);
  small.runs[0].nodes=250;
  assert.throws(()=>validatePlan(small),/no size escalation/);
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

// A painted solve as the driver reports it: its first_paint mark within the 30 s gate.
const painted = (paintAt = 12_000) => ({ ended: "first_paint", gate: { paint_ms: 30_000, started_at_ms: 1_000, deadline_at_ms: 31_000, first_paint_at_ms: paintAt, first_paint_timely: paintAt <= 31_000 }, timeline: { marks: [{ stage: "solve_requested", at_ms: 1_005 }, { stage: "first_paint", at_ms: paintAt }] }, shown: {} });
const base = (over = {}) => ({
  exit: { status: 0, signal: null, ended_ms: 10_000 },
  timedOut: false,
  progress: [],
  memory: { pids: { 1: { kind: "app", last_ms: 10_000 }, 2: { kind: "webcontent", last_ms: 9_900 } } },
  store: { quick_check: ["ok"], files: [{ name: "openpipestress-projects.sqlite3", bytes: 4096 }], projects: [{ project_id: "p", result_type: "text", result_chars: 1000 }] },
  run: { id: "r", project_id: "p", timeout_s: 60, paint_ms: 30_000 },
  resultBytes: 1000,
  report: {
    solve: painted(),
    record_run: { attempted: true, message: "Recorded run perf r." },
    save_project: { message: "Saved current project model snapshot." }
  },
  ...over
});

test("a painted, recorded and stored run is ok", () => {
  assert.equal(classify(base()).class, "ok");
});

// RT-U4 F1: the classifier applies the gate to the mark's own time, whatever the driver said.
test("F1: a first_paint mark after the gate is a hang; one at the deadline, or noticed late, is not", () => {
  const late = base(); late.report.solve = painted(31_001);
  assert.deepEqual(classify(late).findings, [{ class: "hang", step: "solve", detail: "first paint 30001 ms after Analyze > Run, after the 30000 ms gate" }]);
  const atDeadline = base(); atDeadline.report.solve = { ...painted(31_000), elapsed_ms: 31_180 };
  assert.equal(classify(atDeadline).class, "ok");
});

test("F1: a first_paint without its mark or the driver's start time is not shown to be within the gate", () => {
  for (const solve of [{ ...painted(), timeline: { marks: [] } }, { ...painted(), gate: undefined }, { ...painted(), gate: { started_at_ms: null } }]) {
    const input = base(); input.report.solve = solve;
    assert.deepEqual(classify(input).findings, [{ class: "hang", step: "solve", detail: "first paint not shown to be within 30000 ms of Analyze > Run" }]);
  }
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

test("a C1 run whose page was hidden during the solve is host_unobservable, not a hang, and its store is still checked", () => {
  const run = { c1_probes: true, c1_profile_readers: true, paint_ms: 300000, timeout_s: 720, project_id: "project:invented" };
  const report = { error: "C1 host condition not established (during the solve: the page was visibilityState=hidden at a poll). …",
    solve: { ended: "host_unobservable", timeline: { marks: [{ stage: "result_committed", at_ms: 190000 }] }, gate: { started_at_ms: 455 }, shown: {} },
    c1: { completed: false, host: { established: false, step: "during the solve", detail: "the page was visibilityState=hidden at a poll", solve_started: true } } };
  const base = { report, exit: { status: 0, signal: null, ended_ms: null }, timedOut: false, progress: [], memory: { pids: {} }, run };
  const verdict = classify({ ...base, store: null });
  assert.equal(verdict.class, "host_unobservable");
  assert.deepEqual(verdict.findings.map((finding) => finding.class), ["host_unobservable"]);
  assert.match(verdict.findings[0].detail, /^the page was visibilityState=hidden at a poll; solve started: true\. .*unobservable, not evidence of a product hang; not retried\.$/);
  // A product defect the store shows still outranks it.
  const torn = classify({ ...base, store: { quick_check: ["page 3 is never used"], files: [], projects: [] } });
  assert.equal(torn.class, "half_written_save");
  assert.ok(torn.findings.some((finding) => finding.class === "host_unobservable"));
  // Without the host record, the same timeout is the gate's hang (the classifier's meaning is unchanged).
  const hung = classify({ ...base, store: null, report: { ...report, solve: { ...report.solve, ended: "timeout" }, c1: { completed: false } } });
  assert.equal(hung.class, "hang");
});

test("a refused Record Run or Save Project is quoted", () => {
  const result = classify(base({ report: { solve: painted(), record_run: { attempted: true, message: "Not recorded: RESOURCE_UNAVAILABLE" }, save_project: { message: "Save failed: string or blob too big" } } }));
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

// The commands whose Rust handler returns a raw `tauri::ipc::Response` of JSON bytes (lib.rs), so
// the webview receives `application/octet-stream`, as the App decodes them.
const RAW_JSON_REPLIES = new Set(["finish_local_csv_stream", "record_preview_mechanics_job_analysis_run", "save_preview_mechanics_job_local_project", "open_local_project_in_native_custody", "read_analysis_run_in_native_custody", "save_restored_result_local_project"]);

/** Tauri 2's webview IPC as the driver meets it (tauri-2.11.1 `scripts/core.js` and
 * `scripts/ipc-protocol.js`): `__TAURI_INTERNALS__.invoke` is defined with
 * Object.defineProperty (not writable), and each call is one `fetch` of
 * `ipc://localhost/<command>` whose reply carries `Tauri-Response` and is read by its content
 * type. `handler(command, args, options)` answers as the Rust command would. */
function tauriIpc(window, handler) {
  window.fetch = async (url, init) => {
    const command = decodeURIComponent(String(url).slice("ipc://localhost/".length));
    const body = init.body;
    const args = typeof body === "string" ? JSON.parse(body) : body;
    const headers = Object.fromEntries([...new Headers(init.headers)].filter(([name]) => !name.startsWith("tauri-") && name !== "content-type"));
    try {
      const value = await handler(command, args, { headers });
      if (value instanceof ArrayBuffer || ArrayBuffer.isView(value)) return new Response(value, { headers: { "Tauri-Response": "ok", "Content-Type": "application/octet-stream" } });
      if (RAW_JSON_REPLIES.has(command)) return new Response(new TextEncoder().encode(JSON.stringify(value)), { headers: { "Tauri-Response": "ok", "Content-Type": "application/octet-stream" } });
      return new Response(JSON.stringify(value ?? null), { headers: { "Tauri-Response": "ok", "Content-Type": "application/json" } });
    } catch (error) {
      return new Response(JSON.stringify(String(error?.message ?? error)), { headers: { "Tauri-Response": "error", "Content-Type": "application/json" } });
    }
  };
  const internals = {};
  Object.defineProperty(internals, "invoke", {
    value: async (command, payload = {}, options) => {
      const bytes = payload instanceof ArrayBuffer || ArrayBuffer.isView(payload);
      const headers = new Headers(options?.headers ?? {});
      headers.set("Content-Type", bytes ? "application/octet-stream" : "application/json");
      headers.set("Tauri-Callback", "1");
      headers.set("Tauri-Error", "2");
      const response = await window.fetch(`ipc://localhost/${encodeURIComponent(command)}`, { method: "POST", body: bytes ? payload : JSON.stringify(payload), headers });
      const type = (response.headers.get("content-type") || "").split(",")[0];
      const data = type === "application/json" ? await response.json() : type === "text/plain" ? await response.text() : await response.arrayBuffer();
      if (response.headers.get("Tauri-Response") !== "ok") throw data;
      return data;
    }
  });
  window.__TAURI_INTERNALS__ = internals;
  return internals;
}

async function driverDom(t, html, invoke, overrides = {}) {
  const { JSDOM } = await import("jsdom");
  const vm = await import("node:vm");
  const dom = new JSDOM(html);
  t.after(() => dom.window.close());
  // A shown page (jsdom's own default is "prerender"); a test of another state sets its own.
  setVisibility(dom.window.document, "visible");
  const hooks = {};
  tauriIpc(dom.window, invoke);
  const context = { window: dom.window, document: dom.window.document, navigator: dom.window.navigator, Response, Headers,
    // Hex escapes are literal in jsdom as well as WebKit, including encoded IDs.
    CSS: { escape: value => String(value).replace(/[^a-zA-Z0-9_-]/gu, char => `\\${char.codePointAt(0).toString(16)} `) }, performance,
    Event: dom.window.Event, HTMLInputElement: dom.window.HTMLInputElement, HTMLTextAreaElement: dom.window.HTMLTextAreaElement,
    setTimeout, clearTimeout, setInterval, clearInterval, requestAnimationFrame: callback => setTimeout(() => callback(performance.now()), 0),
    TextEncoder, TextDecoder, ArrayBuffer, Uint8Array, DataView,
    __SWBPIPE_PERF_DRIVER_TEST__: hooks, ...overrides };
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
    if(command==="save_preview_mechanics_job_local_project") return {result:{outcome:"not_saved",code:"PROJECT-SAVE-RESULT-TOO-LARGE",rows:[{id:"row-payload"}]}};
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

// ------------------------------------------------- F1: Tauri 2.11.1's actual IPC and its fallback
// The fixture is tauri 2.11.1's own `scripts/` (test-fixtures/tauri-2.11.1/README.md).
const TAURI_FIXTURE = new URL("./test-fixtures/tauri-2.11.1/", import.meta.url);
const TAURI_SCRIPTS = {
  "core.js": "71056442a732cca1f03572ccae96d68e16104c3a591c64de9618cd304e063070",
  "ipc-protocol.js": "68ae690606006f733bde75dd8bb6748464954a22c7e83914a93b1abc221d7072",
  "ipc.js": "3e3766fc90efa68003096cbf0a07c6a00cb31c5756d55b5a68ebbfd2930c4865",
  "pattern.js": "149570459687f1636c02e41ede8b40efe52378965eeddb6dbfebc0c525a3551b",
  "process-ipc-message-fn.js": "d51917f89e4d3d318d63f1f4f5a7ac7ea1399a5b85c81a840c4d606597f29132"
};
// Values from the page's realm, compared by value.
const plain = (value) => JSON.parse(JSON.stringify(value));
const tauriScript = (name) => readFileSync(new URL(`scripts/${name}`, TAURI_FIXTURE), "utf8");
// wry 0.55.1 `src/wkwebview/mod.rs:638–640`, verbatim: the fallback transport, defined frozen.
const WRY_IPC = `Object.defineProperty(window, 'ipc', {
  value: Object.freeze({postMessage: function(s) {window.webkit.messageHandlers.ipc.postMessage(s);}})
});`;
const POOL = () => ({ headroom: { pressure: "normal", available_bytes: 30 * 1024 ** 3 }, invocation_budget_bytes: 12 * 1024 ** 3, retained_results: 0, running_workers: 0, queued_jobs: 0, running_jobs: 0, webview_custody_results: 0, reserved_bytes: 0 });

test("the Tauri IPC fixture is tauri 2.11.1's own scripts, the version the App's lockfile resolves", async () => {
  const { createHash } = await import("node:crypto");
  const { readdirSync } = await import("node:fs");
  const lock = readFileSync(new URL("../src-tauri/Cargo.lock", import.meta.url), "utf8");
  assert.match(lock, /\nname = "tauri"\nversion = "2\.11\.1"\nsource = "registry\+https:\/\/github\.com\/rust-lang\/crates\.io-index"\nchecksum = "b93bd86d231f0a8138f11a02a584769fe4b703dc36ae133d783228dbc4801405"\n/);
  assert.match(lock, /\nname = "wry"\nversion = "0\.55\.1"\n/);
  for (const [name, digest] of Object.entries(TAURI_SCRIPTS)) assert.equal(createHash("sha256").update(readFileSync(new URL(`scripts/${name}`, TAURI_FIXTURE))).digest("hex"), digest, name);
  // Where the cargo registry holds the crates' source (a host that built the App), the copies are its bytes.
  const registry = path.join(process.env.CARGO_HOME ?? path.join(os.homedir(), ".cargo"), "registry", "src");
  for (const index of existsSync(registry) ? readdirSync(registry) : []) {
    const scripts = path.join(registry, index, "tauri-2.11.1", "scripts");
    if (existsSync(scripts)) for (const name of Object.keys(TAURI_SCRIPTS)) assert.equal(readFileSync(path.join(scripts, name), "utf8"), tauriScript(name), name);
    const wry = path.join(registry, index, "wry-0.55.1", "src", "wkwebview", "mod.rs");
    if (existsSync(wry)) assert.ok(readFileSync(wry, "utf8").includes(WRY_IPC), "wry 0.55.1 defines window.ipc as quoted");
  }
});

/** The App's page as tauri 2.11.1 and wry 0.55.1 set it up on macOS, with the fixture's actual
 * scripts in their order (tauri `src/manager/webview.rs:166–182`, `scripts/init.js`): wry's frozen
 * `window.ipc`, the internals object, `ipc-protocol.js`, `pattern.js` (brownfield), `ipc.js`,
 * `core.js`. `native(command, args)` answers as the Rust command would over both transports: the
 * `ipc://` custom protocol (`fetch`), and WebKit's message handler (postMessage), answered as Rust
 * answers it (`runCallback(id, value)`, `src/ipc/format_callback.rs:103`). `fault(command)` makes a
 * command's fetch fail: "reject" (the request fails) or "body" (its reply body errors when read).
 * The WebKit handler's `postMessage` is a prototype method (a WebIDL operation), replaceable unless
 * `handlerReplaceable` is false. Without `run`, the driver hands its steps to the test. */
async function tauriPage(t, native, { fault = () => null, handlerReplaceable = true, run = false, html = "" } = {}) {
  const { JSDOM } = await import("jsdom");
  const dom = new JSDOM(`<!doctype html><body>${html}</body>`, { runScripts: "outside-only" });
  t.after(() => dom.window.close());
  const w = dom.window;
  setVisibility(w.document, "visible");
  Object.assign(w, { Headers, Response, ReadableStream, TextEncoder, TextDecoder, CSS: { escape: (value) => String(value) } });
  const sent = { fetch: [], post: [], native: [], warnings: [] };
  w.console.warn = (...args) => { sent.warnings.push(String(args[0])); };
  const answer = async (command, args) => { sent.native.push(command); return native(command, args); };
  w.fetch = async (url, init) => {
    const command = decodeURIComponent(String(url).slice("ipc://localhost/".length));
    const kind = fault(command);
    if (kind === "reject") throw new TypeError("Load failed");
    sent.fetch.push(command);
    let value;
    try { value = await answer(command, typeof init.body === "string" ? JSON.parse(init.body) : init.body); }
    catch (error) { return new Response(JSON.stringify(String(error?.message ?? error)), { headers: { "Tauri-Response": "error", "Content-Type": "application/json" } }); }
    if (kind === "body") return new Response(new ReadableStream({ pull(controller) { controller.error(new TypeError("the reply body failed")); } }), { headers: { "Tauri-Response": "ok", "Content-Type": "application/json" } });
    return new Response(JSON.stringify(value ?? null), { headers: { "Tauri-Response": "ok", "Content-Type": "application/json" } });
  };
  class UserMessageHandler {
    postMessage(message) {
      const request = JSON.parse(message);
      sent.post.push(request.cmd);
      setTimeout(async () => {
        try { w.__TAURI_INTERNALS__.runCallback(request.callback, await answer(request.cmd, request.payload)); }
        catch (error) { w.__TAURI_INTERNALS__.runCallback(request.error, String(error?.message ?? error)); }
      }, 0);
    }
  }
  if (!handlerReplaceable) Object.defineProperty(UserMessageHandler.prototype, "postMessage", { writable: false, configurable: false });
  w.webkit = { messageHandlers: { ipc: new UserMessageHandler() } };
  const template = (source, values) => source.replace(/__(TEMPLATE|RAW)_([a-z_]+)__/gu, (_, kind, name) => {
    assert.ok(name in values, name);
    return kind === "RAW" ? values[name] : JSON.stringify(values[name]);
  });
  w.eval(WRY_IPC);
  w.eval(`Object.defineProperty(window, 'isTauri', { value: true });
    if (!window.__TAURI_INTERNALS__) { Object.defineProperty(window, '__TAURI_INTERNALS__', { value: { plugins: {} } }) }`);
  w.eval(template(tauriScript("ipc-protocol.js"), { invoke_key: "invented-invoke-key", process_ipc_message_fn: tauriScript("process-ipc-message-fn.js"), os_name: "macos", fetch_channel_data_command: "plugin:__TAURI_CHANNEL__|fetch" }));
  w.eval(template(tauriScript("pattern.js"), { pattern: { pattern: "brownfield" } }));
  w.eval(template(tauriScript("ipc.js"), { isolation_origin: "" }));
  w.eval(template(tauriScript("core.js"), { os_name: "macos", protocol_scheme: "https" }));
  assert.equal(Object.getOwnPropertyDescriptor(w.__TAURI_INTERNALS__, "invoke").writable, false);
  const hooks = {};
  if (!run) w.__SWBPIPE_PERF_DRIVER_TEST__ = hooks;
  w.eval(readFileSync(new URL("./perf-driver.js", import.meta.url), "utf8"));
  return { w, hooks, sent, invoke: (command, args) => w.__TAURI_INTERNALS__.invoke(command, args) };
}
const poolNative = async (command) => (command === "get_memory_pool_status" ? POOL() : command === "lookup_preview_mechanics_job_rows" ? { rows: [] } : command === "take_preview_mechanics_job_result" ? { whole: "result" } : null);
const LOST = "C1 command observation lost Tauri's IPC";
// An error thrown in the page's realm, matched on its message.
const messageMatches = (pattern) => (error) => { assert.match(String(error?.message), pattern); return true; };

test("F1: on Tauri's actual IPC the preflight's pool status is one observed fetch round trip", async (t) => {
  const page = await tauriPage(t, poolNative);
  const trace = page.hooks.beginC1Trace();
  await page.hooks.preflight({});
  page.hooks.requireObservedIpc(trace);
  assert.deepEqual(plain(trace.data.completed), { get_memory_pool_status: 1 });
  assert.deepEqual(page.sent.post, []);
  assert.equal(trace.data.transport.lost, false);
  assert.equal(trace.data.transport.fallback_observer, "webkit.messageHandlers.ipc");
  trace.restore();
});

test("F1: a preflight fetch that fails falls back to postMessage and is no observed round trip", async (t) => {
  const page = await tauriPage(t, poolNative, { fault: (command) => (command === "get_memory_pool_status" ? "reject" : null) });
  const trace = page.hooks.beginC1Trace();
  const report = {};
  await page.hooks.preflight(report); // Tauri's fallback answers it
  assert.equal(report.preflight.retained_results, 0);
  assert.throws(() => page.hooks.requireObservedIpc(trace), messageMatches(new RegExp(`^${LOST}: an IPC fetch was rejected \\(get_memory_pool_status\\)\\. Tauri now sends every command over postMessage`)));
  assert.deepEqual(page.sent.post, ["get_memory_pool_status"]);
  assert.match(page.sent.warnings[0], /^IPC custom protocol failed, Tauri will now use the postMessage interface instead/);
  assert.deepEqual(plain(trace.data.transport.events.map(({ reason, command }) => [reason, command])), [
    ["an IPC fetch was rejected", "get_memory_pool_status"],
    ["Tauri reported its IPC custom protocol failed and fell back to postMessage", null],
    ["an IPC command was sent over postMessage", "get_memory_pool_status"]]);
  assert.equal(trace.data.completed.get_memory_pool_status, undefined);
  trace.restore();
});

test("F1: only Tauri's exact fallback warning marks the transport lost; an unrelated warning that begins alike does not", async (t) => {
  const page = await tauriPage(t, poolNative);
  const trace = page.hooks.beginC1Trace();
  page.w.console.warn("IPC custom protocol failed to load a font", new Error("x"));
  page.w.console.warn("IPC custom protocol failed, Tauri will now use the postMessage interface instead!", new Error("x"));
  assert.equal(trace.data.transport.lost, false);
  await page.hooks.preflight({});
  page.hooks.requireObservedIpc(trace);
  page.w.console.warn("IPC custom protocol failed, Tauri will now use the postMessage interface instead", new Error("x"));
  assert.equal(trace.data.transport.lost, true);
  assert.equal(page.sent.warnings.length, 3);
  trace.restore();
});

test("F1: a preflight reply whose body cannot be read falls back to postMessage and is no observed round trip", async (t) => {
  const page = await tauriPage(t, poolNative, { fault: (command) => (command === "get_memory_pool_status" ? "body" : null) });
  const trace = page.hooks.beginC1Trace();
  await page.hooks.preflight({});
  assert.throws(() => page.hooks.requireObservedIpc(trace), messageMatches(new RegExp(`^${LOST}: the App's read of an IPC reply failed \\(get_memory_pool_status\\)`)));
  // Tauri sends the command again over postMessage: the native side ran it twice.
  assert.deepEqual(page.sent.native, ["get_memory_pool_status", "get_memory_pool_status"]);
  assert.deepEqual(page.sent.post, ["get_memory_pool_status"]);
  assert.equal(trace.data.completed.get_memory_pool_status, undefined);
  trace.restore();
});

test("F1: the whole driver on a preflight fallback stops before any page check or solve", async (t) => {
  let finished;
  const done = new Promise((resolve) => { finished = resolve; });
  const sentMenus = [];
  let report = null;
  const page = await tauriPage(t, async (command, args) => {
    if (command === "native_smoke_plan") return { run_id: "c1-fallback", c1_probes: true, c1_reader_diagnostics: true, model_path: "/invented/rack.json", project_id: "project:invented", model_jcs_sha256: "sha256:invented", timeouts: { start_ms: 1000, open_ms: 1000, paint_ms: 1000, interaction_ms: 1000, csv_ms: 1000, record_ms: 1000, save_ms: 1000 } };
    if (command === "native_smoke_menu") sentMenus.push(args.commandId);
    if (command === "native_smoke_finish") { report = args.report; finished(); }
    return poolNative(command);
  }, { run: true, html: HIDDEN_SHELL, fault: (command) => (command === "get_memory_pool_status" ? "reject" : null) });
  await Promise.race([done, new Promise((_, reject) => setTimeout(() => reject(new Error("driver did not finish")), 5000))]);
  assert.match(report.error, new RegExp(`^${LOST}: an IPC fetch was rejected \\(get_memory_pool_status\\)`));
  assert.deepEqual(sentMenus, []);
  assert.equal(report.c1.page, undefined);
  assert.equal(report.c1.trace.transport.lost, true);
  // The report itself still arrives, over the fallback.
  assert.ok(page.sent.post.includes("native_smoke_finish"));
});

test("F1: a fallback later in the run is recorded, stops the next wait and fails the C1 evidence", async (t) => {
  let failLookup = false;
  const page = await tauriPage(t, poolNative, { fault: (command) => (failLookup && command === "lookup_preview_mechanics_job_rows" ? "reject" : null) });
  const trace = page.hooks.beginC1Trace();
  await page.hooks.preflight({});
  page.hooks.requireObservedIpc(trace);
  failLookup = true;
  assert.deepEqual(plain(await page.invoke("lookup_preview_mechanics_job_rows", {})), { rows: [] }); // answered over postMessage
  const started = Date.now();
  await assert.rejects(page.hooks.waitFor("the next step", () => null, 1000, 10), messageMatches(new RegExp(`^${LOST}: an IPC fetch was rejected \\(lookup_preview_mechanics_job_rows\\)`)));
  assert.ok(Date.now() - started < 500);
  // Everything after it goes over postMessage, seen only by the fallback observer.
  const fetched = page.sent.fetch.length;
  await page.invoke("get_memory_pool_status", {});
  assert.equal(page.sent.fetch.length, fetched);
  assert.deepEqual(plain(trace.data.transport.fallback_counts), { lookup_preview_mechanics_job_rows: 1, get_memory_pool_status: 1 });
  const verdict = classify({ report: { solve: { ended: "first_paint", timeline: { marks: [{ stage: "first_paint", at_ms: 10 }] }, gate: { started_at_ms: 0 } }, c1: { completed: false, trace: trace.data, host: { established: true } } }, exit: { status: 0, signal: null, ended_ms: null }, timedOut: false, progress: [], memory: { pids: {} }, store: null, run: { c1_probes: true, paint_ms: 1000, timeout_s: 10, project_id: "project:invented" } });
  assert.ok(verdict.findings.some((finding) => finding.step === "c1_observation" && /lost Tauri's IPC transport: an IPC fetch was rejected/.test(finding.message)), JSON.stringify(verdict.findings));
  trace.restore();
});

test("F1: a forbidden command sent over the fallback is recorded and refused through Tauri's error callback", async (t) => {
  let failLookup = false;
  const page = await tauriPage(t, poolNative, { fault: (command) => (failLookup && command === "lookup_preview_mechanics_job_rows" ? "reject" : null) });
  const trace = page.hooks.beginC1Trace();
  await page.hooks.preflight({});
  failLookup = true;
  await page.invoke("lookup_preview_mechanics_job_rows", {});
  await assert.rejects(page.invoke("take_preview_mechanics_job_result", { jobId: "invented" }), /C1 probe refused unexpected whole-value\/budget command: take_preview_mechanics_job_result/);
  assert.deepEqual(plain(trace.data.violations), ["take_preview_mechanics_job_result"]);
  assert.equal(page.sent.native.includes("take_preview_mechanics_job_result"), false);
  assert.equal(page.sent.post.includes("take_preview_mechanics_job_result"), false);
  assert.equal(trace.data.transport.fallback_counts.take_preview_mechanics_job_result, 1);
  trace.restore();
});

test("F1: where the fallback cannot be observed, a switch to it still stops the run and fails the evidence", async (t) => {
  let failLookup = false;
  const page = await tauriPage(t, poolNative, { handlerReplaceable: false, fault: (command) => (failLookup && command === "lookup_preview_mechanics_job_rows" ? "reject" : null) });
  const trace = page.hooks.beginC1Trace();
  assert.equal(trace.data.transport.fallback_observer, "unavailable");
  await page.hooks.preflight({});
  page.hooks.requireObservedIpc(trace);
  failLookup = true;
  await page.invoke("lookup_preview_mechanics_job_rows", {});
  // Unobservable after the switch: the forbidden command reaches the native side, unrefused.
  assert.deepEqual(plain(await page.invoke("take_preview_mechanics_job_result", {})), { whole: "result" });
  assert.equal(trace.data.transport.lost, true);
  await assert.rejects(page.hooks.waitFor("the next step", () => null, 1000, 10), messageMatches(new RegExp(`^${LOST}: an IPC fetch was rejected`)));
  trace.restore();
});

// --------------------------------------------- F2: bounded reads of receipts, refusals and headers
/** A reply whose body is `total` bytes in `chunk`-byte chunks, made on demand: `head`, spaces, then
 * `tail`. `state.pulled` counts the bytes the stream has been asked for. */
function countedReply({ total, chunk, head = "", tail = "", type, ok = true }) {
  const state = { pulled: 0 };
  const headBytes = new TextEncoder().encode(head), tailBytes = new TextEncoder().encode(tail);
  const stream = new ReadableStream({ pull(controller) {
    if (state.pulled >= total) { controller.close(); return; }
    const n = Math.min(chunk, total - state.pulled);
    const bytes = new Uint8Array(n).fill(0x20);
    for (let i = 0; i < n; i++) {
      const at = state.pulled + i;
      if (at < headBytes.length) bytes[i] = headBytes[at];
      else if (at >= total - tailBytes.length) bytes[i] = tailBytes[at - (total - tailBytes.length)];
    }
    state.pulled += n;
    controller.enqueue(bytes);
  } }, { highWaterMark: 0 });
  return { state, response: new Response(stream, { headers: { "Tauri-Response": ok ? "ok" : "error", "Content-Type": type } }) };
}
/** The trace over one reply, fetched as the App's Tauri fetches it (the App's own read is not made). */
async function observeOne(t, command, reply, body = "{}") {
  const ui = await driverDom(t, "", async () => null);
  ui.dom.window.fetch = async () => reply.response;
  const trace = ui.hooks.beginC1Trace();
  const response = await ui.dom.window.fetch(`ipc://localhost/${command}`, { method: "POST", body });
  assert.equal(response, reply.response, "the App receives Tauri's reply itself");
  trace.restore();
  return trace.data;
}
const RECEIPT_BOUND = 4 << 20;

test("F2: a JSON receipt over 4 MiB is refused by the observation, and the App still reads its reply", async (t) => {
  const ui = await driverDom(t, "", async (command) => (command === "dispose_preview_mechanics_result" ? { released: true, job_id: "job-1", pad: " ".repeat(RECEIPT_BOUND) } : null));
  const trace = ui.hooks.beginC1Trace();
  const value = await ui.call("dispose_preview_mechanics_result", {});
  assert.equal(value.released, true);
  assert.deepEqual(plain(trace.data.disposed), {});
  assert.match(trace.data.errors[0], /^probe reply exceeds its 4194304-byte bound$/);
  trace.restore();
});

test("F2: a raw receipt over 4 MiB is cancelled at the bound, not read to its end", async (t) => {
  const chunk = 64 << 10;
  const reply = countedReply({ total: 32 << 20, chunk, head: '{"message":"Recorded run","run":{"recording_id":"r"}', tail: "}", type: "application/octet-stream" });
  const data = await observeOne(t, "record_preview_mechanics_job_analysis_run", reply);
  assert.ok(reply.state.pulled <= RECEIPT_BOUND + 4 * chunk, `pulled ${reply.state.pulled} bytes`);
  assert.equal(data.record, null);
  assert.match(data.errors[0], /exceeds its 4194304-byte bound/);
  assert.ok(data.reply_bytes_max <= RECEIPT_BOUND);
});

test("F2: an oversized error reply is read only to its 2 KiB prefix", async (t) => {
  const chunk = 64 << 10;
  const reply = countedReply({ total: 32 << 20, chunk, head: '"refused: ', tail: '"', type: "application/json", ok: false });
  const data = await observeOne(t, "query_preview_mechanics_job_rows", reply);
  assert.ok(reply.state.pulled <= 2048 + 4 * chunk, `pulled ${reply.state.pulled} bytes`);
  assert.equal(data.refusals.length, 1);
  assert.equal(data.refusals[0].truncated, true);
  assert.ok(data.refusals[0].message.startsWith('"refused: ') && data.refusals[0].message.length === 512);
  assert.equal(data.reply_bytes_max, 2048);
});

test("F2: one oversized chunk: a receipt keeps nothing, a row header copies only the header", async (t) => {
  // A JSON receipt delivered as one chunk over the bound: refused before any of it is copied.
  const single = countedReply({ total: RECEIPT_BOUND + 1, chunk: RECEIPT_BOUND + 1, head: '{"released":true,"job_id":"job-1"', tail: "}", type: "application/json" });
  const data = await observeOne(t, "dispose_preview_mechanics_result", single);
  assert.deepEqual(plain(data.disposed), {});
  assert.match(data.errors[0], /exceeds its 4194304-byte bound/);
  assert.equal(data.reply_bytes_max, 0);
  // A row reply whose header and 8 MiB of rows arrive as one chunk: only the header's bytes are copied.
  const header = new TextEncoder().encode(JSON.stringify({ document_kind: "swbpipe.desktop.result_detail_rows", version: 1, total: 27, offset: 0, returned: 0, ordinals: [], lengths: [], truncated: false }));
  const rows = new Uint8Array(16 + header.length + (8 << 20));
  rows.set(new TextEncoder().encode("SWBDETv1"));
  new DataView(rows.buffer).setBigUint64(8, BigInt(header.length), true);
  rows.set(header, 16);
  const ui = await driverDom(t, "", async () => null);
  ui.dom.window.fetch = async () => new Response(new ReadableStream({ start(controller) { controller.enqueue(rows); controller.close(); } }), { headers: { "Tauri-Response": "ok", "Content-Type": "application/octet-stream" } });
  const trace = ui.hooks.beginC1Trace();
  trace.data.csv = { family: "force", pages: 0, expected_rows: null, receipt: null };
  await ui.dom.window.fetch("ipc://localhost/query_preview_mechanics_job_rows", { method: "POST", body: JSON.stringify({ filter: { family: "force" } }) });
  assert.equal(trace.data.csv.expected_rows, 27);
  assert.deepEqual(plain(trace.data.errors), []);
  assert.equal(trace.data.reply_bytes_max, 16 + header.length);
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

// A C1 solve's first paint is its Results table's first page (solveTimeline.ts), so the driver
// opens Results once the run has begun (Analyze > Run itself shows Solve) and ends the wait on a
// refused first page instead of running out the gate.
async function c1SolveDom(t, { refuse = false } = {}) {
  const menus=[];
  let shown="solve", timeline=null;
  const ui=await driverDom(t,`<section data-testid="workspace-section-solve"><button data-testid="solver-mode-sparse" aria-pressed="false"></button><p data-testid="solve-job-summary">state=running</p></section><section data-testid="workspace-section-results"></section>`,async (command,args)=>{
    if(command!=="native_smoke_menu") return null;
    menus.push({command:args.commandId,sequence:timeline?.sequence??0});
    if(args.commandId==="analyze.run") { shown="solve"; setTimeout(()=>{timeline={sequence:1,outcome:"running",marks:[{stage:"solve_requested"}]};},20); }
    else shown=args.commandId.replace("view.section.","");
    return null;
  });
  const doc=ui.dom.window.document;
  for(const id of ["solve","results"]) Object.defineProperty(doc.querySelector(`[data-testid="workspace-section-${id}"]`),"offsetParent",{get:()=>shown===id?doc.body:null});
  const sparse=doc.querySelector('[data-testid="solver-mode-sparse"]');
  sparse.onclick=()=>sparse.setAttribute("aria-pressed","true");
  ui.context.__openPipeStressSolveTimelineV1={readCurrent:()=>timeline};
  const results=doc.querySelector('[data-testid="workspace-section-results"]');
  const commit=()=>{
    timeline={...timeline,outcome:"completed",marks:[...timeline.marks,{stage:"result_set"},{stage:"result_committed"}]};
    if(refuse) results.innerHTML='<p data-testid="result-query-refused">The native result query was refused: RESULT-DETAIL-REFUSED</p>';
    // A mark carries its own time (solveTimeline.ts); the driver judges the gate by it (F1).
    else setTimeout(()=>{timeline={...timeline,marks:[...timeline.marks,{stage:"first_paint",at_ms:performance.now()}]};},50);
  };
  return {ui,menus,commit,shown:()=>shown};
}

for (const refuse of [false,true]) {
  test(`the C1 solve wait opens Results after the run begins and ends at ${refuse?"a refused first page":"first_paint"}`, async (t) => {
    const dom=await c1SolveDom(t,{refuse});
    const report={};
    setTimeout(()=>dom.commit(),400);
    await dom.ui.hooks.solve({c1_probes:true,timeouts:{open_ms:1000,paint_ms:5000}},report);
    assert.deepEqual(dom.menus,[{command:"analyze.run",sequence:0},{command:"view.section.results",sequence:1}]);
    assert.equal(dom.shown(),"results");
    assert.equal(report.solve.ended,refuse?"results_refused":"first_paint");
    assert.equal(report.solve.shown.results_refusal,refuse?"The native result query was refused: RESULT-DETAIL-REFUSED":null);
    assert.ok(report.solve.elapsed_ms<5000);
  });
}

test("the C1 solve's paint gate runs from Analyze > Run, including opening Results", async (t) => {
  const dom=await c1SolveDom(t);
  const report={};
  // Never committed: the gate ends at paint_ms from the run, not paint_ms after Results opened.
  await dom.ui.hooks.solve({c1_probes:true,timeouts:{open_ms:1000,paint_ms:600}},report);
  assert.equal(report.solve.ended,"timeout");
  assert.ok(report.solve.elapsed_ms<900,`elapsed ${report.solve.elapsed_ms}`);
});

// RT-U4 F1: a simulated clock for the driver's own clocks (`performance.now()`, `Date.now()`,
// `setTimeout`). Timers run in time order; between them the real event loop drains the
// promise work (IPC replies, jsdom). Nothing waits in real time.
function fakeClock() {
  let clock = 0, seq = 0;
  const timers = [];
  const add = (at, fn) => { const id = ++seq; timers.push({ at, id, fn }); return id; };
  return {
    now: () => clock,
    context: {
      performance: { now: () => clock, timeOrigin: 0 },
      Date: { now: () => clock },
      setTimeout: (fn, ms = 0, ...args) => add(clock + Math.max(0, Number(ms) || 0), () => fn(...args)),
      clearTimeout: id => { const at = timers.findIndex(timer => timer.id === id); if (at >= 0) timers.splice(at, 1); }
    },
    at: (ms, fn) => add(ms, fn),
    async run(promise, limitMs = 2_000_000) {
      let done = false, error = null;
      promise.then(() => { done = true; }, failure => { done = true; error = failure; });
      for (;;) {
        await new Promise(resolve => setImmediate(resolve));
        if (done) break;
        timers.sort((a, b) => a.at - b.at || a.id - b.id);
        const next = timers.shift();
        if (!next || next.at > limitMs) throw new Error(`simulated clock stalled at ${clock} ms`);
        clock = Math.max(clock, next.at);
        next.fn();
      }
      if (error) throw error;
    }
  };
}

// The solve step on a simulated clock. `begunAt` starts the run's timeline, `resultsAt` is when
// Results shows after the driver asks for it, `paintAt` when the first_paint mark is written (its
// own `at_ms`). The run is sent at clock 0, so the gate's deadline is `paint_ms`.
async function clockedSolve(t, { c1, paintMs = 300_000, openMs = 600_000, begunAt = 20, resultsAt = null, paintAt }) {
  const clock = fakeClock();
  let shown = "solve", timeline = null;
  const ui = await driverDom(t, `<section data-testid="workspace-section-solve"><button data-testid="solver-mode-sparse" aria-pressed="false"></button><p data-testid="solve-job-summary">state=running</p></section><section data-testid="workspace-section-results"></section>`, async (command, args) => {
    if (command !== "native_smoke_menu") return null;
    if (args.commandId === "analyze.run") clock.at(begunAt, () => { timeline = { sequence: 1, outcome: "running", marks: [{ stage: "solve_requested", at_ms: clock.now() }] }; });
    else if (args.commandId === "view.section.results") clock.at(resultsAt ?? clock.now(), () => { shown = "results"; });
    return null;
  }, clock.context);
  const doc = ui.dom.window.document;
  for (const id of ["solve", "results"]) Object.defineProperty(doc.querySelector(`[data-testid="workspace-section-${id}"]`), "offsetParent", { get: () => shown === id ? doc.body : null });
  const sparse = doc.querySelector('[data-testid="solver-mode-sparse"]');
  sparse.onclick = () => sparse.setAttribute("aria-pressed", "true");
  ui.context.__openPipeStressSolveTimelineV1 = { readCurrent: () => timeline };
  clock.at(paintAt, () => { timeline = { ...timeline, outcome: "completed", marks: [...timeline.marks, { stage: "result_committed", at_ms: clock.now() }, { stage: "first_paint", at_ms: clock.now() }] }; });
  const report = {};
  await clock.run(ui.hooks.solve({ c1_probes: c1, timeouts: { open_ms: openMs, paint_ms: paintMs } }, report));
  return { report, observedAt: clock.now() };
}

// Each late case is one the driver at e1079a816 reported as `first_paint`.
test("F1: a C1 Results activation that runs past the paint gate is the gate's timeout (the review's 301 s paint)", async (t) => {
  // Results shows only at 300.95 s and the page paints at 300.96 s; `open_ms` alone would allow it.
  const { report } = await clockedSolve(t, { c1: true, resultsAt: 300_950, paintAt: 300_960 });
  assert.equal(report.solve.ended, "timeout");
  assert.equal(report.solve.gate.deadline_at_ms, 300_000);
  assert.equal(report.solve.gate.first_paint_at_ms, null);
});

test("F1: a late first_paint mark is rejected even when the wait notices it", async (t) => {
  // The run begins 50 ms before the deadline, so opening Results overruns it by one poll and the
  // paint wait then sees a mark written 20 ms after the deadline.
  const { report, observedAt } = await clockedSolve(t, { c1: true, begunAt: 299_950, resultsAt: 300_000, paintAt: 300_020 });
  assert.ok(observedAt > 300_020, `observed at ${observedAt}`);
  assert.equal(report.solve.ended, "timeout");
  assert.equal(report.solve.gate.first_paint_at_ms, 300_020);
  assert.equal(report.solve.gate.first_paint_timely, false);
});

test("F1: a timely first_paint mark that the wait notices after the deadline still passes", async (t) => {
  const { report, observedAt } = await clockedSolve(t, { c1: true, begunAt: 299_950, resultsAt: 299_980, paintAt: 299_990 });
  assert.ok(observedAt > 300_000, `observed at ${observedAt}`);
  assert.equal(report.solve.ended, "first_paint");
  assert.deepEqual({ ...report.solve.gate }, { paint_ms: 300_000, started_at_ms: 0, deadline_at_ms: 300_000, first_paint_at_ms: 299_990, first_paint_timely: true });
});

test("F1: without C1, a mark one poll past the gate is a timeout and one just within it passes", async (t) => {
  const late = await clockedSolve(t, { c1: false, paintAt: 300_100 });
  assert.equal(late.report.solve.ended, "timeout");
  const timely = await clockedSolve(t, { c1: false, paintAt: 299_999 });
  assert.equal(timely.report.solve.ended, "first_paint");
  assert.equal(timely.report.solve.gate.first_paint_at_ms, 299_999);
});

// RT-U4 A+B diagnosis (`c1-slice7-run-proposal.md`): the attempt's page was not shown (WebKit
// aligned the driver's timers to 1 s, its hidden-page throttling), and a page that is not shown
// runs no animation frame, so its first paint could not be observed. The driver now refuses to
// start a solve on such a page, and journals what the page showed during the solve.
const HIDDEN_SHELL = `<div data-testid="desktop-preview-shell"></div><p data-testid="local-project-message">Initial</p>`;
function setVisibility(doc, state) { Object.defineProperty(doc, "visibilityState", { configurable: true, get: () => state }); }

/** The whole driver as the App runs it (no test hooks): a C1 plan, the pool preflight, then
 * the page checks. Returns the menus sent, the files chosen and the finished report. */
async function wholeDriver(t, { visibility, frames, observeIpc = true, hideOnOpen = false }) {
  const { JSDOM } = await import("jsdom");
  const vm = await import("node:vm");
  const dom = new JSDOM(HIDDEN_SHELL);
  t.after(() => dom.window.close());
  let state = visibility;
  Object.defineProperty(dom.window.document, "visibilityState", { configurable: true, get: () => state });
  // The window covered (or the display asleep) once the model is being opened.
  const hide = () => setTimeout(() => { state = "hidden"; dom.window.document.dispatchEvent(new dom.window.Event("visibilitychange")); }, 50);
  const pool = { headroom: { pressure: "normal", available_bytes: 30 * 1024 ** 3 }, invocation_budget_bytes: 12 * 1024 ** 3, retained_results: 0, running_workers: 0, queued_jobs: 0, running_jobs: 0, webview_custody_results: 0, reserved_bytes: 0 };
  const sent = { menus: [], chosen: [], report: null };
  let finished;
  const done = new Promise((resolve) => { finished = resolve; });
  const handler = async (command, args) => {
    if (command === "native_smoke_plan") return { run_id: "c1-hidden", c1_probes: true, c1_reader_diagnostics: true, model_path: "/invented/rack.json", project_id: "project:invented", model_jcs_sha256: "sha256:invented", timeouts: { start_ms: 1000, open_ms: 1000, paint_ms: 1000, interaction_ms: 1000, csv_ms: 1000, record_ms: 1000, save_ms: 1000 } };
    if (command === "get_memory_pool_status") return pool;
    if (command === "native_smoke_menu") { sent.menus.push(args.commandId); if (hideOnOpen && args.commandId === "file.open-model") hide(); return null; }
    if (command === "native_smoke_choose") { sent.chosen.push(args.path); return null; }
    if (command === "native_smoke_finish") { sent.report = args.report; finished(); return null; }
    return null;
  };
  const internals = tauriIpc(dom.window, handler);
  // Before the fix's transport: an invoke that never reaches `fetch` (nothing to observe).
  if (!observeIpc) { const direct = {}; Object.defineProperty(direct, "invoke", { value: (command, args) => handler(command, args) }); dom.window.__TAURI_INTERNALS__ = direct; }
  void internals;
  const context = { window: dom.window, document: dom.window.document, navigator: dom.window.navigator, Response, Headers,
    CSS: { escape: value => String(value) }, performance, setTimeout, clearTimeout, setInterval, clearInterval,
    requestAnimationFrame: frames ? callback => setTimeout(() => callback(performance.now()), 5) : () => 0,
    TextEncoder, TextDecoder, ArrayBuffer, Uint8Array, DataView };
  context.globalThis = context;
  vm.runInNewContext(readFileSync(new URL("./perf-driver.js", import.meta.url), "utf8"), context);
  await Promise.race([done, new Promise((_, reject) => setTimeout(() => reject(new Error("driver did not finish")), 8000))]);
  return sent;
}

for (const [visibility, frames, why] of [["hidden", false, "a hidden page"], ["visible", false, "a shown page that delivers no animation frame"]]) {
  test(`a C1 run on ${why} is refused before the model opens: no solve is started`, async (t) => {
    const sent = await wholeDriver(t, { visibility, frames });
    assert.equal(sent.report.ok, false);
    assert.match(sent.report.error, new RegExp(`^C1 host condition not established \\(before opening the model: visibilityState=${visibility}, animation frames 0/2 within a 2000 ms timer that fired after \\d+ ms\\)\\. The display must be awake, the session unlocked and the App window uncovered and visible, delivering frames, for the whole observation\\. The run is unobservable: this is not evidence of a product hang, no solve was started, and the run is not retried\\.`));
    assert.deepEqual({ ...sent.report.c1.host, at_ms: typeof sent.report.c1.host.at_ms, detail: typeof sent.report.c1.host.detail }, { established: false, step: "before opening the model", detail: "string", at_ms: "number", solve_started: false });
    // Classified as unobservable: not a hang, not a product refusal.
    const verdict = classify({ report: sent.report, exit: { status: 0, signal: null, ended_ms: null }, timedOut: false, progress: [], memory: { pids: {} }, store: null, run: { c1_probes: true, c1_profile_readers: true, paint_ms: 1000, timeout_s: 10, project_id: "project:invented" } });
    assert.equal(verdict.class, "host_unobservable");
    assert.deepEqual(verdict.findings.map((finding) => finding.class), ["host_unobservable"]);
    assert.match(verdict.findings[0].detail, /not evidence of a product hang; not retried/);
    assert.deepEqual(sent.menus, []);
    assert.deepEqual(sent.chosen, []);
    assert.equal(sent.report.solve, undefined);
  });
}

test("a C1 run on a shown page with frames passes the page check and goes on to open the model", async (t) => {
  const sent = await wholeDriver(t, { visibility: "visible", frames: true });
  assert.equal(sent.report.c1.page.at_start.visibility, "visible");
  assert.equal(sent.report.c1.page.at_start.frames, 2);
  assert.equal(sent.report.c1.host.established, true);
  assert.deepEqual(sent.chosen, ["/invented/rack.json"]);
  assert.deepEqual(sent.menus, ["file.open-model"]);
  assert.match(sent.report.error, /timed out after 1000 ms waiting for open rack\.json/);
});

test("a C1 page hidden after the checks stops the run at once as unobservable, before any solve", async (t) => {
  const sent = await wholeDriver(t, { visibility: "visible", frames: true, hideOnOpen: true });
  assert.match(sent.report.error, /^C1 host condition not established \(during the observation: the page became visibilityState=hidden\)\. .* no solve was started, and the run is not retried\./);
  assert.equal(sent.report.c1.host.established, false);
  assert.equal(sent.report.c1.host.solve_started, false);
  assert.deepEqual(sent.menus, ["file.open-model"]);
  assert.equal(sent.report.solve, undefined);
  // Stopped by the next 100 ms poll of the open wait, not by its 1,000 ms timeout.
  assert.doesNotMatch(sent.report.error, /timed out/);
});

test("a C1 run whose command observation sees no IPC is refused before any page check or solve", async (t) => {
  const sent = await wholeDriver(t, { visibility: "visible", frames: true, observeIpc: false });
  assert.match(sent.report.error, /^C1 command observation saw no IPC/);
  assert.deepEqual(sent.menus, []);
  assert.equal(sent.report.c1.page, undefined);
});

/** The C1 solve step on the simulated clock with the native Results panel, journalling on:
 * the result is committed at `commitAt`, the panel shows settled rows at `rowsAt` (or never)
 * and first_paint is marked at `paintAt` (or never). Returns the report and the journal lines. */
async function observedSolve(t, { visibility, frames, commitAt = 10_000, rowsAt = null, paintAt = null, paintMs = 30_000, hideAt = null }) {
  const clock = fakeClock();
  let shown = "solve", timeline = null;
  const lines = [];
  const html = `<section data-testid="workspace-section-solve"><button data-testid="solver-mode-sparse" aria-pressed="false"></button><p data-testid="solve-job-summary">state=running</p></section><section data-testid="workspace-section-results"><section data-testid="results-panel" data-result-custody="native"><p data-testid="result-page-reading">Reading the result rows natively…</p></section></section>`;
  const ui = await driverDom(t, html, async (command, args) => {
    if (command === "native_smoke_log") { if (String(args.line).startsWith("reader-phase:")) lines.push(JSON.parse(args.line.slice("reader-phase:".length))); return null; }
    if (command !== "native_smoke_menu") return null;
    if (args.commandId === "analyze.run") clock.at(20, () => { timeline = { sequence: 1, outcome: "running", marks: [{ stage: "solve_requested", at_ms: clock.now() }] }; });
    else if (args.commandId === "view.section.results") clock.at(clock.now() + 30, () => { shown = "results"; });
    return null;
  }, { ...clock.context, requestAnimationFrame: frames ? callback => clock.context.setTimeout(() => callback(clock.now()), 16) : () => 0 });
  const doc = ui.dom.window.document;
  let state = visibility;
  setVisibility(doc, null);
  Object.defineProperty(doc, "visibilityState", { configurable: true, get: () => state });
  if (hideAt !== null) clock.at(hideAt, () => { state = "hidden"; doc.dispatchEvent(new ui.dom.window.Event("visibilitychange")); });
  for (const id of ["solve", "results"]) Object.defineProperty(doc.querySelector(`[data-testid="workspace-section-${id}"]`), "offsetParent", { get: () => shown === id ? doc.body : null });
  const sparse = doc.querySelector('[data-testid="solver-mode-sparse"]');
  sparse.onclick = () => sparse.setAttribute("aria-pressed", "true");
  ui.context.__openPipeStressSolveTimelineV1 = { readCurrent: () => timeline };
  clock.at(commitAt, () => { timeline = { ...timeline, outcome: "completed", marks: [...timeline.marks, { stage: "result_committed", at_ms: clock.now() }] }; });
  if (rowsAt !== null) clock.at(rowsAt, () => { doc.querySelector('[data-testid="results-panel"]').innerHTML = '<table><tbody><tr data-testid="result-row-r1"><td>r1</td></tr></tbody></table>'; });
  if (paintAt !== null) clock.at(paintAt, () => { timeline = { ...timeline, marks: [...timeline.marks, { stage: "first_paint", at_ms: clock.now() }] }; });
  const report = {};
  await clock.run(ui.hooks.solve({ c1_probes: true, c1_reader_diagnostics: true, timeouts: { open_ms: 1000, paint_ms: paintMs } }, report));
  return { report, lines };
}
const edges = lines => lines.map(({ phase, call, edge, status }) => `${phase}#${call}:${edge}:${status}`);

test("the C1 solve journals a shown page: Results opened, the first page settled, then its frames to first_paint", async (t) => {
  const { report, lines } = await observedSolve(t, { visibility: "visible", frames: true, rowsAt: 12_000, paintAt: 12_500 });
  assert.equal(report.solve.ended, "first_paint");
  assert.deepEqual(edges(lines), ["results_open#1:enter:pending", "results_open#1:exit:ok", "first_page#1:enter:pending", "first_page#1:exit:ok", "paint_frames#1:enter:pending", "paint_frames#1:exit:ok"]);
  assert.equal(report.solve.page.hidden_polls, 0);
  // Every frame probe is answered, except at most the one sent as the wait ends.
  assert.ok(report.solve.page.frame_acks >= 1 && report.solve.page.frame_acks >= report.solve.page.frame_probes - 1, JSON.stringify(report.solve.page));
  // Seen at the first 200 ms poll after the rows render, before the mark.
  assert.ok(report.solve.page.first_page_seen_at_ms >= 12_000 && report.solve.page.first_page_seen_at_ms <= 12_200, String(report.solve.page.first_page_seen_at_ms));
  for (const line of lines) assert.deepEqual(Object.keys(line).sort(), ["call", "edge", "layer", "mono_ms", "phase", "status"]);
});

test("the C1 solve on the A+B state (a page not shown) stops at its first poll as unobservable, not at the gate", async (t) => {
  const { report, lines } = await observedSolve(t, { visibility: "hidden", frames: false });
  assert.equal(report.solve.ended, "host_unobservable");
  assert.ok(report.solve.elapsed_ms < 1000, String(report.solve.elapsed_ms));
  assert.deepEqual(edges(lines), ["results_open#1:enter:pending", "results_open#1:exit:ok", "page_hidden#1:enter:pending"]);
  assert.equal(report.solve.page.hidden_polls, 1);
  assert.equal(report.solve.page.visibility_at_end, "hidden");
});

test("a C1 page hidden during the solve stops it as unobservable by the next 200 ms poll: no timeout, no hang", async (t) => {
  const { report, lines } = await observedSolve(t, { visibility: "visible", frames: false, hideAt: 20_000 });
  assert.equal(report.solve.ended, "host_unobservable");
  // Seen at the first poll after it, stopped at the one after that, long before the 30 s gate.
  const endedAt = report.solve.gate.started_at_ms + report.solve.elapsed_ms;
  assert.ok(endedAt > 20_000 && endedAt <= 20_000 + 2 * 200 + 1, String(endedAt));
  assert.deepEqual(edges(lines), ["results_open#1:enter:pending", "results_open#1:exit:ok", "frame_wait#1:enter:pending", "first_page#1:enter:pending", "page_hidden#1:enter:pending"]);
  assert.equal(report.solve.page.first_page_seen_at_ms, null);
});

test("the driver's journal lines are accepted by the fixed phase journal and leave the reader evidence complete", async (t) => {
  const dir = mkdtempSync(path.join(os.tmpdir(), "perf-observed-"));
  t.after(() => rmSync(dir, { recursive: true, force: true }));
  const { lines } = await observedSolve(t, { visibility: "visible", frames: false, hideAt: 20_000 });
  const evidence = createReaderPhaseEvidence(path.join(dir, "journal.jsonl"));
  // The app's own fixed awaits and native boundaries, all complete.
  for (const [call, phase] of [[1, "header"], [2, "registration"], [3, "standing"], [4, "summary"], [5, "run_preview"]]) {
    evidence.accept({ layer: "web", call, phase, edge: "enter", status: "pending", mono_ms: call });
    evidence.accept({ layer: "web", call, phase, edge: "exit", status: "ok", mono_ms: call + 0.5 });
  }
  for (const phase of ["args_admit", "source_prescan", "source_admit", "inputs_admit", "source_parse", "inputs_parse", "validate", "serialize", "source_drop", "summary_health", "summary_build"]) {
    evidence.accept({ layer: "native", call: 1, phase, edge: "enter", status: "pending", mono_ms: 1 });
    evidence.accept({ layer: "native", call: 1, phase, edge: "exit", status: "ok", mono_ms: 2 });
  }
  for (const line of lines) evidence.accept(line);
  const result = evidence.close();
  assert.equal(result.evidence_complete, true);
  assert.deepEqual(result.incomplete.map(({ phase }) => phase).sort(), ["first_page", "frame_wait", "page_hidden"]);
  // An open observed phase never stands in for a missing reader await.
  const partial = createReaderPhaseEvidence(path.join(dir, "partial.jsonl"));
  for (const line of lines) partial.accept(line);
  assert.equal(partial.close().evidence_complete, false);
  // Observed phases are fixed names, web only.
  const strict = createReaderPhaseEvidence(path.join(dir, "strict.jsonl"));
  assert.throws(() => strict.accept({ layer: "native", call: 1, phase: "first_page", edge: "enter", status: "pending", mono_ms: 1 }), /malformed/);
  assert.throws(() => strict.accept({ layer: "web", call: 1, phase: "first_page_job", edge: "enter", status: "pending", mono_ms: 1 }), /malformed/);
  strict.close();
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

// RT-U4 (82-node lesson): Record Run's prerequisite save and the File > Save after it show the
// same status text. A save step ends on its own native save invocation, never on the status
// line's change, an earlier receipt or a later step.
const SAME_SAVE_TEXT = "Saved current project model snapshot without external file copies.";
async function saveStepDom(t, onSave) {
  let ui;
  const sent = [];
  ui = await driverDom(t, `<p data-testid="local-project-message">${SAME_SAVE_TEXT}</p>`, async (command, args) => {
    if (command === "native_smoke_menu" && args.commandId === "file.save-local") {
      // The App answers the menu at once and saves afterwards, as the native menu does.
      setTimeout(() => { void onSave(ui, sent); }, 20);
      return null;
    }
    if (command === "save_preview_mechanics_job_local_project" || command === "save_restored_result_local_project") {
      sent.push(command);
      const reply = await (ui.reply ?? (() => ({ result: { outcome: "saved", run_id: "run:x", result_bytes: 1, record_bytes: 2 }, project_envelope_hash: { value: "sha256:e" } })))(command);
      return reply;
    }
    return null;
  });
  return ui;
}
const setLocalMessage = (ui, value) => { ui.dom.window.document.querySelector('[data-testid="local-project-message"]').textContent = value; };
const savePlan = (save_ms = 2000) => ({ timeouts: { save_ms } });

test("C1 save steps: a File > Save after Record Run's same-text save ends on its own fresh receipt, not a timeout", async (t) => {
  const ui = await saveStepDom(t, async (ui) => { await ui.call("save_preview_mechanics_job_local_project", {}); setLocalMessage(ui, SAME_SAVE_TEXT); });
  const trace = ui.hooks.beginC1Trace();
  t.after(() => trace.restore());
  // Record Run's prerequisite save: invocation 1, leaving the same status text.
  await ui.call("save_preview_mechanics_job_local_project", {});
  const report = {};
  const started = performance.now();
  const step = await ui.hooks.saveProject(savePlan(), report, trace);
  assert.ok(performance.now() - started < 1500, "ended on its receipt, well before save_ms");
  assert.equal(step.timed_out, undefined);
  assert.equal(step.message, SAME_SAVE_TEXT);
  assert.equal(step.receipt.seq, 2);
  assert.equal(step.receipt.command, "save_preview_mechanics_job_local_project");
  assert.equal(step.receipt.read, true);
  assert.equal(step.receipt.reply.result.outcome, "saved");
  assert.equal(report.save_projects.length, 1);
  assert.equal(report.save_projects[0], step);
  assert.equal(report.save_project, step);
});

test("C1 save steps: the old changed-text wait times out on the same sequence (the defect, without the trace)", async (t) => {
  const ui = await saveStepDom(t, async (ui) => { await ui.call("save_preview_mechanics_job_local_project", {}); setLocalMessage(ui, SAME_SAVE_TEXT); });
  const report = {};
  const step = await ui.hooks.saveProject(savePlan(300), report);
  assert.equal(step.timed_out, true);
  assert.equal(step.message, null);
});

test("C1 save steps: no own native save cannot finish on an earlier receipt or the unchanged status text", async (t) => {
  const ui = await saveStepDom(t, async () => { /* File > Save sends nothing */ });
  const trace = ui.hooks.beginC1Trace();
  t.after(() => trace.restore());
  await ui.call("save_preview_mechanics_job_local_project", {}); // an earlier, successful receipt
  const report = {};
  const step = await ui.hooks.saveProject(savePlan(300), report, trace);
  assert.equal(step.timed_out, true);
  assert.equal(step.receipt, null);
  assert.equal(trace.data.saves.length, 1);
});

test("C1 save steps: a refused native save ends its step with the refusal and no saved reply", async (t) => {
  const ui = await saveStepDom(t, async (ui) => { await ui.call("save_preview_mechanics_job_local_project", {}).catch(() => {}); setLocalMessage(ui, "Save failed: PROJECT-SAVE-TOO-LARGE"); });
  ui.reply = () => { throw new Error("PROJECT-SAVE-TOO-LARGE: the project does not fit"); };
  const trace = ui.hooks.beginC1Trace();
  t.after(() => trace.restore());
  const step = await ui.hooks.saveProject(savePlan(), {}, trace);
  assert.equal(step.timed_out, undefined);
  assert.equal(step.receipt.seq, 1);
  assert.equal(step.receipt.reply, null);
  assert.match(step.receipt.refusal, /PROJECT-SAVE-TOO-LARGE/);
});

test("C1 save steps: one File > Save that sends two native saves fails the step", async (t) => {
  const ui = await saveStepDom(t, async (ui) => { await ui.call("save_preview_mechanics_job_local_project", {}); await ui.call("save_preview_mechanics_job_local_project", {}); setLocalMessage(ui, SAME_SAVE_TEXT); });
  const trace = ui.hooks.beginC1Trace();
  t.after(() => trace.restore());
  await assert.rejects(ui.hooks.saveProject(savePlan(), {}, trace), /sent 2 native saves/);
});

test("C1 save steps: a later successful re-save does not mask an earlier step's timeout or not_saved receipt", () => {
  const first = { step: 1, message: null, timed_out: true, last_message: SAME_SAVE_TEXT, receipt: null };
  const second = { step: 2, message: "Saved restored SQLite project snapshot.", receipt: { seq: 2, command: "save_restored_result_local_project", read: true, reply: { result: { outcome: "saved" } } } };
  const timedOut = classify(base({ report: { solve: painted(), record_run: { attempted: true, message: "Recorded run perf r." }, save_project: first, save_projects: [first, second] } }));
  assert.equal(timedOut.class, "hang");
  assert.ok(timedOut.findings.some((finding) => finding.step === "save_project#1"));
  const omitted = base();
  omitted.report.c1 = { trace: { saves: [{ seq: 1, reply: { result: { outcome: "not_saved", code: "PROJECT-SAVE-RESULT-TOO-LARGE" } } }, { seq: 2, reply: { result: { outcome: "saved" } } }], save: { result: { outcome: "saved" } } } };
  const refused = classify(omitted);
  assert.ok(refused.findings.some((finding) => finding.step === "save_result" && finding.message === "PROJECT-SAVE-RESULT-TOO-LARGE"));
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
  metrics.result={rows:[{id:"a result payload must not be retained"}]};
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


test("fixed C1 phase evidence is durable before a blocked phase and keeps missing exits", t => {
  const dir=mkdtempSync(path.join(os.tmpdir(),"c1-phases-"));
  t.after(()=>rmSync(dir,{recursive:true,force:true}));
  const file=path.join(dir,"phases.jsonl"), journal=createReaderPhaseEvidence(file);
  const event={layer:"native",call:1,phase:"source_parse",edge:"enter",status:"pending",mono_ms:1,bytes:128};
  journal.accept(event);
  assert.equal(JSON.parse(readFileSync(file,"utf8").trim()).phase,"source_parse");
  // No final reply is needed to recover the last entered phase.
  assert.equal(journal.summary().incomplete.length,1);
  const closed=journal.close();
  assert.equal(closed.evidence_complete,false);
  assert.equal(closed.incomplete[0].phase,"source_parse");
  assert.equal(JSON.parse(readFileSync(file,"utf8").trim().split("\n").at(-1)).evidence_complete,false);
  assert.throws(()=>journal.accept({...event,edge:"exit",status:"ok"}),/closed/);
});

test("C1 phase journal bounds retention, rejects contents and never hides overflow", t => {
  const dir=mkdtempSync(path.join(os.tmpdir(),"c1-phases-bound-"));
  t.after(()=>rmSync(dir,{recursive:true,force:true}));
  const file=path.join(dir,"phases.jsonl"),journal=createReaderPhaseEvidence(file);
  const event={layer:"native",call:1,phase:"validate",edge:"enter",status:"pending",mono_ms:1};
  assert.throws(()=>journal.accept({...event,source:"must not be logged"}),/malformed/);
  assert.throws(()=>journal.accept({...event,phase:"customer-text"}),/malformed/);
  for(let i=0;i<128;i++) {journal.accept(event);journal.accept({...event,edge:"exit",status:"ok"});}
  journal.accept(event);journal.accept({...event,edge:"exit",status:"ok"});
  const result=journal.close();
  assert.equal(result.events.length,256);assert.equal(result.overflow,true);assert.equal(result.evidence_complete,false);
  assert.ok(readFileSync(file).length<100000);
  assert.equal(readFileSync(file,"utf8").includes("must not be logged"),false);
});

test("an interrupted fake App leaves its entered native phase in the evidence file", {timeout:5000}, async t => {
  const dir=mkdtempSync(path.join(os.tmpdir(),"c1-phases-child-"));
  t.after(()=>rmSync(dir,{recursive:true,force:true}));
  const file=path.join(dir,"phases.jsonl"),journal=createReaderPhaseEvidence(file);
  const child=spawn(process.execPath,['-e',String.raw`process.stdout.write(JSON.stringify({layer:'native',call:1,phase:'validate',edge:'enter',status:'pending',mono_ms:1})+'\n');setInterval(()=>{},1000)`],{stdio:['ignore','pipe','ignore']});
  t.after(()=>{try{child.kill('SIGKILL');}catch{}});
  const exited=once(child,'exit');
  const [bytes]=await Promise.race([once(child.stdout,'data'), exited.then(()=>{throw new Error('fixture exited before phase entry');})]);journal.accept(JSON.parse(String(bytes)));
  child.kill('SIGKILL');await exited;
  assert.equal(journal.close().incomplete[0].phase,'validate');
  assert.equal(readFileSync(file,"utf8").includes('"edge":"enter"'),true);
});


test("C1 phase evidence requires all fixed awaits and native boundaries, not just an empty active set", t => {
  const dir=mkdtempSync(path.join(os.tmpdir(),"c1-phase-coverage-"));
  t.after(()=>rmSync(dir,{recursive:true,force:true}));
  assert.equal(createReaderPhaseEvidence(path.join(dir,"empty.jsonl")).close().evidence_complete,false);
  const phases={web:["header","registration","standing","summary","run_preview"],native:["args_admit","source_prescan","source_admit","inputs_admit","source_parse","inputs_parse","validate","serialize","source_drop","summary_health","summary_build"]};
  const complete=(name,omit=null)=>{
    const journal=createReaderPhaseEvidence(path.join(dir,name));
    for(const [layer,names] of Object.entries(phases)) for(const phase of names) {
      if(phase===omit) continue;
      journal.accept({layer,phase,call:1,edge:"enter",status:"pending",mono_ms:1});
      journal.accept({layer,phase,call:1,edge:"exit",status:"ok",mono_ms:2});
    }
    return journal.close();
  };
  assert.equal(complete("complete.jsonl").evidence_complete,true);
  // The fresh solve's analysis-run preview await is a required web phase: without its exit the
  // evidence is incomplete, as for the four reader awaits.
  assert.equal(complete("no-run-preview.jsonl","run_preview").evidence_complete,false);
});

test("the run_preview phase is web-only and its pending entry survives as incomplete", t => {
  const dir=mkdtempSync(path.join(os.tmpdir(),"c1-run-preview-"));
  t.after(()=>rmSync(dir,{recursive:true,force:true}));
  const journal=createReaderPhaseEvidence(path.join(dir,"phases.jsonl"));
  assert.throws(()=>journal.accept({layer:"native",phase:"run_preview",call:1,edge:"enter",status:"pending",mono_ms:1}),/malformed/);
  journal.accept({layer:"web",phase:"run_preview",call:7,edge:"enter",status:"pending",mono_ms:1});
  const closed=journal.close();
  assert.deepEqual(closed.incomplete.map(({layer,call,phase})=>({layer,call,phase})),[{layer:"web",call:7,phase:"run_preview"}]);
  assert.equal(closed.evidence_complete,false);
});
