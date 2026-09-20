import { expect, test, type Page } from "@playwright/test";
import {
  activateWithKeyboard, clearTreeFilter, currentModelHashThroughVisibleExport, gotoModel,
  readFixture, selectTreeRow, setAppearance, typedTreeRow
} from "./ui-foundation-workflows";
import { openWorkspaceSection, projectCommand, showCanvas, startPropertyTaskFromTreeEntity } from "./workspace-driver";

// Synthetic presentation fixture derived in memory from the maintained model envelope.
// No performance lane, new diagnostics, or stored duplicate fixture.
async function fixture(overlap = false) {
  const { model } = await readFixture("precision-origin-base.model.json");
  const node = (id: string, x: number, y: number, z = 0) => ({
    ...model.nodes[0], id, label: id, position: { x, y, z }
  });
  model.nodes = [node("node:C3-A0", -4, 0), node("node:C3-A1", 4, 0),
    node("node:C3-B0", -4, overlap ? 0 : 3, overlap ? 2 : 0),
    node("node:C3-B1", 4, overlap ? 0 : 3, overlap ? 2 : 0), node("node:C3-loose", 0, -3)];
  if (overlap) model.nodes = model.nodes.slice(0, 4);
  const basis = model.pipe_segments[0];
  model.pipe_segments = [
    { ...basis, id: "pipe:C3-A", label: "C3 A", from: "node:C3-A0", to: "node:C3-A1" },
    { ...basis, id: "pipe:C3-B", label: "C3 B", from: "node:C3-B0", to: "node:C3-B1" }
  ];
  model.supports = []; model.components = []; model.load_cases = []; model.combinations = []; model.diagnostics = [];
  return model;
}
const control = (page: Page, name: string) => page.getByRole("button", { name, exact: true });
const label = (page: Page, id: string) => page.getByTestId(`viewport-select-${id}`);
async function ready(page: Page) {
  await expect.poll(async () => page.evaluate(() => {
    const current = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
    return "status" in current.viewport ? null : current.viewport.resources.ownedPendingRafCount;
  })).toBe(0);
}
async function selected(page: Page) {
  return page.evaluate(() => {
    const current = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
    if ("status" in current.viewport) throw new Error("Viewport unavailable");
    return current.viewport.selection.orderedRefs;
  });
}
async function point(page: Page, authoredPoint: { x: number; y: number; z: number }) {
  await ready(page);
  return page.evaluate((authoredPoint) => {
    const api = globalThis.__openPipeStressUiDiagnosticsV1;
    const current = api.readCurrent();
    if ("status" in current.viewport || current.model.generation === null) throw new Error("Viewport unavailable");
    const projected = api.projectAuthoredPoint({ modelGeneration: current.model.generation, cameraSequence: current.viewport.camera.sequence, authoredPoint });
    if (projected.status !== "available" || !projected.insideCanvasCss) throw new Error("Point outside canvas");
    const bounds = document.querySelector('[data-testid="viewport-canvas"] canvas')!.getBoundingClientRect();
    return { x: bounds.left + projected.canvasCssPoint.x, y: bounds.top + projected.canvasCssPoint.y };
  }, authoredPoint);
}
async function expectMainCanvasPoint(page: Page, p: { x: number; y: number }) {
  expect(await page.evaluate(({ x, y }) => document.elementFromPoint(x, y) === document.querySelector('[data-testid="viewport-canvas"] canvas'), p)).toBe(true);
}
async function history(page: Page) {
  await openWorkspaceSection(page, "operations");
  const value = await page.getByTestId("session-history-chip").innerText();
  await showCanvas(page);
  return value;
}

async function narrowBothPane(page: Page) {
  await page.getByTestId("view-switch-both").click();
  const splitter = page.getByTestId("resize-model-tree");
  await splitter.focus();
  for (let step = 0; step < 50; step++) {
    if (await splitter.getAttribute("aria-valuenow") === await splitter.getAttribute("aria-valuemax")) break;
    await page.keyboard.press("ArrowRight");
  }
  await expect(splitter).toHaveAttribute("aria-valuenow", (await splitter.getAttribute("aria-valuemax"))!);
  const box = await page.getByTestId("viewport-canvas").boundingBox();
  expect(box).not.toBeNull();
  expect(Math.round(box!.width)).toBe(220);
}

test("C3 snapshot, Hide precedence, persistent hidden count, theme and Show All preserve model/history", async ({ page }, info) => {
  await gotoModel(page, await fixture());
  const beforeHash = await currentModelHashThroughVisibleExport(page);
  const beforeHistory = await history(page);
  await selectTreeRow(page, "pipe", "pipe:C3-A");
  await clearTreeFilter(page);
  await ready(page);
  const normalImage = await page.getByTestId("viewport-canvas").screenshot();
  await control(page, "Isolate").click();
  await ready(page);
  const isolatedImage = await page.getByTestId("viewport-canvas").screenshot();
  expect(isolatedImage.equals(normalImage), "Isolate changes the drawn body presentation").toBe(false);
  await info.attach("before-isolate", { body: normalImage, contentType: "image/png" });
  await info.attach("after-isolate", { body: isolatedImage, contentType: "image/png" });
  await expect(label(page, "pipe:C3-A")).toHaveAttribute("data-dimmed", "false");
  await expect(label(page, "pipe:C3-B")).toHaveAttribute("data-dimmed", "true");
  await expect(control(page, "Show All")).toBeEnabled();
  await expect(page.getByTestId("viewport-hidden-count")).toHaveCount(0);
  for (const theme of ["dark", "light"] as const) {
    await setAppearance(page, theme, "comfortable");
    await expect(label(page, "pipe:C3-B")).toHaveAttribute("data-dimmed", "true");
    expect(await label(page, "pipe:C3-B").locator("span").evaluate(el => getComputedStyle(el).opacity)).toBe("0.2");
  }
  // Table selection does not rewrite the snapshot, including hidden table selections.
  await selectTreeRow(page, "pipe", "pipe:C3-B");
  await expect(label(page, "pipe:C3-B")).toHaveAttribute("data-dimmed", "true");
  await control(page, "Hide").click();
  await expect(label(page, "pipe:C3-B")).toHaveCount(0);
  const hidden = page.getByTestId("viewport-hidden-count");
  await expect(hidden).toHaveText("1 hidden · Show all");
  await selectTreeRow(page, "pipe", "pipe:C3-B", { toggle: true });
  await expect.poll(() => selected(page)).toEqual([]);
  await expect(hidden).toHaveText("1 hidden · Show all");
  await selectTreeRow(page, "pipe", "pipe:C3-B");
  await expect(label(page, "pipe:C3-B")).toHaveCount(0);
  await narrowBothPane(page);
  await hidden.focus();
  await page.keyboard.press("Shift+Tab"); await page.keyboard.press("Tab");
  await expect(hidden).toBeFocused();
  const containment = await hidden.evaluate(el => {
    const r = el.getBoundingClientRect(), parent = el.closest(".viewport-toolbar-status-strip")!.getBoundingClientRect();
    const style = getComputedStyle(el);
    const points = [[r.left + 2, r.top + 2], [r.right - 2, r.top + 2], [r.left + 2, r.bottom - 2], [r.right - 2, r.bottom - 2], [(r.left + r.right) / 2, (r.top + r.bottom) / 2]];
    return { top: r.top >= parent.top, bottom: r.bottom <= parent.bottom, left: r.left >= parent.left, right: r.right <= parent.right,
      width: r.width, height: r.height, focusVisible: el.matches(":focus-visible"), outlineWidth: style.outlineWidth, outlineOffset: style.outlineOffset,
      unobscured: points.every(([x, y]) => el.contains(document.elementFromPoint(x, y))) };
  });
  expect(containment).toMatchObject({ top: true, bottom: true, left: true, right: true, focusVisible: true, unobscured: true });
  expect(containment.width).toBeGreaterThanOrEqual(24); expect(containment.height).toBeGreaterThanOrEqual(24);
  expect(parseFloat(containment.outlineWidth)).toBeGreaterThan(0); expect(parseFloat(containment.outlineOffset)).toBeLessThan(0);
  await info.attach("hidden-count-containment", { body: JSON.stringify(containment), contentType: "application/json" });
  await info.attach("narrow-hidden-count-focus", { body: await page.locator(".viewport-toolbar-status-strip").screenshot(), contentType: "image/png" });
  await hidden.click();
  await expect(hidden).toHaveCount(0);
  await expect(label(page, "pipe:C3-B")).toHaveAttribute("data-dimmed", "false");
  await expect(control(page, "Show All")).toBeDisabled();
  expect(await currentModelHashThroughVisibleExport(page)).toBe(beforeHash);
  expect(await history(page)).toBe(beforeHistory);
  await info.attach("shown-after-clear", { body: await page.getByTestId("viewport-canvas").screenshot(), contentType: "image/png" });
});

test("C3 dimmed geometry remains available to hover, click and box selection", async ({ page }, info) => {
  await gotoModel(page, await fixture());
  await control(page, "Front").click();
  await page.getByTestId("viewport-fit-model").click();
  await selectTreeRow(page, "pipe", "pipe:C3-A");
  await control(page, "Isolate").click();
  await page.getByTestId("toggle-viewport-labels").click();
  const p = await point(page, { x: 0, y: 3, z: 0 });
  await expectMainCanvasPoint(page, p);
  await page.mouse.move(0, 0); await ready(page);
  const unhovered = await page.getByTestId("viewport-canvas").screenshot();
  await page.mouse.move(p.x, p.y); await ready(page);
  const hovered = await page.getByTestId("viewport-canvas").screenshot();
  expect(hovered.equals(unhovered), "dimmed body gains a hover halo").toBe(false);
  await info.attach("dimmed-hover", { body: hovered, contentType: "image/png" });
  await page.mouse.click(p.x, p.y);
  await expect.poll(() => selected(page)).toEqual([{ type: "pipe", id: "pipe:C3-B" }]);
  await page.getByTestId("toggle-viewport-labels").click();
  await expect(label(page, "pipe:C3-B")).toHaveAttribute("data-dimmed", "true");
  await page.getByTestId("toggle-viewport-labels").click();
  await page.getByTestId("viewport-box-select").click();
  await page.getByTestId("viewport-selection-filter").selectOption("pipes");
  const left = await point(page, { x: -4, y: 3, z: 0 });
  const right = await point(page, { x: 4, y: 3, z: 0 });
  const start = { x: Math.min(left.x, right.x) - 12, y: Math.min(left.y, right.y) - 12 };
  const end = { x: Math.max(left.x, right.x) + 12, y: Math.max(left.y, right.y) + 12 };
  await expectMainCanvasPoint(page, start); await expectMainCanvasPoint(page, end);
  await page.mouse.move(start.x, start.y); await page.mouse.down();
  await page.mouse.move(end.x, end.y, { steps: 5 }); await page.mouse.up();
  await expect.poll(() => selected(page)).toEqual([{ type: "pipe", id: "pipe:C3-B" }]);
  await page.getByTestId("viewport-box-select").click();
  await page.getByTestId("toggle-viewport-labels").click();
  await expect(label(page, "pipe:C3-B")).toHaveAttribute("data-dimmed", "true");
});

test("C3 nearer dimmed pipe wins unchanged point ordering and viewport keyboard focus is visible", async ({ page }, info) => {
  await gotoModel(page, await fixture(true));
  await control(page, "Front").click();
  await page.getByTestId("viewport-fit-model").click();
  const canvas = page.getByTestId("viewport-canvas");
  await narrowBothPane(page);
  await page.getByTestId("viewport-fit-model").click();
  await page.getByTestId("toggle-viewport-labels").click();
  const far = await point(page, { x: 0, y: 0, z: 0 });
  const near = await point(page, { x: 0, y: 0, z: 2 });
  expect(far.x).toBeCloseTo(near.x, 5); expect(far.y).toBeCloseTo(near.y, 5);
  await expectMainCanvasPoint(page, near); await page.mouse.click(near.x, near.y);
  await expect.poll(() => selected(page)).toEqual([{ type: "pipe", id: "pipe:C3-B" }]);
  await selectTreeRow(page, "pipe", "pipe:C3-A");
  await canvas.focus(); await page.keyboard.press("i");
  await expect(control(page, "Show All")).toBeEnabled();
  const focus = await canvas.evaluate(el => {
    const s = getComputedStyle(el), r = el.getBoundingClientRect();
    return { visible: el.matches(":focus-visible"), width: s.outlineWidth, offset: s.outlineOffset, color: s.outlineColor, rect: r.toJSON() };
  });
  expect(focus.visible).toBe(true); expect(parseFloat(focus.width)).toBeGreaterThan(0); expect(parseFloat(focus.offset)).toBeLessThan(0);
  await info.attach("canvas-keyboard-focus", { body: await canvas.screenshot(), contentType: "image/png" });
  await info.attach("canvas-focus-style", { body: JSON.stringify(focus), contentType: "application/json" });
  const p = await point(page, { x: 0, y: 0, z: 2 });
  expect(p.x).toBeCloseTo(near.x, 5); expect(p.y).toBeCloseTo(near.y, 5);
  await expectMainCanvasPoint(page, p); await page.mouse.click(p.x, p.y);
  await expect.poll(() => selected(page)).toEqual([{ type: "pipe", id: "pipe:C3-B" }]);
  await canvas.focus(); await page.keyboard.press("h");
  await expect(page.getByTestId("viewport-hidden-count")).toHaveText("1 hidden · Show all");
  // Shortcuts are inert while typing outside the viewport and with modifiers.
  const filter = page.getByTestId("model-tree-filter-input");
  await filter.fill("i"); await filter.press("h");
  await expect(page.getByTestId("viewport-hidden-count")).toHaveText("1 hidden · Show all");
  await filter.fill("");
  await control(page, "Show All").click();
  await canvas.focus(); await page.keyboard.press("Alt+i");
  await expect(control(page, "Show All")).toBeDisabled();
});

test("C3 isolation remains clearable when every drawable entity belongs to the snapshot", async ({ page }) => {
  const model = await fixture();
  model.nodes = [model.nodes[4]];
  model.pipe_segments = [];
  await gotoModel(page, model);
  await selectTreeRow(page, "node", "node:C3-loose");
  await control(page, "Isolate").click();
  await expect(label(page, "node:C3-loose")).toHaveAttribute("data-dimmed", "false");
  await expect(page.locator('.viewport-select-target[data-dimmed="true"]')).toHaveCount(0);
  await expect(page.getByTestId("viewport-hidden-count")).toHaveCount(0);
  await selectTreeRow(page, "node", "node:C3-loose", { toggle: true });
  await expect.poll(() => selected(page)).toEqual([]);
  await expect(control(page, "Show All")).toBeEnabled();
  await activateWithKeyboard(page, control(page, "Show All"));
  await expect(control(page, "Show All")).toBeDisabled();
});

test("C3 deletion retains active empty snapshot; new geometry dims; project replacement clears it", async ({ page }) => {
  await gotoModel(page, await fixture());
  await selectTreeRow(page, "node", "node:C3-loose");
  await control(page, "Isolate").click();
  await startPropertyTaskFromTreeEntity(page, "node", "node:C3-loose");
  await page.getByTestId("queue-delete-node-intent").click();
  await openWorkspaceSection(page, "operations");
  await page.locator('[data-testid^="operation-apply-row-"]').filter({ hasText: "node:C3-loose" }).getByRole("button", { name: "Apply", exact: true }).click();
  await showCanvas(page);
  await clearTreeFilter(page);
  await expect(typedTreeRow(page, "node", "node:C3-loose")).toHaveCount(0);
  await expect(control(page, "Show All")).toBeEnabled();
  await expect(label(page, "pipe:C3-A")).toHaveAttribute("data-dimmed", "true");
  await page.getByTestId("command-node").click();
  for (const [field, value] of Object.entries({ id: "node:C3-new", label: "C3 new", x: "1", y: "-2", z: "0", provenance: "invented_c3_visibility_input" })) {
    await page.getByTestId(`viewport-create-node-${field}`).fill(value);
  }
  await page.getByTestId("queue-explicit-node-intent").click();
  await page.getByTestId("apply-reviewed-draft").click();
  await page.getByTestId("workspace-select").click();
  await page.getByTestId("viewport-fit-model").click();
  await expect(label(page, "node:C3-new")).toHaveAttribute("data-dimmed", "true");
  await projectCommand(page, "new-blank");
  await expect(control(page, "Show All")).toBeDisabled();
  await expect(page.getByTestId("viewport-hidden-count")).toHaveCount(0);
});

test("C3 visibility leaves Current results current and never revives Historical overlays", async ({ page }) => {
  await page.goto("/");
  await openWorkspaceSection(page, "solve");
  await page.getByTestId("run-mechanics-preview").click();
  await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
  await showCanvas(page);
  await selectTreeRow(page, "pipe", "pipe:P-100");
  await control(page, "Isolate").click();
  await control(page, "Hide").click();
  await control(page, "Show All").click();
  await openWorkspaceSection(page, "results");
  await expect(page.getByTestId("historical-run-context")).toHaveCount(0);
  await projectCommand(page, "save-local", true);
  await expect(page.getByTestId("local-project-message")).toContainText("Saved");
  await projectCommand(page, "open-local", true);
  await openWorkspaceSection(page, "results");
  await expect(page.getByTestId("historical-run-context")).toBeVisible();
  await showCanvas(page);
  await selectTreeRow(page, "pipe", "pipe:P-100");
  await control(page, "Isolate").click();
  await control(page, "Show All").click();
  await expect(page.getByTestId("viewport-deformation-summary")).toContainText("result rows=0");
  await openWorkspaceSection(page, "results");
  await expect(page.getByTestId("historical-run-context")).toBeVisible();
  await openWorkspaceSection(page, "evidence");
  await expect(page.getByTestId("status-pill-solve-proof")).toHaveCount(0);
});
