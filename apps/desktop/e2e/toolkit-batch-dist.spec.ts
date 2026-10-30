import { selectCompactOption } from "./workspace-driver";
import { expect, test } from "@playwright/test";
import {
  expectTreeEntity,
  expectTreeEntityMissing,
  openWorkspaceSection,
  selectTreeEntity,
  startPropertyTaskFromCurrentSelection,
} from "./workspace-driver";

// Exercise both published Wasm artifacts through actual browser imports and the
// built application. Input values here are explicit invented test data.
test("dist self-weight plan previews, applies atomically and restores one batch with one undo", async ({ page }) => {
  const errors: string[] = [];
  const artifacts: string[] = [];
  page.on("pageerror", error => errors.push(error.message));
  page.on("response", response => {
    if (response.url().endsWith(".wasm") && response.ok()) artifacts.push(response.url());
  });
  await page.goto("/");
  await openWorkspaceSection(page, "operations");
  await expect(page.getByTestId("operation-engine-chip")).toContainText("Engine ready");
  await page.getByTestId("toolkit-entry").click();
  await page.getByTestId("toolkit-view.select").click();
  await selectTreeEntity(page, "pipe", "pipe:P-100");
  await startPropertyTaskFromCurrentSelection(page, "pipe", "pipe:P-100");
  await selectCompactOption(page.getByTestId("editor-intent-field"), "section.material_density.value");
  await page.getByTestId("editor-intent-value").fill("7800");
  await page.getByTestId("editor-intent-unit").fill("kg/m^3");
  await page.getByTestId("queue-editor-intent").click();
  await page.getByTestId("toolkit-entry").click();
  await page.getByTestId("toolkit-review.pending").click();
  await page.getByTestId("apply-intent-editor-intent-1").click();
  await expect(page.getByTestId("operation-apply-summary")).toContainText("1 applied");
  await page.getByTestId("toolkit-entry").click();
  await page.getByTestId("toolkit-loads.self-weight").click();
  const panel = page.locator("#self-weight-plan");
  await expect(panel).toBeFocused();
  for (const [label, value] of [
    ["Self-weight case ID", "load:dist-weight"], ["Self-weight case label", "Invented dist weight"],
    ["Gravity value", "-9.81"], ["Gravity unit", "m/s^2"], ["Self-weight provenance", "invented browser regression"]
  ]) await panel.getByLabel(label, { exact: true }).fill(value);
  await panel.getByLabel("Gravity direction").selectOption("global_y");
  await panel.getByRole("button", { name: "Use selected pipes" }).click();
  await expect(panel.getByText("Frozen selected-pipe snapshot: pipe:P-100", { exact: true })).toBeVisible();
  await panel.getByRole("button", { name: "Generate self-weight plan" }).click();
  await expect(panel.getByText("2 proposed changes for 1 selected pipes.")).toBeVisible();
  await panel.getByRole("button", { name: "Queue complete self-weight plan" }).click();
  await page.getByTestId("validate-batch-operation-batch-1").click();
  await expect(page.getByText(/Preview only. Temporary state was discarded/)).toBeVisible();
  await expect(page.getByTestId("batch-review-summary")).toContainText("0 batches applied");
  await page.getByTestId("apply-batch-operation-batch-1").click();
  await expect(page.getByTestId("batch-review-summary")).toContainText("1 batches applied");
  await expectTreeEntity(page, "load", "load:dist-weight");
  // Slice B3: Undo of the session's model edit lives on the Model stage's Review changes tab;
  // showing the tree (above) left that tab, so return to it, as the source lane does.
  await openWorkspaceSection(page, "operations");
  await page.getByTestId("undo-session-model-edit").click();
  await expectTreeEntityMissing(page, "load", "load:dist-weight");
  await selectTreeEntity(page, "pipe", "pipe:P-100");
  // End the retained draft and inspect a fresh task's model basis after Undo.
  const inspector = page.getByTestId("property-inspector");
  await inspector.getByRole("tab", { name: "Task", exact: true }).click();
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText("Draft target: pipe: pipe:P-100");
  await inspector.getByRole("button", { name: "Cancel", exact: true }).click();
  await startPropertyTaskFromCurrentSelection(page, "pipe", "pipe:P-100");
  await selectCompactOption(page.getByTestId("editor-intent-field"), "section.material_density.value");
  await expect(page.getByTestId("editor-intent-value")).toHaveValue("7800");
  expect(artifacts.some(url => url.includes("open_pipe_stress_operation_applier_bg.wasm"))).toBe(true);
  expect(artifacts.some(url => url.includes("open_pipe_stress_self_weight_wasm_bg.wasm"))).toBe(true);
  expect(errors).toEqual([]);
});
