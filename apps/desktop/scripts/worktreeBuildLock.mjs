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
// Interruption. The holder runs its build commands (and the smoke its app
// launches) through `runUnderLock`, asynchronously, each as the leader of its
// own process group, so cargo's rustc, build-script and wasm-bindgen
// descendants are in that group. The stop handlers are installed before the
// lock is published. On SIGINT, SIGTERM or SIGHUP the holder sends the signal
// to every such group, waits (bounded) until each group is empty, sends SIGKILL
// to any group that is not and waits again, and only when every group is gone
// releases its lock; it then exits 128 + the signal number. A group still not
// empty after SIGKILL keeps the lock, and the holder prints the removal steps
// (written synchronously, so exiting cannot drop them). Each command's group
// is settled before the command returns: after its leader exits the group gets
// a bounded wait, then SIGTERM, then SIGKILL, and is forgotten once seen empty,
// so no later signal can reach a reused group id; a group that cannot be
// confirmed empty stops the run with the lock kept. A signal arriving during
// synchronous work (`ps`, `rustc -vV`, `cargo metadata`) is handled when the
// event loop next runs: at the next command, or at `exitAfterSignals`, which
// the tooling exits through. The process exiting otherwise (uncaught) with a
// group not yet empty sends that group SIGKILL and keeps the lock. SIGKILL of
// the holder, or a power loss, cannot be handled: the lock stays, and recovery
// is the removal steps the refusal prints.
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
// costs those three commands. When the holder was a native smoke whose private
// app copy (`target/native-smoke/run-<token>`, about 115 MB) is still there,
// the refusal also names that one directory and its `rm -rf`, to run once the
// lock is removed and nothing runs from it; never a blanket `run-*` delete.
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

import { execFileSync, spawn } from "node:child_process";
import { randomUUID } from "node:crypto";
import { closeSync, existsSync, linkSync, lstatSync, mkdirSync, openSync, readFileSync, readSync, realpathSync, rmSync, statSync, writeFileSync, writeSync } from "node:fs";
import { constants as osConstants } from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

export const BUILD_LOCK_TOKEN_ENV = "OPENPIPESTRESS_BUILD_LOCK_TOKEN";
// Every spelling cargo reads a compiler wrapper from in the environment.
export const COMPILER_WRAPPER_ENV = ["RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER", "CARGO_BUILD_RUSTC_WRAPPER", "CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER"];

const desktopRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
export const buildLockPath = (root = desktopRoot) => path.join(root, "target", "build.lock");

// The lock this process holds or inherited, if any.
let held = null;
// Whether this process last kept its lock because a build process may survive.
let kept = false;
// The signal being handled (or "unconfirmed"), once the holder is stopping.
let stopping = null;
// Whether the exit and stop-signal handlers are installed.
let listening = false;
// The process groups of the commands this process started holding its own lock,
// until each is seen empty.
const groups = new Set();

export const STOP_SIGNALS = ["SIGINT", "SIGTERM", "SIGHUP"];
// How long the holder waits for its build process groups to empty after
// forwarding a stop signal, and after SIGKILL.
const STOP_GRACE_MS = 5000;
const KILL_WAIT_MS = 2000;
const POLL_MS = 25;
const TOKEN_FORM = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/;

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

// The steps that confirm nothing is building in this worktree and then remove
// the lock at `lockPath` (and the private app copy a native smoke holding it left).
function removalSteps(lockPath, record, root) {
  const projectRoot = path.resolve(root, "..", "..");
  const readable = record && Number.isInteger(record.pid) && record.pid > 0;
  // The smoke's private launch copy, `target/native-smoke/run-<token>` (appBinaryBinding.mjs).
  const runDirectory = typeof record?.token === "string" && TOKEN_FORM.test(record.token) ? path.join(root, "target", "native-smoke", `run-${record.token}`) : null;
  return [
    ...(readable ? [`  TZ=UTC ps -p ${record.pid} -o pid,lstart,command`] : []),
    `  pgrep -fl -- ${shellQuote(regexLiteral(projectRoot))}`,
    `(${readable ? "ps prints only its header, or a process that is not the holder; " : ""}pgrep prints no cargo, rustc, wasm-bindgen, wasm-opt, build-script, tauri or app process), then remove the lock:`,
    `  rm -f ${shellQuote(lockPath)}`,
    ...(runDirectory && existsSync(runDirectory)
      ? [`The native smoke that held the lock left its private app copy. With the lock removed and nothing running from it (pgrep above prints no process from it), remove that directory, and no other run-* directory:`, `  rm -rf ${shellQuote(runDirectory)}`]
      : [])
  ];
}

// The refusal for a held lock: the holder, the hint and the exact removal steps.
export function heldLockMessage(lockPath, record, root = desktopRoot) {
  const readable = record && Number.isInteger(record.pid) && record.pid > 0;
  const holder = readable
    ? `pid ${record.pid} (${record.purpose}), started ${record.started} (UTC, by ps), acquired ${record.acquired}`
    : "unreadable lock record";
  return [
    `refusing to build: the worktree build lock ${lockPath} is held by ${holder}; one build or smoke per worktree runs at a time, and a second is refused, not queued`,
    `  hint from ps (informational; the lock is never broken automatically): ${livenessHint(record)}`,
    "If that build is still running, let it finish and run again. If a crashed run left the lock, confirm that nothing is building in this worktree:",
    ...removalSteps(lockPath, record, root)
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

// Whether any process is in group `pgid`. EPERM counts as alive: a member this
// user may not signal, or (macOS) a group left with only an unreaped zombie.
function groupAlive(pgid) {
  try {
    process.kill(-pgid, 0);
    return true;
  } catch (error) {
    return error.code !== "ESRCH";
  }
}

// Signal group `pgid` only while it is one of ours (tracked) and seen alive
// just before: a group forgotten once empty is never signalled, whatever later
// reuses its id.
function signalLiveGroup(pgid, signal) {
  if (!groups.has(pgid) || !groupAlive(pgid)) return;
  try {
    process.kill(-pgid, signal);
  } catch {
    // Gone meanwhile: groupAlive decides.
  }
}

// Write `text` to standard error synchronously, retrying (bounded) while the
// pipe is full, so that exiting at once cannot drop it.
function writeErrorSync(text) {
  const bytes = Buffer.from(`${text}\n`);
  const deadline = Date.now() + 2000;
  for (let offset = 0; offset < bytes.length; ) {
    try {
      offset += writeSync(2, bytes, offset);
    } catch (error) {
      if (error.code !== "EAGAIN" || Date.now() > deadline) return;
      Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 5);
    }
  }
}

const pause = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

// Wait up to `ms` for the groups `pgids` to be empty, forgetting each one seen
// empty; true when all are.
async function groupsEmptied(pgids, ms) {
  const deadline = Date.now() + ms;
  for (;;) {
    for (const pgid of pgids) if (!groupAlive(pgid)) groups.delete(pgid);
    if (pgids.every((pgid) => !groups.has(pgid))) return true;
    if (Date.now() >= deadline) return false;
    await pause(POLL_MS);
  }
}

function listen() {
  if (listening) return;
  listening = true;
  process.on("exit", releaseHeld);
  for (const signal of STOP_SIGNALS) process.on(signal, onStopSignal);
}

function unlisten() {
  if (!listening) return;
  listening = false;
  process.off("exit", releaseHeld);
  for (const signal of STOP_SIGNALS) process.off(signal, onStopSignal);
}

// Holding nothing now, stop listening once the event loop has run: a signal
// received during synchronous work and not yet delivered still reaches
// onStopSignal, which honours it.
function forget() {
  held = null;
  setImmediate(() => {
    if (held === null) unlisten();
  });
}

// Keep the lock: a build process may survive. Prints the removal steps.
function keepHeld(lock, reason) {
  forget();
  kept = true;
  writeErrorSync([
    `${reason}; the worktree build lock ${lock.path} is kept, since a build process may survive. Once they have exited, confirm that nothing is building in this worktree:`,
    ...removalSteps(lock.path, lock.record, lock.root)
  ].join("\n"));
}

function releaseHeld() {
  if (held === null) return;
  const lock = held;
  if (!lock.inherited) {
    for (const pgid of groups) if (!groupAlive(pgid)) groups.delete(pgid);
    const alive = [...groups];
    if (alive.length > 0) {
      for (const pgid of alive) signalLiveGroup(pgid, "SIGKILL");
      keepHeld(lock, `process group ${alive.join(", ")} of this build was still running at release and was sent SIGKILL`);
      return;
    }
  }
  forget();
  if (lock.inherited) return;
  try {
    if (readLock(lock.path)?.record?.token === lock.token) rmSync(lock.path, { force: true });
  } catch {
    // Nothing more to release.
  }
}

// SIGINT, SIGTERM or SIGHUP while holding the lock: stop and reap the build
// process groups, release only when every one is gone, and exit 128 + signo.
function onStopSignal(signal) {
  if (stopping !== null) return;
  stopping = signal;
  // Released (or kept) already, and no command runs: honour the signal.
  if (held === null) {
    unlisten();
    process.exit(128 + osConstants.signals[signal]);
  }
  const lock = held;
  const pgids = [...groups];
  for (const pgid of pgids) signalLiveGroup(pgid, signal);
  (async () => {
    let gone = await groupsEmptied(pgids, lock.stopGraceMs);
    if (!gone) {
      for (const pgid of pgids) signalLiveGroup(pgid, "SIGKILL");
      gone = await groupsEmptied(pgids, lock.killWaitMs);
    }
    if (gone) releaseHeld();
    else keepHeld(lock, `${signal}: process group ${pgids.filter((id) => groups.has(id)).join(", ")} of this build did not empty after ${signal} and SIGKILL`);
    process.exit(128 + osConstants.signals[signal]);
  })();
}

// Exit with `code` once a signal already received has been handled: a signal
// that arrived during synchronous work is delivered when the event loop next
// runs, and its handler then decides the exit (128 + the signal number).
export async function exitAfterSignals(code) {
  await new Promise((resolve) => setImmediate(resolve));
  if (stopping === null) process.exit(code);
  return new Promise(() => {});
}

// After a command's leader has exited, settle its group before going on: a
// bounded wait for it to empty, then SIGTERM, then SIGKILL, each with the same
// wait. True once it is seen empty (and forgotten).
async function settleGroup(pgid, waitMs) {
  for (const signal of [null, "SIGTERM", "SIGKILL"]) {
    if (signal !== null) signalLiveGroup(pgid, signal);
    if (await groupsEmptied([pgid], waitMs)) return true;
  }
  return false;
}

// Run `command` under the lock, asynchronously, so a stop signal is handled
// while it runs. Holding the lock itself, the command leads its own process
// group, which the stop handler signals whole and waits to see empty; under an
// inherited lock (or none) it stays in this process's group, which the holder
// above signals. `timeoutMs` sends SIGKILL to the command (its group) once
// passed. Resolves with { status, signal, stdout, stderr, error, timedOut } once
// the command has closed and, holding the lock, its group is settled. A group
// that cannot be confirmed empty keeps the lock, prints the removal steps and
// ends the process (exit 1). Never settles once a stop signal is being handled:
// the handler ends the process.
export function runUnderLock(command, args = [], { cwd, env = process.env, stdio = ["ignore", "pipe", "pipe"], timeoutMs } = {}) {
  if (stopping !== null) return new Promise(() => {});
  const own = held !== null && !held.inherited;
  const killWaitMs = held?.killWaitMs ?? KILL_WAIT_MS;
  return new Promise((resolve) => {
    const child = spawn(command, args, { cwd, env, stdio, detached: own });
    const pgid = own && child.pid ? child.pid : null;
    if (pgid !== null) groups.add(pgid);
    const out = [];
    const err = [];
    child.stdout?.on("data", (chunk) => out.push(chunk));
    child.stderr?.on("data", (chunk) => err.push(chunk));
    let error = null;
    let timedOut = false;
    child.on("error", (failure) => { error = failure; });
    const timer = timeoutMs
      ? setTimeout(() => {
          timedOut = true;
          if (pgid !== null) signalLiveGroup(pgid, "SIGKILL");
          else child.kill("SIGKILL");
        }, timeoutMs)
      : null;
    child.on("close", async (status, signal) => {
      if (timer) clearTimeout(timer);
      if (pgid !== null && !(await settleGroup(pgid, killWaitMs)) && stopping === null && held !== null) {
        stopping = "unconfirmed";
        keepHeld(held, `process group ${pgid} of ${command} did not empty after ${command} exited, SIGTERM and SIGKILL`);
        process.exit(1);
      }
      if (stopping !== null) return;
      resolve({ status: error ? null : status, signal, stdout: Buffer.concat(out).toString("utf8"), stderr: Buffer.concat(err).toString("utf8"), error, timedOut });
    });
  });
}

// Take the worktree build lock for `purpose`, or refuse at once when it is held.
// Returns { token, inherited, release, kept }; `kept()` is true once the lock
// was kept because a build process may survive. `stopGraceMs` and `killWaitMs`
// bound the waits after a stop signal and after SIGKILL. `onPublished` (tests)
// runs as soon as the lock is published, before this returns.
export function acquireBuildLock({ purpose, root = desktopRoot, env = process.env, stopGraceMs = STOP_GRACE_MS, killWaitMs = KILL_WAIT_MS, onPublished = null }) {
  if (held !== null) throw new Error(`refusing to build: this process already holds the build lock ${held.path}`);
  const lockPath = buildLockPath(root);
  const inheritedToken = env[BUILD_LOCK_TOKEN_ENV];
  if (inheritedToken) {
    if (readLock(lockPath)?.record?.token !== inheritedToken) {
      throw new Error(`refusing to build: ${BUILD_LOCK_TOKEN_ENV} names a build lock that ${lockPath} does not hold`);
    }
    held = { path: lockPath, token: inheritedToken, inherited: true };
    kept = false;
    return { token: inheritedToken, inherited: true, release: releaseHeld, kept: () => kept };
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
  // Cleanup and signal protection first: from the moment the lock is published
  // an exit or a stop signal releases it.
  kept = false;
  stopping = null;
  listen();
  try {
    writeFileSync(pending, `${JSON.stringify(record)}\n`, { flag: "wx" });
    try {
      for (;;) {
        try {
          linkSync(pending, lockPath);
          held = { path: lockPath, token, inherited: false, record, root, stopGraceMs, killWaitMs };
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
  } catch (error) {
    // Not taken: nothing to release; stop listening (a signal received meanwhile is still honoured).
    if (held === null) forget();
    throw error;
  }
  onPublished?.();
  return { token, inherited: false, release: releaseHeld, kept: () => kept };
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
