import { writeFile } from "node:fs/promises";
import { gotoRoutedFixture, selectTreeRow, ensureRail, setAppearance, expectCenterUnobscured, expectPassiveOrientationFrame, keyboardMeasureTargets, attachBrowserIdentity, captureState } from "./ui-foundation-workflows";
import { expect, test, type Page } from "@playwright/test";
import { openWorkspaceSection, selectTreeEntity, startPropertyTaskFromCurrentSelection } from "./workspace-driver";

async function expectCanvasInWorkspace(page: Page) {
  await expect.poll(async () => page.evaluate(() => {
    const canvas = document.querySelector("canvas")?.getBoundingClientRect();
    const host = document.querySelector('[data-testid="viewport-canvas"]')?.getBoundingClientRect();
    return Boolean(canvas && host && Math.abs(canvas.width - host.width) < 2 && Math.abs(canvas.height - host.height) < 2);
  })).toBe(true);
  const measured = await page.evaluate(() => {
    const canvas = document.querySelector("canvas")!.getBoundingClientRect();
    const workspace = document.querySelector('[data-testid="modeling-workspace"]')!.getBoundingClientRect();
    return { canvas: canvas.toJSON(), workspace: workspace.toJSON(), windowHeight: innerHeight,
      horizontalOverflow: document.body.scrollWidth - document.body.clientWidth };
  });
  expect(measured.horizontalOverflow).toBe(0);
  expect(measured.canvas.width).toBeGreaterThan(0);
  expect(measured.canvas.height).toBeGreaterThan(0);
  expect(measured.canvas.top).toBeGreaterThanOrEqual(measured.workspace.top);
  expect(measured.canvas.bottom).toBeLessThanOrEqual(measured.workspace.bottom + 1);
  expect(measured.canvas.bottom).toBeLessThanOrEqual(measured.windowHeight);
  return measured.canvas;
}

test("workspace preserves the visible model during discovery, routing, property editing and analysis navigation", async ({ page }, testInfo) => {
  // The compact lane exercises the owner-selected initial minimum; this is
  // layout regression evidence, not independent usability acceptance.
  if (testInfo.project.name.includes("compact")) await page.setViewportSize({ width: 1024, height: 768 });
  await page.goto("/");
  await expect(page.getByTestId("workspace-toolbar")).toBeVisible();
  const before = await expectCanvasInWorkspace(page);
  await page.getByTestId("toolkit-entry").click();
  const during = await expectCanvasInWorkspace(page);
  expect(during).toEqual(before);
  await page.getByTestId("toolkit-build.pipe").click();
  await expect(page.getByTestId("viewport-create-pipe-id")).toBeFocused();
  await page.getByTestId("queue-explicit-pipe-intent").scrollIntoViewIfNeeded();
  await expectCanvasInWorkspace(page);
  await page.getByTestId("cancel-pipe-draft").click();
  await selectTreeEntity(page, "pipe", "pipe:P-100");
  if (await page.getByTestId("toggle-inspector").getAttribute("aria-expanded") !== "true") await page.getByTestId("toggle-inspector").click();
  // Slice B3: the table pane collapses only where it is a drawer (below 1280 px here); in the
  // wide Both view it is always open and its chevron says so.
  if (await page.getByTestId("toggle-tree").getAttribute("aria-disabled") === "true") {
    await expect(page.getByTestId("toggle-tree")).toHaveAttribute("title", "Both view keeps the tables open");
  } else if (await page.getByTestId("toggle-tree").getAttribute("aria-expanded") === "true") await page.getByTestId("toggle-tree").click();
  await startPropertyTaskFromCurrentSelection(page, "pipe", "pipe:P-100");
  await expect(page.getByTestId("inspector-text-value")).toContainText("Pump discharge run");
  await expect(page.getByTestId("inspector-dual-unit-display")).toHaveCount(0);
  await expectCanvasInWorkspace(page);
  await page.getByTestId("editor-intent-value").fill("Retained navigation support draft");
  const frozenTask = await page.getByTestId("inspector-frozen-task-target").innerText();
  await page.getByTestId("command-support").click();
  await expect(page.getByTestId("create-support-id")).toBeFocused();
  await expect(page.getByTestId("create-support-intent-panel")).toBeVisible();
  const inspector = page.getByTestId("property-inspector");
  await expect(inspector.getByRole("tab", { name: "Properties", exact: true })).toHaveAttribute("aria-selected", "true");
  await inspector.getByRole("tab", { name: "Task", exact: true }).click();
  await expect(page.getByTestId("inspector-frozen-task-target")).toHaveText(frozenTask);
  await expect(page.getByTestId("editor-intent-value")).toHaveValue("Retained navigation support draft");
  await expectCanvasInWorkspace(page);
  // Slice B3: Loads opens in Table view, where the canvas is not drawn: it keeps its box, hidden,
  // so the renderer is never resized to nothing. Results opens in Both view with the canvas shown;
  // the Analyze page covers the surfaces and the canvas keeps its box underneath.
  const canvasHost = page.getByTestId("viewport-canvas");
  await openWorkspaceSection(page, "loads");
  await expect(page.getByTestId("modeling-workspace")).toHaveAttribute("data-view", "table");
  await expect(canvasHost).toBeHidden();
  await expectCanvasInWorkspace(page);
  await openWorkspaceSection(page, "solve");
  await expectCanvasInWorkspace(page);
  await openWorkspaceSection(page, "results");
  await expect(page.getByTestId("modeling-workspace")).toHaveAttribute("data-view", "both");
  await expect(canvasHost).toBeVisible();
  await expectCanvasInWorkspace(page);
  await page.getByTestId("toolkit-entry").click();
  await page.keyboard.press("Escape");
  await expect(page.getByTestId("toolkit-entry")).toBeFocused();
  await expect(page.locator("#toolkit-commands")).toHaveCount(0);
});

test.beforeAll(async ({ browser }, testInfo) => { await attachBrowserIdentity(browser, testInfo); });

// --- Slice B3: the shell's geometry ------------------------------------------------------------
// Structure and geometry only; no colour, contrast or computed-style assertion is made here.

type Box = { x: number; y: number; width: number; height: number };

async function readShellGeometry(page: Page) {
  return page.evaluate(() => {
    const box = (selector: string): Box | null => {
      const element = document.querySelector<HTMLElement>(selector);
      if (!element) return null;
      const style = getComputedStyle(element);
      if (style.display === "none") return null;
      const r = element.getBoundingClientRect();
      return { x: r.x, y: r.y, width: r.width, height: r.height };
    };
    type Box = { x: number; y: number; width: number; height: number };
    return {
      window: { width: innerWidth, height: innerHeight },
      menuBar: box('[data-testid="app-menu-bar"]'),
      toolbar: box('[data-testid="workspace-toolbar"]'),
      rail: box('[data-testid="stage-rail"]'),
      surfaces: box('[data-testid="modeling-workspace"]'),
      agentStrip: box('[data-testid="agent-strip"]'),
      statusBar: box('[data-testid="workspace-status-bar"]'),
      tablePane: box(".workspace-pane-tree"),
      tabStrip: box('[data-testid="stage-tab-strip"]'),
      canvasPane: box(".workspace-pane-viewport"),
      drawnCanvas: box('[data-testid="viewport-canvas"] canvas'),
      canvasHidden: getComputedStyle(document.querySelector(".workspace-pane-viewport")!).visibility === "hidden",
      inspector: box(".workspace-pane-inspector"),
      paneCounts: [document.querySelectorAll(".workspace-pane-tree").length, document.querySelectorAll(".workspace-pane-inspector").length],
      pageOverflow: document.documentElement.scrollWidth - document.documentElement.clientWidth,
      stored: JSON.parse(localStorage.getItem("chirality.desktop.ui-preferences.v1") ?? "null")
    };
  });
}

async function chooseView(page: Page, view: "table" | "model" | "both") {
  await page.getByTestId(`view-switch-${view}`).click();
  await expect(page.getByTestId("modeling-workspace")).toHaveAttribute("data-view", view);
}

test("D-72 canvases and the 48 / 56 / 44 / 24 px regions at 1440 x 900 under the native-runtime class", { tag: "@explicit-viewport" }, async ({ page }, testInfo) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await expect(page.getByTestId("viewport-canvas").locator("canvas").first()).toBeVisible();

  // The browser runtime keeps the in-app menu bar row, which the native runtime replaces with
  // the OS menu. Both sets of numbers are recorded; D-72's sizes are the native runtime's.
  const browserBoth = await readShellGeometry(page);
  expect(browserBoth.menuBar).not.toBeNull();
  const menuRow = browserBoth.menuBar!.height;
  expect(browserBoth.toolbar).toEqual({ x: 0, y: menuRow, width: 1440, height: 48 });
  expect(browserBoth.surfaces).toEqual({ x: 56, y: menuRow + 48, width: 1340, height: 828 - menuRow });
  expect(browserBoth.tablePane).toMatchObject({ x: 56, width: 737 });
  expect(browserBoth.canvasPane).toEqual({ x: 793, y: menuRow + 48, width: 603, height: 828 - menuRow });

  // The native-runtime marker is what the product itself tests for (services/nativeMenu.ts). It is
  // set after the model has loaded, and the next render (a view switch) drops the in-app menu bar
  // and takes the `native-menu` class. Only view switches follow, which call no service.
  await page.evaluate(() => { (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {}; });
  await chooseView(page, "model");
  await expect(page.getByTestId("desktop-preview-shell")).toHaveClass(/native-menu/);
  await expect(page.getByTestId("app-menu-bar")).toHaveCount(0);

  const model = await readShellGeometry(page);
  expect(model.toolbar).toEqual({ x: 0, y: 0, width: 1440, height: 48 });
  expect(model.rail).toEqual({ x: 0, y: 48, width: 56, height: 828 });
  expect(model.agentStrip).toEqual({ x: 1396, y: 48, width: 44, height: 828 });
  expect(model.statusBar).toEqual({ x: 0, y: 876, width: 1440, height: 24 });
  expect(model.surfaces).toEqual({ x: 56, y: 48, width: 1340, height: 828 });
  // Model view, drawer open: the 280 px drawer under the canvas, the inspector docked at 340 px.
  expect(model.inspector).toEqual({ x: 1056, y: 48, width: 340, height: 828 });
  expect(model.tablePane).toEqual({ x: 56, y: 596, width: 1000, height: 280 });
  expect(model.canvasPane).toEqual({ x: 56, y: 48, width: 1000, height: 548 });
  // Drawer collapsed to its 28 px tab strip by the chevron at the strip's right end: D-72's 1000 x 828.
  const chevron = page.getByTestId("toggle-tree");
  const chevronBox = (await chevron.boundingBox())!;
  expect(chevronBox.x + chevronBox.width).toBeCloseTo(1056, 0);
  await chevron.click();
  await expect(chevron).toHaveAttribute("aria-expanded", "false");
  const collapsed = await readShellGeometry(page);
  expect(collapsed.canvasPane).toEqual({ x: 56, y: 48, width: 1000, height: 828 });
  expect(collapsed.tablePane).toEqual({ x: 56, y: 848, width: 1000, height: 28 });
  expect(collapsed.inspector).toEqual(model.inspector);
  await chevron.click();
  await expect(chevron).toHaveAttribute("aria-expanded", "true");

  // Both view: 55 % / 45 %, D-72's 603 x 828. The inspector borrows from
  // the table, so the canvas keeps its width and moves 300 px left.
  await chooseView(page, "both");
  const both = await readShellGeometry(page);
  expect(both.tablePane).toEqual({ x: 56, y: 48, width: 737, height: 828 });
  expect(both.canvasPane).toEqual({ x: 793, y: 48, width: 603, height: 828 });
  expect(both.inspector).toBeNull();
  await page.getByTestId("toggle-inspector").click();
  const docked = await readShellGeometry(page);
  expect(docked.tablePane).toEqual({ x: 56, y: 48, width: 437, height: 828 });
  expect(docked.canvasPane).toEqual({ x: 493, y: 48, width: 603, height: 828 });
  expect(docked.inspector).toEqual({ x: 1096, y: 48, width: 300, height: 828 });
  await page.getByTestId("inspector-close").click();
  await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");

  // Table view: the table pane at full surface width; no canvas drawn, no inspector. The canvas
  // keeps its box underneath, hidden, so its renderer is not resized.
  await chooseView(page, "table");
  const table = await readShellGeometry(page);
  expect(table.tablePane).toEqual({ x: 56, y: 48, width: 1340, height: 828 });
  expect(table.inspector).toBeNull();
  expect(table.canvasHidden).toBe(true);
  expect(table.canvasPane).toEqual(both.canvasPane);
  await expect(page.getByTestId("viewport-canvas")).toBeHidden();

  for (const geometry of [browserBoth, model, collapsed, both, docked, table]) {
    expect(geometry.paneCounts).toEqual([1, 1]);
    expect(geometry.pageOverflow).toBe(0);
  }
  await testInfo.attach("shell-geometry-1440x900", { body: JSON.stringify({ browserBoth, native: { model, collapsed, both, docked, table } }, null, 2), contentType: "application/json" });
});

// D2 and routing portal: the docked inspector always borrows its 300 px from
// the table first. Arming moves the existing editor into that inspector without
// changing either pane's dimensions; disarming restores prior inspector state.
for (const [width, height, table, canvas] of [[1440, 900, 737, 603], [1280, 800, 649, 531]] as const) {
  test(`routing uses the inspector without changing the D2 pane allocation, ${width} x ${height}, native-runtime class`, { tag: "@explicit-viewport" }, async ({ page }, testInfo) => {
    await page.setViewportSize({ width, height });
    await page.goto("/");
    await expect(page.getByTestId("viewport-canvas").locator("canvas").first()).toBeVisible();
    await page.evaluate(() => { (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {}; });
    await chooseView(page, "model"); await chooseView(page, "both");
    await expect(page.getByTestId("app-menu-bar")).toHaveCount(0);
    const surfacesHeight = height - 48 - 24;
    const intents = page.getByTestId("viewport-editor-intents");
    const surfaces = page.getByTestId("modeling-workspace");

    const closed = await readShellGeometry(page);
    await expect(intents).toHaveClass(/collapsed/);
    await expect(surfaces).not.toHaveAttribute("data-canvas-authoring", "true");
    expect(closed.tablePane).toEqual({ x: 56, y: 48, width: table, height: surfacesHeight });
    expect(closed.canvasPane).toEqual({ x: 56 + table, y: 48, width: canvas, height: surfacesHeight });

    // Arm from a closed inspector: the session opens it and the unchanged
    // authoring panel is portalled into its stable routing container.
    await page.getByTestId("command-pipe").click();
    await expect(intents).toHaveClass(/active/);
    await expect(surfaces).toHaveAttribute("data-canvas-authoring", "true");
    await expect(page.locator("#shell-routing-panel").getByTestId("viewport-editor-intents")).toBeVisible();
    await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "true");
    const armed = await readShellGeometry(page);
    expect(armed.tablePane).toEqual({ x: 56, y: 48, width: table - 300, height: surfacesHeight });
    expect(armed.canvasPane).toEqual({ x: 56 + table - 300, y: 48, width: canvas, height: surfacesHeight });
    expect(armed.inspector).toEqual({ x: width - 44 - 300, y: 48, width: 300, height: surfacesHeight });
    const drawn = (await page.locator(".viewport-canvas canvas").first().boundingBox())!;
    expect(drawn.width).toBeGreaterThanOrEqual(200);
    expect(drawn.height).toBeGreaterThanOrEqual(200);
    await expect(page.getByTestId("command-pipe")).toHaveAttribute("aria-pressed", "true");
    // The 24 px splitter lies wholly on the table side and keeps the engineer's stored split.
    const splitter = (await page.getByTestId("resize-model-tree").boundingBox())!;
    expect(splitter.width).toBe(24);
    expect(splitter.x + splitter.width).toBeCloseTo(56 + table - 300, 0);
    await expect(page.getByTestId("resize-model-tree")).toHaveAttribute("aria-valuenow", "55");

    // The table pane lends down to 320 px and no further; past that the canvas gives the rest.
    await page.getByTestId("resize-model-tree").focus();
    for (let press = 0; press < 13; press += 1) await page.keyboard.press("ArrowLeft");
    await expect(page.getByTestId("resize-model-tree")).toHaveAttribute("aria-valuenow", "29");
    const lendingMinimum = await readShellGeometry(page);
    expect(Math.round(lendingMinimum.tablePane!.width)).toBe(320);
    expect(Math.round(lendingMinimum.canvasPane!.width)).toBe(width - 100 - 320 - 300);
    for (let press = 0; press < 13; press += 1) await page.keyboard.press("ArrowRight");

    // Disarm (Select): the shell-owned inspector closes because it began
    // closed, while the stored split and closed geometry remain unchanged.
    await page.getByTestId("workspace-select").click();
    await expect(intents).toHaveClass(/collapsed/);
    const disarmed = await readShellGeometry(page);
    await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");
    expect(disarmed.tablePane).toEqual(closed.tablePane);
    expect(disarmed.canvasPane).toEqual(closed.canvasPane);

    // If the engineer had it open, disarming leaves it open.
    await page.getByTestId("toggle-inspector").click();
    const openBeforeArm = await readShellGeometry(page);
    await page.getByTestId("command-node").click();
    await page.getByTestId("workspace-select").click();
    await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "true");
    expect(await readShellGeometry(page)).toMatchObject({
      tablePane: openBeforeArm.tablePane,
      canvasPane: openBeforeArm.canvasPane,
      inspector: openBeforeArm.inspector
    });
    await testInfo.attach(`routing-inspector-rule-${width}x${height}`, { body: JSON.stringify({ closed, armed, lendingMinimum, disarmed, openBeforeArm, drawn }, null, 2), contentType: "application/json" });
  });
}

test("the table strip clears DOM readouts and the painted orientation frame in Model and narrow Both views", { tag: "@explicit-viewport" }, async ({ page }, testInfo) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  const model = await gotoRoutedFixture(page);
  await expect(page.getByTestId("viewport-canvas").locator("canvas").first()).toBeVisible();
  const rects = () => page.evaluate(() => {
    const rect = (selector: string) => { const r = document.querySelector(selector)!.getBoundingClientRect(); return { left: r.left, top: r.top, right: r.right, bottom: r.bottom }; };
    return { strip: rect('[data-testid="stage-tab-strip"]'), pane: rect(".workspace-pane-tree"), scaleBar: rect(".viewport-scale-bar"), triad: rect('[data-testid="viewport-axis-triad"]'), readout: rect('[data-testid="viewport-measurement-readout"]'), canvasPane: rect(".workspace-pane-viewport") };
  });
  const intersects = (a: { left: number; top: number; right: number; bottom: number }, b: typeof a) =>
    a.left < b.right && b.left < a.right && a.top < b.bottom && b.top < a.bottom;
  const check = async (label: string) => {
    const readoutTopmost = await page.getByTestId("viewport-measurement-readout").evaluate((element) => {
      const rect = element.getBoundingClientRect();
      const point = { x: rect.left + rect.width / 2, y: rect.top + rect.height / 2 };
      const owner = document.elementFromPoint(point.x, point.y);
      const ancestry = [];
      for (let node = owner; node; node = node.parentElement) {
        ancestry.push({ tag: node.tagName, id: node.id, class: node.getAttribute("class"), testId: node.getAttribute("data-testid") });
      }
      return { rect: rect.toJSON(), point, owner: owner?.getAttribute("data-testid") ?? owner?.getAttribute("aria-label") ?? owner?.tagName ?? null,
        ownerClass: owner?.getAttribute("class"), ownerRect: owner?.getBoundingClientRect().toJSON(), ancestry };
    });
    const witnessPath = testInfo.outputPath(`${label.replace(/[^a-z0-9]+/gi, "-").toLowerCase()}-readout-topmost.json`);
    await writeFile(witnessPath, JSON.stringify({ ...readoutTopmost, rectangles: await rects() }, null, 2));
    await testInfo.attach("readout-topmost", { path: witnessPath, contentType: "application/json" });
    await expectCenterUnobscured(page.getByTestId("viewport-measurement-readout"));
    const measured = await rects();
    const orientation = await expectPassiveOrientationFrame(page, testInfo, label.replace(/[^a-z0-9]+/gi, "-").toLowerCase());
    const painted = { left: orientation.clip.x, top: orientation.clip.y, right: orientation.clip.x + orientation.clip.width, bottom: orientation.clip.y + orientation.clip.height };
    for (const furniture of ["scaleBar", "triad", "readout"] as const) {
      expect(intersects(measured[furniture], measured.pane), `${label}: ${furniture} ${JSON.stringify(measured[furniture])} against the table pane ${JSON.stringify(measured.pane)}`).toBe(false);
      expect(measured[furniture].bottom).toBeLessThanOrEqual(measured.canvasPane.bottom);
    }
    expect(intersects(painted, measured.pane), `${label}: painted orientation ${JSON.stringify(painted)} against the table pane ${JSON.stringify(measured.pane)}`).toBe(false);
    expect(painted.bottom).toBeLessThanOrEqual(measured.canvasPane.bottom);
    return { ...measured, painted, orientation };
  };
  await chooseView(page, "model");
  const measurementPipe = model.pipe_segments[10];
  await selectTreeRow(page, "pipe", measurementPipe.id);
  await page.getByTestId("clear-model-tree-filter").click();
  await keyboardMeasureTargets(page, [`Select ${measurementPipe.label} in viewport`]);
  await check("Model view, drawer open");
  await page.getByTestId("toggle-tree").click();
  await expect(page.getByTestId("toggle-tree")).toHaveAttribute("aria-expanded", "false");
  const collapsed = await check("Model view, drawer collapsed");
  expect(collapsed.canvasPane.bottom).toBeCloseTo(collapsed.pane.bottom, 0);
  await page.getByTestId("toggle-tree").click();
  // In narrow Both view the expanded drawer is in flow; only its collapsed
  // 28 px strip overlays the unchanged drawn canvas.
  await chooseView(page, "both");
  await page.setViewportSize({ width: 1024, height: 768 });
  await ensureRail(page, "tree", true);
  await check("below 1280 px, drawer open");
  await ensureRail(page, "tree", false);
  await check("below 1280 px, drawer collapsed");
});

test("a switch to Table view hides the canvas without resizing its renderer", { tag: "@explicit-viewport" }, async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.goto("/");
  const canvas = page.getByTestId("viewport-canvas").locator("canvas").first();
  await expect(canvas).toBeVisible();
  const buffer = () => canvas.evaluate((element) => { const c = element as HTMLCanvasElement; const r = c.getBoundingClientRect(); return { bufferWidth: c.width, bufferHeight: c.height, cssWidth: r.width, cssHeight: r.height }; });
  const camera = () => page.evaluate(() => { const snapshot = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent(); if ("status" in snapshot.viewport) throw new Error("Viewport unavailable"); return snapshot.viewport.camera; });
  const before = { buffer: await buffer(), camera: await camera() };
  expect(before.buffer.bufferWidth).toBeGreaterThan(1);
  await chooseView(page, "table");
  await expect(page.getByTestId("viewport-canvas")).toBeHidden();
  await page.waitForTimeout(300);
  expect(await buffer()).toEqual(before.buffer);
  expect(await camera()).toEqual(before.camera);
  await chooseView(page, "both");
  await expect(canvas).toBeVisible();
  expect(await buffer()).toEqual(before.buffer);
  expect(await camera()).toEqual(before.camera);
});

test("browser native-class structural review captures retain pane and drawn-canvas geometry", { tag: "@explicit-viewport" }, async ({ page }, testInfo) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await gotoRoutedFixture(page);
  await page.evaluate(() => { (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {}; });
  await chooseView(page, "model");
  await chooseView(page, "both");
  await expect(page.getByTestId("desktop-preview-shell")).toHaveClass(/native-menu/);
  const records: Array<{ name: string; geometry: Awaited<ReturnType<typeof readShellGeometry>> }> = [];
  const capture = async (name: string) => {
    const geometry = await readShellGeometry(page);
    expect(geometry.pageOverflow, name).toBeLessThanOrEqual(1);
    if (geometry.drawnCanvas && geometry.canvasPane) {
      expect(geometry.drawnCanvas.x, name).toBeGreaterThanOrEqual(geometry.canvasPane.x);
      expect(geometry.drawnCanvas.y, name).toBeGreaterThanOrEqual(geometry.canvasPane.y);
      expect(geometry.drawnCanvas.x + geometry.drawnCanvas.width, name).toBeLessThanOrEqual(geometry.canvasPane.x + geometry.canvasPane.width + 1);
      expect(geometry.drawnCanvas.y + geometry.drawnCanvas.height, name).toBeLessThanOrEqual(geometry.canvasPane.y + geometry.canvasPane.height + 1);
    }
    records.push({ name, geometry });
    await captureState(page, testInfo, `browser-native-class-${name}`);
  };

  await capture("both-inspector-closed");
  await ensureRail(page, "inspector", true);
  await capture("both-inspector-open");
  await page.getByTestId("command-pipe").click();
  await expect(page.locator("#shell-routing-panel").getByTestId("viewport-editor-intents")).toBeVisible();
  await capture("both-routing");
  await page.getByTestId("workspace-select").click();
  await chooseView(page, "model");
  await ensureRail(page, "tree", true);
  await capture("model-drawer-open");
  await ensureRail(page, "tree", false);
  await capture("model-drawer-collapsed");
  await chooseView(page, "both");
  await page.setViewportSize({ width: 1024, height: 768 });
  await ensureRail(page, "inspector", false);
  await ensureRail(page, "tree", true);
  await capture("narrow-both-drawer-open");
  await testInfo.attach("browser-native-class-structural-geometry", { body: JSON.stringify(records, null, 2), contentType: "application/json" });
});

for (const viewport of [{ width: 1440, height: 920 }, { width: 1280, height: 800 }]) {
  for (const [theme, density] of [["light", "comfortable"], ["dark", "compact"]] as const) {
    test(`the split and the drawer resize actual panes, persist, and keep the inspector's controls contained ${theme} ${viewport.width}`, { tag: "@explicit-viewport" }, async ({ page }, testInfo) => {
      await page.setViewportSize(viewport);
      const model = await gotoRoutedFixture(page, "ui-foundation-1000.model.json");
      await setAppearance(page, theme, density);
      const pipe = model.pipe_segments.find((entry: any) => entry.section_ref)!;
      expect(pipe).toBeTruthy();
      await selectTreeRow(page, "pipe", pipe.id);
      await ensureRail(page, "tree", true); await ensureRail(page, "inspector", true);
      const surfaceWidth = viewport.width - 56 - 44;
      const witnesses: unknown[] = [];
      const inspectorChildren = () => page.evaluate(() => {
        const pane = document.querySelector<HTMLElement>(".workspace-pane-inspector")!;
        const inspector = document.querySelector<HTMLElement>('[data-testid="property-inspector"]')!;
        const assignment = document.querySelector<HTMLElement>("#section-assignment");
        return { pane: pane.getBoundingClientRect().toJSON(), paneOverflow: pane.scrollWidth - pane.clientWidth, inspectorOverflow: inspector.scrollWidth - inspector.clientWidth,
          children: assignment ? [...assignment.querySelectorAll<HTMLElement>("p, select, button")].map((element) => ({ tag: element.tagName, text: element.textContent, rect: element.getBoundingClientRect().toJSON(), overflow: element.scrollWidth - element.clientWidth })) : [] };
      });
      const check = async (splitPct: number, label: string) => {
        const closedTableWidth = Math.min(Math.round(surfaceWidth * splitPct / 100), surfaceWidth - 220);
        const lent = Math.max(0, Math.min(300, closedTableWidth - 320));
        const tableWidth = closedTableWidth - lent;
        await expect.poll(async () => Math.round((await readShellGeometry(page)).tablePane!.width)).toBe(tableWidth);
        const geometry = await readShellGeometry(page);
        witnesses.push({ label, ...geometry });
        expect(geometry.pageOverflow).toBeLessThanOrEqual(1);
        // D2: the inspector borrows from the closed table split down to 320 px,
        // then consumes canvas width down to its 220 px minimum.
        expect(Math.round(geometry.inspector!.width)).toBe(300);
        expect(Math.round(geometry.canvasPane!.width)).toBe(surfaceWidth - tableWidth - 300);
        expect(geometry.canvasPane!.width).toBeGreaterThanOrEqual(220);
        const contained = await inspectorChildren();
        expect(contained.paneOverflow).toBeLessThanOrEqual(1);
        expect(contained.inspectorOverflow).toBeLessThanOrEqual(1);
        expect(contained.children.length).toBeGreaterThan(0);
        for (const child of contained.children) {
          expect(child.rect.left, child.text ?? child.tag).toBeGreaterThanOrEqual(contained.pane.left);
          expect(child.rect.right, child.text ?? child.tag).toBeLessThanOrEqual(contained.pane.right + 1);
          expect(child.overflow, child.text ?? child.tag).toBeLessThanOrEqual(1);
        }
        await expect(page.getByTestId("resize-model-tree")).toHaveAttribute("aria-valuenow", String(splitPct));
        expect(geometry.stored.bothSplitPct).toBe(splitPct);
        return geometry;
      };
      const drag = async (id: string, dx: number, dy = 0) => {
        const splitter = page.getByTestId(id); await expectCenterUnobscured(splitter, { minimumTarget: true });
        const box = (await splitter.boundingBox())!;
        await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2); await page.mouse.down();
        await page.mouse.move(box.x + box.width / 2 + dx, box.y + box.height / 2 + dy, { steps: 8 }); await page.mouse.up();
      };
      const initial = await check(55, "default split");
      // A drag changes the stored closed split. The open layout applies D2's
      // lending rule while keeping the stored percentage unchanged.
      await drag("resize-model-tree", -Math.round(surfaceWidth / 10)); const narrower = await check(45, "pointer, table narrower");
      expect(narrower.canvasPane!.width - initial.canvasPane!.width).toBeCloseTo(initial.tablePane!.width - narrower.tablePane!.width, 0);
      for (const preset of ["Front", "Top", "Isometric"]) {
        await page.getByRole("button", { name: preset, exact: true }).click();
        for (const command of ["Fit Visible", "Fit Selection"]) {
          await page.getByRole("button", { name: command, exact: true }).click();
          const points = (command === "Fit Visible" ? model.nodes : model.nodes.filter((node: any) => node.id === pipe.from || node.id === pipe.to)).map((node: any) => node.position);
          const fit = await page.evaluate((points) => {
            const api = globalThis.__openPipeStressUiDiagnosticsV1;
            const snapshot = api.readCurrent();
            if ("status" in snapshot.viewport) throw new Error("Viewport unavailable");
            return { camera: snapshot.viewport.camera, projections: points.map((authoredPoint: any) => api.projectAuthoredPoint({ modelGeneration: snapshot.model.generation, cameraSequence: snapshot.viewport.camera.sequence, authoredPoint })) };
          }, points);
          expect(fit.projections.length).toBeGreaterThan(0);
          for (const projection of fit.projections) {
            expect(projection.status).toBe("available");
            if (projection.status !== "available") throw new Error("Projection unavailable");
            expect(projection.insideClosedNdc).toBe(true);
          }
          witnesses.push({ label: `supported split-driven ${preset} ${command}`, ...fit });
        }
      }
      // The splitter stops where the canvas would fall under 220 px: the stored share is bounded at 85 %,
      // and with the inspector docked the layout holds the canvas at its minimum whatever is stored.
      await drag("resize-model-tree", surfaceWidth);
      await expect(page.getByTestId("resize-model-tree")).toHaveAttribute("aria-valuenow", "85");
      const clamped = await readShellGeometry(page);
      expect(Math.round(clamped.canvasPane!.width)).toBe(220);
      expect(Math.round(clamped.inspector!.width)).toBe(300);
      expect(Math.round(clamped.tablePane!.width)).toBe(surfaceWidth - 520);
      expect(clamped.pageOverflow).toBeLessThanOrEqual(1);
      witnesses.push({ label: "canvas minimum", ...clamped });
      await drag("resize-model-tree", -surfaceWidth);
      await expect(page.getByTestId("resize-model-tree")).toHaveAttribute("aria-valuenow", "15");
      await page.getByTestId("resize-model-tree").focus();
      for (let press = 0; press < 20; press += 1) await page.keyboard.press("ArrowRight");
      await check(55, "keyboard steps of two points");
      await page.keyboard.press("ArrowLeft");
      await check(53, "keyboard step back");
      await page.reload(); await selectTreeRow(page, "pipe", pipe.id); await ensureRail(page, "inspector", true);
      await check(53, "reload persisted split");

      // The Model view's drawer: 280 px by default, resized by its splitter, persisted, collapsible to 28 px.
      await chooseView(page, "model");
      const drawerDefault = await readShellGeometry(page);
      expect(Math.round(drawerDefault.tablePane!.height)).toBe(280);
      expect(Math.round(drawerDefault.inspector!.width)).toBe(340);
      await drag("resize-task-dock", 0, -60);
      await expect(page.getByTestId("resize-task-dock")).toHaveAttribute("aria-valuenow", "340");
      await expect.poll(async () => Math.round((await readShellGeometry(page)).tablePane!.height)).toBe(340);
      await page.getByTestId("resize-task-dock").focus(); await page.keyboard.press("ArrowDown");
      await expect(page.getByTestId("resize-task-dock")).toHaveAttribute("aria-valuenow", "324");
      const resized = await readShellGeometry(page);
      expect(Math.round(resized.tablePane!.height)).toBe(324);
      expect(Math.round(resized.canvasPane!.height + resized.tablePane!.height)).toBe(Math.round(resized.surfaces!.height));
      expect(resized.stored.tableDrawerPx).toBe(324);
      await ensureRail(page, "tree", false);
      await expect(page.getByTestId("resize-task-dock")).toBeHidden();
      const collapsed = await readShellGeometry(page);
      expect(Math.round(collapsed.tablePane!.height)).toBe(28);
      expect(Math.round(collapsed.canvasPane!.height)).toBe(Math.round(collapsed.surfaces!.height));
      await ensureRail(page, "tree", true);
      await expect.poll(async () => Math.round((await readShellGeometry(page)).tablePane!.height)).toBe(324);
      await testInfo.attach("actual-shell-geometry", { body: JSON.stringify(witnesses, null, 2), contentType: "application/json" });
    });
  }
}

test("below 1280 px the expanded table drawer is in flow, its splitter remains operable, and the inspector stays a focus-restoring slide-over", { tag: "@explicit-viewport" }, async ({ page }, testInfo) => {
  await page.setViewportSize({ width: 1440, height: 920 });
  await gotoRoutedFixture(page, "ui-foundation-1000.model.json");
  await page.getByTestId("resize-model-tree").focus(); await page.keyboard.press("ArrowRight");
  await page.setViewportSize({ width: 1024, height: 768 });
  await ensureRail(page, "tree", false); await ensureRail(page, "inspector", false);
  const before = await readShellGeometry(page);
  // Collapsed, the 28 px strip overlays the canvas foot.
  expect(before.canvasPane).toEqual(before.surfaces);
  expect(Math.round(before.tablePane!.height)).toBe(28);
  expect(before.inspector).toBeNull();
  for (const side of ["tree", "inspector"] as const) {
    await ensureRail(page, side, true);
    const during = await readShellGeometry(page);
    if (side === "tree") {
      expect(Math.round(during.tablePane!.height)).toBe(280);
      expect(Math.round(during.tablePane!.width)).toBe(Math.round(during.surfaces!.width));
      expect(Math.round(during.canvasPane!.height + during.tablePane!.height)).toBe(Math.round(during.surfaces!.height));
      await expect(page.getByTestId("resize-task-dock")).toBeVisible();
      const drawerSplitter = (await page.getByTestId("resize-task-dock").boundingBox())!;
      expect(drawerSplitter.height).toBe(24);
      await page.getByTestId("resize-task-dock").focus();
      for (let press = 0; press < 12; press += 1) await page.keyboard.press("ArrowDown");
      await expect(page.getByTestId("resize-task-dock")).toHaveAttribute("aria-valuenow", "180");
      for (let press = 0; press < 30; press += 1) await page.keyboard.press("ArrowUp");
      await expect(page.getByTestId("resize-task-dock")).toHaveAttribute("aria-valuenow", "600");
      expect((await readShellGeometry(page)).stored.tableDrawerPx).toBe(600);
    } else {
      expect(Math.round(during.inspector!.width)).toBe(300);
      expect(Math.round(during.inspector!.x + during.inspector!.width)).toBe(Math.round(during.surfaces!.x + during.surfaces!.width));
    }
    if (side === "inspector") expect(during.canvasPane).toEqual(before.canvasPane);
    expect(during.pageOverflow).toBeLessThanOrEqual(1);
    await expect(page.getByTestId("resize-model-tree")).toBeHidden();
    const control = side === "tree" ? page.getByTestId("model-tree-filter-input") : page.getByTestId("property-inspector").getByRole("tab", { name: "Task", exact: true });
    await control.focus(); await page.keyboard.press("Escape");
    await expect(page.getByTestId(side === "tree" ? "toggle-tree" : "toggle-inspector")).toBeFocused();
    await expect(page.getByTestId(side === "tree" ? "toggle-tree" : "toggle-inspector")).toHaveAttribute("aria-expanded", "false");
  }
  await testInfo.attach("drawer-geometry", { body: JSON.stringify({ before, after: await readShellGeometry(page) }, null, 2), contentType: "application/json" });
});
