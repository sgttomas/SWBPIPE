import { expect, type Locator, type Page } from "@playwright/test";

export type TreeEntityType =
  | "project"
  | "material"
  | "section"
  | "node"
  | "pipe"
  | "support"
  | "component"
  | "load"
  | "combination"
  | "diagnostic";

type WorkspaceSectionId =
  | "operations"
  | "loads"
  | "libraries"
  | "rule-packs"
  | "solve"
  | "results"
  | "report"
  | "project"
  | "exports"
  | "evidence";

export async function openWorkspaceSection(page: Page, sectionId: WorkspaceSectionId): Promise<Locator> {
  const section = page.getByTestId(`workspace-section-${sectionId}`);
  if (!await section.isVisible()) {
    // Review changes is a tab of the Model stage's strip; from another stage it is summoned by the
    // section command, as every other section is.
    if (sectionId === "operations" && await page.getByTestId("workspace-review").isVisible()) {
      await page.getByTestId("workspace-review").click();
    } else {
      await page.getByTestId("menu-view").click();
      await page.getByTestId(`menu-item-view.section.${sectionId}`).click();
    }
  }
  await expect(section).toBeVisible();
  if (sectionId === "operations") {
    const review = page.getByTestId("operation-tab-review");
    if (await review.getAttribute("aria-pressed") !== "true") await review.click();
    await expect(review).toHaveAttribute("aria-pressed", "true");
  }
  return section;
}

// Slice B3: the dock is gone. A page closes onto its stage by its close control;
// "no section on screen" is the Model stage's model tree, reached by the rail and the tab.
export async function closeWorkspacePanels(page: Page): Promise<void> {
  const close = page.getByTestId("workspace-dock-close");
  if (await close.isVisible()) await close.click();
  await showModelTree(page);
  await expect(page.locator('[data-testid^="workspace-section-"]:visible')).toHaveCount(0);
}

/** The model tree is the Model stage's first tab. */
export async function showModelTree(page: Page): Promise<void> {
  const host = page.getByTestId("shell-tree-host");
  if (!await host.isVisible()) {
    const close = page.getByTestId("workspace-dock-close");
    if (await close.isVisible()) await close.click();
    if (await page.getByTestId("rail-stage-model").getAttribute("aria-current") !== "page") await page.getByTestId("rail-stage-model").click();
    const tab = page.getByTestId("stage-tab-model-tree");
    if (await tab.getAttribute("aria-pressed") !== "true") await tab.click();
    await ensureRailExpanded(page, "toggle-tree");
  }
  await expect(host).toBeVisible();
}

/** Close the Both view's docked inspector, which takes its 300 px from the canvas. */
export async function ensureInspectorCollapsed(page: Page): Promise<void> {
  const toggle = page.getByTestId("toggle-inspector");
  if (await toggle.getAttribute("aria-disabled") !== "true" && await toggle.getAttribute("aria-expanded") === "true") await toggle.click();
}

export async function ensureTreeExpanded(page: Page): Promise<void> {
  await showModelTree(page);
  await ensureRailExpanded(page, "toggle-tree");
}

// The inspector is docked in Model view, opened by the toolbar's toggle in Both view,
// and absent in Table view: a test that needs it in Table view asks for Both view first.
export async function ensureInspectorExpanded(page: Page): Promise<void> {
  const toggle = page.getByTestId("toggle-inspector");
  if (await toggle.getAttribute("aria-disabled") === "true" && await toggle.getAttribute("aria-expanded") !== "true") {
    await page.getByTestId("view-switch-both").click();
  }
  await ensureRailExpanded(page, "toggle-inspector");
}

async function ensureRailExpanded(page: Page, testId: "toggle-tree" | "toggle-inspector"): Promise<void> {
  const toggle = page.getByTestId(testId);
  if (await toggle.getAttribute("aria-expanded") !== "true") await toggle.click();
  await expect(toggle).toHaveAttribute("aria-expanded", "true");
}

export function treeRowTestId(type: TreeEntityType, id: string): string {
  return `tree-row-${encodeURIComponent(type)}-${encodeURIComponent(id)}`;
}

export function treeEntity(page: Page, type: TreeEntityType, id: string): Locator {
  return page.getByTestId(treeRowTestId(type, id));
}

export async function revealTreeEntity(page: Page, type: TreeEntityType, id: string): Promise<Locator> {
  await ensureTreeExpanded(page);
  const filter = page.getByTestId("model-tree-filter-input");
  await filter.fill(id);
  const row = treeEntity(page, type, id);
  await expect(row).toBeVisible();
  return row;
}

export async function selectTreeEntity(page: Page, type: TreeEntityType, id: string): Promise<Locator> {
  const row = await revealTreeEntity(page, type, id);
  await row.click();
  await expect(row).toHaveAttribute("aria-selected", "true");
  return row;
}

export async function expectTreeEntity(
  page: Page,
  type: TreeEntityType,
  id: string,
  expectedText?: string
): Promise<Locator> {
  const row = await revealTreeEntity(page, type, id);
  if (expectedText) await expect(row).toContainText(expectedText);
  return row;
}

export async function expectTreeEntityMissing(page: Page, type: TreeEntityType, id: string): Promise<void> {
  await ensureTreeExpanded(page);
  await page.getByTestId("model-tree-filter-input").fill(id);
  await expect(treeEntity(page, type, id)).toHaveCount(0);
}

export async function startPropertyTaskFromTreeEntity(
  page: Page,
  type: TreeEntityType,
  id: string
): Promise<Locator> {
  await selectTreeEntity(page, type, id);
  return startPropertyTaskFromCurrentSelection(page, type, id);
}

export async function startPropertyTaskFromCurrentSelection(
  page: Page,
  type: TreeEntityType,
  id: string
): Promise<Locator> {
  await ensureInspectorExpanded(page);
  const inspector = page.getByTestId("property-inspector");
  const taskTab = inspector.getByRole("tab", { name: "Task", exact: true });
  if (await taskTab.getAttribute("aria-selected") !== "true") await taskTab.click();
  await expect(taskTab).toHaveAttribute("aria-selected", "true");
  // This helper requests a fresh task, never inspection of a retained draft.
  const frozenTarget = inspector.getByTestId("inspector-frozen-task-target");
  if (await frozenTarget.isVisible()) {
    const cancel = inspector.getByTestId("cancel-editor-intent");
    await expect(cancel).toBeVisible();
    await expect(cancel).toBeEnabled();
    await cancel.click();
    await expect(frozenTarget).toHaveCount(0);
  }
  await expect(inspector.getByTestId("inspector-task-empty")).toContainText(`Current selection: ${type}: ${id}`);
  const start = inspector.getByTestId("inspector-start-task");
  await expect(start).toBeVisible();
  await expect(start).toBeEnabled();
  await start.click();
  await expect(frozenTarget).toBeVisible();
  await expect(frozenTarget).toContainText(`Draft target: ${type}: ${id}`);
  const panel = inspector.getByTestId("editor-intent-panel");
  await expect(panel).toBeVisible();
  return panel;
}

export async function chooseVirtualTarget(
  scope: Page | Locator,
  testId: string,
  value: string
): Promise<Locator> {
  const picker = scope.getByTestId(testId);
  const search = picker.getByRole("combobox");
  await search.fill(value);
  const option = picker.getByRole("option", { name: new RegExp(`${escapeRegExp(value)}$`) });
  await expect(option).toBeVisible();
  await option.click();
  await expectVirtualTarget(scope, testId, value);
  return picker;
}

export async function chooseVirtualMultiTarget(
  scope: Page | Locator,
  testId: string,
  value: string
): Promise<Locator> {
  const picker = scope.getByTestId(testId);
  const search = picker.getByRole("combobox");
  await search.fill(value);
  const option = picker.getByRole("option", { name: new RegExp(`${escapeRegExp(value)}$`) });
  await expect(option).toBeVisible();
  if (await option.getAttribute("aria-selected") !== "true") await option.click();
  await expect(option).toHaveAttribute("aria-selected", "true");
  return picker;
}

export async function expectVirtualTarget(scope: Page | Locator, testId: string, value: string): Promise<void> {
  await expect(scope.getByTestId(testId).locator(".virtual-target-picker-current")).toContainText(value);
}

function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

// --- Slice B3 helpers -------------------------------------------------------

export type ProjectCommand = "new-local" | "new-blank" | "open-local" | "list-local" | "save-local";

const PROJECT_BUTTON_NAMES: Record<ProjectCommand, string> = {
  "new-local": "Create local",
  "new-blank": "New blank",
  "open-local": "Open local",
  "list-local": "List local",
  "save-local": "Save local"
};

/**
 * The five project buttons live in the Project page's header. This opens that
 * page when it is not the open page and returns the button, found by its role
 * and its unchanged accessible name.
 */
export async function projectButton(page: Page, command: ProjectCommand): Promise<Locator> {
  const button = page.getByRole("button", { name: PROJECT_BUTTON_NAMES[command], exact: true });
  if (!await button.isVisible()) await openWorkspaceSection(page, "project");
  await expect(button).toBeVisible();
  return button;
}

/**
 * The same command from the File menu, which runs it through the same command
 * sink without opening a page over the canvas. `keyboard` drives the menu by
 * focus and Enter, for tests that hold a pointer button down on the canvas.
 */
export async function projectCommand(page: Page, command: ProjectCommand, keyboard = false): Promise<void> {
  const menu = page.getByTestId("menu-file");
  const item = page.getByTestId(`menu-item-file.${command}`);
  if (keyboard) {
    await menu.focus(); await page.keyboard.press("Enter");
    await item.focus(); await page.keyboard.press("Enter");
  } else {
    await menu.click();
    await item.click();
  }
}

/**
 * A status chip of the status bar (specification §5.4): its face is "Domain · Label", the
 * recorded token is its tooltip and, on a click, the monospace line of its popover.
 */
export async function expectStatusChip(page: Page, testId: string, token: string, face: string): Promise<void> {
  const chip = page.getByTestId(testId);
  await expect(chip).toHaveText(face);
  await expect(chip).toHaveAttribute("title", token);
  await chip.click();
  const recorded = page.getByTestId(`${testId}-popover`).locator("code");
  await expect(recorded).toBeVisible();
  await expect(recorded).toHaveText(token);
  await chip.click();
  await expect(page.getByTestId(`${testId}-popover`)).toHaveCount(0);
}

export async function expectNoStatusChip(page: Page, testId: string): Promise<void> {
  await expect(page.getByTestId(testId)).toHaveCount(0);
}

/**
 * A recorded status that the chip policy does not draw stays readable, with its
 * token, in the Analyze page's readiness summary. The page is closed again.
 */
export async function expectRecordedStatusOnAnalyzePage(page: Page, row: "mechanics" | "rule" | "professional", token: string): Promise<void> {
  const wasOpen = await page.getByTestId("workspace-section-solve").isVisible();
  const section = await openWorkspaceSection(page, "solve");
  await expect(section.getByTestId(`readiness-${row}`)).toContainText(`(${token})`);
  if (!wasOpen) await page.getByTestId("workspace-dock-close").click();
}

/**
 * The canvas is drawn in Model and Both view and is covered by an open page. This closes a
 * page and, where the current stage is in Table view, returns to the Model stage (Both view
 * at first open), so that a test can reach the canvas's own controls.
 */
export async function showCanvas(page: Page): Promise<void> {
  const close = page.getByTestId("workspace-dock-close");
  if (await close.isVisible()) await close.click();
  const canvas = page.getByTestId("viewport-canvas");
  if (!await canvas.isVisible()) {
    await page.getByTestId("rail-stage-model").click();
    if (!await canvas.isVisible()) await page.getByTestId("view-switch-both").click();
  }
  await expect(canvas).toBeVisible();
}
