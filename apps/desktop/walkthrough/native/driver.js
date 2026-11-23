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
// With the plan's `attended` (the runner's --attended) it makes only the start, writes the checkpoint `start` and then
// takes no step and touches nothing until the launch ends (attendedStart): it stops the page's file fields and the
// agent's Start, file drops and file field changes (installAttendedGuards) and records, with times, what the launch
// stopped and the counts on screen whenever the page settles (observeAttended); the build itself logs each print it
// substitutes.
(() => {
  const ipc = window.__TAURI_INTERNALS__;
  // The scripted-run build's substitutions for a person's act (DEL-07-11 REQ-010). Each one a step uses is recorded
  // on that step, so the index can mark it: the file a chooser returns, a native menu command, a request to quit or
  // to close the main window, the print dialog.
  let substitutions = null;
  const SUBSTITUTED = {
    native_smoke_choose: (args) => ({ kind: "file chooser", what: `the next file chooser returns ${String(args.path).split("/").pop()}, named by the harness; no chooser is shown` }),
    native_smoke_menu: (args) => ({ kind: "native menu command", what: `${args.commandId} delivered through the menu's own dispatch, not by a click in the menu bar` }),
    native_smoke_request_close: (args) => ({ kind: "quit or window close request", what: args.kind === "window" ? "the main window asked to close by the window's own close request, not by a click on its close button" : "Quit asked by the menu item's own handling, not by a click in the menu bar or the keyboard shortcut" }),
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

  // ------------------------------------------------------------ the attended launch's guards and records

  // In an attended launch only, before the handover. Each act the launch stops, and each substitution the build
  // performs while the attendant works, is recorded with its time in the checkpoint `substitutions` (rewritten whole
  // on each, at most ATTENDED_LIMITS.substitutions of them); the runner marks each on the capture that follows it.
  const ATTENDED_LIMITS = { substitutions: 2000 };
  const STOPPED_FILE_PANEL = "The launch stopped this control from opening the system's file panel.";
  const STOPPED_FILE_DROP = "The launch stopped a file dropped on the window from reaching the page.";
  const STOPPED_FILE_CHANGE = "The launch stopped a file given to this field from reaching the page, and cleared the field.";
  const STOPPED_AGENT_START = "The launch stopped this control from starting the embedded agent: an attended launch has no scripted provider, and no other provider is used.";
  const attendedSubstitutions = [];
  const substituted = (item) => {
    if (attendedSubstitutions.length >= ATTENDED_LIMITS.substitutions) return;
    attendedSubstitutions.push({ at: Date.now(), ...item });
    call("native_smoke_checkpoint", { name: "substitutions", report: { nonce, substitutions: attendedSubstitutions }, storeSnapshot: false }).catch(() => {});
  };
  const accessibleName = (element) => String(element?.labels?.[0]?.textContent ?? element?.getAttribute?.("aria-label") ?? element?.textContent ?? "").replace(/\s+/g, " ").trim().slice(0, 200) || "an unnamed control";
  const fileInputOf = (target) => {
    if (typeof target?.closest !== "function") return null;
    if (target.matches('input[type="file"]')) return target;
    const control = target.closest("label")?.control;
    return control && typeof control.matches === "function" && control.matches('input[type="file"]') ? control : null;
  };
  // A page file field (Import hanger library, Proposal file) would open the system's file panel, which the strict
  // chooser does not govern: every activation of one is stopped. A click on the field or on its label (which is how
  // a keyboard press reaches it too) has its default prevented in the capture phase, before anything else sees it,
  // and showPicker on a file field does nothing. The embedded agent's Start agent is stopped the same way: only the
  // agent journeys' scripted provider (a loopback stand-in their own runner starts) may be used, and an attended launch
  // has none, so no provider, model, key, network or sign-in is reached from its pane. A file reaches the page by
  // two other ways, and each is stopped in the capture phase before the page's own handlers: a drag carrying files
  // (dragenter, dragover and drop on the window, with the drop's default prevented so the web view opens nothing), and
  // a change of a file field (its change and input events, the field cleared). Each stop is recorded. The guards are
  // installed as soon as the plan says the launch is attended, before the window is brought to the front and before
  // the start; nothing else on the page is changed.
  const agentStartOf = (target) => {
    if (typeof target?.closest !== "function") return null;
    const button = target.closest('section[aria-label="Embedded agent"] button');
    return button && String(button.textContent ?? "").trim() === "Start agent" ? button : null;
  };
  const guards = { installed: [], at: null, error: null };
  function installAttendedGuards() {
    document.addEventListener("click", (event) => {
      const input = fileInputOf(event.target);
      const start = input ? null : agentStartOf(event.target);
      if (!input && !start) return;
      event.preventDefault();
      event.stopImmediatePropagation();
      if (input) substituted({ kind: "file panel stopped by the launch", control: accessibleName(input), what: STOPPED_FILE_PANEL });
      else substituted({ kind: "agent start stopped by the launch", control: "Embedded agent: Start agent", what: STOPPED_AGENT_START });
    }, true);
    const carriesFiles = (event) => {
      const types = event.dataTransfer?.types;
      return Boolean(types && Array.from(types).includes("Files")) || (event.dataTransfer?.files?.length ?? 0) > 0;
    };
    for (const type of ["dragenter", "dragover", "drop"]) {
      window.addEventListener(type, (event) => {
        if (!carriesFiles(event)) return;
        event.preventDefault();
        event.stopImmediatePropagation();
        try { if (event.dataTransfer) event.dataTransfer.dropEffect = "none"; } catch { /* read-only in some events */ }
        if (type === "drop") substituted({ kind: "file drop stopped by the launch", control: accessibleName(fileInputOf(event.target) ?? event.target), what: STOPPED_FILE_DROP });
      }, true);
    }
    for (const type of ["change", "input"]) {
      document.addEventListener(type, (event) => {
        const input = typeof event.target?.matches === "function" && event.target.matches('input[type="file"]') ? event.target : null;
        if (!input) return;
        event.preventDefault();
        event.stopImmediatePropagation();
        try { input.value = ""; } catch { /* a field that cannot be cleared is still kept from the page */ }
        if (type === "change") substituted({ kind: "file field change stopped by the launch", control: accessibleName(input), what: STOPPED_FILE_CHANGE });
      }, true);
    }
    const Input = typeof HTMLInputElement === "function" ? HTMLInputElement : null;
    const showPicker = Input?.prototype?.showPicker;
    if (typeof showPicker === "function") {
      Input.prototype.showPicker = function attendedShowPicker() {
        if (this.type === "file") { substituted({ kind: "file panel stopped by the launch", control: accessibleName(this), what: STOPPED_FILE_PANEL }); return undefined; }
        return showPicker.call(this);
      };
    }
  }

  // The application's own report of the words and controls on screen while the attendant works: readScreenCounts, the
  // count a scripted step records, read whenever the page settles after a change and written with its time as the
  // checkpoint `observed-<n>`. It reads and changes nothing: no press, focus, scroll or resize, and no document
  // snapshot (takeSnapshot dispatches a resize to have the 3D view draw, which a record must not). A change is a
  // mutation anywhere in the document, a scroll or a resize of the window; a record follows once the page has been
  // quiet for quiet_ms, no sooner than min_gap_ms after the previous record and no later than max_wait_ms after the
  // first change it covers; one is taken at the handover. At most `records` records, each at most `record_bytes`
  // (the controls list is cut to fit and says so). Each record says when the first change it covers came
  // (`covers_from`), and the first change after a record is written at once as the checkpoint `pending` (the record it
  // follows and its time), so that a capture taken after a change no record yet covers is known not to match the last
  // record. The report prints the build substitutes for the system print dialog are not taken here: the build appends
  // each to the run's print log at the instant it is recorded (native_smoke.rs record_report_print).
  const OBSERVED = { quiet_ms: 800, min_gap_ms: 1500, max_wait_ms: 5000, records: 1200, record_bytes: 256 * 1024 };
  function observedRecord(n, coversFrom) {
    let counts;
    try { counts = readScreenCounts(); } catch (error) { counts = { counts_error: String(error?.message ?? error).slice(0, 400) }; }
    const report = { nonce, n, at: Date.now(), covers_from: coversFrom, page_visibility: document.visibilityState, ...counts };
    if (n === OBSERVED.records) report.last = `the bound of ${OBSERVED.records} records was reached: no later record is made`;
    if (Array.isArray(report.controls) && JSON.stringify(report).length > OBSERVED.record_bytes) {
      const all = report.controls.length;
      let kept = all;
      while (kept > 0 && JSON.stringify({ ...report, controls: report.controls.slice(0, kept) }).length > OBSERVED.record_bytes - 200) kept = Math.floor(kept * 0.9);
      report.controls = report.controls.slice(0, kept);
      report.controls_cut = `${kept} of ${all} controls kept, to keep the record within ${OBSERVED.record_bytes} bytes`;
    }
    return report;
  }
  function observeAttended() {
    let n = 0;
    let last = 0;
    let first = null;
    let timer = null;
    const record = () => {
      timer = null;
      const coversFrom = first;
      first = null;
      if (n >= OBSERVED.records) return;
      n += 1;
      const report = observedRecord(n, coversFrom);
      last = report.at;
      call("native_smoke_checkpoint", { name: `observed-${n}`, report, storeSnapshot: false }).catch(() => {});
    };
    const changed = () => {
      const now = Date.now();
      if (first === null) {
        first = now;
        // The page has changed since record n: until a record covers it, the last record may not match the page.
        call("native_smoke_checkpoint", { name: "pending", report: { nonce, after: n, at: now }, storeSnapshot: false }).catch(() => {});
      }
      if (n >= OBSERVED.records) return;
      if (timer !== null) clearTimeout(timer);
      const due = Math.max(Math.min(now + OBSERVED.quiet_ms, first + OBSERVED.max_wait_ms), last + OBSERVED.min_gap_ms);
      timer = setTimeout(record, Math.max(0, due - now));
    };
    record();
    new MutationObserver(changed).observe(document.documentElement, { subtree: true, childList: true, characterData: true, attributes: true });
    document.addEventListener("scroll", changed, { capture: true, passive: true });
    window.addEventListener("resize", changed, { passive: true });
  }

  // ------------------------------------------------------------ the attended launch's start (run.mjs --attended)

  // The start and nothing after it. With the plan's `open` (a journey's `place` and `open`), the prepared project is
  // opened by the File menu exactly as that journey opens it (openPrepared, its own action, no step recorded), and any
  // chooser answer left queued is cleared, so the attendant's own chooser meets the strict chooser. The checkpoint
  // `start` records what was done, then the driver takes no step and touches nothing, and only records (observeAttended): the promise never settles, so
  // native_smoke_finish is never called. A failed start is recorded, and the app is then ended by native_smoke_finish.
  async function attendedStart(journey, plan) {
    const record = { nonce, attended: true, start: plan.start ?? null, at: Date.now(), window_shown: windowShown, page_visibility: document.visibilityState, opened: null, substitutions: [], unused_answers: null };
    try {
      if (plan.open) {
        substitutions = [];
        try {
          const opening = { step: async (step, note, action) => { record.opened = { step, note, outcome: (await action()) ?? null }; } };
          await openPrepared(opening, plan, `the start of ${plan.start}`);
        } finally {
          record.substitutions = substitutions ?? [];
          substitutions = null;
          record.unused_answers = await call("native_smoke_clear_choices");
        }
      }
      record.ok = true;
    } catch (error) {
      record.ok = false;
      record.error = String(error?.message ?? error).split("\n")[0].slice(0, 400);
    }
    if (record.ok) {
      if (guards.error) { record.ok = false; record.error = `the launch's guards could not be installed: ${guards.error}`; }
      else { record.guards = guards.installed; record.guards_installed_at = guards.at; }
    }
    record.ready_at = Date.now();
    await call("native_smoke_checkpoint", { name: "start", report: record, storeSnapshot: false });
    if (!record.ok) throw new Error(`the attended start failed: ${record.error}`);
    observeAttended();
    await log("attended: the start is made; the driver takes no further step and only records");
    return new Promise(() => {});
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
    // Open in a window (report_view.rs): pressed as a person does; the build records what the view window's protocol
    // served it (native_smoke_take_report_views), not what WebKit drew. The window is left open: the launch ends with it.
    await journey.step("view-window", "Open in a window: the rendered report in a native window of its own", async () => {
      await press("rendered-report-open-window");
      const status = await waitFor("the window's outcome", () => { const now = text("rendered-report-view-status"); return now && !/^Opening/.test(now) ? now : null; }, T.step);
      const views = [];
      const started = Date.now();
      while (!views.some((view) => view.document_served) && Date.now() - started < T.ui) {
        views.push(...(await call("native_smoke_take_report_views")));
        if (!views.some((view) => view.document_served)) await sleep(250);
      }
      const served = views.map((view) => `${view.window_label} ${view.kind} wrapper ${view.wrapper_served ? "served" : "not served"}, document ${view.document_served ? `served (${view.document_byte_count} B, SHA-256 ${view.document_sha256_hex})` : "not served"}`);
      return `${status}; ${served.length ? served.join("; ") : "no view window was served"}`;
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

  // A copy of the local project store, written by the app through a read-only connection (the checkpoint's store
  // snapshot) as `<report>.store-<name>.store.sqlite3`; the runner reads its stored runs after the launch
  // (run.mjs storedRunEvidence). Nothing in the store is written.
  const storeCopy = (name) => call("native_smoke_checkpoint", { name: `store-${name}`, report: { nonce, store_copy: name, at: Date.now() }, storeSnapshot: true });
  const projectMessage = () => text("local-project-message") ?? "";

  // F. A saved project whose stored run's record binds its result rules at a revision this version does not read,
  // opened from the placed store: what the person is told, the Results stage, and a save in place.
  async function refusedStoredRun(journey, plan) {
    await openPrepared(journey, plan, "a saved project whose stored run this version does not read");
    await journey.step("project-page", "The Project page after the open, with what it says of the stored run", async () => {
      await storeCopy("after-open");
      await closeOverlays();
      await showSection("project");
      return projectMessage() || "the Project page shows no message";
    });
    await journey.step("results-stage", "The Results stage: what the window says of the run", async () => {
      await closeOverlays();
      const stage = await waitFor("the Results stage in the rail", () => { const item = byTestId("rail-stage-results"); return visible(item) ? item : null; }, T.ui);
      if (stage.disabled || stage.getAttribute("aria-disabled") === "true") {
        // Not pressed: a disabled stage is read where a person reads it, its reason shown on focus.
        acting(stage, "rail-stage-results").focus();
        await sleep(300);
        const caption = text("rail-caption-results");
        return `the Results stage is disabled: "${disabledReason(stage)}"${caption ? `; its caption: "${caption}"` : ""}`;
      }
      await openStage("results");
      const section = await waitFor("the Results section", () => { const item = byTestId("workspace-section-results"); return visible(item) ? item : null; }, T.ui);
      return section.textContent.replace(/\s+/g, " ").trim().slice(0, 400);
    });
    await journey.step("save", "File > Save Local Project: what the person is told", async () => {
      await closeOverlays();
      const before = projectMessage();
      await menu("file.save-local");
      const said = await waitFor("the save's outcome", () => { const now = projectMessage(); return now && now !== before ? now : null; }, T.save);
      await storeCopy("after-save");
      // The message is the project strip's; it is on screen only where the window shows that strip.
      return `${said}${visible(byTestId("local-project-message")) ? "" : " (this message is not on screen here: the window shows it in the Project page's strip)"}`;
    });
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

  const JOURNEYS = { "solve-and-results": solveAndResults, report, exports: exportsPage, "reference-system": referenceSystem, "prepared-project": preparedProject, "refused-stored-run": refusedStoredRun };

  (async () => {
    const result = { nonce: null, ok: false, journey: null, user_agent: navigator.userAgent };
    let journey = null;
    try {
      const plan = await call("native_smoke_plan");
      nonce = plan.nonce;
      result.nonce = plan.nonce;
      result.journey = plan.journey;
      if (plan.timeouts) T = { ...T, ...plan.timeouts };
      const run = plan.attended === true ? attendedStart : JOURNEYS[plan.journey];
      if (!run) throw new Error(`unknown journey ${plan.journey}`);
      // An attended launch's guards, first of all: before the window is brought to the front and before the start.
      if (plan.attended === true) {
        try { installAttendedGuards(); guards.installed = ["file panels", "file drops", "file field changes", "agent start"]; guards.at = Date.now(); } catch (error) { guards.error = String(error?.message ?? error).slice(0, 300); }
      }
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
