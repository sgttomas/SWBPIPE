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

function load({ shown = {}, record = null, pool = null, poolError = null, projectRows = [] } = {}) {
  const logs = [];
  const elements = new Map(Object.entries(shown).map(([id, value]) => [id, { textContent: value, getAttribute: () => null }]));
  // The model tree's project rows, `tree-row-project-<encoded id>`, in order; the
  // caller may replace the array's contents to change what the window shows.
  const rowElements = () => projectRows.map((id) => {
    const testId = `tree-row-project-${encodeURIComponent(id)}`;
    return { textContent: id, getAttribute: (name) => (name === "data-testid" ? testId : null) };
  });
  const byPrefix = (selector) => /^\[data-testid\^="(.*)"\]$/.exec(selector)?.[1];
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
    document: {
      querySelector: (selector) => {
        const prefix = byPrefix(selector);
        if (prefix !== undefined) return rowElements().find((row) => row.getAttribute("data-testid").startsWith(prefix)) ?? null;
        return elements.get(/^\[data-testid="(.*)"\]$/.exec(selector)?.[1]) ?? null;
      },
      querySelectorAll: (selector) => {
        const prefix = byPrefix(selector);
        return prefix === undefined ? [] : rowElements().filter((row) => row.getAttribute("data-testid").startsWith(prefix));
      }
    },
    setTimeout,
    Date,
    Promise
  };
  context.window = context;
  vm.runInNewContext(source, context);
  return { hooks, logs, elements };
}

// A refusal as the product shows it (SolvePanel, memoryPool.ts) and records it.
const REFUSAL = { kind: "headroom_insufficient", required_bytes: 266096640, available_bytes: 89243648 };
const POOL = {
  pool_bytes: 17179869184, invocation_budget_bytes: 12884901888, reserved_bytes: 0, running_workers: 0, queued_jobs: 0, running_jobs: 0,
  headroom: { available_bytes: 91000000, pressure: "normal", sample_age_ms: 40 }, last_refusal: REFUSAL
};
const FAILED = {
  "solve-job-summary": "state=failed; events=3; result_rows=0; cancellation_requested=false",
  "solve-job-error": "RESOURCE_UNAVAILABLE: Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs 253.77 MiB, 85.11 MiB available.",
  "solve-job-resource-refusal": "Refused by the native memory pool: Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs 253.77 MiB, 85.11 MiB available. No result was published.",
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
  assert.match(error.message, /error="RESOURCE_UNAVAILABLE: Not enough available memory on this Mac right now \(free plus reclaimable file cache and purgeable memory, less the kernel's reserve\): needs 253\.77 MiB, 85\.11 MiB available\."/);
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

test("a solve job that ended refused, failed or cancelled fails the wait at once, with its refusal and reason code", async () => {
  // As the smoke saw it: the native memory pool refused the solve worker's growth.
  const growth = { kind: "worker_growth_refused", granted_bytes: 0, requested_bytes: 67108864, cause: { kind: "headroom_insufficient", required_bytes: 67108864, available_bytes: 0 } };
  const refused = {
    "solve-job-summary": "state=refused; events=2; result_rows=0; cancellation_requested=false",
    "solve-job-resource-refusal": "Refused by the native memory pool: Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs 64 MiB, 0 MiB available. No result was published."
  };
  for (const [shown, state] of [[refused, "refused"], [FAILED, "failed"], [{ "solve-job-summary": "state=cancelled; events=2; result_rows=0; cancellation_requested=true" }, "cancelled"]]) {
    const { hooks } = load({ shown, record: { resource: { queue_reason: null, refusal: state === "refused" ? growth : null } }, pool: POOL });
    const started = Date.now();
    // The timeout here is the test's bound: a wait that only times out fails the test.
    await assert.rejects(hooks.waitForSolve("demo/dense_scrutiny solve completed", hooks.solveDone("demo/dense_scrutiny", "project-1", null), 3000), (error) => {
      assert.doesNotMatch(error.message, /timed out/, state);
      assert.match(error.message, new RegExp(`^demo/dense_scrutiny: state=${state}; `), state);
      if (state === "refused") {
        assert.match(error.message, /refusal="Refused by the native memory pool: Not enough available memory on this Mac right now \(free plus reclaimable file cache and purgeable memory, less the kernel's reserve\): needs 64 MiB, 0 MiB available\. No result was published\."/);
        assert.match(error.message, /reason code=worker_growth_refused\/headroom_insufficient \(job refusal\); required_bytes=67108864; headroom_bytes=0/);
        assert.equal(error.solve_job.reason.code, "worker_growth_refused/headroom_insufficient");
      }
      return true;
    });
    assert.ok(Date.now() - started < 1000, `${state}: failed at once`);
  }
  // A job still cancelling has not ended: the wait goes on.
  const { hooks } = load({ shown: { "solve-job-summary": "state=cancelling; events=2; result_rows=0; cancellation_requested=true" } });
  assert.equal(hooks.solveDone("x", "project-1", null)(), false);
});

// Run 2 of the #1634 smoke at 4847f14b13: Create closed the units dialog while the
// blank's native create was still in flight, so the tree still showed the project
// opened before it. The driver read that row as the blank's id and authored at
// once; the first edit landed in the previous project and superseded the create.
test("after New Blank's Create, the blank's id is read only once the blank has replaced the previous project", async () => {
  const previous = "project:exact-pressure-connected-fixture";
  const blank = "project:blank-local-20261025t215818z";
  const projectRows = [previous];
  const { hooks, elements } = load({ shown: { "local-project-message": "Opened model document exact_pressure_authoring_model.json (6826 bytes; schema_version 0.8.0, current)." }, projectRows });
  const status = elements.get("local-project-message");
  const waited = hooks.awaitBlankProject(previous, 5000);
  // The create is still in flight: the tree shows the previous project.
  await new Promise((resolve) => setTimeout(resolve, 300));
  // The create's status line can be seen before the tree's next paint.
  status.textContent = "Created blank local model document without fixture entities or external file copies.";
  await new Promise((resolve) => setTimeout(resolve, 300));
  projectRows.splice(0, projectRows.length, blank);
  assert.equal(await waited, blank);
});

test("a blank create that failed ends the wait at once with its message; one that never lands times out naming the tree", async () => {
  const previous = "project:exact-pressure-connected-fixture";
  const { hooks } = load({ shown: { "local-project-message": "Blank create failed: Error: PROJECT-STORE-UNAVAILABLE" }, projectRows: [previous] });
  const started = Date.now();
  await assert.rejects(hooks.awaitBlankProject(previous, 3000), { message: /^Blank create failed: Error: PROJECT-STORE-UNAVAILABLE; message="Blank create failed/ });
  assert.ok(Date.now() - started < 1000);
  const superseded = load({ shown: { "local-project-message": "Opened model document exact_pressure_authoring_model.json." }, projectRows: [previous] });
  await assert.rejects(superseded.hooks.awaitBlankProject(previous, 150),
    { message: `timed out after 150 ms waiting for blank project in the tree in place of ${previous}; message="Opened model document exact_pressure_authoring_model.json."; tree projects=["${previous}"]` });
});
