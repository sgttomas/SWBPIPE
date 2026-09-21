import { expect, test } from "@playwright/test";
import { attachBrowserIdentity } from "./ui-foundation-workflows";
import { ensureTreeExpanded, openWorkspaceSection } from "./workspace-driver";

test("canonical edit/save/Undo/Redo/reopen marker follows the persisted snapshot", async ({ page, browser }, info) => {
  await attachBrowserIdentity(browser, info);
  await page.goto("/");
  await expect(page.getByTestId("workspace-toolbar")).toBeVisible();
  await expect(page.getByTestId("project-edited")).toHaveCount(0);
  await ensureTreeExpanded(page);
  await page.getByTestId("layout-mode-grid").click();
  await openNodeGridReview(page);
  await reviewFill(page, "node:N-100-y", "0.5");
  await page.getByTestId("queue-entity-grid-intents").click();
  await page.getByTestId("apply-intent-editor-intent-1").click();
  await expect(page.getByTestId("operation-apply-summary")).toContainText("1 applied");
  await expect(page.getByTestId("project-edited")).toBeVisible();
  await expect(page.getByLabel("Unsaved model edits")).toBeVisible();
  await openWorkspaceSection(page, "project");
  await page.getByRole("button", { name: "Save local", exact: true }).click();
  await expect(page.getByTestId("local-project-message")).toContainText("Saved local browser-preview project");
  await expect(page.getByTestId("project-edited")).toHaveCount(0);
  await page.getByTestId("workspace-undo").focus();
  await page.keyboard.press("Enter");
  await expect(page.getByTestId("project-edited")).toBeVisible();
  await page.getByTestId("workspace-redo").focus();
  await page.keyboard.press("Enter");
  await expect(page.getByTestId("project-edited")).toHaveCount(0);
  await page.getByRole("button", { name: "Open local", exact: true }).click();
  await expect(page.getByTestId("local-project-message")).toContainText("Opened local browser-preview project");
  await expect(page.getByTestId("project-edited")).toHaveCount(0);
  await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  await ensureTreeExpanded(page);
  await page.getByTestId("layout-mode-grid").click();
  await openNodeGridReview(page);
  await expect(page.getByTestId("review-cell-node:N-100-y")).toHaveText("0.5");
  await page.screenshot({ path: info.outputPath("reopened-clean.png") });
});

// Preserve this journey's reviewed multi-cell operation setup.
async function openNodeGridReview(page: import("@playwright/test").Page) {
  const summary = page.getByTestId("node-grid-review-disclosure");
  if (await summary.getAttribute("aria-expanded") !== "true") {
    await summary.click();
  }
}

async function reviewFill(page: import("@playwright/test").Page, cell: string, value: string) {
  const table = page.getByTestId("engineering-table-review");
  const keep = table.getByRole("button", { name: "Keep draft", exact: true });
  if (await keep.count()) await keep.click();
  await table.getByTestId(`review-cell-${cell}`).dblclick();
  await table.getByRole("textbox").fill(value);
}
