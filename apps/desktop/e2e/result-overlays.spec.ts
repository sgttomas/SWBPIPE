import { expect, test, type Page } from "@playwright/test";

// Browser evidence for the viewport result overlays (OD-5, FR-GUI-001). The harness
// (e2e/result-overlays/) renders the production PipeViewport over the bundled invented reference
// with the session's standing path. Result colour must draw on the tubes with no added draw call
// or triangle, at the scale colour the overlay's table gives for that member; the reaction
// arrows must add exactly their meshes and change the pixels at each force arrow; turning both
// off must restore the neutral drawing. Display evidence only: no engineering result or claim is
// asserted.

type Vec3 = { x: number; y: number; z: number };
type Segment = { from: Vec3; to: Vec3 };
type Arrow = { supportId: string; anchor: Vec3; direction: Vec3; displayLength: number };
type Harness = {
  mode: string;
  theme: "light" | "dark";
  eligible: boolean;
  healthHeadline: string | null;
  quantityLabel: string | null;
  caseLabel: string | null;
  colourState: string;
  noValueCount: number;
  memberCount: number;
  pipes: Array<{ id: string; from: Vec3; to: Vec3; colour: number | null }>;
  nodes: Vec3[];
  deformedPipes: Segment[];
  rotationGlyphs: Segment[];
  reactionState: string;
  withheldCount: number;
  forces: Arrow[];
  moments: Arrow[];
  forceChunks: number;
  momentChunks: number;
};
type Point = { x: number; y: number };

// Triangles per drawn mesh instance: a cylinder of 8 radial segments (16 side + 16 cap) and a cone
// of 10 (10 side + 10 base). A force arrow is one cylinder and one cone; a moment glyph is the
// rotation glyph, one cylinder and two cones.
const FORCE_ARROW_TRIANGLES = 32 + 20;
const MOMENT_GLYPH_TRIANGLES = 32 + 2 * 20;
// Must equal REACTION_FORCE_TIP_GAP in viewportReactionArrows.ts; pinned here independently.
const FORCE_TIP_GAP = 0.11;

async function settle(page: Page) {
  await expect.poll(async () => page.evaluate(() => {
    const current = globalThis.__openPipeStressUiDiagnosticsV1?.readCurrent();
    if (!current || "status" in current.viewport) return null;
    const { ownedPendingRafCount, rendererInfo } = current.viewport.resources;
    return ownedPendingRafCount === 0 && rendererInfo.calls > 0;
  }), { timeout: 30_000 }).toBe(true);
  await page.evaluate(() => new Promise((resolve) => requestAnimationFrame(() => requestAnimationFrame(resolve))));
}

async function renderer(page: Page) {
  await settle(page);
  return page.evaluate(() => {
    const current = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
    if ("status" in current.viewport) throw new Error("Viewport unavailable");
    return current.viewport.resources.rendererInfo;
  });
}

async function setPressed(page: Page, testId: string, pressed: boolean) {
  const toggle = page.getByTestId(testId);
  if ((await toggle.getAttribute("aria-pressed")) !== String(pressed)) await toggle.click();
  await expect(toggle).toHaveAttribute("aria-pressed", String(pressed));
}

async function open(page: Page, query: string): Promise<Harness> {
  await page.goto(`/e2e/result-overlays/index.html?${query}`);
  await expect(page.getByTestId("viewport-canvas")).toBeVisible();
  await settle(page);
  // Labels are HTML over the canvas; loads and the grid are drawn near the tubes. Hide them so
  // pixel clips see only the tubes, the deformed shape, the glyphs and the overlays.
  await setPressed(page, "toggle-viewport-labels", false);
  await setPressed(page, "toggle-viewport-loads", false);
  await setPressed(page, "toggle-viewport-grid", false);
  await settle(page);
  return page.evaluate(() => (globalThis as unknown as { __resultOverlayHarness: Harness }).__resultOverlayHarness);
}

async function canvasPoint(page: Page, authoredPoint: Vec3): Promise<Point> {
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

const clip = (center: Point, half = 4) =>
  ({ x: Math.round(center.x) - half, y: Math.round(center.y) - half, width: half * 2 + 1, height: half * 2 + 1 });

/** The drawn colour at one CSS pixel, decoded in the page from a screenshot of that pixel. */
async function pixel(page: Page, point: Point): Promise<[number, number, number]> {
  const png = await page.screenshot({ clip: { x: Math.round(point.x), y: Math.round(point.y), width: 1, height: 1 } });
  return page.evaluate(async (base64) => {
    const image = new Image();
    image.src = `data:image/png;base64,${base64}`;
    await image.decode();
    const canvas = document.createElement("canvas");
    canvas.width = 1;
    canvas.height = 1;
    const context = canvas.getContext("2d")!;
    context.drawImage(image, 0, 0);
    const data = context.getImageData(0, 0, 1, 1).data;
    return [data[0], data[1], data[2]] as [number, number, number];
  }, png.toString("base64"));
}

const channels = (hex: number): [number, number, number] => [(hex >> 16) & 0xff, (hex >> 8) & 0xff, hex & 0xff];
const lerp = (a: Vec3, b: Vec3, t: number): Vec3 => ({ x: a.x + (b.x - a.x) * t, y: a.y + (b.y - a.y) * t, z: a.z + (b.z - a.z) * t });

function distanceToSegment(p: Point, a: Point, b: Point): number {
  const dx = b.x - a.x, dy = b.y - a.y;
  const length = dx * dx + dy * dy;
  const t = length > 0 ? Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / length)) : 0;
  return Math.hypot(p.x - (a.x + t * dx), p.y - (a.y + t * dy));
}

/** A coloured tube's centreline point, as far on screen as possible from every node, deformed
 * tube and rotation glyph, so its pixel is the tube's own flat colour. */
async function clearTubePoint(page: Page, harness: Harness) {
  const nodes = await Promise.all(harness.nodes.map((node) => canvasPoint(page, node)));
  const segments = await Promise.all([...harness.deformedPipes, ...harness.rotationGlyphs].map(async (segment) => ({
    from: await canvasPoint(page, segment.from),
    to: await canvasPoint(page, segment.to)
  })));
  let best: { pipe: Harness["pipes"][number]; point: Point; clearance: number } | null = null;
  for (const pipe of harness.pipes) {
    if (pipe.colour === null) continue;
    for (const t of [0.3, 0.4, 0.5, 0.6, 0.7]) {
      const point = await canvasPoint(page, lerp(pipe.from, pipe.to, t));
      const clearance = Math.min(
        ...nodes.map((node) => Math.hypot(node.x - point.x, node.y - point.y)),
        ...segments.map((segment) => distanceToSegment(point, segment.from, segment.to))
      );
      if (!best || clearance > best.clearance) best = { pipe, point, clearance };
    }
  }
  return best;
}

for (const { mode, theme } of [
  { mode: "sparse_interactive", theme: "light" },
  { mode: "dense_scrutiny", theme: "light" },
  { mode: "sparse_interactive", theme: "dark" }
] as const) {
  // The harness fixes the canvas size, so the compact project would repeat this run.
  test(`result colour and reaction arrows draw and clear in a real browser (${mode}, ${theme}) @explicit-viewport`, async ({ page }, info) => {
    const harness = await open(page, `mode=${mode}&theme=${theme}`);
    expect(harness.eligible).toBe(true);
    expect(harness.colourState).toBe("available");
    expect(harness.reactionState).toBe("available");
    expect(harness.forces.length).toBeGreaterThan(0);

    // Off by default: no legend; the toggles are there because a result is.
    await expect(page.getByTestId("viewport-result-legend")).toHaveCount(0);
    await expect(page.getByTestId("toggle-viewport-result-colour")).toHaveAttribute("aria-pressed", "false");
    await expect(page.getByTestId("toggle-viewport-reactions")).toHaveAttribute("aria-pressed", "false");
    const neutral = await renderer(page);
    const sample = (await clearTubePoint(page, harness))!;
    expect(sample.clearance, "a tube point clear of other marks").toBeGreaterThan(6);
    const neutralPixel = await pixel(page, sample.point);
    const neutralClip = await page.screenshot({ clip: clip(sample.point) });
    // Arrows are proportional to the largest reaction, so a small one can be a few pixels long or
    // hidden inside its node's own marks; the pixel check takes the arrows of at least 0.2 units.
    const visibleForces = harness.forces.filter((arrow) => arrow.displayLength >= 0.2);
    expect(visibleForces.length).toBeGreaterThanOrEqual(2);
    const forcePoints = await Promise.all(visibleForces.map((arrow) => canvasPoint(page, {
      x: arrow.anchor.x - arrow.direction.x * (FORCE_TIP_GAP + arrow.displayLength / 2),
      y: arrow.anchor.y - arrow.direction.y * (FORCE_TIP_GAP + arrow.displayLength / 2),
      z: arrow.anchor.z - arrow.direction.z * (FORCE_TIP_GAP + arrow.displayLength / 2)
    })));
    await info.attach(`overlays-off-${mode}-${theme}`, { body: await page.getByTestId("viewport-canvas").screenshot(), contentType: "image/png" });

    // Result colour on: the legend states quantity, unit, case, ticks, range, no-value count and the standing.
    await setPressed(page, "toggle-viewport-result-colour", true);
    const legend = page.getByTestId("viewport-result-colour-legend");
    await expect(legend).toHaveAttribute("data-state", "available");
    await expect(page.getByTestId("viewport-result-colour-title")).toContainText(`${harness.quantityLabel} [mm] · ${harness.caseLabel}`);
    for (let index = 0; index < 5; index += 1) await expect(page.getByTestId(`viewport-result-colour-tick-${index}`)).toContainText("mm");
    await expect(page.getByTestId("viewport-result-colour-range")).toContainText("max");
    await expect(page.getByTestId("viewport-result-colour-no-value")).toContainText(`No value: ${harness.noValueCount} of ${harness.memberCount} members`);
    await expect(page.getByTestId("viewport-overlay-standing")).toContainText(harness.healthHeadline!);
    const coloured = await renderer(page);
    // Colour is a repaint of existing instances: no draw call and no triangle is added.
    expect(coloured.calls).toBe(neutral.calls);
    expect(coloured.triangles).toBe(neutral.triangles);
    const colouredPixel = await pixel(page, sample.point);
    const expected = channels(sample.pipe.colour!);
    for (let index = 0; index < 3; index += 1) {
      expect(Math.abs(colouredPixel[index] - expected[index]), `${sample.pipe.id} channel ${index}: ${colouredPixel} vs ${expected}`).toBeLessThanOrEqual(8);
    }
    expect(colouredPixel).not.toEqual(neutralPixel);
    await info.attach(`result-colour-${mode}-${theme}`, { body: await page.getByTestId("viewport-canvas").screenshot(), contentType: "image/png" });
    // The arrow baselines are taken with colour already drawn, so only the arrows can change them.
    const forceClipsBefore = [];
    for (const point of forcePoints) forceClipsBefore.push(await page.screenshot({ clip: clip(point) }));

    // Reaction arrows on: exactly their meshes are added, and every force arrow's pixels change.
    await setPressed(page, "toggle-viewport-reactions", true);
    await expect(page.getByTestId("viewport-reaction-legend")).toHaveAttribute("data-state", "available");
    await expect(page.getByTestId("viewport-reaction-summary")).toContainText(`Force arrows ${harness.forces.length}`);
    if (harness.withheldCount > 0) await expect(page.getByTestId("viewport-reaction-withheld")).toContainText("Ambiguous attribution is withheld");
    const withArrows = await renderer(page);
    expect(withArrows.calls - coloured.calls).toBe(2 * harness.forceChunks + 2 * harness.momentChunks);
    expect(withArrows.triangles - coloured.triangles).toBe(FORCE_ARROW_TRIANGLES * harness.forces.length + MOMENT_GLYPH_TRIANGLES * harness.moments.length);
    for (const [index, point] of forcePoints.entries()) {
      expect(await page.screenshot({ clip: clip(point) }), `force arrow at ${visibleForces[index].supportId}`).not.toEqual(forceClipsBefore[index]);
    }
    await info.attach(`result-overlays-${mode}-${theme}`, { body: await page.getByTestId("viewport-canvas").screenshot(), contentType: "image/png" });

    // Both off: the legend goes, the renderer draws what it drew, and the tube is neutral again.
    await setPressed(page, "toggle-viewport-result-colour", false);
    await setPressed(page, "toggle-viewport-reactions", false);
    await expect(page.getByTestId("viewport-result-legend")).toHaveCount(0);
    const cleared = await renderer(page);
    expect(cleared.calls).toBe(neutral.calls);
    expect(cleared.triangles).toBe(neutral.triangles);
    expect(await pixel(page, sample.point)).toEqual(neutralPixel);
    expect(await page.screenshot({ clip: clip(sample.point) })).toEqual(neutralClip);
  });
}
