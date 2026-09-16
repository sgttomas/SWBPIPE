import { createHash } from "node:crypto";
import { mkdir } from "node:fs/promises";
import path from "node:path";
import { test } from "@playwright/test";
import {
  ensureViewportToggle,
  installInstrumentation,
  loadFixture,
  projectCandidateAuthoredPoint,
  readCandidateDiagnostics,
  requireEvidenceRoot,
  routeModelFixture,
  waitForFirstUsable,
  writeJson
} from "./benchmark-harness";
import { assertBoundCandidateDocumentResponse, bindCandidateDriverEntry } from "./candidate-server-response";

const evidenceRoot = requireEvidenceRoot();
const candidateDriverBinding = bindCandidateDriverEntry("candidate-camera-preflight");
if (process.env.UI_FOUNDATION_CANDIDATE_ORACLE_DIR) {
  throw new Error("candidate camera preflight must use the static probe oracle; unset UI_FOUNDATION_CANDIDATE_ORACLE_DIR");
}
const recipeName = process.env.UI_FOUNDATION_CAMERA_RECIPE ?? "isometric_then_fit_model";
if (recipeName !== "isometric_then_fit_model") {
  throw new Error("UI_FOUNDATION_CAMERA_RECIPE must be isometric_then_fit_model for the frozen candidate run");
}
const stageTimeoutMs = Number(process.env.UI_FOUNDATION_PREFLIGHT_STAGE_TIMEOUT_MS ?? 60_000);
if (!Number.isFinite(stageTimeoutMs) || stageTimeoutMs < 5_000 || stageTimeoutMs > 120_000) {
  throw new Error("UI_FOUNDATION_PREFLIGHT_STAGE_TIMEOUT_MS must be from 5000 through 120000");
}

test.describe.serial("untimed candidate camera, observer, and point-actionability preflight", () => {
  for (const pipeCount of [1_000, 10_000]) {
    test(`candidate N=${pipeCount} real standard-command camera preflight`, async ({ page, browserName }) => {
      const fixture = await loadFixture(pipeCount);
      const outputDir = path.join(evidenceRoot, "candidate-camera-preflight");
      await mkdir(outputDir, { recursive: true });
      const captureCheckpoint = async (label: string, error: unknown = null) => {
        const screenshotPath = path.join(outputDir, `candidate-camera-preflight-${pipeCount}-${label}.png`);
        let screenshotError: string | null = null;
        try {
          await page.screenshot({ path: screenshotPath, fullPage: true });
        } catch (captureError) {
          screenshotError = String(captureError);
        }
        const state = await page.evaluate(({ expectedPipeCount }) => {
          const host = document.querySelector('[data-testid="viewport-canvas"]');
          const canvas = host instanceof HTMLCanvasElement ? host : host?.querySelector("canvas");
          const summary = document.querySelector('[data-testid="model-tree-filter-summary"]')?.textContent?.trim() ?? null;
          const diagnostics = (globalThis as any).__openPipeStressUiDiagnosticsV1?.readCurrent?.() ?? null;
          return {
            expectedPipeCount,
            location: location.href,
            title: document.title,
            readyState: document.readyState,
            summary,
            hostTag: host?.tagName ?? null,
            canvasPresent: canvas instanceof HTMLCanvasElement,
            canvasRect: canvas instanceof HTMLCanvasElement ? canvas.getBoundingClientRect().toJSON() : null,
            appTheme: document.querySelector(".app-shell")?.getAttribute("data-theme") ?? null,
            diagnostics
          };
        }, { expectedPipeCount: pipeCount }).catch((stateError) => ({ stateError: String(stateError) }));
        await writeJson(path.join(outputDir, `candidate-camera-preflight-${pipeCount}-${label}.json`), {
          schema: "openpipestress.ui-foundation.candidate-camera-checkpoint/v1",
          pipeCount,
          label,
          capturedAt: new Date().toISOString(),
          screenshot: { path: path.basename(screenshotPath), fullPage: true, error: screenshotError },
          state,
          triggeringError: error === null ? null : String(error)
        });
      };
      await installInstrumentation(page, { captureGlobalRaf: false });
      await routeModelFixture(page, fixture);
      const documentResponse = await page.goto("/", { waitUntil: "domcontentloaded", timeout: stageTimeoutMs });
      assertBoundCandidateDocumentResponse(documentResponse);
      await captureCheckpoint("after-navigation");
      try {
        await waitForFirstUsable(page, fixture, stageTimeoutMs, "candidate");
      } catch (error) {
        await captureCheckpoint("wait-for-first-usable-failure", error);
        throw error;
      }
      const before = (await readCandidateDiagnostics(page, true)).snapshot;
      await page.getByTestId("viewport-view-isometric").click({ timeout: stageTimeoutMs });
      await page.getByTestId("viewport-fit-model").click({ timeout: stageTimeoutMs });
      await page.waitForFunction(({ generation, priorCameraSequence, priorRenderSequence }) => {
        const snapshot = (globalThis as any).__openPipeStressUiDiagnosticsV1?.readCurrent?.();
        return snapshot?.model?.generation === generation &&
          snapshot?.viewport?.generation === generation &&
          snapshot.viewport.camera?.sequence > priorCameraSequence &&
          snapshot.viewport.mainRender?.submissionSequence > priorRenderSequence &&
          snapshot.viewport.mainRender?.generation === generation &&
          typeof snapshot.viewport.mainRender?.submittedAt === "number";
      }, {
        generation: before.model.generation,
        priorCameraSequence: before.viewport.camera.sequence,
        priorRenderSequence: before.viewport.mainRender.submissionSequence
      }, { timeout: stageTimeoutMs });
      await ensureViewportToggle(page, "toggle-viewport-labels", false, 30_000);
      const after = (await readCandidateDiagnostics(page, true)).snapshot;
      const visualStyleReadback = await page.evaluate(() => {
        const shell = document.querySelector(".app-shell");
        if (!(shell instanceof HTMLElement)) return { status: "UNAVAILABLE_APP_SHELL" };
        const style = getComputedStyle(shell);
        return {
          status: "AVAILABLE",
          resolvedTheme: shell.dataset.theme ?? null,
          themePreference: shell.dataset.themePreference ?? null,
          viewportSelectionGeometry: style.getPropertyValue("--ui-viewport-selection-geometry").trim(),
          viewportScene: style.getPropertyValue("--ui-canvas").trim()
        };
      });
      if (after.model.identityHash !== before.model.identityHash || after.model.generation !== before.model.generation) {
        throw new Error("standard camera commands changed candidate model identity or generation");
      }
      const projectionCrosschecks = [];
      for (const probe of fixture.pointOracle.probes) {
        projectionCrosschecks.push({
          sample: probe.sample,
          probeAnchorRef: probe.probe_anchor_ref,
          authoredAnchor: probe.authored_anchor,
          result: await projectCandidateAuthoredPoint(page, {
            modelGeneration: after.model.generation,
            cameraSequence: after.viewport.camera.sequence,
            authoredPoint: { x: probe.authored_anchor[0], y: probe.authored_anchor[1], z: probe.authored_anchor[2] }
          })
        });
      }
      const actionableCount = projectionCrosschecks.filter(({ result }) =>
        result.status === "available" && result.insideClosedNdc && result.insideCanvasCss).length;
      const screenshotPath = path.join(outputDir, `candidate-camera-preflight-${pipeCount}.png`);
      await page.screenshot({ path: screenshotPath, fullPage: true });
      const postCaptureAt = await page.evaluate(() => performance.now());
      const record = {
        schema: "openpipestress.ui-foundation.candidate-camera-preflight/v1",
        status: actionableCount === 200 ? "PASS_ALL_200_PRODUCT_PROJECTIONS_ACTIONABLE" : "FAIL_NOT_ALL_200_PRODUCT_PROJECTIONS_ACTIONABLE",
        pipeCount,
        browserName,
        measuredSampleContribution: 0,
        standardCommandRecipe: ["Isometric", "Fit Model"],
        prohibitedPaths: ["hidden camera mutation", "synthetic event dispatch", "benchmark-only camera control"],
        pointHitPolicySha256: fixture.pointOracle.point_hit_policy_sha256,
        fixtureManifestSha256: candidateDriverBinding.fixtureManifestSha256,
        candidateDriverBinding,
        modelSha256: createHash("sha256").update(fixture.bytes).digest("hex"),
        sourcePointOracleSha256: fixture.pointOracleSha256,
        diagnostics: after,
        visualStyleReadback,
        actionableCount,
        projectionCrosschecks,
        browserCapture: { path: path.basename(screenshotPath), postCaptureAt },
        claim: "Untimed camera/observer/driver preflight only; no result contributes to measured p95. Independent oracle freezing is a separate offline step."
      };
      await writeJson(path.join(outputDir, `candidate-camera-preflight-${pipeCount}.json`), record);
      if (actionableCount !== 200) throw new Error(`candidate camera preflight has ${actionableCount}/200 actionable probes`);
    });
  }
});
