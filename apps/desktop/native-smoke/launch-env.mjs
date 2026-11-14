// What a native-smoke launch of the desktop app is given, what it must not
// reach, and how the run is closed (DEL-07-11 VER-003; `run.mjs`).
//
// The launched application's environment is built from nothing: only
// variables on PARENT_ALLOWLIST are carried from the runner's environment;
// then HOME and CFFIXED_USER_HOME, both the throwaway home; TMPDIR, a folder
// inside that home; the strict chooser switch; and the runner's own
// SWBPIPE_NATIVE_SMOKE_* variables. `refusedLaunchEnv` names every reason the
// runner must not launch with a built environment, and `refusedPlanPaths`
// every path in the plan (each chooser answer among them) that does not
// resolve inside the home. The home is created empty under one root. After
// each launch the runner takes a census of the processes running its private
// copy of the app and waits for none (`awaitNoProcesses`); `closeRun` decides,
// on every exit the runner can catch, the tripwire's comparison, the exit code
// and whether the home, the private copy and the lock are retained. A marker
// file records a run between its first launch and its completed cleanup, so a
// later invocation recognises an interrupted or unresolved one.
import { spawnSync } from "node:child_process";
import { cpSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, readlinkSync, realpathSync, statSync, writeFileSync } from "node:fs";
import path from "node:path";

// Variables carried from the runner's environment, each with why the app or
// /usr/bin/caffeinate needs it. Empty, by reading at the time of writing:
// - caffeinate is started by absolute path and given the app by absolute path,
//   so it needs no PATH lookup;
// - the app reads, by name, only HOME, CFFIXED_USER_HOME and TMPDIR (passed to
//   its solve worker, `solve_worker.rs` WORKER_PASSED_ENV) and its own
//   SWBPIPE_NATIVE_SMOKE_* variables (`native_smoke.rs`); every child it starts
//   is started with a cleared environment (`solve_worker.rs`,
//   `embedded_agent/macos.rs`);
// - PATH, USER, LOGNAME, SHELL, LANG and LC_*, __CF_USER_TEXT_ENCODING and the
//   XPC_* variables were considered and left out: nothing found shows the app
//   needs them (macOS resolves the user from the account database and
//   CoreFoundation supplies its own text encoding when the variable is absent).
// A variable added here needs its one-line reason beside it.
export const PARENT_ALLOWLIST = Object.freeze([]);

// The strict chooser switch (`native_smoke.rs` STRICT_CHOOSER_ENV and
// STRICT_CHOOSER_VALUE): a chooser with no queued answer, or with a queued
// answer outside HOME, ends the app (exit 86) before any native dialog.
export const STRICT_CHOOSER_ENV = "SWBPIPE_NATIVE_SMOKE_STRICT_CHOOSER";
export const STRICT_CHOOSER_VALUE = "1";

// The runner's own variables (`native_smoke.rs` reads each).
export const RUNNER_KEYS = Object.freeze([
  "SWBPIPE_NATIVE_SMOKE_DRIVER", // the script evaluated in the main webview
  "SWBPIPE_NATIVE_SMOKE_PLAN", // the JSON plan handed to the driver
  "SWBPIPE_NATIVE_SMOKE_REPORT", // where native_smoke_finish writes the report
  STRICT_CHOOSER_ENV // refuse, never show, a chooser without an answer inside the home
]);

// The home variables and the temporary folder the runner sets itself.
export const HOME_KEYS = Object.freeze(["HOME", "CFFIXED_USER_HOME", "TMPDIR"]);

// The folder inside the home given to the app as TMPDIR.
const TMP_NAME = "tmp";

// The throwaway home: a new folder under `homeRoot` (an existing folder),
// refused unless it is empty when created; then TMPDIR's folder and Downloads
// are created inside it. Both paths are returned resolved (no links), so the
// app and the runner judge the same names. `mkdtemp` is replaceable for tests.
export function createThrowawayHome(homeRoot, { mkdtemp = mkdtempSync } = {}) {
  let root;
  try {
    root = realpathSync(homeRoot);
  } catch {
    throw new Error(`native smoke: --home-root ${homeRoot} does not exist`);
  }
  if (!statSync(root).isDirectory()) throw new Error(`native smoke: --home-root ${homeRoot} is not a folder`);
  const home = realpathSync(mkdtemp(path.join(root, "swbpipe-native-smoke-home-")));
  const found = readdirSync(home);
  if (found.length > 0) throw new Error(`native smoke: HOME-NOT-EMPTY: the new home ${home} already holds ${found.length} entr${found.length === 1 ? "y" : "ies"} at creation; nothing is launched`);
  const tmpdir = path.join(home, TMP_NAME);
  mkdirSync(tmpdir);
  mkdirSync(path.join(home, "Downloads"));
  return { home, tmpdir };
}

// Where the plan's chooser answers live: all inside the home, so the strict
// chooser admits them. Folders, then the files the driver queues.
export function chooserLayout(home) {
  const area = path.join(home, "chooser");
  const models = path.join(area, "models");
  const savedModels = path.join(area, "saved-model-documents");
  const savedHandoffs = path.join(area, "saved-handoffs");
  const savedReportPackages = path.join(area, "saved-report-packages");
  return {
    area,
    folders: [area, models, savedModels, savedHandoffs, savedReportPackages],
    models,
    savedModels,
    savedHandoffs,
    modelDocumentSave: { first: path.join(savedModels, "saved-demo.json"), second: path.join(savedModels, "saved-demo-reopened.json") },
    handoffSave: { path: path.join(savedHandoffs, "saved-demo.swbhandoff") },
    reportPackageSave: { path: path.join(savedReportPackages, "saved-demo.opsproj") },
    exactReportPackageSave: { path: path.join(savedReportPackages, "saved-exact.opsproj") }
  };
}

// What a run keeps out of the home before the home is removed: the files its
// checks read back. Paths relative to the home.
export function keptRelativePaths(identifier) {
  return ["chooser", "Downloads", path.join("Library", "Application Support", identifier)];
}

// Copy each of `relatives` that exists from `home` to `destination/kept/`,
// links copied as links (never followed). Returns the copied relative paths.
export function keepOutOfHome(home, destination, relatives) {
  const copied = [];
  for (const relative of relatives) {
    const source = path.join(home, relative);
    if (lstatSync(source, { throwIfNoEntry: false }) === undefined) continue;
    cpSync(source, path.join(destination, "kept", relative), { recursive: true, verbatimSymlinks: true, errorOnExist: true, force: false });
    copied.push(relative);
  }
  return copied;
}

// The launched application's whole environment.
// The smoke build's windows: the product's, with background throttling off and a NON-PERSISTENT web view
// data store. Tauri's `incognito` selects WKWebsiteDataStore.nonPersistentDataStore() for the web view on
// macOS (wry, wkwebview/mod.rs). What is established is that selection. Where the system's web view keeps
// a persistent store under a home override was not established by reading, which is why none is selected;
// whether the framework or its helper processes keep caches of their own is not established either.
// A run by this runner therefore does not show that a product build keeps web storage across launches.
// Product builds and tauri.conf.json are unchanged.
export function smokeWindows(windows) {
  return windows.map((window) => ({ ...window, backgroundThrottling: "disabled", incognito: true }));
}

// The reasons a window configuration may not be launched: none, or a window whose web view would persist.
export function refusedSmokeWindows(windows) {
  if (!Array.isArray(windows) || windows.length === 0) return ["WEBVIEW-STORE-REFUSED: the smoke build declares no window, so its web view's store is not established"];
  return windows.flatMap((window, index) =>
    window?.incognito === true ? [] : [`WEBVIEW-STORE-REFUSED: window ${index} of the smoke build would use a persistent web view store (incognito is not true)`]);
}

export function buildLaunchEnv({ parent, home, tmpdir, runner }) {
  const env = {};
  for (const key of PARENT_ALLOWLIST) if (Object.hasOwn(parent, key)) env[key] = parent[key];
  env.HOME = home;
  env.CFFIXED_USER_HOME = home;
  env.TMPDIR = tmpdir;
  env[STRICT_CHOOSER_ENV] = STRICT_CHOOSER_VALUE;
  for (const [key, value] of Object.entries(runner)) env[key] = value;
  return env;
}

// Every reason not to launch with `env`, each a named line; empty when it may
// be used.
export function refusedLaunchEnv(env) {
  const reasons = [];
  const allowed = new Set([...PARENT_ALLOWLIST, ...HOME_KEYS, ...RUNNER_KEYS]);
  const extra = Object.keys(env).filter((key) => !allowed.has(key)).sort();
  if (extra.length > 0) reasons.push(`LAUNCH-ENV-REFUSED: variables outside the allowlist and the runner's own: ${extra.join(", ")}`);
  if (typeof env.HOME !== "string" || env.HOME === "" || !path.isAbsolute(env.HOME)) {
    reasons.push("LAUNCH-ENV-REFUSED: HOME is not an absolute path");
  }
  if (env.HOME !== env.CFFIXED_USER_HOME) reasons.push("LAUNCH-ENV-REFUSED: HOME and CFFIXED_USER_HOME differ");
  if (!insideFolder(env.TMPDIR, env.HOME)) reasons.push("LAUNCH-ENV-REFUSED: TMPDIR is not inside HOME");
  if (env[STRICT_CHOOSER_ENV] !== STRICT_CHOOSER_VALUE) reasons.push(`LAUNCH-ENV-REFUSED: ${STRICT_CHOOSER_ENV} is not ${STRICT_CHOOSER_VALUE}`);
  return reasons;
}

// Whether `child` is a folder strictly below `parent`, both absolute, by name.
function insideFolder(child, parent) {
  if (typeof child !== "string" || typeof parent !== "string" || !path.isAbsolute(child) || !path.isAbsolute(parent)) return false;
  const relative = path.relative(parent, child);
  return relative !== "" && !relative.startsWith("..") && !path.isAbsolute(relative);
}

// `file` resolved as far as it exists, as the app's strict chooser judges a
// queued answer (`native_smoke.rs` resolved_answer): its deepest existing
// ancestor resolved (links followed), then the rest appended; null for a
// relative path, an unresolvable existing ancestor, or a `..` below it.
function resolvedAsFarAsExists(file) {
  if (!path.isAbsolute(file)) return null;
  const parts = file.split(path.sep);
  for (let end = parts.length; end > 0; end -= 1) {
    const ancestor = parts.slice(0, end).join(path.sep) || path.sep;
    let entry;
    try {
      entry = lstatSync(ancestor);
    } catch (error) {
      if (error.code === "ENOENT" || error.code === "ENOTDIR") continue;
      return null;
    }
    void entry;
    const rest = parts.slice(end);
    if (rest.some((part) => part === ".." || part === ".")) return null;
    try {
      return path.join(realpathSync(ancestor), ...rest);
    } catch {
      return null;
    }
  }
  return null;
}

// Every absolute path in `plan` (at any depth; every chooser answer the driver
// queues is one) that does not resolve inside `home`, as named lines. A path
// equal to the home is outside: the answers are files and folders below it.
export function refusedPlanPaths(plan, home) {
  const lines = [];
  let resolvedHome;
  try {
    resolvedHome = realpathSync(home);
  } catch {
    return [`LAUNCH-PLAN-REFUSED: the home ${home} cannot be resolved`];
  }
  const visit = (value, where) => {
    if (typeof value === "string") {
      if (!value.startsWith("/")) return;
      const resolved = resolvedAsFarAsExists(value);
      if (resolved === null || !insideFolder(resolved, resolvedHome)) lines.push(`LAUNCH-PLAN-REFUSED: ${where} (${value}) does not resolve inside the home`);
    } else if (Array.isArray(value)) value.forEach((item, index) => visit(item, `${where}[${index}]`));
    else if (value && typeof value === "object") for (const [key, item] of Object.entries(value)) visit(item, where === "" ? key : `${where}.${key}`);
  };
  visit(plan, "");
  return lines;
}

// The tripwire's fixed list: product-named paths under `library` (the real
// user's `~/Library`, or an invented one in a test). `names` are the bundle
// identifier and the product's names (the product name and the executable's
// name, under which an unbundled WebKit process files its data).
export function watchedLibraryPaths(library, { identifier, names }) {
  const all = [identifier, ...names];
  const paths = [];
  for (const folder of ["Application Support", "Caches", "WebKit", "HTTPStorages", "Logs"]) {
    for (const name of all) paths.push(path.join(library, folder, name));
  }
  paths.push(path.join(library, "HTTPStorages", `${identifier}.binarycookies`));
  paths.push(path.join(library, "Saved Application State", `${identifier}.savedState`));
  paths.push(path.join(library, "Preferences", `${identifier}.plist`));
  return [...new Set(paths)];
}

// Each watched path's presence and modification time, by lstat of that exact
// path (a link is not followed). Lists nothing; reads no contents.
export function statWatched(paths) {
  return Object.fromEntries(paths.map((file) => {
    try {
      return [file, { present: true, mtimeNs: lstatSync(file, { bigint: true }).mtimeNs.toString() }];
    } catch (error) {
      if (error.code === "ENOENT" || error.code === "ENOTDIR") return [file, { present: false }];
      return [file, { present: null, error: error.code ?? String(error) }];
    }
  }));
}

// The named lines for every watched path that appeared, disappeared, changed
// its modification time, or could not be examined, between two stats.
export function tripwireChanges(before, after) {
  const lines = [];
  for (const file of Object.keys(before)) {
    const was = before[file];
    const now = after[file];
    if (!now || was.present === null || now.present === null) lines.push(`TRIPWIRE: ${file} could not be examined (${was.error ?? now?.error ?? "missing"})`);
    else if (!was.present && now.present) lines.push(`TRIPWIRE: ${file} appeared`);
    else if (was.present && !now.present) lines.push(`TRIPWIRE: ${file} disappeared`);
    else if (was.present && was.mtimeNs !== now.mtimeNs) lines.push(`TRIPWIRE: ${file} changed its modification time`);
  }
  return lines;
}

// The census: the processes whose executable is `executable` (the run's
// private copy of the app; the solve worker is that same executable in another
// process group). { ok: true, pids } or { ok: false, reason } when it cannot be
// taken. macOS: `lsof` on the file (its txt mapping names the real executable,
// whatever argv[0] says; `ps` shows argv[0]). Linux: each /proc/<pid>/exe.
// `platform`, `lsof` and `run` (spawnSync's shape) are replaceable for tests.
export function censusOf(executable, { platform = process.platform, run = spawnSync, self = process.pid, lsof = ["/usr/sbin/lsof", "/usr/bin/lsof"].find((candidate) => existsSync(candidate)) } = {}) {
  if (platform === "linux") return procCensus(executable, self);
  if (!lsof) return { ok: false, reason: "lsof was not found" };
  const result = run(lsof, ["-w", "-F", "p", "--", executable], { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"], timeout: 30000 });
  if (result.error) return { ok: false, reason: `lsof could not run (${result.error.code ?? result.error.message})` };
  const stderr = String(result.stderr ?? "").trim();
  const pids = String(result.stdout ?? "").split("\n").filter((line) => /^p\d+$/.test(line)).map((line) => Number(line.slice(1))).filter((pid) => pid !== self);
  if (result.status === 0 && stderr === "") return { ok: true, pids };
  // lsof exits 1 with no output when no process has the file open.
  if (result.status === 1 && stderr === "" && String(result.stdout ?? "").trim() === "") return { ok: true, pids: [] };
  return { ok: false, reason: `lsof exited ${result.status ?? result.signal}${stderr ? `: ${stderr.split("\n")[0]}` : ""}` };
}

function procCensus(executable, self) {
  let target;
  try {
    target = realpathSync(executable);
  } catch (error) {
    return { ok: false, reason: `the executable cannot be resolved (${error.code})` };
  }
  let entries;
  try {
    entries = readdirSync("/proc").filter((name) => /^\d+$/.test(name));
  } catch (error) {
    return { ok: false, reason: `/proc cannot be read (${error.code})` };
  }
  const pids = [];
  for (const name of entries) {
    let link;
    try {
      link = readlinkSync(`/proc/${name}/exe`);
    } catch {
      continue; // gone meanwhile, or another user's
    }
    if ((link === target || link === `${target} (deleted)`) && Number(name) !== self) pids.push(Number(name));
  }
  return { ok: true, pids };
}

// Wait, up to `boundMs`, for a census with no process. { resolved: true } once
// one is seen; otherwise { resolved: false, reason } naming the processes left
// or why the census could not be taken: unresolved termination, never "zero
// processes". `sleep(ms)` is awaited between censuses.
export async function awaitNoProcesses(executable, { boundMs, pollMs = 250, census = censusOf, sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms)) }) {
  const deadline = Date.now() + boundMs;
  for (;;) {
    const result = census(executable);
    if (!result.ok) return { resolved: false, reason: `the census could not be taken: ${result.reason}` };
    if (result.pids.length === 0) return { resolved: true };
    if (Date.now() >= deadline) return { resolved: false, reason: `process ${result.pids.join(", ")} still ran ${executable} after ${boundMs} ms` };
    await sleep(pollMs);
  }
}

// The same wait, synchronously, for the exit handler (no event loop there).
export function awaitNoProcessesSync(executable, { boundMs, pollMs = 250, census = censusOf }) {
  const pause = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);
  const deadline = Date.now() + boundMs;
  for (;;) {
    const result = census(executable);
    if (!result.ok) return { resolved: false, reason: `the census could not be taken: ${result.reason}` };
    if (result.pids.length === 0) return { resolved: true };
    if (Date.now() >= deadline) return { resolved: false, reason: `process ${result.pids.join(", ")} still ran ${executable} after ${boundMs} ms` };
    pause(pollMs);
  }
}

// How a run is closed, on every exit the runner can catch. Inputs:
// - `exitCode`: the code the process is exiting with (an interruption's
//   128 + signal, an exception's 1, a failed run's 1, or 0);
// - `termination`: null when no launch started, else { resolved, reason };
// - `lockKept`: the worktree lock was kept because a build process may survive;
// - `tripwire`: null before the baseline, else { before, after, done } where
//   `done` says the after-last comparison already ran in the run's checks.
// Returns { exitCode, lines, retain, afterLast }: the exit code, raised to 1 by
// a tripwire change or unresolved termination and never lowered; the named
// lines to print; whether the home, the private copy, the marker and the lock
// are retained; and whether the snapshot is the after-last observation.
export function closeRun({ exitCode, termination, lockKept, tripwire }) {
  const lines = [];
  const unresolved = (termination !== null && !termination.resolved) || lockKept;
  let code = exitCode;
  const fail = () => { if (code === 0) code = 1; };
  if (termination !== null && !termination.resolved) lines.push(`UNRESOLVED TERMINATION: ${termination.reason}`);
  if (lockKept) lines.push("UNRESOLVED TERMINATION: the worktree build lock was kept because a process of the run may survive");
  let afterLast = false;
  if (tripwire !== null && !tripwire.done) {
    const changes = tripwireChanges(tripwire.before, tripwire.after);
    afterLast = !unresolved;
    if (afterLast) lines.push(changes.length === 0 ? "tripwire, after the last launch: no watched path appeared, disappeared or changed" : "tripwire, after the last launch:");
    else lines.push("tripwire snapshot taken while a process of the run may still be alive; it is recorded, and it is not the after-last observation:");
    lines.push(...changes);
    if (changes.length > 0) fail();
  }
  if (unresolved) fail();
  return { exitCode: code, lines, retain: unresolved, afterLast };
}

// The marker of a run between its first launch and its completed cleanup.
export function writeRunMarker(markerPath, record) {
  mkdirSync(path.dirname(markerPath), { recursive: true });
  writeFileSync(markerPath, `${JSON.stringify(record, null, 2)}\n`);
}

// The marker's record; null when there is none; { unreadable: true } when it
// cannot be parsed.
export function readRunMarker(markerPath) {
  let text;
  try {
    text = readFileSync(markerPath, "utf8");
  } catch (error) {
    if (error.code === "ENOENT") return null;
    return { unreadable: true, error: error.code };
  }
  try {
    return JSON.parse(text);
  } catch {
    return { unreadable: true };
  }
}

// The refusal a later invocation prints when it finds a marker: what was found
// and how to recover. `alive` says whether the recorded runner process still
// runs (same pid and start time); `census` is a fresh census of the recorded
// executable.
export function runMarkerRefusal(markerPath, record, { alive, census }) {
  const quote = (value) => `'${String(value).replace(/'/g, "'\\''")}'`;
  const lines = [`native smoke: EARLIER-RUN-UNRESOLVED: ${markerPath} records a native smoke whose cleanup did not complete (interrupted, killed outright, or its termination unresolved); refusing to start`];
  if (record?.unreadable) {
    lines.push("  the marker cannot be read; inspect it, confirm no smoke or app runs in this worktree, then remove it:");
    lines.push(`  rm -f ${quote(markerPath)}`);
    return lines;
  }
  lines.push(`  runner: pid ${record.pid}, started ${record.started} (${alive ? "STILL RUNNING" : "not running"})`);
  if (record.reason) lines.push(`  recorded: ${record.reason}`);
  lines.push(`  home: ${record.home}`, `  private app copy: ${record.executable}`, `  evidence root: ${record.evidence_root}`);
  lines.push(census.ok ? `  census of the private copy now: ${census.pids.length === 0 ? "no process" : `process ${census.pids.join(", ")}`}` : `  census of the private copy now: could not be taken (${census.reason})`);
  lines.push("Once no process runs the private copy (the census above shows none) and no smoke runs in this worktree, remove what the run left, then the marker:");
  lines.push(`  rm -rf ${quote(record.home)}`, `  rm -rf ${quote(path.dirname(record.executable))}`, `  rm -f ${quote(markerPath)}`);
  lines.push("and, if the worktree build lock is still held, follow the steps its refusal prints.");
  return lines;
}
