#!/usr/bin/env node
// DO NOT use this runner to launch the application (--run). Its launch hold stands until Agent 0 lifts it, after the independent launch-safety reading of this launch path (the allowlisted environment, the throwaway home and window marker, the census and the stop) and the host's quieting (issue #360). Nothing in this file, and no passing test, lifts it.
// Desktop performance runner (RT-U4, release target exit criterion 9;
// DEL-04-05), macOS. Observation only:
// nothing here asserts a time or memory bound.
//
//   node apps/desktop/native-smoke/perf.mjs --build --app-dir DIR [--target-dir DIR]
//   node apps/desktop/native-smoke/perf.mjs --plan [--plan-file F] [--subset S]
//   node apps/desktop/native-smoke/perf.mjs --run --app-dir DIR --models DIR [--results DIR]
//        --out DIR [--plan-file F] [--subset S | --runs id] [--budget-s N] [--keep]
//
// --build (under the host slot, which the caller takes first; it runs cargo itself):
//   under the worktree build lock, refuses a stale WebAssembly engine (run `npm run
//   build:wasm` first), builds the frontend (`npm run build`) and a RELEASE build of the
//   app with `--features native-smoke,tauri/custom-protocol` and the smoke's
//   TAURI_CONFIG (`smokeWindows`, as `run.mjs`: background throttling off and a
//   non-persistent web view store), into --target-dir (default
//   CARGO_TARGET_DIR; one is required). The executable cargo reports is copied to
//   DIR/openpipestress-desktop and bound in DIR/perf-binding.json (SHA-256, source
//   commit and dirty state, engine stamps, dist digest, cargo command). `run.mjs`
//   builds debug into src-tauri/target; timings need a release build, hence this one.
//
// --run: one window holds exactly one launch. A selection (--subset or --runs) naming
//   any other number of runs is refused (PERF-ONE-LAUNCH-PER-WINDOW), as is a binding
//   whose recorded smoke window configuration lacks the non-persistent store or the
//   throttling switch (PERF-BINDING-REFUSED: build again), before any state is made. The
//   window is then admitted only by creating the window marker (WINDOW_MARKER) atomically:
//   a marker already there, left by a window running, interrupted, killed outright or
//   whose teardown or cleanup was not established, refuses it (EARLIER-WINDOW-UNRESOLVED)
//   and is never cleared automatically. The window launches a private copy of the bound
//   app (its SHA-256 checked) once, against a throwaway home created empty
//   (`createThrowawayHome`) with `perf-driver.js` as the native-smoke driver. The app's
//   environment is built from nothing (`buildLaunchEnv`; `refusedPerfLaunchEnv` checks the
//   whole of it before the start): HOME and CFFIXED_USER_HOME the home, TMPDIR inside it,
//   the strict chooser on, the driver, plan and report, and only the variables the run
//   selects (PERF_RUNNER_KEYS). The model the driver's chooser opens is copied inside the
//   home, and every plan path must resolve there (`refusedPlanPaths`); the report and the
//   samples are runner outputs outside it. The run is one of:
//     - mode `native`: the app's own solve worker solves the model;
//     - mode `injected`: SWBPIPE_NATIVE_SMOKE_INJECT_RESULT names the run's result file
//       (`release_timing --write` from the same model document), which the started job
//       publishes instead of starting its worker (`src-tauri/src/native_smoke_inject.rs`).
//   It samples the app, its solve workers and the WebKit processes started for it
//   (`perf-sampler.py`), waits for the app to finish or the run's timeout, takes the
//   census of the private copy (no process still runs it; after the sampler has stopped,
//   outside the timed span), inspects the SQLite store and classifies the run. An
//   unresolved census is a `launch_failed` run named LAUNCH-CENSUS-UNRESOLVED: its record
//   is committed without inspecting the store, and no further run starts in the window.
//   The census covers the private executable only: not the WebKit helper processes
//   launchd starts for the app, and not the host between windows. An independent --run supervisor bounds
//   its internal --run-child, including synchronous inspection. Raw sampler lines
//   and completed-run records are persisted before advancing; forced-stop receipts
//   explicitly describe partial evidence. The supervisor writes the window record first;
//   then, only when its teardown recorded the owned group and the owned executable's
//   processes gone, it removes an ok window's scratch (the home with it; with --keep only
//   the app copy) and its marker. A failed window's scratch is retained as evidence. A
//   teardown not established, or a cleanup that fails, retains the scratch, the home, the
//   private copy and the marker, which then names why, so that no next window starts.
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
// half_written_save, misleading_message, host_unobservable (C1: the host condition, a shown
// page delivering frames, was not established or was lost; nothing was observable, so it is
// no hang and no product finding, and it is not retried), hang (timeout),
// refused_with_message (quoted), ok. A message is quoted verbatim; `needs_review` marks one a person should judge.
import { execFile, fork, spawn, spawnSync } from "node:child_process";
import { rm, writeFile } from "node:fs/promises";
import { createHash, randomUUID } from "node:crypto";
import { chmodSync, constants, copyFileSync, existsSync, linkSync, lstatSync, mkdirSync, mkdtempSync, openSync, closeSync, fsyncSync, readdirSync, readFileSync, realpathSync, renameSync, rmSync, statSync, writeFileSync, writeSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { parseArgs } from "node:util";
import { isMainThread, parentPort, Worker, workerData } from "node:worker_threads";
import { awaitNoProcesses, buildLaunchEnv, censusOf, createThrowawayHome, readRunMarker, refusedPerfLaunchEnv, refusedPlanPaths, refusedSmokeWindows, smokeWindows } from "./launch-env.mjs";
import { processStartTime, shellQuote } from "../scripts/worktreeBuildLock.mjs";
import { launchFailures, newNonce, readReport, recordedCensusFailures, recordedOutcome } from "./launch-outcome.mjs";
import { marksCheck, marksRefuse } from "./marks-check.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const desktop = path.resolve(here, "..");
const project = path.resolve(desktop, "..", "..");
const tauriDir = path.join(desktop, "src-tauri");
const TAURI_CONF = JSON.parse(readFileSync(path.join(tauriDir, "tauri.conf.json"), "utf8"));
// The smoke build's windows, as `run.mjs`'s (`smokeWindows`): background throttling off and a non-persistent web
// view store. A binding whose recorded configuration lacks either is refused at --run (`refusedPerfBinding`).
const SMOKE_TAURI_CONFIG = JSON.stringify({ app: { windows: smokeWindows(TAURI_CONF.app.windows) } });
const BIN_NAME = "openpipestress-desktop";
const BINDING = "perf-binding.json";
const INJECT_ENV = "SWBPIPE_NATIVE_SMOKE_INJECT_RESULT";
// The directory the injected file must lie in (the hook refuses anything else).
const INJECT_ROOT_ENV = "SWBPIPE_NATIVE_SMOKE_INJECT_ROOT";
const READER_DIAGNOSTICS_ENV = "SWBPIPE_NATIVE_SMOKE_READER_DIAGNOSTICS";
const SEVERITY = ["crash", "launch_failed", "half_written_save", "misleading_message", "host_unobservable", "hang", "refused_with_message", "ok"];
const GIB = 1024 ** 3;
// The census helper's polling deadline. The 10 s target leaves room for sampler stop and stderr drain in the
// supervisor's post-run reserve (at most 30 s); it does not bound a synchronous lsof call. The separate supervisor
// may stop the runner before a per-run census record is written, leaving failed-window evidence; nothing passes.
const PERF_CENSUS_BOUND_MS = 10 * 1000;
// The asynchronous-answer timer allows this grace after the polling deadline; it cannot interrupt synchronous work.
const CENSUS_ANSWER_GRACE_MS = 2000;
export const CENSUS_UNRESOLVED_STOP = "LAUNCH-CENSUS-UNRESOLVED";
// A store not inspected because a process of the private copy may still be writing it (an unresolved census).
const STORE_NOT_INSPECTED = "not_made";

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
    // The App starts the runner announced (its `app` event with a PID): the window's actual launch count.
    let launchesStarted = 0;
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
        completed_run_files: completedRuns, launches_started: launchesStarted,
        evidence_complete: reason === null && code === 0 && reaped && live === false && workersVerified && owned.every(({pid}) => !pidLive(pid)) });
    };
    const message = (event) => {
      if (event?.type === "app" && Number.isSafeInteger(event.pid) && event.pid > 1) { activeAppPid = event.pid; launchesStarted++; }
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

// The window marker (git-ignored): it exists from a window's admission, before its scratch is made, until its teardown
// has recorded the owned group and the owned executable's processes gone and its cleanup has completed. It is
// created atomically (a complete record hard-linked to the path, which fails when one is there), so two invocations
// in this worktree cannot both be admitted, and a window that is running, whose supervisor was interrupted or killed
// outright, whose teardown was not established or whose cleanup failed leaves it for the next invocation to refuse by
// name (EARLIER-WINDOW-UNRESOLVED). Only the window that wrote it removes it (its token), and only at those
// established facts; a stale marker is never cleared automatically. It sees this worktree's windows only, not
// another worktree's and not the host between windows.
export const WINDOW_MARKER = path.join(desktop, "target", "perf-window.json");
export const WINDOW_UNRESOLVED = "EARLIER-WINDOW-UNRESOLVED";
export const ONE_LAUNCH_REFUSED = "PERF-ONE-LAUNCH-PER-WINDOW";
export const BINDING_REFUSED = "PERF-BINDING-REFUSED";
// What the window's teardown, its marker and the WebKit baseline do not establish (the window record and the report).
export const WINDOW_LIMITS = Object.freeze([
  "The window's teardown records whether the owned process group and the processes running the private app copy are gone; neither covers the WebKit helper processes launchd starts for the app, which are not its children.",
  "Nothing observes the host between windows: the window marker keeps a next window from starting until this one's teardown and cleanup are established, and says nothing about what the system runs in the gap.",
  "The census after the launch covers only the processes running the private app copy. The census helper is given a 10 s polling deadline; a separate asynchronous-answer timer allows 2 s more. Neither interrupts or bounds a synchronous lsof call. When such a call does not return, the supervisor's window deadline stops the runner, and the window keeps this failed-window evidence instead of a per-run census record or checkpoint.",
  "The WebKit baseline (the WebKit processes running before the launch) only lets the sampler attribute WebContent processes that appear after the launch to this run; it does not prove they belong to this app, does not end them, and says nothing about what remains after the app exits or between windows."
]);

const markerText = (record) => `${JSON.stringify(record, null, 2)}\n`;

/** Create the marker at `markerPath` holding `record`, atomically: true when created, false when one is there. */
export function acquireWindowMarker(markerPath, record) {
  mkdirSync(path.dirname(markerPath), { recursive: true });
  const temporary = `${markerPath}.${process.pid}.${randomUUID()}.tmp`;
  writeFileSync(temporary, markerText(record), { flag: "wx", mode: 0o600 });
  try { linkSync(temporary, markerPath); return true; }
  catch (error) { if (error.code === "EEXIST") return false; throw error; }
  finally { rmSync(temporary, { force: true }); }
}

/** Replace this window's marker (its `token`) with `fields` added; another's, or none, is left as found and refused. */
export function updateWindowMarker(markerPath, token, fields) {
  const current = readRunMarker(markerPath);
  if (current?.token !== token) throw new Error(`perf window: ${markerPath} is not this window's marker; it is left as found`);
  const temporary = `${markerPath}.${process.pid}.${randomUUID()}.tmp`;
  writeFileSync(temporary, markerText({ ...current, ...fields, updated: new Date().toISOString() }), { flag: "wx", mode: 0o600 });
  renameSync(temporary, markerPath);
}

/** Remove this window's marker (its `token`); another's, or none, is left as found and refused. */
export function releaseWindowMarker(markerPath, token) {
  if (readRunMarker(markerPath)?.token !== token) throw new Error(`perf window: ${markerPath} is not this window's marker; it is left as found`);
  rmSync(markerPath);
}

/** The refusal printed when a marker is there: what it records, whether its supervisor still runs (pid and start time;
 * informational, never a reason to clear it), a fresh census of its private copy, and the recovery steps. */
export function windowMarkerRefusal(markerPath, record, { startTime = processStartTime, census = censusOf } = {}) {
  const lines = [`perf window: ${WINDOW_UNRESOLVED}: ${markerPath} records a performance window that is running or whose teardown or cleanup was not established (interrupted, killed outright, teardown unresolved or cleanup failed); no window is admitted until it is resolved, and nothing was made or launched`];
  if (!record || record.unreadable) {
    lines.push("  the marker cannot be read; inspect it, confirm no perf window or app runs in this worktree, then remove it:", `  rm -f ${shellQuote(markerPath)}`);
    return lines;
  }
  const now = Number.isSafeInteger(record.pid) && record.pid > 1 ? startTime(record.pid) : null;
  const status = now === null ? "not running" : now === record.started ? "STILL RUNNING" : `running but started ${now}, not at the recorded time: the pid looks reused`;
  lines.push(`  supervisor: pid ${record.pid}, started ${record.started} (${status})`, `  state: ${record.state}${record.reason ? `; ${record.reason}` : ""}`);
  if (record.window_file) lines.push(`  window record: ${record.window_file}`);
  if (typeof record.executable === "string") {
    const found = census(record.executable);
    lines.push(`  window root: ${record.scratch}`, `  private app copy: ${record.executable}`,
      found.ok ? `  census of the private copy now: ${found.pids.length === 0 ? "no process" : `process ${found.pids.join(", ")}`}` : `  census of the private copy now: could not be taken (${found.reason})`);
  } else lines.push("  the window made no scratch (it was refused or stopped before its private copy)");
  lines.push("  The census covers the private copy only, not the WebKit helper processes launchd starts, and not the host between windows.",
    "Once no process runs the private copy and no perf window runs in this worktree, keep or remove the window root as its evidence requires, then remove the marker:");
  if (typeof record.scratch === "string") lines.push(`  rm -rf ${shellQuote(record.scratch)}`);
  lines.push(`  rm -f ${shellQuote(markerPath)}`);
  return lines;
}

/** The window's runs: exactly one, or PERF-ONE-LAUNCH-PER-WINDOW naming what was selected. A plan's subsets and
 * `--plan` may still list several; only a window's selection is held to one. */
export function oneLaunchSelection(plan, options) {
  const runs = selectRuns(plan, options);
  if (runs.length !== 1) throw new Error(`${ONE_LAUNCH_REFUSED}: the selection names ${runs.length} runs (${runs.map((run) => run.id).join(", ")}); a measurement window holds exactly one launch, so select one run (--runs <id>); nothing was made or launched`);
  return runs;
}

/** Every reason a binding may not be launched: its schema, or a recorded smoke window configuration that does not
 * select the non-persistent web view store and disable background throttling in every window (an older build).
 * Judged from the binding's recorded fields; the executable itself is checked only by its SHA-256. */
export function refusedPerfBinding(binding) {
  const reasons = [];
  if (binding?.schema !== "swbpipe.desktop-perf-binding/v1") reasons.push(`${BINDING_REFUSED}: not a desktop perf binding; run --build again`);
  let windows = null;
  try { windows = JSON.parse(binding?.tauri_config)?.app?.windows ?? null; } catch { windows = null; }
  if (!Array.isArray(windows)) return [...reasons, `${BINDING_REFUSED}: the binding records no smoke window configuration; run --build again`];
  for (const line of refusedSmokeWindows(windows)) reasons.push(`${BINDING_REFUSED}: ${line}; run --build again`);
  windows.forEach((window, index) => { if (window?.backgroundThrottling !== "disabled") reasons.push(`${BINDING_REFUSED}: window ${index} of the bound build does not disable background throttling; run --build again`); });
  return reasons;
}

/** The bound app's binding, refused by name (PERF-BINDING-REFUSED) when it may not be launched or the bound
 * executable is not the one it records. */
export function readPerfBinding(appDir) {
  const binding = JSON.parse(readFileSync(path.join(appDir, BINDING), "utf8"));
  const refused = refusedPerfBinding(binding);
  if (refused.length) throw new Error(refused.join("\n"));
  if (sha256File(binding.binary.path) !== binding.binary.sha256) throw new Error(`${BINDING_REFUSED}: ${binding.binary.path} is not the bound binary (${binding.binary.sha256}); run --build again`);
  return binding;
}

/** What a window admits before anything is made: the plan, its one selected run and the binding. */
export function windowPreflight(options) {
  const plan = loadPlan(options.planFile);
  const runs = oneLaunchSelection(plan, options);
  readPerfBinding(options.appDir);
  return { plan, runs };
}

function forkRunChild(args) {
  return fork(fileURLToPath(import.meta.url), args, { detached: true, stdio: ["ignore", "inherit", "inherit", "ipc"] });
}

function listOwnedExecutable(binary) {
  return new Promise((resolve, reject) => {
    execFile("python3", [path.join(here, "perf-sampler.py"), "--list-owned", "--app-path", binary],
      { timeout: 1000, killSignal: "SIGKILL", maxBuffer: 64 * 1024 }, (error, stdout) => {
        if (error) return reject(error);
        try { resolve(JSON.parse(stdout)); } catch (invalid) { reject(invalid); }
      });
  });
}

/** The --run supervisor: preflight, admission by the marker, the window's scratch, its one supervised --run-child, the
 * window record, then cleanup and the marker's removal only on established teardown. `seams` are set only by
 * `perf.test.mjs` (a marker path, a stand-in child and owned-process lookup, a stand-in census for the refusal);
 * `supervisedRun` passes none. */
export async function superviseWindow(options, seams = {}) {
  const { markerPath = WINDOW_MARKER, startChild = forkRunChild, listOwned: listOwnedFor = listOwnedExecutable, census = censusOf, startTime = processStartTime, watch = {} } = seams;
  const { plan, runs } = windowPreflight(options);
  const budgetS = options.budgetS ?? Math.min(plan.budget_s, 840);
  if (!Number.isFinite(budgetS) || budgetS <= 0 || budgetS > 840) throw new Error("perf window: work budget must be > 0 and <= 840 s (teardown stays within 15 min)");
  const token = randomUUID();
  const admitted = acquireWindowMarker(markerPath, { schema: "swbpipe.desktop-perf-window-marker/v1", token, pid: process.pid, started: startTime(process.pid), state: "admitted", out: options.out, runs: runs.map((run) => run.id), created: new Date().toISOString() });
  if (!admitted) {
    for (const line of windowMarkerRefusal(markerPath, readRunMarker(markerPath), { startTime, census })) console.error(line);
    process.exitCode = 2;
    return { refused: WINDOW_UNRESOLVED, launches_selected: runs.length, launches_started: 0 };
  }
  let scratch = null;
  let started = false;
  let file = null;
  try {
    mkdirSync(options.out, { recursive: true });
    scratch = mkdtempSync(path.join(options.out, "perf-"));
    const binary = path.join(scratch, BIN_NAME);
    updateWindowMarker(markerPath, token, { state: "running", scratch, executable: binary });
    const postRunReserveS = Math.min(30, budgetS / 4);
    const args = ["--run-child", "--app-dir", options.appDir, "--models", options.models,
      "--out", options.out, "--plan-file", options.planFile, "--budget-s", String(budgetS - postRunReserveS),
      "--window-root", scratch, "--window-token", token, "--supervisor-pid", String(process.pid)];
    if (options.results) args.push("--results", options.results);
    if (options.subset) args.push("--subset", options.subset);
    if (options.runs) args.push("--runs", options.runs);
    if (options.keep) args.push("--keep");
    // From here a process may exist: any failure retains the marker, the scratch and whatever it holds.
    started = true;
    const child = startChild(args);
    const result = await watchWindow(child, { budgetMs: budgetS * 1000, listOwned: () => listOwnedFor(binary), ...watch });
    result.post_run_reserve_s = postRunReserveS;
    result.launches_selected = runs.length;
    result.window_marker = markerPath;
    result.limits = WINDOW_LIMITS;
    // The previous window's teardown, as the next window's admission requires it (the marker is kept otherwise).
    result.teardown_established = result.owned_group_gone === true && result.owned_executable_pids_gone === true;
    file = path.join(options.out, `perf-window-${result.started_ms}.json`);
    // The parent does not synchronously inspect SQLite, recursively remove the
    // store, or serialize the sampler's large report on its deadline event loop.
    // A stalled write exits with the marker in place.
    const emergency = setTimeout(() => { console.error("perf window: report write stalled; owned runner already stopped; the window marker is retained"); process.exit(124); }, 10000);
    // Failed, and so no figures kept, also for a window that did not start exactly one App.
    const failed = result.stop_reason || result.exit.code !== 0 || !result.exit.reaped || !result.teardown_established || result.launches_started !== 1;
    try {
      result.kept = failed ? scratch : null;
      result.cleanup = result.teardown_established ? (failed ? "retained_failed_window" : "pending") : "retained_teardown_unresolved";
      // The window's record is durable before anything is removed.
      await writeFile(file, `${JSON.stringify(result, null, 2)}\n`);
      if (!result.teardown_established) {
        updateWindowMarker(markerPath, token, { state: "teardown_unresolved", reason: `owned_group_gone=${result.owned_group_gone}, owned_executable_pids_gone=${result.owned_executable_pids_gone}`, window_file: file });
        console.error(`perf window: ${WINDOW_UNRESOLVED}: the teardown did not record the owned group and the owned executable's processes gone; retained: the window root ${scratch} (its home and private app copy) and the marker ${markerPath}; no next window starts until it is resolved`);
      } else {
        if (!failed) {
          try {
            await finishScratch({ scratch, binary, keep: options.keep });
            result.cleanup = options.keep ? "private_app_removed_store_kept" : "private_scratch_removed";
            result.kept = options.keep ? scratch : null;
          } catch (error) {
            result.cleanup = "failed";
            result.cleanup_error = String(error?.message ?? error).slice(0, 2000);
            result.kept = scratch;
          }
          await writeFile(file, `${JSON.stringify(result, null, 2)}\n`);
        }
        if (result.cleanup === "failed") {
          updateWindowMarker(markerPath, token, { state: "cleanup_failed", reason: result.cleanup_error, window_file: file });
          console.error(`perf window: ${WINDOW_UNRESOLVED}: removing the window root failed (${result.cleanup_error}); retained: what remains of ${scratch} and the marker ${markerPath}`);
        } else releaseWindowMarker(markerPath, token);
      }
    }
    finally { clearTimeout(emergency); }
    if (failed || result.cleanup === "failed") process.exitCode = 1;
    return result;
  } catch (error) {
    const reason = `supervisor_error: ${String(error?.message ?? error)}`.slice(0, 2000);
    if (!started) {
      // Nothing was started: the empty scratch, if made, and the marker are this window's to remove.
      if (scratch !== null) rmSync(scratch, { recursive: true, force: true });
      try { releaseWindowMarker(markerPath, token); } catch { /* left as found, and refused next time */ }
    } else {
      try { updateWindowMarker(markerPath, token, { state: "supervisor_error", reason, ...(file ? { window_file: file } : {}) }); }
      catch { /* the earlier marker record stays, and is refused next time */ }
    }
    throw error;
  }
}

function supervisedRun(options) {
  return superviseWindow(options);
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
  if (plan.c1_reader_diagnostics !== undefined && typeof plan.c1_reader_diagnostics !== "boolean") throw new Error("perf plan: reader diagnostics must be boolean");
  if (plan.c1_reader_diagnostics && !plan.c1_probes) throw new Error("perf plan: reader diagnostics require C1 probes");
  const ids = new Set();
  for (const run of plan.runs) {
    planName(run.id, "run id");
    if (ids.has(run.id)) throw new Error(`perf plan: run id ${run.id} is repeated`);
    ids.add(run.id);
    if (!MODES.has(run.mode)) throw new Error(`perf plan: run ${run.id} has mode ${JSON.stringify(run.mode)}`);
    if (plan.c1_probes && run.mode !== "native") throw new Error("perf plan: C1 usability probes require a native solve");
    // The 82-node rack is the smallest maintained one (perf-plan.json's functional subset): the
    // small connected C1 validation before any further 500-node attempt (perf-plan-c1-small.json).
    if (plan.c1_probes && (![82, 500, 1000].includes(run.nodes) || !/^[a-f0-9]{64}$/.test(run.model_sha256))) throw new Error("perf plan: C1 needs a bound 82/500/1000-node fixture; no size escalation");
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

// A C1 run's model is a recorded reference rack: its (SHA-256, profile, nodes) is a line of the
// generator's recorded hashes, `models_sha256.txt` (the scaling series) or `models_sha256_small.txt`
// (the small connected validation's 82-node rack), each checked against `rack_model.py` by
// `tests/test_performance_rack_model.py`. The launch then checks the file's bytes against it.
const RACK_MANIFESTS = ["models_sha256.txt", "models_sha256_small.txt"].map((name) => path.join(project, "core", "solver", "performance_harness", "observations", "rack", name));
export function rackManifestEntries(files = RACK_MANIFESTS) {
  const entries = new Set();
  for (const file of files) {
    for (const line of readFileSync(file, "utf8").split("\n")) {
      if (!line || line.startsWith("#")) continue;
      const match = /^([a-f0-9]{64}) {2}([a-z-]+) {2}([1-9][0-9]*)$/u.exec(line);
      if (!match) throw new Error(`${path.basename(file)}: malformed line ${JSON.stringify(line.slice(0, 120))}`);
      entries.add(`${match[1]} ${match[2]} ${match[3]}`);
    }
  }
  return entries;
}
export function requireRackBinding(plan, entries = null) {
  if (!plan.c1_probes) return plan;
  entries ??= rackManifestEntries();
  for (const run of plan.runs) {
    const name = /^([a-z-]+)-n([0-9]{4})\.json$/u.exec(run.model);
    if (!name || Number(name[2]) !== run.nodes || !entries.has(`${run.model_sha256} ${name[1]} ${run.nodes}`)) {
      throw new Error(`perf plan: C1 run ${run.id}'s model is not a recorded reference rack (observations/rack/models_sha256.txt or models_sha256_small.txt)`);
    }
  }
  return plan;
}

export function loadPlan(file) {
  const plan = JSON.parse(readFileSync(file, "utf8"));
  if (plan.schema !== "swbpipe.desktop-perf-plan/v1") throw new Error(`${file}: not a desktop perf plan`);
  return requireRackBinding(validatePlan(plan));
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

// The run's findings and its most severe class. A report does not rescue a launch that did not end cleanly or a
// report that is not this run's: the rule of `launch-outcome.mjs` (DEL-07-11: exit, signal, timeout, the census,
// this run's report and nonce) judges every launch (`launchFailures`). This runner's contract requires the census
// (`takeCensus`), so a census not given, or null, is LAUNCH-CENSUS-UNRESOLVED, never "no census taken". With a
// report, the named failures are a `launch_failed` finding's `failures` and the report's own observations are still
// read and classified beside it; without one, they are the `failures` of the app's crash or hang finding, so every
// failed condition is named, and an unresolved census adds its own `launch_failed` finding (the class is then
// crash, or launch_failed where it would have been hang). `reportProblem` says why a report file that exists was
// not read. A store not inspected (an unresolved census) is not read as a store without the run's project.
export function classify({ report, reportProblem = null, exit, timedOut, census, progress, memory, store, run, resultBytes, nonce }) {
  const findings = [];
  const add = (kind, detail) => findings.push({ class: kind, ...detail });
  const webcontentExited = Object.entries(memory.pids).filter(([, entry]) => entry.kind === "webcontent" && exit.ended_ms !== null && entry.last_ms < exit.ended_ms - 1000);
  for (const [pid, entry] of webcontentExited) add("crash", { process: "webcontent", pid: Number(pid), detail: `WebContent ${pid} stopped being sampled ${exit.ended_ms - entry.last_ms} ms before the app ended` });
  const failures = launchFailures({ status: exit.status, signal: exit.signal, timedOut, census: census ?? null, report, problem: reportProblem, nonce });
  const inspected = store?.inspection !== STORE_NOT_INSPECTED;
  if (!report) {
    const unread = reportProblem && reportProblem !== "no report" ? ` (${reportProblem})` : "";
    if (timedOut) add("hang", { process: "app", failures, detail: `no report within ${run.timeout_s} s${unread}; last progress: ${JSON.stringify(progress.at(-1) ?? null)}` });
    else add("crash", { process: "app", failures, detail: `the app exited (${exit.status ?? exit.signal}) without a report${unread}; last progress: ${JSON.stringify(progress.at(-1) ?? null)}` });
    const unresolved = failures.find((line) => line.startsWith(`${CENSUS_UNRESOLVED_STOP}:`));
    if (unresolved) add("launch_failed", { process: "app", step: "census", failures: [unresolved], detail: `${unresolved}. A process of the private app copy may still run; no further run starts in this window.`, needs_review: true });
  } else {
    if (failures.length > 0) add("launch_failed", { process: "app", failures, detail: `the launch did not end cleanly or its report is not this run's: ${failures.join("; ")}. The report's observations are still read.`, needs_review: true });
    const solve = report.solve;
    // C1's host condition not established (or lost): the run is unobservable. Its solve, driver
    // and probe outcomes are consequences, not findings; the store is still checked below.
    const host = report.c1?.host;
    const unobservable = run.c1_probes && host?.established === false;
    if (unobservable) add("host_unobservable", { step: host.step ?? null, detail: `${host.detail ?? "host condition lost"}; solve started: ${host.solve_started === true}. The display must be awake, the session unlocked and the App window uncovered and visible, delivering frames, for the whole observation: unobservable, not evidence of a product hang; not retried.` });
    if (unobservable) { /* no solve finding */ }
    else if (!solve) add(report.error ? "refused_with_message" : "hang", { step: "open", message: report.error ?? null, needs_review: true });
    else if (solve.ended === "first_paint") {
      // Painted only within the gate (RT-U4 F1): the run's own first_paint mark, on the
      // timeline's clock, at most `paint_ms` after the driver sent Analyze > Run. A mark
      // after it, or a report that cannot show when the run began, is the gate's hang.
      const mark = solve.timeline?.marks?.find((item) => item.stage === "first_paint");
      const startedAt = solve.gate?.started_at_ms;
      if (!mark || !Number.isFinite(mark.at_ms) || !Number.isFinite(startedAt)) add("hang", { step: "solve", detail: `first paint not shown to be within ${run.paint_ms} ms of Analyze > Run` });
      else if (mark.at_ms - startedAt > run.paint_ms) add("hang", { step: "solve", detail: `first paint ${mark.at_ms - startedAt} ms after Analyze > Run, after the ${run.paint_ms} ms gate` });
    }
    else if (solve.ended === "timeout") add("hang", { step: "solve", detail: `no first paint within ${run.paint_ms} ms; last stage ${solve.timeline?.marks?.at(-1)?.stage ?? null}` });
    // The window's text, verbatim (the timeline carries only the failure's code and stage).
    else add("refused_with_message", { step: "solve", message: solve.shown?.error ?? solve.shown?.refusal ?? solve.shown?.results_refusal ?? solve.shown?.summary ?? null, code: solve.timeline?.failure?.code ?? null, stage: solve.timeline?.failure?.stage ?? null, needs_review: true });
    const record = report.record_run;
    if (record?.timed_out) add("hang", { step: "record_run", detail: `last message: ${record.last_message}` });
    else if (record && record.message && /^(Not recorded|Record Run:)/.test(record.message)) add("refused_with_message", { step: "record_run", message: record.message, needs_review: true });
    // Every save step on its own record (RT-U4: a later step must not mask an earlier one's
    // timeout or refusal); `report.save_project` is the first step's.
    const save = report.save_project;
    const saveSteps = Array.isArray(report.save_projects) ? report.save_projects : save ? [save] : [];
    for (const step of saveSteps) {
      const name = saveSteps.length > 1 ? `save_project#${step.step}` : "save_project";
      if (step?.timed_out) add("hang", { step: name, detail: `last message: ${step.last_message}` });
      else if (step && step.message && !/^(Saved|Created)|SQLite project snapshot/.test(step.message)) add("refused_with_message", { step: name, message: step.message, needs_review: true });
    }
    if (report.error && solve && !unobservable) add("refused_with_message", { step: "driver", message: report.error.split("\n")[0], needs_review: true });
    if (run.c1_probes && report.c1?.trace?.transport?.lost) add("refused_with_message", { step: "c1_observation", message: `The command observation lost Tauri's IPC transport: ${report.c1.trace.transport.events?.[0]?.reason ?? "unknown"}.`, needs_review: true });
    if (unobservable) { /* the probes below could not be observed */ }
    else if (run.c1_profile_readers && !report.c1?.reader_allocations) add("refused_with_message", { step: "reader_allocations", message: "Reader allocation evidence is incomplete.", needs_review: true });
    // Every native save invocation on its own (review of 4e209edf3): a refusal, an observation
    // error, an unread reply or a missing receipt is a finding whatever any step reported, and the
    // invocations must be the probe's expected commands.
    const invocations = Array.isArray(report.c1?.trace?.saves) ? report.c1.trace.saves : [];
    for (const invocation of invocations) {
      const problem = invocation?.refusal ?? invocation?.error ?? (invocation?.read !== true ? "its reply was not read" : !invocation?.reply?.result ? "no native receipt" : invocation.reply.result.outcome !== "saved" && invocation.reply.result.outcome !== "not_saved" ? `outcome ${invocation.reply.result.outcome}` : null);
      if (problem) add("refused_with_message", { step: `save_invocation#${invocation?.seq}`, message: `${invocation?.command}: ${problem}`, needs_review: true });
    }
    const expectedSaves = report.c1?.expected_saves;
    if (Array.isArray(expectedSaves) && (invocations.length !== expectedSaves.length || invocations.some((invocation, index) => invocation?.command !== expectedSaves[index]))) add("refused_with_message", { step: "save_reconciliation", message: `save invocations [${invocations.map((invocation) => invocation?.command).join(", ")}] differ from the expected [${expectedSaves.join(", ")}]`, needs_review: true });
    const saveReceipts = Array.isArray(report.c1?.trace?.saves) ? report.c1.trace.saves.map(invocation => invocation?.reply?.result) : [report.c1?.trace?.save?.result, report.c1?.trace?.restoredSave?.result];
    const omitted = saveReceipts.find(result => result?.outcome === "not_saved");
    if (omitted) add("refused_with_message", { step: "save_result", message: omitted.code ?? "The native save kept only the model.", needs_review: true });
    if (run.c1_probes && !unobservable && report.c1?.completed !== true) add("refused_with_message", { step: "c1_probes", message: "The connected C1 usability probes did not complete.", needs_review: true });
    if (run.c1_probes && inspected && report.c1?.completed && report.c1.csv_file?.verified !== true) {
      if (report.c1.csv_file?.error) add("refused_with_message", { step: "csv_inspection", message: report.c1.csv_file.error, needs_review: true });
      else add("misleading_message", { step: "csv", detail: "CSV rows do not match the native query total." });
    }
    // What the store holds against what the window said.
    const stored = store?.projects?.find((row) => row.project_id === run.project_id);
    const claimedSaved = Boolean(save?.message && /^(Saved|Created)|SQLite project snapshot/.test(save.message)) || Boolean(record?.attempted && record.message && !/^Not recorded/.test(record.message));
    if (store?.quick_check && store.quick_check.join(",") !== "ok") add("half_written_save", { detail: `quick_check: ${store.quick_check.slice(0, 3).join("; ")}` });
    if ((store?.files ?? []).some((file) => /-journal$|-wal$/.test(file.name) && file.bytes > 0)) add("half_written_save", { detail: `left over: ${store.files.filter((file) => /-journal$|-wal$/.test(file.name)).map((file) => `${file.name} (${file.bytes} bytes)`).join(", ")}` });
    if (inspected && claimedSaved && solve?.ended === "first_paint" && (!stored || stored.result_type !== "text")) add("misleading_message", { detail: `the window reported a save or a recorded run, but the store holds ${stored ? `a ${stored.result_type} result` : "no project row"} for ${run.project_id}` });
    // Only a session that held the result (painted) can be judged against its size.
    if (!omitted && solve?.ended === "first_paint" && stored && stored.result_type === "text" && resultBytes && stored.result_chars < resultBytes / 2) add("half_written_save", { detail: `the stored result has ${stored.result_chars} characters, the solve's result ${resultBytes} bytes` });
  }
  if (findings.length === 0) add("ok", {});
  const worst = SEVERITY.find((kind) => findings.some((finding) => finding.class === kind));
  return { class: worst, findings };
}

/** Entry requires a fresh known-normal host before any App launch. The
 * existing supervisor deadline also covers a stalled interpreter/sysctl here.
 * `execute` and `now` are injected only by offline tests, never CLI options. */
export async function readStartupHost(appPath, execute = execFile, now = Date.now) {
  const sentAt = now();
  const text = await new Promise((resolve, reject) => {
    execute("python3", [path.join(here, "perf-sampler.py"), "--host-status", "--app-path", appPath],
      { timeout: 1500, killSignal: "SIGKILL", maxBuffer: 4096 }, (error, stdout) => {
        if (error) reject(new Error("HOST-ADMISSION-REFUSED: fresh host telemetry failed or stalled; no App launched"));
        else resolve(stdout);
      });
  });
  let sample;
  try { sample = JSON.parse(text); }
  catch { throw new Error("HOST-ADMISSION-REFUSED: malformed host telemetry; no App launched"); }
  const receivedAt = now();
  audit({ type: "stage", step: "host_admission", stage: "read", host: sample, sent_at_ms: sentAt, received_at_ms: receivedAt });
  if (!sample || !Number.isFinite(sample.available_percent) || sample.available_percent < 0 || sample.available_percent > 100 || ![1, 2, 4].includes(sample.pressure_level) || !Number.isFinite(sample.sampled_at_ms) || sample.sampled_at_ms < sentAt - 1 || sample.sampled_at_ms > receivedAt + 1) {
    throw new Error("HOST-ADMISSION-REFUSED: host telemetry is unknown or not fresh for this request; no App launched");
  }
  if (sample.available_percent < 30 || sample.pressure_level !== 1) throw new Error("HOST-ADMISSION-REFUSED: host headroom/pressure does not admit the App; no App launched");
  return { ...sample, requested_at_ms: sentAt, received_at_ms: receivedAt };
}

/** C1-only fixed phase journal. A received entry is fsynced before it is
 * retained/reported. Entries without exits remain explicit, including after
 * App/runner failure. Never accepts model/result/identity or free-form text. */
export function createReaderPhaseEvidence(file) {
  const fd = openSync(file, "wx", 0o600);
  const events = [], active = new Map();
  let overflow = false, closed = false;
  // `run_preview`: the fresh solve's awaited native analysis-run preview (web layer only).
  const web = new Set(["header", "registration", "standing", "summary", "run_preview"]);
  const native = new Set(["args_admit", "source_prescan", "source_admit", "inputs_admit", "source_parse", "inputs_parse", "validate", "serialize", "source_drop", "summary_health", "summary_build"]);
  // The driver's observations of the page (perf-driver.js `journal`): Results opened, the native
  // table's first page seen settled with rows, its two frames up to the first_paint mark, the page
  // not shown, and an animation frame not delivered. Recorded and matched like the awaits above,
  // and listed when an exit is missing, but not required: the reader evidence is complete
  // without them (a page that is never hidden has no `page_hidden` entry at all).
  const observed = new Set(["results_open", "first_page", "paint_frames", "page_hidden", "frame_wait"]);
  const required = event => event.layer === "native" || web.has(event.phase);
  const write = value => {
    const bytes = Buffer.from(`${JSON.stringify(value)}\n`);
    let offset = 0;
    while (offset < bytes.length) {
      const written = writeSync(fd, bytes, offset, bytes.length - offset);
      if (written <= 0) throw new Error("C1 reader phase evidence write made no progress");
      offset += written;
    }
    fsyncSync(fd);
  };
  const summary = () => {
    const covered = layer => new Set(events.filter(event => event.layer === layer && event.edge === "exit" && event.status === "ok").map(event => event.phase));
    const webCovered = covered("web"), nativeCovered = covered("native");
    return { events: events.slice(), incomplete: [...active.values()], overflow,
      evidence_complete: !overflow && ![...active.values()].some(required) && [...web].every(phase => webCovered.has(phase)) && [...native].every(phase => nativeCovered.has(phase)) && !events.some(event => required(event) && event.edge === "exit" && event.status !== "ok") };
  };
  return {
    accept(value) {
      if (closed) throw new Error("C1 reader phase evidence is closed");
      if (value?.overflow === true && value.layer === "native" && Object.keys(value).every(key => ["layer", "overflow"].includes(key))) {
        if (!overflow) write({ overflow: true });
        overflow = true; return;
      }
      const allowed = ["layer", "call", "phase", "edge", "status", "mono_ms", "bytes"];
      const fields = value && Object.keys(value);
      if (!fields || fields.some(key => !allowed.includes(key)) || !["web", "native"].includes(value.layer) || !(value.layer === "web" ? web.has(value.phase) || observed.has(value.phase) : native.has(value.phase)) || !Number.isInteger(value.call) || value.call < 1 || value.call > 32 || !Number.isFinite(value.mono_ms) || value.mono_ms < 0 || !["enter", "exit"].includes(value.edge) || !(value.edge === "enter" ? value.status === "pending" : ["ok", "error", "incomplete"].includes(value.status)) || (value.bytes !== undefined && (!Number.isSafeInteger(value.bytes) || value.bytes < 0))) throw new Error("C1 reader phase event is malformed");
      if (events.length >= 256) {
        if (!overflow) write({ overflow: true });
        overflow = true; return;
      }
      const event = { ...value, received_epoch_ms: Date.now() };
      const key = `${value.layer}:${value.call}:${value.phase}`;
      if (value.edge === "enter" && active.has(key) || value.edge === "exit" && !active.has(key)) throw new Error("C1 reader phase has an unmatched boundary");
      write(event);
      events.push(event);
      if (value.edge === "enter") active.set(key, event);
      else if (value.status !== "incomplete") active.delete(key);
    },
    summary,
    close() {
      if (closed) return summary();
      const result = summary();
      write({ terminal: true, incomplete: result.incomplete.map(({ layer, call, phase }) => ({ layer, call, phase })), overflow, evidence_complete: result.evidence_complete });
      closeSync(fd); closed = true; return result;
    }
  };
}

/** The census after one launch (`awaitNoProcesses` on the private copy): `{ resolved: true }`, or
 * `{ resolved: false, reason }`. It never throws and never reads anything but `resolved === true` as resolved: an
 * exception, a missing or malformed answer, or the asynchronous-answer timer expiring is unresolved evidence.
 * That timer cannot interrupt synchronous work; the separate supervisor may stop the runner first. */
export async function takeCensus(binary, { boundMs, census = awaitNoProcesses, graceMs = CENSUS_ANSWER_GRACE_MS }) {
  let timer;
  try {
    const answer = await Promise.race([
      Promise.resolve().then(() => census(binary, { boundMs })),
      new Promise((resolve) => { timer = setTimeout(() => resolve({ resolved: false, reason: `the census gave no answer within ${boundMs + graceMs} ms` }), boundMs + graceMs); })
    ]);
    if (answer?.resolved === true) return { resolved: true };
    const reason = typeof answer?.reason === "string" && answer.reason ? answer.reason : "the census gave no result";
    return { resolved: false, reason: reason.slice(0, 2000) };
  } catch (error) {
    return { resolved: false, reason: `the census could not be taken: ${String(error?.message ?? error).slice(0, 2000)}` };
  } finally {
    clearTimeout(timer);
  }
}

/** The named failures of a kept perf run record, judged again from its recorded launch outcome and report without a
 * launch. Census-bearing (`recordedCensusFailures`): a record without a census, or with a null one, as every
 * record written before this runner took one, is LAUNCH-CENSUS-UNRESOLVED and never a pass. */
export function keptRunFailures(record) {
  const report = record?.report && typeof record.report === "object" ? record.report : null;
  return recordedCensusFailures(record?.launch_outcome, report, report ? null : "no report recorded");
}

// What a launch uses from the host besides the app itself. `ctx.deps` overrides them only in `perf.test.mjs`
// (controlled answers, a stand-in sampler and census, a short census bound); `runPlan` never sets it.
const LAUNCH_DEPS = Object.freeze({
  marks: () => marksCheck(),
  webkitPids: () => webkitPids(),
  startupHost: (binary) => readStartupHost(binary),
  // By absolute path, given the App by PID, with an empty environment: it needs nothing from this runner's.
  caffeinate: (pid) => spawn("/usr/bin/caffeinate", ["-di", "-w", String(pid)], { stdio: "ignore", env: {} }),
  sampler: (samplerArgs) => spawn("python3", [path.join(here, "perf-sampler.py"), ...samplerArgs], { stdio: ["ignore", "pipe", "inherit"] }),
  census: awaitNoProcesses,
  censusBoundMs: PERF_CENSUS_BOUND_MS,
  // The App starts one window may make. Only a test sets another, to drive the window path through several runs;
  // the CLI and the plan cannot (`oneLaunchSelection` also holds every window's selection to one run).
  launchLimit: 1
});

/** A run refused before its App started: launched false, the reason quoted. */
const refusedBeforeLaunch = (record, step, message, kept) => ({ ...record, launched: false, class: "refused_with_message", findings: [{ class: "refused_with_message", step, message, needs_review: true }], ...(kept ? { kept } : {}) });

async function runOne(run, ctx) {
  const deps = { ...LAUNCH_DEPS, ...ctx.deps };
  const record = { id: run.id, mode: run.mode, model: run.model, binding: ctx.binding, harness: ctx.harness, started_at: new Date().toISOString(), launched: false };
  // One App start per window, counted before any of this run's private state is made.
  if ((ctx.launchAttempts ?? 0) >= deps.launchLimit) return refusedBeforeLaunch(record, "launch_count", `${ONE_LAUNCH_REFUSED}: this window has already started its App; a measurement window holds one launch, and no further App is started in it.`);
  // Every plan-named source and destination stays inside its root (validatePlan, containedPath).
  let modelSource, resultFile;
  try {
    modelSource = containedPath(ctx.models, run.model, { mustExist: true });
    resultFile = run.mode === "injected" ? containedPath(ctx.results, run.result, { mustExist: true }) : null;
  } catch (error) {
    return { ...record, class: "skipped", detail: error.message };
  }
  // The run's root holds its report and its throwaway home: created empty, with TMPDIR's folder and Downloads inside
  // (`createThrowawayHome`). The model the driver's chooser opens is copied inside the home, so that the strict
  // chooser admits it and nothing outside the home.
  const root = mkdtempSync(path.join(ctx.scratch, `run-${planName(run.id, "run id")}-`));
  const { home, tmpdir } = createThrowawayHome(root);
  record.home = home;
  const chooserModels = path.join(home, "chooser", "models");
  mkdirSync(chooserModels, { recursive: true });
  const modelPath = containedPath(chooserModels, run.model);
  copyFileSync(modelSource, modelPath, constants.COPYFILE_EXCL);
  const document = JSON.parse(readFileSync(modelPath, "utf8"));
  run = { ...run, project_id: document.project.id, c1_probes: ctx.plan.c1_probes === true, c1_reader_diagnostics: ctx.plan.c1_reader_diagnostics === true, c1_profile_readers: ctx.plan.c1_profile_readers === true };
  record.project_id = run.project_id;
  record.model_bytes = statSync(modelPath).size;
  record.model_sha256 = sha256File(modelPath);
  record.model_jcs_sha256 = `sha256:${createHash("sha256").update(canonical(document), "utf8").digest("hex")}`;
  if (run.c1_probes && (record.model_sha256 !== run.model_sha256 || !Array.isArray(document.nodes) || document.nodes.length !== run.nodes)) {
    return { ...record, class: "refused_with_message", findings: [{ class: "refused_with_message", step: "fixture", message: "C1 fixture bytes/node count differ from the selected 82/500/1000-node binding; no App was launched.", needs_review: true }], kept: root };
  }
  if (resultFile) record.injected_result = { file: path.basename(resultFile), bytes: statSync(resultFile).size };
  const timeoutMs = Math.min(run.timeout_s * 1000, ctx.remainingMs());
  // The run's nonce: the driver hands it back in its report, so that the report is known to be this run's.
  const nonce = newNonce();
  const plan = {
    run_id: run.id, nonce, model_path: modelPath, project_id: run.project_id, record_run: true, save_project: true,
    c1_probes: ctx.plan.c1_probes === true, c1_reader_diagnostics: ctx.plan.c1_reader_diagnostics === true, c1_profile_readers: ctx.plan.c1_profile_readers === true, model_jcs_sha256: record.model_jcs_sha256,
    timeouts: { ...ctx.plan.timeouts, paint_ms: Math.min(run.paint_ms, timeoutMs) }
  };
  // The App's whole environment, built from nothing (`buildLaunchEnv`; PARENT_ALLOWLIST carries nothing): the home
  // variables, TMPDIR inside the home, the strict chooser, the driver, plan and report, and only the variables this run
  // selects. The object checked here is the frozen object the App is started with.
  const runner = { SWBPIPE_NATIVE_SMOKE_DRIVER: path.join(here, "perf-driver.js"), SWBPIPE_NATIVE_SMOKE_PLAN: JSON.stringify(plan), SWBPIPE_NATIVE_SMOKE_REPORT: path.join(root, "report.json") };
  if (resultFile) {
    runner[INJECT_ENV] = resultFile;
    runner[INJECT_ROOT_ENV] = realpathSync(ctx.results);
  }
  if (ctx.plan.c1_reader_diagnostics) runner[READER_DIAGNOSTICS_ENV] = "1";
  const env = Object.freeze(buildLaunchEnv({ parent: process.env, home, tmpdir, runner }));
  const envRefused = [...refusedPerfLaunchEnv(env, { inject: resultFile !== null, readerDiagnostics: ctx.plan.c1_reader_diagnostics === true }), ...refusedPlanPaths(plan, home)];
  if (envRefused.length) return refusedBeforeLaunch(record, "launch_env", `${envRefused.join("; ")}; no App was launched.`, root);
  // The three marks (`marks-check.mjs`), checked here, after every build and binding and directly before the
  // launch's own host admission (webkitPids, readStartupHost) and the app's start.
  record.marks = deps.marks();
  if (marksRefuse(record.marks)) {
    return { ...record, class: "refused_with_message", findings: [{ class: "refused_with_message", step: "marks", message: "NATIVE-SMOKE-MARK-PRESENT: an agent may be operating the App on the screen, or the marks could not be checked; no App was launched.", needs_review: true }], kept: root };
  }
  record.reader_phase_file = ctx.plan.c1_reader_diagnostics ? path.join(ctx.out, `perf-reader-phases-${planName(path.basename(ctx.scratch), "window")}-${planName(run.id, "run")}.jsonl`) : null;
  const phases = record.reader_phase_file ? createReaderPhaseEvidence(record.reader_phase_file) : null;
  const baseline = deps.webkitPids();
  record.startup_host = await deps.startupHost(ctx.binary);
  // The window's one App start, counted again directly before it with nothing awaited between.
  if ((ctx.launchAttempts ?? 0) >= deps.launchLimit) return refusedBeforeLaunch(record, "launch_count", `${ONE_LAUNCH_REFUSED}: this window has already started its App; no further App is started in it.`, root);
  ctx.launchAttempts = (ctx.launchAttempts ?? 0) + 1;
  const startedMs = Date.now();
  // Inherit the supervised runner's group. A detached App could outlive the
  // supervisor's deadline when its runner is blocked in synchronous SQLite.
  const app = spawn(ctx.binary, [], { cwd: root, env, stdio: ["ignore", "ignore", "pipe"] });
  record.launched = Number.isSafeInteger(app.pid);
  audit({ type: "app", pid: app.pid });
  let appExited = false;
  app.once("exit", () => { appExited = true; audit({ type: "app_exit", pid: app.pid }); });
  const caffeinate = deps.caffeinate(app.pid);
  record.samples_file = path.join(ctx.out, `perf-samples-${planName(path.basename(ctx.scratch), "window")}-${planName(run.id, "run")}.jsonl`);
  const sampler = deps.sampler(["--app-pid", String(app.pid), "--app-path", ctx.binary, "--baseline", baseline.join(","), "--interval-ms", String(ctx.plan.sample_interval_ms), "--out", record.samples_file]);
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
      if (line.startsWith("[native-smoke] reader-phase:")) {
        try {
          if (!phases || line.length > 1024) throw new Error("unexpected/unbounded C1 phase event");
          phases.accept(JSON.parse(line.slice("[native-smoke] reader-phase:".length)));
          if (phases.summary().overflow) audit({ type: "stop", reason: "reader_phase_evidence_overflow" });
        } catch { audit({ type: "stop", reason: "reader_phase_evidence_invalid_or_write_failed" }); }
        continue;
      }
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
  // Exit can precede the final pipe data. Drain only within a fixed bound;
  // an undrained pipe is incomplete evidence, never a fabricated phase exit.
  let phasePipeDrained = app.stderr.readableEnded;
  if (phases && !phasePipeDrained) await new Promise(resolve => {
    const ended = () => { clearTimeout(timer); phasePipeDrained = true; resolve(); };
    const timer = setTimeout(() => { app.stderr.off("end", ended); resolve(); }, 1000);
    app.stderr.once("end", ended);
  });
  record.reader_phases = phases?.close() ?? null;
  if (record.reader_phases && !phasePipeDrained) record.reader_phases.evidence_complete = false;
  // The timed span ends at the app's exit; nothing after this line is in it.
  record.elapsed_s = (exit.ended_ms - startedMs) / 1000;
  record.exit = { ...exit, timed_out: timedOut, timeout_ms: timeoutMs };
  // The census (DEL-07-11 VER-008): no process may still run the private copy (a solve worker runs it in its own
  // process group). Taken after the app's exit, the sampler's stop and the bounded stderr drain; outside the timed
  // span; before the checkpoint, the store and CSV inspections and the record's commit, and so before the next
  // run's marks and admission. It covers the private executable only, not the WebKit helper processes launchd starts
  // for the app, and nothing here observes the host between windows.
  const censusStartedMs = Date.now();
  const census = await takeCensus(ctx.binary, { boundMs: deps.censusBoundMs, census: deps.census });
  record.census_window = { bound_ms: deps.censusBoundMs, started_ms: censusStartedMs, ended_ms: Date.now() };
  // The launch outcome as `launch-outcome.mjs` records it, census included, so that the record can be judged again
  // (`keptRunFailures`).
  record.launch_outcome = recordedOutcome({ status: exit.status, signal: exit.signal, timedOut, census, nonce, started: new Date(startedMs).toISOString(), ended: new Date(exit.ended_ms).toISOString() });
  const { report, problem: reportProblem } = readReport(path.join(root, "report.json"));
  const jobDone = markAt(report?.solve?.timeline, "job_done");
  const memory = summarizeSamples(samples, { since: jobDone });
  const webcontent = Object.entries(memory.pids).filter(([, entry]) => entry.kind === "webcontent").map(([pid]) => Number(pid));
  memory.webcontent_identification = {
    method: "WebKit WebContent processes (by executable path) not running before the launch; WebKit starts them from launchd, not as the app's children",
    candidates: webcontent,
    ambiguous: webcontent.length > 1
  };
  memory.peak_footprint_since = jobDone === null ? null : "job_done";
  let store;
  if (census.resolved === true) {
    checkpointRun({ ...record, report, progress, memory, kept: root, store: { inspection: "pending" } }, ctx);
    audit({ type: "stage", step: "inspect_store", stage: "started" });
    store = await inspectStore(home);
    audit({ type: "stage", step: "inspect_store", stage: "completed" });
    if (run.c1_probes && report?.c1?.fresh?.csv) report.c1.csv_file = await inspectCsv(home, report.c1.fresh.csv);
  } else {
    // A process of the private copy may still run and write the store and Downloads. Neither is inspected, so that
    // no inspection can block before the failure is recorded: the record goes to its commit (`runWindow`) at once,
    // and the window stops there.
    store = { inspection: STORE_NOT_INSPECTED, reason: `${CENSUS_UNRESOLVED_STOP}: ${census.reason}` };
    record.window_stop = CENSUS_UNRESOLVED_STOP;
  }
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
  Object.assign(record, classify({ report, reportProblem, exit, timedOut, census, progress, memory, store, run, resultBytes, nonce }));
  if (ctx.plan.c1_reader_diagnostics && record.reader_phases?.evidence_complete !== true) {
    record.findings.push({ class: "refused_with_message", step: "reader_phases", message: "Native reader phase exits are incomplete or omitted.", needs_review: true });
    if (record.class === "ok") record.class = "refused_with_message";
  }
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

/** Commit a finished run before anything else follows it. Samples live outside
 * scratch. Nothing is removed here: the run's home stays until the supervisor's
 * teardown has recorded the owned group and the owned executable's processes
 * gone, and only an ok window's scratch is then removed (`superviseWindow`). */
export async function commitRun(record, { out, scratch }) {
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
}

/** The window's runs in plan order: one from the CLI (`oneLaunchSelection`), and `runOne` starts at most
 * `launchLimit` Apps (one; only a test sets more, to drive this path through several runs). Each run's record is
 * committed before the next run's marks, admission or start.
 * A run that is not ok ends the window (nothing is retried), and an unresolved census ends it whatever the run's
 * class: no further run starts while a process of the private copy may still run. An ok record must also pass the
 * census-bearing judgment its kept record will get (`keptRunFailures`), or it is committed as launch_failed. */
export async function runWindow(runs, ctx) {
  const records = [];
  for (const run of runs) {
    if (run.estimate_s * 1000 > ctx.remainingMs()) {
      const skipped = { binding: ctx.binding, harness: ctx.harness, id: run.id, mode: run.mode, model: run.model, class: "skipped", detail: `estimate ${run.estimate_s} s exceeds the remaining budget ${Math.round(ctx.remainingMs() / 1000)} s` };
      await commitRun(skipped, ctx);
      records.push(skipped);
      console.log(`${run.id}: skipped (window budget)`);
      continue;
    }
    const record = await runOne(run, ctx);
    if (record.class === "ok") {
      const kept = keptRunFailures(record);
      if (kept.length > 0) {
        record.class = "launch_failed";
        record.findings = [{ class: "launch_failed", process: "app", failures: kept, detail: `the record does not pass its kept-record judgment: ${kept.join("; ")}`, needs_review: true }];
      }
    }
    await commitRun(record, ctx);
    records.push(record);
    if (record.window_stop) {
      console.error(`${run.id}: ${record.window_stop}: ${record.launch_outcome?.census?.reason ?? "no census"}; the failed launch is recorded in ${record.record_file}; no further run starts in this window`);
      break;
    }
    // A run that is not ok (host_unobservable included) ends the window: nothing is retried.
    if (record.class !== "ok") break;
  }
  return records;
}

async function runPlan(options) {
  if (!process.send) throw new Error("perf run: only the supervised --run path can start the App");
  const binding = readPerfBinding(options.appDir);
  const bound = binding.binary.path;
  const plan = loadPlan(options.planFile);
  // The window's run count is set here, by the plan's subset or --runs. The standing rule for criterion 9's
  // measurement (the INTERLOCUTOR, issue #360): "one launch per kept window, and no next window unless the previous
  // window's teardown recorded the owned group and the owned executable's processes gone; one launch alone does not
  // establish cleanup". A kept window is one whose figures are kept as measurement, not --keep's retained scratch.
  // Enforced: a selection of any other number than one run is refused here and in the supervisor's preflight
  // (`oneLaunchSelection`), `runOne` starts at most one App per window, and the supervisor admits a window only by
  // creating the window marker, which a window whose teardown or cleanup was not established leaves in place.
  // The plans' subsets may still list several runs for --plan; a window runs one of them.
  const runs = oneLaunchSelection(plan, options);
  const budgetMs = (options.budgetS ?? plan.budget_s) * 1000;
  mkdirSync(options.out, { recursive: true });
  const scratch = options.windowRoot;
  if (!scratch || path.dirname(scratch) !== options.out || !path.basename(scratch).startsWith("perf-")) throw new Error("perf run-child: no private window root");
  // Started only by the supervisor that holds this window's marker (its token and window root).
  const marker = readRunMarker(WINDOW_MARKER);
  if (!options.windowToken || marker?.token !== options.windowToken || marker.scratch !== scratch) throw new Error(`perf run-child: WINDOW-NOT-ADMITTED: ${WINDOW_MARKER} does not record this window; no App is started`);
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
  const ctx = { plan, binding, harness, binary, models: options.models, results: options.results ?? options.models, out: options.out, scratch, keep: options.keep, launchAttempts: 0, remainingMs: () => budgetMs - (Date.now() - started) };
  process.stdout.write(formatPlan(runs, budgetMs / 1000));
  const records = await runWindow(runs, ctx);
  const report = {
    schema: "swbpipe.desktop-perf-report/v1",
    written_at: new Date().toISOString(),
    host: { platform: process.platform, release: os.release(), cpus: os.cpus().length, cpu_model: os.cpus()[0]?.model ?? null, memory_bytes: os.totalmem() },
    binding, harness,
    plan: { file: path.relative(project, options.planFile), subset: options.runs ? null : options.subset ?? "window", runs: runs.map((run) => run.id), budget_s: budgetMs / 1000 },
    // Selected, attempted and started App launches in this window (zero when refused or skipped before the start).
    launches: { selected: runs.length, attempted: ctx.launchAttempts, started: records.filter((record) => record.launched === true).length },
    conditions: [
      "Observation only: no time or memory bound is asserted, and criterion 9's provisional bounds are not evaluated.",
      "A release build with the native-smoke feature (product code plus the smoke harness and the result injection); background throttling disabled and a non-persistent web view store selected, as the native smoke.",
      "Injected runs skip the solve worker: no solve runs, worker_peak_footprint_bytes is unset, and d_M, d_S and their parsed values are computed in the app process before publication (so the app's peak before job_done includes them; peak_footprint_since counts from job_done).",
      "WebContent is identified as the WebKit WebContent processes started after the launch; another WebKit client starting one during the run would be counted (memory.webcontent_identification).",
      "One launch per window, against a throwaway home created empty (HOME and CFFIXED_USER_HOME; TMPDIR inside it), with an environment built from nothing and the strict chooser on, the opened model copied inside the home; the host is the one in `host`, not necessarily the 16 GiB base machine.",
      "After the launch, outside its timed span, a census finds whether any process still runs the private app copy; an unresolved census is launch_failed (LAUNCH-CENSUS-UNRESOLVED) and ends the window. It does not cover the WebKit helper processes launchd starts for the app, or the host between windows.",
      ...WINDOW_LIMITS,
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
      out: { type: "string" }, "budget-s": { type: "string" }, keep: { type: "boolean" }, "window-root": { type: "string" }, "window-token": { type: "string" }, "supervisor-pid": { type: "string" }
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
      windowRoot: values["window-root"] ? path.resolve(values["window-root"]) : null, windowToken: values["window-token"] ?? null
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
