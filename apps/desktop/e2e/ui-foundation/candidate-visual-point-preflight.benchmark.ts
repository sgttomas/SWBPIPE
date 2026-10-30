import { mkdir } from "node:fs/promises";
import path from "node:path";
import { test } from "@playwright/test";
import {
  ensureViewportToggle,
  expectedVisibleEntityLabel,
  installInstrumentation,
  loadFixture,
  measureCanvasPointSelection,
  readCandidateDiagnostics,
  requireEvidenceRoot,
  routeModelFixture,
  validateCandidateMeasuredCameraBinding,
  validateCandidateOracleBinding,
  waitForFirstUsable,
  writeJson
} from "./benchmark-harness";
import { assertBoundCandidateDocumentResponse, bindCandidateDriverEntry } from "./candidate-server-response";

const candidateDriverBinding = bindCandidateDriverEntry("candidate-visual-point-preflight");

test("untimed candidate V4 visual point preflight on three frozen probes", async ({ page }) => {
  const evidenceRoot = requireEvidenceRoot();
  const outputDir = path.join(evidenceRoot, "candidate-visual-point-preflight");
  await mkdir(outputDir, { recursive: true });
  const fixture = await loadFixture(1_000);
  const oracleBinding = await validateCandidateOracleBinding(fixture, 1_000);
  await installInstrumentation(page, { captureGlobalRaf: false });
  await routeModelFixture(page, fixture);
  const documentResponse = await page.goto("/", { waitUntil: "domcontentloaded" });
  assertBoundCandidateDocumentResponse(documentResponse);
  await waitForFirstUsable(page, fixture, 180_000, "candidate");
  const before = (await readCandidateDiagnostics(page, true)).snapshot;
  await page.getByTestId("viewport-view-isometric").click();
  await page.getByTestId("viewport-fit-model").click();
  await page.waitForFunction(({ generation, cameraSequence, renderSequence }) => {
    const snapshot = (globalThis as any).__openPipeStressUiDiagnosticsV1?.readCurrent?.();
    return snapshot?.model?.generation === generation && snapshot?.viewport?.generation === generation &&
      snapshot.viewport.camera?.sequence > cameraSequence &&
      snapshot.viewport.mainRender?.generation === generation &&
      snapshot.viewport.mainRender?.submissionSequence > renderSequence;
  }, {
    generation: before.model.generation,
    cameraSequence: before.viewport.camera.sequence,
    renderSequence: before.viewport.mainRender.submissionSequence
  }, { timeout: 30_000 });
  const cameraBinding = await validateCandidateMeasuredCameraBinding(page, fixture.pointOracle);
  await ensureViewportToggle(page, "toggle-viewport-labels", false, 30_000);
  // Open the identity disclosure after display setup; its popup covers view controls.
  await page.getByTestId("command-bar").locator(".command-context > summary").click();
  await page.getByTestId("command-selection-readout").waitFor({ state: "visible", timeout: 30_000 });
  const selectionVisualOracle = fixture.pointOracle?.candidate_preflight?.selection_visual_oracle;
  if (selectionVisualOracle?.sourceSha256 !== "009b27db887e3de224f72d0df221966fbaaa0305de218ee20848050ac6a1be1b") {
    throw new Error("candidate visual preflight is not bound to VISUAL_TOKENS_V4");
  }
  const records = [];
  for (const sample of fixture.samples.point_selection.slice(0, 3)) {
    const oracleProbe = fixture.pointOracle.probes.find((probe: any) => probe.sample === sample.sample);
    if (!oracleProbe?.candidate_runtime?.oracle?.expectedHitRef) throw new Error(`candidate sample ${sample.sample} has no expected hit`);
    records.push(await measureCanvasPointSelection(
      page,
      sample,
      oracleProbe,
      30_000,
      path.join(outputDir, `candidate-visual-point-sample-${String(sample.sample).padStart(3, "0")}.png`),
      "candidate",
      fixture.model.project.id,
      expectedVisibleEntityLabel(fixture.model, oracleProbe.candidate_runtime.oracle.expectedHitRef),
      selectionVisualOracle
    ));
  }
  await page.getByTestId("command-bar").locator(".command-context > summary").click();
  await page.getByTestId("command-selection-readout").waitFor({ state: "hidden", timeout: 30_000 });
  await ensureViewportToggle(page, "toggle-viewport-labels", true, 30_000);
  const failed = records.filter((record) => record.status !== "CANDIDATE_EXPECTED_SELECTION_AND_PROPERTY_OBSERVED_CAPTURED");
  await writeJson(path.join(outputDir, "result.json"), {
    schema: "openpipestress.ui-foundation.candidate-visual-point-preflight/v1",
    status: failed.length === 0 ? "PASS_THREE_EXACT_TYPED_V4_VISUAL_PROBES" : "FAIL_VISUAL_OR_TYPED_POINT_PREFLIGHT",
    measuredSampleContribution: 0,
    candidateDriverBinding,
    oracleBinding,
    cameraBinding,
    visualTokenSha256: selectionVisualOracle.sourceSha256,
    records,
    claim: "Untimed method/visual feasibility only. Screenshot bounds do not establish the <=100ms presented-feedback target."
  });
  if (failed.length) throw new Error(`candidate visual point preflight failed ${failed.length}/3 probes`);
});
