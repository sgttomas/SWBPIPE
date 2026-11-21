#!/usr/bin/env node
// The native walkthrough's command (../README.md, "The launched app"), macOS:
//
//   npm run walkthrough:native --workspace apps/desktop -- --build-only
//   npm run walkthrough:native --workspace apps/desktop -- [--journey <names>] [--out DIR]
//
// Run each under a host slot, as separate jobs, as `native-smoke/evidence-journeys.mjs` says of itself; each takes
// the worktree build lock itself. --build-only builds and binds the `native-smoke` app exactly as that runner does
// (the same build command and smoke window configuration, so one binding serves both), prints the app's identity and
// launches nothing. A launch invocation builds nothing: anything that would need building refuses it by name before
// any launch.
//
// Journeys (--journey a,b; without it, the first three), each one launch in its own new throwaway home:
//
//   solve-and-results  20  The bundled demonstration model: Run, the progress, the Results stage and each of its
//                          panels, the filter to a support's rows, a row chosen and its place in the viewport, Key
//                          results and each of its views, the Evidence tab, the Analyze page, the Issues drawer.
//   report             21  Solved, then the Review stage, the Report tab, "Load for…" where offered, the composer,
//                          Render (local private), the preview, Save Shareable Report Package… to its release dialog
//                          and its Cancel, Save Report Package… answered by the queued chooser, the outcome list.
//                          Print is not pressed.
//   exports            22  Solved, then the Exports page, top to foot.
//   reference-system   23  Named alone (its solve is long, as evidence-journeys.mjs keeps it apart): File > Open Model
//                          Document… of the reference system fixture, Run, the standing, the health line, Results,
//                          Report.
//   prepared-project   24  A model document placed in the throwaway home before launch (the journey's `place`),
//                          opened by File > Open Model Document…: the start other journeys build on.
//
// At each step the driver (`driver.js`, given the shared counts of `../screen-counts.mjs` and the snapshot of
// `snapshot.mjs`) records the counts and a snapshot of the document as drawn, as the checkpoint `step-<number>`.
// After the launch has ended and been censused and its home removed, each snapshot is rendered in Playwright's
// Chromium at the recorded window size (`render.mjs`) into the output folder, in the browser walkthrough's form
// (numbered pictures, part files, INDEX.json, index.html). The pictures are redrawn, not captures of the window:
// capturing the window from outside needs a permission this runner does not ask for.
//
// Confinement: as every native-smoke runner (`launch-env.mjs`, `judged-launch.mjs`, `marks-check.mjs`), with their
// own functions: an environment built from nothing, HOME and CFFIXED_USER_HOME the throwaway home with TMPDIR inside
// it, the strict chooser (a chooser with no queued answer inside the home ends the app, exit 86, before any dialog),
// a non-persistent web view store, a private copy of the bound app checked before each launch, the three marks checked
// directly before each start, the run marker shared with the other runners, the census of the private copy after each
// launch, the tripwire over the real user's product-named Library paths before the first launch and after the last,
// and closeRun on every exit the runner can catch. A home is removed only as evidence-journeys.mjs removes one
// (homeRemovalRefusal, removalIdentityRefusal, the run's token in its marker). In this build the print dialog is
// replaced by a record, and Print is not pressed. The embedded agent is never started; no provider, model, key,
// network or sign-in is used. Chromium renders after the app has ended, loads nothing, and is given no file of the
// home. Evidence of what a person sees, not a test: a journey that stops is a finding in the index.
import { randomBytes } from "node:crypto";
import { execFileSync } from "node:child_process";
import * as fs from "node:fs";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync, writeSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { emptyOutput } from "../run.mjs";
import { composeIndex } from "../compose-index.mjs";
import { readScreenCounts, scrollLongestRegion } from "../screen-counts.mjs";
import { takeSnapshot } from "./snapshot.mjs";
import { readStepRecords, renderScreens, screensFromRecords, writePart } from "./render.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const walkthrough = path.resolve(here, "..");
const desktop = path.resolve(walkthrough, "..");
const project = path.resolve(desktop, "..", "..");
const tauriDir = path.join(desktop, "src-tauri");
const NAME = "native walkthrough";

/**
 * The journeys, numbered from 20, after the browser walkthrough's. A journey may declare `place`: files copied from
 * the repository's fixtures into the launch's throwaway home before the app starts (placementsFor), each with a `key`
 * the driver finds its path by (`plan.placed[key]`), `at` a path inside the home (`{app-data}/` is the app's own
 * data folder there, where its local project store lives) and `from` a committed file under `fixtures/`. `open`
 * says how the journey opens what was placed, by the app's own File menu: `{ by: "model-document", file: key }`
 * (File > Open Model Document…, the chooser's answer the placed file) or `{ by: "local-project", project_id }` (File >
 * Open Local Project…, from a placed store). The driver's journey of the same name opens it (driver.js
 * openPrepared).
 */
export const JOURNEYS = Object.freeze({
  "solve-and-results": {
    number: 20, title: "Solve and read results (launched app): Run, the progress, Results and its panels, a support's rows, Key results, Evidence, Analyze and Issues",
    use_case: "DEL-07-06 AC-007 Review and export results, in part: on the bundled demonstration model, not the reference system; no CSV export"
  },
  report: {
    number: 21, title: "Report (launched app): the Review stage, the Report tab, the composer, Render, the preview, the release dialog and a saved package",
    use_case: "DEL-07-06 AC-007 Blank to released report, in part: from the solve of the demonstration model to a saved local-private package; the release is cancelled and Print is not pressed"
  },
  exports: {
    number: 22, title: "Exports (launched app): the Exports page after the solve, top to foot",
    use_case: "DEL-07-06 AC-007 Review and export results, in part: the Exports page as a person meets it; nothing is exported"
  },
  "reference-system": {
    number: 23, title: "Reference system (launched app): opened as a model document, solved, its standing, health line, Results and Report",
    use_case: "DEL-07-06 AC-007 Review and export results, in part: the reference system opened, solved and its results and report met; no sorting to the largest values and no CSV export",
    place: [{ key: "model", at: "chooser/models/reference_system.json", from: "fixtures/reference_system/reference_system.json" }],
    open: { by: "model-document", file: "model" },
    timeouts: { solve: 45 * 60 * 1000 }, launch_ms: 75 * 60 * 1000
  },
  "prepared-project": {
    number: 24, title: "Prepared project (launched app): a model document placed in the throwaway home before launch, opened by File > Open Model Document…",
    use_case: "none of its own: shows that a journey can start from a prepared project, the start of the use cases that open one",
    place: [{ key: "model", at: "chooser/models/exact_pressure_authoring_model.json", from: "fixtures/model_operations/exact_pressure_authoring_model.json" }],
    open: { by: "model-document", file: "model" }
  }
});
export const DEFAULT_JOURNEYS = Object.freeze(["solve-and-results", "report", "exports"]);
// Named alone: its solve is long, as evidence-journeys.mjs keeps it apart.
export const SEPARATE_JOURNEYS = Object.freeze(["reference-system"]);

/** The command line: { buildOnly, journeys, out } or { error }. --build-only takes nothing else. */
export function parseArguments(argv, { defaultOut = path.join(walkthrough, "output") } = {}) {
  const options = { buildOnly: false, journeys: null, out: null };
  for (let index = 0; index < argv.length; index += 1) {
    const name = argv[index];
    if (name === "--build-only") { options.buildOnly = true; continue; }
    if (name !== "--journey" && name !== "--out") return { error: `unknown argument ${name}` };
    const value = argv[index + 1];
    if (value === undefined || value.startsWith("--")) return { error: `${name} needs a value` };
    if (options[name.slice(2) === "journey" ? "journeys" : "out"] !== null) return { error: `${name} is given twice` };
    if (name === "--journey") options.journeys = value.split(",").map((item) => item.trim()).filter(Boolean);
    else options.out = value;
    index += 1;
  }
  if (options.buildOnly) {
    if (options.journeys !== null || options.out !== null) return { error: "--build-only builds and binds the app and takes no journey or output folder" };
    return { buildOnly: true, journeys: [], out: null };
  }
  const journeys = options.journeys ?? [...DEFAULT_JOURNEYS];
  const unknown = journeys.filter((item) => !Object.hasOwn(JOURNEYS, item));
  if (journeys.length === 0 || unknown.length > 0) return { error: `choose --journey with one or more of ${DEFAULT_JOURNEYS.join(", ")} (or ${SEPARATE_JOURNEYS.join(", ")} alone)${unknown.length ? `; unknown: ${unknown.join(", ")}` : ""}` };
  if (new Set(journeys).size !== journeys.length) return { error: "a journey is named twice" };
  if (journeys.some((item) => SEPARATE_JOURNEYS.includes(item)) && journeys.length > 1) return { error: `${SEPARATE_JOURNEYS.join(", ")} runs alone: name it as the only journey` };
  return { buildOnly: false, journeys, out: path.resolve(options.out ?? defaultOut) };
}

/** The app's own data folder inside a home, where a placed local project store goes. */
export const APP_DATA = "{app-data}";

/**
 * Where each declared file goes: [{ key, from, to }], absolute paths, `to` strictly inside `home`. Throws, naming the
 * entry, when `at` is absolute, climbs (`..`) or is empty; when `from` is not a committed path under `fixtures/`; when
 * the source is not a regular file (a link is refused); or when two entries share a key or a destination.
 */
export function placementsFor(place = [], { home, identifier, projectRoot, lstat = fs.lstatSync }) {
  const seen = new Set();
  return place.map((entry, index) => {
    const name = `place[${index}] (${entry?.key ?? "no key"})`;
    if (!entry || typeof entry.key !== "string" || !/^[a-z][a-z0-9-]*$/.test(entry.key)) throw new Error(`${name}: a key is a-z, 0-9 and -`);
    if (typeof entry.at !== "string" || entry.at === "" || path.isAbsolute(entry.at) || entry.at.split("/").some((part) => part === ".." || part === "." || part === "")) {
      throw new Error(`${name}: at must be a relative path inside the home without . or ..`);
    }
    if (typeof entry.from !== "string" || !entry.from.startsWith("fixtures/") || entry.from.split("/").some((part) => part === ".." || part === "." || part === "")) {
      throw new Error(`${name}: from must be a committed path under fixtures/`);
    }
    const relative = entry.at.startsWith(`${APP_DATA}/`) ? path.join("Library", "Application Support", identifier, entry.at.slice(APP_DATA.length + 1)) : entry.at;
    if (relative.includes("{")) throw new Error(`${name}: ${APP_DATA}/ is the only placeholder`);
    const from = path.join(projectRoot, entry.from);
    let stat;
    try { stat = lstat(from); } catch { throw new Error(`${name}: ${entry.from} does not exist`); }
    if (!stat.isFile()) throw new Error(`${name}: ${entry.from} is not a regular file`);
    const to = path.join(home, relative);
    if (seen.has(entry.key) || seen.has(to)) throw new Error(`${name}: a key or destination is declared twice`);
    seen.add(entry.key);
    seen.add(to);
    return { key: entry.key, from, to };
  });
}

/** Copies each placement into the home (never over an existing file) and returns { key: path }. */
export function placeFiles(placements) {
  const placed = {};
  for (const { key, from, to } of placements) {
    fs.mkdirSync(path.dirname(to), { recursive: true });
    fs.copyFileSync(from, to, fs.constants.COPYFILE_EXCL);
    placed[key] = to;
  }
  return placed;
}

/** The one script the app evaluates: the shared functions' own text, then the driver. */
export function composeDriver(driverText) {
  return [
    "// Composed by walkthrough/native/run.mjs: screen-counts.mjs and snapshot.mjs, then driver.js.",
    `const readScreenCounts = ${readScreenCounts.toString()};`,
    `const scrollLongestRegion = ${scrollLongestRegion.toString()};`,
    `const takeSnapshot = ${takeSnapshot.toString()};`,
    driverText
  ].join("\n");
}

/** Removes the earlier output of the journeys this run replaces (their numbers), keeping every other journey's. */
export function emptyNativeOutput(out, journeys) {
  const numbers = new Set(journeys.map((item) => JOURNEYS[item].number));
  emptyOutput(out, (journeyNumber) => numbers.has(journeyNumber));
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) await main();

async function main() {
  const { ENGINE_BUILDS } = await import("../../scripts/build-wasm-engine.mjs");
  const { bindingPathFor, fileIdentity, prepareSmokeApp, privateLaunchCopy } = await import("../../scripts/appBinaryBinding.mjs");
  const { checkEngine, engineStampContext } = await import("../../scripts/engineSourceStamp.mjs");
  const { BUILD_LOCK_TOKEN_ENV, acquireBuildLock, exitAfterSignals, processStartTime, refuseCompilerWrapperEnv, refuseHostSlotCommand, runUnderLock } = await import("../../scripts/worktreeBuildLock.mjs");
  const {
    awaitNoProcessesSync, buildLaunchEnv, censusOf, closeRun, createThrowawayHome, keepOutOfHome, keptRelativePaths, readRunMarker,
    refusedLaunchEnv, refusedPlanPaths, refusedSmokeWindows, runMarkerRefusal, smokeWindows, statWatched, tripwireChanges, watchedLibraryPaths, writeRunMarker
  } = await import("../../native-smoke/launch-env.mjs");
  const { newNonce, readReport } = await import("../../native-smoke/launch-outcome.mjs");
  const { judgedPhase, phaseContinuation } = await import("../../native-smoke/judged-launch.mjs");
  const { HOME_MARKER, LEFT_HOME_RULE, NOTHING_ELSE_WATCHED, PROCESS_SCOPE, STRICT_CHOOSER_EXIT, TRIPWIRE_LIMITS, homeRemovalRefusal, readHomeMarker, removalIdentityRefusal, sha256Hex } = await import("../../native-smoke/evidence-checks.mjs");

  const refuseLine = (text) => console.error(`${NAME}: ${text}`);
  if (process.platform !== "darwin") {
    refuseLine("macOS only");
    process.exit(2);
  }
  const parsed = parseArguments(process.argv.slice(2));
  if (parsed.error) {
    refuseLine(parsed.error);
    process.exit(2);
  }
  const { buildOnly, journeys: selected, out } = parsed;

  // The named timeouts (ms), passed to the driver, and each launch's hard limit.
  const TIMEOUTS = { ui: 60_000, step: 300_000, solve: 900_000, save: 900_000 };
  const LAUNCH_MS = 45 * 60 * 1000;
  const TERMINATION_BOUND_MS = 20 * 1000;
  const CAFFEINATE = "/usr/bin/caffeinate";
  const TAURI_CONF = JSON.parse(readFileSync(path.join(tauriDir, "tauri.conf.json"), "utf8"));
  const IDENTIFIER = TAURI_CONF.identifier;
  // As every native-smoke runner: background throttling off and a non-persistent web view store.
  const SMOKE_WINDOWS = smokeWindows(TAURI_CONF.app.windows);
  const SMOKE_TAURI_CONFIG = JSON.stringify({ app: { windows: SMOKE_WINDOWS } });
  // The run marker the other native-smoke runners use, so each recognises the others' unresolved run.
  const RUN_MARKER = path.join(desktop, "target", "native-smoke-run.json");
  const DEMO_FIXTURE = "fixtures/product_preview/invented_demo_model.json";

  {
    const record = readRunMarker(RUN_MARKER);
    if (record !== null) {
      const alive = !record.unreadable && Number.isInteger(record.pid) && typeof record.started === "string" && processStartTime(record.pid) === record.started;
      const census = !record.unreadable && typeof record.executable === "string" ? censusOf(record.executable) : { ok: false, reason: "the marker records no executable" };
      for (const line of runMarkerRefusal(RUN_MARKER, record, { alive, census })) console.error(line);
      process.exit(2);
    }
  }
  // The output folder is checked before anything is built or launched: a folder of other work is refused here.
  if (!buildOnly) {
    try { emptyNativeOutput(out, selected); } catch (error) { refuseLine(error.message); process.exit(2); }
  }

  // ---------------------------------------------------------------- build and bind (as evidence-journeys.mjs), or check only

  let buildLock;
  try {
    buildLock = acquireBuildLock({ purpose: NAME });
  } catch (error) {
    refuseLine(error.message);
    process.exit(2);
  }
  process.env[BUILD_LOCK_TOKEN_ENV] = buildLock.token;
  const commandFailed = (command, commandArgs, result) => new Error(`Command failed: ${command} ${commandArgs.join(" ")}${result.error ? ` (${result.error.message})` : ""}`);
  async function run(command, commandArgs, cwd) {
    if (!buildOnly) throw new Error(`NATIVE-WALKTHROUGH-LAUNCH-WOULD-BUILD: ${command} ${commandArgs.join(" ")}; build first with --build-only`);
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
        if (!buildOnly) throw new Error(`NATIVE-WALKTHROUGH-LAUNCH-WOULD-BUILD: ${command} ${commandArgs.join(" ")}; build first with --build-only`);
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
  const fileSha = (file) => (fs.existsSync(file) ? sha256Hex(readFileSync(file)) : null);
  const bindingSha = fileSha(bindingPathFor(desktop));
  console.log(`${NAME}: app ${app.binary} (sha256 ${app.identity.sha256}, ${app.identity.size} bytes); binding record sha256 ${bindingSha}`);
  if (buildOnly) {
    delete process.env[BUILD_LOCK_TOKEN_ENV];
    buildLock.release();
    console.log(`${NAME}: built and bound; nothing launched`);
    await exitAfterSignals(0);
  }

  // ---------------------------------------------------------------- the launch copy and the run's state

  const demo = JSON.parse(readFileSync(path.join(project, DEMO_FIXTURE), "utf8"));
  const firstSupport = demo.supports?.[0]?.id;
  if (typeof firstSupport !== "string") {
    refuseLine(`${DEMO_FIXTURE} names no support to filter the results by`);
    await exitAfterSignals(2);
  }
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
  let uncommitted = null;
  try {
    revision = execFileSync("git", ["-C", project, "rev-parse", "HEAD"], { encoding: "utf8" }).trim();
    uncommitted = execFileSync("git", ["-C", project, "status", "--porcelain"], { encoding: "utf8" }).trim().split("\n").filter(Boolean).length;
  } catch { /* recorded as unknown */ }

  // The run's folder, kept as its evidence (reports, step records, snapshots); its homes are removed.
  const RUN_ROOT = realpathSync(mkdtempSync(path.join(os.tmpdir(), "swbpipe-native-walkthrough-")));
  const TOKEN = randomBytes(16).toString("hex");
  const identities = new Map();
  const remember = (dir) => { const seen = lstatSync(dir); identities.set(dir, { dev: seen.dev, ino: seen.ino }); };
  remember(RUN_ROOT);
  const createdHomes = new Set();
  const removedHomes = new Set();
  const keptHomes = new Set();
  const leftHomes = new Map();
  const DRIVER = path.join(RUN_ROOT, "driver.js");
  writeFileSync(DRIVER, composeDriver(readFileSync(path.join(here, "driver.js"), "utf8")), { flag: "wx" });
  process.on("exit", closeOnExit);
  console.log(`${NAME}: launching ${binary}, a private copy of ${app.binary} (${app.identity.sha256}); evidence in ${RUN_ROOT}; pictures in ${out}`);

  function check(name, ok, detail) {
    checks.push({ journey, name, ok: Boolean(ok), detail });
    console.log(`${ok ? "PASS" : "FAIL"}  ${journey}: ${name}${detail === undefined ? "" : `  [${(typeof detail === "string" ? detail : JSON.stringify(detail) ?? "").slice(0, 600)}]`}`);
  }
  function say(text) {
    try { writeSync(2, `${text}\n`); } catch { /* the run's record holds it */ }
  }

  // A journey's folder in the run's folder (named as homeRemovalRefusal requires), its throwaway home inside it,
  // marked with the run's token; every chooser answer lies in the home.
  function journeyRoot(name) {
    const root = realpathSync(mkdtempSync(path.join(RUN_ROOT, `swbpipe-evidence-walk-${name}-`)));
    remember(root);
    const { home, tmpdir } = createThrowawayHome(root);
    remember(home);
    createdHomes.add(home);
    writeFileSync(path.join(home, HOME_MARKER), TOKEN, { flag: "wx" });
    const chooser = path.join(home, "chooser");
    const models = path.join(chooser, "models");
    const saved = path.join(chooser, "saved");
    for (const dir of [chooser, models, saved]) mkdirSync(dir);
    return { root, home, tmpdir, models, saved };
  }

  // As evidence-journeys.mjs removeHome: the home's files copied out once, then removed only when it is one this run
  // created, its folders up to the run's folder are the recorded ones and its marker holds this run's token.
  function removeHome(home) {
    if (removedHomes.has(home)) return null;
    if (!keptHomes.has(home)) {
      keptHomes.add(home);
      try {
        keepOutOfHome(home, path.dirname(home), keptRelativePaths(IDENTIFIER));
      } catch (error) {
        leftHomes.set(home, `copying its files out failed: ${error.message}`);
        say(`${NAME}: the files of ${home} could not all be copied out (${error.message}); the home is left`);
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
  function beforeStart(where) {
    if (state.watchedBefore === null) state.watchedBefore = statWatched(WATCHED);
    writeRunMarker(RUN_MARKER, { pid: process.pid, started: processStartTime(process.pid), home: where.home, executable: binary, evidence_root: RUN_ROOT, created: new Date().toISOString() });
    state.launched = true;
    state.termination = null;
  }
  const runRecord = () => ({ run_folder: RUN_ROOT, output: out, source_revision: revision, journeys: selected, checks });

  // One judged launch (`judged-launch.mjs`); a mark present or an unresolved census ends the run there.
  async function launch(where, plan, { timeoutMs }) {
    const phase = `${journey}/walk`;
    const report = path.join(where.root, `report-${journey}.json`);
    const nonce = newNonce();
    console.log(`\n== ${journey} (HOME=${where.home})`);
    const env = launchEnv(where, { ...plan, nonce, timeouts: { ...TIMEOUTS, ...(plan.timeouts ?? {}) } }, report);
    if (env === null) return exitAfterSignals(2);
    const seen = await judgedPhase({
      phase, checkName: "the launch ended cleanly and its report is this run's", launches, check,
      summaryPath: path.join(RUN_ROOT, "summary.json"), summary: runRecord,
      executable: binary, command: CAFFEINATE, commandArgs: ["-di", binary], cwd: where.root, env, timeoutMs, reportPath: report, nonce,
      censusBoundMs: TERMINATION_BOUND_MS, beforeStart: () => beforeStart(where)
    }).then(phaseContinuation({ prefix: NAME, phase, state, evidenceRoot: RUN_ROOT }));
    if (launches[phase]?.outcome?.status === STRICT_CHOOSER_EXIT) {
      const lastAct = readReport(`${report}.act.json`).report?.act ?? "unnamed (the driver recorded no act)";
      check("no chooser was asked without a queued answer inside the home", false, `the app ended with the strict chooser's exit code ${STRICT_CHOOSER_EXIT} during the act: ${lastAct}`);
    }
    return { report, seen, failures: launches[phase]?.failures ?? [] };
  }

  async function walk(name) {
    const spec = JOURNEYS[name];
    const where = journeyRoot(name);
    const plan = { journey: name, number: spec.number, bundled_project_id: demo.project.id, support_text: firstSupport.replace(/^support:/, "") };
    const timeoutMs = spec.launch_ms ?? LAUNCH_MS;
    if (spec.timeouts) plan.timeouts = spec.timeouts;
    if (name === "report") plan.local_package = path.join(where.saved, "walkthrough-local.opsproj");
    // The prepared files, in the home before the app starts; refused by name when a declaration is wrong.
    try {
      const placements = placementsFor(spec.place, { home: where.home, identifier: IDENTIFIER, projectRoot: project });
      plan.placed = placeFiles(placements);
      if (spec.open) {
        plan.open = { ...spec.open, ...(spec.open.file ? { file: plan.placed[spec.open.file] } : {}) };
        if (spec.open.by === "model-document") plan.open.project_id = JSON.parse(readFileSync(plan.open.file, "utf8")).project.id;
      }
      check("the prepared files were placed in the throwaway home", true, placements.map(({ key, from }) => `${key}: ${path.relative(project, from)}`));
    } catch (error) {
      check("the prepared files were placed in the throwaway home", false, error.message);
      return;
    }
    const { report, seen, failures } = await launch(where, plan, { timeoutMs });
    if (seen && seen.ok === false && seen.error) console.log(`${NAME}: ${name}: the driver reported ${seen.error.split("\n")[0]}`);
    // The launch was censused (an unresolved census ended the run), so its home may go before anything is rendered.
    if (removeHome(where.home) !== null) check("its throwaway home was removed", false, leftHomes.get(where.home));
    else check("its throwaway home was removed", true);
    const records = readStepRecords(report);
    const launchFailure = failures.length > 0 ? failures.join("; ") : (seen?.ok === false ? seen.error?.split("\n")[0] ?? "the driver did not finish" : null);
    const screens = screensFromRecords(records, { journey: name, number: spec.number, launchFailure });
    const { chromium } = await import("@playwright/test");
    const executablePath = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH || undefined;
    await renderScreens(records, screens, { out, snapshots: path.join(where.root, "snapshots"), launchBrowser: () => chromium.launch({ executablePath }) });
    writePart(out, {
      number: spec.number, name, title: spec.title, use_case: spec.use_case, screens,
      revision: { commit: revision, uncommitted_paths: uncommitted },
      build: { kind: "scripted-run build (native-smoke feature), launched as a private copy", app_sha256: app.identity.sha256, app_bytes: app.identity.size, binding_record_sha256: bindingSha }, launch: { ended_cleanly: failures.length === 0, failures, driver_finished: seen?.ok === true } });
    const pictured = screens.filter((screen) => screen.file).length;
    const stop = screens.find((screen) => screen.failed);
    console.log(`${NAME}: ${name}: ${pictured} screens${stop ? `; stopped at "${stop.step}": ${stop.failed}` : ""}`);
    writeFileSync(path.join(where.root, "summary.json"), JSON.stringify({ journey: name, source_revision: revision, app: { binary: app.binary, identity: app.identity, binding_record_sha256: bindingSha }, launches: { [name]: launches[`${name}/walk`] }, checks: checks.filter((item) => item.journey === name) }, null, 2));
  }

  for (const name of selected) {
    journey = name;
    await walk(name);
  }
  journey = "run";
  if (state.watchedBefore !== null) {
    const changes = tripwireChanges(state.watchedBefore, statWatched(WATCHED));
    state.tripwireDone = true;
    check(`the real user's Library: none of the ${WATCHED.length} product-named paths appeared, disappeared or changed`, changes.length === 0, changes);
  }
  const index = composeIndex(out);
  console.log(`${NAME}: the index now holds ${index.screens} screens over ${index.journeys} journeys; open ${path.join(out, "index.html")}`);
  const failed = checks.filter((item) => !item.ok);
  console.log(`${NAME}: ${checks.length - failed.length}/${checks.length} confinement and launch checks passed; evidence in ${RUN_ROOT}`);
  buildLock.release();
  await exitAfterSignals(failed.length === 0 ? 0 : 1);

  // As evidence-journeys.mjs closeOnExit: the census if a launch was not censused, the tripwire through closeRun,
  // then retention (an unresolved termination keeps the homes, the copy, the marker and the lock) or the removal of
  // each home this run created and of the private copy. A home left keeps the marker and fails the run.
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
        process_scope: PROCESS_SCOPE, tripwire: { watched: WATCHED, limits: TRIPWIRE_LIMITS },
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
}
