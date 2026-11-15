// The real launch and judgment path of `run.mjs` and `mbf-journey.mjs` (`judged-launch.mjs`, DEL-07-11), driven with
// a stand-in app: a private copy of this Node executable running an invented script, so that the census finds its
// processes as it finds the app copy's. The marks are checked by the real `marksCheck`, given controlled responses
// to its four commands; no screen-control service is started. Everything is under the system temporary folder.
// Runs on macOS and Linux (the wrapper case on macOS only).
//
//   node --test apps/desktop/native-smoke/judged-launch.test.mjs
import { test, before, after } from "node:test";
import assert from "node:assert/strict";
import { constants, copyFileSync, existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { MARK_PRESENT, UNRESOLVED_TERMINATION, judgedPhase } from "./judged-launch.mjs";

let dir, executable, script;
const survivors = [];

// The stand-in app. It reads the runner's plan and report path as the drivers do, marks that it started, and then
// behaves as its mode says.
const STAND_IN = `
const fs = require("node:fs");
const { spawn } = require("node:child_process");
const plan = JSON.parse(process.env.SWBPIPE_NATIVE_SMOKE_PLAN);
const report = process.env.SWBPIPE_NATIVE_SMOKE_REPORT;
fs.writeFileSync(report + ".started", "");
const write = () => fs.writeFileSync(report, JSON.stringify({ ok: true, nonce: plan.nonce }));
switch (plan.mode) {
  case "clean": write(); process.exit(0);
  case "exit-before-report": process.exit(3);
  case "timeout": setInterval(() => {}, 1000); break;
  case "kill-after-report": write(); process.kill(process.pid, "SIGKILL"); break;
  case "no-write": process.exit(0);
  case "survivor": {
    const child = spawn(process.execPath, ["-e", "setTimeout(() => {}, 20000)"], { detached: true, stdio: "ignore" });
    fs.writeFileSync(report + ".survivor", String(child.pid));
    child.unref();
    write();
    process.exit(0);
  }
}
`;

before(() => {
  dir = mkdtempSync(path.join(os.tmpdir(), "judged-launch-test-"));
  executable = path.join(dir, "stand-in-app");
  // A copy (a clone where the file system has them), never a link: the census identifies the file itself.
  copyFileSync(process.execPath, executable, constants.COPYFILE_FICLONE);
  script = path.join(dir, "stand-in.cjs");
  writeFileSync(script, STAND_IN);
});
after(() => {
  for (const pid of survivors) { try { process.kill(pid, "SIGKILL"); } catch { /* gone */ } }
  rmSync(dir, { recursive: true, force: true });
});

// Controlled responses to the marks commands: every mark absent, or the named one present, or one check unmade.
const answer = (status, stdout = "") => ({ status, stdout, stderr: "", error: undefined });
const marksAbsent = (command) => (command.endsWith("pgrep") ? answer(1) : answer(0, ""));
const skyPresent = (command, args) => (command.endsWith("pgrep") && args[1] === "SkyComputerUseService" ? answer(0, "4242\n") : marksAbsent(command));
const launchctlFails = (command) => (command === "/bin/launchctl" ? answer(5) : marksAbsent(command));

const NONCE = "0123456789abcdef0123456789abcdef";
const codes = (failures) => failures.map((line) => line.split(":")[0]);

/** One judged phase of the stand-in in `mode`, in a fresh root, as a runner launches it. */
async function phase(mode, { marksRun = marksAbsent, wrapper = null, timeoutMs = 30_000, censusBoundMs = 2_000, stale = false, launches = {} } = {}) {
  const root = mkdtempSync(path.join(dir, `root-${mode}-`));
  const reportPath = path.join(root, `report-${mode}.json`);
  if (stale) writeFileSync(reportPath, JSON.stringify({ ok: true, nonce: "an earlier run" }));
  const checks = [];
  const [command, commandArgs] = wrapper ? [wrapper, ["-di", executable, script]] : [executable, [script]];
  const result = await judgedPhase({
    phase: mode, checkName: `${mode}: the launch ended cleanly and its report is this run's`, launches,
    check: (name, ok, detail) => checks.push({ name, ok, detail }),
    summaryPath: path.join(root, "summary.json"), summary: () => ({ checks }),
    executable, command, commandArgs, cwd: root, stdio: ["ignore", "ignore", "ignore"],
    env: { ...process.env, SWBPIPE_NATIVE_SMOKE_PLAN: JSON.stringify({ mode, nonce: NONCE }), SWBPIPE_NATIVE_SMOKE_REPORT: reportPath },
    timeoutMs, reportPath, nonce: NONCE, censusBoundMs, marksRun
  });
  const survivor = path.join(`${reportPath}.survivor`);
  if (existsSync(survivor)) survivors.push(Number(readFileSync(survivor, "utf8")));
  const summaryFile = path.join(root, "summary.json");
  return {
    result, checks, launches, root, reportPath,
    started: existsSync(`${reportPath}.started`),
    summary: existsSync(summaryFile) ? JSON.parse(readFileSync(summaryFile, "utf8")) : null
  };
}

test("the clean case reaches every boundary and passes: started, ended 0, census resolved, this run's report", async () => {
  const run = await phase("clean");
  assert.equal(run.started, true);
  assert.equal(run.result.stop, undefined);
  assert.deepEqual(run.result.failures, []);
  assert.deepEqual({ status: run.result.outcome.status, signal: run.result.outcome.signal, timed_out: run.result.outcome.timed_out, nonce: run.result.outcome.nonce }, { status: 0, signal: null, timed_out: false, nonce: NONCE });
  assert.equal(run.result.census.resolved, true);
  assert.deepEqual(run.result.report, { ok: true, nonce: NONCE });
  assert.deepEqual(run.checks.map((item) => item.ok), [true]);
  assert.equal(run.summary, null, "a passing phase does not stop the run");
});

test("a valid report cannot rescue a launch that ended on its own signal; the report is still read", async () => {
  const run = await phase("kill-after-report");
  assert.deepEqual(codes(run.result.failures), ["LAUNCH-SIGNALLED", "LAUNCH-EXIT-NOT-ZERO"]);
  assert.equal(run.result.outcome.signal, "SIGKILL");
  assert.deepEqual(run.result.report, { ok: true, nonce: NONCE });
  assert.deepEqual(run.checks.map((item) => item.ok), [false]);
  assert.deepEqual(codes(run.launches["kill-after-report"].failures), ["LAUNCH-SIGNALLED", "LAUNCH-EXIT-NOT-ZERO"]);
});

test("the app's own signal reaches the judge through the runners' wrapper, caffeinate", { skip: process.platform !== "darwin" && "caffeinate is macOS only" }, async () => {
  const run = await phase("kill-after-report", { wrapper: "/usr/bin/caffeinate" });
  assert.equal(run.started, true);
  assert.ok(codes(run.result.failures).includes("LAUNCH-EXIT-NOT-ZERO"), JSON.stringify(run.result.outcome));
  assert.ok(run.result.outcome.signal === "SIGKILL" || run.result.outcome.status !== 0, JSON.stringify(run.result.outcome));
  assert.deepEqual(run.result.report, { ok: true, nonce: NONCE });
});

test("a launch that fails before any report keeps its named process failures beside the missing report", async () => {
  const exited = await phase("exit-before-report");
  assert.deepEqual(codes(exited.result.failures), ["LAUNCH-EXIT-NOT-ZERO", "LAUNCH-NO-REPORT"]);
  assert.equal(exited.result.outcome.status, 3);
  const timedOut = await phase("timeout", { timeoutMs: 1_500 });
  assert.deepEqual(codes(timedOut.result.failures), ["LAUNCH-TIMED-OUT", "LAUNCH-SIGNALLED", "LAUNCH-EXIT-NOT-ZERO", "LAUNCH-NO-REPORT"]);
  assert.equal(timedOut.result.outcome.timed_out, true);
  for (const run of [exited, timedOut]) assert.equal(run.result.report.ok, false);
});

test("a report left in the root by an earlier run is not this run's", async () => {
  const run = await phase("no-write", { stale: true });
  assert.deepEqual(codes(run.result.failures), ["LAUNCH-REPORT-NOT-THIS-RUN"]);
});

test("an unresolved census is a named, recorded failure that saves the summary, keeps the evidence and stops the run", async () => {
  const launches = {};
  const run = await phase("survivor", { launches, censusBoundMs: 1_500 });
  assert.equal(run.result.stop, UNRESOLVED_TERMINATION);
  assert.equal(run.result.exitCode, 1);
  assert.equal(run.result.census.resolved, false);
  assert.deepEqual(codes(run.result.failures), ["LAUNCH-CENSUS-UNRESOLVED"]);
  assert.equal(run.summary.stopped, UNRESOLVED_TERMINATION);
  assert.deepEqual(codes(run.summary.launches.survivor.failures), ["LAUNCH-CENSUS-UNRESOLVED"]);
  assert.equal(run.summary.launches.survivor.outcome.census.resolved, false);
  assert.deepEqual(run.summary.checks.map((item) => item.ok), [false]);
  assert.ok(existsSync(run.reportPath), "the evidence is kept");
  // As the runners do: a stop ends the run, so the next phase never starts.
  let second = null;
  if (!run.result.stop) second = await phase("clean", { launches });
  assert.equal(second, null);
  assert.deepEqual(Object.keys(launches), ["survivor"]);
});

test("a mark observed at the call site prevents the start and saves the refusal with the marks", async () => {
  for (const [label, marksRun, field] of [["the supplier's window service running", skyPresent, "sky_computer_use_service_process"], ["a marks check that cannot be made", launchctlFails, null]]) {
    const launches = {};
    const run = await phase("clean", { marksRun, launches });
    assert.equal(run.started, false, `${label}: the stand-in never started`);
    assert.equal(run.result.stop, MARK_PRESENT, label);
    assert.equal(run.result.exitCode, 2, label);
    assert.equal(run.summary.refused, MARK_PRESENT, label);
    assert.equal(run.summary.launches.clean.launched, false, label);
    if (field) assert.equal(run.summary.launches.clean.marks[field], true, label);
    else assert.match(run.summary.launches.clean.marks.check_failures[0], /^launchctl list: exit 5$/, label);
    assert.equal(existsSync(run.reportPath), false, label);
  }
});
