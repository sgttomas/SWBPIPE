import { expect, test } from "@playwright/test";

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
  await expect(page.getByTestId("operation-engine-chip")).toContainText("Engine ready");
  await page.getByTestId("toolkit-entry").click();
  await page.getByTestId("toolkit-view.select").click();
  await page.getByTestId("tree-row-pipe:P-100").click();
  if (await page.getByTestId("toggle-inspector").getAttribute("aria-expanded") === "false") await page.getByTestId("toggle-inspector").click();
  await page.getByTestId("editor-intent-field").selectOption("section.material_density.value");
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
  await panel.getByLabel("pipe:P-100", { exact: true }).check();
  await panel.getByRole("button", { name: "Generate self-weight plan" }).click();
  await expect(panel.getByText("2 proposed changes for 1 selected pipes.")).toBeVisible();
  await panel.getByRole("button", { name: "Queue complete self-weight plan" }).click();
  await page.getByTestId("validate-batch-operation-batch-1").click();
  await expect(page.getByText(/Preview only. Temporary state was discarded/)).toBeVisible();
  await expect(page.getByTestId("batch-review-summary")).toContainText("0 batches applied");
  await page.getByTestId("apply-batch-operation-batch-1").click();
  await expect(page.getByTestId("batch-review-summary")).toContainText("1 batches applied");
  await expect(page.getByTestId("tree-row-load:dist-weight")).toBeVisible();
  await page.getByTestId("undo-session-model-edit").click();
  await expect(page.getByTestId("tree-row-load:dist-weight")).toHaveCount(0);
  await page.getByTestId("tree-row-pipe:P-100").click();
  await page.getByTestId("editor-intent-field").selectOption("section.material_density.value");
  await expect(page.getByTestId("editor-intent-value")).toHaveValue("7800");
  expect(artifacts.some(url => url.includes("open_pipe_stress_operation_applier_bg.wasm"))).toBe(true);
  expect(artifacts.some(url => url.includes("open_pipe_stress_self_weight_wasm_bg.wasm"))).toBe(true);
  expect(errors).toEqual([]);
});
