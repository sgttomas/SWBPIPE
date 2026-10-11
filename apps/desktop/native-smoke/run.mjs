#!/usr/bin/env node
// Native smoke test of the desktop walking skeleton (DEL-07-11 VER-003), macOS.
//
//   node apps/desktop/native-smoke/run.mjs [--skip-build] [--keep]
//
// Builds the frontend and the app with the `native-smoke` cargo feature, then
// runs the real app twice against a throwaway HOME (so the project store and
// Downloads are isolated from the user's own):
//
//   1. open-solve-save: three named refusals; then for the milestone model and
//      the demo model, File > Open Model Document…, solve sparse and dense,
//      read standing, rows, N1 displacement and the solve proof, export the
//      result JSON where the product offers it, and save as a local project;
//   2. reopen: a fresh process opens each saved project and solves it again.
//
// It then checks the observations against `expected.json`, the model hash
// against an RFC 8785 JCS SHA-256 of the files it opened, the saved projects
// in the SQLite store, and the exported file. It prints one line per check and
// exits non-zero on any failure. This is evidence, not the owner's witness.
import { execFileSync, spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const desktop = path.resolve(here, "..");
const project = path.resolve(desktop, "..", "..");
const tauriDir = path.join(desktop, "src-tauri");
const binary = path.join(tauriDir, "target", "debug", "openpipestress-desktop");
const args = new Set(process.argv.slice(2));
const expected = JSON.parse(readFileSync(path.join(here, "expected.json"), "utf8"));
const PHASE_TIMEOUT_MS = 15 * 60 * 1000;
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

function run(command, commandArgs, cwd, env = {}) {
  console.log(`$ (${path.relative(project, cwd) || "."}) ${command} ${commandArgs.join(" ")}`);
  execFileSync(command, commandArgs, { cwd, stdio: "inherit", env: { ...process.env, CARGO_NET_OFFLINE: "true", ...env } });
}

if (!args.has("--skip-build")) {
  if (!existsSync(path.join(desktop, "public", "wasm-engine"))) run("npm", ["run", "build:wasm"], desktop);
  run("npm", ["run", "build"], desktop);
  run("cargo", ["build", "--locked", "--offline", "--features", "native-smoke,tauri/custom-protocol"], tauriDir, { TAURI_CONFIG: SMOKE_TAURI_CONFIG });
}
if (!existsSync(binary)) {
  console.error(`native smoke: ${binary} is missing; run without --skip-build`);
  process.exit(2);
}

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

function launch(phase) {
  const report = path.join(root, `report-${phase}.json`);
  const plan = {
    phase,
    bundled_project_id: expected.bundled_project_id,
    refusals: expected.refusals.map((item) => ({ path: path.join(models, item.file) })),
    // The first case also exercises the replace prompt (HELP_HUMAN ruling 2026-10-10).
    documents: documents.map(({ case: name, path: file, project_id, modes, probe }, index) => ({ case: name, path: file, project_id, modes, probe: probe ?? null, replace_flows: index === 0 }))
  };
  console.log(`\n== phase ${phase} (HOME=${home})`);
  return new Promise((resolve) => {
    // caffeinate keeps the display and system awake for the app's lifetime.
    const child = spawn("/usr/bin/caffeinate", ["-di", binary], {
      cwd: root,
      env: {
        ...process.env,
        HOME: home,
        SWBPIPE_NATIVE_SMOKE_DRIVER: path.join(here, "driver.js"),
        SWBPIPE_NATIVE_SMOKE_PLAN: JSON.stringify(plan),
        SWBPIPE_NATIVE_SMOKE_REPORT: report
      },
      stdio: ["ignore", "inherit", "inherit"],
      detached: true
    });
    // On timeout or interruption, kill the whole group: caffeinate and the app under it.
    const killGroup = () => { try { process.kill(-child.pid, "SIGKILL"); } catch { /* already gone */ } };
    const interrupted = () => { killGroup(); process.exit(130); };
    process.once("SIGINT", interrupted);
    process.once("SIGTERM", interrupted);
    const timer = setTimeout(() => { console.error(`phase ${phase}: timed out`); killGroup(); }, PHASE_TIMEOUT_MS);
    child.on("exit", (code, signal) => {
      clearTimeout(timer);
      process.off("SIGINT", interrupted);
      process.off("SIGTERM", interrupted);
      const parsed = existsSync(report) ? JSON.parse(readFileSync(report, "utf8")) : { ok: false, error: `no report (exit ${code ?? signal})` };
      resolve(parsed);
    });
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

const first = await launch("open-solve-save");
check("phase open-solve-save completed", first.ok, first.error);
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
  check("store holds exactly the expected projects", canonical(Object.keys(stored).sort()) === canonical([...documents.map((doc) => doc.project_id), copyId].sort()), Object.keys(stored));
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
check("at least one result export written", exported.length > 0, exported);

const second = await launch("reopen");
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

const failed = checks.filter((item) => !item.ok);
writeFileSync(path.join(root, "summary.json"), JSON.stringify({ checks, first, second }, null, 2));
console.log(`\nnative smoke: ${checks.length - failed.length}/${checks.length} checks passed; evidence in ${root}`);
if (!args.has("--keep") && failed.length === 0) rmSync(root, { recursive: true, force: true });
process.exit(failed.length === 0 ? 0 : 1);
