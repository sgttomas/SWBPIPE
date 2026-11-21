#!/usr/bin/env node
// The native walkthrough's command (../README.md, "The launched app"), macOS:
//
//   npm run walkthrough:native --workspace apps/desktop -- --build-only
//   npm run walkthrough:native --workspace apps/desktop -- [--journey <names>] [--out DIR] [--show-window]
//   npm run walkthrough:native --workspace apps/desktop -- --attended [--journey <name>] [--limit-minutes N] [--hidden] --out DIR
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
// replaced by a record, and Print is not pressed. The embedded agent is never started; no provider, model, key,
// network or sign-in is used. Chromium renders after the app has ended, loads nothing, and is given no file of the
// home. Evidence of what a person sees, not a test: a journey that stops is a finding in the index.
//
// --attended (ATTENDED below): one launch made exactly as a journey's (the same launch function, environment,
// confinement, census, tripwire and close), for an attendant who drives the actual window. The driver makes only the
// start (with --journey, that journey's `place` and `open`; without it, the app as first opened), shows the window
// unless --hidden, reads the menu, writes the checkpoint `start` and then takes no step. The launch ends when the app
// exits by itself, when the finish file the command prints appears (the runner then sends SIGKILL to the launch's
// process group, as a time limit does), or at --limit-minutes (runUnderLock's own time limit). The index gets one part
// (number 90) recording the launch, and one screen for each picture the attendant put in <out>/captures/, marked as a
// capture of the actual window, not redrawn and not scripted; the runner takes no capture and alters none.
import { randomBytes } from "node:crypto";
import { execFileSync, spawnSync } from "node:child_process";
import * as fs from "node:fs";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync, writeSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { emptyOutput } from "../run.mjs";
import { readReport } from "../../native-smoke/launch-outcome.mjs";
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
  limit: Object.freeze({ default_minutes: 20, min_minutes: 1, max_minutes: 60 }),
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
 * The command line with --attended: { buildOnly: false, journeys: [], out, showWindow, attended: { start, limitMinutes,
 * hidden } } or { error }. --out is required; --journey names at most one journey, and it must declare `place`.
 */
export function parseAttendedArguments(argv) {
  const options = { attended: 0, journey: null, out: null, limit: null, hidden: 0 };
  for (let index = 0; index < argv.length; index += 1) {
    const name = argv[index];
    if (name === "--attended" || name === "--hidden") {
      options[name.slice(2)] += 1;
      if (options[name.slice(2)] > 1) return { error: `${name} is given twice` };
      continue;
    }
    if (name === "--build-only") return { error: "--attended launches the bound app and cannot be given with --build-only: build first, as its own job" };
    if (name === "--show-window") return { error: "--attended brings the window to the front unless --hidden; --show-window is not given with it" };
    const key = { "--journey": "journey", "--out": "out", "--limit-minutes": "limit" }[name];
    if (!key) return { error: `unknown argument ${name}` };
    const value = argv[index + 1];
    if (value === undefined || value.startsWith("--")) return { error: `${name} needs a value` };
    if (options[key] !== null) return { error: `${name} is given twice` };
    options[key] = value;
    index += 1;
  }
  if (options.out === null) return { error: "--attended needs --out <folder>: the record and the captures folder go there" };
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
  return { buildOnly: false, journeys: [], out: path.resolve(options.out), showWindow: options.hidden === 0, attended: { start, limitMinutes, hidden: options.hidden === 1 } };
}

/** The whole command line: the attended form when --attended is given, otherwise parseArguments. */
export function parseCommandLine(argv, options) {
  if (argv.includes("--attended")) return parseAttendedArguments(argv);
  const attendedOnly = argv.find((item) => item === "--hidden" || item === "--limit-minutes");
  if (attendedOnly) return { error: `${attendedOnly} is an option of --attended` };
  return { ...parseArguments(argv, options), attended: null };
}

/** Before anything is built or launched: an earlier attended part goes; an earlier captures folder refuses the run. */
export function emptyAttendedOutput(out) {
  emptyOutput(out, (journeyNumber) => journeyNumber === ATTENDED.number);
  if (lstatOrNull(path.join(out, ATTENDED.captures)) !== null) {
    throw new Error(`${path.join(out, ATTENDED.captures)} holds an earlier attended launch's captures; choose another --out`);
  }
}
const lstatOrNull = (file) => { try { return fs.lstatSync(file); } catch { return null; } };

/** The lines the command prints once the start is made, `attended.<key>=<value>`, the last `attended.ready=yes`. */
export function attendedStartLines({ pid, boundApp, launchCopy, finishFile, captures, limitMinutes, endsAt, shown, start }) {
  return [
    `attended.pid=${pid ?? "unknown"}`,
    `attended.process_name=${ATTENDED.process_name}`,
    `attended.bound_app=${boundApp}`,
    `attended.launch_copy=${launchCopy}`,
    `attended.window_title_ends_with=${ATTENDED.window_title_ends_with}`,
    `attended.window_shown=${shown ? "yes" : "no (--hidden)"}`,
    `attended.start=${start ?? "the application as first opened"}`,
    `attended.finish_file=${finishFile}`,
    `attended.captures=${captures}`,
    `attended.limit_minutes=${limitMinutes}`,
    `attended.ends_at=${endsAt}`,
    `attended.chooser=${ATTENDED.chooser}`,
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
 * Ends the launch as a time limit ends it: SIGKILL to the app's process group, which runUnderLock started and tracks.
 * Sent only when that group is not this runner's own and its leader is this runner's child; otherwise refused by name.
 * Returns a receipt in runUnderLock's form: { requested_at, target, signal, delivered, delivered_at, error }.
 */
export function endLaunchGroup(appPid, { table, self = process.pid, signal = (pgid, name) => process.kill(-pgid, name) }) {
  const requested_at = Date.now();
  const refused = (target, error) => ({ requested_at, target, signal: "SIGKILL", delivered: false, delivered_at: null, error });
  const row = appPid === null || appPid === undefined ? null : table?.get(appPid);
  if (!row) return refused(`process ${appPid ?? "unknown"}`, "the app's process was not found");
  const own = table.get(self)?.pgid;
  if (own === undefined || row.pgid === own) return refused(`group ${row.pgid}`, "the app is in this runner's own process group");
  if (table.get(row.pgid)?.ppid !== self) return refused(`group ${row.pgid}`, "the group's leader is not this runner's child");
  try {
    signal(row.pgid, "SIGKILL");
    return { requested_at, target: `group ${row.pgid}`, signal: "SIGKILL", delivered: true, delivered_at: Date.now(), error: null };
  } catch (error) {
    return refused(`group ${row.pgid}`, error.code ?? error.message);
  }
}

/**
 * Watches one attended launch from directly before its start until stop(): the app's pid (from `census`, the
 * native-smoke census of the private copy), the driver's `start` checkpoint carrying `nonce`, and the finish file.
 * Once both the start and the pid are known (or the pid is not found within `pidGraceMs` of the start), `onReady` is
 * called once. When the finish file appears, the launch's group is ended (endLaunchGroup) once. stop() returns
 * { pid, start, ready_printed, finish }, where `finish` is null or { observed_at, receipt }.
 */
export function watchAttended({ executable, finishFile, startReport, nonce, census, onReady = () => {}, pollMs = 250, pidGraceMs = 10_000, processTable = readProcessTable, signal, self = process.pid }) {
  const seen = { pid: null, start: null, startAt: null, printed: false, finish: null };
  const exists = (file) => lstatOrNull(file) !== null;
  const findPid = () => {
    if (seen.pid !== null) return seen.pid;
    const result = census(executable);
    if (result?.ok && result.pids.length > 0) seen.pid = appPidOf(result.pids, processTable(), self);
    return seen.pid;
  };
  const tick = () => {
    findPid();
    if (seen.start === null) {
      const read = readReport(startReport).report;
      if (read && read.nonce === nonce) { seen.start = read; seen.startAt = Date.now(); }
    }
    if (seen.start !== null && !seen.printed && (seen.pid !== null || Date.now() - seen.startAt > pidGraceMs)) {
      seen.printed = true;
      onReady({ pid: seen.pid, start: seen.start });
    }
    if (seen.finish === null && exists(finishFile)) {
      const observed_at = Date.now();
      seen.finish = { observed_at, receipt: endLaunchGroup(findPid(), { table: processTable(), self, ...(signal ? { signal } : {}) }) };
    }
  };
  const timer = setInterval(() => { try { tick(); } catch { /* the next poll tries again */ } }, pollMs);
  return {
    stop() {
      clearInterval(timer);
      return { pid: seen.pid, start: seen.start, ready_printed: seen.printed, finish: seen.finish };
    }
  };
}

/**
 * How an attended launch ended, from its recorded outcome (`recordedOutcome`), the watcher's finish record, the
 * driver's final report if it wrote one (only when the start failed) and its `start` checkpoint:
 * { by, ok, what } with `by` one of limit, finish-file, strict-chooser, start-failed, app-exited.
 */
export function attendedEnding({ outcome, finish = null, finalReport = null, start = null, nonce, limitMinutes, strictChooserExit = 86 }) {
  if (!outcome) return { by: "unknown", ok: false, what: "no outcome of the launch was recorded" };
  let ending;
  if (outcome.timed_out) {
    ending = { by: "limit", ok: true, what: `the limit of ${limitMinutes} minute(s) passed; the runner ended the app as at any launch's time limit (SIGKILL to its process group)` };
  } else if (finish?.receipt?.delivered === true && outcome.signal === "SIGKILL") {
    ending = { by: "finish-file", ok: true, what: "the finish file appeared; the runner ended the app as a time limit does (SIGKILL to its process group)" };
  } else if (outcome.status === strictChooserExit) {
    ending = { by: "strict-chooser", ok: true, what: `a file chooser was opened with no answer queued; the strict chooser ended the app before any panel (exit ${strictChooserExit})` };
  } else if (finalReport) {
    ending = { by: "start-failed", ok: false, what: `the driver ended the app because the start failed: ${String(finalReport.error ?? "no reason recorded").split("\n")[0]}` };
  } else if (outcome.status === 0 && outcome.signal === null) {
    ending = { by: "app-exited", ok: true, what: "the application exited by itself (status 0), as Quit or closing its window ends it" };
  } else {
    ending = { by: "app-exited", ok: false, what: `the application ended by itself with ${outcome.signal ? `signal ${outcome.signal}` : `status ${outcome.status}`}` };
  }
  if (finish && ending.by !== "finish-file") ending.what += `; the finish file appeared, and ending the group ${finish.receipt?.delivered ? "was signalled" : `was not signalled (${finish.receipt?.error ?? "no receipt"})`}`;
  const started = start !== null && start.nonce === nonce && start.ok === true;
  if (!started) return { ...ending, ok: false, what: `${ending.what}; the start was not reported made by this launch${start?.error ? ` (${start.error})` : ""}` };
  return ending;
}

const SIDECAR_LIMIT = 64 * 1024;
const PNG_SIGNATURE = Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]);

/**
 * A sidecar's optional surface claims: `surfaces`, the native surfaces the picture shows, and `prevented`, one record
 * or a list of { surface, what } for an attempt at a surface that something prevented. Each surface is a key of
 * SURFACE_KEYS; an unknown one is kept apart as unexplained. { surfaces, surfaces_unexplained, prevented,
 * prevented_unexplained } or { why } when a member is not of that form.
 */
export function surfaceClaims(value) {
  const claims = { surfaces: [], surfaces_unexplained: [], prevented: [], prevented_unexplained: [] };
  if (value.surfaces !== undefined) {
    if (!Array.isArray(value.surfaces) || value.surfaces.length > 20 || !value.surfaces.every((item) => typeof item === "string" && item.length > 0 && item.length <= 80)) return { why: "its sidecar's surfaces is not a list of surface names" };
    for (const item of value.surfaces) (Object.hasOwn(SURFACE_KEYS, item) ? claims.surfaces : claims.surfaces_unexplained).push(item);
  }
  if (value.prevented !== undefined) {
    const records = Array.isArray(value.prevented) ? value.prevented : [value.prevented];
    const valid = (record) => record && typeof record === "object" && !Array.isArray(record) && typeof record.surface === "string" && record.surface.length > 0 && record.surface.length <= 80 && typeof record.what === "string" && record.what.trim() !== "" && record.what.length <= 1000;
    if (records.length === 0 || records.length > 20 || !records.every(valid)) return { why: "its sidecar's prevented is not { surface, what } or a list of them" };
    for (const { surface, what } of records) (Object.hasOwn(SURFACE_KEYS, surface) ? claims.prevented : claims.prevented_unexplained).push({ surface, what: what.trim() });
  }
  return claims;
}

/** A sidecar's claims: { step, note, acted, ...surfaceClaims } or { why } when it is not one. */
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
  if (typeof value.step !== "string" || value.step.trim() === "" || value.step.length > 200) return { why: "its sidecar names no step (a string of 1 to 200 characters)" };
  if (typeof value.note !== "string" || value.note.length > 4000) return { why: "its sidecar has no note (a string of at most 4000 characters)" };
  if (!Array.isArray(value.acted) || value.acted.length > 200 || !value.acted.every((item) => typeof item === "string" && item.length <= 1000)) return { why: "its sidecar's acted is not a list of what was done (strings)" };
  const claims = surfaceClaims(value);
  if (claims.why) return claims;
  return { step: value.step.trim(), note: value.note, acted: value.acted, ...claims, raw: value };
}

function pngFacts(file) {
  const handle = fs.openSync(file, "r");
  try {
    const head = Buffer.alloc(24);
    const read = fs.readSync(handle, head, 0, 24, 0);
    if (read < 24 || !head.subarray(0, 8).equals(PNG_SIGNATURE) || head.toString("latin1", 12, 16) !== "IHDR") return null;
    return { width: head.readUInt32BE(16), height: head.readUInt32BE(20), bytes: fs.fstatSync(handle).size };
  } finally {
    fs.closeSync(handle);
  }
}

/**
 * The attendant's captures in `folder` (<out>/captures), read, never changed: { screens, records_without_picture,
 * other }. Each `.png` that is a regular file becomes a screen: with a sidecar `<same name>.json` of the form
 * { step, note, acted }, a capture of the actual window; without one, or with one that is not of that form, or when
 * the file is not a PNG by its signature, an unexplained entry that says why. A sidecar with no picture is listed in
 * records_without_picture, and anything else (a link, a folder, another kind of file) in other; nothing is dropped
 * and nothing is followed.
 */
export function readCaptures(folder, { number = ATTENDED.number, journey = ATTENDED.name } = {}) {
  const found = { screens: [], records_without_picture: [], other: [] };
  let entries;
  try { entries = fs.readdirSync(folder, { withFileTypes: true }).map((entry) => entry.name).sort(); } catch { return found; }
  const regular = (name) => lstatOrNull(path.join(folder, name))?.isFile() === true;
  const pictures = entries.filter((name) => /\.png$/i.test(name) && regular(name));
  const pictureBases = new Set(pictures.map((name) => name.replace(/\.png$/i, "")));
  let count = 0;
  for (const name of pictures) {
    count += 1;
    const base = name.replace(/\.png$/i, "");
    const file = `${ATTENDED.captures}/${name}`;
    const entry = { number: number * 100 + count, journey, file, redrawn: false, scripted: false };
    let facts = null;
    try { facts = pngFacts(path.join(folder, name)); } catch { facts = null; }
    const sidecar = entries.includes(`${base}.json`) ? readSidecar(path.join(folder, `${base}.json`)) : { why: "it has no sidecar of the same name" };
    const why = facts === null ? "it is not a PNG file by its signature" : sidecar.why;
    if (why) {
      found.screens.push({ ...entry, step: `unexplained-${base}`.slice(0, 120), note: `${name}: unexplained, because ${why}`, capture: "unexplained", unexplained: why, picture: UNEXPLAINED_BASIS, ...(facts ? { png: facts } : {}) });
      continue;
    }
    const { surfaces, surfaces_unexplained, prevented, prevented_unexplained } = sidecar;
    found.screens.push({ ...entry, step: sidecar.step, note: sidecar.note, acted: sidecar.acted, capture: "the actual window", driven_by: "an attendant, through the window", picture: CAPTURE_BASIS, png: facts, surfaces, surfaces_unexplained, prevented, prevented_unexplained });
  }
  for (const name of entries) {
    if (pictures.includes(name)) continue;
    const base = name.replace(/\.json$/i, "");
    if (/\.json$/i.test(name) && regular(name)) {
      if (pictureBases.has(base)) continue;
      found.records_without_picture.push({ file: `${ATTENDED.captures}/${name}`, ...(({ raw, ...claims }) => claims)(readSidecar(path.join(folder, name))) });
      continue;
    }
    const stat = lstatOrNull(path.join(folder, name));
    found.other.push({ file: `${ATTENDED.captures}/${name}`, why: stat?.isSymbolicLink() ? "a link, not followed" : stat?.isDirectory() ? "a folder, not entered" : "neither a PNG picture nor a sidecar" });
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
    captures: { folder: ATTENDED.captures, records_without_picture: captures.records_without_picture, other: captures.other },
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
  // The output folder is checked before anything is built or launched: a folder of other work is refused here.
  if (!buildOnly) {
    try { emptyNativeOutput(out, selected); } catch (error) { refuseLine(error.message); process.exit(2); }
  }
  if (attended) {
    try { emptyAttendedOutput(out); } catch (error) { refuseLine(error.message); process.exit(2); }
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
  // (`reportPath`, the driver's start checkpoint), where the judgment goes (`check`) and a hook directly before the
  // start (`beforeStart`, given the nonce); its strict chooser stop is recorded as its ending instead of here.
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
      censusBoundMs: TERMINATION_BOUND_MS, beforeStart: () => { beforeStart(where); judge?.beforeStart({ nonce }); }
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
    const launchFailure = failures.length > 0 ? failures.join("; ") : (seen?.ok === false ? seen.error?.split("\n")[0] ?? "the driver did not finish" : null);
    const screens = screensFromRecords(records, { journey: name, number: spec.number, launchFailure });
    const { chromium } = await import("@playwright/test");
    const executablePath = process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH || undefined;
    await renderScreens(records, screens, { out, snapshots: path.join(where.root, "snapshots"), launchBrowser: () => chromium.launch({ executablePath }) });
    writePart(out, {
      number: spec.number, name, title: spec.title, use_case: spec.use_case, screens, native_menu: nativeMenu, window_shown: shown,
      revision: { commit: revision, uncommitted_paths: uncommitted },
      build: { kind: "scripted-run build (native-smoke feature), launched as a private copy", app_sha256: app.identity.sha256, app_bytes: app.identity.size, binding_record_sha256: bindingSha }, launch: { ended_cleanly: failures.length === 0, failures, driver_finished: seen?.ok === true, window_shown: shown.shown } });
    const pictured = screens.filter((screen) => screen.file).length;
    const stop = screens.find((screen) => screen.failed);
    const drawn = screens.filter((screen) => screen.view_3d === "captured").length;
    const blank = screens.filter((screen) => screen.view_3d === "not captured").length;
    console.log(`${NAME}: ${name}: ${pictured} screens${stop ? `; stopped at "${stop.step}": ${stop.failed}` : ""}; window ${shown.shown ? "shown" : "not shown"}; 3D view captured at ${drawn} step(s), not captured at ${blank}`);
    writeFileSync(path.join(where.root, "summary.json"), JSON.stringify({ journey: name, source_revision: revision, app: { binary: app.binary, identity: app.identity, binding_record_sha256: bindingSha }, launches: { [name]: launches[`${name}/walk`] }, checks: checks.filter((item) => item.journey === name) }, null, 2));
  }

  // The attended launch (--attended): launch() as every journey's, the start only, then the attendant's; its record and
  // the attendant's captures after the census and the home's removal.
  async function walkAttended({ start, limitMinutes, hidden }) {
    const name = ATTENDED.name;
    if (buildLock.inherited) {
      check("the runner holds the worktree build lock itself, so the launch leads a process group of its own", false, "an inherited build lock leaves the launch in this runner's group, which the finish file cannot end alone; not launched");
      return;
    }
    const where = journeyRoot(name);
    const finishFile = path.join(RUN_ROOT, ATTENDED.finish_file);
    const captures = path.join(out, ATTENDED.captures);
    mkdirSync(captures);
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
    let watch = null;
    let startedAt = null;
    const lockedBefore = hidden ? null : readScreenLock();
    console.log(`${NAME}: attended launch; it ends when the app exits, when ${finishFile} appears, or after ${limitMinutes} minute(s)`);
    await launch(where, plan, {
      timeoutMs: limitMinutes * 60_000,
      judge: {
        reportPath: startReport,
        check: (checkName, ok, detail) => judged.push({ name: checkName, ok, detail }),
        beforeStart: ({ nonce }) => {
          startedAt = Date.now();
          watch = watchAttended({
            executable: binary, finishFile, startReport, nonce, census: censusOf,
            onReady: ({ pid }) => {
              for (const line of attendedStartLines({ pid, boundApp: app.binary, launchCopy: binary, finishFile, captures, limitMinutes, endsAt: new Date(startedAt + limitMinutes * 60_000).toISOString(), shown: !hidden, start })) console.log(line);
            }
          });
        }
      }
    });
    const watched = watch ? watch.stop() : { pid: null, start: null, ready_printed: false, finish: null };
    const phase = `${name}/walk`;
    const outcome = launches[phase]?.outcome ?? null;
    const finalReport = readReport(report).report;
    const ending = attendedEnding({ outcome, finish: watched.finish, finalReport, start: readReport(startReport).report, nonce: outcome?.nonce, limitMinutes });
    check(`the attended launch ended in one of its ways (${ending.by})`, ending.ok, ending.what);
    if (removeHome(where.home) !== null) check("its throwaway home was removed", false, leftHomes.get(where.home));
    else check("its throwaway home was removed", true);
    const nativeMenu = nativeMenuRecord(readReport(`${report}.menu.json`), { journey: name, number: ATTENDED.number });
    const windowRecord = readReport(`${report}.window.json`).report ?? null;
    const shown = hidden
      ? { option: false, shown: false, why: "--hidden: the window was left where the system put it, so that the launch's life could be checked without taking the screen" }
      : windowShownRecord(true, windowRecord, { before: lockedBefore, after: readScreenLock() });
    check("the menu the app built was read", nativeMenu.read, nativeMenu.read ? `${nativeMenu.menus.length} menus` : nativeMenu.error);
    if (!hidden) check("the window was brought to the front", shown.shown, shown);
    const found = readCaptures(captures);
    const startRecord = watched.start ?? readReport(startReport).report;
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
        pid: watched.pid,
        started: outcome?.started ?? null, ended: outcome?.ended ?? null,
        ended_by: ending.by, ending: ending.what, ending_ok: ending.ok,
        limit_minutes: limitMinutes, finish_file: finishFile, finish: watched.finish,
        status: outcome?.status ?? null, signal: outcome?.signal ?? null, timed_out: outcome?.timed_out ?? null,
        census: outcome?.census ?? null,
        prevented: ending.by === "strict-chooser" ? [{ surface: "file_chooser", what: "the attendant opened a file chooser and the strict chooser ended the application before any panel (exit 86): this build shows no chooser without a queued answer" }] : [],
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
