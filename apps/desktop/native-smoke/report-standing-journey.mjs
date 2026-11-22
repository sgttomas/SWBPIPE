#!/usr/bin/env node
// Launched-app witness of the rendered report of a solved exact-route result that is not numerically
// eligible (DEL-08-01 REQ-017 and REQ-024; PR-1a), macOS.
//
//   node apps/desktop/native-smoke/report-standing-journey.mjs --build-only
//   node apps/desktop/native-smoke/report-standing-journey.mjs [--discard]
//
// Run each under a host slot (`t3_slot.sh`), as two separate jobs; each takes the worktree build lock itself.
//
// --build-only builds and binds the `native-smoke` app exactly as `run.mjs` does (engine stamps, frontend, cargo
// with the `native-smoke` feature, the binding record), prints the binding record's SHA-256 and the app's identity,
// and launches nothing. The launch invocation builds nothing: when anything would need building (a stale engine,
// frontend, app source or binding), it refuses by name before any launch.
//
// The launch copies the bound app privately and launches it once, with `report-standing-journey.js`, in the
// confinement `run.mjs` uses (`launch-env.mjs`): an environment built from nothing, a throwaway HOME (and
// CFFIXED_USER_HOME) with TMPDIR inside it, the strict chooser (a chooser with no queued answer inside the home ends
// the app, exit 86, before any dialog), a non-persistent web view store, the run marker and the tripwire over the
// real user's product-named Library paths. The `native-smoke` build uses the smoke Keychain backend (no Security
// import, no item call); the embedded agent is never started. A hard timeout kills the app; the census then waits for
// no process of the private copy. `expected.json` is neither read nor written. Directly before the app is started the
// runner checks the three marks of an agent operating the App on the screen and refuses to launch if any is present.
//
// The driver works the window as a person would (native menu commands through the menu dispatch, controls in the
// window). Two presentations are substituted: the Open Model Document chooser (`native_smoke_choose`, as every
// native-smoke run) and the system print dialog. Print… goes to the native print route (DEL-08-01 REQ-022); in place
// of the dialog this build records, on the print window's own handle, the print document that reached the native
// print, and the driver takes that record and closes the window. The print checks are judged from the recorded
// document and the rendered one (`report-print-evidence.mjs`): no dialog appearance, preview composition or job
// outcome is observed or claimed. Rendering, the derived print view, solve, the gates and the user-release dialog run
// on the product's code; the dialog is cancelled, and no package is saved.
//
// The model is the committed coupled-friction witness request's model with the three display additions the App test
// makes, written into the home. What the report must state is derived from the committed App render request's
// standing record (`renderedReportGate.app.test.tsx` writes it) and the generated statement table
// (`fixtures/reports/standing_statements.json`), and the frame's lines the dialog must state from the committed frame
// statements of the same witness (`report_standing_package_tests.rs` writes them), never from an earlier run.
//
// A launch passes only on exit 0, no signal, no timeout, a resolved census, an observations file in the fresh
// evidence root carrying this run's nonce, and every step reached and observed as expected. The evidence root is kept
// (`summary.json` names every step) unless --discard is given and every check passed. Unit of evidence: the launched
// native-smoke build on this host; not a product release qualification.
import { createHash, randomBytes } from "node:crypto";
import { execFileSync, spawnSync } from "node:child_process";
import { existsSync, lstatSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, realpathSync, rmSync, writeFileSync, writeSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { ENGINE_BUILDS } from "../scripts/build-wasm-engine.mjs";
import { judgeReportPrint, standingLineOf } from "./report-print-evidence.mjs";
import { bindingPathFor, fileIdentity, prepareSmokeApp, privateLaunchCopy } from "../scripts/appBinaryBinding.mjs";
import { checkEngine, engineStampContext } from "../scripts/engineSourceStamp.mjs";
import { BUILD_LOCK_TOKEN_ENV, acquireBuildLock, exitAfterSignals, processStartTime, refuseCompilerWrapperEnv, refuseHostSlotCommand, runUnderLock } from "../scripts/worktreeBuildLock.mjs";
import {
  awaitNoProcesses, awaitNoProcessesSync, buildLaunchEnv, censusOf, chooserLayout, closeRun, createThrowawayHome, readRunMarker,
  refusedLaunchEnv, refusedPlanPaths, refusedSmokeWindows, runMarkerRefusal, smokeWindows, statWatched, tripwireChanges, watchedLibraryPaths, writeRunMarker
} from "./launch-env.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const desktop = path.resolve(here, "..");
const project = path.resolve(desktop, "..", "..");
const tauriDir = path.join(desktop, "src-tauri");
const args = new Set(process.argv.slice(2));
const buildOnly = args.has("--build-only");
const NAME = "report standing journey";
const say = (line) => console.log(`${NAME}: ${line}`);
const refuse = (line) => console.error(`${NAME}: ${line}`);

if (process.platform !== "darwin") {
  refuse("macOS only");
  process.exit(2);
}

// The launch's hard limit: the app (caffeinate's process group) is killed at it, and the launch fails. Steps 1 to 7
// took 35 s in the first launch; every driver wait is capped at 90 s.
const LAUNCH_TIMEOUT_MS = 4 * 60 * 1000;
const TERMINATION_BOUND_MS = 20 * 1000;
const TAURI_CONF = JSON.parse(readFileSync(path.join(tauriDir, "tauri.conf.json"), "utf8"));
const IDENTIFIER = TAURI_CONF.identifier;
const SMOKE_WINDOWS = smokeWindows(TAURI_CONF.app.windows);
const SMOKE_TAURI_CONFIG = JSON.stringify({ app: { windows: SMOKE_WINDOWS } });
// The run marker `run.mjs` uses, so either runner recognises the other's unresolved run.
const RUN_MARKER = path.join(desktop, "target", "native-smoke-run.json");

const DIR = "fixtures/reports/invented/report_standing_witnesses";
const REQUEST_FIXTURE = `${DIR}/coupled_friction.request.json`;
const APP_REQUEST_FIXTURE = `${DIR}/pressure_3_coupled_friction_sensitive.app_render_request.json`;
const STATEMENTS_FIXTURE = "fixtures/reports/standing_statements.json";
const FRAME_FIXTURE = `${DIR}/pressure_3_coupled_friction_sensitive.frame_statements.json`;
const DEMO_FIXTURE = "fixtures/product_preview/invented_demo_model.json";
const MODEL_NAME = "report-standing-coupled-friction.json";

const sha = (bytes) => createHash("sha256").update(bytes).digest("hex");
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
  refuse(error.message);
  process.exit(2);
}
process.env[BUILD_LOCK_TOKEN_ENV] = buildLock.token;
async function run(command, commandArgs, cwd) {
  if (!buildOnly) throw new Error(`REPORT-JOURNEY-LAUNCH-WOULD-BUILD: ${command} ${commandArgs.join(" ")}; build first with --build-only`);
  refuseHostSlotCommand(command, commandArgs);
  console.log(`$ (${path.relative(project, cwd) || "."}) ${command} ${commandArgs.join(" ")}`);
  const result = await runUnderLock(command, commandArgs, { cwd, stdio: ["ignore", "inherit", "inherit"], env: { ...process.env, CARGO_NET_OFFLINE: "true" } });
  if (result.error !== null || result.status !== 0) throw commandFailed(command, commandArgs, result);
}
let stampContext;
try {
  stampContext = engineStampContext(desktop);
} catch (error) {
  refuse(error.message);
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
  refuse(error.message);
  await exitAfterSignals(2);
}
const engines = Object.fromEntries(engineStates().map((engine) => [engine.name, engine.hash]));
const APP_BUILD = {
  command: ["cargo", "build", "--locked", "--offline", "--manifest-path", path.join(tauriDir, "Cargo.toml"), "--message-format=json-render-diagnostics", "--features", "native-smoke,tauri/custom-protocol"],
  tauri_config: SMOKE_TAURI_CONFIG
};
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
    build: APP_BUILD,
    context: engineStampContext(tauriDir),
    runFrontend: () => run("npm", ["run", "build"], desktop),
    runCargo: async ([command, ...commandArgs], env) => {
      if (!buildOnly) throw new Error(`REPORT-JOURNEY-LAUNCH-WOULD-BUILD: ${command} ${commandArgs.join(" ")}; build first with --build-only`);
      refuseHostSlotCommand(command, commandArgs);
      refuseCompilerWrapperEnv({ ...process.env, ...env });
      console.log(`$ (${path.relative(project, tauriDir)}) ${command} ${commandArgs.join(" ")}`);
      const result = await runUnderLock(command, commandArgs, { cwd: tauriDir, stdio: ["ignore", "pipe", "inherit"], env: { ...process.env, CARGO_NET_OFFLINE: "true", ...env } });
      if (result.error !== null || result.status !== 0) throw commandFailed(command, commandArgs, result);
      return result.stdout;
    }
  });
} catch (error) {
  refuse(error.message.startsWith("native smoke:") ? error.message : `app build or binding failed: ${error.message}`);
  await exitAfterSignals(2);
}
const bindingPath = bindingPathFor(desktop);
const bindingSha = fileSha(bindingPath);
say(`app ${app.binary} (sha256 ${app.identity.sha256}, ${app.identity.size} bytes); binding record ${bindingPath} sha256 ${bindingSha}`);
if (buildOnly) {
  delete process.env[BUILD_LOCK_TOKEN_ENV];
  buildLock.release();
  say("built and bound; nothing launched");
  await exitAfterSignals(0);
}

// ---------------------------------------------------------------- the launch

let launchCopy;
try {
  launchCopy = privateLaunchCopy({ desktopRoot: desktop, binary: app.binary, identity: app.identity, token: buildLock.token });
} catch (error) {
  refuse(`could not copy the app for launch: ${error.message}`);
  await exitAfterSignals(2);
}
delete process.env[BUILD_LOCK_TOKEN_ENV];
const binary = launchCopy.executable;
const WATCHED = watchedLibraryPaths(path.join(os.userInfo().homedir, "Library"), { identifier: IDENTIFIER, names: [TAURI_CONF.productName, "openpipestress-desktop"] });
const state = { launched: false, termination: null, watchedBefore: null, tripwireDone: false, passed: false, closed: false };
process.on("exit", closeOnExit);

// The evidence root, fresh; the throwaway home inside it, created empty.
// Resolved (no links), as the throwaway home is, so that containment is judged on the same names.
const root = realpathSync(mkdtempSync(path.join(os.tmpdir(), "swbpipe-report-standing-journey-")));
let home;
let tmpdir;
try {
  ({ home, tmpdir } = createThrowawayHome(root));
} catch (error) {
  refuse(error.message);
  await exitAfterSignals(2);
}
const layout = chooserLayout(home);
for (const folder of layout.folders) mkdirSync(folder);
// The Save canonical HTML… chooser's answer, inside the home like every chooser answer.
const htmlSaveFolder = path.join(layout.area, "saved-report-html");
mkdirSync(htmlSaveFolder);
const htmlSavePath = path.join(htmlSaveFolder, "selected-report.html");

// The model document: the committed request's model with the App test's three display additions.
const request = JSON.parse(readFileSync(path.join(project, REQUEST_FIXTURE), "utf8"));
const demo = JSON.parse(readFileSync(path.join(project, DEMO_FIXTURE), "utf8"));
const model = structuredClone(request.model);
for (const loadCase of model.load_cases) if (loadCase.label === undefined) Object.assign(loadCase, { label: loadCase.id, kind: "invented_test_case", status: "invented_test_status" });
if (typeof model.project.name !== "string") Object.assign(model.project, { name: "Invented report-standing witness pressure_3_coupled_friction_sensitive" });
if (model.diagnostics === undefined) model.diagnostics = [];
if (model.data_boundary === undefined) model.data_boundary = structuredClone(demo.data_boundary);
const modelPath = path.join(layout.models, MODEL_NAME);
writeFileSync(modelPath, `${JSON.stringify(model, null, 2)}\n`);

// What the report must state, derived from the committed App render request's standing record and the generated
// statement table.
const record = JSON.parse(readFileSync(path.join(project, APP_REQUEST_FIXTURE), "utf8")).input.numerical_standing;
const statements = JSON.parse(readFileSync(path.join(project, STATEMENTS_FIXTURE), "utf8")).statements;
const statement = (vocabulary, tag) => statements.find((item) => item.vocabulary === vocabulary && item.tag === tag)?.statement ?? null;
const count = (health) => record.cases.filter((item) => item.health === health).length;
const n = record.cases.length;
const failedText = count("solve_failed") > 0 ? `, ${count("solve_failed")} solve failed` : "";
const expected = {
  // standing_line's form when not every case is checks passed (core/reporting/report_renderer/src/standing.rs).
  standing_lead: `Numerical health, as the solver reports it: ${count("checks_passed")} checks passed, ${count("sensitive")} sensitive, ${count("quality_unresolved")} quality unresolved, ${count("not_assessed")} not assessed${failedText}, of ${n} ${n === 1 ? "load case" : "load cases"}.`,
  case_statements: record.cases.flatMap((item) => [
    statement("case_health", item.health),
    statement("historical_confirmation", item.confirmation.outcome),
    item.integrity_code ? statement("historical_integrity_code", item.integrity_code) : null
  ]),
  // DEL-08-01 REQ-024: the frame's lines the user-release dialog states, the native host's for this witness.
  frame_lines: JSON.parse(readFileSync(path.join(project, FRAME_FIXTURE), "utf8")).frame_statements
};

const nonce = randomBytes(16).toString("hex");
const report = path.join(root, "observations.json");
// The driver reports, for each expected statement the runner derived, whether the rendered document states it.
const plan = { phase: "report-standing", nonce, model: { path: modelPath, project_id: model.project.id }, html_save: { path: htmlSavePath }, edit_node: model.nodes[1]?.id ?? model.nodes[0].id, expect: { case_statements: expected.case_statements } };

const checks = [];
function check(name, ok, detail) {
  checks.push({ name, ok: Boolean(ok), detail });
  console.log(`${ok ? "PASS" : "FAIL"}  ${name}${detail === undefined ? "" : `  [${typeof detail === "string" ? detail.slice(0, 600) : JSON.stringify(detail).slice(0, 600)}]`}`);
}
check("inputs: every expected statement is in the generated table", expected.case_statements.every((item) => typeof item === "string"), expected.case_statements);
check("inputs: the witness record is not checks passed in every case", count("checks_passed") < n, { n, checks_passed: count("checks_passed") });
check("inputs: the witness's frame has a line to state", Array.isArray(expected.frame_lines) && expected.frame_lines.length > 0 && expected.frame_lines.every((line) => typeof line === "string"), expected.frame_lines);

// One launch.
const env = buildLaunchEnv({
  parent: process.env, home, tmpdir,
  runner: { SWBPIPE_NATIVE_SMOKE_DRIVER: path.join(here, "report-standing-journey.js"), SWBPIPE_NATIVE_SMOKE_PLAN: JSON.stringify(plan), SWBPIPE_NATIVE_SMOKE_REPORT: report }
});
const refusedLines = [...refusedLaunchEnv(env), ...refusedPlanPaths(plan, home), ...refusedSmokeWindows(SMOKE_WINDOWS)];
if (refusedLines.length > 0) {
  for (const line of refusedLines) refuse(`${line}; not launched`);
  await exitAfterSignals(2);
}
if (JSON.stringify(fileIdentity(binary)) !== JSON.stringify(app.identity)) {
  refuse(`${binary}, the private copy of the bound app, changed since it was checked; not launched`);
  await exitAfterSignals(2);
}
// The three marks of an agent operating the App on the host's screen, checked here, directly before the app is started
// (Agent 0 and the Inter-Project): the supplier's window service under launchd, the Codex app-server helper on stdio, a
// running App v4 candidate bundle. Any one present, or a check that cannot be made, refuses the launch.
function marksCheck() {
  const run = (command, commandArgs) => spawnSync(command, commandArgs, { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"], timeout: 15000 });
  const launchd = run("/bin/launchctl", ["list"]);
  const sky = run("/usr/bin/pgrep", ["-f", "SkyComputerUseService"]);
  const codex = run("/usr/bin/pgrep", ["-f", "codex app-server --listen stdio://"]);
  const apps = run("/usr/bin/lsappinfo", ["list"]);
  const failed = [["launchctl list", launchd, [0]], ["pgrep SkyComputerUseService", sky, [0, 1]], ["pgrep codex app-server", codex, [0, 1]], ["lsappinfo list", apps, [0]]]
    .filter(([, result, ok]) => result.error || !ok.includes(result.status)).map(([name, result]) => `${name}: ${result.error?.message ?? `exit ${result.status}`}`);
  return {
    checked_at: new Date().toISOString(),
    sky_computer_use_service_launchd: /SkyComputerUseService/i.test(launchd.stdout ?? ""),
    sky_computer_use_service_process: sky.status === 0,
    codex_app_server_stdio: codex.status === 0,
    app_v4_candidate_bundle: /dev\.chirality\.app-v4\.candidate/i.test(apps.stdout ?? ""),
    check_failures: failed
  };
}
const marks = marksCheck();
say(`marks, directly before the launch: ${JSON.stringify(marks)}`);
if (marks.check_failures.length > 0 || marks.sky_computer_use_service_launchd || marks.sky_computer_use_service_process || marks.codex_app_server_stdio || marks.app_v4_candidate_bundle) {
  refuse("REPORT-JOURNEY-MARK-PRESENT: an agent may be operating the App on the screen, or the marks could not be checked; not launched");
  writeFileSync(path.join(root, "summary.json"), JSON.stringify({ marks, launched: false }, null, 2));
  await exitAfterSignals(2);
}
state.watchedBefore = statWatched(WATCHED);
writeRunMarker(RUN_MARKER, { pid: process.pid, started: processStartTime(process.pid), home, executable: binary, evidence_root: root, created: new Date().toISOString() });
state.launched = true;
say(`launching ${binary}, a private copy of ${app.binary} (${app.identity.sha256}); HOME=${home}`);
const started = new Date().toISOString();
const outcome = await runUnderLock("/usr/bin/caffeinate", ["-di", binary], { cwd: root, env, stdio: ["ignore", "inherit", "inherit"], timeoutMs: LAUNCH_TIMEOUT_MS });
const ended = new Date().toISOString();
state.termination = await awaitNoProcesses(binary, { boundMs: TERMINATION_BOUND_MS });
const launch = { started, ended, status: outcome.status ?? null, signal: outcome.signal ?? null, timed_out: Boolean(outcome.timedOut), census: state.termination };
say(`launch ended: ${JSON.stringify(launch)}`);
check("launch: exit 0, no signal, no timeout", launch.status === 0 && launch.signal === null && !launch.timed_out, launch);
check("launch: the census finds no process of the private copy", state.termination.resolved, state.termination);

let seen = null;
try { seen = JSON.parse(readFileSync(report, "utf8")); } catch { seen = null; }
check("observations: written in the fresh root, carrying this run's nonce", seen?.nonce === nonce, seen === null ? "no observations file" : { nonce: seen.nonce });
check("observations: the journey completed", seen?.ok === true, seen?.error);
const step = (id) => seen?.steps?.find((item) => item.id === id) ?? null;
const reached = (id) => step(id)?.reached === true;

const opened = step("open-and-solve");
check("open and solve: the model opened and the solve completed", reached("open-and-solve") && /Opened model document/.test(opened?.opened ?? "") && /state=completed/.test(opened?.solve_summary ?? ""), opened);
const gate = step("report-gate");
check("report: the solved result reaches the panel, Render enabled", reached("report-gate") && gate?.precondition === false && gate?.render_enabled === true, gate);
check("report: both package saves offered (enabled, no reason given)", gate?.package_buttons?.length === 2
  && gate.package_buttons.every((button) => button.present === true && button.disabled === false && button.title === null), gate?.package_buttons);
const packageDialog = step("package-dialog");
check("package: the shareable command opens the user-release dialog, which states the frame's lines, the native host's for this witness, before the statement",
  reached("package-dialog") && packageDialog?.refusal === null && JSON.stringify(packageDialog?.frame_lines) === JSON.stringify(expected.frame_lines) && packageDialog?.frame_before_statement === true,
  packageDialog && { expected: expected.frame_lines, seen: packageDialog.frame_lines, before_statement: packageDialog.frame_before_statement, refusal: packageDialog.refusal });
check("package: the statement is unticked and the save not offered until it is; cancelled, the dialog closes with the package line unchanged and no chooser",
  packageDialog?.statement_present === true && packageDialog?.statement_checked === false && packageDialog?.confirm_disabled === true && packageDialog?.dialog_closed === true && packageDialog?.status_unchanged === true,
  packageDialog && { statement_present: packageDialog.statement_present, statement_checked: packageDialog.statement_checked, confirm_disabled: packageDialog.confirm_disabled, dialog_closed: packageDialog.dialog_closed, status_unchanged: packageDialog.status_unchanged });
const outputs = step("other-outputs");
check("outputs: the result JSON export and the rule check are not offered", reached("other-outputs") && outputs?.result_export_empty === true && outputs?.result_export_link === false && outputs?.rule_check_disabled === true, outputs);
const rendered = step("render");
check("render: the rendered report is shown, export gates passed", reached("render") && rendered?.preview === true && /export gates passed/.test(rendered?.gate ?? ""), rendered && { gate: rendered.gate, hash: rendered.hash_text });
check("render: the standing line is the witness record's", (rendered?.standing_line ?? "").startsWith(expected.standing_lead), { expected: expected.standing_lead, seen: rendered?.standing_line });
check("render: each case's health, confirmation and integrity statements are stated", expected.case_statements.every((item) => rendered?.statements_present?.[item] === true),
  expected.case_statements.filter((item) => rendered?.statements_present?.[item] !== true));
check("render: never \"integrity checked\"", rendered?.integrity_checked === false, rendered?.integrity_checked);
check("render: the canonical hash shown is the rendered document's", rendered?.preview_sha256 && rendered.hash_text?.includes(rendered.preview_sha256), { hash_text: rendered?.hash_text, preview_sha256: rendered?.preview_sha256 });
// The native HTML save: judged from the bytes the product wrote, read here outside the app, and the document the
// render step read, never from the app's account of the file. The expected values are re-derived in this run.
const savedHtml = step("save-html");
const savedBytes = existsSync(htmlSavePath) ? readFileSync(htmlSavePath) : null;
const savedSha = savedBytes === null ? null : sha(savedBytes);
const canonicalBytes = typeof savedHtml?.canonical_html === "string" && savedHtml.canonical_html.length > 0 ? Buffer.from(savedHtml.canonical_html, "utf8") : null;
const htmlName = path.basename(htmlSavePath);
check("save HTML: reached through the native Save canonical HTML… control", reached("save-html") && savedHtml?.control_tag === "button", savedHtml && { control: savedHtml.control, tag: savedHtml.control_tag, error: savedHtml.error ?? null });
check("save HTML: the document saved is the one rendered", typeof savedHtml?.shown_hash === "string" && savedHtml.shown_hash === rendered?.hash_text && canonicalBytes !== null,
  { save_step_hash: savedHtml?.shown_hash ?? null, render_step_hash: rendered?.hash_text ?? null });
check("save HTML: its chooser took the queued answer (none left queued)", savedHtml?.chooser_paths_unused === 0, savedHtml?.chooser_paths_unused ?? null);
check("save HTML: the file the runner reads from the chosen path is the rendered canonical HTML, byte for byte",
  savedBytes !== null && canonicalBytes !== null && Buffer.compare(savedBytes, canonicalBytes) === 0,
  { saved_bytes: savedBytes?.length ?? null, canonical_bytes: canonicalBytes?.length ?? null });
check("save HTML: the SHA-256 the runner recomputes from the saved file is the canonical hash the panel shows",
  savedSha !== null && savedHtml?.shown_hash === `Canonical HTML SHA-256: ${savedSha}`, { recomputed: savedSha, shown: savedHtml?.shown_hash ?? null });
const savedLine = /^Saved (\S+): (\d+) bytes, SHA-256 ([0-9a-f]{64})\./.exec(savedHtml?.status ?? "");
check("save HTML: the panel's outcome names the file, the byte count and the SHA-256 the runner reads",
  savedLine !== null && savedBytes !== null && savedLine[1] === htmlName && Number(savedLine[2]) === savedBytes.length && savedLine[3] === savedSha, savedHtml?.status ?? null);
const savedEntry = savedHtml?.entry ?? null;
check("save HTML: the session's outcome entry says saved, for that file, its bytes and SHA-256, with the workspace unchanged since the report was prepared",
  savedEntry !== null && ["saved", "saved_durability_unconfirmed"].includes(savedEntry.state) && savedEntry.workspace_changed === "false"
    && (savedEntry.text ?? "").startsWith(`Saved the selected report. File: ${htmlName}.`) && savedBytes !== null && (savedEntry.text ?? "").includes(`${savedBytes.length} bytes, SHA-256 ${savedSha}`),
  savedEntry);
say(`OBSERVED  save HTML: durability as the product reports it: ${savedEntry?.state === "saved" ? "confirmed" : savedEntry?.state === "saved_durability_unconfirmed" ? "the system did not confirm it is durable on disk" : "no saved entry"}`);
const printed = step("print");
check("print: the step was reached", reached("print"), printed?.error ?? null);
// The rendered report's own document and hash, as the render step saw them, are those the print step handed over.
check("print: the document printed is the one rendered", printed?.shown_hash === rendered?.hash_text && typeof printed?.canonical_html === "string" && printed.canonical_html.length > 0,
  { print_step_hash: printed?.shown_hash ?? null, render_step_hash: rendered?.hash_text ?? null });
for (const item of judgeReportPrint("print", printed, { canonicalHtml: printed?.canonical_html, shownHash: printed?.shown_hash, expectStanding: expected.standing_lead }))
  check(item.name, item.ok, item.detail);
// The line read from the recorded print document's bytes is the one the render step read from the shown preview.
const printedDocument = printed?.events?.length === 1 ? printed.events[0].print_html : undefined;
const printedLine = typeof printedDocument === "string" ? standingLineOf(printedDocument) : undefined;
check("print: the printed standing line is the one the render step read", typeof printedLine === "string" && printedLine === rendered?.standing_line,
  { printed: printedLine ?? null, rendered: rendered?.standing_line ?? null });
const edited = step("edit-invalidates");
check("edit: an edit applied; the panel, present, shows the precondition and no rendered document", reached("edit-invalidates") && edited?.applied === true && edited?.panel_present === true && edited?.precondition === true && edited?.preview === false, edited && { ...edited, save_entry_after_edit: undefined });
// The save completed before the edit (ordinary ordering, no save in transit): its surviving outcome still says it
// saved, and now says the workspace has changed since its report was prepared.
const afterEdit = edited?.save_entry_after_edit ?? null;
check("save HTML after the edit: the outcome entry still says saved, now stating that the workspace has changed since its report was prepared",
  afterEdit !== null && savedEntry !== null && afterEdit.id === savedEntry.id && afterEdit.state === savedEntry.state && afterEdit.workspace_changed === "true"
    && (afterEdit.text ?? "").startsWith("Saved the selected report. The workspace has changed since it was prepared.") && savedBytes !== null && (afterEdit.text ?? "").includes(`${savedBytes.length} bytes, SHA-256 ${savedSha}`),
  afterEdit);
const resolved = step("resolve-invalidates");
check("re-solve: rendered again with the witness record's standing line", reached("resolve-invalidates") && resolved?.preview_before === true && (resolved?.rerendered_line ?? "").startsWith(expected.standing_lead), resolved && { preview_before: resolved.preview_before, rerendered_line: resolved.rerendered_line });
check("re-solve: the fresh solve published a new result (a new solve proof job)", typeof resolved?.proof_job_before === "string" && typeof resolved?.proof_job_after === "string" && resolved.proof_job_before !== resolved.proof_job_after, resolved && { before: resolved.proof_job_before, after: resolved.proof_job_after });
check("re-solve: with the panel present for the fresh result (no loading line, no precondition, Render enabled), no rendered document is shown",
  reached("resolve-invalidates") && resolved?.panel_present_after === true && resolved?.gate_loading_after === false && resolved?.precondition_after === false && resolved?.render_enabled_after === true && resolved?.preview_after === false, resolved);
for (const item of seen?.steps ?? []) if (item.reached !== true) check(`step ${item.id}: reached`, false, item.error ?? "not reached");

// Everything the run wrote lies in the evidence root, the home among it; the real user's product-named paths are unchanged.
function listing(folder) {
  const out = [];
  const walk = (dir) => {
    for (const entry of readdirSync(dir)) {
      const full = path.join(dir, entry);
      const info = lstatSync(full);
      out.push(`${path.relative(root, full)}${info.isDirectory() ? "/" : ""}`);
      if (info.isDirectory() && !info.isSymbolicLink()) walk(full);
    }
  };
  walk(folder);
  return out;
}
const written = listing(root);
const changes = tripwireChanges(state.watchedBefore, statWatched(WATCHED));
state.tripwireDone = true;
check(`the real user's Library: none of the ${WATCHED.length} product-named paths appeared, disappeared or changed`, changes.length === 0, changes);
check("the model document, the saved HTML and the observations are inside the evidence root (the home inside it)",
  modelPath.startsWith(home) && htmlSavePath.startsWith(home) && home.startsWith(root) && report.startsWith(root), { root, home, model: modelPath, html_save: htmlSavePath });

let revision = null;
let dirty = null;
try {
  revision = execFileSync("git", ["-C", project, "rev-parse", "HEAD"], { encoding: "utf8" }).trim();
  dirty = execFileSync("git", ["-C", project, "status", "--porcelain"], { encoding: "utf8" }).trim().split("\n").filter(Boolean);
} catch { /* recorded as unknown */ }
const failed = checks.filter((item) => !item.ok);
writeFileSync(path.join(root, "summary.json"), JSON.stringify({
  witness: "rendered report of a solved exact-route result that is not numerically eligible, in the launched native-smoke app, with its canonical HTML saved through the native save route; the Open and Save canonical HTML choosers and the system print dialog substituted (the native print records its document; no dialog presented)",
  source_revision: revision,
  uncommitted: dirty,
  app: { binary: app.binary, identity: app.identity, binding_record_sha256: bindingSha },
  engines,
  inputs: { [REQUEST_FIXTURE]: fileSha(path.join(project, REQUEST_FIXTURE)), [APP_REQUEST_FIXTURE]: fileSha(path.join(project, APP_REQUEST_FIXTURE)), [STATEMENTS_FIXTURE]: fileSha(path.join(project, STATEMENTS_FIXTURE)), [FRAME_FIXTURE]: fileSha(path.join(project, FRAME_FIXTURE)), [DEMO_FIXTURE]: fileSha(path.join(project, DEMO_FIXTURE)) },
  model_document: { path: modelPath, sha256: fileSha(modelPath) },
  html_save: { path: htmlSavePath, bytes: savedBytes?.length ?? null, sha256: savedSha },
  expected,
  marks,
  launch,
  observations: { path: report, sha256: fileSha(report) },
  steps: (seen?.steps ?? []).map((item) => ({ id: item.id, reached: item.reached === true, error: item.error ?? null })),
  written,
  checks
}, null, 2));
say(`${checks.length - failed.length}/${checks.length} checks passed; evidence in ${root}`);
state.passed = failed.length === 0;
buildLock.release();
await exitAfterSignals(failed.length === 0 ? 0 : 1);

// Close the run on every exit the runner can catch: the census if a launch was not censused, the tripwire through
// `closeRun`, then retention (an unresolved termination keeps the home, the copy, the marker and the lock) or removal
// of the home and the private copy. The evidence root is kept unless --discard and every check passed.
function closeOnExit(code) {
  if (state.closed) return;
  state.closed = true;
  const line = (text) => { try { writeSync(2, `${NAME}: ${text}\n`); } catch { /* nothing more to do */ } };
  try {
    const termination = state.launched && state.termination === null ? awaitNoProcessesSync(launchCopy.executable, { boundMs: TERMINATION_BOUND_MS }) : (state.launched ? state.termination : null);
    const tripwire = state.watchedBefore === null ? null : { before: state.watchedBefore, after: state.tripwireDone ? null : statWatched(WATCHED), done: state.tripwireDone };
    const closed = closeRun({ exitCode: code ?? process.exitCode ?? 0, termination, lockKept: buildLock.kept(), tripwire });
    for (const text of closed.lines) line(text);
    process.exitCode = closed.exitCode;
    if (closed.retain) {
      writeRunMarker(RUN_MARKER, { pid: process.pid, started: processStartTime(process.pid), home, executable: launchCopy.executable, evidence_root: root, reason: termination?.reason ?? "the worktree build lock was kept", retained: new Date().toISOString() });
      line(`UNRESOLVED: the home, the private copy and ${RUN_MARKER} are retained`);
      return;
    }
    if (home !== undefined) rmSync(home, { recursive: true, force: true });
    launchCopy.remove();
    rmSync(RUN_MARKER, { force: true });
    if (args.has("--discard") && state.passed && closed.exitCode === 0) rmSync(root, { recursive: true, force: true });
    else line(`evidence kept in ${root}`);
  } catch (error) {
    if ((process.exitCode ?? 0) === 0) process.exitCode = 1;
    line(`closing the run failed: ${error.message}; the run marker stays`);
  }
}
