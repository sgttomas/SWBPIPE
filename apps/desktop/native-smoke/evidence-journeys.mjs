#!/usr/bin/env node
// Evidence journeys in the launched desktop app, macOS: the report outputs on the pressure-free and
// the exact route (DEL-08-01 AC-011, DEL-12-02 AC-007), and autosave recovery after the app is ended
// without warning (DEL-02-05 AC-011 / VER-008, without its fault injection, AC-012).
//
//   tools/t3_slot.sh node apps/desktop/native-smoke/evidence-journeys.mjs --build-only
//   tools/t3_slot.sh node apps/desktop/native-smoke/evidence-journeys.mjs --journey <names> [--skip-build] [--keep]
//   tools/t3_slot.sh node apps/desktop/native-smoke/evidence-journeys.mjs --all [--skip-build] [--keep]
//
// (`tools/t3_slot.sh` is the shared development host's slot wrapper,
// `/Users/ryan/dev/chirality-t3/tools/t3_slot.sh` there.) --build-only checks the engines, builds and binds
// the app exactly as a run does, checks the executable's identity against the binding, and exits: no copy,
// no window, no launch, and the executable is never run. --all runs autosave-recovery, report-exact and
// report-pressure-free in that order, so the run's first launch is its shortest (autosave-recovery's first:
// settings, one save, two edits and at most one autosave interval), where the web view's data location is
// decided. Journeys, each in its own throwaway HOME:
//
//   report-pressure-free  1 launch. The bundled demo: solve (sparse); an observation of selection by keyboard
//                         (below); render the local-private report; File > Save Shareable Report Package…
//                         through the dialog's own tick box and statement, the save chooser answered by
//                         script; the dialog opened again (it starts unticked) and cancelled; the report
//                         rendered again, which must be local private (DEL-08-01 AC-010): its context, intent,
//                         warning and notice, no user-release notice, and its canonical hash recomputed by the
//                         runner from the canonical HTML, as for the first render; File > Save Report Package…
//                         (local private); the results table's CSV export through its own control (a native
//                         save into Downloads with no chooser): its data rows against the group's own count as
//                         the window states it, its model hash and the product's layout; Save Model Document…
//                         of the session; an observation of "Save canonical HTML" (below).
//   report-exact          1 launch. The same on the exact route, after File > Open Model Document… of
//                         `fixtures/model_operations/exact_pressure_authoring_model.json`, without the
//                         selection observation and the CSV save.
//   reference-system      Kept apart: named alone, never part of --all, and not run under the present
//                         leave. 1 launch; see referenceJourney below.
//   autosave-recovery     2 launches. A fresh settings folder: the interval as first shown, turned off (the
//                         notice) and on at DEL-02-05's shortest choice through the product's own control; the
//                         demo saved as a local project; a checkpoint with a read-only copy of the store; a
//                         first edit (node N-100), left unsaved; once the product reports a record written, a
//                         second edit (node N-110) at once. The runner ends the app with SIGKILL as soon as the
//                         second edit has landed, before the next interval, and reads the autosave folder at
//                         that moment: no record of the second edit may exist. The bound: the first edit's
//                         record is no more than one interval old at the end, and the work lost (the second
//                         edit) spans less than one interval. The autosave timer is one setInterval per
//                         interval setting and an edit does not restart it (apps/desktop/src/features/autosave/
//                         useAutosave.ts:147-151), so the next write comes one interval after the first record's.
//                         The relaunch shows the recovery offer; the window is read before any choice and again
//                         after a pause; Open autosave; Save Model Document… of the recovered session: its model
//                         hash is the record's, with the first edit and without the second. The store is
//                         compared, row by row, before and after the whole sequence.
//
// Observations, printed as OBSERVED lines, pass or fail nothing:
// - "Save canonical HTML", in each report journey: the control pressed once, the app watched for
//   SAVE_HTML_WINDOW_MS; the app lists the throwaway home before and after (native_smoke_checkpoint), a
//   chooser path is queued first so a chooser the press reached would take it and show nothing, and the
//   page's location, dialogs and IPC response are read. Read from the sources (below), nothing is saved.
// - Selection by keyboard, in report-pressure-free after the solve, for the model tree, the results table,
//   the displacement extremes table and the support load summary: no surface offers a select-all or copy
//   of its own; each is focused and WebKit's SelectAll editing command, the one Edit > Select All (⌘A)
//   sends, is run in the page; the selection's text, which is what Copy takes, is compared with the text
//   the surface shows; and each surface's own statement of rows shown against rows in all, and whether a
//   CSV export control sits beside it, is recorded. Observed: selectable, and the selection is what Copy
//   would take. Unobserved: that Copy is enabled for it, and the key press itself; reading either needs
//   Objective-C calls from native code, and native_smoke.rs holds no `unsafe` code (the shell's
//   unsafe-code list, pinned by the user-release reach scan, is not widened for this). Copy is not
//   pressed and the system pasteboard is neither written nor read: pressing Copy is an act a person's
//   hands observe.
// - The CSV save in report-pressure-free: whether a chooser was presented (a chooser path is queued
//   first), where under the throwaway home the file was written, and the exact words the window shows the
//   person about it.
//
// What these journeys do not check, and why: the print of the rendered report and the "Save canonical
// HTML" link. Read from the sources of this build's webview stack (tauri 2.11.1, wry 0.55.1), the
// product's print call (`contentWindow.print()` of a srcdoc iframe, RenderedReportPanel.tsx) reaches no
// native print dialog: Tauri replaces `window.print` with its native print command only in the main frame
// (tauri src/webview/plugin.rs init(), `js_init_script`, for_main_frame_only: true), and wry's WKUIDelegate
// implements no print delegate method (wry src/wkwebview/class/wry_web_view_ui_delegate.rs), which WebKit
// needs to show a print panel for a frame's `window.print()`. Print is not pressed: were that reading
// wrong, a system print panel would appear that nothing in the journey may dismiss. The data: link with
// `download` is a download navigation; wry cancels it when no download handler is set (wry
// src/wkwebview/navigation.rs, navigation_policy), and the app sets none (its window comes from
// tauri.conf.json, no `on_download`). Both are product shortfalls, repaired as a separate unit.
//
// Terms every launch runs under, made true here by construction:
// - a private copy of the app built from this worktree's head with the `native-smoke` cargo feature
//   (`scripts/appBinaryBinding.mjs`, as run.mjs), never the installed product and never a signed or
//   notarized build; its identity is checked before each launch;
// - a throwaway home under the system temporary folder (`os.tmpdir()`), removed when the journey ends
//   (kept only with --keep). Two variables redirect to it, both set to the same folder for every launch:
//   HOME, which the app's own paths follow (its store, settings, autosave folder, Downloads), and
//   CFFIXED_USER_HOME, which Foundation's home directory follows and through which the system web view
//   keeps its website data and caches (Library/WebKit/<executable>, Library/Caches/<executable>). A launch
//   is refused by name unless both are set, equal and under the temporary folder
//   (homeRedirectionRefusal). No other variable was found documented as moving the web view's data, and
//   none is set on a guess. THIS REDIRECTION IS UNTESTED UNTIL THE FIRST LAUNCH: every launch records
//   the web view's data found under the throwaway home and checks that some is there; on a run's first
//   launch, if none has appeared WEBVIEW_DATA_GRACE_MS after the shell loads, the app is ended at once and
//   the run stops with "web view data did not appear under the throwaway home". The runner reads nothing
//   under the owner's home. With it, the owner's project store, settings, Downloads and keychain are
//   neither read nor written;
// - no model, network, key, embedded agent or sign-in: the plan names none, the driver starts none, no
//   provider key is passed (`SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY` is removed from the app's environment), and
//   builds run offline (CARGO_NET_OFFLINE);
// - run inside the host's slot wrapper: the runner refuses to build or launch unless an ancestor process is
//   `t3_slot_run.sh` (the job `t3_slot.sh` starts once it holds a slot), so the slot is taken first, by a job
//   that opens nothing, and the foreground is asked for only by the launches that follow the build;
// - under the worktree build lock (`scripts/worktreeBuildLock.mjs`), from the build through the last launch;
// - before every launch, the screen session is read (`ioreg -n Root -d1 -a`, CGSSessionScreenIsLocked): if
//   it is locked the runner prints "session locked" and stops, without retrying;
// - no page reaches a printer and no system dialog is meant to appear: no journey presses Print, and the
//   only press of "Save canonical HTML" has a chooser path queued, so a chooser it reached would show nothing;
// - the system pasteboard is neither written nor read;
// - every cargo build the runner starts, directly or through `npm run build:wasm`, has at most 4 build
//   jobs: CARGO_BUILD_JOBS=4 is set in the environment (lowered to 4 when larger) before any build. The
//   engine stamps count CARGO_BUILD_* as build context, so an engine last stamped without it is rebuilt
//   once.
//
// Waits on a loaded host. Every wait is for an observed condition, bounded by a named timeout (TIMEOUTS
// below, passed to the driver in the plan); none is a fixed sleep that assumes a quiet machine. The only
// sleeps are the driver's polling (100 ms), its retry backoffs (0.5 s after a model hash still being
// computed, 5 s after a memory-pool refusal) and the 3 s observation window with the recovery offer open,
// which a slow host can only shorten, not pass or fail. While the app runs the runner samples the load
// average every 2 s; a timeout is reported as "TIMEOUT waiting for …" with its elapsed time, its limit and
// the load average read nearest its moment, so a failure by timing can be told from a failure of the product.
//
// Every check prints one line; the runner exits 1 if any check fails (2 on a refusal before any launch,
// 3 on "session locked"). Expectations are computed here on independent grounds
// (`evidence-checks.mjs`): SHA-256 and RFC 8785 by the runner, the package read from its own bytes, the
// notice compared with the text of PRD §19.3 read from docs/PRD.md, the autosave default and choices as
// DEL-02-05's ScopeOfWork.md states them (written here; nothing under execution/ is read), the store compared with a copy taken before. Nothing here reads or
// writes `expected.json`. This is evidence for the owner's review, not the owner's witness.
import { execFileSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { DatabaseSync } from "node:sqlite";
import { ENGINE_BUILDS } from "../scripts/build-wasm-engine.mjs";
import { fileIdentity, prepareSmokeApp, privateLaunchCopy } from "../scripts/appBinaryBinding.mjs";
import { checkEngine, engineStampContext } from "../scripts/engineSourceStamp.mjs";
import { BUILD_LOCK_TOKEN_ENV, acquireBuildLock, exitAfterSignals, refuseCompilerWrapperEnv, refuseHostSlotCommand, runUnderLock } from "../scripts/worktreeBuildLock.mjs";
import {
  autosaveSlot, documentSha256, dumpStore, inspectReportPackage, isCanonicalText, judgeAutosaveRecord,
  driverTimeout, homeRedirection, homeRedirectionRefusal, launchedPid, packageStatusFields, parseRunnerArgs, webviewDataEntries, parseAutosaveName, parseCsvFile, prdRequiredNotice, readStoredZip, screenLockedFromIoreg, sha256Hex, timeoutText, underHostSlot
} from "./evidence-checks.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const desktop = path.resolve(here, "..");
const project = path.resolve(desktop, "..", "..");
const tauriDir = path.join(desktop, "src-tauri");
const argv = process.argv.slice(2);
const args = new Set(argv);
// --all runs ALL_JOURNEYS (evidence-checks.mjs) in its order: autosave-recovery first, whose first launch
// is the run's shortest. reference-system is kept apart (SEPARATE_JOURNEYS): named alone, never in --all.
const parsed = parseRunnerArgs(argv);
// The named timeouts (ms). ui: a control's response to a click; step: a dialog, the shell at start, the
// settings read, an applied edit, the recovery offer, the results table for the selection observation;
// solve: one sparse solve; save: a render, a package save, a model document save; record: the driver's
// wait for the product to report the first edit's record (one interval of 1 minute plus the write, on a
// loaded host). The runner waits record + 2 × step for the second edit before it ends the app regardless,
// and PHASE_TIMEOUT_MS for one whole launch.
const TIMEOUTS = { ui: 60_000, step: 300_000, solve: 900_000, save: 900_000, record: 600_000 };
const RECORD_WAIT_MS = 10 * 60 * 1000;
const PHASE_TIMEOUT_MS = 45 * 60 * 1000;
const TAURI_CONF = JSON.parse(readFileSync(path.join(tauriDir, "tauri.conf.json"), "utf8"));
const IDENTIFIER = TAURI_CONF.identifier;
// As run.mjs: the smoke build alone turns the webview's background throttling off.
const SMOKE_TAURI_CONFIG = JSON.stringify({ app: { windows: TAURI_CONF.app.windows.map((window) => ({ ...window, backgroundThrottling: "disabled" })) } });

const refuse = (text) => { console.error(`evidence journeys: ${text}`); process.exit(2); };
// At most 4 cargo build jobs for every build started from here (inherited by npm and cargo).
if (!(Number(process.env.CARGO_BUILD_JOBS) >= 1 && Number(process.env.CARGO_BUILD_JOBS) <= 4)) process.env.CARGO_BUILD_JOBS = "4";
console.log(`evidence journeys: CARGO_BUILD_JOBS=${process.env.CARGO_BUILD_JOBS}`);
const loadNow = () => os.loadavg().map((value) => value.toFixed(2)).join(" / ");
if (process.platform !== "darwin") refuse("macOS only");
if (parsed.error) refuse(parsed.error);
const selected = parsed.journeys;
const psRows = () => execFileSync("/bin/ps", ["-axo", "pid=,ppid=,command="], { encoding: "utf8" });
if (!underHostSlot(psRows(), process.pid)) refuse("not under the host's slot wrapper; start it as tools/t3_slot.sh node apps/desktop/native-smoke/evidence-journeys.mjs …");

// A window does not paint on a locked session: no launch is made then, and none is retried.
function guardSession() {
  let plist;
  try {
    plist = execFileSync("/usr/sbin/ioreg", ["-n", "Root", "-d1", "-a"], { encoding: "utf8", maxBuffer: 16 * 1024 * 1024 });
  } catch (error) {
    console.error(`evidence journeys: the session's lock state could not be read (${error.message}); not launched`);
    return exitAfterSignals(2);
  }
  if (screenLockedFromIoreg(plist)) {
    console.error("session locked");
    return exitAfterSignals(3);
  }
  return null;
}

// Governing text and fixtures, read before anything is built.
const NOTICE = prdRequiredNotice(readFileSync(path.join(project, "docs/PRD.md"), "utf8"));
// DEL-02-05's definition of the autosave interval (ScopeOfWork.md, "*Autosave interval.*": "5 minutes by
// default, with choices of 1, 2, 5, 10, 15 and 30 minutes, and off"), stated here from that text and not
// from the product's code. It is not read at run time: maintained code reads nothing under execution/
// (tools/validation/execution_boundary.py).
const INTERVAL = { default_minutes: 5, choices: [1, 2, 5, 10, 15, 30], shortest_minutes: 1 };
const DEMO_FIXTURE = "fixtures/product_preview/invented_demo_model.json";
const EXACT_FIXTURE = "fixtures/model_operations/exact_pressure_authoring_model.json";
const demo = JSON.parse(readFileSync(path.join(project, DEMO_FIXTURE), "utf8"));
const exactDocument = JSON.parse(readFileSync(path.join(project, EXACT_FIXTURE), "utf8"));
// DEL-08-01 Design `report-user-release.md`: the shareable package's profile and the routes it carries
// (pressure-free preview-physics-2; the exact routes pressure-2 and pressure-source-1); the local package's profile.
const USER_RELEASED_PROFILE = "desktop_user_released_report_package_1";
const LOCAL_PROFILE = "desktop_local_private_report_package_1";
const CONTRACT = (route) => `openpipestress.result_semantics/0.3.0/${route}`;
const ROUTE_CONTRACTS = { "pressure-free": [CONTRACT("preview-physics-2")], exact: [CONTRACT("pressure-2"), CONTRACT("pressure-source-1")] };
// The node the autosave journey edits, and the value it enters (any value the demo does not hold).
// The two edits of the autosave journey: the first is recorded; the second, made after that record and
// lost at the end, must not be in what is recovered.
const EDIT = { node: "node:N-100", y: "0.25" };
const SECOND_EDIT = { node: "node:N-110", y: "0.75" };
const editedNode = demo.nodes.find((node) => node.id === EDIT.node);
const secondNode = demo.nodes.find((node) => node.id === SECOND_EDIT.node);
for (const [node, edit] of [[editedNode, EDIT], [secondNode, SECOND_EDIT]]) {
  if (!node || String(node.position.y) === edit.y) refuse(`${DEMO_FIXTURE} no longer has ${edit.node} at a y other than ${edit.y}`);
}
// The report journeys' observation of "Save canonical HTML": how long the app is watched after the press.
const SAVE_HTML_WINDOW_MS = 20_000;

const commandFailed = (command, commandArgs, result) =>
  new Error(`Command failed: ${command} ${commandArgs.join(" ")}${result.error ? ` (${result.error.message})` : ""}`);
async function run(command, commandArgs, cwd, env = {}) {
  refuseHostSlotCommand(command, commandArgs);
  console.log(`$ (${path.relative(project, cwd) || "."}) ${command} ${commandArgs.join(" ")}`);
  const result = await runUnderLock(command, commandArgs, { cwd, stdio: ["ignore", "inherit", "inherit"], env: { ...process.env, CARGO_NET_OFFLINE: "true", ...env } });
  if (result.error !== null || result.status !== 0) throw commandFailed(command, commandArgs, result);
}

// ---------------------------------------------------------------- build and bind (as run.mjs)

// A build opens no window, so --build-only does not read the session's lock state.
if (!parsed.buildOnly && guardSession()) await new Promise(() => {});
let buildLock;
try {
  buildLock = acquireBuildLock({ purpose: "evidence journeys" });
} catch (error) {
  refuse(error.message);
}
process.env[BUILD_LOCK_TOKEN_ENV] = buildLock.token;
let stampContext;
try {
  stampContext = engineStampContext(desktop);
} catch (error) {
  console.error(`evidence journeys: ${error.message}`);
  await exitAfterSignals(2);
}
const engineStates = () =>
  ENGINE_BUILDS.map(({ crateDirectory, destination }) => ({
    name: path.basename(destination),
    ...checkEngine({ projectRoot: project, crateDirectory, outDir: destination, context: stampContext })
  }));
const staleEngines = engineStates().filter((engine) => engine.action === "rebuild");
if (staleEngines.length > 0) {
  await run("npm", ["run", "build:wasm"], desktop);
  for (const engine of engineStates()) {
    if (engine.action !== "current") {
      console.error(`evidence journeys: engine ${engine.name} still does not match its sources after build:wasm (${engine.reason})`);
      await exitAfterSignals(2);
    }
  }
}
let app;
try {
  app = await prepareSmokeApp({
    projectRoot: project,
    desktopRoot: desktop,
    tauriDir,
    binName: "openpipestress-desktop",
    skipBuild: parsed.skipBuild,
    staleEngines: staleEngines.map((engine) => engine.name),
    engineFolders: ENGINE_BUILDS.map(({ destination }) => path.basename(destination)),
    engineHashes: () => Object.fromEntries(engineStates().map((engine) => [engine.name, engine.hash])),
    build: {
      command: ["cargo", "build", "--locked", "--offline", "--manifest-path", path.join(tauriDir, "Cargo.toml"), "--message-format=json-render-diagnostics", "--features", "native-smoke,tauri/custom-protocol"],
      tauri_config: SMOKE_TAURI_CONFIG
    },
    context: engineStampContext(tauriDir),
    runFrontend: () => run("npm", ["run", "build"], desktop),
    runCargo: async ([command, ...commandArgs], env) => {
      refuseHostSlotCommand(command, commandArgs);
      refuseCompilerWrapperEnv({ ...process.env, ...env });
      console.log(`$ (${path.relative(project, tauriDir)}) ${command} ${commandArgs.join(" ")}`);
      const result = await runUnderLock(command, commandArgs, { cwd: tauriDir, stdio: ["ignore", "pipe", "inherit"], env: { ...process.env, CARGO_NET_OFFLINE: "true", ...env } });
      if (result.error !== null || result.status !== 0) throw commandFailed(command, commandArgs, result);
      return result.stdout;
    }
  });
} catch (error) {
  console.error(error.message.startsWith("native smoke:") ? error.message : `evidence journeys: app build failed: ${error.message}`);
  await exitAfterSignals(2);
}
// --build-only: built and bound; the executable's identity is checked against the binding as each launch
// checks it, and the runner exits before any copy or launch. The executable is never run.
if (parsed.buildOnly) {
  const now = fileIdentity(app.binary);
  const same = JSON.stringify(now) === JSON.stringify(app.identity);
  console.log(`evidence journeys: built and bound, not launched: ${app.binary}`);
  console.log(`evidence journeys: identity ${JSON.stringify(app.identity)}${same ? " (the file on disk matches the binding)" : " (THE FILE ON DISK DIFFERS FROM THE BINDING)"}`);
  buildLock.release();
  await exitAfterSignals(same ? 0 : 1);
}
let launchCopy;
try {
  launchCopy = privateLaunchCopy({ desktopRoot: desktop, binary: app.binary, identity: app.identity, token: buildLock.token });
} catch (error) {
  console.error(`evidence journeys: could not copy the app for launch: ${error.message}`);
  await exitAfterSignals(2);
}
process.on("exit", () => { if (!buildLock.kept()) launchCopy.remove(); });
delete process.env[BUILD_LOCK_TOKEN_ENV];
const binary = launchCopy.executable;
console.log(`evidence journeys: launching ${binary}, a private copy of ${app.binary} (${app.identity.sha256})`);

// ---------------------------------------------------------------- launches

const checks = [];
let journey = null;
function check(name, ok, detail) {
  checks.push({ journey, name, ok: Boolean(ok), detail });
  console.log(`${ok ? "PASS" : "FAIL"}  ${journey}: ${name}${detail === undefined ? "" : `  [${typeof detail === "string" ? detail : JSON.stringify(detail)}]`}`);
}

function journeyRoot(name) {
  const root = mkdtempSync(path.join(os.tmpdir(), `swbpipe-evidence-${name}-`));
  const home = path.join(root, "home");
  for (const dir of [home, path.join(home, "Downloads"), path.join(root, "models"), path.join(root, "saved")]) mkdirSync(dir, { recursive: true });
  return { root, home, data: path.join(home, "Library", "Application Support", IDENTIFIER), saved: path.join(root, "saved"), models: path.join(root, "models") };
}

// Every file under `root`, by path relative to it; links listed, not followed. Used on throwaway homes only.
function listRelative(root) {
  const found = [];
  const pending = [root];
  while (pending.length > 0) {
    const dir = pending.pop();
    let entries = [];
    try { entries = readdirSync(dir, { withFileTypes: true }); } catch { continue; }
    for (const entry of entries) {
      const full = path.join(dir, entry.name);
      if (entry.isDirectory()) pending.push(full);
      else found.push(path.relative(root, full));
    }
  }
  return found;
}
// How long after the shell loads the web view may take to write its data under the throwaway home.
const WEBVIEW_DATA_GRACE_MS = 15_000;
let firstLaunchDone = false;

// One launch of the private copy with `plan`. With `whileRunning`, that function is polled while the
// app runs; when it returns a value the app is ended with SIGKILL and the value is returned with the outcome.
async function launch(where, plan, whileRunning = null, waitLimitMs = RECORD_WAIT_MS, launchTimeoutMs = PHASE_TIMEOUT_MS) {
  const report = path.join(where.root, `report-${plan.phase}.json`);
  console.log(`\n== ${journey}: phase ${plan.phase} (HOME=${where.home})`);
  if (guardSession()) await new Promise(() => {});
  if (JSON.stringify(fileIdentity(binary)) !== JSON.stringify(app.identity)) {
    console.error(`evidence journeys: ${binary} changed since it was checked; not launched`);
    await exitAfterSignals(2);
  }
  // Both homes redirected to the throwaway home: HOME for the app's own paths, CFFIXED_USER_HOME for
  // Foundation's home directory, through which the system web view places its data. Refused by name
  // otherwise, before anything starts.
  const env = { ...process.env, ...homeRedirection(where.home), SWBPIPE_NATIVE_SMOKE_DRIVER: path.join(here, "evidence-journeys.js"), SWBPIPE_NATIVE_SMOKE_PLAN: JSON.stringify({ ...plan, timeouts: { ...TIMEOUTS, ...(plan.timeouts ?? {}) } }), SWBPIPE_NATIVE_SMOKE_REPORT: report };
  delete env.SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY;
  const refusal = homeRedirectionRefusal(env, os.tmpdir());
  if (refusal) {
    console.error(`evidence journeys: not launched: the home redirection is refused: ${refusal}`);
    await exitAfterSignals(2);
  }
  const samples = [];
  const sample = () => { samples.push({ at: Date.now(), load: os.loadavg() }); if (samples.length > 2000) samples.shift(); };
  sample();
  const sampler = setInterval(sample, 2000);
  const launchedAt = Date.now();
  let exited = null;
  const running = runUnderLock("/usr/bin/caffeinate", ["-di", binary], { cwd: where.root, env, stdio: ["ignore", "inherit", "inherit"], timeoutMs: launchTimeoutMs })
    .then((outcome) => { exited = outcome; return outcome; });
  const killNow = () => {
    const pid = launchedPid(execFileSync("/bin/ps", ["-axo", "pid=,command="], { encoding: "utf8" }), binary);
    if (pid !== null) process.kill(pid, "SIGKILL");
    return pid;
  };
  // The web view's data under the throwaway home, watched from the start of the launch. It is decided
  // once the driver reports the shell loaded (its `shell-loaded` checkpoint), with WEBVIEW_DATA_GRACE_MS
  // more for the web view to write lazily. On the run's first launch, none there ends the app at once
  // and stops the whole run.
  const shellLoaded = `${report}.shell-loaded.json`;
  const webview = { entries: [], decided: false, shell_loaded_at: null };
  let ended = null;
  const started = Date.now();
  while (exited === null) {
    if (!webview.decided) {
      webview.entries = webviewDataEntries(listRelative(where.home), path.basename(binary));
      if (webview.entries.length > 0) webview.decided = true;
      else if (existsSync(shellLoaded)) {
        webview.shell_loaded_at ??= Date.now();
        if (Date.now() - webview.shell_loaded_at > WEBVIEW_DATA_GRACE_MS) {
          webview.decided = true;
          if (!firstLaunchDone) {
            killNow();
            await running;
            clearInterval(sampler);
            console.error("evidence journeys: web view data did not appear under the throwaway home");
            console.error(`  (the shell had loaded ${Math.round(WEBVIEW_DATA_GRACE_MS / 1000)} s earlier; the home redirection did not take, so the run stops here: ${where.home})`);
            buildLock.release();
            await exitAfterSignals(4);
          }
        }
      }
    }
    if (whileRunning && ended === null) {
      const value = whileRunning();
      if (value) {
        const at = Date.now();
        ended = { value, pid: killNow(), at, load: os.loadavg() };
      } else if (Date.now() - started > waitLimitMs) {
        console.error(`${journey}: TIMEOUT waiting for the condition to end the app (phase ${plan.phase}): ${Math.round((Date.now() - started) / 1000)} s elapsed of a ${Math.round(waitLimitMs / 1000)} s limit; load average ${loadNow()} (1/5/15 min), on ${os.cpus().length} processors`);
        ended = { value: null, pid: killNow(), at: Date.now(), timed_out: true, load: os.loadavg() };
      }
    }
    await new Promise((resolve) => setTimeout(resolve, 200));
  }
  firstLaunchDone = true;
  const outcome = await running;
  if (!webview.decided) webview.entries = webviewDataEntries(listRelative(where.home), path.basename(binary));
  check(`phase ${plan.phase}: web view data appeared under the throwaway home`, webview.entries.length > 0, webview.entries.slice(0, 10));
  observe(`phase ${plan.phase}: web view data under the throwaway home`, webview.entries.length > 0 ? webview.entries.slice(0, 20) : "none");
  clearInterval(sampler);
  if (outcome.timedOut) console.error(`${journey}: TIMEOUT of the whole launch (phase ${plan.phase}): ${Math.round((Date.now() - launchedAt) / 1000)} s elapsed of a ${launchTimeoutMs / 1000} s limit; load average ${loadNow()} (1/5/15 min), on ${os.cpus().length} processors`);
  const result = existsSync(report) ? JSON.parse(readFileSync(report, "utf8")) : { ok: false, error: `no report (exit ${outcome.status ?? outcome.signal}${outcome.timedOut ? "; the launch timed out" : ""})` };
  const timeout = driverTimeout(result.error, samples);
  if (timeout) {
    result.timeout = timeout;
    console.error(`${journey}: ${timeoutText(timeout)}`);
  }
  return { report: result, report_written: existsSync(report), ended, outcome, report_path: report, samples };
}

// A failed launch's first line, or its timeout with the load average then.
const failure = (report) => (report.timeout ? timeoutText(report.timeout) : report.error?.split("\n")[0]);
const fileOf = (file) => (existsSync(file) ? readFileSync(file) : null);
const readJson = (file) => { try { return JSON.parse(readFileSync(file, "utf8")); } catch { return null; } };
// The session's model document as saved: its RFC 8785 hash recomputed here, and whether the file is that text.
function savedDocument(file) {
  const bytes = fileOf(file);
  let document = null;
  try { document = bytes ? JSON.parse(bytes.toString("utf8")) : null; } catch { document = null; }
  return { bytes, document, canonical: bytes ? isCanonicalText(bytes) : false, sha256: document ? documentSha256(document) : null };
}

// ---------------------------------------------------------------- report journeys

async function reportJourney(route) {
  const where = journeyRoot(`report-${route}`);
  const files = {
    user_released_package: path.join(where.saved, `${route}-user-released.opsproj`),
    local_package: path.join(where.saved, `${route}-local.opsproj`),
    model_document: path.join(where.saved, `${route}-model-document.json`)
  };
  let open = null;
  let projectId = demo.project.id;
  if (route === "exact") {
    open = path.join(where.models, path.basename(EXACT_FIXTURE));
    copyFileSync(path.join(project, EXACT_FIXTURE), open);
    projectId = exactDocument.project.id;
  }
  const sentinel = path.join(where.saved, "chooser-sentinel.html");
  const csvSentinel = path.join(where.saved, "chooser-sentinel.csv");
  const { report } = await launch(where, {
    phase: "report", route, open, project_id: projectId, bundled_project_id: demo.project.id, ...files,
    observe_selection: route === "pressure-free", sentinel_chooser_path: sentinel, save_html_window_ms: SAVE_HTML_WINDOW_MS,
    csv_save: route === "pressure-free" ? { group: "displacement", family: "displacement", sentinel_chooser_path: csvSentinel } : null
  });
  check("the journey completed", report.ok, failure(report));
  check("solved: the job completed, its proof bound to the session's project", report.solve?.summary?.includes("state=completed") && report.solve?.proof?.includes(`project=${projectId}`), report.solve?.summary);
  judgeRender("first local-private render", report.render);
  if (report.render) console.log(`INFO  ${journey}: save and print controls shown: save=${report.render.save_control}; print=${report.render.print_control} (not activated; see the header)`);

  // The user-released package.
  const released = report.user_released ?? {};
  const model = savedDocument(files.model_document);
  check("the session's model document saved as its own RFC 8785 text", model.canonical && model.sha256 !== null, model.sha256);
  if (route === "pressure-free") check("the session's model is the bundled demo (RFC 8785 SHA-256 of the fixture)", model.sha256 === documentSha256(demo), model.sha256);
  check("dialog: presented with its title, no refusal, the statement unticked and Save disabled until it is ticked",
    released.title === "Release your project data in one shareable report" && released.refusal === null && released.statement_checked_initially === false && released.confirm_disabled_before_statement === true,
    { title: released.title, refusal: released.refusal, unticked: released.statement_checked_initially, disabled: released.confirm_disabled_before_statement });
  check("dialog: its scope names the session's model by the first 12 hex digits of its SHA-256", model.sha256 !== null && released.scope?.includes(`SHA-256 ${model.sha256.slice(0, 12)}…`), released.scope);
  const status = packageStatusFields(released.status);
  check("saved (REPORT-PACKAGE-SAVED) to the chosen file, the user release recorded for the session's model",
    status.outcome === "saved" && status.code === "REPORT-PACKAGE-SAVED" && existsSync(files.user_released_package)
      && released.user_release_status === `user_release_recorded=true; model_document_sha256=${model.sha256}`, { status: released.status, user_release: released.user_release_status });
  judgePackage("user-released package", files.user_released_package, released, { route, profile: USER_RELEASED_PROFILE, userReleased: true, modelSha256: model.sha256 });

  // The next report is local private.
  check("the dialog opened again starts unticked with Save disabled, and Cancel closes it", report.reopened_dialog?.statement_checked_initially === false && report.reopened_dialog.confirm_disabled && report.reopened_dialog.closed, report.reopened_dialog);
  const local = report.local ?? {};
  const localStatus = packageStatusFields(local.status);
  check("the next package is saved local private, with no user release recorded",
    localStatus.outcome === "saved" && localStatus.code === "REPORT-PACKAGE-SAVED" && local.user_release_status === null && existsSync(files.local_package), { status: local.status, user_release: local.user_release_status });
  judgePackage("next (local-private) package", files.local_package, local, { route, profile: LOCAL_PROFILE, userReleased: false, modelSha256: model.sha256 });
  // DEL-08-01 AC-010: rendered again after the user release, the report is local private.
  judgeRender("second render, after the user release", report.second_render);
  if (report.second_render) {
    observe("second render: the package area's user-release line at that moment (it describes the last package saved, not the render)", report.second_render.user_release_line_in_panel ?? "none shown");
  }
  if (report.csv) judgeCsv(where, report.csv, model.sha256, csvSentinel);
  observeSelection(report.selection);
  observeSaveHtml(where, report.save_html, sentinel);
  return where;
}

// A local-private render: rendered in the local-private context with the intent shown ticked, the
// local-private warning shown, the document's own local-private notice present and no user-release
// notice; its canonical hash, shown by the panel, is the SHA-256 the runner recomputes from the
// canonical HTML the preview holds, and the export gates passed.
function judgeRender(label, render) {
  if (!render) return check(`${label}: rendered`, false, "no render recorded");
  const html = typeof render.preview_html === "string" ? render.preview_html : null;
  const shown = render.hash?.match(/^Canonical HTML SHA-256: ([0-9a-f]{64})$/)?.[1] ?? null;
  const flat = html === null ? "" : html.replace(/\s+/g, " ");
  check(`${label}: rendered, export gates passed`, shown !== null && render.gate === "export gates passed" && !render.error, { hash: render.hash, gate: render.gate, error: render.error });
  check(`${label}: its canonical hash is the SHA-256 the runner recomputes from the canonical HTML`, html !== null && shown === sha256Hex(Buffer.from(html, "utf8")), shown);
  check(`${label}: local private: the context, the intent shown ticked and the warning shown`,
    /context=local_private/.test(render.redaction ?? "") && /blocked=false/.test(render.redaction ?? "") && render.intent_checked === true && Boolean(render.local_private_warning),
    { redaction: render.redaction, intent: render.intent_checked, warning: Boolean(render.local_private_warning) });
  check(`${label}: the document carries the local-private notice and no user-release notice`,
    flat.includes("LOCAL PRIVATE REPORT —") && !flat.includes("USER-RELEASED REPORT"), { local_private: flat.includes("LOCAL PRIVATE REPORT —"), user_release: flat.includes("USER-RELEASED REPORT") });
}

// The results table's native CSV save. Checked: the file exists as the status line names it, its
// bytes and the product's layout, its model hash the session's, and its data rows the group's own count
// as its family button states it. Observed: whether a chooser was asked for, where the file landed under
// the throwaway home, and the exact words the window shows about it.
function judgeCsv(where, csv, modelSha256, sentinel) {
  if (!csv.pressed) {
    observe(`CSV export: the control is a <${csv.tag}> link (href scheme ${(csv.href ?? "").split(":")[0] || "none"}, download="${csv.download}"), not pressed as a save`);
    return;
  }
  const saved = csv.status?.match(/^Saved (.+) \((\d+) bytes\)\.$/);
  const name = saved?.[1] ?? null;
  const found = name ? findUnder(where.home, name) : [];
  observe(`CSV export: a chooser was ${csv.chooser_paths_unused === 1 ? "not presented (the queued chooser path was never asked for)" : `asked for (queued paths left: ${csv.chooser_paths_unused})`}; the sentinel file ${existsSync(sentinel) ? "exists" : "does not exist"}`);
  observe("CSV export: written under the throwaway home at", found.length > 0 ? found.map((file) => path.relative(where.home, file)) : "not found");
  observe("CSV export: the words the window shows the person about the file", csv.status ?? "none");
  const file = found.length === 1 ? found[0] : null;
  const bytes = file ? readFileSync(file) : null;
  check("CSV export: saved, the file the status line names found once under the throwaway home with the byte count it states",
    name !== null && bytes !== null && bytes.length === Number(saved[2]), { status: csv.status, found: found.map((item) => path.relative(where.home, item)) });
  if (!bytes) return;
  const table = parseCsvFile(bytes.toString("utf8"));
  const preamble = Object.fromEntries(table.preamble.map(([key, value]) => [key, value]));
  check("CSV export: the product's layout (BOM, CRLF rows, preamble, empty row, header)", table.bom && table.crlf_only && table.preamble.length > 0 && table.header.length > 0, { preamble_keys: Object.keys(preamble) });
  check("CSV export: its model hash is the session's model by RFC 8785 SHA-256", preamble.model_hash === `sha256:${modelSha256}`, preamble.model_hash);
  const stated = Number(csv.family_count);
  check(`CSV export: its data rows are the ${csv.control} group's own count as its family button states it`, Number.isInteger(stated) && table.rows.length === stated && csv.filter === "",
    { rows: table.rows.length, family_count: csv.family_count, filter: csv.filter, filter_summary: csv.filter_summary });
  observe("CSV export: the header as the product writes it", table.header);
}

function findUnder(root, name) {
  const found = [];
  const pending = [root];
  while (pending.length > 0) {
    const dir = pending.pop();
    let entries = [];
    try { entries = readdirSync(dir, { withFileTypes: true }); } catch { continue; }
    for (const entry of entries) {
      const full = path.join(dir, entry.name);
      if (entry.isDirectory()) pending.push(full);
      else if (entry.name === name) found.push(full);
    }
  }
  return found;
}

// Observations print OBSERVED lines and are not checks: they record what the app did and pass or fail nothing.
const observe = (text, detail) => console.log(`OBSERVED  ${journey}: ${text}${detail === undefined ? "" : `  [${typeof detail === "string" ? detail : JSON.stringify(detail)}]`}`);

function observeSelection(selection) {
  if (!Array.isArray(selection)) return;
  observe("selection by keyboard: no surface has a select-all or copy of its own; Select All (the editing command ⌘A sends) selects the whole page; Copy was not pressed and the system pasteboard was neither written nor read");
  for (const item of selection) {
    if (!item.present) { observe(`${item.surface}: not on screen`); continue; }
    observe(`${item.surface}: Select All ${item.select_all_command ? "ran" : "did not run"}; the selection ${item.selection_includes_shown_text ? "includes" : "does not include"} the text the surface shows${item.selection_is_shown_text ? " and is exactly it" : ", and is not only it"}; a selection of the surface alone ${item.surface_range_is_shown_text ? "gives" : "does not give"} its shown text`,
      { focused: item.focused, selection_chars: item.selection_chars, shown_chars: item.shown_chars });
    observe(`${item.surface}: the surface states "${item.stated ?? "nothing"}"; rows in the page: ${item.rows_rendered}; CSV control beside it: ${item.csv_beside?.length ? item.csv_beside.join(", ") : "none"}`);
  }
}

function observeSaveHtml(where, saved, sentinel) {
  if (!saved?.control) { observe("Save canonical HTML: no control on screen", saved); return; }
  const listing = (name) => readJson(path.join(where.root, `report-report.json.${name}.json`))?.home_listing ?? null;
  const before = listing("before-save-html");
  const after = listing("after-save-html");
  const key = (file) => `${file.path}\u0000${file.bytes}\u0000${file.modified_unix_millis}`;
  const known = new Set((before?.files ?? []).map(key));
  const appeared = (after?.files ?? []).filter((file) => !known.has(key(file))).map((file) => file.path);
  const downloads = appeared.filter((name) => name.startsWith("Downloads/"));
  const named = appeared.filter((name) => saved.download && name.endsWith(saved.download));
  observe(`Save canonical HTML pressed once (${saved.tag} ${saved.href_scheme}: link, download="${saved.download}"); watched for ${Math.round(saved.window_ms / 1000)} s`);
  observe(`Save canonical HTML: files new or changed under the throwaway home: ${appeared.length} (in Downloads: ${downloads.length}; named as the download: ${named.length})`, appeared.slice(0, 20));
  observe(`Save canonical HTML: the queued chooser path was ${saved.chooser_paths_unused === 1 ? "never asked for (no save chooser reached)" : `taken or missing (unused paths: ${saved.chooser_paths_unused})`}; the sentinel file ${existsSync(sentinel) ? "exists" : "does not exist"}`);
  observe(`Save canonical HTML: the page ${saved.location_changed ? "navigated away" : "stayed"}; dialogs in the page: ${saved.dialogs_in_page}; the app answered IPC in ${saved.ipc_round_trip_ms} ms; status line`, saved.status_line);
  observe("Print: not pressed. The print frame's print() reaches no print delegate in this build's webview stack, and a native print panel, if one appeared against that reading, is a system dialog nothing in the journey may dismiss");
}

function judgePackage(label, file, record, { route, profile, userReleased, modelSha256 }) {
  const bytes = fileOf(file);
  let seen = null;
  try { seen = bytes ? inspectReportPackage(bytes, NOTICE) : null; } catch (error) { seen = { error: error.message }; }
  const shown = (name) => record.status?.match(new RegExp(`${name}=([0-9a-f]{64})`))?.[1] ?? null;
  check(`${label}: read by the runner as a stored archive, every member's CRC-32 recomputed`, seen && !seen.error, seen?.error ?? seen?.member_names);
  if (!seen || seen.error) return;
  check(`${label}: the file's SHA-256 and length are the receipt's`, seen.file_sha256 === shown("container_sha256") && record.status?.includes(`bytes=${seen.file_byte_length};`), seen.file_sha256);
  check(`${label}: the manifest is its own RFC 8785 text, its SHA-256 the receipt's package identity`, seen.manifest_canonical && seen.package_identity_sha256 === shown("package_identity"), seen.package_identity_sha256);
  check(`${label}: every member's SHA-256 and length, recomputed, are the manifest's`, seen.inventory_matches, seen.member_names);
  check(`${label}: profile ${profile}, not blocked`, seen.export_profile_id === profile && seen.export_blocked === false, { profile: seen.export_profile_id, blocked: seen.export_blocked });
  const read = record.read ?? {};
  check(`${label}: the product's reader re-reads it, unblocked, under the same profile and identity, ${userReleased ? "with" : "without"} a user-release record`,
    !read.error && read.export_profile_id === profile && read.export_blocked === false && read.package_identity_sha256_hex === seen.package_identity_sha256
      && read.file_sha256_hex === seen.file_sha256 && (userReleased ? read.user_release !== null && read.user_release !== undefined : read.user_release === null),
    read.error ?? { profile: read.export_profile_id, identity: read.package_identity_sha256_hex, user_release: read.user_release === null ? null : typeof read.user_release });
  check(`${label}: the rendered report carries PRD §19.3's notice (read from docs/PRD.md)`, seen.required_notice_count >= 1, seen.required_notice_count);
  check(`${label}: the result member is a ${route} result (${ROUTE_CONTRACTS[route].join(" or ")})`, ROUTE_CONTRACTS[route].includes(seen.result_contract_id), seen.result_contract_id);
  if (userReleased) {
    check(`${label}: the user-release notice present once, the local-private notice absent`, seen.user_release_notice_count === 1 && seen.local_private_notice_count === 0, { user_release: seen.user_release_notice_count, local_private: seen.local_private_notice_count });
    check(`${label}: the canonical HTML's SHA-256, recomputed, is the manifest's and the record's content binding's`,
      seen.html_sha256 !== null && seen.html_sha256 === seen.html_manifest_sha256 && seen.html_sha256 === seen.html_binding_sha256 && seen.content_binding_matches === true, seen.html_sha256);
    check(`${label}: the notice, the record and the status line name the session's model by its RFC 8785 SHA-256`,
      modelSha256 !== null && seen.user_release_notice_model_sha256 === modelSha256 && seen.user_release?.bound_to?.model_document_sha256 === modelSha256, seen.user_release_notice_model_sha256);
    check(`${label}: the record's statement is the one the dialog's tick box showed`, typeof seen.user_release?.statement === "string" && seen.user_release.statement === record.statement_label, seen.user_release?.statement);
  } else {
    check(`${label}: the local-private notice present once, no user-release notice or record`, seen.local_private_notice_count === 1 && seen.user_release_notice_count === 0 && seen.user_release === null, { local_private: seen.local_private_notice_count, user_release: seen.user_release_notice_count });
    check(`${label}: the canonical HTML's SHA-256, recomputed, is the manifest's`, seen.html_sha256 !== null && seen.html_sha256 === seen.html_manifest_sha256, seen.html_sha256);
  }
}

// ---------------------------------------------------------------- autosave journey

// The rows of every table of a store, read from a copy of its files (the database and any WAL), so no
// read touches the app's own files.
function storeDump(file, scratch) {
  if (!existsSync(file)) return null;
  mkdirSync(scratch, { recursive: true });
  const target = path.join(scratch, path.basename(file));
  for (const suffix of ["", "-wal", "-shm"]) if (existsSync(`${file}${suffix}`)) copyFileSync(`${file}${suffix}`, `${target}${suffix}`);
  const db = new DatabaseSync(target, { readOnly: true });
  try { return JSON.stringify(dumpStore(db)); } finally { db.close(); }
}

async function autosaveJourney() {
  const where = journeyRoot("autosave-recovery");
  const autosaveDir = path.join(where.data, "autosave");
  const storeFile = path.join(where.data, "openpipestress-projects.sqlite3");
  const files = { recovered: path.join(where.saved, "recovered.json") };
  const intervalMs = INTERVAL.shortest_minutes * 60_000;
  check("a fresh settings folder: the app's data folder does not exist before the first launch", !existsSync(where.data), where.data);
  const checkpoint = (name) => readJson(path.join(where.root, `report-autosave-end.json.${name}.json`));
  const records = () => (existsSync(autosaveDir) ? readdirSync(autosaveDir).filter((entry) => parseAutosaveName(entry) !== null) : []);
  // Once the second edit has landed: the end comes at once, before the interval after the first record
  // elapses, and the folder is read at that moment. If a record of another model is already there, or the
  // interval has passed, the case could not be arranged: the app is still ended, and that is reported.
  const atSecondEdit = () => {
    const second = checkpoint("second-edit");
    if (!second) return null;
    const names = records();
    // The first edit's record: the newest written before the second edit began.
    const firstName = names.filter((entry) => parseAutosaveName(entry).written_unix_millis < second.second_edit.started_at)
      .sort((a, b) => parseAutosaveName(b).written_unix_millis - parseAutosaveName(a).written_unix_millis)[0] ?? null;
    return { second, names, first_name: firstName, first_bytes: firstName ? readFileSync(path.join(autosaveDir, firstName)) : null };
  };
  const first = await launch(where, {
    phase: "autosave-end", bundled_project_id: demo.project.id, project_id: demo.project.id,
    interval_minutes: INTERVAL.shortest_minutes, edit: EDIT, second_edit: SECOND_EDIT
  }, atSecondEdit, TIMEOUTS.record + 2 * TIMEOUTS.step);
  const end = first.report;
  const before = checkpoint("before-edit");
  const edited = checkpoint("edited");
  check("first launch reached the end of its steps (no error reported)", !first.report_written || end.ok, failure(end));
  check(`on a fresh settings folder autosave is on at DEL-02-05's default, every ${INTERVAL.default_minutes} minutes, with no off notice`,
    before?.setting_on_start?.value === String(INTERVAL.default_minutes) && !before.setting_on_start.off_notice && !before.setting_on_start.off_status, before?.setting_on_start);
  check(`the interval offers off and DEL-02-05's choices (${INTERVAL.choices.join(", ")})`,
    JSON.stringify(before?.setting_on_start?.options) === JSON.stringify(["0", ...INTERVAL.choices.map(String)]), before?.setting_on_start?.options);
  check("turned off, the standing notice is shown", before?.setting_off?.value === "0" && /^Autosave is off: a crash can lose unsaved work\.$/.test(before.setting_off.off_notice ?? ""), before?.setting_off);
  check(`turned on at the shortest choice (${INTERVAL.shortest_minutes} minute), the notice is gone`, before?.setting_on?.value === String(INTERVAL.shortest_minutes) && !before.setting_on.off_notice && !before.setting_on.off_status, before?.setting_on);
  const settings = readJson(path.join(where.data, "autosave-settings.json"));
  check("the product's settings file holds the shortest interval", settings?.autosave_minutes === INTERVAL.shortest_minutes, settings);
  const snapshot = path.join(where.root, "report-autosave-end.json.before-edit.store.sqlite3");
  const storeBefore = existsSync(snapshot) ? storeDump(snapshot, path.join(where.root, "dump-before")) : null;
  check("the demo saved as a local project, and the store copied before the edits", before?.saved_project?.includes("Created local SQLite project snapshot") && storeBefore?.includes(demo.project.id), before?.saved_project);
  check("first edit (node N-100) applied and the session marked edited", edited?.edit?.summary && /Edited/.test(edited.edit.edited ?? ""), edited?.edit);
  const y = (document, id) => document?.nodes?.find((node) => node.id === id)?.position?.y;
  const ended = first.ended;
  const at = ended?.value ?? null;
  const second = at?.second?.second_edit ?? null;
  check("the product reported a record written after the first edit, then the second edit (node N-110) applied",
    /^Autosaved at /.test(at?.second?.first_record_status ?? "") && second?.summary && second.at >= second.started_at, at?.second && { status: at.second.first_record_status, second });
  check("the app ended without warning: its process found and sent SIGKILL after the second edit, and it wrote no final report",
    ended && !ended.timed_out && ended.pid !== null && at && !first.report_written, ended && { pid: ended.pid, timed_out: ended.timed_out ?? false, load_at_timeout: ended.load?.map((value) => Number(value.toFixed(2))), report: first.report_written });
  const record = at?.first_name ? { name: at.first_name, bytes: at.first_bytes } : null;
  let recordDocument = null;
  try { recordDocument = record ? JSON.parse(record.bytes.toString("utf8")) : null; } catch { recordDocument = null; }
  if (record && second) {
    const judged = judgeAutosaveRecord({ name: record.name, bytes: record.bytes, projectId: demo.project.id, endedAt: ended.at, intervalMs, notBefore: edited.edit.at - 1000 });
    check("the first edit's record: its name parses and its slot is the demo project's", judged.name_parses && judged.slot_is_project, record.name);
    check("the first edit's record is RFC 8785 text whose SHA-256 is its name's model hash",
      judged.canonical_text && judged.bytes_hash_is_name_hash && judged.document_hash_is_name_hash, judged.model_hash);
    check("the first edit's record holds the first edit (node N-100 moved) and not the second (node N-110 as in the demo)",
      y(recordDocument, EDIT.node) !== undefined && y(recordDocument, EDIT.node) !== editedNode.position.y && y(recordDocument, SECOND_EDIT.node) === secondNode.position.y,
      { [EDIT.node]: y(recordDocument, EDIT.node), [SECOND_EDIT.node]: y(recordDocument, SECOND_EDIT.node) });
    check("the first edit's record was on disk before the second edit began", judged.written_unix_millis !== null && judged.written_unix_millis < second.started_at, { written: judged.written_unix_millis, second_started: second.started_at });
    // A record of the second edit here means the second edit landed after the next tick: a timing
    // failure on a loaded host (the record's age and the load at the end say so), not a product failure.
    check("at the end no record of the second edit exists: every record on disk holds the first edit's model",
      at.names.length > 0 && at.names.every((name) => parseAutosaveName(name).model_hash === judged.model_hash),
      { records: at.names, first_record_age_at_end_ms: judged.age_at_end_ms, load_at_end: ended.load?.map((value) => Number(value.toFixed(2))) });
    check(`the end came before the next interval: the first edit's record no more than one interval (${intervalMs} ms) old`, judged.within_one_interval && judged.after_the_edit, { age_at_end_ms: judged.age_at_end_ms });
    check(`the work lost (the second edit) spans less than one interval: ${intervalMs} ms`, ended.at - second.at < intervalMs && ended.at >= second.at,
      { second_edit_at: second.at, ended_at: ended.at, lost_span_ms: ended.at - second.at, load_at_end: ended.load?.map((value) => Number(value.toFixed(2))) });
    const after = existsSync(path.join(autosaveDir, record.name)) ? readFileSync(path.join(autosaveDir, record.name)) : null;
    check("after the end the first edit's record is on disk unchanged", after && Buffer.compare(after, record.bytes) === 0, record.name);
  } else {
    check("the first edit's record was on disk when the second edit had landed", false, at ? at.names : (existsSync(autosaveDir) ? readdirSync(autosaveDir) : "no autosave folder"));
  }
  const storeAfterEnd = storeDump(storeFile, path.join(where.root, "dump-after-end"));
  check("after the end the store's rows are the copy taken before the edit", storeBefore !== null && storeAfterEnd === storeBefore);

  const relaunch = await launch(where, { phase: "autosave-reopen", bundled_project_id: demo.project.id, project_id: demo.project.id, hold_ms: 3000, model_document: files.recovered });
  const reopen = relaunch.report;
  check("relaunch: the journey completed", reopen.ok, failure(reopen));
  const hash = record ? parseAutosaveName(record.name).model_hash : null;
  check("relaunch: the offer is made for the demo's record, naming its model hash, with Open offered",
    reopen.offer?.open_offered && !reopen.offer.unreadable && hash !== null && reopen.offer.what?.includes(`(${demo.project.id})`) && reopen.offer.what.includes(`model hash ${hash.slice(0, 12)}`), reopen.offer?.what ?? reopen.offer?.unreadable);
  const quiet = (shown) => shown && !shown.edited && shown.tree_has_project && !/^Opened the autosave/.test(shown.message ?? "");
  check("relaunch: nothing is applied before a choice (no edit, no open), now and after a pause with the offer open",
    quiet(reopen.before_choice) && quiet(reopen.before_choice_later) && reopen.before_choice_later.offer_open && reopen.before_choice_later.message === reopen.before_choice.message,
    { before: reopen.before_choice, later: reopen.before_choice_later });
  check("Open autosave opens the record, no stored project changed", /^Opened the autosave of .+\. No stored project was changed\.$/.test(reopen.opened ?? ""), reopen.opened);
  const recovered = savedDocument(files.recovered);
  check("the recovered document has the model hash the record's name carries",
    recovered.canonical && hash !== null && recovered.sha256 === hash, recovered.sha256);
  check("the recovered model holds the first edit and not the second",
    y(recovered.document, EDIT.node) === y(recordDocument, EDIT.node) && y(recovered.document, EDIT.node) !== editedNode.position.y && y(recovered.document, SECOND_EDIT.node) === secondNode.position.y,
    { [EDIT.node]: y(recovered.document, EDIT.node), [SECOND_EDIT.node]: y(recovered.document, SECOND_EDIT.node) });
  const storeAfter = storeDump(storeFile, path.join(where.root, "dump-after"));
  check("the store's rows after the whole sequence are the copy taken before it", storeBefore !== null && storeAfter === storeBefore);
  return where;
}

// ---------------------------------------------------------------- the reference system (kept apart)

// Not part of --all and not run under the present leave. One launch: File > Open Model Document… of the
// reference system fixture, a solve by the desktop's own controls (its route is the document's: its
// pressure_contract names exact_pressure_v3), the solve's summary, proof and standing as the window shows
// them, the time from Run to the results table's first rows, and the session's model document. Then what a
// person meets, as OBSERVED lines, in the window's own words: on main every solve of this fixture is
// "sensitive" (M03-INTEGRITY-v1) and the desktop reads "needs recompute", so the result export and the
// report package are refused (CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE, resultExportAdapter.ts:265).
// Recorded: the standing; whether Render is offered and what it shows or refuses; the package save
// control and its refusal; the Exports section; every sentence that mentions recompute. No package is
// expected and nothing is worked around; the journey ends cleanly. Bytes: a package only if one was
// saved; the request the webview built is not exposed by the product (the job record states only the
// bound reserved for the model intake), which is recorded as such.
const REFERENCE_FIXTURE = "fixtures/reference_system/reference_system.json";
const REFERENCE_SOLVE_MS = 45 * 60 * 1000;
const REFERENCE_LAUNCH_MS = 75 * 60 * 1000;

async function referenceJourney() {
  const where = journeyRoot("reference-system");
  const document = JSON.parse(readFileSync(path.join(project, REFERENCE_FIXTURE), "utf8"));
  const open = path.join(where.models, path.basename(REFERENCE_FIXTURE));
  copyFileSync(path.join(project, REFERENCE_FIXTURE), open);
  const files = { local_package: path.join(where.saved, "reference-local.opsproj"), model_document: path.join(where.saved, "reference-model-document.json") };
  const { report } = await launch(where, {
    phase: "reference", open, project_id: document.project.id, bundled_project_id: demo.project.id, ...files,
    timeouts: { solve: REFERENCE_SOLVE_MS }
  }, null, RECORD_WAIT_MS, REFERENCE_LAUNCH_MS);
  check("the journey completed", report.ok, failure(report));
  const contract = document.pressure_contract ? `${document.pressure_contract.version}/${document.pressure_contract.mode}` : "none";
  check("opened by File > Open Model Document…", report.opened?.includes(`Opened model document ${path.basename(REFERENCE_FIXTURE)} `), report.opened);
  check("solved by the desktop's controls: the job completed, its proof bound to the reference project", report.solve?.summary?.includes("state=completed") && report.solve?.proof?.includes(`project=${document.project.id}`), report.solve?.summary);
  check(`the standing names the document's route (${contract})`, contract !== "none" && report.solve?.standing?.includes(contract), report.solve?.standing?.slice(0, 120));
  observe("solve as the window shows it", { summary: report.solve?.summary, proof: report.solve?.proof, standing: report.solve?.standing?.slice(0, 300), document_pressure_contract: contract });
  observe("time from Run to the job completed and to the results table's first rows (ms)", { completed: report.solve?.run_to_completed_ms, first_rows: report.solve?.run_to_first_rows_ms, rows: report.solve?.filter_summary });
  // What a person meets after the solve, in the window's own words; no package is expected (on main the
  // reference system solves "sensitive" and the desktop reads "needs recompute").
  const met = report.met ?? {};
  observe("the standing after the solve, as the window shows it", met.standing ?? "none shown");
  observe("the numerical health line", met.health ?? "none shown");
  observe(`Render: ${met.render?.offered ? (met.render.enabled ? "offered and pressed" : "shown but disabled") : "not offered"}`,
    { refusal: met.render?.refusal, precondition: met.render?.precondition, title: met.render?.title, hash: met.render?.hash, gate: met.render?.gate, error: met.render?.error, route: met.render?.route, redaction: met.render?.redaction, blocking_reasons: met.render?.blocking_reasons });
  observe("the report package's save control", { refusal: met.package?.refusal ?? "none shown", save_enabled: met.package?.save_enabled, title: met.package?.save_title, status_after_press: met.package?.status ?? "not pressed" });
  observe("the result export as the Exports section shows it", met.result_export ?? "not read");
  const said = [...new Set([...(met.recompute_sentences_results ?? []), ...(met.recompute_sentences_report ?? []), ...(met.recompute_sentences_exports ?? [])])];
  observe(`what tells the person what "needs recompute" asks of them: ${said.length} sentence(s) mentioning recompute`, said.length > 0 ? said : "none");
  const model = savedDocument(files.model_document);
  check("the session's model document is the opened fixture's model by RFC 8785 SHA-256", model.canonical && model.sha256 === documentSha256(document), model.sha256);
  let seen = null;
  try { seen = existsSync(files.local_package) ? inspectReportPackage(readFileSync(files.local_package), NOTICE) : null; } catch (error) { seen = { error: error.message }; }
  const members = seen && !seen.error ? readStoredZip(readFileSync(files.local_package)) : [];
  const resultMember = members.find((member) => /result/i.test(member.name) && member.name.endsWith(".json") && member.name !== "package_manifest.json");
  observe("bytes", {
    package_file: seen?.file_byte_length ?? "no package saved",
    result_member: resultMember ? { name: resultMember.name, bytes: resultMember.bytes.length } : null,
    model_document: model.bytes?.length ?? null,
    request_built_by_the_webview: "not exposed by the product without new product code",
    model_intake_reserved_bytes: report.solve?.job_resource?.ingress_reserved_bytes ?? null
  });
  // The model's vertical axis as the document states it: no field names one (searched for vertical, up
  // and gravity keys); what the document does state is the direction of its loads.
  const axisKeys = [];
  const directions = {};
  (function walk(value, at) {
    if (Array.isArray(value)) value.forEach((item, index) => walk(item, `${at}/${index}`));
    else if (value && typeof value === "object") {
      for (const [key, item] of Object.entries(value)) {
        if (/vertical|up_axis|upAxis|gravity/i.test(key)) axisKeys.push({ at: `${at}/${key}`, value: item });
        if (key === "direction" && typeof item === "string") directions[item] = (directions[item] ?? 0) + 1;
        walk(item, `${at}/${key}`);
      }
    }
  })(document, "");
  observe("the model's vertical axis as the document states it", { axis_fields: axisKeys.length > 0 ? axisKeys.slice(0, 10) : "none", load_directions: directions });
  observe("the viewport's up-axis controls as the window shows them", report.up?.controls?.length ? report.up.controls : "none found (no viewport control is labelled for an up direction or vertical axis)");
  return where;
}

// ---------------------------------------------------------------- run

let revision = null;
try { revision = execFileSync("git", ["-C", project, "rev-parse", "HEAD"], { encoding: "utf8" }).trim(); } catch { /* recorded as unknown */ }
const roots = [];
for (const name of selected) {
  journey = name;
  const where = name === "autosave-recovery" ? await autosaveJourney()
    : name === "reference-system" ? await referenceJourney()
      : await reportJourney(name === "report-exact" ? "exact" : "pressure-free");
  roots.push(where.root);
  writeFileSync(path.join(where.root, "summary.json"), JSON.stringify({ journey: name, source_revision: revision, app: { binary: app.binary, identity: app.identity }, checks: checks.filter((item) => item.journey === name) }, null, 2));
  if (parsed.keep) console.log(`evidence kept in ${where.root}`);
  else rmSync(where.root, { recursive: true, force: true });
}
const failed = checks.filter((item) => !item.ok);
console.log(`\nevidence journeys: ${checks.length - failed.length}/${checks.length} checks passed (${selected.join(", ")})`);
buildLock.release();
await exitAfterSignals(failed.length === 0 ? 0 : 1);
