import { expect, test, type Page } from "@playwright/test";
import { expectViewportLabelTargets, gotoModel, readFixture, selectTreeRow, showModelTree } from "./ui-foundation-workflows";

// DEL-07-01 labels (viewportLabelLayout.ts): a label's 24 px target lets a Box Select press through as
// the label does; no two drawn targets meet, coincident selections included; and a press on an
// entity beside its own label reaches the canvas.

async function model() {
  const { model } = await readFixture("precision-origin-base.model.json");
  const node = (id: string, x: number, y: number, z = 0) => ({ ...model.nodes[0], id, label: id, position: { x, y, z } });
  // LB-A and LB-B, and LB-C and LB-D, stand at the same X and Y: seen from the front each pair is one point.
  model.nodes = [node("node:LB-A", -6, 0), node("node:LB-B", -6, 0, 3), node("node:LB-C", 6, 0), node("node:LB-D", 6, 0, 3), node("node:LB-E", 0, 4)];
  const basis = model.pipe_segments[0];
  model.pipe_segments = [
    { ...basis, id: "pipe:LB-1", label: "LB 1", from: "node:LB-A", to: "node:LB-C" },
    { ...basis, id: "pipe:LB-2", label: "LB 2", from: "node:LB-B", to: "node:LB-D" },
    { ...basis, id: "pipe:LB-3", label: "LB 3", from: "node:LB-C", to: "node:LB-E" }
  ];
  model.supports = []; model.components = []; model.load_cases = []; model.combinations = []; model.diagnostics = [];
  return model;
}

async function settled(page: Page) {
  await expect.poll(async () => page.evaluate(() => {
    const current = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
    return "status" in current.viewport ? null : current.viewport.resources.ownedPendingRafCount;
  })).toBe(0);
}

async function open(page: Page) {
  await page.setViewportSize({ width: 1440, height: 920 });
  await gotoModel(page, await model());
  await page.getByTestId("view-switch-model").click();
  await page.getByTestId("viewport-view-front").click();
  await settled(page);
}

async function canvasPoint(page: Page, authoredPoint: { x: number; y: number; z: number }) {
  await settled(page);
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

async function drawnLabelBox(page: Page, id: string) {
  const label = page.getByTestId(`viewport-select-${id}`);
  await expect(label).toBeVisible();
  const box = await label.boundingBox();
  if (!box) throw new Error(`${id} has no box`);
  return box;
}

test("Box Select: a drag that starts in a drawn label's transparent margin draws the box", async ({ page }) => {
  await open(page);
  const plate = await drawnLabelBox(page, "node:LB-E");
  await page.getByTestId("viewport-box-select").click();
  // 2 px above the plate: inside the label's 24 px target, outside its drawn plate.
  const start = { x: plate.x + plate.width / 2, y: plate.y - 2 };
  expect(await page.evaluate(({ x, y }) => document.elementFromPoint(x, y)?.closest('[data-testid="viewport-canvas"]') !== null, start)).toBe(true);
  await page.mouse.move(start.x, start.y);
  await page.mouse.down();
  await page.mouse.move(start.x + 120, start.y + 90, { steps: 8 });
  await expect(page.locator(".viewport-box-rect")).toBeVisible();
  await page.mouse.up();
});

test("coincident selections: the primary's label is drawn, and no two drawn targets meet or are covered", async ({ page }) => {
  await open(page);
  await showModelTree(page);
  await selectTreeRow(page, "node", "node:LB-A");
  for (const id of ["node:LB-B", "node:LB-C", "node:LB-D"]) await selectTreeRow(page, "node", id, { toggle: true });
  await page.getByTestId("view-switch-model").click();
  await settled(page);
  await expect(page.getByTestId("viewport-select-node:LB-D")).toBeVisible();
  const witness = await expectViewportLabelTargets(page);
  const boxes = await page.evaluate(() => [...document.querySelectorAll<HTMLElement>(".viewport-selection-layer .viewport-select-target")]
    .filter((label) => getComputedStyle(label).visibility !== "hidden")
    .map((label) => {
      const plate = label.getBoundingClientRect();
      const width = Math.max(24, plate.width);
      const height = Math.max(24, plate.height);
      const left = plate.left + plate.width / 2 - width / 2;
      const top = plate.top + plate.height / 2 - height / 2;
      return { left, top, right: left + width, bottom: top + height };
    }));
  expect(boxes.length).toBe(witness.length);
  for (let i = 0; i < boxes.length; i += 1) for (let j = i + 1; j < boxes.length; j += 1) {
    const [a, b] = [boxes[i], boxes[j]];
    expect(a.left < b.right && a.right > b.left && a.top < b.bottom && a.bottom > b.top, `targets ${i} and ${j} meet`).toBe(false);
  }
});

test("a press on a pipe beside its own label reaches the canvas and picks the pipe", async ({ page }) => {
  await open(page);
  const pipeLabel = await drawnLabelBox(page, "pipe:LB-3");
  const from = await canvasPoint(page, { x: 6, y: 0, z: 0 });
  const to = await canvasPoint(page, { x: 0, y: 4, z: 0 });
  // The point on the pipe nearest its label's plate.
  const centre = { x: pipeLabel.x + pipeLabel.width / 2, y: pipeLabel.y + pipeLabel.height / 2 };
  const dx = to.x - from.x;
  const dy = to.y - from.y;
  const t = Math.max(0, Math.min(1, ((centre.x - from.x) * dx + (centre.y - from.y) * dy) / (dx * dx + dy * dy)));
  const onPipe = { x: from.x + dx * t, y: from.y + dy * t };
  const owner = await page.evaluate(({ x, y }) => document.elementFromPoint(x, y)?.closest('[data-testid="viewport-canvas"]') !== null, onPipe);
  expect(owner, "the pipe beside its label is the canvas's to press").toBe(true);
  await page.mouse.click(onPipe.x, onPipe.y);
  await expect(page.getByTestId("viewport-select-pipe:LB-3")).toHaveAttribute("aria-pressed", "true");
});

// A tall riser with a dense row of nodes at its foot: fitted from the front, the row sits just above
// the canvas's foot, where the closed drawer's tab strip lies over the canvas.
async function tallModel() {
  const { model } = await readFixture("precision-origin-base.model.json");
  const node = (id: string, x: number, y: number) => ({ ...model.nodes[0], id, label: id.replace("node:", ""), position: { x, y, z: 0 } });
  const row = Array.from({ length: 9 }, (_, index) => node(`node:W${index}`, -2 + index * 0.5, 0));
  model.nodes = [...row, node("node:TOP", 0, 20)];
  const basis = model.pipe_segments[0];
  model.pipe_segments = [
    { ...basis, id: "pipe:RISER", label: "RISER", from: "node:W4", to: "node:TOP" },
    ...row.slice(1).map((end, index) => ({ ...basis, id: `pipe:ROW-${index}`, label: `ROW ${index}`, from: row[index].id, to: end.id }))
  ];
  model.supports = []; model.components = []; model.load_cases = []; model.combinations = []; model.diagnostics = [];
  return model;
}

test("with the table drawer closed, no drawn label's target lies under its tab strip", async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 920 });
  await gotoModel(page, await tallModel());
  await page.getByTestId("view-switch-model").click();
  const drawer = page.getByTestId("toggle-tree");
  if (await drawer.getAttribute("aria-expanded") === "true") await drawer.click();
  await expect(drawer).toHaveAttribute("aria-expanded", "false");
  await page.getByTestId("viewport-view-front").click();
  await settled(page);
  const strip = await page.getByTestId("stage-tab-strip").boundingBox();
  if (!strip) throw new Error("the tab strip has no box");
  // The case is engaged: the row's anchors stand within a label's reach of the strip (a label placed
  // below its anchor reaches 28 px down: 8 px off, a 16 px plate, its 24 px target's margin).
  const foot = await canvasPoint(page, { x: 0, y: 0, z: 0 });
  expect(strip.y - foot.y, `the row is ${strip.y - foot.y} px above the strip`).toBeLessThan(28);
  expect(strip.y - foot.y).toBeGreaterThan(0);
  await page.waitForTimeout(300);
  const targets = await page.evaluate(() => [...document.querySelectorAll<HTMLElement>(".viewport-selection-layer .viewport-select-target")]
    .filter((label) => getComputedStyle(label).visibility !== "hidden")
    .map((label) => {
      const plate = label.getBoundingClientRect();
      const height = Math.max(24, plate.height);
      return { key: label.dataset.entityKey, bottom: plate.top + plate.height / 2 + height / 2 };
    }));
  expect(targets.length).toBeGreaterThan(0);
  for (const target of targets) expect(target.bottom, `${target.key} reaches ${target.bottom}, the strip starts at ${strip.y}`).toBeLessThanOrEqual(strip.y + 0.5);
});
