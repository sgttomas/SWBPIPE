// The per-worktree build lock: one exclusive holder at a time for the engine
// check, build, glue and publication (`build-wasm-engine.mjs`) and for the
// native smoke's engine and app check, build and binding (`native-smoke/run.mjs`),
// so two builds in one worktree never interleave.
//
// The lock is `apps/desktop/target/build.lock` (git-ignored), created
// atomically: the holder's record (pid, the process start time `ps` reports,
// purpose, a random token) is written to a private file and hard-linked to the
// lock path, which fails if the lock exists, so the lock always holds a
// complete record. Released by its holder (only while it still holds its own
// token) and when the process exits.
//
// A second taker waits, logging `waiting for the build lock held by pid N`,
// for at most OPENPIPESTRESS_BUILD_LOCK_WAIT_MS (default 30 minutes), then
// refuses by name. A lock is broken, with a log line, only when its pid is not
// running, or is running with a start time other than the recorded one (the
// pid was reused); a live holder is never broken.
//
// A child the holder runs (the smoke running `build:wasm`) inherits the lock
// through OPENPIPESTRESS_BUILD_LOCK_TOKEN, which must name the token the lock
// file holds.
//
// Lock order. The host slot (`t3_slot.sh`, `t3_cargo.sh`, `t3_exclusive.sh`)
// is taken first, by the wrapper that launches the build or the smoke; the
// worktree lock second. Code holding the worktree lock never acquires a host
// slot: `refuseHostSlotCommand` refuses, by name, a command that resolves to
// one of those wrappers or to a script naming them.

import { execFileSync } from "node:child_process";
import { randomUUID } from "node:crypto";
import { closeSync, existsSync, linkSync, lstatSync, mkdirSync, openSync, readFileSync, readSync, realpathSync, renameSync, rmSync, statSync, writeFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

export const BUILD_LOCK_TOKEN_ENV = "OPENPIPESTRESS_BUILD_LOCK_TOKEN";
export const BUILD_LOCK_WAIT_ENV = "OPENPIPESTRESS_BUILD_LOCK_WAIT_MS";
export const DEFAULT_BUILD_LOCK_WAIT_MS = 30 * 60 * 1000;
const POLL_MS = 250;

const desktopRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
export const buildLockPath = (root = desktopRoot) => path.join(root, "target", "build.lock");

// The lock this process holds or inherited, if any.
let held = null;

const sleep = (ms) => Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, ms);

// The start time `ps` reports for `pid`, or null when it is not running.
export function processStartTime(pid) {
  try {
    return execFileSync("/bin/ps", ["-o", "lstart=", "-p", String(pid)], { encoding: "utf8", stdio: ["ignore", "pipe", "ignore"] }).trim() || null;
  } catch {
    return null;
  }
}

function pidRunning(pid) {
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    return error.code === "EPERM";
  }
}

// Why a recorded holder no longer holds the lock, or null while it does.
export function staleReason(record) {
  if (!record || !Number.isInteger(record.pid) || record.pid <= 0 || typeof record.started !== "string") return "the lock record is unreadable";
  if (!pidRunning(record.pid)) return `pid ${record.pid} is not running`;
  const started = processStartTime(record.pid);
  if (started === null) return `pid ${record.pid} is not running`;
  if (started !== record.started) return `pid ${record.pid} was reused: it started ${started}, the lock records ${record.started}`;
  return null;
}

// The lock file's record and inode, or null when there is no lock file.
function readLock(lockPath) {
  let ino;
  let text;
  try {
    ino = lstatSync(lockPath).ino;
    text = readFileSync(lockPath, "utf8");
  } catch (error) {
    if (error.code === "ENOENT") return null;
    throw error;
  }
  let record = null;
  try {
    record = JSON.parse(text);
  } catch {
    record = null;
  }
  return { ino, record };
}

// Remove the stale lock with inode `ino`. It is first moved aside; a lock that
// changed hands in between (another inode) is put back.
function breakLock(lockPath, ino) {
  const aside = `${lockPath}.stale-${process.pid}-${randomUUID()}`;
  try {
    renameSync(lockPath, aside);
  } catch (error) {
    if (error.code === "ENOENT") return;
    throw error;
  }
  if (lstatSync(aside).ino !== ino) {
    try {
      linkSync(aside, lockPath);
    } catch (error) {
      rmSync(aside, { force: true });
      throw new Error(`refusing to build: the build lock ${lockPath} changed hands while a stale lock was broken; run again`);
    }
  }
  rmSync(aside, { force: true });
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

// Take the worktree build lock for `purpose`, waiting at most `waitMs`. Returns
// { token, inherited, release }.
export function acquireBuildLock({ purpose, root = desktopRoot, env = process.env, waitMs, log = (line) => console.log(line) }) {
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
  const bound = waitMs ?? (Number(env[BUILD_LOCK_WAIT_ENV]) > 0 ? Number(env[BUILD_LOCK_WAIT_ENV]) : DEFAULT_BUILD_LOCK_WAIT_MS);
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
  const start = Date.now();
  let waitingFor = null;
  try {
    for (;;) {
      try {
        linkSync(pending, lockPath);
        break;
      } catch (error) {
        if (error.code !== "EEXIST") throw error;
      }
      const current = readLock(lockPath);
      if (current === null) continue;
      const stale = staleReason(current.record);
      if (stale !== null) {
        log(`[build-lock] breaking the stale build lock ${lockPath} (${stale})`);
        breakLock(lockPath, current.ino);
        continue;
      }
      const holder = current.record;
      if (waitingFor !== holder.token) {
        waitingFor = holder.token;
        log(`[build-lock] waiting for the build lock held by pid ${holder.pid} (${holder.purpose}, since ${holder.acquired}); up to ${Math.round(bound / 1000)} s`);
      }
      const waited = Date.now() - start;
      if (waited >= bound) {
        throw new Error(`refusing to build: the build lock ${lockPath} is still held by pid ${holder.pid} (${holder.purpose}, started ${holder.started}) after waiting ${Math.round(waited / 1000)} s`);
      }
      sleep(Math.min(POLL_MS, bound - waited));
    }
  } finally {
    rmSync(pending, { force: true });
  }
  held = { path: lockPath, token, inherited: false };
  process.on("exit", releaseHeld);
  return { token, inherited: false, release: releaseHeld };
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
