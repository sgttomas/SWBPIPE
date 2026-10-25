import { test } from 'node:test';
import assert from 'node:assert/strict';
import { spawn, spawnSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, existsSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { BUILD_LOCK_TOKEN_ENV, acquireBuildLock, buildLockPath, processStartTime, refuseHostSlotCommand } from './worktreeBuildLock.mjs';

const lockModule = new URL('./worktreeBuildLock.mjs', import.meta.url).href;

function tempRoot() {
  const root = realpathSync(mkdtempSync(path.join(tmpdir(), 'piping-build-lock-')));
  return { root, cleanup: () => rmSync(root, { recursive: true, force: true }) };
}

// A separate process that takes the lock and holds it until killed.
async function startHolder(root) {
  const child = spawn(process.execPath, ['--input-type=module', '-e', `
    import { acquireBuildLock } from ${JSON.stringify(lockModule)};
    acquireBuildLock({ purpose: 'holder', root: ${JSON.stringify(root)}, waitMs: 1000 });
    console.log('held');
    setInterval(() => {}, 1000);
  `], { stdio: ['ignore', 'pipe', 'inherit'] });
  await new Promise((resolve, reject) => {
    child.stdout.on('data', (chunk) => { if (String(chunk).includes('held')) resolve(); });
    child.on('exit', (code) => reject(new Error(`holder exited ${code}`)));
  });
  return child;
}

function stop(child) {
  return new Promise((resolve) => { child.on('exit', resolve); child.kill('SIGKILL'); });
}

test('a second taker waits for a live holder, then refuses by name after the bound; the live holder is never broken', async () => {
  const t = tempRoot();
  const holder = await startHolder(t.root);
  try {
    const lockPath = buildLockPath(t.root);
    const before = readFileSync(lockPath, 'utf8');
    const lines = [];
    const start = Date.now();
    assert.throws(
      () => acquireBuildLock({ purpose: 'second', root: t.root, waitMs: 800, log: (line) => lines.push(line) }),
      new RegExp(`refusing to build: the build lock .*build\\.lock is still held by pid ${holder.pid} \\(holder, started .+\\) after waiting 1 s`)
    );
    const waited = Date.now() - start;
    assert.ok(waited >= 800 && waited < 10000, `waited ${waited} ms`);
    assert.deepEqual(lines.map((line) => line.replace(/since [^)]+/, 'since T')), [`[build-lock] waiting for the build lock held by pid ${holder.pid} (holder, since T); up to 1 s`]);
    assert.equal(readFileSync(lockPath, 'utf8'), before, 'the live holder keeps its lock');
    assert.equal(holder.exitCode, null, 'the holder still runs');
  } finally { await stop(holder); t.cleanup(); }
});

test('a lock whose pid is not running is broken with a log line', () => {
  const t = tempRoot();
  try {
    const dead = spawnSync(process.execPath, ['-e', '']).pid;
    assert.equal(processStartTime(dead), null);
    const lockPath = buildLockPath(t.root);
    mkdirSync(path.dirname(lockPath), { recursive: true });
    writeFileSync(lockPath, JSON.stringify({ pid: dead, started: 'Sat Oct 25 00:00:00 2026', purpose: 'dead', token: 'x', acquired: 'T' }));
    const lines = [];
    const lock = acquireBuildLock({ purpose: 'next', root: t.root, waitMs: 1000, log: (line) => lines.push(line) });
    assert.deepEqual(lines, [`[build-lock] breaking the stale build lock ${lockPath} (pid ${dead} is not running)`]);
    const record = JSON.parse(readFileSync(lockPath, 'utf8'));
    assert.equal(record.pid, process.pid);
    assert.equal(record.token, lock.token);
    lock.release();
    assert.ok(!existsSync(lockPath));
  } finally { t.cleanup(); }
});

test('a lock whose pid was reused (running, another start time) is broken with a log line', async () => {
  const t = tempRoot();
  const other = spawn(process.execPath, ['-e', 'setInterval(() => {}, 1000)'], { stdio: 'ignore' });
  try {
    const lockPath = buildLockPath(t.root);
    mkdirSync(path.dirname(lockPath), { recursive: true });
    writeFileSync(lockPath, JSON.stringify({ pid: other.pid, started: 'Thu Jan  1 00:00:00 1970', purpose: 'earlier', token: 'x', acquired: 'T' }));
    const lines = [];
    const lock = acquireBuildLock({ purpose: 'next', root: t.root, waitMs: 1000, log: (line) => lines.push(line) });
    assert.equal(lines.length, 1);
    assert.match(lines[0], new RegExp(`breaking the stale build lock .* \\(pid ${other.pid} was reused: it started .+, the lock records Thu Jan  1 00:00:00 1970\\)`));
    assert.equal(other.exitCode, null);
    lock.release();
  } finally { await stop(other); t.cleanup(); }
});

test('a child inherits the lock only through the token the lock holds; holding it, a host-slot command is refused by name', () => {
  const t = tempRoot();
  try {
    assert.doesNotThrow(() => refuseHostSlotCommand('/opt/tools/t3_cargo.sh'), 'nothing is refused without the lock');
    const lock = acquireBuildLock({ purpose: 'parent', root: t.root, waitMs: 1000, log: () => {} });
    for (const [command, args, name] of [
      ['/opt/tools/t3_slot.sh', ['cargo', 'build'], 't3_slot.sh'],
      ['bash', ['/opt/tools/t3_exclusive.sh', 'npm'], 't3_exclusive.sh']
    ]) {
      assert.throws(() => refuseHostSlotCommand(command, args), new RegExp(`refusing to run ${command}.* while holding the worktree build lock .*build\\.lock: it would acquire a host slot \\(${name.replace('.', '\\.')}\\)`));
    }
    // A command on PATH that is a script naming a slot wrapper.
    const bin = path.join(t.root, 'bin');
    mkdirSync(bin);
    writeFileSync(path.join(bin, 'cargo'), '#!/bin/sh\nexec /opt/tools/t3_cargo.sh "$@"\n', { mode: 0o755 });
    assert.throws(() => refuseHostSlotCommand('cargo', [], { PATH: bin }), /refusing to run cargo \(.*bin\/cargo\) .*host slot \(t3_cargo\.sh\)/);
    assert.doesNotThrow(() => refuseHostSlotCommand(process.execPath, ['--version']));
    lock.release();
    // Inheritance: only the token the lock file holds.
    const parent = acquireBuildLock({ purpose: 'parent', root: t.root, waitMs: 1000, log: () => {} });
    const childScript = (token) => spawnSync(process.execPath, ['--input-type=module', '-e', `
      import { acquireBuildLock } from ${JSON.stringify(lockModule)};
      try { const lock = acquireBuildLock({ purpose: 'child', root: ${JSON.stringify(t.root)}, waitMs: 300, log: () => {} }); console.log(lock.inherited ? 'inherited' : 'own'); lock.release(); }
      catch (error) { console.log(error.message); }
    `], { encoding: 'utf8', env: { ...process.env, [BUILD_LOCK_TOKEN_ENV]: token } }).stdout.trim();
    assert.equal(childScript(parent.token), 'inherited');
    assert.ok(existsSync(buildLockPath(t.root)), 'an inheriting child does not release the parent lock');
    assert.match(childScript('another'), new RegExp(`refusing to build: ${BUILD_LOCK_TOKEN_ENV} names a build lock that .*build\\.lock does not hold`));
    parent.release();
    assert.ok(!existsSync(buildLockPath(t.root)));
  } finally { t.cleanup(); }
});
