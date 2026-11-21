#!/usr/bin/env node
// The native walkthrough's command (../README.md, "The launched app"), macOS:
//
//   npm run walkthrough:native --workspace apps/desktop -- --build-only
//   npm run walkthrough:native --workspace apps/desktop -- [--journey <names>] [--out DIR] [--show-window]
//   npm run walkthrough:native --workspace apps/desktop -- --attended [--journey <name>] [--limit-minutes N] [--hidden] [--attendant-on-screen <who>] --out DIR
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
//   refused-stored-run 25  A local project store placed before launch, its saved run's record binding the result
//                          rules at a revision this version does not read: File > Open Local Project…, the Project
//                          page, the Results stage, File > Save Local Project, and whether the stored run's texts
//                          are still the placed ones in copies of the store taken after the open and the save.
//
// Each launch reads the menu the app built once (`native_smoke_menu_items`, a command of the scripted-run build), and
// the index records it (`native_menus`). --show-window, off by default, has the app bring its own window to the front
// for the run (`native_smoke_show_window`: the window's own API, no system automation, nothing captured from the
// screen), so its web view reports its page visible and the 3D view draws; the window closes with the app.
//
// At each step the driver (`driver.js`, given the shared counts of `../screen-counts.mjs` and the snapshot of
// `snapshot.mjs`) records the counts and a snapshot of the document as drawn, as the checkpoint `step-<number>`.
// After the launch has ended and been censused and its home removed, each snapshot is rendered in Playwright's
// Chromium at the recorded window size (`render.mjs`) into the output folder, in the browser walkthrough's form
// (numbered pictures, part files, INDEX.json, index.html). The pictures are redrawn, not captures of the window:
// this runner takes no capture of the window, pictures no menu bar and presses no Print (an attended capture can
// supply those surfaces: compose-index.mjs nativeSurfacesOf).
//
// Confinement: as every native-smoke runner (`launch-env.mjs`, `judged-launch.mjs`, `marks-check.mjs`), with their
// own functions: an environment built from nothing, HOME and CFFIXED_USER_HOME the throwaway home with TMPDIR inside
// it, the strict chooser (a chooser with no queued answer inside the home ends the app, exit 86, before any dialog),
// a non-persistent web view store, a private copy of the bound app checked before each launch, the three marks checked
// directly before each start, the run marker shared with the other runners, the census of the private copy after each
// launch, the tripwire over the real user's product-named Library paths before the first launch and after the last,
// and closeRun on every exit the runner can catch. A home is removed only as evidence-journeys.mjs removes one
// (homeRemovalRefusal, removalIdentityRefusal, the run's token in its marker). In this build the print dialog is
// replaced by a record, and Print is not pressed. The embedded agent is never started (an attended launch stops its
// Start agent); no provider, model, key, network or sign-in is used. Chromium renders after the app has ended, loads nothing, and is given no file of the
// home. Evidence of what a person sees, not a test: a journey that stops is a finding in the index.
//
// --attended (ATTENDED below): one launch made exactly as a journey's (the same launch function, environment,
// confinement, census, tripwire and close), for an attendant, an agent outside the application that drives the actual
// window. A refusal for --out, or for a mark, is read-only and comes before either cleanup. With
// --attendant-on-screen <who>, the caller's statement that the agent operating the screen is this launch's attendant,
// the marks of that agent do not refuse the start (judged-launch.mjs marksDecision; never a check not made or another
// application's bundle), and the marks and the statement are printed at the handover and recorded. The driver makes only the start (with
// --journey, that journey's `place` and `open`; without it, the app as first opened), shows the window unless
// --hidden, reads the menu, stops the page's file fields and the agent's Start agent, writes the checkpoint `start`,
// and then takes no step: it only records, with times, what it stopped, each report print the build substitutes and
// the counts on screen whenever the page settles. The handover lines are printed only once that checkpoint says the
// start was made with this launch's nonce; a failed start ends the launch without them. The launch ends when the app
// exits by itself, when the finish file the command prints appears, or at --limit-minutes: for those two the watcher
// aborts the AbortSignal judgedPhase hands to runUnderLock, which owns the started child and sends it SIGKILL, and the
// record keeps runUnderLock's receipt (its own time limit, two minutes later, is the backstop). The strict chooser's
// exit is recorded as the ending only. The index gets one part (number 90) recording the launch, and one screen for
// each picture the attendant put in <out>/captures/, read once at the end from the folder the runner created (its
// device and inode), bound to the launch by the SHA-256 of the picture and sidecar, the decoded size, and the
// revision and build, and given the application's own count nearest its stated time and the substitutions since
// the previous capture (readCaptures). The runner takes no capture and alters none.
import { createHash, randomBytes } from "node:crypto";
import { execFileSync, spawnSync } from "node:child_process";
import * as fs from "node:fs";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync, writeSync } from "node:fs";
import os from "node:os";
import zlib from "node:zlib";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { emptyOutput, outputRefusal } from "../run.mjs";
import { readReport } from "../../native-smoke/launch-outcome.mjs";
import { ATTENDANT_STATEMENT, marksDecision } from "../../native-smoke/judged-launch.mjs";
import { ATTENDED_SURFACE, composeIndex } from "../compose-index.mjs";
import { readScreenCounts, scrollLongestRegion } from "../screen-counts.mjs";
import { takeSnapshot } from "./snapshot.mjs";
import { SURFACE_KEYS, readStepRecords, renderScreens, screensFromRecords, writePart } from "./render.mjs";

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
  },
  "refused-stored-run": {
    number: 25, title: "Reopen a saved project whose stored run this version does not read (launched app): the refusal, the Results stage and a save in place",
    use_case: "DEL-14-02 AX-003, refuse by name over reading leniently: a saved project whose run's record binds its result rules at a revision this version does not read, opened from the local store; the run is refused and the model opens",
    place: [{ key: "store", at: "{app-data}/openpipestress-projects.sqlite3", from: "fixtures/persistence/refused_stored_run/openpipestress-projects.sql", build: "sqlite3-from-sql" }],
    open: { by: "local-project", project_id: "project:precision-connected-fixture" },
    // Copies of the store the driver has the app write at these moments (driver.js storeCopy), read after the launch
    // for whether the stored run's texts are still the placed ones (storedRunEvidence).
    store_copies: ["after-open", "after-save"]
  }
});
export const DEFAULT_JOURNEYS = Object.freeze(["solve-and-results", "report", "exports"]);
// Named alone: its solve is long, as evidence-journeys.mjs keeps it apart.
export const SEPARATE_JOURNEYS = Object.freeze(["reference-system"]);

/** The command line: { buildOnly, journeys, out, showWindow } or { error }. --build-only takes nothing else. */
export function parseArguments(argv, { defaultOut = path.join(walkthrough, "output") } = {}) {
  const options = { buildOnly: false, journeys: null, out: null, showWindow: false };
  for (let index = 0; index < argv.length; index += 1) {
    const name = argv[index];
    if (name === "--build-only") { options.buildOnly = true; continue; }
    if (name === "--show-window") {
      if (options.showWindow) return { error: `${name} is given twice` };
      options.showWindow = true;
      continue;
    }
    if (name !== "--journey" && name !== "--out") return { error: `unknown argument ${name}` };
    const value = argv[index + 1];
    if (value === undefined || value.startsWith("--")) return { error: `${name} needs a value` };
    if (options[name.slice(2) === "journey" ? "journeys" : "out"] !== null) return { error: `${name} is given twice` };
    if (name === "--journey") options.journeys = value.split(",").map((item) => item.trim()).filter(Boolean);
    else options.out = value;
    index += 1;
  }
  if (options.buildOnly) {
    if (options.journeys !== null || options.out !== null || options.showWindow) return { error: "--build-only builds and binds the app and takes no journey, output folder or --show-window" };
    return { buildOnly: true, journeys: [], out: null, showWindow: false };
  }
  const journeys = options.journeys ?? [...DEFAULT_JOURNEYS];
  const unknown = journeys.filter((item) => !Object.hasOwn(JOURNEYS, item));
  if (journeys.length === 0 || unknown.length > 0) return { error: `choose --journey with one or more of ${DEFAULT_JOURNEYS.join(", ")} (or ${SEPARATE_JOURNEYS.join(", ")} alone)${unknown.length ? `; unknown: ${unknown.join(", ")}` : ""}` };
  if (new Set(journeys).size !== journeys.length) return { error: "a journey is named twice" };
  if (journeys.some((item) => SEPARATE_JOURNEYS.includes(item)) && journeys.length > 1) return { error: `${SEPARATE_JOURNEYS.join(", ")} runs alone: name it as the only journey` };
  return { buildOnly: false, journeys, out: path.resolve(options.out ?? defaultOut), showWindow: options.showWindow };
}

/** The app's own data folder inside a home, where a placed local project store goes. */
export const APP_DATA = "{app-data}";

/**
 * Where each declared file goes: [{ key, from, to }], absolute paths, `to` strictly inside `home`. Throws, naming the
 * entry, when `at` is absolute, climbs (`..`) or is empty; when `from` is not a committed path under `fixtures/`; when
 * the source is not a regular file (a link is refused); or when two entries share a key or a destination. An entry
 * with `build: "sqlite3-from-sql"` is SQL text that placeFiles runs through sqlite3 into a store at `at`; it is also
 * refused when sqlite3 is not there to run.
 */
export function placementsFor(place = [], { home, identifier, projectRoot, lstat = fs.lstatSync, sqlite3 = SQLITE3, access = fs.accessSync }) {
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
    if (entry.build !== undefined) {
      if (entry.build !== SQLITE3_FROM_SQL) throw new Error(`${name}: build must be "${SQLITE3_FROM_SQL}"`);
      try { access(sqlite3, fs.constants.X_OK); } catch { throw new Error(`${name}: ${SQLITE3_FROM_SQL} needs ${sqlite3}, which is not there to run`); }
    }
    const to = path.join(home, relative);
    if (seen.has(entry.key) || seen.has(to)) throw new Error(`${name}: a key or destination is declared twice`);
    seen.add(entry.key);
    seen.add(to);
    return { key: entry.key, from, to, ...(entry.build ? { build: entry.build } : {}) };
  });
}

/** sqlite3 by its absolute path on this platform, never found on PATH. */
export const SQLITE3 = "/usr/bin/sqlite3";
export const SQLITE3_FROM_SQL = "sqlite3-from-sql";
/** Its fixed arguments. `-safe`: the shell obeys dot-commands on standard input, and in safe mode it refuses the ones
 * that reach the host (.shell, .system, .open of another file), so a line of a committed SQL text cannot run a program. */
export const SQLITE3_ARGS = Object.freeze(["-bail", "-batch", "-safe", "built.sqlite3"]);

/**
 * Copies each placement into the home (never over an existing file) and returns { key: path }. A `sqlite3-from-sql`
 * placement is built instead: sqlite3, by its absolute path, with a fixed argument list (nothing of the declaration
 * in it), no shell, an environment of its own and the SQL text on standard input, writes a store in a new folder
 * beside the destination (inside the home), which is then
 * moved into place; a failing sqlite3, or a journal file left beside the store, refuses by name and places nothing.
 * Each build is recorded in `built`: the SQL text's SHA-256 and the built file's.
 */
export function placeFiles(placements, { sqlite3 = SQLITE3, run = execFileSync, built = [] } = {}) {
  const placed = {};
  for (const { key, from, to, build } of placements) {
    fs.mkdirSync(path.dirname(to), { recursive: true });
    if (build === SQLITE3_FROM_SQL) {
      if (fs.existsSync(to)) throw new Error(`place (${key}): ${to} exists`);
      const sql = fs.readFileSync(from);
      const work = fs.mkdtempSync(path.join(path.dirname(to), ".placement-"));
      try {
        try {
          // An environment of its own (HOME the build folder, so no ~/.sqliterc of the real user is read).
          run(sqlite3, SQLITE3_ARGS, { cwd: work, env: { HOME: work }, input: sql, stdio: ["pipe", "ignore", "pipe"], timeout: 120_000 });
        } catch (error) {
          throw new Error(`place (${key}): ${SQLITE3_FROM_SQL}: ${sqlite3} failed (${String(error.stderr ?? error.message).trim().split("\n")[0].slice(0, 300)})`);
        }
        // This platform's sqlite3 keeps its WAL files after a clean close: an empty WAL holds nothing and goes with
        // its index; a WAL with content, or any other file, refuses.
        const wal = path.join(work, "built.sqlite3-wal");
        if (fs.existsSync(wal) && fs.statSync(wal).size === 0) {
          fs.rmSync(wal);
          fs.rmSync(path.join(work, "built.sqlite3-shm"), { force: true });
        }
        const left = fs.readdirSync(work).filter((name) => name !== "built.sqlite3");
        if (left.length > 0 || !fs.existsSync(path.join(work, "built.sqlite3"))) throw new Error(`place (${key}): ${SQLITE3_FROM_SQL}: sqlite3 left ${left.length ? left.join(", ") : "no store"}`);
        fs.renameSync(path.join(work, "built.sqlite3"), to);
      } finally {
        fs.rmSync(work, { recursive: true, force: true });
      }
      const sha = (bytes) => createHash("sha256").update(bytes).digest("hex");
      const bytes = fs.readFileSync(to);
      built.push({ key, from: path.basename(from), build, sql_sha256: sha(sql), sha256: sha(bytes), bytes: bytes.length });
    } else {
      fs.copyFileSync(from, to, fs.constants.COPYFILE_EXCL);
    }
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

const STORED_RUN_TEXTS = "SELECT project_id, mechanics_result_json AS result, analysis_run_json AS record FROM local_projects ORDER BY project_id";
const STORE_AT = `${APP_DATA}/openpipestress-projects.sqlite3`;

/**
 * A journey's `store_copies`: names (a-z, 0-9 and -, up to 26 characters) of the moments its driver has the app copy
 * its own store (driver.js storeCopy). A name is not a path: the copy is written by the app, through a read-only
 * connection, beside the launch's report in the run's folder. Refused by name when an entry is not such a name, is
 * repeated, or the journey places no store at {app-data}/openpipestress-projects.sqlite3. None declared is [].
 */
export function storeCopiesFor(spec) {
  if (spec.store_copies === undefined) return [];
  if (!Array.isArray(spec.store_copies)) throw new Error("store_copies must be a list of names");
  spec.store_copies.forEach((name, index) => {
    if (typeof name !== "string" || !/^[a-z0-9][a-z0-9-]{0,25}$/.test(name)) throw new Error(`store_copies[${index}] (${String(name)}): a store copy is named by a-z, 0-9 and -, up to 26 characters; it is not a path`);
  });
  if (new Set(spec.store_copies).size !== spec.store_copies.length) throw new Error("store_copies names a copy twice");
  if (!(spec.place ?? []).some((entry) => entry?.at === STORE_AT)) throw new Error(`store_copies needs a store placed at ${STORE_AT}`);
  return [...spec.store_copies];
}

/** Each project row's stored result and analysis-run record texts in the store `file`, by SHA-256. */
export async function readStoredRuns(file) {
  const { DatabaseSync } = await import("node:sqlite");
  const digest = (text) => (text === null || text === undefined ? null : createHash("sha256").update(text, "utf8").digest("hex"));
  const db = new DatabaseSync(file, { readOnly: true });
  try {
    return db.prepare(STORED_RUN_TEXTS).all().map((row) => ({ project_id: row.project_id, result_sha256: digest(row.result), record_sha256: digest(row.record) }));
  } finally {
    db.close();
  }
}

/**
 * Whether the placed store still holds the same stored runs at each of its `store_copies`: `placed` is read before the
 * launch from a copy the runner made of the placed store in the run's own folder, beside the home; each copy the app wrote at a named moment
 * is `<report>.store-<name>.store.sqlite3`. A copy the app did not write is recorded as missing, and then nothing is
 * claimed unchanged.
 */
export async function storedRunEvidence({ names, report, placed }) {
  const at = {};
  for (const name of names) {
    const file = `${report}.store-${name}.store.sqlite3`;
    at[name] = fs.existsSync(file) ? await readStoredRuns(file) : { missing: "the app wrote no copy of the store at this moment (the journey stopped before it)" };
  }
  const same = (rows) => Array.isArray(rows) && JSON.stringify(rows) === JSON.stringify(placed);
  return {
    what: "each stored run's result and analysis-run record texts, by SHA-256: as placed, and in the copies of the store the app wrote at these moments through a read-only connection",
    placed,
    at,
    unchanged: names.length > 0 && names.every((name) => same(at[name]))
  };
}

/**
 * The launch's reading of the menu the app built, from the driver's `menu` checkpoint (`readReport`'s { report,
 * problem }): { read: true, journey, number, read_when, menus } or { read: false, journey, number, error }.
 */
export function nativeMenuRecord({ report, problem } = {}, { journey, number }) {
  if (!report) return { read: false, journey, number, error: `the driver recorded no reading of the menu (${problem ?? "no checkpoint"})` };
  if (!Array.isArray(report.menus)) return { read: false, journey, number, error: String(report.error ?? "the reading holds no menus") };
  return { read: true, journey, number, read_when: report.read_when ?? null, source: "native_smoke_menu_items: the application menu as the app built and installed it, read item by item through the menu API", menus: report.menus };
}

/**
 * Whether this launch's window was shown for the run: the option, what the window and its page then reported, and
 * `screenLocked` (screenLockState, read before and after the launch), which says why a shown window's page can stay
 * hidden: a locked screen shows no window of the session.
 */
export function windowShownRecord(option, record, screenLocked = null) {
  if (!option) return { option: false, shown: false, why: "--show-window was not given: the window is left where the system put it" };
  const locked = screenLocked ? { screen_locked: screenLocked } : {};
  if (!record) return { option: true, shown: false, ...locked, why: "the driver recorded no outcome of showing the window" };
  if (record.error) return { option: true, shown: false, ...locked, why: `showing the window failed: ${record.error}` };
  const visible = record.page_visibility === "visible";
  const lockedNow = screenLocked && (screenLocked.before === true || screenLocked.after === true);
  const why = `the page still reported itself ${record.page_visibility ?? "of unknown visibility"} after the window was shown${lockedNow ? ": the host's screen was locked, and a locked screen shows no window of the session" : ""}`;
  return { option: true, shown: Boolean(record.window?.visible) && visible, window: record.window ?? null, page_visibility: record.page_visibility ?? null, focus: record.focus ?? null, waited_ms: record.waited_ms ?? null, ...locked, ...(visible ? {} : { why }) };
}

/**
 * Whether the session's screen is locked, from the console session's own record (`ioreg`, the IORegistry root's
 * IOConsoleUsers: CGSSessionScreenIsLocked); true, false, or null when it cannot be read. Reads no window and needs
 * no permission. `text` is ioreg's output (a seam for the test).
 */
export function screenLockState(text) {
  if (typeof text !== "string") return null;
  const match = text.match(/<key>CGSSessionScreenIsLocked<\/key>\s*<(true|false)\/>/);
  return match ? match[1] === "true" : (text.includes("<key>IOConsoleUsers</key>") ? false : null);
}

/** Removes the earlier output of the journeys this run replaces (their numbers), keeping every other journey's. */
export function emptyNativeOutput(out, journeys) {
  const numbers = new Set(journeys.map((item) => JOURNEYS[item].number));
  emptyOutput(out, (journeyNumber) => numbers.has(journeyNumber));
}

// ---------------------------------------------------------------- the attended launch (--attended)

/**
 * The attended launch: one launch of the bound scripted-run build for an attendant who drives the actual window. Its
 * part in the index is number 90 (`part-90-attended.json`), after every journey's number; its screens are the
 * attendant's captures in <out>/captures/, numbered 9001 on.
 */
export const ATTENDED = Object.freeze({
  name: "attended",
  number: 90,
  captures: "captures",
  finish_file: "attended-finish",
  // The limit ends the launch through the watcher's abort; runUnderLock's own time limit, this much later, is the backstop.
  limit: Object.freeze({ default_minutes: 20, min_minutes: 1, max_minutes: 60, backstop_ms: 2 * 60 * 1000 }),
  // The window's title as the app sets it (lib.rs native_window_title): "SWBPIPE", "<project> — SWBPIPE" or
  // "<project> · Edited — SWBPIPE".
  window_title_ends_with: "SWBPIPE",
  process_name: "openpipestress-desktop",
  chooser: "the strict chooser ends the application if a file chooser is opened: no answer is queued for the attendant"
});
export { ATTENDED_SURFACE };
export const CAPTURE_BASIS = "This picture is a capture of the actual window of the launched app, put in captures/ by the attendant who drove the window through its own controls; the runner neither took it nor altered it. It is not redrawn and not scripted, and it is evidence only of what it shows.";
export const UNEXPLAINED_BASIS = "This file was found in captures/ without a sidecar that says what it shows and how it was made: it is unexplained, and nothing is claimed for it.";

/**
 * The refusal line for the marks of an agent operating the App on the screen (native-smoke/marks-check.mjs
 * marksCheck), naming each mark found and each check that could not be made, as the run's record does. An attended
 * launch is started before the attendant's screen tool is up: that tool's service is one of the marks.
 */
export function markRefusalLine(marks, { attendant = null } = {}) {
  const found = markNames(marks);
  const head = `NATIVE-SMOKE-MARK-PRESENT: ${found.length ? found.join("; ") : "a mark"}: an agent may be operating the App on the screen; not launched, and nothing in --out was changed.`;
  if (attendant) return `${head} The statement that the agent on the screen is this launch's attendant (--attendant-on-screen) covers the screen service and the app-server only, never a check that could not be made or another application's candidate bundle`;
  return `${head} Start the attended launch before the attendant's screen tool is started, and start that tool only after the handover is printed; or, when the agent on the screen is this launch's attendant, say so with --attendant-on-screen <who states it>`;
}
/** Each mark marksCheck found, and each check it could not make, in words. */
export function markNames(marks) {
  return [
    marks?.sky_computer_use_service_launchd ? "SkyComputerUseService under launchd" : null,
    marks?.sky_computer_use_service_process ? "a SkyComputerUseService process" : null,
    marks?.codex_app_server_stdio ? "a \"codex app-server --listen stdio://\" process" : null,
    marks?.app_v4_candidate_bundle ? "a running App v4 candidate bundle" : null,
    ...(marks?.check_failures ?? []).map((failure) => `a check that could not be made (${failure})`)
  ].filter(Boolean);
}

/**
 * The command line with --attended: { buildOnly: false, journeys: [], out, showWindow, attended: { start, limitMinutes,
 * hidden, attendantOnScreen } } or { error }. --out is required; --journey names at most one journey, and it must
 * declare `place`. --attendant-on-screen <who> is the caller's statement, by who makes it, that the agent operating
 * the screen is this launch's attendant (judged-launch.mjs marksDecision); null when not given.
 */
export function parseAttendedArguments(argv) {
  const options = { attended: 0, journey: null, out: null, limit: null, hidden: 0, attendant: null };
  for (let index = 0; index < argv.length; index += 1) {
    const name = argv[index];
    if (name === "--attended" || name === "--hidden") {
      options[name.slice(2)] += 1;
      if (options[name.slice(2)] > 1) return { error: `${name} is given twice` };
      continue;
    }
    if (name === "--build-only") return { error: "--attended launches the bound app and cannot be given with --build-only: build first, as its own job" };
    if (name === "--show-window") return { error: "--attended brings the window to the front unless --hidden; --show-window is not given with it" };
    const key = { "--journey": "journey", "--out": "out", "--limit-minutes": "limit", "--attendant-on-screen": "attendant" }[name];
    if (!key) return { error: `unknown argument ${name}` };
    const value = argv[index + 1];
    if (value === undefined || value.startsWith("--")) return { error: `${name} needs a value` };
    if (options[key] !== null) return { error: `${name} is given twice` };
    options[key] = value;
    index += 1;
  }
  if (options.out === null) return { error: "--attended needs --out <folder>: the record and the captures folder go there" };
  if (options.attendant !== null && (options.attendant.trim() === "" || options.attendant.length > 200 || /[\u0000-\u001f\u007f]/.test(options.attendant))) {
    return { error: "--attendant-on-screen names who states that the agent on the screen is this launch's attendant: 1 to 200 characters on one line" };
  }
  let start = null;
  if (options.journey !== null) {
    const names = options.journey.split(",").map((item) => item.trim()).filter(Boolean);
    if (names.length !== 1) return { error: "--attended starts from at most one journey: name one with --journey" };
    [start] = names;
    if (!Object.hasOwn(JOURNEYS, start)) return { error: `unknown journey ${start}` };
    if (!Array.isArray(JOURNEYS[start].place) || JOURNEYS[start].place.length === 0) {
      return { error: `--attended --journey ${start}: that journey places nothing before launch, so it has no start to give an attended launch; omit --journey for the application as first opened` };
    }
  }
  let limitMinutes = ATTENDED.limit.default_minutes;
  if (options.limit !== null) {
    limitMinutes = /^\d+$/.test(options.limit) ? Number(options.limit) : NaN;
    if (!Number.isInteger(limitMinutes) || limitMinutes < ATTENDED.limit.min_minutes || limitMinutes > ATTENDED.limit.max_minutes) {
      return { error: `--limit-minutes is a whole number of minutes from ${ATTENDED.limit.min_minutes} to ${ATTENDED.limit.max_minutes}` };
    }
  }
  return { buildOnly: false, journeys: [], out: path.resolve(options.out), showWindow: options.hidden === 0, attended: { start, limitMinutes, hidden: options.hidden === 1, attendantOnScreen: options.attendant === null ? null : options.attendant.trim() } };
}

/** The whole command line: the attended form when --attended is given, otherwise parseArguments. */
export function parseCommandLine(argv, options) {
  if (argv.includes("--attended")) return parseAttendedArguments(argv);
  const attendedOnly = argv.find((item) => item === "--hidden" || item === "--limit-minutes" || item === "--attendant-on-screen");
  if (attendedOnly) return { error: `${attendedOnly} is an option of --attended` };
  return { ...parseArguments(argv, options), attended: null };
}

/**
 * The refusals of a launch's output folder, read-only and made before any cleanup: what the folder holds that a
 * walkthrough did not write, and, for an attended launch, an earlier attended launch's captures folder. Null when none.
 */
export function outputRefusals(out, { attended = false } = {}) {
  const refusal = outputRefusal(out);
  if (refusal) return refusal;
  if (attended && lstatOrNull(path.join(out, ATTENDED.captures)) !== null) {
    return `${path.join(out, ATTENDED.captures)} holds an earlier attended launch's captures; choose another --out (nothing in it was changed)`;
  }
  return null;
}

/** After outputRefusals found none: an earlier attended part goes (its captures folder would have refused the run). */
export function emptyAttendedOutput(out) {
  const refusal = outputRefusals(out, { attended: true });
  if (refusal) throw new Error(refusal);
  emptyOutput(out, (journeyNumber) => journeyNumber === ATTENDED.number);
}
const lstatOrNull = (file) => { try { return fs.lstatSync(file); } catch { return null; } };

/**
 * An admitted attended launch's output folder: what it replaces goes (the index and an earlier part 90, after the
 * refusals of emptyAttendedOutput) and its captures folder is made. Returns that folder's identity (device and inode).
 * Called only from the launch's beforeStart, which judgedPhase calls only once its marks check has admitted the start.
 */
export function prepareAttendedOutput(out) {
  emptyNativeOutput(out, []);
  emptyAttendedOutput(out);
  const captures = path.join(out, ATTENDED.captures);
  mkdirSync(captures);
  return (({ dev, ino }) => ({ dev, ino }))(lstatSync(captures));
}

/**
 * The handover: the lines the command prints once the driver's start checkpoint says the start was made (ok: true,
 * with this launch's nonce) and the census has found the process running this launch's private copy,
 * `attended.<key>=<value>`, the last `attended.ready=yes`. What it says of the window is what the driver observed at
 * the start (`observed`, the checkpoint's window_shown and page_visibility), never the option asked for.
 */
export function attendedStartLines({ pid, boundApp, launchCopy, finishFile, captures, limitMinutes, endsAt, observed, start, attendantOnScreen = null }) {
  return [
    `attended.pid=${pid}`,
    `attended.process_name=${ATTENDED.process_name}`,
    `attended.bound_app=${boundApp}`,
    `attended.launch_copy=${launchCopy}`,
    `attended.window_title_ends_with=${ATTENDED.window_title_ends_with}`,
    `attended.window_shown=${observed?.window_shown === true ? "yes" : "no"}`,
    `attended.page_visibility=${observed?.page_visibility ?? "not reported"}`,
    `attended.start=${start ?? "the application as first opened"}`,
    `attended.finish_file=${finishFile}`,
    `attended.captures=${captures}`,
    `attended.limit_minutes=${limitMinutes}`,
    `attended.ends_at=${endsAt}`,
    `attended.chooser=${ATTENDED.chooser}`,
    `attended.attendant_on_screen=${attendantOnScreen ? `stated by ${attendantOnScreen.stated_by}: ${ATTENDANT_STATEMENT}; marks present directly before the start: ${attendantOnScreen.marks.length ? attendantOnScreen.marks.join("; ") : "none"}` : "not stated"}`,
    "attended.ready=yes"
  ];
}

/** Every process's parent and group: Map pid -> { ppid, pgid }, from ps; null when ps cannot be read. */
export function readProcessTable(run = spawnSync) {
  const result = run("/bin/ps", ["-A", "-o", "pid=,ppid=,pgid="], { encoding: "utf8", timeout: 15000, maxBuffer: 16 * 1024 * 1024 });
  if (result.error || result.status !== 0) return null;
  const table = new Map();
  for (const line of String(result.stdout).split("\n")) {
    const fields = line.trim().split(/\s+/).map(Number);
    if (fields.length === 3 && fields.every(Number.isInteger)) table.set(fields[0], { ppid: fields[1], pgid: fields[2] });
  }
  return table;
}

/**
 * Of the census's pids (each runs the run's private copy), the app's: the one whose parent is this runner, or whose
 * parent is this runner's child (the wrapper); otherwise the lowest.
 */
export function appPidOf(pids, table, self = process.pid) {
  if (!Array.isArray(pids) || pids.length === 0) return null;
  const ours = pids.find((pid) => { const row = table?.get(pid); return row && (row.ppid === self || table.get(row.ppid)?.ppid === self); });
  return ours ?? Math.min(...pids);
}

/**
 * The attended launch's watcher. It owns the AbortController whose signal judgedPhase hands to runUnderLock, the
 * owner of the started child; no process or group is ever signalled here. start({ nonce }) is called directly before
 * the start. From then on, every `pollMs`, it looks for the driver's `start` checkpoint carrying `nonce`, the finish
 * file and, until the handover, the processes running the run's private copy (the census, read afresh each time). The
 * handover, `onReady`, is called once, only while the application is running: the checkpoint says the start was made
 * (ok: true), the census of that same poll still finds the pid first found, and the application has not been seen to
 * exit (once the private copy that ran is no longer running, nothing is ever handed over, so a start checkpoint read
 * late, while the run censuses an application that has already quit, prints nothing). With `requireVisible` (a launch
 * without --hidden) the checkpoint must also say the window was shown and its page visible; otherwise the attendant is
 * not directed to the window and the watcher aborts the launch ("window-not-shown"). A checkpoint that says the start
 * failed is never handed over, and the watcher aborts the launch ("start-failed") in case the driver's own ending does
 * not come. When the finish file appears, or `limitMs` passes, it aborts once: runUnderLock then sends
 * SIGKILL to the child it started if that child
 * is still running, and its receipt (the phase's `abort`) says what was sent. The first poll comes after the start,
 * when the child already exists, so a finish file that is there before the pid is known is acted on, not lost; one
 * that appears after the application has exited signals nothing. stop() returns { pid, start, ready_printed,
 * handed_over_at, exited_at, stop }, where `stop` is null or { by: "finish-file" | "limit" | "start-failed" |
 * "window-not-shown" | "output-not-prepared", at }; halt(by) aborts the launch from outside, once.
 */
export function attendedWatch({ executable, finishFile, startReport, census, limitMs, onReady = () => {}, pollMs = 250, processTable = readProcessTable, self = process.pid, requireVisible = false }) {
  const controller = new AbortController();
  const seen = { pid: null, start: null, printed: false, handedOverAt: null, stop: null, exitedAt: null };
  let nonce = null;
  let poll = null;
  let limit = null;
  const halt = (by, detail = {}) => {
    if (seen.stop !== null) return;
    seen.stop = { by, at: Date.now(), ...detail };
    controller.abort();
  };
  const tick = () => {
    if (seen.stop === null && lstatOrNull(finishFile) !== null) halt("finish-file");
    // Until the handover, the processes running the private copy now: the pid first found, and whether it still runs.
    let running = null;
    if (!seen.printed && seen.exitedAt === null) {
      const result = census(executable);
      if (result?.ok) {
        running = result.pids;
        if (seen.pid === null && running.length > 0) seen.pid = appPidOf(running, processTable(), self);
        else if (seen.pid !== null && !running.includes(seen.pid)) seen.exitedAt = Date.now();
      }
    }
    if (seen.start === null) {
      const read = readReport(startReport).report;
      if (read && read.nonce === nonce) seen.start = read;
    }
    if (seen.start !== null && seen.start.ok !== true) halt("start-failed");
    const shown = seen.start?.window_shown === true && seen.start?.page_visibility === "visible";
    if (requireVisible && seen.start?.ok === true && !shown) halt("window-not-shown");
    if (seen.start?.ok === true && seen.stop === null && !seen.printed && seen.exitedAt === null && seen.pid !== null && running?.includes(seen.pid) && (!requireVisible || shown)) {
      seen.printed = true;
      seen.handedOverAt = Date.now();
      onReady({ pid: seen.pid, start: seen.start });
    }
  };
  return {
    signal: controller.signal,
    start({ nonce: given }) {
      nonce = given;
      poll = setInterval(() => { try { tick(); } catch { /* the next poll tries again */ } }, pollMs);
      limit = setTimeout(() => halt("limit"), limitMs);
    },
    halt,
    stop() {
      clearInterval(poll);
      clearTimeout(limit);
      return { pid: seen.pid, start: seen.start, ready_printed: seen.printed, handed_over_at: seen.handedOverAt, exited_at: seen.exitedAt, stop: seen.stop };
    }
  };
}

/**
 * How an attended launch ended, from its recorded outcome (`recordedOutcome`), runUnderLock's abort receipt (the
 * phase's `abort`), the watcher's stop, the driver's `start` checkpoint and whether the handover was printed
 * (`handedOver`): { by, ok, what } with `by` one of start-failed, finish-file, limit, strict-chooser, app-exited. A
 * failed start is its own ending, whatever came after. The finish file and the limit are the ending only when the
 * abort's SIGKILL was delivered to the running child and the child ended on it. An exit is the attendant's (Quit,
 * the window closed) only after the handover; before it, it is a failure of the launch.
 */
export function attendedEnding({ outcome, receipt = null, stop = null, start = null, nonce, limitMinutes, handedOver = false, strictChooserExit = 86 }) {
  if (!outcome) return { by: "unknown", ok: false, what: "no outcome of the launch was recorded" };
  const killed = receipt?.delivered === true && outcome.signal === "SIGKILL";
  const thisStart = start !== null && start.nonce === nonce;
  let ending;
  if (thisStart && start.ok !== true) {
    ending = { by: "start-failed", ok: false, what: `the start failed and the launch was ended without a handover: ${String(start.error ?? "no reason recorded").split("\n")[0]}` };
  } else if (stop?.by === "window-not-shown") {
    ending = { by: "window-not-shown", ok: false, what: `the window was not observed shown with its page visible (window shown: ${start?.window_shown === true ? "yes" : "no"}, page ${start?.page_visibility ?? "not reported"}), so the attendant was not directed to it and the launch was ended without a handover${killed ? " (SIGKILL delivered to its child)" : ""}` };
  } else if (stop?.by === "output-not-prepared") {
    ending = { by: "output-not-prepared", ok: false, what: `the output folder could not be prepared once the launch was admitted (${stop.why ?? "no reason recorded"}), so the launch was ended at once without a handover${killed ? " (SIGKILL delivered to its child)" : ""}` };
  } else if (killed && stop?.by === "finish-file") {
    ending = { by: "finish-file", ok: true, what: "the finish file appeared; runUnderLock, which started the application, ended it (SIGKILL delivered to its child)" };
  } else if (killed && stop?.by === "limit") {
    ending = { by: "limit", ok: true, what: `the limit of ${limitMinutes} minute(s) passed; runUnderLock, which started the application, ended it (SIGKILL delivered to its child)` };
  } else if (outcome.timed_out) {
    ending = { by: "limit", ok: true, what: `runUnderLock's own time limit, the backstop after ${limitMinutes} minute(s), ended the application (SIGKILL)` };
  } else if (outcome.status === strictChooserExit) {
    ending = { by: "strict-chooser", ok: true, what: `a file chooser was opened with no answer queued; the strict chooser ended the app before any panel (exit ${strictChooserExit})` };
  } else if (outcome.status === 0 && outcome.signal === null && handedOver) {
    ending = { by: "app-exited", ok: true, what: "the application exited by itself (status 0) after the handover, as the attendant's Quit or closing its window ends it" };
  } else if (outcome.status === 0 && outcome.signal === null) {
    ending = { by: "app-exited", ok: false, what: "the application exited by itself (status 0) before the handover" };
  } else {
    ending = { by: "app-exited", ok: false, what: `the application ended by itself with ${outcome.signal ? `signal ${outcome.signal}` : `status ${outcome.status}`}` };
  }
  if (stop && !killed) ending.what += `; the ${stop.by === "limit" ? "limit passed" : "finish file appeared"}, and nothing was signalled (${receipt?.error ?? "the application had already exited"})`;
  if (stop?.by === "start-failed" && ending.by !== "start-failed") ending.what += "; the watcher had ended the launch because the start failed";
  if (!thisStart && ending.by !== "start-failed") return { ...ending, ok: false, what: `${ending.what}; the start was not reported made by this launch` };
  if (!handedOver && ending.ok) return { ...ending, ok: false, what: `${ending.what}; there was no handover to an attendant` };
  return ending;
}

const SIDECAR_LIMIT = 64 * 1024;
/**
 * An account of a surface not shown that does not say what in the system or the application kept it from view: one
 * naming the attendant's own capture tool, its setup or its permission, or one that leaves the reason unresolved.
 */
export const UNACCOUNTED = /\b(tools?|tooling|set ?up|setup|not configured|unconfigured|(capture|recording|screen[- ]?(capture|recording)|accessibility) permission|unknown|unclear|unresolved|undetermined|not sure|no idea|could not (tell|determine|say)|for some reason)\b/i;
/** DEL-07-06 AC-007's use cases by their names there: an attended capture's sidecar names one of them, or the launch's journey. */
export const AC007_USE_CASES = Object.freeze(["Blank to released report", "Timed entry", "Missing data", "Failed solve", "Change a support and compare", "Review and export results", "Every required analysis", "User rule pack", "Save, close and resume"]);
/** How far a sidecar's taken_at may lie from its picture's file time before the index says the two disagree. */
export const TAKEN_AT_TOLERANCE_MS = 5000;
/** The driver's settled-page records (driver.js OBSERVED), in words. */
export const OBSERVED_BOUNDS = "a record when the page has been quiet for 0.8 s after a change (a mutation, a scroll or a resize), at least 1.5 s after the previous record and at most 5 s after the first change it covers, and one at the handover; at most 1200 records, each at most 256 KiB, its controls cut to fit";
const ISO_TIME = /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}(:\d{2}(\.\d{1,9})?)?(Z|[+-]\d{2}:\d{2})$/;
const PNG_SIGNATURE = Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]);

/**
 * A sidecar's optional surface claims, the attendant's: `surfaces`, the native surfaces the picture shows, and
 * `prevented`, one statement or a list of { step, surface, observed, obstacle, cause } that a surface could not be
 * shown: the step attempted, the surface, what was observed when it was attempted, whether the obstacle is the
 * "system" or the "application", and the cause, a concrete account of what in that system or application kept the
 * surface from the capture. Each surface is a key of SURFACE_KEYS; an unknown one is kept apart as unexplained. A
 * statement lacking any member, or whose account names the attendant's own tool or its setup or leaves the reason
 * unresolved (UNACCOUNTED), is kept apart as not counted, with why, and makes nothing unseen: it is missing evidence.
 * { surfaces, surfaces_unexplained, prevented, prevented_incomplete, prevented_unexplained } or { why } when a member
 * is not of that form.
 */
export function surfaceClaims(value) {
  const claims = { surfaces: [], surfaces_unexplained: [], prevented: [], prevented_incomplete: [], prevented_unexplained: [] };
  if (value.surfaces !== undefined) {
    if (!Array.isArray(value.surfaces) || value.surfaces.length > 20 || !value.surfaces.every((item) => typeof item === "string" && item.length > 0 && item.length <= 80)) return { why: "its sidecar's surfaces is not a list of surface names" };
    for (const item of value.surfaces) (Object.hasOwn(SURFACE_KEYS, item) ? claims.surfaces : claims.surfaces_unexplained).push(item);
  }
  if (value.prevented !== undefined) {
    const records = Array.isArray(value.prevented) ? value.prevented : [value.prevented];
    const valid = (record) => record && typeof record === "object" && !Array.isArray(record) && typeof record.surface === "string" && record.surface.length > 0 && record.surface.length <= 80;
    if (records.length === 0 || records.length > 20 || !records.every(valid)) return { why: "its sidecar's prevented is not { step, surface, observed, obstacle, cause } or a list of them" };
    const text = (item, limit) => (typeof item === "string" && item.trim() !== "" && item.length <= limit ? item.trim() : null);
    for (const record of records) {
      const statement = { step: text(record.step, 200), surface: record.surface, observed: text(record.observed, 1000), obstacle: ["system", "application"].includes(record.obstacle) ? record.obstacle : null, cause: text(record.cause, 1000) };
      const lacks = ["step", "observed", "obstacle", "cause"].filter((member) => statement[member] === null);
      if (!Object.hasOwn(SURFACE_KEYS, record.surface)) claims.prevented_unexplained.push(statement);
      else if (lacks.length > 0) claims.prevented_incomplete.push({ ...statement, lacks, why: `it lacks ${lacks.join(", ")}: a statement names the step attempted, the surface, what was observed, whether the obstacle is the system's or the application's, and what in it kept the surface from the capture` });
      else if (UNACCOUNTED.test(statement.cause) || UNACCOUNTED.test(statement.observed)) claims.prevented_incomplete.push({ ...statement, lacks: [], why: "its account names the attendant's own tool or its setup, or leaves the reason unresolved: that is missing evidence, not a surface the system or the application kept from view" });
      else claims.prevented.push(statement);
    }
  }
  return claims;
}

/**
 * A sidecar's claims, every one the attendant's statement: { use_case, step, acted, note, taken_at, taken_at_ms,
 * ...surfaceClaims } or { why } when it is not one. use_case (one of AC007_USE_CASES by its name, or the launch's
 * journey: checked by readCaptures), step and acted (what was done since the previous capture; "observed only" when
 * nothing was) are required and not empty; note is a string; taken_at is an ISO 8601 time with its zone.
 */
export function readSidecar(file) {
  let text;
  try {
    const stat = fs.lstatSync(file);
    if (!stat.isFile()) return { why: "its sidecar is not a regular file" };
    if (stat.size > SIDECAR_LIMIT) return { why: `its sidecar is larger than ${SIDECAR_LIMIT} bytes` };
    text = fs.readFileSync(file, "utf8");
  } catch (error) {
    return { why: `its sidecar could not be read (${error.code ?? error.message})` };
  }
  let value;
  try { value = JSON.parse(text); } catch { return { why: "its sidecar is not JSON" }; }
  if (!value || typeof value !== "object" || Array.isArray(value)) return { why: "its sidecar is not a JSON object" };
  if (typeof value.use_case !== "string" || value.use_case.trim() === "" || value.use_case.length > 200) return { why: "its sidecar names no use case (DEL-07-06 AC-007's name for it, or the launch's journey)" };
  if (typeof value.step !== "string" || value.step.trim() === "" || value.step.length > 200) return { why: "its sidecar names no step (a string of 1 to 200 characters)" };
  if (!Array.isArray(value.acted) || value.acted.length === 0 || value.acted.length > 200 || !value.acted.every((item) => typeof item === "string" && item.trim() !== "" && item.length <= 1000)) return { why: "its sidecar's acted does not say what was done since the previous capture (a list of at least one non-empty string; \"observed only\" when nothing was done)" };
  if (typeof value.note !== "string" || value.note.length > 4000) return { why: "its sidecar has no note (a string of at most 4000 characters)" };
  const takenAt = typeof value.taken_at === "string" && ISO_TIME.test(value.taken_at) ? Date.parse(value.taken_at) : Number.NaN;
  if (!Number.isFinite(takenAt)) return { why: "its sidecar does not say when the picture was taken (taken_at, an ISO 8601 time with its zone)" };
  const claims = surfaceClaims(value);
  if (claims.why) return claims;
  return { use_case: value.use_case.trim(), step: value.step.trim(), acted: value.acted.map((item) => item.trim()), note: value.note, taken_at: value.taken_at, taken_at_ms: takenAt, ...claims, raw: value };
}

const CRC_TABLE = (() => {
  const table = new Int32Array(256);
  for (let n = 0; n < 256; n += 1) {
    let c = n;
    for (let k = 0; k < 8; k += 1) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
    table[n] = c;
  }
  return table;
})();
const crc32 = (bytes) => {
  let c = -1;
  for (let i = 0; i < bytes.length; i += 1) c = CRC_TABLE[(c ^ bytes[i]) & 0xff] ^ (c >>> 8);
  return (c ^ -1) >>> 0;
};
const PNG_CHANNELS = { 0: 1, 2: 3, 3: 1, 4: 2, 6: 4 };
const PNG_MAX_SIDE = 16384;

/**
 * Whether `bytes` decode as a PNG picture with a nonzero size: the signature, every chunk whole with its checksum,
 * IHDR first with a known colour type, IDAT data that inflates to exactly the rows its size and interlacing need,
 * and IEND. { width, height } or { why }. The pixels are not interpreted further.
 */
export function pngDecode(bytes) {
  if (bytes.length < 8 || !bytes.subarray(0, 8).equals(PNG_SIGNATURE)) return { why: "it is not a PNG file by its signature" };
  let offset = 8;
  let header = null;
  const data = [];
  let ended = false;
  while (offset + 12 <= bytes.length) {
    const length = bytes.readUInt32BE(offset);
    const type = bytes.toString("latin1", offset + 4, offset + 8);
    if (offset + 12 + length > bytes.length) return { why: `its ${type} chunk runs past the end of the file` };
    if (crc32(bytes.subarray(offset + 4, offset + 8 + length)) !== bytes.readUInt32BE(offset + 8 + length)) return { why: `its ${type} chunk's checksum is wrong` };
    const body = bytes.subarray(offset + 8, offset + 8 + length);
    if (header === null && type !== "IHDR") return { why: "its first chunk is not IHDR" };
    if (type === "IHDR") {
      if (header !== null || length !== 13) return { why: "its IHDR chunk is not one of 13 bytes" };
      header = { width: body.readUInt32BE(0), height: body.readUInt32BE(4), depth: body[8], colour: body[9], interlace: body[12] };
    } else if (type === "IDAT") data.push(body);
    else if (type === "IEND") { ended = true; break; }
    offset += 12 + length;
  }
  if (header === null) return { why: "it has no IHDR chunk" };
  const { width, height, depth, colour, interlace } = header;
  if (width === 0 || height === 0) return { why: "its size is zero" };
  if (width > PNG_MAX_SIDE || height > PNG_MAX_SIDE) return { why: `it is larger than ${PNG_MAX_SIDE} pixels on a side` };
  if (!Object.hasOwn(PNG_CHANNELS, colour) || ![1, 2, 4, 8, 16].includes(depth) || interlace > 1) return { why: "its colour type, bit depth or interlacing is not one PNG defines" };
  if (data.length === 0) return { why: "it has no image data" };
  if (!ended) return { why: "it has no IEND chunk" };
  const bits = depth * PNG_CHANNELS[colour];
  const rows = (w, h) => (w === 0 || h === 0 ? 0 : h * (1 + Math.ceil((w * bits) / 8)));
  const passes = [[0, 0, 8, 8], [4, 0, 8, 8], [0, 4, 4, 8], [2, 0, 4, 4], [0, 2, 2, 4], [1, 0, 2, 2], [0, 1, 1, 2]];
  const expected = interlace === 0 ? rows(width, height)
    : passes.reduce((sum, [x, y, dx, dy]) => sum + rows(Math.ceil((width - x) / dx), Math.ceil((height - y) / dy)), 0);
  let inflated;
  try { inflated = zlib.inflateSync(Buffer.concat(data), { maxOutputLength: expected + 1 }); } catch { return { why: "its image data does not inflate to the rows its size needs" }; }
  if (inflated.length !== expected) return { why: "its image data does not inflate to the rows its size needs" };
  return { width, height };
}

const sha256Of = (bytes) => `sha256:${createHash("sha256").update(bytes).digest("hex")}`;
const readBounded = (file, limit) => {
  const stat = fs.lstatSync(file);
  if (!stat.isFile()) throw new Error("not a regular file");
  if (stat.size > limit) throw new Error(`larger than ${limit} bytes`);
  return fs.readFileSync(file);
};
const PICTURE_LIMIT = 64 * 1024 * 1024;

/** The use case a sidecar names, as the index states it: an AC-007 use case by its name there, or the launch's journey's; null otherwise. */
function useCaseOf(named, journeyUseCase) {
  if (AC007_USE_CASES.includes(named)) return `DEL-07-06 AC-007 ${named}, as the attendant states`;
  if (journeyUseCase && (named === journeyUseCase.name || named === journeyUseCase.use_case)) return `${journeyUseCase.use_case}, as the attendant states`;
  return null;
}
/** Why a picture's file time places it outside the launch (before it started, or after it ended), or null. */
function outsideLaunch(fileTime, launchTimes) {
  if (!launchTimes || fileTime === null) return null;
  if (fileTime < launchTimes.from - TAKEN_AT_TOLERANCE_MS) return `its file time (${new Date(fileTime).toISOString()}) is before this launch started (${new Date(launchTimes.from).toISOString()})`;
  if (fileTime > launchTimes.to + TAKEN_AT_TOLERANCE_MS) return `its file time (${new Date(fileTime).toISOString()}) is after this launch ended (${new Date(launchTimes.to).toISOString()})`;
  return null;
}
/** The sidecar's taken_at checked against the picture's own file time, in words. */
function takenAtCheck(takenAt, fileTime) {
  if (fileTime === null) return "the picture's file time could not be read, so the time the attendant states is not checked";
  const apart = Math.abs(fileTime - takenAt);
  return apart <= TAKEN_AT_TOLERANCE_MS
    ? `agrees with the picture's file time (${new Date(fileTime).toISOString()}), within ${TAKEN_AT_TOLERANCE_MS / 1000} s`
    : `disagrees with the picture's file time (${new Date(fileTime).toISOString()}): ${Math.round(apart / 100) / 10} s apart`;
}
/** The application's own report of the words and controls on screen nearest at or before `at` (records sorted by time). */
function observedFor(records, at) {
  let nearest = null;
  for (const record of records) { if (record.at <= at) nearest = record; else break; }
  if (nearest === null) return { counts_source: "no report of the application's own at or before the time the attendant states: the words and controls on screen are not recorded for this capture" };
  const when = new Date(nearest.at).toISOString();
  if (nearest.counts_error) return { counts_source: `the application's own report at ${when} could not count the screen (${nearest.counts_error})`, counts_at: when };
  return {
    words_on_screen: nearest.words_on_screen, words_in_page: nearest.words_in_page, prose_blocks_on_screen: nearest.prose_blocks_on_screen,
    buttons_on_screen: nearest.buttons_on_screen, inputs_on_screen: nearest.inputs_on_screen, longest_scroll_screens: nearest.longest_scroll_screens,
    controls: Array.isArray(nearest.controls) ? nearest.controls : [],
    ...(nearest.controls_cut ? { controls_cut: nearest.controls_cut } : {}),
    page_visibility: nearest.page_visibility ?? null,
    counts_at: when,
    counts_source: `the application's own report at ${when} (record ${nearest.n} of the launch's driver, read as the page settled), not the attendant's`
  };
}

/**
 * The driver's settled-page records of the attended launch (`<report>.observed-<n>.json`, driver.js observeAttended)
 * whose nonce is the launch's: { records (sorted by time), rejected }.
 */
export function readObserved(report, nonce) {
  const dir = path.dirname(report);
  const prefix = `${path.basename(report)}.observed-`;
  let names = [];
  try { names = fs.readdirSync(dir).filter((name) => name.startsWith(prefix) && /^\d+\.json$/.test(name.slice(prefix.length))); } catch { names = []; }
  const records = [];
  let rejected = 0;
  for (const name of names) {
    const record = readReport(path.join(dir, name)).report;
    if (record && nonce && record.nonce === nonce && Number.isFinite(record.at) && Number.isInteger(record.n)) records.push(record);
    else rejected += 1;
  }
  return { records: records.sort((a, b) => a.at - b.at || a.n - b.n), rejected };
}

/** The words a scorer reads for each thing in an attended launch's captures that supplies nothing. */
export function unexplainedOf(captures) {
  return [
    ...(captures.error ? [{ file: `${ATTENDED.captures}/`, why: `the captures folder was not read at the end of the launch: ${captures.error}` }] : []),
    ...captures.screens.filter((screen) => screen.unexplained).map((screen) => ({ file: screen.file, why: `a picture that is unexplained, because ${screen.unexplained}` })),
    ...captures.records_without_picture.map((record) => ({ file: record.file, why: "a sidecar with no picture of the same name" })),
    ...captures.other.map((item) => ({ file: item.file, why: item.why }))
  ];
}

/**
 * The attendant's captures in `folder` (<out>/captures), read once at the end of the launch and never changed:
 * { screens, records_without_picture, other, error? }. The folder must still be the one the runner created for this
 * launch (`identity`, its device and inode; not a link); otherwise, or when it cannot be read, `error` says why and
 * nothing in it is taken. Each `.png` that is a regular file becomes a screen. With a sidecar `<same name>.json` of the
 * form readSidecar takes, naming a use case useCaseOf accepts, and a picture that decodes (pngDecode) and whose file
 * time lies within the launch (`launchTimes`), it is a capture of the actual window, bound to this launch: `binding`
 * holds the SHA-256 of the picture and of the sidecar, the decoded width and height, and the launch's revision and
 * build (`bindTo`). What the attendant states is kept as `stated_by_attendant`; from the driver's records each capture
 * also gets the application's own report nearest at or before its stated time (`observed`; counts_source says whose
 * and when) and the substitutions recorded since the previous capture (`substitutions`). Without a sidecar, with one of another form, or with a picture that does not
 * decode, it is unexplained and says why. A sidecar with no picture is listed in records_without_picture, anything
 * else (a link, a folder, another kind of file) in other; nothing is dropped and nothing is followed.
 */
export function readCaptures(folder, { number = ATTENDED.number, journey = ATTENDED.name, identity = null, bindTo = {}, journeyUseCase = null, observed = [], substitutions = [], launchTimes = null } = {}) {
  const found = { screens: [], records_without_picture: [], other: [] };
  const stat = lstatOrNull(folder);
  if (stat === null) return { ...found, error: "it is not there" };
  if (stat.isSymbolicLink() || !stat.isDirectory()) return { ...found, error: "it is no longer a folder (a link or a file stands in its place)" };
  if (identity && (stat.dev !== identity.dev || stat.ino !== identity.ino)) return { ...found, error: "it is not the folder the runner created for this launch (another stands in its place)" };
  let entries;
  try { entries = fs.readdirSync(folder, { withFileTypes: true }).map((entry) => entry.name).sort(); } catch (error) { return { ...found, error: `it could not be read (${error.code ?? error.message})` }; }
  const regular = (name) => lstatOrNull(path.join(folder, name))?.isFile() === true;
  const pictures = entries.filter((name) => /\.png$/i.test(name) && regular(name));
  const pictureBases = new Set(pictures.map((name) => name.replace(/\.png$/i, "")));
  let count = 0;
  const times = new Map();
  for (const name of pictures) {
    count += 1;
    const base = name.replace(/\.png$/i, "");
    const file = `${ATTENDED.captures}/${name}`;
    const entry = { number: number * 100 + count, journey, file, redrawn: false, scripted: false };
    let bytes = null;
    let decoded;
    try { bytes = readBounded(path.join(folder, name), PICTURE_LIMIT); decoded = pngDecode(bytes); } catch (error) { decoded = { why: `it could not be read (${error.code ?? error.message})` }; }
    const sidecarFile = path.join(folder, `${base}.json`);
    const sidecar = entries.includes(`${base}.json`) ? readSidecar(sidecarFile) : { why: "it has no sidecar of the same name" };
    const useCase = sidecar.why ? null : useCaseOf(sidecar.use_case, journeyUseCase);
    const fileTime = lstatOrNull(path.join(folder, name))?.mtimeMs ?? null;
    const why = decoded.why ?? sidecar.why
      ?? (useCase === null ? `its sidecar's use case "${sidecar.use_case}" is neither one of DEL-07-06 AC-007's by its name there nor the launch's journey${journeyUseCase ? ` (${journeyUseCase.name})` : " (it has none)"}` : null)
      ?? outsideLaunch(fileTime, launchTimes);
    if (why) {
      found.screens.push({ ...entry, step: `unexplained-${base}`.slice(0, 120), note: `${name}: unexplained, because ${why}`, capture: "unexplained", unexplained: why, picture: UNEXPLAINED_BASIS });
      continue;
    }
    let sidecarBytes;
    try { sidecarBytes = readBounded(sidecarFile, SIDECAR_LIMIT); } catch (error) { sidecarBytes = null; }
    if (sidecarBytes === null) {
      found.screens.push({ ...entry, step: `unexplained-${base}`.slice(0, 120), note: `${name}: unexplained, because its sidecar changed while it was read`, capture: "unexplained", unexplained: "its sidecar changed while it was read", picture: UNEXPLAINED_BASIS });
      continue;
    }
    const binding = { picture_sha256: sha256Of(bytes), sidecar_sha256: sha256Of(sidecarBytes), width: decoded.width, height: decoded.height, bytes: bytes.length, revision: bindTo.revision ?? null, build_app_sha256: bindTo.build_app_sha256 ?? null };
    const { surfaces, surfaces_unexplained, prevented, prevented_incomplete, prevented_unexplained } = sidecar;
    const stated = { use_case: sidecar.use_case, step: sidecar.step, acted: sidecar.acted, note: sidecar.note, taken_at: sidecar.taken_at, ...(surfaces.length || surfaces_unexplained.length ? { surfaces: [...surfaces, ...surfaces_unexplained] } : {}), ...(sidecar.raw.prevented !== undefined ? { prevented: sidecar.raw.prevented } : {}) };
    const screen = { ...entry, use_case: useCase, step: sidecar.step, note: sidecar.note, acted: sidecar.acted, taken_at: sidecar.taken_at, taken_at_check: takenAtCheck(sidecar.taken_at_ms, fileTime), stated_by_attendant: stated, capture: "the actual window", driven_by: "an attendant, through the window", picture: CAPTURE_BASIS, binding, surfaces, surfaces_unexplained, prevented, prevented_incomplete, prevented_unexplained };
    times.set(screen, sidecar.taken_at_ms);
    found.screens.push(screen);
  }
  // In the order the attendant states the captures were taken: each gets the application's own report nearest at or
  // before its time, and every substitution recorded after the previous capture's time and up to its own.
  const records = [...observed].filter((record) => Number.isFinite(record?.at)).sort((a, b) => a.at - b.at);
  const marks = [...substitutions].filter((item) => Number.isFinite(item?.at)).sort((a, b) => a.at - b.at);
  let previous = -Infinity;
  for (const screen of [...times.keys()].sort((a, b) => times.get(a) - times.get(b) || a.number - b.number)) {
    const at = times.get(screen);
    Object.assign(screen, observedFor(records, at));
    screen.substitutions = marks.filter((item) => item.at > previous && item.at <= at).map((item) => ({ at: new Date(item.at).toISOString(), kind: item.kind ?? "a substitution", ...(item.control ? { control: item.control } : {}), what: item.what ?? "", ...(item.print_sha256 ? { print_sha256: item.print_sha256 } : {}) }));
    screen.substitutions_since = previous === -Infinity ? "the start of the launch" : `the previous capture, stated as taken at ${new Date(previous).toISOString()}`;
    previous = at;
  }
  for (const name of entries) {
    if (pictures.includes(name)) continue;
    const base = name.replace(/\.json$/i, "");
    if (/\.json$/i.test(name) && regular(name)) {
      if (pictureBases.has(base)) continue;
      found.records_without_picture.push({ file: `${ATTENDED.captures}/${name}`, ...(({ raw, ...claims }) => claims)(readSidecar(path.join(folder, name))) });
      continue;
    }
    const kind = lstatOrNull(path.join(folder, name));
    found.other.push({ file: `${ATTENDED.captures}/${name}`, why: kind?.isSymbolicLink() ? "a link, not followed" : kind?.isDirectory() ? "a folder, not entered" : "neither a PNG picture nor a sidecar" });
  }
  return found;
}

/** The attended launch's part file, in the index's form: `part-90-attended.json`. */
export function writeAttendedPart(out, { title, revision, build, launch, native_menu = null, window_shown = null, captures }) {
  const part = {
    number: ATTENDED.number, name: ATTENDED.name, title,
    use_case: "none of its own: the actual window of the launched app, driven by an attendant; each capture is evidence only of what it shows",
    revision, build, surface: ATTENDED_SURFACE, picture_basis: CAPTURE_BASIS, window: null, attended: true,
    ...(native_menu ? { native_menu } : {}),
    ...(window_shown ? { window_shown } : {}),
    launch,
    captures: { folder: ATTENDED.captures, records_without_picture: captures.records_without_picture, other: captures.other, ...(captures.error ? { error: captures.error } : {}), unexplained: unexplainedOf(captures) },
    screens: captures.screens
  };
  const file = path.join(out, `part-${ATTENDED.number}-${ATTENDED.name}.json`);
  fs.writeFileSync(file, `${JSON.stringify(part, null, 2)}\n`);
  return file;
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
  const { marksCheck } = await import("../../native-smoke/marks-check.mjs");
  const { HOME_MARKER, LEFT_HOME_RULE, NOTHING_ELSE_WATCHED, PROCESS_SCOPE, STRICT_CHOOSER_EXIT, TRIPWIRE_LIMITS, homeRemovalRefusal, readHomeMarker, removalIdentityRefusal, sha256Hex } = await import("../../native-smoke/evidence-checks.mjs");

  const refuseLine = (text) => console.error(`${NAME}: ${text}`);
  if (process.platform !== "darwin") {
    refuseLine("macOS only");
    process.exit(2);
  }
  const parsed = parseCommandLine(process.argv.slice(2));
  if (parsed.error) {
    refuseLine(parsed.error);
    process.exit(2);
  }
  const { buildOnly, journeys: selected, out, showWindow, attended } = parsed;

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
  // ---------------------------------------------------------------- build and bind (as evidence-journeys.mjs), or check only

  let buildLock;
  try {
    buildLock = acquireBuildLock({ purpose: NAME });
  } catch (error) {
    refuseLine(error.message);
    process.exit(2);
  }
  // The output folder, under the lock and before anything is built or launched: every refusal is read-only and made
  // before any cleanup, so a refused run changes nothing in the folder.
  if (!buildOnly) {
    const refusal = outputRefusals(out, { attended: Boolean(attended) });
    if (refusal) {
      refuseLine(refusal);
      await exitAfterSignals(2);
    }
    // An attended launch's marks, read once here, before any cleanup, so that a refusal for a mark changes nothing;
    // judgedPhase checks them again directly before the start, as for every launch. With --attendant-on-screen the
    // marks of the agent operating the screen do not refuse it (marksDecision); nothing else changes.
    if (attended) {
      const marks = marksCheck();
      const attendant = attended.attendantOnScreen ? { stated_by: attended.attendantOnScreen } : null;
      if (marksDecision(marks, attendant).refuse) {
        refuseLine(markRefusalLine(marks, { attendant }));
        await exitAfterSignals(2);
      }
      attended.marksAtCheck = marks;
    }
    // A journey's run empties what it replaces here. An attended launch changes nothing in --out until judgedPhase's own
    // marks check, directly before the start, has admitted it (walkAttended, beforeStart).
    try {
      if (!attended) emptyNativeOutput(out, selected);
    } catch (error) {
      refuseLine(error.message);
      await exitAfterSignals(2);
    }
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

  function readScreenLock() {
    try { return screenLockState(execFileSync("/usr/sbin/ioreg", ["-n", "Root", "-d1", "-a"], { encoding: "utf8", timeout: 15000, maxBuffer: 16 * 1024 * 1024 })); } catch { return null; }
  }
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

  // One judged launch (`judged-launch.mjs`); a mark present or an unresolved census ends the run there. `judge`, given
  // only by the attended launch, changes what is judged and nothing of how the launch is made: the report judged
  // (`reportPath`, the driver's start checkpoint), where the judgment goes (`check`), a hook directly before the
  // start (`beforeStart`, given the nonce) and the AbortSignal that ends the launch through runUnderLock (`abort`);
  // its strict chooser stop is recorded as its ending instead of here.
  async function launch(where, plan, { timeoutMs, judge = null }) {
    const phase = `${journey}/walk`;
    const report = path.join(where.root, `report-${journey}.json`);
    const nonce = newNonce();
    console.log(`\n== ${journey} (HOME=${where.home})`);
    const env = launchEnv(where, { ...plan, nonce, timeouts: { ...TIMEOUTS, ...(plan.timeouts ?? {}) } }, report);
    if (env === null) return exitAfterSignals(2);
    const seen = await judgedPhase({
      phase, checkName: "the launch ended cleanly and its report is this run's", launches, check: judge?.check ?? check,
      summaryPath: path.join(RUN_ROOT, "summary.json"), summary: runRecord,
      executable: binary, command: CAFFEINATE, commandArgs: ["-di", binary], cwd: where.root, env, timeoutMs, reportPath: judge?.reportPath ?? report, nonce,
      censusBoundMs: TERMINATION_BOUND_MS, beforeStart: (seen) => { beforeStart(where); judge?.beforeStart({ nonce, ...(seen ?? {}) }); },
      ...(judge?.abort ? { abort: judge.abort } : {}),
      ...(judge?.attendant ? { attendant: judge.attendant } : {})
    }).then(phaseContinuation({ prefix: NAME, phase, state, evidenceRoot: RUN_ROOT }));
    if (!judge && launches[phase]?.outcome?.status === STRICT_CHOOSER_EXIT) {
      const lastAct = readReport(`${report}.act.json`).report?.act ?? "unnamed (the driver recorded no act)";
      check("no chooser was asked without a queued answer inside the home", false, `the app ended with the strict chooser's exit code ${STRICT_CHOOSER_EXIT} during the act: ${lastAct}`);
    }
    return { report, seen, failures: launches[phase]?.failures ?? [] };
  }

  async function walk(name) {
    const spec = JOURNEYS[name];
    const where = journeyRoot(name);
    const plan = { journey: name, number: spec.number, bundled_project_id: demo.project.id, support_text: firstSupport.replace(/^support:/, ""), show_window: showWindow };
    const timeoutMs = spec.launch_ms ?? LAUNCH_MS;
    if (spec.timeouts) plan.timeouts = spec.timeouts;
    if (name === "report") plan.local_package = path.join(where.saved, "walkthrough-local.opsproj");
    // The prepared files, in the home before the app starts; refused by name when a declaration is wrong.
    const built = [];
    let storeCopies = [];
    let placedRuns = null;
    try {
      storeCopies = storeCopiesFor(spec);
      const placements = placementsFor(spec.place, { home: where.home, identifier: IDENTIFIER, projectRoot: project });
      plan.placed = placeFiles(placements, { built });
      // The placed store's stored runs, read before the launch from the runner's copy of it in the run's own folder.
      if (storeCopies.length > 0) {
        // Found by its declared `at`, so `{app-data}` is expanded in one place (placementsFor).
        const store = plan.placed[spec.place.find((entry) => entry.at === STORE_AT).key];
        const copy = path.join(where.root, "placed-store.sqlite3");
        fs.copyFileSync(store, copy, fs.constants.COPYFILE_EXCL);
        placedRuns = await readStoredRuns(copy);
      }
      if (spec.open) {
        plan.open = { ...spec.open, ...(spec.open.file ? { file: plan.placed[spec.open.file] } : {}) };
        if (spec.open.by === "model-document") plan.open.project_id = JSON.parse(readFileSync(plan.open.file, "utf8")).project.id;
      }
      check("the prepared files were placed in the throwaway home", true, placements.map(({ key, from }) => `${key}: ${path.relative(project, from)}`));
    } catch (error) {
      check("the prepared files were placed in the throwaway home", false, error.message);
      return;
    }
    const lockedBefore = showWindow ? readScreenLock() : null;
    const { report, seen, failures } = await launch(where, plan, { timeoutMs });
    if (seen && seen.ok === false && seen.error) console.log(`${NAME}: ${name}: the driver reported ${seen.error.split("\n")[0]}`);
    // The launch was censused (an unresolved census ended the run), so its home may go before anything is rendered.
    if (removeHome(where.home) !== null) check("its throwaway home was removed", false, leftHomes.get(where.home));
    else check("its throwaway home was removed", true);
    const records = readStepRecords(report);
    const nativeMenu = nativeMenuRecord(readReport(`${report}.menu.json`), { journey: name, number: spec.number });
    const windowRecord = readReport(`${report}.window.json`).report ?? null;
    const shown = windowShownRecord(showWindow, windowRecord, showWindow ? { before: lockedBefore, after: readScreenLock() } : null);
    check("the menu the app built was read", nativeMenu.read, nativeMenu.read ? `${nativeMenu.menus.length} menus` : nativeMenu.error);
    if (showWindow) check("the window was brought to the front (--show-window)", shown.shown, shown);
    let storedRuns;
    if (storeCopies.length > 0) {
      try {
        storedRuns = await storedRunEvidence({ names: storeCopies, report, placed: placedRuns });
        // Read, not judged: the line says which, so a pass here is not taken for custody shown.
        check(`the placed store's stored runs are read before and at each store copy: ${storedRuns.unchanged ? "unchanged" : "NOT shown unchanged"}`, true, { unchanged: storedRuns.unchanged });
      } catch (error) {
        storedRuns = { error: error.message };
        check("the placed store's stored runs are read before and at each store copy", false, error.message);
      }
    }
    const launchFailure = failures.length > 0 ? failures.join("; ") : (seen?.ok === false ? seen.error?.split("\n")[0] ?? "the driver did not finish" : null);
    const screens = screensFromRecords(records, { journey: name, number: spec.number, launchFailure });
    const { chromium } = await import("@playwright/test");
    const executablePath = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH || undefined;
    await renderScreens(records, screens, { out, snapshots: path.join(where.root, "snapshots"), launchBrowser: () => chromium.launch({ executablePath }) });
    writePart(out, {
      number: spec.number, name, title: spec.title, use_case: spec.use_case, screens, native_menu: nativeMenu, window_shown: shown,
      revision: { commit: revision, uncommitted_paths: uncommitted },
      build: { kind: "scripted-run build (native-smoke feature), launched as a private copy", app_sha256: app.identity.sha256, app_bytes: app.identity.size, binding_record_sha256: bindingSha }, launch: { ended_cleanly: failures.length === 0, failures, driver_finished: seen?.ok === true, window_shown: shown.shown },
      ...(built.length ? { built_placements: built } : {}), ...(storedRuns ? { stored_runs: storedRuns } : {}) });
    const pictured = screens.filter((screen) => screen.file).length;
    const stop = screens.find((screen) => screen.failed);
    const drawn = screens.filter((screen) => screen.view_3d === "captured").length;
    const blank = screens.filter((screen) => screen.view_3d === "not captured").length;
    console.log(`${NAME}: ${name}: ${pictured} screens${stop ? `; stopped at "${stop.step}": ${stop.failed}` : ""}; window ${shown.shown ? "shown" : "not shown"}; 3D view captured at ${drawn} step(s), not captured at ${blank}`);
    writeFileSync(path.join(where.root, "summary.json"), JSON.stringify({ journey: name, source_revision: revision, app: { binary: app.binary, identity: app.identity, binding_record_sha256: bindingSha }, launches: { [name]: launches[`${name}/walk`] }, checks: checks.filter((item) => item.journey === name) }, null, 2));
  }

  // The attended launch (--attended): launch() as every journey's, the start only, then the attendant's; its record and
  // the attendant's captures after the census and the home's removal.
  // Set when an attended launch could not prepare --out once admitted: nothing is then written there.
  let outputUntouched = false;
  async function walkAttended({ start, limitMinutes, hidden, attendantOnScreen = null, marksAtCheck = null }) {
    const name = ATTENDED.name;
    if (buildLock.inherited) {
      check("the runner holds the worktree build lock itself, so the launch leads a process group of its own", false, "an inherited build lock leaves the launch in this runner's group, which the finish file cannot end alone; not launched");
      return;
    }
    const where = journeyRoot(name);
    const finishFile = path.join(RUN_ROOT, ATTENDED.finish_file);
    const captures = path.join(out, ATTENDED.captures);
    // The captures folder is this launch's own, made once the launch is admitted, and read at the end only if it is
    // still that folder.
    let capturesIdentity = null;
    let prepared = false;
    const plan = { journey: name, number: ATTENDED.number, attended: true, start, bundled_project_id: demo.project.id, show_window: !hidden };
    const spec = start ? JOURNEYS[start] : null;
    // The start's prepared files, exactly as walk() places a journey's.
    let placements = [];
    try {
      placements = placementsFor(spec?.place, { home: where.home, identifier: IDENTIFIER, projectRoot: project });
      plan.placed = placeFiles(placements);
      if (spec?.open) {
        plan.open = { ...spec.open, ...(spec.open.file ? { file: plan.placed[spec.open.file] } : {}) };
        if (spec.open.by === "model-document") plan.open.project_id = JSON.parse(readFileSync(plan.open.file, "utf8")).project.id;
      }
      check("the prepared files were placed in the throwaway home", true, placements.map(({ key, from }) => `${key}: ${path.relative(project, from)}`));
    } catch (error) {
      check("the prepared files were placed in the throwaway home", false, error.message);
      return;
    }
    const report = path.join(where.root, `report-${name}.json`);
    const startReport = `${report}.start.json`;
    const judged = [];
    let startedAt = null;
    let marksBeforeStart = null;
    const watch = attendedWatch({
      executable: binary, finishFile, startReport, census: censusOf, limitMs: limitMinutes * 60_000, requireVisible: !hidden,
      onReady: ({ pid, start: observed }) => {
        const onScreen = attendantOnScreen ? { stated_by: attendantOnScreen, marks: markNames(marksBeforeStart) } : null;
        for (const line of attendedStartLines({ pid, boundApp: app.binary, launchCopy: binary, finishFile, captures, limitMinutes, endsAt: new Date(startedAt + limitMinutes * 60_000).toISOString(), observed, start, attendantOnScreen: onScreen })) console.log(line);
      }
    });
    const lockedBefore = hidden ? null : readScreenLock();
    console.log(`${NAME}: attended launch; it ends when the app exits, when ${finishFile} appears, or after ${limitMinutes} minute(s)`);
    await launch(where, plan, {
      timeoutMs: limitMinutes * 60_000 + ATTENDED.limit.backstop_ms,
      judge: {
        reportPath: startReport,
        check: (checkName, ok, detail) => judged.push({ name: checkName, ok, detail }),
        abort: watch.signal,
        beforeStart: ({ nonce, marks }) => {
          startedAt = Date.now();
          marksBeforeStart = marks ?? null;
          watch.start({ nonce });
          // Only now, the launch admitted by judgedPhase's marks check, is --out changed: what this launch replaces goes
          // (the index, an earlier part 90) and the captures folder is made. A late mark refuses before this point and
          // leaves the folder, with any completed journeys' screens and index, exactly as it was.
          try {
            capturesIdentity = prepareAttendedOutput(out);
            prepared = true;
          } catch (error) {
            watch.halt("output-not-prepared", { why: String(error?.message ?? error) });
          }
        },
        ...(attendantOnScreen ? { attendant: { stated_by: attendantOnScreen } } : {})
      }
    });
    const watched = watch.stop();
    const phase = `${name}/walk`;
    const outcome = launches[phase]?.outcome ?? null;
    const receipt = launches[phase]?.abort ?? null;
    const ending = attendedEnding({ outcome, receipt, stop: watched.stop, start: readReport(startReport).report, nonce: outcome?.nonce, limitMinutes, handedOver: watched.ready_printed });
    check(`the attended launch ended in one of its ways (${ending.by})`, ending.ok, ending.what);
    if (removeHome(where.home) !== null) check("its throwaway home was removed", false, leftHomes.get(where.home));
    else check("its throwaway home was removed", true);
    if (!prepared) {
      // Nothing of this launch is written into a folder it could not prepare, and the index is not recomposed.
      outputUntouched = true;
      writeFileSync(path.join(where.root, "summary.json"), JSON.stringify({ journey: name, attended: true, source_revision: revision, launches: { [name]: launches[phase] }, ending, checks: checks.filter((item) => item.journey === name) }, null, 2));
      return;
    }
    const nativeMenu = nativeMenuRecord(readReport(`${report}.menu.json`), { journey: name, number: ATTENDED.number });
    const windowRecord = readReport(`${report}.window.json`).report ?? null;
    const shown = hidden
      ? { option: false, shown: false, why: "--hidden: the window was left where the system put it, so that the launch's life could be checked without taking the screen" }
      : windowShownRecord(true, windowRecord, { before: lockedBefore, after: readScreenLock() });
    check("the menu the app built was read", nativeMenu.read, nativeMenu.read ? `${nativeMenu.menus.length} menus` : nativeMenu.error);
    if (!hidden) check("the window was brought to the front", shown.shown, shown);
    const startRecord = watched.start ?? readReport(startReport).report;
    // The driver's records, made while the attendant worked: what the launch stopped and what the build substituted,
    // and the application's own report of the screen whenever the page settled.
    const substitutionsRead = readReport(`${report}.substitutions.json`).report;
    const substitutions = substitutionsRead?.nonce === outcome?.nonce && Array.isArray(substitutionsRead?.substitutions) ? substitutionsRead.substitutions : [];
    const observed = readObserved(report, outcome?.nonce);
    const found = readCaptures(captures, {
      identity: capturesIdentity,
      bindTo: { revision: { commit: revision, uncommitted_paths: uncommitted }, build_app_sha256: app.identity.sha256 },
      journeyUseCase: spec ? { name: start, use_case: spec.use_case } : null,
      observed: observed.records, substitutions,
      launchTimes: { from: startedAt ?? Date.now(), to: Date.now() }
    });
    if (found.error) check("the captures folder was read at the end of the launch", false, found.error);
    writeAttendedPart(out, {
      title: `Attended launch (launched app): the actual window, driven by an attendant, from ${start ? `the start of ${start}` : "the application as first opened"}`,
      revision: { commit: revision, uncommitted_paths: uncommitted },
      build: { kind: "scripted-run build (native-smoke feature), launched as a private copy", app_sha256: app.identity.sha256, app_bytes: app.identity.size, binding_record_sha256: bindingSha },
      native_menu: nativeMenu, window_shown: shown, captures: found,
      launch: {
        attended: true,
        scripted_steps_after_start: 0,
        scripted: "only the start; after it the driver took no step and touched nothing, and every screen of this part is the attendant's capture",
        journey_start: start
          ? { journey: start, number: spec.number, placed: placements.map(({ key, from }) => ({ key, from: path.relative(project, from) })), open: spec.open ?? null, opened: startRecord?.opened ?? null, substitutions: startRecord?.substitutions ?? [], unused_answers: startRecord?.unused_answers ?? null }
          : { journey: null, what: "the application as first opened, with its demonstration model" },
        start_made: startRecord?.ok === true, start_error: startRecord?.error ?? null,
        window_shown: shown.shown,
        pid: watched.pid, handed_over: watched.ready_printed, handed_over_at: watched.handed_over_at ? new Date(watched.handed_over_at).toISOString() : null,
        started: outcome?.started ?? null, ended: outcome?.ended ?? null,
        ended_by: ending.by, ending: ending.what, ending_ok: ending.ok,
        limit_minutes: limitMinutes, finish_file: finishFile, stop: watched.stop, abort: receipt,
        status: outcome?.status ?? null, signal: outcome?.signal ?? null, timed_out: outcome?.timed_out ?? null,
        census: outcome?.census ?? null,
        guards: startRecord?.guards ?? [],
        attendant_on_screen: attendantOnScreen
          ? { stated: true, stated_by: attendantOnScreen, statement: ATTENDANT_STATEMENT, option: "--attendant-on-screen", marks_at_runner_check: markNames(marksAtCheck), marks_before_start: markNames(launches[phase]?.marks), exempted: launches[phase]?.attendant?.exempted ?? [] }
          : { stated: false, marks_before_start: markNames(launches[phase]?.marks) },
        substitutions,
        stopped_by_launch: substitutions.filter((item) => /stopped by the launch$/.test(item.kind ?? "")),
        observed: { records: observed.records.length, rejected: observed.rejected, bounds: OBSERVED_BOUNDS, what: "the application's own report of the words and controls on screen, made by the driver without pressing, focusing or changing anything" },
        judged: judged.map(({ name: checkName, ok, detail }) => ({ name: checkName, ok, detail: detail ?? null }))
      }
    });
    console.log(`${NAME}: attended: ended by ${ending.by} (${ending.what}); ${found.screens.length} capture(s), ${found.screens.filter((screen) => screen.unexplained).length} unexplained`);
    writeFileSync(path.join(where.root, "summary.json"), JSON.stringify({ journey: name, attended: true, source_revision: revision, app: { binary: app.binary, identity: app.identity, binding_record_sha256: bindingSha }, launches: { [name]: launches[phase] }, ending, checks: checks.filter((item) => item.journey === name) }, null, 2));
  }

  for (const name of selected) {
    journey = name;
    await walk(name);
  }
  if (attended) {
    journey = ATTENDED.name;
    await walkAttended(attended);
  }
  journey = "run";
  if (state.watchedBefore !== null) {
    const changes = tripwireChanges(state.watchedBefore, statWatched(WATCHED));
    state.tripwireDone = true;
    check(`the real user's Library: none of the ${WATCHED.length} product-named paths appeared, disappeared or changed`, changes.length === 0, changes);
  }
  if (outputUntouched) {
    const failed = checks.filter((item) => !item.ok);
    console.log(`${NAME}: --out was left as it was; ${checks.length - failed.length}/${checks.length} confinement and launch checks passed; evidence in ${RUN_ROOT}`);
    buildLock.release();
    await exitAfterSignals(1);
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
