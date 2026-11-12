#!/usr/bin/env node
// Evidence journeys in the launched desktop app, macOS: the report outputs on the pressure-free and
// the exact route (DEL-08-01 AC-011, DEL-12-02 AC-007), and autosave recovery after the app is ended
// without warning (DEL-02-05 AC-011 / VER-008, without its fault injection, AC-012).
//
//   tools/t3_slot.sh node apps/desktop/native-smoke/evidence-journeys.mjs --journey <names> [--skip-build] [--keep]
//   tools/t3_slot.sh node apps/desktop/native-smoke/evidence-journeys.mjs --all [--skip-build] [--keep]
//
// (`tools/t3_slot.sh` is the shared development host's slot wrapper,
// `/Users/ryan/dev/chirality-t3/tools/t3_slot.sh` there.) Journeys, each in its own throwaway HOME:
//
//   report-pressure-free  1 launch. The bundled demo: solve (sparse); an observation of selection by keyboard
//                         (below); render the local-private report; File > Save Shareable Report Package…
//                         through the dialog's own tick box and statement, the save chooser answered by
//                         script; the dialog opened again (it starts unticked) and cancelled; File > Save
//                         Report Package… (local private); Save Model Document… of the session; an
//                         observation of "Save canonical HTML" (below).
//   report-exact          1 launch. The same on the exact route, after File > Open Model Document… of
//                         `fixtures/model_operations/exact_pressure_authoring_model.json`, without the
//                         selection observation.
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
//   the surface shows. Copy is not pressed and the system pasteboard is neither written nor read. Not
//   covered: the key press itself and the Copy menu item's enabled state; reading either needs
//   Objective-C calls from native code, and native_smoke.rs may not hold `unsafe` code (the shell's
//   unsafe-code list is pinned by the user-release reach scan).
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
// - a throwaway HOME under the system temporary folder (`os.tmpdir()`), removed when the journey ends
//   (kept only with --keep); the app's store, settings, autosave folder and Downloads are under it, so the
//   owner's project store, settings, Downloads and keychain are neither read nor written;
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
// notice compared with the text of PRD §19.3 read from docs/PRD.md, the autosave default and choices read
// from DEL-02-05's ScopeOfWork.md, the store compared with a copy taken before. Nothing here reads or
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
  autosaveIntervalTerms, autosaveSlot, documentSha256, dumpStore, inspectReportPackage, isCanonicalText, judgeAutosaveRecord,
  driverTimeout, launchedPid, packageStatusFields, parseAutosaveName, prdRequiredNotice, screenLockedFromIoreg, sha256Hex, timeoutText, underHostSlot
} from "./evidence-checks.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const desktop = path.resolve(here, "..");
const project = path.resolve(desktop, "..", "..");
const tauriDir = path.join(desktop, "src-tauri");
const argv = process.argv.slice(2);
const args = new Set(argv);
const JOURNEYS = ["report-pressure-free", "report-exact", "autosave-recovery"];
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
const named = argv.includes("--journey") ? (argv[argv.indexOf("--journey") + 1] ?? "").split(",").filter(Boolean) : [];
const selected = args.has("--all") ? JOURNEYS : named;
if (selected.length === 0 || selected.some((name) => !JOURNEYS.includes(name))) refuse(`choose --all or --journey with one or more of ${JOURNEYS.join(", ")}`);
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
const INTERVAL = autosaveIntervalTerms(readFileSync(path.join(project, "execution/PKG-02/DEL-02-05/ScopeOfWork.md"), "utf8"));
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

if (guardSession()) await new Promise(() => {});
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
    skipBuild: args.has("--skip-build"),
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

// One launch of the private copy with `plan`. With `whileRunning`, that function is polled while the
// app runs; when it returns a value the app is ended with SIGKILL and the value is returned with the outcome.
async function launch(where, plan, whileRunning = null, waitLimitMs = RECORD_WAIT_MS) {
  const report = path.join(where.root, `report-${plan.phase}.json`);
  console.log(`\n== ${journey}: phase ${plan.phase} (HOME=${where.home})`);
  if (guardSession()) await new Promise(() => {});
  if (JSON.stringify(fileIdentity(binary)) !== JSON.stringify(app.identity)) {
    console.error(`evidence journeys: ${binary} changed since it was checked; not launched`);
    await exitAfterSignals(2);
  }
  const env = { ...process.env, HOME: where.home, SWBPIPE_NATIVE_SMOKE_DRIVER: path.join(here, "evidence-journeys.js"), SWBPIPE_NATIVE_SMOKE_PLAN: JSON.stringify({ ...plan, timeouts: TIMEOUTS }), SWBPIPE_NATIVE_SMOKE_REPORT: report };
  const samples = [];
  const sample = () => { samples.push({ at: Date.now(), load: os.loadavg() }); if (samples.length > 2000) samples.shift(); };
  sample();
  const sampler = setInterval(sample, 2000);
  const launchedAt = Date.now();
  delete env.SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY;
  let exited = null;
  const running = runUnderLock("/usr/bin/caffeinate", ["-di", binary], { cwd: where.root, env, stdio: ["ignore", "inherit", "inherit"], timeoutMs: PHASE_TIMEOUT_MS })
    .then((outcome) => { exited = outcome; return outcome; });
  let ended = null;
  if (whileRunning) {
    const started = Date.now();
    while (exited === null && ended === null) {
      const value = whileRunning();
      if (value) {
        const pid = launchedPid(execFileSync("/bin/ps", ["-axo", "pid=,command="], { encoding: "utf8" }), binary);
        const at = Date.now();
        if (pid !== null) process.kill(pid, "SIGKILL");
        ended = { value, pid, at };
        break;
      }
      if (Date.now() - started > waitLimitMs) {
        console.error(`${journey}: TIMEOUT waiting for the condition to end the app (phase ${plan.phase}): ${Math.round((Date.now() - started) / 1000)} s elapsed of a ${Math.round(waitLimitMs / 1000)} s limit; load average ${loadNow()} (1/5/15 min), on ${os.cpus().length} processors`);
        ended = { value: null, pid: null, at: Date.now(), timed_out: true, load: os.loadavg() };
        break;
      }
      await new Promise((resolve) => setTimeout(resolve, 200));
    }
  }
  if (ended?.timed_out) {
    const pid = launchedPid(execFileSync("/bin/ps", ["-axo", "pid=,command="], { encoding: "utf8" }), binary);
    if (pid !== null) process.kill(pid, "SIGKILL");
  }
  const outcome = await running;
  clearInterval(sampler);
  if (outcome.timedOut) console.error(`${journey}: TIMEOUT of the whole launch (phase ${plan.phase}): ${Math.round((Date.now() - launchedAt) / 1000)} s elapsed of a ${PHASE_TIMEOUT_MS / 1000} s limit; load average ${loadNow()} (1/5/15 min), on ${os.cpus().length} processors`);
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
  const { report } = await launch(where, {
    phase: "report", route, open, project_id: projectId, bundled_project_id: demo.project.id, ...files,
    observe_selection: route === "pressure-free", sentinel_chooser_path: sentinel, save_html_window_ms: SAVE_HTML_WINDOW_MS
  });
  check("the journey completed", report.ok, failure(report));
  check("solved: the job completed, its proof bound to the session's project", report.solve?.summary?.includes("state=completed") && report.solve?.proof?.includes(`project=${projectId}`), report.solve?.summary);
  check("local-private report rendered, export gates passed", /^Canonical HTML SHA-256: [0-9a-f]{64}$/.test(report.render?.hash ?? "") && report.render.gate === "export gates passed", report.render && { hash: report.render.hash, gate: report.render.gate, error: report.render.error });
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
  observeSelection(report.selection);
  observeSaveHtml(where, report.save_html, sentinel);
  return where;
}

// Observations print OBSERVED lines and are not checks: they record what the app did and pass or fail nothing.
const observe = (text, detail) => console.log(`OBSERVED  ${journey}: ${text}${detail === undefined ? "" : `  [${typeof detail === "string" ? detail : JSON.stringify(detail)}]`}`);

function observeSelection(selection) {
  if (!Array.isArray(selection)) return;
  observe("selection by keyboard: no surface has a select-all or copy of its own; Select All (the editing command ⌘A sends) selects the whole page; Copy was not pressed and the system pasteboard was neither written nor read");
  for (const item of selection) {
    if (!item.present) { observe(`${item.surface}: not on screen`); continue; }
    observe(`${item.surface}: Select All ${item.select_all_command ? "ran" : "did not run"}; the selection ${item.selection_includes_shown_text ? "includes" : "does not include"} the text the surface shows${item.selection_is_shown_text ? " and is exactly it" : ", and is not only it"}; a selection of the surface alone ${item.surface_range_is_shown_text ? "gives" : "does not give"} its shown text`,
      { focused: item.focused, selection_chars: item.selection_chars, shown_chars: item.shown_chars, rows_rendered: item.rows_rendered, summary: item.summary, page: item.page, position: item.position });
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
    check("at the end no record of the second edit exists: every record on disk holds the first edit's model",
      at.names.length > 0 && at.names.every((name) => parseAutosaveName(name).model_hash === judged.model_hash), at.names);
    check(`the end came before the next interval: the first edit's record no more than one interval (${intervalMs} ms) old`, judged.within_one_interval && judged.after_the_edit, { age_at_end_ms: judged.age_at_end_ms });
    check(`the work lost (the second edit) spans less than one interval: ${intervalMs} ms`, ended.at - second.at < intervalMs && ended.at >= second.at, { second_edit_at: second.at, ended_at: ended.at, lost_span_ms: ended.at - second.at });
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

// ---------------------------------------------------------------- run

let revision = null;
try { revision = execFileSync("git", ["-C", project, "rev-parse", "HEAD"], { encoding: "utf8" }).trim(); } catch { /* recorded as unknown */ }
const roots = [];
for (const name of selected) {
  journey = name;
  const where = name === "autosave-recovery" ? await autosaveJourney() : await reportJourney(name === "report-exact" ? "exact" : "pressure-free");
  roots.push(where.root);
  writeFileSync(path.join(where.root, "summary.json"), JSON.stringify({ journey: name, source_revision: revision, app: { binary: app.binary, identity: app.identity }, checks: checks.filter((item) => item.journey === name) }, null, 2));
  if (args.has("--keep")) console.log(`evidence kept in ${where.root}`);
  else rmSync(where.root, { recursive: true, force: true });
}
const failed = checks.filter((item) => !item.ok);
console.log(`\nevidence journeys: ${checks.length - failed.length}/${checks.length} checks passed (${selected.join(", ")})`);
buildLock.release();
await exitAfterSignals(failed.length === 0 ? 0 : 1);
