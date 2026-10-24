// Native smoke driver for the desktop walking skeleton. `run.mjs` launches the
// `native-smoke` build of the app, which evaluates this script in the real
// macOS webview once the page has loaded (`src-tauri/src/native_smoke.rs`).
//
// It works the app as a person would: native menu commands go through the same
// dispatch a menu click uses, buttons are clicked, and every observation is read
// from what the window shows. The only substitution is the open chooser: the
// driver names the file it returns. It records what it saw; `run.mjs` judges.
(() => {
  const ipc = window.__TAURI_INTERNALS__;
  const call = (command, args) => ipc.invoke(command, args ?? {});
  const log = (line) => call("native_smoke_log", { line: String(line) }).catch(() => {});
  const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
  const byTestId = (id) => document.querySelector(`[data-testid="${CSS.escape(id)}"]`);
  const text = (id) => byTestId(id)?.textContent?.trim() ?? null;

  async function waitFor(label, probe, timeoutMs = 120000) {
    const started = Date.now();
    for (;;) {
      let value;
      try { value = probe(); } catch { value = null; }
      if (value) return value;
      if (Date.now() - started > timeoutMs) throw new Error(`timed out after ${timeoutMs} ms waiting for ${label}`);
      await sleep(100);
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
    Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value").set.call(element, value);
    element.dispatchEvent(new Event("input", { bubbles: true }));
  }
  const treeRow = (projectId) => byTestId(`tree-row-project-${encodeURIComponent(projectId)}`);
  const message = () => text("local-project-message") ?? "";
  async function showSection(section) {
    await log(`show section ${section}`);
    await menu(`view.section.${section}`);
    await waitFor(`section ${section}`, () => byTestId(`workspace-section-${section}`)?.offsetParent !== null);
  }

  // File > Open Model Document…, answering the chooser with `path`.
  async function openModelDocument(path) {
    const name = path.split("/").pop();
    const before = message();
    await call("native_smoke_choose", { path });
    await menu("file.open-model");
    const shown = await waitFor(`open ${name}`, () => {
      const now = message();
      return now !== before && (now.includes(`Opened model document ${name} `) || now.includes(`refused (${name})`)) ? now : null;
    });
    await log(`open ${name}: ${shown}`);
    return shown;
  }

  async function observe(label, projectId, probe) {
    await showSection("results");
    const standing = await waitFor("result standing", () => text("numerical-result-standing"));
    let probeEntered = null;
    if (probe) {
      fill("result-filter-input", probe);
      probeEntered = await waitFor(`${probe} row`, () => text(`result-row-dual-${probe}`));
    }
    const filterSummary = text("result-filter-summary");
    const proof = byTestId("status-pill-solve-proof")?.querySelector("code")?.textContent ?? null;
    const field = (name) => proof?.match(new RegExp(`(?:^|; )${name}=([^;]*)`))?.[1] ?? null;
    const dom = document.body.textContent ?? "";
    const observation = {
      label,
      project_id: projectId,
      solver_mode_line: text("solve-job-solver-mode"),
      solve_job_summary: text("solve-job-summary"),
      standing,
      result_filter_summary: filterSummary,
      probe,
      probe_entered: probeEntered,
      solve_proof: proof,
      proof_rows: field("rows") === null ? null : Number(field("rows")),
      proof_model_sha256: field("model_sha256"),
      proof_project: field("project"),
      retained_precision_codes: [...new Set(dom.match(/RETAINED_PRECISION_[A-Z_]+/g) ?? [])],
      retained_precision_elements: document.querySelectorAll('[data-testid*="retained-precision"]').length
    };
    if (probe) fill("result-filter-input", "");
    await log(`${label}: ${JSON.stringify(observation)}`);
    return observation;
  }

  async function solve(mode, label, projectId, probe) {
    await showSection("solve");
    click(mode === "dense_scrutiny" ? "solver-mode-dense" : "solver-mode-sparse");
    await waitFor(`${mode} selected`, () => byTestId(mode === "dense_scrutiny" ? "solver-mode-dense" : "solver-mode-sparse")?.getAttribute("aria-pressed") === "true");
    const priorProof = byTestId("status-pill-solve-proof")?.querySelector("code")?.textContent ?? null;
    await log(`${label}: run`);
    await menu("analyze.run");
    await waitFor(`${label} solve completed`, () => {
      const summary = text("solve-job-summary") ?? "";
      if (/state=(failed|cancelled)/.test(summary)) throw new Error(`${label}: ${summary}; ${text("solve-job-error") ?? ""}`);
      const proof = byTestId("status-pill-solve-proof")?.querySelector("code")?.textContent ?? null;
      return summary.includes("state=completed") && proof && proof !== priorProof && proof.includes(`project=${projectId}`);
    }, 300000);
    return observe(label, projectId, probe);
  }

  async function saveAsProject(projectId) {
    const before = message();
    await menu("file.new-local");
    const shown = await waitFor(`save ${projectId}`, () => {
      const now = message();
      if (now !== before && now.startsWith("Create failed")) throw new Error(now);
      return now !== before && now.includes("Created local SQLite project snapshot") ? now : null;
    });
    await log(`saved ${projectId}: ${shown}`);
    return shown;
  }

  // A save onto a stored project id asks first (HELP_HUMAN ruling 2026-10-10):
  // record the prompt as shown, then answer it with `choice`.
  async function saveThroughPrompt(command, choice, label) {
    const before = message();
    await menu(command);
    await waitFor(`${label}: replace prompt`, () => byTestId("replace-project-prompt"));
    const prompt = {
      label,
      command,
      choice,
      detail: text("replace-project-detail"),
      cancel_focused: document.activeElement === byTestId("replace-project-cancel")
    };
    // Every status line shown while the answer is processed, for a failure report.
    const seen = [];
    const observer = new MutationObserver(() => { const now = message(); if (seen.at(-1) !== now) seen.push(now); });
    observer.observe(document.body, { subtree: true, childList: true, characterData: true });
    click(`replace-project-${choice}`);
    await waitFor(`${label}: prompt closed`, () => !byTestId("replace-project-prompt"));
    const wanted = choice === "cancel" ? "Not saved: the stored project"
      : command === "file.new-local" ? "Created local SQLite project snapshot" : "Saved current project model snapshot";
    prompt.message = await waitFor(`${label}: ${choice}`, () => {
      const now = message();
      if (/^(Create|Save) failed/.test(now)) throw new Error(now);
      return now !== before && (choice === "cancel" ? now.startsWith(wanted) : now.includes(wanted)) ? now : null;
    }).catch((error) => {
      throw new Error(`${error.message}; status lines seen: ${JSON.stringify(seen)}`);
    }).finally(() => observer.disconnect());
    await log(`${label}: ${JSON.stringify(prompt)}`);
    return prompt;
  }

  // Save Local Project of the session's own stored project: a save in place,
  // with no prompt (HELP_HUMAN, 2026-10-10).
  async function saveInPlace(label) {
    const before = message();
    let prompted = false;
    await menu("file.save-local");
    const shown = await waitFor(`${label}: saved in place`, () => {
      if (byTestId("replace-project-prompt")) { prompted = true; click("replace-project-cancel"); }
      const now = message();
      if (/^(Create|Save) failed/.test(now)) throw new Error(now);
      return now !== before && (now.includes("Saved current project model snapshot") || now.startsWith("Not saved")) ? now : null;
    });
    const record = { label, command: "file.save-local", prompted, message: shown };
    await log(`${label}: ${JSON.stringify(record)}`);
    return record;
  }

  async function replaceFlows(doc, report) {
    // A second New Local Project of the same document asks; Cancel keeps the stored project.
    report.prompts.push(await saveThroughPrompt("file.new-local", "cancel", `${doc.case}/second-save`));
    // Save as a new project: same document, new project id, so a new model hash.
    const prompt = await saveThroughPrompt("file.new-local", "save-as-new", `${doc.case}/save-as-new`);
    const prefix = `tree-row-project-${encodeURIComponent(`${doc.project_id}:copy-`)}`;
    const row = await waitFor("renamed project in the model tree", () => document.querySelector(`[data-testid^="${CSS.escape(prefix)}"]`));
    prompt.new_project_id = decodeURIComponent(row.getAttribute("data-testid").slice("tree-row-project-".length));
    prompt.original_kept_in_tree = Boolean(treeRow(doc.project_id));
    report.prompts.push(prompt);
    report.observations.push({ case: `${doc.case}/copy`, mode: doc.modes[0], ...(await solve(doc.modes[0], `${doc.case}/copy/${doc.modes[0]}`, prompt.new_project_id, doc.probe)) });
    // The session is now the copy: Save Local Project saves it in place.
    report.prompts.push(await saveInPlace(`${doc.case}/copy/own-save`));
    // Another writer saves the copy (smoke-only, through the product's own
    // persistence functions): this session's base is no longer the stored
    // revision, so its next save asks; Cancel keeps the other writer's copy.
    await call("native_smoke_external_save", { projectId: prompt.new_project_id });
    report.prompts.push(await saveThroughPrompt("file.save-local", "cancel", `${doc.case}/copy/stale-base`));
    // Replace: the reopened file saved over its stored project, with confirmation.
    await openModelDocument(doc.path);
    await waitFor(`${doc.case} reopened from file`, () => treeRow(doc.project_id));
    report.prompts.push(await saveThroughPrompt("file.new-local", "replace", `${doc.case}/replace`));
  }

  async function exportResult(label) {
    await showSection("exports");
    const available = await waitFor("result export panel", () => byTestId("result-export-link") ? "available" : byTestId("result-export-empty") ? "empty" : null, 30000);
    if (available === "empty") {
      // The packet is assembled asynchronously after a solve: the demo's took
      // 8.5 s on a loaded Mac, so a fixed 2 s sample missed it. A result the
      // product does not offer (the milestone) still reads as not offered.
      const offered = await waitFor("result export packet", () => byTestId("result-export-link"), 30000).catch(() => null);
      if (!offered) return { label, available: false, reason: text("result-export-empty") };
    }
    const intentId = "result-export-link-local-private-intent";
    if (byTestId(intentId) && !byTestId(intentId).checked) {
      click(intentId);
      await waitFor("local-private intent recorded", () => byTestId(intentId)?.checked, 30000);
      await sleep(250);
    }
    await waitFor("export enabled", () => byTestId("result-export-link") && !byTestId("result-export-link").disabled, 30000);
    const summary = text("result-export-summary");
    click("result-export-link");
    const status = await waitFor("export saved", () => {
      const now = text("result-export-link-native-save-status") ?? "";
      if (now.startsWith("Save failed")) throw new Error(now);
      return now.startsWith("Saved ") ? now : null;
    }, 60000).catch((error) => {
      throw new Error(`${error.message}; status="${text("result-export-link-native-save-status")}"; intent=${byTestId(intentId)?.checked}`);
    });
    await log(`${label} export: ${status}`);
    return { label, available: true, summary, status };
  }

  // The open's status line repeats from one project open to the next, so the
  // model tree is the witness: the reopened project replaces `previousId`.
  async function openProjectById(projectId, previousId) {
    // List once. Listing is a project operation, and a click that lands while
    // a list is in flight is dropped, so a project already listed is opened
    // from the existing list.
    if (!byTestId(`project-index-open-${projectId}`)) await menu("file.list-local");
    await waitFor(`listed ${projectId}`, () => byTestId(`project-index-open-${projectId}`) && !byTestId(`project-index-open-${projectId}`).disabled);
    click(`project-index-open-${projectId}`);
    const shown = await waitFor(`reopen ${projectId}`, () => {
      const now = message();
      if (now.startsWith("Open failed")) throw new Error(now);
      return now.includes("Opened local SQLite project snapshot") && treeRow(projectId) && !treeRow(previousId) ? now : null;
    }).catch((error) => {
      throw new Error(`${error.message}; message="${message()}"; shown=${Boolean(treeRow(projectId))}; previous=${Boolean(treeRow(previousId))}`);
    });
    await log(`reopened ${projectId}: ${shown}`);
    return shown;
  }

  // --- W-4: a model authored from blank in the GUI, and the output gates ----
  // Controlled inputs other than <input> (selects, textareas) take the value
  // through their own prototype's setter.
  function setValue(element, value) {
    const proto = element instanceof HTMLTextAreaElement ? HTMLTextAreaElement.prototype
      : element instanceof HTMLSelectElement ? HTMLSelectElement.prototype : HTMLInputElement.prototype;
    Object.getOwnPropertyDescriptor(proto, "value").set.call(element, value);
    element.dispatchEvent(new Event("input", { bubbles: true }));
    element.dispatchEvent(new Event("change", { bubbles: true }));
  }
  function fillAny(id, value) {
    const element = byTestId(id);
    if (!element) throw new Error(`no element ${id}`);
    setValue(element, value);
  }
  function checkBox(id) {
    const element = byTestId(id);
    if (!element) throw new Error(`no element ${id}`);
    if (!element.checked) element.click();
  }
  function chooseRadio(name) {
    const radio = [...document.querySelectorAll('input[type="radio"]')]
      .find((item) => item.labels?.[0]?.textContent?.trim() === name || item.getAttribute("aria-label") === name);
    if (!radio) throw new Error(`no radio ${name}`);
    if (!radio.checked) radio.click();
  }
  // The searchable target picker: type the id, then click the option that ends with it.
  async function pickTarget(testId, value) {
    const picker = byTestId(testId);
    if (!picker) throw new Error(`no picker ${testId}`);
    const search = picker.querySelector('[role="combobox"]');
    if (!search) throw new Error(`no combobox in ${testId}`);
    search.focus();
    setValue(search, value);
    const option = await waitFor(`${testId} option ${value}`, () => [...picker.querySelectorAll('[role="option"]')].find((item) => item.textContent.trim().endsWith(value)), 10000);
    option.dispatchEvent(new MouseEvent("mousedown", { bubbles: true }));
    option.click();
    await waitFor(`${testId} = ${value}`, () => picker.querySelector(".virtual-target-picker-current")?.textContent?.includes(value), 10000);
  }
  const visible = (element) => Boolean(element && element.offsetParent !== null);
  async function showOperations() {
    if (visible(byTestId("workspace-review"))) click("workspace-review");
    else await menu("view.section.operations");
    await waitFor("operations section", () => visible(byTestId("workspace-section-operations")));
    const tab = byTestId("operation-tab-review");
    if (tab && tab.getAttribute("aria-pressed") !== "true") tab.click();
  }
  async function closeSection() {
    const close = byTestId("workspace-dock-close");
    if (visible(close)) close.click();
    await sleep(200);
  }
  // Apply the queued intent through Review changes, as a person does.
  async function applyQueued(label) {
    await showOperations();
    const apply = await waitFor(`${label}: apply`, () => { const item = document.querySelector('[data-testid^="apply-intent-"]'); return item && !item.disabled ? item : null; }, 15000);
    const before = text("operation-apply-message");
    apply.click();
    const shown = await waitFor(`${label}: applied`, () => {
      const now = text("operation-apply-message") ?? "";
      if (now !== before && /refus|reject|fail/i.test(now)) throw Object.assign(new Error(`${label}: ${now}`), { hard: true });
      return now !== before && now.includes("Applied") ? now : null;
    }, 30000);
    await closeSection();
    return shown;
  }
  async function applyReviewedDraft(label) {
    await waitFor(`${label}: review`, () => byTestId("viewport-draft-review-preview"), 10000);
    click("apply-reviewed-draft");
  }
  async function ensureInspector() {
    const toggle = byTestId("toggle-inspector");
    if (toggle?.getAttribute("aria-disabled") === "true" && toggle.getAttribute("aria-expanded") !== "true") click("view-switch-both");
    if (byTestId("toggle-inspector")?.getAttribute("aria-expanded") !== "true") byTestId("toggle-inspector")?.click();
    await sleep(300);
  }
  const typedTreeRow = (type, id) => byTestId(`tree-row-${encodeURIComponent(type)}-${encodeURIComponent(id)}`);
  const CODE_PATTERN = /\b[A-Z][A-Z0-9]*(?:[-_][A-Z0-9]+)+\b/g;
  const codesIn = (element) => [...new Set((element?.textContent ?? "").match(CODE_PATTERN) ?? [])];
  const proofText = () => byTestId("status-pill-solve-proof")?.querySelector("code")?.textContent ?? null;

  // One solve from the current session in the default (sparse) mode, read from the window.
  async function solveCurrent(label, projectId, probe) {
    await showSection("solve");
    click("solver-mode-sparse");
    await waitFor("sparse selected", () => byTestId("solver-mode-sparse")?.getAttribute("aria-pressed") === "true");
    const prior = proofText();
    await menu("analyze.run");
    await waitFor(`${label} solve`, () => {
      const summary = text("solve-job-summary") ?? "";
      if (/state=(failed|cancelled)/.test(summary)) throw Object.assign(new Error(`${label}: ${summary}; ${text("solve-job-error") ?? ""}`), { hard: true });
      const proof = proofText();
      return summary.includes("state=completed") && proof && proof !== prior && proof.includes(`project=${projectId}`);
    }, 300000);
    const observation = await observe(label, projectId, probe);
    if (probe) {
      // The probe row is the displacement magnitude; its signed components are separate rows.
      fill("result-filter-input", probe);
      observation.components = {};
      for (const axis of ["ux", "uy", "uz"]) observation.components[axis] = await waitFor(`${probe}:${axis} row`, () => text(`result-row-dual-${probe}:${axis}`));
      fill("result-filter-input", "");
    }
    // With the filter cleared, the summary must count every solved row.
    const all = `${observation.proof_rows} of ${observation.proof_rows} results match filter`;
    observation.all_rows_summary = await waitFor(`${label}: unfiltered summary`, () => text("result-filter-summary") === all ? all : null, 15000).catch(() => text("result-filter-summary"));
    observation.deformation = text("viewport-deformation-summary");
    observation.health = text("numerical-health-header-line");
    return observation;
  }

  async function renderLocalPrivateReport(label) {
    await showSection("report");
    const intent = byTestId("rendered-report-local-private-intent");
    if (!intent) throw new Error("no local-private report selection");
    if (!intent.checked) { intent.click(); await waitFor("report intent", () => byTestId("rendered-report-local-private-intent")?.checked, 10000); }
    await waitFor("render enabled", () => byTestId("rendered-report-render") && !byTestId("rendered-report-render").disabled, 30000);
    click("rendered-report-render");
    await waitFor(`${label} render`, () => byTestId("rendered-report-hash") || byTestId("rendered-report-error") || byTestId("rendered-report-route"), 180000);
    await waitFor(`${label} render settled`, () => !/Rendering/.test(byTestId("rendered-report-render")?.textContent ?? ""), 180000);
    const html = byTestId("rendered-report-preview")?.getAttribute("srcdoc") ?? "";
    const observation = {
      label,
      redaction: text("rendered-report-redaction-summary"),
      hash: text("rendered-report-hash"),
      gate: text("rendered-report-gate"),
      error: text("rendered-report-error"),
      local_private_warning: Boolean(byTestId("rendered-report-local-private-warning")),
      save_offered: Boolean(byTestId("rendered-report-save-local-private")),
      preview_local_private_notice: /LOCAL PRIVATE REPORT/.test(html),
      preview_chars: html.length
    };
    await log(`${label}: ${JSON.stringify(observation)}`);
    return observation;
  }

  async function runRuleChecks(label) {
    await showSection("solve");
    const load = byTestId("rule-check-load-demo");
    if (load && !load.disabled) { load.click(); await sleep(500); }
    const run = await waitFor(`${label}: rule-check run enabled`, () => { const button = byTestId("rule-check-run"); return button && !button.disabled ? button : null; }, 30000);
    run.click();
    const aggregate = await waitFor(`${label}: rule-check aggregate`, () => text("rule-check-aggregate-status"), 120000);
    const observation = { label, aggregate, status: text("rule-check-run-status"), outcomes: text("rule-check-outcomes")?.slice(0, 600) ?? null };
    await log(`${label}: ${JSON.stringify(observation)}`);
    return observation;
  }

  // EX (pressure-2) output gates: the report is refused by name; the export panels say what they offer.
  async function exactRouteGates(plan, report) {
    const shown = await openModelDocument(plan.exact.path);
    await waitFor("exact model in the model tree", () => treeRow(plan.exact.project_id));
    const solved = await solveCurrent("exact", plan.exact.project_id, null);
    await showSection("report");
    // The rendered-report panel's own output gate: its refusal replaces the render control.
    const renderedPanel = byTestId("rendered-report-panel");
    const gates = {
      opened: shown,
      solve: solved,
      rendered_report_panel: Boolean(renderedPanel),
      render_control_present: Boolean(renderedPanel?.querySelector('[data-testid="rendered-report-render"]')),
      render_refusal: renderedPanel?.querySelector('[data-testid="rendered-report-load-reference-output-unavailable"]')?.textContent?.trim() ?? null
    };
    await showSection("exports");
    // The result packet is assembled asynchronously (8.5 s observed on a loaded Mac).
    gates.result_export_link = Boolean(await waitFor("exact result export link", () => byTestId("result-export-link"), 40000).catch(() => null));
    const mbfPanel = byTestId("caepipe-mbf-panel");
    gates.mbf_panel = Boolean(mbfPanel);
    gates.mbf_refusal = mbfPanel?.querySelector('[data-testid="caepipe-mbf-load-reference-output-unavailable"]')?.textContent?.trim() ?? null;
    gates.mbf_download_controls = Boolean(mbfPanel?.querySelector('[data-testid="caepipe-mbf-export-link"], [data-testid="caepipe-mbf-text-link"]'));
    gates.export_codes = codesIn(byTestId("workspace-section-exports"));
    gates.rule_checks = await runRuleChecks("exact/rule-checks");
    await log(`exact gates: ${JSON.stringify({ ...gates, solve: undefined, rule_checks: undefined })}`);
    report.exact = gates;
  }

  // Author a cantilever from blank through the GUI: units dialog, node, material,
  // pipe to a new node, fixed support, load case and a tip force; solve; save in place.
  async function authorFromBlank(plan, report) {
    const blank = plan.blank;
    const steps = [];
    report.blank = { steps };
    const step = async (name, fn) => { await log(`blank: ${name}`); const value = await fn(); steps.push(name); return value; };
    await closeSection();
    await step("units dialog", async () => {
      await menu("file.new-blank");
      await waitFor("units dialog", () => byTestId("project-units-dialog"), 20000);
      const preset = byTestId("project-units-preset");
      report.blank.unit_presets = preset ? [...preset.querySelectorAll("option")].map((option) => option.value) : [];
      click("project-units-create");
      await waitFor("units dialog closed", () => !byTestId("project-units-dialog"), 20000);
      const row = await waitFor("blank project in the tree", () => document.querySelector('[data-testid^="tree-row-project-"]'), 20000);
      report.blank.project_id = decodeURIComponent(row.getAttribute("data-testid").slice("tree-row-project-".length));
    });
    await step("node", async () => {
      click("command-node");
      await waitFor("node form", () => byTestId("viewport-create-node-id"), 10000);
      fillAny("viewport-create-node-id", blank.anchor); fillAny("viewport-create-node-label", "Anchor");
      fillAny("viewport-create-node-x", "0"); fillAny("viewport-create-node-y", "0"); fillAny("viewport-create-node-z", "0");
      fillAny("viewport-create-node-provenance", blank.provenance);
      click("queue-explicit-node-intent");
      await applyReviewedDraft("node");
      await waitFor("node in the tree", () => typedTreeRow("node", blank.anchor), 20000);
    });
    await step("material", async () => {
      await ensureInspector();
      click("toolkit-entry"); await sleep(300); click("toolkit-properties.material");
      await waitFor("material form", () => byTestId("create-material-id"), 10000);
      fillAny("create-material-id", blank.material); fillAny("create-material-label", "Invented steel");
      fillAny("create-material-elastic", String(blank.elastic_modulus_pa)); fillAny("create-material-shear", String(blank.shear_modulus_pa));
      fillAny("create-material-provenance", blank.provenance);
      click("queue-create-material-intent");
      await applyQueued("material");
    });
    await step("pipe", async () => {
      click("command-pipe");
      await waitFor("pipe form", () => byTestId("viewport-create-pipe-id"), 10000);
      fillAny("viewport-create-pipe-id", blank.pipe); fillAny("viewport-create-pipe-label", "Cantilever");
      await pickTarget("viewport-create-pipe-from", blank.anchor);
      chooseRadio("New node");
      fillAny("viewport-route-end-id", blank.tip); fillAny("viewport-route-end-label", "Free end");
      fillAny("viewport-route-end-x", String(blank.length_m)); fillAny("viewport-route-end-y", "0"); fillAny("viewport-route-end-z", "0");
      fillAny("viewport-route-end-provenance", blank.provenance);
      await pickTarget("viewport-create-pipe-material", blank.material);
      fillAny("viewport-create-pipe-od", String(blank.outside_diameter_m)); fillAny("viewport-create-pipe-wall", String(blank.wall_m));
      fillAny("viewport-create-pipe-yref-x", "0"); fillAny("viewport-create-pipe-yref-y", "0"); fillAny("viewport-create-pipe-yref-z", "1");
      fillAny("viewport-create-pipe-provenance", blank.provenance);
      click("queue-explicit-pipe-intent");
      await applyReviewedDraft("pipe");
      await waitFor("pipe in the tree", () => typedTreeRow("pipe", blank.pipe), 20000);
    });
    await step("support", async () => {
      click("command-support");
      await waitFor("support form", () => byTestId("create-support-id"), 10000);
      fillAny("create-support-id", blank.support); fillAny("create-support-label", "Anchor support");
      await pickTarget("create-support-node", blank.anchor);
      for (const dof of ["TX", "TY", "TZ", "RX", "RY", "RZ", "UX", "UY", "UZ"]) if (byTestId(`create-support-restraint-${dof}`)) checkBox(`create-support-restraint-${dof}`);
      fillAny("create-support-provenance", blank.provenance);
      click("queue-create-support-intent");
      await applyQueued("support");
    });
    await step("load", async () => {
      await showSection("loads");
      fillAny("load-manager-create-load-id", blank.load_case); fillAny("load-manager-create-load-label", "Invented tip force");
      fillAny("load-manager-create-load-kind", "primitive_user_load"); fillAny("load-manager-create-load-status", "preview_only");
      fillAny("load-manager-create-load-provenance", blank.provenance);
      click("queue-create-load-case-intent");
      await applyQueued("load case");
      await showSection("loads");
      await pickTarget("load-manager-create-primitive-load-case", blank.load_case);
      fillAny("load-manager-create-primitive-category", "concentrated_force");
      fillAny("load-manager-create-primitive-id", `${blank.load_case}-FY`);
      await pickTarget("load-manager-create-primitive-node", blank.tip);
      fillAny("load-manager-create-primitive-direction", "global_y");
      fillAny("load-manager-create-primitive-magnitude", String(blank.force_n));
      fillAny("load-manager-create-primitive-provenance", blank.provenance);
      click("queue-create-primitive-intent");
      await applyQueued("tip force");
    });
    report.blank.solve = await step("solve", () => solveCurrent("blank", report.blank.project_id, blank.probe));
    report.blank.save = await step("save in place", () => saveInPlace("blank/save"));
  }

  async function phaseOpenSolveSave(plan, report) {
    for (const refusal of plan.refusals) {
      report.refusals.push({ file: refusal.path.split("/").pop(), message: await openModelDocument(refusal.path), prior_model_kept: Boolean(treeRow(plan.bundled_project_id)) });
    }
    for (const doc of plan.documents) {
      const shown = await openModelDocument(doc.path);
      await waitFor(`${doc.case} in the model tree`, () => treeRow(doc.project_id));
      report.opened.push({ case: doc.case, message: shown });
      for (const mode of doc.modes) report.observations.push({ case: doc.case, mode, ...(await solve(mode, `${doc.case}/${mode}`, doc.project_id, doc.probe)) });
      report.exports.push(await exportResult(doc.case));
      report.saved.push({ case: doc.case, message: await saveAsProject(doc.project_id) });
      if (doc.replace_flows) await replaceFlows(doc, report);
    }
    // The session is now the saved demo: its local-private report and rule checks.
    report.report_render = await renderLocalPrivateReport("demo/report");
    report.rule_checks = await runRuleChecks("demo/rule-checks");
    await exactRouteGates(plan, report);
    await authorFromBlank(plan, report);
  }

  async function phaseReopen(plan, report) {
    // The bundled demo is not exempt: saving it over the stored demo asks.
    report.prompts.push(await saveThroughPrompt("file.save-local", "cancel", "bundled-demo/save"));
    let previousId = plan.bundled_project_id;
    for (const doc of plan.documents) {
      report.opened.push({ case: doc.case, message: await openProjectById(doc.project_id, previousId) });
      previousId = doc.project_id;
      for (const mode of doc.modes) report.observations.push({ case: doc.case, mode, ...(await solve(mode, `${doc.case}/${mode}/reopened`, doc.project_id, doc.probe)) });
    }
    // W-4: the project authored from blank, reopened in this fresh process and solved again.
    if (plan.blank?.project_id) {
      report.opened.push({ case: "blank", message: await openProjectById(plan.blank.project_id, previousId) });
      report.blank = { solve: await solveCurrent("blank/reopened", plan.blank.project_id, plan.blank.probe) };
    }
  }

  (async () => {
    const report = { phase: null, ok: false, refusals: [], opened: [], observations: [], exports: [], saved: [], prompts: [], user_agent: navigator.userAgent, location: String(window.location) };
    try {
      const plan = await call("native_smoke_plan");
      report.phase = plan.phase;
      await waitFor("bundled demo session", () => byTestId("desktop-preview-shell") && treeRow(plan.bundled_project_id), 120000);
      await log(`phase ${plan.phase} started`);
      if (plan.phase === "open-solve-save") await phaseOpenSolveSave(plan, report);
      else if (plan.phase === "reopen") await phaseReopen(plan, report);
      else throw new Error(`unknown phase ${plan.phase}`);
      report.ok = true;
    } catch (error) {
      report.error = `${error?.message ?? error}${error?.stack ? `\n${error.stack}` : ""}`;
      await log(`FAILED: ${report.error}`);
    }
    await call("native_smoke_finish", { report });
  })();
})();
