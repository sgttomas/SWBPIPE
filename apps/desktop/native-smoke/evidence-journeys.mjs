#!/usr/bin/env node
// Evidence journeys in the launched desktop app, macOS: the report outputs on the pressure-free and the exact route
// (DEL-08-01 AC-011, DEL-12-02 AC-007), and autosave recovery after the app is ended without warning (DEL-02-05
// AC-011 / VER-008, without its fault injection, AC-012).
//
//   node apps/desktop/native-smoke/evidence-journeys.mjs --build-only
//   node apps/desktop/native-smoke/evidence-journeys.mjs --journey <names>
//   node apps/desktop/native-smoke/evidence-journeys.mjs --all
//
// Run each under a host slot, as separate jobs; each takes the worktree build lock itself. --build-only builds and
// binds the `native-smoke` app exactly as `run.mjs` and `report-standing-journey.mjs` do (the same build command and
// smoke window configuration, so one binding serves all three), prints the app's identity and launches nothing. A
// launch invocation builds nothing: anything that would need building (a stale engine, frontend, app source or
// binding) refuses it by name before any launch.
//
// --all runs autosave-recovery, report-exact and report-pressure-free in that order. Journeys, each in its own
// throwaway home:
//
//   report-pressure-free  1 launch. The bundled demo: solve (sparse); an observation of selection by keyboard
//                         (below); render the local-private report; File > Save Shareable Report Package… through
//                         the dialog's own tick box and statement, the save chooser answered by script; the dialog
//                         opened again (it starts unticked) and cancelled; the report rendered again, which must be
//                         local private (DEL-08-01 AC-010); File > Save Report Package… (local private); the results
//                         table's CSV export through its own control (a native save into Downloads with no chooser);
//                         Save Model Document… of the session.
//   report-exact          1 launch. The same on the exact route, after File > Open Model Document… of
//                         `fixtures/model_operations/exact_pressure_authoring_model.json`, without the selection
//                         observation and the CSV save.
//   reference-system      Kept apart: named alone, never part of --all. 1 launch; see referenceJourney below. A
//                         refusal the window shows is recorded in its own words; nothing is worked around.
//   autosave-recovery     2 launches. A fresh settings folder: the interval as first shown, turned off (the notice)
//                         and on at DEL-02-05's shortest choice; the demo saved as a local project; a checkpoint with a
//                         read-only copy of the store; a first edit (node N-100), left unsaved; once the product
//                         reports a record written, a second edit (node N-110) at once. As soon as the driver's
//                         checkpoint of the second edit (carrying this launch's nonce) is on disk, the runner aborts
//                         the launch: runUnderLock sends SIGKILL to the launch's process group, the app among it, and
//                         the runner reads the autosave folder at that moment. The relaunch shows the recovery offer;
//                         the window is read before any choice and again after a pause; Open autosave; Save Model
//                         Document… of the recovered session. The store is compared, row by row, before and after.
//                         This is a process kill on this host: it does not show what survives a machine crash or a
//                         power loss, nor that the record's directory entry was synced to disk.
//
// Save canonical HTML and Print are native routes in this build, driven and judged by `report-standing-journey.mjs`
// (Print by `run.mjs` too); these journeys record whether the controls are shown and do not press them. Copy is not
// pressed: it would write the owner's system pasteboard (UNFINISHED_BY_MACHINE). Selection by keyboard, in
// report-pressure-free after the solve, is observed instead: each surface is focused and WebKit's SelectAll editing
// command, the one Edit > Select All sends, is run in the page; the selection's text, which is what Copy takes, is
// compared with the text the surface shows. The other acts: the model document open and save and the package saves go
// through the application's choosers (native_smoke::take_chosen_path), answered from the queue; the CSV export is a
// native save into the app's Downloads folder; the render fills an iframe's srcdoc; menu commands go through the
// menu's own dispatch; the autosave controls and the recovery offer are page controls.
//
// Confinement, as every native-smoke runner of this repository (`launch-env.mjs`, `judged-launch.mjs`): an
// environment built from nothing (buildLaunchEnv, refusedLaunchEnv: HOME and CFFIXED_USER_HOME both the throwaway
// home, TMPDIR inside it, the strict chooser switch and the runner's own variables, nothing carried from this
// process); every chooser answer inside the home (refusedPlanPaths), and the strict chooser, which ends the app (exit
// 86) at a chooser with no queued answer inside the home before any native dialog; a non-persistent web view store in
// every smoke window (smokeWindows, refusedSmokeWindows); a private copy of the bound app, its identity checked
// before each launch; the three marks of an agent operating the App on the screen checked directly before each start
// (marks-check.mjs); the run marker shared with `run.mjs`, so an unresolved run of either is recognised; after each
// launch the census of the private copy (awaitNoProcesses); the tripwire over the real user's product-named Library
// paths before the first launch and after the last (TRIPWIRE_LIMITS states what it cannot show); and closeRun on
// every exit the runner can catch, which keeps the homes, the copy, the marker and the lock when termination is
// unresolved. A launch is judged by the one rule of `launch-outcome.mjs` (exit 0, no signal, no timeout, a resolved
// census, this launch's nonce in its report); the autosave journey's first launch is judged by its own rule
// (judgeAbortedLaunch below). The embedded agent is never started; no model, network, key or sign-in is used.
//
// A throwaway home is removed only when it is one this run created (homeRemovalRefusal), every folder from it up to
// the run's folder still has the device and inode recorded at creation, and its marker holds the random token made
// for this run (removalIdentityRefusal). The folders above the run's folder (the system's temporary folder) are not
// checked; a replacement that copied the token, or a swap between the check and the removal, is not excluded. A home
// that is not removed is named, the marker is kept and the run exits non-zero (LEFT_HOME_RULE). Before removal, the
// files the checks read are copied out of the home into the journey's folder (keepOutOfHome), which is kept.
//
// Waits are for observed conditions, bounded by named timeouts (TIMEOUTS, passed to the driver); a driver timeout is
// reported with the load average sampled nearest its moment. Expectations are computed here on independent grounds
// (`evidence-checks.mjs`): SHA-256 and RFC 8785 by the runner, packages read from their own bytes, the notice from
// docs/PRD.md §19.3, the autosave choices as DEL-02-05's ScopeOfWork.md states them (written here; nothing under
// execution/ is read), the store compared with a copy taken before. Nothing here reads or writes `expected.json`.
// Evidence for the owner's review on this host; not a release qualification.
import { randomBytes } from "node:crypto";
import { execFileSync } from "node:child_process";
import * as fs from "node:fs";
import { copyFileSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, realpathSync, rmSync, writeFileSync, writeSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { DatabaseSync } from "node:sqlite";
import { ENGINE_BUILDS } from "../scripts/build-wasm-engine.mjs";
import { bindingPathFor, fileIdentity, prepareSmokeApp, privateLaunchCopy } from "../scripts/appBinaryBinding.mjs";
import { checkEngine, engineStampContext } from "../scripts/engineSourceStamp.mjs";
import { BUILD_LOCK_TOKEN_ENV, acquireBuildLock, exitAfterSignals, processStartTime, refuseCompilerWrapperEnv, refuseHostSlotCommand, runUnderLock } from "../scripts/worktreeBuildLock.mjs";
import {
  awaitNoProcesses, awaitNoProcessesSync, buildLaunchEnv, censusOf, closeRun, createThrowawayHome, keepOutOfHome, keptRelativePaths, readRunMarker,
  refusedLaunchEnv, refusedPlanPaths, refusedSmokeWindows, runMarkerRefusal, smokeWindows, statWatched, tripwireChanges, watchedLibraryPaths, writeRunMarker
} from "./launch-env.mjs";
import { newNonce, readReport, recordedOutcome } from "./launch-outcome.mjs";
import { MARK_PRESENT, UNRESOLVED_TERMINATION, judgedPhase, phaseContinuation } from "./judged-launch.mjs";
import { marksCheck, marksRefuse } from "./marks-check.mjs";
import {
  CONFINEMENT_TERM, HOME_MARKER, LEFT_HOME_RULE, NOTHING_ELSE_WATCHED, PROCESS_SCOPE, STRICT_CHOOSER_EXIT, STRICT_CHOOSER_LIMIT, TRIPWIRE_LIMITS, UNFINISHED_BY_MACHINE,
  documentSha256, driverTimeout, dumpStore, homeRemovalRefusal, inspectReportPackage, isCanonicalText, judgeAutosaveRecord, packageStatusFields, parseAutosaveName,
  parseCsvFile, parseRunnerArgs, prdRequiredNotice, readHomeMarker, readStoredZip, removalIdentityRefusal, sha256Hex, timeoutText
} from "./evidence-checks.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const desktop = path.resolve(here, "..");
const project = path.resolve(desktop, "..", "..");
const tauriDir = path.join(desktop, "src-tauri");
const NAME = "evidence journeys";
const refuseLine = (text) => console.error(`${NAME}: ${text}`);
const parsed = parseRunnerArgs(process.argv.slice(2));
if (process.platform !== "darwin") {
  refuseLine("macOS only");
  process.exit(2);
}
if (parsed.error) {
  refuseLine(parsed.error);
  process.exit(2);
}
const selected = parsed.journeys;
const buildOnly = parsed.buildOnly;

// The named timeouts (ms), passed to the driver. ui: a control's response to a click; step: a dialog, the shell at
// start, the settings read, an applied edit, the recovery offer; solve: one sparse solve; save: a render, a package
// save, a model document save; record: the driver's wait for the product to report the first edit's record (one
// interval of 1 minute plus the write, on a loaded host).
const TIMEOUTS = { ui: 60_000, step: 300_000, solve: 900_000, save: 900_000, record: 600_000 };
const PHASE_TIMEOUT_MS = 45 * 60 * 1000;
const TERMINATION_BOUND_MS = 20 * 1000;
const CAFFEINATE = "/usr/bin/caffeinate";
const DRIVER = path.join(here, "evidence-journeys.js");
const TAURI_CONF = JSON.parse(readFileSync(path.join(tauriDir, "tauri.conf.json"), "utf8"));
const IDENTIFIER = TAURI_CONF.identifier;
// As run.mjs and report-standing-journey.mjs: background throttling off and a non-persistent web view store.
const SMOKE_WINDOWS = smokeWindows(TAURI_CONF.app.windows);
const SMOKE_TAURI_CONFIG = JSON.stringify({ app: { windows: SMOKE_WINDOWS } });
// The run marker `run.mjs` and `report-standing-journey.mjs` use, so each recognises the others' unresolved run.
const RUN_MARKER = path.join(desktop, "target", "native-smoke-run.json");
const sha = (bytes) => sha256Hex(bytes);
const fileSha = (file) => (existsSync(file) ? sha(readFileSync(file)) : null);
const commandFailed = (command, commandArgs, result) =>
  new Error(`Command failed: ${command} ${commandArgs.join(" ")}${result.error ? ` (${result.error.message})` : ""}`);

{
  const record = readRunMarker(RUN_MARKER);
  if (record !== null) {
    const alive = !record.unreadable && Number.isInteger(record.pid) && typeof record.started === "string" && processStartTime(record.pid) === record.started;
    const census = !record.unreadable && typeof record.executable === "string" ? censusOf(record.executable) : { ok: false, reason: "the marker records no executable" };
    for (const line of runMarkerRefusal(RUN_MARKER, record, { alive, census })) console.error(line);
    process.exit(2);
  }
}

// ---------------------------------------------------------------- build and bind (as run.mjs), or check only

let buildLock;
try {
  buildLock = acquireBuildLock({ purpose: NAME });
} catch (error) {
  refuseLine(error.message);
  process.exit(2);
}
process.env[BUILD_LOCK_TOKEN_ENV] = buildLock.token;
async function run(command, commandArgs, cwd) {
  if (!buildOnly) throw new Error(`EVIDENCE-JOURNEYS-LAUNCH-WOULD-BUILD: ${command} ${commandArgs.join(" ")}; build first with --build-only`);
  refuseHostSlotCommand(command, commandArgs);
  console.log(`$ (${path.relative(project, cwd) || "."}) ${command} ${commandArgs.join(" ")}`);
  const result = await runUnderLock(command, commandArgs, { cwd, stdio: ["ignore", "inherit", "inherit"], env: { ...process.env, CARGO_NET_OFFLINE: "true" } });
  if (result.error !== null || result.status !== 0) throw commandFailed(command, commandArgs, result);
}
let stampContext;
try {
  stampContext = engineStampContext(desktop);
} catch (error) {
  refuseLine(error.message);
  await exitAfterSignals(2);
}
const engineStates = () =>
  ENGINE_BUILDS.map(({ crateDirectory, destination }) => ({ name: path.basename(destination), ...checkEngine({ projectRoot: project, crateDirectory, outDir: destination, context: stampContext }) }));
const staleEngines = engineStates().filter((engine) => engine.action === "rebuild");
try {
  if (staleEngines.length > 0) {
    await run("npm", ["run", "build:wasm"], desktop);
    const still = engineStates().filter((engine) => engine.action !== "current");
    if (still.length > 0) throw new Error(`engine ${still.map((engine) => `${engine.name} (${engine.reason})`).join(", ")} still does not match its sources after build:wasm`);
  }
} catch (error) {
  refuseLine(error.message);
  await exitAfterSignals(2);
}
let app;
try {
  app = await prepareSmokeApp({
    projectRoot: project,
    desktopRoot: desktop,
    tauriDir,
    binName: "openpipestress-desktop",
    // The launch never builds: with --skip-build the binding decides, and any build it would need refuses below.
    skipBuild: !buildOnly,
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
      if (!buildOnly) throw new Error(`EVIDENCE-JOURNEYS-LAUNCH-WOULD-BUILD: ${command} ${commandArgs.join(" ")}; build first with --build-only`);
      refuseHostSlotCommand(command, commandArgs);
      refuseCompilerWrapperEnv({ ...process.env, ...env });
      console.log(`$ (${path.relative(project, tauriDir)}) ${command} ${commandArgs.join(" ")}`);
      const result = await runUnderLock(command, commandArgs, { cwd: tauriDir, stdio: ["ignore", "pipe", "inherit"], env: { ...process.env, CARGO_NET_OFFLINE: "true", ...env } });
      if (result.error !== null || result.status !== 0) throw commandFailed(command, commandArgs, result);
      return result.stdout;
    }
  });
} catch (error) {
  refuseLine(error.message.startsWith("native smoke:") ? error.message : `app build or binding failed: ${error.message}`);
  await exitAfterSignals(2);
}
const bindingSha = fileSha(bindingPathFor(desktop));
console.log(`${NAME}: app ${app.binary} (sha256 ${app.identity.sha256}, ${app.identity.size} bytes); binding record sha256 ${bindingSha}`);
if (buildOnly) {
  delete process.env[BUILD_LOCK_TOKEN_ENV];
  buildLock.release();
  console.log(`${NAME}: built and bound; nothing launched`);
  await exitAfterSignals(0);
}

// ---------------------------------------------------------------- governing text and fixtures

const NOTICE = prdRequiredNotice(readFileSync(path.join(project, "docs/PRD.md"), "utf8"));
// DEL-02-05's definition of the autosave interval (ScopeOfWork.md, "*Autosave interval.*": "5 minutes by default,
// with choices of 1, 2, 5, 10, 15 and 30 minutes, and off"), stated here from that text and not from the product's
// code. It is not read at run time: maintained code reads nothing under execution/ (tools/validation/execution_boundary.py).
const INTERVAL = { default_minutes: 5, choices: [1, 2, 5, 10, 15, 30], shortest_minutes: 1 };
const DEMO_FIXTURE = "fixtures/product_preview/invented_demo_model.json";
const EXACT_FIXTURE = "fixtures/model_operations/exact_pressure_authoring_model.json";
const demo = JSON.parse(readFileSync(path.join(project, DEMO_FIXTURE), "utf8"));
const exactDocument = JSON.parse(readFileSync(path.join(project, EXACT_FIXTURE), "utf8"));
// The packages' profiles (DEL-08-01's user-release design) and the result contract each journey's fixture solves to.
// The pressure-free journey solves the bundled Euler-Bernoulli demo, so its packages carry preview-physics-2 (the
// shareable path admits only that identity on the pressure-free route: report_package user_release.rs
// check_pressure_free); the exact journey's carry one of the exact identities report_package admits (document_form.rs
// ADMITTED_SEMANTIC_CONTRACTS). Written out in full so that the identity inventory scan
// (tests/test_preview_identity_inventory.py) finds and classifies them.
const USER_RELEASED_PROFILE = "desktop_user_released_report_package_1";
const LOCAL_PROFILE = "desktop_local_private_report_package_1";
const ROUTE_CONTRACTS = {
  "pressure-free": ["openpipestress.result_semantics/0.3.0/preview-physics-2"],
  exact: ["openpipestress.result_semantics/0.3.0/pressure-3", "openpipestress.result_semantics/0.3.0/pressure-source-1"]
};
// The two edits of the autosave journey: the first is recorded; the second, made after that record and lost at the
// end, must not be in what is recovered.
const EDIT = { node: "node:N-100", y: "0.25" };
const SECOND_EDIT = { node: "node:N-110", y: "0.75" };
const editedNode = demo.nodes.find((node) => node.id === EDIT.node);
const secondNode = demo.nodes.find((node) => node.id === SECOND_EDIT.node);
for (const [node, edit] of [[editedNode, EDIT], [secondNode, SECOND_EDIT]]) {
  if (!node || String(node.position.y) === edit.y) {
    refuseLine(`${DEMO_FIXTURE} no longer has ${edit.node} at a y other than ${edit.y}`);
    await exitAfterSignals(2);
  }
}

// ---------------------------------------------------------------- the launch copy and the run's state

let launchCopy;
try {
  launchCopy = privateLaunchCopy({ desktopRoot: desktop, binary: app.binary, identity: app.identity, token: buildLock.token });
} catch (error) {
  refuseLine(`could not copy the app for launch: ${error.message}`);
  await exitAfterSignals(2);
}
delete process.env[BUILD_LOCK_TOKEN_ENV];
const binary = launchCopy.executable;
const WATCHED = watchedLibraryPaths(path.join(os.userInfo().homedir, "Library"), { identifier: IDENTIFIER, names: [TAURI_CONF.productName, "openpipestress-desktop"] });
const state = { launched: false, termination: null, watchedBefore: null, tripwireDone: false, closed: false };
const launches = {};
const checks = [];
let journey = null;
let revision = null;
let dirty = null;
try {
  revision = execFileSync("git", ["-C", project, "rev-parse", "HEAD"], { encoding: "utf8" }).trim();
  dirty = execFileSync("git", ["-C", project, "status", "--porcelain"], { encoding: "utf8" }).trim().split("\n").filter(Boolean);
} catch { /* recorded as unknown */ }

// The run's folder, kept as its evidence; resolved (no links), as the throwaway homes are. The random token every
// throwaway home of this run carries in its marker, and the device and inode of each folder this run makes, recorded
// when made (removalIdentityRefusal).
const RUN_ROOT = realpathSync(mkdtempSync(path.join(os.tmpdir(), "swbpipe-evidence-journeys-")));
const TOKEN = randomBytes(16).toString("hex");
const identities = new Map();
const remember = (dir) => { const seen = lstatSync(dir); identities.set(dir, { dev: seen.dev, ino: seen.ino }); };
remember(RUN_ROOT);
const createdHomes = new Set();
const removedHomes = new Set();
const keptHomes = new Set();
const leftHomes = new Map();
// From here every exit the runner can catch closes the run (closeOnExit).
process.on("exit", closeOnExit);
console.log(`${NAME}: launching ${binary}, a private copy of ${app.binary} (${app.identity.sha256}); evidence in ${RUN_ROOT}`);

function check(name, ok, detail) {
  checks.push({ journey, name, ok: Boolean(ok), detail });
  console.log(`${ok ? "PASS" : "FAIL"}  ${journey}: ${name}${detail === undefined ? "" : `  [${typeof detail === "string" ? detail.slice(0, 600) : JSON.stringify(detail)?.slice(0, 600)}]`}`);
}
// Observations print OBSERVED lines and are not checks: they record what the app did and pass or fail nothing.
const observe = (text, detail) => console.log(`OBSERVED  ${journey}: ${text}${detail === undefined ? "" : `  [${typeof detail === "string" ? detail : JSON.stringify(detail)}]`}`);

// A journey's folder in the run's folder, and its throwaway home inside it (created empty by createThrowawayHome),
// marked with the run's token. Every chooser answer the journey queues lies in the home's chooser folder.
function journeyRoot(name) {
  const root = realpathSync(mkdtempSync(path.join(RUN_ROOT, `swbpipe-evidence-${name}-`)));
  remember(root);
  const { home, tmpdir } = createThrowawayHome(root);
  remember(home);
  createdHomes.add(home);
  writeFileSync(path.join(home, HOME_MARKER), TOKEN, { flag: "wx" });
  const chooser = path.join(home, "chooser");
  const models = path.join(chooser, "models");
  const saved = path.join(chooser, "saved");
  for (const dir of [chooser, models, saved]) mkdirSync(dir);
  return { root, home, tmpdir, data: path.join(home, "Library", "Application Support", IDENTIFIER), models, saved };
}

// Copies the files the checks read out of `home` into its journey's folder (once), then removes the home when it is
// one this run created, its folders up to the run's folder are the recorded ones and its marker holds this run's
// token. Returns null, or why it was left. Called only once the census found no process of the private copy.
function removeHome(home) {
  if (removedHomes.has(home)) return null;
  if (!keptHomes.has(home)) {
    keptHomes.add(home);
    try {
      keepOutOfHome(home, path.dirname(home), keptRelativePaths(IDENTIFIER));
    } catch (error) {
      say(`${NAME}: the files of ${home} could not all be copied out (${error.message}); the home is left`);
      leftHomes.set(home, `copying its files out failed: ${error.message}`);
      return error.message;
    }
  }
  const refusal = homeRemovalRefusal(home, createdHomes, RUN_ROOT)
    ?? removalIdentityRefusal(home, RUN_ROOT, identities, lstatSync, { token: TOKEN, readMarker: (dir) => readHomeMarker(dir, fs) });
  if (refusal) {
    leftHomes.set(home, refusal);
    say(`${NAME}: not removed: ${refusal}`);
    return refusal;
  }
  try {
    rmSync(home, { recursive: true });
  } catch (error) {
    leftHomes.set(home, `removal failed: ${error.message}`);
    say(`${NAME}: not removed: ${home}: ${error.message}`);
    return error.message;
  }
  leftHomes.delete(home);
  removedHomes.add(home);
  return null;
}
// Written straight to standard error: the exit handler runs synchronously, and a pipe write could be lost.
function say(text) {
  try { writeSync(2, `${text}\n`); } catch { /* the run's record holds it */ }
}

// ---------------------------------------------------------------- launches

// The environment and the refusals every launch passes before it starts; null once refused (the caller exits 2).
function launchEnv(where, plan, report) {
  const env = buildLaunchEnv({
    parent: process.env, home: where.home, tmpdir: where.tmpdir,
    runner: { SWBPIPE_NATIVE_SMOKE_DRIVER: DRIVER, SWBPIPE_NATIVE_SMOKE_PLAN: JSON.stringify(plan), SWBPIPE_NATIVE_SMOKE_REPORT: report }
  });
  const refused = [...refusedLaunchEnv(env), ...refusedPlanPaths(plan, where.home), ...refusedSmokeWindows(SMOKE_WINDOWS)];
  if (JSON.stringify(fileIdentity(binary)) !== JSON.stringify(app.identity)) refused.push(`${binary}, the private copy of the bound app, changed since it was checked`);
  for (const line of refused) refuseLine(`${line}; not launched`);
  return refused.length > 0 ? null : env;
}
// Directly before each start: the tripwire's baseline (before the first launch), the run marker naming this
// journey's home, and the launch state the exit handler reads.
function beforeStart(where) {
  if (state.watchedBefore === null) state.watchedBefore = statWatched(WATCHED);
  writeRunMarker(RUN_MARKER, { pid: process.pid, started: processStartTime(process.pid), home: where.home, executable: binary, evidence_root: RUN_ROOT, created: new Date().toISOString() });
  state.launched = true;
  state.termination = null;
}
const runRecord = () => ({ run_folder: RUN_ROOT, source_revision: revision, uncommitted: dirty, journeys: selected, checks });
// A plan for the driver: the phase's own fields, this launch's nonce and the named timeouts.
const planFor = (plan, nonce) => ({ ...plan, nonce, timeouts: { ...TIMEOUTS, ...(plan.timeouts ?? {}) } });

// One judged launch (`judged-launch.mjs`): the marks, the start through runUnderLock, the census and the one rule over
// the exit and the report; a mark present or an unresolved census ends the run there (phaseContinuation). Returns the
// launch's report (a failed launch's included), with a strict chooser stop named by the act the driver last recorded.
async function launch(where, plan, { timeoutMs = PHASE_TIMEOUT_MS } = {}) {
  const phase = `${journey}/${plan.phase}`;
  const report = path.join(where.root, `report-${plan.phase}.json`);
  const nonce = newNonce();
  console.log(`\n== ${journey}: phase ${plan.phase} (HOME=${where.home})`);
  const env = launchEnv(where, planFor(plan, nonce), report);
  if (env === null) return exitAfterSignals(2);
  const seen = await judgedPhase({
    phase, checkName: `phase ${plan.phase}: the launch ended cleanly and its report is this run's`, launches, check,
    summaryPath: path.join(RUN_ROOT, "summary.json"), summary: runRecord,
    executable: binary, command: CAFFEINATE, commandArgs: ["-di", binary], cwd: where.root, env, timeoutMs, reportPath: report, nonce,
    censusBoundMs: TERMINATION_BOUND_MS, beforeStart: () => beforeStart(where)
  }).then(phaseContinuation({ prefix: NAME, phase, state, evidenceRoot: RUN_ROOT }));
  strictChooserStop(plan.phase, launches[phase]?.outcome, report);
  const timeout = driverTimeout(seen?.error, []);
  if (timeout && seen && typeof seen === "object") seen.timeout = timeout;
  if (timeout) console.error(`${journey}: ${timeoutText(timeout)}; load average now ${os.loadavg().map((value) => value.toFixed(2)).join(" / ")}`);
  return seen;
}
// The strict chooser's stop (exit 86) names the act the driver last recorded (its `act` checkpoint).
function strictChooserStop(phase, outcome, report) {
  if (outcome?.status !== STRICT_CHOOSER_EXIT) return;
  const act = readReport(`${report}.act.json`).report?.act ?? "unnamed (the driver recorded no act)";
  check(`phase ${phase}: no chooser was asked without a queued answer inside the home`, false, `the app ended with the strict chooser's exit code ${STRICT_CHOOSER_EXIT} during the act: ${act}`);
}

// The autosave journey's first launch, which the runner ends itself. The same refusals, marks, start, census and
// record as a judged launch, with one difference: once `condition()` returns a value (polled every 100 ms) or
// `waitLimitMs` passes, the runner aborts the launch, and runUnderLock sends SIGKILL to the launch's own process group
// (caffeinate and the app under it), the same group its time limit would kill. An unresolved census ends the run.
// Returns { ended, outcome, report }; judgeAbortedLaunch decides whether it is the planned end.
async function launchUntilEnded(where, plan, condition, { waitLimitMs, timeoutMs = PHASE_TIMEOUT_MS }) {
  const phase = `${journey}/${plan.phase}`;
  const report = path.join(where.root, `report-${plan.phase}.json`);
  const nonce = newNonce();
  console.log(`\n== ${journey}: phase ${plan.phase} (HOME=${where.home})`);
  const env = launchEnv(where, planFor(plan, nonce), report);
  if (env === null) return exitAfterSignals(2);
  const marks = marksCheck();
  console.log(`phase ${phase}: marks, directly before the launch: ${JSON.stringify(marks)}`);
  if (marksRefuse(marks)) {
    launches[phase] = { marks, launched: false };
    writeFileSync(path.join(RUN_ROOT, "summary.json"), JSON.stringify({ ...runRecord(), launches, refused: MARK_PRESENT }, null, 2));
    refuseLine(`${MARK_PRESENT}: an agent may be operating the App on the screen, or the marks could not be checked; not launched`);
    return exitAfterSignals(2);
  }
  beforeStart(where);
  const controller = new AbortController();
  const startedAt = Date.now();
  let ended = null;
  const poll = setInterval(() => {
    if (ended !== null) return;
    let value = null;
    try { value = condition(nonce); } catch { value = null; }
    if (value) ended = { value, at: Date.now(), waited_out: false, load: os.loadavg() };
    else if (Date.now() - startedAt > waitLimitMs) ended = { value: null, at: Date.now(), waited_out: true, load: os.loadavg() };
    if (ended !== null) controller.abort();
  }, 100);
  const started = new Date().toISOString();
  const result = await runUnderLock(CAFFEINATE, ["-di", binary], { cwd: where.root, env, stdio: ["ignore", "inherit", "inherit"], timeoutMs, abort: controller.signal });
  clearInterval(poll);
  const endedIso = new Date().toISOString();
  state.termination = await awaitNoProcesses(binary, { boundMs: TERMINATION_BOUND_MS });
  const outcome = { ...recordedOutcome({ status: result.status, signal: result.signal, timedOut: result.timedOut, census: state.termination, nonce, started, ended: endedIso }), aborted: result.aborted === true, started_command: result.started === true };
  launches[phase] = { marks, outcome, abort: ended && { at: ended.at, waited_out: ended.waited_out } };
  if (!state.termination?.resolved) {
    writeFileSync(path.join(RUN_ROOT, "summary.json"), JSON.stringify({ ...runRecord(), launches, stopped: UNRESOLVED_TERMINATION }, null, 2));
    refuseLine(`${UNRESOLVED_TERMINATION} after phase ${phase}: ${state.termination?.reason ?? "no census"}; no further phase starts; evidence in ${RUN_ROOT}`);
    return exitAfterSignals(1);
  }
  console.log(`phase ${phase}: the census finds no process running the private app copy`);
  strictChooserStop(plan.phase, outcome, report);
  return { ended, outcome, report: readReport(report) };
}

// The planned end of the autosave journey's first launch, from what happened, never from the request alone: the
// intended checkpoint carrying this launch's nonce was on disk when the end was asked for (not the wait running
// out); runUnderLock sent the SIGKILL (`aborted`, which it sets only when it sent it before the command closed and
// not after its time limit fired); the launch's command ended on SIGKILL, with no exit status and no time limit; the
// census then found no process of the private copy; and the app wrote no final report. A natural exit, a time limit
// or an exit before the SIGKILL fails it.
function judgeAbortedLaunch(first) {
  const outcome = first.outcome;
  return {
    intended_checkpoint: first.ended !== null && first.ended.waited_out === false && first.ended.value !== null,
    sigkill_sent: outcome.aborted === true,
    ended_on_sigkill: outcome.signal === "SIGKILL" && outcome.status === null && outcome.timed_out === false,
    terminated: outcome.census?.resolved === true,
    no_final_report: first.report.report === null
  };
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
  const csvSentinel = path.join(where.saved, "chooser-sentinel.csv");
  const report = await launch(where, {
    phase: "report", route, open, project_id: projectId, bundled_project_id: demo.project.id, ...files,
    observe_selection: route === "pressure-free",
    csv_save: route === "pressure-free" ? { group: "displacement", family: "displacement", sentinel_chooser_path: csvSentinel } : null
  });
  check("the journey completed", report.ok, failure(report));
  check("solved: the job completed, its proof bound to the session's project", report.solve?.summary?.includes("state=completed") && report.solve?.proof?.includes(`project=${projectId}`), report.solve?.summary);
  judgeRender("first local-private render", report.render);

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
  observeUnfinished(report.render);
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

// Save canonical HTML and Print: their controls recorded as shown or not, not pressed here (report-standing-journey.mjs
// drives and judges both native routes). Copy: unfinished by machine, observed by a person.
function observeUnfinished(render) {
  observe("Save canonical HTML and Print: not pressed in this journey (their native routes are driven and judged by report-standing-journey.mjs)", { save_control_shown: render?.save_control ?? null, print_control_shown: render?.print_control ?? null });
  for (const { act, why } of UNFINISHED_BY_MACHINE) observe(`${act}: unfinished by machine, observed by a person; not pressed`, { why });
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
  // The driver's checkpoints of the first launch, each carrying that launch's nonce.
  const checkpoint = (name) => readJson(path.join(where.root, `report-autosave-end.json.${name}.json`));
  const records = () => (existsSync(autosaveDir) ? readdirSync(autosaveDir).filter((entry) => parseAutosaveName(entry) !== null) : []);
  // Once the second edit has landed: the end comes at once, before the interval after the first record
  // elapses, and the folder is read at that moment. If a record of another model is already there, or the
  // interval has passed, the case could not be arranged: the app is still ended, and that is reported.
  const atSecondEdit = (nonce) => {
    const second = checkpoint("second-edit");
    if (!second || second.nonce !== nonce) return null;
    const names = records();
    // The first edit's record: the newest written before the second edit began.
    const firstName = names.filter((entry) => parseAutosaveName(entry).written_unix_millis < second.second_edit.started_at)
      .sort((a, b) => parseAutosaveName(b).written_unix_millis - parseAutosaveName(a).written_unix_millis)[0] ?? null;
    return { second, names, first_name: firstName, first_bytes: firstName ? readFileSync(path.join(autosaveDir, firstName)) : null };
  };
  const first = await launchUntilEnded(where, {
    phase: "autosave-end", bundled_project_id: demo.project.id, project_id: demo.project.id,
    interval_minutes: INTERVAL.shortest_minutes, edit: EDIT, second_edit: SECOND_EDIT
  }, atSecondEdit, { waitLimitMs: TIMEOUTS.record + 2 * TIMEOUTS.step });
  const end = first.report.report ?? { ok: false, error: first.report.problem };
  const reportWritten = first.report.report !== null;
  const before = checkpoint("before-edit");
  const edited = checkpoint("edited");
  check("first launch: the driver reported no error before it was ended", !reportWritten || end.ok, reportWritten ? failure(end) : "no final report (the launch was ended)");
  for (const name of ["before-edit", "edited", "second-edit"]) {
    const seen = checkpoint(name);
    check(`first launch: the ${name} checkpoint carries this launch's nonce`, seen !== null && seen.nonce === first.outcome.nonce, seen?.nonce ?? null);
  }
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
  // The planned end, judged from what happened (judgeAbortedLaunch): every part must hold.
  const planned = judgeAbortedLaunch(first);
  check("the app ended without warning: after the second edit's checkpoint, runUnderLock sent SIGKILL to the launch's process group, the launch ended on SIGKILL (no exit status, no time limit), the census found no process of the private copy, and no final report was written",
    Object.values(planned).every(Boolean), { ...planned, outcome: first.outcome, waited_out: ended?.waited_out ?? null, load_at_end: ended?.load?.map((value) => Number(value.toFixed(2))) });
  const record = at?.first_name ? { name: at.first_name, bytes: at.first_bytes } : null;
  let recordDocument = null;
  try { recordDocument = record ? JSON.parse(record.bytes.toString("utf8")) : null; } catch { recordDocument = null; }
  if (record && second) {
    const judged = judgeAutosaveRecord({ name: record.name, bytes: record.bytes, projectId: demo.project.id, endedAt: ended.at, intervalMs, notBefore: (edited?.edit?.at ?? Number.POSITIVE_INFINITY) - 1000 });
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

  const reopen = await launch(where, { phase: "autosave-reopen", bundled_project_id: demo.project.id, project_id: demo.project.id, hold_ms: 3000, model_document: files.recovered });
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

// Not part of --all. One launch: File > Open Model Document… of the
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
  const report = await launch(where, {
    phase: "reference", open, project_id: document.project.id, bundled_project_id: demo.project.id, ...files,
    timeouts: { solve: REFERENCE_SOLVE_MS }
  }, { timeoutMs: REFERENCE_LAUNCH_MS });
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

for (const name of selected) {
  journey = name;
  const where = name === "autosave-recovery" ? await autosaveJourney()
    : name === "reference-system" ? await referenceJourney()
      : await reportJourney(name === "report-exact" ? "exact" : "pressure-free");
  writeFileSync(path.join(where.root, "summary.json"), JSON.stringify({ journey: name, source_revision: revision, app: { binary: app.binary, identity: app.identity, binding_record_sha256: bindingSha }, launches: Object.fromEntries(Object.entries(launches).filter(([phase]) => phase.startsWith(`${name}/`))), checks: checks.filter((item) => item.journey === name) }, null, 2));
  // Every launch of the journey was censused (a launch whose census was unresolved ended the run), so its home may go.
  if (removeHome(where.home) !== null) check("its throwaway home was removed", false, leftHomes.get(where.home));
}
journey = "run";
// The tripwire after the last launch, in the run's checks.
if (state.watchedBefore !== null) {
  const changes = tripwireChanges(state.watchedBefore, statWatched(WATCHED));
  state.tripwireDone = true;
  check(`the real user's Library: none of the ${WATCHED.length} product-named paths appeared, disappeared or changed`, changes.length === 0, changes);
}
const failed = checks.filter((item) => !item.ok);
console.log(`\n${NAME}: ${checks.length - failed.length}/${checks.length} checks passed (${selected.join(", ")}); evidence in ${RUN_ROOT}`);
buildLock.release();
await exitAfterSignals(failed.length === 0 ? 0 : 1);

// Close the run on every exit the runner can catch: the census if a launch was not censused, the tripwire through
// closeRun, then retention (an unresolved termination keeps the homes, the copy, the marker and the lock) or the
// removal of each home this run created (removeHome, with its checks) and of the private copy. A home left keeps the
// marker and fails the run. The run's folder is kept, with its record.
function closeOnExit(code) {
  if (state.closed) return;
  state.closed = true;
  let exitCode = code ?? process.exitCode ?? 0;
  let outcome = "completed";
  try {
    const termination = state.launched && state.termination === null ? awaitNoProcessesSync(binary, { boundMs: TERMINATION_BOUND_MS }) : (state.launched ? state.termination : null);
    const tripwire = state.watchedBefore === null ? null : { before: state.watchedBefore, after: state.tripwireDone ? null : statWatched(WATCHED), done: state.tripwireDone };
    const closed = closeRun({ exitCode, termination, lockKept: buildLock.kept(), tripwire });
    for (const text of closed.lines) say(`${NAME}: ${text}`);
    exitCode = closed.exitCode;
    if (closed.retain) {
      outcome = "retained: termination unresolved";
      retain(termination?.reason ?? "the worktree build lock was kept");
    } else {
      for (const home of createdHomes) removeHome(home);
      for (const [home, why] of leftHomes) say(`${NAME}: THROWAWAY HOME LEFT: ${home} (${why}). ${LEFT_HOME_RULE}`);
      if (leftHomes.size > 0) {
        if (exitCode === 0) exitCode = 1;
        outcome = "a throwaway home was left";
        retain(`throwaway home left: ${[...leftHomes.keys()].join(", ")}`);
      } else {
        launchCopy.remove();
        rmSync(RUN_MARKER, { force: true });
      }
    }
  } catch (error) {
    if (exitCode === 0) exitCode = 1;
    outcome = `closing the run failed: ${error.message}`;
    say(`${NAME}: ${outcome}; the run marker stays`);
  }
  process.exitCode = exitCode;
  try {
    writeFileSync(path.join(RUN_ROOT, "run-record.json"), JSON.stringify({
      ...runRecord(), exit_code: exitCode, close: outcome, app: { binary: app.binary, identity: app.identity, binding_record_sha256: bindingSha }, launches,
      homes_created: [...createdHomes], homes_removed: [...removedHomes], homes_left: [...leftHomes].map(([home, why]) => ({ home, why })), left_home_rule: LEFT_HOME_RULE,
      strict_chooser: { exit_code: STRICT_CHOOSER_EXIT, limit: STRICT_CHOOSER_LIMIT }, process_scope: PROCESS_SCOPE, tripwire: { watched: WATCHED, limits: TRIPWIRE_LIMITS },
      unfinished_by_machine: UNFINISHED_BY_MACHINE, confinement_term: CONFINEMENT_TERM,
      disclosure: [
        "Observed: the product-named paths under the real user's Library, by the tripwire before the first launch and after the last, and each throwaway home.",
        "This runner neither lists nor reads the owner's home folder or its Desktop, Documents and Downloads folders.",
        NOTHING_ELSE_WATCHED
      ]
    }, null, 2));
    say(`${NAME}: run record in ${RUN_ROOT}; throwaway homes removed: ${removedHomes.size} of ${createdHomes.size}`);
  } catch (error) {
    say(`${NAME}: the run's record could not be written: ${error.message}`);
  }

  // Nothing more is removed. The marker records why; the worktree lock, released by its own exit handler when no
  // tracked group remained, is taken again under a purpose naming the marker.
  function retain(reason) {
    try {
      writeRunMarker(RUN_MARKER, { pid: process.pid, started: processStartTime(process.pid), home: [...createdHomes].filter((home) => !removedHomes.has(home)).join(" ") || "(none left)", executable: binary, evidence_root: RUN_ROOT, reason, retained: new Date().toISOString() });
    } catch (error) {
      say(`${NAME}: the run marker ${RUN_MARKER} could not be written: ${error.message}`);
    }
    if (!buildLock.kept()) {
      try {
        acquireBuildLock({ purpose: `${NAME}: cleanup not completed, see ${RUN_MARKER}` });
        say(`${NAME}: the worktree build lock is retained for the unresolved run`);
      } catch (error) {
        say(`${NAME}: the worktree build lock could not be retained: ${error.message.split("\n")[0]}`);
      }
    }
    say(`${NAME}: cleanup NOT completed (${reason}); retained: private app copy ${binary}, run marker ${RUN_MARKER}`);
  }
}
