import { expect, test, type Page } from "@playwright/test";

async function applyQueued(page: Page) {
  await page.getByTestId("workspace-review").click();
  const apply = page.locator('[data-testid^="apply-intent-"]').first();
  await expect(apply).toBeEnabled();
  await apply.click();
  await expect(page.getByTestId("operation-apply-message")).toContainText("Applied");
}

async function assertPersistentCompactCanvas(page: Page) {
  const geometry = await page.evaluate(() => {
    const canvas = document.querySelector('[data-testid="viewport-canvas"]')?.getBoundingClientRect();
    const card = document.querySelector('[data-testid="viewport-editor-intents"]')?.getBoundingClientRect();
    return { canvas: canvas?.toJSON(), card: card?.toJSON(), width: innerWidth, height: innerHeight, overflow: document.body.scrollWidth - document.body.clientWidth };
  });
  expect(geometry.width).toBe(1024);
  expect(geometry.height).toBe(768);
  expect(geometry.overflow).toBe(0);
  expect(geometry.canvas?.width).toBeGreaterThan(300);
  expect(geometry.canvas?.height).toBeGreaterThan(250);
  expect(geometry.card?.right).toBeLessThanOrEqual(1024);
  expect(geometry.card?.bottom).toBeLessThanOrEqual(768);
}

test("compact blank-to-straight authoring keeps the canvas and exact Add/Apply review", async ({ page }) => {
  await page.setViewportSize({ width: 1024, height: 768 });
  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await expect(page.getByTestId("operation-engine-chip")).toContainText("Engine ready");
  await page.getByRole("button", { name: "New blank" }).click();
  await expect(page.getByTestId("status-pill-mechanics")).toContainText("MODEL_INCOMPLETE");

  await page.getByTestId("command-node").click();
  await page.getByTestId("viewport-create-node-id").fill("node:UI-A-100");
  await page.getByTestId("viewport-create-node-label").fill("Anchor");
  await page.getByTestId("viewport-create-node-x").fill("0");
  await page.getByTestId("viewport-create-node-y").fill("2.4");
  await page.getByTestId("viewport-create-node-z").fill("0");
  await page.getByTestId("viewport-create-node-provenance").fill("synthetic_ui_acceptance_input");
  await assertPersistentCompactCanvas(page);
  await page.getByTestId("queue-explicit-node-intent").click();
  await expect(page.getByTestId("viewport-draft-review-preview")).toContainText("Single operation");
  await page.getByTestId("apply-reviewed-draft").click();
  await expect(page.getByTestId("tree-row-node:UI-A-100")).toBeAttached();
  await page.getByRole("button", { name: "Save local" }).click();
  await page.getByRole("button", { name: "Open local" }).click();
  await expect(page.getByTestId("tree-row-node:UI-A-100")).toBeAttached();
  await expect(page.getByTestId("status-pill-mechanics")).toContainText("MODEL_INCOMPLETE");

  if (await page.getByTestId("toggle-inspector").getAttribute("aria-expanded") !== "true") await page.getByTestId("toggle-inspector").click();
  await page.getByTestId("toolkit-entry").click();
  await page.getByTestId("toolkit-properties.material").click();
  await page.getByTestId("create-material-id").fill("material:ui-phase-a-invented");
  await page.getByTestId("create-material-label").fill("Invented carbon steel");
  await page.getByTestId("create-material-elastic").fill("200000000000");
  await page.getByTestId("create-material-shear").fill("77000000000");
  await page.getByTestId("create-material-provenance").fill("synthetic_ui_acceptance_input");
  await page.getByTestId("queue-create-material-intent").click();
  await applyQueued(page);

  await page.getByTestId("toolkit-entry").click();
  await page.getByTestId("toolkit-properties.section").click();
  await page.getByTestId("create-section-id").fill("section:ui-phase-a-straight");
  await page.getByTestId("create-section-name").fill("Invented straight pipe");
  await page.getByTestId("create-section-od").fill("0.168");
  await page.getByTestId("create-section-wall").fill("0.007");
  await page.getByTestId("create-section-provenance").fill("synthetic_ui_acceptance_input");
  await page.getByTestId("queue-create-section-intent").click();
  await applyQueued(page);

  await page.getByTestId("command-pipe").click();
  await page.getByTestId("viewport-create-pipe-id").fill("pipe:UI-A-100");
  await page.getByTestId("viewport-create-pipe-label").fill("Straight run");
  await page.getByTestId("viewport-create-pipe-from").selectOption("node:UI-A-100");
  await page.getByRole("radio", { name: "New node", exact: true }).check();
  await expect(page.getByTestId("viewport-construction-plane")).toContainText("XZ · Y=2.4 m · through node:UI-A-100");
  await page.getByRole("radio", { name: "X", exact: true }).check();
  await expect(page.getByRole("radio", { name: "Y", exact: true })).toBeDisabled();

  const canvas = page.getByTestId("viewport-canvas").locator("canvas");
  const bounds = await canvas.boundingBox();
  expect(bounds).not.toBeNull();
  const start = { x: bounds!.x + bounds!.width * 0.58, y: bounds!.y + bounds!.height * 0.48 };
  await page.mouse.move(start.x, start.y);
  await expect(page.getByTestId("viewport-route-ghost-status")).toContainText("hover route ghost");

  await page.mouse.move(start.x, start.y);
  await page.mouse.down();
  await page.mouse.move(start.x + 45, start.y + 32, { steps: 5 });
  await page.mouse.up();
  await expect(page.getByTestId("viewport-route-end-x")).toHaveValue("");
  await expect(page.getByTestId("viewport-route-end-y")).toHaveValue("");
  await expect(page.getByTestId("viewport-route-end-z")).toHaveValue("");
  await expect(page.getByTestId("viewport-pointer-placement-status")).toContainText("exceeded 4 CSS pixels");

  await canvas.click({ position: { x: bounds!.width * 0.58, y: bounds!.height * 0.48 } });
  await expect(page.getByTestId("viewport-route-end-id")).toHaveValue("node:V-001");
  await expect(page.getByTestId("viewport-route-end-label")).toHaveValue("Viewport node V-001");
  await expect(page.getByTestId("viewport-route-ghost-status")).toContainText("captured route ghost");
  await expect(page.getByTestId("viewport-route-end-y")).toHaveValue("2.4");

  await page.getByTestId("viewport-route-end-id").fill("node:UI-A-110");
  await page.getByTestId("viewport-route-end-label").fill("Loaded end");
  await page.getByTestId("viewport-route-end-x").fill("3.2");
  await page.getByTestId("viewport-route-end-y").fill("2.4");
  await page.getByTestId("viewport-route-end-z").fill("0");
  await expect(page.getByTestId("viewport-route-ghost-status")).toContainText("No route ghost is visible");
  await page.getByTestId("viewport-route-end-provenance").fill("synthetic_ui_acceptance_input");
  await page.getByTestId("viewport-create-pipe-material").selectOption("material:ui-phase-a-invented");
  await page.getByTestId("viewport-create-pipe-od").fill("0.168");
  await page.getByTestId("viewport-create-pipe-wall").fill("0.007");
  await page.getByTestId("viewport-create-pipe-yref-x").fill("0");
  await page.getByTestId("viewport-create-pipe-yref-y").fill("0");
  await page.getByTestId("viewport-create-pipe-yref-z").fill("1");
  await page.getByTestId("viewport-create-pipe-provenance").fill("synthetic_ui_acceptance_input");
  await page.getByTestId("continue-pipe-after-queue").check();
  await page.getByTestId("queue-explicit-pipe-intent").click();
  const review = page.getByTestId("viewport-draft-review-preview");
  await expect(review).toContainText("Atomic batch");
  await expect(review).toContainText("op:viewport-create-node-node:UI-A-110");
  await expect(review).toContainText("op:viewport-connect-pipe-pipe:UI-A-100");
  await assertPersistentCompactCanvas(page);
  await page.getByTestId("apply-reviewed-draft").click();
  await expect(page.getByTestId("tree-row-node:UI-A-110")).toBeAttached();
  await expect(page.getByTestId("tree-row-pipe:UI-A-100")).toBeAttached();
  await expect(page.getByTestId("session-history-chip")).toContainText("3 undo / 0 redo");
  await expect(page.getByTestId("operation-applied-ledger")).toContainText("Applied through local_wasm_engine");
  await expect(page.getByTestId("viewport-create-pipe-from")).toHaveValue("node:UI-A-110");
  await expect(page.getByRole("radio", { name: "New node", exact: true })).toBeChecked();
  await expect(page.getByTestId("viewport-routing-plane")).toHaveValue("XZ");
  await expect(page.getByRole("radio", { name: "X", exact: true })).toBeChecked();
  await expect(page.getByTestId("viewport-route-end-unit")).toHaveValue("m");
  await expect(page.getByTestId("viewport-construction-plane")).toContainText("XZ · Y=2.4 m · through node:UI-A-110");

  await page.getByRole("radio", { name: "Existing node", exact: true }).check();
  await page.getByTestId("viewport-create-pipe-to").selectOption("node:UI-A-100");
  await expect(page.getByTestId("viewport-route-ghost-status")).toContainText("existing route ghost");
  await expect(page.getByTestId("viewport-routing-aids")).toHaveAttribute("disabled", "");
  await page.getByRole("radio", { name: "New node", exact: true }).check();
  await expect(page.getByTestId("viewport-route-ghost-status")).toContainText("No route ghost is visible");
  await assertPersistentCompactCanvas(page);

  await page.getByTestId("workspace-review").click();
  await page.getByTestId("undo-session-model-edit").click();
  await expect(page.getByTestId("tree-row-node:UI-A-110")).toHaveCount(0);
  await expect(page.getByTestId("tree-row-pipe:UI-A-100")).toHaveCount(0);
  await page.getByTestId("redo-session-model-edit").click();
  await expect(page.getByTestId("tree-row-node:UI-A-110")).toBeAttached();

  if (await page.getByTestId("toggle-tree").getAttribute("aria-expanded") !== "true") await page.getByTestId("toggle-tree").click();
  await page.getByTestId("tree-row-pipe:UI-A-100").click();
  await page.getByTestId("toolkit-entry").click();
  await page.getByTestId("toolkit-properties.assign-section").click();
  await page.getByLabel("Shared section").selectOption("section:ui-phase-a-straight");
  await page.getByRole("button", { name: "Queue section assignment" }).click();
  await applyQueued(page);

  await page.getByTestId("command-support").click();
  await page.getByTestId("create-support-id").fill("support:UI-A-100");
  await page.getByTestId("create-support-label").fill("Anchor support");
  await page.getByTestId("create-support-node").selectOption("node:UI-A-100");
  for (const restraint of ["RX", "RY", "RZ"]) await page.getByTestId(`create-support-restraint-${restraint}`).check();
  await page.getByTestId("create-support-provenance").fill("synthetic_ui_acceptance_input");
  await page.getByTestId("queue-create-support-intent").click();
  await applyQueued(page);

  await page.getByTestId("workspace-task-loads").click();
  await page.getByTestId("load-manager-create-load-id").fill("load:UI-A");
  await page.getByTestId("load-manager-create-load-label").fill("Invented Phase A force");
  await page.getByTestId("load-manager-create-load-kind").fill("primitive_user_load");
  await page.getByTestId("load-manager-create-load-status").fill("preview_only");
  await page.getByTestId("load-manager-create-load-provenance").fill("synthetic_ui_acceptance_input");
  await page.getByTestId("queue-create-load-case-intent").click();
  await applyQueued(page);
  await page.getByTestId("workspace-task-loads").click();
  await page.getByTestId("load-manager-create-primitive-load-case").selectOption("load:UI-A");
  await page.getByTestId("load-manager-create-primitive-category").selectOption("concentrated_force");
  await page.getByTestId("load-manager-create-primitive-id").fill("load:UI-A-FY");
  await page.getByTestId("load-manager-create-primitive-node").selectOption("node:UI-A-110");
  await page.getByTestId("load-manager-create-primitive-direction").selectOption("global_y");
  await page.getByTestId("load-manager-create-primitive-magnitude").fill("350");
  await page.getByTestId("load-manager-create-primitive-provenance").fill("synthetic_ui_acceptance_input");
  await page.getByTestId("queue-create-primitive-intent").click();
  await applyQueued(page);

  await page.getByTestId("workspace-task-solve").click();
  await page.getByTestId("run-mechanics-preview").click();
  await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  await page.getByTestId("issues-drawer-toggle").click();
  await expect(page.getByTestId("diagnostic-BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL")).toContainText("BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL");
  await page.getByTestId("issues-home").getByRole("button", { name: /Close/i }).click();

  await page.getByTestId("tree-row-load:UI-A").click();
  const inspector = page.getByLabel("Property inspector");
  await inspector.getByTestId("editor-intent-field").selectOption("primitive_loads.0.magnitude.value");
  await inspector.getByTestId("editor-intent-value").fill("500");
  await inspector.getByTestId("apply-editor-intent-inline").click();
  await expect(page.getByTestId("operation-apply-message")).toContainText("Applied op:editor-intent-load:UI-A-primitive_loads.0.magnitude.value");
  await page.getByTestId("workspace-review").click();
  await page.getByTestId("undo-session-model-edit").click();
  await inspector.getByTestId("editor-intent-field").selectOption("primitive_loads.0.magnitude.value");
  await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("350");
  await page.getByTestId("redo-session-model-edit").click();
  await inspector.getByTestId("editor-intent-field").selectOption("primitive_loads.0.magnitude.value");
  await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("500");
  await page.getByRole("button", { name: "Save local" }).click();
  await page.getByRole("button", { name: "Open local" }).click();
  await expect(page.getByTestId("tree-row-pipe:UI-A-100")).toBeAttached();
  await page.getByTestId("tree-row-load:UI-A").click();
  await inspector.getByTestId("editor-intent-field").selectOption("primitive_loads.0.magnitude.value");
  await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("500");
});
