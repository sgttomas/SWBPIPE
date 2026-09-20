# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: ui-foundation.spec.ts >> task and analysis dock preserve usable canvas dark comfortable 1440x920
- Location: e2e/ui-foundation.spec.ts:671:7

# Error details

```
Error: page.goto: Test ended.
Call log:
  - navigating to "http://127.0.0.1:5174/", waiting until "load"

```

# Test source

```ts
  573 | 
  574 |   await expect(page.getByTestId("toggle-viewport-labels")).toHaveAttribute("aria-pressed", "true");
  575 |   const labelSample = await page.getByTestId("viewport-selection-layer").locator(".viewport-select-target").evaluateAll((targets) => {
  576 |     const mainCanvas = document.querySelector<HTMLCanvasElement>('[data-testid="viewport-canvas"] canvas');
  577 |     const canvasRect = mainCanvas?.getBoundingClientRect();
  578 |     if (!mainCanvas || !canvasRect) return null;
  579 |     for (const target of targets) {
  580 |       if (!(target instanceof HTMLElement)) continue;
  581 |       const rect = target.getBoundingClientRect();
  582 |       const center = { x: rect.left + rect.width / 2, y: rect.top + rect.height / 2 };
  583 |       if (center.x < canvasRect.left || center.x > canvasRect.right || center.y < canvasRect.top || center.y > canvasRect.bottom) continue;
  584 |       const hit = document.elementFromPoint(center.x, center.y);
  585 |       if (hit?.closest(".viewport-select-target") === target && target.dataset.testid) {
  586 |         return {
  587 |           ariaLabel: target.getAttribute("aria-label"),
  588 |           center,
  589 |           testId: target.dataset.testid,
  590 |         };
  591 |       }
  592 |     }
  593 |     return null;
  594 |   });
  595 |   expect(labelSample, "an ordinary visible viewport label has a real pointer-hit center").not.toBeNull();
  596 |   const sampledLabel = page.getByTestId(labelSample!.testId);
  597 |   const ordinaryLabelHit = await describeViewportHit(page, labelSample!.center);
  598 |   expect(ordinaryLabelHit.closestSelectionTargetTestId).toBe(labelSample!.testId);
  599 |   expect(ordinaryLabelHit.isMainCanvas).toBe(false);
  600 |   await page.mouse.click(labelSample!.center.x, labelSample!.center.y);
  601 |   await expect(sampledLabel).toHaveAttribute("aria-pressed", "true");
  602 | 
  603 |   // Freeze the label center and a separated canvas endpoint before Box mode.
  604 |   // The start coordinate is intentionally never resampled after Box is armed.
  605 |   const supportingPath = await visibleCanvasDragPath(canvas);
  606 |   const distanceTo = (point: Readonly<{ x: number; y: number }>) =>
  607 |     Math.hypot(point.x - labelSample!.center.x, point.y - labelSample!.center.y);
  608 |   const labelBoxPath = {
  609 |     start: labelSample!.center,
  610 |     end: distanceTo(supportingPath.start) >= distanceTo(supportingPath.end) ? supportingPath.start : supportingPath.end,
  611 |   };
  612 |   expect(distanceTo(labelBoxPath.end)).toBeGreaterThan(40);
  613 | 
  614 |   await activateWithKeyboard(page, boxSelect);
  615 |   await expect(boxSelect).toHaveAttribute("aria-pressed", "true");
  616 |   const boxLabelHit = await describeViewportHit(page, labelSample!.center);
  617 |   expect(boxLabelHit.isMainCanvas).toBe(true);
  618 |   expect(boxLabelHit.closestSelectionTargetTestId).toBeNull();
  619 | 
  620 |   const boxGestures = [];
  621 |   for (const modifier of [null, "Shift", "Control"] as const) {
  622 |     const before = await settledCameraProjectionWitness(page, authoredPoints);
  623 |     const path = modifier === null ? labelBoxPath : await visibleCanvasDragPath(canvas);
  624 |     if (modifier) await page.keyboard.down(modifier);
  625 |     try {
  626 |       await page.mouse.move(path.start.x, path.start.y);
  627 |       await page.mouse.down({ button: "left" });
  628 |       await page.mouse.move((path.start.x + path.end.x) / 2, (path.start.y + path.end.y) / 2, { steps: 3 });
  629 |       await expect(page.locator(".viewport-box-rect")).toBeVisible();
  630 |       await page.mouse.move(path.end.x, path.end.y, { steps: 3 });
  631 |       await page.mouse.up({ button: "left" });
  632 |     } finally {
  633 |       if (modifier) await page.keyboard.up(modifier);
  634 |     }
  635 |     await expect(page.locator(".viewport-box-rect")).toHaveCount(0);
  636 |     const after = await settledCameraProjectionWitness(page, authoredPoints);
  637 |     expect(after.camera, `${modifier ?? "plain"} Box Select camera`).toEqual(before.camera);
  638 |     expect(after.projections, `${modifier ?? "plain"} Box Select authored projection`).toEqual(before.projections);
  639 |     expect(after.canvasRect, `${modifier ?? "plain"} Box Select published canvas rectangle`).toEqual(before.canvasRect);
  640 |     expect(after.canvasDomRect, `${modifier ?? "plain"} Box Select DOM canvas rectangle`).toEqual(before.canvasDomRect);
  641 |     expect(after.ownedPendingRafCount, `${modifier ?? "plain"} Box Select pending product RAF`).toBe(0);
  642 |     boxGestures.push({ modifier: modifier ?? "plain", path, before, after });
  643 |   }
  644 | 
  645 |   await testInfo.attach("box-select-camera-projection-invariance", {
  646 |     body: JSON.stringify({
  647 |       ordinaryOrbit: { path: ordinaryPath, before: ordinaryBefore, after: ordinaryAfter },
  648 |       activeInertiaTakeover: { immediate: activeInertia, before: takeoverBefore, after: takeoverAfter },
  649 |       navigationEndpoints: { firstIsometric, firstFit, orbitMoved, secondFit, rightPanMoved, frontEndpoint, isometricEndpoint },
  650 |       frozenLabelInterception: {
  651 |         sample: labelSample,
  652 |         ordinaryModeHit: ordinaryLabelHit,
  653 |         boxModeHit: boxLabelHit,
  654 |         boxDragPath: labelBoxPath,
  655 |       },
  656 |       boxGestures,
  657 |     }, null, 2),
  658 |     contentType: "application/json",
  659 |   });
  660 | });
  661 | 
  662 | // Frozen regression floor: authoring and analysis together must retain a
  663 | // 200 × 200 CSS-pixel canvas, with controls and drafts still usable. The floor is unchanged.
  664 | // Slice B3 moves the structure it is measured in: Analyze is a page that opens over the stage's
  665 | // surfaces (the dock under the canvas is gone), so the canvas keeps its box under the open page,
  666 | // the page's controls are checked with the page open, and the canvas's own controls and drafts are
  667 | // checked with it closed. The solve proof is read on the Results stage's Evidence tab.
  668 | for (const theme of APPEARANCE_THEMES) {
  669 |   for (const density of APPEARANCE_DENSITIES) {
  670 |     for (const viewport of APPEARANCE_VIEWPORTS) {
  671 |       test(`task and analysis dock preserve usable canvas ${theme} ${density} ${viewport.width}x${viewport.height}`, async ({ page }, testInfo) => {
  672 |         await page.setViewportSize(viewport);
> 673 |         await page.goto("/");
      |                    ^ Error: page.goto: Test ended.
  674 |         await setAppearance(page, theme, density);
  675 |         const closePage = async () => {
  676 |           const close = page.getByTestId("workspace-dock-close");
  677 |           if (await close.isVisible()) await activateWithKeyboard(page, close);
  678 |           await expect(page.getByTestId("workspace-dock")).toHaveClass(/collapsed/);
  679 |         };
  680 |         await startPropertyTaskFromTreeEntity(page, "node", "node:N-100");
  681 |         const inspector = page.getByTestId("property-inspector");
  682 |         await inspector.getByTestId("editor-intent-field").selectOption("label");
  683 |         await inspector.getByTestId("editor-intent-value").fill("Retained inspector task");
  684 |         if (viewport.width < 1280) await ensureRail(page, "inspector", false);
  685 |         await activateWithKeyboard(page, page.getByTestId("command-pipe"));
  686 |         const pipeForm = page.getByTestId("viewport-editor-intents");
  687 |         await expect(pipeForm).toHaveClass(/active/);
  688 |         await page.getByTestId("viewport-create-pipe-label").fill("Retained pipe draft");
  689 |         await page.getByTestId("viewport-create-pipe-provenance").fill("layout regression draft");
  690 |         await openWorkspaceSection(page, "solve");
  691 |         const dock = page.getByTestId("workspace-dock");
  692 |         await expect(dock).not.toHaveClass(/collapsed/);
  693 |         await expect(page.getByTestId("workspace-section-solve")).toBeVisible();
  694 | 
  695 |         const evidence: unknown[] = [];
  696 |         const measure = async (phase: string) => {
  697 |           await openWorkspaceSection(page, "solve");
  698 |           await expect(pipeForm).toHaveClass(/active/);
  699 |           await expect(dock).not.toHaveClass(/collapsed/);
  700 |           const geometry = await page.evaluate(() => {
  701 |             const rect = (selector: string) => {
  702 |               const element = document.querySelector<HTMLElement>(selector)!;
  703 |               const box = element.getBoundingClientRect();
  704 |               return { x: box.x, y: box.y, width: box.width, height: box.height, right: box.right, bottom: box.bottom };
  705 |             };
  706 |             const body = document.querySelector<HTMLElement>(".workspace-dock .workspace-dock-body")!;
  707 |             const bodyBox = body.getBoundingClientRect();
  708 |             const bodyClip = { top: Math.max(0, bodyBox.top + body.clientTop),
  709 |               bottom: Math.min(innerHeight, bodyBox.top + body.clientTop + body.clientHeight) };
  710 |             for (let ancestor = body.parentElement; ancestor; ancestor = ancestor.parentElement) {
  711 |               if (!/(auto|scroll|hidden|clip)/.test(getComputedStyle(ancestor).overflowY)) continue;
  712 |               const box = ancestor.getBoundingClientRect();
  713 |               bodyClip.top = Math.max(bodyClip.top, box.top + ancestor.clientTop);
  714 |               bodyClip.bottom = Math.min(bodyClip.bottom, box.top + ancestor.clientTop + ancestor.clientHeight);
  715 |             }
  716 |             return {
  717 |               dockBody: { ...rect(".workspace-dock .workspace-dock-body"), clientHeight: body.clientHeight,
  718 |                 scrollHeight: body.scrollHeight, scrollTop: body.scrollTop, clip: bodyClip,
  719 |                 usableHeight: Math.max(0, bodyClip.bottom - bodyClip.top) },
  720 |               canvas: rect(".viewport-canvas canvas"), toolbar: rect(".viewport-toolbar"),
  721 |               task: rect('[data-testid="viewport-editor-intents"]'), dock: rect(".workspace-dock"),
  722 |               surfaces: rect('[data-testid="modeling-workspace"]'),
  723 |               status: rect('[data-testid="workspace-status-bar"]'),
  724 |               bodyOverflowX: document.documentElement.scrollWidth - innerWidth,
  725 |               bodyOverflowY: document.documentElement.scrollHeight - innerHeight,
  726 |             };
  727 |           });
  728 |           evidence.push({ phase, geometry });
  729 |           await testInfo.attach(`canvas-budget-${phase.replace(/[^a-z0-9]+/gi, "-")}`, { body: JSON.stringify(geometry, null, 2), contentType: "application/json" });
  730 |           expect(geometry.canvas.width, `${phase} canvas width`).toBeGreaterThanOrEqual(200);
  731 |           expect(geometry.canvas.height, `${phase} canvas height`).toBeGreaterThanOrEqual(200);
  732 |           expect(geometry.canvas.x).toBeGreaterThanOrEqual(0);
  733 |           expect(geometry.canvas.y).toBeGreaterThanOrEqual(0);
  734 |           expect(geometry.canvas.right).toBeLessThanOrEqual(viewport.width);
  735 |           // The open page covers the stage's surfaces exactly; the canvas and the task form keep
  736 |           // their boxes inside those surfaces, under the page.
  737 |           for (const edge of ["x", "y", "width", "height"] as const) expect(geometry.dock[edge], `${phase} page ${edge}`).toBeCloseTo(geometry.surfaces[edge], 0);
  738 |           expect(geometry.canvas.bottom).toBeLessThanOrEqual(geometry.surfaces.bottom + 1);
  739 |           expect(geometry.task.bottom).toBeLessThanOrEqual(geometry.surfaces.bottom + 1);
  740 |           expect(geometry.dock.bottom).toBeLessThanOrEqual(geometry.status.y);
  741 |           expect(geometry.bodyOverflowX).toBeLessThanOrEqual(0);
  742 |           expect(geometry.bodyOverflowY).toBeLessThanOrEqual(0);
  743 |           if (phase.startsWith("solved")) {
  744 |             expect(geometry.dockBody.usableHeight, `${phase} usable dock body height`).toBeGreaterThan(64);
  745 |           }
  746 |           return geometry.canvas;
  747 |         };
  748 |         const beforeSelection = await measure("before solve");
  749 |         const selected = await selectTreeRow(page, "node", "node:N-110");
  750 |         await expect(selected).toHaveAttribute("aria-selected", "true");
  751 |         // Selecting in the tree closes the page onto the Model stage. Restore the same task and
  752 |         // page layout before comparing rectangles (measure reopens the page).
  753 |         if (viewport.width < 1280) await ensureRail(page, "tree", false);
  754 |         await expect(page.locator(".viewport-toolbar-selection-status")).toHaveText("Selected: node:N-110");
  755 |         expect(await measure("changed primary; task/dock restored")).toEqual(beforeSelection);
  756 |         await closePage();
  757 |         await ensureRail(page, "inspector", true);
  758 |         await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText("node: node:N-100");
  759 |         await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("Retained inspector task");
  760 |         const footer = inspector.getByTestId("task-action-footer");
  761 |         for (const control of await footer.getByRole("button").all()) {
  762 |           await control.scrollIntoViewIfNeeded();
  763 |           await expectCenterUnobscured(control);
  764 |           if (await control.isEnabled()) {
  765 |             await control.focus();
  766 |             await expect(control).toBeFocused();
  767 |           }
  768 |         }
  769 |         if (viewport.width < 1280) {
  770 |           await ensureRail(page, "inspector", false);
  771 |         } else {
  772 |           await expect(page.getByTestId("toggle-tree")).toHaveAttribute("aria-expanded", "true");
  773 |           await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "true");
```