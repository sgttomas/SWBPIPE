import { gotoRoutedFixture, selectTreeRow, ensureRail, setAppearance, expectCenterUnobscured, attachBrowserIdentity } from "./ui-foundation-workflows";
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

test("D-72 canvases and the 48 / 56 / 44 / 24 px regions at 1440 x 900 under the native-runtime class", async ({ page }, testInfo) => {
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

  // Both view: 55 % / 45 %, D-72's 603 x 828; the inspector docks at 300 px and takes its width
  // from the canvas, never from the table.
  await chooseView(page, "both");
  const both = await readShellGeometry(page);
  expect(both.tablePane).toEqual({ x: 56, y: 48, width: 737, height: 828 });
  expect(both.canvasPane).toEqual({ x: 793, y: 48, width: 603, height: 828 });
  expect(both.inspector).toBeNull();
  await page.getByTestId("toggle-inspector").click();
  const docked = await readShellGeometry(page);
  expect(docked.tablePane).toEqual(both.tablePane);
  expect(docked.canvasPane).toEqual({ x: 793, y: 48, width: 303, height: 828 });
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

for (const viewport of [{ width: 1440, height: 920 }, { width: 1280, height: 800 }]) {
  for (const [theme, density] of [["light", "comfortable"], ["dark", "compact"]] as const) {
    test(`the split and the drawer resize actual panes, persist, and keep the inspector's controls contained ${theme} ${viewport.width}`, async ({ page }, testInfo) => {
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
        const tableWidth = Math.round(surfaceWidth * splitPct / 100);
        await expect.poll(async () => Math.round((await readShellGeometry(page)).tablePane!.width)).toBe(tableWidth);
        const geometry = await readShellGeometry(page);
        witnesses.push({ label, ...geometry });
        expect(geometry.pageOverflow).toBeLessThanOrEqual(1);
        // The inspector is 300 px and the canvas takes what is left; the table never gives way to the inspector.
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
      // A drag of one tenth of the surface moves the split ten points; the canvas gives exactly what the table takes.
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

test("below 1280 px the table drawer and the inspector lie over the canvas, keep its geometry, and return focus to their openers", async ({ page }, testInfo) => {
  await page.setViewportSize({ width: 1440, height: 920 });
  await gotoRoutedFixture(page, "ui-foundation-1000.model.json");
  await page.getByTestId("resize-model-tree").focus(); await page.keyboard.press("ArrowRight");
  await page.setViewportSize({ width: 1024, height: 768 });
  await ensureRail(page, "tree", false); await ensureRail(page, "inspector", false);
  const before = await readShellGeometry(page);
  // The narrow fallback: the canvas takes the surface; the table pane is its 28 px strip at the foot.
  expect(before.canvasPane).toEqual(before.surfaces);
  expect(Math.round(before.tablePane!.height)).toBe(28);
  expect(before.inspector).toBeNull();
  for (const side of ["tree", "inspector"] as const) {
    await ensureRail(page, side, true);
    const during = await readShellGeometry(page);
    if (side === "tree") {
      expect(Math.round(during.tablePane!.height)).toBe(280);
      expect(Math.round(during.tablePane!.width)).toBe(Math.round(during.surfaces!.width));
    } else {
      expect(Math.round(during.inspector!.width)).toBe(300);
      expect(Math.round(during.inspector!.x + during.inspector!.width)).toBe(Math.round(during.surfaces!.x + during.surfaces!.width));
    }
    expect(during.canvasPane).toEqual(before.canvasPane);
    expect(during.pageOverflow).toBeLessThanOrEqual(1);
    await expect(page.getByTestId("resize-model-tree")).toBeHidden();
    const control = side === "tree" ? page.getByTestId("model-tree-filter-input") : page.getByTestId("property-inspector").getByRole("tab", { name: "Task", exact: true });
    await control.focus(); await page.keyboard.press("Escape");
    await expect(page.getByTestId(side === "tree" ? "toggle-tree" : "toggle-inspector")).toBeFocused();
    await expect(page.getByTestId(side === "tree" ? "toggle-tree" : "toggle-inspector")).toHaveAttribute("aria-expanded", "false");
  }
  await testInfo.attach("drawer-geometry", { body: JSON.stringify({ before, after: await readShellGeometry(page) }, null, 2), contentType: "application/json" });
});
