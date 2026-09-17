import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
const defaultPreviewModel = JSON.parse(readFileSync(fileURLToPath(
  new URL("../../../fixtures/product_preview/invented_preview_model.json", import.meta.url),
), "utf8"));
import { expect, test, type Page } from "@playwright/test";
import { startPropertyTaskFromTreeEntity } from "./workspace-driver";
import {
  APPEARANCE_DENSITIES,
  APPEARANCE_THEMES,
  APPEARANCE_VIEWPORTS,
  COMMAND_GROUPS,
  activateWithKeyboard,
  choosePaletteNodeThenSelectWithFocusEvidence,
  attachBrowserIdentity,
  captureElementState,
  captureState,
  commandGroupControl,
  ensureRail,
  expectCenterUnobscured,
  expectPassiveOrientationFrame,
  expectClearOfClosedRailHandles,
  expectContentFits,
  expectFlatTokenBorders,
  expectOverlayAboveWorkspaceControls,
  expectResolvedContrast,
  expectResolvedStyleAndTargets,
  expectSelectedTreeTarget,
  expectTreeRowIdentityReadable,
  expectWorkspaceGeometry,
  gotoRoutedFixture,
  gotoModel,
  readFixture,
  withTypedCollision,
  keyboardMeasureTargets,
  openWorkspaceSection,
  selectTreeRow,
  setAppearance,
} from "./ui-foundation-workflows";

test.beforeAll(async ({ browser }, testInfo) => {
  await attachBrowserIdentity(browser, testInfo);
});

test("[preflight] production dist exposes compact command groups and keyboard measurement", async ({ page }, testInfo) => {
  await page.setViewportSize({ width: 1024, height: 768 });
  const model = await gotoRoutedFixture(page);
  await setAppearance(page, "light", "compact");

  await expect(page.getByTestId("toolkit-entry")).toBeVisible();
  const dialog = page.getByRole("dialog", { name: "Find a modeling tool" });
  const groupWitnesses = {
    Build: "build.node",
    Supports: "supports.restraint",
    Properties: "properties.material",
    Loads: "loads.cases",
    Edit: "edit.remove",
    "Select and View": "view.select",
    Review: "review.pending",
  } as const;
  for (const group of COMMAND_GROUPS) {
    const control = commandGroupControl(page, group);
    await expect(control).toBeVisible();
    await activateWithKeyboard(page, control);
    await expect(dialog).toBeVisible();
    await expect(dialog.getByText(`Showing ${group} commands. Search stays within this group.`, { exact: true })).toBeVisible();
    await expect(dialog.getByTestId(`toolkit-${groupWitnesses[group]}`)).toBeVisible();
    if (group !== "Build") await expect(dialog.getByTestId("toolkit-build.node")).toHaveCount(0);
    await page.keyboard.press("Escape");
    await expect(control).toBeFocused();
  }

  await page.keyboard.press(`${process.platform === "darwin" ? "Meta" : "Control"}+k`);
  await expect(dialog).toBeVisible();
  await expect(dialog.getByTestId("toolkit-build.node")).toBeVisible();
  await expect(dialog.getByTestId("toolkit-loads.cases")).toBeVisible();
  await page.keyboard.press("Escape");

  for (const invoker of ["group", "shortcut"] as const) {
    const trigger = invoker === "group" ? commandGroupControl(page, "Build") : page.getByTestId("toolkit-entry");
    if (invoker === "group") await trigger.click();
    else await page.keyboard.press(`${process.platform === "darwin" ? "Meta" : "Control"}+k`);
    await expect(dialog).toBeVisible();
    const point = { x: 10, y: 738 };
    expect(await page.evaluate(({ x, y }) => document.elementFromPoint(x, y)?.classList.contains("toolkit-backdrop"), point)).toBe(true);
    await page.mouse.click(point.x, point.y);
    await expect(dialog).toHaveCount(0);
    await expect(trigger).toBeFocused();
  }
  await commandGroupControl(page, "Build").click();
  await dialog.getByRole("button", { name: "Close toolkit" }).click();
  await expect(commandGroupControl(page, "Build")).toBeFocused();
  await choosePaletteNodeThenSelectWithFocusEvidence(page, testInfo, "pointer");

  const pipe = model.pipe_segments[10];
  await selectTreeRow(page, "pipe", pipe.id);
  await ensureRail(page, "inspector", true);
  const inspector = page.getByTestId("property-inspector");
  await expect(inspector.getByRole("tab", { name: "Properties", exact: true })).toHaveAttribute("aria-selected", "true");
  await expect(inspector.locator("#inspector-properties-view")).toBeVisible();
  await expect(inspector.getByTestId("editor-intent-panel")).toBeHidden();
  await activateWithKeyboard(page, inspector.getByRole("tab", { name: "Task", exact: true }));
  await expect(inspector.locator("#inspector-properties-view")).toBeHidden();
  await expect(inspector.getByTestId("editor-intent-panel")).toBeVisible();
  await expect(inspector.getByTestId("inspector-task-empty")).toContainText(`Current selection: pipe: ${pipe.id}`);
  await expect(inspector.getByTestId("inspector-start-task")).toBeEnabled();
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toHaveCount(0);
  await expect(inspector.getByTestId("task-action-footer")).toHaveCount(0);
  await expect(inspector.getByTestId("cancel-editor-intent")).toHaveCount(0);
  await activateWithKeyboard(page, page.getByTestId("viewport-fit-selection"));
  const readout = await keyboardMeasureTargets(page, [`Select ${pipe.label} in viewport`]);
  await expect(page.getByTestId("toggle-tree")).toHaveAttribute("aria-expanded", "false");
  await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");
  await expectCenterUnobscured(page.getByRole("button", { name: `Select ${pipe.label} in viewport`, exact: true }), { minimumTarget: true });
  await expectCenterUnobscured(readout);
  await captureState(page, testInfo, "normal-authoring-preflight-light-compact-1024x768-keyboard-measurement");
});

test("[visual-clarification] measurement and real pipe task remain unobscured as narrow drawers close", async ({ page }, testInfo) => {
  await page.setViewportSize({ width: 1440, height: 920 });
  const model = await gotoRoutedFixture(page);
  await setAppearance(page, "light", "comfortable");
  await expect(page.getByTestId("toggle-tree")).toHaveAttribute("aria-expanded", "true");
  await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "true");
  await captureState(page, testInfo, "visual-clarification-1440x920-both-rails-default");

  const pipe = model.pipe_segments[10];
  await selectTreeRow(page, "pipe", pipe.id);
  await page.getByTestId("clear-model-tree-filter").click();
  await activateWithKeyboard(page, page.getByTestId("viewport-fit-selection"));
  const target = page.getByRole("button", { name: `Select ${pipe.label} in viewport`, exact: true });
  const readout = await keyboardMeasureTargets(page, [`Select ${pipe.label} in viewport`]);

  const inspector = page.getByTestId("property-inspector");
  await activateWithKeyboard(page, inspector.getByRole("tab", { name: "Task", exact: true }));
  await expect(inspector.getByTestId("inspector-task-empty")).toContainText(`Current selection: pipe: ${pipe.id}`);
  await expect(inspector.getByTestId("inspector-start-task")).toBeEnabled();
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toHaveCount(0);
  await expect(inspector.getByTestId("task-action-footer")).toHaveCount(0);
  await expect(inspector.getByTestId("cancel-editor-intent")).toHaveCount(0);
  await activateWithKeyboard(page, inspector.getByTestId("inspector-start-task"));
  await inspector.getByTestId("editor-intent-value").fill("Retained through measurement drawer close");
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText(`pipe: ${pipe.id}`);
  const footer = inspector.getByTestId("task-action-footer");
  await expect(footer).toBeVisible();
  await footer.scrollIntoViewIfNeeded();
  const footer1440 = await expectCenterUnobscured(footer);
  await target.focus();
  await expect(target).toBeFocused();
  const target1440 = await expectCenterUnobscured(target, { minimumTarget: true });
  const readout1440 = await expectCenterUnobscured(readout);
  await testInfo.attach("visibility-witness-1440x920-both-rails", {
    body: JSON.stringify({ footer: footer1440, focusedMeasurementTarget: target1440, measurementReadout: readout1440 }, null, 2),
    contentType: "application/json",
  });
  await captureState(page, testInfo, "visual-clarification-1440x920-both-rails-measurement-pipe-task-footer");

  await page.setViewportSize({ width: 1024, height: 768 });
  await ensureRail(page, "tree", false);
  await ensureRail(page, "inspector", true);
  await expect(page.getByTestId("toggle-tree")).toHaveAttribute("aria-expanded", "false");
  await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("Retained through measurement drawer close");
  await captureState(page, testInfo, "visual-clarification-1024x768-inspector-only-measurement-pipe-task-footer");
  const measure = page.getByRole("button", { name: "Measure", exact: true });
  await activateWithKeyboard(page, measure);
  await expect(measure).toHaveAttribute("aria-pressed", "false");
  await ensureRail(page, "inspector", true);
  await activateWithKeyboard(page, measure);
  await expect(measure).toHaveAttribute("aria-pressed", "true");
  await expect(page.getByTestId("toggle-tree")).toHaveAttribute("aria-expanded", "false");
  await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");
  await expect(measure).toBeFocused();
  const measureClosed = await expectCenterUnobscured(measure, { minimumTarget: true });
  await activateWithKeyboard(page, target);
  const readoutClosed = page.getByTestId("viewport-measurement-readout");
  await expect(readoutClosed).toContainText("Distance");
  await expect(readoutClosed).toContainText("ΔX");
  await expect(readoutClosed).toContainText("ΔY");
  await expect(readoutClosed).toContainText("ΔZ");
  await captureState(page, testInfo, "visual-clarification-1024x768-rails-closed-measurement");
  await target.focus();
  await expect(target).toBeFocused();
  const targetClosed = await expectCenterUnobscured(target, { minimumTarget: true });
  const readoutClosedWitness = await expectCenterUnobscured(readoutClosed);
  const gizmoClosed = await expectPassiveOrientationFrame(page, testInfo, "rails-closed-measurement");
  await testInfo.attach("visibility-witness-1024x768-rails-closed", {
    body: JSON.stringify({ measure: measureClosed, focusedMeasurementTarget: targetClosed, measurementReadout: readoutClosedWitness, gizmo: gizmoClosed }, null, 2),
    contentType: "application/json",
  });

  const readSavedPanelSizes = () => page.evaluate(() => {
    const saved = JSON.parse(localStorage.getItem("chirality.desktop.ui-preferences.v1")!);
    return { left: saved.leftRailPx, right: saved.rightRailPx, dock: saved.dockPx };
  });
  const savedPanels = await readSavedPanelSizes();
  await openWorkspaceSection(page, "operations");
  await expect(page.getByTestId("workspace-dock")).not.toHaveClass(/collapsed/);
  await expect(page.getByTestId("workspace-section-operations")).toBeVisible();
  for (const dockState of ["open", "closed"] as const) {
    if (dockState === "closed") await page.getByTestId("workspace-dock-close").click();
    await expect(page.getByTestId("workspace-dock")).toHaveClass(dockState === "closed" ? /collapsed/ : /^(?!.*collapsed)/);
    await target.focus();
    await expect(target).toBeFocused();
    await expectCenterUnobscured(target, { minimumTarget: true });
    await expectCenterUnobscured(readoutClosed);
    await expectContentFits(readoutClosed, `measurement with dock ${dockState}`);
    for (const label of ["Distance", "ΔX", "ΔY", "ΔZ"]) await expect(readoutClosed).toContainText(label);
    const orientation = await expectPassiveOrientationFrame(page, testInfo, `narrow-measurement-dock-${dockState}`);
    expect(orientation.canvas.width).toBeGreaterThanOrEqual(200);
    expect(orientation.canvas.height).toBeGreaterThanOrEqual(200);
    await expect(page.getByTestId("workspace-dock")).toHaveClass(dockState === "closed" ? /collapsed/ : /^(?!.*collapsed)/);
    expect(await readSavedPanelSizes()).toEqual(savedPanels);
    await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("Retained through measurement drawer close");
    await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText(`pipe: ${pipe.id}`);
  }

  await ensureRail(page, "inspector", true);
  await expect(page.getByTestId("toggle-tree")).toHaveAttribute("aria-expanded", "false");
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText(`pipe: ${pipe.id}`);
  await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("Retained through measurement drawer close");
  await expect(inspector.getByTestId("task-action-footer")).toBeVisible();
  await expect(inspector.getByTestId("cancel-editor-intent")).toBeVisible();
  await activateWithKeyboard(page, inspector.getByTestId("cancel-editor-intent"));
  await expect(inspector.getByTestId("inspector-task-empty")).toContainText(`Current selection: pipe: ${pipe.id}`);
  await expect(inspector.getByTestId("inspector-start-task")).toBeEnabled();
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toHaveCount(0);
  await expect(inspector.getByTestId("task-action-footer")).toHaveCount(0);
  await expect(inspector.getByTestId("cancel-editor-intent")).toHaveCount(0);
});

test("[rail-clearance] closed rail handles leave active viewport controls and complete measurement content usable", async ({ page }) => {
  await page.setViewportSize({ width: 1024, height: 768 });
  const model = await gotoRoutedFixture(page);
  const pipe = model.pipe_segments[10];
  await selectTreeRow(page, "pipe", pipe.id);
  await page.getByTestId("clear-model-tree-filter").click();
  await ensureRail(page, "inspector", true);

  const inspector = page.getByTestId("property-inspector");
  await activateWithKeyboard(page, inspector.getByRole("tab", { name: "Task", exact: true }));
  await activateWithKeyboard(page, inspector.getByTestId("inspector-start-task"));
  await inspector.getByTestId("editor-intent-value").fill("Retained across Measure rail cycles");
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText(`pipe: ${pipe.id}`);

  const measure = page.getByRole("button", { name: "Measure", exact: true });
  await activateWithKeyboard(page, measure);
  await expect(measure).toHaveAttribute("aria-pressed", "true");
  await expect(page.getByTestId("toggle-tree")).toHaveAttribute("aria-expanded", "false");
  await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");
  await expect(measure).toBeFocused();
  await expectCenterUnobscured(measure, { minimumTarget: true });

  const target = page.getByRole("button", { name: `Select ${pipe.label} in viewport`, exact: true });
  await activateWithKeyboard(page, target);
  const readout = page.getByTestId("viewport-measurement-readout");
  await expect(readout).toContainText(`Measure · pipe: ${pipe.id}`);
  for (const label of ["Distance", "ΔX", "ΔY", "ΔZ"] as const) {
    const quantity = readout.locator(`[data-measurement-label="${label}"]`);
    await expect(quantity).toBeVisible();
    await expect(quantity).toHaveAttribute("data-raw-unit", "m");
    await expect(quantity).toContainText(new RegExp(`^${label} [+-]?[0-9]+(?:\\.[0-9]+)?(?:e[+-]?[0-9]+)? m$`, "i"));
  }
  await expectContentFits(readout, "complete measurement readout");

  await expectClearOfClosedRailHandles(page, [
    { name: "Node toolbar action", locator: page.getByTestId("command-node") },
    { name: "Load toolbar action", locator: page.getByTestId("command-load") },
    { name: "selection filter", locator: page.getByTestId("viewport-selection-filter") },
    { name: "Actual OD control", locator: page.getByTestId("viewport-geometry-actual-od") },
    { name: "Measure control", locator: measure },
    { name: "Isometric camera control", locator: page.getByTestId("viewport-view-isometric") },
    { name: "measurement target text", locator: readout.locator(":scope > strong") },
    { name: "measurement value text", locator: readout.locator(":scope > span") },
  ]);

  await ensureRail(page, "inspector", true);
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText(`pipe: ${pipe.id}`);
  await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("Retained across Measure rail cycles");
  await activateWithKeyboard(page, measure);
  await expect(measure).toHaveAttribute("aria-pressed", "false");
  await ensureRail(page, "inspector", true);
  await activateWithKeyboard(page, measure);
  await expect(measure).toHaveAttribute("aria-pressed", "true");
  await expect(page.getByTestId("toggle-tree")).toHaveAttribute("aria-expanded", "false");
  await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");
  await expect(measure).toBeFocused();
  await expectCenterUnobscured(measure, { minimumTarget: true });
  await ensureRail(page, "inspector", true);
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText(`pipe: ${pipe.id}`);
  await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("Retained across Measure rail cycles");
});

for (const theme of APPEARANCE_THEMES) for (const density of APPEARANCE_DENSITIES) {
  test(`content-aware narrow canvas budget ${theme} ${density}`, async ({ page }, testInfo) => {
    await page.setViewportSize({ width: 1024, height: 768 });
    for (const content of ["regular", "wrapped"] as const) {
      let model: any;
      if (content === "regular") {
        model = defaultPreviewModel;
        await page.goto("/");
        await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
      } else {
        const original = (await readFixture("precision-origin-base.model.json")).model;
        model = structuredClone(original);
        // Only pipe IDs differ; this exact fixture has no pipe-reference owners.
        expect(original.components).toHaveLength(0);
        expect(original.load_cases).toHaveLength(0);
        expect(original.combinations).toHaveLength(0);
        const mapping = model.pipe_segments.map((pipe: any, index: number) => ({
          original: pipe.id, replacement: `pipe:${"MEASUREMENT-TARGET-".repeat(8)}${index}`,
        }));
        model.pipe_segments.forEach((pipe: any, index: number) => { pipe.id = mapping[index].replacement; });
        const restored = structuredClone(model);
        restored.pipe_segments.forEach((pipe: any, index: number) => { pipe.id = mapping[index].original; });
        expect(restored).toEqual(original);
        await testInfo.attach(`wrapped-id-map-${theme}-${density}`, { body: JSON.stringify(mapping), contentType: "application/json" });
        await gotoModel(page, model);
      }
      await setAppearance(page, theme, density);
      if (content === "regular") {
        await openWorkspaceSection(page, "solve");
        await activateWithKeyboard(page, page.getByTestId("run-mechanics-preview"));
        await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
        await expect(page.getByTestId("solve-job-summary")).toContainText("result_rows=830");
        await page.getByTestId("workspace-dock-close").click();
      }
      const pipe = model.pipe_segments[0];
      await selectTreeRow(page, "pipe", pipe.id);
      await page.getByTestId("clear-model-tree-filter").click();
      await ensureRail(page, "inspector", true);
      const inspector = page.getByTestId("property-inspector");
      await activateWithKeyboard(page, inspector.getByRole("tab", { name: "Task", exact: true }));
      await activateWithKeyboard(page, inspector.getByTestId("inspector-start-task"));
      await inspector.getByTestId("editor-intent-value").fill(`Retained ${content} measurement task`);
      await activateWithKeyboard(page, page.getByTestId("viewport-fit-selection"));
      const readout = await keyboardMeasureTargets(page, [`Select ${pipe.label} in viewport`]);
      const target = page.getByRole("button", { name: `Select ${pipe.label} in viewport`, exact: true });
      const readSizes = () => page.evaluate(() => {
        const preferences = JSON.parse(localStorage.getItem("chirality.desktop.ui-preferences.v1")!);
        return { left: preferences.leftRailPx, right: preferences.rightRailPx, dock: preferences.dockPx };
      });
      for (const section of content === "regular" ? ["operations", "results"] as const : ["operations"] as const) {
        for (const desired of [180, 600]) {
          await openWorkspaceSection(page, section);
          const dock = page.getByTestId("workspace-dock");
          await expect(dock).not.toHaveClass(/collapsed/);
          if (section === "results") await expect(page.getByTestId("result-group-displacement")).toBeVisible();
          const splitter = page.getByTestId("resize-task-dock");
          await splitter.focus();
          const before = Number(await splitter.getAttribute("aria-valuenow"));
          for (let step = 0; step < Math.ceil(Math.abs(desired - before) / 16); step += 1) {
            await page.keyboard.press(desired > before ? "ArrowUp" : "ArrowDown");
          }
          await expect(splitter).toHaveAttribute("aria-valuenow", String(desired));
          const saved = await readSizes();
          for (const state of ["open", "closed"] as const) {
            if (state === "closed") await page.getByTestId("workspace-dock-close").click();
            const name = `budget-${theme}-${density}-${content}-${section}-${desired}-${state}`;
            try {
              await expect.poll(async () => (await page.getByTestId("viewport-canvas").locator("canvas").boundingBox())?.height ?? 0).toBeGreaterThanOrEqual(200);
              await expectContentFits(readout, "complete measurement content");
              await expectCenterUnobscured(readout);
              for (const label of ["Distance", "ΔX", "ΔY", "ΔZ"]) await expect(readout).toContainText(label);
              if (content === "wrapped") {
                const wraps = await readout.evaluate((element) => {
                  const title = element.querySelector("strong")!.getBoundingClientRect();
                  const values = element.querySelector(":scope > span")!.getBoundingClientRect();
                  return values.top > title.top;
                });
                expect(wraps, "long authored identity moves values onto another readout row").toBe(true);
              }
              await target.focus(); await expect(target).toBeFocused();
              await expectCenterUnobscured(target, { minimumTarget: true });
              if (state === "open") {
                await expect(dock).not.toHaveClass(/collapsed/);
                await expectCenterUnobscured(splitter, { minimumTarget: true });
                await expectCenterUnobscured(page.getByTestId("workspace-dock-close"), { minimumTarget: true });
                const body = page.locator(".workspace-dock-body");
                if (section === "operations") {
                  const tab = page.getByTestId("operation-tab-geometry");
                  await tab.scrollIntoViewIfNeeded(); await tab.focus();
                  await expect(tab).toBeFocused(); await expectCenterUnobscured(tab);
                  await tab.click(); await expect(tab).toHaveAttribute("aria-pressed", "true");
                }
                const control = section === "operations"
                  ? body.locator('.operation-tool-page:not([hidden])').getByLabel("Geometry tool", { exact: true })
                  : body.locator('button:visible:not(:disabled), input:visible:not(:disabled), select:visible:not(:disabled)').first();
                await expect(control).toBeEnabled();
                await control.scrollIntoViewIfNeeded(); await control.focus();
                const readAccess = () => control.evaluate((element) => {
                  const body = element.closest(".workspace-dock-body")!;
                  const box = body.getBoundingClientRect();
                  const clip = { left: Math.max(0, box.left + body.clientLeft), top: Math.max(0, box.top + body.clientTop),
                    right: Math.min(innerWidth, box.left + body.clientLeft + body.clientWidth),
                    bottom: Math.min(innerHeight, box.top + body.clientTop + body.clientHeight) };
                  for (let parent = body.parentElement; parent; parent = parent.parentElement) {
                    const style = getComputedStyle(parent), rect = parent.getBoundingClientRect();
                    if (/(auto|scroll|hidden|clip)/.test(style.overflowX)) {
                      clip.left = Math.max(clip.left, rect.left + parent.clientLeft);
                      clip.right = Math.min(clip.right, rect.left + parent.clientLeft + parent.clientWidth);
                    }
                    if (/(auto|scroll|hidden|clip)/.test(style.overflowY)) {
                      clip.top = Math.max(clip.top, rect.top + parent.clientTop);
                      clip.bottom = Math.min(clip.bottom, rect.top + parent.clientTop + parent.clientHeight);
                    }
                  }
                  const target = element.getBoundingClientRect();
                  const hit = document.elementFromPoint(target.x + target.width / 2, target.y + target.height / 2);
                  const bodyHit = document.elementFromPoint((clip.left + clip.right) / 2, (clip.top + clip.bottom) / 2);
                  return { body: box.toJSON(), clientHeight: body.clientHeight, scrollHeight: body.scrollHeight,
                    scrollTop: body.scrollTop, overflowY: getComputedStyle(body).overflowY,
                    identity: { tag: element.tagName, id: element.id, label: element.getAttribute("aria-label") },
                    page: element.closest(".operation-tool-page")?.getBoundingClientRect().toJSON(),
                    tabs: body.querySelector(".operation-tabs")?.getBoundingClientRect().toJSON(),
                    activeTab: body.querySelector('.operation-tabs [aria-pressed="true"]')?.textContent,
                    focused: document.activeElement === element, owned: hit === element || element.contains(hit),
                    hit: hit?.outerHTML.slice(0, 500), bodyPointOwned: bodyHit === body || body.contains(bodyHit),
                    target: target.toJSON(), clip };
                });
                const initial = await readAccess();
                let wheel: { deltaY: number; before: number; after?: number } | undefined;
                try {
                  // A single ordinary wheel can center an edge-clipped control; no repeated search or tolerance.
                  if (initial.target.top < initial.clip.top || initial.target.bottom > initial.clip.bottom) {
                    const deltaY = Math.round((initial.target.top + initial.target.bottom - initial.clip.top - initial.clip.bottom) / 2);
                    if (deltaY !== 0 && (deltaY < 0 ? initial.scrollTop > 0 : initial.scrollTop < initial.scrollHeight - initial.clientHeight)) {
                      expect(initial.bodyPointOwned, "wheel point belongs to the scrolling dock body").toBe(true);
                      wheel = { deltaY, before: initial.scrollTop };
                      await page.mouse.move((initial.clip.left + initial.clip.right) / 2, (initial.clip.top + initial.clip.bottom) / 2);
                      await page.mouse.wheel(0, deltaY);
                      await expect.poll(() => body.evaluate((element) => element.scrollTop)).not.toBe(initial.scrollTop);
                      wheel.after = await body.evaluate((element) => element.scrollTop);
                    }
                  }
                } finally {
                  await testInfo.attach(`${name}-dock-control-access`, {
                    body: JSON.stringify({ initial, wheel, final: await readAccess() }), contentType: "application/json",
                  });
                  await captureState(page, testInfo, `${name}-actual-page-control`);
                }
                const access = await readAccess();
                await expect(control).toBeFocused(); await expectCenterUnobscured(control);
                expect(access.owned).toBe(true);
                expect(access.clip.right - access.clip.left).toBeGreaterThan(0);
                expect(access.clip.bottom - access.clip.top).toBeGreaterThan(0);
                expect(access.overflowY).toMatch(/auto|scroll/);
                expect(access.target.left).toBeGreaterThanOrEqual(access.clip.left);
                expect(access.target.right).toBeLessThanOrEqual(access.clip.right);
                expect(access.target.top).toBeGreaterThanOrEqual(access.clip.top);
                expect(access.target.bottom).toBeLessThanOrEqual(access.clip.bottom);
                if (section === "operations") {
                  const review = page.getByTestId("operation-tab-review");
                  await review.scrollIntoViewIfNeeded(); await review.focus();
                  await expect(review).toBeFocused(); await expectCenterUnobscured(review);
                  await review.click(); await expect(review).toHaveAttribute("aria-pressed", "true");
                  await testInfo.attach(`${name}-navigation-return`, { body: JSON.stringify({
                    tab: await review.boundingBox(), body: await body.boundingBox(),
                    scrollTop: await body.evaluate((element) => element.scrollTop), activeTab: "Review changes",
                  }), contentType: "application/json" });
                }
              } else await expect(dock).toHaveClass(/collapsed/);
              expect(await readSizes()).toEqual(saved);
              await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText(`pipe: ${pipe.id}`);
              await expect(inspector.getByTestId("editor-intent-value")).toHaveValue(`Retained ${content} measurement task`);
              const orientation = await expectPassiveOrientationFrame(page, testInfo, name);
              expect(orientation.canvas.width).toBeGreaterThanOrEqual(200);
            } finally {
              const geometry = await page.evaluate(() => {
                const rect = (selector: string) => document.querySelector(selector)?.getBoundingClientRect().toJSON();
                return { canvas: rect('[data-testid="viewport-canvas"] canvas'), command: rect(".command-bar"),
                  toolbar: rect(".viewport-toolbar"), measurement: rect(".viewport-measurement-strip"),
                  dock: rect(".workspace-dock"), body: rect(".workspace-dock-body"),
                  reserve: document.querySelector<HTMLElement>(".workspace")?.style.getPropertyValue("--workspace-modeling-reserve"),
                  overflowX: document.documentElement.scrollWidth - innerWidth,
                  overflowY: document.documentElement.scrollHeight - innerHeight };
              });
              await testInfo.attach(`${name}-geometry`, { body: JSON.stringify({ geometry, saved, actualSaved: await readSizes() }), contentType: "application/json" });
              await captureState(page, testInfo, name);
              expect(geometry.overflowX).toBeLessThanOrEqual(0); expect(geometry.overflowY).toBeLessThanOrEqual(0);
            }
          }
        }
      }
    }
  });
}

for (const width of [1024, 1440] as const) {
  test(`outer overlays own actual workspace intersections and restore controls ${width}`, async ({ page }, testInfo) => {
    await page.setViewportSize({ width, height: width === 1024 ? 768 : 920 });
    await gotoRoutedFixture(page);
    const inspectorToggle = page.getByTestId("toggle-inspector");
    const obstacle = page.getByTestId(width < 1280 ? "toggle-inspector" : "resize-property-inspector");
    for (const railOpen of width < 1280 ? [false, true] : [true]) {
      await ensureRail(page, "inspector", railOpen);
      for (const [drawerId, toggleId] of [
        ["issues-home", "issues-drawer-toggle"], ["audit-boundary-drawer", "audit-drawer-toggle"],
      ]) {
        await page.getByTestId(toggleId).click();
        const drawer = page.getByTestId(drawerId);
        await expect(drawer).toBeVisible();
        const close = drawer.getByRole("button", { name: "Close", exact: true });
        await expectCenterUnobscured(close, { minimumTarget: true });
        await expectCenterUnobscured(drawer.getByRole("heading").first());
        const overlap = await expectOverlayAboveWorkspaceControls(page, drawer, [{ name: "actual inspector layer", locator: obstacle }]);
        const point = await obstacle.evaluate((element, id) => {
          const box = element.getBoundingClientRect();
          const drawerBox = document.querySelector(`[data-testid="${id}"]`)!.getBoundingClientRect();
          const left = Math.max(box.left, drawerBox.left), right = Math.min(box.right, drawerBox.right);
          const top = Math.max(box.top, drawerBox.top), bottom = Math.min(box.bottom, drawerBox.bottom);
          return { x: (left + right) / 2, y: (top + bottom) / 2, width: right - left, height: bottom - top };
        }, drawerId);
        expect(point.width).toBeGreaterThan(0);
        expect(point.height).toBeGreaterThan(0);
        const assertOwner = async (selector: string, phase: string) => {
          const hit = await page.evaluate(({ point, selector }) => {
            const owner = document.elementFromPoint(point.x, point.y);
            const surface = document.querySelector(selector);
            return { tag: owner?.tagName, testId: owner?.getAttribute("data-testid") || null,
              className: owner?.className, owned: !!surface && !!owner && (surface === owner || surface.contains(owner)) };
          }, { point, selector });
          await testInfo.attach(`${width}-${railOpen}-${drawerId}-${phase}`, {
            body: JSON.stringify({ point, overlap, hit }), contentType: "application/json",
          });
          expect(hit.owned, `${phase} owns real drawer/workspace intersection`).toBe(true);
        };
        await page.getByTestId("menu-view").click();
        await expectCenterUnobscured(page.getByTestId("menu-item-view.section.operations"), { minimumTarget: true });
        await assertOwner('[data-testid="app-menu-backdrop"]', "menu-backdrop");
        await page.mouse.click(point.x, point.y);
        await expect(page.getByTestId("app-menu-backdrop")).toHaveCount(0);
        await expect(drawer).toBeVisible();
        await page.getByTestId("menu-view").click();
        await page.getByTestId("menu-item-view.section.operations").click();
        await expect(page.getByTestId("workspace-section-operations")).toBeVisible();
        await expect(drawer).toBeVisible();
        await page.getByTestId("toolkit-entry").click();
        const dialog = page.getByRole("dialog", { name: "Find a modeling tool" });
        await expect(dialog).toBeVisible();
        await expectCenterUnobscured(dialog.getByRole("searchbox", { name: "Find a tool" }));
        const palettePoint = await obstacle.evaluate((element, id) => {
          const obstacleBox = element.getBoundingClientRect();
          const drawerBox = document.querySelector(`[data-testid="${id}"]`)!.getBoundingClientRect();
          const dialogBox = document.querySelector('[role="dialog"][aria-label="Find a modeling tool"]')!.getBoundingClientRect();
          const intersection = { left: Math.max(0, obstacleBox.left, drawerBox.left),
            top: Math.max(0, obstacleBox.top, drawerBox.top),
            right: Math.min(innerWidth, obstacleBox.right, drawerBox.right),
            bottom: Math.min(innerHeight, obstacleBox.bottom, drawerBox.bottom) };
          // Subtract the dialog rectangle from the real drawer/control overlap.
          // Select the largest positive remainder deterministically, without hit sampling.
          const pieces = [
            { ...intersection, right: Math.min(intersection.right, dialogBox.left) },
            { ...intersection, left: Math.max(intersection.left, dialogBox.right) },
            { left: Math.max(intersection.left, dialogBox.left), right: Math.min(intersection.right, dialogBox.right),
              top: intersection.top, bottom: Math.min(intersection.bottom, dialogBox.top) },
            { left: Math.max(intersection.left, dialogBox.left), right: Math.min(intersection.right, dialogBox.right),
              top: Math.max(intersection.top, dialogBox.bottom), bottom: intersection.bottom },
          ].map((rect) => ({ ...rect, area: Math.max(0, rect.right - rect.left) * Math.max(0, rect.bottom - rect.top) }))
            .filter((rect) => rect.area > 0).sort((a, b) => b.area - a.area);
          const region = pieces[0] ?? null;
          const point = region ? { x: (region.left + region.right) / 2, y: (region.top + region.bottom) / 2 } : null;
          const hit = point ? document.elementFromPoint(point.x, point.y) : null;
          const backdrop = document.querySelector(".toolkit-backdrop");
          return { obstacle: obstacleBox.toJSON(), drawer: drawerBox.toJSON(), dialog: dialogBox.toJSON(),
            viewport: { width: innerWidth, height: innerHeight }, intersection, pieces, region, point,
            owner: hit?.getAttribute("data-testid") || hit?.id || hit?.tagName || null,
            backdropOwnsPoint: !!backdrop && !!hit && (hit === backdrop || backdrop.contains(hit)) };
        }, drawerId);
        await testInfo.attach(`${width}-${railOpen}-${drawerId}-palette-backdrop`, {
          body: JSON.stringify(palettePoint, null, 2), contentType: "application/json",
        });
        expect(palettePoint.region?.area ?? 0).toBeGreaterThan(0);
        expect(palettePoint.point).not.toBeNull();
        expect(palettePoint.backdropOwnsPoint).toBe(true);
        await page.mouse.click(palettePoint.point!.x, palettePoint.point!.y);
        await expect(dialog).toHaveCount(0);
        await expect(page.getByTestId("toolkit-entry")).toBeFocused();
        await expect(drawer).toBeVisible();
        await page.getByTestId("toolkit-entry").click();
        const selectCommand = page.getByTestId("toolkit-view.select");
        await selectCommand.scrollIntoViewIfNeeded();
        await expectCenterUnobscured(selectCommand, { minimumTarget: true });
        await selectCommand.click();
        await expect(dialog).toHaveCount(0);
        await expect(drawer).toBeVisible();
        await close.click();
        await expect(drawer).toHaveCount(0);
        await ensureRail(page, "inspector", railOpen);
        await expectCenterUnobscured(obstacle, { minimumTarget: true });
        await obstacle.focus();
        await expect(obstacle).toBeFocused();
        if (width >= 1280) {
          await obstacle.press("ArrowLeft");
          await expectCenterUnobscured(obstacle, { minimumTarget: true });
        } else {
          await inspectorToggle.click();
          await expect(inspectorToggle).toHaveAttribute("aria-expanded", String(!railOpen));
          await inspectorToggle.click();
          await expect(inspectorToggle).toHaveAttribute("aria-expanded", String(railOpen));
        }
      }
    }
  });
}

for (const theme of APPEARANCE_THEMES) {
  for (const density of APPEARANCE_DENSITIES) {
    for (const viewport of APPEARANCE_VIEWPORTS) {
      test(`production appearance ${theme} ${density} ${viewport.width}x${viewport.height}`, async ({ page }, testInfo) => {
        await page.setViewportSize(viewport);
        const model = await gotoRoutedFixture(page);
        const treeToggle = page.getByTestId("toggle-tree");
        const inspectorToggle = page.getByTestId("toggle-inspector");

        if (viewport.width >= 1280) {
          await expect(treeToggle).toHaveAttribute("aria-expanded", "true");
          await expect(inspectorToggle).toHaveAttribute("aria-expanded", "true");
        } else {
          await expect(treeToggle).toHaveAttribute("aria-expanded", "false");
          await expect(inspectorToggle).toHaveAttribute("aria-expanded", "false");
        }

        await setAppearance(page, theme, density);
        const railHoverWitnesses: unknown[] = [];
        try {
          for (const rail of ["tree", "inspector"] as const) {
            const toggle = page.getByTestId(`toggle-${rail}`);
            const originalOpen = await toggle.getAttribute("aria-expanded") === "true";
            for (const open of [false, true]) {
              await ensureRail(page, rail, open);
              await toggle.hover();
              await expect(toggle).toHaveAttribute("aria-expanded", String(open));
              await expectCenterUnobscured(toggle, { minimumTarget: true });
              await captureElementState(toggle, testInfo, `rail-hover-${theme}-${density}-${viewport.width}-${rail}-${open}`);
              railHoverWitnesses.push(...await expectResolvedContrast([
                { name: `${rail} ${open ? "expanded" : "collapsed"} hovered label`, locator: toggle.locator(".workspace-pane-toggle-label"), minimum: 4.5 },
                { name: `${rail} hovered disclosure glyph`, locator: toggle.locator(".workspace-pane-toggle-icon"), source: "graphic", minimum: 3 },
                ...await Promise.all((await toggle.locator("svg").all()).map(async (locator, index) => ({
                  name: `${rail} hovered icon ${index}`, locator, source: "graphic" as const, minimum: 3 as const,
                }))),
              ]));
              await activateWithKeyboard(page, toggle);
              await expect(toggle).toHaveAttribute("aria-expanded", String(!open));
              await expect(toggle).toBeFocused();
            }
            await ensureRail(page, rail, originalOpen);
          }
        } finally {
          await testInfo.attach(`rail-hover-contrast-${theme}-${density}-${viewport.width}`, {
            body: JSON.stringify(railHoverWitnesses), contentType: "application/json",
          });
        }
        const pipe = model.pipe_segments[4];
        const selected = await selectTreeRow(page, "pipe", pipe.id);
        await expect(selected).toHaveAttribute("aria-selected", "true");
        await expectTreeRowIdentityReadable(selected, pipe.label, pipe.id);
        await expectSelectedTreeTarget(selected);
        const contrastWitnesses = [
          ...await expectResolvedContrast([
            { name: "Model panel heading", locator: page.locator(".model-tree > .panel-title") },
            { name: "Selected tree entity name", locator: selected.locator("strong") },
            { name: "Selected tree entity typed ID", locator: selected.locator("small") },
            {
              name: "Model tree filter label",
              locator: page.getByTestId("model-tree-controls").getByText("Filter model", { exact: true }),
            },
            { name: "Model tree visible count", locator: page.getByTestId("model-tree-filter-summary") },
          ]),
        ];

        const treeMode = page.getByTestId("layout-mode-tree");
        const gridMode = page.getByTestId("layout-mode-grid");
        await expect(treeMode).toBeVisible();
        await expect(gridMode).toBeVisible();
        await expectFlatTokenBorders(page, [treeMode, gridMode]);
        await activateWithKeyboard(page, gridMode);
        const pipeGridTab = page.getByTestId("entity-grid-type-pipes");
        const nodeGridTab = page.getByTestId("entity-grid-type-nodes");
        const queueGridEdits = page.getByTestId("queue-entity-grid-intents");
        const clearGridEdits = page.getByTestId("clear-entity-grid-drafts");
        await expect(pipeGridTab).toHaveAttribute("aria-pressed", "true");
        for (const control of [pipeGridTab, nodeGridTab, queueGridEdits, clearGridEdits]) {
          await expect(control).toBeVisible();
        }
        await expectFlatTokenBorders(page, [pipeGridTab, nodeGridTab, queueGridEdits, clearGridEdits]);
        await activateWithKeyboard(page, treeMode);
        await expect(treeMode).toHaveAttribute("aria-pressed", "true");

        await ensureRail(page, "inspector", true);
        const inspector = page.getByTestId("property-inspector");
        await activateWithKeyboard(page, inspector.getByRole("tab", { name: "Task", exact: true }));
        await expect(inspector.getByTestId("inspector-task-empty")).toContainText(`Current selection: pipe: ${pipe.id}`);
        await expect(inspector.getByTestId("inspector-start-task")).toBeEnabled();
        await expect(inspector.getByTestId("inspector-frozen-task-target")).toHaveCount(0);
        await expect(inspector.getByTestId("task-action-footer")).toHaveCount(0);
        await expect(inspector.getByTestId("cancel-editor-intent")).toHaveCount(0);
        await activateWithKeyboard(page, inspector.getByTestId("inspector-start-task"));
        await expect(inspector.getByTestId("task-action-footer")).toBeVisible();
        await expect(inspector.getByTestId("queue-editor-intent")).toBeDisabled();
        contrastWitnesses.push(...await expectResolvedContrast([
          { name: "Inspector selected entity heading", locator: inspector.locator(":scope > h2") },
          { name: "Inspector selected entity typed identity", locator: inspector.locator(":scope > h2 .typed-identity") },
          { name: "Task edit heading", locator: inspector.getByRole("heading", { name: /^Edit / }).first() },
          {
            name: "Task property label",
            locator: inspector.locator(".editor-intent-controls label span").first(),
          },
        ]));
        await expectFlatTokenBorders(page, [
          page.getByTestId("viewport-box-select"),
          page.getByTestId("viewport-selection-filter"),
          page.getByTestId("viewport-geometry-actual-od"),
          inspector.getByRole("tab", { name: "Properties", exact: true }),
          inspector.getByRole("tab", { name: "Task", exact: true }),
        ]);

        await activateWithKeyboard(page, page.getByTestId("issues-drawer-toggle"));
        const issues = page.getByTestId("issues-home");
        const issuesHeading = issues.getByRole("heading", { name: "Diagnostics and required inputs" });
        const issuesClose = issues.getByRole("button", { name: "Close", exact: true });
        await expect(issues).toBeVisible();
        await expect(issuesHeading).toBeVisible();
        await activateWithKeyboard(page, page.getByTestId("workspace-select"));
        await expect(page.getByTestId("workspace-select")).toBeFocused();
        await expect(page.getByTestId("workspace-undo")).toBeDisabled();
        if (viewport.width < 1280) {
          await ensureRail(page, "inspector", false);
          await ensureRail(page, "tree", false);
          await expect(treeToggle).toHaveAttribute("aria-expanded", "false");
          await expect(inspectorToggle).toHaveAttribute("aria-expanded", "false");
        }
        const diagnostics = issues.getByTestId("diagnostics-panel");
        const missingData = issues.getByTestId("missing-data-panel");
        contrastWitnesses.push(...await expectResolvedContrast([
          { name: "Issues drawer heading", locator: issuesHeading },
          { name: "Issues drawer eyebrow", locator: issues.locator(".drawer-header > div > span") },
          { name: "Issues drawer Close action", locator: issuesClose },
          { name: "Diagnostics panel heading", locator: diagnostics.locator(":scope > .panel-title") },
          {
            name: "Diagnostic code SUPPORT_STIFFNESS_UNRESOLVED",
            locator: diagnostics.getByTestId("diagnostic-SUPPORT_STIFFNESS_UNRESOLVED").locator("strong"),
          },
          { name: "Missing Data Blocking panel heading", locator: missingData.locator(":scope > .panel-title") },
          { name: "Missing-data summary", locator: missingData.getByTestId("missing-data-summary") },
          {
            name: "Missing-data detail label",
            locator: missingData.getByTestId("missing-data-class-coverage").locator("span"),
          },
          {
            name: "Missing-data detail value",
            locator: missingData.getByTestId("missing-data-class-coverage").locator("strong"),
          },
          {
            name: "Diagnostics warning icon",
            locator: diagnostics.locator(":scope > .panel-title svg"),
            source: "graphic",
            minimum: 3,
          },
          {
            name: "Missing-data blocking icon",
            locator: missingData.locator(":scope > .panel-title svg"),
            source: "graphic",
            minimum: 3,
          },
        ]));
        const overlayWitness = await expectOverlayAboveWorkspaceControls(page, issues, [
          { name: "Model rail splitter", locator: page.getByTestId("resize-model-tree") },
          { name: "Property inspector splitter", locator: page.getByTestId("resize-property-inspector") },
          { name: "Model rail handle", locator: treeToggle },
          { name: "Property inspector rail handle", locator: inspectorToggle },
        ]);
        const headingTopmost = await expectCenterUnobscured(issuesHeading);
        const closeTopmost = await expectCenterUnobscured(issuesClose, { minimumTarget: true });
        await testInfo.attach(`resolved-theme-and-overlay-${theme}-${density}-${viewport.width}x${viewport.height}`, {
          body: JSON.stringify({ contrastWitnesses, overlayWitness, headingTopmost, closeTopmost }, null, 2),
          contentType: "application/json",
        });

        await expectWorkspaceGeometry(page, viewport);
        await expectResolvedStyleAndTargets(page);
        await testInfo.attach(`appearance-capture-semantics-${theme}-${density}-${viewport.width}x${viewport.height}`, {
          body: JSON.stringify({
            originalCaptureName: `normal-authoring-${theme}-${density}-${viewport.width}x${viewport.height}`,
            originalCaptureState: "Issues drawer open after contrast and topmost stacking assertions",
            cleanCaptureName: `unobscured-authoring-${theme}-${density}-${viewport.width}x${viewport.height}`,
            cleanCaptureState: "Issues drawer closed; authoring rails retain the state appropriate to this width",
          }, null, 2),
          contentType: "application/json",
        });
        await captureState(page, testInfo, `normal-authoring-${theme}-${density}-${viewport.width}x${viewport.height}`);
        await activateWithKeyboard(page, issuesClose);
        await expect(issues).toHaveCount(0);

        // These enabled status labels remain ordinary text even when there
        // are no deformation results. Exercise the disclosure through its
        // keyboard control and measure its actual composited backgrounds.
        const deformation = page.getByTestId("viewport-deformation-status");
        const deformationSummary = deformation.locator(":scope > summary");
        await expect(deformation).not.toHaveAttribute("open", "");
        await expect(deformationSummary).toBeEnabled();
        const statusContrastWitnesses = [
          ...await expectResolvedContrast([
            { name: "Armed creation tool status", locator: page.getByTestId("armed-creation-tool"), minimum: 4.5 },
            { name: "Closed enabled deformation summary", locator: deformationSummary, minimum: 4.5 },
          ]),
        ];
        const menuSurfaceWitness = await page.locator(".app-menu-bar").evaluate((element) => {
          const style = getComputedStyle(element);
          const tokenRgb = (name: string) => {
            const token = style.getPropertyValue(name).trim();
            if (!/^#[0-9a-f]{3}([0-9a-f]{3})?$/i.test(token)) {
              throw new Error(`Expected established hex theme token ${name}, received ${token}`);
            }
            const hex = token.slice(1);
            const full = hex.length === 3 ? [...hex].map((digit) => digit + digit).join("") : hex;
            return `rgb(${[0, 2, 4].map((offset) => parseInt(full.slice(offset, offset + 2), 16)).join(", ")})`;
          };
          return {
            background: style.backgroundColor,
            expectedSurface: tokenRgb("--ui-surface"),
            border: style.borderBottomColor,
            expectedDivider: tokenRgb("--ui-divider"),
            borderWidth: style.borderBottomWidth,
            borderStyle: style.borderBottomStyle,
          };
        });
        expect(menuSurfaceWitness.background, "menubar uses the active theme surface").toBe(menuSurfaceWitness.expectedSurface);
        expect(menuSurfaceWitness.border, "menubar uses the active theme divider").toBe(menuSurfaceWitness.expectedDivider);
        expect(parseFloat(menuSurfaceWitness.borderWidth)).toBeGreaterThan(0);
        expect(menuSurfaceWitness.borderStyle).toBe("solid");
        await activateWithKeyboard(page, deformationSummary);
        await expect(deformation).toHaveAttribute("open", "");
        statusContrastWitnesses.push(...await expectResolvedContrast([
          { name: "Expanded deformation strong text", locator: page.getByTestId("viewport-deformation-summary"), minimum: 4.5 },
          { name: "Expanded deformation small text", locator: page.getByTestId("viewport-deformation-boundary"), minimum: 4.5 },
        ]));
        await captureState(page, testInfo, `expanded-deformation-${theme}-${density}-${viewport.width}x${viewport.height}`);
        await activateWithKeyboard(page, deformationSummary);
        await expect(deformation).not.toHaveAttribute("open", "");
        await expect(deformationSummary).toBeFocused();
        await expect(page.getByTestId("viewport-deformation-summary")).toBeHidden();
        await expect(page.getByTestId("viewport-canvas")).toBeVisible();
        await expectWorkspaceGeometry(page, viewport);
        await ensureRail(page, "inspector", true);
        await expect(inspector.getByTestId("task-action-footer")).toBeVisible();
        await expectCenterUnobscured(inspector.getByTestId("task-action-footer"));
        if (viewport.width < 1280) await ensureRail(page, "inspector", false);
        await testInfo.attach(`resolved-dark-status-${theme}-${density}-${viewport.width}x${viewport.height}`, {
          body: JSON.stringify({ statusContrastWitnesses, menuSurfaceWitness, keyboardDisclosureClosed: true }, null, 2),
          contentType: "application/json",
        });
        await captureState(page, testInfo, `unobscured-authoring-${theme}-${density}-${viewport.width}x${viewport.height}`);
        await expectPassiveOrientationFrame(page, testInfo, `clean-appearance-${theme}-${density}-${viewport.width}x${viewport.height}`);
      });
    }
  }
}

test("System follows the emulated OS preference; explicit theme and density persist after reload", async ({ page }) => {
  await page.emulateMedia({ colorScheme: "dark" });
  await gotoRoutedFixture(page);
  const shell = page.getByTestId("desktop-preview-shell");
  await expect(page.getByLabel("Appearance theme")).toHaveValue("system");
  await expect(shell).toHaveAttribute("data-theme", "dark");
  await page.emulateMedia({ colorScheme: "light" });
  await expect(shell).toHaveAttribute("data-theme", "light");

  await page.getByLabel("Appearance theme").selectOption("dark");
  await page.getByLabel("Workspace density").selectOption("compact");
  await page.reload();
  await expect(page.getByLabel("Appearance theme")).toHaveValue("dark");
  await expect(page.getByLabel("Workspace density")).toHaveValue("compact");
  await expect(shell).toHaveAttribute("data-theme", "dark");
  await expect(shell).toHaveAttribute("data-density", "compact");
});

test("production browser keeps reopened results Historical until a fresh exact-basis solve succeeds", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
  await openWorkspaceSection(page, "solve");
  await activateWithKeyboard(page, page.getByTestId("run-mechanics-preview"));
  await expect(page.getByTestId("status-pill-mechanics")).toContainText("MECHANICS_SOLVED");
  await expect(page.getByTestId("status-pill-solve-proof")).toBeVisible();

  await activateWithKeyboard(page, page.getByRole("button", { name: "Save local", exact: true }));
  await expect(page.getByTestId("local-project-message")).toContainText("Saved");
  await activateWithKeyboard(page, page.getByTestId("open-local-project"));
  await openWorkspaceSection(page, "results");
  const historical = page.getByTestId("historical-run-context");
  await expect(historical).toBeVisible();
  await expect(historical).toContainText("Historical");
  await expect(page.getByTestId("viewport-deformation-summary")).toContainText("result rows=0");
  await expect(page.getByTestId("comparison-summary")).toHaveCount(0);
  await expect(page.getByTestId("status-pill-solve-proof")).toHaveCount(0);

  await openWorkspaceSection(page, "solve");
  await expect(page.getByTestId("rule-check-run")).toBeDisabled();
  await activateWithKeyboard(page, page.getByTestId("run-mechanics-preview"));
  await expect(page.getByTestId("status-pill-mechanics")).toContainText("MECHANICS_SOLVED");
  await openWorkspaceSection(page, "results");
  await expect(historical).toHaveCount(0);
  await expect(page.getByTestId("status-pill-solve-proof")).toBeVisible();
});



async function expectPopulatedResultsGeometry(page: Page, width: number, historical: boolean) {
  await page.setViewportSize({ width, height: width === 1024 ? 768 : 900 });
  const results = page.getByTestId("results-panel");
  await expect(results.locator("tbody tr").first()).toBeVisible();
  const geometry = await results.evaluate((element, isHistorical) => {
    const section = element.closest('[data-testid="workspace-section-results"]')!;
    const owner = isHistorical ? element.closest('[data-testid="historical-run-context"]')! : element;
    const rect = (node: Element) => node.getBoundingClientRect().toJSON();
    const groups = [...element.querySelectorAll(".result-group")].map((group) => ({
      group: rect(group), table: rect(group.querySelector("table")!), overflowX: getComputedStyle(group).overflowX,
    }));
    const values = [...element.querySelectorAll('td:nth-child(4) [data-display-status]')].map((value) => {
      const range = document.createRange();
      range.selectNodeContents(value);
      const lines = [...range.getClientRects()].filter((box) => box.width > 0 && box.height > 0);
      return { text: value.textContent, box: rect(value), cell: rect(value.closest("td")!), lineTops: lines.map((box) => box.top) };
    });
    return { section: rect(section), owner: rect(owner), results: rect(element), groups, values,
      horizontalOverflow: document.documentElement.scrollWidth - innerWidth };
  }, historical);
  expect(geometry.owner.width, "main result owner spans the complete dock row").toBeGreaterThanOrEqual(geometry.section.width - 2);
  expect(geometry.horizontalOverflow).toBeLessThanOrEqual(1);
  expect(geometry.groups.length).toBeGreaterThan(0);
  for (const group of geometry.groups) {
    expect(group.table.width, "populated table uses the available result panel width").toBeGreaterThanOrEqual(geometry.results.width * 0.85);
    if (group.table.width > group.group.width + 1) expect(group.overflowX).toBe("auto");
  }
  expect(geometry.values.length).toBeGreaterThan(0);
  for (const value of geometry.values) {
    expect(value.text).toMatch(/\d/);
    expect(value.lineTops.length).toBeGreaterThan(0);
    expect(Math.max(...value.lineTops) - Math.min(...value.lineTops), `one-line numerical value and units: ${value.text}`).toBeLessThanOrEqual(1);
    expect(value.box.width).toBeLessThanOrEqual(value.cell.width);
  }
  return { width, historical, ...geometry };
}

for (const theme of APPEARANCE_THEMES) {
  test(`Solve readiness rows retain resolved contrast in ${theme}`, async ({ page }, testInfo) => {
    await page.setViewportSize({ width: 1440, height: 920 });
    await page.goto("/");
    await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
    await setAppearance(page, theme, "comfortable");
    const tones = new Set<string>();
    for (const phase of ["before-solve", "solved", "blocked"] as const) {
      if (phase === "blocked") {
        await openWorkspaceSection(page, "operations");
        await expect(page.getByTestId("operation-engine-chip")).toContainText("Engine ready");
        const editor = await startPropertyTaskFromTreeEntity(page, "load", "load:L-100");
        await editor.getByTestId("editor-intent-field").selectOption("primitive_loads.0.magnitude.value");
        await expect(editor.getByTestId("editor-intent-unit")).toHaveValue("N/m");
        await editor.getByLabel("New first primitive magnitude", { exact: true }).fill("-225");
        await editor.getByTestId("queue-editor-intent").click();
        await openWorkspaceSection(page, "operations");
        await page.getByTestId("apply-intent-editor-intent-1").click();
        await expect(page.getByTestId("applied-operation-route-applied-1-editor-intent-1")).toContainText("Applied through local_wasm_engine");
      }
      await openWorkspaceSection(page, "solve");
      if (phase !== "before-solve") {
        await activateWithKeyboard(page, page.getByTestId("run-mechanics-preview"));
        await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
        await expect(page.getByTestId("status-pill-mechanics")).toContainText(phase === "solved" ? "MECHANICS_SOLVED" : "MODEL_INCOMPLETE");
      }
      if (phase === "blocked") {
        await expect(page.getByTestId("solve-job-summary")).toContainText("result_rows=0");
        await expect(page.getByTestId("readiness-mechanics")).toContainText("0 computed result rows; model incomplete");
        await expect(page.getByTestId("readiness-diagnostics")).toHaveClass(/\bblocking\b/);
        await expect(page.getByTestId("readiness-diagnostics")).toContainText("1 blocking/error");
      }
      const rows = page.getByTestId("solve-readiness-summary").locator(".readiness-row");
      await expect(rows).toHaveCount(4);
      const witnesses: unknown[] = [];
      try {
        for (const row of await rows.all()) {
          await row.scrollIntoViewIfNeeded();
          const tone = (await row.getAttribute("class"))!.split(/\s+/).find((value) => ["ok", "info", "warning", "blocking"].includes(value))!;
          expect(tone).toBeTruthy(); tones.add(tone);
          const identity = await row.getAttribute("data-testid");
          await captureElementState(row, testInfo, `readiness-${theme}-${phase}-${identity}-${tone}`);
          witnesses.push(...await expectResolvedContrast([
            { name: `${phase} ${identity} ${tone} label`, locator: row.locator(":scope > span"), minimum: 4.5 },
            { name: `${phase} ${identity} ${tone} value`, locator: row.locator(":scope > strong"), minimum: 4.5 },
            { name: `${phase} ${identity} ${tone} icon`, locator: row.locator(":scope > svg"), source: "graphic", minimum: 3 },
          ]));
        }
      } finally {
        await testInfo.attach(`readiness-contrast-${theme}-${phase}`, { body: JSON.stringify({ tones: [...tones], witnesses }), contentType: "application/json" });
        await captureState(page, testInfo, `readiness-${theme}-${phase}`);
      }
      if (phase === "blocked") {
        await page.getByTestId("issues-drawer-toggle").click();
        const diagnostic = page.getByTestId("diagnostic-BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL");
        await expect(diagnostic).toBeVisible();
        await expect(diagnostic).toContainText("Browser fixture mode will not reuse bundled solved-result rows for an edited model");
        await testInfo.attach(`readiness-blocked-diagnostic-${theme}`, {
          body: JSON.stringify({ code: "BROWSER_SOLVE_BACKEND_REQUIRED_FOR_EDITED_MODEL", text: await diagnostic.innerText() }), contentType: "application/json",
        });
        await captureElementState(diagnostic, testInfo, `readiness-blocked-diagnostic-${theme}`);
      }
    }
    expect([...tones].sort()).toEqual(["blocking", "info", "ok", "warning"]);
  });
}

for (const theme of APPEARANCE_THEMES) {
  test(`populated Results title and entered values retain resolved contrast in ${theme}`, async ({ page }, testInfo) => {
    await page.goto("/");
    await expect(page.getByTestId("desktop-preview-shell")).toBeVisible();
    await setAppearance(page, theme, "comfortable");
    await openWorkspaceSection(page, "solve");
    await activateWithKeyboard(page, page.getByTestId("run-mechanics-preview"));
    await expect(page.getByTestId("status-pill-mechanics")).toContainText("MECHANICS_SOLVED");
    await openWorkspaceSection(page, "results");
    const results = page.getByTestId("results-panel");
    const row = results.locator("tbody tr").first();
    const entered = row.locator(".dual-unit-cell > span");
    await expect(row).toBeVisible();
    await expect(row).toHaveAttribute("aria-selected", "false");
    await expect(entered).toContainText("Entered:");
    const originalEnteredText = await entered.innerText();
    const witnesses = [...await expectResolvedContrast([
      { name: "Results dock title", locator: page.getByTestId("workspace-dock-header").getByRole("heading", { name: "Results", exact: true }), minimum: 4.5 },
      { name: "Results entered source value", locator: entered, minimum: 4.5 },
    ])];
    const headers = await results.getByRole("columnheader").all();
    expect(headers.length).toBeGreaterThan(0);
    witnesses.push(...await expectResolvedContrast(await Promise.all(headers.map(async (locator, index) => ({
      name: `Current Results column heading ${index}: ${await locator.innerText()}`, locator, minimum: 4.5 as const,
    })))));
    await captureState(page, testInfo, `populated-results-${theme}`);
    await activateWithKeyboard(page, row);
    await expect(row).toHaveAttribute("aria-selected", "true");
    await expect(entered).toHaveText(originalEnteredText);
    const requiredFlag = page.getByTestId("property-inspector").getByTestId("inspector-required-flags").locator(".required-flag");
    await expect(requiredFlag).toContainText("Restraints");
    witnesses.push(...await expectResolvedContrast([
      { name: "Selected result target required-field text", locator: requiredFlag, minimum: 4.5 },
    ]));
    witnesses.push(...await expectResolvedContrast([
      { name: "Selected Results entered source value", locator: entered, minimum: 4.5 },
    ]));
    await testInfo.attach(`resolved-results-contrast-${theme}`, {
      body: JSON.stringify({ witnesses, originalEnteredText }, null, 2), contentType: "application/json",
    });
    await captureState(page, testInfo, `populated-results-selected-${theme}`);
    const currentGapLedger = results.getByTestId("mechanics-gap-ledger");
    const currentGapNotes = await currentGapLedger.locator(".gap-list p").all();
    expect(currentGapNotes.length).toBeGreaterThan(0);
    const currentGapWitnesses = await expectResolvedContrast(await Promise.all(currentGapNotes.map(async (locator, index) => ({
      name: `Current Mechanics Gap Ledger note ${index}: ${await locator.innerText()}`, locator, minimum: 4.5 as const,
    }))));
    await testInfo.attach(`resolved-current-gap-notes-${theme}`, {
      body: JSON.stringify(currentGapWitnesses, null, 2), contentType: "application/json",
    });
    await captureElementState(currentGapLedger, testInfo, `populated-current-gap-ledger-${theme}`);
    const geometryWitnesses = [];
    for (const width of [1280, 1024]) {
      geometryWitnesses.push(await expectPopulatedResultsGeometry(page, width, false));
      await captureState(page, testInfo, `populated-results-current-${theme}-${width}`);
    }
    await activateWithKeyboard(page, page.getByRole("button", { name: "Save local", exact: true }));
    await expect(page.getByTestId("local-project-message")).toContainText("Saved");
    await activateWithKeyboard(page, page.getByTestId("open-local-project"));
    await openWorkspaceSection(page, "results");
    await expect(page.getByTestId("historical-run-context")).toBeVisible();
    const historicalHeaders = await results.getByRole("columnheader").all();
    expect(historicalHeaders.length).toBeGreaterThan(0);
    const historicalHeaderWitnesses = await expectResolvedContrast(await Promise.all(historicalHeaders.map(async (locator, index) => ({
      name: `Historical Results column heading ${index}: ${await locator.innerText()}`, locator, minimum: 4.5 as const,
    }))));
    await testInfo.attach(`resolved-historical-results-headers-${theme}`, {
      body: JSON.stringify(historicalHeaderWitnesses, null, 2), contentType: "application/json",
    });
    const historicalGapLedger = results.getByTestId("mechanics-gap-ledger");
    const historicalGapNotes = await historicalGapLedger.locator(".gap-list p").all();
    expect(historicalGapNotes.length).toBeGreaterThan(0);
    const historicalGapWitnesses = await expectResolvedContrast(await Promise.all(historicalGapNotes.map(async (locator, index) => ({
      name: `Historical Mechanics Gap Ledger note ${index}: ${await locator.innerText()}`, locator, minimum: 4.5 as const,
    }))));
    await testInfo.attach(`resolved-historical-gap-notes-${theme}`, {
      body: JSON.stringify(historicalGapWitnesses, null, 2), contentType: "application/json",
    });
    await captureElementState(historicalGapLedger, testInfo, `populated-historical-gap-ledger-${theme}`);
    for (const width of [1280, 1024]) {
      geometryWitnesses.push(await expectPopulatedResultsGeometry(page, width, true));
      await captureState(page, testInfo, `populated-results-historical-${theme}-${width}`);
    }
    await testInfo.attach(`populated-results-line-geometry-${theme}`, {
      body: JSON.stringify(geometryWitnesses, null, 2), contentType: "application/json",
    });
  });
}

test("same raw ID pipe task retains its typed values and queued intent after node selection", async ({ page }, testInfo) => {
  const fixture = await readFixture("precision-origin-base.model.json");
  const sharedId = "entity:UIF-TYPED-COLLISION-01000";
  const model = withTypedCollision(fixture.model, sharedId);
  model.nodes[0].label = "Distinct colliding node name";
  model.pipe_segments[0].label = "Distinct colliding pipe name";
  await gotoModel(page, model);
  await selectTreeRow(page, "pipe", sharedId);
  await ensureRail(page, "inspector", true);
  const inspector = page.getByTestId("property-inspector");
  await activateWithKeyboard(page, inspector.getByRole("tab", { name: "Task", exact: true }));
  await activateWithKeyboard(page, inspector.getByTestId("inspector-start-task"));
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText(`Draft target: pipe: ${sharedId}`);
  await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("Distinct colliding pipe name");
  await inspector.getByTestId("editor-intent-value").fill("Edited colliding pipe name");
  await selectTreeRow(page, "node", sharedId);
  await ensureRail(page, "inspector", true);
  await activateWithKeyboard(page, inspector.getByRole("tab", { name: "Task", exact: true }));
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText(`Draft target: pipe: ${sharedId}`);
  await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("Edited colliding pipe name");
  await activateWithKeyboard(page, inspector.locator("summary").filter({ hasText: /^Operation details$/ }));
  const preview = inspector.getByTestId("editor-intent-panel").getByTestId("editor-operation-preview");
  await expect(preview).toBeVisible();
  await expect(preview).toContainText(`Element; ${sharedId}`);
  await expect(preview).toContainText("before=Distinct colliding pipe name; after=Edited colliding pipe name");
  await expect(preview).toContainText(`op:editor-intent-pipe-${sharedId}-label`);
  await activateWithKeyboard(page, inspector.getByTestId("queue-editor-intent"));
  await expect(inspector.getByTestId("editor-intent-queue")).toContainText(`op:editor-intent-pipe-${sharedId}-label`);
  await captureState(page, testInfo, "typed-collision-pipe-task-after-node-selection");
});

test("narrow canvas fits visible and selected envelopes in every view preset", async ({ page }, testInfo) => {
  await page.setViewportSize({ width: 600, height: 1600 });
  const model = await gotoRoutedFixture(page, "precision-origin-base.model.json");
  const pipe = model.pipe_segments[0];
  await selectTreeRow(page, "pipe", pipe.id);
  await ensureRail(page, "tree", false);
  await ensureRail(page, "inspector", false);
  const witnesses = [];
  for (const preset of ["Front", "Top", "Isometric"]) {
    await activateWithKeyboard(page, page.getByRole("button", { name: preset, exact: true }));
    for (const command of ["Fit Visible", "Fit Selection"]) {
      await activateWithKeyboard(page, page.getByRole("button", { name: command, exact: true }));
      const points = (command === "Fit Visible" ? model.nodes : model.nodes.filter((node: any) => node.id === pipe.from || node.id === pipe.to)).map((node: any) => node.position);
      const witness = await page.evaluate((points) => {
        const api = globalThis.__openPipeStressUiDiagnosticsV1;
        const snapshot = api.readCurrent();
        if ("status" in snapshot.viewport) throw new Error("Viewport unavailable");
        const camera = snapshot.viewport.camera;
        return { camera, canvas: snapshot.viewport.canvas, projections: points.map((authoredPoint: any) => api.projectAuthoredPoint({ modelGeneration: snapshot.model.generation, cameraSequence: camera.sequence, authoredPoint })) };
      }, points);
      expect(witness.camera.aspect).toBeLessThan(0.7);
      expect(witness.projections.length).toBeGreaterThan(0);
      for (const projection of witness.projections) {
        expect(projection.status).toBe("available");
        if (projection.status !== "available") throw new Error("Projection unavailable");
        expect(projection.insideClosedNdc).toBe(true);
        expect(Math.abs(projection.ndc.x)).toBeLessThanOrEqual(0.900001);
        expect(Math.abs(projection.ndc.y)).toBeLessThanOrEqual(0.900001);
      }
      witnesses.push({ preset, command, ...witness });
    }
  }
  await testInfo.attach("narrow-fit-envelope-witnesses", { body: JSON.stringify(witnesses, null, 2), contentType: "application/json" });
});

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
        if (label === "defaults" || label === "pointer lower clamps" || label === "both pointer max") {
          await captureState(page, testInfo, `rail-${theme}-${viewport.width}-${label}`);
        }
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

for (const route of ["toolbar", "Insert menu"] as const) {
  test(`Support creation preserves an active property Task through ${route}`, async ({ page }, testInfo) => {
    await page.setViewportSize({ width: 1440, height: 920 });
    const model = await gotoRoutedFixture(page);
    const pipe = model.pipe_segments[0];
    await selectTreeRow(page, "pipe", pipe.id);
    await ensureRail(page, "inspector", true);
    const inspector = page.getByTestId("property-inspector");
    await activateWithKeyboard(page, inspector.getByRole("tab", { name: "Task", exact: true }));
    await activateWithKeyboard(page, inspector.getByTestId("inspector-start-task"));
    await inspector.getByTestId("editor-intent-value").fill("Retained production support draft");
    const frozenTarget = await inspector.getByTestId("inspector-frozen-task-target").innerText();
    const readout = page.getByTestId("command-selection-readout");
    await expect(readout).toHaveText(`Selected pipe: ${pipe.id}; 0 queued`);
    const selected = (await readout.textContent())!.trim();
    if (route === "toolbar") await activateWithKeyboard(page, page.getByTestId("command-support"));
    else {
      await activateWithKeyboard(page, page.getByTestId("menu-insert"));
      await activateWithKeyboard(page, page.getByTestId("menu-item-insert.support"));
    }
    await expect(inspector.getByRole("tab", { name: "Properties", exact: true })).toHaveAttribute("aria-selected", "true");
    await expect(page.getByTestId("create-support-id")).toBeFocused();
    await expect(page.getByTestId("create-support-id")).toBeVisible();
    await expect(page.getByTestId("create-support-id").locator("xpath=ancestor::details[1]")).toHaveAttribute("open", "");
    await expect(page.getByTestId("command-selection-readout")).toHaveText(selected);
    await activateWithKeyboard(page, inspector.getByRole("tab", { name: "Task", exact: true }));
    await expect(inspector.getByTestId("inspector-frozen-task-target")).toHaveText(frozenTarget);
    await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("Retained production support draft");
    await captureState(page, testInfo, `support-route-retained-task-${route}`);
  });
}


test("empty ordered selection publishes independently from project inspector", async ({ page }, testInfo) => {
  await page.setViewportSize({ width: 1440, height: 920 });
  const model = await gotoRoutedFixture(page);
  expect(model.components ?? []).toHaveLength(0);
  await ensureRail(page, "inspector", true);
  await openWorkspaceSection(page, "results");
  await expect(page.getByTestId("results-panel")).toHaveCount(1);
  await activateWithKeyboard(page, page.getByTestId("workspace-dock-close"));
  const read = async () => page.evaluate(() => {
    const snapshot = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
    if ("status" in snapshot.viewport) throw new Error("Committed viewport unavailable");
    return { model: snapshot.model, viewport: snapshot.viewport };
  });
  const records: any[] = [];
  const identity = async (orderedRefs: any[], primaryRef: any, inspectorRef: any) => {
    await expect.poll(async () => {
      const current = await read();
      return { ordered: current.viewport.selection.orderedRefs, primary: current.viewport.selection.primaryRef,
        inspector: current.viewport.inspector.ref, pending: current.viewport.resources.ownedPendingRafCount,
        rendered: current.viewport.mainRender.selectionPresentation?.orderedRefs,
        selectionMatchesCurrentFrame: current.viewport.selection.renderSubmissionSequence > 0 &&
          current.viewport.selection.renderSubmissionSequence === current.viewport.mainRender.submissionSequence &&
          current.viewport.mainRender.selectionPresentation?.renderedSubmissionSequence === current.viewport.mainRender.submissionSequence };
    }).toEqual({ ordered: orderedRefs, primary: primaryRef, inspector: inspectorRef, pending: 0, rendered: orderedRefs, selectionMatchesCurrentFrame: true });
    await expect(page.getByTestId("property-inspector").getByRole("heading", { level: 2 })).toContainText(`${inspectorRef.type}: ${inspectorRef.id}`);
    const snapshot = await read();
    expect(snapshot.viewport.selection.generation).toBe(snapshot.model.generation);
    expect(snapshot.viewport.inspector.generation).toBe(snapshot.model.generation);
    const presentation = snapshot.viewport.mainRender.selectionPresentation!;
    expect(presentation.resourceGeneration).toBe(snapshot.viewport.resources.context.generation);
    expect(presentation.modelGeneration).toBe(snapshot.model.generation);
    expect(presentation.revision).toBeGreaterThan(0);
    expect(presentation.renderedSubmissionSequence).toBeGreaterThan(presentation.appliedAfterSubmissionSequence);
    records.push(snapshot);
    return snapshot;
  };
  const project = { type: "project", id: model.project.id };
  const node = { type: "node", id: model.nodes[10].id };
  await selectTreeRow(page, project.type, project.id);
  const capturePresented = async (name: string) => {
    const before = await read();
    await page.getByTestId("viewport-canvas").locator("canvas").screenshot({ path: testInfo.outputPath(`${name}.png`) });
    const after = await read();
    expect(after.viewport.mainRender).toEqual(before.viewport.mainRender);
    expect(after.viewport.selection).toEqual(before.viewport.selection);
    const projected = await page.evaluate((position) => {
      const api = globalThis.__openPipeStressUiDiagnosticsV1;
      const current = api.readCurrent();
      if ("status" in current.viewport || current.model.generation === null) throw new Error("Viewport unavailable");
      return api.projectAuthoredPoint({ modelGeneration: current.model.generation, cameraSequence: current.viewport.camera.sequence, authoredPoint: position });
    }, model.nodes[10].position);
    await testInfo.attach(name, { body: JSON.stringify({ before, after, projected, selectedNode: model.nodes[10] }, null, 2), contentType: "application/json" });
  };
  await activateWithKeyboard(page, page.getByTestId("toggle-viewport-labels"));
  await expect(page.getByTestId("toggle-viewport-labels")).toHaveAttribute("aria-pressed", "false");
  const baseline = await identity([project], project, project);
  const resultsBefore = await page.getByTestId("results-panel").allTextContents();
  await selectTreeRow(page, node.type, node.id);
  const selected = await identity([node], node, node);
  expect(selected.viewport.inspector.publicationSequence).toBeGreaterThan(baseline.viewport.inspector.publicationSequence);
  await capturePresented("selected-node-presentation");
  await activateWithKeyboard(page, page.getByRole("button", { name: "Hide", exact: true }));
  await identity([node], node, node);
  await capturePresented("hidden-selected-node-presentation");
  await activateWithKeyboard(page, page.getByRole("button", { name: "Show All", exact: true }));
  await identity([node], node, node);
  await selectTreeRow(page, node.type, node.id, { toggle: true });
  const empty = await identity([], null, project);
  await capturePresented("empty-after-node-presentation");
  expect(empty.viewport.selection.actionSequence).toBeGreaterThan(selected.viewport.selection.actionSequence);
  await activateWithKeyboard(page, page.getByTestId("viewport-box-select"));
  await page.getByTestId("viewport-selection-filter").selectOption("components");
  const canvas = page.getByTestId("viewport-canvas").locator("canvas");
  const rect = await canvas.boundingBox();
  expect(rect).not.toBeNull();
  const dragEmpty = async (modifier?: "Shift" | "Control") => {
    const before = await read();
    const priorBoxAction = "status" in before.viewport.box ? null : before.viewport.box.actionSequence;
    const start = { x: rect!.x + rect!.width * 0.08, y: rect!.y + rect!.height * 0.10 };
    const end = { x: rect!.x + rect!.width * 0.29, y: rect!.y + rect!.height * 0.28 };
    for (const point of [start, end]) {
      expect(await page.evaluate(({ x, y }) => document.elementFromPoint(x, y) === document.querySelector('[data-testid="viewport-canvas"] canvas'), point)).toBe(true);
    }
    if (modifier) await page.keyboard.down(modifier);
    await page.mouse.move(start.x, start.y); await page.mouse.down();
    await page.mouse.move(end.x, end.y, { steps: 6 }); await page.mouse.up();
    if (modifier) await page.keyboard.up(modifier);
    await expect.poll(async () => {
      const box = (await read()).viewport.box;
      return !("status" in box) && (priorBoxAction === null || box.actionSequence > priorBoxAction);
    }).toBe(true);
    const expected = modifier ? [node] : [];
    const repeated = await identity(expected, modifier ? node : null, modifier ? node : project);
    expect(repeated.viewport.box).toMatchObject({ orderedRefs: expected, primaryRef: modifier ? node : null, filter: "components", direction: "left-to-right" });
    expect(repeated.viewport.selection.renderSubmissionSequence).toBeGreaterThan(before.viewport.mainRender.submissionSequence);
    expect(repeated.viewport.box).toMatchObject({ renderSubmissionSequence: repeated.viewport.mainRender.submissionSequence });
  };
  await dragEmpty(); await dragEmpty();
  await selectTreeRow(page, node.type, node.id); await identity([node], node, node);
  await dragEmpty("Shift"); await dragEmpty("Control");
  await activateWithKeyboard(page, page.getByTestId("viewport-box-select"));
  await selectTreeRow(page, node.type, node.id);
  await identity([node], node, node);
  await selectTreeRow(page, project.type, project.id);
  await identity([project], project, project);
  for (const snapshot of records) {
    expect(snapshot.model).toEqual(baseline.model);
    expect(snapshot.viewport.camera).toEqual(baseline.viewport.camera);
    expect(snapshot.viewport.canvas).toEqual(baseline.viewport.canvas);
    expect(snapshot.viewport.resources.ownedPendingRafCount).toBe(0);
  }
  expect(await page.getByTestId("results-panel").allTextContents()).toEqual(resultsBefore);
  await expect(page.getByTestId("command-selection-readout")).toContainText("0 queued");
  await activateWithKeyboard(page, page.getByRole("button", { name: "Save local", exact: true }));
  await expect(page.getByTestId("local-project-message")).toContainText("Saved");
  await selectTreeRow(page, node.type, node.id);
  await identity([node], node, node);
  await selectTreeRow(page, node.type, node.id, { toggle: true });
  const beforeReplacement = await identity([], null, project);
  await activateWithKeyboard(page, page.getByTestId("open-local-project"));
  await expect.poll(async () => (await read()).model.generation).not.toBe(beforeReplacement.model.generation);
  const reopened = await identity([project], project, project);
  expect(reopened.model.identityHash).toBe(baseline.model.identityHash);
  expect(reopened.viewport.inspector.publicationSequence).toBeGreaterThan(beforeReplacement.viewport.inspector.publicationSequence);
  await selectTreeRow(page, node.type, node.id);
  await identity([node], node, node);
  await selectTreeRow(page, node.type, node.id, { toggle: true });
  await identity([], null, project);
  await testInfo.attach("actual-empty-selection-publications", { body: JSON.stringify({ model: model.project.id, independentEmptyBasis: "fixture has zero components", records }, null, 2), contentType: "application/json" });
  await page.screenshot({ path: testInfo.outputPath("project-after-empty.png") });
});


test("decorative viewport overlays pass real canvas gestures while view controls stay interactive", async ({ page }, testInfo) => {
  await page.setViewportSize({ width: 1440, height: 920 });
  const model = await gotoRoutedFixture(page);
  expect(model.components ?? []).toHaveLength(0);
  await activateWithKeyboard(page, page.getByTestId("toggle-viewport-labels"));
  await expect(page.getByTestId("toggle-viewport-labels")).toHaveAttribute("aria-pressed", "false");
  const canvas = page.getByTestId("viewport-canvas").locator("canvas");
  const axis = page.getByRole("img", { name: "Orientation gizmo showing X, Y, Z axes", exact: true });
  const scale = page.getByTestId("viewport-scale-bar");
  await expect(axis).toBeVisible(); await expect(scale).toHaveText(/1\s*m/);
  const rect = (await canvas.boundingBox())!;
  const axisRect = (await axis.boundingBox())!;
  const scaleRect = (await scale.boundingBox())!;
  // Preserve both frozen Box16 endpoints. This component-free control fixture
  // supplies an independent empty expectation; it is not a cohort replay.
  const start = { x: rect.x + rect.width * 0.29, y: rect.y + rect.height * 0.58 };
  const end = { x: rect.x + rect.width * 0.08, y: rect.y + rect.height * 0.81 };
  const scalePoint = { x: scaleRect.x + scaleRect.width / 2, y: scaleRect.y + scaleRect.height / 2 };
  expect(end.x).toBeGreaterThan(axisRect.x); expect(end.x).toBeLessThan(axisRect.x + axisRect.width);
  expect(end.y).toBeGreaterThan(axisRect.y); expect(end.y).toBeLessThan(axisRect.y + axisRect.height);
  await page.evaluate(() => {
    const records: any[] = []; (globalThis as any).__decorativePointerWitness = records;
    for (const type of ["pointerdown", "pointerup", "wheel"]) document.addEventListener(type, (event) => {
      const target = event.target as HTMLElement;
      const pointer = event as MouseEvent;
      records.push({ type, trusted: event.isTrusted, x: pointer.clientX, y: pointer.clientY,
        mainCanvas: target === document.querySelector('[data-testid="viewport-canvas"] canvas'),
        targetTag: target.tagName, targetTestId: target.dataset.testid ?? null });
    }, { capture: true, passive: true });
  });
  const hits = await page.evaluate((points) => points.map((point) => {
    const target = document.elementFromPoint(point.x, point.y) as HTMLElement | null;
    return { point, mainCanvas: target === document.querySelector('[data-testid="viewport-canvas"] canvas'),
      tag: target?.tagName, testId: target?.dataset.testid ?? null };
  }), [start, end, scalePoint]);
  // Record trusted events at both decorative surfaces before asserting passthrough,
  // so the unpatched failure includes actual pointer ownership, not CSS inspection.
  for (const point of [end, scalePoint]) { await page.mouse.move(point.x, point.y); await page.mouse.down(); await page.mouse.up(); }
  const pointerPreconditions = await page.evaluate(() => (globalThis as any).__decorativePointerWitness);
  await testInfo.attach("decorative-pointer-preconditions", { body: JSON.stringify({ rect, axisRect, scaleRect, hits, pointerPreconditions }, null, 2), contentType: "application/json" });
  expect(hits.every((hit) => hit.mainCanvas), JSON.stringify(hits)).toBe(true);
  expect(pointerPreconditions.every((event: any) => event.trusted && event.mainCanvas)).toBe(true);
  const read = async () => page.evaluate(() => {
    const snapshot = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
    if ("status" in snapshot.viewport) throw new Error("Viewport unavailable");
    return snapshot.viewport;
  });
  const settled = async () => {
    await expect.poll(async () => (await read()).resources.ownedPendingRafCount).toBe(0);
    return read();
  };
  await activateWithKeyboard(page, page.getByTestId("viewport-box-select"));
  await page.getByTestId("viewport-selection-filter").selectOption("components");
  const beforeBox = await settled();
  await page.mouse.move(start.x, start.y); await page.mouse.down();
  await page.mouse.move(end.x, end.y, { steps: 6 }); await page.mouse.up();
  await expect.poll(async () => { const box = (await read()).box; return "status" in box ? null : box.orderedRefs; }).toEqual([]);
  const afterBox = await settled();
  expect(afterBox.box).toMatchObject({ orderedRefs: [], primaryRef: null, direction: "right-to-left", filter: "components" });
  expect(afterBox.selection.orderedRefs).toEqual([]); expect(afterBox.selection.primaryRef).toBeNull();
  expect(afterBox.camera).toEqual(beforeBox.camera); expect(afterBox.canvas).toEqual(beforeBox.canvas);
  await activateWithKeyboard(page, page.getByTestId("viewport-box-select"));
  const orbitBefore = await settled();
  await page.mouse.move(end.x, end.y); await page.mouse.down();
  await page.mouse.move(end.x + 42, end.y - 28, { steps: 8 }); await page.mouse.up();
  const orbitAfter = await settled();
  expect(orbitAfter.camera.position).not.toEqual(orbitBefore.camera.position);
  expect(orbitAfter.camera.target).toEqual(orbitBefore.camera.target);
  await page.mouse.move(scalePoint.x, scalePoint.y); await page.mouse.down({ button: "right" });
  await page.mouse.move(scalePoint.x - 40, scalePoint.y - 25, { steps: 8 }); await page.mouse.up({ button: "right" });
  const panAfter = await settled();
  expect(panAfter.camera.target).not.toEqual(orbitAfter.camera.target);
  panAfter.camera.position.forEach((value, i) => expect(value - orbitAfter.camera.position[i]).toBeCloseTo(panAfter.camera.target[i] - orbitAfter.camera.target[i], 8));
  const distance = (state: typeof panAfter) => Math.hypot(...state.camera.position.map((value, i) => value - state.camera.target[i]));
  await page.mouse.move(scalePoint.x, scalePoint.y); await page.mouse.wheel(0, -120);
  await expect.poll(async () => distance(await read())).toBeLessThan(distance(panAfter));
  const wheelAfter = await settled();
  expect(wheelAfter.camera.target).toEqual(panAfter.camera.target);
  const front = page.getByRole("button", { name: "Front", exact: true });
  await front.click(); await expect(front).toHaveAttribute("aria-pressed", "true");
  const frontAfter = await settled(); expect(frontAfter.camera.position).not.toEqual(wheelAfter.camera.position);
  await page.getByTestId("viewport-view-isometric").click(); await settled();
  await activateWithKeyboard(page, page.getByTestId("toggle-viewport-labels"));
  const node = model.nodes[10]; await selectTreeRow(page, "node", node.id);
  const label = page.getByRole("button", { name: `Select ${node.label} in viewport`, exact: true });
  await expect(label).toBeVisible(); await label.click();
  await expect(page.getByTestId("command-selection-readout")).toContainText(`node: ${node.id}`);
  await expect(axis).toBeVisible(); await expect(scale).toHaveText(/1\s*m/);
  const events = await page.evaluate(() => (globalThis as any).__decorativePointerWitness);
  expect(events.filter((event: any) => event.type === "wheel")).toEqual(expect.arrayContaining([expect.objectContaining({ trusted: true, mainCanvas: true })]));
  await testInfo.attach("decorative-pointer-gesture-results", { body: JSON.stringify({ beforeBox, afterBox, orbitBefore, orbitAfter, panAfter, wheelAfter, frontAfter, events }, null, 2), contentType: "application/json" });
  await page.screenshot({ path: testInfo.outputPath("decorative-overlays-and-view-controls.png") });
});

// V30: genuine captured-pointer lifetime, including independent normal Open control.
test.describe("captured Box lifetime", () => {
async function read(page: Page) {
  return page.evaluate(() => {
    const snapshot = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
    if ("status" in snapshot.viewport) throw new Error("Viewport unavailable");
    return { snapshot, rectangleCount: document.querySelectorAll(".viewport-box-rect").length,
      readout: document.querySelector('[data-testid="command-selection-readout"]')?.textContent,
      inspector: document.querySelector('[data-testid="property-inspector"] h2')?.textContent,
      publicPreparationControls: [...document.querySelectorAll<HTMLElement>('[data-testid*="draft-review"], [data-testid*="direct-review"]')].map(e => ({ id: e.dataset.testid, text: e.textContent })),
      events: (globalThis as any).__boxGestureEvents };
  });
}
async function settle(page: Page) {
  await expect.poll(async () => (await read(page)).snapshot.viewport.resources.ownedPendingRafCount).toBe(0);
  return read(page);
}
async function setup(page: Page) {
  const model = await gotoRoutedFixture(page); expect(model.components ?? []).toHaveLength(0);
  await activateWithKeyboard(page, page.getByTestId("toggle-viewport-labels"));
  await selectTreeRow(page, "node", model.nodes[10].id);
  await expect.poll(async () => (await read(page)).snapshot.viewport.selection.orderedRefs).toEqual([{ type: "node", id: model.nodes[10].id }]);
  await page.evaluate(() => {
    const events: any[] = []; (globalThis as any).__boxGestureEvents = events;
    for (const type of ["pointerdown", "pointermove", "pointerup", "pointercancel", "lostpointercapture", "keydown"]) document.addEventListener(type, event => {
      const e = event as PointerEvent & KeyboardEvent;
      events.push({ type, trusted: event.isTrusted, key: e.key ?? null, pointerId: e.pointerId ?? null, buttons: e.buttons ?? null,
        targetTag: (event.target as Element).tagName, mainCanvas: event.target === document.querySelector('[data-testid="viewport-canvas"] canvas') });
    }, true);
  });
  return model;
}
async function begin(page: Page) {
  await activateWithKeyboard(page, page.getByTestId("viewport-box-select"));
  await page.getByTestId("viewport-selection-filter").selectOption("components");
  const rect = (await page.getByTestId("viewport-canvas").locator("canvas").boundingBox())!;
  const start = { x: rect.x + .29 * rect.width, y: rect.y + .58 * rect.height };
  const end = { x: rect.x + .08 * rect.width, y: rect.y + .81 * rect.height };
  for (const point of [start, end]) expect(await page.evaluate(({ x, y }) => document.elementFromPoint(x, y) === document.querySelector('[data-testid="viewport-canvas"] canvas'), point)).toBe(true);
  const before = await settle(page);
  await page.mouse.move(start.x, start.y); await page.mouse.down();
  await page.mouse.move((start.x + end.x) / 2, (start.y + end.y) / 2, { steps: 5 });
  expect(await page.locator(".viewport-box-rect").count()).toBe(1);
  return { before, start, end, rect };
}
async function finish(page: Page, end: { x: number; y: number }) {
  await page.mouse.move(end.x, end.y, { steps: 5 }); await page.mouse.up();
  return settle(page);
}

test("ordinary real Box applies independent empty Components membership", async ({ page }, info) => {
  await setup(page); const input = await begin(page); const after = await finish(page, input.end);
  await info.attach("ordinary-box", { body: JSON.stringify({ input, after }, null, 2), contentType: "application/json" });
  expect(after.snapshot.viewport.selection.orderedRefs).toEqual([]); expect(after.snapshot.viewport.selection.primaryRef).toBeNull();
  expect(after.snapshot.viewport.box).toMatchObject({ orderedRefs: [], primaryRef: null, filter: "components" });
  expect(after.snapshot.viewport.camera).toEqual(input.before.snapshot.viewport.camera);
});

test("Escape retires captured Box before delayed pointer up", async ({ page }, info) => {
  await setup(page); const input = await begin(page);
  await page.keyboard.press("Escape"); const afterEscape = await read(page);
  await page.screenshot({ path: info.outputPath("after-escape-before-up.png") });
  const after = await finish(page, input.end);
  await info.attach("escape-reproduction", { body: JSON.stringify({ input, afterEscape, after }, null, 2), contentType: "application/json" });
  await page.screenshot({ path: info.outputPath("after-delayed-up.png") });
  expect.soft(afterEscape.rectangleCount).toBe(0);
  expect.soft(after.snapshot.viewport.selection.orderedRefs).toEqual(input.before.snapshot.viewport.selection.orderedRefs);
  expect.soft(after.snapshot.viewport.selection.primaryRef).toEqual(input.before.snapshot.viewport.selection.primaryRef);
  expect.soft(after.snapshot.viewport.selection.actionSequence).toBe(input.before.snapshot.viewport.selection.actionSequence);
  expect.soft(after.snapshot.viewport.box).toEqual(input.before.snapshot.viewport.box);
  expect.soft(after.publicPreparationControls).toEqual(input.before.publicPreparationControls);
  expect(after.snapshot.viewport.camera).toEqual(input.before.snapshot.viewport.camera);
});

test("same-ID Open retires old captured Box before delayed pointer up", async ({ page }, info) => {
  const model = await setup(page); const project = { type: "project", id: model.project.id };
  await page.getByRole("button", { name: "Save local", exact: true }).click();
  await expect(page.getByTestId("local-project-message")).toContainText("Saved");
  const beforeOrdinaryOpen = await settle(page);
  await activateWithKeyboard(page, page.getByTestId("open-local-project"));
  await expect.poll(async () => (await read(page)).snapshot.model.projectSessionGeneration).toBe(beforeOrdinaryOpen.snapshot.model.projectSessionGeneration + 1);
  await expect.poll(async () => (await read(page)).snapshot.viewport.selection.orderedRefs).toEqual([project]);
  const ordinaryOpen = await settle(page);
  expect(ordinaryOpen.snapshot.viewport.selection.primaryRef).toEqual(project);
  expect(ordinaryOpen.snapshot.viewport.inspector.ref).toEqual(project);
  await selectTreeRow(page, "node", model.nodes[10].id);
  const input = await begin(page);
  // Public keyboard route: keep the genuine mouse button held and captured.
  await page.getByTestId("open-local-project").focus(); await page.keyboard.press("Enter");
  await expect.poll(async () => (await read(page)).snapshot.model.projectSessionGeneration).toBe(input.before.snapshot.model.projectSessionGeneration + 1);
  await expect.poll(async () => (await read(page)).snapshot.viewport.selection.orderedRefs).toEqual([project]);
  const replaced = await settle(page);
  expect(replaced.snapshot.viewport.selection.primaryRef).toEqual(project);
  expect(replaced.snapshot.viewport.inspector.ref).toEqual(project);
  expect(replaced.snapshot.model.identityHash).toBe(ordinaryOpen.snapshot.model.identityHash);
  await page.mouse.move(input.end.x, input.end.y, { steps: 5 });
  const beforeDelayedUp = await read(page);
  await page.screenshot({ path: info.outputPath("replacement-delayed-move-before-up.png") });
  await page.mouse.up(); const after = await settle(page);
  await info.attach("replacement-reproduction", { body: JSON.stringify({ beforeOrdinaryOpen, ordinaryOpen, input, replaced, beforeDelayedUp, after }, null, 2), contentType: "application/json" });
  await page.screenshot({ path: info.outputPath("replacement-after-delayed-up.png") });
  expect.soft(beforeDelayedUp.rectangleCount).toBe(0);
  expect.soft(after.snapshot.viewport.selection.orderedRefs).toEqual([project]);
  expect.soft(after.snapshot.viewport.selection.primaryRef).toEqual(project);
  expect.soft(after.snapshot.viewport.selection.actionSequence).toBe(replaced.snapshot.viewport.selection.actionSequence);
  expect.soft(after.snapshot.viewport.box).toEqual(replaced.snapshot.viewport.box);
  expect.soft(after.snapshot.model).toEqual(replaced.snapshot.model);
  expect(after.snapshot.viewport.camera).toEqual(replaced.snapshot.viewport.camera);
});

for (const route of ["Select", "New blank"] as const) test(`${route} retires captured Box before delayed pointer up`, async ({ page }, info) => {
  await setup(page); const input = await begin(page);
  const control = route === "Select" ? page.getByTestId("workspace-select") : page.getByRole("button", { name: "New blank", exact: true });
  await control.focus(); await page.keyboard.press("Enter");
  if (route === "New blank") {
    await expect.poll(async () => (await read(page)).snapshot.model.projectSessionGeneration).toBe(input.before.snapshot.model.projectSessionGeneration + 1);
    await expect.poll(async () => (await read(page)).snapshot.viewport.selection.primaryRef?.type).toBe("project");
  }
  const cancelled = await settle(page);
  expect(cancelled.rectangleCount).toBe(0);
  if (route === "Select") expect(cancelled.snapshot.viewport.selection.orderedRefs).toEqual(input.before.snapshot.viewport.selection.orderedRefs);
  else {
    const primary = cancelled.snapshot.viewport.selection.primaryRef!;
    expect(primary.id).toMatch(/^project:blank-local-/);
    expect(cancelled.snapshot.viewport.selection.orderedRefs).toEqual([primary]);
    expect(cancelled.snapshot.viewport.inspector.ref).toEqual(primary);
  }
  const after = await finish(page, input.end);
  expect(after.rectangleCount).toBe(0);
  expect(after.snapshot.viewport.selection.orderedRefs).toEqual(cancelled.snapshot.viewport.selection.orderedRefs);
  expect(after.snapshot.viewport.selection.primaryRef).toEqual(cancelled.snapshot.viewport.selection.primaryRef);
  expect(after.snapshot.viewport.selection.actionSequence).toBe(cancelled.snapshot.viewport.selection.actionSequence);
  expect(after.snapshot.viewport.box).toEqual(cancelled.snapshot.viewport.box);
  expect(after.snapshot.model).toEqual(cancelled.snapshot.model);
  expect(after.snapshot.viewport.camera).toEqual(cancelled.snapshot.viewport.camera);
  expect(after.publicPreparationControls).toEqual(cancelled.publicPreparationControls);
  await info.attach("cancelled-delayed-input", { body: JSON.stringify({ input, cancelled, after }, null, 2), contentType: "application/json" });
});
for (const route of ["Measure", "palette Node and Select"] as const) test(`${route} cannot revive a captured Box through a mode ABA`, async ({ page }, info) => {
  await setup(page); const input = await begin(page);
  if (route === "Measure") {
    await page.getByRole("button", { name: "Measure", exact: true }).focus(); await page.keyboard.press("Enter");
    await expect(page.getByRole("button", { name: "Measure", exact: true })).toHaveAttribute("aria-pressed", "true");
  } else {
    await choosePaletteNodeThenSelectWithFocusEvidence(page, info);
  }
  const exited = await read(page);
  if (await page.getByTestId("viewport-box-select").getAttribute("aria-pressed") !== "true") {
    await page.getByTestId("viewport-box-select").focus(); await page.keyboard.press("Enter");
  }
  await expect(page.getByTestId("viewport-box-select")).toHaveAttribute("aria-pressed", "true");
  const after = await finish(page, input.end);
  await info.attach("mode-aba", { body: JSON.stringify({ input, exited, after }, null, 2), contentType: "application/json" });
  expect.soft(exited.rectangleCount).toBe(0);
  expect.soft(after.snapshot.viewport.selection.orderedRefs).toEqual(input.before.snapshot.viewport.selection.orderedRefs);
  expect.soft(after.snapshot.viewport.selection.primaryRef).toEqual(input.before.snapshot.viewport.selection.primaryRef);
  expect.soft(after.snapshot.viewport.selection.actionSequence).toBe(input.before.snapshot.viewport.selection.actionSequence);
  expect.soft(after.snapshot.viewport.box).toEqual(input.before.snapshot.viewport.box);
  expect(after.snapshot.model).toEqual(input.before.snapshot.model);
});

for (const captured of [false, true]) test(`workspace Escape event ownership body ${captured ? "captured" : "idle"}`, async ({ page }, info) => {
  await setup(page);
  const input = captured ? await begin(page) : null;
  if (!captured) await activateWithKeyboard(page, page.getByTestId("viewport-box-select"));
  const before = input?.before ?? await settle(page);
  // Deliberate browser focus setup; the subsequent Escape is trusted keyboard input.
  await page.evaluate(() => (document.activeElement as HTMLElement)?.blur());
  expect(await page.evaluate(() => document.activeElement === document.body)).toBe(true);
  await page.keyboard.press("Escape");
  const cancelled = await read(page);
  expect.soft(cancelled.rectangleCount).toBe(0);
  await expect.soft(page.getByTestId("viewport-box-select")).toHaveAttribute("aria-pressed", "false");
  await expect.soft(page.getByTestId("workspace-select")).toBeFocused();
  const after = input ? await finish(page, input.end) : await settle(page);
  await info.attach("body-Escape", { body: JSON.stringify({ before, cancelled, after }, null, 2), contentType: "application/json" });
  expect(after.events.some((event: any) => event.type === "keydown" && event.key === "Escape" && event.trusted && event.targetTag === "BODY")).toBe(true);
  expect(after.snapshot.viewport.selection.orderedRefs).toEqual(before.snapshot.viewport.selection.orderedRefs);
  expect(after.snapshot.viewport.selection.primaryRef).toEqual(before.snapshot.viewport.selection.primaryRef);
  expect(after.snapshot.viewport.selection.actionSequence).toBe(before.snapshot.viewport.selection.actionSequence);
  expect(after.snapshot.viewport.box).toEqual(before.snapshot.viewport.box);
  expect(after.publicPreparationControls).toEqual(before.publicPreparationControls);
  expect(after.snapshot.model).toEqual(before.snapshot.model);
  expect(after.snapshot.viewport.camera).toEqual(before.snapshot.viewport.camera);
});
for (const surface of ["palette", "drawer"] as const) test(`workspace Escape event ownership consumed ${surface}`, async ({ page }, info) => {
  if (surface === "drawer") await page.setViewportSize({ width: 1024, height: 768 });
  await setup(page); await activateWithKeyboard(page, page.getByTestId("viewport-box-select"));
  const before = await settle(page);
  if (surface === "palette") {
    await activateWithKeyboard(page, page.getByTestId("toolkit-entry"));
    await expect(page.getByRole("searchbox", { name: "Find a tool" })).toBeFocused();
  } else {
    await activateWithKeyboard(page, page.getByTestId("toggle-inspector"));
    await page.getByTestId("property-inspector").getByRole("button", { name: "Queue delete node", exact: true }).focus();
  }
  await page.keyboard.press("Escape");
  if (surface === "palette") await expect(page.getByRole("dialog", { name: "Find a modeling tool" })).toHaveCount(0);
  else await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");
  await expect(page.getByTestId(surface === "palette" ? "toolkit-entry" : "toggle-inspector")).toBeFocused();
  await expect(page.getByTestId("viewport-box-select")).toHaveAttribute("aria-pressed", "true");
  const consumed = await settle(page);
  expect(consumed.snapshot.viewport.selection.orderedRefs).toEqual(before.snapshot.viewport.selection.orderedRefs);
  expect(consumed.snapshot.viewport.selection.actionSequence).toBe(before.snapshot.viewport.selection.actionSequence);
  await page.evaluate(() => (document.activeElement as HTMLElement)?.blur());
  expect(await page.evaluate(() => document.activeElement === document.body)).toBe(true);
  await page.keyboard.press("Escape");
  await expect(page.getByTestId("viewport-box-select")).toHaveAttribute("aria-pressed", "false");
  await expect(page.getByTestId("workspace-select")).toBeFocused();
  await info.attach("consumed-Escape", { body: JSON.stringify({ before, consumed, after: await settle(page) }, null, 2), contentType: "application/json" });
});

});
