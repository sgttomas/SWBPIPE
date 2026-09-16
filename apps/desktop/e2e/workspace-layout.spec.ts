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
  await page.getByTestId("toggle-tree").click();
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
  for (const section of ["loads", "solve", "results"] as const) {
    await openWorkspaceSection(page, section);
    await expectCanvasInWorkspace(page);
  }
  await page.getByTestId("toolkit-entry").click();
  await page.keyboard.press("Escape");
  await expect(page.getByTestId("toolkit-entry")).toBeFocused();
  await expect(page.locator("#toolkit-commands")).toHaveCount(0);
});

test.beforeAll(async ({ browser }, testInfo) => { await attachBrowserIdentity(browser, testInfo); });

async function readRailGeometry(page: Page) {
  return page.evaluate(() => {
    const rect = (selector: string) => document.querySelector(selector)!.getBoundingClientRect().toJSON();
    const pane = document.querySelector<HTMLElement>(".workspace-pane-inspector")!;
    const inspector = document.querySelector<HTMLElement>('[data-testid="property-inspector"]')!;
    const assignment = document.querySelector<HTMLElement>("#section-assignment");
    return { tree: rect(".workspace-pane-tree"), inspector: rect(".workspace-pane-inspector"), canvas: rect('[data-testid="viewport-canvas"] canvas'),
      pageOverflow: document.documentElement.scrollWidth - document.documentElement.clientWidth,
      inspectorOverflow: inspector.scrollWidth - inspector.clientWidth,
      paneOverflow: pane.scrollWidth - pane.clientWidth,
      children: assignment ? [...assignment.querySelectorAll<HTMLElement>("p, select, button")].map((element) => ({ tag: element.tagName, text: element.textContent, rect: element.getBoundingClientRect().toJSON(), overflow: element.scrollWidth - element.clientWidth })) : [],
      stored: JSON.parse(localStorage.getItem("chirality.desktop.ui-preferences.v1") ?? "null") };
  });
}

for (const viewport of [{ width: 1440, height: 920 }, { width: 1280, height: 800 }]) {
  for (const [theme, density] of [["light", "comfortable"], ["dark", "compact"]] as const) {
    test(`rail preferences resize actual panes and preserve contained controls ${theme} ${viewport.width}`, async ({ page }, testInfo) => {
      await page.setViewportSize(viewport);
      const model = await gotoRoutedFixture(page, "ui-foundation-1000.model.json");
      await setAppearance(page, theme, density);
      const pipe = model.pipe_segments.find((entry: any) => entry.section_ref)!;
      expect(pipe).toBeTruthy();
      await selectTreeRow(page, "pipe", pipe.id);
      await ensureRail(page, "tree", true); await ensureRail(page, "inspector", true);
      const witnesses = [];
      const check = async (left: number, right: number, label: string) => {
        await expect.poll(async () => Math.round((await readRailGeometry(page)).tree.width)).toBe(left);
        await expect.poll(async () => Math.round((await readRailGeometry(page)).inspector.width)).toBe(right);
        const geometry = await readRailGeometry(page);
        witnesses.push({ label, ...geometry });
        expect(geometry.pageOverflow).toBeLessThanOrEqual(1);
        expect(geometry.canvas.width).toBeGreaterThan(0); expect(geometry.canvas.height).toBeGreaterThan(0);
        expect(geometry.paneOverflow).toBeLessThanOrEqual(1);
        expect(geometry.inspectorOverflow).toBeLessThanOrEqual(1);
        expect(geometry.children.length).toBeGreaterThan(0);
        for (const child of geometry.children) {
          expect(child.rect.left, child.text ?? child.tag).toBeGreaterThanOrEqual(geometry.inspector.left);
          expect(child.rect.right, child.text ?? child.tag).toBeLessThanOrEqual(geometry.inspector.right + 1);
          expect(child.overflow, child.text ?? child.tag).toBeLessThanOrEqual(1);
        }
        await expect(page.getByTestId("resize-model-tree")).toHaveAttribute("aria-valuenow", String(left));
        await expect(page.getByTestId("resize-property-inspector")).toHaveAttribute("aria-valuenow", String(right));
        expect(geometry.stored.leftRailPx).toBe(left); expect(geometry.stored.rightRailPx).toBe(right);
        return geometry;
      };
      const drag = async (id: string, dx: number) => {
        const splitter = page.getByTestId(id); await expectCenterUnobscured(splitter, { minimumTarget: true });
        const box = (await splitter.boundingBox())!;
        await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2); await page.mouse.down();
        await page.mouse.move(box.x + box.width / 2 + dx, box.y + box.height / 2, { steps: 8 }); await page.mouse.up();
      };
      const initial = await check(280, 340, "defaults");
      await drag("resize-model-tree", 140); const leftMax = await check(420, 340, "tree pointer max");
      expect(initial.canvas.width - leftMax.canvas.width).toBeCloseTo(140, 0);
      await drag("resize-property-inspector", -180); const bothMax = await check(420, 520, "both pointer max");
      expect(leftMax.canvas.width - bothMax.canvas.width).toBeCloseTo(180, 0);
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
          witnesses.push({ label: `supported rail-driven ${preset} ${command}`, ...fit });
        }
      }
      await drag("resize-model-tree", -240); await drag("resize-property-inspector", 280);
      await check(220, 280, "pointer lower clamps");
      await page.getByTestId("resize-model-tree").focus(); await page.keyboard.press("ArrowRight");
      await page.getByTestId("resize-property-inspector").focus(); await page.keyboard.press("ArrowRight");
      await check(236, 296, "keyboard steps");
      await page.reload(); await selectTreeRow(page, "pipe", pipe.id);
      await check(236, 296, "reload persisted geometry");
      await ensureRail(page, "tree", false); await ensureRail(page, "inspector", false);
      await expect(page.getByTestId("resize-model-tree")).toBeHidden(); await expect(page.getByTestId("resize-property-inspector")).toBeHidden();
      const collapsed = await readRailGeometry(page);
      expect(collapsed.tree.width).toBeCloseTo(30, 0); expect(collapsed.inspector.width).toBeCloseTo(30, 0);
      await ensureRail(page, "tree", true); await ensureRail(page, "inspector", true);
      await check(236, 296, "reexpanded persisted geometry");
      await testInfo.attach("actual-rail-geometry", { body: JSON.stringify(witnesses, null, 2), contentType: "application/json" });
    });
  }
}

test("1024 drawers retain overlay geometry and keyboard focus after desktop rail preferences", async ({ page }, testInfo) => {
  await page.setViewportSize({ width: 1440, height: 920 });
  await gotoRoutedFixture(page, "ui-foundation-1000.model.json");
  await page.getByTestId("resize-model-tree").focus(); await page.keyboard.press("ArrowRight");
  await page.getByTestId("resize-property-inspector").focus(); await page.keyboard.press("ArrowRight");
  await page.setViewportSize({ width: 1024, height: 768 });
  await ensureRail(page, "tree", false); await ensureRail(page, "inspector", false);
  const before = await readRailGeometry(page);
  expect(before.tree.width).toBeCloseTo(40, 0); expect(before.inspector.width).toBeCloseTo(40, 0);
  for (const side of ["tree", "inspector"] as const) {
    await ensureRail(page, side, true);
    const during = await readRailGeometry(page);
    expect(during[side].width).toBeCloseTo(390, 0); expect(during.canvas).toEqual(before.canvas);
    expect(during.pageOverflow).toBeLessThanOrEqual(1);
    await expect(page.getByTestId("resize-model-tree")).toBeHidden(); await expect(page.getByTestId("resize-property-inspector")).toBeHidden();
    const control = side === "tree" ? page.getByTestId("model-tree-filter-input") : page.getByTestId("property-inspector").getByRole("tab", { name: "Task", exact: true });
    await control.focus(); await page.keyboard.press("Escape");
    await expect(page.getByTestId(side === "tree" ? "toggle-tree" : "toggle-inspector")).toBeFocused();
  }
  await testInfo.attach("drawer-geometry", { body: JSON.stringify({ before, after: await readRailGeometry(page) }, null, 2), contentType: "application/json" });
});
