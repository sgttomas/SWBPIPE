// Driver of the evidence journeys (`evidence-journeys.mjs`) in the launched `native-smoke` app,
// which evaluates this script in the real webview once the page has loaded
// (`src-tauri/src/native_smoke.rs`).
//
// It works the app as a person would: native menu commands go through the dispatch a menu click
// uses, and the window's own controls (the user-release dialog's tick box and statement, the
// autosave interval, the recovery offer) are operated in the window. The only substitution is the
// file choosers: `native_smoke_choose` names the file the next one returns. It records what the
// window shows; the runner judges, from the files the app wrote.
//
// Phases (plan.phase):
//   report          - on plan.route ("pressure-free": the bundled demo; "exact": the opened exact
//                     fixture), solve, render the local-private report, save the user-released
//                     package through File > Save Shareable Report Package… and its dialog, open the
//                     dialog again and cancel it, save the next (local-private) package, and save the
//                     session's model document.
//   autosave-end    - read the autosave setting on the fresh settings folder, turn it off and on at
//                     the shortest interval, save the demo as a local project, checkpoint (with a copy
//                     of the store), edit node N-100 and leave it unsaved, save the session's model
//                     document, checkpoint again, and wait: the runner ends the app with SIGKILL.
//   autosave-reopen - read the recovery offer and the window before any choice, choose Open
//                     autosave, and save the session's model document.
(() => {
  const ipc = window.__TAURI_INTERNALS__;
  const call = (command, args) => ipc.invoke(command, args ?? {});
  const log = (line) => call("native_smoke_log", { line: String(line) }).catch(() => {});
  const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
  const byTestId = (id, root = document) => root.querySelector(`[data-testid="${CSS.escape(id)}"]`);
  const text = (id, root) => byTestId(id, root)?.textContent?.trim() ?? null;
  const message = () => text("local-project-message") ?? "";
  const menu = (commandId) => call("native_smoke_menu", { commandId });
  const treeRow = (projectId) => byTestId(`tree-row-project-${encodeURIComponent(projectId)}`);
  const settle = () => new Promise((resolve) => setTimeout(resolve, 0));

  async function waitFor(label, probe, timeoutMs = 120000) {
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
    const prototype = target instanceof HTMLSelectElement ? HTMLSelectElement.prototype : HTMLInputElement.prototype;
    Object.getOwnPropertyDescriptor(prototype, "value").set.call(target, value);
    target.dispatchEvent(new Event(target instanceof HTMLSelectElement ? "change" : "input", { bubbles: true }));
  }
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

  async function openModelDocument(path) {
    const name = path.split("/").pop();
    const before = message();
    await call("native_smoke_choose", { path });
    await menu("file.open-model");
    return waitFor(`open ${name}`, () => {
      const now = message();
      return now !== before && (now.includes(`Opened model document ${name} `) || now.includes(`refused (${name})`)) ? now : null;
    });
  }

  // File > Save Model Document…: the session's model document as RFC 8785 text, which the runner hashes.
  async function saveModelDocument(path, label) {
    await call("native_smoke_choose", { path });
    for (let attempt = 0; ; attempt += 1) {
      const before = message();
      await menu("file.save-model-document");
      await waitFor(`${label}: save dialog`, () => byTestId("model-document-save-dialog"));
      click("model-document-save-intent");
      await waitFor(`${label}: confirm enabled`, () => byTestId("model-document-save-confirm")?.disabled === false);
      click("model-document-save-confirm");
      const shown = await waitFor(`${label}: saved`, () => {
        const now = message();
        return now !== before && (now.startsWith("Saved model document ") || now.startsWith("Save model document")) ? now : null;
      });
      await log(`${label}: ${shown}`);
      if (shown.includes("MODEL-DOC-SAVE-HASH-PENDING") && attempt < 20) { await sleep(500); continue; }
      if (!shown.startsWith("Saved model document ")) {
        await call("native_smoke_clear_choices");
        throw new Error(`${label}: ${shown}`);
      }
      return { message: shown, at: Date.now() };
    }
  }

  // One sparse solve of the session, read from the window: the job completed with a new proof of `projectId`.
  async function solve(label, projectId) {
    await showSection("solve");
    click("solver-mode-sparse");
    await waitFor("sparse selected", () => byTestId("solver-mode-sparse")?.getAttribute("aria-pressed") === "true");
    const proofText = () => byTestId("status-pill-solve-proof")?.querySelector("code")?.textContent ?? null;
    const prior = proofText();
    await menu("analyze.run");
    await waitFor(`${label} solve`, () => {
      const summary = text("solve-job-summary") ?? "";
      if (/\bstate=(failed|cancelled|refused)(?:;|$)/.test(summary)) throw new Error(`${label}: ${summary}; ${text("solve-job-error") ?? ""}`);
      const proof = proofText();
      return summary.includes("state=completed") && proof && proof !== prior && proof.includes(`project=${projectId}`);
    }, 300000);
    return { summary: text("solve-job-summary"), proof: proofText() };
  }

  async function renderLocalPrivateReport(label) {
    await showSection("report");
    const intent = element("rendered-report-local-private-intent");
    if (!intent.checked) { intent.click(); await waitFor("report intent", () => byTestId("rendered-report-local-private-intent")?.checked, 10000); }
    await waitFor("render enabled", () => byTestId("rendered-report-render") && !byTestId("rendered-report-render").disabled, 30000);
    click("rendered-report-render");
    await waitFor(`${label} render`, () => byTestId("rendered-report-hash") || byTestId("rendered-report-error") || byTestId("rendered-report-route"), 180000);
    await waitFor(`${label} render settled`, () => !/Rendering/.test(byTestId("rendered-report-render")?.textContent ?? ""), 180000);
    return {
      hash: text("rendered-report-hash"),
      gate: text("rendered-report-gate"),
      error: text("rendered-report-error"),
      // Recorded only: save and print are not observed by this journey (see the runner's header).
      save_control: Boolean(byTestId("rendered-report-save-local-private")),
      print_control: Boolean(byTestId("rendered-report-print"))
    };
  }

  // The package save's status, once the save is no longer in progress; a save the memory pool
  // refused only for this Mac's momentary available memory is reported as such to the caller.
  // `prior` is the status line shown before this save started, which is never this save's.
  async function packageOutcome(label, prior) {
    await waitFor(`${label}: save started`, () => /Saving report package/.test(byTestId("report-package-save")?.textContent ?? "") || null, 10000).catch(() => null);
    return waitFor(`${label}: saved`, () => {
      const now = text("report-package-save-status");
      return now && now !== prior && !/Saving report package/.test(byTestId("report-package-save")?.textContent ?? "") ? now : null;
    }, 300000);
  }

  // File > Save Shareable Report Package…: the dialog as shown, then its statement ticked and
  // confirmed, the save chooser answered with `path`; the saved file re-read with the product's reader.
  async function saveUserReleasedPackage(path, label) {
    await showSection("report");
    const record = { label, memory_refusals: 0 };
    for (;;) {
      await waitFor(`${label}: controls`, () => byTestId("report-package-user-release") && !byTestId("report-package-user-release").disabled, 60000);
      await menu("file.save-shareable-report-package");
      const dialog = await waitFor(`${label}: dialog`, () => byTestId("user-release-dialog"), 30000);
      const statement = byTestId("user-release-statement", dialog);
      Object.assign(record, {
        title: dialog.querySelector("h2")?.textContent?.trim() ?? null,
        scope: text("user-release-scope", dialog),
        refusal: text("user-release-refusal", dialog),
        statement_label: statement?.closest("label")?.textContent?.trim() ?? null,
        statement_checked_initially: statement ? statement.checked : null,
        confirm_disabled_before_statement: byTestId("user-release-confirm", dialog)?.disabled === true
      });
      if (!statement) {
        click("user-release-cancel", dialog);
        return record;
      }
      statement.click();
      await waitFor(`${label}: confirm enabled`, () => byTestId("user-release-confirm")?.disabled === false, 10000);
      await call("native_smoke_choose", { path });
      const prior = text("report-package-save-status");
      click("user-release-confirm");
      record.status = await packageOutcome(label, prior);
      record.user_release_status = text("report-package-user-release-status");
      await log(`${label}: ${record.status}`);
      if (!/RESOURCE_UNAVAILABLE: Not enough available memory/.test(record.status) || record.memory_refusals >= 20) break;
      await call("native_smoke_clear_choices");
      record.memory_refusals += 1;
      await sleep(5000);
    }
    if (!record.status.startsWith("outcome=saved")) {
      await call("native_smoke_clear_choices");
      return record;
    }
    record.read = await call("native_smoke_read_report_package", { path }).then((value) => value, (error) => ({ error: String(error) }));
    return record;
  }

  // The dialog opened again after the save: how it starts, then Cancel.
  async function reopenUserReleaseDialog() {
    await menu("file.save-shareable-report-package");
    const dialog = await waitFor("dialog again", () => byTestId("user-release-dialog"), 30000);
    const shown = {
      statement_checked_initially: byTestId("user-release-statement", dialog)?.checked ?? null,
      confirm_disabled: byTestId("user-release-confirm", dialog)?.disabled === true
    };
    click("user-release-cancel", dialog);
    shown.closed = Boolean(await waitFor("dialog closed", () => !byTestId("user-release-dialog"), 10000));
    return shown;
  }

  // File > Save Report Package… (local private), after its own selection; the file re-read.
  async function saveLocalPackage(path, label) {
    await showSection("report");
    const intent = await waitFor(`${label}: package controls`, () => byTestId("report-package-private-intent"), 30000);
    const record = { label, intent_checked_initially: intent.checked === true };
    if (!intent.checked) {
      intent.click();
      await waitFor(`${label}: intent recorded`, () => byTestId("report-package-private-intent")?.checked, 10000);
    }
    record.memory_refusals = 0;
    for (;;) {
      await call("native_smoke_choose", { path });
      await waitFor(`${label}: save enabled`, () => byTestId("report-package-save") && !byTestId("report-package-save").disabled, 30000);
      const prior = text("report-package-save-status");
      await menu("file.save-report-package");
      record.status = await packageOutcome(label, prior);
      record.user_release_status = text("report-package-user-release-status");
      if (!/RESOURCE_UNAVAILABLE: Not enough available memory/.test(record.status) || record.memory_refusals >= 20) break;
      await call("native_smoke_clear_choices");
      record.memory_refusals += 1;
      await sleep(5000);
    }
    if (!record.status.startsWith("outcome=saved")) {
      await call("native_smoke_clear_choices");
      return record;
    }
    record.read = await call("native_smoke_read_report_package", { path }).then((value) => value, (error) => ({ error: String(error) }));
    return record;
  }

  async function phaseReport(plan, report) {
    if (plan.open) {
      report.opened = await openModelDocument(plan.open);
      await waitFor("opened model in the model tree", () => treeRow(plan.project_id));
    }
    report.solve = await solve(plan.route, plan.project_id);
    report.render = await renderLocalPrivateReport(`${plan.route}/render`);
    report.user_released = await saveUserReleasedPackage(plan.user_released_package, `${plan.route}/user-released`);
    if (!report.user_released.status?.startsWith("outcome=saved")) throw new Error(`user-released package not saved: ${report.user_released.refusal ?? report.user_released.status}`);
    report.reopened_dialog = await reopenUserReleaseDialog();
    report.local = await saveLocalPackage(plan.local_package, `${plan.route}/local`);
    report.model_document = await saveModelDocument(plan.model_document, `${plan.route}/model-document`);
  }

  // ------------------------------------------------------------ autosave

  async function openAppearance() {
    const details = element("toolbar-appearance");
    if (!details.open) details.querySelector("summary").click();
    await waitFor("appearance open", () => byTestId("toolbar-appearance")?.open, 5000);
  }
  const intervalSelect = () => byTestId("autosave-interval");
  async function chooseInterval(minutes) {
    const select = await waitFor("interval control enabled", () => (intervalSelect() && !intervalSelect().disabled ? intervalSelect() : null), 30000);
    setValue(select, String(minutes));
    await waitFor(`interval ${minutes}`, () => intervalSelect()?.value === String(minutes) && !intervalSelect().disabled, 30000);
  }
  const offNotices = () => ({ off_notice: text("autosave-off-notice"), off_status: text("autosave-off-status") });

  // Node N-100's y through the node grid's review cell, queued and applied: an unsaved edit.
  async function editNode(nodeId, value) {
    click("layout-mode-grid");
    const disclosure = await waitFor("node grid disclosure", () => byTestId("node-grid-review-disclosure"));
    if (disclosure.getAttribute("aria-expanded") !== "true") disclosure.click();
    const cell = await waitFor(`${nodeId} y cell`, () => byTestId(`review-cell-${nodeId}-y`));
    cell.dispatchEvent(new MouseEvent("dblclick", { bubbles: true }));
    const input = await waitFor("cell editor", () => byTestId("engineering-table-review")?.querySelector("input"));
    setValue(input, value);
    click("queue-entity-grid-intents");
    const apply = await waitFor("apply control", () => [...document.querySelectorAll('[data-testid^="apply-intent-editor-intent-"]')].at(-1));
    apply.click();
    const summary = await waitFor("operation applied", () => /1 applied/.test(text("operation-apply-summary") ?? "") && text("operation-apply-summary"), 30000);
    const edited = await waitFor("session edited", () => /Edited/.test(text("project-edited") ?? "") && text("project-edited"), 30000);
    return { summary, edited, at: Date.now() };
  }

  async function phaseAutosaveEnd(plan, report) {
    await openAppearance();
    const select = await waitFor("autosave settings read", () => (intervalSelect() && !intervalSelect().disabled && intervalSelect().value !== "" ? intervalSelect() : null), 30000);
    report.setting_on_start = { value: select.value, options: [...select.options].map((option) => option.value), ...offNotices() };
    await chooseInterval(0);
    report.setting_off = { value: intervalSelect()?.value, ...(await waitFor("off notice", () => (offNotices().off_notice ? offNotices() : null), 10000).catch(() => offNotices())) };
    await chooseInterval(plan.interval_minutes);
    report.setting_on = { value: intervalSelect()?.value, autosave_status: text("autosave-status"), ...offNotices() };
    const before = message();
    await menu("file.new-local");
    report.saved_project = await waitFor("demo saved as a local project", () => {
      const now = message();
      if (now !== before && now.startsWith("Create failed")) throw new Error(now);
      return now !== before && now.includes("Created local SQLite project snapshot") ? now : null;
    });
    report.before_edit_at = Date.now();
    await call("native_smoke_checkpoint", { name: "before-edit", report, storeSnapshot: true });
    report.edit = await editNode(plan.edit.node, plan.edit.y);
    report.model_document = await saveModelDocument(plan.model_document, "autosave/before-end");
    report.edited_after_save = text("project-edited");
    await call("native_smoke_checkpoint", { name: "edited", report, storeSnapshot: false });
    await log("autosave-end: waiting to be ended");
    // The runner ends this process with SIGKILL once a record of the edited model is on disk.
    await new Promise(() => {});
  }

  async function phaseAutosaveReopen(plan, report) {
    const prompt = await waitFor("recovery offer", () => byTestId("autosave-recovery-prompt"), 120000);
    const windowState = () => ({
      project: text("toolbar-project-name"),
      edited: text("project-edited"),
      message: message(),
      tree_has_project: Boolean(treeRow(plan.project_id)),
      offers: document.querySelectorAll('[data-testid="autosave-recovery-prompt"]').length
    });
    report.offer = {
      title: prompt.querySelector("h2")?.textContent?.trim() ?? null,
      what: text("autosave-recovery-what", prompt),
      unreadable: text("autosave-recovery-unreadable", prompt),
      newer: text("autosave-recovery-newer", prompt),
      replaces: text("autosave-recovery-replaces", prompt),
      stored: text("autosave-recovery-stored", prompt),
      open_offered: Boolean(byTestId("autosave-recovery-open", prompt))
    };
    // Before any choice: the window, now and after a pause with the offer still open.
    report.before_choice = windowState();
    await sleep(plan.hold_ms);
    report.before_choice_later = { ...windowState(), offer_open: Boolean(byTestId("autosave-recovery-prompt")) };
    const before = message();
    click("autosave-recovery-open");
    report.opened = await waitFor("autosave opened", () => {
      const now = message();
      return now !== before && now !== "" ? now : null;
    }, 60000);
    report.after_open = windowState();
    report.model_document = await saveModelDocument(plan.model_document, "autosave/recovered");
  }

  (async () => {
    const report = { phase: null, ok: false, user_agent: navigator.userAgent };
    try {
      const plan = await call("native_smoke_plan");
      report.phase = plan.phase;
      await waitFor("desktop shell", () => byTestId("desktop-preview-shell") && treeRow(plan.bundled_project_id), 120000);
      await log(`evidence journey phase ${plan.phase} started`);
      if (plan.phase === "report") await phaseReport(plan, report);
      else if (plan.phase === "autosave-end") await phaseAutosaveEnd(plan, report);
      else if (plan.phase === "autosave-reopen") await phaseAutosaveReopen(plan, report);
      else throw new Error(`unknown phase ${plan.phase}`);
      report.ok = true;
    } catch (error) {
      report.error = `${error?.message ?? error}${error?.stack ? `\n${error.stack}` : ""}`;
      await log(`FAILED: ${report.error}`);
    }
    await call("native_smoke_finish", { report });
  })();
})();
