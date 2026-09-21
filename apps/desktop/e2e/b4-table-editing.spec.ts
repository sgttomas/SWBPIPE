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

async function gridChromeBounds(page: import("@playwright/test").Page, review = false) {
  return page.evaluate((review) => {
    const bounds: Record<string, { x: number; y: number; width: number; height: number }> = {};
    const selectors = { pane: ".shell-table-pane", host: ".shell-tree-host", title: ".model-tree > .panel-title", mode: ".layout-mode-toggle", filter: ".model-tree-controls", families: ".entity-grid-tabs", header: review ? ".entity-grid-summary" : ".engineering-table-header", footer: review ? ".entity-grid-actions" : ".engineering-table-footer", viewportHost: '[data-testid="viewport-canvas"]', drawnCanvas: '[data-testid="viewport-canvas"] canvas' };
    for (const [key, selector] of Object.entries(selectors)) {
      const rect = document.querySelector(selector)!.getBoundingClientRect(); bounds[key] = { x: rect.x, y: rect.y, width: rect.width, height: rect.height };
    }
    return bounds;
  }, review);
}

async function openBoundedGrid(page: import("@playwright/test").Page) {
  await page.getByTestId("view-switch-both").click();
  if (await page.getByTestId("toggle-inspector").getAttribute("aria-expanded") !== "true") await page.getByTestId("toggle-inspector").click();
  await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
}

// Wheel input returns before browser delivery. Observe that local event and two
// paints so an unchanged-boundary assertion cannot pass before the input occurs.
async function tableWheel(page: import("@playwright/test").Page, deltaY: number, info: import("@playwright/test").TestInfo, moves = false, scroller = "engineering-table-rows") {
  const observed = await page.locator(".model-tree").evaluateHandle((root, { moves, scroller }) => {
    const rows = root.querySelector<HTMLElement>(`[data-testid="${scroller}"]`)!;
    const state: { complete: boolean; ended: boolean; supported: boolean; result?: unknown; cleanup?: () => void } = { complete: false, ended: false, supported: "onscrollend" in rows };
    const sample = () => {
      const slot = root.querySelector<HTMLElement>(scroller === "engineering-table-rows" ? ".engineering-table-body-slot" : ".bulk-grid-body-slot")!;
      return { top: rows.scrollTop, clientHeight: rows.clientHeight, scrollHeight: rows.scrollHeight, slotHeight: slot.clientHeight,
        filterY: root.querySelector(".model-tree-controls")!.getBoundingClientRect().y,
        familyY: root.querySelector(".entity-grid-tabs")!.getBoundingClientRect().y,
        footerY: root.querySelector(scroller === "engineering-table-rows" ? ".engineering-table-footer" : ".entity-grid-actions")!.getBoundingClientRect().y };
    };
    const ended = () => { state.ended = true; };
    if (moves) rows.addEventListener("scrollend", ended, { once: true });
    state.cleanup = () => rows.removeEventListener("scrollend", ended);
    root.addEventListener("wheel", (event) => {
      const target = (event.target as Element).outerHTML.slice(0, 400);
      requestAnimationFrame(() => { const first = sample(); requestAnimationFrame(() => {
        state.result = { deltaX: (event as WheelEvent).deltaX, deltaY: (event as WheelEvent).deltaY, target, frames: [first, sample()] }; state.complete = true;
      }); });
    }, { once: true, passive: true, capture: true });
    return state;
  }, { moves, scroller });
  try {
    if (moves) expect(await observed.evaluate((state) => state.supported)).toBe(true);
    await page.mouse.wheel(0, deltaY);
    await expect.poll(() => observed.evaluate((state) => state.complete)).toBe(true);
    if (moves) await expect.poll(() => observed.evaluate((state) => state.ended), { message: "scrolling gesture completed" }).toBe(true);
    const result = await observed.evaluate((state) => ({ receipt: state.result, scrollEnded: state.ended }));
    await info.attach(`wheel-${deltaY}-${Date.now()}`, { body: JSON.stringify(result), contentType: "application/json" });
    return result;
  } finally { await observed.evaluate((state) => state.cleanup?.()); await observed.dispose(); }
}

test("B4 short Grid keeps vertical chrome fixed and retains alternate review drafts", async ({ page, browser }, info) => {
  await attachBrowserIdentity(browser, info); await page.goto("/"); await expect(page.getByTestId("workspace-toolbar")).toBeVisible(); await openBoundedGrid(page);
  const table = page.getByTestId("engineering-table"); const rows = page.getByTestId("engineering-table-rows");
  const wheelEvidence: unknown[] = [];
  const before = await gridChromeBounds(page);
  await rows.hover(); wheelEvidence.push(await tableWheel(page, 600, info));
  await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  await page.locator(".entity-grid-tabs").hover(); wheelEvidence.push(await tableWheel(page, 600, info));
  await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  expect(await rows.evaluate((node) => ({ client: node.clientHeight, scroll: node.scrollHeight, top: node.scrollTop }))).toEqual({ client: 180, scroll: 180, top: 0 });
  const cell = page.getByTestId("table-cell-node:N-100-x"); await cell.dblclick(); const editor = table.getByRole("textbox", { name: "node:N-100 X [m]" }); await editor.fill("invalid retained"); await editor.press("Enter");
  await expect(editor).toHaveAttribute("aria-invalid", "true");
  expect(await page.locator(".engineering-table-body-slot").evaluate((node) => node.clientHeight)).toBeGreaterThan(0);
  const errorState = await gridChromeBounds(page); await rows.hover(); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(errorState);
  const toggle = page.getByTestId("node-grid-review-disclosure"); await toggle.click(); await expect(table).toBeHidden(); await expect(page.getByTestId("retained-direct-draft")).toBeVisible();
  await expect(toggle).toContainText("Return to node coordinates"); const bulk = page.getByTestId("entity-grid-input-node:N-100-y"); await bulk.fill("0.5");
  const reviewState = await gridChromeBounds(page, true); await page.locator(".entity-grid-scroll").hover(); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page, true)).toEqual(reviewState);
  await page.getByTestId("entity-grid-type-pipes").click(); await expect(page.getByTestId("entity-grid-table-pipes")).toBeVisible();
  await page.getByTestId("entity-grid-type-nodes").click(); await expect(bulk).toHaveValue("0.5"); await toggle.click(); await expect(table).toBeVisible(); await expect(editor).toHaveValue("invalid retained");
  await expect(toggle).toContainText("1 retained draft"); await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText("0");
  await toggle.click(); await page.getByTestId("clear-entity-grid-drafts").click(); await toggle.click(); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  const controlledId = await toggle.getAttribute("aria-controls"); expect(controlledId).toBeTruthy();
  const controlled = page.locator(`[id="${controlledId}"]`); await expect(controlled).toBeHidden();
  await page.getByTestId("table-cell-node:N-140-z").dblclick(); await page.keyboard.press("Tab");
  await expect(table.getByRole("group", { name: "Node coordinates footer" })).toBeFocused();
  await page.keyboard.press("Tab"); await expect(toggle).toBeFocused();
  await page.keyboard.press("Enter"); await expect(toggle).toHaveAttribute("aria-expanded", "true"); await expect(controlled).toBeVisible(); await expect(table).toBeHidden();
  await page.keyboard.press("Space"); await expect(toggle).toHaveAttribute("aria-expanded", "false"); await expect(controlled).toBeHidden(); await expect(table).toBeVisible();
  await expect(toggle).toHaveAttribute("aria-controls", controlledId!); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  await info.attach("vertical-fixed-rectangles", { body: JSON.stringify({ before, errorState, reviewState, wheelEvidence }, null, 2), contentType: "application/json" });
  await page.screenshot({ path: info.outputPath("b4-short-fixed-chrome.png") });
});

test("B4 virtual Grid confines body scrolling and boundary wheel without moving chrome", async ({ page, browser }, info) => {
  await attachBrowserIdentity(browser, info); const model = await gotoRoutedFixture(page, "ui-foundation-1000.model.json"); await openBoundedGrid(page);
  const wheelEvidence: unknown[] = [];
  const before = await gridChromeBounds(page); const rows = page.getByTestId("engineering-table-rows");
  const scrollState = () => rows.evaluate((node) => ({ top: node.scrollTop, maximum: node.scrollHeight - node.clientHeight, height: node.clientHeight }));
  expect((await scrollState()).height).toBeGreaterThan(0);
  await rows.hover(); wheelEvidence.push(await tableWheel(page, 700, info, true)); await expect.poll(async () => (await scrollState()).top).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  wheelEvidence.push(await tableWheel(page, 1000000, info, true)); await expect.poll(async () => { const state = await scrollState(); return state.maximum - state.top; }).toBe(0);
  wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  wheelEvidence.push(await tableWheel(page, -1000000, info, true));
  // CDP's large reversal can end a few pixels above the boundary. One further
  // real wheel input establishes top; the following separate input tests chaining.
  if ((await scrollState()).top > 0) wheelEvidence.push(await tableWheel(page, -600, info, true));
  await expect.poll(async () => (await scrollState()).top).toBe(0);
  wheelEvidence.push(await tableWheel(page, -600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  await page.locator(".entity-grid-tabs").hover(); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  const filter = page.getByTestId("model-tree-filter-input"); await filter.fill(model.nodes.at(-1).id);
  await expect(rows.locator('[role="row"]')).toHaveCount(1); const filtered = await gridChromeBounds(page); await rows.hover(); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(filtered);
  await filter.fill(""); const restored = await gridChromeBounds(page); await rows.hover(); wheelEvidence.push(await tableWheel(page, 700, info, true)); await expect.poll(async () => (await scrollState()).top).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page)).toEqual(restored);
  // Swap the mounted virtual body through a small family, changing available
  // width while it is absent, then verify the new element owns its observation.
  const review = page.getByTestId("node-grid-review-disclosure"); await review.click();
  const bulkRows = page.getByTestId("entity-grid-virtual-rows"); await expect(bulkRows).toBeVisible();
  await page.getByTestId("entity-grid-type-sections").click(); await expect(bulkRows).toHaveCount(0);
  await page.getByTestId("toggle-inspector").click(); await page.getByTestId("entity-grid-type-nodes").click(); await expect(bulkRows).toBeVisible();
  await expect.poll(() => bulkRows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  const bulkChrome = await gridChromeBounds(page, true); await bulkRows.hover(); wheelEvidence.push(await tableWheel(page, 700, info, true, "entity-grid-virtual-rows"));
  await expect.poll(() => bulkRows.evaluate((node) => node.scrollTop)).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page, true)).toEqual(bulkChrome);
  await review.click(); await expect(rows).toBeVisible(); await expect.poll(() => rows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  // A page makes the still-laid-out stage inert. Positive resize observations
  // must remain current even before interaction is restored.
  const retainedCell = page.getByTestId(`table-cell-${model.nodes[0].id}-x`); await retainedCell.dblclick();
  const retainedEditor = page.getByTestId("engineering-table").getByRole("textbox", { name: `${model.nodes[0].id} X [${model.project.units.length}]` });
  await retainedEditor.fill("retained page draft"); await retainedEditor.press("Enter"); await expect(retainedEditor).toHaveAttribute("aria-invalid", "true");
  const originalViewport = page.viewportSize()!;
  const selectionBeforePage = await page.getByTestId("command-selection-readout").textContent();
  await openWorkspaceSection(page, "libraries");
  await page.setViewportSize({ width: originalViewport.width, height: originalViewport.height + 120 });
  await expect.poll(() => rows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  await page.getByTestId("workspace-dock-close").click(); await expect(rows).toBeVisible();
  await expect.poll(() => rows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  expect(await page.evaluate(() => document.activeElement !== document.body)).toBe(true);
  expect(await page.getByTestId("command-selection-readout").textContent()).toBe(selectionBeforePage);
  await expect(retainedEditor).toHaveValue("retained page draft"); await expect(retainedEditor).toHaveAttribute("aria-invalid", "true");
  await page.setViewportSize(originalViewport); await expect.poll(() => rows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  await expect(retainedEditor).toHaveValue("retained page draft");
  await page.getByTestId("engineering-table").getByRole("button", { name: "Cancel", exact: true }).click(); await expect(retainedCell).toHaveText(String(model.nodes[0].position.x));
  await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  await info.attach("vertical-wheel-steps", { body: JSON.stringify({ before, filtered, restored, bulkChrome, wheelEvidence }, null, 2), contentType: "application/json" });
  await page.screenshot({ path: info.outputPath("b4-virtual-fixed-chrome.png") });
});
