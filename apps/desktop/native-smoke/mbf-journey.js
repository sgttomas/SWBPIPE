// Native driver for the supported-subset MBF round trip in the launched app (RT-R7/RT-R8; DEL-17-04 Design
// `desktop-mbf-export-import.md` §5.1). `mbf-journey.mjs` launches the `native-smoke` build with this script,
// which the build evaluates in the real webview once the page has loaded (`src-tauri/src/native_smoke.rs`).
//
// It works the app as a person would: native menu commands go through the dispatch a menu click uses, and
// buttons, the review's own intent control and the declaration selects are operated in the window. The only
// substitution is the file choosers (Open Model Document…, Export MBF's save, Import MBF's open, Designate original
// source… and Save Model Document…): `native_smoke_choose` names the file the next chooser returns. Export review, save, read-back,
// import review and Adopt run on the product's native workers. It records what the window shows; the runner
// judges against the maintained fixtures.
//
// Phases (plan.phase):
//   bound   - Open the invented source, Export MBF with the dialog's default settings and save the pair; Import
//             MBF of that pair (bound), Designate original source… with the opened source document (the
//             model-document chooser) and Adopt the designated review; Save Model Document of the adopted session.
//             Then the Open guard: edit node:A, import the pair again, Adopt, and cancel the unsaved-changes prompt.
//   unbound - Import the MBF copied without its identity file, declare its material and Adopt; Save Model Document.
(() => {
  const ipc = window.__TAURI_INTERNALS__;
  const call = (command, args) => ipc.invoke(command, args ?? {});
  const log = (line) => call("native_smoke_log", { line: String(line) }).catch(() => {});
  const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
  const byTestId = (id, root = document) => root.querySelector(`[data-testid="${CSS.escape(id)}"]`);
  const text = (id, root) => byTestId(id, root)?.textContent?.trim() ?? null;
  const bounded = (value, limit = 8000) => (value === null || value === undefined ? null : String(value).slice(0, limit));
  const message = () => text("local-project-message") ?? "";
  const menu = (commandId) => call("native_smoke_menu", { commandId });
  const treeRow = (projectId) => byTestId(`tree-row-project-${encodeURIComponent(projectId)}`);

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
  // Set a control's value through its native setter and dispatch the event React listens for.
  function setValue(target, value) {
    const prototype = target instanceof HTMLSelectElement ? HTMLSelectElement.prototype : HTMLInputElement.prototype;
    Object.getOwnPropertyDescriptor(prototype, "value").set.call(target, value);
    target.dispatchEvent(new Event(target instanceof HTMLSelectElement ? "change" : "input", { bubbles: true }));
  }
  const listItems = (id, root) => [...(byTestId(id, root)?.querySelectorAll("li") ?? [])].map((item) => item.textContent.trim());

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
      if (!shown.startsWith("Saved model document ")) throw new Error(`${label}: ${shown}`);
      return shown;
    }
  }

  // The export opens once the session's model hash has settled; the menu action is offered again until then.
  async function openExport() {
    for (let attempt = 0; attempt < 120; attempt += 1) {
      await menu("file.export-mbf");
      try { return await waitFor("export dialog", () => byTestId("mbf-export-dialog"), 1000); } catch { /* hash pending */ }
    }
    throw new Error("Export MBF did not open after its model hash settled");
  }

  async function exportAndSave(path) {
    const dialog = await openExport();
    await waitFor("export review", () => byTestId("mbf-export-profile", dialog) || byTestId("mbf-export-refusal", dialog), 300000);
    const seen = {
      title: byTestId("mbf-export-title", dialog)?.value ?? null,
      reference_temperature: byTestId("mbf-export-reference-temperature", dialog)?.value ?? null,
      suggested_name: text("mbf-export-suggested-name", dialog),
      completeness: text("mbf-export-completeness-state", dialog),
      ceiling: bounded(text("mbf-export-ceiling", dialog)),
      omitted: listItems("mbf-export-omitted", dialog).length,
      refusal: bounded(text("mbf-export-refusal", dialog)),
      save_disabled_before_intent: byTestId("mbf-export-save", dialog)?.disabled === true
    };
    await call("native_smoke_choose", { path });
    click("mbf-export-intent", dialog);
    await waitFor("export save enabled", () => byTestId("mbf-export-save", dialog)?.disabled === false);
    click("mbf-export-save", dialog);
    seen.status = await waitFor("export saved", () => {
      const status = text("mbf-export-status", dialog);
      return status && /^Saved |refused|failed/i.test(status) ? status : null;
    }, 300000);
    await log(`export: ${seen.status}`);
    click("mbf-export-cancel", dialog);
    await waitFor("export closed", () => !byTestId("mbf-export-dialog"));
    return seen;
  }

  async function openImport(path) {
    await call("native_smoke_choose", { path });
    await menu("file.import-mbf");
    const dialog = await waitFor("import dialog", () => byTestId("mbf-import-dialog"));
    await waitFor("import review", () => byTestId("mbf-import-review", dialog) || byTestId("mbf-import-refusal", dialog), 300000);
    return dialog;
  }

  function importReview(dialog) {
    return {
      source: bounded(text("mbf-import-source", dialog)),
      binding: bounded(text("mbf-import-binding", dialog)),
      completeness: text("mbf-import-completeness-state", dialog),
      identity: bounded(text("mbf-import-identity", dialog)),
      loss: listItems("mbf-import-loss", dialog),
      refusal: bounded(text("mbf-import-refusal", dialog)),
      adopt_disabled: byTestId("mbf-import-adopt", dialog)?.disabled === true
    };
  }

  // Designate original source…: the model-document chooser returns `path`; the dialog then shows the
  // designated review, or the designation's refusal over the ordinary review.
  async function designateSource(dialog, path) {
    await waitFor("Designate enabled", () => byTestId("mbf-import-designate", dialog)?.disabled === false, 300000);
    await call("native_smoke_choose", { path });
    click("mbf-import-designate", dialog);
    await waitFor("designated review", () => byTestId("mbf-import-designation", dialog) || byTestId("mbf-import-designation-refusal", dialog), 300000);
    const designation = byTestId("mbf-import-designation", dialog);
    const seen = {
      ...importReview(dialog),
      designation: bounded(designation?.textContent?.trim()),
      reasons: [...(designation?.querySelectorAll("[data-reason]") ?? [])].map((item) => item.getAttribute("data-reason")),
      differing_classes: bounded(text("mbf-import-designation-classes", dialog)),
      source_completeness: bounded(text("mbf-import-source-completeness", dialog)),
      designation_refusal: bounded(text("mbf-import-designation-refusal", dialog))
    };
    await log(`designate: ${seen.designation ?? seen.designation_refusal}`);
    return seen;
  }

  async function adopt(dialog, label) {
    await waitFor(`${label}: Adopt enabled`, () => byTestId("mbf-import-adopt", dialog)?.disabled === false, 300000);
    const before = message();
    click("mbf-import-adopt", dialog);
    const shown = await waitFor(`${label}: adopted`, () => {
      const now = message();
      return now !== before && now ? now : null;
    }, 300000);
    await waitFor(`${label}: import closed`, () => !byTestId("mbf-import-dialog"));
    await log(`${label}: ${shown}`);
    return { message: shown, edited: text("project-edited") };
  }

  // Edit node:A's y through the node grid and apply it as one operation.
  async function editNodeA() {
    click("layout-mode-grid");
    const disclosure = await waitFor("node grid disclosure", () => byTestId("node-grid-review-disclosure"));
    if (disclosure.getAttribute("aria-expanded") !== "true") disclosure.click();
    const cell = await waitFor("node:A y cell", () => byTestId("review-cell-node:A-y"));
    cell.dispatchEvent(new MouseEvent("dblclick", { bubbles: true }));
    const input = await waitFor("cell editor", () => byTestId("engineering-table-review")?.querySelector("input"));
    setValue(input, "0.5");
    click("queue-entity-grid-intents");
    const apply = await waitFor("apply control", () => [...document.querySelectorAll('[data-testid^="apply-intent-editor-intent-"]')].at(-1));
    apply.click();
    const summary = await waitFor("operation applied", () => /1 applied/.test(text("operation-apply-summary") ?? "") && text("operation-apply-summary"), 30000);
    const edited = await waitFor("session edited", () => /Edited/.test(text("project-edited") ?? "") && text("project-edited"), 30000);
    return { summary, edited };
  }

  async function phaseBound(plan, report) {
    report.opened = await openModelDocument(plan.source.path);
    await waitFor("source in the model tree", () => treeRow(plan.source.project_id));
    report.export = await exportAndSave(plan.pair.mbf);
    const dialog = await openImport(plan.pair.mbf);
    report.import = importReview(dialog);
    report.designated = await designateSource(dialog, plan.source.path);
    report.adopted = await adopt(dialog, "bound/designated");
    report.saved = await saveModelDocument(plan.adopted_document, "bound/adopted");
    // The Open guard: a later unsaved edit survives a cancelled import of the earlier pair.
    report.guard = { edit: await editNodeA() };
    const again = await openImport(plan.pair.mbf);
    await waitFor("guarded Adopt enabled", () => byTestId("mbf-import-adopt", again)?.disabled === false, 300000);
    click("mbf-import-adopt", again);
    report.guard.prompt = Boolean(await waitFor("unsaved-changes prompt", () => byTestId("unsaved-changes-prompt")));
    click("unsaved-changes-cancel");
    report.guard.message = await waitFor("import not opened", () => /Not opened/.test(message()) && message());
    report.guard.dialog_closed = Boolean(await waitFor("guarded import closed", () => !byTestId("mbf-import-dialog")));
    report.guard.edited_after = text("project-edited");
    report.guard.saved = await saveModelDocument(plan.guard_document, "bound/guard-kept");
  }

  async function phaseUnbound(plan, report) {
    const dialog = await openImport(plan.moved_mbf);
    report.import = importReview(dialog);
    const selects = [...dialog.querySelectorAll("select")].filter((select) => /^Source of /.test(select.getAttribute("aria-label") ?? ""));
    report.declared = selects.map((select) => select.getAttribute("aria-label"));
    for (const select of selects) setValue(select, "invented");
    await waitFor("declared review", () => byTestId("mbf-import-adopt", dialog)?.disabled === false, 300000);
    report.declared_review = importReview(dialog);
    report.adopted = await adopt(dialog, "unbound");
    report.saved = await saveModelDocument(plan.adopted_document, "unbound/adopted");
  }

  (async () => {
    const report = { phase: null, ok: false, user_agent: navigator.userAgent };
    try {
      const plan = await call("native_smoke_plan");
      report.phase = plan.phase;
      await waitFor("desktop shell", () => byTestId("desktop-preview-shell"), 120000);
      await log(`mbf journey phase ${plan.phase} started`);
      if (plan.phase === "bound") await phaseBound(plan, report);
      else if (plan.phase === "unbound") await phaseUnbound(plan, report);
      else throw new Error(`unknown phase ${plan.phase}`);
      report.ok = true;
    } catch (error) {
      report.error = `${error?.message ?? error}${error?.stack ? `\n${error.stack}` : ""}`;
      await log(`FAILED: ${report.error}`);
    }
    await call("native_smoke_finish", { report });
  })();
})();
