// The per-worktree build lock: one exclusive holder at a time for the engine
// check, build, glue and publication (`build-wasm-engine.mjs`) and for the
// native smoke's engine and app check, build and binding (`native-smoke/run.mjs`),
// so two builds in one worktree never interleave. One build or smoke per
// worktree at a time is the supported use; a second is refused, not queued.
//
// The lock is `apps/desktop/target/build.lock` (git-ignored), created
// atomically: the holder's record (pid, the process start time `ps` reports in
// UTC, purpose, a random token, the time it was acquired) is written to a
// private file and hard-linked to the lock path, which fails if the lock exists,
// so the lock always holds a complete record. Released by its holder (only while
// it still holds its own token) and when the process exits.
//
// The lock fails closed: it is never broken automatically. A taker that finds
// it held refuses at once, by name, with the holder's pid, purpose, recorded
// start and acquired times, a liveness hint from `ps` (informational only:
// a pid's start time or absence does not show that no build process of the
// holder survives), and the exact steps that confirm nothing is building in
// this worktree (`ps` of the holder pid, `pgrep -fl` of the processes whose
// command line names this project's directory: rustc, wasm-bindgen and build
// scripts carry it in their paths, and every cargo the tooling runs names a
// manifest in it) and then remove the lock (`rm -f` of its path). A crashed run
// costs those three commands.
//
// A child the holder runs (the smoke running `build:wasm`) inherits the lock
// through OPENPIPESTRESS_BUILD_LOCK_TOKEN, which must name the token the lock
// file holds.
//
// Lock order. The host slot (`t3_slot.sh`, `t3_cargo.sh`, `t3_exclusive.sh`)
// is taken first, by the wrapper that launches the build or the smoke; the
// worktree lock second. Code holding the worktree lock never acquires a host
// slot: `refuseHostSlotCommand` refuses, by name, a command that resolves to
// one of those wrappers or to a script naming them, and
// `refuseCompilerWrapperEnv` refuses, by name, an inherited compiler wrapper
// (RUSTC_WRAPPER and the like), which cargo would run unseen for every
// compile; the tooling never sets one.

import { execFileSync } from "node:child_process";
import { randomUUID } from "node:crypto";
import { closeSync, existsSync, linkSync, lstatSync, mkdirSync, openSync, readFileSync, readSync, realpathSync, rmSync, statSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

export const BUILD_LOCK_TOKEN_ENV = "OPENPIPESTRESS_BUILD_LOCK_TOKEN";
// Every spelling cargo reads a compiler wrapper from in the environment.
export const COMPILER_WRAPPER_ENV = ["RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER", "CARGO_BUILD_RUSTC_WRAPPER", "CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER"];

const desktopRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
export const buildLockPath = (root = desktopRoot) => path.join(root, "target", "build.lock");

// The lock this process holds or inherited, if any.
let held = null;

// The start time `ps` reports for `pid`, in UTC and the C locale (so the same
// process always gives the same text), or null when it is not running.
export function processStartTime(pid) {
  try {
    return execFileSync("/bin/ps", ["-o", "lstart=", "-p", String(pid)], { encoding: "utf8", env: { ...process.env, TZ: "UTC", LC_ALL: "C" }, stdio: ["ignore", "pipe", "ignore"] }).trim() || null;
  } catch {
    return null;
  }
}

// A word for a POSIX shell (zsh, bash): single-quoted.
export const shellQuote = (value) => `'${String(value).replace(/'/g, "'\\''")}'`;
// `value` as a literal in an extended regular expression (pgrep's patterns).
const regexLiteral = (value) => String(value).replace(/[\\^$.|?*+()[\]{}]/g, "\\$&");

// What `ps` says of a recorded holder now. Informational: it never removes anything.
export function livenessHint(record) {
  if (!record || !Number.isInteger(record.pid) || record.pid <= 0) return "the lock record is unreadable, so there is no holder pid to check";
  const started = processStartTime(record.pid);
  if (started === null) return `pid ${record.pid} is not running; a build process it started may still be`;
  if (started === record.started) return `pid ${record.pid} is running and started at the recorded time: the holder looks alive`;
  return `pid ${record.pid} is running but started ${started} (UTC), not at the recorded time: the pid looks reused`;
}

// The refusal for a held lock: the holder, the hint and the exact removal steps.
export function heldLockMessage(lockPath, record, root = desktopRoot) {
  const projectRoot = path.resolve(root, "..", "..");
  const readable = record && Number.isInteger(record.pid) && record.pid > 0;
  const holder = readable
    ? `pid ${record.pid} (${record.purpose}), started ${record.started} (UTC, by ps), acquired ${record.acquired}`
    : "unreadable lock record";
  const confirm = [
    ...(readable ? [`  TZ=UTC ps -p ${record.pid} -o pid,lstart,command`] : []),
    `  pgrep -fl -- ${shellQuote(regexLiteral(projectRoot))}`
  ];
  return [
    `refusing to build: the worktree build lock ${lockPath} is held by ${holder}; one build or smoke per worktree runs at a time, and a second is refused, not queued`,
    `  hint from ps (informational; the lock is never broken automatically): ${livenessHint(record)}`,
    "If that build is still running, let it finish and run again. If a crashed run left the lock, confirm that nothing is building in this worktree:",
    ...confirm,
    `(${readable ? "ps prints only its header, or a process that is not the holder; " : ""}pgrep prints no cargo, rustc, wasm-bindgen, wasm-opt, build-script, tauri or app process), then remove the lock:`,
    `  rm -f ${shellQuote(lockPath)}`
  ].join("\n");
}

// The lock file's record, or null when there is no lock file.
function readLock(lockPath) {
  let text;
  try {
    text = readFileSync(lockPath, "utf8");
  } catch (error) {
    if (error.code === "ENOENT") return null;
    throw error;
  }
  try {
    return { record: JSON.parse(text) };
  } catch {
    return { record: null };
  }
}

function releaseHeld() {
  if (held === null) return;
  const lock = held;
  held = null;
  process.off("exit", releaseHeld);
  if (lock.inherited) return;
  try {
    if (readLock(lock.path)?.record?.token === lock.token) rmSync(lock.path, { force: true });
  } catch {
    // Nothing more to release.
  }
}

// Take the worktree build lock for `purpose`, or refuse at once when it is held.
// Returns { token, inherited, release }.
export function acquireBuildLock({ purpose, root = desktopRoot, env = process.env }) {
  if (held !== null) throw new Error(`refusing to build: this process already holds the build lock ${held.path}`);
  const lockPath = buildLockPath(root);
  const inheritedToken = env[BUILD_LOCK_TOKEN_ENV];
  if (inheritedToken) {
    if (readLock(lockPath)?.record?.token !== inheritedToken) {
      throw new Error(`refusing to build: ${BUILD_LOCK_TOKEN_ENV} names a build lock that ${lockPath} does not hold`);
    }
    held = { path: lockPath, token: inheritedToken, inherited: true };
    return { token: inheritedToken, inherited: true, release: releaseHeld };
  }
  const targetRoot = path.dirname(lockPath);
  for (const step of [path.dirname(targetRoot), targetRoot]) {
    if (lstatSync(step, { throwIfNoEntry: false })?.isSymbolicLink()) throw new Error(`refusing to take the build lock ${lockPath}: ${step} is a symbolic link`);
  }
  mkdirSync(targetRoot, { recursive: true });
  const token = randomUUID();
  const record = { pid: process.pid, started: processStartTime(process.pid), purpose, token, acquired: new Date().toISOString() };
  if (record.started === null) throw new Error(`refusing to take the build lock ${lockPath}: ps does not report this process's start time`);
  const pending = `${lockPath}.${process.pid}.${token}.tmp`;
  writeFileSync(pending, `${JSON.stringify(record)}\n`, { flag: "wx" });
  try {
    for (;;) {
      try {
        linkSync(pending, lockPath);
        break;
      } catch (error) {
        if (error.code !== "EEXIST") throw error;
      }
      const current = readLock(lockPath);
      // Released between the link and the read: try once more.
      if (current === null) continue;
      throw new Error(heldLockMessage(lockPath, current.record, root));
    }
  } finally {
    rmSync(pending, { force: true });
  }
  held = { path: lockPath, token, inherited: false };
  process.on("exit", releaseHeld);
  return { token, inherited: false, release: releaseHeld };
}

// Refuse, by name, an inherited compiler wrapper before any cargo runs. Cargo
// would run it for every compile, out of sight of the stamp and the host-slot
// check: one calling `t3_slot.sh` would take a host slot under the worktree
// build lock. The build tooling never sets one.
export function refuseCompilerWrapperEnv(env = process.env) {
  const name = COMPILER_WRAPPER_ENV.find((key) => env[key]);
  if (name) {
    throw new Error(`refusing to run cargo: ${name} is set (${JSON.stringify(env[name])}); the build tooling runs cargo with no compiler wrapper, and an inherited one would run unseen for every compile (it could take a host slot under the worktree build lock); unset ${name} and run again`);
  }
}

const HOST_SLOT_TOOL = /\bt3_(?:slot|slot_run|cargo|cargo_run|exclusive|exclusive_run|exclusive_stage2)\.sh\b/;
const SCRIPT_SCAN_LIMIT = 1 << 20;

function resolveCommand(command, searchPath) {
  if (command.includes("/")) return path.resolve(command);
  for (const directory of (searchPath ?? "").split(path.delimiter).filter(Boolean)) {
    const candidate = path.join(directory, command);
    try {
      if (statSync(candidate).isFile()) return candidate;
    } catch {
      // Not here.
    }
  }
  return null;
}

// A script's text, when `file` is a script (starts with `#!`) of modest size.
function scriptText(file) {
  try {
    if (statSync(file).size > SCRIPT_SCAN_LIMIT) return null;
    const fd = openSync(file, "r");
    const head = Buffer.alloc(2);
    try {
      readSync(fd, head, 0, 2, 0);
    } finally {
      closeSync(fd);
    }
    return head.toString("latin1") === "#!" ? readFileSync(file, "latin1") : null;
  } catch {
    return null;
  }
}

// While this process holds the build lock, refuse by name to run a command that
// would take a host slot: the slot is taken first, by the wrapper outside.
export function refuseHostSlotCommand(command, args = [], env = process.env) {
  if (held === null) return;
  const resolved = resolveCommand(command, env.PATH);
  const real = resolved && existsSync(resolved) ? realpathSync(resolved) : null;
  const named = [command, resolved, real, ...args].filter(Boolean).find((value) => HOST_SLOT_TOOL.test(path.basename(String(value))));
  const scripted = named ? null : real && scriptText(real)?.match(HOST_SLOT_TOOL)?.[0];
  if (named || scripted) {
    throw new Error(`refusing to run ${command}${resolved ? ` (${resolved})` : ""} while holding the worktree build lock ${held.path}: it would acquire a host slot (${named ? path.basename(String(named)) : scripted}); take the host slot first, outside the build`);
  }
}
