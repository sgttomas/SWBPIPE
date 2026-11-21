// The screenshot walkthrough: drives the desktop as a person would, by its own
// visible controls, and takes a numbered picture at each step. It is evidence
// of what a person sees, not a test: nothing here asserts that a screen is
// right. A step whose control is missing or disabled is recorded as the place a
// person stops, and the journey's later steps are recorded as not reached.
//
// The journeys follow DEL-07-06 AC-007's use cases as far as the browser build
// reaches. The browser build cannot solve an edited model, so a journey that
// needs a solve stops at the refusal; README.md says what that leaves unseen.
import { expect, test, type Page } from "@playwright/test";
import {
  confirmProjectUnits,
  ensureInspectorExpanded,
  openWorkspaceSection,
  projectCommand,
  selectTreeEntity,
  showCanvas,
  showModelTree
} from "../e2e/workspace-driver";
import { Journey } from "./recorder";

const INVENTED = "invented_walkthrough_input";
// The parts of the screen a person reads when they are not the tallest scrolling region.
const ISSUES = '[data-testid="issues-home"]';
const INSPECTOR = "#shell-inspector";

async function open(page: Page): Promise<void> {
  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await expect(page.getByTestId("viewport-canvas").locator("canvas")).toHaveCount(1);
}

async function menu(page: Page, menuId: string, itemId?: string): Promise<void> {
  await page.getByTestId(`menu-${menuId}`).click();
  await expect(page.getByTestId(`menu-dropdown-${menuId}`)).toBeVisible();
  if (!itemId) return;
  const item = page.getByTestId(`menu-item-${itemId}`);
  if (await item.isDisabled()) {
    const reason = (await item.getAttribute("title")) ?? "no reason given";
    await page.keyboard.press("Escape");
    throw new Error(`the menu item ${itemId} is disabled: ${reason}`);
  }
  await item.click();
}

async function closeMenu(page: Page): Promise<void> {
  const backdrop = page.getByTestId("app-menu-backdrop");
  if (await backdrop.count()) await backdrop.click({ position: { x: 5, y: 600 } });
}

/** Closes whatever lies over the workspace, so each journey step starts from the workspace itself. */
async function clearOverlays(page: Page): Promise<void> {
  await closeMenu(page);
  for (const testId of ["key-results-close"]) {
    const control = page.getByTestId(testId);
    if (await control.isVisible()) await control.click();
  }
  for (const drawer of ["issues-home"]) {
    const close = page.getByTestId(drawer).getByRole("button", { name: "Close", exact: true });
    if (await close.isVisible()) await close.click();
  }
}

async function newBlankProject(page: Page): Promise<void> {
  await projectCommand(page, "new-blank");
  await confirmProjectUnits(page);
}

async function applyReviewedDraft(page: Page): Promise<void> {
  const apply = page.getByTestId("apply-reviewed-draft");
  await expect(apply).toBeEnabled();
  await apply.click();
}

test.describe.configure({ mode: "serial" });

test("01 orientation: every menu, stage and page as first met", async ({ page }) => {
  const journey = new Journey(page, 1, "orientation", "Orientation: what the application shows before any work is done");
  await open(page);
  await journey.step("first-open", "The application as first opened, with its demonstration model");

  for (const id of ["file", "edit", "view", "insert", "analyze"]) {
    await journey.step(`menu-${id}`, `The ${id} menu opened`, async () => { await menu(page, id); });
    await closeMenu(page);
  }

  await journey.step("toolbar-appearance", "The toolbar's appearance control opened", async () => {
    await page.getByTestId("toolbar-appearance").locator("summary").click();
  });
  await page.getByTestId("toolbar-appearance").locator("summary").click();

  await journey.step("command-search", "The toolbar's search and command box opened", async () => {
    await page.getByTestId("toolkit-entry").click();
  });
  await page.keyboard.press("Escape");

  await journey.step("agent-control", "The agent strip's Agent control: Review changes opened on its Agent tab", async () => {
    await page.getByTestId("agent-strip-open").click();
  });

  for (const stage of ["model", "loads", "results", "review"]) {
    // A disabled stage is pictured with its stated reason; it does not end the tour.
    await journey.step(`stage-${stage}`, `The ${stage} stage, chosen from the rail`, async () => {
      const control = page.getByTestId(`rail-stage-${stage}`);
      if (await control.isDisabled() || await control.getAttribute("aria-disabled") === "true") {
        await control.hover({ force: true });
        return `disabled, "${(await control.getAttribute("title")) ?? "no reason given"}"`;
      }
      await control.click();
    });
  }

  // A dialog in each theme: the units dialog of File > New Blank Project, with the theme set as a
  // person sets it (View > Dark, View > Light) and the page's colour scheme emulated to match.
  for (const theme of ["dark", "light"] as const) {
    await journey.step(`dialog-${theme}`, `The units dialog of File > New Blank Project in the ${theme} theme`, async () => {
      await page.emulateMedia({ colorScheme: theme });
      await menu(page, "view", `view.theme.${theme}`);
      await projectCommand(page, "new-blank");
      await expect(page.getByTestId("project-units-dialog")).toBeVisible();
    });
    const cancel = page.getByTestId("project-units-cancel");
    if (await cancel.isVisible()) await cancel.click();
  }
  await menu(page, "view", "view.theme.system");
  await page.emulateMedia({ colorScheme: null });
  journey.finish();
});

test("02 pages: each page of the workspace, top to bottom", async ({ page }) => {
  const journey = new Journey(page, 2, "pages", "Pages: each page, reached from the rail or the View menu, with what lies below the fold");
  await open(page);
  const pages: ReadonlyArray<[Parameters<typeof openWorkspaceSection>[1], string]> = [
    ["operations", "Review changes"], ["loads", "Load Cases"], ["libraries", "Libraries"], ["rule-packs", "Rules"],
    ["solve", "Analyze"], ["results", "Results"], ["report", "Report"], ["project", "Project"],
    ["exports", "Exports"], ["evidence", "Evidence"]
  ];
  for (const [id, label] of pages) {
    // Review changes and Load Cases are reached by the rail and the stage tab; the View menu does not list them.
    const route = id === "operations" || id === "loads" ? "the rail and its tab" : "the View menu";
    await journey.step(`page-${id}`, `The ${label} page, opened from ${route}`, async () => { await openWorkspaceSection(page, id); });
    await journey.scrolled(`page-${id}`, `The ${label} page`);
  }
  await showModelTree(page);
  for (const tab of ["bulk", "geometry", "supports", "weight", "agent", "records", "details"]) {
    await journey.step(`review-tab-${tab}`, `Review changes, its ${tab} tab`, async () => {
      await openWorkspaceSection(page, "operations");
      await page.getByTestId(`operation-tab-${tab}`).click();
    });
  }
  await showModelTree(page);
  await journey.step("drawer-issues", "The Issues drawer, from the status bar", async () => { await page.getByTestId("issues-drawer-toggle").click(); });
  await journey.scrolled("drawer-issues", "The Issues drawer", 2, ISSUES);
  await clearOverlays(page);
  await journey.step("key-results", "Key results, from the View menu, before any solve", async () => { await menu(page, "view", "view.key-results"); });
  journey.finish();
});

test("03 blank to released report (DEL-07-06 AC-007)", async ({ page }) => {
  const journey = new Journey(page, 3, "blank-to-report", "Blank to released report: units, a pipe with supports, a load case, a solve, results and a report");
  await open(page);
  await journey.step("new-blank-units", "File > New Blank Project: the units dialog", async () => { await projectCommand(page, "new-blank"); });
  await journey.step("blank-project", "The blank project, units accepted", async () => { await confirmProjectUnits(page); });
  await journey.step("node-armed", "Insert > Node armed", async () => { await page.getByTestId("command-node").click(); });
  await journey.step("node-entered", "The first node's values entered", async () => {
    await page.getByTestId("viewport-create-node-id").fill("node:W-100");
    await page.getByTestId("viewport-create-node-label").fill("Anchor");
    await page.getByTestId("viewport-create-node-x").fill("0");
    await page.getByTestId("viewport-create-node-y").fill("2.4");
    await page.getByTestId("viewport-create-node-z").fill("0");
    await page.getByTestId("viewport-create-node-provenance").fill(INVENTED);
  });
  await journey.step("node-queued", "The node sent for review", async () => { await page.getByTestId("queue-explicit-node-intent").click(); });
  await journey.step("node-applied", "The node applied to the model", async () => { await applyReviewedDraft(page); });
  await journey.step("pipe-armed", "Insert > Pipe Run armed", async () => { await page.getByTestId("command-pipe").click(); });
  await journey.scrolled("pipe-armed", "The pipe run form", 2, INSPECTOR);
  await journey.step("support-armed", "Insert > Support armed", async () => { await page.getByTestId("command-support").click(); });
  await journey.scrolled("support-armed", "The support form", 2, INSPECTOR);
  await journey.step("component-armed", "Insert > Component armed", async () => { await page.getByTestId("command-component").click(); });
  await journey.step("load-armed", "Insert > Load Case armed", async () => { await page.getByTestId("command-load").click(); });
  await journey.step("loads-stage", "The Loads stage on the blank project", async () => { await page.getByTestId("rail-stage-loads").click(); });
  await journey.scrolled("loads-stage", "The Loads stage on the blank project", 3);
  await journey.step("run", "Run pressed on the unfinished model", async () => { await showCanvas(page); await page.getByTestId("toolbar-run").click(); });
  await journey.step("run-outcome", "What the application says after Run", async () => { await openWorkspaceSection(page, "solve"); });
  await journey.step("issues-after-run", "The Issues drawer after Run", async () => { await page.getByTestId("issues-drawer-toggle").click(); });
  await clearOverlays(page);
  await journey.step("results-after-run", "Results after Run", async () => { await openWorkspaceSection(page, "results"); });
  await journey.step("report-after-run", "The Report page after Run", async () => { await openWorkspaceSection(page, "report"); });
  journey.finish();
});

test("04 edit a model: a node, a support and a load on the demonstration model", async ({ page }) => {
  const journey = new Journey(page, 4, "edit-model", "Editing: change a value on a node, a support and a load case, then undo");
  await open(page);
  await journey.step("tree", "The model tree", async () => { await showModelTree(page); });
  await journey.step("select-node", "A node selected, with the inspector", async () => {
    await selectTreeEntity(page, "node", "node:N-110");
    await ensureInspectorExpanded(page);
  });
  await journey.step("node-task", "The inspector's Task tab for the node", async () => {
    await page.getByTestId("property-inspector").getByRole("tab", { name: "Task", exact: true }).click();
  });
  await journey.step("node-task-started", "An edit started on the node", async () => { await page.getByTestId("inspector-start-task").click(); });
  await journey.scrolled("node-task-started", "The node edit", 2, INSPECTOR);
  await journey.step("select-support", "A support selected, with the inspector", async () => {
    const cancel = page.getByTestId("cancel-editor-intent");
    if (await cancel.isVisible()) await cancel.click();
    await selectTreeEntity(page, "support", "support:S-120");
    await page.getByTestId("property-inspector").getByRole("tab", { name: "Properties", exact: true }).click();
  });
  await journey.scrolled("select-support", "The support's properties", 2, INSPECTOR);
  await journey.step("select-load", "A load case selected, with the inspector", async () => { await selectTreeEntity(page, "load", "load:L-100"); });
  await journey.step("grid", "The model as a grid", async () => { await page.getByRole("button", { name: "Grid", exact: true }).first().click(); });
  await journey.step("table-view", "Table view", async () => { await page.getByTestId("view-switch-table").click(); });
  await journey.step("model-view", "Model view", async () => { await page.getByTestId("view-switch-model").click(); });
  await journey.step("viewport-selection", "The viewport's selection and navigation controls opened", async () => {
    await page.getByText("Selection & navigation", { exact: true }).click();
  });
  await journey.step("review-changes", "Review changes, with nothing queued", async () => {
    await page.getByTestId("view-switch-both").click();
    await openWorkspaceSection(page, "operations");
  });
  journey.finish();
});

test("05 loads: define cases and combinations (DEL-07-06 AC-007, every required analysis)", async ({ page }) => {
  const journey = new Journey(page, 5, "loads", "Loads: the page an engineer uses to define cases, loads and combinations");
  await open(page);
  await journey.step("loads-stage", "The Loads stage on the demonstration model", async () => { await page.getByTestId("rail-stage-loads").click(); });
  await journey.scrolled("loads-stage", "The Loads stage", 2);
  const loads = page.getByTestId("load-case-manager");
  const caseRows = loads.getByTestId("load-case-manager-cases").locator('button[data-testid^="load-manager-case-"]');
  for (const testId of await caseRows.evaluateAll((rows) => rows.map((row) => row.getAttribute("data-testid") ?? ""))) {
    const id = testId.replace("load-manager-case-", "");
    await journey.step(`case-${id.replace(/[^A-Za-z0-9-]+/g, "-")}`, `The case ${id} chosen in the list`, async () => {
      await loads.getByTestId(testId).click();
      await loads.getByTestId("load-manager-selected-case").scrollIntoViewIfNeeded();
    });
  }
  await journey.step("load-edit", "A load row chosen: its magnitude edit and delete open in place", async () => {
    await caseRows.first().click();
    await loads.locator('[data-testid^="load-manager-primitive-"]').first().click();
    await loads.getByTestId("load-manager-selected-primitive").scrollIntoViewIfNeeded();
  });
  await journey.step("add-load", "Add load, with the selected case already chosen", async () => {
    await loads.getByTestId("load-manager-add-load").click();
    await loads.getByTestId("load-manager-add-load-form").scrollIntoViewIfNeeded();
  });
  await journey.step("pressure-and-conditions", "The case's Pressure and Conditions sections opened", async () => {
    await loads.getByTestId("load-manager-add-load-cancel").click();
    for (const section of ["load-case-section-pressure", "load-case-section-conditions"]) {
      const details = loads.getByTestId(section);
      if (!(await details.evaluate((element) => (element as HTMLDetailsElement).open))) await details.locator(":scope > summary").click();
    }
    await loads.getByTestId("load-case-section-pressure").scrollIntoViewIfNeeded();
  });
  await journey.scrolled("pressure-and-conditions", "The Pressure and Conditions sections", 2);
  await journey.step("combination", "A combination chosen: its terms, basis and delete", async () => {
    await loads.locator('[data-testid^="load-manager-combination-select-"]').first().click();
    await loads.getByTestId("load-manager-selected-combination").scrollIntoViewIfNeeded();
  });
  await journey.step("new-case", "New case", async () => { await loads.getByTestId("load-manager-new-case").click(); });
  await journey.step("new-combination", "New combination", async () => {
    await loads.getByTestId("load-manager-create-load-cancel").click();
    await loads.getByTestId("load-manager-new-combination").click();
  });
  await journey.step("templates", "The Templates command opened", async () => {
    await loads.getByTestId("load-manager-create-combination-cancel").click();
    await loads.getByTestId("load-manager-templates").click();
  });
  await journey.step("seismic-template", "The three-axis static seismic template", async () => {
    await loads.getByTestId("load-manager-template-seismic").click();
  });
  await journey.scrolled("seismic-template", "The seismic template", 2);
  await journey.step("analysis-settings", "Analysis settings opened at the foot of the page", async () => {
    await loads.getByTestId("load-manager-seismic-template-cancel").click();
    const settings = loads.getByTestId("load-manager-analysis-settings");
    await settings.locator(":scope > summary").click();
    await settings.scrollIntoViewIfNeeded();
  });
  journey.finish();
});

test("06 solve and read results (DEL-07-06 AC-007, review and export results)", async ({ page }) => {
  const journey = new Journey(page, 6, "solve-and-results", "Solve and results: run, then find the largest displacement and support load");
  await open(page);
  await journey.step("run", "Run pressed on the demonstration model", async () => { await page.getByTestId("toolbar-run").click(); });
  await journey.step("analyze-page", "The Analyze page after Run", async () => { await openWorkspaceSection(page, "solve"); });
  await journey.scrolled("analyze-page", "The Analyze page");
  await journey.step("bundled-reference", "The bundled reference results, the only results the browser build shows", async () => {
    await page.getByTestId("workspace-section-solve").getByRole("button", { name: "Inspect bundled reference", exact: true }).click();
    await expect(page.getByTestId("historical-run-context")).toBeVisible();
  });
  await journey.scrolled("bundled-reference", "The bundled reference results", 6);
  await journey.step("key-results", "Key results over the results", async () => { await page.getByTestId("key-results-open").click(); });
  await journey.step("key-results-over-insert", "Key results with the insert controls behind it", async () => { await page.getByTestId("command-bar").hover({ force: true }); });
  await clearOverlays(page);
  await journey.step("check-model", "Analyze > Check Model", async () => { await menu(page, "analyze", "analyze.check-model"); });
  await journey.scrolled("check-model", "Check Model", 2, ISSUES);
  await clearOverlays(page);
  await journey.step("rule-checks", "Analyze > Rule Checks", async () => { await menu(page, "analyze", "analyze.rule-checks"); });
  await journey.step("new-blank-after-results", "A new blank project after viewing results: what stays behind", async () => { await newBlankProject(page); });
  journey.finish();
});

test("07 user rule pack (DEL-07-06 AC-007)", async ({ page }) => {
  const journey = new Journey(page, 7, "rule-pack", "Rule pack: bring a private rule pack into the local store and apply it");
  await open(page);
  await journey.step("rules-page", "The Rules page", async () => { await page.getByTestId("rail-page-rules").click(); });
  await journey.step("new-draft", "A new rule pack draft", async () => { await page.getByTestId("rule-pack-new-draft").click(); });
  await journey.scrolled("new-draft", "The rule pack draft", 4);
  await journey.step("libraries-page", "The Libraries page", async () => { await page.getByTestId("rail-page-libraries").click(); });
  await journey.step("library-sample", "Libraries with the invented sample loaded", async () => { await page.getByTestId("library-load-template").click(); });
  await journey.scrolled("library-sample", "The library sample");
  journey.finish();
});

test("08 save, close and resume (DEL-07-06 AC-007)", async ({ page }) => {
  const journey = new Journey(page, 8, "save-and-resume", "Save and resume: save the project, list it, open it again, and its saved states");
  await open(page);
  await journey.step("file-menu", "The File menu", async () => { await menu(page, "file"); });
  await journey.step("saved", "File > Save Local Project", async () => { await page.getByTestId("menu-item-file.save-local").click(); });
  await journey.step("project-page", "The Project page after saving", async () => { await openWorkspaceSection(page, "project"); });
  await journey.scrolled("project-page", "The Project page", 4);
  await journey.step("list-local", "File > List Local Projects", async () => { await menu(page, "file", "file.list-local"); });
  await journey.step("reopened", "The application opened again in the same browser", async () => { await open(page); });
  await journey.step("open-local", "File > Open Local Project", async () => { await menu(page, "file", "file.open-local"); });
  journey.finish();
});

const PROJECT = "project:invented-demo-loop-01";

/** Applies the first queued change or batch from Review changes, and returns to the model. */
async function applyFirstQueued(page: Page): Promise<void> {
  await openWorkspaceSection(page, "operations");
  const apply = page.locator('[data-testid^="apply-intent-"], [data-testid^="apply-batch-"]').first();
  await expect(apply).toBeEnabled();
  await apply.click();
  await showModelTree(page);
}

async function clearTreeFilter(page: Page): Promise<void> {
  const filter = page.getByTestId("model-tree-filter-input");
  if (await filter.isVisible() && await filter.inputValue()) await filter.fill("");
}

/** Selects an entity in the 3D view by its own label there, as a person clicks it. */
async function selectInView(page: Page, id: string): Promise<string | void> {
  await showCanvas(page);
  const target = page.getByTestId(`viewport-select-${id}`);
  if (await target.isVisible()) { await target.click(); return; }
  // A label the view hides where labels overlap is still the entity's target; it is pressed by its event.
  await target.dispatchEvent("click");
  return "the entity's label in the view was hidden by an overlapping label, so it was pressed by its event";
}

/** A selection, and the inspector scrolled to its end. */
async function pictureSelection(journey: Journey, step: string, note: string, select: () => Promise<void | string>): Promise<void> {
  await journey.step(step, note, select);
  await journey.scrolled(step, note, 6, INSPECTOR);
}

test("09 inspector: what the inspector shows for each kind of selection", async ({ page }) => {
  const journey = new Journey(page, 9, "inspector", "Inspector: each kind of selection on the demonstration model, the inspector to its end, and the tree after a selection in the view");
  await open(page);
  await showModelTree(page);
  await ensureInspectorExpanded(page);
  const pick = (type: Parameters<typeof selectTreeEntity>[1], id: string) => async () => { await selectTreeEntity(page, type, id); };
  await pictureSelection(journey, "project", "The project selected in the tree", pick("project", PROJECT));
  await pictureSelection(journey, "node-corner", "A node at a corner (node:N-120, two pipes at a right angle, supports on it)", pick("node", "node:N-120"));
  await pictureSelection(journey, "node-corner-bend-marker", "A node at a corner with a bend marker (node:N-110)", pick("node", "node:N-110"));
  await pictureSelection(journey, "node-end", "A node at the end of the run (node:N-100, one pipe: not a corner)", pick("node", "node:N-100"));
  await pictureSelection(journey, "pipe", "A pipe selected (pipe:P-110)", pick("pipe", "pipe:P-110"));
  await pictureSelection(journey, "support", "A support selected (support:S-120, a guide)", pick("support", "support:S-120"));
  await pictureSelection(journey, "component", "A component selected (component:C-130, a valve)", pick("component", "component:C-130"));
  await pictureSelection(journey, "component-bend", "A bend marker selected (component:C-110)", pick("component", "component:C-110"));
  await pictureSelection(journey, "load", "A load case selected (load:L-100)", pick("load", "load:L-100"));
  await pictureSelection(journey, "material", "A material selected", pick("material", "material:invented-carbon-steel"));
  await pictureSelection(journey, "several", "Several things selected: two nodes and a support, by Ctrl or Cmd and a click in the tree", async () => {
    await clearTreeFilter(page);
    await page.getByTestId("tree-row-node-node%3AN-100").click();
    await page.getByTestId("tree-row-node-node%3AN-110").click({ modifiers: ["ControlOrMeta"] });
    await page.getByTestId("tree-row-support-support%3AS-120").click({ modifiers: ["ControlOrMeta"] });
  });
  await journey.step("tree-after-view-selection", "The tree after a selection in the 3D view: node:N-100 chosen in the tree, then support:S-130 clicked in the view", async () => {
    await clearTreeFilter(page);
    await page.getByTestId("tree-row-node-node%3AN-100").click();
    return selectInView(page, "support:S-130");
  });
  journey.finish();
});

test("10 inspector actions: creation forms, the bend at a corner, settings, the tree following the view, the panes", async ({ page }) => {
  const journey = new Journey(page, 10, "inspector-actions", "Inspector actions: a creation form from its command, the bend at a corner, settings, the tree following the view, the inspector closed, the dark theme");
  await open(page);
  await showModelTree(page);
  await ensureInspectorExpanded(page);
  await journey.step("insert-support", "Insert > Support with node:N-130 selected: the support form", async () => {
    await selectTreeEntity(page, "node", "node:N-130");
    await menu(page, "insert", "insert.support");
    await expect(page.getByTestId("create-support-id")).toBeVisible();
  });
  await journey.scrolled("insert-support", "The support form", 3, INSPECTOR);
  await journey.step("palette-section", "Create section, chosen from the toolbar's command box", async () => {
    await page.getByTestId("toolkit-entry").click();
    await page.getByTestId("toolkit-properties.section").click();
    await expect(page.getByTestId("create-section-id")).toBeVisible();
  });
  await journey.step("palette-material", "Create material, chosen from the toolbar's command box", async () => {
    await page.getByTestId("toolkit-entry").click();
    await page.getByTestId("toolkit-properties.material").click();
    await expect(page.getByTestId("create-material-id")).toBeVisible();
  });
  await journey.step("palette-component", "Components, chosen from the toolbar's command box", async () => {
    await page.getByTestId("toolkit-entry").click();
    await page.getByTestId("toolkit-build.component").click();
    await expect(page.getByTestId("create-component-kind")).toBeVisible();
  });
  await journey.step("bend-on-corner", "node:N-110, a corner: its bend line opened", async () => {
    await showModelTree(page);
    await selectTreeEntity(page, "node", "node:N-110");
    const line = page.getByTestId("insert-bend-at-corner-details");
    if (await line.count() === 0) return "there is no closed line for the bend; the form stands open";
    if (!(await line.evaluate((element) => (element as HTMLDetailsElement).open))) await line.locator(":scope > summary").click();
    await page.getByTestId("insert-bend-at-corner-form").scrollIntoViewIfNeeded();
  });
  await journey.scrolled("bend-on-corner", "The bend at a corner", 2, INSPECTOR);
  await journey.step("bend-absent-end", "node:N-100, the end of the run: no bend offered", async () => {
    await selectTreeEntity(page, "node", "node:N-100");
  });
  await journey.step("settings-closed", "The project selected: its settings lines closed", async () => {
    await selectTreeEntity(page, "project", PROJECT);
    for (const id of ["model-settings-details", "project-units-details"]) {
      const details = page.getByTestId(id);
      if (await details.evaluate((element) => (element as HTMLDetailsElement).open)) await details.locator(":scope > summary").click();
    }
  });
  await journey.step("settings-open", "The project's Model settings line opened", async () => {
    const details = page.getByTestId("model-settings-details");
    if (!(await details.evaluate((element) => (element as HTMLDetailsElement).open))) await details.locator(":scope > summary").click();
    await details.scrollIntoViewIfNeeded();
  });
  await journey.scrolled("settings-open", "Model settings", 2, INSPECTOR);
  await journey.step("tree-group-closed-view-selection", "The Supports group closed in the tree, then support:S-130 clicked in the view", async () => {
    await clearTreeFilter(page);
    const group = page.getByTestId("tree-group-Supports");
    if (await group.getAttribute("aria-expanded") === "true") await group.click();
    return selectInView(page, "support:S-130");
  });
  await journey.step("tree-filtered-view-selection", "The tree filtered to pipes, then support:S-120 clicked in the view", async () => {
    await page.getByTestId("model-tree-filter-input").fill("pipe");
    return selectInView(page, "support:S-120");
  });
  await journey.step("inspector-closed-model-view", "The Model view, then the toolbar's Inspector control pressed", async () => {
    await clearTreeFilter(page);
    await page.getByTestId("view-switch-model").click();
    const toggle = page.getByTestId("toggle-inspector");
    if (await toggle.isDisabled() || await toggle.getAttribute("aria-disabled") === "true") {
      return `the Inspector control is disabled, "${(await toggle.getAttribute("title")) ?? "no reason given"}"`;
    }
    await toggle.click();
  });
  await journey.step("both-view-tree", "The Both view, with the way to the tree and table", async () => {
    await page.getByTestId("view-switch-both").click();
    const toggle = page.getByTestId("toggle-inspector");
    if (await toggle.getAttribute("aria-expanded") !== "true" && !(await toggle.isDisabled())) await toggle.click();
  });
  await journey.step("dark-theme", "A pipe selected in the dark theme", async () => {
    await page.emulateMedia({ colorScheme: "dark" });
    await menu(page, "view", "view.theme.dark");
    await showModelTree(page);
    await ensureInspectorExpanded(page);
    await selectTreeEntity(page, "pipe", "pipe:P-120");
  });
  await menu(page, "view", "view.theme.system");
  await page.emulateMedia({ colorScheme: null });
  await journey.step("split-straight", "pipe:P-120 split in two from its menu in the tree, and applied: a node on a straight run", async () => {
    await clearTreeFilter(page);
    const row = page.getByTestId("tree-row-pipe-pipe%3AP-120");
    await row.click({ button: "right" });
    await page.getByRole("menuitem", { name: "Split into N…" }).click();
    const panel = page.getByTestId("split-into-parts");
    await expect(panel).toBeVisible();
    for (const [label, value] of [["Number of equal parts", "2"], ["New node ID pattern", "node:N-125"], ["New node label pattern", "Mid run"],
      ["New pipe ID pattern", "pipe:P-125"], ["New pipe label pattern", "Second half"], ["First number", "1"], ["Number step", "1"], ["Provenance", INVENTED]] as const) {
      const field = panel.getByLabel(label, { exact: true });
      if (await field.count()) await field.fill(value);
    }
    await panel.getByTestId("split-parts-queue").click();
    await applyFirstQueued(page);
  });
  await pictureSelection(journey, "node-straight", "The new node on the straight run selected: no bend offered", async () => {
    await showModelTree(page);
    await ensureInspectorExpanded(page);
    const filter = page.getByTestId("model-tree-filter-input");
    await filter.fill("node:N-125");
    const row = page.locator('[data-testid^="tree-row-node-node%3AN-125"]').first();
    await row.click();
  });
  await pictureSelection(journey, "section", "A section created from Create section and applied, then selected", async () => {
    await page.getByTestId("toolkit-entry").click();
    await page.getByTestId("toolkit-properties.section").click();
    await page.getByTestId("create-section-id").fill("section:WALK-1");
    await page.getByTestId("create-section-name").fill("Walkthrough section");
    await page.getByTestId("create-section-od").fill("0.168");
    await page.getByTestId("create-section-wall").fill("0.0071");
    await page.getByTestId("create-section-provenance").fill(INVENTED);
    await page.getByTestId("queue-create-section-intent").click();
    await applyFirstQueued(page);
    await selectTreeEntity(page, "section", "section:WALK-1");
  });
  journey.finish();
});
