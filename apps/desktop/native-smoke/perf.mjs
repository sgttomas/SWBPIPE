#!/usr/bin/env node
// Desktop performance runner (RT-U4, release target exit criterion 9; DEL-04-05
// `Work/criterion-9-measurement.md`, "Desktop measurement"), macOS. Observation only:
// nothing here asserts a time or memory bound.
//
//   node apps/desktop/native-smoke/perf.mjs --build --app-dir DIR [--target-dir DIR]
//   node apps/desktop/native-smoke/perf.mjs --plan [--plan-file F] [--subset S]
//   node apps/desktop/native-smoke/perf.mjs --run --app-dir DIR --models DIR [--results DIR]
//        --out DIR [--plan-file F] [--subset S | --runs id,id] [--budget-s N] [--keep]
//
// --build (under the host slot, which the caller takes first; it runs cargo itself):
//   under the worktree build lock, refuses a stale WebAssembly engine (run `npm run
//   build:wasm` first), builds the frontend (`npm run build`) and a RELEASE build of the
//   app with `--features native-smoke,tauri/custom-protocol` and the smoke's
//   TAURI_CONFIG (background throttling off, as `run.mjs`), into --target-dir (default
//   CARGO_TARGET_DIR; one is required). The executable cargo reports is copied to
//   DIR/openpipestress-desktop and bound in DIR/perf-binding.json (SHA-256, source
//   commit and dirty state, engine stamps, dist digest, cargo command). `run.mjs`
//   builds debug into src-tauri/target; timings need a release build, hence this one.
//
// --run: for each planned run (one model, one mode), in plan order within the window
//   budget, launches a private copy of the bound app (its SHA-256 checked) against a
//   throwaway HOME with `perf-driver.js` as the native-smoke driver:
//     - mode `native`: the app's own solve worker solves the model;
//     - mode `injected`: SWBPIPE_NATIVE_SMOKE_INJECT_RESULT names the run's result file
//       (`release_timing --write` from the same model document), which the started job
//       publishes instead of starting its worker (`src-tauri/src/native_smoke_inject.rs`).
//   It samples the app, its solve workers and the WebKit processes started for it
//   (`perf-sampler.py`), waits for the app to finish or the run's timeout, inspects the
//   SQLite store and classifies the run. An independent --run supervisor bounds
//   its internal --run-child, including synchronous inspection. Raw sampler lines
//   and completed-run records are persisted before advancing; forced-stop receipts
//   explicitly describe partial evidence. Without --keep, an ok run's private
//   HOME is removed after its record is committed; failed stores are retained.
//   The supervisor removes remaining scratch only after process verification.
//
// Threat model (accepted for PR #53's permissions review): the plan checks (bare names,
// containedPath) and the injection root guard against mistakes in plan text and caller input:
// traversal names, absolute paths, links present when checked, wrong roots. They are NOT a
// defence against a hostile local process that changes the scratch, models or results
// directories concurrently (a time-of-check to time-of-use race between containedPath and the
// copy or the app's open); that finding is recorded out of model. Native-smoke builds are
// never distributed.
//
// Classes (a run can show several; `class` is the most severe): crash (which process),
// half_written_save, misleading_message, hang (timeout), refused_with_message (quoted),
// ok. A message is quoted verbatim; `needs_review` marks one a person should judge.
import { execFile, fork, spawn, spawnSync } from "node:child_process";
import { rm, writeFile } from "node:fs/promises";
import { createHash } from "node:crypto";
import { chmodSync, constants, copyFileSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, realpathSync, renameSync, rmSync, statSync, writeFileSync, writeSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { parseArgs } from "node:util";
import { isMainThread, parentPort, Worker, workerData } from "node:worker_threads";

const here = path.dirname(fileURLToPath(import.meta.url));
const desktop = path.resolve(here, "..");
const project = path.resolve(desktop, "..", "..");
const tauriDir = path.join(desktop, "src-tauri");
const TAURI_CONF = JSON.parse(readFileSync(path.join(tauriDir, "tauri.conf.json"), "utf8"));
const SMOKE_TAURI_CONFIG = JSON.stringify({ app: { windows: TAURI_CONF.app.windows.map((window) => ({ ...window, backgroundThrottling: "disabled" })) } });
const BIN_NAME = "openpipestress-desktop";
const BINDING = "perf-binding.json";
const INJECT_ENV = "SWBPIPE_NATIVE_SMOKE_INJECT_RESULT";
// The directory the injected file must lie in (the hook refuses anything else).
const INJECT_ROOT_ENV = "SWBPIPE_NATIVE_SMOKE_INJECT_ROOT";
const SEVERITY = ["crash", "half_written_save", "misleading_message", "hang", "refused_with_message", "ok"];
const GIB = 1024 ** 3;

function killOwnRunnerGroup() {
  try { process.kill(-process.pid, "SIGKILL"); }
  finally { process.exit(125); }
}

/** IPC disconnect catches an ordinary parent exit. A same-module worker thread
 * also checks reparenting while this runner's main thread is blocked in SQLite.
 * Arm and acknowledge it before any App launch; unexpected guard failure stops
 * the owned runner group. Product workers retain their own parent-death guard. */
export async function guardSupervisor(parentPid) {
  if (!Number.isSafeInteger(parentPid) || parentPid <= 1) throw new Error("perf run-child: invalid supervisor PID");
  process.on("disconnect", killOwnRunnerGroup);
  if (process.ppid !== parentPid) killOwnRunnerGroup();
  const guard = new Worker(fileURLToPath(import.meta.url), { execArgv: [], workerData: { perfParentPid: parentPid } });
  let closing = false;
  guard.on("error", killOwnRunnerGroup);
  guard.on("exit", () => { if (!closing) killOwnRunnerGroup(); });
  await new Promise((resolve) => guard.once("message", resolve));
  return async () => {
    closing = true;
    process.off("disconnect", killOwnRunnerGroup);
    // Do not keep a normally finished runner alive, but still protect it if
    // a remaining helper or pending I/O delays exit after IPC is closed.
    guard.unref();
  };
}

/** Preserve the cause before the failure path kills this process group.
 * Synchronous stderr is necessary: a pipe-backed console write may still be
 * queued when SIGKILL lands. The same lifecycle is used by --run-child. */
export async function runChild(body, parentPid) {
  const stopGuard = await guardSupervisor(parentPid);
  let completed = false;
  try { await body(); completed = true; }
  catch (error) {
    const message = String(error?.message ?? error).slice(0, 2000);
    try { writeSync(2, `perf runner: ${message}\n`); }
    catch { /* A closed stderr must not prevent owned-group teardown. */ }
    audit({ type: "stop", reason: `runner_error: ${message}`.slice(0, 200) });
    throw error;
  }
  finally {
    if (!completed) killOwnRunnerGroup();
    await stopGuard();
    if (process.connected) process.disconnect();
  }
}

// The runner/App/helpers share the group created by fork. Product workers
// create their own groups; listOwned identifies them by the unique private
// executable path. Freeze the current App before that list, preventing new
// worker spawns, but leave the runner's parent-death guard runnable.
export function watchWindow(child, { budgetMs, killGraceMs = 5000, finalGraceMs = 5000, listOwned = async () => [] }) {
  if (!Number.isSafeInteger(child.pid) || child.pid <= 1) throw new Error("perf window: no owned runner PID");
  const started = Date.now();
  const events = [];
  const groupErrors = [];
  const signalGroup = (signal) => {
    try { process.kill(-child.pid, signal); return true; }
    catch (error) {
      if (error.code === "ESRCH") return false;
      if (signal === 0 && error.code === "EPERM") return true;
      groupErrors.push({ signal, error: error.message });
      return null;
    }
  };
  return new Promise((resolve) => {
    let reason = null;
    let triggerSample = null;
    let latestSample = null;
    let activeAppPid = null;
    const completedRuns = [];
    let killTimer = null;
    let finalTimer = null;
    let finished = false;
    const stop = (why) => {
      if (finished) { reason ??= why; return; }
      if (reason) return;
      reason = why;
      signalGroup("SIGTERM");
      killTimer = setTimeout(() => signalGroup("SIGKILL"), killGraceMs);
      finalTimer = setTimeout(() => finish(null, null, false), killGraceMs + finalGraceMs);
    };
    const interrupted = (signal) => stop(`runner_${signal}`);
    const onInt = () => interrupted("SIGINT");
    const onTerm = () => interrupted("SIGTERM");
    const onHup = () => interrupted("SIGHUP");
    const deadline = setTimeout(() => stop("window_deadline"), budgetMs);
    const finish = async (code, signal, reaped) => {
      if (finished) return;
      finished = true;
      clearTimeout(deadline); clearTimeout(killTimer); clearTimeout(finalTimer);
      if (activeAppPid !== null) {
        try { process.kill(activeAppPid, "SIGSTOP"); }
        catch (error) { if (error.code !== "ESRCH") groupErrors.push({ pid: activeAppPid, error: error.message }); }
      }
      let owned = [];
      let workersVerified = true;
      let discoveryTimer;
      try {
        owned = await Promise.race([listOwned(), new Promise((_, reject) => {
          discoveryTimer = setTimeout(() => reject(new Error("owned executable lookup timed out")), 1500);
        })]);
        if (!Array.isArray(owned) || owned.some((row) => !Number.isSafeInteger(row?.pid) || row.pid <= 1 || !Number.isSafeInteger(row.group) || row.group <= 1)) {
          owned = [];
          throw new Error("invalid owned executable lookup");
        }
      }
      catch (error) { workersVerified = false; groupErrors.push({ signal: "enumerate", error: error.message }); }
      finally { clearTimeout(discoveryTimer); }
      for (const { pid, group } of owned) {
        if (!Number.isSafeInteger(pid) || pid <= 1 || pid === process.pid) {
          workersVerified = false;
          continue;
        }
        try { process.kill(group === pid ? -pid : pid, "SIGKILL"); }
        catch (error) { if (error.code !== "ESRCH") { workersVerified = false; groupErrors.push({ pid, error: error.message }); } }
      }
      const groupSignalled = signalGroup("SIGKILL");
      child.off("message", message);
      child.off("exit", exited);
      child.off("error", failed);
      if (child.connected) child.disconnect();
      child.unref();
      let live = signalGroup(0);
      const until = performance.now() + 2000;
      const pidLive = (pid) => { try { process.kill(pid, 0); return true; } catch (error) { return error.code !== "ESRCH"; } };
      while ((live === true || owned.some(({pid}) => pidLive(pid))) && performance.now() < until) {
        await new Promise((done) => setTimeout(done, 50));
        live = signalGroup(0);
      }
      process.off("SIGINT", onInt); process.off("SIGTERM", onTerm); process.off("SIGHUP", onHup);
      resolve({ schema: "swbpipe.desktop-perf-window/v1", started_ms: started,
        ended_ms: Date.now(), runner_pid: child.pid, process_group: child.pid,
        stop_reason: reason, exit: { code, signal, reaped },
        remaining_group_signalled: groupSignalled,
        // A sent signal is not proof of exit, and exit is not proof of commit.
        owned_group_gone: live === null ? null : !live,
        owned_executable_pids_gone: workersVerified && owned.every(({pid}) => !pidLive(pid)),
        owned_pids: owned, group_errors: groupErrors, events,
        trigger_sample: triggerSample, latest_sample: latestSample,
        completed_run_files: completedRuns,
        evidence_complete: reason === null && code === 0 && reaped && live === false && workersVerified && owned.every(({pid}) => !pidLive(pid)) });
    };
    const message = (event) => {
      if (event?.type === "app" && Number.isSafeInteger(event.pid) && event.pid > 1) activeAppPid = event.pid;
      if (event?.type === "app_exit" && event.pid === activeAppPid) activeAppPid = null;
      if (event?.type === "sample") latestSample = event.sample;
      if (event?.type === "completed_run") completedRuns.push(event.path);
      if (event?.type === "stop" && typeof event.reason === "string") {
        triggerSample ??= event.sample ?? latestSample;
        stop(event.reason.slice(0, 200));
      }
      if (["scratch", "app", "stage"].includes(event?.type)) {
        events.push(event);
        if (events.length > 64) events.shift();
      }
    };
    const exited = (code, signal) => finish(code, signal, true);
    const failed = (error) => { reason ??= `runner_error: ${error.message}`; finish(null, null, false); };
    child.on("message", message); child.once("exit", exited); child.once("error", failed);
    process.on("SIGINT", onInt); process.on("SIGTERM", onTerm); process.on("SIGHUP", onHup);
  });
}

/** Only the sample's aligned sum is tested, never a sum of separate peaks.
 * This is a protective run stop; it changes no product admission policy. */
export function sampleStopReason(sample, { allowUnknownHost = false, allowUnknownPids = new Set(), allowMissingApp = false } = {}) {
  const knownHeadroom = Number.isFinite(sample.host?.available_percent) && sample.host.available_percent >= 0 && sample.host.available_percent <= 100;
  if (!knownHeadroom && !allowUnknownHost) return "host_headroom_unknown";
  if (knownHeadroom && sample.host.available_percent < 30) return "host_headroom_below_30_percent";
  const knownPressure = [1, 2, 4].includes(sample.host?.pressure_level);
  if (!knownPressure && !allowUnknownHost) return "host_pressure_unknown";
  if (knownPressure && sample.host.pressure_level !== 1) return "host_pressure_elevated";
  let total = 0;
  const webcontent = [];
  let app = false;
  for (const [pid, kind, rss, footprint] of sample.p) {
    if (Number.isFinite(rss) && rss > 16 * GIB) return "owned_pid_rss_above_16_gib";
    if (!["app", "worker", "webcontent"].includes(kind)) continue;
    if (kind === "app") app = true;
    if (kind === "webcontent") webcontent.push(pid);
    if (!Number.isFinite(rss) || !Number.isFinite(footprint) || rss < 0 || footprint < 0) {
      if (!allowUnknownPids.has(pid)) return "process_memory_unknown";
      continue;
    }
    total += footprint;
  }
  if (!app && !allowMissingApp) return "app_memory_unknown";
  if (webcontent.length > 1) return "webcontent_attribution_ambiguous";
  if (total > 12 * GIB) return "aligned_footprint_above_12_gib";
  return null;
}

/** One vanished PID/read failure is retained as unknown evidence, not a stop.
 * Require a second consecutive unknown for that PID, or a second independent
 * bad host reading; a cached 1Hz host result is not five separate failures. */
export function createSampleGuard() {
  const unknown = new Map();
  let badHostReadings = 0;
  let lastHostReading = null;
  let missingApp = 0;
  return (sample) => {
    const host = sample.host;
    const badHost = !Number.isFinite(host?.available_percent) || host.available_percent < 0 || host.available_percent > 100 || ![1, 2, 4].includes(host?.pressure_level);
    if (!badHost) badHostReadings = 0;
    else if (!Number.isFinite(host?.sampled_at_ms) || host.sampled_at_ms !== lastHostReading) badHostReadings++;
    lastHostReading = host?.sampled_at_ms;
    const present = new Set(sample.p.map(([pid]) => pid));
    for (const pid of unknown.keys()) if (!present.has(pid)) unknown.delete(pid);
    const allowed = new Set();
    let app = false;
    for (const [pid, kind, rss, footprint] of sample.p) {
      if (kind === "app") app = true;
      if (!["app", "worker", "webcontent"].includes(kind)) continue;
      if (!Number.isFinite(rss) || !Number.isFinite(footprint) || rss < 0 || footprint < 0) {
        const count = (unknown.get(pid) ?? 0) + 1;
        unknown.set(pid, count);
        if (count < 2) allowed.add(pid);
      } else unknown.delete(pid);
    }
    missingApp = app ? 0 : missingApp + 1;
    return sampleStopReason(sample, { allowUnknownHost: badHostReadings < 2, allowUnknownPids: allowed, allowMissingApp: missingApp < 2 });
  };
}

function audit(event) { if (process.send && process.connected) process.send(event); }
function compactSample(value) {
  return { t: value.t, host: value.host, p: value.p.slice(0, 64), truncated: value.p.length > 64 };
}

async function supervisedRun(options) {
  const plan = loadPlan(options.planFile);
  const budgetS = options.budgetS ?? Math.min(plan.budget_s, 840);
  if (!Number.isFinite(budgetS) || budgetS <= 0 || budgetS > 840) throw new Error("perf window: work budget must be > 0 and <= 840 s (teardown stays within 15 min)");
  mkdirSync(options.out, { recursive: true });
  const scratch = mkdtempSync(path.join(options.out, "perf-"));
  const binary = path.join(scratch, BIN_NAME);
  const postRunReserveS = Math.min(30, budgetS / 4);
  const args = ["--run-child", "--app-dir", options.appDir, "--models", options.models,
    "--out", options.out, "--plan-file", options.planFile, "--budget-s", String(budgetS - postRunReserveS),
    "--window-root", scratch, "--supervisor-pid", String(process.pid)];
  if (options.results) args.push("--results", options.results);
  if (options.subset) args.push("--subset", options.subset);
  if (options.runs) args.push("--runs", options.runs);
  if (options.keep) args.push("--keep");
  const child = fork(fileURLToPath(import.meta.url), args, {
    detached: true, stdio: ["ignore", "inherit", "inherit", "ipc"]
  });
  const listOwned = () => new Promise((resolve, reject) => {
    execFile("python3", [path.join(here, "perf-sampler.py"), "--list-owned", "--app-path", binary],
      { timeout: 1000, killSignal: "SIGKILL", maxBuffer: 64 * 1024 }, (error, stdout) => {
        if (error) return reject(error);
        try { resolve(JSON.parse(stdout)); } catch (invalid) { reject(invalid); }
      });
  });
  const result = await watchWindow(child, { budgetMs: budgetS * 1000, listOwned });
  result.post_run_reserve_s = postRunReserveS;
  const file = path.join(options.out, `perf-window-${result.started_ms}.json`);
  // The parent does not synchronously inspect SQLite, recursively remove the
  // store, or serialize the sampler's large report on its deadline event loop.
  const emergency = setTimeout(() => { console.error("perf window: report write stalled; owned runner already stopped"); process.exit(124); }, 10000);
  const failed = result.stop_reason || result.exit.code !== 0 || !result.exit.reaped || result.owned_group_gone !== true || !result.owned_executable_pids_gone;
  try {
    result.kept = failed ? scratch : null;
    await writeFile(file, `${JSON.stringify(result, null, 2)}\n`);
    if (!failed) {
      await finishScratch({ scratch, binary, keep: options.keep });
      result.cleanup = options.keep ? "private_app_removed_store_kept" : "private_scratch_removed";
      result.kept = options.keep ? scratch : null;
      await writeFile(file, `${JSON.stringify(result, null, 2)}\n`);
    }
  }
  finally { clearTimeout(emergency); }
  if (failed) process.exitCode = 1;
}

const sha256File = (file) => {
  const hash = createHash("sha256");
  hash.update(readFileSync(file));
  return hash.digest("hex");
};
// RFC 8785 JCS for JSON documents (finite numbers; ECMAScript number text), as run.mjs.
function canonical(value) {
  if (Array.isArray(value)) return `[${value.map(canonical).join(",")}]`;
  if (value && typeof value === "object") return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonical(value[key])}`).join(",")}}`;
  return JSON.stringify(value);
}
const git = (...args) => spawnSync("git", args, { cwd: project, encoding: "utf8" }).stdout.trim();

// A plan names runs and files by bare names only: no separator, no `..`, no leading dot.
const PLAN_NAME = /^[A-Za-z0-9_-][A-Za-z0-9._-]{0,127}$/;
const MODES = new Set(["native", "injected"]);

/** `name` when it is a bare plan name, else a refusal naming `what`. */
export function planName(name, what) {
  if (typeof name !== "string" || !PLAN_NAME.test(name) || name.includes("..")) {
    throw new Error(`perf plan: ${what} ${JSON.stringify(name)} is not a bare name ([A-Za-z0-9._-], no separator, no "..", no leading dot)`);
  }
  return name;
}

/** The path of `name` inside `root`, refused if it resolves outside it. An existing
 * path is checked again after resolving symbolic links (both sides), and `mustExist`
 * requires a regular file there. */
export function containedPath(root, name, { mustExist = false } = {}) {
  const base = path.resolve(root);
  const candidate = path.resolve(base, planName(name, "file name"));
  const inside = (outer, inner) => {
    const relative = path.relative(outer, inner);
    return relative !== "" && !relative.startsWith("..") && !path.isAbsolute(relative);
  };
  if (!inside(base, candidate)) throw new Error(`perf plan: ${name} resolves outside ${base}`);
  if (lstatSync(candidate, { throwIfNoEntry: false })) {
    let real;
    try { real = realpathSync(candidate); } catch { throw new Error(`perf plan: ${name} is a dangling symbolic link in ${base}`); }
    if (!inside(realpathSync(base), real)) throw new Error(`perf plan: ${name} resolves (through a symbolic link) outside ${base}`);
    if (mustExist && !statSync(real).isFile()) throw new Error(`perf plan: ${name} is not a regular file`);
  } else if (mustExist) {
    throw new Error(`perf plan: ${name} does not exist in ${base}`);
  }
  return candidate;
}

/** Refuses a plan whose run ids, modes, file names or subsets are not what it may name. */
export function validatePlan(plan) {
  if (!Array.isArray(plan.runs) || !plan.subsets || typeof plan.subsets !== "object") throw new Error("perf plan: runs and subsets are required");
  if (plan.c1_probes !== undefined && typeof plan.c1_probes !== "boolean") throw new Error("perf plan: c1_probes must be a boolean");
  if (plan.c1_probes && ["interaction_ms", "csv_ms"].some(key => !Number.isFinite(plan.timeouts?.[key]) || plan.timeouts[key] <= 0)) throw new Error("perf plan: C1 interaction and CSV timeouts are required");
  if (plan.c1_profile_readers !== undefined && typeof plan.c1_profile_readers !== "boolean") throw new Error("perf plan: c1_profile_readers must be boolean");
  if (plan.c1_profile_readers && (!plan.c1_probes || !Number.isFinite(plan.timeouts?.profile_ms) || plan.timeouts.profile_ms <= 0)) throw new Error("perf plan: reader profiling needs C1 probes and a bounded timeout");
  const ids = new Set();
  for (const run of plan.runs) {
    planName(run.id, "run id");
    if (ids.has(run.id)) throw new Error(`perf plan: run id ${run.id} is repeated`);
    ids.add(run.id);
    if (!MODES.has(run.mode)) throw new Error(`perf plan: run ${run.id} has mode ${JSON.stringify(run.mode)}`);
    if (plan.c1_probes && run.mode !== "native") throw new Error("perf plan: C1 usability probes require a native solve");
    if (plan.c1_probes && (![500, 1000].includes(run.nodes) || !/^[a-f0-9]{64}$/.test(run.model_sha256))) throw new Error("perf plan: C1 needs a bound 500/1000-node fixture; no size escalation");
    planName(run.model, `run ${run.id}'s model`);
    if (run.mode === "injected") planName(run.result, `run ${run.id}'s result`);
    else if (run.result !== undefined) throw new Error(`perf plan: native run ${run.id} names a result`);
    for (const key of ["estimate_s", "timeout_s", "paint_ms"]) {
      if (!Number.isFinite(run[key]) || run[key] <= 0) throw new Error(`perf plan: run ${run.id} has ${key} ${JSON.stringify(run[key])}`);
    }
  }
  for (const [subset, members] of Object.entries(plan.subsets)) {
    planName(subset, "subset");
    if (!Array.isArray(members) || members.some((id) => !ids.has(id))) throw new Error(`perf plan: subset ${subset} names an unknown run`);
  }
  return plan;
}

export function loadPlan(file) {
  const plan = JSON.parse(readFileSync(file, "utf8"));
  if (plan.schema !== "swbpipe.desktop-perf-plan/v1") throw new Error(`${file}: not a desktop perf plan`);
  return validatePlan(plan);
}

export function selectRuns(plan, { subset, runs }) {
  if (runs) {
    const wanted = runs.split(",").filter(Boolean);
    const unknown = wanted.filter((id) => !plan.runs.some((run) => run.id === id));
    if (unknown.length || !wanted.length) throw new Error(`unknown run ids: ${unknown.join(", ") || runs}`);
    return plan.runs.filter((run) => wanted.includes(run.id));
  }
  const ids = plan.subsets[subset ?? "window"];
  if (!ids) throw new Error(`unknown subset ${subset}; subsets: ${Object.keys(plan.subsets).join(", ")}`);
  return plan.runs.filter((run) => ids.includes(run.id));
}

export function formatPlan(runs, budgetS) {
  const lines = ["run id                          mode      model                     estimate_s timeout_s"];
  for (const run of runs) lines.push(`${run.id.padEnd(31)} ${run.mode.padEnd(9)} ${run.model.padEnd(25)} ${String(run.estimate_s).padStart(10)} ${String(run.timeout_s).padStart(9)}`);
  const total = runs.reduce((sum, run) => sum + run.estimate_s, 0);
  lines.push(`total estimate: ${total} s (${(total / 60).toFixed(1)} min); budget ${budgetS} s: a run whose estimate exceeds the remaining budget is skipped, and each timeout is capped at it.`);
  lines.push("estimates: each run's timeout (a run ends at its first failure, its report or its timeout); not measurements.");
  return `${lines.join("\n")}\n`;
}

// ---------------------------------------------------------------------- build

async function build({ appDir, targetDir }) {
  const { acquireBuildLock, runUnderLock, refuseCompilerWrapperEnv, BUILD_LOCK_TOKEN_ENV } = await import("../scripts/worktreeBuildLock.mjs");
  const { ENGINE_BUILDS } = await import("../scripts/build-wasm-engine.mjs");
  const { checkEngine, engineStampContext } = await import("../scripts/engineSourceStamp.mjs");
  const { appExecutableFromCargoMessages, treeDigest } = await import("../scripts/appBinaryBinding.mjs");
  if (!targetDir) throw new Error("perf build: give --target-dir or set CARGO_TARGET_DIR (the host's target directory for this lane)");
  refuseCompilerWrapperEnv();
  const lock = acquireBuildLock({ purpose: "desktop perf build" });
  process.env[BUILD_LOCK_TOKEN_ENV] = lock.token;
  try {
    const context = engineStampContext(desktop);
    const engines = ENGINE_BUILDS.map(({ crateDirectory, destination }) => ({ name: path.basename(destination), ...checkEngine({ projectRoot: project, crateDirectory, outDir: destination, context }) }));
    const stale = engines.filter((engine) => engine.action !== "current");
    if (stale.length) throw new Error(`perf build: stale or missing WebAssembly engines (${stale.map((engine) => `${engine.name}: ${engine.reason}`).join("; ")}); run \`npm run build:wasm\` in apps/desktop first`);
    const run = async (command, args, cwd, env = {}, stdio = ["ignore", "inherit", "inherit"]) => {
      console.log(`$ (${path.relative(project, cwd) || "."}) ${command} ${args.join(" ")}`);
      const result = await runUnderLock(command, args, { cwd, stdio, env: { ...process.env, CARGO_NET_OFFLINE: "true", ...env } });
      if (result.error !== null || result.status !== 0) throw new Error(`perf build: ${command} ${args.join(" ")} failed (${result.error?.message ?? result.status ?? result.signal})`);
      return result.stdout;
    };
    await run("npm", ["run", "build"], desktop);
    const distDigest = treeDigest(path.join(desktop, "dist"));
    const cargo = ["build", "--release", "--locked", "--offline", "--manifest-path", path.join(tauriDir, "Cargo.toml"), "--message-format=json-render-diagnostics", "--features", "native-smoke,tauri/custom-protocol"];
    const stdout = await run("cargo", cargo, tauriDir, { CARGO_TARGET_DIR: targetDir, TAURI_CONFIG: SMOKE_TAURI_CONFIG }, ["ignore", "pipe", "inherit"]);
    const binary = appExecutableFromCargoMessages(stdout, path.join(tauriDir, "Cargo.toml"), BIN_NAME);
    if (treeDigest(path.join(desktop, "dist")) !== distDigest) throw new Error("perf build: dist/ changed while cargo built the app; nothing is bound");
    mkdirSync(appDir, { recursive: true });
    const copy = path.join(appDir, BIN_NAME);
    rmSync(copy, { force: true });
    copyFileSync(binary, copy);
    chmodSync(copy, 0o700);
    const binding = {
      schema: "swbpipe.desktop-perf-binding/v1",
      built_at: new Date().toISOString(),
      source_commit: git("rev-parse", "HEAD"),
      source_dirty: git("status", "--porcelain").length > 0,
      engines: Object.fromEntries(engines.map((engine) => [engine.name, engine.hash])),
      dist_digest: distDigest,
      cargo: ["cargo", ...cargo],
      tauri_config: SMOKE_TAURI_CONFIG,
      cargo_binary: binary,
      binary: { path: copy, bytes: statSync(copy).size, sha256: sha256File(copy) }
    };
    writeFileSync(path.join(appDir, BINDING), `${JSON.stringify(binding, null, 2)}\n`);
    console.log(`perf build: bound ${copy} (${binding.binary.sha256}; commit ${binding.source_commit}${binding.source_dirty ? ", dirty" : ""})`);
  } finally {
    lock.release();
  }
}

// ---------------------------------------------------------------------- run

function webkitPids() {
  const listed = spawnSync("ps", ["-axo", "pid=,comm="], { encoding: "utf8" }).stdout;
  return listed.split("\n").map((line) => line.trim().match(/^(\d+)\s+(.*)$/)).filter((m) => m && /com\.apple\.WebKit\./.test(m[2])).map((m) => Number(m[1]));
}

// Peaks per process kind from the sampler's lines, and a compact timeline.
export function summarizeSamples(samples, { since = null } = {}) {
  const kinds = {};
  const pids = {};
  const timeline = [];
  for (const sample of samples) {
    const sums = {};
    for (const [pid, kind, rss, footprint, lifetime] of sample.p) {
      const entry = (pids[pid] ??= { kind, first_ms: sample.t, last_ms: sample.t, peak_rss: 0, peak_footprint: 0, lifetime_max_footprint: null });
      entry.last_ms = sample.t;
      if (footprint === null) continue;
      entry.peak_rss = Math.max(entry.peak_rss, rss);
      entry.peak_footprint = Math.max(entry.peak_footprint, footprint);
      entry.lifetime_max_footprint = lifetime;
      const sum = (sums[kind] ??= { rss: 0, footprint: 0 });
      sum.rss += rss;
      sum.footprint += footprint;
    }
    for (const [kind, sum] of Object.entries(sums)) {
      const total = (kinds[kind] ??= { peak_rss: 0, peak_footprint: 0, peak_footprint_since: null });
      total.peak_rss = Math.max(total.peak_rss, sum.rss);
      total.peak_footprint = Math.max(total.peak_footprint, sum.footprint);
      if (since !== null && sample.t >= since) total.peak_footprint_since = Math.max(total.peak_footprint_since ?? 0, sum.footprint);
    }
    timeline.push([sample.t, sums.app?.footprint ?? null, sums.app?.rss ?? null, sums.webcontent?.footprint ?? null, sums.webcontent?.rss ?? null, sums.worker?.footprint ?? null]);
  }
  return { kinds, pids, timeline_columns: ["epoch_ms", "app_footprint", "app_rss", "webcontent_footprint", "webcontent_rss", "worker_footprint"], timeline };
}

/** Sequential PID reads from one sample, never a sum of independent maxima.
 * Missing workers count as zero only after the product's job_done mark; that
 * zero is inferred from completion, not observed by a missing sampler row. */
export function c1PhysicalPhases(samples, profile, workerEndedMs) {
  const peak = rows => {
    let value = null, unknown = 0, inferredWorkerZero = 0;
    for (const row of rows) {
      const app = row.p.filter(([,kind]) => kind === "app");
      const web = row.p.filter(([,kind]) => kind === "webcontent");
      const workers = row.p.filter(([,kind]) => kind === "worker");
      const pids = [...app, ...web, ...workers];
      if (app.length !== 1 || web.length !== 1 || pids.some(([, , ,footprint]) => !Number.isFinite(footprint) || footprint < 0) || (!workers.length && (!Number.isFinite(workerEndedMs) || row.t < workerEndedMs))) { unknown++; continue; }
      if (!workers.length) inferredWorkerZero++;
      const sum = pids.reduce((total,[,,,footprint]) => total + footprint, 0);
      if (value === null || sum > value.bytes) value = { epoch_ms: row.t, bytes: sum, pids: pids.map(([pid]) => pid), worker_zero_inferred: !workers.length };
    }
    return { aligned_sampled_peak: value, unknown_samples: unknown, completed_worker_zero_samples: inferredWorkerZero, samples: rows.length };
  };
  const start = profile?.driver_started_ms, end = profile?.driver_finished_ms;
  return { all: peak(samples), before_reader_probe: Number.isFinite(start) ? peak(samples.filter(row => row.t < start)) : null,
    reader_probe: Number.isFinite(start) && Number.isFinite(end) ? peak(samples.filter(row => row.t >= start && row.t <= end)) : null,
    after_reader_probe: Number.isFinite(end) ? peak(samples.filter(row => row.t > end)) : null,
    interpretation: "Sequential reads aligned to one sample, not an atomic concurrent maximum; missing readings stay unknown. Worker absence is zero only by the product completion mark. Later phases can retain profiling allocator caches; lifetime per-PID maxima are not phase-local." };
}

function inspectStore(home) {
  const dir = path.join(home, "Library", "Application Support", TAURI_CONF.identifier);
  const file = path.join(dir, "openpipestress-projects.sqlite3");
  const out = { path_exists: existsSync(file), files: existsSync(dir) ? readdirSync(dir).map((name) => ({ name, bytes: statSync(path.join(dir, name)).size })) : [] };
  if (!out.path_exists) return out;
  return import("node:sqlite").then(({ DatabaseSync }) => {
    const db = new DatabaseSync(file, { readOnly: true });
    try {
      out.quick_check = db.prepare("PRAGMA quick_check").all().map((row) => Object.values(row)[0]);
      out.tables = {};
      for (const { name } of db.prepare("SELECT name FROM sqlite_master WHERE type = 'table'").all()) {
        const quoted = `"${name.replaceAll('"', '""')}"`;
        const columns = db.prepare(`PRAGMA table_info(${quoted})`).all().map((column) => column.name).filter((column) => column.endsWith("_json"));
        const lengths = columns.map((column) => `max(length("${column}")) AS "${column}"`).join(", ");
        out.tables[name] = db.prepare(`SELECT count(*) AS rows${lengths ? `, ${lengths}` : ""} FROM ${quoted}`).get();
      }
      if (out.tables.local_projects) {
        out.projects = db.prepare("SELECT project_id, length(model_json) AS model_chars, length(mechanics_result_json) AS result_chars, typeof(mechanics_result_json) AS result_type FROM local_projects").all();
      }
    } catch (error) {
      out.error = String(error?.message ?? error);
    } finally {
      db.close();
    }
    return out;
  }, (error) => ({ ...out, error: String(error?.message ?? error) }));
}

/** Verify the native receipt and complete CSV rows without materializing the
 * file in the webview or this Node process. Standard-library csv handles quoted
 * newlines; inspection runs only after the App exits, within the runner deadline. */
export async function inspectCsv(home, exported) {
  try {
    const name = exported?.receipt?.file_name;
    if (typeof name !== "string" || path.basename(name) !== name || !name.endsWith(".csv")) throw new Error("CSV receipt is not a bare CSV name");
    const downloads = path.join(home, "Downloads");
    const file = path.join(downloads, name);
    if (!lstatSync(file).isFile() || lstatSync(file).isSymbolicLink() || path.dirname(realpathSync(file)) !== realpathSync(downloads)) throw new Error("CSV receipt leaves the private Downloads directory");
    const bytes = statSync(file).size;
    if (bytes !== exported.receipt.byte_count) throw new Error("CSV receipt byte count differs from the file");
    const script = `
import csv,hashlib,json,sys
csv.field_size_limit(64<<20)
digest=hashlib.sha256()
with open(sys.argv[1],'rb') as raw:
 if raw.read(3)!=b'\\xef\\xbb\\xbf': raise ValueError('CSV has no UTF-8 BOM')
 raw.seek(0)
 while True:
  chunk=raw.read(1<<20)
  if not chunk: break
  digest.update(chunk)
with open(sys.argv[1],encoding='utf-8-sig',newline='') as source:
 rows=csv.reader(source,strict=True)
 preamble={}
 for row in rows:
  if not row: break
  if len(row)!=2: raise ValueError('invalid CSV preamble')
  preamble[row[0]]=row[1]
 header=next(rows)
 if not header: raise ValueError('CSV has no columns')
 count=0
 for row in rows:
  if len(row)!=len(header): raise ValueError('CSV data width differs from its header')
  count+=1
 if preamble.get('export_context')!='local_private' or preamble.get('released_report')!='false': raise ValueError('CSV boundary preamble differs')
 print(json.dumps({'rows':count,'columns':len(header),'sha256':digest.hexdigest()}))
`;
    const read = await new Promise((resolve, reject) => {
      execFile("python3", ["-I", "-c", script, file], { timeout: 30000, killSignal: "SIGKILL", maxBuffer: 64 * 1024 }, (error, stdout) => {
        if (error) reject(error); else { try { resolve(JSON.parse(stdout)); } catch (invalid) { reject(invalid); } }
      });
    });
    return { ...read, bytes, expected_rows: exported.expected_rows, verified: read.rows === exported.expected_rows };
  } catch (error) { return { verified: false, error: String(error.stderr ?? error.message).slice(-1000) }; }
}

const markAt = (timeline, stage) => {
  const mark = timeline?.marks?.find((item) => item.stage === stage);
  return mark ? timeline.time_origin_ms + mark.at_ms : null;
};

// The run's findings and its most severe class.
export function classify({ report, exit, timedOut, progress, memory, store, run, resultBytes }) {
  const findings = [];
  const add = (kind, detail) => findings.push({ class: kind, ...detail });
  const webcontentExited = Object.entries(memory.pids).filter(([, entry]) => entry.kind === "webcontent" && exit.ended_ms !== null && entry.last_ms < exit.ended_ms - 1000);
  for (const [pid, entry] of webcontentExited) add("crash", { process: "webcontent", pid: Number(pid), detail: `WebContent ${pid} stopped being sampled ${exit.ended_ms - entry.last_ms} ms before the app ended` });
  if (!report) {
    if (timedOut) add("hang", { process: "app", detail: `no report within ${run.timeout_s} s; last progress: ${JSON.stringify(progress.at(-1) ?? null)}` });
    else add("crash", { process: "app", detail: `the app exited (${exit.status ?? exit.signal}) without a report; last progress: ${JSON.stringify(progress.at(-1) ?? null)}` });
  } else {
    const solve = report.solve;
    if (!solve) add(report.error ? "refused_with_message" : "hang", { step: "open", message: report.error ?? null, needs_review: true });
    else if (solve.ended === "first_paint") { /* painted */ }
    else if (solve.ended === "timeout") add("hang", { step: "solve", detail: `no first paint within ${run.paint_ms} ms; last stage ${solve.timeline?.marks?.at(-1)?.stage ?? null}` });
    // The window's text, verbatim (the timeline carries only the failure's code and stage).
    else add("refused_with_message", { step: "solve", message: solve.shown?.error ?? solve.shown?.refusal ?? solve.shown?.summary ?? null, code: solve.timeline?.failure?.code ?? null, stage: solve.timeline?.failure?.stage ?? null, needs_review: true });
    const record = report.record_run;
    if (record?.timed_out) add("hang", { step: "record_run", detail: `last message: ${record.last_message}` });
    else if (record && record.message && /^(Not recorded|Record Run:)/.test(record.message)) add("refused_with_message", { step: "record_run", message: record.message, needs_review: true });
    const save = report.save_project;
    if (save?.timed_out) add("hang", { step: "save_project", detail: `last message: ${save.last_message}` });
    else if (save && save.message && !/^(Saved|Created)|SQLite project snapshot/.test(save.message)) add("refused_with_message", { step: "save_project", message: save.message, needs_review: true });
    if (report.error && solve) add("refused_with_message", { step: "driver", message: report.error.split("\n")[0], needs_review: true });
    if (run.c1_profile_readers && !report.c1?.reader_allocations) add("refused_with_message", { step: "reader_allocations", message: "Reader allocation evidence is incomplete.", needs_review: true });
    const omitted = [report.c1?.trace?.save?.result, report.c1?.trace?.restoredSave?.result].find(result => result?.outcome === "not_saved");
    if (omitted) add("refused_with_message", { step: "save_result", message: omitted.code ?? "The native save kept only the model.", needs_review: true });
    if (run.c1_probes && report.c1?.completed !== true) add("refused_with_message", { step: "c1_probes", message: "The connected C1 usability probes did not complete.", needs_review: true });
    if (run.c1_probes && report.c1?.completed && report.c1.csv_file?.verified !== true) {
      if (report.c1.csv_file?.error) add("refused_with_message", { step: "csv_inspection", message: report.c1.csv_file.error, needs_review: true });
      else add("misleading_message", { step: "csv", detail: "CSV rows do not match the native query total." });
    }
    // What the store holds against what the window said.
    const stored = store?.projects?.find((row) => row.project_id === run.project_id);
    const claimedSaved = Boolean(save?.message && /^(Saved|Created)|SQLite project snapshot/.test(save.message)) || Boolean(record?.attempted && record.message && !/^Not recorded/.test(record.message));
    if (store?.quick_check && store.quick_check.join(",") !== "ok") add("half_written_save", { detail: `quick_check: ${store.quick_check.slice(0, 3).join("; ")}` });
    if ((store?.files ?? []).some((file) => /-journal$|-wal$/.test(file.name) && file.bytes > 0)) add("half_written_save", { detail: `left over: ${store.files.filter((file) => /-journal$|-wal$/.test(file.name)).map((file) => `${file.name} (${file.bytes} bytes)`).join(", ")}` });
    if (claimedSaved && solve?.ended === "first_paint" && (!stored || stored.result_type !== "text")) add("misleading_message", { detail: `the window reported a save or a recorded run, but the store holds ${stored ? `a ${stored.result_type} result` : "no project row"} for ${run.project_id}` });
    // Only a session that held the result (painted) can be judged against its size.
    if (!omitted && solve?.ended === "first_paint" && stored && stored.result_type === "text" && resultBytes && stored.result_chars < resultBytes / 2) add("half_written_save", { detail: `the stored result has ${stored.result_chars} characters, the solve's result ${resultBytes} bytes` });
  }
  if (findings.length === 0) add("ok", {});
  const worst = SEVERITY.find((kind) => findings.some((finding) => finding.class === kind));
  return { class: worst, findings };
}

async function runOne(run, ctx) {
  const record = { id: run.id, mode: run.mode, model: run.model, binding: ctx.binding, harness: ctx.harness, started_at: new Date().toISOString() };
  // Every plan-named source and destination stays inside its root (validatePlan, containedPath).
  let modelSource, resultFile;
  try {
    modelSource = containedPath(ctx.models, run.model, { mustExist: true });
    resultFile = run.mode === "injected" ? containedPath(ctx.results, run.result, { mustExist: true }) : null;
  } catch (error) {
    return { ...record, class: "skipped", detail: error.message };
  }
  const root = mkdtempSync(path.join(ctx.scratch, `run-${planName(run.id, "run id")}-`));
  const home = path.join(root, "home");
  mkdirSync(path.join(home, "Downloads"), { recursive: true });
  const modelPath = containedPath(root, run.model);
  copyFileSync(modelSource, modelPath, constants.COPYFILE_EXCL);
  const document = JSON.parse(readFileSync(modelPath, "utf8"));
  run = { ...run, project_id: document.project.id, c1_probes: ctx.plan.c1_probes === true, c1_profile_readers: ctx.plan.c1_profile_readers === true };
  record.project_id = run.project_id;
  record.model_bytes = statSync(modelPath).size;
  record.model_sha256 = sha256File(modelPath);
  record.model_jcs_sha256 = `sha256:${createHash("sha256").update(canonical(document), "utf8").digest("hex")}`;
  if (run.c1_probes && (record.model_sha256 !== run.model_sha256 || !Array.isArray(document.nodes) || document.nodes.length !== run.nodes)) {
    return { ...record, class: "refused_with_message", findings: [{ class: "refused_with_message", step: "fixture", message: "C1 fixture bytes/node count differ from the selected 500/1000-node binding; no App was launched.", needs_review: true }], kept: root };
  }
  if (resultFile) record.injected_result = { file: path.basename(resultFile), bytes: statSync(resultFile).size };
  const timeoutMs = Math.min(run.timeout_s * 1000, ctx.remainingMs());
  const plan = {
    run_id: run.id, model_path: modelPath, project_id: run.project_id, record_run: true, save_project: true,
    c1_probes: ctx.plan.c1_probes === true, c1_profile_readers: ctx.plan.c1_profile_readers === true, model_jcs_sha256: record.model_jcs_sha256,
    timeouts: { ...ctx.plan.timeouts, paint_ms: Math.min(run.paint_ms, timeoutMs) }
  };
  const env = { ...process.env, HOME: home, SWBPIPE_NATIVE_SMOKE_DRIVER: path.join(here, "perf-driver.js"), SWBPIPE_NATIVE_SMOKE_PLAN: JSON.stringify(plan), SWBPIPE_NATIVE_SMOKE_REPORT: path.join(root, "report.json") };
  delete env[INJECT_ENV];
  delete env[INJECT_ROOT_ENV];
  if (resultFile) {
    env[INJECT_ENV] = resultFile;
    env[INJECT_ROOT_ENV] = realpathSync(ctx.results);
  }
  const baseline = webkitPids();
  const startedMs = Date.now();
  // Inherit the supervised runner's group. A detached App could outlive the
  // supervisor's deadline when its runner is blocked in synchronous SQLite.
  const app = spawn(ctx.binary, [], { cwd: root, env, stdio: ["ignore", "ignore", "pipe"] });
  audit({ type: "app", pid: app.pid });
  let appExited = false;
  app.once("exit", () => { appExited = true; audit({ type: "app_exit", pid: app.pid }); });
  const caffeinate = spawn("/usr/bin/caffeinate", ["-di", "-w", String(app.pid)], { stdio: "ignore" });
  record.samples_file = path.join(ctx.out, `perf-samples-${planName(path.basename(ctx.scratch), "window")}-${planName(run.id, "run")}.jsonl`);
  const sampler = spawn("python3", [path.join(here, "perf-sampler.py"), "--app-pid", String(app.pid), "--app-path", ctx.binary, "--baseline", baseline.join(","), "--interval-ms", String(ctx.plan.sample_interval_ms), "--out", record.samples_file], { stdio: ["ignore", "pipe", "inherit"] });
  let lastSample = Date.now();
  const webcontentSeen = new Set();
  const sampleGuard = createSampleGuard();
  const staleSampler = setInterval(() => {
    if (!appExited && Date.now() - lastSample > 3000) audit({ type: "stop", reason: "sampler_stale" });
  }, 1000);
  sampler.on("error", () => audit({ type: "stop", reason: "sampler_spawn_failed" }));
  sampler.once("exit", () => {
    if (!appExited) audit({ type: "stop", reason: "sampler_exited_before_app" });
  });
  const samples = [];
  let samplerBuffer = "";
  sampler.stdout.on("data", (chunk) => {
    samplerBuffer += chunk;
    let index;
    while ((index = samplerBuffer.indexOf("\n")) >= 0) {
      const line = samplerBuffer.slice(0, index);
      samplerBuffer = samplerBuffer.slice(index + 1);
      try {
        const value = JSON.parse(line);
        if (Array.isArray(value.p)) {
          lastSample = Date.now();
          samples.push(value);
          const compact = compactSample(value);
          audit({ type: "sample", sample: compact });
          if (!appExited) {
            for (const [pid, kind] of value.p) if (kind === "webcontent") webcontentSeen.add(pid);
            const reason = sampleGuard(value);
            if (reason || webcontentSeen.size > 1) audit({ type: "stop", reason: reason ?? "webcontent_replaced", sample: compact });
          }
        }
      } catch { /* not a sample */ }
    }
  });
  const progress = [];
  const stderrTail = [];
  let stderrBuffer = "";
  app.stderr.on("data", (chunk) => {
    stderrBuffer += chunk;
    let index;
    while ((index = stderrBuffer.indexOf("\n")) >= 0) {
      const line = stderrBuffer.slice(0, index);
      stderrBuffer = stderrBuffer.slice(index + 1);
      const perf = line.match(/^\[native-smoke\] perf:(.*)$/);
      if (perf) { try {
        const event = JSON.parse(perf[1]); progress.push(event);
        audit({ type: "stage", step: event.step, stage: event.stage ?? null });
        console.log(`  ${run.id}: ${perf[1]}`);
      } catch { /* partial */ } continue; }
      stderrTail.push(line.slice(0, 2000));
      if (stderrTail.length > 200) stderrTail.shift();
    }
  });
  let timedOut = false;
  const exit = await new Promise((resolve) => {
    const timer = setTimeout(() => {
      timedOut = true;
      audit({ type: "stop", reason: "app_timeout" });
      try { app.kill("SIGKILL"); } catch { /* gone */ }
    }, timeoutMs);
    app.on("exit", (status, signal) => { clearTimeout(timer); resolve({ status, signal, ended_ms: Date.now() }); });
    app.on("error", (error) => { clearTimeout(timer); audit({ type: "stop", reason: `app_spawn_failed: ${error.message}` }); resolve({ status: null, signal: null, ended_ms: Date.now() }); });
  });
  clearInterval(staleSampler);
  caffeinate.kill();
  await new Promise((resolve) => {
    if (sampler.exitCode !== null || sampler.signalCode !== null) { resolve(); return; }
    const fallback = setTimeout(() => sampler.kill("SIGTERM"), 5000);
    sampler.on("exit", () => { clearTimeout(fallback); resolve(); });
  });
  record.elapsed_s = (exit.ended_ms - startedMs) / 1000;
  record.exit = { ...exit, timed_out: timedOut, timeout_ms: timeoutMs };
  const reportPath = path.join(root, "report.json");
  const report = existsSync(reportPath) ? JSON.parse(readFileSync(reportPath, "utf8")) : null;
  const jobDone = markAt(report?.solve?.timeline, "job_done");
  const memory = summarizeSamples(samples, { since: jobDone });
  const webcontent = Object.entries(memory.pids).filter(([, entry]) => entry.kind === "webcontent").map(([pid]) => Number(pid));
  memory.webcontent_identification = {
    method: "WebKit WebContent processes (by executable path) not running before the launch; WebKit starts them from launchd, not as the app's children",
    candidates: webcontent,
    ambiguous: webcontent.length > 1
  };
  memory.peak_footprint_since = jobDone === null ? null : "job_done";
  checkpointRun({ ...record, report, progress, memory, kept: root, store: { inspection: "pending" } }, ctx);
  audit({ type: "stage", step: "inspect_store", stage: "started" });
  const store = await inspectStore(home);
  audit({ type: "stage", step: "inspect_store", stage: "completed" });
  if (run.c1_probes && report?.c1?.fresh?.csv) report.c1.csv_file = await inspectCsv(home, report.c1.fresh.csv);
  const proof = report?.solve?.shown?.proof ?? null;
  const proofField = (name) => proof?.match(new RegExp(`(?:^|; )${name}=([^;]*)`))?.[1] ?? null;
  record.model_binding = {
    file_jcs_sha256: record.model_jcs_sha256,
    proof_model_sha256: proofField("model_sha256"),
    proof_identity_match: proof ? proof.includes("identity=match") : null,
    // Unknown (null) without a solve proof: the proof is shown only for a published result.
    same_model: proof ? proofField("model_sha256") === record.model_jcs_sha256 : null
  };
  const resultBytes = report?.solve?.timeline?.marks?.find((mark) => mark.stage === "job_done")?.detail?.result_bytes ?? null;
  Object.assign(record, classify({ report, exit, timedOut, progress, memory, store, run, resultBytes }));
  if (run.c1_probes) record.c1_physical = c1PhysicalPhases(samples, report?.c1?.reader_allocations, jobDone);
  Object.assign(record, { report, progress, memory, store, stderr_tail: stderrTail.slice(-40) });
  record.kept = root; // Supervisor removes only after all owned processes exit.
  console.log(`${run.id}: ${record.class} in ${record.elapsed_s.toFixed(1)} s${record.findings.filter((f) => f.class !== "ok").map((f) => `; ${f.class}: ${f.message ?? f.detail ?? ""}`).join("")}`);
  return record;
}

/** The run's scratch at the end: removed whole, or with --keep only the app copy is
 * removed and every run's directory (`kept` in its record) stays. */
export async function finishScratch({ scratch, binary, keep }) {
  if (keep) await rm(binary, { force: true });
  else await rm(scratch, { recursive: true, force: true });
}

/** Preserve the exited App's evidence before an inspection can block. This
 * checkpoint cannot pass a gate and is never announced as a completed run. */
export function checkpointRun(record, { out, scratch }) {
  const file = path.join(out, `perf-partial-${planName(path.basename(scratch), "window")}-${planName(record.id, "run")}.json`);
  const temporary = `${file}.tmp`;
  const partial = { ...record, class: "refused_with_message", evidence_complete: false,
    findings: [{ class: "refused_with_message", step: "inspection", message: "Post-App store/CSV inspection is incomplete; this is a checkpoint, not a successful run.", needs_review: true }] };
  writeFileSync(temporary, `${JSON.stringify(partial, null, 1)}\n`, { flag: "wx", mode: 0o600 });
  renameSync(temporary, file);
  audit({ type: "stage", step: "inspection_checkpoint", stage: "committed", path: file });
  return file;
}

/** Commit a finished run before another App can start. Samples live outside
 * scratch; only an ok run's App-written HOME is removed, after that commit. */
export async function commitRun(record, { out, scratch, keep }) {
  const file = path.join(out, `perf-run-${planName(path.basename(scratch), "window")}-${planName(record.id, "run")}.json`);
  record.record_file = file;
  const persist = () => {
    const temporary = `${file}.tmp`;
    writeFileSync(temporary, `${JSON.stringify(record, null, 1)}\n`, { flag: "wx", mode: 0o600 });
    renameSync(temporary, file);
  };
  persist();
  // The final record is now durable; the deliberately incomplete checkpoint
  // has no remaining consumer. Its name is derived from this window/run only.
  await rm(path.join(out, `perf-partial-${planName(path.basename(scratch), "window")}-${planName(record.id, "run")}.json`), { force: true });
  audit({ type: "completed_run", path: file });
  if (record.class === "ok" && !keep && record.kept) {
    if (path.dirname(record.kept) !== scratch || !path.basename(record.kept).startsWith("run-")) throw new Error("perf cleanup: not this window's run root");
    record.home_cleanup = "pending";
    persist();
    try {
      await rm(path.join(record.kept, "home"), { recursive: true, force: true });
      record.home_cleanup = "removed";
    } catch (error) {
      record.home_cleanup = "failed";
      record.home_cleanup_error = error.message;
      record.class = "refused_with_message";
      (record.findings ??= []).push({ class: "refused_with_message", step: "home_cleanup", message: error.message, needs_review: true });
    }
    persist();
  }
}

async function runPlan(options) {
  if (!process.send) throw new Error("perf run: only the supervised --run path can start the App");
  const binding = JSON.parse(readFileSync(path.join(options.appDir, BINDING), "utf8"));
  const bound = binding.binary.path;
  if (sha256File(bound) !== binding.binary.sha256) throw new Error(`perf run: ${bound} is not the bound binary (${binding.binary.sha256}); run --build again`);
  const plan = loadPlan(options.planFile);
  const runs = selectRuns(plan, options);
  const budgetMs = (options.budgetS ?? plan.budget_s) * 1000;
  mkdirSync(options.out, { recursive: true });
  const scratch = options.windowRoot;
  if (!scratch || path.dirname(scratch) !== options.out || !path.basename(scratch).startsWith("perf-")) throw new Error("perf run-child: no private window root");
  audit({ type: "scratch", path: scratch });
  // A private copy, checked against the binding, is what each launch runs.
  const binary = path.join(scratch, BIN_NAME);
  copyFileSync(bound, binary);
  chmodSync(binary, 0o700);
  if (sha256File(binary) !== binding.binary.sha256) throw new Error("perf run: the private copy differs from the bound binary");
  const started = Date.now();
  const harness = {
    source_commit: git("rev-parse", "HEAD"), source_dirty: git("status", "--porcelain").length > 0,
    files_sha256: Object.fromEntries(["perf.mjs", "perf-driver.js", "perf-sampler.py"].map(name => [name, sha256File(path.join(here, name))])),
    plan_sha256: sha256File(options.planFile),
    rack_generator_sha256: sha256File(path.join(project, "core/solver/performance_harness/rack/rack_model.py"))
  };
  const ctx = { plan, binding, harness, binary, models: options.models, results: options.results ?? options.models, out: options.out, scratch, keep: options.keep, remainingMs: () => budgetMs - (Date.now() - started) };
  process.stdout.write(formatPlan(runs, budgetMs / 1000));
  const records = [];
  for (const run of runs) {
    if (run.estimate_s * 1000 > ctx.remainingMs()) {
      const skipped = { binding, harness, id: run.id, mode: run.mode, model: run.model, class: "skipped", detail: `estimate ${run.estimate_s} s exceeds the remaining budget ${Math.round(ctx.remainingMs() / 1000)} s` };
      await commitRun(skipped, ctx);
      records.push(skipped);
      console.log(`${run.id}: skipped (window budget)`);
      continue;
    }
    const record = await runOne(run, ctx);
    await commitRun(record, ctx);
    records.push(record);
    if (records.at(-1).class !== "ok") break;
  }
  const report = {
    schema: "swbpipe.desktop-perf-report/v1",
    written_at: new Date().toISOString(),
    host: { platform: process.platform, release: os.release(), cpus: os.cpus().length, cpu_model: os.cpus()[0]?.model ?? null, memory_bytes: os.totalmem() },
    binding, harness,
    plan: { file: path.relative(project, options.planFile), subset: options.runs ? null : options.subset ?? "window", runs: runs.map((run) => run.id), budget_s: budgetMs / 1000 },
    conditions: [
      "Observation only: no time or memory bound is asserted, and criterion 9's provisional bounds are not evaluated.",
      "A release build with the native-smoke feature (product code plus the smoke harness and the result injection); background throttling disabled, as the native smoke.",
      "Injected runs skip the solve worker: no solve runs, worker_peak_footprint_bytes is unset, and d_M, d_S and their parsed values are computed in the app process before publication (so the app's peak before job_done includes them; peak_footprint_since counts from job_done).",
      "WebContent is identified as the WebKit WebContent processes started after the launch; another WebKit client starting one during the run would be counted (memory.webcontent_identification).",
      "One run per app launch, against a throwaway HOME; the host is the one in `host`, not necessarily the 16 GiB base machine.",
      "C1 probe completion, when selected, covers its DOM/native-command/storage/CSV interactions only; it does not establish reader working-peak attribution, memory fit, base-machine usability or acceptance."
    ],
    elapsed_s: (Date.now() - started) / 1000,
    runs: records
  };
  const file = path.join(options.out, `perf-report-${new Date().toISOString().replace(/[:.]/g, "").slice(0, 15)}Z.json`);
  writeFileSync(file, `${JSON.stringify(report, null, 1)}\n`);
  if (records.some((record) => record.class !== "ok")) process.exitCode = 1;
  console.log(`perf run: ${records.length} runs in ${report.elapsed_s.toFixed(0)} s; report ${file}`);
  for (const record of records) console.log(`  ${record.id}: ${record.class}`);
}

async function main() {
  if (process.platform !== "darwin") throw new Error("perf: macOS only");
  const { values } = parseArgs({
    options: {
      build: { type: "boolean" }, plan: { type: "boolean" }, run: { type: "boolean" }, "run-child": { type: "boolean" },
      "app-dir": { type: "string" }, "target-dir": { type: "string" }, "plan-file": { type: "string" },
      subset: { type: "string" }, runs: { type: "string" }, models: { type: "string" }, results: { type: "string" },
      out: { type: "string" }, "budget-s": { type: "string" }, keep: { type: "boolean" }, "window-root": { type: "string" }, "supervisor-pid": { type: "string" }
    }
  });
  const planFile = path.resolve(values["plan-file"] ?? path.join(here, "perf-plan.json"));
  const need = (name) => { if (!values[name]) throw new Error(`perf: --${name} is required`); return path.resolve(values[name]); };
  if (values.build) return build({ appDir: need("app-dir"), targetDir: values["target-dir"] ?? process.env.CARGO_TARGET_DIR });
  if (values.plan) {
    const plan = loadPlan(planFile);
    return process.stdout.write(formatPlan(selectRuns(plan, values), plan.budget_s));
  }
  if (values.run || values["run-child"]) {
    const options = {
      appDir: need("app-dir"), models: need("models"), results: values.results ? path.resolve(values.results) : null, out: need("out"),
      planFile, subset: values.subset, runs: values.runs, budgetS: values["budget-s"] ? Number(values["budget-s"]) : null, keep: Boolean(values.keep),
      windowRoot: values["window-root"] ? path.resolve(values["window-root"]) : null
    };
    if (values["run-child"]) {
      if (!process.send) throw new Error("perf run-child: no parent supervisor");
      return runChild(() => runPlan(options), Number(values["supervisor-pid"]));
    }
    return supervisedRun(options);
  }
  throw new Error("perf: give --build, --plan or --run (see the header of perf.mjs)");
}

if (!isMainThread && Number.isSafeInteger(workerData?.perfParentPid)) {
  const check = () => { if (process.ppid !== workerData.perfParentPid) killOwnRunnerGroup(); };
  check();
  parentPort.postMessage("armed");
  setInterval(check, 100);
} else if (isMainThread && process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main().catch((error) => { console.error(error?.message ?? error); process.exit(2); });
}
