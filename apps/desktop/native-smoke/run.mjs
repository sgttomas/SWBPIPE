#!/usr/bin/env node
// Native smoke test of the desktop walking skeleton (DEL-07-11 VER-003), macOS.
//
//   node apps/desktop/native-smoke/run.mjs [--skip-build] [--keep]
//
// Checks each WebAssembly engine's content stamp (`scripts/engineSourceStamp.mjs`)
// and rebuilds a stale or unstamped engine, even under --skip-build; builds the
// frontend and the app with the `native-smoke` cargo feature (under
// --skip-build, whatever a stale engine, dist/ copy, frontend or app source or
// app binding requires) and binds the executable cargo reported
// (`scripts/appBinaryBinding.mjs`), then runs a private copy of that executable
// twice against a throwaway HOME (so the project store and Downloads are
// isolated from the user's own):
//
//   1. open-solve-save: File > Save Model Document… on the bundled demo, re-open
//      the saved file and save it again (W-6a); three named refusals; then for the milestone model and
//      the demo model, File > Open Model Document…, solve sparse and dense,
//      read standing, rows, N1 displacement and the solve proof, export the
//      result JSON where the product offers it (W-18: and File > Save Report
//      Package… for the demo, re-read with the product's reader), and save as a
//      local project;
//   2. reopen: a fresh process opens each saved project and solves it again,
//      then File > Save Handoff… on the stored demo twice: a new handoff-basis
//      state and package, then the same package replaced (W-6b).
//
// It then checks the observations against `expected.json`, the model hash
// against an RFC 8785 JCS SHA-256 of the files it opened, the saved projects
// in the SQLite store, and the exported file. It prints one line per check and
// exits non-zero on any failure. This is evidence, not the owner's witness.
//
// What the engine stamps and the app binding guard against. In scope, and to be
// closed: accidental staleness or mis-binding in our own environments: ordinary
// edits (including ones that keep a file's modification time), branch switches,
// reused worktrees, the environment our wrappers set and the smoke inherits
// (CARGO_TARGET_DIR, CARGO_BUILD_BUILD_DIR and the like: every cargo build,
// clean and target-reporting metadata the smoke tooling runs has its output and
// build directories set to its own dedicated directory), and shared or linked
// target directories. Out of scope, documented
// limits: local cargo configuration or filesystem tricks deliberately
// constructed to defeat the check (for example a linker program whose
// contents change under the same name, or a file swapped between the check and
// cargo's read). Some refusals (cargo configuration keys in any TOML spelling)
// also stop such constructions where that was cheap; that does not bring them in
// scope. DEL-07-11 Design `model-document-open-and-native-smoke.md` lists the
// remaining limits.
//
// Build lock and lock order. The whole smoke, from the engine and app check,
// build and binding through the end of its last app launch, runs under the
// per-worktree build lock (`scripts/worktreeBuildLock.mjs`,
// `apps/desktop/target/build.lock`), which `build:wasm` also takes (inherited
// here through OPENPIPESTRESS_BUILD_LOCK_TOKEN), so two smokes, or a smoke and
// a direct `npm run build:wasm`, in one worktree never interleave. One build or
// smoke per worktree at a time is the supported use: a second is refused at
// once, not queued. The smoke releases the lock only after its last launch
// ends, on every exit path. The lock fails closed and is never broken
// automatically: the refusal names the holder (pid, purpose, start and acquired
// times), gives an informational liveness hint from `ps`, and prints the
// commands that confirm no build is running in this worktree and then remove
// the lock (`rm -f` of its path), so a crashed run costs a minute. One order:
// the host slot first (`t3_slot.sh`, `t3_cargo.sh` or `t3_exclusive.sh`, taken
// by the wrapper that launches this script), then the worktree lock. While
// holding it, no command that would acquire a host slot runs: such a command,
// or an inherited compiler wrapper (RUSTC_WRAPPER, RUSTC_WORKSPACE_WRAPPER,
// CARGO_BUILD_RUSTC_WRAPPER, CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER) before any
// cargo, is refused by name. Every build and launch runs through
// `runUnderLock`, in its own process group: on SIGINT, SIGTERM or SIGHUP the
// smoke stops and reaps it (SIGKILL after a bounded wait), releases the lock,
// and exits 128 + the signal number, removing its private app copy as it
// exits; a process that cannot be confirmed gone keeps the lock and the copy,
// and the removal steps are printed.
// Under the lock, the bound executable is copied into a private per-run
// directory (`target/native-smoke/run-<token>/`, about 115 MB) and the copy's
// SHA-256 checked against the binding; each launch runs that copy, its identity
// checked again first, and the copy is removed when the run ends, normally or
// by signal. SIGKILL of the smoke, or a power loss, leaves the lock and that
// directory; the refusal the next run prints names both, and recovery removes
// that one directory, once the lock is removed and nothing runs from it, never
// every `run-*`.
import { createHash } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { ENGINE_BUILDS } from "../scripts/build-wasm-engine.mjs";
import { fileIdentity, prepareSmokeApp, privateLaunchCopy } from "../scripts/appBinaryBinding.mjs";
import { checkEngine, engineStampContext } from "../scripts/engineSourceStamp.mjs";
import { BUILD_LOCK_TOKEN_ENV, acquireBuildLock, refuseCompilerWrapperEnv, refuseHostSlotCommand, runUnderLock } from "../scripts/worktreeBuildLock.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const desktop = path.resolve(here, "..");
const project = path.resolve(desktop, "..", "..");
const tauriDir = path.join(desktop, "src-tauri");
const args = new Set(process.argv.slice(2));
const expected = JSON.parse(readFileSync(path.join(here, "expected.json"), "utf8"));
const PHASE_TIMEOUT_MS = 25 * 60 * 1000;
const TAURI_CONF = JSON.parse(readFileSync(path.join(tauriDir, "tauri.conf.json"), "utf8"));
const IDENTIFIER = TAURI_CONF.identifier;
// macOS suspends the timers of a webview that is not on screen (display asleep,
// screen locked, window occluded), which would stall the driver. The smoke build
// alone turns that throttling off, through Tauri's TAURI_CONFIG build-time merge;
// tauri.conf.json and product builds are unchanged.
const SMOKE_TAURI_CONFIG = JSON.stringify({ app: { windows: TAURI_CONF.app.windows.map((window) => ({ ...window, backgroundThrottling: "disabled" })) } });

if (process.platform !== "darwin") {
  console.error("native smoke: macOS only");
  process.exit(2);
}

// A failed command, as execFileSync named one.
const commandFailed = (command, commandArgs, result) =>
  new Error(`Command failed: ${command} ${commandArgs.join(" ")}${result.error ? ` (${result.error.message})` : ""}`);

async function run(command, commandArgs, cwd, env = {}) {
  refuseHostSlotCommand(command, commandArgs);
  console.log(`$ (${path.relative(project, cwd) || "."}) ${command} ${commandArgs.join(" ")}`);
  const result = await runUnderLock(command, commandArgs, { cwd, stdio: ["ignore", "inherit", "inherit"], env: { ...process.env, CARGO_NET_OFFLINE: "true", ...env } });
  if (result.error !== null || result.status !== 0) throw commandFailed(command, commandArgs, result);
}

// A stale engine is never acceptable: each engine's stamp is recomputed from
// its sources, with or without --skip-build, and `build:wasm` (which builds
// each missing, unstamped or stale engine, clean) runs when any is.
const skipBuild = args.has("--skip-build");
let buildLock;
try {
  buildLock = acquireBuildLock({ purpose: "native smoke" });
} catch (error) {
  console.error(`native smoke: ${error.message}`);
  process.exit(2);
}
// `build:wasm` and the other builds below run under this lock.
process.env[BUILD_LOCK_TOKEN_ENV] = buildLock.token;
let stampContext;
try {
  // Refuses an inherited compiler wrapper before any cargo runs.
  stampContext = engineStampContext(desktop);
} catch (error) {
  console.error(`native smoke: ${error.message}`);
  process.exit(2);
}
const engineStates = () =>
  ENGINE_BUILDS.map(({ crateDirectory, destination }) => ({
    name: path.basename(destination),
    ...checkEngine({ projectRoot: project, crateDirectory, outDir: destination, context: stampContext })
  }));
const enginesBefore = engineStates();
const staleEngines = enginesBefore.filter((engine) => engine.action === "rebuild");
console.log(`native smoke: checking engine stamps${skipBuild ? " (--skip-build given; a stale engine is rebuilt regardless)" : ""}`);
if (staleEngines.length > 0) {
  await run("npm", ["run", "build:wasm"], desktop);
  for (const engine of engineStates()) {
    if (engine.action !== "current") {
      console.error(`native smoke: engine ${engine.name} still does not match its sources after build:wasm (${engine.reason})`);
      process.exit(2);
    }
    const before = enginesBefore.find((entry) => entry.name === engine.name);
    if (before.action === "rebuild") console.log(`engine rebuilt: ${engine.name} (${before.reason}; stamp ${engine.hash})`);
    else console.log(`engine current, stamp ${engine.hash} (${engine.name}; build:wasm did not build it)`);
  }
} else {
  for (const engine of enginesBefore) console.log(`engine current, stamp ${engine.hash} (${engine.name})`);
}
// The app embeds dist/ when cargo builds it. What is rebuilt, the binding record
// and the executable launched: scripts/appBinaryBinding.mjs.
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
    skipBuild,
    staleEngines: staleEngines.map((engine) => engine.name),
    engineFolders: ENGINE_BUILDS.map(({ destination }) => path.basename(destination)),
    engineHashes: () => Object.fromEntries(engineStates().map((engine) => [engine.name, engine.hash])),
    build: APP_BUILD,
    context: engineStampContext(tauriDir),
    runFrontend: () => run("npm", ["run", "build"], desktop),
    runCargo: async ([command, ...commandArgs], env) => {
      refuseHostSlotCommand(command, commandArgs);
      refuseCompilerWrapperEnv({ ...process.env, ...env });
      console.log(`$ (${path.relative(project, tauriDir)}) ${command} ${commandArgs.join(" ")}`);
      const result = await runUnderLock(command, commandArgs, {
        cwd: tauriDir,
        stdio: ["ignore", "pipe", "inherit"],
        env: { ...process.env, CARGO_NET_OFFLINE: "true", ...env }
      });
      if (result.error !== null || result.status !== 0) throw commandFailed(command, commandArgs, result);
      return result.stdout;
    }
  });
} catch (error) {
  console.error(error.message.startsWith("native smoke:") ? error.message : `native smoke: app build failed: ${error.message}`);
  process.exit(2);
}
// Checked, built and bound. The bound executable is copied to this run's
// private directory and the copy checked against the binding; the launches run
// the copy, still under the lock, which is released after the last one ends.
let launchCopy;
try {
  launchCopy = privateLaunchCopy({ desktopRoot: desktop, binary: app.binary, identity: app.identity, token: buildLock.token });
} catch (error) {
  console.error(error.message.startsWith("native smoke:") ? error.message : `native smoke: could not copy the app for launch: ${error.message}`);
  process.exit(2);
}
// Removed on every exit, unless the lock was kept because a process may still run from it.
process.on("exit", () => { if (!buildLock.kept()) launchCopy.remove(); });
// No build runs from here; the launches still hold the lock.
delete process.env[BUILD_LOCK_TOKEN_ENV];
const binary = launchCopy.executable;
console.log(`native smoke: launching ${binary}, a private copy of ${app.binary} (${app.identity.sha256})`);

// RFC 8785 JCS for JSON documents (finite numbers; ECMAScript number text).
function canonical(value) {
  if (Array.isArray(value)) return `[${value.map(canonical).join(",")}]`;
  if (value && typeof value === "object") {
    return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonical(value[key])}`).join(",")}}`;
  }
  return JSON.stringify(value);
}
const modelSha256 = (document) => `sha256:${createHash("sha256").update(canonical(document), "utf8").digest("hex")}`;

const root = mkdtempSync(path.join(os.tmpdir(), "swbpipe-native-smoke-"));
const home = path.join(root, "home");
const models = path.join(root, "models");
mkdirSync(home);
mkdirSync(path.join(home, "Downloads"));
mkdirSync(models);
// W-6a. Where the save chooser is answered: outside HOME's Documents, so the
// files are the ones this run wrote.
const savedModels = path.join(root, "saved-model-documents");
mkdirSync(savedModels);
const MODEL_DOCUMENT_SAVE = { first: path.join(savedModels, "saved-demo.json"), second: path.join(savedModels, "saved-demo-reopened.json") };
// W-6b. Where the handoff folder chooser is answered, outside HOME's Documents.
const savedHandoffs = path.join(root, "saved-handoffs");
mkdirSync(savedHandoffs);
const HANDOFF_SAVE = { path: path.join(savedHandoffs, "saved-demo.swbhandoff") };
// W-18 (b2). Where the report-package save chooser is answered, outside HOME's
// Documents and Downloads.
const savedReportPackages = path.join(root, "saved-report-packages");
mkdirSync(savedReportPackages);
const REPORT_PACKAGE_SAVE = { path: path.join(savedReportPackages, "saved-demo.opsproj") };
const documents = expected.cases.map((entry) => {
  const source = path.join(project, entry.fixture);
  const target = path.join(models, path.basename(entry.fixture));
  copyFileSync(source, target);
  const document = JSON.parse(readFileSync(target, "utf8"));
  return { ...entry, path: target, document, project_id: document.project.id, modes: Object.keys(entry.observations) };
});
const demo = JSON.parse(readFileSync(path.join(project, "fixtures/product_preview/invented_demo_model.json"), "utf8"));
const request = JSON.parse(readFileSync(path.join(project, "fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json"), "utf8"));
writeFileSync(path.join(models, "newer-schema.json"), JSON.stringify({ ...demo, schema_version: "9.0.0" }));
writeFileSync(path.join(models, "solve-request-model.json"), JSON.stringify(request.model));
writeFileSync(path.join(models, "not-json.json"), "{ this is not JSON");

// W-4. An exact-route (pressure-2) document for the output gates, and the model
// the driver authors from blank. Neither needs a pinned value: the blank
// cantilever's tip deflection is checked against PL^3/3EI from these inputs.
const exactFixture = "fixtures/model_operations/exact_pressure_authoring_model.json";
const exactPath = path.join(models, path.basename(exactFixture));
copyFileSync(path.join(project, exactFixture), exactPath);
const exactDocument = JSON.parse(readFileSync(exactPath, "utf8"));
const BLANK = {
  anchor: "node:W4-A", tip: "node:W4-B", material: "material:w4-steel", pipe: "pipe:W4-1", support: "support:W4-A",
  load_case: "load:W4", provenance: "invented_synthetic_native_smoke_input",
  elastic_modulus_pa: 200e9, shear_modulus_pa: 77e9, length_m: 3, outside_diameter_m: 0.168, wall_m: 0.007, force_n: 350,
  probe: "result:disp:node-W4-B"
};
// Euler-Bernoulli cantilever tip deflection in mm (the pressure-free route's straight member).
const blankInertia = (Math.PI / 64) * (BLANK.outside_diameter_m ** 4 - (BLANK.outside_diameter_m - 2 * BLANK.wall_m) ** 4);
const blankTipMm = (BLANK.force_n * BLANK.length_m ** 3) / (3 * BLANK.elastic_modulus_pa * blankInertia) * 1000;
const UNIT_KEYS = ["length", "force", "moment", "pressure", "stress", "temperature", "angle"];

function launch(phase, extra = {}) {
  const report = path.join(root, `report-${phase}.json`);
  const plan = {
    phase,
    bundled_project_id: expected.bundled_project_id,
    refusals: expected.refusals.map((item) => ({ path: path.join(models, item.file) })),
    // The first case also exercises the replace prompt (HELP_HUMAN ruling 2026-10-10).
    documents: documents.map(({ case: name, path: file, project_id, modes, probe }, index) => ({ case: name, path: file, project_id, modes, probe: probe ?? null, replace_flows: index === 0 })),
    exact: { path: exactPath, project_id: exactDocument.project.id },
    blank: BLANK,
    model_document: MODEL_DOCUMENT_SAVE,
    ...extra
  };
  console.log(`\n== phase ${phase} (HOME=${home})`);
  if (JSON.stringify(fileIdentity(binary)) !== JSON.stringify(app.identity)) {
    console.error(`native smoke: ${binary}, the private copy of the bound app, changed since it was checked; not launched`);
    process.exit(2);
  }
  // caffeinate keeps the display and system awake for the app's lifetime. It
  // runs under the lock in its own process group: on timeout the whole group,
  // caffeinate and the app under it, is killed; on interruption the lock's
  // handler stops and reaps it before the lock is released.
  return runUnderLock("/usr/bin/caffeinate", ["-di", binary], {
    cwd: root,
    env: {
      ...process.env,
      HOME: home,
      SWBPIPE_NATIVE_SMOKE_DRIVER: path.join(here, "driver.js"),
      SWBPIPE_NATIVE_SMOKE_PLAN: JSON.stringify(plan),
      SWBPIPE_NATIVE_SMOKE_REPORT: report
    },
    stdio: ["ignore", "inherit", "inherit"],
    timeoutMs: PHASE_TIMEOUT_MS
  }).then(({ status, signal, timedOut }) => {
    if (timedOut) console.error(`phase ${phase}: timed out`);
    return existsSync(report) ? JSON.parse(readFileSync(report, "utf8")) : { ok: false, error: `no report (exit ${status ?? signal})` };
  });
}

const checks = [];
function check(name, ok, detail) {
  checks.push({ name, ok: Boolean(ok), detail });
  console.log(`${ok ? "PASS" : "FAIL"}  ${name}${detail === undefined ? "" : `  [${typeof detail === "string" ? detail : JSON.stringify(detail)}]`}`);
}
function observation(report, name, mode) {
  return report.observations?.find((item) => item.case === name && item.mode === mode);
}
function checkObservation(prefix, seen, want, modelHash) {
  if (!seen) return check(`${prefix}: observed`, false, "no observation");
  check(`${prefix}: rows`, seen.proof_rows === want.rows && seen.result_filter_summary?.endsWith(`of ${want.rows} results match filter`), `${seen.proof_rows}; ${seen.result_filter_summary}`);
  if (want.probe_entered) check(`${prefix}: ${seen.probe}`, seen.probe_entered === want.probe_entered, seen.probe_entered);
  if (want.standing_prefix) check(`${prefix}: standing`, seen.standing?.startsWith(want.standing_prefix), seen.standing?.slice(0, 80));
  check(`${prefix}: solve proof model_sha256`, seen.proof_model_sha256 === modelHash, seen.proof_model_sha256);
  check(`${prefix}: solve proof identity`, seen.solve_proof?.includes("identity=match") && seen.proof_project === seen.project_id, seen.proof_project);
  check(`${prefix}: no retained-precision diagnostics`, seen.retained_precision_codes.length === 0 && seen.retained_precision_elements === 0, seen.retained_precision_codes);
}

const first = await launch("open-solve-save", { report_package: REPORT_PACKAGE_SAVE });
check("phase open-solve-save completed", first.ok, first.error);
// W-6a (DEL-15-01 Design canonical-model-export.md §2). Nothing here is pinned:
// the expected bytes and hash are computed from the bundled demo fixture.
{
  const saved = first.model_document?.saved;
  const resaved = first.model_document?.resaved;
  const demoText = canonical(demo);
  const demoHash = modelSha256(demo);
  const fileOf = (file) => (existsSync(file) ? readFileSync(file) : null);
  const firstBytes = fileOf(MODEL_DOCUMENT_SAVE.first);
  const secondBytes = fileOf(MODEL_DOCUMENT_SAVE.second);
  const sha = (bytes) => `sha256:${createHash("sha256").update(bytes).digest("hex")}`;
  const reported = (shown) => shown?.match(/^Saved model document (\S+) \((\d+) bytes; model hash (sha256:[0-9a-f]{64})\)\.$/);
  check("save model document: the dialog asks for the explicit intent before the chooser (unchecked; Save disabled; Cancel focused)",
    saved && !saved.intent_checked_initially && saved.confirm_disabled_before_intent && saved.cancel_focused
      && saved.intent_label === "Save my model document, including private project data, to a file I choose", saved && { ...saved, message: undefined });
  check("save model document: the file is the demo's RFC 8785 text, with no trailing newline",
    firstBytes && firstBytes.toString("utf8") === demoText && firstBytes.at(-1) !== 0x0a, firstBytes?.length);
  check("save model document: SHA-256 of the file is the demo's model hash, as the status line reports",
    firstBytes && sha(firstBytes) === demoHash && reported(saved?.message)?.[3] === demoHash
      && Number(reported(saved?.message)?.[2]) === firstBytes.length && reported(saved?.message)?.[1] === path.basename(MODEL_DOCUMENT_SAVE.first), saved?.message);
  check("save model document: the saved file re-opens through Open Model Document…",
    first.model_document?.reopened?.startsWith(`Opened model document ${path.basename(MODEL_DOCUMENT_SAVE.first)} (${firstBytes?.length} bytes; `), first.model_document?.reopened);
  check("save model document: the re-opened session saves identical bytes under an equal model hash",
    firstBytes && secondBytes && Buffer.compare(firstBytes, secondBytes) === 0 && reported(resaved?.message)?.[3] === demoHash, resaved?.message);
}
for (const want of expected.refusals) {
  const seen = first.refusals?.find((item) => item.file === want.file);
  check(`refusal ${want.file}: ${want.code}`, seen?.message?.includes(`refused (${want.file}): ${want.code}`) && seen.prior_model_kept, seen?.message);
}
for (const doc of documents) {
  const hash = modelSha256(doc.document);
  for (const mode of doc.modes) checkObservation(`${doc.case}/${mode}`, observation(first, doc.case, mode), doc.observations[mode], hash);
}

// HELP_HUMAN ruling (2026-10-10): the app never silently replaces a stored project.
const promptCase = documents[0];
const promptFor = (report, label) => report.prompts?.find((item) => item.label === label);
function checkPrompt(name, prompt, storedName, storedId) {
  check(`${name}: prompt shown, naming the stored project and when it was last saved`,
    prompt?.detail?.includes(`"${storedName}" (${storedId}), last saved `), prompt?.detail);
  check(`${name}: Cancel is the default (focused)`, prompt?.cancel_focused === true);
}
{
  const cancel = promptFor(first, `${promptCase.case}/second-save`);
  checkPrompt("second save of the same document", cancel, promptCase.document.project.name, promptCase.project_id);
  check("second save, Cancel: the stored project is kept", cancel?.message?.startsWith("Not saved: the stored project "), cancel?.message);
  const copy = promptFor(first, `${promptCase.case}/save-as-new`);
  checkPrompt("save as a new project", copy, promptCase.document.project.name, promptCase.project_id);
  check("save as a new project: a new project id", copy?.new_project_id?.startsWith(`${promptCase.project_id}:copy-`) && !copy.original_kept_in_tree, copy?.new_project_id);
  const seen = observation(first, `${promptCase.case}/copy`, promptCase.modes[0]);
  const original = observation(first, promptCase.case, promptCase.modes[0]);
  const own = promptFor(first, `${promptCase.case}/copy/own-save`);
  check("own project, Save Local Project: saved in place without a prompt", own && !own.prompted && own.message?.includes("Saved current project model snapshot"), own?.message);
  const stale = promptFor(first, `${promptCase.case}/copy/stale-base`);
  check("own project saved elsewhere since: the save asks, naming the stored copy",
    stale?.detail?.includes(`has been saved since this session last saw it`) && stale.detail.includes(`(${copy?.new_project_id}), last saved `), stale?.detail);
  check("own project saved elsewhere since: Cancel is the default (focused)", stale?.cancel_focused === true);
  check("own project saved elsewhere since, Cancel: the stored copy is kept", stale?.message?.startsWith("Not saved: the stored project "), stale?.message);
  check("save as a new project: same rows and probed value", seen && original && seen.proof_rows === original.proof_rows && seen.probe_entered === original.probe_entered, `${seen?.proof_rows}; ${seen?.probe_entered}`);
  const replace = promptFor(first, `${promptCase.case}/replace`);
  checkPrompt("save over the stored project", replace, promptCase.document.project.name, promptCase.project_id);
  check("Replace: written only after confirmation", replace?.message?.includes("Created local SQLite project snapshot"), replace?.message);
}

// W-4. The bundled demo pack's two inputs are named missing on every model the smoke checks.
const DEMO_PACK_INPUTS = ["demo_actual_quantity", "demo_limit_quantity"];
const namesMissingInputs = (rules) => Boolean(rules?.aggregate?.includes("RULE_INPUTS_INCOMPLETE")
  && DEMO_PACK_INPUTS.every((input) => rules.outcomes?.includes(`${input}=MISSING`)));
// W-4. The demo's local-private report and rule checks.
{
  const render = first.report_render;
  check("demo local-private report: rendered unblocked in the local-private context", render?.redaction?.includes("context=local_private") && render.redaction.includes("blocked=false"), render?.redaction ?? render?.error);
  check("demo local-private report: canonical HTML SHA-256 shown", /^Canonical HTML SHA-256: [0-9a-f]{64}$/.test(render?.hash ?? ""), render?.hash);
  check("demo local-private report: export gates passed", render?.gate === "export gates passed", render?.gate);
  check("demo local-private report: local-private warning, save and the document's notice", render?.local_private_warning && render.save_offered && render.preview_local_private_notice, render && { warning: render.local_private_warning, save: render.save_offered, notice: render.preview_local_private_notice });
  const rules = first.rule_checks;
  check("demo rule checks: refused as RULE_INPUTS_INCOMPLETE, naming the missing inputs", namesMissingInputs(rules), rules?.outcomes);
}
// W-4. Exact-route (pressure-2) gates, by name.
{
  const exact = first.exact;
  const solved = exact?.solve;
  check("exact route: solved, its proof bound to the opened document", solved?.solve_job_summary?.includes("state=completed") && solved.proof_model_sha256 === modelSha256(exactDocument) && solved.solve_proof?.includes("identity=match"), solved?.proof_model_sha256);
  check("exact route: the standing names the route", solved?.standing?.includes("3.0.0/exact_pressure_v3"), solved?.standing?.slice(0, 80));
  check("exact route: rendered report refused by its own gate (PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE), no render control",
    exact?.rendered_report_panel && !exact.render_control_present && exact.render_refusal?.startsWith("PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE"), exact?.render_refusal);
  check("exact route: MBF refused by its own gate (PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE), no package or MBF text download",
    exact?.mbf_panel && !exact.mbf_download_controls && exact.mbf_refusal?.startsWith("PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE"), exact?.mbf_refusal);
  check("exact route: result JSON export offered", exact?.result_export_link === true);
  check("exact route: report package refused by its route (PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE), no package controls",
    exact?.report_package_refusal?.startsWith("PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE") && exact.report_package_controls === false, exact?.report_package_refusal);
  check("exact route: rule checks refused as RULE_INPUTS_INCOMPLETE, naming the missing inputs", namesMissingInputs(exact?.rule_checks), exact?.rule_checks?.outcomes);
}
// W-4. The model authored from blank in the GUI.
const blankId = first.blank?.project_id ?? null;
const enteredMm = (shown) => {
  const match = shown?.match(/^Entered: (\S+) mm$/);
  return match ? Number(match[1]) : NaN;
};
const tipMm = (observed) => enteredMm(observed?.probe_entered);
const withinCriterion = (observedMm) => Math.abs(observedMm - blankTipMm) <= 1e-9 * Math.abs(blankTipMm); // DEC-024/026 criterion
// The +Y tip force deflects the tip by +PL^3/3EI along Y and nowhere else.
const signedTip = (observed) => {
  const [ux, uy, uz] = ["ux", "uy", "uz"].map((axis) => enteredMm(observed?.components?.[axis]));
  return withinCriterion(uy) && Math.abs(ux) <= 1e-9 * blankTipMm && Math.abs(uz) <= 1e-9 * blankTipMm;
};
{
  const blank = first.blank;
  check("from blank: New Blank Project opens the units dialog (SI, US customary, custom)", ["si", "us_customary", "custom"].every((preset) => blank?.unit_presets?.includes(preset)), blank?.unit_presets);
  check("from blank: node, material, pipe, support, load case and force authored through the GUI", blank?.steps?.length === 8, blank?.steps);
  const solved = blank?.solve;
  check("from blank: native solve completed, its proof bound to the session's project", solved?.solve_job_summary?.includes("state=completed") && solved.proof_project === blankId && solved.solve_proof?.includes("identity=match"), solved?.solve_job_summary);
  check("from blank: numerical integrity checks passed", solved?.standing?.startsWith("Numerical integrity: checks_passed."), solved?.standing?.slice(0, 60));
  check("from blank: the unfiltered results count every solved row", solved && solved.proof_rows > 0 && solved.all_rows_summary === `${solved.proof_rows} of ${solved.proof_rows} results match filter`, `${solved?.proof_rows}; ${solved?.all_rows_summary}`);
  check("from blank: deformation overlay available for both nodes", solved?.deformation?.startsWith("available; nodes=2;"), solved?.deformation);
  check(`from blank: tip deflection magnitude equals PL^3/3EI = ${blankTipMm} mm within 1e-9 relative`, withinCriterion(tipMm(solved)), solved?.probe_entered);
  check("from blank: tip uy = +PL^3/3EI, ux and uz zero", signedTip(solved), solved?.components);
  check("from blank: Save Local Project saves in place without a prompt", blank?.save && !blank.save.prompted && blank.save.message?.includes("Saved current project model snapshot"), blank?.save?.message);
}

// The store under the throwaway HOME holds both projects.
const storePath = path.join(home, "Library", "Application Support", IDENTIFIER, "openpipestress-projects.sqlite3");
const stored = {};
const storedRuns = {};
if (existsSync(storePath)) {
  const { DatabaseSync } = await import("node:sqlite");
  const db = new DatabaseSync(storePath, { readOnly: true });
  for (const row of db.prepare("SELECT project_id, model_json, mechanics_result_json FROM local_projects").all()) {
    stored[row.project_id] = JSON.parse(row.model_json);
    storedRuns[row.project_id] = JSON.parse(row.mechanics_result_json);
  }
  db.close();
}
check("project store isolated under the throwaway HOME", existsSync(storePath), storePath);
for (const doc of documents) {
  const envelope = stored[doc.project_id];
  const model = envelope?.model ?? envelope;
  const { schema_version: savedVersion, ...savedRest } = model ?? {};
  const { schema_version: fileVersion, ...fileRest } = doc.document;
  check(`${doc.case}: saved project's model is the opened document (schema_version ${fileVersion} -> ${savedVersion})`,
    model && canonical(savedRest) === canonical(fileRest), doc.project_id);
  doc.saved_model = model;
}
{
  const copyId = promptFor(first, `${promptCase.case}/save-as-new`)?.new_project_id;
  const copyModel = stored[copyId];
  const { schema_version: copyVersion, project: copyProject, ...copyRest } = copyModel ?? {};
  const { schema_version: fileVersion, project: fileProject, ...fileRest } = promptCase.document;
  void copyVersion; void fileVersion;
  check("save as a new project: stored under the new id, the document otherwise unchanged",
    copyModel && canonical(copyRest) === canonical(fileRest) && canonical({ ...copyProject, id: fileProject.id }) === canonical(fileProject), copyId);
  // The session had adopted the first save's normalized document, so the copy is
  // that document renamed: it differs from the stored original in project.id
  // alone, and the proof of its solve carries its own hash.
  const originalModel = stored[promptCase.project_id];
  const copySeen = observation(first, `${promptCase.case}/copy`, promptCase.modes[0]);
  check("save as a new project: the stored copy differs from the stored original in project.id alone",
    copyModel && originalModel && canonical({ ...copyModel, project: { ...copyModel.project, id: originalModel.project.id } }) === canonical(originalModel), copyId);
  check("save as a new project: the solve proof's model hash is the copy's, not the original's",
    copyModel && copySeen?.proof_model_sha256 === modelSha256(copyModel) && modelSha256(copyModel) !== modelSha256(originalModel), copySeen?.proof_model_sha256);
  check("own project saved elsewhere since, Cancel: the other writer's revision (no run) is what the store holds", copyModel && storedRuns[copyId] === null);
  check("store holds exactly the expected projects", canonical(Object.keys(stored).sort()) === canonical([...documents.map((doc) => doc.project_id), copyId, ...(blankId ? [blankId] : [])].sort()), Object.keys(stored));
}
// W-4. The stored blank project is the model the GUI authored and solved.
const blankStored = blankId ? (stored[blankId]?.model ?? stored[blankId]) : null;
{
  const ids = (key) => (blankStored?.[key] ?? []).map((item) => item.id);
  check("from blank: stored with all seven unit keys", blankStored && UNIT_KEYS.every((key) => typeof blankStored.project?.units?.[key] === "string"), blankStored?.project?.units);
  check("from blank: stored model holds the authored entities",
    blankStored && [BLANK.anchor, BLANK.tip].every((id) => ids("nodes").includes(id)) && ids("pipe_segments").includes(BLANK.pipe)
      && ids("supports").includes(BLANK.support) && ids("materials").includes(BLANK.material) && ids("load_cases").includes(BLANK.load_case), blankId);
  const force = (blankStored?.load_cases ?? []).find((item) => item.id === BLANK.load_case)?.primitive_loads?.find((item) => item.id === `${BLANK.load_case}-FY`);
  check("from blank: stored tip force is 350 N along global_y at the tip node",
    force?.category === "concentrated_force" && force.target?.node === BLANK.tip && force.direction === "global_y"
      && force.magnitude?.value === BLANK.force_n && force.magnitude?.unit === "N", force && { target: force.target, direction: force.direction, magnitude: force.magnitude });
  const fixed = (blankStored?.supports ?? []).find((item) => item.id === BLANK.support);
  check("from blank: stored support fixes all six DOF at the anchor", fixed?.node === BLANK.anchor
    && ["UX", "UY", "UZ", "RX", "RY", "RZ"].every((dof) => fixed.restraints?.includes(dof)), fixed && { node: fixed.node, restraints: fixed.restraints });
  check("from blank: the stored model is the model that was solved", blankStored && first.blank?.solve?.proof_model_sha256 === modelSha256(blankStored), first.blank?.solve?.proof_model_sha256);
}

const downloads = path.join(home, "Downloads");
const exported = existsSync(downloads) ? readdirSync(downloads).filter((name) => name.endsWith(".json")) : [];
for (const item of first.exports ?? []) {
  if (!item.available) { console.log(`INFO  ${item.case ?? item.label}: result export not offered: ${item.reason}`); continue; }
  const name = item.status.match(/^Saved (.+) \(\d+ bytes\)\.$/)?.[1];
  check(`${item.label}: exported result JSON in Downloads`, name && exported.includes(name), item.status);
  if (!name || !exported.includes(name)) continue;
  // The file is the solved run's: its model ref is the case's project, and its
  // row count is the one the export panel showed for the current result.
  const packet = JSON.parse(readFileSync(path.join(downloads, name), "utf8"));
  const doc = documents.find((entry) => entry.case === item.label);
  const envelope = packet.result_envelope ?? {};
  const rows = envelope.result_sets?.[0]?.values?.length;
  // Every solved row is accounted for as an exported value, review evidence or
  // a disclosure; the export follows the case's last solve.
  const accounted = (envelope.result_sets ?? []).reduce((n, set) => n + set.values.length, 0)
    + (envelope.review_evidence?.length ?? 0) + (envelope.row_disclosures?.length ?? 0);
  const solvedRows = observation(first, item.label, doc?.modes.at(-1))?.proof_rows;
  check(`${item.label}: exported file is bound to the solved model`, envelope.model_ref?.ref_id === doc?.project_id, envelope.model_ref?.ref_id);
  check(`${item.label}: exported rows match the export panel`, rows !== undefined && item.summary?.includes(`rows=${rows};`), `${rows}; ${item.summary}`);
  check(`${item.label}: export accounts for every solved row`, accounted === solvedRows, `${accounted} accounted; ${solvedRows} solved`);
}
// Each case's export is offered and written, or not offered as `expected.json`
// says. Under K-D5 (DEL-04-01 kd5-member-relative-bound.md §5) the nonlinear-
// support demo is Sensitive like the milestone; a Sensitive result is not
// Current, so no case offers a result export and nothing may be written. The
// demo's refusal is named by its standing (checked above) and by the saved run.
for (const doc of documents) {
  const item = first.exports?.find((entry) => entry.label === doc.case);
  const want = doc.result_export ?? { offered: true };
  if (want.offered) check(`${doc.case}: result export offered`, item?.available, item?.reason);
  else check(`${doc.case}: result export not offered (${want.panel_text_prefix})`, item && !item.available && item.reason?.startsWith(want.panel_text_prefix), item?.reason);
  const integrity = doc.stored_run_integrity;
  if (!integrity) continue;
  const run = storedRuns[doc.project_id];
  const named = (run?.diagnostics ?? []).filter((item) => item.code === integrity.code);
  check(`${doc.case}: saved run is ${integrity.status}, its rows published (K-D5 §5)`,
    run?.numerical_quality?.status === integrity.status && run.results?.length === observation(first, doc.case, doc.modes.at(-1))?.proof_rows, `${run?.numerical_quality?.status}; ${run?.results?.length} rows`);
  check(`${doc.case}: saved run names the refusal: ${integrity.count} ${integrity.severity} ${integrity.code} with the route's sentence, none blocking`,
    named.length === integrity.count && named.every((item) => item.severity === integrity.severity && item.message?.includes(integrity.sentence))
      && !(run?.diagnostics ?? []).some((item) => item.severity === "blocking" || item.severity === "error"), named.map((item) => item.severity));
}
// W-18 (b2; DEL-08-01 Design report-package-current-results.md §4). File > Save
// Report Package… on the demo's Current pressure-free result. Nothing here is
// pinned: the receipt, the file and the product reader's re-read are compared with
// each other and with the result JSON the same session exported.
{
  const pkg = first.report_package;
  const read = pkg?.read;
  const fileBytes = existsSync(REPORT_PACKAGE_SAVE.path) ? readFileSync(REPORT_PACKAGE_SAVE.path) : null;
  const hex = (bytes) => createHash("sha256").update(bytes).digest("hex");
  const shown = (pattern) => pkg?.status?.match(pattern)?.[1] ?? null;
  check("save report package: offered for the demo's Current pressure-free result, after its own local-private selection (unchecked first)",
    pkg && pkg.unavailable === null && pkg.intent_checked_initially === false && pkg.intent_label === "Include known private values in this local package", pkg && { ...pkg, read: undefined });
  check("save report package: saved (REPORT-PACKAGE-SAVED) to the chosen file", pkg?.status?.startsWith("outcome=saved; code=REPORT-PACKAGE-SAVED; file=") && fileBytes !== null, pkg?.status);
  check("save report package: the file's SHA-256 and length are the receipt's",
    fileBytes && shown(/container_sha256=([0-9a-f]{64})/) === hex(fileBytes) && Number(shown(/bytes=(\d+);/)) === fileBytes.length && read?.file_sha256_hex === hex(fileBytes), shown(/container_sha256=([0-9a-f]{64})/));
  check("save report package: the product reader re-reads an unblocked version 2.0.0 desktop package, its identity the receipt's",
    read && !read.error && read.schema_version === "2.0.0" && read.export_profile_id === "desktop_local_private_report_package_1" && read.export_blocked === false
      && read.package_identity_sha256_hex === shown(/package_identity=([0-9a-f]{64})/), read?.error ?? (read && { schema_version: read.schema_version, export_blocked: read.export_blocked }));
  const inventory = Array.isArray(read?.inventory) ? read.inventory : [];
  const members = Array.isArray(read?.members) ? read.members.filter((member) => member.role !== "package_manifest") : [];
  check("save report package: every member's recomputed SHA-256 and length match the manifest inventory",
    inventory.length > 0 && inventory.length === members.length && inventory.every((entry, index) => members[index]?.role === entry.role && members[index].file_name === entry.file_name
      && members[index].sha256_hex === entry.sha256_hex && members[index].byte_length === entry.byte_length), members.map((member) => member.role));
  const demoExport = (first.exports ?? []).find((item) => item.label === "demo");
  const exportName = demoExport?.status?.match(/^Saved (.+) \(\d+ bytes\)\.$/)?.[1];
  let exportedDocument = null;
  try { exportedDocument = exportName ? JSON.parse(readFileSync(path.join(downloads, exportName), "utf8")) : null; } catch { exportedDocument = null; }
  check("save report package: the result member is the RFC 8785 text of the Export Result JSON document the same session saved",
    exportedDocument && typeof read?.result_member_text === "string" && read.result_member_text === canonical(exportedDocument), exportName);
}
if (documents.some((doc) => (doc.result_export ?? { offered: true }).offered)) check("at least one result export written", exported.length > 0, exported);
else check("no result export written: no case offers one", exported.length === 0, exported);

const second = await launch("reopen", { handoff: HANDOFF_SAVE, ...(blankId ? { blank: { ...BLANK, project_id: blankId } } : {}) });
check("phase reopen completed", second.ok, second.error);
{
  const bundled = promptFor(second, "bundled-demo/save");
  const demoDoc = documents.find((doc) => doc.project_id === expected.bundled_project_id);
  checkPrompt("bundled demo, Save Local Project", bundled, demoDoc?.document.project.name, expected.bundled_project_id);
  check("bundled demo, Cancel: the stored demo is kept", bundled?.message?.startsWith("Not saved: the stored project "), bundled?.message);
}
for (const doc of documents) {
  const hash = doc.saved_model ? modelSha256(doc.saved_model) : null;
  for (const mode of doc.modes) {
    const seen = observation(second, doc.case, mode);
    checkObservation(`${doc.case}/${mode}/reopened`, seen, doc.observations[mode], hash);
    const before = observation(first, doc.case, mode);
    check(`${doc.case}/${mode}/reopened: same rows and probed value as before save`, seen && before && seen.proof_rows === before.proof_rows && seen.probe_entered === before.probe_entered);
  }
}

// W-4. The blank project, reopened in a fresh process, solves to the same rows and value.
{
  const before = first.blank?.solve;
  const after = second.blank?.solve;
  check("from blank, reopened: solved, its proof bound to the stored model", after?.solve_job_summary?.includes("state=completed") && blankStored && after.proof_model_sha256 === modelSha256(blankStored) && after.solve_proof?.includes("identity=match"), after?.proof_model_sha256 ?? after?.solve_job_summary);
  check("from blank, reopened: same rows, tip value and model hash as before save", after && before && after.proof_rows === before.proof_rows && after.probe_entered === before.probe_entered && after.proof_model_sha256 === before.proof_model_sha256, `${after?.proof_rows}; ${after?.probe_entered}`);
  check("from blank, reopened: tip deflection still equals PL^3/3EI, along +Y only", withinCriterion(tipMm(after)) && signedTip(after), after?.components);
  check("from blank, reopened: the unfiltered results count every solved row", after && after.all_rows_summary === `${after.proof_rows} of ${after.proof_rows} results match filter`, after?.all_rows_summary);
}

// W-6b (DEL-15-01 Design canonical-model-export.md §3). File > Save Handoff… on
// the stored demo, twice. Nothing here is pinned: the bytes and hashes are
// computed from the stored demo model and from the saved package itself.
{
  const handoff = second.handoff;
  const firstSave = handoff?.first;
  const replaced = handoff?.second;
  const demoDoc = documents.find((doc) => doc.project_id === expected.bundled_project_id);
  const demoModel = demoDoc?.saved_model ?? null;
  const reported = (shown) => shown?.match(/^Saved handoff package (\S+) \(model hash (sha256:[0-9a-f]{64}); export hash (sha256:[0-9a-f]{64}); basis state (\S+)\)\./);
  const sha = (bytes) => `sha256:${createHash("sha256").update(bytes).digest("hex")}`;
  const packageDir = HANDOFF_SAVE.path;
  const listing = (dir) => (existsSync(dir) ? readdirSync(dir).sort() : null);
  check("save handoff: the dialog asks for the explicit intent and confirms the new basis state before the chooser",
    firstSave && !firstSave.intent_checked_initially && firstSave.confirm_disabled_before_intent && firstSave.cancel_focused
      && firstSave.new_state && firstSave.confirm_disabled_before_state_confirmed && /^Handoff \d{4}-\d{2}-\d{2}$/.test(firstSave.state_name ?? "")
      && firstSave.intent_label === "Save my handoff package, including private project data, to a folder I choose", firstSave && { ...firstSave, message: undefined, verified: undefined });
  const shown = reported(firstSave?.message);
  check("save handoff: saved with a new handoff-basis model state", shown && firstSave.message.includes(`Saved model state "${firstSave.state_name}" (${shown[4]}) as the handoff basis`), firstSave?.message);
  check("save handoff: the package folder holds exactly handoff_package.json and model/model.json",
    canonical(listing(packageDir)) === canonical(["handoff_package.json", "model"]) && canonical(listing(path.join(packageDir, "model"))) === canonical(["model.json"]), listing(packageDir));
  check("save handoff: the folder beside it holds only the package (no staging folder, the replaced package removed)",
    canonical(listing(savedHandoffs)) === canonical([path.basename(packageDir)]), listing(savedHandoffs));
  const modelBytes = existsSync(path.join(packageDir, "model", "model.json")) ? readFileSync(path.join(packageDir, "model", "model.json")) : null;
  const manifestBytes = existsSync(path.join(packageDir, "handoff_package.json")) ? readFileSync(path.join(packageDir, "handoff_package.json")) : null;
  const finalShown = reported(replaced?.message);
  check("save handoff: model/model.json is the stored demo's RFC 8785 text, its SHA-256 the model hash the status line and the verifier report",
    modelBytes && demoModel && modelBytes.toString("utf8") === canonical(demoModel) && sha(modelBytes) === modelSha256(demoModel)
      && finalShown?.[2] === sha(modelBytes) && replaced?.verified?.model_hash === sha(modelBytes), finalShown?.[2] ?? replaced?.message);
  let manifest = null;
  try { manifest = manifestBytes ? JSON.parse(manifestBytes.toString("utf8")) : null; } catch { manifest = null; }
  const inner = manifest?.handoff_package_manifest;
  const { checksums: _checksums, ...unsealed } = inner ?? {};
  void _checksums;
  const exportHash = inner ? `sha256:${createHash("sha256").update(canonical(unsealed), "utf8").digest("hex")}` : null;
  check("save handoff: handoff_package.json is its own RFC 8785 text, and its recomputed export hash is the verifier's and the status line's",
    manifest && manifestBytes.toString("utf8") === canonical(manifest) && exportHash === replaced?.verified?.export_hash && exportHash === finalShown?.[3], exportHash);
  check("save handoff: the manifest binds model/model.json by its SHA-256 and claims nothing embedded",
    inner && inner.checksums?.[0]?.value === sha(modelBytes ?? Buffer.alloc(0)).slice(7) && inner.model_hash?.value === inner.checksums[0].value
      && inner.privacy?.private_payload_embedded === false && inner.professional_boundary?.human_review_required === true, inner?.checksums);
  check("save handoff: the manifest's basis is the new handoff_basis state the status line names",
    inner && shown && inner.model_state_refs?.[0]?.model_state_ref?.ref === shown[4] && inner.model_state_refs[0].state_kind === "handoff_basis"
      && inner.model_basis?.basis_state_ref?.ref === shown[4], inner?.model_state_refs?.[0]?.model_state_ref);
  check("save handoff: the product's verifier verified both saves", firstSave?.verified?.model_hash && !firstSave.verified.error && replaced?.verified?.export_hash && !replaced.verified.error,
    { first: firstSave?.verified, second: replaced?.verified });
  check("save handoff again: the dialog names the saved basis state, the same package is replaced, the state reused",
    replaced && !replaced.new_state && replaced.basis?.includes(`(${shown?.[4]})`) && finalShown?.[4] === shown?.[4]
      && finalShown?.[1] === path.basename(packageDir) && replaced.verified?.export_hash !== firstSave?.verified?.export_hash, replaced && { basis: replaced.basis, message: replaced.message });
  let storedState = null;
  if (existsSync(storePath) && shown) {
    const { DatabaseSync } = await import("node:sqlite");
    const db = new DatabaseSync(storePath, { readOnly: true });
    storedState = db.prepare("SELECT state_kind, model_hash FROM model_states WHERE project_id = ? AND state_id = ?").get(expected.bundled_project_id, shown[4]) ?? null;
    db.close();
  }
  check("save handoff: the basis state is in the local store as handoff_basis with the package's model hash",
    storedState?.state_kind === "handoff_basis" && storedState.model_hash === sha(modelBytes ?? Buffer.alloc(0)), storedState);
}

const failed = checks.filter((item) => !item.ok);
writeFileSync(path.join(root, "summary.json"), JSON.stringify({ checks, first, second }, null, 2));
console.log(`\nnative smoke: ${checks.length - failed.length}/${checks.length} checks passed; evidence in ${root}`);
if (!args.has("--keep") && failed.length === 0) rmSync(root, { recursive: true, force: true });
// The last launch has ended: release the lock (kept if a process of it may
// still run); the private copy is removed as the process exits.
buildLock.release();
process.exit(failed.length === 0 ? 0 : 1);
