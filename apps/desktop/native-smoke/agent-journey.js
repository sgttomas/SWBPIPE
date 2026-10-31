// Native smoke driver for the embedded agent's W9-A journey (RT-E1 D4; DEL-16-06
// `Work/agent-journey-scenario.md`). `tools/agent_journey_fixture.py` launches the
// `native-smoke` build of the app with this script on stdin, which the build
// evaluates in the real webview once the page has loaded (`src-tauri/src/native_smoke.rs`).
//
// It works the app as a person would: New Blank Project through the native menu
// dispatch, the provider entered in the Agent panel's own form, Start, two typed
// messages, and the batch applied by clicking Batch review's own Validate and
// Apply buttons. That click is a qualification stand-in for the person, never the
// person's acceptance. No command, shortcut or IPC call applies anything. It reads
// only what the window shows and records it; the controller judges.
(() => {
  const ipc = window.__TAURI_INTERNALS__;
  const call = (command, args) => ipc.invoke(command, args ?? {});
  const log = (line) => call("native_smoke_log", { line: String(line) }).catch(() => {});
  const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
  const settle = () => new Promise((resolve) => setTimeout(resolve, 0));
  const byTestId = (id) => document.querySelector(`[data-testid="${CSS.escape(id)}"]`);
  const text = (id) => byTestId(id)?.textContent?.trim() ?? null;
  const visible = (element) => Boolean(element && element.offsetParent !== null);
  const panel = () => document.querySelector('section[aria-label="Embedded agent"]');
  const byLabel = (label) => panel()?.querySelector(`[aria-label="${CSS.escape(label)}"]`) ?? null;
  const button = (root, label) => [...(root?.querySelectorAll("button") ?? [])].find((item) => item.textContent.trim() === label) ?? null;
  const bounded = (value, limit = 4000) => (value === null || value === undefined ? null : String(value).slice(0, limit));

  // A probe that throws ends the wait at once with its own error.
  async function waitFor(label, probe, timeoutMs) {
    const started = Date.now();
    for (;;) {
      const value = probe();
      if (value) return value;
      if (Date.now() - started > timeoutMs) throw new Error(`timed out after ${timeoutMs} ms waiting for ${label}`);
      await sleep(100);
    }
  }
  function press(element, label) {
    if (!element) throw new Error(`no control ${label}`);
    if (element.disabled) throw new Error(`${label} is disabled`);
    element.click();
  }
  function enter(element, label, value) {
    if (!element) throw new Error(`no field ${label}`);
    const prototype = element instanceof HTMLTextAreaElement ? HTMLTextAreaElement.prototype : element instanceof HTMLSelectElement ? HTMLSelectElement.prototype : HTMLInputElement.prototype;
    Object.getOwnPropertyDescriptor(prototype, "value").set.call(element, value);
    element.dispatchEvent(new Event(element instanceof HTMLSelectElement ? "change" : "input", { bubbles: true }));
  }
  const sectionShown = (section) => visible(byTestId(`workspace-section-${section}`));
  async function showSection(section) {
    await settle();
    if (sectionShown(section)) return;
    await call("native_smoke_menu", { commandId: `view.section.${section}` });
    await waitFor(`section ${section}`, () => sectionShown(section), 20000);
  }
  async function showOperationTab(tab) {
    await showSection("operations");
    const element = byTestId(`operation-tab-${tab}`);
    if (element?.getAttribute("aria-pressed") !== "true") press(element, `operation tab ${tab}`);
    await waitFor(`operation tab ${tab}`, () => byTestId(`operation-tab-${tab}`)?.getAttribute("aria-pressed") === "true", 10000);
  }
  const standing = () => text("embedded-agent-standing") ?? "";
  const statusLines = () => [...(panel()?.querySelectorAll('p[role="status"]') ?? [])].map((item) => item.textContent.trim());
  const conversation = () => [...(panel()?.querySelectorAll('[aria-label="Draft agent conversation"] p') ?? [])].map((item) => bounded(item.textContent, 8000));
  const projectRowIds = () => [...document.querySelectorAll('[data-testid^="tree-row-project-"]')]
    .map((row) => decodeURIComponent(row.getAttribute("data-testid").slice("tree-row-project-".length)));
  const liveBatches = () => [...document.querySelectorAll('article[data-testid^="batch-"]')].filter((item) => item.getAttribute("data-retired") !== "true");

  async function newBlank(report, timeouts) {
    // The app opens its bundled demo first; New Blank Project waits for that load.
    await waitFor("app shell and first project", () => byTestId("desktop-preview-shell") && projectRowIds().length > 0, timeouts.start);
    const previous = projectRowIds()[0] ?? null;
    await call("native_smoke_menu", { commandId: "file.new-blank" });
    await waitFor("units dialog", () => byTestId("project-units-dialog"), timeouts.ui);
    report.blank = { unit_preset: byTestId("project-units-preset")?.value ?? null };
    press(byTestId("project-units-create"), "project-units-create");
    report.blank.project_id = await waitFor("blank project", () => {
      const message = text("local-project-message") ?? "";
      if (message.startsWith("Blank create failed")) throw new Error(message);
      const ids = [...new Set(projectRowIds())];
      return message.startsWith("Created blank local model document") && ids.length === 1 && ids[0] !== previous ? ids[0] : null;
    }, timeouts.ui);
  }

  async function configureAndStart(plan, report, timeouts) {
    await showOperationTab("agent");
    const availability = await waitFor("agent availability", () => {
      const status = panel()?.querySelector('p[role="status"]')?.textContent ?? "";
      if (byLabel("Agent endpoint") || button(panel(), "Start agent")) return "available";
      return status && !status.startsWith("Checking availability") ? status : null;
    }, timeouts.ui);
    report.availability = availability;
    if (availability !== "available") throw new Error(`agent unavailable: ${availability}`);
    const details = panel().querySelector("details");
    if (details && !details.open) { details.open = true; details.dispatchEvent(new Event("toggle")); await settle(); }
    enter(byLabel("Agent endpoint"), "endpoint", plan.provider.endpoint);
    enter(byLabel("Agent model"), "model", plan.provider.model);
    enter(byLabel("Agent context window"), "context window", String(plan.provider.contextWindow));
    enter(byLabel("Agent authentication"), "authentication", "no_key");
    await settle();
    press(button(panel(), "Save provider"), "Save provider");
    report.provider_saved = await waitFor("provider saved", () => {
      const alert = panel()?.querySelector('p[role="alert"]')?.textContent;
      if (alert) throw new Error(`provider: ${alert}`);
      return [...panel().querySelectorAll('p[role="status"]')].map((item) => item.textContent).find((line) => line.startsWith("Provider saved")) ?? null;
    }, timeouts.ui);
    // Start refuses until the new blank's fingerprint is ready; a person waits and presses again.
    report.start_attempts = [];
    for (let attempt = 1; ; attempt++) {
      await waitFor("Start enabled", () => button(panel(), "Start agent") && !button(panel(), "Start agent").disabled, timeouts.ui);
      report.started_at = new Date().toISOString();
      press(button(panel(), "Start agent"), "Start agent");
      await sleep(300); // lets the press clear the previous attempt's status line
      const outcome = await waitFor("agent ready", () => {
        const alert = panel()?.querySelector('p[role="alert"]')?.textContent;
        if (alert) throw new Error(`start: ${alert}`);
        if (standing() === "Ready.") return { ready: standing() };
        // Only the fingerprint refusal is retried: it comes before anything is registered or
        // spawned. Any other failure could follow a spawn, and a retry could start a second supplier.
        const failed = statusLines().find((line) => line.startsWith("The request did not complete"));
        if (failed) throw new Error(`start: ${failed}`);
        const refusal = statusLines().find((line) => line.startsWith("Open a model and wait for its fingerprint"));
        return refusal ? { refusal } : null;
      }, timeouts.start);
      report.start_attempts.push(outcome.ready ?? outcome.refusal);
      if (outcome.ready) { report.ready = outcome.ready; break; }
      if (attempt >= 10) throw new Error(`start: ${outcome.refusal}`);
      await sleep(1000);
    }
  }

  // One turn: type, Send, then wait for the turn to start and to end. Only the bare
  // standing "Ready." is a completed turn; "Ready. The last turn failed." and an
  // interrupted turn are failures, and a completed turn must have produced an agent message.
  async function turn(label, message, timeouts) {
    await showOperationTab("agent");
    const before = conversation().length;
    enter(byLabel("Message to agent"), "message", message);
    await waitFor(`${label}: Send enabled`, () => button(panel(), "Send") && !button(panel(), "Send").disabled, timeouts.ui);
    press(button(panel(), "Send"), "Send");
    await waitFor(`${label}: turn started`, () => standing().startsWith("Working") || conversation().length > before + 1, timeouts.ui);
    const ended = await waitFor(`${label}: turn ended`, () => {
      const alert = panel()?.querySelector('p[role="alert"]')?.textContent;
      if (alert) throw new Error(`${label}: ${alert}`);
      const now = standing();
      if (/failed|interrupted/i.test(now)) throw new Error(`${label}: ${now}`);
      return now === "Ready." ? now : null;
    }, timeouts.turn);
    const messages = conversation().slice(before);
    if (!messages.some((line) => line?.startsWith("Agent:"))) throw new Error(`${label}: the turn ended without an agent message`);
    return { standing: ended, messages };
  }

  // The batch as Batch review shows it submitted ("Submitted changes and source metadata"):
  // each operation's id, kind and target, in order.
  function submittedOperations(entry) {
    const shown = [...entry.querySelectorAll("details")].find((item) => item.querySelector("summary")?.textContent.trim().startsWith("Submitted changes"));
    try {
      const batch = JSON.parse(shown?.querySelector("pre")?.textContent ?? "null");
      return (batch?.operations ?? []).slice(0, 64).map((operation) => [
        bounded(operation.operation_id, 160),
        bounded(`${operation.operation_kind}/${operation.target?.object_type}/${operation.change?.change_kind}/${operation.change?.field_path}`, 200),
        bounded(operation.target?.ref, 160)
      ]);
    } catch { return null; }
  }

  // The stand-in click: Batch review's own Validate (when its preview gate asks) and Apply.
  async function applyAgentBatch(report, timeouts) {
    const entry = await waitFor("agent batch in Batch review", () => liveBatches()[0] ?? null, timeouts.ui);
    const key = entry.getAttribute("data-testid").slice("batch-".length);
    report.batch = {
      key,
      batch_id: bounded(entry.querySelector("h4")?.textContent),
      batches_shown: liveBatches().length,
      context: bounded(entry.querySelector('[aria-label="Agent proposal context"]')?.textContent),
      summary: bounded([...entry.querySelectorAll("p")].map((item) => item.textContent).find((line) => /proposed changes/.test(line))),
      submitted: submittedOperations(entry)
    };
    press(button(panel(), "Open Batch review"), "Open Batch review");
    await waitFor("Batch review shown", () => byTestId("operation-tab-review")?.getAttribute("aria-pressed") === "true", timeouts.ui);
    if (byTestId(`preview-required-${key}`)) {
      press(byTestId(`validate-batch-${key}`), "Validate batch");
      await waitFor("preview satisfied", () => !byTestId(`preview-required-${key}`) && !byTestId(`apply-batch-${key}`)?.disabled, timeouts.ui);
    }
    const message = text("batch-review-message");
    press(byTestId(`apply-batch-${key}`), "Apply entire batch");
    report.batch.applied_by = "native-smoke driver click on Batch review Apply (qualification stand-in, not the person's acceptance)";
    report.batch.apply_message = await waitFor("batch applied", () => {
      const now = text("batch-review-message");
      if (now && now !== message && /refus|reject|fail|blocked/i.test(now)) throw new Error(`apply: ${now}`);
      return byTestId("batch-receipt") ? text("batch-receipt") : null;
    }, timeouts.ui);
  }

  // The model tree's rows and the Results section's rows for the probe entity, as shown.
  async function observeModelAndResults(plan, report, timeouts) {
    // Every row of the listed types the tree shows, not only the expected ones.
    report.tree_rows = [...document.querySelectorAll('[data-testid^="tree-row-"]')].map((row) => {
      const [type, ...id] = row.getAttribute("data-testid").slice("tree-row-".length).split("-");
      return [decodeURIComponent(type), decodeURIComponent(id.join("-"))];
    }).filter(([type]) => plan.treeTypes.includes(type)).slice(0, 200);
    await showSection("results");
    report.results = { standing: await waitFor("result standing", () => text("numerical-result-standing"), timeouts.ui) };
    const proof = byTestId("status-pill-solve-proof")?.querySelector("code")?.textContent ?? null;
    report.results.solve_proof = bounded(proof);
    report.results.model_sha256 = proof?.match(/(?:^|; )model_sha256=([^;]*)/)?.[1] ?? null;
    enter(byTestId("result-filter-input"), "result filter", plan.resultProbe);
    await waitFor("filtered result rows", () => document.querySelector('[data-testid^="result-row-dual-"]'), timeouts.ui);
    report.results.rows = [...document.querySelectorAll('[data-testid^="result-row-dual-"]')].slice(0, 200)
      .map((cell) => ({ id: cell.getAttribute("data-testid").slice("result-row-dual-".length), text: bounded(cell.textContent, 200) }));
    enter(byTestId("result-filter-input"), "result filter", "");
  }

  async function stop(report, timeouts) {
    await showOperationTab("agent");
    const control = button(panel(), "Stop agent");
    if (control && !control.disabled) control.click();
    report.stopped = await waitFor("agent closed", () => (standing().startsWith("Closed") ? standing() : null), timeouts.ui);
  }

  (async () => {
    const report = { driver: "agent-journey", steps: [] };
    const step = async (name, fn) => { await log(`agent-journey: ${name}`); await fn(); report.steps.push(name); };
    try {
      const plan = await call("native_smoke_plan");
      const timeouts = plan.timeoutsMs;
      await step("new blank", () => newBlank(report, timeouts));
      await step("provider and start", () => configureAndStart(plan, report, timeouts));
      await step("turn 1", async () => { report.turn1 = await turn("turn 1", plan.turns[0], timeouts); });
      await step("apply", () => applyAgentBatch(report, timeouts));
      await step("turn 2", async () => { report.turn2 = await turn("turn 2", plan.turns[1], timeouts); });
      await step("observe", () => observeModelAndResults(plan, report, timeouts));
      await step("stop", () => stop(report, timeouts));
    } catch (error) {
      report.error = bounded(error?.message ?? error);
      report.panel_status = statusLines().map((line) => bounded(line, 400));
      try { await stop(report, { ui: 20000 }); } catch (stopError) { report.stop_error = bounded(stopError?.message ?? stopError); }
    }
    report.finished_at = new Date().toISOString();
    await call("native_smoke_finish", { report });
  })();
})();
