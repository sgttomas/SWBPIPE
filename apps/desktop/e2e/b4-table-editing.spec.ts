import { expect, test } from "@playwright/test";
import { attachBrowserIdentity, currentModelHashThroughVisibleExport, gotoRoutedFixture } from "./ui-foundation-workflows";
import { ensureTreeExpanded, openWorkspaceSection, showModelTree } from "./workspace-driver";

// One connected journey per configured source viewport; no explicit-size repetition.
test("B4 node coordinates apply through one operation with keyboard, history and saved-state ownership", async ({ page, browser }, info) => {
  await attachBrowserIdentity(browser, info);
  await page.goto("/"); await expect(page.getByTestId("workspace-toolbar")).toBeVisible();
  await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
  const table = page.getByTestId("engineering-table");
  const cell = page.getByTestId("table-cell-node:N-100-y");
  await cell.click(); await expect(table.getByRole("button", { name: "Apply", exact: true })).toHaveCount(0);
  await cell.click(); const editor = table.getByRole("textbox", { name: "node:N-100 Y [m]" });
  await editor.fill("invalid"); await table.getByRole("button", { name: "Apply", exact: true }).click();
  await expect(editor).toHaveValue("invalid"); await expect(editor).toHaveAttribute("aria-invalid", "true");
  await page.getByTestId("entity-grid-type-pipes").click(); await page.getByTestId("entity-grid-type-nodes").click();
  await expect(editor).toHaveValue("invalid"); await table.getByRole("button", { name: "Cancel", exact: true }).click();
  await expect(cell).toHaveText("0");
  // A valid draft must also cancel without blur submitting an operation first.
  await cell.dblclick(); await editor.fill("4.6"); await table.getByRole("button", { name: "Cancel", exact: true }).click();
  await expect(cell).toHaveText("0"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  await expect(page.getByTestId("project-edited")).toHaveCount(0);
  await cell.dblclick(); await editor.fill("0.5"); await table.getByRole("button", { name: "Apply", exact: true }).click();
  await expect(cell).toHaveText("0.5"); await expect(page.getByTestId("project-edited")).toBeVisible();
  await page.getByTestId("workspace-undo").click(); await expect(cell).toHaveText("0"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  await page.getByTestId("workspace-redo").click(); await expect(cell).toHaveText("0.5");
  await table.getByRole("button", { name: "Sort X", exact: true }).click(); await table.getByRole("button", { name: "Sort X", exact: true }).click();
  await cell.focus(); await page.keyboard.press("Enter"); await editor.fill("0.75"); await page.keyboard.press("Enter");
  await expect(cell).toHaveText("0.75");
  await expect(page.getByTestId("table-cell-node:N-110-y")).toBeFocused();
  await page.keyboard.press("Tab"); await expect(page.getByTestId("table-cell-node:N-110-z")).toBeFocused();
  await page.keyboard.press("2"); await page.keyboard.press("Escape"); await expect(page.getByTestId("table-cell-node:N-110-z")).toHaveText("0");
  // The boundaries must leave the coordinate control, including blur-driven no-op Apply.
  await table.getByRole("button", { name: /Sorted by/ }).click();
  const first = table.locator("[data-table-cell]").first(); await first.focus(); await page.keyboard.press("Shift+Tab"); await expect(first).not.toBeFocused();
  const last = table.locator("[data-table-cell]").last(); const lastLabel = await last.getAttribute("aria-label");
  await last.focus(); await page.keyboard.press("Tab"); await expect(last).not.toBeFocused();
  await last.dblclick(); const lastEditor = table.getByRole("textbox"); await page.keyboard.press("Tab");
  await expect(lastEditor).toHaveCount(0); await expect(last).not.toBeFocused();
  expect(await page.evaluate(() => document.activeElement !== document.body)).toBe(true);
  expect(lastLabel).toBeTruthy();
  await openWorkspaceSection(page, "project"); await page.getByRole("button", { name: "Save local", exact: true }).click();
  await expect(page.getByTestId("local-project-message")).toContainText("Saved local browser-preview project"); await expect(page.getByTestId("project-edited")).toHaveCount(0);
  await page.getByRole("button", { name: "Open local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Opened local browser-preview project");
  await showModelTree(page); await page.getByTestId("layout-mode-grid").click(); await expect(cell).toHaveText("0.75"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  await page.screenshot({ path: info.outputPath("b4-reopened-coordinate.png") });
});


test("B4 virtualized invalid editor survives scrolling and a filter threshold without changing the model", async ({ page, browser }, info) => {
  await attachBrowserIdentity(browser, info);
  const model = await gotoRoutedFixture(page, "ui-foundation-1000.model.json");
  const hashBefore = await currentModelHashThroughVisibleExport(page);
  await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
  const table = page.getByTestId("engineering-table");
  const first = model.nodes[0]; const last = model.nodes.at(-1);
  const cell = page.getByTestId(`table-cell-${first.id}-x`);
  await cell.dblclick(); const editor = table.getByRole("textbox", { name: `${first.id} X [${model.project.units.length}]` });
  await editor.fill("retained invalid coordinate"); await editor.press("Enter");
  await expect(editor).toHaveAttribute("aria-invalid", "true"); await expect(editor).toBeFocused();
  const rows = page.getByTestId("engineering-table-rows"); await rows.hover(); await page.mouse.wheel(0, 2400);
  await expect.poll(() => rows.evaluate((element) => element.scrollTop)).toBeGreaterThan(1000);
  await expect(editor).toHaveValue("retained invalid coordinate"); await expect(editor).toBeFocused();
  const filter = page.getByTestId("model-tree-filter-input"); await filter.fill(last.id);
  await expect(table.getByText("Editing row retained outside the filter.")).toBeVisible();
  await expect(editor).toHaveValue("retained invalid coordinate"); await expect(filter).toBeFocused();
  await expect(rows.locator('[role="row"]')).toHaveCount(2);
  await filter.fill(""); await expect(editor).toHaveValue("retained invalid coordinate");
  await table.getByRole("button", { name: "Cancel", exact: true }).click();
  await expect(cell).toBeFocused(); await expect(cell).toHaveText(String(first.position.x));
  // A filtered-out remembered cell must leave a real keyboard entry in the view.
  await filter.fill(last.id);
  await table.getByRole("rowheader").getByRole("button", { name: last.id, exact: true }).click();
  await page.keyboard.press("Tab");
  await expect(page.getByTestId(`table-cell-${last.id}-x`)).toBeFocused();
  expect(await currentModelHashThroughVisibleExport(page)).toBe(hashBefore);
  await page.screenshot({ path: info.outputPath("b4-virtual-editor-cancelled.png") });
});

test("B4 Both with Inspector keeps pointer horizontal scrolling inside the coordinate table", async ({ page, browser }, info) => {
  await attachBrowserIdentity(browser, info);
  await page.goto("/"); await expect(page.getByTestId("workspace-toolbar")).toBeVisible();
  await page.getByTestId("view-switch-both").click();
  if (await page.getByTestId("toggle-inspector").getAttribute("aria-expanded") !== "true") await page.getByTestId("toggle-inspector").click();
  await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
  const table = page.getByTestId("engineering-table"); const grid = table.getByRole("grid", { name: "Node coordinates" });
  const measure = () => page.evaluate(() => {
    const state: Record<string, { x: number; y: number; width: number; height: number; right: number; clientWidth: number; scrollWidth: number; scrollLeft: number }> = {};
    for (const [name, selector] of Object.entries({ grid: '.engineering-table [role="grid"]', model: '.model-tree', pane: '.shell-table-pane', canvas: '[data-testid="viewport-canvas"]', filter: '[data-testid="model-tree-filter-input"]', tabs: '.entity-grid-tabs', footer: '.engineering-table-footer', header: '.engineering-table-header', body: '[data-testid="engineering-table-rows"]' })) {
      const element = document.querySelector(selector)!; const rect = element.getBoundingClientRect();
      state[name] = { x: rect.x, y: rect.y, width: rect.width, height: rect.height, right: rect.right, clientWidth: element.clientWidth, scrollWidth: element.scrollWidth, scrollLeft: element.scrollLeft };
    }
    return state;
  });
  const before = await measure();
  await grid.hover({ position: { x: 150, y: 100 } }); await page.mouse.wheel(700, 0);
  await expect.poll(async () => (await measure()).grid.scrollLeft).toBeGreaterThan(0);
  const after = await measure();
  await info.attach("coordinate-scroll-widths", { body: JSON.stringify({ viewport: page.viewportSize(), before, after }, null, 2), contentType: "application/json" });
  expect(after.grid.scrollWidth).toBeGreaterThan(after.grid.clientWidth);
  expect(after.model.scrollLeft).toBe(0);
  for (const name of ["filter", "tabs", "footer", "pane", "canvas"]) {
    expect(after[name].x).toBe(before[name].x); expect(after[name].width).toBe(before[name].width);
  }
  for (const name of ["pane", "canvas"]) {
    expect(after[name].y).toBe(before[name].y); expect(after[name].height).toBe(before[name].height);
  }
  for (const name of ["filter", "tabs", "footer"]) {
    expect(after[name].x).toBeGreaterThanOrEqual(after.pane.x); expect(after[name].right).toBeLessThanOrEqual(after.pane.right);
  }
  expect(after.header.width).toBe(after.body.width);
  const z = page.getByTestId("table-cell-node:N-100-z");
  const headerZ = await grid.getByRole("columnheader").last().boundingBox(); const bodyZ = await z.locator("..").boundingBox();
  expect(bodyZ!.x).toBeCloseTo(headerZ!.x, 1); expect(bodyZ!.width).toBeCloseTo(headerZ!.width, 1);
  await z.dblclick(); const editor = table.getByRole("textbox", { name: "node:N-100 Z [m]" }); await editor.fill("4.6");
  await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(z).toHaveText("0");
  await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  await page.getByTestId("entity-grid-type-pipes").click(); await page.getByTestId("entity-grid-type-nodes").click();
  await expect(grid).toBeVisible();
  await page.screenshot({ path: info.outputPath("b4-inspector-pointer-fit.png") });
});
