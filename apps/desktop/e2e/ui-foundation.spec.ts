import { buildModelIndex } from "../src/features/workspace/modelIndex";
import { displayedBoundsForEntityKeys, fittedViewportDistance } from "../src/features/viewport/viewportSelection";
import { expect, test, type Locator, type Page } from "@playwright/test";
import {
  activateWithKeyboard,
  choosePaletteNodeThenSelectWithFocusEvidence,
  APPEARANCE_THEMES,
  APPEARANCE_DENSITIES,
  APPEARANCE_VIEWPORTS,
  attachBrowserIdentity,
  clearTreeFilter,
  commandGroupControl,
  currentModelHashThroughVisibleExport,
  ensureRail,
  expectCenterUnobscured,
  gotoModel,
  gotoRoutedFixture,
  keyboardMeasureTargets,
  openWorkspaceSection,
  readFixture,
  revealTreeRow,
  selectTreeRow,
  typedTreeRow,
  waitForSettledGlobalRaf,
  withOneInvalidOd,
  withTypedCollision,
} from "./ui-foundation-workflows";

import { startPropertyTaskFromTreeEntity } from "./workspace-driver";

test.beforeAll(async ({ browser }, testInfo) => {
  await attachBrowserIdentity(browser, testInfo);
});

test("[preflight] maintained fixture supports typed selection and a keyboard-authored measurement", async ({ page }) => {
  const model = await gotoRoutedFixture(page);
  const hashBefore = await currentModelHashThroughVisibleExport(page);
  const pipe = model.pipe_segments[10];
  const from = model.nodes.find((node: any) => node.id === pipe.from)!;
  const to = model.nodes.find((node: any) => node.id === pipe.to)!;
  await selectTreeRow(page, "pipe", pipe.id);
  await clearTreeFilter(page);
  await activateWithKeyboard(page, page.getByTestId("viewport-fit-selection"));

  const target = page.getByRole("button", { name: `Select ${pipe.label} in viewport`, exact: true });
  await expect(target).toBeVisible();
  const readout = await keyboardMeasureTargets(page, [`Select ${pipe.label} in viewport`]);
  const expectedMetres: Record<MeasurementLabel, number> = {
    Distance: Math.hypot(
      to.position.x - from.position.x,
      to.position.y - from.position.y,
      to.position.z - from.position.z,
    ),
    "ΔX": to.position.x - from.position.x,
    "ΔY": to.position.y - from.position.y,
    "ΔZ": to.position.z - from.position.z,
  };
  await expect(readout.getByText(`Measure · pipe: ${pipe.id}`, { exact: true })).toBeVisible();
  await expectMeasurementQuantities(readout, expectedMetres, "m", "entered");

  const unitPreference = page.locator("details.display-preference-control");
  await unitPreference.locator("summary").click();
  const unitSelector = unitPreference.getByRole("combobox", { name: "Display units", exact: true });
  await unitSelector.selectOption("SI");
  await expect(unitSelector).toHaveValue("SI");
  await expectMeasurementQuantities(readout, expectedMetres, "m", "converted");
  await expect(readout.getByText(`Measure · pipe: ${pipe.id}`, { exact: true })).toBeVisible();

  const expectedInches = Object.fromEntries(
    MEASUREMENT_LABELS.map((label) => [label, expectedMetres[label] / METRES_PER_INCH]),
  ) as Record<MeasurementLabel, number>;
  await unitSelector.selectOption("US");
  await expect(unitSelector).toHaveValue("US");
  await expectMeasurementQuantities(readout, expectedInches, "in", "converted");
  await expect(readout.getByText(`Measure · pipe: ${pipe.id}`, { exact: true })).toBeVisible();
  await expect(page.getByTestId("command-selection-readout")).toContainText(`pipe: ${pipe.id}`);
  expect(await currentModelHashThroughVisibleExport(page)).toBe(hashBefore);
});

test("a first late diagnostics pull is RAF-neutral and projection refreshes after a real camera change", async ({ page }, testInfo) => {
  const model = await gotoRoutedFixture(page, "precision-origin-base.model.json", { captureGlobalRaf: true });
  // Bind visible model readiness outside the observed span. Projected target
  // visibility depends on the current camera and is not a model-load signal.
  await expect(typedTreeRow(page, "project", model.project.id)).toBeVisible();
  const settledBeforeLateRead = await waitForSettledGlobalRaf(page);
  expect(settledBeforeLateRead.status).toBe("PASS");
  expect(settledBeforeLateRead.pending).toBe(0);

  // All Playwright locator auto-waits finish before this interval. The page
  // uses bounded timers, not RAF, to observe whether the first diagnostics
  // pull perturbs a viewport that settled before it was ever observed.
  const lateRead = await page.evaluate(async (settleMs) => {
    const harness = (globalThis as any).__uifHarness;
    const counters = () => ({
      requested: harness.rafRequested as number,
      completed: harness.rafCompleted as number,
      cancelled: harness.rafCancelled as number,
      pending: harness.pendingRaf.size as number,
    });
    const api = globalThis.__openPipeStressUiDiagnosticsV1;
    const globalBefore = counters();
    const first = api.readCurrent();
    const globalAfterFirstRead = counters();
    await new Promise<void>((resolve) => setTimeout(resolve, settleMs));
    const globalBeforeSecondRead = counters();
    const second = api.readCurrent();
    const globalAfterSecondRead = counters();
    if ("status" in first.viewport || "status" in second.viewport) {
      throw new Error(`Committed viewport diagnostics were unavailable during the late-read witness: ${JSON.stringify({
        first: { model: first.model, viewport: first.viewport },
        second: { model: second.model, viewport: second.viewport },
      })}`);
    }
    return {
      globalBefore,
      globalAfterFirstRead,
      globalBeforeSecondRead,
      globalAfterSecondRead,
      first: {
        modelGeneration: first.model.generation,
        cameraSequence: first.viewport.camera.sequence,
        mainSubmissionSequence: first.viewport.mainRender.submissionSequence,
        ownedPendingRafCount: first.viewport.resources.ownedPendingRafCount,
      },
      second: {
        modelGeneration: second.model.generation,
        cameraSequence: second.viewport.camera.sequence,
        mainSubmissionSequence: second.viewport.mainRender.submissionSequence,
        ownedPendingRafCount: second.viewport.resources.ownedPendingRafCount,
      },
    };
  }, 120);

  expect(lateRead.first.modelGeneration).not.toBeNull();
  expect(lateRead.first.mainSubmissionSequence).toBeGreaterThan(0);
  expect(lateRead.first.ownedPendingRafCount).toBe(0);
  expect(lateRead.second.ownedPendingRafCount).toBe(0);
  expect(lateRead.second.ownedPendingRafCount - lateRead.first.ownedPendingRafCount).toBe(0);
  expect(lateRead.second.mainSubmissionSequence - lateRead.first.mainSubmissionSequence).toBe(0);
  expect(lateRead.globalAfterFirstRead.requested - lateRead.globalBefore.requested).toBe(0);
  expect(lateRead.globalBeforeSecondRead.requested - lateRead.globalBefore.requested).toBe(0);
  expect(lateRead.globalAfterSecondRead.requested - lateRead.globalBefore.requested).toBe(0);
  expect(lateRead.globalAfterSecondRead.completed - lateRead.globalBefore.completed).toBe(0);
  expect(lateRead.globalAfterSecondRead.cancelled - lateRead.globalBefore.cancelled).toBe(0);
  expect(lateRead.globalAfterSecondRead.pending).toBe(0);

  const front = page.getByRole("button", { name: "Front", exact: true });
  await activateWithKeyboard(page, front);
  await expect(front).toHaveAttribute("aria-pressed", "true");
  const settledAfterCameraChange = await waitForSettledGlobalRaf(page);
  expect(settledAfterCameraChange.status).toBe("PASS");
  expect(settledAfterCameraChange.pending).toBe(0);

  // No readCurrent call occurs between the real Front-camera change and the
  // old-pair projection request. projectAuthoredPoint must pull the current
  // camera itself, return that pair as stale evidence, then accept its retry.
  const projection = await page.evaluate(async ({ oldPair, authoredPoint, settleMs }) => {
    const harness = (globalThis as any).__uifHarness;
    const counters = () => ({
      requested: harness.rafRequested as number,
      completed: harness.rafCompleted as number,
      cancelled: harness.rafCancelled as number,
      pending: harness.pendingRaf.size as number,
    });
    const api = globalThis.__openPipeStressUiDiagnosticsV1;
    const globalBefore = counters();
    const stale = api.projectAuthoredPoint({ ...oldPair, authoredPoint });
    if (stale.status !== "stale") return { stale, retry: null, postRead: null, globalBefore, globalAfter: counters() };
    const retry = api.projectAuthoredPoint({ ...stale.current, authoredPoint });
    await new Promise<void>((resolve) => setTimeout(resolve, settleMs));
    const postRead = api.readCurrent();
    const globalAfter = counters();
    return {
      stale,
      retry,
      postRead: "status" in postRead.viewport ? null : {
        mainSubmissionSequence: postRead.viewport.mainRender.submissionSequence,
        ownedPendingRafCount: postRead.viewport.resources.ownedPendingRafCount,
      },
      globalBefore,
      globalAfter,
    };
  }, {
    oldPair: {
      modelGeneration: lateRead.second.modelGeneration!,
      cameraSequence: lateRead.second.cameraSequence,
    },
    authoredPoint: model.nodes[0].position,
    settleMs: 80,
  });

  expect(projection.stale).toMatchObject({
    status: "stale",
    requested: {
      modelGeneration: lateRead.second.modelGeneration,
      cameraSequence: lateRead.second.cameraSequence,
    },
  });
  if (projection.stale.status !== "stale") throw new Error("Expected an old camera sequence to be stale.");
  expect(projection.stale.current.modelGeneration).toBe(lateRead.second.modelGeneration);
  expect(projection.stale.current.cameraSequence).toBeGreaterThan(lateRead.second.cameraSequence);
  expect(projection.retry).toMatchObject({
    status: "available",
    modelGeneration: projection.stale.current.modelGeneration,
    cameraSequence: projection.stale.current.cameraSequence,
  });
  expect(projection.postRead?.ownedPendingRafCount).toBe(0);
  expect(projection.globalAfter.requested - projection.globalBefore.requested).toBe(0);
  expect(projection.globalAfter.completed - projection.globalBefore.completed).toBe(0);
  expect(projection.globalAfter.cancelled - projection.globalBefore.cancelled).toBe(0);
  expect(projection.globalAfter.pending).toBe(0);

  await testInfo.attach("late-diagnostics-raf-and-camera-projection", {
    body: JSON.stringify({
      settledBeforeLateRead,
      firstLateRead: {
        ...lateRead,
        deltas: {
          productOwnedPending: lateRead.second.ownedPendingRafCount - lateRead.first.ownedPendingRafCount,
          mainSubmission: lateRead.second.mainSubmissionSequence - lateRead.first.mainSubmissionSequence,
          globalRequested: lateRead.globalAfterSecondRead.requested - lateRead.globalBefore.requested,
          globalCompleted: lateRead.globalAfterSecondRead.completed - lateRead.globalBefore.completed,
          globalCancelled: lateRead.globalAfterSecondRead.cancelled - lateRead.globalBefore.cancelled,
        },
      },
      settledAfterCameraChange,
      projection: {
        ...projection,
        globalDeltas: {
          requested: projection.globalAfter.requested - projection.globalBefore.requested,
          completed: projection.globalAfter.completed - projection.globalBefore.completed,
          cancelled: projection.globalAfter.cancelled - projection.globalBefore.cancelled,
        },
      },
    }, null, 2),
    contentType: "application/json",
  });
});

test("typed raw-ID collisions remain distinct through ordered selection and an ABA membership change", async ({ page }) => {
  const fixture = await readFixture("precision-origin-base.model.json");
  const sharedId = "same/id?typed";
  await gotoModel(page, withTypedCollision(fixture.model, sharedId));
  await ensureRail(page, "tree", true);
  await page.getByTestId("model-tree-filter-input").fill(sharedId);

  const node = typedTreeRow(page, "node", sharedId);
  const pipe = typedTreeRow(page, "pipe", sharedId);
  await expect(node).toBeVisible();
  await expect(pipe).toBeVisible();
  expect(await node.getAttribute("data-testid")).not.toBe(await pipe.getAttribute("data-testid"));

  await node.click();
  await expect(node).toHaveAttribute("aria-selected", "true");
  await pipe.click({ modifiers: ["Shift"] });
  await expect(node).toHaveAttribute("aria-selected", "true");
  await expect(pipe).toHaveAttribute("aria-selected", "true");
  await pipe.click({ modifiers: [process.platform === "darwin" ? "Meta" : "Control"] });
  await expect(pipe).toHaveAttribute("aria-selected", "false");
  await expect(node).toHaveAttribute("aria-selected", "true");
  await expect(page.getByTestId("command-selection-readout")).toContainText(`node: ${sharedId}`);
});

for (const [fixtureName, scale] of [
  ["ui-foundation-1000.model.json", "1k"],
  ["ui-foundation-10000.model.json", "10k"],
] as const) {
  test(`Project pointer click followed by Shift-node preserves the displayed entity range and excludes Project on the ${scale} fixture`, async ({ page }) => {
    const model = await gotoRoutedFixture(page, fixtureName);
    await ensureRail(page, "tree", true);
    const tree = page.getByRole("tree", { name: "Model" });
    const project = typedTreeRow(page, "project", model.project.id);
    const node = typedTreeRow(page, "node", model.nodes[0].id);
    await expect(project).toBeVisible();
    await expect(node).toBeVisible();

    // buildTree's accepted displayed type order places Materials and Sections
    // before Nodes. Freeze the exact fixture-derived entity slice from Project
    // to the first node; group rows are navigation nodes, not range members.
    const expectedRange = [
      ...model.materials.map((entry: any) => ({ type: "material", id: entry.id })),
      ...model.sections.map((entry: any) => ({ type: "section", id: entry.id })),
      { type: "node", id: model.nodes[0].id },
    ];
    const expectedTestIds = expectedRange.map((ref) =>
      `tree-row-${encodeURIComponent(ref.type)}-${encodeURIComponent(ref.id)}`,
    );

    // This is intentionally a direct hit on each rendered row. Filtering,
    // keyboard activation, force clicks, and DOM event dispatch would bypass
    // the pointer target and range behavior being qualified here.
    await project.click();
    await expect(project).toHaveAttribute("aria-selected", "true");
    await node.click({ modifiers: ["Shift"] });
    await expect(project).toHaveAttribute("aria-selected", "false");
    for (const ref of expectedRange) {
      await expect(typedTreeRow(page, ref.type, ref.id)).toHaveAttribute("aria-selected", "true");
    }
    const selectedTestIds = await tree.locator('[role="treeitem"][aria-selected="true"]').evaluateAll((rows) =>
      rows.map((row) => row.getAttribute("data-testid")),
    );
    expect(selectedTestIds).toEqual(expectedTestIds);
    await expect(page.getByTestId("command-selection-readout")).toContainText(`node: ${model.nodes[0].id}`);
  });
}

test("virtual multiselect tree exposes full-list semantics while mounting a bounded off-window focus", async ({ page }) => {
  const model = await gotoRoutedFixture(page, "ui-foundation-1000.model.json");
  await ensureRail(page, "tree", true);
  const tree = page.getByRole("tree", { name: "Model" });
  await expect(tree).toHaveAttribute("aria-multiselectable", "true");

  const target = model.nodes[999];
  await page.getByTestId("model-tree-filter-input").fill(target.id);
  const filteredTarget = typedTreeRow(page, "node", target.id);
  await expect(filteredTarget).toHaveAttribute("aria-level", "2");
  await expect(filteredTarget).toHaveAttribute("aria-posinset", "1");
  await expect(filteredTarget).toHaveAttribute("aria-setsize", "1");
  await filteredTarget.click();
  await clearTreeFilter(page);
  await expect(filteredTarget).toHaveAttribute("aria-posinset", "1000");
  await expect(filteredTarget).toHaveAttribute("aria-setsize", "1001");

  await tree.focus();
  await expect(tree).toBeFocused();
  await expect(tree).toHaveAttribute("aria-activedescendant", `tree-row-node-${encodeURIComponent(target.id)}`);
  await expect(filteredTarget).toBeVisible();
  expect(await tree.locator("[data-virtual-index]").count()).toBeLessThan(60);

  await page.keyboard.press("ArrowUp");
  await page.keyboard.press("Space");
  const previous = model.nodes[998];
  await expect(typedTreeRow(page, "node", previous.id)).toHaveAttribute("aria-selected", "true");
  await page.keyboard.press("ArrowDown");
  await page.keyboard.press("Shift+Space");
  await expect(typedTreeRow(page, "node", target.id)).toHaveAttribute("aria-selected", "true");
  await page.keyboard.press(`${process.platform === "darwin" ? "Meta" : "Control"}+Space`);
  await expect(typedTreeRow(page, "node", target.id)).toHaveAttribute("aria-selected", "false");
  await page.keyboard.press("ArrowLeft");
  await expect(tree).toHaveAttribute("aria-activedescendant", "tree-group-Nodes");
  await page.keyboard.press("ArrowLeft");
  await expect(page.getByTestId("tree-group-Nodes")).toHaveAttribute("aria-expanded", "false");
  await page.keyboard.press("ArrowRight");
  await expect(page.getByTestId("tree-group-Nodes")).toHaveAttribute("aria-expanded", "true");
});

test("Grid drafts survive Tree round-trips and a filtered queue clears only its visible draft", async ({ page }) => {
  const model = await gotoRoutedFixture(page);
  const [visibleNode, retainedNode] = model.nodes;
  const visibleBase = String(visibleNode.position.x);
  const retainedBase = String(retainedNode.position.x);
  const visibleDraft = String(Number(visibleBase) + 0.125);
  const retainedDraft = String(Number(retainedBase) + 0.25);
  expect(visibleDraft).not.toBe(visibleBase);
  expect(retainedDraft).not.toBe(retainedBase);

  await ensureRail(page, "tree", true);
  await activateWithKeyboard(page, page.getByTestId("layout-mode-grid"));
  await activateWithKeyboard(page, page.getByTestId("entity-grid-type-nodes"));
  const visibleInput = page.getByTestId(`entity-grid-input-${visibleNode.id}-x`);
  const retainedInput = page.getByTestId(`entity-grid-input-${retainedNode.id}-x`);
  await visibleInput.fill(visibleDraft);
  await retainedInput.fill(retainedDraft);
  await expect(page.getByTestId("entity-grid-change-count")).toContainText("2 changed cells");

  await activateWithKeyboard(page, page.getByTestId("layout-mode-tree"));
  await expect(page.getByRole("tree", { name: "Model" })).toBeVisible();
  await activateWithKeyboard(page, page.getByTestId("layout-mode-grid"));
  await expect(visibleInput).toHaveValue(visibleDraft);
  await expect(retainedInput).toHaveValue(retainedDraft);
  await expect(page.getByTestId("entity-grid-change-count")).toContainText("2 changed cells");

  const filter = page.getByTestId("model-tree-filter-input");
  await filter.fill(visibleNode.id);
  await expect(page.getByTestId("entity-grid-summary")).toContainText("1 of");
  await expect(page.getByTestId("entity-grid-change-count")).toContainText("1 changed cells");
  await expect(retainedInput).toHaveCount(0);
  await activateWithKeyboard(page, page.getByTestId("queue-entity-grid-intents"));
  const queued = page.getByTestId("operation-apply-row-editor-intent-1");
  await expect(queued).toContainText(`Node ${visibleNode.id}`);
  await expect(queued).toContainText("position.x");
  await expect(queued).not.toContainText(retainedNode.id);

  await filter.fill("");
  await expect(visibleInput).toHaveValue(visibleBase);
  await expect(retainedInput).toHaveValue(retainedDraft);
  await expect(page.getByTestId("entity-grid-change-count")).toContainText("1 changed cells");
});

test("real box gestures obey the visible Nodes and Pipes filters without replacing orbit gestures", async ({ page }, testInfo) => {
  await gotoRoutedFixture(page);
  const canvas = page.getByTestId("viewport-canvas").locator("canvas");
  await activateWithKeyboard(page, page.getByTestId("viewport-fit-model"));
  await activateWithKeyboard(page, page.getByTestId("viewport-box-select"));
  await page.getByTestId("viewport-selection-filter").selectOption("nodes");
  const box = await canvas.boundingBox();
  expect(box).not.toBeNull();
  const allProjectedTargets = await page.getByTestId("viewport-selection-layer").locator(".viewport-select-target.node, .viewport-select-target.pipe").evaluateAll((targets) =>
    targets.map((target) => {
      const rect = target.getBoundingClientRect();
      return { x: rect.x + rect.width / 2, y: rect.y + rect.height / 2 };
    }),
  );
  const projectedTargets = allProjectedTargets.filter((target) =>
    target.x >= box!.x && target.x <= box!.x + box!.width && target.y >= box!.y && target.y <= box!.y + box!.height,
  );
  expect(projectedTargets.length).toBeGreaterThan(0);
  const gestureRect = {
    left: Math.max(box!.x + 12, Math.min(...projectedTargets.map((target) => target.x)) - 40),
    top: Math.max(box!.y + 12, Math.min(...projectedTargets.map((target) => target.y)) - 40),
    right: Math.min(box!.x + box!.width - 12, Math.max(...projectedTargets.map((target) => target.x)) + 40),
    bottom: Math.min(box!.y + box!.height - 12, Math.max(...projectedTargets.map((target) => target.y)) + 40),
  };
  for (const target of projectedTargets) {
    expect(target.x).toBeGreaterThanOrEqual(gestureRect.left);
    expect(target.x).toBeLessThanOrEqual(gestureRect.right);
    expect(target.y).toBeGreaterThanOrEqual(gestureRect.top);
    expect(target.y).toBeLessThanOrEqual(gestureRect.bottom);
  }
  const endpointTargets = await page.evaluate(({ topLeft, bottomRight }) => {
    const describe = (point: { x: number; y: number }) => {
      const element = document.elementFromPoint(point.x, point.y);
      return {
        className: element instanceof HTMLElement ? element.className : null,
        isCanvas: element instanceof HTMLCanvasElement,
        tagName: element?.tagName ?? null,
        testId: element instanceof HTMLElement ? element.dataset.testid ?? null : null,
      };
    };
    return { topLeft: describe(topLeft), bottomRight: describe(bottomRight) };
  }, {
    topLeft: { x: gestureRect.left, y: gestureRect.top },
    bottomRight: { x: gestureRect.right, y: gestureRect.bottom },
  });
  expect(endpointTargets.topLeft.isCanvas).toBe(true);
  expect(endpointTargets.bottomRight.isCanvas).toBe(true);
  await page.mouse.move(gestureRect.left, gestureRect.top);
  await page.mouse.down();
  await page.mouse.move(gestureRect.right, gestureRect.bottom, { steps: 6 });
  await page.mouse.up();
  await expect(page.getByTestId("command-selection-readout")).toContainText("Selected node:");
  const nodePublication = await page.evaluate(() => globalThis.__openPipeStressUiDiagnosticsV1.readCurrent().viewport);

  await page.getByTestId("viewport-selection-filter").selectOption("pipes");
  await page.mouse.move(gestureRect.right, gestureRect.bottom);
  await page.mouse.down();
  await page.mouse.move(gestureRect.left, gestureRect.top, { steps: 6 });
  await page.mouse.up();
  const pipePublication = await page.evaluate(() => globalThis.__openPipeStressUiDiagnosticsV1.readCurrent().viewport);
  await testInfo.attach("box-gesture-real-canvas-publications", {
    body: JSON.stringify({
      canvasBoundingBox: box,
      elementFromPoint: endpointTargets,
      allProjectedTargetCount: allProjectedTargets.length,
      projectedTargetCount: projectedTargets.length,
      frozenGestureRect: gestureRect,
      nodeGesture: {
        clientPath: { from: { x: gestureRect.left, y: gestureRect.top }, to: { x: gestureRect.right, y: gestureRect.bottom } },
        publication: "box" in nodePublication ? nodePublication.box : nodePublication,
      },
      pipeGesture: {
        clientPath: { from: { x: gestureRect.right, y: gestureRect.bottom }, to: { x: gestureRect.left, y: gestureRect.top } },
        publication: "box" in pipePublication ? pipePublication.box : pipePublication,
      },
      selectionReadout: await page.getByTestId("command-selection-readout").innerText(),
    }, null, 2),
    contentType: "application/json",
  });
  await expect(page.getByTestId("command-selection-readout")).toContainText("Selected pipe:");
  await activateWithKeyboard(page, page.getByTestId("viewport-box-select"));
  await expect(page.getByTestId("viewport-box-select")).toHaveAttribute("aria-pressed", "false");
  await expect(page.getByTestId("viewport-orbit-hint")).toContainText("Drag to orbit");
});

test("Box Select keeps the camera and authored projection invariant for plain, Shift, and Control drags", async ({ page }, testInfo) => {
  const model = await gotoRoutedFixture(page);
  const canvas = page.getByTestId("viewport-canvas").locator("canvas");
  const boxSelect = page.getByTestId("viewport-box-select");
  await expect(boxSelect).toHaveAttribute("aria-pressed", "false");
  const authoredPoints = model.nodes.slice(0, 3).map((node: any) => node.position);
  // Presets fit authored bounds; Fit Model fits displayed entity bounds.
  // Freeze each command's own exact endpoint rather than equating the two.
  await activateWithKeyboard(page, page.getByRole("button", { name: "Front", exact: true }));
  await activateWithKeyboard(page, page.getByTestId("viewport-view-isometric"));
  const firstIsometric = await settledCameraProjectionWitness(page, authoredPoints);
  await activateWithKeyboard(page, page.getByTestId("viewport-fit-model"));

  const ordinaryBefore = await settledCameraProjectionWitness(page, authoredPoints);
  const ordinaryPath = await visibleCanvasDragPath(canvas);
  await realCanvasDrag(page, ordinaryPath);

  // Take ownership while the real OrbitControls damping tail is still live.
  // This deliberately does not call the settling helper between pointer-up
  // and arming Box Select.
  const activeInertia = await readCameraProjectionWitness(page, authoredPoints);
  expect(activeInertia.ownedPendingRafCount, "ordinary orbit leaves a product-owned damping frame pending").toBeGreaterThan(0);
  const selectionReadoutBeforeTakeover = await page.getByTestId("command-selection-readout").innerText();
  await activateWithKeyboard(page, boxSelect);
  await expect(boxSelect).toHaveAttribute("aria-pressed", "true");
  const ordinaryAfter = await settledCameraProjectionWitness(page, authoredPoints);
  expect(ordinaryAfter.camera.sequence).toBeGreaterThan(ordinaryBefore.camera.sequence);
  expect(ordinaryAfter.camera.position).not.toEqual(ordinaryBefore.camera.position);
  expect(ordinaryAfter.projections.map(projectionGeometry)).not.toEqual(ordinaryBefore.projections.map(projectionGeometry));

  // The fixture has no components. A real Shift-drag therefore exercises an
  // empty additive Box gesture without changing the selection readout. It
  // must also be presentation-only once Box has cancelled the inertia tail.
  await page.getByTestId("viewport-selection-filter").selectOption("components");
  const takeoverBefore = await settledCameraProjectionWitness(page, authoredPoints);
  const takeoverPath = await visibleCanvasDragPath(canvas);
  await page.keyboard.down("Shift");
  try {
    await page.mouse.move(takeoverPath.start.x, takeoverPath.start.y);
    await page.mouse.down({ button: "left" });
    await page.mouse.move(takeoverPath.end.x, takeoverPath.end.y, { steps: 6 });
    await expect(page.locator(".viewport-box-rect")).toBeVisible();
    await page.mouse.up({ button: "left" });
  } finally {
    await page.keyboard.up("Shift");
  }
  await expect(page.locator(".viewport-box-rect")).toHaveCount(0);
  const takeoverAfter = await settledCameraProjectionWitness(page, authoredPoints);
  expect(takeoverAfter.camera, "active-inertia Box takeover camera").toEqual(takeoverBefore.camera);
  expect(takeoverAfter.projections, "active-inertia Box takeover authored projection").toEqual(takeoverBefore.projections);
  expect(takeoverAfter.canvasRect, "active-inertia Box takeover published canvas rectangle").toEqual(takeoverBefore.canvasRect);
  expect(takeoverAfter.canvasDomRect, "active-inertia Box takeover DOM canvas rectangle").toEqual(takeoverBefore.canvasDomRect);
  expect(takeoverAfter.ownedPendingRafCount).toBe(0);
  expect(await page.getByTestId("command-selection-readout").innerText()).toBe(selectionReadoutBeforeTakeover);
  await activateWithKeyboard(page, boxSelect);
  await expect(boxSelect).toHaveAttribute("aria-pressed", "false");
  await page.getByTestId("viewport-selection-filter").selectOption("all");

  await activateWithKeyboard(page, page.getByTestId("viewport-fit-model"));
  const firstFit = await settledCameraProjectionWitness(page, authoredPoints);
  const selectionReadoutAfterFit = await page.getByTestId("command-selection-readout").innerText();

  // Real orbit and right-pan gestures must move the view. Fit and preset
  // commands must then reach deterministic exact endpoints with no owned RAF.
  const orbitBeforeFit = await settledCameraProjectionWitness(page, authoredPoints);
  await realCanvasDrag(page, await visibleCanvasDragPath(canvas));
  const orbitMoved = await settledCameraProjectionWitness(page, authoredPoints);
  expect(cameraPose(orbitMoved.camera)).not.toEqual(cameraPose(orbitBeforeFit.camera));
  expect(orbitMoved.projections.map(projectionGeometry)).not.toEqual(orbitBeforeFit.projections.map(projectionGeometry));
  await activateWithKeyboard(page, page.getByTestId("viewport-fit-model"));
  const secondFit = await settledCameraProjectionWitness(page, authoredPoints);
  expectStableViewportEndpoint(secondFit, firstFit, "orbit to Fit Model");
  expect(await page.getByTestId("command-selection-readout").innerText()).toBe(selectionReadoutAfterFit);

  const beforeRightPan = await settledCameraProjectionWitness(page, authoredPoints);
  await realCanvasDrag(page, await visibleCanvasDragPath(canvas), "right");
  const rightPanMoved = await settledCameraProjectionWitness(page, authoredPoints);
  expect(cameraPose(rightPanMoved.camera)).not.toEqual(cameraPose(beforeRightPan.camera));
  expect(rightPanMoved.projections.map(projectionGeometry)).not.toEqual(beforeRightPan.projections.map(projectionGeometry));
  const front = page.getByRole("button", { name: "Front", exact: true });
  await activateWithKeyboard(page, front);
  await expect(front).toHaveAttribute("aria-pressed", "true");
  const frontEndpoint = await settledCameraProjectionWitness(page, authoredPoints);
  expect(cameraPose(frontEndpoint.camera)).not.toEqual(cameraPose(rightPanMoved.camera));
  expect(frontEndpoint.ownedPendingRafCount).toBe(0);
  expect(await page.getByTestId("command-selection-readout").innerText()).toBe(selectionReadoutAfterFit);

  await realCanvasDrag(page, await visibleCanvasDragPath(canvas));
  await settledCameraProjectionWitness(page, authoredPoints);
  const isometric = page.getByTestId("viewport-view-isometric");
  await activateWithKeyboard(page, isometric);
  await expect(isometric).toHaveAttribute("aria-pressed", "true");
  const isometricEndpoint = await settledCameraProjectionWitness(page, authoredPoints);
  expectStableViewportEndpoint(isometricEndpoint, firstIsometric, "orbit to Isometric preset");
  expect(await page.getByTestId("command-selection-readout").innerText()).toBe(selectionReadoutAfterFit);

  await expect(page.getByTestId("toggle-viewport-labels")).toHaveAttribute("aria-pressed", "true");
  const labelSample = await page.getByTestId("viewport-selection-layer").locator(".viewport-select-target").evaluateAll((targets) => {
    const mainCanvas = document.querySelector<HTMLCanvasElement>('[data-testid="viewport-canvas"] canvas');
    const canvasRect = mainCanvas?.getBoundingClientRect();
    if (!mainCanvas || !canvasRect) return null;
    for (const target of targets) {
      if (!(target instanceof HTMLElement)) continue;
      const rect = target.getBoundingClientRect();
      const center = { x: rect.left + rect.width / 2, y: rect.top + rect.height / 2 };
      if (center.x < canvasRect.left || center.x > canvasRect.right || center.y < canvasRect.top || center.y > canvasRect.bottom) continue;
      const hit = document.elementFromPoint(center.x, center.y);
      if (hit?.closest(".viewport-select-target") === target && target.dataset.testid) {
        return {
          ariaLabel: target.getAttribute("aria-label"),
          center,
          testId: target.dataset.testid,
        };
      }
    }
    return null;
  });
  expect(labelSample, "an ordinary visible viewport label has a real pointer-hit center").not.toBeNull();
  const sampledLabel = page.getByTestId(labelSample!.testId);
  const ordinaryLabelHit = await describeViewportHit(page, labelSample!.center);
  expect(ordinaryLabelHit.closestSelectionTargetTestId).toBe(labelSample!.testId);
  expect(ordinaryLabelHit.isMainCanvas).toBe(false);
  await page.mouse.click(labelSample!.center.x, labelSample!.center.y);
  await expect(sampledLabel).toHaveAttribute("aria-pressed", "true");

  // Freeze the label center and a separated canvas endpoint before Box mode.
  // The start coordinate is intentionally never resampled after Box is armed.
  const supportingPath = await visibleCanvasDragPath(canvas);
  const distanceTo = (point: Readonly<{ x: number; y: number }>) =>
    Math.hypot(point.x - labelSample!.center.x, point.y - labelSample!.center.y);
  const labelBoxPath = {
    start: labelSample!.center,
    end: distanceTo(supportingPath.start) >= distanceTo(supportingPath.end) ? supportingPath.start : supportingPath.end,
  };
  expect(distanceTo(labelBoxPath.end)).toBeGreaterThan(40);

  await activateWithKeyboard(page, boxSelect);
  await expect(boxSelect).toHaveAttribute("aria-pressed", "true");
  const boxLabelHit = await describeViewportHit(page, labelSample!.center);
  expect(boxLabelHit.isMainCanvas).toBe(true);
  expect(boxLabelHit.closestSelectionTargetTestId).toBeNull();

  const boxGestures = [];
  for (const modifier of [null, "Shift", "Control"] as const) {
    const before = await settledCameraProjectionWitness(page, authoredPoints);
    const path = modifier === null ? labelBoxPath : await visibleCanvasDragPath(canvas);
    if (modifier) await page.keyboard.down(modifier);
    try {
      await page.mouse.move(path.start.x, path.start.y);
      await page.mouse.down({ button: "left" });
      await page.mouse.move((path.start.x + path.end.x) / 2, (path.start.y + path.end.y) / 2, { steps: 3 });
      await expect(page.locator(".viewport-box-rect")).toBeVisible();
      await page.mouse.move(path.end.x, path.end.y, { steps: 3 });
      await page.mouse.up({ button: "left" });
    } finally {
      if (modifier) await page.keyboard.up(modifier);
    }
    await expect(page.locator(".viewport-box-rect")).toHaveCount(0);
    const after = await settledCameraProjectionWitness(page, authoredPoints);
    expect(after.camera, `${modifier ?? "plain"} Box Select camera`).toEqual(before.camera);
    expect(after.projections, `${modifier ?? "plain"} Box Select authored projection`).toEqual(before.projections);
    expect(after.canvasRect, `${modifier ?? "plain"} Box Select published canvas rectangle`).toEqual(before.canvasRect);
    expect(after.canvasDomRect, `${modifier ?? "plain"} Box Select DOM canvas rectangle`).toEqual(before.canvasDomRect);
    expect(after.ownedPendingRafCount, `${modifier ?? "plain"} Box Select pending product RAF`).toBe(0);
    boxGestures.push({ modifier: modifier ?? "plain", path, before, after });
  }

  await testInfo.attach("box-select-camera-projection-invariance", {
    body: JSON.stringify({
      ordinaryOrbit: { path: ordinaryPath, before: ordinaryBefore, after: ordinaryAfter },
      activeInertiaTakeover: { immediate: activeInertia, before: takeoverBefore, after: takeoverAfter },
      navigationEndpoints: { firstIsometric, firstFit, orbitMoved, secondFit, rightPanMoved, frontEndpoint, isometricEndpoint },
      frozenLabelInterception: {
        sample: labelSample,
        ordinaryModeHit: ordinaryLabelHit,
        boxModeHit: boxLabelHit,
        boxDragPath: labelBoxPath,
      },
      boxGestures,
    }, null, 2),
    contentType: "application/json",
  });
});

// Frozen regression floor: simultaneous authoring and analysis must retain a
// 200 × 200 CSS-pixel canvas, with controls and drafts still usable.
for (const theme of APPEARANCE_THEMES) {
  for (const density of APPEARANCE_DENSITIES) {
    for (const viewport of APPEARANCE_VIEWPORTS) {
      test(`task and analysis dock preserve usable canvas ${theme} ${density} ${viewport.width}x${viewport.height}`, async ({ page }, testInfo) => {
        await page.setViewportSize(viewport);
        await page.goto("/");
        await page.getByLabel("Appearance theme").selectOption(theme);
        await page.getByLabel("Workspace density").selectOption(density);
        await startPropertyTaskFromTreeEntity(page, "node", "node:N-100");
        const inspector = page.getByTestId("property-inspector");
        await inspector.getByTestId("editor-intent-field").selectOption("label");
        await inspector.getByTestId("editor-intent-value").fill("Retained inspector task");
        if (viewport.width < 1280) await ensureRail(page, "inspector", false);
        await activateWithKeyboard(page, page.getByTestId("command-pipe"));
        const pipeForm = page.getByTestId("viewport-editor-intents");
        await expect(pipeForm).toHaveClass(/active/);
        await page.getByTestId("viewport-create-pipe-label").fill("Retained pipe draft");
        await page.getByTestId("viewport-create-pipe-provenance").fill("layout regression draft");
        await openWorkspaceSection(page, "solve");
        const dock = page.getByTestId("workspace-dock");
        await expect(dock).not.toHaveClass(/collapsed/);
        await expect(page.getByTestId("workspace-section-solve")).toBeVisible();

        const evidence: unknown[] = [];
        const measure = async (phase: string) => {
          await expect(pipeForm).toHaveClass(/active/);
          await expect(dock).not.toHaveClass(/collapsed/);
          const geometry = await page.evaluate(() => {
            const rect = (selector: string) => {
              const element = document.querySelector<HTMLElement>(selector)!;
              const box = element.getBoundingClientRect();
              return { x: box.x, y: box.y, width: box.width, height: box.height, right: box.right, bottom: box.bottom };
            };
            const body = document.querySelector<HTMLElement>(".workspace-dock-body")!;
            const bodyBox = body.getBoundingClientRect();
            const bodyClip = { top: Math.max(0, bodyBox.top + body.clientTop),
              bottom: Math.min(innerHeight, bodyBox.top + body.clientTop + body.clientHeight) };
            for (let ancestor = body.parentElement; ancestor; ancestor = ancestor.parentElement) {
              if (!/(auto|scroll|hidden|clip)/.test(getComputedStyle(ancestor).overflowY)) continue;
              const box = ancestor.getBoundingClientRect();
              bodyClip.top = Math.max(bodyClip.top, box.top + ancestor.clientTop);
              bodyClip.bottom = Math.min(bodyClip.bottom, box.top + ancestor.clientTop + ancestor.clientHeight);
            }
            return {
              dockBody: { ...rect(".workspace-dock-body"), clientHeight: body.clientHeight,
                scrollHeight: body.scrollHeight, scrollTop: body.scrollTop, clip: bodyClip,
                usableHeight: Math.max(0, bodyClip.bottom - bodyClip.top) },
              canvas: rect(".viewport-canvas canvas"), toolbar: rect(".viewport-toolbar"),
              task: rect('[data-testid="viewport-editor-intents"]'), dock: rect(".workspace-dock"),
              status: rect('[data-testid="workspace-status-bar"]'),
              bodyOverflowX: document.documentElement.scrollWidth - innerWidth,
              bodyOverflowY: document.documentElement.scrollHeight - innerHeight,
            };
          });
          evidence.push({ phase, geometry });
          expect(geometry.canvas.width, `${phase} canvas width`).toBeGreaterThanOrEqual(200);
          expect(geometry.canvas.height, `${phase} canvas height`).toBeGreaterThanOrEqual(200);
          expect(geometry.canvas.x).toBeGreaterThanOrEqual(0);
          expect(geometry.canvas.y).toBeGreaterThanOrEqual(0);
          expect(geometry.canvas.right).toBeLessThanOrEqual(viewport.width);
          expect(geometry.canvas.bottom).toBeLessThanOrEqual(geometry.dock.y);
          expect(geometry.task.bottom).toBeLessThanOrEqual(geometry.dock.y);
          expect(geometry.dock.bottom).toBeLessThanOrEqual(geometry.status.y);
          expect(geometry.bodyOverflowX).toBeLessThanOrEqual(0);
          expect(geometry.bodyOverflowY).toBeLessThanOrEqual(0);
          if (phase.startsWith("solved")) {
            expect(geometry.dockBody.usableHeight, `${phase} usable dock body height`).toBeGreaterThan(64);
          }
          return geometry.canvas;
        };
        const beforeSelection = await measure("before solve");
        const selected = await selectTreeRow(page, "node", "node:N-110");
        await expect(selected).toHaveAttribute("aria-selected", "true");
        if (viewport.width < 1280) {
          // Narrow rail navigation normally dismisses the dock. Restore the
          // same simultaneous task/dock layout before comparing rectangles.
          await ensureRail(page, "tree", false);
          await openWorkspaceSection(page, "solve");
        }
        await expect(page.locator(".viewport-toolbar-selection-status")).toHaveText("Selected: node:N-110");
        expect(await measure("changed primary; task/dock restored")).toEqual(beforeSelection);
        await ensureRail(page, "inspector", true);
        await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText("node: node:N-100");
        await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("Retained inspector task");
        const footer = inspector.getByTestId("task-action-footer");
        for (const control of await footer.getByRole("button").all()) {
          await control.scrollIntoViewIfNeeded();
          await expectCenterUnobscured(control);
          if (await control.isEnabled()) {
            await control.focus();
            await expect(control).toBeFocused();
          }
        }
        if (viewport.width < 1280) {
          await ensureRail(page, "inspector", false);
          await openWorkspaceSection(page, "solve");
        } else {
          await expect(page.getByTestId("toggle-tree")).toHaveAttribute("aria-expanded", "true");
          await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "true");
        }
        expect(await measure("after footer navigation; task/dock restored")).toEqual(beforeSelection);
        const toolbar = page.getByRole("group", { name: "Viewport controls", exact: true });
        for (const control of await toolbar.locator("button, select, summary").all()) {
          await control.scrollIntoViewIfNeeded();
          await expectCenterUnobscured(control);
          if (await control.isEnabled()) {
            await control.focus();
            await expect(control).toBeFocused();
          }
        }
        await page.getByTestId("cancel-pipe-draft").scrollIntoViewIfNeeded();
        await page.getByTestId("cancel-pipe-draft").focus();
        await expect(page.getByTestId("cancel-pipe-draft")).toBeFocused();
        await expectCenterUnobscured(page.getByTestId("cancel-pipe-draft"));
        await page.getByTestId("queue-explicit-pipe-intent").scrollIntoViewIfNeeded();
        await expectCenterUnobscured(page.getByTestId("queue-explicit-pipe-intent"));
        await activateWithKeyboard(page, page.getByTestId("run-mechanics-preview"));
        await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
        await expect(page.getByTestId("solve-job-summary")).toContainText("result_rows=830");
        await measure("solved");
        const verifySolvedDockControls = async (phase: string) => {
          await expect(page.getByTestId("solve-job-summary")).toContainText("state=completed");
          for (const testId of ["solver-mode-sparse", "solver-mode-dense", "run-mechanics-preview"]) {
            const control = page.getByTestId(testId);
            await expect(control).toBeEnabled();
            await control.scrollIntoViewIfNeeded();
            await control.focus();
            await expect(control).toBeFocused();
            await expectCenterUnobscured(control);
            const readControlWitness = () => control.evaluate((element) => {
              const body = element.closest(".workspace-dock-body")!;
              const box = body.getBoundingClientRect();
              const target = element.getBoundingClientRect();
              const hit = document.elementFromPoint(target.x + target.width / 2, target.y + target.height / 2);
              return { target: target.toJSON(), body: box.toJSON(), scrollTop: body.scrollTop,
                clip: { top: box.top + body.clientTop, bottom: box.top + body.clientTop + body.clientHeight },
                focused: document.activeElement === element,
                hitInsideBody: hit !== null && body.contains(hit),
                owned: hit !== null && (hit === element || element.contains(hit)) };
            });
            const initial = await readControlWitness();
            let witness = initial;
            let wheel: { deltaY: number; dispatched: boolean; beforeScrollTop: number;
              afterScrollTop: number | null } | null = null;
            try {
              if (initial.target.top < initial.clip.top && initial.owned) {
                const center = { x: initial.target.x + initial.target.width / 2,
                  y: initial.target.y + initial.target.height / 2 };
                await page.mouse.move(center.x, center.y);
                const pointerOwned = await control.evaluate((element, point) => {
                  const hit = document.elementFromPoint(point.x, point.y);
                  return hit !== null && element.closest(".workspace-dock-body")!.contains(hit) &&
                    (hit === element || element.contains(hit));
                }, center);
                expect(pointerOwned).toBe(true);
                const beforeWheel = await readControlWitness();
                expect(beforeWheel.owned).toBe(true);
                expect(beforeWheel.hitInsideBody).toBe(true);
                expect(center.y).toBeGreaterThanOrEqual(beforeWheel.clip.top);
                expect(center.y).toBeLessThanOrEqual(beforeWheel.clip.bottom);
                wheel = { deltaY: -24, dispatched: false, beforeScrollTop: beforeWheel.scrollTop,
                  afterScrollTop: null };
                await page.mouse.wheel(0, wheel.deltaY);
                wheel.dispatched = true;
                await expect.poll(async () => (await readControlWitness()).scrollTop)
                  .toBeLessThan(beforeWheel.scrollTop);
              }
            } finally {
              witness = await readControlWitness();
              if (wheel) wheel.afterScrollTop = witness.scrollTop;
              const controlEvidence = { phase, testId, initial, wheel, final: witness,
                reachability: wheel?.dispatched ? "wheel-assisted" : "ordinary scroll and focus" };
              evidence.push(controlEvidence);
              await testInfo.attach(`${phase}-${testId}-reachability`, {
                body: JSON.stringify(controlEvidence, null, 2), contentType: "application/json",
              });
            }
            expect(witness.target.top).toBeGreaterThanOrEqual(witness.clip.top);
            expect(witness.target.bottom).toBeLessThanOrEqual(witness.clip.bottom);
            expect(witness.focused).toBe(true);
            expect(witness.owned).toBe(true);
          }
        };
        await verifySolvedDockControls("solved controls before proof disclosure");
        const proof = page.getByTestId("status-pill-solve-proof");
        const proofSummary = proof.locator("summary");
        await expect(proofSummary).toHaveText("Solve proof Run identity matches");
        await proofSummary.scrollIntoViewIfNeeded();
        await expectCenterUnobscured(proofSummary);
        await proofSummary.click();
        await expect(proof).toHaveAttribute("open", "");
        const rawProof = proof.locator("code");
        await expect(rawProof).toBeVisible();
        await rawProof.scrollIntoViewIfNeeded();
        await expectCenterUnobscured(rawProof);
        const rawProofText = await rawProof.innerText();
        expect(rawProofText).toMatch(/^seam=browser_fixture_no_backend_job; project=project:invented-loop-01; result_model=project:invented-loop-01; identity=match; rows=830; generation=\d+; job=[^;]+; model_sha256=sha256:[a-f0-9]{64}; input_manifest_sha256=[a-f0-9]{64}$/);
        evidence.push({ phase: "solved proof Details", summary: await proofSummary.innerText(),
          rawProof: rawProofText, bounds: await rawProof.boundingBox() });
        await proofSummary.click();
        await expect(proof).not.toHaveAttribute("open", "");
        await expect(rawProof).toBeHidden();
        await verifySolvedDockControls("solved controls after proof disclosure");
        await measure("solved after proof disclosure");
        const deformation = page.getByTestId("viewport-deformation-status");
        await deformation.locator("summary").click();
        await expect(deformation.getByTestId("viewport-deformation-summary")).toContainText("available; nodes=5");
        // Noninteractive wrapped inline text has disjoint fragments; its union
        // center can legitimately fall on a sibling. Check actual text fragments
        // inside the scroll clip, retaining center-hit checks for all controls.
        for (const [testId, expected] of [
          ["viewport-deformation-summary", "available; nodes=5"],
          ["viewport-deformation-boundary", "vector_direction=global_cartesian_displacement_components"],
        ] as const) {
          const text = deformation.getByTestId(testId);
          await expect(text).toContainText(expected);
          await text.scrollIntoViewIfNeeded();
          const witness = await text.evaluate((element) => {
            let clip = { left: 0, top: 0, right: innerWidth, bottom: innerHeight };
            const clips = [];
            for (let ancestor = element.parentElement; ancestor; ancestor = ancestor.parentElement) {
              const style = getComputedStyle(ancestor);
              const box = ancestor.getBoundingClientRect();
              const bounds = { left: box.left + ancestor.clientLeft, top: box.top + ancestor.clientTop,
                right: box.left + ancestor.clientLeft + ancestor.clientWidth,
                bottom: box.top + ancestor.clientTop + ancestor.clientHeight };
              if (/(auto|scroll|hidden|clip)/.test(style.overflowX)) {
                clip.left = Math.max(clip.left, bounds.left);
                clip.right = Math.min(clip.right, bounds.right);
              }
              if (/(auto|scroll|hidden|clip)/.test(style.overflowY)) {
                clip.top = Math.max(clip.top, bounds.top);
                clip.bottom = Math.min(clip.bottom, bounds.bottom);
              }
              clips.push({ tag: ancestor.tagName, className: ancestor.className,
                overflowX: style.overflowX, overflowY: style.overflowY, bounds });
            }
            const fragments = [];
            const walker = document.createTreeWalker(element, NodeFilter.SHOW_TEXT);
            for (let node = walker.nextNode(); node; node = walker.nextNode()) {
              if (!node.textContent?.trim()) continue;
              const range = document.createRange();
              range.selectNodeContents(node);
              for (const rect of range.getClientRects()) {
                if (rect.width <= 0 || rect.height <= 0) continue;
                const visible = { left: Math.max(rect.left, clip.left), top: Math.max(rect.top, clip.top),
                  right: Math.min(rect.right, clip.right), bottom: Math.min(rect.bottom, clip.bottom) };
                const fullLineHeightVisible = visible.bottom - visible.top >= rect.height - 0.5;
                const hasVisibleArea = visible.right > visible.left && visible.bottom > visible.top;
                const hit = hasVisibleArea ? document.elementFromPoint(
                  (visible.left + visible.right) / 2, (visible.top + visible.bottom) / 2,
                ) : null;
                fragments.push({ text: node.textContent, rect: rect.toJSON(), visible,
                  fullLineHeightVisible, hasVisibleArea,
                  owner: hit?.getAttribute("data-testid") ?? hit?.tagName ?? null,
                  owned: hit !== null && (hit === element || element.contains(hit)) });
              }
            }
            return { text: element.textContent, clip, clips, fragments };
          });
          evidence.push({ phase: "expanded deformation text", testId, witness });
          expect(witness.fragments.length, `${testId} has nonempty text fragments`).toBeGreaterThan(0);
          expect(witness.fragments.some((fragment) => fragment.hasVisibleArea &&
            fragment.fullLineHeightVisible && fragment.owned), `${testId} has readable owned text inside the scroll clip`).toBe(true);
        }
        await measure("expanded deformation details");
        await deformation.locator("summary").click();
        await expect(page.getByTestId("viewport-create-pipe-label")).toHaveValue("Retained pipe draft");
        await expect(page.getByTestId("viewport-create-pipe-provenance")).toHaveValue("layout regression draft");
        await expect(page.getByTestId("command-pipe")).toHaveAttribute("aria-pressed", "true");
        await measure("solved final task and dock");
        await testInfo.attach("task-dock-layout", { body: JSON.stringify(evidence, null, 2), contentType: "application/json" });
        await testInfo.attach("solved-task-dock", { body: await page.screenshot(), contentType: "image/png" });
      });
    }
  }
}

test("selection IDs and cardinality keep the viewport rectangle fixed at three workspace widths", async ({ page }, testInfo) => {
  const model = await gotoRoutedFixture(page, "ui-foundation-1000.model.json");
  const authoredPoints = model.nodes.slice(0, 3).map((node: any) => node.position);
  const node = model.nodes[0];
  const pipe = model.pipe_segments[0];
  const support = model.supports[0];
  const component = model.components[0];
  const widths = [1024, 1280, 1440] as const;
  const evidence = [];

  for (const width of widths) {
    await page.setViewportSize({ width, height: width === 1440 ? 920 : width === 1280 ? 800 : 768 });
    await ensureRail(page, "tree", true);
    await ensureRail(page, "inspector", false);
    await activateWithKeyboard(page, page.getByTestId("viewport-fit-model"));
    const baseline = await settledCameraProjectionWitness(page, authoredPoints);
    const paneState = await readPaneState(page);
    const states = [];

    for (const selected of [
      { type: "node", id: node.id },
      { type: "pipe", id: pipe.id },
      { type: "support", id: support.id },
      { type: "component", id: component.id },
    ]) {
      await selectTreeRow(page, selected.type, selected.id);
      const witness = await settledCameraProjectionWitness(page, authoredPoints);
      expectStableCanvasRectangle(witness, baseline, `${width}px ${selected.type} selection`);
      expect(await readPaneState(page)).toEqual(paneState);
      await expect(page.getByTestId("command-selection-readout")).toContainText(`${selected.type}: ${selected.id}`);
      const toolbarStatus = await readViewportToolbarSelectionStatus(page);
      expectToolbarSelectionStatus(toolbarStatus, selected.id, `${width}px ${selected.type} selection`);
      const selection = await readPublishedSelection(page);
      expect(selection.orderedRefs).toEqual([{ type: selected.type, id: selected.id }]);
      states.push({ name: selected.type, selection, toolbarStatus, witness });
    }

    // Toggle the current singleton off through its real tree row. The ordered
    // selection is empty even though the public primary fallback is Project.
    await selectTreeRow(page, "component", component.id, { toggle: true });
    const emptyWitness = await settledCameraProjectionWitness(page, authoredPoints);
    expectStableCanvasRectangle(emptyWitness, baseline, `${width}px empty ordered selection`);
    expect(await readPaneState(page)).toEqual(paneState);
    const emptySelection = await readPublishedSelection(page);
    expect(emptySelection.orderedRefs).toEqual([]);
    await expect(page.getByTestId("command-selection-readout")).toContainText(`project: ${model.project.id}`);
    const emptyToolbarStatus = await readViewportToolbarSelectionStatus(page);
    expectToolbarSelectionStatus(emptyToolbarStatus, model.project.id, `${width}px empty ordered selection`);
    states.push({ name: "empty", selection: emptySelection, toolbarStatus: emptyToolbarStatus, witness: emptyWitness });

    await selectTreeRow(page, "node", node.id);
    await selectTreeRow(page, "support", support.id, { toggle: true });
    const multiWitness = await settledCameraProjectionWitness(page, authoredPoints);
    expectStableCanvasRectangle(multiWitness, baseline, `${width}px multi-selection`);
    expect(await readPaneState(page)).toEqual(paneState);
    const multiSelection = await readPublishedSelection(page);
    expect(multiSelection.orderedRefs).toEqual([
      { type: "node", id: node.id },
      { type: "support", id: support.id },
    ]);
    expect(multiSelection.primaryRef).toEqual({ type: "support", id: support.id });
    await expect(page.getByTestId("command-selection-readout")).toContainText(`support: ${support.id}`);
    const multiToolbarStatus = await readViewportToolbarSelectionStatus(page);
    expectToolbarSelectionStatus(multiToolbarStatus, support.id, `${width}px multi-selection`);
    states.push({ name: "multi", selection: multiSelection, toolbarStatus: multiToolbarStatus, witness: multiWitness });
    evidence.push({ width, baseline, paneState, states });
  }

  await testInfo.attach("selection-status-stable-viewport-rectangle", {
    body: JSON.stringify(evidence, null, 2),
    contentType: "application/json",
  });
});

test("a property task keeps its typed target and entered value through filter, collapse, and selection ABA", async ({ page }) => {
  await page.setViewportSize({ width: 1024, height: 768 });
  const model = await gotoRoutedFixture(page);
  const first = model.nodes[3];
  const second = model.nodes[4];
  await selectTreeRow(page, "node", first.id);
  await ensureRail(page, "inspector", true);
  const inspector = page.getByTestId("property-inspector");
  const propertiesTab = inspector.getByRole("tab", { name: "Properties", exact: true });
  const taskTab = inspector.getByRole("tab", { name: "Task", exact: true });
  await expect(propertiesTab).toHaveAttribute("aria-selected", "true");
  await expect(inspector.locator("#inspector-properties-view")).toBeVisible();
  await expect(inspector.getByTestId("editor-intent-panel")).toBeHidden();
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toBeHidden();
  await activateWithKeyboard(page, taskTab);
  await expect(taskTab).toHaveAttribute("aria-selected", "true");
  await expect(inspector.locator("#inspector-properties-view")).toBeHidden();
  await expect(inspector.getByTestId("editor-intent-panel")).toBeVisible();
  const emptyTask = inspector.getByTestId("inspector-task-empty");
  const startTask = inspector.getByTestId("inspector-start-task");
  await expect(emptyTask).toContainText(`Current selection: node: ${first.id}`);
  await expect(startTask).toBeEnabled();
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toHaveCount(0);
  await expect(inspector.getByTestId("task-action-footer")).toHaveCount(0);
  await expect(inspector.getByTestId("cancel-editor-intent")).toHaveCount(0);
  await activateWithKeyboard(page, startTask);
  await inspector.getByTestId("editor-intent-value").fill("Frozen task value");
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText(`node: ${first.id}`);

  // Below 1280, either drawer may be opened from its exposed keyboard target,
  // but opening one closes the other without discarding the active draft.
  await ensureRail(page, "tree", true);
  await expect(page.getByTestId("toggle-inspector")).toHaveAttribute("aria-expanded", "false");
  await ensureRail(page, "inspector", true);
  await expect(page.getByTestId("toggle-tree")).toHaveAttribute("aria-expanded", "false");
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText(`node: ${first.id}`);
  await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("Frozen task value");

  await selectTreeRow(page, "node", second.id, { shift: true });
  await expect(inspector.getByTestId("queue-editor-intent")).toBeDisabled();
  await expect(inspector.getByTestId("validate-editor-intent-inline")).toBeDisabled();
  await expect(inspector.getByTestId("apply-editor-intent-inline")).toBeDisabled();
  await selectTreeRow(page, "node", second.id, { toggle: true });
  await page.getByTestId("model-tree-filter-input").fill("does-not-match-any-entity");
  await ensureRail(page, "tree", false);
  await ensureRail(page, "inspector", true);

  await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText(`node: ${first.id}`);
  await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("Frozen task value");
  await activateWithKeyboard(page, inspector.getByTestId("queue-editor-intent"));
  await expect(page.getByTestId("workspace-section-operations")).toBeVisible();
  await expect(page.getByTestId("workspace-dock-header")).toContainText("Review changes");
});

test("live Properties routes follow catalogue target B while frozen Task A remains unchanged", async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 920 });
  const model = await gotoRoutedFixture(page);
  const pipeA = model.pipe_segments[0];
  const pipeB = model.pipe_segments[1];
  const alternateForA = model.sections.find((section: any) => section.id !== pipeA.section_ref)!;
  const alternateForB = model.sections.find((section: any) => section.id !== pipeB.section_ref)!;
  expect(alternateForA).toBeDefined();
  expect(alternateForB).toBeDefined();

  const inspector = page.getByTestId("property-inspector");
  const propertiesTab = inspector.getByRole("tab", { name: "Properties", exact: true });
  const taskTab = inspector.getByRole("tab", { name: "Task", exact: true });
  const openSectionAssignment = async () => {
    await activateWithKeyboard(page, commandGroupControl(page, "Properties"));
    const palette = page.getByRole("dialog", { name: "Find a modeling tool" });
    await expect(palette).toBeVisible();
    await activateWithKeyboard(page, palette.getByTestId("toolkit-properties.assign-section"));
    await expect(propertiesTab).toHaveAttribute("aria-selected", "true");
    const panel = inspector.getByRole("region", { name: "Pipe section assignment" });
    await expect(panel).toBeVisible();
    await expect(panel).toBeFocused();
    return panel;
  };

  // A freshly selected pipe exposes its live rich Properties form before any
  // task has established a frozen mutation target.
  await selectTreeRow(page, "pipe", pipeA.id);
  await ensureRail(page, "inspector", true);
  const freshPanel = await openSectionAssignment();
  await expect(inspector.locator(":scope > h2")).toContainText(`pipe: ${pipeA.id}`);
  await expect(freshPanel).toContainText(`Shared section: ${pipeA.section_ref}`);
  await expect(freshPanel.getByRole("combobox", { name: "Shared section" })).toHaveValue("");
  await freshPanel.getByRole("combobox", { name: "Shared section" }).selectOption(alternateForA.id);
  await expect(freshPanel.getByRole("button", { name: "Queue section assignment", exact: true })).toBeEnabled();

  await activateWithKeyboard(page, taskTab);
  await activateWithKeyboard(page, inspector.getByTestId("inspector-start-task"));
  await inspector.getByTestId("editor-intent-value").fill("Frozen Task A entered value");
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText(`pipe: ${pipeA.id}`);

  await selectTreeRow(page, "pipe", pipeB.id);
  await ensureRail(page, "inspector", true);
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText(`pipe: ${pipeA.id}`);
  await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("Frozen Task A entered value");

  const liveBPanel = await openSectionAssignment();
  await expect(inspector.locator(":scope > h2")).toContainText(`pipe: ${pipeB.id}`);
  await expect(liveBPanel).toContainText(`Shared section: ${pipeB.section_ref}`);
  const liveSection = liveBPanel.getByRole("combobox", { name: "Shared section" });
  await expect(liveSection).toHaveValue("");
  await liveSection.selectOption(alternateForB.id);
  await activateWithKeyboard(page, liveBPanel.getByRole("button", { name: "Queue section assignment", exact: true }));

  const queued = page.getByTestId("operation-apply-row-editor-intent-1");
  await expect(queued).toBeVisible();
  await expect(queued).toContainText(`Element ${pipeB.id}`);
  await expect(queued).toContainText("section_ref:");
  await expect(queued).toContainText(`\"section_ref\":\"${pipeB.section_ref}\"`);
  await expect(queued).toContainText(`\"section_ref\":\"${alternateForB.id}\"`);
  await expect(queued).not.toContainText(`Element ${pipeA.id}`);

  await ensureRail(page, "inspector", true);
  await activateWithKeyboard(page, taskTab);
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toContainText(`pipe: ${pipeA.id}`);
  await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("Frozen Task A entered value");
});

test("an unqueued rich Properties draft survives aggregate inspection with mutation controls disabled", async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 920 });
  const model = await gotoRoutedFixture(page);
  const support = model.supports[0];
  const peer = model.nodes.find((node: any) => node.id !== support.node) ?? model.nodes[0];
  const enteredProvenance = "entered rich support draft retained across aggregate inspection";

  await selectTreeRow(page, "support", support.id);
  await ensureRail(page, "inspector", true);
  await activateWithKeyboard(page, commandGroupControl(page, "Supports"));
  const palette = page.getByRole("dialog", { name: "Find a modeling tool" });
  await expect(palette).toBeVisible();
  await activateWithKeyboard(page, palette.getByTestId("toolkit-supports.restraint"));

  const inspector = page.getByTestId("property-inspector");
  const form = inspector.locator("#rich-support-form");
  const provenance = form.locator('input[aria-label="Support provenance"]');
  const queue = form.locator("button").filter({ hasText: /^Queue support configuration$/ });
  await expect(form).toBeVisible();
  await expect(form).toBeFocused();
  await provenance.fill(enteredProvenance);
  await expect(provenance).toHaveValue(enteredProvenance);
  await expect(queue).toBeEnabled();

  await selectTreeRow(page, "node", peer.id, { toggle: true });
  const aggregate = inspector.getByTestId("aggregate-property-inspector");
  await expect(aggregate).toBeVisible();
  await expect(aggregate).toContainText("Aggregate inspection is read-only");
  await expect(aggregate.getByRole("button", { name: "Start property task", exact: true })).toBeDisabled();
  await expect(provenance).toBeDisabled();
  await expect(queue).toBeDisabled();
  await expect(provenance).toHaveValue(enteredProvenance);

  await selectTreeRow(page, "node", peer.id, { toggle: true });
  await expect(aggregate).toHaveCount(0);
  await expect(inspector.locator(":scope > h2")).toContainText(`support: ${support.id}`);
  await expect(form).toBeVisible();
  await expect(provenance).toBeEnabled();
  await expect(provenance).toHaveValue(enteredProvenance);
  await expect(queue).toBeEnabled();
});

test("a prepared direct review is rejected after selection membership leaves and returns to the same primary", async ({ page }) => {
  const model = await gotoRoutedFixture(page);
  const primary = model.pipe_segments[2];
  const transient = model.nodes[6];
  await selectTreeRow(page, "pipe", primary.id);
  await activateWithKeyboard(page, page.getByTestId("command-node"));
  await page.getByTestId("viewport-create-node-id").fill("node:UIF-ABA-REVIEW");
  await page.getByTestId("viewport-create-node-label").fill("ABA stale review probe");
  await page.getByTestId("viewport-create-node-x").fill("12.5");
  await page.getByTestId("viewport-create-node-y").fill("0");
  await page.getByTestId("viewport-create-node-z").fill("0");
  await page.getByTestId("viewport-create-node-provenance").fill("invented_functional_qualification_input");
  await activateWithKeyboard(page, page.getByTestId("queue-explicit-node-intent"));
  await expect(page.getByTestId("viewport-draft-review-preview")).toContainText("node:UIF-ABA-REVIEW");

  await selectTreeRow(page, "node", transient.id, { shift: true });
  await selectTreeRow(page, "node", transient.id, { toggle: true });
  await expect(page.getByTestId("command-selection-readout")).toContainText(`pipe: ${primary.id}`);
  await expect(page.getByTestId("apply-reviewed-draft")).toBeDisabled();
  await expect(page.getByTestId("viewport-draft-review-preview")).toHaveCount(0);
  await expect(page.getByTestId("viewport-draft-review-message")).toContainText("The affected selection changed. Add again to review the current draft.");
  await page.getByTestId("model-tree-filter-input").fill("node:UIF-ABA-REVIEW");
  await expect(typedTreeRow(page, "node", "node:UIF-ABA-REVIEW")).toHaveCount(0);
});

test("selected-pipe actions copy a stable draft target snapshot instead of following later selection", async ({ page }) => {
  const model = await gotoRoutedFixture(page);
  const [first, second, third] = model.pipe_segments.slice(2, 5);
  await selectTreeRow(page, "pipe", first.id);
  await selectTreeRow(page, "pipe", second.id, { shift: true });

  await activateWithKeyboard(page, page.getByTestId("toolkit-entry"));
  await page.getByRole("searchbox", { name: "Find a tool" }).fill("Copy");
  await activateWithKeyboard(page, page.getByTestId("toolkit-edit.copy"));
  const geometry = page.locator("#geometry-tools");
  await geometry.getByLabel("Geometry tool").selectOption("translate");
  await activateWithKeyboard(page, geometry.getByRole("button", { name: "Use selected pipes", exact: true }));
  await expect(geometry.getByText("Frozen source snapshot: 2 pipes.", { exact: true })).toBeVisible();

  await selectTreeRow(page, "pipe", third.id);
  await expect(geometry.getByText("Frozen source snapshot: 2 pipes.", { exact: true })).toBeVisible();
  const thirdOption = geometry.getByTestId("geometry-source-pipes").getByRole("option", { name: new RegExp(`${escapeRegExp(third.id)}$`) });
  await geometry.getByTestId("geometry-source-pipes").getByRole("combobox").fill(third.id);
  await expect(thirdOption).toHaveAttribute("aria-selected", "false");

  await activateWithKeyboard(page, page.getByTestId("toolkit-entry"));
  await page.getByRole("searchbox", { name: "Find a tool" }).fill("self-weight");
  await activateWithKeyboard(page, page.getByTestId("toolkit-loads.self-weight"));
  const selfWeight = page.locator("#self-weight-plan");
  await activateWithKeyboard(page, selfWeight.getByRole("button", { name: "Use selected pipes", exact: true }));
  await expect(selfWeight.getByText(`Frozen selected-pipe snapshot: ${third.id}`, { exact: true })).toBeVisible();
  await selectTreeRow(page, "pipe", first.id);
  await expect(selfWeight.getByText(`Frozen selected-pipe snapshot: ${third.id}`, { exact: true })).toBeVisible();
});

test("filtered multi-target picker preserves Space text and scrolls a 99-row subset to End before Enter toggles", async ({ page }, testInfo) => {
  const model = await gotoRoutedFixture(page, "ui-foundation-1000.model.json");
  await selectTreeRow(page, "pipe", model.pipe_segments[0].id);
  await activateWithKeyboard(page, page.getByTestId("toolkit-entry"));
  await page.getByRole("searchbox", { name: "Find a tool" }).fill("Copy");
  await activateWithKeyboard(page, page.getByTestId("toolkit-edit.copy"));

  const picker = page.locator("#geometry-tools").getByTestId("geometry-source-pipes");
  const search = picker.getByRole("combobox", { name: "Source pipe runs" });
  await search.focus();
  await search.pressSequentially("benchmark");
  await page.keyboard.press("Space");
  await expect(search).toHaveValue("benchmark ");
  await search.pressSequentially("pipe 000");
  await expect(search).toHaveValue("benchmark pipe 000");
  await expect(picker.locator(".virtual-target-picker-status")).toContainText("99 of 1000 targets; 0 selected");

  const list = picker.getByRole("listbox", { name: "Source pipe runs options" });
  const listGeometry = await list.evaluate((element) => ({
    clientHeight: element.clientHeight,
    scrollHeight: element.scrollHeight,
    overflowY: getComputedStyle(element).overflowY,
  }));
  expect(listGeometry.clientHeight).toBeLessThanOrEqual(240);
  expect(listGeometry.scrollHeight).toBeGreaterThan(listGeometry.clientHeight);
  expect(["auto", "scroll"]).toContain(listGeometry.overflowY);

  const expected = model.pipe_segments[98];
  const option = picker.getByRole("option", { name: new RegExp(`${escapeRegExp(expected.id)}$`) });
  const optionId = await option.getAttribute("id");
  expect(optionId).not.toBeNull();
  const geometryBeforeEnd = await picker.evaluate((root, activeOptionId) => {
    const listbox = root.querySelector<HTMLElement>('[role="listbox"]')!;
    const optionElement = root.querySelector<HTMLElement>(`#${CSS.escape(String(activeOptionId))}`)!;
    const dock = root.closest<HTMLElement>(".workspace-dock-body");
    const rect = (element: HTMLElement | null) => element ? element.getBoundingClientRect().toJSON() : null;
    return {
      dock: dock ? { clientHeight: dock.clientHeight, scrollHeight: dock.scrollHeight, scrollTop: dock.scrollTop, rect: rect(dock) } : null,
      list: { clientHeight: listbox.clientHeight, scrollHeight: listbox.scrollHeight, scrollTop: listbox.scrollTop, rect: rect(listbox) },
      option: { rect: rect(optionElement) },
    };
  }, optionId);
  await page.keyboard.press("End");
  await expect(search).toHaveAttribute("aria-activedescendant", optionId!);
  const geometryAfterEnd = await picker.evaluate((root, activeOptionId) => {
    const listbox = root.querySelector<HTMLElement>('[role="listbox"]')!;
    const optionElement = root.querySelector<HTMLElement>(`#${CSS.escape(String(activeOptionId))}`)!;
    const dock = root.closest<HTMLElement>(".workspace-dock-body");
    const rect = (element: HTMLElement | null) => element ? element.getBoundingClientRect().toJSON() : null;
    return {
      dock: dock ? { clientHeight: dock.clientHeight, scrollHeight: dock.scrollHeight, scrollTop: dock.scrollTop, rect: rect(dock) } : null,
      list: { clientHeight: listbox.clientHeight, scrollHeight: listbox.scrollHeight, scrollTop: listbox.scrollTop, rect: rect(listbox) },
      option: { rect: rect(optionElement) },
    };
  }, optionId);
  await testInfo.attach("filtered-picker-end-scroll-geometry", {
    body: JSON.stringify({ beforeEnd: geometryBeforeEnd, afterEnd: geometryAfterEnd }, null, 2),
    contentType: "application/json",
  });
  await expect(option).toBeInViewport();
  expect(await list.evaluate((element) => element.scrollTop)).toBeGreaterThan(0);
  const rowBox = await option.locator("xpath=..").boundingBox();
  expect(rowBox?.height).toBe(32);

  await page.keyboard.press("Enter");
  await expect(search).toHaveValue("benchmark pipe 000");
  await expect(option).toHaveAttribute("aria-selected", "true");
  await expect(picker.locator(".virtual-target-picker-status")).toContainText("99 of 1000 targets; 1 selected");
});

test("node visibility owns attached symbols while connected spans remain independent, and isolate is bounded", async ({ page }) => {
  const model = await gotoRoutedFixture(page, "ui-foundation-1000.model.json");
  const node = model.nodes[100];
  const isolatePeer = model.nodes[101];
  const support = model.supports.find((item: any) => item.node === node.id)!;
  const component = model.components.find((item: any) => item.node === node.id)!;
  const connected = model.pipe_segments.find((item: any) => item.from === node.id || item.to === node.id)!;
  const other = model.pipe_segments.find((item: any) => item.from !== node.id && item.to !== node.id)!;
  const hashBefore = await currentModelHashThroughVisibleExport(page);

  await selectTreeRow(page, "node", node.id);
  await activateWithKeyboard(page, page.getByRole("button", { name: "Hide", exact: true }));
  await expect(await revealTreeRow(page, "node", node.id)).toContainText("Hidden");
  await expect(await revealTreeRow(page, "support", support.id)).toContainText("Hidden");
  await expect(await revealTreeRow(page, "component", component.id)).toContainText("Hidden");
  await expect(await revealTreeRow(page, "pipe", connected.id)).not.toContainText("Hidden");

  await selectTreeRow(page, "node", isolatePeer.id, { toggle: true });
  await activateWithKeyboard(page, page.getByRole("button", { name: "Isolate", exact: true }));
  await expect(await revealTreeRow(page, "node", node.id)).toContainText("Hidden");
  await expect(await revealTreeRow(page, "node", isolatePeer.id)).not.toContainText("Hidden");
  await expect(await revealTreeRow(page, "pipe", other.id)).toContainText("Hidden");
  await activateWithKeyboard(page, page.getByRole("button", { name: "Show All", exact: true }));
  await expect(await revealTreeRow(page, "node", node.id)).not.toContainText("Hidden");
  await expect(await revealTreeRow(page, "pipe", other.id)).not.toContainText("Hidden");

  await selectTreeRow(page, "support", support.id);
  await activateWithKeyboard(page, page.getByRole("button", { name: "Isolate", exact: true }));
  await expect(await revealTreeRow(page, "support", support.id)).not.toContainText("Hidden");
  await expect(await revealTreeRow(page, "node", node.id)).not.toContainText("Hidden");
  await expect(await revealTreeRow(page, "component", component.id)).not.toContainText("Hidden");
  await expect(await revealTreeRow(page, "pipe", other.id)).toContainText("Hidden");
  await activateWithKeyboard(page, page.getByRole("button", { name: "Show All", exact: true }));
  expect(await currentModelHashThroughVisibleExport(page)).toBe(hashBefore);
});

test("Actual OD uses valid spans and explains the independently injected centerline fallback", async ({ page }) => {
  const fixture = await readFixture("precision-origin-base.model.json");
  await gotoModel(page, withOneInvalidOd(fixture.model));
  const hashBefore = await currentModelHashThroughVisibleExport(page);
  await activateWithKeyboard(page, page.getByTestId("viewport-geometry-actual-od"));
  await expect(page.getByTestId("viewport-od-status")).toContainText("1 pipe span retained centerline");
  await expect(page.getByTestId("viewport-od-status")).not.toContainText("Validating");
  await activateWithKeyboard(page, page.getByRole("button", { name: "Fit Visible", exact: true }));
  expect(await currentModelHashThroughVisibleExport(page)).toBe(hashBefore);
});

test("ordinary typed edit preserves view state and same-ID project replacement clears it", async ({ page }) => {
  const model = await gotoRoutedFixture(page);
  const node = model.nodes[5];
  await activateWithKeyboard(page, page.getByRole("button", { name: "Save local", exact: true }));
  await expect(page.getByTestId("local-project-message")).toContainText("Saved");
  await selectTreeRow(page, "node", node.id);
  await activateWithKeyboard(page, page.getByRole("button", { name: "Hide", exact: true }));

  await ensureRail(page, "inspector", true);
  const inspector = page.getByTestId("property-inspector");
  await activateWithKeyboard(page, inspector.getByRole("tab", { name: "Task", exact: true }));
  await activateWithKeyboard(page, inspector.getByTestId("inspector-start-task"));
  await inspector.getByTestId("editor-intent-value").fill("Edited while hidden");
  await activateWithKeyboard(page, inspector.getByTestId("queue-editor-intent"));
  const apply = page.locator('[data-testid^="apply-intent-"]').first();
  await expect(apply).toBeEnabled();
  await activateWithKeyboard(page, apply);
  await expect(page.getByTestId("operation-apply-message")).toContainText("Applied");
  await expect(await revealTreeRow(page, "node", node.id)).toContainText("Hidden");

  await activateWithKeyboard(page, page.getByTestId("toolkit-entry"));
  await page.getByRole("searchbox", { name: "Find a tool" }).fill("Undo");
  await activateWithKeyboard(page, page.getByTestId("toolkit-edit.undo"));
  await expect(await revealTreeRow(page, "node", node.id)).toContainText(node.label);
  await activateWithKeyboard(page, page.getByTestId("toolkit-entry"));
  await page.getByRole("searchbox", { name: "Find a tool" }).fill("Redo");
  await activateWithKeyboard(page, page.getByTestId("toolkit-edit.redo"));
  await expect(await revealTreeRow(page, "node", node.id)).toContainText("Edited while hidden");

  await activateWithKeyboard(page, page.getByTestId("open-local-project"));
  await expect(page.getByTestId("command-selection-readout")).toContainText(`project: ${model.project.id}`);
  await expect(page.getByRole("button", { name: "Show All", exact: true })).toBeDisabled();
  await expect(page.getByTestId("viewport-measurement-readout")).toHaveCount(0);
  await expect(await revealTreeRow(page, "node", node.id)).not.toContainText("Hidden");
});

test("keyboard splitters stay named and bounded; narrow drawers restore opener focus and hide inactive controls", async ({ page }) => {
  await page.setViewportSize({ width: 1440, height: 920 });
  await gotoRoutedFixture(page);
  for (const name of ["Resize model tree", "Resize property inspector"] as const) {
    const separator = page.getByRole("separator", { name });
    const before = Number(await separator.getAttribute("aria-valuenow"));
    await separator.focus();
    await page.keyboard.press(name.includes("tree") ? "ArrowRight" : "ArrowLeft");
    const after = Number(await separator.getAttribute("aria-valuenow"));
    expect(after).not.toBe(before);
    expect(after).toBeGreaterThanOrEqual(Number(await separator.getAttribute("aria-valuemin")));
    expect(after).toBeLessThanOrEqual(Number(await separator.getAttribute("aria-valuemax")));
  }
  await openWorkspaceSection(page, "operations");
  const dock = page.getByRole("separator", { name: "Resize task dock" });
  await dock.focus();
  const dockBefore = Number(await dock.getAttribute("aria-valuenow"));
  await page.keyboard.press("ArrowUp");
  expect(Number(await dock.getAttribute("aria-valuenow"))).toBeGreaterThan(dockBefore);

  await page.setViewportSize({ width: 1024, height: 768 });
  await activateWithKeyboard(page, page.getByTestId("workspace-dock-close"));
  await ensureRail(page, "tree", false);
  await ensureRail(page, "inspector", false);
  const treeToggle = page.getByTestId("toggle-tree");
  const toggle = page.getByTestId("toggle-inspector");
  await expectCenterUnobscured(treeToggle, { minimumTarget: true });
  await expectCenterUnobscured(toggle, { minimumTarget: true });

  await activateWithKeyboard(page, treeToggle);
  await expect(treeToggle).toHaveAttribute("aria-expanded", "true");
  await expect(toggle).toHaveAttribute("aria-expanded", "false");
  await expectCenterUnobscured(page.getByTestId("model-tree-filter-input"), { minimumTarget: true });

  await activateWithKeyboard(page, toggle);
  await expect(toggle).toHaveAttribute("aria-expanded", "true");
  await expect(treeToggle).toHaveAttribute("aria-expanded", "false");
  await page.getByTestId("property-inspector").getByRole("tab", { name: "Task", exact: true }).focus();
  await expectCenterUnobscured(page.getByTestId("property-inspector").getByRole("tab", { name: "Task", exact: true }), { minimumTarget: true });
  await page.keyboard.press("Escape");
  await expect(toggle).toBeFocused();
  await expect(toggle).toHaveAttribute("aria-expanded", "false");
  await expect(page.getByTestId("property-inspector")).toBeHidden();
  await page.keyboard.press("Tab");
  const tabEnteredInactiveInspector = await page.locator(".workspace-pane-inspector").evaluate((pane) =>
    pane.contains(document.activeElement) && document.activeElement?.getAttribute("data-testid") !== "toggle-inspector",
  );
  expect(tabEnteredInactiveInspector).toBe(false);
  await page.keyboard.press("Shift+Tab");
  await expect(toggle).toBeFocused();

  await activateWithKeyboard(page, treeToggle);
  await page.getByTestId("model-tree-filter-input").focus();
  await page.keyboard.press("Escape");
  await expect(treeToggle).toBeFocused();
  await expect(treeToggle).toHaveAttribute("aria-expanded", "false");
});

const MEASUREMENT_LABELS = ["Distance", "ΔX", "ΔY", "ΔZ"] as const;
type MeasurementLabel = (typeof MEASUREMENT_LABELS)[number];
const MEASUREMENT_PUBLICATION_TOLERANCE = 0.5e-6 + 1e-10;
const METRES_PER_INCH = 0.0254;

type AuthoredPoint = Readonly<{ x: number; y: number; z: number }>;

async function describeViewportHit(
  page: Page,
  point: Readonly<{ x: number; y: number }>,
): Promise<Readonly<{
  closestSelectionTargetTestId: string | null;
  directTagName: string | null;
  directTestId: string | null;
  isMainCanvas: boolean;
}>> {
  return page.evaluate(({ x, y }) => {
    const element = document.elementFromPoint(x, y);
    const mainCanvas = document.querySelector('[data-testid="viewport-canvas"] canvas');
    const selectionTarget = element?.closest(".viewport-select-target");
    return {
      closestSelectionTargetTestId: selectionTarget instanceof HTMLElement ? selectionTarget.dataset.testid ?? null : null,
      directTagName: element?.tagName ?? null,
      directTestId: element instanceof HTMLElement ? element.dataset.testid ?? null : null,
      isMainCanvas: element === mainCanvas,
    };
  }, point);
}

async function readCameraProjectionWitness(page: Page, authoredPoints: readonly AuthoredPoint[]): Promise<any> {
  return page.evaluate((points) => {
    const api = globalThis.__openPipeStressUiDiagnosticsV1;
    const snapshot = api.readCurrent();
    if ("status" in snapshot.viewport) throw new Error(`Viewport diagnostics unavailable: ${JSON.stringify(snapshot.viewport)}`);
    const canvas = document.querySelector<HTMLCanvasElement>('[data-testid="viewport-canvas"] canvas');
    if (!canvas) throw new Error("The public viewport canvas is unavailable.");
    const canvasDomRect = canvas.getBoundingClientRect();
    const pair = {
      modelGeneration: snapshot.model.generation,
      cameraSequence: snapshot.viewport.camera.sequence,
    };
    const projections = points.map((authoredPoint) => api.projectAuthoredPoint({ ...pair, authoredPoint }));
    if (projections.some((projection) => projection.status !== "available")) {
      throw new Error(`Authored projection unavailable: ${JSON.stringify(projections)}`);
    }
    return {
      camera: snapshot.viewport.camera,
      canvasRect: snapshot.viewport.canvas,
      canvasDomRect: {
        left: canvasDomRect.left,
        top: canvasDomRect.top,
        width: canvasDomRect.width,
        height: canvasDomRect.height,
      },
      ownedPendingRafCount: snapshot.viewport.resources.ownedPendingRafCount,
      projections,
    };
  }, authoredPoints);
}

async function settledCameraProjectionWitness(page: Page, authoredPoints: readonly AuthoredPoint[]): Promise<any> {
  let previous = await readCameraProjectionWitness(page, authoredPoints);
  for (let attempt = 0; attempt < 75; attempt += 1) {
    await page.waitForTimeout(40);
    const current = await readCameraProjectionWitness(page, authoredPoints);
    if (
      previous.ownedPendingRafCount === 0 &&
      current.ownedPendingRafCount === 0 &&
      JSON.stringify(current.camera) === JSON.stringify(previous.camera) &&
      JSON.stringify(current.projections) === JSON.stringify(previous.projections) &&
      JSON.stringify(current.canvasRect) === JSON.stringify(previous.canvasRect) &&
      JSON.stringify(current.canvasDomRect) === JSON.stringify(previous.canvasDomRect)
    ) return current;
    previous = current;
  }
  throw new Error(`Viewport camera, authored projections, and canvas rectangle did not reach consecutive exact equality with zero product-owned pending RAFs within 3 seconds; last witness ${JSON.stringify(previous)}.`);
}

function expectStableCanvasRectangle(actual: any, expected: any, label: string): void {
  expect(actual.canvasRect, `${label} published canvas rectangle`).toEqual(expected.canvasRect);
  expect(actual.canvasDomRect, `${label} DOM canvas rectangle`).toEqual(expected.canvasDomRect);
  expect(actual.ownedPendingRafCount, `${label} pending product RAF`).toBe(0);
}

function expectStableViewportEndpoint(actual: any, expected: any, label: string): void {
  expect(cameraPose(actual.camera), `${label} camera pose`).toEqual(cameraPose(expected.camera));
  expect(actual.projections.map(projectionGeometry), `${label} authored projections`).toEqual(expected.projections.map(projectionGeometry));
  expectStableCanvasRectangle(actual, expected, label);
}

function cameraPose({ sequence: _sequence, ...pose }: any): any {
  return pose;
}

function projectionGeometry({ modelGeneration: _modelGeneration, cameraSequence: _cameraSequence, ...geometry }: any): any {
  return geometry;
}

async function readPaneState(page: Page): Promise<any> {
  return page.evaluate(() => {
    const workspace = document.querySelector(".modeling-workspace");
    const treeToggle = document.querySelector<HTMLElement>('[data-testid="toggle-tree"]');
    const inspectorToggle = document.querySelector<HTMLElement>('[data-testid="toggle-inspector"]');
    return {
      workspaceClass: workspace?.className ?? null,
      treeExpanded: treeToggle?.getAttribute("aria-expanded") ?? null,
      inspectorExpanded: inspectorToggle?.getAttribute("aria-expanded") ?? null,
    };
  });
}

async function readPublishedSelection(page: Page): Promise<any> {
  return page.evaluate(() => {
    const snapshot = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
    if ("status" in snapshot.viewport) throw new Error(`Viewport diagnostics unavailable: ${JSON.stringify(snapshot.viewport)}`);
    return {
      orderedRefs: snapshot.viewport.selection.orderedRefs,
      primaryRef: snapshot.viewport.selection.primaryRef,
    };
  });
}

async function readViewportToolbarSelectionStatus(page: Page): Promise<any> {
  const status = page.locator(".viewport-toolbar-selection-status");
  await expect(status).toHaveCount(1);
  return status.evaluate((element) => {
    const toolbar = element.closest(".viewport-toolbar");
    if (!(toolbar instanceof HTMLElement)) throw new Error("Viewport toolbar is unavailable.");
    const rect = element.getBoundingClientRect();
    const toolbarRect = toolbar.getBoundingClientRect();
    const style = getComputedStyle(element);
    return {
      text: element.textContent ?? "",
      title: element.getAttribute("title"),
      rect: { left: rect.left, top: rect.top, right: rect.right, bottom: rect.bottom, width: rect.width, height: rect.height },
      toolbarRect: {
        left: toolbarRect.left,
        top: toolbarRect.top,
        right: toolbarRect.right,
        bottom: toolbarRect.bottom,
        width: toolbarRect.width,
        height: toolbarRect.height,
      },
      overflow: style.overflow,
      textOverflow: style.textOverflow,
      whiteSpace: style.whiteSpace,
    };
  });
}

function expectToolbarSelectionStatus(status: any, id: string, label: string): void {
  expect(status.text, `${label} full selected ID`).toBe(`Selected: ${id}`);
  expect(status.rect.left, `${label} selected status left bound`).toBeGreaterThanOrEqual(status.toolbarRect.left - 0.5);
  expect(status.rect.right, `${label} selected status right bound`).toBeLessThanOrEqual(status.toolbarRect.right + 0.5);
  expect(status.rect.top, `${label} selected status top bound`).toBeGreaterThanOrEqual(status.toolbarRect.top - 0.5);
  expect(status.rect.bottom, `${label} selected status bottom bound`).toBeLessThanOrEqual(status.toolbarRect.bottom + 0.5);
  expect(status.rect.height, `${label} selected status remains a single compact line`).toBeLessThanOrEqual(18);
}

async function visibleCanvasDragPath(canvas: Locator): Promise<Readonly<{
  start: Readonly<{ x: number; y: number }>;
  end: Readonly<{ x: number; y: number }>;
}>> {
  const path = await canvas.evaluate((element) => {
    const box = element.getBoundingClientRect();
    const candidates = [0.18, 0.30, 0.42, 0.58, 0.70, 0.82].flatMap((xFraction) =>
      [0.20, 0.34, 0.48, 0.62, 0.76].flatMap((yFraction) => {
        const point = { x: box.left + box.width * xFraction, y: box.top + box.height * yFraction };
        return document.elementFromPoint(point.x, point.y) === element ? [point] : [];
      }),
    );
    let best: { start: { x: number; y: number }; end: { x: number; y: number }; distance: number } | null = null;
    for (const start of candidates) {
      for (const end of candidates) {
        const distance = Math.hypot(end.x - start.x, end.y - start.y);
        if (!best || distance > best.distance) best = { start, end, distance };
      }
    }
    return best ? { start: best.start, end: best.end } : null;
  });
  expect(path, "two separated real canvas hit targets exist").not.toBeNull();
  expect(Math.hypot(path!.end.x - path!.start.x, path!.end.y - path!.start.y)).toBeGreaterThan(40);
  return path!;
}

async function realCanvasDrag(
  page: Page,
  path: Readonly<{ start: Readonly<{ x: number; y: number }>; end: Readonly<{ x: number; y: number }> }>,
  button: "left" | "right" = "left",
): Promise<void> {
  await page.mouse.move(path.start.x, path.start.y);
  await page.mouse.down({ button });
  await page.mouse.move(path.end.x, path.end.y, { steps: 6 });
  await page.mouse.up({ button });
}

async function expectMeasurementQuantities(
  readout: Locator,
  expected: Record<MeasurementLabel, number>,
  unit: "m" | "in",
  status: "entered" | "converted",
): Promise<void> {
  for (const label of MEASUREMENT_LABELS) {
    const quantity = readout.locator(`[data-measurement-label="${label}"]`);
    await expect(quantity).toHaveCount(1);
    await expect(quantity).toHaveAttribute("data-display-status", status);
    await expect(quantity).toHaveAttribute("data-raw-unit", unit);

    const rawValueText = await quantity.getAttribute("data-raw-value");
    expect(rawValueText, `${label} exposes a full-precision raw value`).not.toBeNull();
    const rawValue = Number(rawValueText);
    expect(Number.isFinite(rawValue), `${label} raw value is numeric`).toBe(true);
    expect(
      Math.abs(rawValue - expected[label]),
      `${label} raw ${unit} value agrees with the independently converted fixture quantity`,
    ).toBeLessThanOrEqual(fullPrecisionTolerance(expected[label]));

    const visibleText = (await quantity.innerText()).trim();
    const visibleMatch = visibleText.match(
      new RegExp(`^${escapeRegExp(label)} ([+-]?[0-9]+(?:\\.[0-9]+)?(?:e[+-]?[0-9]+)?) ${unit}$`, "i"),
    );
    expect(visibleMatch, `${label} has a compact ${unit} publication`).not.toBeNull();
    const presentedValueText = visibleMatch![1];
    expect(
      Math.abs(Number(presentedValueText) - expected[label]),
      `${label} compact ${unit} publication agrees within the declared-unit display tolerance`,
    ).toBeLessThanOrEqual(MEASUREMENT_PUBLICATION_TOLERANCE);

    await expect(quantity).toHaveAttribute("title", `${label} full precision: ${rawValueText} ${unit}`);
    await expect(quantity).toHaveAttribute(
      "aria-label",
      `${label}: ${presentedValueText} ${unit}; full-precision numeric value ${rawValueText} ${unit}.`,
    );
  }
}

function fullPrecisionTolerance(expected: number): number {
  return Math.max(1e-12, Math.abs(expected) * Number.EPSILON * 8);
}

function escapeRegExp(value: string): string {
  return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

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

for (const size of [1000, 10000]) {
  test(`standard ${size} camera recipe compares legacy and corrected fit without changing targets`, async ({ page }, testInfo) => {
    await page.setViewportSize({ width: 1440, height: 920 });
    const model = await gotoRoutedFixture(page, `ui-foundation-${size}.model.json`);
    await activateWithKeyboard(page, page.getByTestId("viewport-view-isometric"));
    await activateWithKeyboard(page, page.getByTestId("viewport-fit-model"));
    const actual = await readCameraProjectionWitness(page, []);
    const index = buildModelIndex(model, 1, 0);
    const bounds = displayedBoundsForEntityKeys(index, index.visibilityEligibleKeys, new Set(), new Map())!;
    const extent = Math.max(bounds.max.x - bounds.min.x, bounds.max.y - bounds.min.y, bounds.max.z - bounds.min.z, 1);
    const legacyDistance = Math.max(4, extent * 1.8);
    const successorDistance = fittedViewportDistance(bounds, "iso", actual.camera.fovDegrees, actual.camera.aspect);
    const center = [bounds.min.x / 2 + bounds.max.x / 2, bounds.min.y / 2 + bounds.max.y / 2, bounds.min.z / 2 + bounds.max.z / 2];
    const legacyPosition = center.map((value) => value + legacyDistance / Math.sqrt(3));
    const successorPosition = center.map((value) => value + successorDistance / Math.sqrt(3));
    actual.camera.position.forEach((value: number, axis: number) => expect(value).toBeCloseTo(successorPosition[axis], 9));
    actual.camera.target.forEach((value: number, axis: number) => expect(value).toBeCloseTo(center[axis], 9));
    await testInfo.attach(`standard-camera-comparison-${size}`, { body: JSON.stringify({ size, recipe: ["Isometric", "Fit Model"], bounds, legacyDistance, successorDistance, legacyPosition, successorPosition, legacyPosePreserved: legacyDistance === successorDistance, actual }, null, 2), contentType: "application/json" });
  });
}

test("filtered tree collapse controls retain typed selection and frozen draft across search changes", async ({ page }, testInfo) => {
  await page.setViewportSize({ width: 1440, height: 920 });
  const model = await gotoRoutedFixture(page, "ui-foundation-1000.model.json");
  const node = model.nodes[0];
  await selectTreeRow(page, "node", node.id);
  await clearTreeFilter(page);
  await ensureRail(page, "inspector", true);
  const inspector = page.getByTestId("property-inspector");
  await activateWithKeyboard(page, inspector.getByRole("tab", { name: "Task", exact: true }));
  await activateWithKeyboard(page, inspector.getByTestId("inspector-start-task"));
  await inspector.getByTestId("editor-intent-value").fill("Retained filtered-tree draft");
  const frozenTarget = await inspector.getByTestId("inspector-frozen-task-target").innerText();
  const selection = async () => page.evaluate(() => {
    const snapshot = globalThis.__openPipeStressUiDiagnosticsV1.readCurrent();
    if ("status" in snapshot.viewport) throw new Error("Viewport unavailable");
    return snapshot.viewport.selection;
  });
  const before = await selection();
  const group = page.getByTestId("tree-group-Nodes");
  const tree = page.getByRole("tree", { name: "Model", exact: true });
  const input = page.getByTestId("model-tree-filter-input");
  const row = page.getByTestId(`tree-row-node-${encodeURIComponent(node.id)}`);
  await group.click(); await expect(group).toHaveAttribute("aria-expanded", "false");
  await input.fill(node.label); await expect(group).toHaveAttribute("aria-expanded", "true");
  await tree.focus(); await page.keyboard.press("Home"); await page.keyboard.press("ArrowRight");
  await expect(tree).toHaveAttribute("aria-activedescendant", `tree-row-node-${encodeURIComponent(node.id)}`);
  await page.keyboard.press("ArrowLeft"); await expect(group).toHaveAttribute("aria-expanded", "true");
  await expect(tree).toHaveAttribute("aria-activedescendant", "tree-group-Nodes");
  await page.keyboard.press("ArrowLeft"); await expect(group).toHaveAttribute("aria-expanded", "false"); await expect(row).toHaveCount(0);
  await input.fill(`  ${node.label.toUpperCase()}  `); await expect(group).toHaveAttribute("aria-expanded", "false");
  await tree.focus();
  for (const [key, expanded] of [["ArrowRight", true], ["Enter", false], ["Space", true], ["Enter", false], ["Space", true]] as const) {
    await page.keyboard.press(key); await expect(group).toHaveAttribute("aria-expanded", String(expanded));
    await expect(row).toHaveCount(expanded ? 1 : 0);
  }
  await group.click(); await expect(group).toHaveAttribute("aria-expanded", "false");
  await input.fill(model.nodes[1].label); await expect(group).toHaveAttribute("aria-expanded", "true");
  await input.fill(""); await expect(group).toHaveAttribute("aria-expanded", "false");
  await input.fill(node.label); await expect(row).toHaveAttribute("aria-selected", "true");
  await expect(inspector.getByTestId("editor-intent-value")).toHaveValue("Retained filtered-tree draft");
  await expect(inspector.getByTestId("inspector-frozen-task-target")).toHaveText(frozenTarget);
  const after = await selection();
  expect(after.orderedRefs).toEqual(before.orderedRefs); expect(after.primaryRef).toEqual(before.primaryRef);
  expect(after.actionSequence).toBe(before.actionSequence); expect(after.generation).toBe(before.generation);
  await testInfo.attach("filtered-tree-selection-and-draft", { body: JSON.stringify({ before, after, frozenTarget }, null, 2), contentType: "application/json" });
});


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
  // Active OrbitControls updates normalize/rescale the target even without a
  // target-radius constraint. Allow binary64 roundoff from that arithmetic and
  // the local/authored translation; idle and Box invariants stay exact above.
  const targetScale = Math.max(1, ...panAfter.camera.target.map(Math.abs), ...panAfter.camera.localRenderOrigin.map(Math.abs));
  const targetRoundoff = 16 * Number.EPSILON * targetScale;
  wheelAfter.camera.target.forEach((value, i) => {
    expect(Number.isFinite(value)).toBe(true);
    expect(Math.abs(value - panAfter.camera.target[i])).toBeLessThanOrEqual(targetRoundoff);
  });
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
