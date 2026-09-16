import { mkdir } from "node:fs/promises";
import path from "node:path";
import { test } from "@playwright/test";
import { beginChromiumCompositorTrace, endChromiumCompositorTrace, traceMarker } from "./chromium-compositor-trace";
import { assertBoundCandidateDocumentResponse, bindCandidateDriverEntry } from "./candidate-server-response";
import {
  benchmarkPhase,
  ensureViewportToggle,
  installInstrumentation,
  loadFixture,
  measureOrbit,
  readCandidateDiagnostics,
  requireEvidenceRoot,
  routeModelFixture,
  waitForFirstUsable,
  writeJson
} from "./benchmark-harness";

const phase = benchmarkPhase();
const candidateDriverBinding = phase === "candidate" ? bindCandidateDriverEntry("candidate-compositor-preflight") : null;
const stageTimeoutMs = Number(process.env.UI_FOUNDATION_PREFLIGHT_STAGE_TIMEOUT_MS ?? 60_000);
if (!Number.isFinite(stageTimeoutMs) || stageTimeoutMs < 5_000 || stageTimeoutMs > 60_000) {
  throw new Error("UI_FOUNDATION_PREFLIGHT_STAGE_TIMEOUT_MS must be from 5000 through 60000");
}

test(`inventory Chromium compositor trace categories and events for ${phase} without claiming qualification`, async ({ page }) => {
  const evidenceRoot = requireEvidenceRoot();
  const outputDir = path.join(evidenceRoot, "compositor-trace-preflight");
  await mkdir(outputDir, { recursive: true });
  const fixture = await loadFixture(1000);
  await installInstrumentation(page, { captureGlobalRaf: false });
  await routeModelFixture(page, fixture);
  const documentResponse = await page.goto("/", { waitUntil: "domcontentloaded", timeout: stageTimeoutMs });
  if (phase === "candidate") assertBoundCandidateDocumentResponse(documentResponse);
  await waitForFirstUsable(page, fixture, stageTimeoutMs, phase);
  let candidateCameraSnapshot: any = null;
  if (phase === "candidate") {
    const before = (await readCandidateDiagnostics(page, true)).snapshot;
    await page.getByTestId("viewport-view-isometric").click({ timeout: stageTimeoutMs });
    await page.getByTestId("viewport-fit-model").click({ timeout: stageTimeoutMs });
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
    candidateCameraSnapshot = (await readCandidateDiagnostics(page, true)).snapshot;
  } else {
    await page.getByTestId("viewport-view-cube").getByRole("button", { name: "Iso" }).click({ timeout: stageTimeoutMs });
  }
  await ensureViewportToggle(page, "toggle-viewport-labels", true, 30_000);

  const browser = page.context().browser();
  let systemInfo: any = { status: "UNAVAILABLE" };
  let processInfo: any = { status: "UNAVAILABLE" };
  let browserProtocolVersion: any = { status: "UNAVAILABLE" };
  let browserCommandLine: any = { status: "UNAVAILABLE" };
  if (browser) {
    try {
      const browserSession = await browser.newBrowserCDPSession();
      const safeSend = async (method: string): Promise<any> => {
        try {
          return await browserSession.send(method as any);
        } catch (error) {
          return { status: "UNAVAILABLE", method, error: String(error) };
        }
      };
      systemInfo = await safeSend("SystemInfo.getInfo");
      processInfo = await safeSend("SystemInfo.getProcessInfo");
      browserProtocolVersion = await safeSend("Browser.getVersion");
      browserCommandLine = await safeSend("Browser.getBrowserCommandLine");
      await browserSession.detach();
    } catch (error) {
      systemInfo = { status: "UNAVAILABLE", error: String(error) };
    }
  }
  const webgl = await page.evaluate(() => {
    const host = document.querySelector('[data-testid="viewport-canvas"]');
    const canvas = host instanceof HTMLCanvasElement ? host : host?.querySelector("canvas");
    if (!(canvas instanceof HTMLCanvasElement)) {
      return { status: "UNAVAILABLE", reason: "viewport host has no HTMLCanvasElement descendant" };
    }
    const gl = canvas.getContext("webgl2") ?? canvas.getContext("webgl");
    if (!gl) return { status: "UNAVAILABLE" };
    const debug = gl.getExtension("WEBGL_debug_renderer_info");
    return {
      status: "AVAILABLE",
      vendor: gl.getParameter(gl.VENDOR),
      renderer: gl.getParameter(gl.RENDERER),
      unmaskedVendor: debug ? gl.getParameter(debug.UNMASKED_VENDOR_WEBGL) : null,
      unmaskedRenderer: debug ? gl.getParameter(debug.UNMASKED_RENDERER_WEBGL) : null
    };
  });

  const rawTracePath = path.join(outputDir, "trace-events.raw.json");
  const trace = await beginChromiumCompositorTrace(page, "UIF_COMPOSITOR_PREFLIGHT", rawTracePath);
  const warmupStartPageClock = await traceMarker(page, "UIF_COMPOSITOR_PREFLIGHT:ORBIT_WARMUP_START");
  const warmupOrbit = await measureOrbit(page, 0, 500);
  const measuredStartPageClock = await traceMarker(page, "UIF_COMPOSITOR_PREFLIGHT:MEASURED_START");
  const orbit = await measureOrbit(page, 0, 2_000);
  const measuredEndPageClock = await traceMarker(page, "UIF_COMPOSITOR_PREFLIGHT:MEASURED_END");
  const summary = await endChromiumCompositorTrace(page, trace);
  await writeJson(path.join(outputDir, "summary.json"), {
    ...summary,
    phase,
    candidateDriverBinding,
    warmupStartPageClock,
    warmupOrbitGlobalRafProxy: warmupOrbit,
    measuredWindowPageClock: { start: measuredStartPageClock, end: measuredEndPageClock },
    orbitGlobalRafProxy: orbit,
    browserVersion: browser?.version() ?? null,
    browserProtocolVersion,
    browserCommandLine,
    candidateCameraSnapshot,
    webgl,
    systemInfo,
    processInfo,
    qualification: "N/A_PREFLIGHT_ONLY"
  });
  if (!trace.rawTraceComplete || trace.rawTraceSha256 === null) {
    throw new Error("Chromium compositor preflight did not durably retain complete ReturnAsStream UTF-8");
  }
  if (String(summary.status).startsWith("FAIL_PREFLIGHT")) {
    throw new Error(`Chromium compositor preflight failed: ${summary.status}; ${JSON.stringify(summary.markerValidation)}`);
  }
});
