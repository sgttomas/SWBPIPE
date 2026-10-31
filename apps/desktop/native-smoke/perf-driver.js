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
//      failure, with a timeout, reading the solve timeline from
//      `__openPipeStressUiDiagnosticsV1.readCurrent().solveTimeline`;
//   3. Record Run… (Project page, Model History), then File > Save Local Project,
//      each outcome as shown.
// Progress lines go to the app's stderr as `[native-smoke] perf:{json}` (a crash
// of the webview still leaves them); the report goes through `native_smoke_finish`.
(() => {
  const ipc = window.__TAURI_INTERNALS__;
  const call = (command, args) => ipc.invoke(command, args ?? {});
  const log = (line) => call("native_smoke_log", { line: String(line) }).catch(() => {});
  const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
  const byTestId = (id) => document.querySelector(`[data-testid="${CSS.escape(id)}"]`);
  const text = (id) => byTestId(id)?.textContent?.trim() ?? null;
  const message = () => text("local-project-message") ?? "";
  const historyMessage = () => text("model-history-message") ?? "";
  const now = () => performance.now();
  let runId = null;
  const progress = (step, detail = {}) => log(`perf:${JSON.stringify({ run_id: runId, step, at_ms: now(), ...detail })}`);

  // As `driver.js`: a probe that throws ends the wait with its own error.
  async function waitFor(label, probe, timeoutMs, intervalMs = 100) {
    const started = Date.now();
    for (;;) {
      const value = probe();
      if (value) return value;
      if (Date.now() - started > timeoutMs) throw Object.assign(new Error(`timed out after ${timeoutMs} ms waiting for ${label}`), { timeout: true });
      await sleep(intervalMs);
    }
  }
  const menu = (commandId) => call("native_smoke_menu", { commandId });
  function click(id) {
    const element = byTestId(id);
    if (!element) throw new Error(`no element ${id}`);
    if (element.disabled) throw new Error(`${id} is disabled`);
    element.click();
  }
  function fill(id, value) {
    const element = byTestId(id);
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
  const timeline = () => globalThis.__openPipeStressUiDiagnosticsV1?.readCurrent?.().solveTimeline ?? null;
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

  // Under the driver's node test only: hand over the helpers instead of running.
  const testHooks = globalThis.__SWBPIPE_PERF_DRIVER_TEST__;
  if (testHooks) {
    Object.assign(testHooks, { waitFor, solve, recordRun, saveProject });
    return;
  }

  (async () => {
    const report = { schema: "swbpipe.desktop-perf-driver/v1", ok: false, user_agent: navigator.userAgent };
    try {
      const plan = await call("native_smoke_plan");
      runId = plan.run_id;
      report.run_id = plan.run_id;
      report.time_origin_ms = performance.timeOrigin;
      await waitFor("app shell", () => byTestId("desktop-preview-shell"), plan.timeouts.start_ms);
      report.shell_ready_ms = now();
      await progress("ready");
      await openModel(plan, report);
      await solve(plan, report);
      if (plan.record_run) await recordRun(plan, report);
      if (plan.save_project) await saveProject(plan, report);
      report.timeline_after = timeline();
      report.ok = true;
    } catch (error) {
      report.error = `${error?.message ?? error}${error?.stack ? `\n${error.stack}` : ""}`;
      report.timeline_after = timeline();
      await log(`perf FAILED: ${report.error}`);
    }
    await call("native_smoke_finish", { report });
  })();
})();
