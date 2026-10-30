import { focusedBoxSamples, performanceExpectation, runCandidateCausalPerformance } from "./full-cohort-controller";
import { createHash, randomUUID } from "node:crypto";
import { mkdir, readFile, writeFile, realpath, stat } from "node:fs/promises";
import path from "node:path";
import { performance as nodePerformance } from "node:perf_hooks";
import { setTimeout as nodeDelay } from "node:timers/promises";
import { expect, test, type Page } from "@playwright/test";
import { assertBoundCandidateDocumentResponse, bindCandidateDriverEntry } from "./candidate-server-response";
import {
  armCausalFeedbackMarker,
  benchmarkPhase,
  captureChromiumEnvironment,
  currentSelectionState,
  ensureViewportToggle,
  expectedVisibleEntityLabel,
  expectedBoxInspectorHeading,
  installInstrumentation,
  loadFixture,
  measureCanvasBoxSelection,
  validatedPriorBoxBaseline,
  persistBoxPostcondition,
  readBoxVisibleIdentity,
  boxCallbackMatchesSnapshot,
  nearestRankP95,
  pngPixelDigest,
  captureWinnerCue,
  cueActionBindingsStable,
  pairedWinnerCueWitness,
  pointCaptureMatchesMarker,
  projectCandidateAuthoredPoint,
  readCausalFeedbackMarker,
  readCandidateDiagnostics,
  requireUniqueConnectedMainCanvas,
  requireEvidenceRoot,
  restoreCausalFeedbackInstrumentation,
  routeModelFixture,
  stopCausalFeedbackMarker,
  treeRowTestId,
  validateMainCanvasHitTarget,
  validateCandidateMeasuredCameraBinding,
  validateCandidateOracleBinding,
  waitForCandidateExclusiveSelection,
  waitForFirstUsable,
  writeJson
} from "./benchmark-harness";
import { CausalPhaseJournal } from "./causal-phase-journal";
import {
  ORBIT_START_NORMALIZED,
  assertMainCanvasHitTarget,
  assertStoppedCausalEvidence,
  canvasLocalToClient,
  normalizedCanvasPoint,
  observerValidity,
  instrumentedPreflightPass
} from "./causal-method-contract";
import { beginChromiumCompositorTrace, endChromiumCompositorTrace, traceMarker } from "./chromium-compositor-trace";
import {
  CausalPresentationExtractionError,
  REQUIRED_CHROMIUM_BINDING,
  extractCausalPresentations,
  inventoryCausalPresentationLineages,
  presentationGapsForMeasuredWindow
} from "./causal-presentation-extractor.mjs";

const phase = benchmarkPhase();
const pipeCount = Number(process.env.UI_FOUNDATION_CAUSAL_PIPE_COUNT ?? "1000");
const sampleNumber = Number(process.env.UI_FOUNDATION_CAUSAL_POINT_SAMPLE ?? "2");
const stageTimeoutMs = Number(process.env.UI_FOUNDATION_PREFLIGHT_STAGE_TIMEOUT_MS ?? "60000");
const pointSettleMs = 500;
const orbitWarmupMs = 500;
const orbitMeasuredMs = 2_000;
const orbitTrailingMs = 250;

if ((pipeCount !== 1_000 && !(phase === "candidate" && pipeCount === 10_000)) || !Number.isInteger(sampleNumber) || sampleNumber < 1 || sampleNumber > 200 ||
    !Number.isFinite(stageTimeoutMs) || stageTimeoutMs < 5_000 || stageTimeoutMs > 60_000) {
  throw new Error("causal preflight requires N=1000 (or candidate N=10000), a point sample from 1 through 200, and a 5s through 60s stage timeout");
}

const exactRef = (left: any, right: any): boolean => left?.type === right?.type && left?.id === right?.id;

async function establishProjectSelection(page: Page, fixture: any): Promise<any> {
  const projectRef = { type: "project", id: fixture.model.project.id };
  const alternateRef = fixture.model.materials?.[0]
    ? { type: "material", id: fixture.model.materials[0].id }
    : { type: "node", id: fixture.model.nodes[0].id };
  const alternateRow = page.getByTestId(treeRowTestId(alternateRef, phase));
  const projectRow = page.getByTestId(treeRowTestId(projectRef, phase));
  if (phase === "candidate") {
    const before = (await readCandidateDiagnostics(page, true)).snapshot;
    await alternateRow.click({ timeout: stageTimeoutMs });
    const alternate = await waitForCandidateExclusiveSelection(
      page, alternateRef, before.model.generation, before.viewport.selection.actionSequence,
      before.viewport.mainRender.submissionSequence, stageTimeoutMs
    );
    await projectRow.click({ timeout: stageTimeoutMs });
    const project = await waitForCandidateExclusiveSelection(
      page, projectRef, alternate.model.generation, alternate.viewport.selection.actionSequence,
      alternate.viewport.mainRender.submissionSequence, stageTimeoutMs
    );
    return { input: "real typed tree-row clicks", alternateRef, projectRef, before: before.viewport.selection,
      alternate: alternate.viewport.selection, project: project.viewport.selection };
  }
  await alternateRow.click({ timeout: stageTimeoutMs });
  await page.waitForFunction((id) =>
    (document.querySelector('[data-testid="command-selection-readout"]')?.textContent ?? "").includes(id),
  alternateRef.id, { timeout: stageTimeoutMs });
  await projectRow.click({ timeout: stageTimeoutMs });
  await page.waitForFunction((id) =>
    (document.querySelector('[data-testid="command-selection-readout"]')?.textContent ?? "").includes(id),
  projectRef.id, { timeout: stageTimeoutMs });
  return { input: "real tree-row clicks", alternateRef, projectRef, observed: await currentSelectionState(page) };
}

async function moveOrbitUntil(page: Page, box: { x: number; y: number; width: number; height: number },
  gestureStartedAt: number, targetElapsedMs: number, priorIndex: number): Promise<number> {
  let index = priorIndex;
  while (nodePerformance.now() - gestureStartedAt < targetElapsedMs) {
    const elapsed = nodePerformance.now() - gestureStartedAt;
    const x = 0.5 + 0.31 * Math.sin((Math.PI * 2 * elapsed) / 2400);
    const y = 0.5 + 0.23 * Math.sin((Math.PI * 2 * elapsed) / 1700 + Math.PI / 3);
    await page.mouse.move(box.x + box.width * x, box.y + box.height * y);
    index += 1;
    await nodeDelay(Math.max(1, 16 - ((index % 5) === 0 ? 1 : 0)));
  }
  return index;
}

function extractionFailure(error: unknown): any {
  return error instanceof CausalPresentationExtractionError
    ? { status: "FAIL_CAUSAL_EXTRACTION", code: error.code, message: error.message, context: error.context }
    : { status: "FAIL_CAUSAL_EXTRACTION", code: "UNEXPECTED_EXTRACTION_ERROR", message: String(error) };
}

test(`validate exact causal Chromium-reported presentation method for ${phase}`, async ({ page, browserName }, testInfo) => {
  const configuredChromiumExecutable = (testInfo.config.metadata.chromiumExecutable as any) ?? null;
  const configuredChromiumSource = (testInfo.config.metadata.chromiumSourceBinding as any) ?? null;
  const evidenceRoot = requireEvidenceRoot();
  const outputDir = path.join(evidenceRoot, "causal-presentation-preflight");
  await mkdir(outputDir, { recursive: true });
  const runToken = `${phase}.${randomUUID().replaceAll("-", "").slice(0, 20)}`;
  const journal = new CausalPhaseJournal(outputDir, runToken);
  await journal.write("initialized", {
    phase, browserName, pipeCount, sampleNumber, runToken,
    fixtureManifestSha256: process.env.UI_FOUNDATION_MANIFEST_SHA256 ?? null,
    candidateBundleManifestSha256: process.env.UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST_SHA256 ?? null,
    candidateOracleManifestSha256: process.env.UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256 ?? null,
    workload: { pointSettleMs, orbitWarmupMs, orbitMeasuredMs, orbitTrailingMs,
      orbitStartNormalized: ORBIT_START_NORMALIZED, labelsDuringOrbit: "ON" }
  });
  const resultPath = path.join(outputDir, "result.json");
  let setup: any;
  try {
    const candidateDriverBinding = phase === "candidate" ? bindCandidateDriverEntry("causal-presentation-preflight") : null;
    const fixture = await loadFixture(pipeCount);
    const candidateOracleBinding = phase === "candidate" ? await validateCandidateOracleBinding(fixture, pipeCount) : null;
    const probe = fixture.pointOracle.probes.find((entry: any) => entry.sample === sampleNumber);
    if (!probe) throw new Error(`causal point sample ${sampleNumber} is absent`);
    const runtimeOracle = phase === "candidate" ? probe.candidate_runtime : probe.baseline;
    if (runtimeOracle?.actionability?.includes("OUTSIDE") || runtimeOracle?.oracle?.status !== "HIT" ||
        !runtimeOracle.oracle.expectedHitRef) {
      throw new Error(`causal point sample ${sampleNumber} is not an actionable exact oracle hit for ${phase}`);
    }
    const expectedRef = runtimeOracle.oracle.expectedHitRef;
    const expectedLabel = expectedVisibleEntityLabel(fixture.model, expectedRef);
    setup = { candidateDriverBinding, fixture, candidateOracleBinding, probe, runtimeOracle, expectedRef, expectedLabel };
  } catch (error) {
    const failed = { schema: "openpipestress.ui-foundation.causal-presentation-method-validation/v2",
      status: "FAIL_INITIAL_BINDING_OR_FIXTURE_SETUP", phase, browserName, pipeCount, sampleNumber, runToken,
      primaryError: String(error), measuredSampleContribution: 0 };
    await journal.write("failed", failed);
    await writeJson(resultPath, failed);
    throw error;
  }
  const { candidateDriverBinding, fixture, candidateOracleBinding, probe, runtimeOracle, expectedRef, expectedLabel } = setup;
  const pointToken = `${runToken}.point.${sampleNumber}`;
  const orbitToken = `${runToken}.orbit`;
  const rawTracePath = path.join(outputDir, "trace-events.raw.json");
  const result: any = {
    schema: "openpipestress.ui-foundation.causal-presentation-method-validation/v2",
    status: "RUNNING",
    phase,
    browserName,
    pipeCount,
    sampleNumber,
    runToken,
    candidateDriverBinding,
    candidateOracleBinding,
    configuredChromiumExecutable,
    configuredChromiumSource,
    runtimeQualification: "NOT_QUALIFIED",
    pointHitPolicySha256: fixture.pointOracle.point_hit_policy_sha256,
    pointOracleSha256: fixture.pointOracleSha256,
    workload: { pointSettleMs, orbitWarmupMs, orbitMeasuredMs, orbitTrailingMs,
      orbitStartNormalized: ORBIT_START_NORMALIZED, labelsDuringOrbit: "ON" },
    thresholdsUnchanged: true,
    measuredSampleContribution: 0
  };
  let traceCapture: Awaited<ReturnType<typeof beginChromiumCompositorTrace>> | null = null;
  let traceEnded = false;
  let traceSummary: any = null;
  let pointEvidence: any = null;
  let orbitEvidence: any = null;
  let primaryError: unknown = null;
  let selectionVisualOracle: any = null;
  let preparedJournalWritten = false;
  let pointJournalWritten = false;
  let orbitJournalWritten = false;
  let traceJournalWritten = false;
  let activeToken: string | null = null;
  let activeKind: "point-selection" | "orbit" | null = null;
  let orbitPointerDown = false;

  try {
    await installInstrumentation(page, { captureGlobalRaf: false, causalFeedbackMarkers: true });
    await routeModelFixture(page, fixture);
    const documentResponse = await page.goto("/", { waitUntil: "domcontentloaded", timeout: stageTimeoutMs });
    if (phase === "candidate") assertBoundCandidateDocumentResponse(documentResponse);
    result.firstUsable = await waitForFirstUsable(page, fixture, stageTimeoutMs, phase);
    if (phase === "candidate") {
      const before = (await readCandidateDiagnostics(page, true)).snapshot;
      await page.getByTestId("viewport-view-isometric").click({ timeout: stageTimeoutMs });
      await page.getByTestId("viewport-fit-model").click({ timeout: stageTimeoutMs });
      await page.waitForFunction(({ generation, cameraSequence, renderSequence }) => {
        const snapshot = (globalThis as any).__openPipeStressUiDiagnosticsV1?.readCurrent?.();
        return snapshot?.model?.generation === generation && snapshot?.viewport?.generation === generation &&
          snapshot.viewport.camera?.sequence > cameraSequence &&
          snapshot.viewport.mainRender?.submissionSequence > renderSequence;
      }, {
        generation: before.model.generation,
        cameraSequence: before.viewport.camera.sequence,
        renderSequence: before.viewport.mainRender.submissionSequence
      }, { timeout: stageTimeoutMs });
      result.cameraBinding = await validateCandidateMeasuredCameraBinding(page, fixture.pointOracle);
    } else {
      await page.getByTestId("viewport-view-cube").getByRole("button", { name: "Iso" }).click({ timeout: stageTimeoutMs });
      result.cameraBinding = {
        status: "BASELINE_REACHABLE_ISO_RECIPE_APPLIED_CAMERA_STATE_UNOBSERVED",
        sourceBoundRecipe: fixture.pointOracle.baseline_camera
      };
    }
    result.projectSelectionPrecondition = await establishProjectSelection(page, fixture);
    await ensureViewportToggle(page, "toggle-viewport-labels", false, stageTimeoutMs);

    const mainCanvas = await requireUniqueConnectedMainCanvas(page);
    const canvasBox = mainCanvas.box;
    result.mainCanvasIdentity = mainCanvas.identity;
    result.mainCanvasCssBox = canvasBox;
    let pointLocal: { x: number; y: number };
    let candidateBeforePoint: any = null;
    if (phase === "candidate") {
      candidateBeforePoint = (await readCandidateDiagnostics(page, true)).snapshot;
      await writeFile(path.join(outputDir, "point-before-state.json"), `${JSON.stringify({ candidateBeforePoint, probe }, null, 2)}\n`, { flag: "wx" });
      const projection = await projectCandidateAuthoredPoint(page, {
        modelGeneration: candidateBeforePoint.model.generation,
        cameraSequence: candidateBeforePoint.viewport.camera.sequence,
        authoredPoint: { x: probe.authored_anchor[0], y: probe.authored_anchor[1], z: probe.authored_anchor[2] }
      });
      if (projection.status !== "available" || !projection.insideClosedNdc || !projection.insideCanvasCss) {
        throw new Error(`candidate causal point projection is not actionable: ${JSON.stringify(projection)}`);
      }
      if (Math.abs(Number(projection.canvasCss?.width) - canvasBox.width) > 1e-6 ||
          Math.abs(Number(projection.canvasCss?.height) - canvasBox.height) > 1e-6) {
        throw new Error(`candidate projection canvas dimensions differ from the unique connected main HTMLCanvasElement: ${JSON.stringify({ projection: projection.canvasCss, canvasBox })}`);
      }
      pointLocal = { x: projection.canvasCssPoint.x, y: projection.canvasCssPoint.y };
      result.productProjection = projection;
    } else {
      pointLocal = {
        x: ((runtimeOracle.ndc[0] + 1) / 2) * canvasBox.width,
        y: ((1 - runtimeOracle.ndc[1]) / 2) * canvasBox.height
      };
    }
    const point = canvasLocalToClient(pointLocal, canvasBox);
    const clip = phase === "candidate" ? runtimeOracle.visual_plan?.clip : {
      x: Math.max(canvasBox.x, point.x - 24),
      y: Math.max(canvasBox.y, point.y - 24),
      width: Math.min(48, canvasBox.x + canvasBox.width - Math.max(canvasBox.x, point.x - 24)),
      height: Math.min(48, canvasBox.y + canvasBox.height - Math.max(canvasBox.y, point.y - 24))
    };
    selectionVisualOracle = phase === "candidate"
      ? fixture.pointOracle.candidate_preflight?.selection_visual_oracle
      : {
          source: "unchanged baseline PipeViewport selected material 0xf08c22",
          theme: "baseline light",
          srgb: [240, 140, 34],
          tolerancePerChannel: 80,
          minimumQualifyingInteriorPixels: 1
        };
    if (!selectionVisualOracle || !Array.isArray(selectionVisualOracle.srgb)) {
      throw new Error("causal point validation requires the phase-bound selection visual oracle");
    }
    const beforePointImage = path.join(outputDir, "point-highlight-before.png");
    const afterPointImage = path.join(outputDir, "point-highlight-after.png");
    const finalImage = path.join(outputDir, "post-trace-full-page.png");
    const beforeCueCapture = phase === "candidate" ? await captureWinnerCue(page, probe, beforePointImage) : null;
    if (phase === "baseline") await page.screenshot({ path: beforePointImage, clip });
    const beforePointDigest = pngPixelDigest(await readFile(beforePointImage), selectionVisualOracle);
    await page.evaluate((binding) => {
      (globalThis as any).__uifHarness.activePointSample = binding;
    }, { sample: sampleNumber, expectedHitRef: expectedRef, expectedCssPoint: point,
      expectedCanvasLocalPoint: pointLocal, expectedClientPoint: point });
    const pointArming = await armCausalFeedbackMarker(page, {
      token: pointToken,
      phase,
      feedbackKind: "point-selection",
      actionStartEvent: "pointerdown",
      expectedActionTargetTestId: "viewport-canvas",
      maximumFeedbackMarkers: 1,
      actionIdentity: { sample: sampleNumber, expectedRef, expectedCssPoint: point,
        expectedCanvasLocalPoint: pointLocal, expectedClientPoint: point },
      candidateExpectation: phase === "candidate" ? {
        modelGeneration: candidateBeforePoint.model.generation,
        priorRenderSubmissionSequence: candidateBeforePoint.viewport.mainRender.submissionSequence,
        priorActionSequence: candidateBeforePoint.viewport.selection.actionSequence,
        expectedInspectorHeading: `${expectedLabel} — ${expectedRef.type}: ${expectedRef.id}`,
        expectedRef
      } : null,
      baselineExpectation: phase === "baseline" ? {
        expectedRef,
        expectedInspectorHeading: expectedLabel
      } : null
    });
    activeToken = pointToken;
    activeKind = "point-selection";
    const pointHitTarget = await validateMainCanvasHitTarget(page, point);
    result.pointTargetPreflight = pointHitTarget;
    assertMainCanvasHitTarget(pointHitTarget);
    await journal.write("prepared", { expectedRef, expectedLabel, mainCanvasIdentity: mainCanvas.identity,
      canvasBox, pointLocal, pointClient: point,
      pointHitTarget, pointArming, selectionVisualOracle, labels: "OFF_FOR_POINT_PROTOCOL" });
    preparedJournalWritten = true;

    traceCapture = await beginChromiumCompositorTrace(page, "UIF_CAUSAL_PREFLIGHT", rawTracePath);
    await page.mouse.click(point.x, point.y, { button: "left" });
    await nodeDelay(pointSettleMs);
    pointEvidence = await stopCausalFeedbackMarker(page, pointToken);
    activeToken = null;
    activeKind = null;
    await journal.write("point-stopped", { token: pointToken, evidence: pointEvidence });
    pointJournalWritten = true;
    const pointSelection = await currentSelectionState(page, expectedRef);
    const candidateAfterPoint = phase === "candidate" ? (await readCandidateDiagnostics(page, true)).snapshot : null;
    await writeFile(path.join(outputDir, "point-complete-stopped-state.json"), `${JSON.stringify({ candidateAfterPoint, beforeCueCapture, plan: runtimeOracle.visual_plan ?? null, pointEvidence }, null, 2)}\n`, { flag: "wx" });
    assertStoppedCausalEvidence(pointEvidence, "point-selection", true);
    const afterCueCapture = phase === "candidate" ? await captureWinnerCue(page, probe, afterPointImage) : null;
    if (phase === "baseline") await page.screenshot({ path: afterPointImage, clip });
    const pairedCue = phase === "candidate" ? pairedWinnerCueWitness(await readFile(beforePointImage), await readFile(afterPointImage), runtimeOracle.visual_plan, probe, selectionVisualOracle) : null;
    const captureMarkerBound = phase !== "candidate" || (cueActionBindingsStable(beforeCueCapture, afterCueCapture) && pointEvidence.active.feedbackMarkers.length === 1 && pointCaptureMatchesMarker(pointEvidence.active.feedbackMarkers[0], afterCueCapture, expectedRef));
    await writeFile(path.join(outputDir, "point-cue-verdict.json"), `${JSON.stringify({ pairedCue, captureMarkerBound, beforeCueCapture, afterCueCapture }, null, 2)}\n`, { flag: "wx" });
    const afterPointDigest = pngPixelDigest(await readFile(afterPointImage), selectionVisualOracle);
    result.point = {
      token: pointToken,
      arming: pointArming,
      expectedRef,
      expectedLabel,
      pointerCanvasLocalPoint: pointLocal,
      pointerClientPoint: point,
      canvasCssBox: canvasBox,
      clip,
      evidence: pointEvidence,
      selection: pointSelection,
      candidateSnapshot: candidateAfterPoint,
      visual: {
        pairedCue, captureMarkerBound, beforeCueCapture, afterCueCapture,
        before: { path: path.basename(beforePointImage), digest: beforePointDigest },
        after: { path: path.basename(afterPointImage), digest: afterPointDigest }
      }
    };
    await page.evaluate(() => { (globalThis as any).__uifHarness.activePointSample = null; });

    await ensureViewportToggle(page, "toggle-viewport-labels", true, stageTimeoutMs);
    await traceMarker(page, "UIF_CAUSAL_PREFLIGHT:ORBIT_WARMUP_START");
    const candidateBeforeOrbit = phase === "candidate" ? (await readCandidateDiagnostics(page, true)).snapshot : null;
    const orbitMainCanvas = await requireUniqueConnectedMainCanvas(page);
    const orbitCanvasBox = orbitMainCanvas.box;
    const orbitStart = normalizedCanvasPoint(orbitCanvasBox, ORBIT_START_NORMALIZED);
    const orbitArming = await armCausalFeedbackMarker(page, {
      token: orbitToken,
      phase,
      feedbackKind: "orbit",
      actionStartEvent: "pointerdown",
      expectedActionTargetTestId: "viewport-canvas",
      maximumFeedbackMarkers: 1000,
      actionIdentity: { gesture: "frozen-orbit-pointer-path", startNormalized: ORBIT_START_NORMALIZED,
        startClient: orbitStart, warmupMs: orbitWarmupMs, measuredMs: orbitMeasuredMs },
      candidateExpectation: phase === "candidate" ? {
        modelGeneration: candidateBeforeOrbit.model.generation,
        priorRenderSubmissionSequence: candidateBeforeOrbit.viewport.mainRender.submissionSequence,
        priorCameraSequence: candidateBeforeOrbit.viewport.camera.sequence
      } : null,
      baselineExpectation: phase === "baseline" ? {} : null
    });
    activeToken = orbitToken;
    activeKind = "orbit";
    const orbitHitTarget = await validateMainCanvasHitTarget(page, orbitStart);
    result.orbitTargetPreflight = orbitHitTarget;
    assertMainCanvasHitTarget(orbitHitTarget);
    await page.mouse.move(orbitStart.x, orbitStart.y);
    await page.mouse.down({ button: "left" });
    orbitPointerDown = true;
    const gestureStartedAt = nodePerformance.now();
    let moveCount = await moveOrbitUntil(page, orbitCanvasBox, gestureStartedAt, orbitWarmupMs, 0);
    const orbitWarmupEvidence = await readCausalFeedbackMarker(page, orbitToken);
    assertStoppedCausalEvidence(orbitWarmupEvidence, "orbit", false);
    if (phase === "candidate") {
      const candidateAfterWarmup = (await readCandidateDiagnostics(page, true)).snapshot;
      if (candidateAfterWarmup.viewport.camera.sequence <= candidateBeforeOrbit.viewport.camera.sequence ||
          candidateAfterWarmup.viewport.mainRender.submissionSequence <= candidateBeforeOrbit.viewport.mainRender.submissionSequence) {
        throw new Error("orbit warmup did not advance candidate camera and main render before measured window");
      }
    }
    const measuredStartPageClock = await traceMarker(page, "UIF_CAUSAL_PREFLIGHT:MEASURED_START");
    moveCount = await moveOrbitUntil(page, orbitCanvasBox, gestureStartedAt, orbitWarmupMs + orbitMeasuredMs, moveCount);
    const measuredEndPageClock = await traceMarker(page, "UIF_CAUSAL_PREFLIGHT:MEASURED_END");
    await page.mouse.move(orbitCanvasBox.x + orbitCanvasBox.width * 0.51, orbitCanvasBox.y + orbitCanvasBox.height * 0.5);
    moveCount += 1;
    await page.mouse.up({ button: "left" });
    orbitPointerDown = false;
    await nodeDelay(orbitTrailingMs);
    orbitEvidence = await stopCausalFeedbackMarker(page, orbitToken);
    activeToken = null;
    activeKind = null;
    await journal.write("orbit-stopped", { token: orbitToken, evidence: orbitEvidence, orbitHitTarget,
      mainCanvasIdentity: orbitMainCanvas.identity, canvasCssBox: orbitCanvasBox,
      moveCount, measuredWindowPageClock: { start: measuredStartPageClock, end: measuredEndPageClock } });
    orbitJournalWritten = true;
    assertStoppedCausalEvidence(orbitEvidence, "orbit", true);
    result.orbit = {
      token: orbitToken,
      arming: orbitArming,
      hitTarget: orbitHitTarget,
      warmupEvidence: orbitWarmupEvidence,
      evidence: orbitEvidence,
      moveCount,
      mainCanvasIdentity: orbitMainCanvas.identity,
      canvasCssBox: orbitCanvasBox,
      measuredWindowPageClock: { start: measuredStartPageClock, end: measuredEndPageClock },
      trailingCoverageMoveAfterMeasuredEnd: true
    };
    traceSummary = await endChromiumCompositorTrace(page, traceCapture);
    traceEnded = true;
    result.trace = traceSummary;
    await journal.write("trace-finalized", { trace: traceSummary, rawTrace: {
      path: rawTracePath, bytes: traceCapture.rawTraceBytes, sha256: traceCapture.rawTraceSha256,
      complete: traceCapture.rawTraceComplete, completion: traceSummary.tracingComplete
    } });
    traceJournalWritten = true;
    result.chromium = await captureChromiumEnvironment(page);
    // This CDP response belongs to page.context().browser(), the browser that
    // just produced this trace. A configured path alone is not this witness.
    const actualArgs = result.chromium.browserCommandLine?.arguments;
    if (browserName !== "chromium" || !configuredChromiumExecutable || !configuredChromiumSource ||
        !Array.isArray(actualArgs) || !actualArgs.includes("--enable-automation") ||
        typeof actualArgs[0] !== "string" || !path.isAbsolute(actualArgs[0])) {
      throw new Error("causal preflight lacks a strict configured pin or actual-browser command-line witness");
    }
    const actualExecutablePath = await realpath(actualArgs[0]);
    const actualExecutableStat = await stat(actualExecutablePath);
    if (!actualExecutableStat.isFile()) throw new Error("actual Chromium executable is not a regular file");
    const actualExecutableBytes = await readFile(actualExecutablePath);
    result.actualChromiumExecutable = { path: actualExecutablePath, bytes: actualExecutableBytes.length,
      sha256: createHash("sha256").update(actualExecutableBytes).digest("hex") };
    const actualProtocol = result.chromium.browserProtocolVersion;
    if (result.actualChromiumExecutable.path !== configuredChromiumExecutable.executablePath ||
        result.actualChromiumExecutable.bytes !== configuredChromiumExecutable.bytes ||
        result.actualChromiumExecutable.sha256 !== configuredChromiumExecutable.sha256 ||
        result.chromium.browserVersion !== configuredChromiumSource.browserVersion ||
        actualProtocol?.product !== configuredChromiumSource.product ||
        actualProtocol?.revision !== configuredChromiumSource.revision) {
      throw new Error("actual Chromium executable or CDP source does not match the configured strict binding");
    }
    result.runtimeQualification = "PASS_CONFIGURED_EXECUTABLE_AND_CDP_SOURCE_BINDING";
    await page.screenshot({ path: finalImage, fullPage: true });
    result.postTraceCapture = { path: path.basename(finalImage), fullPage: true };
  } catch (error) {
    primaryError = error;
    result.primaryError = String(error);
    if (orbitPointerDown) {
      try {
        await page.mouse.up({ button: "left" });
        orbitPointerDown = false;
        result.interruptedPointerRelease = { status: "PASS_POINTERUP_SENT_BEFORE_PHASE_STOP" };
      } catch (releaseError) {
        result.interruptedPointerRelease = {
          status: "FAIL_POINTERUP_DURING_PRIMARY_ERROR",
          error: String(releaseError)
        };
      }
    }
    if (activeToken !== null && activeKind !== null) {
      try {
        const interruptedEvidence = await stopCausalFeedbackMarker(page, activeToken);
        if (activeKind === "point-selection" && !pointJournalWritten) {
          pointEvidence = interruptedEvidence;
          await journal.write("point-stopped", { token: activeToken, evidence: interruptedEvidence,
            interruptedByPrimaryError: String(error) });
          pointJournalWritten = true;
        }
        if (activeKind === "orbit" && !orbitJournalWritten) {
          orbitEvidence = interruptedEvidence;
          await journal.write("orbit-stopped", { token: activeToken, evidence: interruptedEvidence,
            interruptedByPrimaryError: String(error) });
          orbitJournalWritten = true;
        }
      } catch (captureError) {
        result.phaseCaptureError = String(captureError);
      } finally {
        activeToken = null;
        activeKind = null;
      }
    }
  } finally {
    if (traceCapture !== null && !traceEnded) {
      try {
        traceSummary = await endChromiumCompositorTrace(page, traceCapture);
        traceEnded = true;
        result.trace = traceSummary;
        if (!traceJournalWritten) {
          await journal.write("trace-finalized", { trace: traceSummary, rawTrace: {
            path: rawTracePath, bytes: traceCapture.rawTraceBytes, sha256: traceCapture.rawTraceSha256,
            complete: traceCapture.rawTraceComplete, completion: traceSummary.tracingComplete
          }, finalizedDuringCleanup: true });
          traceJournalWritten = true;
        }
      } catch (error) {
        result.traceEndError = String(error);
        if (primaryError === null) {
          primaryError = error;
          result.primaryError = String(error);
        }
      }
    }
    try {
      result.instrumentationRestore = await restoreCausalFeedbackInstrumentation(page);
      if (result.instrumentationRestore?.status !== "PASS_FULL_RESTORE") {
        const error = new Error(`causal instrumentation restoration is incomplete: ${JSON.stringify(result.instrumentationRestore)}`);
        if (primaryError === null) primaryError = error;
        result.restoreError = String(error);
      }
    } catch (error) {
      result.instrumentationRestore = { status: "FAIL_RESTORE", error: String(error) };
      if (primaryError === null) primaryError = error;
      result.restoreError = String(error);
    }
  }

  if (primaryError === null && traceSummary && traceCapture && pointEvidence && orbitEvidence) {
    try {
    const protocol = result.chromium?.browserProtocolVersion;
    const sourceBinding = {
      ...REQUIRED_CHROMIUM_BINDING,
      browserVersion: result.chromium?.browserVersion,
      revision: protocol?.revision
    };
    result.chromiumSourceBinding = sourceBinding;
    result.pointLineageInventory = inventoryCausalPresentationLineages(traceCapture.events, sourceBinding, pointEvidence);
    result.orbitLineageInventory = inventoryCausalPresentationLineages(traceCapture.events, sourceBinding, orbitEvidence);
    try {
      result.pointExtraction = extractCausalPresentations(traceCapture.events, sourceBinding, pointEvidence);
    } catch (error) {
      result.pointExtraction = extractionFailure(error);
    }
    try {
      result.orbitExtraction = extractCausalPresentations(traceCapture.events, sourceBinding, orbitEvidence);
    } catch (error) {
      result.orbitExtraction = extractionFailure(error);
    }
    if (result.orbitExtraction?.status === "PASS_ALL_CAUSAL_PRESENTATIONS_EXACT_AND_UNAMBIGUOUS") {
      try {
        const measuredWindow = traceSummary.appSurfaceMapping?.measuredWindow;
        result.orbitMeasuredWindow = presentationGapsForMeasuredWindow(result.orbitExtraction, measuredWindow);
      } catch (error) {
        result.orbitMeasuredWindow = extractionFailure(error);
      }
    }
    const pointFeedback = pointEvidence.active?.feedbackMarkers ?? [];
    const pointWindow = {
      start: Number(pointEvidence.active?.actionMarker?.listenerObservedAt),
      end: pointFeedback.length ? Number(pointFeedback.at(-1).callbackCompletedAt) : Number.NaN
    };
    const measuredWindow = result.orbit.measuredWindowPageClock;
    const observerRequirements = (evidence: any, window: { start: number; end: number }) => ({
      minimumRafCallbacks: Math.max(1, (evidence.active?.feedbackMarkers ?? []).filter((marker: any) =>
        marker.callbackCompletedAt >= window.start && marker.callbackEntryAt <= window.end).length),
      feedbackObservations: evidence.active?.feedbackMarkers,
      pointerTransaction: evidence.active?.pointerTransaction,
      actionMarker: evidence.active?.actionMarker,
      actionStartEvent: evidence.active?.actionStartEvent,
      // Exact enclosure covers every observation in the complete stopped action.
      // Registration and both pointer boundaries must exist in the complete
      // stopped action, even when they precede/follow the scored subwindow.
      requiredSourceOperations: ["raf-registration", "pointerdown-capture-listener-total", "pointerup-capture-listener-total"],
      scheduledAdditionalAnimationFrames: 0, // Frozen harness wraps only product RAF; no new RAF is scheduled.
      productObservationRaf: phase === "candidate"
        ? result.point.candidateSnapshot?.viewport?.mainRender?.nextPaintOpportunity : null,
      restorationStatus: result.instrumentationRestore?.status
    });
    result.observerValidity = {
      point: observerValidity(pointEvidence.active?.observerSamples ?? [], pointWindow,
        pointEvidence.active?.overflow?.total, pointEvidence.active?.observerErrors?.length,
        observerRequirements(pointEvidence, pointWindow)),
      orbitMeasured: observerValidity(orbitEvidence.active?.observerSamples ?? [], measuredWindow,
        orbitEvidence.active?.overflow?.total, orbitEvidence.active?.observerErrors?.length,
        observerRequirements(orbitEvidence, measuredWindow)),
      schema: "openpipestress.ui-foundation.instrumented-observer-accounting/v2",
      diagnosticReferences: { p95Ms: 0.5, maximumMs: 2, totalPercent: 3 },
      scheduledAdditionalAnimationFramesByHarness: 0,
      productObservationRaf: phase === "candidate" ? {
        requirement: "nextPaintOpportunity is null after removal of observation-only product RAF",
        observed: result.point.candidateSnapshot?.viewport?.mainRender?.nextPaintOpportunity ??
          (result.point.candidateSnapshot?.viewport?.mainRender?.nextPaintOpportunity === null ? null : "UNAVAILABLE")
      } : { requirement: "baseline has no diagnostics-demand observation RAF", observed: null },
      treatment: "Prospective exact instrumented-workload method: structural observer gates and separate raw overhead diagnostics; tracing, diagnostics/layout reads and wrapper costs are disclosed and never subtracted. Observer clockMeasurement records source-bound uncertainty and unmeasured self-recording tail; a raw pass does not resolve a true-cost ceiling that those bounds straddle."
    };
    const beforeDigest = result.point.visual.before.digest;
    const afterDigest = result.point.visual.after.digest;
    const changed = beforeDigest?.status === "PASS_DECODED_PIXEL_PAYLOAD" &&
      afterDigest?.status === "PASS_DECODED_PIXEL_PAYLOAD" && beforeDigest.pixelSha256 !== afterDigest.pixelSha256;
    const increased = changed && Number(afterDigest.selectionColorWitness?.centralCount) -
      Number(beforeDigest.selectionColorWitness?.centralCount) >=
      Number(afterDigest.selectionColorWitness?.minimumQualifyingInteriorPixels);
    const contrastPass = selectionVisualOracle.requiredLocalContrastRatio === undefined ||
      Number(afterDigest.selectionColorWitness?.localContrastQualifyingCount) >=
      Number(afterDigest.selectionColorWitness?.minimumQualifyingInteriorPixels);
    const candidateSelection = result.point.candidateSnapshot?.viewport?.selection;
    const exactSelection = phase === "candidate"
      ? candidateSelection?.orderedRefs?.length === 1 && exactRef(candidateSelection.orderedRefs[0], expectedRef) &&
        exactRef(candidateSelection.primaryRef, expectedRef) &&
        exactRef(result.point.candidateSnapshot?.viewport?.inspector?.ref, expectedRef)
      : result.point.selection?.ref?.type === expectedRef.type &&
        result.point.selection?.ref?.id === expectedRef.id &&
        result.point.selection?.inspectorContainsExpectedId === true;
    result.pointVisualIdentity = {
      status: exactSelection && (phase === "candidate" ? result.point.visual.pairedCue?.status === "PASS_PAIRED_WINNER_CUE_TRANSITION" && result.point.visual.captureMarkerBound : increased && contrastPass)
        ? "PASS_EXACT_IDENTITY_AND_SOURCE_BOUND_SELECTION_COLOR_IN_PROJECTED_ROI"
        : "FAIL_EXACT_IDENTITY_OR_SOURCE_BOUND_SELECTION_COLOR_IN_PROJECTED_ROI",
      exactSelection,
      sourceBoundSelectionColorCountIncreased: increased,
      sourceBoundLocalContrastPass: contrastPass,
      baselineTypedIdentityLimitation: phase === "baseline"
        ? "Baseline typed DOM identity, source/oracle/visual/property evidence are retained; candidate generation/submission diagnostics are unavailable."
        : null
    };
    const traceComplete = traceCapture.rawTraceComplete && traceCapture.rawTraceSha256 !== null &&
      traceSummary.traceDataLossOccurred === false &&
      traceSummary.markerValidation?.status === "PASS_REQUIRED_MARKERS_PRESENT_ONCE_WITH_FINITE_ORDERED_TIMESTAMPS";
    const extractionPass = result.pointExtraction?.status === "PASS_ALL_CAUSAL_PRESENTATIONS_EXACT_AND_UNAMBIGUOUS" &&
      result.orbitExtraction?.status === "PASS_ALL_CAUSAL_PRESENTATIONS_EXACT_AND_UNAMBIGUOUS" &&
      result.orbitMeasuredWindow?.status === "PASS_COMPLETE_MEASURED_WINDOW_CAUSAL_PRESENTATION_COVERAGE";
    result.measurementClaim = {
      version: "instrumented-workload/v2",
      scope: "Only the exact hash-bound instrumented workload under recorded conditions; no uninstrumented counterfactual or product-only speedup claim.",
      productTargets: "Unchanged; the prescribed five-run cohort and conservative metric bounds remain required. This point/short-orbit preflight does not score targets.",
      actionEndpointPolicy: "Capture-listener observation to exact Chromium-reported presentation; future point scoring must consume actionToPresentationIntervalMs.upper without observer-cost subtraction.",
      historicalEvidence: "No prior failed run is reclassified or pooled into prospective results."
    };
    result.status = instrumentedPreflightPass({ traceComplete, extractionPass,
      pointObserver: result.observerValidity.point, orbitObserver: result.observerValidity.orbitMeasured,
      pointVisualIdentityStatus: result.pointVisualIdentity.status })
      ? "PASS_CAUSAL_PRESENTATION_METHOD_VALIDATION_ONLY"
      : "FAIL_CAUSAL_PRESENTATION_METHOD_VALIDATION_RAW_EVIDENCE_PRESERVED";
    await journal.write("derived", { status: result.status, pointExtraction: result.pointExtraction,
      orbitExtraction: result.orbitExtraction, pointLineageInventory: result.pointLineageInventory,
      orbitLineageInventory: result.orbitLineageInventory, orbitMeasuredWindow: result.orbitMeasuredWindow,
      observerValidity: result.observerValidity, pointVisualIdentity: result.pointVisualIdentity });
    } catch (error) {
      primaryError = error;
      result.primaryError = String(error);
      result.derivationError = String(error);
    }
  }
  if (result.status === "RUNNING") result.status = "FAIL_METHOD_VALIDATION_RAW_EVIDENCE_PRESERVED";
  if (primaryError !== null || result.status !== "PASS_CAUSAL_PRESENTATION_METHOD_VALIDATION_ONLY") {
    try {
      await journal.write("failed", { status: result.status, primaryError: primaryError === null ? null : String(primaryError),
        completedStages: { preparedJournalWritten, pointJournalWritten, orbitJournalWritten, traceJournalWritten },
        pointTargetPreflight: result.pointTargetPreflight ?? null,
        orbitTargetPreflight: result.orbitTargetPreflight ?? null,
        interruptedPointerRelease: result.interruptedPointerRelease ?? null,
        phaseCaptureError: result.phaseCaptureError ?? null,
        traceEndError: result.traceEndError ?? null, instrumentationRestore: result.instrumentationRestore ?? null });
    } catch (error) {
      result.journalFailure = String(error);
      if (primaryError === null) primaryError = error;
    }
  }
  try {
    await writeJson(resultPath, result);
  } catch (error) {
    if (primaryError === null) primaryError = error;
    result.resultWriteError = String(error);
  }
  if (primaryError !== null) throw primaryError;
  if (result.status !== "PASS_CAUSAL_PRESENTATION_METHOD_VALIDATION_ONLY") {
    throw new Error(`causal presentation method preflight did not qualify: ${result.status}`);
  }
});


if (phase === "candidate") {
  test("focused complete successor assignment box DOM filter and both orbit entries", async ({ page }) => {
    const fixture = await loadFixture(1000);
    const binding = bindCandidateDriverEntry("candidate-causal-presentation-preflight");
    const runId = `focused-v20-${randomUUID()}`;
    const expected = await performanceExpectation(fixture, binding, { fixtureSize: 1000, runNumber: 1,
      runId, sessionId: randomUUID() });
    await runCandidateCausalPerformance(page, fixture, expected, path.join(requireEvidenceRoot(), "focused-full-v20"),
      { focused: true, timeoutMs: stageTimeoutMs, binding });
  });
}


if (phase === "candidate") {
  test("ordinary box helper binds real product unavailable empty and subsequent nonempty publications", async ({ page }) => {
    const fixture = await loadFixture(1000), binding = bindCandidateDriverEntry("candidate-causal-presentation-preflight");
    const expected = await performanceExpectation(fixture, binding, { fixtureSize: 1000, runNumber: 1,
      runId: `ordinary-box-${randomUUID()}`, sessionId: randomUUID() });
    const samples = focusedBoxSamples(fixture); // Frozen independent sample1/sample7, before any observed result.
    const nonemptyBox = fixture.pointOracle.candidate_box_selection.samples.find((box: any) => box.sample === 7);
    const roiProbe = fixture.pointOracle.probes.find((probe: any) => {
      const ref = probe.candidate_runtime?.oracle?.expectedHitRef;
      return ref?.type === "pipe" && nonemptyBox.orderedRefs.some((member: any) => member.type === ref.type && member.id === ref.id);
    });
    if (!roiProbe) throw new Error("frozen box7 has no independently visible point-oracle pipe");
    const visual = fixture.pointOracle.candidate_preflight?.selection_visual_oracle;
    if (!visual || !Array.isArray(visual.srgb) || visual.minimumQualifyingInteriorPixels < 1) throw new Error("Box visual oracle unavailable");
    const captures: any[] = [];
    const directory = path.join(requireEvidenceRoot(), "ordinary-product-box-union");
    await mkdir(directory);
    await installInstrumentation(page, { captureGlobalRaf: false, causalFeedbackMarkers: true });
    let failure: unknown = null;
    try {
      await routeModelFixture(page, fixture);
      assertBoundCandidateDocumentResponse(await page.goto("/", { waitUntil: "domcontentloaded", timeout: stageTimeoutMs }));
      await waitForFirstUsable(page, fixture, stageTimeoutMs, "candidate");
      const initial = (await readCandidateDiagnostics(page, true)).snapshot;
      await writeJson(path.join(directory, "actual-initial.json"), initial);
      expect(initial.viewport.box).toEqual({ status: "unavailable" });
      expect(validatedPriorBoxBaseline(initial).kind).toBe("unavailable");
      const project = { type: "project", id: fixture.model.project.id };
      await ensureViewportToggle(page, "toggle-viewport-labels", false, stageTimeoutMs);
      const captureBound = async (name: string, callback: any) => {
        const file = path.join(directory, `${name}.png`), domBefore = await readBoxVisibleIdentity(page);
        const capture = await captureWinnerCue(page, roiProbe, file), domAfter = await readBoxVisibleIdentity(page);
        const marker = callback.active.feedbackMarkers[0];
        const noDrift = boxCallbackMatchesSnapshot(marker, capture.before) && JSON.stringify(domBefore) === JSON.stringify(domAfter);
        const record = { name, file, capture, domBefore, domAfter, marker, noDrift, roiProbe,
          qualification: "CALLBACK_AND_VISUAL_ONLY_NO_COMPOSITOR_TIMING" };
        await writeJson(path.join(directory, `${name}.binding.json`), record);
        if (!noDrift) throw new Error("Box cue capture drifted from qualified submission");
        captures.push(record); return record;
      };
      for (const sample of samples) {
        const tree = page.getByTestId("model-tree-virtual");
        await tree.focus(); await tree.press("Home");
        const prior = (await readCandidateDiagnostics(page)).snapshot;
        await page.getByTestId(treeRowTestId(project, "candidate")).click();
        const selected = await waitForCandidateExclusiveSelection(page, project, prior.model.generation,
          prior.viewport.selection.actionSequence, prior.viewport.mainRender.submissionSequence, stageTimeoutMs, true);
        await writeJson(path.join(directory, `project-before-${sample.sample}.json`), selected);
        expect(selected.viewport.selection.orderedRefs).toEqual([project]);
        expect(selected.viewport.selection.primaryRef).toEqual(project);
        expect(selected.viewport.inspector.ref).toEqual(project);
        await page.getByTestId("viewport-view-isometric").click();
        await page.getByTestId("viewport-fit-model").click();
        await nodeDelay(500);
        await validateCandidateMeasuredCameraBinding(page, fixture.pointOracle);
        const box = fixture.pointOracle.candidate_box_selection.samples.find((s: any) => s.sample === sample.sample);
        const record = await measureCanvasBoxSelection(page, sample, box, stageTimeoutMs,
          path.join(directory, `box-${sample.sample}.png`), fixture.model.project.id,
          expectedBoxInspectorHeading(fixture.model, box.orderedRefs, box.primaryRef, fixture.model.project.id),
          `ordinary-box-${sample.sample}`);
        await writeJson(path.join(directory, `result-${sample.sample}.json`), record);
        expect(record.status).toBe("CANDIDATE_EXPECTED_BOX_SELECTION_AND_PROPERTY_OBSERVED_CAPTURED");
        const observed = record.candidateGenerationEvidence;
        expect(validatedPriorBoxBaseline({ model: { generation: observed.before.modelGeneration },
          viewport: { generation: observed.before.modelGeneration, box: observed.before.box } }).kind)
          .toBe(sample.sample === 1 ? "unavailable" : "active");
        expect(observed.after.selection.orderedRefs).toEqual(box.orderedRefs);
        expect(observed.after.selection.primaryRef).toEqual(box.primaryRef);
        expect(observed.after.inspector.ref).toEqual(box.primaryRef ?? project);
        await validateCandidateMeasuredCameraBinding(page, fixture.pointOracle);
        await captureBound(`box-${sample.sample}-roi`, record.callbackProof);
      }
      // Extra control only: no Project reset between the nonempty state and this
      // original empty gesture. This is not a helper/cohort sample or full segment.
      const sample = samples[0], empty = fixture.pointOracle.candidate_box_selection.samples.find((box: any) => box.sample === 1);
      await page.getByTestId("viewport-selection-filter").selectOption({ label: "All" });
      const before = (await readCandidateDiagnostics(page, true)).snapshot;
      const prior = validatedPriorBoxBaseline(before), canvas = await requireUniqueConnectedMainCanvas(page);
      const start = normalizedCanvasPoint(canvas.box, { x: sample.start_normalized[0], y: sample.start_normalized[1] });
      const end = normalizedCanvasPoint(canvas.box, { x: sample.end_normalized[0], y: sample.end_normalized[1] });
      const resourceGeneration = before.viewport.mainRender.selectionPresentation.resourceGeneration;
      const expectedInspectorHeading = expectedBoxInspectorHeading(fixture.model, empty.orderedRefs, empty.primaryRef, project.id);
      const token = "empty-after-nonempty-control";
      await writeJson(path.join(directory, "empty-after-nonempty-before.json"), { before, expected: empty, sample, start, end, canvas });
      await armCausalFeedbackMarker(page, { token, phase: "candidate", feedbackKind: "box-selection", actionStartEvent: "pointerup",
        expectedActionTargetTestId: "viewport-canvas", maximumFeedbackMarkers: 1, actionIdentity: { sample: 1, start, end, controlOnly: true },
        candidateExpectation: { modelGeneration: before.model.generation, priorRenderSubmissionSequence: before.viewport.mainRender.submissionSequence,
          priorBoxActionSequence: prior.actionSequence, priorBoxBaseline: prior, resourceGeneration, expectedInspectorHeading,
          direction: empty.direction, filter: empty.filter, orderedRefs: empty.orderedRefs, primaryRef: empty.primaryRef, projectId: project.id } });
      assertMainCanvasHitTarget(await validateMainCanvasHitTarget(page, start));
      assertMainCanvasHitTarget(await validateMainCanvasHitTarget(page, end));
      let dragError: string | null = null;
      try {
        await page.mouse.move(start.x, start.y); await page.mouse.down({ button: "left" });
        await page.mouse.move(end.x, end.y, { steps: 8 }); await page.mouse.up({ button: "left" });
        await nodeDelay(500);
      } catch (error) { dragError = String(error); }
      const callback = await stopCausalFeedbackMarker(page, token);
      await writeJson(path.join(directory, "empty-after-nonempty-callback.json"), callback);
      const after = (await readCandidateDiagnostics(page, true)).snapshot;
      await persistBoxPostcondition(path.join(directory, "empty-after-nonempty-stopped.json"), { before, after, expected: empty, prior,
        priorRender: before.viewport.mainRender.submissionSequence, resourceGeneration, expectedInspectorHeading, projectId: project.id,
        pointerEvents: { pointerup: callback.active.pointerTransaction.up }, actionIdentity: { sample: 1, controlOnly: true }, canvas,
        dom: await readBoxVisibleIdentity(page), primaryError: dragError ?? (callback.active.feedbackMarkers.length === 1 ? null : "empty control lacks unique callback") });
      await validateCandidateMeasuredCameraBinding(page, fixture.pointOracle);
      await captureBound("empty-after-nonempty-roi", callback);
      const added = pairedWinnerCueWitness(await readFile(captures[0].file), await readFile(captures[1].file), roiProbe.candidate_runtime.visual_plan, roiProbe, visual);
      const removed = pairedWinnerCueWitness(await readFile(captures[2].file), await readFile(captures[1].file), roiProbe.candidate_runtime.visual_plan, roiProbe, visual);
      await writeJson(path.join(directory, "box-visual-verdict.json"), { added, removed, visual,
        roiExpectedRef: roiProbe.candidate_runtime.oracle.expectedHitRef, qualification: "CALLBACK_AND_VISUAL_ONLY_NO_COMPOSITOR_TIMING", cohortSamples: 0 });
      expect(added.status).toBe("PASS_PAIRED_WINNER_CUE_TRANSITION"); expect(removed.status).toBe("PASS_PAIRED_WINNER_CUE_TRANSITION");
      await performanceExpectation(fixture, binding, expected);
    } catch (error) { failure = error; }
    finally {
      let restore: any;
      try { restore = await restoreCausalFeedbackInstrumentation(page); }
      catch (error) { restore = { status: "FAIL_RESTORE", error: String(error) }; }
      const primaryError = failure === null ? null : String(failure);
      if (restore.status !== "PASS_FULL_RESTORE" && failure === null) failure = new Error("ordinary box instrumentation restoration failed");
      await writeJson(path.join(directory, "restore-and-status.json"), { restore, primaryError,
        error: failure === null ? null : String(failure), qualification: "ORDINARY_HELPER_INTEGRATION_ONLY_ZERO_COHORT_SAMPLES" });
    }
    if (failure !== null) throw failure;
  });
}
