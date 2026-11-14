// The real `mbf-journey.mjs --judge` command on minimal kept evidence built here at run time (DEL-07-11): the judge
// takes each phase's launch outcome only from summary.json's `launches` record, and an incomplete or missing outcome
// is never a pass. The evidence holds only what the launch judgment reads: the two fixture hashes the judge insists
// on, each phase's recorded outcome and a report carrying its nonce. Every other check of the journey fails on it;
// these tests read only the two launch checks. Nothing is launched. Runs on macOS and Linux.
//
//   node --test apps/desktop/native-smoke/mbf-journey-judge.test.mjs
import { test } from "node:test";
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { recordedOutcome } from "./launch-outcome.mjs";

const here = path.dirname(fileURLToPath(import.meta.url));
const project = path.resolve(here, "..", "..", "..");
const FIXTURES = ["fixtures/caepipe_mbf/invented/desktop_export/export_model.json", "fixtures/caepipe_mbf/invented/desktop_import/import_replies.json"];
const sha = (file) => createHash("sha256").update(readFileSync(path.join(project, file))).digest("hex");
const NONCE = { bound: "0123456789abcdef0123456789abcdef", unbound: "fedcba9876543210fedcba9876543210" };
const clean = (phase) => recordedOutcome({ status: 0, signal: null, timedOut: false, census: { resolved: true }, nonce: NONCE[phase], started: "s", ended: "e" });

/** Judges minimal evidence whose `launches` is `launches`; the two launch checks by phase. */
function judge(launches) {
  const dir = mkdtempSync(path.join(os.tmpdir(), "mbf-journey-judge-test-"));
  try {
    writeFileSync(path.join(dir, "summary.json"), JSON.stringify({ source_revision: "invented", inputs: Object.fromEntries(FIXTURES.map((file) => [file, sha(file)])), ...(launches === undefined ? {} : { launches }) }));
    for (const phase of ["bound", "unbound"]) writeFileSync(path.join(dir, `report-${phase}.json`), JSON.stringify({ ok: true, nonce: NONCE[phase] }));
    const run = spawnSync(process.execPath, [path.join(here, "mbf-journey.mjs"), "--judge", dir], { cwd: project, encoding: "utf8", timeout: 60_000 });
    assert.equal(run.error, undefined);
    const judged = JSON.parse(readFileSync(path.join(dir, "summary-judged.json"), "utf8"));
    const launch = (phase) => judged.checks.find((item) => item.name === `${phase}: the launch ended cleanly and its report is this run's`);
    const codes = (item) => (item.detail ?? []).map((line) => line.split(":")[0]);
    return { status: run.status, bound: { ok: launch("bound").ok, codes: codes(launch("bound")) }, unbound: { ok: launch("unbound").ok, codes: codes(launch("unbound")) } };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

test("a complete recorded outcome with this run's report passes the launch check", () => {
  const out = judge({ bound: { outcome: clean("bound") }, unbound: { outcome: clean("unbound") } });
  assert.deepEqual([out.bound, out.unbound], [{ ok: true, codes: [] }, { ok: true, codes: [] }]);
  // The journey's other checks need the full evidence, so the command itself exits non-zero here.
  assert.equal(out.status, 1);
});

test("an incomplete, missing or failed outcome is not a pass, through the real judge", () => {
  const { signal, ...noSignal } = clean("bound");
  const { census, ...noCensus } = clean("unbound");
  const partial = judge({ bound: { outcome: noSignal }, unbound: { outcome: noCensus } });
  assert.deepEqual([partial.bound, partial.unbound], [{ ok: false, codes: ["LAUNCH-OUTCOME-NOT-RECORDED"] }, { ok: false, codes: ["LAUNCH-CENSUS-UNRESOLVED"] }]);
  const old = judge(undefined);
  assert.deepEqual([old.bound.codes, old.unbound.codes], [["LAUNCH-OUTCOME-NOT-RECORDED"], ["LAUNCH-OUTCOME-NOT-RECORDED"]]);
  const killed = judge({ bound: { outcome: { ...clean("bound"), status: null, signal: "SIGKILL" } }, unbound: { outcome: { ...clean("unbound"), nonce: "another run" } } });
  assert.deepEqual([killed.bound.codes, killed.unbound.codes], [["LAUNCH-SIGNALLED", "LAUNCH-EXIT-NOT-ZERO"], ["LAUNCH-REPORT-NOT-THIS-RUN"]]);
  assert.equal(killed.status, 1);
});
