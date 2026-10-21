import { expect, test, type Page } from "@playwright/test";

// CLM-024 focused browser evidence for the DEL-07-05:1 rotation glyphs. The
// harness (e2e/rotation-glyph/) renders the production PipeViewport over the
// bundled invented reference and its preserved rows. `full` keeps the emitted
// rx/ry/rz rows; `translational` drops only those rows. The glyphs must show
// in the readout, in the renderer's own draw counts, and as drawn pixels at a
// projected glyph point, while everything else renders identically. Display
// evidence only: no engineering result or claim is asserted.

type Vec3 = { x: number; y: number; z: number };
type HarnessGlyph = { nodeId: string; anchor: Vec3; axis: Vec3; displayLength: number };
type Harness = { variant: string; rotationState: string; glyphs: HarnessGlyph[] };

// One glyph: a cylinder shaft (8 radial segments: 16 side + 16 cap triangles)
// and two cones (10 radial segments: 10 side + 10 base triangles each).
const TRIANGLES_PER_GLYPH = 32 + 2 * 20;

async function open(page: Page, variant: "full" | "translational") {
  await page.goto(`/e2e/rotation-glyph/index.html?variant=${variant}`);
  await expect(page.getByTestId("viewport-canvas")).toBeVisible();
  await expect.poll(async () => page.evaluate(() => {
    const current = globalThis.__openPipeStressUiDiagnosticsV1?.readCurrent();
    if (!current || "status" in current.viewport) return null;
    const { ownedPendingRafCount, rendererInfo } = current.viewport.resources;
    return ownedPendingRafCount === 0 && rendererInfo.calls > 0;
  }), { timeout: 30_000 }).toBe(true);
  // Let any trailing frame settle so both variants are compared at rest.
  await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  const status = page.getByTestId("viewport-deformation-status");
  await status.locator(":scope > summary").click();
  await expect(status).toHaveAttribute("open", "");
  const harness = await page.evaluate(() => (globalThis as unknown as { __rotationGlyphHarness: Harness }).__rotationGlyphHarness);
  const renderer = await page.evaluate(() => {
    const current = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
    if ("status" in current.viewport) throw new Error("Viewport unavailable");
    return current.viewport.resources.rendererInfo;
  });
  return { harness, renderer };
}

async function canvasPoint(page: Page, authoredPoint: Vec3) {
  return page.evaluate((authoredPoint) => {
    const api = globalThis.__openPipeStressUiDiagnosticsV1;
    const current = api.readCurrent();
    if ("status" in current.viewport || current.model.generation === null) throw new Error("Viewport unavailable");
    const projected = api.projectAuthoredPoint({
      modelGeneration: current.model.generation,
      cameraSequence: current.viewport.camera.sequence,
      authoredPoint
    });
    if (projected.status !== "available" || !projected.insideCanvasCss) throw new Error("Point outside canvas");
    const bounds = document.querySelector('[data-testid="viewport-canvas"] canvas')!.getBoundingClientRect();
    return { x: bounds.left + projected.canvasCssPoint.x, y: bounds.top + projected.canvasCssPoint.y };
  }, authoredPoint);
}

const clip = (center: { x: number; y: number }, half = 4) =>
  ({ x: Math.round(center.x) - half, y: Math.round(center.y) - half, width: half * 2 + 1, height: half * 2 + 1 });

test("rotation glyphs draw emitted rx/ry/rz rows in a real browser", async ({ page }, info) => {
  const full = await open(page, "full");
  expect(full.harness.rotationState).toBe("available");
  expect(full.harness.glyphs.length).toBeGreaterThan(0);

  // Readout: emitted maximum with its unit, drawn count, display-only boundary.
  const summary = page.getByTestId("viewport-rotation-summary");
  await expect(summary).toContainText(`rotation: available;`);
  await expect(summary).toContainText(`drawn=${full.harness.glyphs.length}; max |component|=`);
  await expect(summary).toContainText("rad");
  const boundary = page.getByTestId("viewport-rotation-boundary");
  await expect(boundary).toContainText("scale=normalized_display_length_not_physical");
  await expect(boundary).toContainText("glyph=double_arrow_right_hand_rotation_vector");
  await expect(boundary).toContainText("professional_claim=false");
  // The translational readout is the deformed shape's, unchanged.
  await expect(page.getByTestId("viewport-deformation-summary")).toContainText("available; nodes=");

  // A point on the longest glyph's shaft, and a control point in the canvas corner.
  const longest = full.harness.glyphs.reduce((best, glyph) => (glyph.displayLength > best.displayLength ? glyph : best));
  const along = (fraction: number): Vec3 => ({
    x: longest.anchor.x + longest.axis.x * longest.displayLength * fraction,
    y: longest.anchor.y + longest.axis.y * longest.displayLength * fraction,
    z: longest.anchor.z + longest.axis.z * longest.displayLength * fraction
  });
  const glyphPoint = await canvasPoint(page, along(0.5));
  const canvasBox = (await page.getByTestId("viewport-canvas").boundingBox())!;
  const corner = { x: canvasBox.x + 12, y: canvasBox.y + canvasBox.height - 12 };
  const fullGlyphPixels = await page.screenshot({ clip: clip(glyphPoint) });
  const fullCornerPixels = await page.screenshot({ clip: clip(corner) });
  await info.attach("rotation-glyphs-full", { body: await page.getByTestId("viewport-canvas").screenshot(), contentType: "image/png" });

  const translational = await open(page, "translational");
  expect(translational.harness.rotationState).toBe("unavailable");
  expect(translational.harness.glyphs).toEqual([]);
  await expect(page.getByTestId("viewport-rotation-summary"))
    .toContainText("rotation: unavailable; no complete rotation triple on a drawable basis");
  await expect(page.getByTestId("viewport-deformation-summary")).toContainText("available; nodes=");
  await info.attach("rotation-glyphs-translational", { body: await page.getByTestId("viewport-canvas").screenshot(), contentType: "image/png" });

  // The renderer drew exactly the glyph meshes more: two draw calls (shaft and
  // heads) per spatial group, and each glyph's triangles.
  expect(full.renderer.calls).toBeGreaterThan(translational.renderer.calls);
  expect((full.renderer.calls - translational.renderer.calls) % 2).toBe(0);
  expect(full.renderer.triangles - translational.renderer.triangles).toBe(TRIANGLES_PER_GLYPH * full.harness.glyphs.length);

  // Same camera and scene otherwise: the glyph point's pixels differ, the corner's do not.
  expect(await canvasPoint(page, along(0.5))).toEqual(glyphPoint);
  expect(await page.screenshot({ clip: clip(corner) })).toEqual(fullCornerPixels);
  expect(await page.screenshot({ clip: clip(glyphPoint) })).not.toEqual(fullGlyphPixels);
});
