// Native driver for the rendered report of a solved exact-route result that is not numerically eligible
// (DEL-08-01 REQ-017 and REQ-024). `report-standing-journey.mjs` launches the `native-smoke` build with this script,
// which the build evaluates in the real webview once the page has loaded (`src-tauri/src/native_smoke.rs`).
//
// It works the app as a person would: native menu commands go through the dispatch a menu click uses, and buttons
// and checkboxes are operated in the window. Two presentations are substituted: the file choosers (Open Model
// Document… and Save canonical HTML…; `native_smoke_choose` names the file each returns) and the system print
// dialog. Save canonical HTML… goes to the native save route, which writes the file itself. Print… goes to the native print
// route (DEL-08-01 REQ-022), and this build, in place of the dialog, records on the print window's own handle the
// print document that reached the native print (`native_smoke_take_report_prints`); the driver then asks for the open
// print window to be closed (`native_smoke_close_report_print`), which answers whether the window that close selected
// left the window registry. No dialog appears, and none is observed.
//
// It records what the window shows, step by step (reached or not, with what was observed), and hands the runner's
// nonce back; the runner judges. A step that throws is recorded as not reached, with its error, and the journey ends.
(() => {
  const ipc = window.__TAURI_INTERNALS__;
  const call = (command, args) => ipc.invoke(command, args ?? {});
  const log = (line) => call("native_smoke_log", { line: String(line) }).catch(() => {});
  const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
  const settle = () => sleep(0);
  // A missing root (the rendered report panel while the gate still reads the full result, say) holds no element:
  // a probe over it waits instead of throwing.
  const byTestId = (id, root = document) => root?.querySelector(`[data-testid="${CSS.escape(id)}"]`) ?? null;
  const text = (id, root) => byTestId(id, root)?.textContent?.trim() ?? null;
  const message = () => text("local-project-message") ?? "";
  const menu = (commandId) => call("native_smoke_menu", { commandId });
  const treeRow = (projectId) => byTestId(`tree-row-project-${encodeURIComponent(projectId)}`);
  const panel = () => byTestId("rendered-report-panel");
  // Right after a solve the Report section shows the gate's loading line while the full result is read, and no
  // rendered report panel. A probe that needs the panel requires it present and this line gone.
  const gateLoading = () => Boolean(byTestId("c2-result-loading", byTestId("workspace-section-report")));

  // Every wait is capped at WAIT_CAP_MS (90 s): a wait that can only fail fails quickly.
  const WAIT_CAP_MS = 90000;
  async function waitFor(label, probe, timeoutMs = WAIT_CAP_MS) {
    timeoutMs = Math.min(timeoutMs, WAIT_CAP_MS);
    const started = Date.now();
    for (;;) {
      const value = probe();
      if (value) return value;
      if (Date.now() - started > timeoutMs) throw new Error(`timed out after ${timeoutMs} ms waiting for ${label}`);
      await sleep(100);
    }
  }
  function element(id, root) {
    const found = byTestId(id, root);
    if (!found) throw new Error(`no element ${id}`);
    return found;
  }
  function click(id, root) {
    const found = element(id, root);
    if (found.disabled) throw new Error(`${id} is disabled`);
    found.click();
  }
  function setValue(target, value) {
    Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value").set.call(target, value);
    target.dispatchEvent(new Event("input", { bubbles: true }));
  }
  // Choosing a section that is already shown closes it (View menu), so the command is sent only when it is not.
  const sectionShown = (section) => {
    const found = byTestId(`workspace-section-${section}`);
    return Boolean(found && found.offsetParent !== null);
  };
  async function showSection(section) {
    await settle();
    if (sectionShown(section)) return;
    await menu(`view.section.${section}`);
    await waitFor(`section ${section}`, () => sectionShown(section));
  }
  async function sha256Hex(textValue) {
    const digest = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(textValue));
    return [...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("");
  }
  const parse = (html) => new DOMParser().parseFromString(html, "text/html");
  const standingLineOf = (doc) => doc.querySelector(".standing-line")?.textContent?.trim() ?? null;

  const UNSOLVED_STATE = /\bstate=(failed|cancelled|refused)(?:;|$)/;
  const proofText = () => byTestId("status-pill-solve-proof")?.querySelector("code")?.textContent ?? null;
  // Run the solve from the menu and wait for a completed job summary AND a solve proof other than the one before it.
  // The proof names its job and is shown only while the session's published result matches it (solveProof.ts), so a
  // new proof means the new result is the session's. The summary text carries no job identity (SolvePanel.tsx: state,
  // events, rows, cancellation), so two solves of the same model read alike and the proof alone tells them apart. A
  // job that ended any other way, or a withheld Run, ends the wait with its statement.
  async function solve(label, projectId) {
    await showSection("solve");
    const priorSummary = text("solve-job-summary") ?? "";
    const priorProof = proofText();
    await menu("analyze.run");
    await waitFor(`${label}: solve completed and its result published`, () => {
      const summary = text("solve-job-summary") ?? "";
      if (UNSOLVED_STATE.test(summary)) throw new Error(`${label}: ${summary}; ${text("solve-job-error") ?? ""}`);
      const withheld = byTestId("solve-withheld-notice");
      if (withheld) throw new Error(`${label}: ${withheld.textContent.trim().replace(/\s+/g, " ").slice(0, 2000)}`);
      const proof = proofText();
      return summary.includes("state=completed") && proof !== null && proof !== priorProof && proof.includes(`project=${projectId}`);
    }, WAIT_CAP_MS);
    return { summary: text("solve-job-summary"), prior_summary: priorSummary, proof_job: proofText()?.match(/job=([^;]+)/)?.[1] ?? null, prior_proof_job: priorProof?.match(/job=([^;]+)/)?.[1] ?? null };
  }
  // Render for local review and wait for the shown document or a named failure.
  async function renderReport(label) {
    await showSection("report");
    await waitFor(`${label}: the panel present, the gate's loading line gone`, () => panel() && !gateLoading(), WAIT_CAP_MS);
    await waitFor(`${label}: Render enabled`, () => byTestId("rendered-report-render", panel())?.disabled === false, WAIT_CAP_MS);
    const intent = element("rendered-report-local-private-intent", panel());
    if (!intent.checked) intent.click();
    await waitFor(`${label}: local-private selected`, () => element("rendered-report-local-private-intent", panel()).checked);
    click("rendered-report-render", panel());
    await waitFor(`${label}: rendered`, () => {
      if (byTestId("rendered-report-preview", panel())) return byTestId("rendered-report-preview", panel());
      const error = text("rendered-report-error", panel());
      if (error) throw new Error(`${label}: ${error}`);
      const route = text("rendered-report-route", panel());
      if (route && route !== "route=tauri_renderer") throw new Error(`${label}: ${route}`);
      const withheld = text("rendered-report-sink-blocked", panel());
      if (withheld) throw new Error(`${label}: ${withheld}`);
      const blocked = byTestId("rendered-report-blocking-reasons", panel());
      if (blocked) throw new Error(`${label}: export blocked: ${blocked.textContent.trim().slice(0, 2000)}`);
      return byTestId("rendered-report-preview", panel());
    }, WAIT_CAP_MS);
    return byTestId("rendered-report-preview", panel());
  }

  // The session's report output outcomes (`ReportOutputStatus.tsx`), outside the Report section: each entry's id,
  // kind, state, whether the workspace changed since its report was prepared, and its text, as the window shows them.
  function outputEntries() {
    return [...document.querySelectorAll('[data-testid="report-output-status"] > [data-testid^="report-output-"]')].map((item) => ({
      id: item.getAttribute("data-testid").slice("report-output-".length),
      kind: item.getAttribute("data-kind"),
      state: item.getAttribute("data-state"),
      workspace_changed: item.getAttribute("data-workspace-changed"),
      text: item.querySelector('[data-testid$="-text"]')?.textContent?.trim() ?? null
    }));
  }

  const steps = [];
  // A step's body may note what it has seen as it goes (`note`), so a step that fails keeps its observations so far.
  async function step(id, body) {
    const entry = { id, reached: false };
    steps.push(entry);
    await log(`step ${id}`);
    const note = (seen) => Object.assign(entry, seen);
    try {
      Object.assign(entry, await body(note));
      entry.reached = true;
    } catch (error) {
      entry.error = `${error?.message ?? error}`;
      throw error;
    }
  }

  async function journey(plan) {
    let renderedLine = null;
    await step("open-and-solve", async () => {
      const name = plan.model.path.split("/").pop();
      const before = message();
      await call("native_smoke_choose", { path: plan.model.path });
      await menu("file.open-model");
      const opened = await waitFor(`open ${name}`, () => {
        const now = message();
        if (now !== before && now.includes(`refused (${name})`)) throw new Error(now);
        return now !== before && now.includes(`Opened model document ${name} `) ? now : null;
      });
      await waitFor("the project in the model tree", () => treeRow(plan.model.project_id));
      const solved = await solve("first solve", plan.model.project_id);
      return { opened, solve_summary: solved.summary, proof_job: solved.proof_job };
    });
    await step("report-gate", async () => {
      await showSection("report");
      await waitFor("the solved result in the panel", () => panel() && !gateLoading() && !byTestId("rendered-report-precondition", panel()) && byTestId("rendered-report-render", panel())?.disabled === false, WAIT_CAP_MS);
      const package_buttons = ["report-package-save", "report-package-user-release"].map((id) => {
        const button = byTestId(id, panel());
        return { id, present: Boolean(button), disabled: button?.disabled ?? null, title: button?.getAttribute("title") ?? null };
      });
      return { precondition: Boolean(byTestId("rendered-report-precondition", panel())), render_enabled: byTestId("rendered-report-render", panel())?.disabled === false, package_buttons };
    });
    await step("package-refusals", async () => {
      const statuses = [];
      for (const command of ["file.save-report-package", "file.save-shareable-report-package"]) {
        const before = text("report-package-save-status", panel());
        await menu(command);
        // The second command repeats the first's line; the wait is for a line, then a short settle for any dialog.
        statuses.push(await waitFor(`${command}: status`, () => {
          const now = text("report-package-save-status", panel());
          return now && (now !== before || statuses.length === 1) ? now : null;
        }, 30000));
        await sleep(500);
      }
      return { statuses, dialog: Boolean(byTestId("user-release-dialog")) };
    });
    await step("other-outputs", async () => {
      await showSection("exports");
      const exportsPanel = await waitFor("result export panel", () => byTestId("result-export-panel"));
      const result_export_empty = Boolean(byTestId("result-export-empty", exportsPanel));
      const result_export_link = Boolean(byTestId("result-export-link", exportsPanel));
      await showSection("solve");
      const ruleCheck = await waitFor("rule-check control", () => byTestId("rule-check-run"));
      return { result_export_empty, result_export_link, rule_check_disabled: ruleCheck.disabled === true };
    });
    await step("render", async () => {
      const preview = await renderReport("render");
      const srcdoc = preview.getAttribute("srcdoc") ?? "";
      const doc = parse(srcdoc);
      renderedLine = standingLineOf(doc);
      const documentText = doc.body?.textContent ?? "";
      return {
        preview: true,
        gate: text("rendered-report-gate", panel()),
        hash_text: text("rendered-report-hash", panel()),
        preview_sha256: await sha256Hex(srcdoc),
        standing_line: renderedLine,
        statements_present: Object.fromEntries(plan.expect.case_statements.map((statement) => [statement, documentText.includes(statement)])),
        integrity_checked: /integrity checked/.test(documentText)
      };
    });
    await step("save-html", async (note) => {
      // Save canonical HTML… goes to the native save route (DEL-08-01 REQ-022): its chooser takes the path queued
      // here (`native_smoke_choose`, inside the home; the strict chooser ends the app if none is queued), and the
      // product writes the file. The runner reads the saved bytes itself and compares them with the document read here.
      const preview = byTestId("rendered-report-preview", panel());
      const canonical_html = preview?.getAttribute("srcdoc") ?? "";
      const shown_hash = text("rendered-report-hash", panel());
      const control = byTestId("rendered-report-save-local-private", panel()) ?? byTestId("rendered-report-save", panel());
      note({ control: control?.getAttribute("data-testid") ?? null, control_tag: control?.tagName?.toLowerCase() ?? null });
      if (!control || control.tagName !== "BUTTON") throw new Error("no native Save canonical HTML control");
      if (control.disabled) throw new Error(`${control.getAttribute("data-testid")} is disabled`);
      const before = new Set(outputEntries().map((entry) => entry.id));
      await call("native_smoke_choose", { path: plan.html_save.path });
      control.click();
      const status = await waitFor("the save outcome", () => {
        const now = text("rendered-report-save-status", panel()) ?? "";
        return /^(Saved |Not saved|Save cancelled|Save outcome unknown)/.test(now) ? now : null;
      }, 60000);
      note({ status });
      // A queued answer still there afterwards was never taken by a chooser.
      const chooser_paths_unused = await call("native_smoke_clear_choices");
      const entry = await waitFor("the save's outcome entry, settled", () => outputEntries().find((item) => !before.has(item.id) && item.kind === "save" && item.state !== "pending") ?? null, 30000);
      return { status, chooser_paths_unused, entry, canonical_html, shown_hash };
    });
    await step("print", async () => {
      // The native-smoke build presents no system print dialog: on the print window's own handle the native print
      // records the print document. Drain any earlier record, press Print, take the record, close the print window.
      // The runner judges the recorded document against the rendered one (`report-print-evidence.mjs`).
      const drained = await call("native_smoke_take_report_prints");
      const preview = byTestId("rendered-report-preview", panel());
      const canonical_html = preview?.getAttribute("srcdoc") ?? "";
      const shown_hash = text("rendered-report-hash", panel());
      click("rendered-report-print", panel());
      const status = await waitFor("the print outcome", () => {
        const now = text("rendered-report-print-status", panel()) ?? "";
        return /^(Print window opened|Not printed)/.test(now) ? now : null;
      }, 60000);
      const events = [];
      if (/^Print window opened/.test(status)) {
        const started = Date.now();
        while (events.length === 0 && Date.now() - started < 30000) {
          events.push(...(await call("native_smoke_take_report_prints")));
          if (events.length === 0) await sleep(250);
        }
        await sleep(1000);
        events.push(...(await call("native_smoke_take_report_prints")));
      }
      const closed = await call("native_smoke_close_report_print");
      return { drained_before: drained.length, status, events, closed, canonical_html, shown_hash };
    });
    await step("edit-invalidates", async () => {
      click("layout-mode-grid");
      const disclosure = await waitFor("node grid disclosure", () => byTestId("node-grid-review-disclosure"));
      if (disclosure.getAttribute("aria-expanded") !== "true") disclosure.click();
      const cell = await waitFor(`${plan.edit_node} y cell`, () => byTestId(`review-cell-${plan.edit_node}-y`));
      const current = Number.parseFloat(cell.textContent ?? "");
      cell.dispatchEvent(new MouseEvent("dblclick", { bubbles: true }));
      const input = await waitFor("cell editor", () => byTestId("engineering-table-review")?.querySelector("input"));
      setValue(input, String(Number.isFinite(current) ? current + 0.25 : 0.5));
      click("queue-entity-grid-intents");
      const apply = await waitFor("apply control", () => [...document.querySelectorAll('[data-testid^="apply-intent-editor-intent-"]')].at(-1));
      apply.click();
      const summary = await waitFor("operation applied", () => /1 applied/.test(text("operation-apply-summary") ?? "") && text("operation-apply-summary"), 30000);
      await showSection("report");
      // One snapshot: the panel present (no loading line) and showing the precondition, which an edit's cleared result
      // gives; the preview is read in that same snapshot.
      const seen = await waitFor("the panel with the precondition", () => {
        const shown = panel();
        if (!shown || gateLoading() || !byTestId("rendered-report-precondition", shown)) return null;
        return { panel_present: true, precondition: true, preview: Boolean(byTestId("rendered-report-preview", shown)) };
      }, 30000);
      // The earlier save's outcome entry, read after this edit has been applied and the panel has withdrawn the
      // report: the save completed before the edit, so the entry is expected to say that the workspace has changed
      // since its report was prepared. Ordinary ordering only: no save is in transit across the edit.
      const savedId = steps.find((item) => item.id === "save-html")?.entry?.id ?? null;
      const save_entry_after_edit = savedId === null ? null
        : await waitFor("the save's entry after the edit", () => outputEntries().find((item) => item.id === savedId && item.workspace_changed === "true") ?? null, 30000)
          .catch(() => outputEntries().find((item) => item.id === savedId) ?? { missing: savedId });
      return { applied: true, summary, ...seen, save_entry_after_edit };
    });
    await step("resolve-invalidates", async (note) => {
      await menu("edit.undo");
      await sleep(1000);
      note({ undo_message: message() });
      const again = await solve("solve after undo", plan.model.project_id);
      note({ solve_after_undo: again, proof_job_before: again.proof_job });
      const preview = await renderReport("render again");
      const rerendered_line = standingLineOf(parse(preview.getAttribute("srcdoc") ?? ""));
      note({ preview_before: Boolean(preview), rerendered_line });
      const fresh = await solve("fresh solve", plan.model.project_id);
      note({ fresh_solve: fresh, proof_job_after: fresh.proof_job });
      await showSection("report");
      // One snapshot, positive on every side: the panel present, the gate's loading line gone, no precondition and
      // Render enabled, so the panel holds the fresh result (published, by the new proof); only then is the preview's
      // absence read, from that same snapshot. "Not back yet" never satisfies this probe.
      const after = await waitFor("the fresh result in the panel", () => {
        const shown = panel();
        if (!shown || gateLoading() || byTestId("rendered-report-precondition", shown)) return null;
        const render = byTestId("rendered-report-render", shown);
        if (!render || render.disabled) return null;
        return { panel_present_after: true, gate_loading_after: false, precondition_after: false, render_enabled_after: true, preview_after: Boolean(byTestId("rendered-report-preview", shown)) };
      }, WAIT_CAP_MS);
      return { preview_before: Boolean(preview), rerendered_line, proof_job_before: again.proof_job, proof_job_after: fresh.proof_job, ...after, undo_message: message() };
    });
  }

  (async () => {
    const report = { phase: null, nonce: null, ok: false, steps, user_agent: navigator.userAgent };
    try {
      const plan = await call("native_smoke_plan");
      report.phase = plan.phase;
      report.nonce = plan.nonce;
      await waitFor("desktop shell", () => byTestId("desktop-preview-shell"), WAIT_CAP_MS);
      await log(`report standing journey started`);
      await journey(plan);
      report.ok = true;
    } catch (error) {
      report.error = `${error?.message ?? error}${error?.stack ? `\n${error.stack}` : ""}`;
      if (error?.observed) Object.assign(steps.at(-1) ?? {}, error.observed);
      await log(`FAILED: ${report.error}`);
    }
    await call("native_smoke_finish", { report });
  })();
})();
