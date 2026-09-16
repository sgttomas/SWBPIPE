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
    if (sectionId === "operations") {
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

export async function closeWorkspacePanels(page: Page): Promise<void> {
  await page.getByTestId("menu-view").click();
  await page.getByTestId("menu-item-view.close-panels").click();
  await expect(page.locator('[data-testid^="workspace-section-"]:visible')).toHaveCount(0);
}

export async function ensureTreeExpanded(page: Page): Promise<void> {
  await ensureRailExpanded(page, "toggle-tree");
}

export async function ensureInspectorExpanded(page: Page): Promise<void> {
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
