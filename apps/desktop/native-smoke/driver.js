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

  async function exportResult(label) {
    await showSection("exports");
    const available = await waitFor("result export panel", () => byTestId("result-export-link") ? "available" : byTestId("result-export-empty") ? "empty" : null, 30000);
    if (available === "empty") {
      await sleep(2000); // the packet is assembled asynchronously after a solve
      if (!byTestId("result-export-link")) return { label, available: false, reason: text("result-export-empty") };
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
    }
  }

  async function phaseReopen(plan, report) {
    let previousId = plan.bundled_project_id;
    for (const doc of plan.documents) {
      report.opened.push({ case: doc.case, message: await openProjectById(doc.project_id, previousId) });
      previousId = doc.project_id;
      for (const mode of doc.modes) report.observations.push({ case: doc.case, mode, ...(await solve(mode, `${doc.case}/${mode}/reopened`, doc.project_id, doc.probe)) });
    }
  }

  (async () => {
    const report = { phase: null, ok: false, refusals: [], opened: [], observations: [], exports: [], saved: [], user_agent: navigator.userAgent, location: String(window.location) };
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
