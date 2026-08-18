import { expect, test, type Page } from "@playwright/test";

async function openWorkspaceSection(page: Page, sectionId: string): Promise<void> {
  const section = page.getByTestId(`workspace-section-${sectionId}`);
  if (await section.isVisible()) return;
  await page.getByTestId("menu-view").click();
  await page.getByTestId(`menu-item-view.section.${sectionId}`).click();
  await expect(section).toBeVisible();
}

async function ensureRailExpanded(page: Page, testId: "toggle-tree" | "toggle-inspector"): Promise<void> {
  const toggle = page.getByTestId(testId);
  if ((await toggle.getAttribute("aria-expanded")) !== "true") await toggle.click();
  await expect(toggle).toHaveAttribute("aria-expanded", "true");
}

test("DEL-09-04 invented saved-project load edit survives reopen and exposes honest solve state", async ({ page }) => {
  const externalRequests: string[] = [];
  page.on("request", (request) => {
    const url = new URL(request.url());
    if (!["127.0.0.1", "localhost"].includes(url.hostname) && !["data:", "blob:"].includes(url.protocol)) {
      externalRequests.push(request.url());
    }
  });

  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await expect(page.getByTestId("operation-engine-chip")).toContainText("Engine ready");
  await ensureRailExpanded(page, "toggle-tree");
  await ensureRailExpanded(page, "toggle-inspector");

  // Establish the live invented fixture's initial result state before editing.
  await openWorkspaceSection(page, "solve");
  await page.getByTestId("run-mechanics-preview").click();
  await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  await expect(page.getByTestId("solve-job-summary")).toContainText("result_rows=830");
  await expect(page.getByTestId("status-pill-mechanics")).toContainText("MECHANICS_SOLVED");

  // Restore the visible authoring layout before using the inspector. Keeping
  // Solve open lets its dock intercept compact-viewport pointer events.
  await page.getByTestId("workspace-dock-close").click();
  await expect(page.getByTestId("workspace-dock")).toHaveClass(/collapsed/);
  await expect(page.getByTestId("workspace-section-solve")).toBeHidden();
  const inspectorBounds = await page.locator(".workspace-pane-inspector").boundingBox();
  expect(inspectorBounds?.width).toBeGreaterThan(300);

  // Edit explicit invented load data through the visible inspector and apply it
  // through the product's local WASM operation engine.
  await page.getByTestId("tree-row-load:L-100").click();
  const editor = page.getByTestId("editor-intent-panel");
  await editor.getByTestId("editor-intent-field").selectOption("primitive_loads.0.magnitude.value");
  await expect(editor.getByTestId("editor-intent-unit")).toHaveValue("N/m");
  await editor.getByTestId("editor-intent-value").fill("-225");
  await expect(editor.getByTestId("editor-operation-preview")).toContainText(
    'after={"value":-225,"unit":"N/m"}'
  );
  await editor.getByTestId("queue-editor-intent").click();
  await openWorkspaceSection(page, "operations");
  await page.getByTestId("apply-intent-editor-intent-1").click();
  const receipt = page.getByTestId("applied-operation-route-applied-1-editor-intent-1");
  await expect(receipt).toContainText("Applied through local_wasm_engine");
  await expect(receipt).toContainText("professional approval not recorded");
  await expect(page.getByTestId("solve-job-summary")).toContainText("state=not_started");

  // Save, list, and reopen by stable project id through normal visible controls.
  await page.getByRole("button", { name: "Save local" }).click();
  await expect(page.getByTestId("local-project-message")).toContainText(
    "Saved local browser-preview project snapshot without external file copies."
  );
  await page.getByRole("button", { name: "List local" }).click();
  await expect(page.getByTestId("local-project-message")).toContainText(
    "Listed 1 local project snapshot from the local store index."
  );
  await page.getByTestId("project-index-open-project:invented-loop-01").click();
  await expect(page.getByTestId("local-project-message")).toContainText(
    "Opened local browser-preview project snapshot by id project:invented-loop-01."
  );

  // The edited unit-bearing value is still the current value after reopen.
  await page.getByTestId("tree-row-load:L-100").click();
  await editor.getByTestId("editor-intent-field").selectOption("primitive_loads.0.magnitude.value");
  await expect(editor.getByTestId("editor-intent-value")).toHaveValue("-225");
  await expect(editor.getByTestId("editor-intent-unit")).toHaveValue("N/m");

  // Browser Playwright intentionally has no native solver fallback for an
  // edited model. Validate the user-visible blocking state instead of allowing
  // stale fixture results to masquerade as a successful re-solve.
  await openWorkspaceSection(page, "solve");
  await page.getByTestId("run-mechanics-preview").click();
  await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  await expect(page.getByTestId("solve-job-summary")).toContainText("result_rows=0");
  await expect(page.getByTestId("status-pill-mechanics")).toContainText("MODEL_INCOMPLETE");
  await page.getByTestId("issues-drawer-toggle").click();
  await expect(page.getByTestId("diagnostic-BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL")).toBeVisible();
  await expect(page.getByTestId("viewport-deformation-status")).toContainText(
    "blocked; mechanics=model incomplete; rows=0"
  );
  const issuesDrawer = page.getByTestId("issues-home");
  await issuesDrawer.getByRole("button", { name: /Close/i }).click();
  await expect(issuesDrawer).toHaveCount(0);

  await page.getByTestId("audit-drawer-toggle").click();
  await expect(page.getByTestId("audit-boundary-drawer")).toBeVisible();
  await expect(page.getByTestId("local-project-status")).toContainText("network=false");
  await expect(page.getByTestId("local-project-status")).toContainText("telemetry=false");
  expect(externalRequests).toEqual([]);
});
