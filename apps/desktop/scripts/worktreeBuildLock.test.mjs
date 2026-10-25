import { test } from 'node:test';
import assert from 'node:assert/strict';
import { spawn, spawnSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, existsSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { BUILD_LOCK_TOKEN_ENV, acquireBuildLock, buildLockPath, processStartTime, refuseCompilerWrapperEnv, refuseHostSlotCommand, shellQuote } from './worktreeBuildLock.mjs';

const lockModule = new URL('./worktreeBuildLock.mjs', import.meta.url).href;

function tempRoot() {
  const root = realpathSync(mkdtempSync(path.join(tmpdir(), 'piping-build-lock-')));
  return { root, cleanup: () => rmSync(root, { recursive: true, force: true }) };
}

// A separate process that takes the lock and holds it until killed.
async function startHolder(root) {
  const child = spawn(process.execPath, ['--input-type=module', '-e', `
    import { acquireBuildLock } from ${JSON.stringify(lockModule)};
    acquireBuildLock({ purpose: 'holder', root: ${JSON.stringify(root)} });
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

// The refusal's literal removal steps for a lock at `lockPath` held by `pid`.
function removalSteps(root, lockPath, pid) {
  const pattern = path.resolve(root, '..', '..').replace(/[\\^$.|?*+()[\]{}]/g, '\\$&');
  return [`  TZ=UTC ps -p ${pid} -o pid,lstart,command`, `  pgrep -fl -- '${pattern}'`, `  rm -f '${lockPath}'`];
}

test('a held lock with a live holder is refused at once, by name, with the holder and the literal removal steps; nothing is broken', async () => {
  const t = tempRoot();
  const holder = await startHolder(t.root);
  try {
    const lockPath = buildLockPath(t.root);
    const before = readFileSync(lockPath, 'utf8');
    const recorded = JSON.parse(before);
    const start = Date.now();
    let message = null;
    try { acquireBuildLock({ purpose: 'second', root: t.root }); } catch (error) { message = error.message; }
    assert.ok(Date.now() - start < 5000, 'refused without waiting');
    assert.ok(message, 'the second taker is refused');
    console.log(`held-lock refusal:\n${message}`);
    const lines = message.split('\n');
    assert.equal(lines[0], `refusing to build: the worktree build lock ${lockPath} is held by pid ${holder.pid} (holder), started ${recorded.started} (UTC, by ps), acquired ${recorded.acquired}; one build or smoke per worktree runs at a time, and a second is refused, not queued`);
    assert.equal(lines[1], `  hint from ps (informational; the lock is never broken automatically): pid ${holder.pid} is running and started at the recorded time: the holder looks alive`);
    const [ps, pgrep, rm] = removalSteps(t.root, lockPath, holder.pid);
    assert.deepEqual([lines[3], lines[4], lines.at(-1)], [ps, pgrep, rm]);
    assert.equal(readFileSync(lockPath, 'utf8'), before, 'the live holder keeps its lock');
    assert.equal(holder.exitCode, null, 'the holder still runs');
    // The recorded start time is ps's in UTC, whatever this process's TZ.
    const local = spawnSync('/bin/ps', ['-o', 'lstart=', '-p', String(holder.pid)], { encoding: 'utf8', env: { ...process.env, TZ: 'UTC', LC_ALL: 'C' } }).stdout.trim();
    assert.equal(recorded.started, local);
    // The printed pgrep form runs, and finds the holder (its command line names this tree).
    const found = spawnSync('/bin/sh', ['-c', pgrep.trim()], { encoding: 'utf8' }).stdout;
    assert.match(found, new RegExp(`^${holder.pid} `, 'm'));
  } finally { await stop(holder); t.cleanup(); }
});

test('a held lock whose pid is not running is still refused, not broken, and the lock file is untouched', () => {
  const t = tempRoot();
  try {
    const dead = spawnSync(process.execPath, ['-e', '']).pid;
    assert.equal(processStartTime(dead), null);
    const lockPath = buildLockPath(t.root);
    mkdirSync(path.dirname(lockPath), { recursive: true });
    const text = JSON.stringify({ pid: dead, started: 'Sat Oct 25 00:00:00 2026', purpose: 'dead', token: 'x', acquired: 'T' });
    writeFileSync(lockPath, text);
    assert.throws(() => acquireBuildLock({ purpose: 'next', root: t.root }), (error) => {
      const lines = error.message.split('\n');
      assert.match(lines[0], new RegExp(`^refusing to build: the worktree build lock .*build\\.lock is held by pid ${dead} \\(dead\\), started Sat Oct 25 00:00:00 2026 \\(UTC, by ps\\), acquired T;`));
      assert.equal(lines[1], `  hint from ps (informational; the lock is never broken automatically): pid ${dead} is not running; a build process it started may still be`);
      assert.equal(lines.at(-1), `  rm -f '${lockPath}'`);
      return true;
    });
    assert.equal(readFileSync(lockPath, 'utf8'), text, 'the lock file is untouched');
    // A reused pid (running, another start time) and an unreadable record are refused too.
    writeFileSync(lockPath, JSON.stringify({ pid: process.pid, started: 'Thu Jan  1 00:00:00 1970', purpose: 'earlier', token: 'x', acquired: 'T' }));
    assert.throws(() => acquireBuildLock({ purpose: 'next', root: t.root }), new RegExp(`pid ${process.pid} is running but started .+ \\(UTC\\), not at the recorded time: the pid looks reused`));
    writeFileSync(lockPath, 'not json');
    assert.throws(() => acquireBuildLock({ purpose: 'next', root: t.root }), /is held by unreadable lock record;[\s\S]*the lock record is unreadable[\s\S]*pgrep -fl -- [\s\S]*rm -f '/);
    assert.equal(readFileSync(lockPath, 'utf8'), 'not json');
    // Removed by the printed step, the lock is taken.
    rmSync(lockPath);
    const lock = acquireBuildLock({ purpose: 'next', root: t.root });
    assert.equal(JSON.parse(readFileSync(lockPath, 'utf8')).token, lock.token);
    lock.release();
    assert.ok(!existsSync(lockPath));
  } finally { t.cleanup(); }
});

test('the removal step quotes the lock path for zsh and bash', () => {
  assert.equal(shellQuote("/tmp/it's here/build.lock"), `'/tmp/it'\\''s here/build.lock'`);
  for (const shell of ['/bin/sh', '/bin/zsh', '/bin/bash'].filter((file) => existsSync(file))) {
    const echoed = spawnSync(shell, ['-c', `printf %s ${shellQuote("/tmp/it's $HOME `x` here")}`], { encoding: 'utf8' }).stdout;
    assert.equal(echoed, "/tmp/it's $HOME `x` here", shell);
  }
});

test('each inherited compiler wrapper variable is refused by name; none set passes', () => {
  assert.doesNotThrow(() => refuseCompilerWrapperEnv({ PATH: '/usr/bin', RUSTC: 'rustc', RUSTC_WRAPPER: '' }));
  for (const name of ['RUSTC_WRAPPER', 'RUSTC_WORKSPACE_WRAPPER', 'CARGO_BUILD_RUSTC_WRAPPER', 'CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER']) {
    assert.throws(() => refuseCompilerWrapperEnv({ [name]: '/opt/tools/t3_slot.sh' }), new RegExp(`^Error: refusing to run cargo: ${name} is set \\("/opt/tools/t3_slot\\.sh"\\);.* unset ${name} and run again$`), name);
  }
});

test('a child inherits the lock only through the token the lock holds; holding it, a host-slot command is refused by name', () => {
  const t = tempRoot();
  try {
    assert.doesNotThrow(() => refuseHostSlotCommand('/opt/tools/t3_cargo.sh'), 'nothing is refused without the lock');
    const lock = acquireBuildLock({ purpose: 'parent', root: t.root });
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
    const parent = acquireBuildLock({ purpose: 'parent', root: t.root });
    const childScript = (token) => spawnSync(process.execPath, ['--input-type=module', '-e', `
      import { acquireBuildLock } from ${JSON.stringify(lockModule)};
      try { const lock = acquireBuildLock({ purpose: 'child', root: ${JSON.stringify(t.root)} }); console.log(lock.inherited ? 'inherited' : 'own'); lock.release(); }
      catch (error) { console.log(error.message); }
    `], { encoding: 'utf8', env: { ...process.env, [BUILD_LOCK_TOKEN_ENV]: token } }).stdout.trim();
    assert.equal(childScript(parent.token), 'inherited');
    assert.ok(existsSync(buildLockPath(t.root)), 'an inheriting child does not release the parent lock');
    assert.match(childScript('another'), new RegExp(`refusing to build: ${BUILD_LOCK_TOKEN_ENV} names a build lock that .*build\\.lock does not hold`));
    parent.release();
    assert.ok(!existsSync(buildLockPath(t.root)));
  } finally { t.cleanup(); }
});
