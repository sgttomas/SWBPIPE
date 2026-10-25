// The native-smoke driver's wait and solve-failure report, without the app:
// `driver.js` is evaluated against a fake window, DOM and IPC and hands its
// helpers to the test instead of running.
//
//   node --test apps/desktop/native-smoke/driver.test.mjs
import { test } from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import vm from "node:vm";

const source = readFileSync(new URL("./driver.js", import.meta.url), "utf8");

function load({ shown = {}, record = null, pool = null, poolError = null } = {}) {
  const logs = [];
  const elements = new Map(Object.entries(shown).map(([id, value]) => [id, { textContent: value, getAttribute: () => null }]));
  if (record) {
    const href = `data:application/json;charset=utf-8,${encodeURIComponent(JSON.stringify(record, null, 2))}`;
    elements.set("solve-job-export-link", { textContent: "Solve job JSON", getAttribute: (name) => (name === "href" ? href : null) });
  }
  const invoke = async (command, args) => {
    if (command === "native_smoke_log") { logs.push(args.line); return null; }
    if (command === "get_memory_pool_status") { if (poolError) throw new Error(poolError); return pool; }
    throw new Error(`unexpected command ${command}`);
  };
  const hooks = {};
  const context = {
    __SWBPIPE_NATIVE_SMOKE_DRIVER_TEST__: hooks,
    __TAURI_INTERNALS__: { invoke },
    CSS: { escape: (value) => value },
    document: { querySelector: (selector) => elements.get(/^\[data-testid="(.*)"\]$/.exec(selector)?.[1]) ?? null },
    setTimeout,
    Date,
    Promise
  };
  context.window = context;
  vm.runInNewContext(source, context);
  return { hooks, logs };
}

// A refusal as the product shows it (SolvePanel, memoryPool.ts) and records it.
const REFUSAL = { kind: "headroom_insufficient", required_bytes: 266096640, available_bytes: 89243648 };
const POOL = {
  pool_bytes: 17179869184, invocation_budget_bytes: 12884901888, reserved_bytes: 0, running_workers: 0, queued_jobs: 0, running_jobs: 0,
  headroom: { available_bytes: 91000000, pressure: "normal", sample_age_ms: 40 }, last_refusal: REFUSAL
};
const FAILED = {
  "solve-job-summary": "state=failed; events=3; result_rows=0; cancellation_requested=false",
  "solve-job-error": "RESOURCE_UNAVAILABLE: Not enough free memory on this Mac right now: needs 253.77 MiB, 85.11 MiB available.",
  "solve-job-resource-refusal": "Refused by the native memory pool: Not enough free memory on this Mac right now: needs 253.77 MiB, 85.11 MiB available. No result was published.",
  "solve-job-resource": "budget=12 GiB (intake and solve worker cap); pool=16 GiB; route=not granted; refusal=headroom_insufficient"
};

test("waitFor rethrows a probe's error at once instead of waiting out the timeout", async () => {
  const { hooks } = load();
  const started = Date.now();
  await assert.rejects(hooks.waitFor("x", () => { throw new Error("state=failed"); }, 60000), /^Error: state=failed$/);
  assert.ok(Date.now() - started < 1000);
});

test("waitFor still returns a probe's value and times out with the same message", async () => {
  const { hooks } = load();
  let calls = 0;
  assert.equal(await hooks.waitFor("x", () => (++calls >= 2 ? "seen" : null), 5000), "seen");
  await assert.rejects(hooks.waitFor("never", () => null, 150), { message: "timed out after 150 ms waiting for never" });
});

test("a failed solve is reported with its refusal code, required and headroom bytes and pressure", async () => {
  const { hooks, logs } = load({ shown: FAILED, record: { resource: { queue_reason: null, refusal: REFUSAL } }, pool: POOL });
  const error = await hooks.waitForSolve("demo/sparse solve completed", () => {
    const summary = FAILED["solve-job-summary"];
    if (/state=(failed|cancelled)/.test(summary)) throw Object.assign(new Error(`demo/sparse: ${summary}`), { hard: true });
    return false;
  }, 300000).then(() => null, (caught) => caught);
  assert.ok(error);
  assert.equal(error.hard, true);
  assert.match(error.message, /^demo\/sparse: state=failed; /);
  assert.match(error.message, /error="RESOURCE_UNAVAILABLE: Not enough free memory on this Mac right now: needs 253\.77 MiB, 85\.11 MiB available\."/);
  assert.match(error.message, /reason code=headroom_insufficient \(job refusal\); required_bytes=266096640; headroom_bytes=89243648/);
  assert.match(error.message, /memory pool now: headroom_bytes=91000000; pressure=normal; sample_age_ms=40; queued_jobs=0; running_jobs=0; running_workers=0/);
  assert.deepEqual(JSON.parse(JSON.stringify(error.solve_job.reason)), { source: "job refusal", code: "headroom_insufficient", required_bytes: 266096640, headroom_bytes: 89243648, pressure: null, typed: REFUSAL });
  assert.equal(logs.length, 1);
  assert.ok(logs[0].startsWith("demo/sparse solve completed: solve job: summary=\"state=failed;"));
});

test("a solve still queued at the timeout names its queue reason", async () => {
  const queued = { kind: "worker_limit_reached", running: 1, limit: 1 };
  const shown = {
    "solve-job-summary": "state=queued; events=1; result_rows=0; cancellation_requested=false",
    "solve-job-queue-reason": "Waiting in the queue: 1 solve worker is already running (limit 1); this solve waits until one finishes."
  };
  const { hooks } = load({ shown, record: { resource: { queue_reason: queued, refusal: null } }, pool: { ...POOL, queued_jobs: 1, running_jobs: 1, running_workers: 1, last_refusal: null } });
  await assert.rejects(hooks.waitForSolve("blank solve", () => false, 150), (error) => {
    assert.match(error.message, /^timed out after 150 ms waiting for blank solve; solve job: summary="state=queued;/);
    assert.match(error.message, /queue_reason="Waiting in the queue: 1 solve worker is already running/);
    assert.match(error.message, /reason code=worker_limit_reached \(job queue reason\); required_bytes=null; headroom_bytes=null/);
    assert.match(error.message, /queued_jobs=1; running_jobs=1; running_workers=1/);
    return true;
  });
});

test("a slow solve with no reason says so, and a failed pool read is reported, not thrown", async () => {
  const shown = { "solve-job-summary": "state=running; events=2; result_rows=0; cancellation_requested=false" };
  const { hooks } = load({ shown, poolError: "no such command" });
  await assert.rejects(hooks.waitForSolve("slow solve", () => false, 150), (error) => {
    assert.match(error.message, /summary="state=running;/);
    assert.match(error.message, /job record not readable; reason: none reported; memory pool status: no such command$/);
    return true;
  });
});

test("a refused worker growth reports the pool's verdict as its cause", () => {
  const { hooks } = load();
  const growth = { kind: "worker_growth_refused", granted_bytes: 100, requested_bytes: 300, cause: { kind: "pressure_elevated", level: "critical" } };
  const { fields, text } = hooks.composeSolveDiagnostics({}, { queue_reason: null, refusal: growth }, POOL);
  assert.equal(fields.reason.code, "worker_growth_refused/pressure_elevated");
  assert.equal(fields.reason.required_bytes, 300);
  assert.match(text, /required_bytes=300; headroom_bytes=null; pressure=critical/);
});
