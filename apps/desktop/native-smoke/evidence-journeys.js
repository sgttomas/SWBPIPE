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
//                     of the store), edit node N-100 and leave it unsaved, wait for the product to report
//                     a record written, edit node N-110 at once, checkpoint, and wait: the runner ends the
//                     app with SIGKILL before the next interval.
//   autosave-reopen - read the recovery offer and the window before any choice, choose Open
//                     autosave, and save the session's model document.
(() => {
  const ipc = window.__TAURI_INTERNALS__;
  const call = (command, args) => ipc.invoke(command, args ?? {});
  const log = (line) => call("native_smoke_log", { line: String(line) }).catch(() => {});
  // The act about to start, on the app's standard error ("[native-smoke] act: <name>"), awaited before the
  // act begins: when the app ends with the strict chooser's exit code, the runner names the last act shown.
  const act = (name) => log(`act: ${name}`);
  const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
  const byTestId = (id, root = document) => root.querySelector(`[data-testid="${CSS.escape(id)}"]`);
  const text = (id, root) => byTestId(id, root)?.textContent?.trim() ?? null;
  const message = () => text("local-project-message") ?? "";
  const menu = (commandId) => call("native_smoke_menu", { commandId });
  const treeRow = (projectId) => byTestId(`tree-row-project-${encodeURIComponent(projectId)}`);
  const settle = () => new Promise((resolve) => setTimeout(resolve, 0));

  // Every wait is for an observed condition, bounded by one of the plan's named timeouts (the runner
  // states them; this host runs loaded). A timeout names itself, its elapsed time and the moment it
  // ended (`at <unix ms>`), so the runner can read the load average then and tell a slow host from a
  // product that never showed the condition.
  let T = { ui: 60000, step: 300000, solve: 900000, save: 900000, record: 600000 };
  async function waitFor(label, probe, timeoutMs = T.step) {
    const started = Date.now();
    for (;;) {
      const value = probe();
      if (value) return value;
      const elapsed = Date.now() - started;
      if (elapsed > timeoutMs) throw new Error(`TIMEOUT waiting for ${label}: ${elapsed} ms elapsed, limit ${timeoutMs} ms, at ${Date.now()}`);
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
      }, T.save);
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
    }, T.solve);
    return { summary: text("solve-job-summary"), proof: proofText() };
  }

  async function renderLocalPrivateReport(label) {
    await showSection("report");
    const intent = element("rendered-report-local-private-intent");
    if (!intent.checked) { intent.click(); await waitFor("report intent", () => byTestId("rendered-report-local-private-intent")?.checked, T.ui); }
    await waitFor("render enabled", () => byTestId("rendered-report-render") && !byTestId("rendered-report-render").disabled, T.step);
    click("rendered-report-render");
    await waitFor(`${label} render`, () => byTestId("rendered-report-hash") || byTestId("rendered-report-error") || byTestId("rendered-report-route"), T.save);
    await waitFor(`${label} render settled`, () => !/Rendering/.test(byTestId("rendered-report-render")?.textContent ?? ""), T.save);
    const preview = byTestId("rendered-report-preview")?.getAttribute("srcdoc") ?? null;
    return {
      label,
      hash: text("rendered-report-hash"),
      gate: text("rendered-report-gate"),
      error: text("rendered-report-error"),
      redaction: text("rendered-report-redaction-summary"),
      intent_checked: byTestId("rendered-report-local-private-intent")?.checked === true,
      intent_label: byTestId("rendered-report-local-private-intent")?.closest("label")?.textContent?.trim() ?? null,
      local_private_warning: text("rendered-report-local-private-warning"),
      // The preview is the canonical HTML itself (the panel shows the controlled copy only when it is
      // byte-identical), so the runner recomputes its SHA-256 from this text.
      preview_html: preview,
      // Recorded only: the controls are shown or not; neither is pressed (the runner's header).
      save_control: Boolean(byTestId("rendered-report-save-local-private")),
      print_control: Boolean(byTestId("rendered-report-print"))
    };
  }

  // The package save's status, once the save is no longer in progress; a save the memory pool
  // refused only for this Mac's momentary available memory is reported as such to the caller.
  // Two observed conditions: the save started (the button says so, or the status line shown before
  // it, `prior`, was cleared or replaced), then it settled (the button no longer saving, a status shown).
  async function packageOutcome(label, prior) {
    const saving = () => /Saving report package/.test(byTestId("report-package-save")?.textContent ?? "");
    await waitFor(`${label}: save started`, () => saving() || text("report-package-save-status") !== prior, T.step);
    return waitFor(`${label}: saved`, () => {
      const now = text("report-package-save-status");
      return now && !saving() ? now : null;
    }, T.save);
  }

  // File > Save Shareable Report Package…: the dialog as shown, then its statement ticked and
  // confirmed, the save chooser answered with `path`; the saved file re-read with the product's reader.
  async function saveUserReleasedPackage(path, label) {
    await showSection("report");
    const record = { label, memory_refusals: 0 };
    for (;;) {
      await waitFor(`${label}: controls`, () => byTestId("report-package-user-release") && !byTestId("report-package-user-release").disabled, T.step);
      await menu("file.save-shareable-report-package");
      const dialog = await waitFor(`${label}: dialog`, () => byTestId("user-release-dialog"), T.step);
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
      await waitFor(`${label}: confirm enabled`, () => byTestId("user-release-confirm")?.disabled === false, T.ui);
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
    const dialog = await waitFor("dialog again", () => byTestId("user-release-dialog"), T.step);
    const shown = {
      statement_checked_initially: byTestId("user-release-statement", dialog)?.checked ?? null,
      confirm_disabled: byTestId("user-release-confirm", dialog)?.disabled === true
    };
    click("user-release-cancel", dialog);
    shown.closed = Boolean(await waitFor("dialog closed", () => !byTestId("user-release-dialog"), T.ui));
    return shown;
  }

  // File > Save Report Package… (local private), after its own selection; the file re-read.
  async function saveLocalPackage(path, label) {
    await showSection("report");
    const intent = await waitFor(`${label}: package controls`, () => byTestId("report-package-private-intent"), T.step);
    const record = { label, intent_checked_initially: intent.checked === true };
    if (!intent.checked) {
      intent.click();
      await waitFor(`${label}: intent recorded`, () => byTestId("report-package-private-intent")?.checked, T.ui);
    }
    record.memory_refusals = 0;
    for (;;) {
      await call("native_smoke_choose", { path });
      await waitFor(`${label}: save enabled`, () => byTestId("report-package-save") && !byTestId("report-package-save").disabled, T.step);
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
      await act("File > Open Model Document…");
      report.opened = await openModelDocument(plan.open);
      await waitFor("opened model in the model tree", () => treeRow(plan.project_id));
    }
    await act("solve (Analyze > Run, sparse)");
    report.solve = await solve(plan.route, plan.project_id);
    if (plan.observe_selection) {
      await act("selection by keyboard (observation)");
      report.selection = await observeSelection();
    }
    await act("render the local-private report");
    report.render = await renderLocalPrivateReport(`${plan.route}/render`);
    await act("File > Save Shareable Report Package…");
    report.user_released = await saveUserReleasedPackage(plan.user_released_package, `${plan.route}/user-released`);
    if (!report.user_released.status?.startsWith("outcome=saved")) throw new Error(`user-released package not saved: ${report.user_released.refusal ?? report.user_released.status}`);
    await act("the user-release dialog opened again and cancelled");
    report.reopened_dialog = await reopenUserReleaseDialog();
    // DEL-08-01 AC-010: the next report is local private. Rendered again after the user release.
    await act("render the report again");
    report.second_render = await renderLocalPrivateReport(`${plan.route}/second-render`);
    report.second_render.user_release_line_in_panel = text("report-package-user-release-status");
    await act("File > Save Report Package…");
    report.local = await saveLocalPackage(plan.local_package, `${plan.route}/local`);
    if (plan.csv_save) {
      await act("the results table's CSV export");
      report.csv = await saveResultGroupCsv(plan.csv_save);
    }
    await act("File > Save Model Document…");
    report.model_document = await saveModelDocument(plan.model_document, `${plan.route}/model-document`);
    // Save canonical HTML, Copy and Print are not pressed: a person observes them (the runner's header).
  }

  // The results table's CSV export through its own control: a button that saves natively to the
  // app's Downloads folder with no chooser (lib.rs `begin_local_csv_stream`, native_csv_stream.rs).
  // The group's own row count is the one its family button shows; the filter is left empty.
  async function saveResultGroupCsv({ group, family, sentinel_chooser_path: sentinel }) {
    await showSection("results");
    const testId = `result-group-${group}-export-csv`;
    const control = await waitFor(`${testId} control`, () => byTestId(testId), T.step);
    const record = {
      control: testId,
      tag: control.tagName.toLowerCase(),
      href: control.getAttribute("href"),
      download: control.getAttribute("download"),
      filter: byTestId("result-filter-input")?.value ?? null,
      family_count: text(`result-family-count-${family}`),
      filter_summary: text("result-filter-summary")
    };
    // A data: download link would do nothing in this webview; it is not pressed as a save.
    if (record.tag === "a") return { ...record, pressed: false };
    const prior = text(`${testId}-status`);
    // A chooser path queued first: a chooser the press reached would take it (and show nothing);
    // one still queued afterwards was never asked for.
    await call("native_smoke_choose", { path: sentinel });
    control.click();
    record.status = await waitFor(`${testId} saved`, () => {
      const now = text(`${testId}-status`);
      return now && now !== prior && /^(Saved |CSV export did not complete|Downloaded )/.test(now) ? now : null;
    }, T.save);
    record.chooser_paths_unused = await call("native_smoke_clear_choices");
    record.pressed = true;
    return record;
  }

  // ------------------------------------------------------------ observations (recorded, not judged)

  const flat = (value) => String(value ?? "").replace(/\s+/g, " ").trim();
  // Selection by keyboard. No surface offers a select-all or copy of its own (no handler in the
  // desktop source); the keyboard's Select All (Edit > Select All, ⌘A, lib.rs's predefined item) sends
  // `selectAll:` to the webview, which runs WebKit's SelectAll editing command, the command
  // `document.execCommand("selectAll")` runs. The surface is focused first, as Tab would focus it, and
  // the selection's text (what Copy would take) is read; Copy itself is not pressed and the system
  // pasteboard is neither written nor read.
  async function observeSelection() {
    await showSection("results");
    await waitFor("results table rows", () => document.querySelector('[data-testid^="result-row-dual-"]'), T.step);
    // What each surface itself states about rows shown against rows in all, and whether a CSV export
    // control sits beside it (in its own panel or group), read from the window.
    const csvBeside = (root) => [...(root?.querySelectorAll('[data-testid$="-export-csv"], [data-testid$="-export-csv-all"]') ?? [])].map((item) => item.getAttribute("data-testid"));
    const surfaces = [
      ["model tree", () => document.querySelector('[aria-label="Model"]'), () => ({
        rows_rendered: document.querySelectorAll('[data-testid^="tree-row-"]').length,
        stated: text("model-tree-filter-summary"),
        // The tree's own panel: its filter controls' parent, which also holds the tree.
        csv_beside: csvBeside(byTestId("model-tree-controls")?.parentElement)
      })],
      ["results table", () => document.querySelector('[data-testid^="result-row-dual-"]')?.closest("table"), () => ({
        rows_rendered: document.querySelectorAll('tr[data-testid^="result-row-"]').length,
        stated: [text("result-filter-summary"), text("result-page-summary")].filter(Boolean).join(" | "),
        csv_beside: csvBeside(document.querySelector('[data-testid^="result-row-dual-"]')?.closest("section.result-group"))
      })],
      ["displacement extremes table", () => byTestId("displacement-extremes-table"), () => ({
        rows_rendered: byTestId("displacement-extremes-table")?.querySelectorAll("tbody tr").length ?? 0,
        stated: text("displacement-extremes-page"),
        csv_beside: csvBeside(byTestId("displacement-extremes-panel"))
      })],
      ["support load summary", () => byTestId("support-load-summary"), () => ({
        rows_rendered: byTestId("support-load-summary")?.querySelectorAll("tbody tr").length ?? 0,
        stated: text("support-load-position"),
        csv_beside: csvBeside(byTestId("support-load-summary"))
      })]
    ];
    const observed = [];
    for (const [name, find, extent] of surfaces) {
      const surface = find();
      if (!surface) { observed.push({ surface: name, present: false }); continue; }
      const focusable = surface.matches("[tabindex], a[href], button, input, select, textarea") ? surface
        : surface.querySelector('[tabindex]:not([tabindex="-1"]), a[href], button:not([disabled])');
      if (document.activeElement instanceof HTMLElement) document.activeElement.blur();
      if (focusable && !(focusable instanceof HTMLInputElement || focusable instanceof HTMLTextAreaElement || focusable instanceof HTMLSelectElement)) focusable.focus();
      const selection = window.getSelection();
      selection.removeAllRanges();
      const command = document.execCommand("selectAll");
      const selected = selection.toString();
      const shown = surface.innerText;
      // The surface's own text as a selection would give it (a range over just the surface; not a
      // keyboard action, for comparison only).
      const range = document.createRange();
      range.selectNodeContents(surface);
      const surfaceRange = range.toString();
      observed.push({
        surface: name, present: true,
        focused: document.activeElement === focusable ? focusable.tagName.toLowerCase() + (focusable.getAttribute("role") ? `[role=${focusable.getAttribute("role")}]` : "") : null,
        select_all_command: command,
        selection_chars: selected.length,
        selection_contains_surface: selection.containsNode(surface, true),
        selection_includes_shown_text: flat(shown).length > 0 && flat(selected).includes(flat(shown)),
        selection_is_shown_text: flat(selected) === flat(shown),
        surface_range_is_shown_text: flat(surfaceRange) === flat(shown),
        shown_chars: shown.length,
        ...extent()
      });
      selection.removeAllRanges();
    }
    if (document.activeElement instanceof HTMLElement) document.activeElement.blur();
    return observed;
  }

  // ------------------------------------------------------------ autosave

  async function openAppearance() {
    const details = element("toolbar-appearance");
    if (!details.open) details.querySelector("summary").click();
    await waitFor("appearance open", () => byTestId("toolbar-appearance")?.open, T.ui);
  }
  const intervalSelect = () => byTestId("autosave-interval");
  async function chooseInterval(minutes) {
    const select = await waitFor("interval control enabled", () => (intervalSelect() && !intervalSelect().disabled ? intervalSelect() : null), T.step);
    setValue(select, String(minutes));
    await waitFor(`interval ${minutes}`, () => intervalSelect()?.value === String(minutes) && !intervalSelect().disabled, T.step);
  }
  const offNotices = () => ({ off_notice: text("autosave-off-notice"), off_status: text("autosave-off-status") });

  // A node's y through the node grid's review cell, queued and applied: an unsaved edit. `applied`
  // is the session's applied-operation count once this edit lands.
  async function editNode(nodeId, value, applied) {
    const startedAt = Date.now();
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
    const summary = await waitFor(`operation ${applied} applied`, () => new RegExp(`(^|\\D)${applied} applied`).test(text("operation-apply-summary") ?? "") && text("operation-apply-summary"), T.step);
    const edited = await waitFor("session edited", () => /Edited/.test(text("project-edited") ?? "") && text("project-edited"), T.step);
    return { node: nodeId, value, summary, edited, started_at: startedAt, at: Date.now() };
  }

  async function phaseAutosaveEnd(plan, report) {
    await act("the autosave interval: read, turned off, turned on at the shortest choice");
    await openAppearance();
    const select = await waitFor("autosave settings read", () => (intervalSelect() && !intervalSelect().disabled && intervalSelect().value !== "" ? intervalSelect() : null), T.step);
    report.setting_on_start = { value: select.value, options: [...select.options].map((option) => option.value), ...offNotices() };
    await chooseInterval(0);
    report.setting_off = { value: intervalSelect()?.value, ...(await waitFor("off notice", () => (offNotices().off_notice ? offNotices() : null), T.ui)) };
    await chooseInterval(plan.interval_minutes);
    report.setting_on = { value: intervalSelect()?.value, autosave_status: text("autosave-status"), ...offNotices() };
    const before = message();
    await act("File > New Local Project (the demo saved as a local project)");
    await menu("file.new-local");
    report.saved_project = await waitFor("demo saved as a local project", () => {
      const now = message();
      if (now !== before && now.startsWith("Create failed")) throw new Error(now);
      return now !== before && now.includes("Created local SQLite project snapshot") ? now : null;
    });
    report.before_edit_at = Date.now();
    await call("native_smoke_checkpoint", { name: "before-edit", report, storeSnapshot: true });
    await act("first edit (node N-100)");
    report.edit = await editNode(plan.edit.node, plan.edit.y, 1);
    await call("native_smoke_checkpoint", { name: "edited", report, storeSnapshot: false });
    // The product's own report of a record written (its status line), then at once a second edit,
    // within the interval that follows that record. The timer is not restarted by an edit
    // (useAutosave.ts: one setInterval per interval setting), so the next write is one interval after
    // the record's: the runner ends the app before it, and observes that no record of the second edit exists.
    report.first_record_status = await waitFor("a recovery record written", () => /^Autosaved at /.test(text("autosave-status") ?? "") && text("autosave-status"), T.record);
    report.first_record_seen_at = Date.now();
    await act("second edit (node N-110)");
    report.second_edit = await editNode(plan.second_edit.node, plan.second_edit.y, 2);
    await call("native_smoke_checkpoint", { name: "second-edit", report, storeSnapshot: false });
    await act("waiting to be ended");
    // The runner ends this process with SIGKILL before the next interval elapses.
    await new Promise(() => {});
  }

  async function phaseAutosaveReopen(plan, report) {
    await act("the recovery offer, read before any choice");
    const prompt = await waitFor("recovery offer", () => byTestId("autosave-recovery-prompt"), T.step);
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
    // An observation window with the offer open, not a wait for anything: a slow host only shortens
    // what it shows, it cannot make the check pass or fail.
    await sleep(plan.hold_ms);
    report.before_choice_later = { ...windowState(), offer_open: Boolean(byTestId("autosave-recovery-prompt")) };
    const before = message();
    await act("Open autosave");
    click("autosave-recovery-open");
    report.opened = await waitFor("autosave opened", () => {
      const now = message();
      return now !== before && now !== "" ? now : null;
    }, T.step);
    report.after_open = windowState();
    await act("File > Save Model Document…");
    report.model_document = await saveModelDocument(plan.model_document, "autosave/recovered");
  }

  // ------------------------------------------------------------ the reference system (kept apart)

  // The solve job's own record, as its export link carries it (the record a person can save).
  function solveJobResource() {
    try {
      const href = byTestId("solve-job-export-link")?.getAttribute("href") ?? "";
      if (!href.startsWith("data:")) return null;
      return JSON.parse(decodeURIComponent(href.slice(href.indexOf(",") + 1)))?.resource ?? null;
    } catch {
      return null;
    }
  }

  // The viewport's controls whose label names an up direction or a vertical axis, as the window shows them.
  function viewportUpControls() {
    const viewport = document.querySelector('[aria-label="Viewport controls"]')?.closest("section, div.viewport, [data-testid]") ?? document;
    return [...viewport.querySelectorAll("button, select, input, [role='radio'], [role='switch']")]
      .map((item) => ({ label: item.getAttribute("aria-label") ?? item.textContent?.trim() ?? "", value: item.value ?? null, pressed: item.getAttribute("aria-pressed") }))
      .filter((item) => /\bup\b|vertical|z-up|y-up|up axis/i.test(item.label));
  }

  async function phaseReference(plan, report) {
    await act("File > Open Model Document…");
    report.opened = await openModelDocument(plan.open);
    await waitFor("reference system in the model tree", () => treeRow(plan.project_id), T.step);
    await showSection("solve");
    click("solver-mode-sparse");
    await waitFor("sparse selected", () => byTestId("solver-mode-sparse")?.getAttribute("aria-pressed") === "true", T.ui);
    const proofText = () => byTestId("status-pill-solve-proof")?.querySelector("code")?.textContent ?? null;
    const prior = proofText();
    const runAt = Date.now();
    await act("solve (Analyze > Run, sparse)");
    await menu("analyze.run");
    await waitFor("reference system solve", () => {
      const summary = text("solve-job-summary") ?? "";
      if (/\bstate=(failed|cancelled|refused)(?:;|$)/.test(summary)) throw new Error(`reference: ${summary}; ${text("solve-job-error") ?? ""}`);
      const proof = proofText();
      return summary.includes("state=completed") && proof && proof !== prior && proof.includes(`project=${plan.project_id}`);
    }, T.solve);
    const solvedAt = Date.now();
    await showSection("results");
    await waitFor("the results table's first rows", () => document.querySelector('[data-testid^="result-row-dual-"]'), T.step);
    report.solve = {
      summary: text("solve-job-summary"), proof: proofText(), standing: text("numerical-result-standing"),
      run_to_completed_ms: solvedAt - runAt, run_to_first_rows_ms: Date.now() - runAt,
      filter_summary: text("result-filter-summary"),
      // The webview's request is not exposed by the product; the job record states only the bound
      // reserved for the model intake, which is not the request's byte count.
      job_resource: solveJobResource()
    };
    report.up = { controls: viewportUpControls() };
    // What a person meets after the solve, recorded in the window's own words. On main the reference
    // system solves "sensitive" (M03-INTEGRITY-v1) and the desktop reads "needs recompute", so the
    // report outputs may be refused: nothing here expects a package, and nothing is worked around.
    await act("the report outputs as a person meets them (observation)");
    report.met = await observeReportOutputs(plan);
    await act("File > Save Model Document…");
    report.model_document = await saveModelDocument(plan.model_document, "reference/model-document");
  }

  // Sentences in the window that mention recompute or recomputing, bounded.
  function recomputeSentences() {
    const text = document.body.innerText ?? "";
    return [...new Set(text.split(/(?<=[.!?])\s+|\n+/).map((line) => line.trim()).filter((line) => /recompute|recomputing|re-run|run (the )?(solve|mechanics) again/i.test(line)))].slice(0, 20);
  }

  async function observeReportOutputs(plan) {
    const met = { standing: text("numerical-result-standing"), health: text("numerical-health-header-line"), recompute_sentences_results: recomputeSentences() };
    await showSection("report");
    met.recompute_sentences_report = recomputeSentences();
    const panel = await waitFor("rendered report panel", () => byTestId("rendered-report-panel"), T.step);
    const render = byTestId("rendered-report-render", panel);
    met.render = {
      offered: Boolean(render),
      refusal: text("rendered-report-load-reference-output-unavailable"),
      precondition: text("rendered-report-precondition")
    };
    if (render) {
      const intent = byTestId("rendered-report-local-private-intent");
      if (intent && !intent.checked) { intent.click(); await waitFor("report intent", () => byTestId("rendered-report-local-private-intent")?.checked, T.ui); }
      met.render.enabled = !render.disabled;
      met.render.title = render.getAttribute("title");
      if (!render.disabled) {
        render.click();
        await waitFor("render outcome", () => byTestId("rendered-report-hash") || byTestId("rendered-report-error") || byTestId("rendered-report-route") || byTestId("rendered-report-redaction-summary"), T.save);
        await waitFor("render settled", () => !/Rendering/.test(byTestId("rendered-report-render")?.textContent ?? ""), T.save);
        Object.assign(met.render, {
          hash: text("rendered-report-hash"), gate: text("rendered-report-gate"), error: text("rendered-report-error"),
          route: text("rendered-report-route"), redaction: text("rendered-report-redaction-summary"),
          blocking_reasons: text("rendered-report-blocking-reasons")
        });
      }
    }
    met.package = {
      refusal: text("report-package-source-unavailable"),
      controls: Boolean(byTestId("report-package-controls")),
      save_enabled: byTestId("report-package-save") ? !byTestId("report-package-save").disabled : null,
      save_title: byTestId("report-package-save")?.getAttribute("title") ?? null
    };
    // Pressed only when the window offers it, with the chooser path queued; a refusal it then shows is recorded.
    if (met.package.save_enabled && !met.package.refusal) {
      const intent = byTestId("report-package-private-intent");
      if (intent && !intent.checked) { intent.click(); await waitFor("package intent", () => byTestId("report-package-private-intent")?.checked, T.ui); }
      await call("native_smoke_choose", { path: plan.local_package });
      const prior = text("report-package-save-status");
      await menu("file.save-report-package");
      met.package.status = await packageOutcome("reference/package", prior).catch((error) => `not settled: ${error?.message ?? error}`);
      met.package.chooser_paths_unused = await call("native_smoke_clear_choices");
    }
    await showSection("exports");
    met.result_export = {
      link: Boolean(byTestId("result-export-link")),
      panel_text: (byTestId("workspace-section-exports")?.innerText ?? "").split("\n").map((line) => line.trim()).filter((line) => /recompute|not offered|Run mechanics preview|Current|refus/i.test(line)).slice(0, 10)
    };
    met.recompute_sentences_exports = recomputeSentences();
    return met;
  }

  (async () => {
    const report = { phase: null, ok: false, user_agent: navigator.userAgent };
    try {
      const plan = await call("native_smoke_plan");
      report.phase = plan.phase;
      if (plan.timeouts) T = { ...T, ...plan.timeouts };
      await waitFor("desktop shell", () => byTestId("desktop-preview-shell") && treeRow(plan.bundled_project_id), T.step);
      // The shell has loaded: the runner now decides whether the web view's data is under the throwaway
      // home (on the run's first launch it ends the app if not). The app's own listing is recorded too.
      await call("native_smoke_checkpoint", { name: "shell-loaded", report: { phase: plan.phase, at: Date.now() }, storeSnapshot: false, homeListing: true });
      await log(`evidence journey phase ${plan.phase} started`);
      await act("the desktop shell loaded");
      if (plan.phase === "report") await phaseReport(plan, report);
      else if (plan.phase === "autosave-end") await phaseAutosaveEnd(plan, report);
      else if (plan.phase === "autosave-reopen") await phaseAutosaveReopen(plan, report);
      else if (plan.phase === "reference") await phaseReference(plan, report);
      else throw new Error(`unknown phase ${plan.phase}`);
      report.ok = true;
    } catch (error) {
      report.error = `${error?.message ?? error}${error?.stack ? `\n${error.stack}` : ""}`;
      await log(`FAILED: ${report.error}`);
    }
    await call("native_smoke_finish", { report });
  })();
})();
