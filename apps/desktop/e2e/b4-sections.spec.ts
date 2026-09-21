import { expect, test, type Page } from "@playwright/test";
import { attachBrowserIdentity, currentModelHashThroughVisibleExport, gotoModel, readFixture } from "./ui-foundation-workflows";
import { ensureTreeExpanded } from "./workspace-driver";

async function inventedSections(count = 2) {
  const { model } = await readFixture("precision-origin-base.model.json");
  model.sections = Array.from({ length: count }, (_, index) => ({ id: `section:B4-${index}`, name: `Invented ${index}`, section_type: "pipe",
    properties: { outside_diameter: { value: index === 0 ? 2 : index === 1 ? 100 : 200 + index, unit: index === 0 ? "m" : "mm" }, wall_thickness: { value: 10, unit: "mm" } }, provenance: "invented Sections browser fixture" }));
  return model;
}
async function openSections(page: Page, model: any) {
  await gotoModel(page, model); await page.getByTestId("view-switch-table").click(); await ensureTreeExpanded(page);
  await page.getByTestId("layout-mode-grid").click(); await page.getByTestId("entity-grid-type-sections").click();
  await expect(page.getByTestId("table-cell-section:B4-0-outside").locator("..")).toHaveAttribute("aria-readonly", "false");
}

test("B4 Sections mixed-unit direct editing preserves no-op, rejection, shared-pipe rejection and history", async ({ page, browser }, info) => {
  await attachBrowserIdentity(browser, info); const model = await inventedSections();
  // Bind one existing invented pipe with an exact shared cache; retain its other fields.
  const pipe = model.pipe_segments[0]; pipe.section_ref = model.sections[0].id;
  pipe.section.outside_diameter = { value: 2, unit: "m" }; pipe.section.wall_thickness = { value: 10, unit: "mm" };
  pipe.section.mill_tolerance = { value: 4, unit: "mm" };
  await openSections(page, model); const table = page.getByTestId("section-engineering-table");
  await table.getByRole("button", { name: "Sort Outside dia.", exact: true }).click(); await expect(table.getByRole("rowheader").first()).toHaveText("section:B4-1");
  const od = table.getByTestId("table-cell-section:B4-0-outside"), wall = table.getByTestId("table-cell-section:B4-0-wall");
  await od.dblclick(); const input = table.getByRole("textbox"); await input.fill("2.00"); await table.getByRole("button", { name: "Apply", exact: true }).click();
  await expect(input).toHaveCount(0); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  await wall.dblclick(); await expect(input).toHaveAccessibleName("section:B4-0 Wall [mm]"); await input.fill("3"); await table.getByRole("button", { name: "Apply", exact: true }).click();
  await expect(input).toHaveValue("10"); await expect(table.getByRole("alert")).toContainText("Engine rejected"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  await table.getByRole("button", { name: "Cancel", exact: true }).click();
  await wall.dblclick(); await input.fill("12"); await table.getByRole("button", { name: "Apply", exact: true }).click(); await expect(wall).toHaveText("12");
  await page.getByTestId("workspace-undo").click(); await expect(wall).toHaveText("10"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  await page.getByTestId("workspace-redo").click(); await expect(wall).toHaveText("12");
});

test("B4 Sections enum explicit completion, passive cancellation and raw review retention", async ({ page, browser }, info) => {
  await attachBrowserIdentity(browser, info); const model = await inventedSections(); model.sections[0].section_type = "p";
  await openSections(page, model); const table = page.getByTestId("section-engineering-table"); const type = table.getByTestId("table-cell-section:B4-0-type");
  await type.dblclick(); const input = table.getByRole("combobox"); await input.click(); await expect(page.getByRole("listbox", { name: "Supported values" })).toBeVisible();
  await page.keyboard.press("Enter"); await expect(input).toHaveValue("p"); await expect(input).toHaveAttribute("aria-invalid", "true"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  await input.click(); await page.keyboard.press("ArrowDown"); await page.keyboard.press("Escape"); await expect(input).toHaveValue("p"); await expect(page.getByRole("listbox", { name: "Supported values" })).toHaveCount(0);
  await page.keyboard.press("Escape"); await expect(type).toHaveText("p");
  await type.dblclick(); await input.fill("pi"); const option = page.getByRole("option", { name: "pipe", exact: true }); await expect(option).toBeVisible();
  const box = await option.boundingBox(); expect(box).not.toBeNull();
  const visibleAtCenter = await option.evaluate((node) => { const r = node.getBoundingClientRect(); return node.contains(document.elementFromPoint(r.x + r.width / 2, r.y + r.height / 2)); }); expect(visibleAtCenter).toBe(true);
  await page.mouse.move(box!.x + box!.width / 2, box!.y + box!.height / 2); await page.mouse.down(); await expect(input).toHaveValue("pi");
  await page.mouse.move(box!.x + box!.width + 30, box!.y); await page.mouse.up(); await expect(input).toHaveValue("pi");
  await option.click(); await expect(input).toHaveValue("pipe"); await expect(input).toBeFocused(); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  await table.getByRole("button", { name: "Cancel", exact: true }).click(); await expect(type).toHaveText("p");
  await type.dblclick(); await input.fill("pi"); await page.keyboard.press("Tab"); await expect(type).toHaveText("pipe"); await expect(table.getByTestId("table-cell-section:B4-0-outside")).toBeFocused();
  await page.getByTestId("workspace-undo").click(); await expect(type).toHaveText("p"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
  await page.getByTestId("section-grid-review-disclosure").click(); const review = page.getByTestId("section-engineering-table-review");
  await review.getByTestId("review-cell-section:B4-0-type").dblclick(); await review.getByRole("combobox").fill("TBD"); await review.getByRole("button", { name: "Keep draft", exact: true }).click();
  await expect(page.getByTestId("queue-entity-grid-intents")).toBeDisabled(); await page.getByTestId("section-grid-review-disclosure").click(); await expect(page.getByTestId("section-grid-review-disclosure")).toContainText("1 retained draft");
  await page.getByTestId("section-grid-review-disclosure").click(); await page.getByTestId("clear-entity-grid-drafts").click(); await expect(review.getByTestId("review-cell-section:B4-0-type")).toHaveText("p");
});

test("B4 Sections Apply Tab accepts the next actual key during unrelated quantity reconversion", async ({ page, browser }, info) => {
  await attachBrowserIdentity(browser, info); const model = await inventedSections();
  await page.route("**/src/services/displayQuantityService.ts", async (route) => {
    const response = await route.fetch(); const source = await response.text(); expect(source).toContain("export async function convertDisplayQuantities(");
    await route.fulfill({ response, body: source.replace("export async function convertDisplayQuantities(", "async function originalConvertDisplayQuantities(") + `
export async function convertDisplayQuantities(items) {
 const result = await originalConvertDisplayQuantities(items);
 const gate = window.__b4SectionGate;
 if (gate.hold && items.some(item => item.id.includes('section') && item.id.includes('outside'))) await new Promise(resolve => gate.pending.push(resolve));
 return result;
}
` });
  });
  await page.addInitScript(() => { (window as any).__b4SectionGate = { hold: false, pending: [] }; });
  await openSections(page, model); const table = page.getByTestId("section-engineering-table");
  const od = table.getByTestId("table-cell-section:B4-0-outside"), wall = table.getByTestId("table-cell-section:B4-0-wall");
  await table.getByRole("button", { name: "Sort Outside dia.", exact: true }).click(); await expect(table.getByRole("rowheader").first()).toHaveText("section:B4-1");
  await od.dblclick(); await table.getByRole("textbox").fill("2.1"); await page.evaluate(() => { (window as any).__b4SectionGate.hold = true; }); await page.keyboard.press("Tab");
  await expect(od).toHaveText("2.1"); await expect(wall).toBeFocused(); await expect.poll(() => page.evaluate(() => (window as any).__b4SectionGate.pending.length)).toBeGreaterThan(0);
  await expect(table.getByRole("status").filter({ hasText: "Quantity sort unavailable" })).toBeVisible(); await page.keyboard.press("8");
  const input = table.getByRole("textbox", { name: "section:B4-0 Wall [mm]" }); await expect(input).toHaveValue("8"); await expect(input).toBeFocused(); await page.keyboard.press("7"); await expect(input).toHaveValue("87");
  await table.getByRole("button", { name: "Cancel", exact: true }).click();
  await page.evaluate(() => { const gate = (window as any).__b4SectionGate; gate.hold = false; gate.pending.splice(0).forEach((resolve: () => void) => resolve()); });
  await expect(table.getByRole("rowheader").first()).toHaveText("section:B4-1"); await page.getByTestId("workspace-undo").click(); await expect(od).toHaveText("2"); await expect(wall).toHaveText("10"); await expect(page.getByTestId("workspace-undo")).toBeDisabled();
});

test("B4 Sections moved review row preserves text Undo, input ownership and virtual hidden lifetimes", async ({ page, browser }, info) => {
  await attachBrowserIdentity(browser, info); const model = await inventedSections(140);
  model.sections[0].properties.outside_diameter = { value: 2, unit: "mm" }; model.sections[1].properties.outside_diameter = { value: 15, unit: "mm" };
  await openSections(page, model); const hashBefore = await currentModelHashThroughVisibleExport(page);
  await page.getByTestId("section-grid-review-disclosure").click(); const table = page.getByTestId("section-engineering-table-review"); const cell = table.getByTestId("review-cell-section:B4-0-outside");
  await table.getByRole("button", { name: "Sort Outside dia.", exact: true }).click(); await cell.focus(); await page.keyboard.press("1"); await table.getByRole("button", { name: "Keep draft", exact: true }).click();
  await cell.dblclick(); const input = table.getByRole("textbox"); await page.keyboard.press("ArrowRight"); const original = await input.elementHandle();
  const movement = await table.evaluateHandle((root) => {
    const input = root.querySelector("input")!, row = root.querySelector('[data-testid="review-cell-section:B4-0-wall"]')!.closest("[data-virtual-index]")!;
    const counts = { rowRemoved: 0, inputRemoved: 0 }; const observer = new MutationObserver((records) => records.forEach((r) => {
      counts.rowRemoved += [...r.removedNodes].filter((n) => n === row).length; counts.inputRemoved += [...r.removedNodes].filter((n) => n === input || n.contains(input)).length;
    })); observer.observe(root, { childList: true, subtree: true }); return { counts, observer };
  });
  await page.keyboard.press("9"); await expect(table.getByRole("rowheader").first()).toHaveText("section:B4-1"); await expect(input).toHaveValue("19");
  expect(await input.evaluate((node, prior) => node === prior, original)).toBe(true); await expect(input).toBeFocused();
  expect(await table.locator('[aria-owns]').count()).toBe(1); expect(await table.locator('[aria-owns]').getAttribute("aria-owns")).toBe(await input.getAttribute("id"));
  await page.keyboard.press("ControlOrMeta+z"); await expect(input).toHaveValue("1"); await expect(table.getByRole("rowheader").first()).toHaveText("section:B4-0");
  const moved = await movement.evaluate((state) => { state.observer.disconnect(); return state.counts; }); expect(moved.rowRemoved).toBeGreaterThan(0); expect(moved.inputRemoved).toBe(0);
  await input.fill("invalid"); const rows = page.getByTestId("section-engineering-table-review-rows"); await rows.hover(); await page.mouse.wheel(0, 2500); await expect(input).toHaveValue("invalid");
  const filter = page.getByTestId("model-tree-filter-input"); await filter.fill("section:B4-139");
  // Natural review blur Keeps the raw draft and closes the live editor. The
  // filter owns focus; clearing it reveals the retained cell without reopening.
  await expect(input).toHaveCount(0); await expect(filter).toBeFocused(); await filter.fill(""); await expect(cell).toHaveText("invalid");
  await page.getByTestId("entity-grid-type-nodes").click(); await page.getByTestId("entity-grid-type-sections").click(); await expect(cell).toHaveText("invalid");
  await page.getByTestId("clear-entity-grid-drafts").click(); await filter.fill(""); await expect(cell).toHaveText("2"); expect(await currentModelHashThroughVisibleExport(page)).toBe(hashBefore);
  await expect(page.getByTestId("workspace-undo")).toBeDisabled(); await movement.dispose();
});
