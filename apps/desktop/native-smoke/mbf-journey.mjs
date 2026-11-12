#!/usr/bin/env node
// Launched-app witness of the supported-subset MBF round trip (RT-R7/RT-R8; DEL-17-04 Design
// `desktop-mbf-export-import.md` §5.1), macOS.
//
//   node apps/desktop/native-smoke/mbf-journey.mjs [--skip-build] [--discard]
//   node apps/desktop/native-smoke/mbf-journey.mjs --judge <evidence directory>
//
// Builds and binds the `native-smoke` app exactly as `run.mjs` does (engine stamps, frontend, cargo with the
// `native-smoke` feature, the binding record, the per-worktree build lock and a private launch copy), then
// launches that copy twice against a throwaway HOME with `mbf-journey.js`:
//
//   1. bound:   Open the invented `export_model.json`, Export MBF with the dialog's defaults and save the pair;
//               Import MBF of that pair, Designate original source… with the opened `export_model.json` and Adopt
//               the designated review; Save Model Document of the adopted session; then the Open guard (edit
//               node:A, import again, Adopt, cancel the unsaved-changes prompt).
//   2. unbound: the MBF copied alone to another folder (the identity file left behind) is imported, its
//               material declared as project-invented, and adopted; Save Model Document.
//
// The only substitution for a person is the file choosers (`native_smoke_choose`). Everything else, from the
// export review and atomic save to the native import review, Adopt and the Open validator, runs on the product's
// code. The runner judges against the maintained fixtures: the saved pair's SHA-256 values and the declaration in
// `fixtures/caepipe_mbf/invented/desktop_import/import_replies.json`, its designated-review statement and its
// designated-Adopt and unbound adopted model hashes (RFC 8785 JCS SHA-256 of the saved model documents), and the
// reviews' identity rows. It writes `summary.json`
// with the app's identity, engine stamps, source revision and every file digest, and keeps that evidence unless
// `--discard` is given and every check passed. `--judge` re-judges a kept evidence directory with no build or launch
// (its `summary.json` keeps the run's app identity, engine stamps and revision; the result is `summary-judged.json`); it
// refuses unless today's source and replies fixtures hash as the run recorded them. This witnesses the supported subset only: no reference-system,
// criterion-6 or C6-5 claim follows from it, and the OS chooser presentation itself is substituted.
//
// Run it under a host slot (`t3_slot.sh`); it takes the worktree build lock itself, like `run.mjs`.
import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { ENGINE_BUILDS } from "../scripts/build-wasm-engine.mjs";
import { fileIdentity, prepareSmokeApp, privateLaunchCopy } from "../scripts/appBinaryBinding.mjs";
import { checkEngine, engineStampContext } from "../scripts/engineSourceStamp.mjs";
import { BUILD_LOCK_TOKEN_ENV, acquireBuildLock, exitAfterSignals, refuseCompilerWrapperEnv, refuseHostSlotCommand, runUnderLock } from "../scripts/worktreeBuildLock.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const desktop = path.resolve(here, "..");
const project = path.resolve(desktop, "..", "..");
const tauriDir = path.join(desktop, "src-tauri");
const args = new Set(process.argv.slice(2));
const judgeAt = process.argv.indexOf("--judge");
const judgeDir = judgeAt > 1 ? process.argv[judgeAt + 1] : null;
if (judgeAt > 1 && !judgeDir) {
  console.error("mbf journey: --judge needs the evidence directory");
  process.exit(2);
}
const PHASE_TIMEOUT_MS = 20 * 60 * 1000;
const TAURI_CONF = JSON.parse(readFileSync(path.join(tauriDir, "tauri.conf.json"), "utf8"));
const SMOKE_TAURI_CONFIG = JSON.stringify({ app: { windows: TAURI_CONF.app.windows.map((window) => ({ ...window, backgroundThrottling: "disabled" })) } });

const SOURCE_FIXTURE = "fixtures/caepipe_mbf/invented/desktop_export/export_model.json";
const REPLIES_FIXTURE = "fixtures/caepipe_mbf/invented/desktop_import/import_replies.json";
const MBF_NAME = "swbpipe-model.mbf";
const IDS_NAME = "swbpipe-model.swbpipe-ids.json";

if (process.platform !== "darwin") {
  console.error("mbf journey: macOS only");
  process.exit(2);
}

const sha = (bytes) => createHash("sha256").update(bytes).digest("hex");
const fileSha = (file) => (existsSync(file) ? sha(readFileSync(file)) : null);
// RFC 8785 JCS for JSON documents (finite numbers; ECMAScript number text), as run.mjs.
function canonical(value) {
  if (Array.isArray(value)) return `[${value.map(canonical).join(",")}]`;
  if (value && typeof value === "object") {
    return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonical(value[key])}`).join(",")}}`;
  }
  return JSON.stringify(value);
}
const documentSha = (file) => {
  try { return sha(Buffer.from(canonical(JSON.parse(readFileSync(file, "utf8"))), "utf8")); } catch { return null; }
};

const commandFailed = (command, commandArgs, result) =>
  new Error(`Command failed: ${command} ${commandArgs.join(" ")}${result.error ? ` (${result.error.message})` : ""}`);
async function run(command, commandArgs, cwd, env = {}) {
  refuseHostSlotCommand(command, commandArgs);
  console.log(`$ (${path.relative(project, cwd) || "."}) ${command} ${commandArgs.join(" ")}`);
  const result = await runUnderLock(command, commandArgs, { cwd, stdio: ["ignore", "inherit", "inherit"], env: { ...process.env, CARGO_NET_OFFLINE: "true", ...env } });
  if (result.error !== null || result.status !== 0) throw commandFailed(command, commandArgs, result);
}

// ---------------------------------------------------------------- build and bind (as run.mjs)

let buildLock = null;
let engines = null;
let app = null;
let binary = null;
if (!judgeDir) {
const skipBuild = args.has("--skip-build");
try {
  buildLock = acquireBuildLock({ purpose: "mbf journey" });
} catch (error) {
  console.error(`mbf journey: ${error.message}`);
  process.exit(2);
}
process.env[BUILD_LOCK_TOKEN_ENV] = buildLock.token;
let stampContext;
try {
  stampContext = engineStampContext(desktop);
} catch (error) {
  console.error(`mbf journey: ${error.message}`);
  await exitAfterSignals(2);
}
const engineStates = () =>
  ENGINE_BUILDS.map(({ crateDirectory, destination }) => ({
    name: path.basename(destination),
    ...checkEngine({ projectRoot: project, crateDirectory, outDir: destination, context: stampContext })
  }));
const enginesBefore = engineStates();
const staleEngines = enginesBefore.filter((engine) => engine.action === "rebuild");
if (staleEngines.length > 0) {
  await run("npm", ["run", "build:wasm"], desktop);
  for (const engine of engineStates()) {
    if (engine.action !== "current") {
      console.error(`mbf journey: engine ${engine.name} still does not match its sources after build:wasm (${engine.reason})`);
      await exitAfterSignals(2);
    }
  }
}
engines = Object.fromEntries(engineStates().map((engine) => [engine.name, engine.hash]));
const APP_BUILD = {
  command: ["cargo", "build", "--locked", "--offline", "--manifest-path", path.join(tauriDir, "Cargo.toml"), "--message-format=json-render-diagnostics", "--features", "native-smoke,tauri/custom-protocol"],
  tauri_config: SMOKE_TAURI_CONFIG
};
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
  console.error(error.message.startsWith("native smoke:") ? error.message : `mbf journey: app build failed: ${error.message}`);
  await exitAfterSignals(2);
}
let launchCopy;
try {
  launchCopy = privateLaunchCopy({ desktopRoot: desktop, binary: app.binary, identity: app.identity, token: buildLock.token });
} catch (error) {
  console.error(`mbf journey: could not copy the app for launch: ${error.message}`);
  await exitAfterSignals(2);
}
process.on("exit", () => { if (!buildLock.kept()) launchCopy.remove(); });
delete process.env[BUILD_LOCK_TOKEN_ENV];
binary = launchCopy.executable;
console.log(`mbf journey: launching ${binary}, a private copy of ${app.binary} (${app.identity.sha256})`);
}

// ---------------------------------------------------------------- inputs and launches

// A re-judge interprets the kept evidence only against the exact inputs the run recorded.
const recorded = judgeDir ? JSON.parse(readFileSync(path.join(path.resolve(judgeDir), "summary.json"), "utf8")) : null;
if (recorded) {
  const now = { [SOURCE_FIXTURE]: fileSha(path.join(project, SOURCE_FIXTURE)), [REPLIES_FIXTURE]: fileSha(path.join(project, REPLIES_FIXTURE)) };
  const differs = Object.keys(now).filter((name) => recorded.inputs?.[name] !== now[name]);
  if (differs.length > 0) {
    console.error(`mbf journey: not judged: ${differs.join(", ")} differ from the inputs the run recorded (${JSON.stringify(recorded.inputs)}); check out the run's revision ${recorded.source_revision} to re-judge it`);
    process.exit(2);
  }
}
const replies = JSON.parse(readFileSync(path.join(project, REPLIES_FIXTURE), "utf8"));
const reply = (name) => JSON.parse(replies.replies[name]);
const sourceBytes = readFileSync(path.join(project, SOURCE_FIXTURE));
const source = JSON.parse(sourceBytes.toString("utf8"));
const declaration = JSON.parse(replies.inputs.declaration_json);

const root = judgeDir ? path.resolve(judgeDir) : mkdtempSync(path.join(os.tmpdir(), "swbpipe-mbf-journey-"));
const home = path.join(root, "home");
const models = path.join(root, "models");
const pairDir = path.join(root, "pair");
const movedDir = path.join(root, "moved");
const savedDir = path.join(root, "saved-model-documents");
const sourcePath = path.join(models, path.basename(SOURCE_FIXTURE));
if (!judgeDir) {
  for (const dir of [home, path.join(home, "Downloads"), models, pairDir, movedDir, savedDir]) mkdirSync(dir, { recursive: true });
  copyFileSync(path.join(project, SOURCE_FIXTURE), sourcePath);
}
const pair = { mbf: path.join(pairDir, MBF_NAME), ids: path.join(pairDir, IDS_NAME) };
const movedMbf = path.join(movedDir, MBF_NAME);
const saved = {
  bound: path.join(savedDir, "bound-adopted.json"),
  guard: path.join(savedDir, "bound-guard-kept.json"),
  unbound: path.join(savedDir, "unbound-adopted.json")
};

function launch(plan) {
  const report = path.join(root, `report-${plan.phase}.json`);
  if (judgeDir) return Promise.resolve(existsSync(report) ? JSON.parse(readFileSync(report, "utf8")) : { ok: false, error: "no report in the evidence" });
  console.log(`\n== phase ${plan.phase} (HOME=${home})`);
  if (JSON.stringify(fileIdentity(binary)) !== JSON.stringify(app.identity)) {
    console.error(`mbf journey: ${binary} changed since it was checked; not launched`);
    return exitAfterSignals(2);
  }
  return runUnderLock("/usr/bin/caffeinate", ["-di", binary], {
    cwd: root,
    env: {
      ...process.env,
      HOME: home,
      SWBPIPE_NATIVE_SMOKE_DRIVER: path.join(here, "mbf-journey.js"),
      SWBPIPE_NATIVE_SMOKE_PLAN: JSON.stringify(plan),
      SWBPIPE_NATIVE_SMOKE_REPORT: report
    },
    stdio: ["ignore", "inherit", "inherit"],
    timeoutMs: PHASE_TIMEOUT_MS
  }).then(({ status, signal, timedOut }) => {
    if (timedOut) console.error(`phase ${plan.phase}: timed out`);
    return existsSync(report) ? JSON.parse(readFileSync(report, "utf8")) : { ok: false, error: `no report (exit ${status ?? signal})` };
  });
}

const checks = [];
function check(name, ok, detail) {
  checks.push({ name, ok: Boolean(ok), detail });
  console.log(`${ok ? "PASS" : "FAIL"}  ${name}${detail === undefined ? "" : `  [${typeof detail === "string" ? detail : JSON.stringify(detail)}]`}`);
}
// "load case: 2 (2 restored, 0 formed)" rows, as the dialog prints each class of the review's identity block with
// at least one id (MbfImportDialog: underscores read as spaces; empty classes are not listed).
const identityRows = (review) => review.identity.classes.filter((row) => row.count > 0).map((row) =>
  `${row.class.replace(/_/g, " ")}: ${row.count} (${row.restored} restored, ${row.formed_from_mbf_names} formed)`);
// The native reply's adopted document, by the same RFC 8785 hash as a saved file (the reply's `modelSha256` hashes
// the worker's model bytes, whose number spelling differs from RFC 8785 text).
const adoptedSha = (name) => {
  const { document } = reply(name);
  return sha(Buffer.from(canonical(typeof document === "string" ? JSON.parse(document) : document), "utf8"));
};

check("inputs: the source fixture is the replies fixture's selected source", sha(sourceBytes) === replies.inputs.source_model_file_sha256, sha(sourceBytes));

const bound = await launch({
  phase: "bound",
  source: { path: sourcePath, project_id: source.project.id },
  pair,
  adopted_document: saved.bound,
  guard_document: saved.guard
});
check("bound: the journey completed", bound.ok, bound.error);
check("export: the dialog's default settings are the selected settings",
  bound.export && JSON.stringify({ title: bound.export.title, referenceTemperature: { value: Number(bound.export.reference_temperature), unit: "degC" }, maxLayoutRows: null })
    === replies.inputs.export_settings_json, bound.export && { title: bound.export.title, reference_temperature: bound.export.reference_temperature });
check("export: review complete, Save offered only after the intent, then saved", bound.export?.completeness && /complete/i.test(bound.export.completeness)
  && !/incomplete/i.test(bound.export.completeness) && bound.export.save_disabled_before_intent && /^Saved /.test(bound.export.status ?? ""), bound.export);
check("export: the saved MBF is the selected bytes", fileSha(pair.mbf) === replies.inputs.saved_mbf_sha256, fileSha(pair.mbf));
check("export: the saved identity file is the selected bytes", fileSha(pair.ids) === replies.inputs.saved_identity_file_sha256, fileSha(pair.ids));
check("export: the chosen folder holds exactly the pair", existsSync(pairDir) && JSON.stringify(readdirSync(pairDir).sort()) === JSON.stringify([MBF_NAME, IDS_NAME].sort()), existsSync(pairDir) && readdirSync(pairDir));
const boundReview = reply("bound-open").review;
check("bound import: the identity file was applied", /Identity file applied/.test(bound.import?.identity ?? ""), bound.import?.identity);
check("bound import: every identity class row is the native review's", identityRows(boundReview).every((row) => bound.import?.identity?.includes(row)), identityRows(boundReview));
check("bound import: the review is complete", /complete/i.test(bound.import?.completeness ?? "") && !/incomplete|truncat/i.test(bound.import?.completeness ?? ""), bound.import?.completeness);
check("bound import: every loss entry is listed, the source-completeness notice included", bound.import?.loss?.length === boundReview.loss_entries.length
  && boundReview.loss_entries.every((entry) => bound.import.loss.some((item) => item.includes(entry.code))), bound.import?.loss);
const designateReply = reply("bound-designate");
const designatedReview = designateReply.review;
const designation = designatedReview.identity.designation;
check("bound designate: the native designation rests on the bound review and names the opened source",
  designateReply.baseReviewSha256 === reply("bound-open").reviewSha256 && designation?.sha256 === sha(sourceBytes) && designation?.file_name === path.basename(SOURCE_FIXTURE),
  { base: designateReply.baseReviewSha256, designation });
check("bound designate: the review states the identity set established for the designated source, with no reason or refusal",
  designation?.outcome === "established_for_identity_set" && bound.designated?.designation?.includes(`${designation.file_name} (${designation.bytes} bytes)`)
  && /the import has the same ids in each class/.test(bound.designated?.designation ?? "") && bound.designated?.reasons?.length === 0
  && !bound.designated?.differing_classes && !bound.designated?.designation_refusal && /^Established for the identity set/.test(bound.designated?.source_completeness ?? ""),
  bound.designated && { designation: bound.designated.designation, reasons: bound.designated.reasons, refusal: bound.designated.designation_refusal, source_completeness: bound.designated.source_completeness });
check("bound designate: every identity class row is the designated review's", identityRows(designatedReview).every((row) => bound.designated?.identity?.includes(row)), identityRows(designatedReview));
check("bound designate: the designated review is complete and offers Adopt", /complete/i.test(bound.designated?.completeness ?? "") && !/incomplete|truncat/i.test(bound.designated?.completeness ?? "")
  && bound.designated?.adopt_disabled === false, bound.designated && { completeness: bound.designated.completeness, adopt_disabled: bound.designated.adopt_disabled });
check("bound adopt of the designated review: the session holds the unsaved import", /Not saved as a local project yet/.test(bound.adopted?.message ?? ""), bound.adopted);
// Adopt of the designated review takes its model from the pair: the native designated Adopt's document is the bound Adopt's.
check("bound adopt of the designated review: the adopted model is the native designated Adopt's, which is the bound import's",
  documentSha(saved.bound) === adoptedSha("bound-designated-adopt") && adoptedSha("bound-designated-adopt") === adoptedSha("bound-adopt"), documentSha(saved.bound));
check("guard: the edit applied and the session is edited", /1 applied/.test(bound.guard?.edit?.summary ?? "") && /Edited/.test(bound.guard?.edit?.edited ?? ""), bound.guard?.edit);
check("guard: Adopt over unsaved edits asks first; cancelling opens nothing and keeps the edits",
  bound.guard?.prompt && /Not opened/.test(bound.guard.message ?? "") && bound.guard.dialog_closed && /Edited/.test(bound.guard.edited_after ?? ""), bound.guard);
let guardModel = null;
try { guardModel = JSON.parse(readFileSync(saved.guard, "utf8")); } catch { guardModel = null; }
check("guard: the kept session is the adopted import with node:A's y edited", guardModel
  && guardModel.nodes?.find((node) => node.id === "node:A")?.position?.y === 0.5 && documentSha(saved.guard) !== documentSha(saved.bound), guardModel?.nodes?.find((node) => node.id === "node:A"));

if (!judgeDir) copyFileSync(pair.mbf, movedMbf);
const unbound = await launch({ phase: "unbound", moved_mbf: movedMbf, adopted_document: saved.unbound });
check("unbound: the journey completed", unbound.ok, unbound.error);
const unboundReview = reply("unbound-declared").review;
check("unbound import: Adopt waits for the material declaration", unbound.import?.adopt_disabled === true, unbound.import);
check("unbound import: one declaration per undeclared material", JSON.stringify(unbound.declared) === JSON.stringify(Object.keys(declaration).map((name) => `Source of ${name}`)), unbound.declared);
check("unbound import: every identity class is formed from MBF names", identityRows(unboundReview).every((row) => unbound.declared_review?.identity?.includes(row)), unbound.declared_review?.identity);
check("unbound adopt: the adopted model is the native declared unbound import's", documentSha(saved.unbound) === adoptedSha("unbound-adopt"), documentSha(saved.unbound));

let revision = null;
let dirty = null;
try {
  revision = execFileSync("git", ["-C", project, "rev-parse", "HEAD"], { encoding: "utf8" }).trim();
  dirty = execFileSync("git", ["-C", project, "status", "--porcelain"], { encoding: "utf8" }).trim().split("\n").filter(Boolean);
} catch { /* recorded as unknown */ }
const failed = checks.filter((item) => !item.ok);
writeFileSync(path.join(root, judgeDir ? "summary-judged.json" : "summary.json"), JSON.stringify({
  witness: "supported-subset MBF round trip in the launched native-smoke app; chooser presentation substituted",
  source_revision: recorded ? recorded.source_revision : revision,
  uncommitted: recorded ? recorded.uncommitted : dirty,
  ...(recorded ? { judged_at_revision: revision, judged_uncommitted: dirty } : {}),
  app: recorded ? recorded.app : { binary: app.binary, identity: app.identity },
  engines: recorded ? recorded.engines : engines,
  inputs: { [SOURCE_FIXTURE]: sha(sourceBytes), [REPLIES_FIXTURE]: fileSha(path.join(project, REPLIES_FIXTURE)) },
  outputs: {
    mbf: fileSha(pair.mbf), identity_file: fileSha(pair.ids),
    bound_adopted_model: documentSha(saved.bound), guard_kept_model: documentSha(saved.guard), unbound_adopted_model: documentSha(saved.unbound)
  },
  checks,
  reports: { bound, unbound }
}, null, 2));
console.log(`\nmbf journey: ${checks.length - failed.length}/${checks.length} checks passed; evidence in ${root}`);
if (!judgeDir && args.has("--discard") && failed.length === 0) rmSync(root, { recursive: true, force: true });
buildLock?.release();
await exitAfterSignals(failed.length === 0 ? 0 : 1);
