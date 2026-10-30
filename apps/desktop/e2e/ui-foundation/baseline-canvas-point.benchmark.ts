import path from "node:path";
import { stat } from "node:fs/promises";
import { test } from "@playwright/test";
import {
  appendJsonLine,
  captureBrowserProcessAndHeapState,
  captureChromiumEnvironment,
  ensureViewportToggle,
  expectedTreeFilterCount,
  expectedVisibleEntityLabel,
  installInstrumentation,
  loadFixture,
  measureCanvasPointSelection,
  measureTreeFilter,
  nearestRankP95,
  requireEvidenceRoot,
  routeModelFixture,
  waitForFirstUsable,
  writeJson
} from "./benchmark-harness";

const evidenceRoot = requireEvidenceRoot();
const configuredRuns = (process.env.UI_FOUNDATION_RUNS ?? "1,2,3,4,5").split(",").map(Number);
const timeoutMs = Number(process.env.UI_FOUNDATION_ACTION_TIMEOUT_MS ?? 30_000);
const feasibilitySampleCount = process.env.UI_FOUNDATION_FEASIBILITY_SAMPLE_COUNT
  ? Number(process.env.UI_FOUNDATION_FEASIBILITY_SAMPLE_COUNT)
  : null;
if (feasibilitySampleCount !== null && (!Number.isInteger(feasibilitySampleCount) || feasibilitySampleCount < 1 || feasibilitySampleCount > 200)) {
  throw new Error("UI_FOUNDATION_FEASIBILITY_SAMPLE_COUNT must be an integer from 1 through 200");
}

test.describe.serial("corrected baseline real canvas-point observations", () => {
  for (const run of configuredRuns) {
    test(`baseline N=1000 point run ${run}`, async ({ page, browserName }) => {
      const pipeCount = 1_000;
      const fixture = await loadFixture(pipeCount);
      const runDir = path.join(evidenceRoot, "raw", `run-${String(run).padStart(2, "0")}`, String(pipeCount));
      const failuresPath = path.join(runDir, "failures.jsonl");
      const environment: any = {
        schema: "openpipestress.ui-foundation.baseline-canvas-point-environment/v1",
        phase: "baseline",
        run,
        pipeCount,
        browserName,
        viewportCssPixels: [1440, 920],
        deviceScaleFactor: 2,
        manifestSha256ExpectedByDriver: process.env.UI_FOUNDATION_MANIFEST_SHA256 ?? null,
        productionBundleSha256ExpectedByDriver: process.env.UI_FOUNDATION_BUNDLE_MANIFEST_SHA256 ?? null,
        pointHitPolicySha256: fixture.pointOracle.point_hit_policy_sha256,
        pointOraclePath: fixture.pointOraclePath,
        pointOracleSha256: fixture.pointOracleSha256,
        startedAt: new Date().toISOString()
      };
      await writeJson(path.join(runDir, "environment.json"), environment);
      await installInstrumentation(page, { captureGlobalRaf: false });
      await routeModelFixture(page, fixture);
      const records = [];
      const filterRecords = [];
      try {
        const navigationStarted = Date.now();
        await page.goto("/", { waitUntil: "domcontentloaded", timeout: 180_000 });
        const firstUsable = await waitForFirstUsable(page, fixture, 180_000, "baseline");
        const browserNavigationToFirstUsableMs = Date.now() - navigationStarted;
        const firstUsableCapturePath = path.join(runDir, "first-usable.png");
        const firstUsableBeforeCaptureAt = await page.evaluate(() => performance.now());
        await page.screenshot({ path: firstUsableCapturePath });
        const firstUsablePostCaptureAt = await page.evaluate(() => performance.now());
        firstUsable.browserObservedCapture = {
          path: "first-usable.png",
          beforeCaptureAt: firstUsableBeforeCaptureAt,
          timestampUpperBoundAt: firstUsablePostCaptureAt,
          captureElapsedUpperBoundMs: firstUsablePostCaptureAt - firstUsableBeforeCaptureAt,
          proxyDurationUpperBoundMs: typeof firstUsable.fixtureModuleEvaluated === "number"
            ? firstUsablePostCaptureAt - firstUsable.fixtureModuleEvaluated : null,
          status: "BROWSER_OBSERVED_CAPTURE_WITH_PROXY_START_CONSERVATIVE_UPPER_BOUND",
          format: "png",
          scale: "device",
          fullPage: false
        };
        environment.chromium = await captureChromiumEnvironment(page);
        environment.environmentCapturedAt = new Date().toISOString();
        await writeJson(path.join(runDir, "environment.json"), environment);
        await writeJson(path.join(runDir, "startup.json"), {
          processToFirstWindow: "N/A_BROWSER_RUN_RECORDED_SEPARATELY_FROM_NATIVE_STARTUP",
          browserNavigationToFirstUsableMs,
          fixtureFileReadMs: fixture.readMs,
          fixtureJsonParseMs: fixture.parseMs,
          firstUsable
        });
        await writeJson(path.join(runDir, "model-assignment.json"), firstUsable);
        await page.getByTestId("viewport-view-cube").getByRole("button", { name: "Iso", exact: true }).click({ timeout: timeoutMs });
        await writeJson(path.join(runDir, "camera.json"), {
          status: "BASELINE_REAL_ISO_CONTROL_APPLIED_CAMERA_READBACK_UNAVAILABLE",
          sourceBoundProjectionState: fixture.pointOracle.baseline_camera,
          actualCameraState: null,
          limitation: "Source-bound baseline Iso constants support the independent probe projection, but the baseline has no runtime camera readback. They are not reported as an observed state."
        });
        await ensureViewportToggle(page, "toggle-viewport-labels", false, timeoutMs);
        const resourceBefore = await captureBrowserProcessAndHeapState(page);
        const plannedSamples = feasibilitySampleCount === null
          ? fixture.samples.point_selection
          : fixture.samples.point_selection.slice(0, feasibilitySampleCount);
        for (const sample of plannedSamples) {
          const oracleProbe = fixture.pointOracle.probes.find((probe: any) => probe.sample === sample.sample);
          if (oracleProbe?.baseline?.actionability === "NOT_ATTEMPTED_OUTSIDE_BASELINE_FRUSTUM") {
            const record = {
              sample: sample.sample,
              probeAnchorRef: sample.probe_anchor_ref,
              status: "NOT_ATTEMPTED_OUTSIDE_BASELINE_FRUSTUM",
              durationMs: null,
              baselineNdc: oracleProbe.baseline.ndc,
              actionability: oracleProbe.baseline.actionability,
              oracleStatus: oracleProbe.baseline.oracle?.status ?? null,
              oracleReason: oracleProbe.baseline.oracle?.reason ?? null
            };
            records.push(record);
            await appendJsonLine(path.join(runDir, "point-selection.jsonl"), record);
            continue;
          }
          try {
            const record = await measureCanvasPointSelection(
              page,
              sample,
              oracleProbe,
              timeoutMs,
              path.join(runDir, `point-sample-${String(sample.sample).padStart(3, "0")}.png`),
              "baseline",
              fixture.model.project.id,
              expectedVisibleEntityLabel(fixture.model, oracleProbe.baseline.oracle.expectedHitRef),
              {
                source: "unchanged baseline PipeViewport selected material 0xf08c22",
                theme: "baseline light",
                srgb: [240, 140, 34],
                tolerancePerChannel: 80,
                minimumQualifyingInteriorPixels: 1
              }
            );
            records.push(record);
            await appendJsonLine(path.join(runDir, "point-selection.jsonl"), record);
          } catch (error) {
            const record = {
              sample: sample.sample,
              probeAnchorRef: sample.probe_anchor_ref,
              status: "FAIL_BASELINE_CANVAS_POINT_SAMPLE",
              durationMs: null,
              error: String(error)
            };
            records.push(record);
            await appendJsonLine(path.join(runDir, "point-selection.jsonl"), record);
            await appendJsonLine(failuresPath, { stage: "point-selection", ...record });
            if (page.isClosed()) break;
          }
        }
        if (!page.isClosed()) {
          const resourceAfter = await captureBrowserProcessAndHeapState(page);
          const captures = [];
          for (const record of records) {
            const captureName = record.browserObservedCapture?.path;
            if (typeof captureName !== "string") continue;
            const capturePath = path.join(runDir, captureName);
            try {
              const metadata = await stat(capturePath);
              captures.push({ sample: record.sample, path: captureName, bytes: metadata.size, captureElapsedUpperBoundMs: record.browserObservedCapture.captureElapsedUpperBoundMs });
            } catch (error) {
              captures.push({ sample: record.sample, path: captureName, status: "UNAVAILABLE", error: String(error) });
            }
          }
          await writeJson(path.join(runDir, "trace-off-resource-capture-context.json"), {
            schema: "openpipestress.ui-foundation.trace-off-resource-capture-context/v1",
            playwrightTrace: "off",
            video: "off",
            globalRafWrapper: false,
            resourceBefore,
            resourceAfter,
            deltas: {
              javascriptUsedHeapBytes: typeof resourceBefore.javascriptHeap?.usedJSHeapSize === "number" && typeof resourceAfter.javascriptHeap?.usedJSHeapSize === "number"
                ? resourceAfter.javascriptHeap.usedJSHeapSize - resourceBefore.javascriptHeap.usedJSHeapSize : null,
              chromiumReportedProcessRssKiB: typeof resourceBefore.operatingSystemProcessSnapshot?.rssKiBSum === "number" && typeof resourceAfter.operatingSystemProcessSnapshot?.rssKiBSum === "number"
                ? resourceAfter.operatingSystemProcessSnapshot.rssKiBSum - resourceBefore.operatingSystemProcessSnapshot.rssKiBSum : null
            },
            explicitPointCaptures: {
              count: captures.length,
              totalBytes: captures.reduce((sum, capture: any) => sum + (typeof capture.bytes === "number" ? capture.bytes : 0), 0),
              captures
            },
            overheadPolicy: "No screenshot, CDP, filesystem, trace, process-sampling, or wait cost is subtracted from a reported latency. Resource snapshots are outside individual point timers."
          });
          await ensureViewportToggle(page, "toggle-viewport-labels", true, timeoutMs);
          if (feasibilitySampleCount === null) {
            for (const sample of fixture.samples.tree_filters) {
              try {
                const totalCount = expectedTreeFilterCount(fixture.model, "");
                const expectedCount = expectedTreeFilterCount(fixture.model, sample.query);
                const record = await measureTreeFilter(
                  page,
                  sample.query,
                  expectedCount,
                  totalCount,
                  timeoutMs,
                  "baseline",
                  path.join(runDir, `tree-filter-${String(sample.sample).padStart(2, "0")}.png`)
                );
                filterRecords.push({ sample: sample.sample, ...record });
                await appendJsonLine(path.join(runDir, "tree-filter.jsonl"), { sample: sample.sample, ...record });
              } catch (error) {
                const failure = { stage: "tree-filter", sample: sample.sample, query: sample.query, error: String(error) };
                await appendJsonLine(failuresPath, failure);
                throw error;
              }
            }
          }
        }
      } catch (error) {
        await appendJsonLine(failuresPath, { stage: "run", error: String(error), at: new Date().toISOString() });
      }
      const remaining = fixture.samples.point_selection.filter((sample: any) => !records.some((record: any) => record.sample === sample.sample));
      for (const sample of remaining) {
        const record = {
          sample: sample.sample,
          probeAnchorRef: sample.probe_anchor_ref,
          status: feasibilitySampleCount === null
            ? "NOT_ATTEMPTED_BASELINE_PAGE_UNAVAILABLE_AFTER_RECORDED_FAILURE"
            : "NOT_ATTEMPTED_BOUNDED_FEASIBILITY_ONLY",
          durationMs: null
        };
        records.push(record);
        await appendJsonLine(path.join(runDir, "point-selection.jsonl"), record);
      }
      records.sort((a, b) => a.sample - b.sample);
      const successful = records.filter((record) =>
        record.status === "BASELINE_EXPECTED_SELECTION_AND_PROPERTY_OBSERVED_CAPTURED" && typeof record.durationMs === "number");
      const timedRecords = records.filter((record) => typeof record.durationMs === "number");
      const failedCount = records.filter((record) => String(record.status).startsWith("FAIL_")).length;
      const unavailableAfterFailureCount = records.filter((record) => String(record.status).startsWith("NOT_ATTEMPTED_BASELINE_PAGE_UNAVAILABLE")).length;
      const pointCaptureP95 = nearestRankP95(timedRecords.map((record) => record.durationMs));
      const filterCaptureP95 = nearestRankP95(filterRecords.map((record: any) => record.durationMs).filter((value: any) => typeof value === "number"));
      const summary = {
        schema: "openpipestress.ui-foundation.baseline-canvas-point-run/v2",
        status: feasibilitySampleCount !== null
          ? (failedCount ? "BOUNDED_FEASIBILITY_RECORDED_WITH_ACTION_FAILURES" : "BOUNDED_FEASIBILITY_COMPLETE_NOT_A_QUALIFICATION_RUN")
          : failedCount || unavailableAfterFailureCount
          ? "BASELINE_OBSERVATION_RECORDED_WITH_ACTION_OR_PAGE_FAILURES"
          : "BASELINE_OBSERVATION_COMPLETE_WITH_STRICT_PRESENTATION_LIMITATION",
        evidenceClass: feasibilitySampleCount === null ? "FULL_BASELINE_SESSION" : "BOUNDED_FEASIBILITY_ONLY",
        run,
        prescribedCount: 200,
        plannedAttemptCount: feasibilitySampleCount ?? 200,
        recordedCount: records.length,
        successfulRealCanvasExpectedHitCount: successful.length,
        outsideFrustumCount: records.filter((record) => record.status === "NOT_ATTEMPTED_OUTSIDE_BASELINE_FRUSTUM").length,
        failedCount,
        unavailableAfterFailureCount,
        boundedFeasibilityNotAttemptedCount: records.filter((record) => record.status === "NOT_ATTEMPTED_BOUNDED_FEASIBILITY_ONLY").length,
        browserCaptureConservativeUpperBoundP95Ms: pointCaptureP95,
        timedAttemptCountIncludingWrongFeedback: timedRecords.length,
        targetInterpretation: typeof pointCaptureP95 === "number" && pointCaptureP95 <= 100
          ? "BROWSER_CAPTURE_BOUND_AT_OR_BELOW_100MS_WITH_BASELINE_GENERATION_LIMITATION"
          : "CONSERVATIVE_BOUND_DID_NOT_ESTABLISH_100MS_TARGET_NOT_PROOF_PHYSICAL_FEEDBACK_EXCEEDED_100MS",
        treeFilter: feasibilitySampleCount === null ? {
          prescribedCount: 20,
          recordedCount: filterRecords.length,
          browserCaptureConservativeUpperBoundP95Ms: filterCaptureP95,
          targetInterpretation: typeof filterCaptureP95 === "number" && filterCaptureP95 <= 200
            ? "BROWSER_CAPTURE_BOUND_AT_OR_BELOW_200MS_WITH_BASELINE_GENERATION_LIMITATION"
            : "CONSERVATIVE_BOUND_DID_NOT_ESTABLISH_200MS_TARGET_NOT_PROOF_PHYSICAL_FEEDBACK_EXCEEDED_200MS",
          status: filterRecords.length === 20
            ? "BASELINE_MATCHED_FILTER_CAPTURE_ENDPOINT_COMPLETE"
            : "BASELINE_MATCHED_FILTER_CAPTURE_ENDPOINT_INCOMPLETE"
        } : {
          prescribedCount: 20,
          recordedCount: 0,
          status: "NOT_ATTEMPTED_BOUNDED_POINT_FEASIBILITY_ONLY"
        },
        strictPresentedStatus: "N/A_BASELINE_NO_GENERATION_BOUND_MAIN_RENDER_SUBMISSION",
        resetBoundary: "Every attempted sample used the real project tree row to establish exclusive Project selection outside its timer; reset records are embedded per sample.",
        finishedAt: new Date().toISOString()
      };
      await writeJson(path.join(runDir, "run-summary.json"), summary);
    });
  }
});
