// Driver of the native walkthrough (README.md, "The launched app"), evaluated in the launched `native-smoke` app's
// real web view once the page has loaded (`src-tauri/src/native_smoke.rs`). It is not evaluated as it stands: the
// runner (`run.mjs`, composeDriver) writes it into one script after the text of readScreenCounts and
// scrollLongestRegion (`../screen-counts.mjs`, the browser walkthrough's own) and takeSnapshot (`snapshot.mjs`).
//
// It works the window as a person would: native menu commands through the dispatch a menu click uses, and the
// window's own controls. The only substitution is the file chooser: `native_smoke_choose` names the file the next
// one returns (every answer the runner plans lies inside the throwaway home). Print is not pressed. At each step it
// records the counts and a snapshot of the document as drawn, and writes them as the checkpoint `step-<number>`
// (`native_smoke_checkpoint`, no store copy). Once per launch it reads the menu the app built
// (`native_smoke_menu_items`) and writes it as the checkpoint `menu`; with the plan's `show_window` (the runner's
// --show-window) it first has the app bring its own window to the front (`native_smoke_show_window`), waits for the
// page to report itself visible, writes the outcome as the checkpoint `window`, and at each step waits for a drawn
// frame of the 3D view before the record. A step whose control is missing or disabled is recorded as where a
// person stops, with the reason the window gives, and the journey's later steps as not reached. It judges nothing.
(() => {
  const ipc = window.__TAURI_INTERNALS__;
  // The scripted-run build's substitutions for a person's act (DEL-07-11 REQ-010). Each one a step uses is recorded
  // on that step, so the index can mark it: the file a chooser returns, a native menu command, the print dialog.
  let substitutions = null;
  const SUBSTITUTED = {
    native_smoke_choose: (args) => ({ kind: "file chooser", what: `the next file chooser returns ${String(args.path).split("/").pop()}, named by the harness; no chooser is shown` }),
    native_smoke_menu: (args) => ({ kind: "native menu command", what: `${args.commandId} delivered through the menu's own dispatch, not by a click in the menu bar` }),
    native_smoke_take_report_prints: () => ({ kind: "report print dialog", what: "the print document recorded in place of the system print dialog" })
  };
  const call = (command, args) => {
    if (substitutions && SUBSTITUTED[command]) substitutions.push(SUBSTITUTED[command](args ?? {}));
    return ipc.invoke(command, args ?? {});
  };
  const log = (line) => call("native_smoke_log", { line: String(line) }).catch(() => {});
  let nonce = null;
  let T = { ui: 60000, step: 300000, solve: 900000, save: 900000 };
  // Whether this launch's window was brought to the front (--show-window) and its page then reported visible.
  let windowShown = false;
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
  // A step acts as a person does: only on what is shown, never inside a section or form left closed (a query can find
  // a control there, a person cannot). `acted` collects, for the step's record, each control used and that it was
  // shown and in no closed section at that moment.
  let acted = null;
  const closedSection = (element) => {
    const details = element.closest("details:not([open])");
    const summary = element.closest("summary");
    if (details && !(summary && summary.parentElement === details)) return `the closed section "${details.querySelector(":scope > summary")?.textContent?.replace(/\s+/g, " ").trim() ?? "untitled"}"`;
    if (element.closest("[hidden], [inert], [aria-hidden='true']")) return "a hidden or inert part of the page";
    return null;
  };
  function acting(element, name) {
    const closed = closedSection(element);
    if (closed) throw new Error(`${name} is inside ${closed}: a person opens it first`);
    if (!visible(element)) throw new Error(`${name} is not shown`);
    if (acted) acted.push({ control: name, shown: true, in_closed_section: false });
    return element;
  }
  // The control a person would press: missing, hidden, closed away or disabled is where they stop, with the reason.
  async function control(id, { root, timeoutMs = T.ui } = {}) {
    const found = await waitFor(`the control ${id}`, () => { const item = byTestId(id, root); return visible(item) ? item : null; }, timeoutMs).catch(() => null);
    if (!found) {
      const present = byTestId(id, root);
      const closed = present && closedSection(present);
      throw new Error(closed ? `the control ${id} is inside ${closed}: a person opens it first` : present ? `the control ${id} is on the page but not shown` : `the window shows no control ${id}`);
    }
    if (found.disabled || found.getAttribute("aria-disabled") === "true") throw new Error(`the control ${id} is disabled: "${disabledReason(found)}"`);
    return acting(found, id);
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
    for (const id of ["key-results-close", "solve-progress-close"]) { const item = byTestId(id); if (visible(item)) { acting(item, id).click(); await settle(); } }
    for (const drawer of ["issues-home", "audit-boundary-drawer"]) {
      const close = byTestId(drawer) && [...byTestId(drawer).querySelectorAll("button")].find((button) => button.textContent.trim() === "Close" || button.getAttribute("aria-label") === "Close");
      if (visible(close)) { acting(close, `${drawer} Close`).click(); await settle(); }
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
      try { snapshot = windowShown ? await drawnSnapshot() : await takeSnapshot(); } catch (error) { snapshot = null; counts.snapshot_error = String(error?.message ?? error); }
      return { ...counts, snapshot, window_shown: windowShown, page_visibility: document.visibilityState };
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
      substitutions = [];
      acted = [];
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
      record.substitutions = substitutions;
      record.acted_on = acted;
      substitutions = null;
      acted = null;
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

  // ------------------------------------------------------------ the window and the menu (once per launch)

  // With the window shown: the page visible first, then a snapshot whose 3D view read a drawn frame, tried up to
  // three times, half a second apart; the last attempt is kept, and the notes say how many were made.
  async function drawnSnapshot() {
    await waitFor("the page reported visible", () => document.visibilityState === "visible", 5000).catch(() => null);
    let snapshot = null;
    let attempts = 0;
    while (attempts < 3) {
      attempts += 1;
      snapshot = await takeSnapshot({ frameTimeoutMs: 5000 });
      if (snapshot.view_3d !== "not captured") break;
      await sleep(500);
    }
    if (attempts > 1) snapshot.notes.push(`the 3D view was read ${attempts} times for a drawn frame (${snapshot.view_3d} at the last)`);
    return snapshot;
  }
  async function showWindow() {
    const record = { nonce, at: Date.now() };
    try {
      record.window = await call("native_smoke_show_window");
      const started = Date.now();
      const visible = await waitFor("the page reported visible", () => document.visibilityState === "visible", 10000).then(() => true, () => false);
      record.waited_ms = Date.now() - started;
      record.page_visibility = document.visibilityState;
      record.focus = document.hasFocus() ? "held" : "not held";
      windowShown = Boolean(record.window?.visible) && visible;
    } catch (error) {
      record.error = String(error?.message ?? error).slice(0, 400);
    }
    await log(`window: ${JSON.stringify(record)}`);
    await call("native_smoke_checkpoint", { name: "window", report: record, storeSnapshot: false });
  }
  async function readMenu() {
    const record = { nonce, read_when: "once, when the shell first showed and before the journey's first step" };
    try { record.menus = await call("native_smoke_menu_items"); } catch (error) { record.error = String(error?.message ?? error).slice(0, 400); }
    await call("native_smoke_checkpoint", { name: "menu", report: record, storeSnapshot: false });
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
      acting(row, row.getAttribute("data-testid")).click();
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
        const button = await waitFor(`key-results-${action}`, () => { const item = byTestId(`key-results-${action}`); return visible(item) ? item : null; }, T.ui).catch(() => null);
        if (!button) throw new Error(`Key results shows no ${action} view control`);
        // An unavailable view is pressed too: it says why, which is what a person meets.
        acting(button, `key-results-${action}`).click();
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
      acting(load, "c2-result-load").click();
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

  // A prepared project (run.mjs JOURNEYS `place` and `open`): the files were placed in the home before the app
  // started; it is opened here by the File menu, as a person opens it. One step, "opened".
  async function openPrepared(journey, plan, what) {
    const open = plan.open;
    if (!open) throw new Error("the journey declares nothing to open");
    if (open.by === "model-document") {
      const name = open.file.split("/").pop();
      await journey.step("opened", `File > Open Model Document… of ${what} (the chooser returns the placed ${name})`, async () => {
        const before = text("local-project-message") ?? "";
        await call("native_smoke_choose", { path: open.file });
        await menu("file.open-model");
        const shown = await waitFor(`the open of ${name}`, () => {
          const now = text("local-project-message") ?? "";
          return now !== before && (now.includes(`Opened model document ${name} `) || now.includes(`refused (${name})`)) ? now : null;
        }, T.step);
        if (!shown.includes("Opened model document")) throw new Error(shown);
        await waitFor("the opened project in the model tree", () => byTestId(`tree-row-project-${encodeURIComponent(open.project_id)}`), T.step);
        return shown;
      });
    } else if (open.by === "local-project") {
      await journey.step("opened", `File > Open Local Project… of ${what}, from the store placed in the home`, async () => {
        const before = text("local-project-message") ?? "";
        await menu("file.open-local");
        const shown = await waitFor("the open's outcome", () => { const now = text("local-project-message") ?? ""; return now && now !== before ? now : null; }, T.step);
        if (open.project_id) await waitFor("the opened project in the model tree", () => byTestId(`tree-row-project-${encodeURIComponent(open.project_id)}`), T.ui).catch(() => { throw new Error(shown); });
        return shown;
      });
    } else {
      throw new Error(`the journey declares an unknown way to open: ${open.by}`);
    }
  }

  // E. A prepared project, opened: the proof that a journey can start from files placed before launch.
  async function preparedProject(journey, plan) {
    await journey.step("first-open", "The application as first opened, before the prepared project is opened");
    await openPrepared(journey, plan, "the prepared project");
    await journey.step("model-tree", "The opened project in the model tree", async () => {
      await closeOverlays();
      const row = byTestId(`tree-row-project-${encodeURIComponent(plan.open.project_id)}`);
      if (!visible(row)) await press("rail-stage-model");
      await scrollTo(byTestId(`tree-row-project-${encodeURIComponent(plan.open.project_id)}`), "start");
    });
    await journey.step("loads-stage", "The opened project's Loads stage", async () => { await openStage("loads"); });
  }

  // D. The reference system, opened as a model document and solved.
  async function referenceSystem(journey, plan) {
    await journey.step("first-open", "The application as first opened");
    await openPrepared(journey, plan, "the reference system");
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

  // ------------------------------------------------------------ the use cases of DEL-07-06 AC-007 (journeys 32 to 36)

  const said = (element) => element?.textContent?.replace(/\s+/g, " ").trim() ?? "";
  const shown = (id) => { const item = byTestId(id); return visible(item) ? said(item) : null; };
  // A native menu command, as a person chooses it: only an item the built menu shows enabled (read from the menu the
  // app built), then delivered through the menu's own dispatch (marked as substituted on the step).
  function menuEntry(items, commandId) {
    for (const item of items ?? []) {
      if (item.kind === "item" && item.command_id === commandId) return item;
      if (item.kind === "submenu") { const found = menuEntry(item.items, commandId); if (found) return found; }
    }
    return null;
  }
  async function menuCommand(commandId) {
    const item = menuEntry(await call("native_smoke_menu_items"), commandId);
    if (!item) throw new Error(`the menu bar has no item ${commandId}`);
    if (item.enabled === false) throw new Error(`the menu item "${item.label}" is disabled`);
    await menu(commandId);
    return item.label;
  }
  // A choice in the product's compact selector: the selector pressed, then the option a person reads in its list.
  async function choose(id, value, { root } = {}) {
    const selector = await control(id, { root });
    if (selector.getAttribute("data-value") === value) return said(selector);
    selector.click();
    const list = await waitFor(`the list of ${id}`, () => { const listId = selector.getAttribute("aria-controls"); const found = listId && document.getElementById(listId); return visible(found) ? found : null; }, T.ui);
    const option = [...list.querySelectorAll('[role="option"]')].find((item) => item.getAttribute("data-value") === value);
    if (!option) throw new Error(`the list of ${id} offers no ${value}: it offers ${[...list.querySelectorAll('[role="option"]')].map(said).join(", ").slice(0, 300)}`);
    if (option.getAttribute("aria-disabled") === "true") throw new Error(`the option ${said(option)} of ${id} is disabled`);
    acting(option, `${id} option ${value}`).click();
    await waitFor(`${id} set to ${value}`, () => selector.getAttribute("data-value") === value, T.ui);
    return said(option);
  }
  // Typing into a field: focused, its value replaced, the input event a keystroke sends.
  async function type(id, value, { root } = {}) {
    const field = await control(id, { root });
    field.focus();
    setValue(field, String(value));
    await settle();
    return field;
  }
  // The model tree, as a person opens it: any page over it closed, the Model stage, its Tree tab, the tree rail open.
  async function showTree() {
    await closeOverlays();
    if (visible(byTestId("workspace-dock-close"))) await press("workspace-dock-close");
    if (!visible(byTestId("shell-tree-host"))) {
      if (byTestId("rail-stage-model")?.getAttribute("aria-current") !== "page") await press("rail-stage-model");
      const tab = byTestId("stage-tab-model-tree");
      if (tab && tab.getAttribute("aria-pressed") !== "true") await press("stage-tab-model-tree");
      if (byTestId("toggle-tree")?.getAttribute("aria-expanded") !== "true") await press("toggle-tree");
    }
    await waitFor("the model tree", () => visible(byTestId("shell-tree-host")), T.ui);
  }
  // An entity chosen in the model tree, found by typing its id into the tree's filter.
  async function selectInTree(type_, id) {
    await showTree();
    await type("model-tree-filter-input", id);
    await press(`tree-row-${encodeURIComponent(type_)}-${encodeURIComponent(id)}`);
    return shown("status-selection") ?? "";
  }
  async function openInspector() {
    const toggle = byTestId("toggle-inspector");
    if (toggle?.getAttribute("aria-disabled") === "true" && toggle.getAttribute("aria-expanded") !== "true") await press("view-switch-both");
    if (byTestId("toggle-inspector")?.getAttribute("aria-expanded") !== "true") await press("toggle-inspector");
    return waitFor("the property inspector", () => (visible(byTestId("property-inspector")) ? byTestId("property-inspector") : null), T.ui);
  }

  // THE edit: every change these journeys make to the model goes through this one helper, so that when the way a
  // person commits an edit changes, only this changes. Today (the inspector's Task tab, on the selected entity): the
  // Task tab, "Start task from current selection", the Property chosen, the new value typed (and its unit chosen where
  // the field takes one), then Apply (`apply-editor-intent-inline`), which applies that exact change at once. Three
  // pictures: the empty form, the form filled, and the window once Apply has answered. Returns nothing; a refused
  // Apply stops the journey with the window's words.
  async function editValue(journey, name, what, { fieldPath, value, unit = null }) {
    await journey.step(`${name}-form`, `The inspector's Task form for ${what}, the property chosen, nothing entered yet`, async () => {
      await closeOverlays();
      const inspector = await openInspector();
      const tab = [...inspector.querySelectorAll('[role="tab"]')].find((item) => said(item) === "Task");
      if (!tab) throw new Error("the inspector shows no Task tab");
      if (tab.getAttribute("aria-selected") !== "true") { acting(tab, "inspector Task tab").click(); await settle(); }
      if (visible(byTestId("inspector-frozen-task-target"))) await press("cancel-editor-intent", { root: inspector });
      await press("inspector-start-task", { root: inspector });
      const property = await choose("editor-intent-field", fieldPath, { root: inspector });
      return `${shown("inspector-frozen-task-target") ?? ""}; Property: ${property}`;
    });
    await journey.step(`${name}-entered`, `The value entered for ${what}`, async () => {
      const inspector = byTestId("property-inspector");
      await type("editor-intent-value", value, { root: inspector });
      if (unit !== null) {
        const unitControl = byTestId("editor-intent-unit", inspector);
        if (!visible(unitControl)) throw new Error("the form shows no unit control for this property");
        if (unitControl.getAttribute("role") === "combobox") await choose("editor-intent-unit", unit, { root: inspector });
        else await type("editor-intent-unit", unit, { root: inspector });
      }
      await sleep(300);
      return [shown("editor-intent-missing-project-unit"), shown("editor-intent-inline-validation-status")].filter(Boolean).join("; ");
    });
    await journey.step(`${name}-applied`, `Apply pressed for ${what}`, async () => {
      const inspector = byTestId("property-inspector");
      await press("apply-editor-intent-inline", { root: inspector });
      const outcome = await waitFor("the Apply's outcome", () => {
        if (visible(byTestId("inspector-task-empty"))) return { applied: true };
        const refusal = [...inspector.querySelectorAll('[role="alert"]')].filter(visible).map(said).filter(Boolean);
        return refusal.length ? { refusal: refusal.join(" ") } : null;
      }, T.step).catch(() => ({ refusal: `nothing in the inspector answered Apply: ${[shown("editor-intent-inline-validation-status"), shown("operation-apply-summary")].filter(Boolean).join("; ") || "no message shown"}` }));
      if (outcome.refusal) throw new Error(`Apply was not taken: ${outcome.refusal.slice(0, 300)}`);
      return [shown("operation-apply-summary"), shown("toolbar-project-name")].filter(Boolean).join("; ");
    });
  }

  // What the window shows once Run was pressed: the solve's own outcome, or Run withheld with its reason. Recorded,
  // not judged: a refusal here is what the use case starts from.
  async function runAnswer() {
    const answer = await waitFor("the window's answer to Run", () => {
      for (const id of ["solve-progress-failure", "solve-progress-refusal", "solve-progress-cancellation", "solve-progress-result-too-large", "solve-progress-completed"]) {
        const found = shown(id);
        if (found) return `${id.replace("solve-progress-", "")}: ${found}`;
      }
      const run = byTestId("toolbar-run");
      if (run?.getAttribute("aria-disabled") === "true") return `Run withheld: ${disabledReason(run)}`;
      return null;
    }, T.solve);
    return `${answer.slice(0, 400)}; the status bar: ${shown("status-chips") ?? "no status chips"}`;
  }
  // Each finding a person can choose in the Issues drawer (the drawer opened from the status bar), with its words.
  const FINDING_SELECTOR = '[data-testid^="diagnostic-"], [data-testid^="model-check-"]';
  const findingButtons = () => [...(byTestId("issues-home")?.querySelectorAll(FINDING_SELECTOR) ?? [])]
    .filter((item) => (item.tagName === "BUTTON" || item.getAttribute("role") === "button") && visible(item));
  async function openIssues() {
    await closeOverlays();
    if (visible(byTestId("workspace-dock-close"))) await press("workspace-dock-close");
    if (!visible(byTestId("issues-home"))) await press("issues-drawer-toggle");
    await waitFor("the Issues drawer", () => visible(byTestId("issues-home")), T.ui);
    const findings = findingButtons().map(said);
    const missing = shown("missing-data-summary");
    return `${findings.length} finding(s) to choose${findings.length ? `: ${findings.join(" | ").slice(0, 600)}` : ""}${missing ? `; ${missing}` : ""}`;
  }
  // From a finding that names `entityId`, to its entity. A finding's row in the Issues drawer shows its code and message;
  // the entity it concerns is read in the Diagnostic Detail once it is chosen. So, as a person does, the findings are
  // chosen in turn (the row's own text first, where it names the entity) until the detail names `entityId`; the note
  // lists each one chosen and what its detail named. Then what the window selected. Where the finding selects nothing,
  // or another entity, the note says so and the journey goes on from the model tree, as a person would.
  // The finding (its test id, which carries its code) that last led to an entity.
  let lastFindingCode = null;
  const names = (text_, entityId) => text_.includes(entityId) || text_.includes(entityId.replace(/^[a-z]+:/, ""));
  async function followFinding(journey, name, entityType, entityId) {
    await journey.step(`${name}-finding`, `The finding that names ${entityId}, chosen in the Issues drawer`, async () => {
      await openIssues();
      const tried = [];
      const rows = findingButtons();
      const ordered = [...rows.filter((item) => names(said(item), entityId)), ...rows.filter((item) => !names(said(item), entityId))];
      for (const finding of ordered) {
        if (!finding.isConnected || !visible(finding)) continue;
        await scrollTo(finding, "center");
        acting(finding, finding.getAttribute("data-testid")).click();
        await sleep(500);
        const refs = shown("selected-diagnostic-affected-refs") ?? "";
        tried.push(`${said(finding).slice(0, 60)} (its detail: ${refs || "no affected refs shown"})`);
        if (names(said(finding), entityId) || names(refs, entityId)) {
          lastFindingCode = finding.getAttribute("data-testid");
          await scrollTo(byTestId("diagnostic-detail-panel") ?? finding, "center");
          return `chosen in turn: ${tried.join("; ")}; the status bar's selection: ${shown("status-selection") || "none"}`;
        }
      }
      throw new Error(`no finding in the Issues drawer names ${entityId} in its row or its detail; chosen in turn: ${tried.join("; ").slice(0, 300) || "the drawer lists none"}`);
    });
    await journey.step(`${name}-entity`, `${entityId}, the entity the finding names`, async () => {
      await closeOverlays();
      const now = shown("status-selection") ?? "";
      if (names(now, entityId)) return `selected by the finding: ${now}`;
      return `the finding did not select it (the status bar's selection: ${now || "none"}); chosen in the model tree: ${await selectInTree(entityType, entityId)}`;
    });
  }

  // 32. Missing data: a committed model that lacks values a solve requires, opened; Run refused; each value found
  // from its finding and entered; the solve completes.
  async function missingData(journey, plan) {
    await journey.step("first-open", "The application as first opened");
    await openPrepared(journey, plan, "the missing-data model");
    await journey.step("model-tree", "The opened model in the model tree", async () => { await showTree(); });
    await journey.step("run", "Run pressed on the opened model", pressRun);
    await journey.step("run-answer", "What the window says after Run", runAnswer);
    await journey.step("issues", "The Issues drawer, from the status bar", openIssues);
    await journey.scrolled("issues", "The Issues drawer", 2, '[data-testid="issues-home"]');
    await journey.step("analyze-page", "The Analyze page after the refused Run", async () => { await closeOverlays(); await showSection("solve"); return shown("solve-progress-state") ?? ""; });
    await journey.step("results-stage", "The Results stage after the refused Run", async () => { await openStage("results"); return shown("result-filter-summary") ?? ""; });
    for (const [index, entry] of (plan.entries ?? []).entries()) {
      const name = `value-${index + 1}`;
      const [entityType] = entry.entity_ref.split(":");
      // Where the Issues drawer no longer lists any finding of the code that led to the last value (an edit clears
      // the refused solve's findings), a person runs again to see what is still missing.
      if (index > 0 && lastFindingCode && !findingButtons().some((item) => item.getAttribute("data-testid") === lastFindingCode)) {
        await journey.step(`${name}-run`, "Run pressed again to see what is still missing", pressRun);
        await journey.step(`${name}-run-answer`, "What the window says after that Run", runAnswer);
      }
      await followFinding(journey, name, entityType, entry.entity_ref);
      await editValue(journey, name, `${entry.field_path} of ${entry.entity_ref}`, { fieldPath: `${entry.field_path}.value`, value: entry.value, unit: entry.unit });
      if (index === 0) {
        await journey.step(`${name}-undone`, "Edit > Undo Model Edit", async () => menuCommand("edit.undo"));
        await journey.step(`${name}-undone-issues`, "The Issues drawer after the undo", openIssues);
        await journey.step(`${name}-redone`, "Edit > Redo Model Edit", async () => { await closeOverlays(); return menuCommand("edit.redo"); });
      }
      await journey.step(`${name}-issues`, `The Issues drawer once ${entry.entity_ref} has its value`, openIssues);
    }
    await journey.step("run-again", "Run pressed again", pressRun);
    await journey.step("run-again-answer", "What the window says after the second Run", runAnswer);
    await journey.step("results", "The Results stage", async () => {
      await openStage("results");
      await waitFor("the results table's rows", () => document.querySelector('[data-testid^="result-row-"]'), T.step).catch(() => null);
      return shown("result-filter-summary") ?? "";
    });
    await journey.step("issues-after", "The Issues drawer after the solve", openIssues);
  }

  // Each disclosure of a region opened by its own summary, as a person opens it.
  async function openDisclosures(id) {
    const region = byTestId(id);
    if (!visible(region)) return 0;
    let opened = 0;
    for (const details of region.querySelectorAll("details:not([open])")) {
      const summary = details.querySelector(":scope > summary");
      if (!visible(summary)) continue;
      acting(summary, `${id} ${said(summary).slice(0, 40)}`).click();
      opened += 1;
      await settle();
    }
    return opened;
  }
  // What the window says of the result it holds: the Results stage's own words about it. Where the rail's Results
  // control is disabled, that is what a person meets: its own reason is recorded, the rail is pictured, and the
  // journey goes on (a person cannot open the stage, but is not stuck).
  async function heldResultWords() {
    await closeOverlays();
    const rail = byTestId("rail-stage-results");
    if (visible(rail) && (rail.disabled || rail.getAttribute("aria-disabled") === "true")) {
      if (acted) acted.push({ control: "rail-stage-results", shown: true, in_closed_section: false, disabled: true });
      return `the rail's Results control is disabled: "${disabledReason(rail)}"`;
    }
    await openStage("results");
    await sleep(500);
    const section = byTestId("workspace-section-results");
    const words = [...(section?.querySelectorAll('[role="status"], [role="note"], [role="alert"], [data-testid*="stale"], [data-testid*="historical"], [data-testid*="held"]') ?? [])]
      .filter(visible).map(said).filter(Boolean);
    return [...new Set(words)].join(" | ").slice(0, 600) || "the Results stage states nothing about the result it holds";
  }

  // 33. Failed solve: the demonstration model, a pipe's wall made thicker than its radius by the person in the
  // window (the start: a model the solver refuses); the refusal read at each place the window shows it; the pipe
  // reached from its finding and corrected; the solve completes.
  async function failedSolve(journey) {
    await journey.step("first-open", "The application as first opened, with its demonstration model");
    await journey.step("pipe-chosen", "pipe:P-100 chosen in the model tree", async () => selectInTree("pipe", "pipe:P-100"));
    await editValue(journey, "wall-made-thick", "the wall thickness of pipe:P-100 (0.1 m on a 0.168 m outside diameter)", { fieldPath: "section.wall_thickness.value", value: "0.1" });
    await journey.step("run", "Run pressed on the edited model", pressRun);
    await journey.step("run-answer", "What the window says after Run", runAnswer);
    await journey.step("run-answer-details", "The solve's progress panel with its disclosures opened", async () => `${await openDisclosures("solve-progress-panel")} disclosure(s) opened; ${shown("solve-progress-state") ?? ""}`);
    await journey.step("issues", "The Issues drawer, from the status bar", openIssues);
    await journey.scrolled("issues", "The Issues drawer", 2, '[data-testid="issues-home"]');
    await journey.step("analyze-page", "The Analyze page after the refused Run", async () => { await closeOverlays(); await showSection("solve"); return shown("solve-progress-state") ?? ""; });
    await journey.scrolled("analyze-page", "The Analyze page", 2);
    await journey.step("results-stage", "The Results stage after the refused Run", heldResultWords);
    await followFinding(journey, "pipe", "pipe", "pipe:P-100");
    await editValue(journey, "wall-corrected", "the wall thickness of pipe:P-100 (0.007 m, as the model had it)", { fieldPath: "section.wall_thickness.value", value: "0.007" });
    await journey.step("issues-corrected", "The Issues drawer after the correction", openIssues);
    await journey.step("run-again", "Run pressed again", pressRun);
    await journey.step("run-again-answer", "What the window says after the second Run", runAnswer);
    await journey.step("results", "The Results stage", async () => {
      await openStage("results");
      await waitFor("the results table's rows", () => document.querySelector('[data-testid^="result-row-"]'), T.step).catch(() => null);
      return shown("result-filter-summary") ?? "";
    });
  }

  // The Project page's Model History, where states and runs are saved and compared.
  async function showHistory() {
    await closeOverlays();
    await showSection("project");
    const panel = await waitFor("the Model History panel", () => (visible(byTestId("model-history-panel")) ? byTestId("model-history-panel") : null), T.ui);
    await scrollTo(panel, "start");
    return shown("model-history-message") ?? "";
  }
  // A history form's outcome, once the history is no longer busy: the form closed (taken), or its submit enabled again
  // (the form still open: refused or not taken). Each message the history showed meanwhile is recorded in order.
  async function historyAnswer(before, formId, submitId) {
    const said_ = [];
    await waitFor("the history's answer", () => {
      const now = shown("model-history-message") ?? "";
      if (now && now !== before && now !== said_.at(-1)) said_.push(now);
      const form = byTestId(formId);
      if (!visible(form)) return "closed";
      const submit = byTestId(submitId);
      return said_.length > 0 && submit && !submit.disabled ? "open" : null;
    }, T.save).catch(() => null);
    const form = visible(byTestId(formId)) ? "the form is still open" : "the form closed";
    return `${said_.join(" / ") || `the history said nothing new (${before || "no message"})`}; ${form}`;
  }
  async function saveState(journey, name, stateName) {
    await journey.step(`${name}-form`, "Save Model State…: its form, empty", async () => { await showHistory(); await press("model-state-save-open"); await scrollTo(await control("model-state-save-form"), "center"); });
    await journey.step(`${name}-filled`, `The form filled: the name "${stateName}"`, async () => { await type("model-state-save-name", stateName); });
    await journey.step(`${name}-saved`, "Save state pressed", async () => {
      const before = shown("model-history-message") ?? "";
      await press("model-state-save-submit");
      return historyAnswer(before, "model-state-save-form", "model-state-save-submit");
    });
  }
  async function recordRun(journey, name, runName) {
    await journey.step(`${name}-form`, "Record Run…: its form, empty", async () => { await showHistory(); await press("record-run-open"); await scrollTo(await control("record-run-form"), "center"); return shown("record-run-state") ?? ""; });
    await journey.step(`${name}-filled`, `The form filled: the name "${runName}"`, async () => {
      await type("record-run-name", runName);
      if (visible(byTestId("record-run-snapshot-name"))) await type("record-run-snapshot-name", `${runName}, solved model`);
    });
    await journey.step(`${name}-recorded`, "Record run pressed", async () => {
      const before = shown("model-history-message") ?? "";
      await press("record-run-submit");
      return historyAnswer(before, "record-run-form", "record-run-submit");
    });
  }
  // Ticks the compare box of the first two listed states (`runs` false) or runs (`runs` true), then Compare….
  async function compareTwo(runs) {
    await showHistory();
    const list = byTestId("model-history-list");
    if (!visible(list)) throw new Error(`the history lists nothing to compare: ${shown("model-history-empty") ?? shown("model-history-message") ?? "no reason shown"}`);
    const boxes = [...list.querySelectorAll('input[type="checkbox"][data-testid$="-compare-select"]')]
      .filter((box) => Boolean(box.closest(".model-history-runs")) === runs && box.getAttribute("data-testid") !== "current-run-compare-select");
    if (boxes.length < 2) throw new Error(`the history offers ${boxes.length} ${runs ? "run" : "state"}(s) to tick for comparison`);
    for (const box of boxes.slice(0, 2)) {
      if (box.checked) continue;
      if (box.disabled) throw new Error(`the compare box ${box.getAttribute("data-testid")} is disabled: "${disabledReason(box)}"`);
      await scrollTo(box, "center");
      acting(box, box.getAttribute("data-testid")).click();
      await settle();
    }
    await press("model-state-compare");
    const view = runs ? "run-comparison" : "state-comparison";
    const shownView = await waitFor(`the ${view}`, () => (visible(byTestId(view)) ? byTestId(view) : null), T.step)
      .catch(() => { throw new Error(`Compare… showed no comparison: ${shown("model-history-message") ?? "no message"}`); });
    await scrollTo(shownView, "start");
    return shown(`${view}-summary`) ?? "";
  }
  async function exportComparison(view) {
    const button = [...(byTestId(view)?.querySelectorAll("button") ?? [])].find((item) => /^Export CSV/.test(item.getAttribute("aria-label") ?? ""));
    if (!button) throw new Error(`the ${view} offers no Export CSV control`);
    await scrollTo(button, "center");
    const id = button.getAttribute("data-testid");
    if (id) await press(id); else { acting(button, `${view} Export CSV`).click(); await settle(); }
    const status = await waitFor("the export's outcome", () => { const now = said(document.querySelector(`[data-testid="${id}-status"]`)); return now && !/pending|Saving|Exporting/i.test(now) ? now : null; }, T.save)
      .catch(() => "no outcome shown");
    return status;
  }

  // 34. Change a support and compare: after a solve, the state saved and the run recorded; support:S-120 moved to
  // node:N-110; the state saved, solved again, the run recorded; the two states and the two runs compared in a table
  // and in the 3D view; the comparison exported.
  async function changeAndCompare(journey) {
    await journey.step("run", "Run pressed on the demonstration model", pressRun);
    await journey.step("solved", "The solve, once it ended", awaitSolve);
    await journey.step("history", "The Project page's Model History after the solve", showHistory);
    await saveState(journey, "state-1", "Before the support change");
    await recordRun(journey, "run-1", "Solve before the support change");
    await journey.step("support-chosen", "support:S-120 chosen in the model tree", async () => selectInTree("support", "support:S-120"));
    await editValue(journey, "support-moved", "the node of support:S-120 (node:N-110 in place of node:N-120)", { fieldPath: "node", value: "node:N-110" });
    await journey.step("held-result", "What the Results stage says of the held result after the edit", heldResultWords);
    await saveState(journey, "state-2", "After the support change");
    await journey.step("run-2", "Run pressed on the changed model", pressRun);
    await journey.step("solved-2", "The second solve, once it ended", awaitSolve);
    await recordRun(journey, "run-2-record", "Solve after the support change");
    await journey.step("history-2", "Model History with both states and runs", showHistory);
    await journey.scrolled("history-2", "Model History", 2);
    await journey.step("states-compared", "The two saved states ticked and compared", async () => compareTwo(false));
    await journey.scrolled("states-compared", "The state comparison", 3);
    await journey.step("states-exported", "The state comparison's Export CSV", async () => exportComparison("state-comparison"));
    await journey.step("states-closed", "The state comparison closed", async () => { await press("state-comparison-close"); });
    await journey.step("runs-compared", "The two recorded runs ticked and compared", async () => compareTwo(true));
    await journey.scrolled("runs-compared", "The run comparison", 3);
    await journey.step("runs-exported", "The run comparison's Export CSV", async () => exportComparison("run-comparison"));
    await journey.step("viewport", "The 3D view after the comparison (Model view)", async () => {
      await closeOverlays();
      if (visible(byTestId("workspace-dock-close"))) await press("workspace-dock-close");
      await press("rail-stage-model");
      if (byTestId("view-switch-model")?.getAttribute("aria-pressed") !== "true") await press("view-switch-model");
      return shown("status-selection") ?? "";
    });
  }

  // 35. User rule pack: after a solve, a private rule pack drafted in the rule-pack editor and saved into the local
  // store, loaded for a run, applied, each outcome read, and the missing rule inputs looked for as findings.
  async function rulePack(journey) {
    await journey.step("run", "Run pressed on the demonstration model", pressRun);
    await journey.step("solved", "The solve, once it ended", awaitSolve);
    await journey.step("rules-page", "The Rules page, from the View menu", async () => { await closeOverlays(); await showSection("rule-packs"); return shown("rule-pack-scope-status") ?? ""; });
    await journey.step("new-draft", "A new rule pack draft", async () => { await press("rule-pack-new-draft"); return shown("rule-pack-action-status") ?? ""; });
    await journey.scrolled("new-draft", "The rule pack draft", 4);
    await journey.step("validated", "Validate pressed", async () => {
      await press("rule-pack-validate");
      await sleep(500);
      const summary = await waitFor("the validation", () => shown("rule-pack-validation-summary") ?? shown("rule-pack-action-status"), T.step);
      await scrollTo(byTestId("rule-pack-validation") ?? byTestId("rule-pack-action-status"), "center");
      return summary;
    });
    await journey.step("saved", "Save pressed: the draft saved into the local store", async () => {
      const before = shown("rule-pack-action-status") ?? "";
      await press("rule-pack-save");
      const answer = await waitFor("the save's answer", () => { const now = shown("rule-pack-action-status") ?? ""; return now !== before ? now : null; }, T.step).catch(() => before);
      await scrollTo(byTestId("rule-pack-list"), "center");
      return `${answer}; ${shown("rule-pack-list-status") ?? ""}`;
    });
    await journey.step("rule-checks", "Analyze > Rule Checks", async () => { await closeOverlays(); await menuCommand("analyze.rule-checks"); await waitFor("the rule check panel", () => visible(byTestId("rule-check-run-panel")), T.ui); await scrollTo(byTestId("rule-check-run-panel"), "start"); return shown("rule-check-run-solve-status") ?? ""; });
    await journey.step("pack-loaded", "The saved pack's Load for run", async () => {
      if (visible(byTestId("rule-check-refresh-list"))) await press("rule-check-refresh-list");
      const open = await waitFor("a saved pack's Load for run", () => [...document.querySelectorAll('[data-testid^="rule-check-open-"]')].find(visible), T.ui).catch(() => null);
      if (!open) throw new Error(`Rule Checks lists no saved pack to load: ${shown("rule-check-run-list-status") ?? "no list status"}`);
      await scrollTo(open, "center");
      await press(open.getAttribute("data-testid"));
      await sleep(500);
      await scrollTo(byTestId("rule-check-binding-plan") ?? open, "start");
      return shown("rule-check-binding-precheck") ?? "";
    });
    await journey.scrolled("pack-loaded", "The binding plan", 3);
    await journey.step("applied", "Run rule checks pressed", async () => {
      await scrollTo(await control("rule-check-run", { timeoutMs: T.ui }), "center");
      await press("rule-check-run");
      const status = await waitFor("the rule check's outcome", () => shown("rule-check-aggregate-status") ?? shown("rule-check-run-refusal"), T.step)
        .catch(() => shown("rule-check-run-status") ?? "no outcome shown");
      await scrollTo(byTestId("rule-check-run-result") ?? byTestId("rule-check-run-status") ?? byTestId("rule-check-run"), "start");
      return status;
    });
    await journey.scrolled("applied", "The rule check outcomes", 3);
    await journey.step("issues", "The Issues drawer after the rule check", openIssues);
    await journey.scrolled("issues", "The Issues drawer", 2, '[data-testid="issues-home"]');
  }

  const JOURNEYS = { "solve-and-results": solveAndResults, report, exports: exportsPage, "reference-system": referenceSystem, "prepared-project": preparedProject };
  Object.assign(JOURNEYS, { "missing-data": missingData, "failed-solve": failedSolve, "change-and-compare": changeAndCompare, "rule-pack": rulePack });

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
      if (plan.show_window) await showWindow();
      await readMenu();
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
