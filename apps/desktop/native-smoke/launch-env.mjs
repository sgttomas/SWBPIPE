// What a native-smoke launch of the desktop app is given, and what it must not
// reach (DEL-07-11 VER-003; `run.mjs`).
//
// The launched application's environment is built from nothing: only
// variables on PARENT_ALLOWLIST are carried from the runner's environment;
// then HOME and CFFIXED_USER_HOME, both the throwaway home; TMPDIR, a folder
// inside that home; and the runner's own SWBPIPE_NATIVE_SMOKE_* variables.
// `refusedLaunchEnv` names every reason the runner must not launch with a
// built environment. The home is created under one root and removed by the
// runner on exit. The tripwire stats a fixed list of product-named paths under
// the real user's Library before the first launch and after the last; it lists
// no folder and reads no file.
import { lstatSync, mkdirSync, mkdtempSync, realpathSync, statSync } from "node:fs";
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

// The runner's own variables (`native_smoke.rs` reads each).
export const RUNNER_KEYS = Object.freeze([
  "SWBPIPE_NATIVE_SMOKE_DRIVER", // the script evaluated in the main webview
  "SWBPIPE_NATIVE_SMOKE_PLAN", // the JSON plan handed to the driver
  "SWBPIPE_NATIVE_SMOKE_REPORT" // where native_smoke_finish writes the report
]);

// The home variables and the temporary folder the runner sets itself.
export const HOME_KEYS = Object.freeze(["HOME", "CFFIXED_USER_HOME", "TMPDIR"]);

// The folder inside the home given to the app as TMPDIR.
const TMP_NAME = "tmp";

// The throwaway home: a new folder under `homeRoot` (an existing folder), with
// TMPDIR's folder created inside it. Both are returned resolved (no links), so
// the app and the runner judge the same names.
export function createThrowawayHome(homeRoot) {
  let root;
  try {
    root = realpathSync(homeRoot);
  } catch {
    throw new Error(`native smoke: --home-root ${homeRoot} does not exist`);
  }
  if (!statSync(root).isDirectory()) throw new Error(`native smoke: --home-root ${homeRoot} is not a folder`);
  const home = realpathSync(mkdtempSync(path.join(root, "swbpipe-native-smoke-home-")));
  const tmpdir = path.join(home, TMP_NAME);
  mkdirSync(tmpdir);
  mkdirSync(path.join(home, "Downloads"));
  return { home, tmpdir };
}

// The launched application's whole environment.
export function buildLaunchEnv({ parent, home, tmpdir, runner }) {
  const env = {};
  for (const key of PARENT_ALLOWLIST) if (Object.hasOwn(parent, key)) env[key] = parent[key];
  env.HOME = home;
  env.CFFIXED_USER_HOME = home;
  env.TMPDIR = tmpdir;
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
  return reasons;
}

// Whether `child` is a folder strictly below `parent`, both absolute, by name.
function insideFolder(child, parent) {
  if (typeof child !== "string" || typeof parent !== "string" || !path.isAbsolute(child) || !path.isAbsolute(parent)) return false;
  const relative = path.relative(parent, child);
  return relative !== "" && !relative.startsWith("..") && !path.isAbsolute(relative);
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
