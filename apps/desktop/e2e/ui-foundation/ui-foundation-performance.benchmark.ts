import { randomUUID } from "node:crypto";
import { mkdir, writeFile, readFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { test } from "@playwright/test";
import {
  appendJsonLine,
  benchmarkPhase,
  baselineOutsideFrustumRecord,
  captureChromiumEnvironment,
  ensureViewportToggle,
  expectedTreeFilterCount,
  expectedVisibleEntityLabel,
  installInstrumentation,
  loadFixture,
  measureCanvasPointSelection,
  measureOrbit,
  measureTreeFilter,
  nearestRankP95,
  requireEvidenceRoot,
  routeModelFixture,
  settledRafState,
  unsupportedRecords,
  validateCandidateOracleBinding,
  waitForFirstUsable,
  writeJson
} from "./benchmark-harness";
import { performanceExpectation, runCandidateCausalPerformance } from "./full-cohort-controller";
import { scorePerformanceCohort, type RunExpectation } from "./performance-targets";
import { bindCandidateDriverEntry } from "./candidate-server-response";

const phase = benchmarkPhase();
const candidateDriverBinding = phase === "candidate" ? bindCandidateDriverEntry("candidate-performance") : null;
const evidenceRoot = requireEvidenceRoot();
const configuredCounts = (process.env.UI_FOUNDATION_PIPE_COUNTS ?? "1000,10000").split(",").map(Number);
const configuredRuns = (process.env.UI_FOUNDATION_RUNS ?? "1,2,3,4,5").split(",").map(Number);
const timeoutMs = Number(process.env.UI_FOUNDATION_ACTION_TIMEOUT_MS ?? 180_000);
const benchmarkDir = path.dirname(fileURLToPath(import.meta.url));

const cohortId = process.env.UI_FOUNDATION_COHORT_ID;
if (phase === "candidate" && (!cohortId || !/^[A-Za-z0-9._:-]{1,48}$/.test(cohortId))) throw new Error("fresh explicit candidate cohort ID required");
const candidatePlan = ([1000, 10000] as const).flatMap((fixtureSize) => [1, 2, 3, 4, 5].map((runNumber) => ({
  fixtureSize, runNumber, runId: `${cohortId}.${fixtureSize}.${runNumber}`, sessionId: randomUUID() })));
const candidateExpectations: RunExpectation[] = [];
if (phase === "candidate") {
  test.beforeAll(async () => {
    await mkdir(evidenceRoot, { recursive: true });
    for (const identity of candidatePlan) {
      candidateExpectations.push(await performanceExpectation(await loadFixture(identity.fixtureSize), candidateDriverBinding!, identity));
    }
    await writeFile(path.join(evidenceRoot, "candidate-cohort-plan.json"), `${JSON.stringify(candidateExpectations, null, 2)}\n`, { flag: "wx" });
  });
  test.afterAll(async () => {
    const completed = [], missing = [];
    for (const expected of candidateExpectations) {
      const file = path.join(evidenceRoot, "raw", `run-${String(expected.runNumber).padStart(2, "0")}`, String(expected.fixtureSize), "result.json");
      try { completed.push(JSON.parse(await readFile(file, "utf8")).evidence); }
      catch (error) { missing.push({ runId: expected.runId, error: String(error) }); }
    }
    const result = scorePerformanceCohort(completed, candidateExpectations);
    await writeFile(path.join(evidenceRoot, "candidate-cohort-result.json"), `${JSON.stringify({ ...result, missing }, null, 2)}\n`, { flag: "wx" });
    if (result.status !== "PASS_COHORT_METRICS") throw new Error("complete five-by-two cohort did not qualify; immutable outcomes retained");
  });
}

test.describe.serial(`UI foundation ${phase} production benchmark`, () => {
  for (const pipeCount of configuredCounts) {
    for (const run of configuredRuns) {
      test(`${phase} N=${pipeCount} run ${run}`, async ({ page, browserName }) => {
        const fixture = await loadFixture(pipeCount);
        const candidateOracleBinding = phase === "candidate"
          ? await validateCandidateOracleBinding(fixture, pipeCount)
          : null;
        const runDir = path.join(evidenceRoot, "raw", `run-${String(run).padStart(2, "0")}`, String(pipeCount));
        const failuresPath = path.join(runDir, "failures.jsonl");
        const startedAt = new Date().toISOString();
        const manifest = JSON.parse(await readFile(path.join(benchmarkDir, "fixture-manifest.json"), "utf8"));
        const environment: any = {
          phase,
          run,
          pipeCount,
          browserName,
          viewportCssPixels: [1440, 920],
          deviceScaleFactor: 2,
          startedAt,
          fixtureReadMs: fixture.readMs,
          fixtureParseMs: fixture.parseMs,
          supportFilesReadMs: fixture.supportFilesReadMs,
          supportFilesParseMs: fixture.supportFilesParseMs,
          pointHitPolicySha256: fixture.pointOracle.point_hit_policy_sha256,
          pointOraclePath: fixture.pointOraclePath,
          pointOracleSha256: fixture.pointOracleSha256,
          candidateOracleBinding,
          modelPath: fixture.modelPath,
          samplesPath: fixture.samplesPath,
          manifestSha256ExpectedByDriver: phase === "candidate"
            ? candidateDriverBinding?.fixtureManifestSha256 ?? null
            : process.env.UI_FOUNDATION_MANIFEST_SHA256 ?? null,
          candidateDriverBinding,
          productionBundleSha256ExpectedByDriver: process.env.UI_FOUNDATION_BUNDLE_MANIFEST_SHA256 ?? null
        };
        if (phase === "candidate") {
          const expected = candidateExpectations.find((e) => e.fixtureSize === pipeCount && e.runNumber === run);
          if (!expected) throw new Error("run is outside the frozen five-by-two cohort plan");
          await runCandidateCausalPerformance(page, fixture, expected, runDir, { timeoutMs, binding: candidateDriverBinding! });
          return;
        }
        // Candidate runs returned above; this historical proxy path is baseline only.
        await writeJson(path.join(runDir, "environment.json"), environment);
        await installInstrumentation(page, { captureGlobalRaf: false });
        await routeModelFixture(page, fixture);

        const runSummary: any = { phase, run, pipeCount, startedAt, status: "RUNNING" };
        try {
          const navigationStarted = Date.now();
          await page.goto("/", { waitUntil: "domcontentloaded", timeout: timeoutMs });
          const firstUsable = await waitForFirstUsable(page, fixture, timeoutMs, phase);
          const navigationToFirstUsableMs = Date.now() - navigationStarted;
          const capturePath = path.join(runDir, "first-usable.png");
          const beforeCaptureAt = await page.evaluate(() => performance.now());
          await page.screenshot({ path: capturePath });
          const browserObservedCaptureUpperBoundAt = await page.evaluate(() => performance.now());
          firstUsable.browserObservedCapture = {
            path: "first-usable.png",
            beforeCaptureAt,
            timestampUpperBoundAt: browserObservedCaptureUpperBoundAt,
            captureElapsedUpperBoundMs: browserObservedCaptureUpperBoundAt - beforeCaptureAt,
            proxyDurationUpperBoundMs: typeof firstUsable.fixtureModuleEvaluated === "number"
              ? browserObservedCaptureUpperBoundAt - firstUsable.fixtureModuleEvaluated
              : null,
            strictDurationUpperBoundMs: typeof firstUsable.strictAssignmentStartedAt === "number"
              ? browserObservedCaptureUpperBoundAt - firstUsable.strictAssignmentStartedAt
              : null,
            status: "BROWSER_OBSERVED_CAPTURE_WITH_PROXY_START_CONSERVATIVE_UPPER_BOUND"
          };
          environment.chromium = await captureChromiumEnvironment(page);
          environment.environmentCapturedAt = new Date().toISOString();
          await writeJson(path.join(runDir, "environment.json"), environment);
          await writeJson(path.join(runDir, "startup.json"), {
            processToFirstWindow: "N/A_BROWSER_RUN_RECORDED_SEPARATELY_FROM_NATIVE_STARTUP",
            browserNavigationToFirstUsableMs: navigationToFirstUsableMs,
            fixtureFileReadMs: fixture.readMs,
            fixtureJsonParseMs: fixture.parseMs,
            firstUsable
          });
          await writeJson(path.join(runDir, "model-assignment.json"), firstUsable);

          await page.getByTestId("viewport-view-cube").getByRole("button", { name: "Iso" }).click({ timeout: timeoutMs });
          await writeJson(path.join(runDir, "camera.json"), {
            status: "BASELINE_REACHABLE_RECIPE_APPLIED_CAMERA_STATE_UNOBSERVED",
            recipe: ["clicked visible Iso preset", "later applied exact frozen orbit pointer path through real canvas"],
            exactCameraState: null,
            limitation: "Baseline exposes no camera readback. Nominal candidate pose values are not claimed as observed."
          });

          const pointDurations: number[] = [];
          const pointRecords: any[] = [];
          await ensureViewportToggle(page, "toggle-viewport-labels", false, timeoutMs);
          for (const sample of fixture.samples.point_selection) {
            const oracleProbe = fixture.pointOracle.probes.find((probe: any) => probe.sample === sample.sample);
            if (!oracleProbe) throw new Error(`point oracle sample ${sample.sample} is absent`);
            const outsideRecord = baselineOutsideFrustumRecord(sample, oracleProbe);
            if (outsideRecord) {
              pointRecords.push(outsideRecord);
              await appendJsonLine(path.join(runDir, "point-selection.jsonl"), outsideRecord);
              continue;
            }
            try {
              const capturePath = path.join(runDir, `point-sample-${String(sample.sample).padStart(3, "0")}.png`);
              const selectionVisualOracle = {
                source: "unchanged baseline PipeViewport selected material 0xf08c22",
                theme: "baseline light",
                srgb: [240, 140, 34] as [number, number, number],
                tolerancePerChannel: 80,
                minimumQualifyingInteriorPixels: 1
              };
              if (!selectionVisualOracle || !Array.isArray(selectionVisualOracle.srgb) || selectionVisualOracle.srgb.length !== 3 ||
                !selectionVisualOracle.srgb.every(Number.isFinite) || !Number.isFinite(selectionVisualOracle.tolerancePerChannel) ||
                !Number.isInteger(selectionVisualOracle.minimumQualifyingInteriorPixels) || selectionVisualOracle.minimumQualifyingInteriorPixels < 1) {
                throw new Error("baseline selection visual oracle is invalid");
              }
              const record = await measureCanvasPointSelection(
                page, sample, oracleProbe, timeoutMs, capturePath, phase, fixture.model.project.id,
                expectedVisibleEntityLabel(fixture.model, oracleProbe.baseline.oracle.expectedHitRef),
                selectionVisualOracle
              );
              pointRecords.push(record);
              if (typeof record.durationMs === "number") pointDurations.push(record.durationMs);
              await appendJsonLine(path.join(runDir, "point-selection.jsonl"), record);
            } catch (error) {
              const failure = { stage: "point-selection", sample: sample.sample, probeAnchorRef: sample.probe_anchor_ref, error: String(error) };
              await appendJsonLine(failuresPath, failure);
              throw error;
            }
          }
          await ensureViewportToggle(page, "toggle-viewport-labels", true, timeoutMs);
          const boxRecords = unsupportedRecords(
            fixture.samples.box_selection,
            "N/A_BASELINE_CONTROL_ABSENT",
            "Baseline has no explicit Box Select tool; no substitute gesture is counted."
          );
          for (const record of boxRecords) await appendJsonLine(path.join(runDir, "box-selection.jsonl"), record);

          const filterDurations: number[] = [];
          for (const sample of fixture.samples.tree_filters) {
            try {
              const totalCount = expectedTreeFilterCount(fixture.model, "");
              const expectedCount = expectedTreeFilterCount(fixture.model, sample.query);
              const filterCapturePath = path.join(runDir, `tree-filter-${String(sample.sample).padStart(2, "0")}.png`);
              const record = await measureTreeFilter(
                page, sample.query, expectedCount, totalCount, timeoutMs, phase, filterCapturePath
              );
              if (typeof record.durationMs === "number") filterDurations.push(record.durationMs);
              await appendJsonLine(path.join(runDir, "tree-filter.jsonl"), { sample: sample.sample, ...record });
            } catch (error) {
              const failure = { stage: "tree-filter", sample: sample.sample, query: sample.query, error: String(error) };
              await appendJsonLine(failuresPath, failure);
              throw error;
            }
          }

          const orbit = await measureOrbit(
            page,
            fixture.samples.orbit_pointer_path.warmup_ms,
            fixture.samples.orbit_pointer_path.measured_ms
          );
          for (const [index, intervalMs] of orbit.callbackOpportunityIntervalsMs.entries()) {
            await appendJsonLine(path.join(runDir, "orbit-centerline.jsonl"), { sample: index + 1, intervalMs });
          }
          await writeJson(path.join(runDir, "orbit-centerline-summary.json"), {
            ...orbit,
            callbackOpportunityIntervalsMs: undefined,
            p95Ms: nearestRankP95(orbit.callbackOpportunityIntervalsMs)
          });
          await writeJson(path.join(runDir, "orbit-actual-od.json"), {
            status: "N/A_BASELINE_CONTROL_ABSENT",
            samples: 0,
            p95Ms: null
          });
          const settled = await settledRafState(page);
          await writeJson(path.join(runDir, "settled-raf.json"), settled);
          const heap = await page.evaluate(() => {
            const memory = (performance as any).memory;
            return memory ? {
              status: "MEASURED_CHROMIUM_NONSTANDARD",
              usedJSHeapSize: memory.usedJSHeapSize,
              totalJSHeapSize: memory.totalJSHeapSize,
              jsHeapSizeLimit: memory.jsHeapSizeLimit
            } : { status: "N/A_PERFORMANCE_MEMORY_UNAVAILABLE" };
          });
          await writeJson(path.join(runDir, "heap-rss.json"), {
            javascriptHeap: heap,
            nativeRss: { status: "N/A_BROWSER_RUN_NATIVE_PROCESS_ATTRIBUTION_RECORDED_SEPARATELY" }
          });
          await writeJson(path.join(runDir, "resources.json"), {
            rendererCounters: { status: "N/A_BASELINE_NO_RENDERER_INFO_READBACK" },
            inSessionReplacement20: {
              status: "N/A_BASELINE_REAL_OPEN_ROUTE_EXISTS_BUT_GENERATION_RESOURCE_OBSERVABILITY_ABSENT_AND_NOT_MEASURED",
              controlAvailabilityIsNotResourceQualification: true
            },
            unmountRemount: {
              status: "N/A_NO_ACCEPTED_PRODUCTION_BROWSER_VIEWPORT_UNMOUNT_COMMAND",
              separateEvidence: ["source component cleanup tests", "native process quit and reopen"],
              injectedReactMutationBridgeAllowed: false
            }
          });
          runSummary.status = "COMPLETE_WITH_BASELINE_LIMITATIONS";
          runSummary.modelAssignment = firstUsable;
          runSummary.pointSelectionCommonProxy = {
            count: pointDurations.length,
            prescribedCount: pointRecords.length,
            attemptedCount: pointRecords.filter((record) => typeof record.durationMs === "number").length,
            failedCount: pointRecords.filter((record) => String(record.status).startsWith("FAIL_")).length,
            outsideFrustumCount: pointRecords.filter((record) => record.status === "NOT_ATTEMPTED_OUTSIDE_BASELINE_FRUSTUM").length,
            p95Ms: nearestRankP95(pointDurations),
            status: "REAL_CANVAS_POINTER_ACTIONS_WITH_BROWSER_CAPTURE",
            strictStatus: "N/A_BASELINE_NO_GENERATION_BOUND_MAIN_RENDER_SUBMISSION"
          };
          runSummary.boxSelection = { count: 0, prescribedCount: boxRecords.length, status: boxRecords[0]?.status ?? "N/A" };
          runSummary.treeFilter = { count: filterDurations.length, p95Ms: nearestRankP95(filterDurations) };
          runSummary.orbitCenterline = { status: orbit.status, count: orbit.callbackOpportunityIntervalsMs.length, p95Ms: nearestRankP95(orbit.callbackOpportunityIntervalsMs), strictStatus: orbit.strictMainRenderFrameStatus };
          runSummary.actualOd = { count: 0, status: "N/A_BASELINE_CONTROL_ABSENT" };
          runSummary.settledRaf = settled;
          runSummary.manifestTargetsUnchanged = manifest.benchmark.acceptance_targets_ms;
        } catch (error) {
          runSummary.status = "FAILED_RAW_PRESERVED";
          runSummary.error = String(error);
          await appendJsonLine(failuresPath, { stage: "run", error: String(error), at: new Date().toISOString() });
          throw error;
        } finally {
          runSummary.finishedAt = new Date().toISOString();
          await writeJson(path.join(runDir, "run-summary.json"), runSummary);
        }
      });
    }
  }
});
