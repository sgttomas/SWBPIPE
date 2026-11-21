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
    // The solve status line lies over nothing (it is a band in the page), so it stays.
    for (const id of ["key-results-close"]) { const item = byTestId(id); if (visible(item)) { acting(item, id).click(); await settle(); } }
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

  // ------------------------------------------------------------ shared acts

  // The solve status line's state, as the line states it: the job's own state on `data-state`.
  const solveOutcome = () => {
    const line = byTestId("solve-progress-panel");
    if (!visible(line)) return null;
    const state = line.getAttribute("data-state");
    const said = text("solve-progress-state") ?? "";
    if (state === "failed" || state === "refused" || state === "cancelled") return { failed: `the solve did not complete: ${said.slice(0, 300)}` };
    return state === "completed" ? { completed: said } : null;
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
    await journey.step("details-open", "The status line's Details, opened", async () => {
      await press("solve-progress-details-toggle");
      const opened = byTestId("solve-progress-details-toggle")?.getAttribute("aria-expanded") === "true" && visible(byTestId("solve-progress-details"));
      return opened ? (text("solve-progress-details") ?? "open").slice(0, 400) : "did not open";
    });
    await journey.step("details-closed", "The status line's Details, closed again", async () => { await press("solve-progress-details-toggle"); });
    await journey.step("results-stage", "The Results stage as first shown, by Open results on the status line", async () => {
      await press("solve-progress-open-results");
      await waitFor("the results table's rows", () => document.querySelector('[data-testid^="result-row-"]'), T.step);
      return "Open results pressed";
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
    await journey.step("issues-drawer", "The Issues drawer, from the status bar", async () => {
      await press("issues-drawer-toggle");
      return `toolbar ${text("toolbar-issues") ?? "?"}; status bar ${text("issues-drawer-toggle") ?? "?"}; drawer ${text("issues-total") ?? "?"}; ${text("issues-breakdown") ?? ""}`;
    });
    await journey.scrolled("issues-drawer", "The Issues drawer", 2, '[data-testid="issues-home"]');
    await journey.step("missing-data-closed", "The drawer's Missing data panel, its uncounted entries closed", async () => {
      const panel = await control("missing-data-panel");
      await scrollTo(panel, "start");
      return `heading count ${text("missing-data-blocking-count") ?? "?"}; ${text("missing-data-not-blocking-toggle") ?? "no uncounted entries"}`;
    });
    await journey.step("missing-data-open", "The drawer's Missing data panel, its uncounted entries opened", async () => {
      if (byTestId("missing-data-not-blocking")) await press("missing-data-not-blocking-toggle");
      return text("missing-data-not-blocking-toggle") ?? "no uncounted entries";
    });
    await journey.step("single-item", "A single-report item in the drawer", async () => {
      const row = [...document.querySelectorAll('[data-testid="issues-home"] button[data-testid^="diagnostic-"], [data-testid="issues-home"] button[data-testid^="model-check-"]')]
        .find((button) => !button.closest('[data-testid^="issue-group-"]') && visible(button));
      if (!row) throw new Error("the Issues drawer shows no single-report item");
      await scrollTo(row, "center");
      return row.textContent.replace(/\s+/g, " ").trim().slice(0, 300);
    });
    const hangerGroup = () => document.querySelector('[data-testid="issues-home"] [data-testid="issue-group-SPRING_HANGER_LOAD_NOT_APPLIED_AS_PRELOAD"]') ?? document.querySelector('[data-testid="issues-home"] [data-testid^="issue-group-"]');
    await journey.step("hanger-item-collapsed", "A spring hanger's grouped item, collapsed", async () => {
      const group = await waitFor("a grouped item", hangerGroup, T.ui).catch(() => null);
      if (!group) throw new Error("the Issues drawer shows no grouped item");
      await scrollTo(group, "start");
      return group.textContent.replace(/\s+/g, " ").trim().slice(0, 300);
    });
    await journey.step("hanger-item-open", "A spring hanger's grouped item, its reports opened", async () => {
      const group = await waitFor("a grouped item", hangerGroup, T.ui).catch(() => null);
      if (!group) throw new Error("the Issues drawer shows no grouped item");
      const toggle = group.querySelector('[data-testid="issue-group-members-toggle"]');
      await scrollTo(group, "start");
      acting(toggle, "issue-group-members-toggle").click();
      await sleep(200);
      const itemClass = group.querySelector('[data-testid="issue-class"]')?.textContent?.trim() ?? "no class supplied";
      const notSupplied = [...group.querySelectorAll('[data-testid="issue-member-not-supplied"]')].map((node) => node.textContent.trim());
      return `${group.getAttribute("data-testid")}; item ${itemClass}; opened rows ${notSupplied.join(" | ") || "every field supplied"}`;
    });
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
    await journey.step("solving", "The status line while the job reports it running", async () => {
      const seen = await waitFor("the running solve", () => {
        const state = byTestId("solve-progress-panel")?.getAttribute("data-state");
        return state === "running" || solveOutcome() ? state : null;
      }, T.solve).catch(() => null);
      return seen === "running" ? text("solve-progress-state") ?? "" : `not seen running (${seen ?? "no line"})`;
    });
    await journey.step("solving-details", "The status line's Details, opened on the running job", async () => {
      if (byTestId("solve-progress-panel")?.getAttribute("data-state") !== "running") return "the job was no longer running";
      await press("solve-progress-details-toggle");
      return (text("solve-progress-details") ?? "").slice(0, 400);
    });
    await journey.step("solving-details-closed", "The status line's Details, closed again while the job runs", async () => {
      const toggle = byTestId("solve-progress-details-toggle");
      if (toggle?.getAttribute("aria-expanded") === "true") await press("solve-progress-details-toggle");
      return text("solve-progress-state") ?? "";
    });
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
    await journey.step("issues-drawer", "The Issues drawer, from the toolbar", async () => {
      await press("toolbar-issues");
      return `toolbar ${text("toolbar-issues") ?? "?"}; status bar ${text("issues-drawer-toggle") ?? "?"}; drawer ${text("issues-total") ?? "?"}; ${text("issues-breakdown") ?? ""}`;
    });
    // A stopped solve, as the line says it: Run again, then Stop on the status line.
    await journey.step("run-again", "Run pressed again on the reference system", pressRun);
    await journey.step("stopped", "Stop pressed on the status line, and the line once the job reported", async () => {
      await press("solve-progress-cancel");
      await waitFor("the stopped solve", () => byTestId("solve-progress-panel")?.getAttribute("data-state") === "cancelled", T.solve);
      return text("solve-progress-state") ?? "";
    });
    await journey.step("stopped-details", "The stopped solve's Details, opened", async () => { await press("solve-progress-details-toggle"); });
  }

  const JOURNEYS = { "solve-and-results": solveAndResults, report, exports: exportsPage, "reference-system": referenceSystem, "prepared-project": preparedProject };

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
