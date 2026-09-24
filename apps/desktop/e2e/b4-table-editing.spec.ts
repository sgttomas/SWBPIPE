import { expect, test } from "@playwright/test";
import { attachBrowserIdentity, currentModelHashThroughVisibleExport, gotoModel, readFixture, gotoRoutedFixture } from "./ui-foundation-workflows";
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
  const rows = page.getByTestId("engineering-table-rows"); await hoverTableBody(page, rows); await page.mouse.wheel(0, 2400);
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
  await expect(page.getByTestId(`table-cell-${last.id}-label`)).toBeFocused();
  expect(await currentModelHashThroughVisibleExport(page)).toBe(hashBefore);
  await page.screenshot({ path: info.outputPath("b4-virtual-editor-cancelled.png") });
});

test("B4 Both with Inspector keeps pointer horizontal scrolling inside the coordinate table", async ({ page, browser }, info) => {
  await attachBrowserIdentity(browser, info);
  await page.goto("/"); await expect(page.getByTestId("workspace-toolbar")).toBeVisible();
  await page.getByTestId("view-switch-both").click();
  if (await page.getByTestId("toggle-inspector").getAttribute("aria-expanded") !== "true") await page.getByTestId("toggle-inspector").click();
  await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
  const table = page.getByTestId("engineering-table"); const grid = table.getByRole("grid", { name: "Node fields" });
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
  const headerZ = await grid.getByRole("button", { name: "Sort Z", exact: true }).locator("..").boundingBox(); const bodyZ = await z.locator("..").boundingBox();
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
      const slot = rows.parentElement!;
      return { top: rows.scrollTop, clientHeight: rows.clientHeight, scrollHeight: rows.scrollHeight, slotHeight: slot.clientHeight,
        filterY: root.querySelector(".model-tree-controls")!.getBoundingClientRect().y,
        familyY: root.querySelector(".entity-grid-tabs")!.getBoundingClientRect().y,
        footerY: root.querySelector(scroller === "engineering-table-rows" ? ".direct-coordinate-workarea .engineering-table-footer" : ".entity-grid-actions")!.getBoundingClientRect().y };
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
  await hoverTableBody(page, rows); wheelEvidence.push(await tableWheel(page, 600, info));
  await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  await page.locator(".entity-grid-tabs").hover(); wheelEvidence.push(await tableWheel(page, 600, info));
  await expect.poll(() => gridChromeBounds(page)).toEqual(before);
  expect(await rows.evaluate((node) => ({ client: node.clientHeight, scroll: node.scrollHeight, top: node.scrollTop }))).toEqual({ client: 180, scroll: 180, top: 0 });
  const cell = page.getByTestId("table-cell-node:N-100-x"); await cell.dblclick(); const editor = table.getByRole("textbox", { name: "node:N-100 X [m]" }); await editor.fill("invalid retained"); await editor.press("Enter");
  await expect(editor).toHaveAttribute("aria-invalid", "true");
  expect(await page.getByTestId("engineering-table").locator(".engineering-table-body-slot").evaluate((node) => node.clientHeight)).toBeGreaterThan(0);
  const errorState = await gridChromeBounds(page); await hoverTableBody(page, rows); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(errorState);
  const toggle = page.getByTestId("node-grid-review-disclosure"); await toggle.click(); await expect(table).toBeHidden(); await expect(page.getByTestId("retained-direct-draft")).toBeVisible();
  await expect(toggle).toContainText("Return to node fields"); const bulk = page.getByTestId("review-cell-node:N-100-y"); await bulk.dblclick(); await page.getByTestId("engineering-table-review").getByRole("textbox").fill("0.5"); await page.getByRole("button", { name: "Keep draft", exact: true }).click();
  const reviewState = await gridChromeBounds(page, true); await hoverTableBody(page, page.getByTestId("engineering-table-review-rows")); wheelEvidence.push(await tableWheel(page, 600, info, false, "engineering-table-review-rows")); await expect.poll(() => gridChromeBounds(page, true)).toEqual(reviewState);
  await page.getByTestId("entity-grid-type-pipes").click(); await expect(page.getByTestId("entity-grid-table-pipes")).toBeVisible();
  await page.getByTestId("entity-grid-type-nodes").click(); await expect(bulk).toHaveText("0.5"); await toggle.click(); await expect(table).toBeVisible(); await expect(editor).toHaveValue("invalid retained");
  await expect(toggle).toContainText("1 retained draft"); await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText("0");
  await toggle.click(); await page.getByTestId("clear-entity-grid-drafts").click(); await toggle.click(); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  const controlledId = await toggle.getAttribute("aria-controls"); expect(controlledId).toBeTruthy();
  const controlled = page.locator(`[id="${controlledId}"]`); await expect(controlled).toBeHidden();
  await page.getByTestId("table-cell-node:N-140-provenance").dblclick(); await page.keyboard.press("Tab");
  await expect(table.getByRole("group", { name: "Node fields footer" })).toBeFocused();
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
  await hoverTableBody(page, rows); wheelEvidence.push(await tableWheel(page, 700, info, true)); await expect.poll(async () => (await scrollState()).top).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page)).toEqual(before);
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
  await expect(rows.locator('[role="row"]')).toHaveCount(1); const filtered = await gridChromeBounds(page); await hoverTableBody(page, rows); wheelEvidence.push(await tableWheel(page, 600, info)); await expect.poll(() => gridChromeBounds(page)).toEqual(filtered);
  await filter.fill(""); const restored = await gridChromeBounds(page); await hoverTableBody(page, rows); wheelEvidence.push(await tableWheel(page, 700, info, true)); await expect.poll(async () => (await scrollState()).top).toBeGreaterThan(0); await expect.poll(() => gridChromeBounds(page)).toEqual(restored);
  // Swap the mounted virtual body through a small family, changing available
  // width while it is absent, then verify the new element owns its observation.
  const review = page.getByTestId("node-grid-review-disclosure"); await review.click();
  const bulkRows = page.getByTestId("engineering-table-review-rows"); await expect(bulkRows).toBeVisible();
  await page.getByTestId("entity-grid-type-sections").click(); await expect(bulkRows).toBeHidden();
  await page.getByTestId("toggle-inspector").click(); await page.getByTestId("entity-grid-type-nodes").click(); await expect(bulkRows).toBeVisible();
  await expect.poll(() => bulkRows.evaluate((node) => node.clientHeight - node.parentElement!.clientHeight)).toBe(0);
  const bulkChrome = await gridChromeBounds(page, true); await hoverTableBody(page, bulkRows); wheelEvidence.push(await tableWheel(page, 700, info, true, "engineering-table-review-rows"));
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

test("B4 text fields apply, recover filtered focus, preserve text Undo and retain review drafts through pointer Queue and Clear", async ({ page, browser }, info) => {
  await attachBrowserIdentity(browser, info); await page.goto("/"); await expect(page.getByTestId("workspace-toolbar")).toBeVisible();
  await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
  const table = page.getByTestId("engineering-table"); const label = table.getByTestId("table-cell-node:N-100-label");
  const original = await label.textContent(); expect(original).toBeTruthy();
  await label.dblclick(); const editor = table.getByRole("textbox", { name: "node:N-100 Label", exact: true });
  await editor.fill(` ${original} `); await table.getByRole("button", { name: "Apply", exact: true }).click(); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  await label.dblclick(); await editor.fill("   "); await table.getByRole("button", { name: "Apply", exact: true }).click();
  await expect(editor).toHaveAttribute("aria-invalid", "true"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  await editor.fill("Cancel me"); await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(label).toHaveText(original!);
  await label.dblclick(); await editor.fill("Unique filter label"); await table.getByRole("button", { name: "Apply", exact: true }).click(); await expect(label).toHaveText("Unique filter label");
  const filter = page.getByTestId("model-tree-filter-input"); await filter.fill("Unique filter label");
  const selection = await page.getByTestId("command-selection-readout").textContent();
  await label.dblclick(); await editor.fill("Renamed outside filter"); await table.getByRole("button", { name: "Apply", exact: true }).click();
  await expect(label).toHaveCount(0); await expect(table.getByRole("group", { name: "Node fields footer" })).toBeFocused();
  expect(await page.getByTestId("command-selection-readout").textContent()).toBe(selection);
  await filter.fill(""); await expect(label).toHaveText("Renamed outside filter");
  const provenance = table.getByTestId("table-cell-node:N-100-provenance"); const beforeProvenance = await provenance.textContent();
  await provenance.dblclick(); const text = table.getByRole("textbox", { name: "node:N-100 Provenance", exact: true });
  await text.fill("TBD"); await table.getByRole("button", { name: "Apply", exact: true }).click(); await expect(provenance).toHaveText("TBD");
  await provenance.dblclick(); await text.press("End"); await text.pressSequentially(" transient"); await text.press("ControlOrMeta+z");
  await expect(provenance).toHaveCount(0); await expect(text).toHaveValue("TBD");
  await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(provenance).toHaveText("TBD");
  await page.getByTestId("workspace-undo").click(); await expect(provenance).toHaveText(beforeProvenance!); await page.getByTestId("workspace-redo").click(); await expect(provenance).toHaveText("TBD");
  await page.getByTestId("node-grid-review-disclosure").click(); const review = page.getByTestId("engineering-table-review");
  await review.getByTestId("review-cell-node:N-100-label").focus(); await page.keyboard.press("Q");
  await review.getByRole("button", { name: "Keep draft", exact: true }).click(); await expect(review.getByRole("status")).toHaveText("Draft retained; model unchanged.");
  await review.getByTestId("review-cell-node:N-110-label").dblclick(); await review.getByRole("textbox").fill("hidden retained");
  await page.getByTestId("node-grid-review-disclosure").click(); await expect(label).toHaveText("Renamed outside filter");
  await page.getByTestId("layout-mode-tree").click(); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("node-grid-review-disclosure").click();
  await page.getByTestId("entity-grid-type-pipes").click(); await page.getByTestId("entity-grid-type-nodes").click();
  await expect(review.getByTestId("review-cell-node:N-100-label")).toHaveText("Q"); await expect(review.getByTestId("review-cell-node:N-110-label")).toHaveText("hidden retained");
  await filter.fill("node:N-100"); await review.getByTestId("review-cell-node:N-100-label").dblclick(); await review.getByRole("textbox").fill("   ");
  // Pointer Queue from the active editor must consume the current raw blank once.
  await page.getByTestId("queue-entity-grid-intents").click(); await expect(page.getByTestId("operation-apply-row-editor-intent-1")).toContainText("TBD");
  const appliedBefore = Number((await page.getByTestId("operation-apply-summary").textContent())!.match(/(\d+) applied/)![1]);
  expect(appliedBefore).toBe(3);
  await page.getByTestId("apply-intent-editor-intent-1").click(); await expect(page.getByTestId("operation-apply-summary")).toContainText(`${appliedBefore + 1} applied`);
  await expect(page.getByTestId("operation-apply-summary")).toContainText("4 applied");
  await showModelTree(page); await filter.fill(""); await expect(review.getByTestId("review-cell-node:N-110-label")).toHaveText("hidden retained");
  await review.getByTestId("review-cell-node:N-100-provenance").dblclick(); await review.getByRole("textbox").fill("clear active");
  await page.getByTestId("clear-entity-grid-drafts").click(); await expect(page.getByTestId("entity-grid-change-count")).toHaveText("0 changed cells");
  await expect(review.getByRole("textbox")).toHaveCount(0); await page.getByTestId("node-grid-review-disclosure").click(); await expect(label).toHaveText("TBD");
  await openWorkspaceSection(page, "project"); await page.getByRole("button", { name: "Save local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Saved local browser-preview project");
  await page.getByRole("button", { name: "Open local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Opened local browser-preview project");
  await showModelTree(page); await page.getByTestId("layout-mode-grid").click(); await expect(label).toHaveText("TBD"); await expect(provenance).toHaveText("TBD"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
});

// Move the real pointer inside the currently visible body intersection. Locator
// hover scrolls a wide rowgroup into view and would contaminate wheel-only bounds.
async function hoverTableBody(page: import("@playwright/test").Page, rows: import("@playwright/test").Locator) {
  const point = await rows.evaluate((body) => {
    const row = body.getBoundingClientRect(); const grid = body.closest('[role="grid"]')!.getBoundingClientRect();
    const left = Math.max(row.left, grid.left); const right = Math.min(row.right, grid.right);
    const top = Math.max(row.top, grid.top); const bottom = Math.min(row.bottom, grid.bottom);
    if (right <= left || bottom <= top) throw new Error("No visible table body for wheel input");
    return { x: (left + right) / 2, y: (top + bottom) / 2 };
  });
  await page.mouse.move(point.x, point.y);
}

for (const policy of ["direct", "review"] as const) {
  for (const field of ["label", "x"] as const) {
    test(`B4 character-start ${policy} ${field} appends ordinary keys and preserves intentional selection`, async ({ page, browser }, info) => {
      await attachBrowserIdentity(browser, info); await page.goto("/"); await expect(page.getByTestId("workspace-toolbar")).toBeVisible();
      await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
      if (policy === "review") await page.getByTestId("node-grid-review-disclosure").click();
      const table = page.getByTestId(policy === "review" ? "engineering-table-review" : "engineering-table");
      const cell = table.getByTestId(`${policy === "review" ? "review" : "table"}-cell-node:N-100-${field}`);
      const original = await cell.textContent(); const first = field === "label" ? "q" : "1"; const second = field === "label" ? "r" : "2";
      const editor = table.getByRole("textbox");
      await cell.focus(); await page.keyboard.press(first); await page.keyboard.press(second);
      // Separate physical key events expose the selection defect; fill() would hide it.
      await expect(editor).toHaveValue(first + second);
      expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([2, 2]);
      await editor.press("ArrowLeft");
      await table.getByRole("button", { name: policy === "review" ? "Keep draft" : "Apply", exact: true }).focus();
      await editor.focus();
      expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([1, 1]);
      await editor.press(field === "label" ? "s" : "3");
      await expect(editor).toHaveValue(first + (field === "label" ? "s" : "3") + second);
      await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText(original!); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
      await cell.focus(); await page.keyboard.press(first); await page.keyboard.press(second); await expect(editor).toHaveValue(first + second);
      await table.getByRole("button", { name: policy === "review" ? "Keep draft" : "Apply", exact: true }).click(); await expect(cell).toHaveText(first + second);
      if (policy === "review") {
        await expect(table.getByRole("status")).toHaveText("Draft retained; model unchanged."); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
      } else {
        await expect(page.getByTestId("workspace-undo")).toBeEnabled(); await page.getByTestId("workspace-undo").click(); await expect(cell).toHaveText(original!);
        await expect(page.getByTestId("workspace-undo")).toBeDisabled(); await page.getByTestId("workspace-redo").click(); await expect(cell).toHaveText(first + second);
      }
      await cell.focus(); await page.keyboard.press("Enter");
      expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([0, 2]);
      await editor.press(field === "label" ? "t" : "4"); await expect(editor).toHaveValue(field === "label" ? "t" : "4");
      await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText(first + second);
      await cell.dblclick(); expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([0, 2]);
      await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText(first + second);
    });
  }
}

// Real operation/conversion services; only the public synthetic starting model
// and delivery timing of conversion results are controlled by this scenario.
test("B4 Materials preserve mixed-unit editing, delayed review sort, history and saved ownership", async ({ page, browser }, info) => {
  await attachBrowserIdentity(browser, info);
  const { model } = await readFixture("precision-origin-base.model.json");
  const material = model.materials[0]; const firstId = material.id; const secondId = "material:B4-second";
  model.materials = [{ ...material, elastic_modulus: { value: 200000, unit: "MPa" } },
    { ...material, id: secondId, label: "Synthetic second material", elastic_modulus: { value: 100000000000, unit: "Pa" } }];
  // Delay delivery after the existing Rust conversion has finished. No factors,
  // replacement conversion answers, or model mutation are injected.
  await page.route("**/src/services/displayQuantityService.ts", async (route) => {
    const response = await route.fetch(); const source = await response.text();
    expect(source).toContain("export async function convertDisplayQuantities(");
    await route.fulfill({ response, body: source.replace("export async function convertDisplayQuantities(", "async function originalConvertDisplayQuantities(") + `
export async function convertDisplayQuantities(items) {
  const result = await originalConvertDisplayQuantities(items);
  const gate = window.__b4MaterialConversionGate;
  if (gate && gate.hold && items.some(item => item.id.includes('material') && item.id.includes('elastic'))) {
    await new Promise(resolve => gate.pending.push(resolve));
  }
  return result;
}
` });
  });
  await page.addInitScript(() => { (window as any).__b4MaterialConversionGate = { hold: false, pending: [] }; });
  await gotoModel(page, model);
  await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("entity-grid-type-materials").click();
  const direct = page.getByTestId("material-engineering-table"); const elastic = direct.getByTestId(`table-cell-${firstId}-elastic`);
  await expect(elastic.locator("..")).toHaveAttribute("aria-readonly", "false");
  await expect(elastic.locator("..").locator(".engineering-table-unit")).toHaveText("MPa");
  await direct.getByRole("button", { name: "Sort Elastic", exact: true }).click();
  await expect(direct.getByRole("rowheader").first()).toHaveText(secondId);
  await elastic.dblclick(); const edit = direct.getByRole("textbox"); await edit.fill("0"); await direct.getByRole("button", { name: "Apply", exact: true }).click();
  await expect(edit).toHaveAttribute("aria-invalid", "true"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  await edit.fill("210000"); await direct.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(elastic).toHaveText("200000");
  await elastic.dblclick(); await edit.fill("210000"); await direct.getByRole("button", { name: "Apply", exact: true }).click(); await expect(elastic).toHaveText("210000");
  await expect(page.getByTestId("project-edited")).toBeVisible(); await page.getByTestId("workspace-undo").click(); await expect(elastic).toHaveText("200000");
  await page.getByTestId("workspace-redo").click(); await expect(elastic).toHaveText("210000");
  await page.getByTestId("material-grid-review-disclosure").click(); const review = page.getByTestId("material-engineering-table-review");
  await review.getByRole("button", { name: "Sort Elastic", exact: true }).click(); await review.getByRole("button", { name: "Sort Elastic", exact: true }).click(); await expect(review.getByRole("rowheader").first()).toHaveText(firstId);
  const reviewCell = review.getByTestId(`review-cell-${firstId}-elastic`); await reviewCell.dblclick(); const input = review.getByRole("textbox");
  const retainedInput = await input.elementHandle();
  await page.evaluate(() => { (window as any).__b4MaterialConversionGate.hold = true; });
  await input.press("ControlOrMeta+a"); await input.press("5"); await input.press("0"); await input.press("ArrowLeft");
  await expect(input).toHaveValue("50"); await expect(input).toBeFocused();
  await expect(review.getByRole("status")).toContainText("sort unavailable"); await expect(review.getByRole("rowheader").first()).toHaveText(firstId);
  await expect.poll(() => page.evaluate(() => (window as any).__b4MaterialConversionGate.pending.length)).toBeGreaterThan(0);
  expect(await input.evaluate((node, original) => node === original, retainedInput)).toBe(true);
  expect(await input.evaluate((node: HTMLInputElement) => [node.selectionStart, node.selectionEnd])).toEqual([1, 1]);
  // Descending completion physically moves the active row while its native
  // editor, selection and text-Undo stack must remain owned and intact.
  await page.evaluate(() => { const gate = (window as any).__b4MaterialConversionGate; gate.hold = false; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  await expect(review.getByRole("rowheader").first()).toHaveText(secondId); await expect(input).toBeFocused();
  expect(await input.evaluate((node, original) => node === original, retainedInput)).toBe(true);
  expect(await input.evaluate((node: HTMLInputElement) => [node.selectionStart, node.selectionEnd])).toEqual([1, 1]);
  await input.press("3"); await expect(input).toHaveValue("530"); await input.press("ControlOrMeta+z"); await expect(input).toHaveValue("50");
  await expect(elastic).toHaveText("210000");
  await review.getByRole("button", { name: "Keep draft", exact: true }).click(); await expect(review.getByText("Draft retained; model unchanged.")).toBeVisible();
  await page.getByTestId("entity-grid-type-nodes").click(); await page.getByTestId("entity-grid-type-materials").click(); await expect(reviewCell).toHaveText("50");
  await review.getByTestId(`review-cell-${secondId}-label`).dblclick(); await review.getByRole("textbox").fill("retained second"); await review.getByRole("button", { name: "Keep draft", exact: true }).click();
  const filter = page.getByTestId("model-tree-filter-input"); await filter.fill(firstId);
  await page.getByTestId("queue-entity-grid-intents").click(); await expect(page.getByTestId("operation-apply-row-editor-intent-1")).toContainText('"unit":"MPa"');
  await page.getByTestId("apply-intent-editor-intent-1").click(); await expect(page.getByTestId("operation-apply-summary")).toContainText("2 applied");
  await showModelTree(page); await filter.fill(""); await expect(review.getByTestId(`review-cell-${secondId}-label`)).toHaveText("retained second");
  await page.getByTestId("clear-entity-grid-drafts").click(); await expect(review.getByTestId(`review-cell-${secondId}-label`)).toHaveText("Synthetic second material");
  await page.getByTestId("material-grid-review-disclosure").click(); await expect(elastic).toHaveText("50");
  const geometry = await direct.evaluate((root) => {
    const box = (element: Element) => { const r = element.getBoundingClientRect(); return { x: r.x, y: r.y, width: r.width, height: r.height, bottom: r.bottom }; };
    return { pane: box(root.closest(".shell-table-pane")!), header: box(root.querySelector(".engineering-table-header")!), body: box(root.querySelector(".engineering-table-body-slot")!), footer: box(root.querySelector(".engineering-table-footer")!), units: [...root.querySelectorAll(".engineering-table-unit")].map((unit) => ({ text: unit.textContent, ...box(unit), client: unit.clientWidth, scroll: unit.scrollWidth })) };
  });
  expect(geometry.body.height).toBeGreaterThan(0); expect(geometry.footer.bottom).toBeLessThanOrEqual(geometry.pane.bottom); expect(geometry.header.bottom).toBeLessThanOrEqual(geometry.body.y);
  for (const unit of geometry.units) { expect(unit.width).toBeGreaterThan(0); expect(unit.scroll).toBeLessThanOrEqual(unit.client); }
  await info.attach("materials-contained-units", { body: JSON.stringify(geometry, null, 2), contentType: "application/json" });
  await openWorkspaceSection(page, "project"); await page.getByRole("button", { name: "Save local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Saved local browser-preview project");
  await showModelTree(page); await page.getByTestId("material-grid-review-disclosure").click();
  await page.evaluate(() => { (window as any).__b4MaterialConversionGate.hold = true; });
  await reviewCell.dblclick(); await review.getByRole("textbox").fill("70");
  await expect.poll(() => page.evaluate(() => (window as any).__b4MaterialConversionGate.pending.length)).toBeGreaterThan(0);
  await page.getByTestId("entity-grid-type-nodes").click();
  const nodeLabel = page.getByTestId(`table-cell-${model.nodes[0].id}-label`); await nodeLabel.dblclick(); const nodeEditor = page.getByTestId("engineering-table").getByRole("textbox"); await nodeEditor.fill(" ");
  await page.evaluate(() => { const gate = (window as any).__b4MaterialConversionGate; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  await expect(nodeEditor).toBeFocused(); await expect(nodeEditor).toHaveValue(" "); await page.getByTestId("engineering-table").getByRole("button", { name: "Cancel", exact: true }).click();
  await page.getByTestId("entity-grid-type-materials").click(); await reviewCell.dblclick(); await review.getByRole("textbox").fill("80");
  await expect.poll(() => page.evaluate(() => (window as any).__b4MaterialConversionGate.pending.length)).toBeGreaterThan(0);
  await openWorkspaceSection(page, "project"); await page.getByRole("button", { name: "Open local", exact: true }).click(); await expect(page.getByTestId("local-project-message")).toContainText("Opened local browser-preview project");
  await showModelTree(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("entity-grid-type-materials").click(); await expect(elastic).toHaveText("50"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  if (await page.getByTestId("material-grid-review-disclosure").getAttribute("aria-expanded") === "true") await page.getByTestId("material-grid-review-disclosure").click();
  await direct.getByTestId(`table-cell-${firstId}-label`).dblclick(); const newEditor = direct.getByRole("textbox"); await newEditor.press("R"); await newEditor.press("S");
  await page.evaluate(() => { const gate = (window as any).__b4MaterialConversionGate; gate.hold = false; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  await expect(newEditor).toHaveValue("RS"); await expect(newEditor).toBeFocused(); await direct.getByRole("button", { name: "Cancel", exact: true }).click();
  await page.screenshot({ path: info.outputPath("b4-materials-reopened.png") });
});

test("B4 Materials stable editor owns character starts, virtual scrolling, filtering and resize", async ({ page, browser }, info) => {
  await attachBrowserIdentity(browser, info);
  const { model } = await readFixture("precision-origin-base.model.json"); const material = model.materials[0];
  model.materials = Array.from({ length: 140 }, (_, index) => ({ ...material, id: index === 0 ? material.id : `material:B4-${index}`, label: `Synthetic material ${index}` }));
  await gotoModel(page, model); await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("entity-grid-type-materials").click();
  const table = page.getByTestId("material-engineering-table"); const cell = table.getByTestId(`table-cell-${material.id}-elastic`);
  await expect(cell.locator("..")).toHaveAttribute("aria-readonly", "false");
  await cell.focus(); await page.keyboard.press("x"); const editor = table.getByRole("textbox"); await expect(editor).toBeFocused();
  await page.keyboard.press("y"); await expect(editor).toHaveValue("xy"); expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([2, 2]);
  await page.keyboard.press("ArrowLeft"); const retained = await editor.elementHandle();
  const originalViewport = page.viewportSize()!; await page.setViewportSize({ ...originalViewport, height: originalViewport.height + 80 });
  await expect(editor).toBeFocused(); expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([1, 1]);
  const aligned = async () => editor.evaluate((input) => {
    const anchor = input.closest(".engineering-table")!.querySelector("[data-editor-anchor]")!; const a = anchor.getBoundingClientRect(), e = input.getBoundingClientRect();
    return { dx: e.x - a.x, dy: e.y - a.y, dw: e.width - a.width, dh: e.height - a.height };
  });
  await expect.poll(aligned).toEqual({ dx: 0, dy: 0, dw: 0, dh: 0 });
  await openWorkspaceSection(page, "libraries");
  await page.setViewportSize({ ...originalViewport, height: originalViewport.height + 120 });
  await page.getByTestId("workspace-dock-close").click();
  const pageReturn = await table.evaluate((root) => {
    const input = root.querySelector<HTMLInputElement>("input")!; const anchor = root.querySelector("[data-editor-anchor]")!;
    return { inputVisibility: getComputedStyle(input).visibility, value: input.value, owner: document.activeElement?.outerHTML.slice(0, 300),
      input: input.getBoundingClientRect().toJSON(), anchor: anchor.getBoundingClientRect().toJSON(), ancestorInert: Boolean(root.closest("[inert]")) };
  });
  await info.attach("material-page-inert-return", { body: JSON.stringify(pageReturn, null, 2), contentType: "application/json" });
  await expect(editor).toBeVisible(); await expect(editor).toHaveValue("xy");
  await expect.poll(aligned).toEqual({ dx: 0, dy: 0, dw: 0, dh: 0 });
  expect(await editor.evaluate((node, original) => node === original, retained)).toBe(true);
  await expect(editor).not.toBeFocused(); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  const rows = page.getByTestId("material-engineering-table-rows"); await hoverTableBody(page, rows); await page.mouse.wheel(0, 2200);
  await expect.poll(() => rows.evaluate((element) => element.scrollTop)).toBeGreaterThan(1000);
  expect(await retained!.evaluate((input) => input.isConnected)).toBe(true);
  await expect(editor).toHaveValue("xy");
  // The clipped editor cannot intercept header/footer pointer controls.
  const hits = await table.evaluate((root) => [".engineering-table-header", ".engineering-table-footer"].map((selector) => {
    const element = root.querySelector(selector)!; const r = element.getBoundingClientRect(); const hit = document.elementFromPoint(r.left + 12, r.top + r.height / 2);
    return Boolean(hit && element.contains(hit));
  })); expect(hits).toEqual([true, true]);
  const filter = page.getByTestId("model-tree-filter-input"); await filter.fill("material:B4-139"); await expect(rows.locator('[role="row"]')).toHaveCount(2);
  expect(await editor.evaluate((node, original) => node === original, retained)).toBe(true); await expect(editor).toHaveValue("xy");
  await expect.poll(aligned).toEqual({ dx: 0, dy: 0, dw: 0, dh: 0 });
  await page.getByTestId("entity-grid-type-nodes").click(); await expect(editor).toBeHidden(); await page.getByTestId("entity-grid-type-materials").click(); await expect(editor).toHaveValue("xy");
  await expect(page.getByTestId("entity-grid-type-materials")).toBeFocused();
  await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(editor).toHaveCount(0); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  await filter.fill(""); await cell.focus(); await page.keyboard.press("Enter"); await expect(editor).toBeFocused();
  expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([0, String(material.elastic_modulus.value).length]);
  await table.getByRole("button", { name: "Cancel", exact: true }).click(); await cell.dblclick(); await expect(editor).toBeFocused();
  expect(await editor.evaluate((input: HTMLInputElement) => [input.selectionStart, input.selectionEnd])).toEqual([0, String(material.elastic_modulus.value).length]);
  await table.getByRole("button", { name: "Cancel", exact: true }).click(); await page.setViewportSize(originalViewport);
  await info.attach("material-editor-clipping", { body: JSON.stringify({ headerFooterHitOwnership: hits }), contentType: "application/json" });
});

test("B4 Materials Apply Tab keeps the first Shear character while another quantity reconverts", async ({ page, browser }, info) => {
  await attachBrowserIdentity(browser, info);
  const { model } = await readFixture("precision-origin-base.model.json"); const material = model.materials[0]; const firstId = material.id; const secondId = "material:B4-P2-second";
  model.materials = [{ ...material, elastic_modulus: { value: 200000, unit: "MPa" } },
    { ...material, id: secondId, elastic_modulus: { value: 100000000000, unit: "Pa" } }];
  await page.route("**/src/services/displayQuantityService.ts", async (route) => {
    const response = await route.fetch(); const source = await response.text(); expect(source).toContain("export async function convertDisplayQuantities(");
    await route.fulfill({ response, body: source.replace("export async function convertDisplayQuantities(", "async function originalConvertDisplayQuantities(") + `
export async function convertDisplayQuantities(items) {
  const result = await originalConvertDisplayQuantities(items);
  const gate = window.__b4MaterialP2Gate;
  if (gate.hold && items.some(item => item.id.includes('material') && item.id.includes('elastic'))) await new Promise(resolve => gate.pending.push(resolve));
  return result;
}
` });
  });
  await page.addInitScript(() => { (window as any).__b4MaterialP2Gate = { hold: false, pending: [] }; });
  await gotoModel(page, model); await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("entity-grid-type-materials").click();
  const table = page.getByTestId("material-engineering-table"); const elastic = table.getByTestId(`table-cell-${firstId}-elastic`); const shear = table.getByTestId(`table-cell-${firstId}-shear`);
  await expect(elastic.locator("..")).toHaveAttribute("aria-readonly", "false"); await expect(shear.locator("..")).toHaveAttribute("aria-readonly", "false");
  await table.getByRole("button", { name: "Sort Elastic", exact: true }).click(); await expect(table.getByRole("rowheader").first()).toHaveText(secondId);
  await elastic.dblclick(); await table.getByRole("textbox").fill("210000");
  await page.evaluate(() => { (window as any).__b4MaterialP2Gate.hold = true; });
  await page.keyboard.press("Tab"); await expect(elastic).toHaveText("210000"); await expect(shear).toBeFocused();
  await expect.poll(() => page.evaluate(() => (window as any).__b4MaterialP2Gate.pending.length)).toBeGreaterThan(0);
  await expect(table.getByRole("status").filter({ hasText: "Quantity sort unavailable" })).toContainText("sort unavailable"); await expect(table.getByRole("rowheader").first()).toHaveText(firstId);
  await expect(page.getByTestId("workspace-undo")).toBeEnabled();
  // No locator focus/fill repair: this is the first actual key after Apply+Tab.
  await page.keyboard.press("8");
  const firstKey = await table.evaluate((root) => ({ editorValue: root.querySelector<HTMLInputElement>("input")?.value ?? null,
    activeTag: document.activeElement?.tagName, activeColumn: (document.activeElement as HTMLElement)?.dataset.columnKey,
    shearReadonly: root.querySelector('[data-column-key="shear"]')?.parentElement?.getAttribute("aria-readonly") }));
  await info.attach("material-p2-first-key", { body: JSON.stringify(firstKey), contentType: "application/json" });
  const input = table.getByRole("textbox", { name: `${firstId} Shear [${material.shear_modulus.unit}]` });
  await expect(input).toHaveValue("8"); await expect(input).toBeFocused();
  await page.keyboard.press("7"); await expect(input).toHaveValue("87");
  expect(await input.evaluate((node: HTMLInputElement) => [node.selectionStart, node.selectionEnd])).toEqual([2, 2]);
  await expect(table.getByRole("status").filter({ hasText: "Quantity sort unavailable" })).toContainText("sort unavailable");
  await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(shear).toHaveText(String(material.shear_modulus.value));
  await page.evaluate(() => { const gate = (window as any).__b4MaterialP2Gate; gate.hold = false; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  await expect(table.getByRole("rowheader").first()).toHaveText(secondId);
  // Exactly one accepted model operation; cancelled Shear entry adds no history.
  await page.getByTestId("workspace-undo").click(); await expect(elastic).toHaveText("200000"); await expect(shear).toHaveText(String(material.shear_modulus.value)); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
});

// Original failing diagnostic bytes are retained at 3ba8b70; this regression
// keeps the same decisive keys/oracles while observing the stationary host.
test("B4 diagnostic Node sorted review preserves text Undo after an active row crosses downward", async ({ page, browser }, info) => {
  await attachBrowserIdentity(browser, info);
  const { model } = await readFixture("precision-origin-base.model.json");
  model.nodes.forEach((node: { position: { x: number } }, index: number) => { node.position.x = index === 0 ? 2 : index === 1 ? 15 : 100 + index; });
  const firstId = model.nodes[0].id; const secondId = model.nodes[1].id;
  await gotoModel(page, model); const hashBefore = await currentModelHashThroughVisibleExport(page);
  await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("node-grid-review-disclosure").click();
  const table = page.getByTestId("engineering-table-review"); const cell = table.getByTestId(`review-cell-${firstId}-x`);
  await table.getByRole("button", { name: "Sort X", exact: true }).click();
  await cell.focus(); await page.keyboard.press("1"); await table.getByRole("button", { name: "Keep draft", exact: true }).click();
  await expect(cell).toHaveText("1"); await expect(page.getByTestId("entity-grid-change-count")).toHaveText("1 changed cells");
  await cell.dblclick(); const input = table.getByRole("textbox", { name: `${firstId} X [${model.project.units.length}]` }); await page.keyboard.press("ArrowRight");
  const originalInput = await input.elementHandle();
  // Read-only DOM observation distinguishes actual active-host movement from
  // reorderings that move only its neighbours; no production behavior is hooked.
  const movement = await table.evaluateHandle((root, rowId) => {
    const rowButton = [...root.querySelectorAll('[role="rowheader"] button')].find((button) => button.textContent === rowId)!;
    const wrapper = rowButton.closest("[data-virtual-index]")!; const input = root.querySelector("input")!;
    const counts = { removed: 0, added: 0, inputHostRemoved: 0, inputHostAdded: 0 };
    const observer = new MutationObserver((records) => records.forEach((record) => {
      counts.removed += [...record.removedNodes].filter((node) => node === wrapper).length;
      counts.added += [...record.addedNodes].filter((node) => node === wrapper).length;
      counts.inputHostRemoved += [...record.removedNodes].filter((node) => node === input || node.contains(input)).length;
      counts.inputHostAdded += [...record.addedNodes].filter((node) => node === input || node.contains(input)).length;
    }));
    observer.observe(root, { childList: true, subtree: true });
    return { counts, observer };
  }, firstId);
  const snapshot = () => table.evaluate((root, original) => {
    const current = root.querySelector<HTMLInputElement>("input");
    return { value: current?.value, sameInput: current === original, focused: document.activeElement === current,
      caret: current ? [current.selectionStart, current.selectionEnd] : null,
      rowOrder: [...root.querySelectorAll('[role="rowheader"] button')].map((button) => button.textContent),
      inputY: current?.getBoundingClientRect().y };
  }, originalInput);
  const before = await snapshot();
  await page.keyboard.press("9"); const afterCrossing = await snapshot();
  await page.keyboard.press("ControlOrMeta+z"); const afterUndo = await snapshot();
  const hostMoves = await movement.evaluate((state) => { state.observer.disconnect(); return state.counts; }); await movement.dispose();
  const canonicalX = await page.getByTestId(`table-cell-${firstId}-x`).textContent();
  const history = { undoDisabled: await page.getByTestId("workspace-undo").isDisabled(), redoDisabled: await page.getByTestId("workspace-redo").isDisabled(), editedMarkers: await page.getByTestId("project-edited").count() };
  await page.screenshot({ path: info.outputPath("node-sorted-review-after-undo.png") });
  // Export is passive with respect to canonical model state; it occurs only
  // after the decisive focus/caret/Undo snapshots and may close the review editor.
  const hashAfterStaging = await currentModelHashThroughVisibleExport(page);
  await page.getByTestId("clear-entity-grid-drafts").click(); await expect(page.getByTestId("entity-grid-change-count")).toHaveText("0 changed cells");
  await table.getByRole("button", { name: /Sorted by X/ }).click();
  await page.getByTestId("node-grid-review-disclosure").click();
  const canonicalAfterClear = await page.getByTestId(`table-cell-${firstId}-x`).textContent();
  const observation = { firstId, secondId, before, afterCrossing, afterUndo, hostMoves, canonicalX, canonicalAfterClear, hashBefore, hashAfterStaging, history };
  await info.attach("node-sorted-review-undo-observation", { body: JSON.stringify(observation, null, 2), contentType: "application/json" });
  // Preserve full observation and test-owned cleanup before the expected Undo
  // oracle, so a failure still proves movement and canonical non-mutation.
  expect(before.value).toBe("1"); expect(before.caret).toEqual([1, 1]); expect(before.rowOrder.slice(0, 2)).toEqual([firstId, secondId]);
  expect(afterCrossing.value).toBe("19"); expect(afterCrossing.rowOrder.slice(0, 2)).toEqual([secondId, firstId]);
  expect(hostMoves.removed).toBeGreaterThan(0); expect(hostMoves.added).toBeGreaterThan(0);
  expect(hostMoves.inputHostRemoved).toBe(0); expect(hostMoves.inputHostAdded).toBe(0);
  expect(afterCrossing.sameInput).toBe(true); expect(afterCrossing.focused).toBe(true); expect(afterCrossing.caret).toEqual([2, 2]);
  expect(afterUndo.sameInput).toBe(true); expect(afterUndo.focused).toBe(true);
  expect(canonicalX).toBe("2"); expect(canonicalAfterClear).toBe("2"); expect(hashAfterStaging).toBe(hashBefore);
  expect(history).toEqual({ undoDisabled: true, redoDisabled: true, editedMarkers: 0 });
  expect(afterUndo.value).toBe("1"); expect(afterUndo.caret).toEqual([1, 1]); expect(afterUndo.rowOrder.slice(0, 2)).toEqual([firstId, secondId]);
});

test("B4 Node review retained editor stays contained through hidden families and page resize", async ({ page, browser }, info) => {
  await attachBrowserIdentity(browser, info); const model = await gotoRoutedFixture(page, "ui-foundation-1000.model.json");
  await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("node-grid-review-disclosure").click();
  const table = page.getByTestId("engineering-table-review"); const first = model.nodes[0];
  await table.getByTestId(`review-cell-${first.id}-x`).dblclick(); const input = table.getByRole("textbox"); await input.fill("retained Node draft"); const originalInput = await input.elementHandle();
  // Footer focus is an existing non-committing boundary. Leave the editor open
  // there before changing family, so the hidden layer is actually retained.
  await table.getByRole("button", { name: "Cancel", exact: true }).focus();
  await page.getByTestId("entity-grid-type-materials").click(); await expect(input).toBeHidden();
  const materialTable = page.getByTestId("material-engineering-table"); const materialCell = materialTable.getByTestId(`table-cell-${model.materials[0].id}-label`); const materialLabel = await materialCell.textContent();
  await materialCell.dblclick(); const materialInput = materialTable.getByRole("textbox"); await expect(materialInput).toBeFocused(); await page.keyboard.press("Q"); await expect(materialInput).toHaveValue("Q");
  await materialTable.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(materialCell).toHaveText(materialLabel!); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  await page.getByTestId("entity-grid-type-nodes").click(); await expect(page.getByTestId("entity-grid-type-nodes")).toBeFocused(); await expect(input).toBeVisible(); await expect(input).toHaveValue("retained Node draft");
  const rows = page.getByTestId("engineering-table-review-rows"); await hoverTableBody(page, rows); await page.mouse.wheel(0, 2500);
  await expect.poll(() => rows.evaluate((body) => body.scrollTop)).toBeGreaterThan(1000);
  expect(await originalInput!.evaluate((element) => element.isConnected)).toBe(true);
  const hits = await table.evaluate((root) => [".engineering-table-header", ".engineering-table-footer"].map((selector) => {
    const element = root.querySelector(selector)!; const r = element.getBoundingClientRect(); const hit = document.elementFromPoint(r.left + 12, r.top + r.height / 2); return Boolean(hit && element.contains(hit));
  })); expect(hits).toEqual([true, true]);
  const filter = page.getByTestId("model-tree-filter-input"); await filter.fill(model.nodes.at(-1).id); await expect(rows.locator('[role="row"]')).toHaveCount(2);
  expect(await input.evaluate((element, original) => element === original, originalInput)).toBe(true); await expect(input).toHaveValue("retained Node draft");
  const viewport = page.viewportSize()!; await openWorkspaceSection(page, "libraries"); await page.setViewportSize({ ...viewport, height: viewport.height + 80 }); await page.getByTestId("workspace-dock-close").click();
  await expect(input).toBeVisible(); await expect(input).not.toBeFocused(); await expect(input).toHaveValue("retained Node draft");
  const alignment = await input.evaluate((element) => {
    const owner = element.closest(".engineering-table")!.querySelector(`[aria-owns="${element.id}"]`)!; const a = owner.querySelector("[data-editor-anchor]")!.getBoundingClientRect(); const e = element.getBoundingClientRect();
    return { dx: e.x - a.x, dy: e.y - a.y, dw: e.width - a.width, dh: e.height - a.height };
  }); expect(alignment).toEqual({ dx: 0, dy: 0, dw: 0, dh: 0 });
  await filter.fill(""); await page.getByTestId("clear-entity-grid-drafts").click(); await expect(table.getByRole("textbox")).toHaveCount(0); await expect(page.getByTestId("entity-grid-change-count")).toHaveText("0 changed cells");
  await expect(page.getByTestId(`table-cell-${first.id}-x`)).toHaveText(String(first.position.x)); await expect(page.getByTestId("workspace-undo")).toBeDisabled(); await page.setViewportSize(viewport);
  await info.attach("node-review-retained-host", { body: JSON.stringify({ headerFooterHits: hits, pageReturnAlignment: alignment }), contentType: "application/json" });
});

for (const density of ["comfortable", "compact"] as const) for (const view of ["model", "both"] as const) for (const drawer of [180, 280, 500]) {
  test(`B4 compact drawer ${view} ${density} ${drawer} @explicit-viewport`, async ({ page, browser }, info) => {
    await attachBrowserIdentity(browser, info);
    // Model uses the native window minimum; stacked Both is browser evidence.
    await page.setViewportSize(view === "model" ? { width: 1280, height: 800 } : { width: 1024, height: 768 });
    await page.addInitScript(({ density, drawer }) => localStorage.setItem("chirality.desktop.ui-preferences.v1", JSON.stringify({ version: 1, density, tableDrawerPx: drawer })), { density, drawer });
    await page.goto("/"); await expect(page.getByTestId("workspace-toolbar")).toBeVisible();
    await page.getByTestId(`view-switch-${view}`).click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
    const host = page.getByTestId("shell-tree-host"), table = page.getByTestId("engineering-table"), family = page.getByRole("combobox", { name: "Grid family" });
    const observations: unknown[] = [];
    async function measure(surface = table) {
      const measured = await surface.evaluate((root) => {
        const host = root.closest(".shell-tree-host")!, body = root.querySelector<HTMLElement>('[role="rowgroup"]')!, header = root.querySelector<HTMLElement>(".engineering-table-header")!, footer = root.querySelector<HTMLElement>(".engineering-table-footer")!;
        const rect = (e: Element) => { const r = e.getBoundingClientRect(); return { x: r.x, y: r.y, height: r.height, width: r.width, bottom: r.bottom }; };
        return { host: rect(host), hostScroll: host.scrollHeight, hostClient: host.clientHeight, body: rect(body), header: rect(header), footer: rect(footer), headerColumns: [...header.children].map(rect), rowColumns: [...body.querySelector('[role="row"]')!.children].map(rect) };
      });
      observations.push(measured);
      if (drawer < 500) expect(measured.host.height).toBeCloseTo(drawer - 53, 0);
      else { expect(measured.host.height).toBeGreaterThan(227); expect(measured.host.height).toBeLessThanOrEqual(drawer - 53); }
      expect(measured.hostScroll).toBe(measured.hostClient);
      expect(measured.body.height).toBeGreaterThanOrEqual(36); expect(measured.body.y).toBeGreaterThanOrEqual(measured.header.bottom - 1);
      expect(measured.body.bottom).toBeLessThanOrEqual(measured.footer.y + 1); expect(measured.footer.bottom).toBeLessThanOrEqual(measured.host.bottom + 1);
      measured.headerColumns.forEach((column, i) => { expect(measured.rowColumns[i].x).toBeCloseTo(column.x, 0); expect(measured.rowColumns[i].width).toBeCloseTo(column.width, 0); });
    }
    async function hit(locator: ReturnType<typeof page.locator>) {
      const scrolls = () => page.locator(".model-grid-toolbar, .engineering-table-footer:visible").evaluateAll((es) => es.map((e) => ({ className: e.className, scrollLeft: e.scrollLeft, clientWidth: e.clientWidth, scrollWidth: e.scrollWidth })));
      const before = await scrolls(); await locator.scrollIntoViewIfNeeded(); observations.push({ pointerTarget: await locator.getAttribute("aria-label") ?? await locator.textContent(), before, after: await scrolls() });
      expect(await locator.evaluate((element) => { const r = element.getBoundingClientRect(), top = document.elementFromPoint(r.x + r.width / 2, r.y + r.height / 2); return top === element || element.contains(top); })).toBe(true);
    }
    await measure(); await expect(family).toHaveAccessibleName("Grid family"); await hit(family); await hit(page.getByTestId("model-tree-filter-input"));
    const cell = page.getByTestId("table-cell-node:N-100-y"); await cell.dblclick(); const editor = table.getByRole("textbox");
    await editor.fill("invalid"); await editor.press("Enter"); await expect(editor).toHaveAttribute("aria-invalid", "true"); await measure(); await hit(editor);
    await hit(table.getByRole("button", { name: "Apply", exact: true })); await hit(table.getByRole("button", { name: "Cancel", exact: true }));
    await expect(table.getByRole("alert")).toContainText("finite");
    const er = await editor.boundingBox(), br = await table.getByRole("rowgroup").boundingBox();
    expect(er!.height).toBeGreaterThanOrEqual(28); expect(er!.y).toBeGreaterThanOrEqual(br!.y); expect(er!.y + er!.height).toBeLessThanOrEqual(br!.y + br!.height);
    await family.selectOption("materials"); await family.selectOption("sections"); await family.selectOption("pipes");
    await expect(page.getByTestId("entity-grid-table-pipes")).toBeVisible(); await hit(page.getByTestId("clear-entity-grid-drafts")); expect(await host.evaluate((e) => e.scrollHeight === e.clientHeight)).toBe(true);
    await family.selectOption("nodes"); await expect(editor).toHaveValue("invalid");
    await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(cell).toHaveText("0"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
    await cell.dblclick(); await table.getByRole("textbox").fill("0.25"); await table.getByRole("button", { name: "Apply", exact: true }).click();
    await expect(cell).toHaveText("0.25"); await page.getByTestId("workspace-undo").click(); await expect(cell).toHaveText("0");
    await page.getByTestId("workspace-redo").click(); await expect(cell).toHaveText("0.25"); await page.getByTestId("workspace-undo").click(); await expect(cell).toHaveText("0");
    await page.getByTestId("node-grid-review-disclosure").click(); const review = page.getByTestId("engineering-table-review"); await measure(review);
    const reviewCell = review.getByTestId("review-cell-node:N-100-y"); await reviewCell.dblclick(); await review.getByRole("textbox").fill("0.5");
    await review.getByRole("button", { name: "Keep draft", exact: true }).click(); await measure(review);
    await hit(page.getByTestId("queue-entity-grid-intents")); await hit(page.getByTestId("clear-entity-grid-drafts"));
    await page.getByRole("button", { name: "Table details", exact: true }).click(); const details = page.locator(".compact-table-details:popover-open");
    await expect(details).toBeFocused(); await expect(details).toContainText("1 changed cells"); await expect(details).toContainText("keeping a draft does not change the model"); await page.keyboard.press("Escape"); await expect(details).toBeHidden(); await expect(review).toBeVisible();
    await page.getByTestId("clear-entity-grid-drafts").click(); await expect(reviewCell).toHaveText("0");
    await reviewCell.dblclick(); await review.getByRole("textbox").fill("0.75"); await review.getByRole("button", { name: "Keep draft", exact: true }).click();
    await page.getByTestId("queue-entity-grid-intents").click(); await showModelTree(page); await expect(page.locator(".compact-queued-message")).toContainText("Queued 1 review intent"); await measure(review);
    await expect(page.getByTestId("workspace-undo")).toBeDisabled(); await page.getByTestId("node-grid-review-disclosure").click(); await expect(cell).toHaveText("0");
    const body = table.getByRole("rowgroup"), b = await body.boundingBox(); await page.mouse.move(b!.x + 80, b!.y + 15); await page.mouse.wheel(0, 500);
    if (drawer === 180) await expect.poll(() => body.evaluate((e) => e.scrollTop)).toBeGreaterThan(0);
    expect(await host.evaluate((e) => e.scrollTop)).toBe(0);
    await page.getByTestId("model-tree-filter-input").fill("N-100"); await expect(cell).toHaveText("0");
    await page.getByTestId("toggle-tree").click(); await expect(table).toBeHidden(); await page.getByTestId("toggle-tree").click(); await expect(table).toBeVisible();
    await expect(page.getByTestId("model-tree-filter-input")).toHaveValue("N-100");
    await page.getByRole("button", { name: "Table details", exact: true }).click(); await expect(page.locator(".compact-table-details:popover-open")).toBeVisible();
    await page.getByTestId("layout-mode-tree").click(); await expect(page.locator(".compact-table-details:popover-open")).toHaveCount(0); await expect(family).toHaveCount(0);
    await page.getByTestId("layout-mode-grid").click(); await expect(family).toHaveValue("nodes");
    await info.attach("compact-drawer-geometry", { body: JSON.stringify({ density, view, drawer, observations }, null, 2), contentType: "application/json" }); await page.screenshot({ path: info.outputPath("compact-drawer.png") });
  });
}

for (const view of ["model", "both"] as const) {
  test(`B4 Details owns traversed Escape and preserves editor ownership in ${view} @explicit-viewport`, async ({ page, browser }, info) => {
    await attachBrowserIdentity(browser, info);
    await page.setViewportSize(view === "model" ? { width: 1280, height: 800 } : { width: 1024, height: 768 });
    const { model } = await readFixture("precision-origin-base.model.json");
    model.sections = [{ id: "section:repair", name: "Invented repair", section_type: "pipe", properties: {
      outside_diameter: { value: 100, unit: "mm" }, wall_thickness: { value: 10, unit: "mm" }
    }, provenance: "invented keyboard regression" }];
    await gotoModel(page, model); await page.getByTestId(`view-switch-${view}`).click(); await ensureTreeExpanded(page); await page.getByTestId("layout-mode-grid").click();
    const family = page.getByRole("combobox", { name: "Grid family" });
    await expect(page.locator(".compact-family-label")).toContainText("Family"); await expect(family).toHaveAccessibleName("Grid family");
    const trigger = page.getByRole("button", { name: "Table details", exact: true });
    const details = page.getByRole("dialog", { name: "Table details", exact: true });
    const drawer = page.getByTestId("toggle-tree"), table = page.getByTestId("engineering-table");
    const observations: unknown[] = [];
    for (const traversal of ["Tab", "Shift+Tab"]) {
      await trigger.click(); await expect(details).toBeFocused(); await page.keyboard.press(traversal);
      await expect(details).toBeVisible(); await expect(details).not.toBeFocused();
      observations.push({ traversal, focusBeforeEscape: await page.evaluate(() => document.activeElement?.outerHTML) });
      await page.keyboard.press("Escape"); await expect(details).toBeHidden(); await expect(trigger).toBeFocused(); await expect(drawer).toHaveAttribute("aria-expanded", "true");
    }
    await trigger.click(); await expect(details).toBeFocused(); await page.keyboard.press("Escape"); await expect(details).toBeHidden(); await expect(trigger).toBeFocused();
    // Native typeahead changes the family through an actual key, while the
    // platform popup itself is outside headless Chromium keyboard control.
    await family.focus(); await page.keyboard.press("p"); await expect(family).toHaveValue("pipes");
    await family.selectOption("nodes"); await expect(family).toHaveValue("nodes");
    const node = model.nodes[0], cell = page.getByTestId(`table-cell-${node.id}-x`);
    await cell.dblclick(); const input = table.getByRole("textbox"); await input.fill("invalid retained"); await input.press("Enter");
    await trigger.click(); await expect(details).toBeVisible(); await input.focus();
    // Descendant editor Escape runs first; it must not be stolen by Details.
    await page.keyboard.press("Escape"); await expect(input).toHaveCount(0); await expect(details).toBeVisible(); await expect(cell).toHaveText(String(node.position.x));
    await page.keyboard.press("Escape"); await expect(details).toBeHidden(); await expect(trigger).toBeFocused(); await expect(drawer).toHaveAttribute("aria-expanded", "true");
    await family.selectOption("sections"); const section = page.getByTestId("section-engineering-table"), type = page.getByTestId("table-cell-section:repair-type");
    await type.dblclick(); const enumeration = section.getByRole("combobox"); await enumeration.fill("p"); await trigger.click(); await expect(details).toBeVisible();
    await enumeration.focus(); const popup = page.getByRole("listbox", { name: "Supported values" }); await expect(popup).toBeVisible();
    await page.keyboard.press("Escape"); await expect(popup).toHaveCount(0); await expect(enumeration).toHaveValue("p"); await expect(details).toBeVisible();
    await page.keyboard.press("Escape"); await expect(enumeration).toHaveCount(0); await expect(type).toHaveText("pipe"); await expect(details).toBeVisible();
    await page.keyboard.press("Escape"); await expect(details).toBeHidden(); await expect(trigger).toBeFocused(); await expect(drawer).toHaveAttribute("aria-expanded", "true");
    await family.selectOption("nodes"); await cell.dblclick(); await input.fill("retained transition"); await input.press("Enter"); const originalInput = await input.elementHandle();
    await trigger.click(); await expect(details).toBeFocused();
    // True focus exit closes Details without stealing focus from the destination.
    const outside = page.getByTestId("workspace-undo"); const destination = page.getByTestId("view-switch-table");
    await destination.focus(); await expect(details).toBeHidden(); await expect(destination).toBeFocused();
    await destination.click(); await expect(family).toHaveCount(0); await expect(page.locator(".compact-table-details")).toHaveCount(0); await expect(input).toHaveValue("retained transition"); expect(await originalInput!.evaluate((element) => element.isConnected)).toBe(true);
    await input.focus(); await page.keyboard.press("Escape"); await expect(input).toHaveCount(0); await expect(cell).toBeFocused(); await expect(outside).toBeDisabled();
    await page.getByTestId(`view-switch-${view}`).click(); await expect(family).toBeVisible();
    await cell.dblclick(); await input.fill("retained tree"); await input.press("Enter"); const treeInput = await input.elementHandle();
    await trigger.click(); await page.getByTestId("layout-mode-tree").click(); await expect(page.locator(".compact-table-details")).toHaveCount(0); await expect(family).toHaveCount(0);
    await page.getByTestId("layout-mode-grid").click(); await expect(input).toHaveValue("retained tree"); expect(await treeInput!.evaluate((element) => element.isConnected)).toBe(true);
    // The previous editor was deliberately cancelled, not unmounted by view change.
    expect(await originalInput!.evaluate((element) => element.isConnected)).toBe(false);
    await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(outside).toBeDisabled();
    if (view === "both") {
      await family.focus(); await page.keyboard.press("Escape"); await expect(drawer).toHaveAttribute("aria-expanded", "false");
      await drawer.click(); await expect(family).toBeVisible(); await expect(page.locator(".compact-table-details:popover-open")).toHaveCount(0);
    }
    await info.attach("details-escape-regression", { body: JSON.stringify({ view, observations }, null, 2), contentType: "application/json" });
  });
}
