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

function load({ shown = {}, record = null, pool = null, poolError = null, projectRows = [], onMenu = null } = {}) {
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
    if (command === "native_smoke_menu" && onMenu) { onMenu(args.commandId); return null; }
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
  const { hooks, elements } = load({ shown: { "local-project-message": "Opened model document exact_pressure_authoring_model.json (6826 bytes; schema_version 0.9.0, current)." }, projectRows });
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

// Run 1 of the A2 smoke at 594e437a09 (#1668): after the exact-route solve the driver
// chose View > Report twice in a row. Choosing the open page again closes it, so the
// second wait passed only when its first sample ran before the close was drawn; that
// run's did not, and it timed out after 120 s.
function sectionWindow(open) {
  const sections = new Map(["results", "report"].map((id) => [id, { textContent: "", getAttribute: () => null, offsetParent: null }]));
  const sent = [];
  const show = (id) => { for (const [key, element] of sections) element.offsetParent = key === id ? {} : null; };
  show(open);
  // The product's View > section command: a closed page opens, the open page closes.
  const onMenu = (commandId) => {
    sent.push(commandId);
    const id = commandId.slice("view.section.".length);
    show(sections.get(id).offsetParent === null ? id : null);
  };
  const { hooks, elements, logs } = load({ onMenu });
  for (const [id, element] of sections) elements.set(`workspace-section-${id}`, element);
  return { hooks, sections, sent, logs };
}

test("showing a section that is already shown sends no command, so the open page is not closed", async () => {
  const { hooks, sections, sent, logs } = sectionWindow("results");
  await hooks.showSection("report");
  await hooks.showSection("report");
  assert.deepEqual(sent, ["view.section.report"]);
  assert.notEqual(sections.get("report").offsetParent, null);
  assert.deepEqual(logs, ["show section report", "show section report (already shown)"]);
});

test("showing a hidden section sends the command and waits until the section shows", async () => {
  const sent = [];
  const element = { textContent: "", getAttribute: () => null, offsetParent: null };
  const { hooks, elements } = load({ onMenu: (commandId) => { sent.push(commandId); setTimeout(() => { element.offsetParent = {}; }, 250); } });
  elements.set("workspace-section-report", element);
  const started = Date.now();
  await hooks.showSection("report");
  assert.ok(Date.now() - started >= 200);
  assert.deepEqual(sent, ["view.section.report"]);
});

// The from-blank load step of the smoke at 15c1996957 (#1679, run 1): queuing the load case
// opened Review changes itself, but React draws a click's update in a microtask, after the
// driver's synchronous code. showOperations sampled the Load Cases stage before that, found
// no Review changes tab, and sent View > Review changes, which closes the open page: the wait
// timed out after 120 s. The window here draws a queued navigation the way React does.
function reviewWindow() {
  const element = (extra = {}) => ({ textContent: "", getAttribute: () => null, offsetParent: null, disabled: false, ...extra });
  const operations = element();
  const reviewTab = element();
  const sent = [];
  const show = (open) => { operations.offsetParent = open ? {} : null; reviewTab.offsetParent = open ? {} : null; };
  // The product's View > Review changes toggles the page, reading the state when it lands.
  // It lands after the queue's own navigation and, in the failing run, before the driver's
  // first sample (here: a microtask queued when the command is sent).
  const onMenu = (commandId) => {
    sent.push(commandId);
    if (commandId === "view.section.operations") queueMicrotask(() => show(operations.offsetParent === null));
  };
  const { hooks, elements } = load({ onMenu });
  elements.set("workspace-section-operations", operations);
  elements.set("workspace-review", Object.assign(reviewTab, { click: () => { sent.push("click workspace-review"); show(true); } }));
  // The Load Cases stage: no Review changes tab, and the page is closed.
  show(false);
  return { hooks, operations, reviewTab, sent, queueOpensReview: () => queueMicrotask(() => show(true)) };
}

test("Review changes that a queue has just opened is not toggled shut", { timeout: 5000 }, async () => {
  const { hooks, operations, sent, queueOpensReview } = reviewWindow();
  queueOpensReview();
  await hooks.showOperations();
  assert.deepEqual(sent, []);
  assert.notEqual(operations.offsetParent, null);
});

test("Review changes that is closed opens through its tab, once, and is waited for", { timeout: 5000 }, async () => {
  const { hooks, operations, sent, reviewTab } = reviewWindow();
  // The Model stage's tree: the tab is shown, the page is not; the tab only opens, after a while.
  reviewTab.offsetParent = {};
  reviewTab.click = () => { sent.push("click workspace-review"); setTimeout(() => { operations.offsetParent = {}; }, 150); };
  await hooks.showOperations();
  assert.deepEqual(sent, ["click workspace-review"]);
  assert.notEqual(operations.offsetParent, null);
});

test("closing a page waits until it has closed", { timeout: 5000 }, async () => {
  const close = { textContent: "", getAttribute: () => null, disabled: false, offsetParent: {}, click: () => { setTimeout(() => { close.offsetParent = null; }, 400); } };
  const { hooks, elements } = load();
  elements.set("workspace-dock-close", close);
  const started = Date.now();
  await hooks.closeSection();
  assert.equal(close.offsetParent, null);
  assert.ok(Date.now() - started >= 350);
});

test("the Inspector is pressed once and waited for until open", { timeout: 5000 }, async () => {
  let expanded = "false";
  let presses = 0;
  const toggle = { textContent: "", disabled: false, offsetParent: {}, getAttribute: (name) => (name === "aria-expanded" ? expanded : null),
    click: () => { presses += 1; setTimeout(() => { expanded = expanded === "true" ? "false" : "true"; }, 400); } };
  const { hooks, elements } = load();
  elements.set("toggle-inspector", toggle);
  await hooks.ensureInspector();
  assert.equal(expanded, "true");
  assert.equal(presses, 1);
  await hooks.ensureInspector();
  assert.equal(presses, 1);
});

// The first launch from main at 5f9e3959b: since 8f2b8c42e the Solve section's rule checks sit
// behind a gate that reads a native result's full rows only when the person clicks "Load for rule
// checks" (C2ResultGate.tsx, `explicit`). The driver never clicked it, so `rule-check-run` never
// existed and the 30 s wait timed out. Here a section holds its gates' elements as the product
// renders them; a gate's button, once clicked, renders its panel in place.
function gatedWindow({ onMenu = null } = {}) {
  const sent = [];
  const loaded = load({ onMenu: (commandId) => { sent.push(commandId); onMenu?.(commandId); } });
  const element = (testId, extra = {}) => ({ testId, textContent: "", disabled: false, offsetParent: {}, getAttribute: (name) => (name === "data-testid" ? testId : null), ...extra });
  const sections = new Map();
  const section = (name, shown = true) => {
    const children = [];
    const root = element(`workspace-section-${name}`, {
      offsetParent: shown ? {} : null,
      children,
      querySelectorAll: (selector) => { const id = /^\[data-testid="(.*)"\]$/.exec(selector)?.[1]; return children.filter((child) => child.testId === id); },
      querySelector: (selector) => root.querySelectorAll(selector)[0] ?? null
    });
    loaded.elements.set(root.testId, root);
    sections.set(name, root);
    return root;
  };
  // An element shown in a section, found by the section's own query and by its test id.
  const place = (root, child) => { root.children.push(child); loaded.elements.set(child.testId, child); return child; };
  // `which` is a test id or the element itself (a section can hold two gates' elements under one id).
  const remove = (root, which) => {
    const at = root.children.findIndex((child) => child === which || child.testId === which);
    const [gone] = root.children.splice(at, 1);
    if (loaded.elements.get(gone.testId) === gone) loaded.elements.delete(gone.testId);
  };
  // An explicit gate as C2ResultGate renders it before the person asks; the click renders `panel`.
  const explicitGate = (root, feature, panel) => {
    const offer = place(root, element("c2-result-explicit", { textContent: `The full result rows for ${feature} are read on request (they stay native until then). Load for ${feature}` }));
    const button = place(root, element("c2-result-load", { textContent: `Load for ${feature}`, clicks: 0 }));
    button.click = () => {
      button.clicks += 1;
      setTimeout(() => { remove(root, offer); remove(root, button); panel(); }, 150);
    };
    return button;
  };
  return { ...loaded, sent, element, section, sections, place, remove, explicitGate };
}

test("the rule-check gate is opened as a person opens it, and the panel is waited for", { timeout: 5000 }, async () => {
  const w = gatedWindow();
  const solve = w.section("solve");
  // Another feature's gate in the same section is not touched.
  const other = w.explicitGate(solve, "comparisons", () => {});
  const button = w.explicitGate(solve, "rule checks", () => w.place(solve, w.element("rule-check-run-panel")));
  const gate = await w.hooks.openC2Gate("solve", "rule checks", "rule-check-run-panel");
  assert.deepEqual(JSON.parse(JSON.stringify(gate)), { loaded_on_request: true });
  assert.equal(button.clicks, 1);
  assert.equal(other.clicks, 0);
  assert.deepEqual(w.sent, []);
  // Shown again for the same result, the panel is there and nothing is clicked.
  assert.deepEqual(JSON.parse(JSON.stringify(await w.hooks.openC2Gate("solve", "rule checks", "rule-check-run-panel"))), { loaded_on_request: false });
  assert.equal(button.clicks, 1);
});

test("a gate that reads the full copy by itself is waited for without a click", { timeout: 5000 }, async () => {
  const w = gatedWindow();
  const report = w.section("report");
  w.place(report, w.element("c2-result-loading", { textContent: "Reading the full result for the rendered report…" }));
  setTimeout(() => { w.remove(report, "c2-result-loading"); w.place(report, w.element("rendered-report-panel")); }, 300);
  const started = Date.now();
  assert.deepEqual(JSON.parse(JSON.stringify(await w.hooks.openC2Gate("report", "the rendered report", "rendered-report-panel"))), { loaded_on_request: false });
  assert.ok(Date.now() - started >= 250);
});

test("a gate that refused or failed ends the wait at once with its own text; another feature's refusal does not", { timeout: 5000 }, async () => {
  const refused = "RESULT-DETAIL-WEBVIEW-LIMIT: This result is 80 MiB, above the 64 MiB the webview can hold whole, so rule checks cannot read it until its native path lands.";
  const w = gatedWindow();
  const solve = w.section("solve");
  w.explicitGate(solve, "rule checks", () => w.place(solve, w.element("c2-result-refused", { textContent: refused })));
  const started = Date.now();
  await assert.rejects(w.hooks.openC2Gate("solve", "rule checks", "rule-check-run-panel", 3000), (error) => {
    assert.match(error.message, /^rule checks: RESULT-DETAIL-WEBVIEW-LIMIT: /);
    assert.match(error.message, /clicked=true$/);
    return true;
  });
  assert.ok(Date.now() - started < 1000);
  const v = gatedWindow();
  const report = v.section("report");
  v.place(report, v.element("c2-result-failed", { textContent: "the report could not read the full result: boom" }));
  v.place(report, v.element("c2-result-loading", { textContent: "Reading the full result for the rendered report…" }));
  await assert.rejects(v.hooks.openC2Gate("report", "the rendered report", "rendered-report-panel", 300), (error) => {
    assert.match(error.message, /^timed out after 300 ms waiting for the rendered report: rendered-report-panel in the report section; gate: reading=true; offered=false; load button=false; clicked=false$/);
    return true;
  });
});

test("rule checks: through the gate, the demo pack is loaded, the run made and its outcome read", { timeout: 8000 }, async () => {
  const w = gatedWindow({ onMenu: (commandId) => {
    for (const [name, root] of w.sections) root.offsetParent = commandId === `view.section.${name}` ? {} : null;
  } });
  const solve = w.section("solve");
  const results = w.section("results", false);
  const status = w.element("rule-check-run-status");
  const run = w.element("rule-check-run", { disabled: true });
  w.explicitGate(solve, "rule checks", () => {
    w.place(solve, w.element("rule-check-run-panel"));
    w.place(solve, status);
    w.place(solve, run);
    w.place(solve, w.element("rule-check-load-demo", { click: () => setTimeout(() => { status.textContent = "Loaded the bundled invented demonstration rule pack."; run.disabled = false; }, 100) }));
  });
  run.click = () => setTimeout(() => {
    w.place(solve, w.element("rule-check-aggregate-status", { textContent: "Aggregate statusINCOMPLETE — 0 check(s) failed; 1 check(s) blocked (RULE_INPUTS_INCOMPLETE)" }));
    w.place(solve, w.element("rule-check-outcomes", { textContent: "demo_training_check: RULE_INPUTS_INCOMPLETE inputs: demo_actual_quantity=MISSING, demo_limit_quantity=MISSING" }));
  }, 100);
  // The Results section's stress and ratio table, holding the run's rule group.
  const rows = [{ getAttribute: (name) => ({ "data-part": "pinned", "data-status": "RULE_INPUTS_INCOMPLETE" })[name] ?? null }];
  w.place(results, w.element("stress-ratio-panel"));
  w.place(results, w.element("stress-ratio-table", { querySelectorAll: () => rows, querySelector: () => ({}) }));
  w.place(results, w.element("stress-ratio-group", { value: "rule:demo_training_check", querySelectorAll: (selector) => (selector.includes("rule:") ? [{}] : []) }));
  w.place(results, w.element("stress-ratio-mode", { getAttribute: (name) => (name === "data-mode" ? "ratio" : null) }));
  const observation = await w.hooks.runRuleChecks("demo/rule-checks");
  assert.equal(observation.gate.loaded_on_request, true);
  assert.match(observation.aggregate, /RULE_INPUTS_INCOMPLETE/);
  assert.match(observation.outcomes, /demo_actual_quantity=MISSING, demo_limit_quantity=MISSING/);
  assert.equal(observation.stress_ratio.rule_groups, 1);
  assert.equal(observation.stress_ratio.pinned_incomplete, 1);
  assert.deepEqual(w.sent, ["view.section.results"]);
});

// The same launch's phase 2: with no blank project, the handoff asked to reopen the stored demo
// while it was the open project, and waited 120 s on a tree that cannot show it both ways.
test("reopening the project already open fails at once, naming it, and asks the window nothing", { timeout: 3000 }, async () => {
  const w = gatedWindow();
  const started = Date.now();
  await assert.rejects(w.hooks.openProjectById("project:invented-demo-loop-01", "project:invented-demo-loop-01"),
    { message: /^reopen project:invented-demo-loop-01: it is already the open project; / });
  assert.ok(Date.now() - started < 500);
  assert.deepEqual(w.sent, []);
});

test("without a blank project, the handoff uses the stored demo it just reopened instead of reopening it", { timeout: 3000 }, async () => {
  const w = gatedWindow();
  const plan = { bundled_project_id: "project:invented-demo-loop-01", handoff: { path: "/tmp/handoff" } };
  // The fake host answers no chooser: the round trip stops at its first save, past the basis step.
  await assert.rejects(w.hooks.handoffRoundTrip(plan, {}, plan.bundled_project_id, plan.bundled_project_id), { message: "unexpected command native_smoke_choose" });
  assert.deepEqual(w.sent, []);
  assert.ok(w.logs.includes("handoff basis: already open: project:invented-demo-loop-01 was the last project reopened from the store in this phase"));
});

// Since e8a322ac6 a Run is withheld while a solve-blocking model-check finding stands: no job
// starts, and the Solve panel says why. The wait ends at once on that statement, not after 300 s.
test("a withheld Run ends the solve wait at once with the Solve panel's reason; a completed solve is not failed by it", async () => {
  const withheld = "Solve is withheld: 1 solve-blocking finding stands for this model, listed in Model checks in the Issues drawer.\n  SUPPORT_MISSING …";
  const { hooks, elements } = load({ shown: { "solve-job-summary": "state=completed; events=3; result_rows=12; cancellation_requested=false", "solve-withheld-notice": withheld } });
  const started = Date.now();
  await assert.rejects(hooks.waitForSolve("blank solve", hooks.solveDone("blank", "project-1", null, true), 3000), (error) => {
    assert.match(error.message, /^blank: Solve is withheld: 1 solve-blocking finding stands for this model, listed in Model checks in the Issues drawer\. SUPPORT_MISSING …; solve job: /);
    assert.equal(error.hard, true);
    return true;
  });
  assert.ok(Date.now() - started < 1000);
  // The new proof of the project is there: the solve completed, whatever else is shown.
  elements.set("status-pill-solve-proof", { textContent: "", getAttribute: () => null, querySelector: () => ({ textContent: "rows=12; project=project-1; identity=match" }) });
  assert.equal(hooks.solveDone("blank", "project-1", null, true)(), true);
});
