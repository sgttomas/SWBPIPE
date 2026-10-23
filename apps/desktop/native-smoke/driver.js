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
