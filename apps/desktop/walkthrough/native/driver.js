// Driver of the native walkthrough (README.md, "The launched app"), evaluated in the launched `native-smoke` app's
// real web view once the page has loaded (`src-tauri/src/native_smoke.rs`). It is not evaluated as it stands: the
// runner (`run.mjs`, composeDriver) writes it into one script after the text of readScreenCounts and
// scrollLongestRegion (`../screen-counts.mjs`, the browser walkthrough's own) and takeSnapshot (`snapshot.mjs`).
//
// It works the window as a person would: native menu commands through the dispatch a menu click uses, and the
// window's own controls. The only substitution is the file chooser: `native_smoke_choose` names the file the next
// one returns (every answer the runner plans lies inside the throwaway home). Print is not pressed. At each step it
// records the counts and a snapshot of the document as drawn, and writes them as the checkpoint `step-<number>`
// (`native_smoke_checkpoint`, no store copy). A step whose control is missing or disabled is recorded as where a
// person stops, with the reason the window gives, and the journey's later steps as not reached. It judges nothing.
(() => {
  const ipc = window.__TAURI_INTERNALS__;
  const call = (command, args) => ipc.invoke(command, args ?? {});
  const log = (line) => call("native_smoke_log", { line: String(line) }).catch(() => {});
  let nonce = null;
  let T = { ui: 60000, step: 300000, solve: 900000, save: 900000 };
  // The act about to start, written as the `act` checkpoint: a strict chooser stop is named by it.
  const act = async (name) => {
    await log(`act: ${name}`);
    await call("native_smoke_checkpoint", { name: "act", report: { nonce, act: name, at: Date.now() }, storeSnapshot: false });
  };
  const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
  const settle = () => new Promise((resolve) => setTimeout(resolve, 0));
  const byTestId = (id, root = document) => root.querySelector(`[data-testid="${CSS.escape(id)}"]`);
  const text = (id, root) => byTestId(id, root)?.textContent?.replace(/\s+/g, " ").trim() ?? null;
  const menu = (commandId) => call("native_smoke_menu", { commandId });
  const visible = (element) => Boolean(element && element.checkVisibility({ checkOpacity: true, checkVisibilityCSS: true }));
  const disabledReason = (element) => {
    const described = element.getAttribute("aria-describedby");
    const fromIds = described ? described.split(/\s+/).map((id) => document.getElementById(id)?.textContent ?? "").join(" ").trim() : "";
    return fromIds || element.getAttribute("title") || "no reason given";
  };

  async function waitFor(label, probe, timeoutMs = T.step) {
    const started = Date.now();
    for (;;) {
      const value = probe();
      if (value) return value;
      const elapsed = Date.now() - started;
      if (elapsed > timeoutMs) throw new Error(`nothing showed ${label} within ${Math.round(elapsed / 1000)} s`);
      await sleep(100);
    }
  }
  // The control a person would press: missing, hidden or disabled is where they stop, with the window's reason.
  async function control(id, { root, timeoutMs = T.ui } = {}) {
    const found = await waitFor(`the control ${id}`, () => { const item = byTestId(id, root); return visible(item) ? item : null; }, timeoutMs).catch(() => null);
    if (!found) throw new Error(byTestId(id, root) ? `the control ${id} is on the page but not shown` : `the window shows no control ${id}`);
    if (found.disabled || found.getAttribute("aria-disabled") === "true") throw new Error(`the control ${id} is disabled: "${disabledReason(found)}"`);
    return found;
  }
  async function press(id, options) {
    (await control(id, options)).click();
    await settle();
  }
  const sectionShown = (section) => visible(byTestId(`workspace-section-${section}`));
  async function showSection(section) {
    await settle();
    if (sectionShown(section)) return;
    await menu(`view.section.${section}`);
    await waitFor(`the ${section} section`, () => sectionShown(section), T.ui);
  }
  function setValue(target, value) {
    const prototype = target instanceof HTMLSelectElement ? HTMLSelectElement.prototype : target instanceof HTMLTextAreaElement ? HTMLTextAreaElement.prototype : HTMLInputElement.prototype;
    Object.getOwnPropertyDescriptor(prototype, "value").set.call(target, value);
    target.dispatchEvent(new Event(target instanceof HTMLSelectElement ? "change" : "input", { bubbles: true }));
  }
  const slug = (value) => String(value).toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "").slice(0, 40) || "unnamed";
  const closeOverlays = async () => {
    for (const id of ["key-results-close", "solve-progress-close"]) { const item = byTestId(id); if (visible(item)) { item.click(); await settle(); } }
    for (const drawer of ["issues-home", "audit-boundary-drawer"]) {
      const close = byTestId(drawer) && [...byTestId(drawer).querySelectorAll("button")].find((button) => button.textContent.trim() === "Close" || button.getAttribute("aria-label") === "Close");
      if (visible(close)) { close.click(); await settle(); }
    }
  };

  // ------------------------------------------------------------ the record of one journey

  class Journey {
    constructor(plan) {
      this.number = plan.number;
      this.name = plan.journey;
      this.count = 0;
      this.broken = null;
    }

    async capture() {
      let counts;
      try { counts = readScreenCounts(); } catch (error) { counts = { counts_error: String(error?.message ?? error) }; }
      let snapshot;
      try { snapshot = await takeSnapshot(); } catch (error) { snapshot = null; counts.snapshot_error = String(error?.message ?? error); }
      return { ...counts, snapshot };
    }

    async write(record) {
      await call("native_smoke_checkpoint", { name: `step-${record.number}`, report: { nonce, ...record }, storeSnapshot: false });
    }

    /** Does `action`, then records the screen. An action may return what it found, which joins the note. */
    async step(step, note, action) {
      this.count += 1;
      const record = { journey: this.name, number: this.number * 100 + this.count, step, note };
      if (this.broken) {
        await this.write({ ...record, not_reached: true });
        return;
      }
      await act(`${this.name}: ${step}`);
      try {
        const found = action ? await action() : undefined;
        if (typeof found === "string" && found) record.note = `${note}: ${found}`;
        // Let transitions and the canvas settle before the record.
        await sleep(350);
        Object.assign(record, await this.capture());
      } catch (error) {
        this.broken = step;
        record.failed = String(error?.message ?? error).split("\n")[0].slice(0, 400);
        Object.assign(record, await this.capture());
      }
      await this.write(record);
    }

    /** Steps that apply only where the page scrolls further: scroll, record, up to `screens` times. */
    async scrolled(step, note, screens = 2, within) {
      for (let index = 1; index <= screens; index += 1) {
        if (this.broken || !scrollLongestRegion({ count: 1, scope: within ?? null })) return;
        await this.step(`${step}-scroll${index}`, `${note}, scrolled ${index} screen${index === 1 ? "" : "s"} down`);
      }
    }
  }

  // ------------------------------------------------------------ shared acts

  const solveOutcome = () => {
    for (const id of ["solve-progress-failure", "solve-progress-refusal", "solve-progress-cancellation", "solve-progress-result-too-large"]) {
      const shown = byTestId(id);
      if (visible(shown)) return { failed: `the solve did not complete: ${shown.textContent.replace(/\s+/g, " ").trim().slice(0, 300)}` };
    }
    const completed = byTestId("solve-progress-completed");
    return visible(completed) ? { completed: completed.textContent.replace(/\s+/g, " ").trim() } : null;
  };
  async function pressRun() {
    await closeOverlays();
    await press("toolbar-run");
    return "Run pressed in the toolbar";
  }
  async function awaitSolve() {
    const outcome = await waitFor("the solve's outcome", solveOutcome, T.solve);
    if (outcome.failed) throw new Error(outcome.failed);
    return outcome.completed;
  }
  async function openStage(stage) {
    await closeOverlays();
    await press(`rail-stage-${stage}`);
  }
  async function scrollTo(element, block = "end") {
    element.scrollIntoView({ block, inline: "nearest" });
    await sleep(100);
  }

  // ------------------------------------------------------------ journeys

  // A. Solve and read results, on the bundled demonstration model.
  async function solveAndResults(journey, plan) {
    await journey.step("first-open", "The application as first opened, with its demonstration model");
    await journey.step("run", "Run pressed on the demonstration model", pressRun);
    await journey.step("solve-progress", "The solve's progress, once it ended", awaitSolve);
    await journey.step("results-stage", "The Results stage as first shown", async () => {
      await openStage("results");
      await waitFor("the results table's rows", () => document.querySelector('[data-testid^="result-row-"]'), T.step);
    });
    // Each table or panel in the Results section, scrolled to its foot.
    const section = byTestId("workspace-section-results");
    let root = section;
    while (root && root.children.length === 1) root = root.children[0];
    const panels = root ? [...root.children].filter((panel) => visible(panel) && panel.getBoundingClientRect().height > 0) : [];
    const names = new Set();
    for (const panel of panels) {
      const heading = panel.querySelector("h2, h3, legend, caption")?.textContent?.trim();
      let name = slug(panel.getAttribute("data-testid") || panel.getAttribute("aria-label") || heading || panel.className);
      while (names.has(name)) name = `${name}-2`;
      names.add(name);
      await journey.step(`panel-${name}`, `Results: ${heading || name}, scrolled to its foot`, async () => { await scrollTo(panel, "end"); });
    }
    await journey.step("filter-support", `The results filter used to find the rows of ${plan.support_text}`, async () => {
      const input = await control("result-filter-input");
      await scrollTo(input, "center");
      input.focus();
      setValue(input, plan.support_text);
      await sleep(500);
      return text("result-filter-summary") ?? "no filter summary shown";
    });
    await journey.step("row-chosen", "A row of the filtered results chosen", async () => {
      const row = await waitFor("a result row", () => [...document.querySelectorAll('tr[data-testid^="result-row-"]')].find(visible), T.ui).catch(() => null);
      if (!row) throw new Error(`the filter shows no row: ${text("result-filter-empty") ?? text("result-filter-summary") ?? "no reason shown"}`);
      row.click();
      await sleep(300);
      return row.getAttribute("data-testid");
    });
    await journey.step("row-in-viewport", "The chosen row's location in the viewport (Both view)", async () => {
      const both = byTestId("view-switch-both");
      if (both && both.getAttribute("aria-pressed") !== "true") await press("view-switch-both");
      return text("status-selection") ?? "";
    });
    await journey.step("key-results", "Key results opened", async () => {
      if (!visible(byTestId("key-results-panel"))) await press("key-results-open");
      await waitFor("the Key results panel", () => visible(byTestId("key-results-panel")), T.ui);
    });
    for (const action of ["ratio", "deflection", "support", "contact", "results-table", "stress-ratio-table", "displacement-extremes"]) {
      await journey.step(`key-results-${action}`, `Key results: ${action.replace(/-/g, " ")}`, async () => {
        if (!visible(byTestId("key-results-panel"))) await press("key-results-open");
        const button = await waitFor(`key-results-${action}`, () => byTestId(`key-results-${action}`), T.ui).catch(() => null);
        if (!button) throw new Error(`Key results offers no ${action} view`);
        button.click();
        await sleep(400);
        // An unavailable view says why and does nothing else: recorded, not a stop.
        return byTestId(`key-results-${action}-reason`)?.textContent?.trim() ?? text("key-results-status") ?? "";
      });
    }
    await journey.step("evidence-tab", "The Results stage's Evidence tab", async () => {
      await closeOverlays();
      await press("stage-tab-evidence");
    });
    await journey.scrolled("evidence-tab", "The Evidence tab");
    await journey.step("analyze-page", "The Analyze page after the solve", async () => { await closeOverlays(); await showSection("solve"); });
    await journey.scrolled("analyze-page", "The Analyze page", 3);
    await journey.step("issues-drawer", "The Issues drawer, from the status bar", async () => { await press("issues-drawer-toggle"); });
    await journey.scrolled("issues-drawer", "The Issues drawer", 2, '[data-testid="issues-home"]');
  }

  // B. The report, from the Review stage to a saved package.
  async function report(journey, plan) {
    await journey.step("run", "Run pressed on the demonstration model", pressRun);
    await journey.step("solved", "The solve, once it ended", awaitSolve);
    await journey.step("review-stage", "The Review stage", async () => { await openStage("review"); });
    await journey.step("report-tab", "The Report tab as first shown", async () => {
      await press("stage-tab-report");
      await waitFor("the report panel", () => byTestId("rendered-report-panel") || byTestId("report-panel"), T.step);
    });
    await journey.scrolled("report-tab", "The Report tab", 3);
    await journey.step("load-for", "“Load for…”, where the window offers it", async () => {
      const load = byTestId("c2-result-load");
      if (!visible(load)) return "not offered on this screen";
      await scrollTo(load, "center");
      load.click();
      await sleep(500);
      return `pressed: ${load.textContent.trim()}`;
    });
    await journey.step("composer", "The report composer", async () => {
      const composer = await control("report-composer", { timeoutMs: T.step });
      await scrollTo(composer, "start");
    });
    await journey.step("render", "Render, local private", async () => {
      const intent = await control("rendered-report-local-private-intent", { timeoutMs: T.step });
      if (!intent.checked) { intent.click(); await waitFor("the local-private intent ticked", () => byTestId("rendered-report-local-private-intent")?.checked, T.ui); }
      const render = await control("rendered-report-render", { timeoutMs: T.step });
      await scrollTo(render, "center");
      render.click();
      await waitFor("the render's outcome", () => byTestId("rendered-report-hash") || byTestId("rendered-report-error") || byTestId("rendered-report-route"), T.save);
      await waitFor("the render settled", () => !/Rendering/.test(byTestId("rendered-report-render")?.textContent ?? ""), T.save);
      const error = text("rendered-report-error");
      if (error) throw new Error(`the render was refused: ${error}`);
      return text("rendered-report-hash") ?? "";
    });
    await journey.step("preview", "The rendered report's preview", async () => {
      const preview = await control("rendered-report-preview", { timeoutMs: T.ui });
      await scrollTo(preview, "start");
    });
    await journey.step("shareable-dialog", "File > Save Shareable Report Package…: the release dialog", async () => {
      await menu("file.save-shareable-report-package");
      await waitFor("the release dialog", () => visible(byTestId("user-release-dialog")), T.step);
      return text("user-release-refusal") ?? "";
    });
    await journey.step("shareable-cancelled", "The release dialog cancelled", async () => {
      await press("user-release-cancel", { root: byTestId("user-release-dialog") ?? document });
      await waitFor("the release dialog closed", () => !byTestId("user-release-dialog"), T.ui);
    });
    await journey.step("save-package", "File > Save Report Package… (local private), its chooser answered inside the throwaway home", async () => {
      const intent = await control("report-package-private-intent", { timeoutMs: T.step });
      if (!intent.checked) { intent.click(); await waitFor("the package intent ticked", () => byTestId("report-package-private-intent")?.checked, T.ui); }
      await control("report-package-save", { timeoutMs: T.step });
      const prior = text("report-package-save-status");
      await call("native_smoke_choose", { path: plan.local_package });
      await menu("file.save-report-package");
      const saving = () => /Saving report package/.test(byTestId("report-package-save")?.textContent ?? "");
      await waitFor("the save started", () => saving() || text("report-package-save-status") !== prior, T.step);
      const status = await waitFor("the save's outcome", () => { const now = text("report-package-save-status"); return now && !saving() ? now : null; }, T.save);
      const unused = await call("native_smoke_clear_choices");
      await scrollTo(byTestId("report-package-save-status") ?? document.body, "center");
      return `${status}${unused > 0 ? ` (the queued chooser answer was not taken)` : ""}`;
    });
    await journey.step("outcome-list", "The report outputs' outcome list", async () => {
      const list = byTestId("report-output-status");
      // The list shows report output outcomes of the kinds it records (save, print); the window may show none here.
      if (!visible(list)) return "the window shows no outcome list after the package save";
      await scrollTo(list, "center");
      return `${list.querySelectorAll('[data-testid^="report-output-"][data-testid$="-text"]').length} outcome(s) listed`;
    });
  }

  // C. The Exports page after the solve, top to foot.
  async function exportsPage(journey) {
    await journey.step("run", "Run pressed on the demonstration model", pressRun);
    await journey.step("solved", "The solve, once it ended", awaitSolve);
    await journey.step("exports-page", "The Exports page, from the View menu", async () => { await closeOverlays(); await showSection("exports"); });
    await journey.scrolled("exports-page", "The Exports page", 10);
  }

  // D. The reference system, opened as a model document and solved.
  async function referenceSystem(journey, plan) {
    await journey.step("first-open", "The application as first opened");
    await journey.step("opened", "File > Open Model Document… of the reference system", async () => {
      const before = text("local-project-message") ?? "";
      const name = plan.open.split("/").pop();
      await call("native_smoke_choose", { path: plan.open });
      await menu("file.open-model");
      const shown = await waitFor(`the open of ${name}`, () => {
        const now = text("local-project-message") ?? "";
        return now !== before && (now.includes(`Opened model document ${name} `) || now.includes(`refused (${name})`)) ? now : null;
      }, T.step);
      if (!shown.includes("Opened model document")) throw new Error(shown);
      await waitFor("the reference system in the model tree", () => byTestId(`tree-row-project-${encodeURIComponent(plan.project_id)}`), T.step);
      return shown;
    });
    await journey.step("run", "Run pressed on the reference system", pressRun);
    await journey.step("solve-progress", "The solve's progress, a minute in", async () => {
      const outcome = await waitFor("a minute of the solve, or its outcome", () => solveOutcome(), 60000).catch(() => null);
      if (outcome?.failed) throw new Error(outcome.failed);
      return outcome?.completed ?? "still running";
    });
    await journey.step("solved", "The solve, once it ended", awaitSolve);
    await journey.step("results-stage", "The Results stage after the solve", async () => {
      await openStage("results");
      await waitFor("the results table's rows", () => document.querySelector('[data-testid^="result-row-"]'), T.step);
    });
    await journey.step("standing", "The result's standing, as the window states it", async () => {
      const standing = await control("numerical-result-standing", { timeoutMs: T.ui });
      await scrollTo(standing, "center");
      return standing.textContent.replace(/\s+/g, " ").trim().slice(0, 300);
    });
    await journey.step("health-line", "The numerical health line", async () => {
      const line = await control("numerical-health-header-line", { timeoutMs: T.ui });
      await scrollTo(line, "center");
      return line.textContent.replace(/\s+/g, " ").trim().slice(0, 300);
    });
    await journey.step("results", "The results table", async () => {
      const row = document.querySelector('[data-testid^="result-row-"]');
      if (row) await scrollTo(row, "start");
      return text("result-filter-summary") ?? "";
    });
    await journey.scrolled("results", "The Results stage", 3);
    await journey.step("report", "The Review stage's Report tab", async () => {
      await openStage("review");
      await waitFor("the report panel", () => byTestId("rendered-report-panel") || byTestId("report-panel"), T.step);
      return text("report-package-source-unavailable") ?? text("rendered-report-precondition") ?? "";
    });
    await journey.scrolled("report", "The Report tab", 4);
  }

  const JOURNEYS = { "solve-and-results": solveAndResults, report, exports: exportsPage, "reference-system": referenceSystem };

  (async () => {
    const result = { nonce: null, ok: false, journey: null, user_agent: navigator.userAgent };
    let journey = null;
    try {
      const plan = await call("native_smoke_plan");
      nonce = plan.nonce;
      result.nonce = plan.nonce;
      result.journey = plan.journey;
      if (plan.timeouts) T = { ...T, ...plan.timeouts };
      const run = JOURNEYS[plan.journey];
      if (!run) throw new Error(`unknown journey ${plan.journey}`);
      await waitFor("the desktop shell", () => byTestId("desktop-preview-shell") && byTestId(`tree-row-project-${encodeURIComponent(plan.bundled_project_id)}`), T.step);
      await sleep(1000);
      await log(`native walkthrough ${plan.journey} started`);
      journey = new Journey(plan);
      await run(journey, plan);
      result.ok = true;
    } catch (error) {
      result.error = `${error?.message ?? error}${error?.stack ? `\n${error.stack}` : ""}`;
      await log(`FAILED: ${result.error}`);
    }
    result.steps = journey?.count ?? 0;
    result.stopped_at = journey?.broken ?? null;
    await call("native_smoke_finish", { report: result });
  })();
})();
