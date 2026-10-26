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

// A separate process running `body` (module JavaScript, with the lock module's
// exports and `root` in scope); its standard error is read only once `readAfter`
// resolves, when given.
function startScript(root, body, { readAfter = null } = {}) {
  const child = spawn(process.execPath, ['--input-type=module', '-e', `
    import { acquireBuildLock, exitAfterSignals, runUnderLock } from ${JSON.stringify(lockModule)};
    import { execFileSync, spawn } from 'node:child_process';
    import { openSync, writeFileSync, writeSync } from 'node:fs';
    const root = ${JSON.stringify(root)};
    ${body}
  `], { stdio: ['ignore', 'pipe', 'pipe'] });
  let output = '';
  child.stdout.on('data', (chunk) => { output += chunk; });
  const readError = () => child.stderr.on('data', (chunk) => { output += chunk; });
  if (readAfter) readAfter.then(readError); else readError();
  const closed = new Promise((resolve) => child.on('close', resolve));
  const done = new Promise((resolve) => child.on('exit', (code, signal) => closed.then(() => resolve({ code, signal }))));
  return { child, done, output: () => output };
}

// A separate process that takes the lock (with short stop waits), runs
// `command args` under it and then runs `after` (JavaScript, with `result`).
function startRunner(root, command, args, { options = {}, after = '', stopGraceMs = 400, killWaitMs = 1000 } = {}) {
  return startScript(root, `
    acquireBuildLock({ purpose: 'runner', root, stopGraceMs: ${stopGraceMs}, killWaitMs: ${killWaitMs} });
    const result = await runUnderLock(${JSON.stringify(command)}, ${JSON.stringify(args)}, { stdio: ['ignore', 'pipe', 'inherit'], ...${JSON.stringify(options)} });
    ${after}
  `);
}

const alive = (pid) => { try { process.kill(pid, 0); return true; } catch (error) { return error.code !== 'ESRCH'; } };

// After a test, whatever it failed on: no stand-in survives to hold the runner's pipes open.
function reap(runner, pids) {
  for (const pid of [runner.child.pid, ...pids]) try { process.kill(pid, 'SIGKILL'); } catch { /* gone */ }
  runner.child.stdout.destroy();
  runner.child.stderr.destroy();
}

// The pids a stand-in build wrote to `file` once it is ready, within a bounded wait.
async function readyPids(file, runner) {
  for (const start = Date.now(); Date.now() - start < 10000; await new Promise((resolve) => setTimeout(resolve, 20))) {
    if (existsSync(file)) return readFileSync(file, 'utf8').trim().split(' ').map(Number);
    if (runner.child.exitCode !== null) break;
  }
  throw new Error(`the stand-in build never got ready:\n${runner.output()}`);
}

// A stand-in cargo that starts a stand-in rustc, both ignoring SIGINT and
// SIGTERM when `stubborn`; the rustc writes both pids to `ready` once both are set up.
function standInCargo(ready, stubborn) {
  const ignore = stubborn ? "process.on('SIGINT', () => {}); process.on('SIGTERM', () => {});" : '';
  const rustc = `${ignore} const fs = require('node:fs'); fs.writeFileSync(${JSON.stringify(`${ready}.tmp`)}, process.argv[1] + ' ' + process.pid); fs.renameSync(${JSON.stringify(`${ready}.tmp`)}, ${JSON.stringify(ready)}); setInterval(() => {}, 1000);`;
  return ['-e', `${ignore} require('node:child_process').spawn(process.execPath, ['-e', ${JSON.stringify(rustc)}, String(process.pid)], { stdio: 'ignore' }); setInterval(() => {}, 1000);`];
}

for (const [signal, status] of [['SIGINT', 130], ['SIGTERM', 143], ['SIGHUP', 129]]) {
  test(`${signal} to a holder running a build: its child and grandchild are stopped and reaped, then the lock is released; exit ${status}`, async () => {
    const t = tempRoot();
    const runner = startRunner(t.root, process.execPath, standInCargo(path.join(t.root, 'ready'), false));
    let pids = [];
    try {
      pids = await readyPids(path.join(t.root, 'ready'), runner);
      const [cargo, rustc] = pids;
      assert.ok(existsSync(buildLockPath(t.root)), 'the lock is held while the build runs');
      runner.child.kill(signal);
      const start = Date.now();
      assert.deepEqual(await runner.done, { code: status, signal: null }, runner.output());
      assert.ok(Date.now() - start < 5000, 'stopped promptly');
      assert.ok(!alive(cargo) && !alive(rustc), 'the child and grandchild are gone');
      assert.ok(!existsSync(buildLockPath(t.root)), 'the lock is released');
    } finally { reap(runner, pids); t.cleanup(); }
  });
}

test('a build ignoring SIGTERM is sent SIGKILL after the bounded wait and reaped; then the lock is released', async () => {
  const t = tempRoot();
  const runner = startRunner(t.root, process.execPath, standInCargo(path.join(t.root, 'ready'), true), { stopGraceMs: 400 });
  let pids = [];
  try {
    pids = await readyPids(path.join(t.root, 'ready'), runner);
    const [cargo, rustc] = pids;
    const start = Date.now();
    runner.child.kill('SIGTERM');
    assert.deepEqual(await runner.done, { code: 143, signal: null }, runner.output());
    assert.ok(Date.now() - start >= 400, 'escalated only after the grace');
    assert.ok(!alive(cargo) && !alive(rustc), 'the child and grandchild are gone');
    assert.ok(!existsSync(buildLockPath(t.root)), 'the lock is released');
    assert.doesNotMatch(runner.output(), /is kept/);
  } finally { reap(runner, pids); t.cleanup(); }
});

test('a build process group that cannot be confirmed empty after SIGKILL keeps the lock, and the removal steps are printed', { skip: !existsSync('/usr/bin/perl') && 'needs /usr/bin/perl' }, async () => {
  const t = tempRoot();
  const ready = path.join(t.root, 'ready');
  // The build's group keeps a member that SIGKILL cannot remove: a zombie whose
  // parent left the group and never reaps it.
  const script = 'my $p = fork(); if ($p == 0) { my $z = fork(); if ($z == 0) { exit 0 } setpgrp(0, 0); open(my $f, ">", "$ARGV[0].tmp"); print $f "$$ $z " . getppid(); close $f; rename("$ARGV[0].tmp", $ARGV[0]); sleep 30; exit 0 } sleep 30';
  // Its stand-ins hold none of the runner's pipes, which the test reads to their end.
  const runner = startRunner(t.root, '/usr/bin/perl', ['-e', script, ready], { options: { stdio: 'ignore' }, stopGraceMs: 200, killWaitMs: 300 });
  let pids = [];
  try {
    pids = await readyPids(ready, runner);
    const lockPath = buildLockPath(t.root);
    const before = readFileSync(lockPath, 'utf8');
    runner.child.kill('SIGTERM');
    assert.deepEqual(await runner.done, { code: 143, signal: null }, runner.output());
    assert.equal(readFileSync(lockPath, 'utf8'), before, 'the lock is kept');
    const output = runner.output();
    assert.match(output, new RegExp(`SIGTERM: process group \\d+ of this build did not empty after SIGTERM and SIGKILL; the worktree build lock ${lockPath.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')} is kept`));
    const [ps, pgrep, rm] = removalSteps(t.root, lockPath, runner.child.pid);
    for (const line of [ps, pgrep, rm]) assert.ok(output.includes(line), line);
  } finally {
    // The zombie's parent (launchd then reaps the zombie) and the group's leader.
    reap(runner, [pids[0], pids[2]].filter(Boolean));
    t.cleanup();
  }
});

test('an ordinary success, a nonzero exit, a thrown error and a timed-out command all release the lock', async () => {
  for (const [args, after, options, code, check] of [
    [['-e', ''], "console.log(JSON.stringify(result));", {}, 0, (result) => result.status === 0 && !result.timedOut],
    [['-e', 'process.exit(3)'], 'process.exit(result.status);', {}, 3, null],
    [['-e', ''], "throw new Error('boom');", {}, 1, null],
    [['-e', 'console.log(process.pid); setInterval(() => {}, 1000)'], "console.log(JSON.stringify(result));", { timeoutMs: 300 }, 0, (result) => result.timedOut && result.signal === 'SIGKILL' && !alive(Number(result.stdout))]
  ]) {
    const t = tempRoot();
    try {
      const runner = startRunner(t.root, process.execPath, args, { after, options });
      assert.equal((await runner.done).code, code, runner.output());
      if (check) assert.ok(check(JSON.parse(runner.output().trim().split('\n').at(-1))), runner.output());
      assert.ok(!existsSync(buildLockPath(t.root)), `${after}: the lock is released`);
    } finally { t.cleanup(); }
  }
});

test("the refusal names a native smoke's private app copy, by its token, for removal after the lock; no other directory", () => {
  const t = tempRoot();
  try {
    const lockPath = buildLockPath(t.root);
    const token = '0b5e6a1c-3d2f-4e8a-9b7c-1a2b3c4d5e6f';
    const runDirectory = path.join(t.root, 'target', 'native-smoke', `run-${token}`);
    mkdirSync(runDirectory, { recursive: true });
    mkdirSync(path.join(t.root, 'target', 'native-smoke', 'run-other'));
    const dead = spawnSync(process.execPath, ['-e', '']).pid;
    writeFileSync(lockPath, JSON.stringify({ pid: dead, started: 'T', purpose: 'native smoke', token, acquired: 'T' }));
    assert.throws(() => acquireBuildLock({ purpose: 'next', root: t.root }), (error) => {
      const lines = error.message.split('\n');
      assert.equal(lines.at(-3), `  rm -f '${lockPath}'`);
      assert.match(lines.at(-2), /^The native smoke that held the lock left its private app copy\. With the lock removed and nothing running from it .*, remove that directory, and no other run-\* directory:$/);
      assert.equal(lines.at(-1), `  rm -rf '${runDirectory}'`);
      return true;
    });
    // A token that is not the lock's own form never names a directory.
    writeFileSync(lockPath, JSON.stringify({ pid: dead, started: 'T', purpose: 'native smoke', token: '../other', acquired: 'T' }));
    assert.throws(() => acquireBuildLock({ purpose: 'next', root: t.root }), (error) => !error.message.includes('rm -rf') && error.message.split('\n').at(-1) === `  rm -f '${lockPath}'`);
  } finally { t.cleanup(); }
});

// Wait (bounded) for `file` to exist.
async function untilFile(file, runner) {
  for (const start = Date.now(); Date.now() - start < 10000; await new Promise((resolve) => setTimeout(resolve, 20))) {
    if (existsSync(file)) return;
    if (runner.child.exitCode !== null) break;
  }
  throw new Error(`${file} never appeared:\n${runner.output()}`);
}

test('Codex r8: a stop signal between publishing the lock and returning from acquireBuildLock releases the lock (exit 143)', async () => {
  const t = tempRoot();
  const ready = path.join(t.root, 'ready');
  // The hook holds the process right after the lock is published.
  const runner = startScript(t.root, `
    acquireBuildLock({ purpose: 'publish', root, onPublished: () => { writeFileSync(${JSON.stringify(ready)}, ''); Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 500); } });
    setTimeout(() => {}, 5000);
  `);
  try {
    await untilFile(ready, runner);
    assert.ok(existsSync(buildLockPath(t.root)), 'published');
    runner.child.kill('SIGTERM');
    assert.deepEqual(await runner.done, { code: 143, signal: null }, runner.output());
    assert.ok(!existsSync(buildLockPath(t.root)), 'the lock is released');
  } finally { reap(runner, []); t.cleanup(); }
});

test('Codex r8: a command whose group outlives it is settled before the next command; its group id is never signalled after it is seen empty', async () => {
  const t = tempRoot();
  const log = path.join(t.root, 'kills.log');
  const ready = path.join(t.root, 'ready');
  // The first command exits at once, leaving a member of its group for 800 ms.
  const first = "const c = require('node:child_process').spawn(process.execPath, ['-e', 'setTimeout(() => {}, 800)'], { stdio: 'ignore' }); c.unref(); console.log(process.pid + ' ' + c.pid);";
  const second = `require('node:fs').writeFileSync(${JSON.stringify(ready)}, String(process.pid)); setInterval(() => {}, 1000);`;
  // Every signal sent to a process group is logged.
  const runner = startScript(t.root, `
    const log = openSync(${JSON.stringify(log)}, 'a');
    const kill = process.kill.bind(process);
    process.kill = (pid, signal) => { if (pid < 0) writeSync(log, 'kill ' + (-pid) + ' ' + (signal ?? 0) + '\\n'); return kill(pid, signal); };
    acquireBuildLock({ purpose: 'groups', root, stopGraceMs: 300, killWaitMs: 200 });
    const one = await runUnderLock(process.execPath, ['-e', ${JSON.stringify(first)}], { stdio: ['ignore', 'pipe', 'ignore'] });
    writeSync(log, 'settled ' + one.stdout.trim() + '\\n');
    await runUnderLock(process.execPath, ['-e', ${JSON.stringify(second)}], { stdio: 'ignore' });
  `);
  let pids = [];
  try {
    await untilFile(ready, runner);
    const lines = readFileSync(log, 'utf8').trim().split('\n');
    const settled = lines.findIndex((line) => line.startsWith('settled '));
    pids = [...lines[settled].split(' ').slice(1).map(Number), Number(readFileSync(ready, 'utf8'))];
    const [leader, member] = pids;
    // Whether the leftover member was gone before the second command ran, leaving the group id free for reuse.
    const settledFirst = !alive(member);
    for (const start = Date.now(); alive(member) && Date.now() - start < 2000; ) await new Promise((resolve) => setTimeout(resolve, 20));
    runner.child.kill('SIGTERM');
    assert.deepEqual(await runner.done, { code: 143, signal: null }, runner.output());
    const after = readFileSync(log, 'utf8').trim().split('\n').slice(settled + 1);
    assert.ok(after.some((line) => line.startsWith('kill ') && !line.startsWith(`kill ${leader} `)), `the second group was signalled: ${after}`);
    assert.deepEqual(after.filter((line) => line.startsWith(`kill ${leader} `)), [], 'nothing is sent to the first group id once it was seen empty');
    assert.ok(settledFirst, 'the first group was emptied before the second command');
    assert.ok(!existsSync(buildLockPath(t.root)), 'the lock is released');
  } finally { reap(runner, pids.slice(1)); t.cleanup(); }
});

test('Codex r8: a signal received during synchronous work is not lost to an immediate exit or to the release', async () => {
  for (const ending of ['lock.release(); await exitAfterSignals(0);', 'lock.release();']) {
    const t = tempRoot();
    const ready = path.join(t.root, 'ready');
    const runner = startScript(t.root, `
      const lock = acquireBuildLock({ purpose: 'sync', root });
      writeFileSync(${JSON.stringify(ready)}, '');
      execFileSync('/bin/sleep', ['0.5']);
      ${ending}
    `);
    try {
      await untilFile(ready, runner);
      runner.child.kill('SIGTERM');
      assert.deepEqual(await runner.done, { code: 143, signal: null }, `${ending}\n${runner.output()}`);
      assert.ok(!existsSync(buildLockPath(t.root)), 'the lock is released');
    } finally { reap(runner, []); t.cleanup(); }
  }
});

test('Codex r8: the kept-lock recovery message survives a full standard-error pipe at exit', { skip: !existsSync('/usr/bin/perl') && 'needs /usr/bin/perl' }, async () => {
  const t = tempRoot();
  const ready = path.join(t.root, 'ready');
  const script = 'my $p = fork(); if ($p == 0) { my $z = fork(); if ($z == 0) { exit 0 } setpgrp(0, 0); open(my $f, ">", "$ARGV[0].tmp"); print $f "$$ $z " . getppid(); close $f; rename("$ARGV[0].tmp", $ARGV[0]); sleep 30; exit 0 } sleep 30';
  // Standard error is not read until a second after the signal, and the
  // runner has queued a megabyte for it first.
  let startReading;
  const reading = new Promise((resolve) => { startReading = resolve; });
  const runner = startScript(t.root, `
    acquireBuildLock({ purpose: 'pipe', root, stopGraceMs: 200, killWaitMs: 300 });
    process.stderr.write('x'.repeat(1 << 20) + '\\n');
    await runUnderLock('/usr/bin/perl', ['-e', ${JSON.stringify(script)}, ${JSON.stringify(ready)}], { stdio: 'ignore' });
  `, { readAfter: reading });
  let pids = [];
  try {
    pids = await readyPids(ready, runner);
    runner.child.kill('SIGTERM');
    setTimeout(startReading, 1000);
    assert.deepEqual(await runner.done, { code: 143, signal: null });
    const output = runner.output();
    const lockPath = buildLockPath(t.root);
    assert.ok(existsSync(lockPath), 'the lock is kept');
    assert.match(output, /did not empty after SIGTERM and SIGKILL; the worktree build lock .* is kept/);
    assert.ok(output.includes(`  rm -f '${lockPath}'`), 'the removal step arrived');
  } finally {
    startReading();
    reap(runner, [pids[0], pids[2]].filter(Boolean));
    t.cleanup();
  }
});
