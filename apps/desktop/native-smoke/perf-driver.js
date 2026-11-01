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
//      from `__openPipeStressSolveTimelineV1.readCurrent()`;
//   3. Record Run… (Project page, Model History), then File > Save Local Project,
//      each outcome as shown.
// Progress lines go to the app's stderr as `[native-smoke] perf:{json}` (a crash
// of the webview still leaves them); the report goes through `native_smoke_finish`.
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
      error: text("solve-job-error"),
      refusal: text("solve-job-resource-refusal"),
      worker_failure: text("solve-job-worker-failure"),
      progress: text("solve-job-progress"),
      queue_reason: text("solve-job-queue-reason"),
      proof: byTestId("status-pill-solve-proof")?.querySelector("code")?.textContent ?? null
    };
  }

  // Analyze > Run, then the first paint of its result, a failure, or the timeout. The
  // marks are the app's own clock readings; the 200 ms poll only bounds when the wait
  // notices them (and keeps its own load on the webview's main thread small).
  async function solve(plan, report) {
    await showSection("solve", plan.timeouts.open_ms);
    click("solver-mode-sparse");
    await waitFor("sparse selected", () => byTestId("solver-mode-sparse")?.getAttribute("aria-pressed") === "true", 10000);
    const priorSequence = timeline()?.sequence ?? 0;
    const started = now();
    await menu("analyze.run");
    let lastStage = null;
    let heartbeat = 0;
    let ended = "timeout";
    try {
      ended = await waitFor("first paint or failure", () => {
        const current = timeline();
        if (current && current.sequence > priorSequence) {
          const stage = current.marks.at(-1)?.stage ?? null;
          if (stage !== lastStage || now() - heartbeat > 5000) {
            lastStage = stage;
            heartbeat = now();
            void progress("solve", { stage, outcome: current.outcome, elapsed_ms: elapsed(started) });
          }
          if (current.marks.some((mark) => mark.stage === "first_paint")) return "first_paint";
          if (TERMINAL.has(current.outcome) || current.outcome === "superseded") return current.outcome;
        }
        if (UNSOLVED_STATE.test(text("solve-job-summary") ?? "")) return "ui_unsolved";
        return null;
      }, plan.timeouts.paint_ms, 200);
    } catch (error) {
      if (!error?.timeout) throw error;
    }
    report.solve = { ended, elapsed_ms: elapsed(started), shown: solveShown(), timeline: timeline() };
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
  function beginC1Trace() {
    const original = ipc.invoke;
    const data = { counts: {}, errors: [], violations: [], refusals: [], refusals_omitted: 0, disposed: {}, csv: null, open: null, recordedRead: null, record: null, save: null, restoredSave: null };
    const forbidden = new Set(["take_preview_mechanics_job_result", "read_preview_mechanics_job_rows_all", "read_restored_result_record_all", "open_local_project", "read_analysis_run", "record_analysis_run", "save_local_project", "create_local_project", "save_local_result_json", "configure_memory_pool"]);
    function request(args) {
      if (ArrayBuffer.isView(args)) {
        if (args.byteLength > 4096) throw new Error("probe request metadata exceeds 4 KiB");
        return JSON.parse(new TextDecoder().decode(args));
      }
      return args;
    }
    function header(value) {
      const bytes = ArrayBuffer.isView(value) ? new Uint8Array(value.buffer, value.byteOffset, value.byteLength) : new Uint8Array(value);
      if (bytes.length < 16 || String.fromCharCode(...bytes.subarray(0, 8)) !== "SWBDETv1") throw new Error("probe rows have no native header");
      const length = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength).getBigUint64(8, true);
      if (length > 65536n || length > BigInt(bytes.length - 16)) throw new Error("probe native header is not bounded");
      return JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(16, 16 + Number(length))));
    }
    function restored(value) {
      return value ? { handle: value.handle, origin: value.origin, state: value.state,
        run_id: value.findings?.run_id, findings: value.findings?.findings?.slice(0, 64), findings_truncated: value.findings?.findings?.length > 64,
        envelope_payload_hash: value.findings?.envelope_payload_hash, result_bytes: value.result_bytes,
        message: typeof value.message === "string" ? value.message.slice(0, 512) : undefined } : null;
    }
    const savedResult = value => ({ outcome: value?.outcome, code: value?.code, run_id: value?.run_id,
      result_bytes: value?.result_bytes, record_bytes: value?.record_bytes });
    const wrapped = async (command, args, options) => {
      data.counts[command] = (data.counts[command] ?? 0) + 1;
      if (forbidden.has(command)) {
        if (data.violations.length < 16) data.violations.push(command);
        throw new Error(`C1 probe refused unexpected whole-value/budget command: ${command}`);
      }
      const csv = data.csv; // pin the send phase, including a delayed reply
      let value;
      try { value = await original.call(ipc, command, args, options); }
      catch (error) {
        const message = String(error?.message ?? error?.code ?? error);
        if (data.refusals.length < 16) data.refusals.push({ command, message: message.slice(0, 512), truncated: message.length > 512 });
        else data.refusals_omitted++;
        throw error;
      }
      try {
        if (csv && command === "query_preview_mechanics_job_rows") {
          const sent = request(args);
          if (sent?.filter?.family === csv.family) {
            const read = header(value);
            if (!Number.isSafeInteger(read.total) || read.total < 1) throw new Error("CSV query has no positive total");
            if (csv.expected_rows !== null && csv.expected_rows !== read.total) throw new Error("CSV query total changed");
            csv.expected_rows = read.total;
            csv.pages++;
          }
        }
        if (command === "finish_local_csv_stream" && csv) csv.receipt = { file_name: value.file_name, byte_count: value.byte_count };
        if (command === "record_preview_mechanics_job_analysis_run") data.record = { message: typeof value.message === "string" ? value.message.slice(0, 512) : null, state_outcome: value.state_outcome, run: { recording_id: value.run?.recording_id, run_name: value.run?.run_name, producer_run_id: value.run?.producer_run_id, project_id: value.run?.project_id } };
        if (command === "save_preview_mechanics_job_local_project") data.save = { result: savedResult(value.result), envelope_hash: value.project_envelope_hash?.value };
        if (command === "open_local_project_in_native_custody") data.open = { ...restored(value.stored_run), envelope_hash: value.project_envelope_hash?.value };
        if (command === "read_analysis_run_in_native_custody") data.recordedRead = restored(value.restored);
        if (command === "save_restored_result_local_project") data.restoredSave = { ...restored(value.restored), kept: value.restored?.envelope_basis_kept, result: savedResult(value.result), envelope_hash: value.project_envelope_hash?.value };
        if (command === "dispose_preview_mechanics_result" && value?.released) data.disposed[value.job_id] = (data.disposed[value.job_id] ?? 0) + 1;
      } catch (error) { if (data.errors.length < 16) data.errors.push(String(error.message)); }
      return value;
    };
    ipc.invoke = wrapped;
    return { data, restore: () => { if (ipc.invoke === wrapped) ipc.invoke = original; } };
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
    let pending = false, lastReply = now(), unknown = 0;
    resourceFailure = null;
    const timer = setInterval(() => {
      if (now() - lastReply > 3000) resourceFailure ??= "C1 pool status became stale";
      if (pending) return;
      pending = true;
      poolStatus().then(pool => {
        lastReply = now();
        report.last_pool = pool;
        if (["warning", "critical"].includes(pool.pressure)) resourceFailure ??= "C1 pool pressure became elevated";
        unknown = pool.pressure !== "normal" || !Number.isFinite(pool.available_bytes) ? unknown + 1 : 0;
        if (unknown >= 2) resourceFailure ??= "C1 pool pressure/headroom stayed unknown";
      }, error => { if (++unknown >= 2) resourceFailure ??= `C1 pool status refused: ${String(error?.message ?? error)}`; }).finally(() => { pending = false; });
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
    await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
    target.table_ready_at_ms = now();
    target.table_ready_elapsed_ms = elapsed(tableStart);
    target.table_ready_method = "settled native rows and two animation frames after results activation";
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
    Object.assign(testHooks, { waitFor, solve, recordRun, saveProject, beginC1Trace, preflight, probeResults, probePersistence, cancelIfRunning, startPoolMonitor, profileReaders });
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
        report.c1 = { completed: false };
        trace = beginC1Trace();
        await preflight(report.c1);
        stopMonitor = startPoolMonitor(report.c1);
      }
      await openModel(plan, report);
      await solve(plan, report);
      if (plan.c1_probes) {
        if (report.solve.ended !== "first_paint") throw new Error(`C1 solve did not paint: ${report.solve.shown.error ?? report.solve.shown.refusal ?? report.solve.ended}`);
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
      if (trace) { trace.restore(); report.c1.trace = trace.data; }
    }
    await call("native_smoke_finish", { report });
  })();
})();
