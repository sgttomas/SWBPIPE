import { expect, test, type Page } from "@playwright/test";
import { gotoModel, readFixture } from "./ui-foundation-workflows";
import { viewportControl } from "./workspace-driver";

// DEL-07-01 REQ-012: a view or stage switch keeps the camera where the person set it, and a fitted
// view is refitted to the new canvas (viewportCameraRule.ts states the rule). These run with real
// canvas sizes: the window is resized and the canvas follows.

type Pose = { position: readonly number[]; target: readonly number[]; sequence: number };

async function settled(page: Page) {
  await expect.poll(async () => page.evaluate(() => {
    const current = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
    return "status" in current.viewport ? null : current.viewport.resources.ownedPendingRafCount;
  })).toBe(0);
}

async function pose(page: Page): Promise<Pose> {
  await settled(page);
  return page.evaluate(() => {
    const current = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
    if ("status" in current.viewport) throw new Error("Viewport unavailable");
    const { position, target, sequence } = current.viewport.camera;
    return { position: [...position], target: [...target], sequence };
  });
}

function expectSamePose(actual: Pose, expected: Pose) {
  for (let axis = 0; axis < 3; axis += 1) {
    expect(actual.position[axis]).toBeCloseTo(expected.position[axis], 9);
    expect(actual.target[axis]).toBeCloseTo(expected.target[axis], 9);
  }
}

function moved(a: Pose, b: Pose): boolean {
  return a.position.some((value, axis) => Math.abs(value - b.position[axis]) > 1e-6) ||
    a.target.some((value, axis) => Math.abs(value - b.target[axis]) > 1e-6);
}

async function canvasBox(page: Page) {
  const box = await page.getByTestId("viewport-canvas").locator("canvas").boundingBox();
  if (!box) throw new Error("the canvas has no box");
  return box;
}

/** The window resized; waits until the canvas has its new size and the view has settled. */
async function resize(page: Page, width: number, height: number) {
  const before = await canvasBox(page);
  await page.setViewportSize({ width, height });
  await expect.poll(async () => {
    const box = await canvasBox(page);
    return Math.round(box.width) !== Math.round(before.width) || Math.round(box.height) !== Math.round(before.height);
  }).toBe(true);
  await settled(page);
}

/** A real orbit: the left button dragged across empty canvas. */
async function orbit(page: Page) {
  const box = await canvasBox(page);
  await page.mouse.move(box.x + box.width * 0.2, box.y + box.height * 0.8);
  await page.mouse.down();
  await page.mouse.move(box.x + box.width * 0.45, box.y + box.height * 0.7, { steps: 10 });
  await page.mouse.up();
  await settled(page);
}

/** The canvas point of an authored point. */
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

// A long run along X, so that on a narrow canvas the fit is bound by the canvas's width and a new
// canvas size gives a new fitted pose (a compact model's fit is its minimum distance at any aspect).
async function longModel() {
  const { model } = await readFixture("precision-origin-base.model.json");
  const node = (id: string, x: number, y: number, z = 0) => ({ ...model.nodes[0], id, label: id, position: { x, y, z } });
  model.nodes = [node("node:CAM-A", -20, 0), node("node:CAM-B", 0, 0), node("node:CAM-C", 20, 0), node("node:CAM-D", 20, 3)];
  const basis = model.pipe_segments[0];
  model.pipe_segments = [
    { ...basis, id: "pipe:CAM-1", label: "CAM 1", from: "node:CAM-A", to: "node:CAM-B" },
    { ...basis, id: "pipe:CAM-2", label: "CAM 2", from: "node:CAM-B", to: "node:CAM-C" },
    { ...basis, id: "pipe:CAM-3", label: "CAM 3", from: "node:CAM-C", to: "node:CAM-D" }
  ];
  model.supports = []; model.components = []; model.load_cases = []; model.combinations = []; model.diagnostics = [];
  return model;
}

// Both view in a 1440 by 1000 window: the canvas is 603 wide and taller than wide, so the Front
// view of the long run is bound by its width; at 1440 by 900 it is not.
const NARROW = { width: 1440, height: 1000 };
const WIDER = { width: 1440, height: 900 };

async function open(page: Page, view: "front" | "top" = "front") {
  await page.setViewportSize(NARROW);
  const model = await longModel();
  await gotoModel(page, model);
  await page.getByTestId("view-switch-both").click();
  // Labels off, so that a press on the canvas reaches the canvas.
  const labels = page.getByTestId("toggle-viewport-labels");
  if (await labels.getAttribute("aria-pressed") === "true") await labels.click();
  await page.getByTestId(`viewport-view-${view}`).click();
  await settled(page);
  return model;
}

test.describe("the camera: fitted views refit, a view the person set is kept", () => {
  test("a fitted view refits when the canvas changes size, and returns to the same pose", async ({ page }) => {
    await open(page);
    const fitted = await pose(page);
    await resize(page, WIDER.width, WIDER.height);
    expect(moved(await pose(page), fitted)).toBe(true);
    await resize(page, NARROW.width, NARROW.height);
    expectSamePose(await pose(page), fitted);
  });

  test("after a real drag the camera is left where the person set it", async ({ page }) => {
    await open(page);
    await orbit(page);
    const set = await pose(page);
    await expect(page.getByTestId("viewport-view-front")).toHaveAttribute("aria-pressed", "false");
    await resize(page, WIDER.width, WIDER.height);
    expectSamePose(await pose(page), set);
  });

  // Top looks straight down: there the controls' own normalisation moves the camera on every
  // update, so a press is the person's move only when their pointer travelled.
  for (const view of ["front", "top"] as const) {
    test(`in the ${view} view, a click on an entity, a right-click and a press released where it began leave the view fitted`, async ({ page }) => {
      const model = await open(page, view);
      const fitted = await pose(page);
      const onEntity = await canvasPoint(page, model.nodes[1].position);
      await page.mouse.click(onEntity.x, onEntity.y);
      const box = await canvasBox(page);
      await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.15, { button: "right" });
      await page.keyboard.press("Escape");
      await page.mouse.move(box.x + box.width * 0.5, box.y + box.height * 0.2);
      await page.mouse.down();
      await page.mouse.up();
      await expect(page.getByTestId(`viewport-view-${view}`)).toHaveAttribute("aria-pressed", "true");
      await resize(page, WIDER.width, WIDER.height);
      expect(moved(await pose(page), fitted)).toBe(true);
      await expect(page.getByTestId(`viewport-view-${view}`)).toHaveAttribute("aria-pressed", "true");
      await resize(page, NARROW.width, NARROW.height);
      expectSamePose(await pose(page), fitted);
    });
  }

  for (const command of ["viewport-fit-model", "viewport-fit-visible", "viewport-fit-selection", "viewport-view-front", "viewport-view-top", "viewport-view-side", "viewport-view-isometric"] as const) {
    test(`${command} returns a moved view to fitted, and it refits as that command would`, async ({ page }) => {
      const model = await open(page);
      if (command === "viewport-fit-selection") {
        const onEntity = await canvasPoint(page, model.nodes[1].position);
        await page.mouse.click(onEntity.x, onEntity.y);
      }
      await orbit(page);
      const press = async () => {
        await (await viewportControl(page, page.getByTestId(command))).click();
        if (await page.getByTestId("viewport-more-menu").count()) await page.keyboard.press("Escape");
        return pose(page);
      };
      const fitted = await press();
      await resize(page, WIDER.width, WIDER.height);
      const refitted = await pose(page);
      // The refit is what the same command gives on this canvas.
      expectSamePose(await press(), refitted);
      await resize(page, NARROW.width, NARROW.height);
      expectSamePose(await pose(page), fitted);
    });
  }

  test("Fit after an orbit keeps the direction the person looks from", async ({ page }) => {
    await open(page);
    await orbit(page);
    const set = await pose(page);
    await page.getByTestId("viewport-fit-model").click();
    const fitted = await pose(page);
    const direction = (p: Pose) => {
      const d = p.target.map((value, axis) => value - p.position[axis]);
      const length = Math.hypot(...d);
      return d.map((value) => value / length);
    };
    const [a, b] = [direction(set), direction(fitted)];
    expect(a[0] * b[0] + a[1] * b[1] + a[2] * b[2]).toBeGreaterThan(1 - 1e-9);
    for (const view of ["front", "top", "side", "isometric"]) {
      await expect(page.getByTestId(`viewport-view-${view}`)).toHaveAttribute("aria-pressed", "false");
    }
  });

  test("re-pressing the pressed standard view after an orbit restores it", async ({ page }) => {
    await open(page);
    const front = await pose(page);
    await orbit(page);
    await expect(page.getByTestId("viewport-view-front")).toHaveAttribute("aria-pressed", "false");
    await page.getByTestId("viewport-view-front").click();
    await expect(page.getByTestId("viewport-view-front")).toHaveAttribute("aria-pressed", "true");
    expectSamePose(await pose(page), front);
  });
});
