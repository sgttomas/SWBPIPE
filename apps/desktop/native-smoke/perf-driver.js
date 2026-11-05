// Desktop performance driver (RT-U4, DEL-04-05 `Work/criterion-9-measurement.md`,
// "Desktop measurement"). `perf.mjs` launches the `native-smoke` build of the app,
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
// A C1 run first checks that its command observation sees the App's IPC and that
// the page is shown and delivers animation frames (before the model opens and again
// before Analyze > Run); otherwise it stops without starting a solve. During the
// solve it records and journals what the page showed (`journal`, `observePage`).
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
  const progress = (step, detail = {}) => log(`perf:${JSON.stringify({ run_id: runId, step, at_ms: now(), ...detail })}`);

  // As `driver.js`: a probe that throws ends the wait with its own error.
  async function waitFor(label, probe, timeoutMs, intervalMs = 100) {
    const started = Date.now();
    for (;;) {
      if (resourceFailure) throw new Error(resourceFailure);
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
      if (resourceFailure) throw new Error(resourceFailure);
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

  // RT-U4 (A+B diagnosis, `c1-slice7-run-proposal.md`): WebKit runs no rendering update, so no
  // animation frame, for a page it does not consider shown (window occluded, display asleep,
  // screen locked), and aligns that page's nested timers to 1 s. The smoke window's
  // `backgroundThrottling: "disabled"` only sets WKPreferences.inactiveSchedulingPolicy (wry), not
  // page visibility. A first paint is two animation frames after the first page, so on a page that
  // is not shown it cannot be observed at all. `frames` asks for two frames and reports what came.
  const visibility = () => (typeof document.visibilityState === "string" ? document.visibilityState : null);
  function frames(timeoutMs) {
    return new Promise((resolve) => {
      let seen = 0, done = false, timer = null;
      const finish = () => {
        if (done) return;
        done = true;
        if (timer !== null) clearTimeout(timer);
        resolve({ visibility: visibility(), frames: seen, timeout_ms: timeoutMs });
      };
      timer = setTimeout(finish, timeoutMs);
      if (typeof requestAnimationFrame !== "function") return;
      requestAnimationFrame(() => { seen = 1; requestAnimationFrame(() => { seen = 2; finish(); }); });
    });
  }
  // A C1 run starts its solve only on a shown page that delivers frames: otherwise the run could
  // only time out at the paint gate. Checked before the model opens and again before Analyze > Run.
  async function requireShownPage(step, timeoutMs = 2000) {
    const probe = await frames(timeoutMs);
    if (probe.visibility !== "visible" || probe.frames < 2) {
      throw new Error(`C1 host: the App's page is not shown (${step}: visibilityState=${probe.visibility}, animation frames ${probe.frames}/2 within ${timeoutMs} ms); a first paint cannot be observed, so no solve is started`);
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
      else { page.hidden_polls++; journal("page_hidden", "enter", "pending"); }
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
          if (error?.timeout && openMs < plan.timeouts.open_ms) throw error;
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
      if (!error?.timeout) throw error;
      // Whichever wait ran out, a mark already within the gate is still the run's first paint.
      if (timely(paintMark())) ended = "first_paint";
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
  async function saveProject(plan, report) {
    const started = now();
    const outcome = { message: null, prompted: null, elapsed_ms: null };
    report.save_project = outcome;
    const before = message();
    await menu("file.save-local");
    try {
      outcome.message = await waitFor("save outcome", () => {
        if (byTestId("replace-project-prompt") && outcome.prompted === null) {
          outcome.prompted = text("replace-project-detail");
          click("replace-project-cancel");
        }
        const current = message();
        return current !== before && /^(Saved|Created|Not saved|Save failed|Create failed)|SQLite project snapshot/.test(current) ? current : null;
      }, plan.timeouts.save_ms, 100);
    } catch (error) {
      if (!error?.timeout) throw error;
      outcome.timed_out = true;
      outcome.last_message = message();
    }
    outcome.elapsed_ms = elapsed(started);
    await progress("save_project", { message: outcome.message, elapsed_ms: outcome.elapsed_ms });
  }

  // C1-only observation: command names, bounded headers and small receipts.
  // Never retain a result body, record, row page or CSV chunk in this trace.
  // Tauri defines `__TAURI_INTERNALS__.invoke` (and `ipc`, `postMessage`) with
  // Object.defineProperty, so they are not writable: assigning a wrapper there
  // changes nothing (A+B diagnosis: every earlier trace observed no command).
  // Every invoke of this webview is one `fetch` of `ipc://localhost/<command>`
  // (tauri `ipc-protocol.js`), so the observation wraps `window.fetch` and
  // passes every other request through untouched. A command's reply is read
  // from a clone only where a receipt or a row header is needed; a refusal is
  // answered as Tauri answers one (`Tauri-Response: error`), so the App's
  // promise rejects and no transport fallback is taken.
  function beginC1Trace() {
    const IPC_PREFIX = "ipc://localhost/";
    const originalFetch = window.fetch;
    const data = { counts: {}, errors: [], violations: [], refusals: [], refusals_omitted: 0, disposed: {}, csv: null, open: null, recordedRead: null, record: null, save: null, restoredSave: null };
    const forbidden = new Set(["take_preview_mechanics_job_result", "read_preview_mechanics_job_rows_all", "read_restored_result_record_all", "open_local_project", "read_analysis_run", "record_analysis_run", "save_local_project", "create_local_project", "save_local_result_json", "configure_memory_pool"]);
    const RECEIPTS = new Set(["finish_local_csv_stream", "record_preview_mechanics_job_analysis_run", "save_preview_mechanics_job_local_project", "open_local_project_in_native_custody", "read_analysis_run_in_native_custody", "save_restored_result_local_project", "dispose_preview_mechanics_result"]);
    const RECEIPT_LIMIT = 4 << 20;
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
    function header(bytes) {
      if (bytes.length < 16 || String.fromCharCode(...bytes.subarray(0, 8)) !== "SWBDETv1") throw new Error("probe rows have no native header");
      const length = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength).getBigUint64(8, true);
      if (length > 65536n || length > BigInt(bytes.length - 16)) throw new Error("probe native header is not bounded");
      return JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(16, 16 + Number(length))));
    }
    // At most the header's bytes of a row reply: read from a clone, which is then cancelled.
    async function headerOf(response) {
      const limit = 16 + 65536;
      const reader = response.body.getReader();
      const chunks = [];
      let have = 0, need = 16;
      try {
        while (have < need && have < limit) {
          const { value, done } = await reader.read();
          if (done) break;
          chunks.push(value);
          have += value.byteLength;
          if (need === 16 && have >= 16) {
            const first = concat(chunks, 16);
            need = Math.min(limit, 16 + Number(new DataView(first.buffer, first.byteOffset, 16).getBigUint64(8, true)));
          }
        }
      } finally { reader.cancel().catch(() => {}); }
      return header(concat(chunks, Math.min(have, limit)));
    }
    function concat(chunks, length) {
      const out = new Uint8Array(length);
      let at = 0;
      for (const chunk of chunks) {
        if (at >= length) break;
        const part = chunk.subarray(0, length - at);
        out.set(part, at);
        at += part.byteLength;
      }
      return out;
    }
    // A receipt as the App reads it: a JSON reply, or a raw `tauri::ipc::Response` body of JSON bytes.
    async function receipt(response) {
      if ((response.headers.get("content-type") ?? "").split(",")[0] === "application/json") return response.json();
      const bytes = new Uint8Array(await response.arrayBuffer());
      if (bytes.byteLength > RECEIPT_LIMIT) throw new Error("probe receipt exceeds its bound");
      return JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
    }
    function restored(value) {
      return value ? { handle: value.handle, origin: value.origin, state: value.state,
        run_id: value.findings?.run_id, findings: value.findings?.findings?.slice(0, 64), findings_truncated: value.findings?.findings?.length > 64,
        envelope_payload_hash: value.findings?.envelope_payload_hash, result_bytes: value.result_bytes,
        message: typeof value.message === "string" ? value.message.slice(0, 512) : undefined } : null;
    }
    const savedResult = value => ({ outcome: value?.outcome, code: value?.code, run_id: value?.run_id,
      result_bytes: value?.result_bytes, record_bytes: value?.record_bytes });
    function refusedReply(message) {
      return new Response(JSON.stringify(message), { status: 200, headers: { "Content-Type": "application/json", "Tauri-Response": "error" } });
    }
    async function observe(command, body, response, csv) {
      if (response.headers.get("Tauri-Response") !== "ok") {
        const message = (await response.clone().text()).slice(0, 2048);
        if (data.refusals.length < 16) data.refusals.push({ command, message: message.slice(0, 512), truncated: message.length > 512 });
        else data.refusals_omitted++;
        return;
      }
      if (csv && command === "query_preview_mechanics_job_rows") {
        const sent = request(body);
        if (sent?.filter?.family === csv.family) {
          const read = await headerOf(response.clone());
          if (!Number.isSafeInteger(read.total) || read.total < 1) throw new Error("CSV query has no positive total");
          if (csv.expected_rows !== null && csv.expected_rows !== read.total) throw new Error("CSV query total changed");
          csv.expected_rows = read.total;
          csv.pages++;
        }
      }
      if (!RECEIPTS.has(command)) return;
      const value = await receipt(response.clone());
      if (command === "finish_local_csv_stream" && csv) csv.receipt = { file_name: value.file_name, byte_count: value.byte_count };
      if (command === "record_preview_mechanics_job_analysis_run") data.record = { message: typeof value.message === "string" ? value.message.slice(0, 512) : null, state_outcome: value.state_outcome, run: { recording_id: value.run?.recording_id, run_name: value.run?.run_name, producer_run_id: value.run?.producer_run_id, project_id: value.run?.project_id } };
      if (command === "save_preview_mechanics_job_local_project") data.save = { result: savedResult(value.result), envelope_hash: value.project_envelope_hash?.value };
      if (command === "open_local_project_in_native_custody") data.open = { ...restored(value.stored_run), envelope_hash: value.project_envelope_hash?.value };
      if (command === "read_analysis_run_in_native_custody") data.recordedRead = restored(value.restored);
      if (command === "save_restored_result_local_project") data.restoredSave = { ...restored(value.restored), kept: value.restored?.envelope_basis_kept, result: savedResult(value.result), envelope_hash: value.project_envelope_hash?.value };
      if (command === "dispose_preview_mechanics_result" && value?.released) data.disposed[value.job_id] = (data.disposed[value.job_id] ?? 0) + 1;
    }
    const wrapped = async function (input, init) {
      const command = commandOf(input);
      if (command === null) return originalFetch.call(window, input, init);
      data.counts[command] = (data.counts[command] ?? 0) + 1;
      if (forbidden.has(command)) {
        if (data.violations.length < 16) data.violations.push(command);
        return refusedReply(`C1 probe refused unexpected whole-value/budget command: ${command}`);
      }
      const csv = data.csv; // pin the send phase, including a delayed reply
      const response = await originalFetch.call(window, input, init);
      try { await observe(command, init?.body, response, csv); }
      catch (error) { if (data.errors.length < 16) data.errors.push(String(error?.message ?? error)); }
      return response;
    };
    window.fetch = wrapped;
    return { data, restore: () => { if (window.fetch === wrapped) window.fetch = originalFetch; } };
  }
  // The observation must see the App's IPC before a solve can depend on it: the preflight's
  // pool status read is one fetch of `ipc://localhost/get_memory_pool_status`.
  function requireObservedIpc(trace) {
    if (!(trace.data.counts.get_memory_pool_status >= 1)) throw new Error("C1 command observation saw no IPC (the preflight's pool status read was not observed); no solve is started");
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

  async function probePersistence(plan, report, trace) {
    await recordRun(plan, report);
    const recorded = trace.data.record;
    if (!recorded?.message?.startsWith("Recorded run") || typeof recorded.run.recording_id !== "string") throw new Error(`C1 Record Run did not complete: ${report.record_run?.message ?? "no native receipt"}`);
    if (recorded.run.run_name !== `perf ${plan.run_id}` || recorded.run.project_id !== plan.project_id) throw new Error("C1 recorded labels/project differ from the UI request");
    await saveProject(plan, report);
    const saved = trace.data.save;
    if (saved?.result?.outcome !== "saved") throw new Error(`C1 result was not saved: ${saved?.result?.code ?? report.save_project?.message ?? "no native receipt"}`);
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
    await saveProject(plan, report);
    const resaved = trace.data.restoredSave;
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
    Object.assign(testHooks, { waitFor, waitUntil, solve, recordRun, saveProject, beginC1Trace, requireObservedIpc, requireShownPage, frames, preflight, probeResults, probePersistence, cancelIfRunning, startPoolMonitor, profileReaders });
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
        report.c1 = { completed: false };
        trace = beginC1Trace();
        await preflight(report.c1);
        requireObservedIpc(trace);
        report.c1.page = { at_start: await requireShownPage("before opening the model") };
        stopMonitor = startPoolMonitor(report.c1);
      }
      await openModel(plan, report);
      if (plan.c1_probes) report.c1.page.before_run = await requireShownPage("before Analyze > Run");
      await solve(plan, report);
      if (plan.c1_probes) {
        if (report.solve.ended !== "first_paint") throw new Error(`C1 solve did not paint: ${report.solve.shown.error ?? report.solve.shown.refusal ?? report.solve.shown.results_refusal ?? report.solve.ended}`);
        if (!report.solve.shown.proof?.includes("identity=match") || !report.solve.shown.proof.includes(`model_sha256=${plan.model_jcs_sha256}`)) throw new Error("C1 solve proof is not bound to the opened model");
        report.c1.fresh = {};
        await probeResults(plan, report.c1.fresh, trace, { csv: true });
        if (plan.c1_profile_readers) await profileReaders(plan, report.c1);
        await probePersistence(plan, report, trace);
        if (trace.data.errors.length || trace.data.violations.length || trace.data.counts.start_preview_mechanics_job !== 1 || !trace.data.counts.lookup_preview_mechanics_job_rows || !trace.data.counts.read_preview_mechanics_job_row_semantics) throw new Error("C1 command observation is incomplete or violated its bounded-read path");
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
      await journalFlushed();
      delete globalThis.__SWBPIPE_NATIVE_SMOKE_READER_PHASE__;
      if (trace) { trace.restore(); report.c1.trace = trace.data; }
    }
    await call("native_smoke_finish", { report });
  })();
})();
