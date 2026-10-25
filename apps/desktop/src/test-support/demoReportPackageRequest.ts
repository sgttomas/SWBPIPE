// The demo's File > Save Report Package… arguments, for the native whole-command
// memory check (DEL-04-07 memory-budget decision, "Report-package request pricing";
// `src-tauri/src/lib.rs` `report_package_save_demo_request_peak_is_within_its_lease`).
// The bundled demo model with its committed dense result, solved through a mocked
// native transport that replays those producer bytes, then the session's own
// builders: the input manifest, the analysis record, the report-package request,
// its route control and the save preparation. Unit transport replay of committed
// producer bytes; NOT a native UI qualification witness. The request (about 7 MB)
// is generated, not committed: `demoReportPackageRequest.test.ts` writes its raw
// args body to the path in `OPENPIPESTRESS_DEMO_REPORT_PACKAGE_REQUEST`.
// Standard claim fence applies (claims taxonomy per DEC-081).
import { isDeepStrictEqual } from "node:util";
import demoModelFixture from "../../../../fixtures/product_preview/invented_demo_model.json";
import demoDenseResult from "../../../../fixtures/product_preview/invented_demo_result_preview_physics_1_dense.json?raw";
import { createSolveJobTransport } from "../test/nativeMechanicsReplay";
import { buildAnalysisRunPreview, runPreviewMechanics } from "../services/previewService";
import { buildCurrentSessionInputManifest } from "../services/inputManifestService";
import { buildReportPackageRequest } from "../features/report/reportPackageRequest";
import { controlReportPackageRequest } from "../features/report/reportRedactionProjector";
import { prepareReportPackageSave, type ReportPackageSaveArgs } from "../services/reportPackageSaveService";
import type { PreviewModel } from "../types";

/** The variable naming the file the generated raw args body is written to (and the
 * ignored native test reads). */
export const DEMO_REPORT_PACKAGE_REQUEST_ENV = "OPENPIPESTRESS_DEMO_REPORT_PACKAGE_REQUEST";

/** The demo model the bundled results were solved from. */
export function demoModel(): PreviewModel {
  return structuredClone(demoModelFixture) as PreviewModel;
}

/** A mocked native solve-job transport that answers a dense solve of the demo model
 * with its committed dense result bytes, and refuses any other request. */
export function demoDenseSolveTransport() {
  return createSolveJobTransport((request) => {
    if (request.mode !== "dense_scrutiny" || !isDeepStrictEqual(request.model, demoModelFixture)) {
      throw new Error("DEMO_REPLAY_REQUEST_MISMATCH: only a dense solve of the bundled demo model is replayed.");
    }
    return demoDenseResult;
  }, { jobIdPrefix: "unit-demo-report-package" });
}

/** The demo session's save arguments, as the workspace builds them after a dense
 * solve with no rule check, comparison or saved project. The caller routes
 * `@tauri-apps/api/core`'s `invoke` to `demoDenseSolveTransport()` and marks the
 * window as the native runtime. */
export async function buildDemoReportPackageSaveArgs(): Promise<ReportPackageSaveArgs> {
  const model = demoModel();
  const result = await runPreviewMechanics(model, "dense_scrutiny");
  const producer = result.producer;
  if (!producer) throw new Error("DEMO_REPLAY_PRODUCER_MISSING");
  // As `workspaceSession.ts` records a solve.
  const inputManifest = await buildCurrentSessionInputManifest({
    model,
    solver: {
      solver_name: producer.component_name,
      solver_version: producer.component_version,
      solver_build_ref: `${producer.component_name}@${producer.component_version}`,
      solver_mode: "dense_scrutiny",
      settings: { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: false }
    },
    active_rule_packs: [],
    external_assets: []
  });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest, rulePackExclusions: [] });
  const request = await buildReportPackageRequest({
    model, result, analysisRun, inputManifest, projectSummary: null, comparison: null, ruleCheckAggregate: null, ruleCheckRun: null
  });
  const controlled = controlReportPackageRequest(request, true);
  const prepared = await prepareReportPackageSave(controlled, model, request, inputManifest.manifest);
  if (prepared.route !== "ready") throw new Error(`DEMO_REPORT_PACKAGE_NOT_READY: ${prepared.diagnostic}`);
  return prepared.args;
}

/** The raw args body the desktop sends for these arguments (`nativeIngress.ts`: the
 * UTF-8 JSON of the argument object). */
export function demoReportPackageSaveBody(args: ReportPackageSaveArgs): Uint8Array {
  return new TextEncoder().encode(JSON.stringify(args));
}
