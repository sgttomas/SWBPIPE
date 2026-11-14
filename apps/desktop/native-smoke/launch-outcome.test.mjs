// The one rule for judging a native-smoke launch (`launch-outcome.mjs`), without launching the app. Every file
// here is invented under the system temporary folder. Runs on macOS and Linux: the desktop test script runs it
// before Vitest (`apps/desktop/package.json`).
//
//   node --test apps/desktop/native-smoke/launch-outcome.test.mjs
import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { keptPhaseFailures, launchFailures, newNonce, readReport, recordedFailures, recordedOutcome } from "./launch-outcome.mjs";

const NONCE = "0123456789abcdef0123456789abcdef";
const clean = { status: 0, signal: null, timedOut: false, census: { resolved: true }, report: { nonce: NONCE, ok: true }, nonce: NONCE };
const codes = (failures) => failures.map((line) => line.split(":")[0]);

test("a clean launch with this run's report passes", () => {
  assert.deepEqual(launchFailures(clean), []);
});

test("a report does not rescue a launch that did not end cleanly (the fault this repairs)", () => {
  assert.deepEqual(codes(launchFailures({ ...clean, status: 1 })), ["LAUNCH-EXIT-NOT-ZERO"]);
  assert.deepEqual(codes(launchFailures({ ...clean, status: null, signal: "SIGKILL" })), ["LAUNCH-SIGNALLED", "LAUNCH-EXIT-NOT-ZERO"]);
  assert.deepEqual(codes(launchFailures({ ...clean, status: null, signal: "SIGKILL", timedOut: true })), ["LAUNCH-TIMED-OUT", "LAUNCH-SIGNALLED", "LAUNCH-EXIT-NOT-ZERO"]);
  assert.deepEqual(codes(launchFailures({ ...clean, census: { resolved: false, reason: "process 7 still ran" } })), ["LAUNCH-CENSUS-UNRESOLVED"]);
});

test("a report that is not this run's, or no report, fails", () => {
  assert.deepEqual(codes(launchFailures({ ...clean, report: { ok: true } })), ["LAUNCH-REPORT-NOT-THIS-RUN"]);
  assert.deepEqual(codes(launchFailures({ ...clean, report: { nonce: "another run" } })), ["LAUNCH-REPORT-NOT-THIS-RUN"]);
  assert.deepEqual(codes(launchFailures({ ...clean, report: null, problem: "no report" })), ["LAUNCH-NO-REPORT"]);
  assert.deepEqual(codes(launchFailures({ ...clean, nonce: "" })), ["LAUNCH-NO-NONCE"]);
});

test("a runner that takes no census is judged on the rest", () => {
  assert.deepEqual(launchFailures({ ...clean, census: undefined }), []);
});

test("readReport reads a JSON object and names anything else", () => {
  const dir = mkdtempSync(path.join(os.tmpdir(), "launch-outcome-test-"));
  try {
    const file = path.join(dir, "report.json");
    assert.equal(readReport(file).problem, "no report");
    writeFileSync(file, "{ not json");
    assert.match(readReport(file).problem, /does not parse/);
    writeFileSync(file, "[1]");
    assert.equal(readReport(file).problem, "the report is not a JSON object");
    writeFileSync(file, JSON.stringify({ nonce: NONCE }));
    assert.deepEqual(readReport(file), { report: { nonce: NONCE }, problem: null });
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});

test("kept evidence is judged from its recorded outcome; evidence without one is not recorded, not a pass", () => {
  const outcome = recordedOutcome({ ...clean, started: "s", ended: "e" });
  assert.deepEqual(recordedFailures(outcome, clean.report), []);
  assert.deepEqual(codes(recordedFailures({ ...outcome, timed_out: true }, clean.report)), ["LAUNCH-TIMED-OUT"]);
  assert.deepEqual(codes(recordedFailures(undefined, clean.report)), ["LAUNCH-OUTCOME-NOT-RECORDED"]);
  assert.deepEqual(codes(recordedFailures(null, clean.report)), ["LAUNCH-OUTCOME-NOT-RECORDED"]);
});

test("each run gets a fresh nonce", () => {
  const first = newNonce();
  assert.match(first, /^[0-9a-f]{32}$/);
  assert.notEqual(first, newNonce());
});

test("a kept summary written before launch outcomes were recorded is judged not recorded, though its checks and report read well", () => {
  const old = JSON.parse(readFileSync(new URL("./test-fixtures/kept-evidence/summary-without-launch-outcome.json", import.meta.url), "utf8"));
  assert.ok(old.checks.every((item) => item.ok) && old.reports.bound.ok);
  for (const phase of ["bound", "unbound"]) assert.deepEqual(codes(keptPhaseFailures(old, phase, clean.report)), ["LAUNCH-OUTCOME-NOT-RECORDED"]);
  // The same summary once it records a clean launch for the phase is judged on that record.
  const recorded = { ...old, launches: { bound: { outcome: recordedOutcome({ ...clean, started: "s", ended: "e" }) } } };
  assert.deepEqual(keptPhaseFailures(recorded, "bound", clean.report), []);
  assert.deepEqual(codes(keptPhaseFailures(recorded, "unbound", clean.report)), ["LAUNCH-OUTCOME-NOT-RECORDED"]);
  assert.deepEqual(codes(keptPhaseFailures(recorded, "bound", { ...clean.report, nonce: "another run" })), ["LAUNCH-REPORT-NOT-THIS-RUN"]);
});

test("an incomplete kept outcome is not a pass: a missing status, signal or timeout field is not recorded, a missing census is unresolved", () => {
  const complete = recordedOutcome({ ...clean, started: "s", ended: "e" });
  const summary = (outcome) => ({ launches: { bound: { outcome } } });
  assert.deepEqual(keptPhaseFailures(summary(complete), "bound", clean.report), []);
  const without = (field) => { const outcome = { ...complete }; delete outcome[field]; return outcome; };
  for (const field of ["status", "signal", "timed_out"]) {
    assert.deepEqual(codes(keptPhaseFailures(summary(without(field)), "bound", clean.report)), ["LAUNCH-OUTCOME-NOT-RECORDED"], field);
    assert.deepEqual(codes(recordedFailures(without(field), clean.report)), ["LAUNCH-OUTCOME-NOT-RECORDED"], field);
  }
  assert.deepEqual(codes(keptPhaseFailures(summary({ ...complete, timed_out: "false" }), "bound", clean.report)), ["LAUNCH-OUTCOME-NOT-RECORDED"]);
  assert.deepEqual(codes(keptPhaseFailures(summary([]), "bound", clean.report)), ["LAUNCH-OUTCOME-NOT-RECORDED"]);
  // A census-taking runner's kept outcome with no census, or a null one, is unresolved, never "no census taken".
  assert.deepEqual(codes(keptPhaseFailures(summary(without("census")), "bound", clean.report)), ["LAUNCH-CENSUS-UNRESOLVED"]);
  assert.deepEqual(codes(keptPhaseFailures(summary({ ...complete, census: null }), "bound", clean.report)), ["LAUNCH-CENSUS-UNRESOLVED"]);
  assert.deepEqual(codes(keptPhaseFailures(summary({ ...complete, census: { resolved: false, reason: "process 7 still ran" } }), "bound", clean.report)), ["LAUNCH-CENSUS-UNRESOLVED"]);
  // recordedFailures is for a runner that takes no census: its census, recorded or not, is not judged.
  assert.deepEqual(recordedFailures({ ...complete, census: null }, clean.report), []);
});
