// Desktop performance driver (RT-U4,
// DEL-04-05). `perf.mjs` launches the `native-smoke` build of the app,
// which evaluates this script in the real macOS webview once the page has loaded
// (`src-tauri/src/native_smoke.rs`). One launch is one run: one model, one mode.
//
// It works the app as `driver.js` does (native menu commands through the menu's own
// dispatch, buttons clicked, the model-document chooser answered with the plan's
// file) and records what it saw, verbatim; `perf.mjs` classifies. Steps:
//   1. File > Open Model Document… with the plan's model;
//   2. Analyze > Run (sparse), then wait for the first paint of the result or a
//      failure, with a timeout, reading the solve timeline (timings, counts and codes)
//      from `__openPipeStressSolveTimelineV1.readCurrent()`. A C1 run opens Results once
//      the solve has begun: a native result's first paint is its Results table's first
//      page (rows and row semantics) committed, then two frames, so Results must be shown;
//   3. Record Run… (Project page, Model History), then File > Save Local Project,
//      each outcome as shown.
// Progress lines go to the app's stderr as `[native-smoke] perf:{json}` (a crash
// of the webview still leaves them); the report goes through `native_smoke_finish`.
// A C1 run first checks that its command observation completed an IPC round trip and that
// the page is shown and delivers animation frames (before the model opens and again
// before Analyze > Run); otherwise it stops without starting a solve. During the
// solve it records and journals what the page showed (`journal`, `observePage`). A
// later loss of the page's visibility (the host condition) or of the observed IPC
// transport stops the run at its next wait.
(() => {
  const ipc = window.__TAURI_INTERNALS__;
  const call = (command, args) => ipc.invoke(command, args ?? {});
  const log = (line) => call("native_smoke_log", { line: String(line) }).catch(() => {});
  const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
  const byTestId = (id, root = document) => root.querySelector(`[data-testid="${CSS.escape(id)}"]`);
  const text = (id, root = document) => byTestId(id, root)?.textContent?.trim() ?? null;
  const message = () => text("local-project-message") ?? "";
  const historyMessage = () => text("model-history-message") ?? "";
  const now = () => performance.now();
  let runId = null;
  let resourceFailure = null;
  // C1 only. `hostFailure`: the host condition (below) was lost; `observationFailure`: the command
  // observation lost Tauri's IPC (`beginC1Trace`). Either stops every wait at its next poll.
  let hostFailure = null;
  let observationFailure = null;
  const progress = (step, detail = {}) => log(`perf:${JSON.stringify({ run_id: runId, step, at_ms: now(), ...detail })}`);
  function stopIfLost() {
    if (hostFailure) throw hostFailure;
    if (observationFailure) throw observationFailure;
    if (resourceFailure) throw new Error(resourceFailure);
  }

  // As `driver.js`: a probe that throws ends the wait with its own error.
  async function waitFor(label, probe, timeoutMs, intervalMs = 100) {
    const started = Date.now();
    for (;;) {
      stopIfLost();
      const value = probe();
      if (value) return value;
      if (Date.now() - started > timeoutMs) throw Object.assign(new Error(`timed out after ${timeoutMs} ms waiting for ${label}`), { timeout: true });
      await sleep(intervalMs);
    }
  }
  // The paint gate's wait (RT-U4 F1): the same probe-first loop as `waitFor`, against an
  // absolute, unrounded `performance.now()` deadline instead of a duration from its own
  // start, sleeping no later than the deadline. Probing first only lets the wait notice what
  // already happened; whether a first paint was in time is its mark's own timestamp (`solve`).
  async function waitUntil(label, probe, deadline, budgetMs, intervalMs) {
    for (;;) {
      stopIfLost();
      const value = probe();
      if (value) return value;
      const left = deadline - now();
      if (left <= 0) throw Object.assign(new Error(`timed out after ${budgetMs} ms waiting for ${label}`), { timeout: true });
      await sleep(Math.min(intervalMs, left));
    }
  }
  const menu = (commandId) => call("native_smoke_menu", { commandId });
  function click(id, root = document) {
    const element = byTestId(id, root);
    if (!element) throw new Error(`no element ${id}`);
    if (element.disabled || element.getAttribute("aria-disabled") === "true") throw new Error(`${id} is disabled`);
    element.click();
  }
  function fill(id, value, root = document) {
    const element = byTestId(id, root);
    if (!element) throw new Error(`no element ${id}`);
    const prototype = element instanceof HTMLTextAreaElement ? HTMLTextAreaElement.prototype : HTMLInputElement.prototype;
    Object.getOwnPropertyDescriptor(prototype, "value").set.call(element, value);
    element.dispatchEvent(new Event("input", { bubbles: true }));
  }
  const treeRow = (projectId) => byTestId(`tree-row-project-${encodeURIComponent(projectId)}`);
  const sectionShown = (section) => {
    const element = byTestId(`workspace-section-${section}`);
    return Boolean(element && element.offsetParent !== null);
  };
  async function showSection(section, timeoutMs) {
    await sleep(0);
    if (sectionShown(section)) return;
    await menu(`view.section.${section}`);
    await waitFor(`section ${section}`, () => sectionShown(section), timeoutMs);
  }
  const timeline = () => globalThis.__openPipeStressSolveTimelineV1?.readCurrent?.() ?? null;
  const elapsed = (started) => Math.round(now() - started);

  // WebKit runs no rendering update, so no
  // animation frame, for a page it does not consider shown (window occluded, display asleep,
  // screen locked), and aligns that page's nested timers to 1 s. The smoke window's
  // `backgroundThrottling: "disabled"` only sets WKPreferences.inactiveSchedulingPolicy (wry), not
  // page visibility. A first paint is two animation frames after the first page, so on a page that
  // is not shown it cannot be observed at all. `frames` asks for two frames and reports what came.
  // Its timeout is a scheduled JS timer: it fires no earlier than `timeoutMs`, but WebKit may
  // defer it (1 s alignment on a hidden page, or a busy or suspended WebContent), so it is no
  // wall-clock bound. `elapsed_ms` is what the wait took; the hard bound on a run is the runner's
  // own App deadline (`perf.mjs`, the run's `timeout_s`), outside the webview.
  const visibility = () => (typeof document.visibilityState === "string" ? document.visibilityState : null);
  function frames(timeoutMs) {
    const started = now();
    return new Promise((resolve) => {
      let seen = 0, done = false, timer = null;
      const finish = () => {
        if (done) return;
        done = true;
        if (timer !== null) clearTimeout(timer);
        resolve({ visibility: visibility(), frames: seen, timeout_ms: timeoutMs, elapsed_ms: elapsed(started) });
      };
      timer = setTimeout(finish, timeoutMs);
      if (typeof requestAnimationFrame !== "function") return;
      requestAnimationFrame(() => { seen = 1; requestAnimationFrame(() => { seen = 2; finish(); }); });
    });
  }

  // The C1 host condition (Agent 0, 2026-11-05): the display awake, the session unlocked and the
  // App's window uncovered and visible, delivering animation frames, for the whole observation.
  // The driver only checks it: it unlocks nothing, asks for no credential or permission, changes
  // no setting and never ends another App. Without it nothing can be observed, so a failed check
  // or a later loss of visibility stops the run at once, as unobservable: that is no evidence of a
  // product hang, and the run is not retried. `report.c1.host` records the outcome (scalars only).
  let hostRecord = null;
  let stopHostWatch = null;
  function hostError(step, detail, solveStarted) {
    if (hostRecord && hostRecord.established !== false) Object.assign(hostRecord, { established: false, step, detail, at_ms: now(), solve_started: solveStarted });
    return Object.assign(new Error(`C1 host condition not established (${step}: ${detail}). The display must be awake, the session unlocked and the App window uncovered and visible, delivering frames, for the whole observation. The run is unobservable: this is not evidence of a product hang, ${solveStarted ? "the solve is cancelled" : "no solve was started"}, and the run is not retried.`), { host: true });
  }
  let solveStarted = false;
  function watchHost(record) {
    hostRecord = record;
    const check = () => {
      if (!hostFailure && visibility() !== "visible") hostFailure = hostError("during the observation", `the page became visibilityState=${visibility()}`, solveStarted);
    };
    document.addEventListener("visibilitychange", check);
    stopHostWatch = () => document.removeEventListener("visibilitychange", check);
    return check;
  }
  // A C1 run starts its solve only on a shown page that delivers frames: otherwise the run could
  // only time out at the paint gate. Checked before the model opens and again before Analyze > Run.
  async function requireShownPage(step, timeoutMs = 2000) {
    const probe = await frames(timeoutMs);
    stopIfLost();
    if (probe.visibility !== "visible" || probe.frames < 2) {
      throw hostError(step, `visibilityState=${probe.visibility}, animation frames ${probe.frames}/2 within a ${timeoutMs} ms timer that fired after ${probe.elapsed_ms} ms`, solveStarted);
    }
    return probe;
  }

  // C1 observation journal (driver side): the fixed web phases `results_open`, `first_page`,
  // `paint_frames`, `page_hidden` and `frame_wait`, as `reader-phase:` lines like the app's own
  // awaits (`solveTimeline.ts` `traceNativeReaderAwait`). Scalars only: no job, model, row or
  // identity. Sent one after another, so the journal never sees an exit before its entry; an
  // entry without an exit stays missing. At most 16 calls per phase (well inside the journal's
  // fixed 256 entries beside the app's own); past that nothing more is sent for that phase.
  let journalOn = false;
  let journalChain = Promise.resolve();
  const journalCalls = {};
  const journalOpen = new Set();
  function journal(phase, edge, status) {
    if (!journalOn) return;
    if (edge === "enter") {
      if (journalOpen.has(phase)) return;
      const next = (journalCalls[phase] ?? 0) + 1;
      if (next > 16) return;
      journalCalls[phase] = next;
      journalOpen.add(phase);
    } else {
      if (!journalOpen.has(phase)) return;
      journalOpen.delete(phase);
    }
    const event = { layer: "web", call: journalCalls[phase], phase, edge, status, mono_ms: now() };
    journalChain = journalChain.then(() => call("native_smoke_log", { line: `reader-phase:${JSON.stringify(event)}` })).catch(() => {});
  }
  const journalFlushed = () => journalChain;

  async function openModel(plan, report) {
    const name = plan.model_path.split("/").pop();
    const started = now();
    const before = message();
    await call("native_smoke_choose", { path: plan.model_path });
    await menu("file.open-model");
    const shown = await waitFor(`open ${name}`, () => {
      const current = message();
      return current !== before && (current.includes(`Opened model document ${name} `) || current.includes(`refused (${name})`)) ? current : null;
    }, plan.timeouts.open_ms);
    report.open = { message: shown, elapsed_ms: elapsed(started) };
    if (!shown.includes("Opened model document")) throw new Error(`open refused: ${shown}`);
    await waitFor(`${plan.project_id} in the model tree`, () => treeRow(plan.project_id), plan.timeouts.open_ms);
    report.open.elapsed_ms = elapsed(started);
    await progress("opened", { elapsed_ms: report.open.elapsed_ms });
  }

  const TERMINAL = new Set(["failed", "refused", "cancelled"]);
  const UNSOLVED_STATE = /\bstate=(failed|cancelled|refused)(?:;|$)/;
  function solveShown() {
    return {
      summary: text("solve-job-summary"),
      // The Solve section is a DormantSection (App.tsx): while Results is shown its summary is the
      // last render before it went dormant, not the job's current state (the A+B record).
      summary_live: sectionShown("solve"),
      error: text("solve-job-error"),
      refusal: text("solve-job-resource-refusal"),
      worker_failure: text("solve-job-worker-failure"),
      results_refusal: text("result-query-refused", byTestId("workspace-section-results") ?? document),
      progress: text("solve-job-progress"),
      queue_reason: text("solve-job-queue-reason"),
      proof: byTestId("status-pill-solve-proof")?.querySelector("code")?.textContent ?? null
    };
  }

  // Analyze > Run, then the first paint of its result, a failure, or the timeout. The
  // marks are the app's own clock readings; the 200 ms poll only bounds when the wait
  // notices them (and keeps its own load on the webview's main thread small).
  // The paint gate (RT-U4 F1) is one absolute deadline on the webview's monotonic clock,
  // `paint_ms` after Analyze > Run is sent, unrounded. Every wait in this step, including a
  // C1 run's opening of Results, is bounded by what remains of it, and a first paint counts
  // only when its own `first_paint` mark is at or before the deadline: a timely mark the
  // wait notices late still counts, a late mark is a timeout however it is noticed.
  async function solve(plan, report) {
    await showSection("solve", plan.timeouts.open_ms);
    click("solver-mode-sparse");
    await waitFor("sparse selected", () => byTestId("solver-mode-sparse")?.getAttribute("aria-pressed") === "true", 10000);
    const priorSequence = timeline()?.sequence ?? 0;
    const paintMs = plan.timeouts.paint_ms;
    const started = now();
    const deadline = started + paintMs;
    solveStarted = true;
    await menu("analyze.run");
    let lastStage = null;
    let heartbeat = 0;
    let ended = "timeout";
    const begun = () => (timeline()?.sequence ?? 0) > priorSequence;
    const remaining = () => Math.max(0, deadline - now());
    // This run's first paint mark, if any, and whether it is within the gate.
    const paintMark = () => {
      const current = timeline();
      return current && current.sequence > priorSequence ? current.marks.find((mark) => mark.stage === "first_paint") ?? null : null;
    };
    const timely = (mark) => Boolean(mark) && typeof mark.at_ms === "number" && mark.at_ms <= deadline;
    // What the page showed (C1): whether it was shown, whether animation frames came, and when
    // the native table's first page was first seen settled with rows (its rows and their row
    // semantics read: NativeResultsPanel renders rows only from a ready page).
    const page = { visibility_at_run: visibility(), hidden_polls: 0, frame_probes: 0, frame_acks: 0, first_page_seen_at_ms: null, first_page_refused: false, visibility_at_end: null };
    let frameProbe = null;
    const probeFrame = () => {
      if (typeof requestAnimationFrame !== "function") return;
      if (frameProbe && !frameProbe.acked) journal("frame_wait", "enter", "pending");
      const probe = { acked: false };
      frameProbe = probe;
      page.frame_probes++;
      requestAnimationFrame(() => {
        probe.acked = true;
        page.frame_acks++;
        journal("frame_wait", "exit", "ok");
      });
    };
    const observePage = (current) => {
      if (!plan.c1_probes) return;
      if (visibility() === "visible") journal("page_hidden", "exit", "ok");
      else {
        page.hidden_polls++;
        journal("page_hidden", "enter", "pending");
        // A page that is not shown ends the observation (the host condition), at this poll.
        hostFailure ??= hostError("during the solve", `the page was visibilityState=${visibility()} at a poll`, true);
      }
      if (page.first_page_seen_at_ms !== null || page.first_page_refused) return;
      if (!current.marks.some((mark) => mark.stage === "result_committed")) return;
      journal("first_page", "enter", "pending");
      const results = byTestId("workspace-section-results");
      if (text("result-query-refused", results ?? document)) {
        page.first_page_refused = true;
        journal("first_page", "exit", "error");
        return;
      }
      const panel = byTestId("results-panel", results ?? document);
      if (panel && panel.getAttribute("data-result-custody") === "native" && !byTestId("result-page-reading", panel) && rowIds(panel).length > 0) {
        page.first_page_seen_at_ms = now();
        journal("first_page", "exit", "ok");
        journal("paint_frames", "enter", "pending");
      }
    };
    journalOn = Boolean(plan.c1_probes && plan.c1_reader_diagnostics);
    try {
      if (plan.c1_probes) {
        // Analyze > Run shows Solve; the result is seen in Results (the native table's first
        // page gates first_paint). Opened once the run has begun, so the run's own section
        // change cannot hide it again.
        await waitUntil("solve begun", () => begun() || UNSOLVED_STATE.test(text("solve-job-summary") ?? ""), deadline, paintMs, 50);
        // Bounded by the gate's remainder as well as `open_ms`: when the remainder is the
        // smaller, running out of it is the gate's timeout, not a failure to open Results.
        const openMs = Math.min(plan.timeouts.open_ms, remaining());
        journal("results_open", "enter", "pending");
        try { await showSection("results", openMs); journal("results_open", "exit", "ok"); }
        catch (error) {
          journal("results_open", "exit", "error");
          if (error?.host || error?.observation || (error?.timeout && openMs < plan.timeouts.open_ms)) throw error;
          throw new Error(`C1 results did not open: ${error?.message ?? error}`);
        }
      }
      ended = await waitUntil("first paint or failure", () => {
        const current = timeline();
        if (current && current.sequence > priorSequence) {
          const stage = current.marks.at(-1)?.stage ?? null;
          if (stage !== lastStage || now() - heartbeat > 5000) {
            lastStage = stage;
            heartbeat = now();
            if (plan.c1_probes) probeFrame();
            void progress("solve", { stage, outcome: current.outcome, elapsed_ms: elapsed(started), ...(plan.c1_probes ? { visibility: visibility(), frame_probes: page.frame_probes, frame_acks: page.frame_acks, first_page: page.first_page_seen_at_ms !== null } : {}) });
          }
          observePage(current);
          const paint = paintMark();
          if (paint) { journal("paint_frames", "exit", "ok"); return timely(paint) ? "first_paint" : "timeout"; }
          if (TERMINAL.has(current.outcome) || current.outcome === "superseded") return current.outcome;
          // A refused first page never paints: it ends the wait rather than run out the gate.
          if (plan.c1_probes && text("result-query-refused", byTestId("workspace-section-results") ?? document)) return "results_refused";
        }
        if (UNSOLVED_STATE.test(text("solve-job-summary") ?? "")) return "ui_unsolved";
        return null;
      }, deadline, paintMs, 200);
    } catch (error) {
      // The host condition lost: the solve is unobservable, whatever its marks (no paint, no hang).
      if (error?.host) ended = "host_unobservable";
      else if (!error?.timeout) throw error;
      // Whichever wait ran out, a mark already within the gate is still the run's first paint.
      else if (timely(paintMark())) ended = "first_paint";
    }
    const paint = paintMark();
    page.visibility_at_end = visibility();
    report.solve = {
      ended, elapsed_ms: elapsed(started),
      // The gate as the driver applied it, on the timeline's clock (`performance.now()`).
      gate: { paint_ms: paintMs, started_at_ms: started, deadline_at_ms: deadline, first_paint_at_ms: paint?.at_ms ?? null, first_paint_timely: paint ? timely(paint) : null },
      shown: solveShown(), timeline: timeline(),
      ...(plan.c1_probes ? { page } : {})
    };
    await journalFlushed();
    await progress("solved", { ended, elapsed_ms: report.solve.elapsed_ms });
  }

  // Record Run… on the Project page's Model History, with a run name (and a
  // post-solve snapshot name when the form asks for one). Record Run saves the local
  // project first when it is not stored yet, as a person's Record Run does.
  async function recordRun(plan, report) {
    const started = now();
    const outcome = { attempted: false, message: null, elapsed_ms: null };
    report.record_run = outcome;
    await showSection("project", plan.timeouts.open_ms);
    await waitFor("model history panel", () => byTestId("model-history-panel"), plan.timeouts.open_ms);
    const unavailable = text("record-run-unavailable");
    const open = byTestId("record-run-open");
    if (unavailable || !open || open.disabled) {
      outcome.message = unavailable ?? open?.getAttribute("title") ?? "Record Run… not shown";
      outcome.elapsed_ms = elapsed(started);
      return;
    }
    click("record-run-open");
    await waitFor("record run form", () => byTestId("record-run-form"), 10000);
    fill("record-run-name", `perf ${plan.run_id}`);
    if (byTestId("record-run-snapshot-name")) fill("record-run-snapshot-name", `perf snapshot ${plan.run_id}`);
    await waitFor("record run enabled", () => byTestId("record-run-submit")?.disabled === false, 10000);
    const before = historyMessage();
    const interim = "A recorded run needs its project's stored copy: saving the local project first.";
    outcome.attempted = true;
    click("record-run-submit");
    await progress("record_run_submitted");
    outcome.prompts = [];
    try {
      outcome.message = await waitFor("record run outcome", () => {
        if (byTestId("replace-project-prompt")) {
          outcome.prompts.push(text("replace-project-detail"));
          click("replace-project-cancel");
        }
        const current = historyMessage();
        return current && current !== before && current !== interim && (current.startsWith("Not recorded") || !byTestId("record-run-form")) ? current : null;
      }, plan.timeouts.record_ms, 100);
    } catch (error) {
      if (!error?.timeout) throw error;
      outcome.message = null;
      outcome.timed_out = true;
      outcome.last_message = historyMessage();
    }
    outcome.project_message = message();
    outcome.elapsed_ms = elapsed(started);
    await progress("record_run", { message: outcome.message, elapsed_ms: outcome.elapsed_ms });
  }

  // File > Save Local Project, answered as shown (a replace prompt is cancelled and recorded).
  // Every step keeps its own record in `report.save_projects` (RT-U4, 82-node lesson: a later
  // save once replaced an earlier step's timed-out record); `report.save_project` is the first
  // step's, never replaced. With the C1 trace, a step ends on its OWN native save: the first save
  // invocation sent after this step's menu command, once its reply is read (or refused). The
  // status line is recorded but does not decide the end, because two consecutive saves can show
  // the same text (Record Run's prerequisite save, then File > Save). A save receipt from an
  // earlier invocation never ends a later step, and a step that sees two invocations fails.
  // Without the trace (plain plans), the status line's change ends it, as before.
  async function saveProject(plan, report, trace = null) {
    const started = now();
    const steps = report.save_projects ??= [];
    const outcome = { step: steps.length + 1, message: null, prompted: null, elapsed_ms: null, receipt: null };
    steps.push(outcome);
    report.save_project ??= outcome;
    const before = message();
    const sent = trace ? trace.data.saves.length : 0;
    await menu("file.save-local");
    try {
      outcome.message = await waitFor("save outcome", () => {
        if (byTestId("replace-project-prompt") && outcome.prompted === null) {
          outcome.prompted = text("replace-project-detail");
          click("replace-project-cancel");
        }
        const current = message();
        const terminal = /^(Saved|Created|Not saved|Save failed|Create failed)|SQLite project snapshot/.test(current);
        if (!trace) return current !== before && terminal ? current : null;
        const own = trace.data.saves.slice(sent);
        if (own.length > 1) throw new Error(`C1 one File > Save sent ${own.length} native saves`);
        const invocation = own[0];
        if (invocation && (invocation.refusal !== null || invocation.error !== null)) return current || "native save refused";
        if (invocation?.read && terminal) return current;
        // A cancelled replace prompt sends no save; its own refusal text ends the step.
        if (!invocation && outcome.prompted !== null && current !== before && terminal) return current;
        return null;
      }, plan.timeouts.save_ms, 100);
    } catch (error) {
      if (!error?.timeout) throw error;
      outcome.timed_out = true;
      outcome.last_message = message();
    }
    if (trace) outcome.receipt = trace.data.saves[sent] ? { ...trace.data.saves[sent] } : null;
    outcome.elapsed_ms = elapsed(started);
    await progress("save_project", { step: outcome.step, message: outcome.message, elapsed_ms: outcome.elapsed_ms, timed_out: outcome.timed_out === true, receipt_seq: outcome.receipt?.seq ?? null });
    return outcome;
  }

  // Bounded reads of a reply body (C1 observation). Each chunk is checked before any of it is kept,
  // only the bytes needed are copied (never a whole chunk), and the stream is cancelled as soon as
  // the bound is reached. `note(bytes)` is told the size of each copy the observation holds.
  const boundError = (what, limit) => Object.assign(new Error(`probe ${what} exceeds its ${limit}-byte bound`), { bound: true });
  function joined(parts, length) {
    const out = new Uint8Array(length);
    let at = 0;
    for (const part of parts) { out.set(part, at); at += part.byteLength; }
    return out;
  }
  // The whole body, refused (and cancelled) once it would pass `limit` bytes.
  async function cappedBody(response, limit, note = () => {}) {
    if (!response.body) return new Uint8Array(0);
    const reader = response.body.getReader();
    const parts = [];
    let have = 0;
    try {
      for (;;) {
        const { value, done } = await reader.read();
        if (done) break;
        if (value.byteLength > limit - have) throw boundError("reply", limit);
        parts.push(value.slice());
        have += value.byteLength;
      }
    } finally { reader.cancel().catch(() => {}); note(have); }
    return joined(parts, have);
  }
  // At most the first `limit` bytes, and whether the body had more; the rest is never read.
  async function bodyPrefix(response, limit, note = () => {}) {
    if (!response.body) return { bytes: new Uint8Array(0), truncated: false };
    const reader = response.body.getReader();
    const out = new Uint8Array(limit);
    let have = 0, truncated = false;
    try {
      while (!truncated) {
        const { value, done } = await reader.read();
        if (done) break;
        const take = Math.min(value.byteLength, limit - have);
        out.set(value.subarray(0, take), have);
        have += take;
        truncated = take < value.byteLength;
        // At exactly `limit` bytes, one more read tells a complete reply from a longer one (review:
        // documented lookahead). It retains nothing (the chunk is dropped unread), so the bound on
        // copied bytes holds; only the observation may wait for that one further chunk or the end.
        if (have === limit && !truncated) truncated = !(await reader.read()).done;
      }
    } finally { reader.cancel().catch(() => {}); note(limit); }
    return { bytes: out.subarray(0, have), truncated };
  }
  // A native row reply's header (`SWBDETv1`, a u64 length, at most 64 KiB of JSON): only its bytes
  // are copied; the row page behind it is never read.
  const HEADER_LIMIT = 65536;
  async function rowHeader(response, note = () => {}) {
    if (!response.body) throw new Error("probe rows have no native header");
    const reader = response.body.getReader();
    let buffer = new Uint8Array(16), have = 0, need = 16;
    try {
      while (have < need) {
        const { value, done } = await reader.read();
        if (done) break;
        let offset = 0;
        while (offset < value.byteLength && have < need) {
          const take = Math.min(value.byteLength - offset, need - have);
          buffer.set(value.subarray(offset, offset + take), have);
          have += take;
          offset += take;
          if (need === 16 && have === 16) {
            if (String.fromCharCode(...buffer.subarray(0, 8)) !== "SWBDETv1") throw new Error("probe rows have no native header");
            const length = new DataView(buffer.buffer, 0, 16).getBigUint64(8, true);
            if (length > BigInt(HEADER_LIMIT)) throw new Error("probe native header is not bounded");
            need = 16 + Number(length);
            const grown = new Uint8Array(need);
            grown.set(buffer);
            buffer = grown;
          }
        }
      }
    } finally { reader.cancel().catch(() => {}); note(buffer.byteLength); }
    if (have < 16) throw new Error("probe rows have no native header");
    if (have < need) throw new Error("probe native header is not bounded");
    return JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(buffer.subarray(16, need)));
  }
  const RECEIPT_LIMIT = 4 << 20;
  const REFUSAL_LIMIT = 2048;
  // A receipt as the App reads it: a JSON reply or a raw `tauri::ipc::Response` body, both JSON
  // bytes, of at most 4 MiB.
  async function receiptOf(response, note) {
    const bytes = await cappedBody(response, RECEIPT_LIMIT, note);
    return JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
  }

  // C1-only observation: command names, bounded headers and small receipts.
  // Never retain a result body, record, row page or CSV chunk in this trace.
  // Tauri defines `__TAURI_INTERNALS__.invoke` (and `ipc`, `postMessage`) with
  // Object.defineProperty, so they are not writable: assigning a wrapper there
  // changes nothing (A+B diagnosis: every earlier trace observed no command).
  // Tauri 2.11.1's `ipc-protocol.js` sends every invoke as one `fetch` of
  // `ipc://localhost/<command>`, so the observation wraps `window.fetch` and
  // passes every other request through untouched. A command's reply is read
  // from a clone only where a receipt or a row header is needed; a refusal is
  // answered as Tauri answers one (`Tauri-Response: error`), so the App's
  // promise rejects and no transport fallback is taken.
  //
  // The fallback: after any failure of that fetch, or of reading its reply, the same script
  // switches for good to `window.ipc.postMessage` (wry; defined frozen, so not replaceable), which
  // the fetch wrapper cannot see. The observation therefore records every sign of that switch (the
  // fetch rejected, the App's read of the reply failed, Tauri's own warning before it falls back,
  // a command sent over postMessage) as `transport.lost`. That is final for the page: the run stops
  // at its next wait and its command evidence fails. Where the WebKit message handler's
  // `postMessage` can be wrapped (`transport.fallback_observer`), commands sent over the fallback
  // are also counted, and a forbidden one is refused through Tauri's error callback instead of
  // being sent. A round trip counts as observed (`completed`) only when the App's read of the reply
  // through the observed fetch succeeded.
  function beginC1Trace() {
    const IPC_PREFIX = "ipc://localhost/";
    const originalFetch = window.fetch;
    const data = { counts: {}, completed: {}, errors: [], violations: [], refusals: [], refusals_omitted: 0, disposed: {}, csv: null, open: null, recordedRead: null, record: null, save: null, restoredSave: null, saves: [],
      transport: { lost: false, events: [], events_omitted: 0, fallback_observer: null, fallback_counts: {} }, reply_bytes_max: 0 };
    const forbidden = new Set(["take_preview_mechanics_job_result", "read_preview_mechanics_job_rows_all", "read_restored_result_record_all", "open_local_project", "read_analysis_run", "record_analysis_run", "save_local_project", "create_local_project", "save_local_result_json", "configure_memory_pool"]);
    // One record per native save invocation, in send order (`seq`): its reply's receipt or
    // refusal and whether the App's read of the reply completed. `saveProject` correlates a step
    // with its own invocation through these, never through the latest receipt.
    const SAVES = new Set(["save_preview_mechanics_job_local_project", "save_restored_result_local_project"]);
    const RECEIPTS = new Set(["finish_local_csv_stream", "record_preview_mechanics_job_analysis_run", "save_preview_mechanics_job_local_project", "open_local_project_in_native_custody", "read_analysis_run_in_native_custody", "save_restored_result_local_project", "dispose_preview_mechanics_result"]);
    const note = (bytes) => { if (bytes > data.reply_bytes_max) data.reply_bytes_max = bytes; };
    const bump = (counts, command) => { counts[command] = (counts[command] ?? 0) + 1; };
    function lost(reason, command = null) {
      const transport = data.transport;
      if (transport.events.length < 16) transport.events.push({ reason, command, at_ms: now() });
      else transport.events_omitted++;
      if (transport.lost) return;
      transport.lost = true;
      observationFailure ??= Object.assign(new Error(`C1 command observation lost Tauri's IPC: ${reason}${command ? ` (${command})` : ""}. Tauri now sends every command over postMessage for the rest of this page, so the run's command evidence is incomplete and the run is stopped.`), { observation: true });
    }
    function commandOf(input) {
      const url = typeof input === "string" ? input : input && typeof input.url === "string" ? input.url : null;
      if (url === null || !url.startsWith(IPC_PREFIX)) return null;
      try { return decodeURIComponent(url.slice(IPC_PREFIX.length).split(/[?#]/u)[0]); } catch { return null; }
    }
    function request(body) {
      if (typeof body === "string") {
        if (body.length > 4096) throw new Error("probe request metadata exceeds 4 KiB");
        return JSON.parse(body);
      }
      if (body instanceof ArrayBuffer || ArrayBuffer.isView(body)) {
        if (body.byteLength > 4096) throw new Error("probe request metadata exceeds 4 KiB");
        const bytes = ArrayBuffer.isView(body) ? new Uint8Array(body.buffer, body.byteOffset, body.byteLength) : new Uint8Array(body);
        return JSON.parse(new TextDecoder().decode(bytes));
      }
      return null;
    }
    function restored(value) {
      return value ? { handle: value.handle, origin: value.origin, state: value.state,
        run_id: value.findings?.run_id, findings: value.findings?.findings?.slice(0, 64), findings_truncated: value.findings?.findings?.length > 64,
        envelope_payload_hash: value.findings?.envelope_payload_hash, result_bytes: value.result_bytes,
        message: typeof value.message === "string" ? value.message.slice(0, 512) : undefined } : null;
    }
    const savedResult = value => ({ outcome: value?.outcome, code: value?.code, run_id: value?.run_id,
      result_bytes: value?.result_bytes, record_bytes: value?.record_bytes });
    const refusalText = (command) => `C1 probe refused unexpected whole-value/budget command: ${command}`;
    function refusedReply(message) {
      return new Response(JSON.stringify(message), { status: 200, headers: { "Content-Type": "application/json", "Tauri-Response": "error" } });
    }
    async function observe(command, body, response, csv, invocation) {
      if (response.headers.get("Tauri-Response") !== "ok") {
        const { bytes, truncated } = await bodyPrefix(response.clone(), REFUSAL_LIMIT, note);
        const message = new TextDecoder().decode(bytes);
        if (invocation) invocation.refusal = message.slice(0, 512);
        if (data.refusals.length < 16) data.refusals.push({ command, message: message.slice(0, 512), truncated: truncated || message.length > 512 });
        else data.refusals_omitted++;
        return;
      }
      if (csv && command === "query_preview_mechanics_job_rows") {
        const sent = request(body);
        if (sent?.filter?.family === csv.family) {
          const read = await rowHeader(response.clone(), note);
          if (!Number.isSafeInteger(read.total) || read.total < 1) throw new Error("CSV query has no positive total");
          if (csv.expected_rows !== null && csv.expected_rows !== read.total) throw new Error("CSV query total changed");
          csv.expected_rows = read.total;
          csv.pages++;
        }
      }
      if (!RECEIPTS.has(command)) return;
      const value = await receiptOf(response.clone(), note);
      if (command === "finish_local_csv_stream" && csv) csv.receipt = { file_name: value.file_name, byte_count: value.byte_count };
      if (command === "record_preview_mechanics_job_analysis_run") data.record = { message: typeof value.message === "string" ? value.message.slice(0, 512) : null, state_outcome: value.state_outcome, run: { recording_id: value.run?.recording_id, run_name: value.run?.run_name, producer_run_id: value.run?.producer_run_id, project_id: value.run?.project_id } };
      if (command === "save_preview_mechanics_job_local_project") data.save = { result: savedResult(value.result), envelope_hash: value.project_envelope_hash?.value };
      if (invocation && command === "save_preview_mechanics_job_local_project") invocation.reply = { ...data.save };
      if (command === "open_local_project_in_native_custody") data.open = { ...restored(value.stored_run), envelope_hash: value.project_envelope_hash?.value };
      if (command === "read_analysis_run_in_native_custody") data.recordedRead = restored(value.restored);
      if (command === "save_restored_result_local_project") data.restoredSave = { ...restored(value.restored), kept: value.restored?.envelope_basis_kept, result: savedResult(value.result), envelope_hash: value.project_envelope_hash?.value };
      if (invocation && command === "save_restored_result_local_project") invocation.reply = { ...data.restoredSave };
      if (command === "dispose_preview_mechanics_result" && value?.released) data.disposed[value.job_id] = (data.disposed[value.job_id] ?? 0) + 1;
    }
    // The App's own read of the reply (Tauri reads it with `json`, `text` or `arrayBuffer` by its
    // content type): unchanged, except that success completes the round trip and failure, which
    // makes Tauri fall back, is recorded.
    function observeAppRead(command, response, invocation) {
      for (const method of ["json", "text", "arrayBuffer"]) {
        const read = response[method];
        if (typeof read !== "function") continue;
        Object.defineProperty(response, method, { configurable: true, writable: true, value: function (...args) {
          return read.apply(this, args).then((value) => { bump(data.completed, command); if (invocation) invocation.read = true; return value; },
            (error) => { lost("the App's read of an IPC reply failed", command); throw error; });
        } });
      }
    }
    const wrapped = async function (input, init) {
      const command = commandOf(input);
      if (command === null) return originalFetch.call(window, input, init);
      bump(data.counts, command);
      if (forbidden.has(command)) {
        if (data.violations.length < 16) data.violations.push(command);
        return refusedReply(refusalText(command));
      }
      const csv = data.csv; // pin the send phase, including a delayed reply
      let invocation = null;
      if (SAVES.has(command)) {
        if (data.saves.length >= 16) throw new Error("C1 probe sent more than 16 native saves");
        invocation = { seq: data.saves.length + 1, command, sent_at_ms: now(), reply: null, refusal: null, error: null, read: false };
        data.saves.push(invocation);
      }
      let response;
      try { response = await originalFetch.call(window, input, init); }
      catch (error) { lost("an IPC fetch was rejected", command); if (invocation) invocation.error = "fetch rejected"; throw error; }
      try { await observe(command, init?.body, response, csv, invocation); }
      catch (error) { if (data.errors.length < 16) data.errors.push(String(error?.message ?? error)); if (invocation) invocation.error = String(error?.message ?? error).slice(0, 512); }
      observeAppRead(command, response, invocation);
      return response;
    };
    // Tauri 2.11.1 `ipc-protocol.js` warns with exactly this text immediately before it falls back
    // (test-fixtures/tauri-2.11.1/scripts/ipc-protocol.js). Only that exact message counts, so an
    // unrelated warning that merely begins alike cannot invalidate a run (review: exact match).
    const TAURI_FALLBACK_WARNING = "IPC custom protocol failed, Tauri will now use the postMessage interface instead";
    const pageConsole = window.console;
    const originalWarn = pageConsole?.warn;
    const warn = function (...args) {
      if (args[0] === TAURI_FALLBACK_WARNING) lost("Tauri reported its IPC custom protocol failed and fell back to postMessage");
      return originalWarn.apply(this, args);
    };
    if (typeof originalWarn === "function") pageConsole.warn = warn;
    // The fallback transport: `ipc-protocol.js` serializes {cmd, callback, error, options, payload,
    // key} in that order (`process-ipc-message-fn.js`), so its head names the command and the error
    // callback without parsing the payload.
    function messageHead(message) {
      if (typeof message !== "string") return null;
      const match = /^\{"cmd":("(?:[^"\\]|\\.){1,256}"),"callback":(\d+),"error":(\d+),/u.exec(message.slice(0, 1024));
      if (!match) return null;
      try { return { cmd: JSON.parse(match[1]), error: Number(match[3]) }; } catch { return null; }
    }
    function fallbackPost(originalPost) {
      return function (message) {
        const head = messageHead(message);
        if (head) {
          lost("an IPC command was sent over postMessage", head.cmd);
          bump(data.transport.fallback_counts, head.cmd);
          const runCallback = window.__TAURI_INTERNALS__?.runCallback;
          if (forbidden.has(head.cmd)) {
            if (data.violations.length < 16) data.violations.push(head.cmd);
            // Tauri's own error reply (Rust evaluates `runCallback(error, message)`).
            if (typeof runCallback === "function") { setTimeout(() => runCallback(head.error, refusalText(head.cmd)), 0); return; }
          }
        }
        return originalPost.call(this, message);
      };
    }
    let restoreFallback = () => {};
    const replaceable = (owner) => {
      const descriptor = owner ? Object.getOwnPropertyDescriptor(owner, "postMessage") : null;
      return descriptor && typeof descriptor.value === "function" && (descriptor.writable || descriptor.configurable) && Object.isExtensible(owner) ? descriptor : null;
    };
    for (const [name, owner] of [["window.ipc", window.ipc], ["webkit.messageHandlers.ipc", (() => { try { const handler = window.webkit?.messageHandlers?.ipc; return handler ? Object.getPrototypeOf(handler) : null; } catch { return null; } })()]]) {
      const descriptor = replaceable(owner);
      if (!descriptor) continue;
      try {
        Object.defineProperty(owner, "postMessage", { ...descriptor, value: fallbackPost(descriptor.value) });
        restoreFallback = () => Object.defineProperty(owner, "postMessage", descriptor);
        data.transport.fallback_observer = name;
        break;
      } catch { /* not replaceable after all */ }
    }
    data.transport.fallback_observer ??= "unavailable";
    window.fetch = wrapped;
    return { data, restore: () => {
      if (window.fetch === wrapped) window.fetch = originalFetch;
      if (pageConsole?.warn === warn) pageConsole.warn = originalWarn;
      try { restoreFallback(); } catch { /* left wrapped; it only passes through */ }
    } };
  }
  // The observation must see the App's IPC before a solve can depend on it: the preflight's pool
  // status read must have completed as one observed round trip (`ipc://localhost/get_memory_pool_status`
  // fetched and its reply read by the App), with Tauri's IPC still on the observed transport.
  function requireObservedIpc(trace) {
    if (trace.data.transport.lost) throw observationFailure ?? new Error("C1 command observation lost Tauri's IPC; no solve is started");
    if (!(trace.data.completed.get_memory_pool_status >= 1)) throw new Error("C1 command observation saw no IPC round trip (the preflight's pool status read was not observed through fetch and read back); no solve is started");
  }

  const poolCell = (value) => ({ retained_results: value.retained_results, running_workers: value.running_workers,
    queued_jobs: value.queued_jobs, running_jobs: value.running_jobs, webview_custody_results: value.webview_custody_results,
    invocation_budget_bytes: value.invocation_budget_bytes, reserved_bytes: value.reserved_bytes,
    pressure: value.headroom?.pressure, available_bytes: value.headroom?.available_bytes });
  async function poolStatus() {
    let timer;
    try {
      const value = await Promise.race([call("get_memory_pool_status"), new Promise((_, reject) => {
        timer = setTimeout(() => reject(new Error("C1 pool status timed out")), 5000);
      })]);
      return poolCell(value);
    } finally { clearTimeout(timer); }
  }
  async function preflight(report) {
    const pool = await poolStatus();
    report.preflight = pool;
    if (pool.pressure !== "normal" || !Number.isFinite(pool.available_bytes) || pool.available_bytes < pool.invocation_budget_bytes) throw new Error("C1 preflight: pressure/headroom is not known sufficient");
    if (pool.invocation_budget_bytes !== 12 * 1024 ** 3 || pool.retained_results !== 0 || pool.running_workers !== 0 || pool.queued_jobs !== 0 || pool.running_jobs !== 0 || pool.webview_custody_results !== 0) throw new Error("C1 preflight: budget or prior-job state differs from the proposal");
    return pool;
  }
  function startPoolMonitor(report) {
    let pending = false, sentAt = 0, staleTicks = 0, unknown = 0;
    resourceFailure = null;
    const timer = setInterval(() => {
      if (pending) {
        staleTicks = now() - sentAt > 3000 ? staleTicks + 1 : 0;
        if (staleTicks >= 2) resourceFailure ??= "C1 pool status request stayed stale";
        return;
      }
      pending = true;
      sentAt = now();
      staleTicks = 0;
      poolStatus().then(pool => {
        report.last_pool = pool;
        if (["warning", "critical"].includes(pool.pressure)) resourceFailure ??= "C1 pool pressure became elevated";
        unknown = pool.pressure !== "normal" || !Number.isFinite(pool.available_bytes) ? unknown + 1 : 0;
        if (unknown >= 2) resourceFailure ??= "C1 pool pressure/headroom stayed unknown";
      }, error => { if (++unknown >= 2) resourceFailure ??= `C1 pool status refused: ${String(error?.message ?? error)}`; }).finally(() => { pending = false; staleTicks = 0; });
    }, 1000);
    return () => clearInterval(timer);
  }
  async function cancelIfRunning(report) {
    if (timeline()?.outcome !== "running") return;
    const prior = text("solve-job-cancellation");
    report.cancellation = { attempted: true, receipt: null };
    const end = now() + 5000;
    try {
      let timer;
      try { await Promise.race([menu("analyze.cancel"), new Promise((_, reject) => {
        timer = setTimeout(() => reject(new Error("cancel dispatch timed out")), 5000);
      })]); }
      finally { clearTimeout(timer); }
      while (now() < end) {
        const receipt = text("solve-job-cancellation");
        if (receipt && receipt !== prior) { report.cancellation.receipt = receipt; break; }
        if (timeline()?.outcome === "cancelled") { report.cancellation.receipt = "timeline cancelled"; break; }
        await sleep(100);
      }
    } catch (error) { report.cancellation.error = String(error?.message ?? error); }
  }

  function assertPanel(root) {
    if (!root || root.getAttribute("data-result-custody") !== "native") throw new Error("C1 panel is not native custody");
    for (const id of ["result-query-refused", "endpoint-pair-refused", "result-detail-unavailable"]) {
      const refusal = text(id, root);
      if (refusal) throw new Error(`C1 result read refused: ${refusal}`);
    }
    const navigation = text("results-navigator-status", root);
    if (navigation?.includes("could not")) throw new Error(navigation);
  }
  function rowIds(root) { return [...root.querySelectorAll("tr[data-testid^='result-row-']")].map(row => row.getAttribute("data-testid").slice(11)); }
  function settled(root) { assertPanel(root); return !byTestId("result-page-reading", root) && !byTestId("result-detail-reading", root); }
  function snapshot(root) {
    return { page: text("result-page-summary", root), row_ids: rowIds(root).slice(0, 3),
      selected: root.querySelector("tr[aria-selected='true']")?.getAttribute("data-testid") ?? null,
      kind_position: text("results-navigator-kind-position", root), case_position: text("results-navigator-case-position", root), entity_position: text("results-navigator-entity-position", root) };
  }
  async function probeResults(plan, target, trace, { csv = false } = {}) {
    const tableStart = now();
    await progress("c1_results", { stage: "native_table_wait" });
    await showSection("results", plan.timeouts.open_ms);
    const root = await waitFor("native results and rows", () => {
      const panel = byTestId("results-panel", byTestId("workspace-section-results"));
      return panel && settled(panel) && rowIds(panel).length ? panel : null;
    }, plan.timeouts.interaction_ms);
    // Two frames, bounded: on a page that is not shown none come (see `frames`).
    const shownFrames = await frames(plan.timeouts.interaction_ms);
    stopIfLost();
    if (shownFrames.visibility !== "visible") throw hostError("the results table", `visibilityState=${shownFrames.visibility}, animation frames ${shownFrames.frames}/2`, true);
    if (shownFrames.frames < 2) throw new Error(`C1 results table frames did not arrive (visibilityState=${shownFrames.visibility}, ${shownFrames.frames}/2 within ${plan.timeouts.interaction_ms} ms)`);
    target.table_ready_at_ms = now();
    target.table_ready_elapsed_ms = elapsed(tableStart);
    target.table_ready_method = "settled native rows and two animation frames, observed by the driver after first_paint (Results opened during the solve)";
    target.actions = [];
    const action = async (name, execute, ready) => {
      const before = snapshot(root), start = now();
      execute();
      await waitFor(name, () => settled(root) && ready(before), plan.timeouts.interaction_ms);
      target.actions.push({ name, elapsed_ms: elapsed(start), before, after: snapshot(root) });
      await progress("c1_action", { stage: name, elapsed_ms: elapsed(start) });
    };
    const first = root.querySelector("tr[data-testid^='result-row-']");
    await action("select row", () => first.click(), () => Boolean(text("result-detail-id", root)));
    for (const axis of ["kind", "case", "entity"]) {
      for (const direction of ["next", "previous"]) {
        const id = `results-navigator-${axis}-${direction}`;
        await waitFor(`${axis} ${direction} available`, () => {
          assertPanel(root);
          const button = byTestId(id, root);
          return button && !button.disabled && button.getAttribute("aria-disabled") !== "true";
        }, plan.timeouts.interaction_ms);
        await action(`${axis} ${direction}`, () => click(id, root), before => text(`results-navigator-${axis}-position`, root) !== before[`${axis}_position`] && Boolean(text("result-detail-id", root)));
      }
    }
    for (const direction of ["next", "previous"]) {
      await action(`page ${direction}`, () => click(`${direction}-result-page`, root), before => text("result-page-summary", root) !== before.page && rowIds(root).length > 0);
    }
    await action("no-match filter", () => fill("result-filter-input", "C1_probe_no_such_row_7cf6", root), () => Boolean(byTestId("result-filter-empty", root)) && rowIds(root).length === 0);
    await action("clear filter", () => click("clear-result-filter", root), () => rowIds(root).length > 0 && !byTestId("result-filter-empty", root));
    if (csv) {
      const button = root.querySelector("button[data-testid^='result-group-'][data-testid$='-export-csv']");
      if (!button) throw new Error("C1 group CSV control missing");
      const id = button.getAttribute("data-testid");
      trace.data.csv = { family: id.slice(13, -11), pages: 0, expected_rows: null, receipt: null };
      const start = now();
      button.click();
      const outcome = await waitFor("streamed group CSV", () => {
        const status = text(`${id}-status`, root);
        if (status?.startsWith("CSV export did not complete")) throw new Error(status);
        return status?.startsWith("Saved ") ? status : null;
      }, plan.timeouts.csv_ms);
      const saved = trace.data.csv;
      if (!saved.receipt || !Number.isSafeInteger(saved.expected_rows) || saved.expected_rows < 1 || saved.pages < 2) throw new Error("C1 CSV lacks native stream receipt or complete-query evidence");
      target.csv = { ...saved, elapsed_ms: elapsed(start), message: outcome };
      trace.data.csv = null;
    }
  }

  // Every native save invocation of the probe, reconciled with the steps that own them (review
  // of 4e209edf3: a step ends on its first invocation, so a delayed second one would otherwise
  // never be judged). Each invocation must be read, unrefused, observed without error and saved,
  // and the invocations must be exactly the expected commands in order: Record Run's own
  // prerequisite saves, then File > Save's fresh save, then the restored re-save. A pending,
  // refused, failed, unsaved, extra or missing invocation fails the probe.
  function reconcileSaves(trace, expected) {
    const saves = trace.data.saves;
    const problems = [];
    for (const save of saves) {
      if (save.refusal !== null) problems.push(`save ${save.seq} (${save.command}) refused: ${save.refusal}`);
      else if (save.error !== null) problems.push(`save ${save.seq} (${save.command}) observation failed: ${save.error}`);
      else if (!save.read) problems.push(`save ${save.seq} (${save.command}) unresolved: its reply was not read`);
      else if (save.reply?.result?.outcome !== "saved") problems.push(`save ${save.seq} (${save.command}) not saved: ${save.reply?.result?.code ?? save.reply?.result?.outcome ?? "no receipt"}`);
    }
    const sent = saves.map((save) => save.command);
    if (sent.length !== expected.length || sent.some((command, index) => command !== expected[index])) problems.push(`save invocations [${sent.join(", ")}] differ from the expected [${expected.join(", ")}]`);
    if (problems.length) throw new Error(`C1 save reconciliation failed: ${problems.join("; ")}`);
  }

  async function probePersistence(plan, report, trace) {
    const beforeRecord = trace.data.saves.length;
    await recordRun(plan, report);
    // Record Run saves an unstored project first: at most one fresh save, its own.
    const prerequisite = trace.data.saves.slice(beforeRecord).map((save) => save.command);
    if (prerequisite.length > 1 || prerequisite.some((command) => command !== "save_preview_mechanics_job_local_project")) throw new Error(`C1 Record Run sent unexpected saves: [${prerequisite.join(", ")}]`);
    const expectedSaves = [...trace.data.saves.slice(0, beforeRecord).map((save) => save.command), ...prerequisite, "save_preview_mechanics_job_local_project", "save_restored_result_local_project"];
    report.c1.expected_saves = expectedSaves;
    const recorded = trace.data.record;
    if (!recorded?.message?.startsWith("Recorded run") || typeof recorded.run.recording_id !== "string") throw new Error(`C1 Record Run did not complete: ${report.record_run?.message ?? "no native receipt"}`);
    if (recorded.run.run_name !== `perf ${plan.run_id}` || recorded.run.project_id !== plan.project_id) throw new Error("C1 recorded labels/project differ from the UI request");
    const saveStep = await saveProject(plan, report, trace);
    if (saveStep.timed_out) throw new Error(`C1 File > Save did not finish on its own native save within ${plan.timeouts.save_ms} ms: ${saveStep.receipt ? `invocation ${saveStep.receipt.seq} read=${saveStep.receipt.read}` : "no native save was sent"}`);
    if (saveStep.receipt?.command !== "save_preview_mechanics_job_local_project") throw new Error(`C1 File > Save sent no fresh native save: ${saveStep.receipt?.command ?? "none"}`);
    const saved = saveStep.receipt.reply;
    if (saved?.result?.outcome !== "saved") throw new Error(`C1 result was not saved: ${saved?.result?.code ?? saveStep.receipt.refusal ?? saveStep.receipt.error ?? saveStep.message ?? "no native receipt"}`);
    if (recorded.run.producer_run_id !== saved.result.run_id) throw new Error("C1 recorded and saved producer identities differ");
    const freshSaves = trace.data.counts.save_preview_mechanics_job_local_project;
    const freshRecords = trace.data.counts.record_preview_mechanics_job_analysis_run;
    await menu("file.open-local");
    await waitFor("native historical project reopen", () => {
      const opened = trace.data.open;
      if (opened?.state && opened.state !== "restored") throw new Error(`C1 stored run not restored: ${opened.message ?? opened.state}`);
      return opened?.origin === "restored_saved_run" && byTestId("historical-run-context");
    }, plan.timeouts.open_ms);
    const opened = { ...trace.data.open };
    if (opened.findings_truncated || opened.run_id !== saved.result.run_id || opened.envelope_hash !== saved.envelope_hash || opened.envelope_payload_hash !== opened.envelope_hash) throw new Error("C1 reopened run/hash differs from the saved receipt or its findings cannot be observed completely");
    report.c1.reopen = opened;
    report.c1.historical = {};
    await probeResults(plan, report.c1.historical, trace);
    await showSection("project", plan.timeouts.open_ms);
    await waitFor("historical envelope integrity", () => text("project-validation-envelope-hash")?.includes("integrity=verified_match"), plan.timeouts.interaction_ms);
    if (!text("record-run-unavailable")) throw new Error("C1 restored result incorrectly offers fresh Record Run");
    const resaveStep = await saveProject(plan, report, trace);
    if (resaveStep.timed_out) throw new Error(`C1 re-save did not finish on its own native save within ${plan.timeouts.save_ms} ms: ${resaveStep.receipt ? `invocation ${resaveStep.receipt.seq} read=${resaveStep.receipt.read}` : "no native save was sent"}`);
    if (resaveStep.receipt?.command !== "save_restored_result_local_project") throw new Error(`C1 re-save sent no restored native save: ${resaveStep.receipt?.command ?? "none"}`);
    const resaved = resaveStep.receipt.reply;
    if (resaved?.result?.outcome !== "saved" || resaved.findings_truncated || resaved.handle !== opened.handle || resaved.origin !== "restored_saved_run" || resaved.kept !== true || resaved.envelope_hash !== opened.envelope_hash || JSON.stringify(resaved.findings) !== JSON.stringify(opened.findings)) throw new Error(`C1 restored re-save did not preserve historical evidence: ${resaved?.result?.code ?? "receipt mismatch"}`);
    report.c1.resave = resaved;
    const checkId = `recorded-run-${recorded.run.recording_id}-check-run`;
    await waitFor("recorded run check control", () => byTestId(checkId) && !byTestId(checkId).disabled, plan.timeouts.interaction_ms);
    click(checkId);
    await waitFor("native recorded run check and disposal", () => {
      const checked = text(`recorded-run-${recorded.run.recording_id}-check`);
      if (checked?.includes("Not read:")) throw new Error(checked);
      const read = trace.data.recordedRead;
      return checked && read?.origin === "restored_recorded_run" && trace.data.disposed[read.handle] === 1;
    }, plan.timeouts.open_ms);
    report.c1.recorded_check = trace.data.recordedRead;
    if (trace.data.counts.save_preview_mechanics_job_local_project !== freshSaves || trace.data.counts.record_preview_mechanics_job_analysis_run !== freshRecords) throw new Error("C1 reopen/re-save invoked fresh persistence");
    // A normal model-document open retires historical custody through the
    // session, rather than a test-only native dispose that bypasses React state.
    await openModel(plan, report.c1);
    await waitFor("historical custody released by the session", () => !byTestId("historical-run-context") && trace.data.disposed[opened.handle] === 1, plan.timeouts.interaction_ms);
    const end = now() + plan.timeouts.interaction_ms;
    for (;;) {
      const pool = await poolStatus();
      report.c1.cleanup = pool;
      if (pool.retained_results === report.c1.preflight.retained_results && pool.running_workers === 0 && pool.queued_jobs === 0 && pool.running_jobs === 0 && pool.webview_custody_results === 0) break;
      if (now() > end) throw new Error("C1 held jobs did not return to the starting pool counts");
      await sleep(100);
    }
    reconcileSaves(trace, expectedSaves);
  }

  async function profileReaders(plan, report) {
    const proof = solveShown().proof;
    const job = proof?.match(/(?:^|; )job=([^;]+)/)?.[1];
    if (!job || !proof.includes("identity=match") || !proof.includes(`model_sha256=${plan.model_jcs_sha256}`)) throw new Error("C1 reader probe has no bound fresh solve proof");
    const started = Date.now();
    await progress("c1_profile_readers", { stage: "started", epoch_ms: started });
    let timer;
    try {
      const metrics = await Promise.race([
        ipc.invoke("native_smoke_profile_readers", new TextEncoder().encode(JSON.stringify({ jobId: job })), { headers: { "x-swbpipe-ingress": "args-json-v1" } }),
        new Promise((_, reject) => { timer = setTimeout(() => reject(new Error("C1 reader probe timed out; native work is not assumed cancelled")), plan.timeouts.profile_ms); })
      ]);
      const phases = ["model_parse", "invocation_parse", "source_parse", "dispatch_working", "registration_working", "semantics_working_max", "health_parse_and_working", "health_response_parse", "health_view_working", "summary_source_parse", "summary_working_and_response"];
      const scalar = ["schema", "started_ms", "finished_ms", "job_id", "route", "reader", "source_bytes", "rows", "publication_sha256", "intake_sha256", "source_parse_bound_bytes", "model_parse_bound_bytes", "invocation_parse_bound_bytes", "semantics_batches", "semantics_batch_limit", "health_working_upper_bound_requested", "health_working_upper_bound_priced", "summary_response_bytes", "coverage"];
      const allowed = new Set([...phases, ...scalar]);
      const validPhase = value => value && Object.keys(value).length === 2 && Number.isFinite(value.elapsed_us) && value.elapsed_us >= 0 && value.allocation && Object.keys(value.allocation).length === 5 && ["peak_requested", "peak_priced", "allocations", "live_requested", "live_priced"].every(key => Number.isFinite(value.allocation[key]) && value.allocation[key] >= 0);
      if (!metrics || Object.keys(metrics).some(key => !allowed.has(key)) || scalar.some(key => !["string", "number"].includes(typeof metrics[key])) || phases.some(key => !validPhase(metrics[key])) || metrics.schema !== "swbpipe.c1-reader-allocations/v1" || metrics.job_id !== job || metrics.rows <= 0 || metrics.rows !== Number(proof.match(/(?:^|; )rows=([^;]+)/)?.[1]) || metrics.semantics_batches !== Math.ceil(metrics.rows / 1000) || metrics.semantics_batch_limit !== 1000 || metrics.coverage.length > 1024 || !/^sha256:[a-f0-9]{64}$/.test(metrics.intake_sha256) || !/^sha256:[a-f0-9]{64}$/.test(metrics.publication_sha256)) throw new Error("C1 reader metrics are incomplete or not bound to this solve");
      if (solveShown().proof !== proof) throw new Error("C1 solve proof changed during reader measurement");
      report.reader_allocations = { ...metrics, driver_started_ms: started, driver_finished_ms: Date.now() };
      await progress("c1_profile_readers", { stage: "completed", epoch_ms: Date.now() });
    } finally { clearTimeout(timer); }
  }

  // Under the driver's node test only: hand over the helpers instead of running.
  const testHooks = globalThis.__SWBPIPE_PERF_DRIVER_TEST__;
  if (testHooks) {
    Object.assign(testHooks, { waitFor, waitUntil, solve, recordRun, saveProject, reconcileSaves, beginC1Trace, requireObservedIpc, requireShownPage, watchHost, frames, preflight, probeResults, probePersistence, cancelIfRunning, startPoolMonitor, profileReaders });
    return;
  }

  (async () => {
    const report = { schema: "swbpipe.desktop-perf-driver/v1", ok: false, user_agent: navigator.userAgent };
    let trace = null, stopMonitor = null;
    try {
      const plan = await call("native_smoke_plan");
      runId = plan.run_id;
      report.run_id = plan.run_id;
      report.time_origin_ms = performance.timeOrigin;
      await waitFor("app shell", () => byTestId("desktop-preview-shell"), plan.timeouts.start_ms);
      report.shell_ready_ms = now();
      await progress("ready");
      if (plan.c1_probes) {
        if (plan.c1_reader_diagnostics) globalThis.__SWBPIPE_NATIVE_SMOKE_READER_PHASE__ = async event => {
          await call("native_smoke_log", { line: `reader-phase:${JSON.stringify(event)}` });
        };
        report.c1 = { completed: false, host: { established: null } };
        trace = beginC1Trace();
        await preflight(report.c1);
        requireObservedIpc(trace);
        watchHost(report.c1.host);
        report.c1.page = { at_start: await requireShownPage("before opening the model") };
        // Established; a later loss sets it false (`hostError`).
        report.c1.host.established = true;
        stopMonitor = startPoolMonitor(report.c1);
      }
      await openModel(plan, report);
      if (plan.c1_probes) report.c1.page.before_run = await requireShownPage("before Analyze > Run");
      await solve(plan, report);
      if (plan.c1_probes) {
        if (report.solve.ended === "host_unobservable") throw hostFailure;
        if (report.solve.ended !== "first_paint") throw new Error(`C1 solve did not paint: ${report.solve.shown.error ?? report.solve.shown.refusal ?? report.solve.shown.results_refusal ?? report.solve.ended}`);
        if (!report.solve.shown.proof?.includes("identity=match") || !report.solve.shown.proof.includes(`model_sha256=${plan.model_jcs_sha256}`)) throw new Error("C1 solve proof is not bound to the opened model");
        report.c1.fresh = {};
        await probeResults(plan, report.c1.fresh, trace, { csv: true });
        if (plan.c1_profile_readers) await profileReaders(plan, report.c1);
        await probePersistence(plan, report, trace);
        stopIfLost();
        reconcileSaves(trace, report.c1.expected_saves);
        if (trace.data.transport.lost || trace.data.errors.length || trace.data.violations.length || trace.data.counts.start_preview_mechanics_job !== 1 || !trace.data.counts.lookup_preview_mechanics_job_rows || !trace.data.counts.read_preview_mechanics_job_row_semantics) throw new Error("C1 command observation is incomplete or violated its bounded-read path");
        report.c1.completed = true;
      } else {
        if (plan.record_run) await recordRun(plan, report);
        if (plan.save_project) await saveProject(plan, report);
      }
      report.timeline_after = timeline();
      report.ok = true;
    } catch (error) {
      report.error = `${error?.message ?? error}${error?.stack ? `\n${error.stack}` : ""}`;
      report.timeline_after = timeline();
      await log(`perf FAILED: ${report.error}`);
      if (trace) await cancelIfRunning(report);
    }
    finally {
      stopMonitor?.();
      stopHostWatch?.();
      await journalFlushed();
      delete globalThis.__SWBPIPE_NATIVE_SMOKE_READER_PHASE__;
      if (trace) { trace.restore(); report.c1.trace = trace.data; }
    }
    await call("native_smoke_finish", { report });
  })();
})();
