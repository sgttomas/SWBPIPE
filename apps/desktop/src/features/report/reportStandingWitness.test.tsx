/** DEL-08-01 REQ-024's connecting path, desktop end: two exact-route results that are
 * solved but not numerically eligible (`fixtures/reports/invented/report_standing_witnesses/`,
 * written by `core/product_physics/tests/report_standing_witnesses.rs`) go through the
 * production report build step (`assembleRenderedReport`) under the native-capture test seams.
 *
 * - The rendered report admits them (readable and solved), with the standing record the
 *   one Rust reader derives, under the historical vocabularies.
 * - The report package request and the user-released package request admit them too
 *   (DEL-08-01 REQ-024, "Package and user release"; Agent 0, 20 November 2026): each carries
 *   the same standing record, the local-private package's report is the rendered report's
 *   input, and the user-release record names no result as not carried on the ground of its
 *   standing.
 * - The render arguments the desktop would send the native host are committed beside the
 *   results (`<name>.render.json`; stale fails; `OPENPIPESTRESS_WRITE_REPORT_STANDING_RENDER_INPUTS=<dir>`
 *   writes them), with what the native host needs to rebuild both package requests and their
 *   canonical SHA-256s (`<name>.package.json`), and the native host's tests render and save them.
 *
 * Unit transport replay of committed producer bytes through mocked IPC; NOT a native UI
 * qualification witness. Job ids are invented; case labels and project names are added
 * where a committed request has none. Standard claim fence applies (claims taxonomy per
 * DEC-081). */
import { afterEach, describe, expect, it, vi } from "vitest";
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import type { MechanicsResult, PreviewModel } from "../../types";
import { createSolveJobTransport } from "../../test/nativeMechanicsReplay";
import { buildAnalysisRunPreview, resetSessionResultCustodyForTests, runPreviewMechanics } from "../../services/previewService";
import { buildCurrentSessionInputManifest, type CurrentSessionInputManifestEvidence } from "../../services/inputManifestService";
import { numericalResultStanding, reportStanding } from "../results/resultStanding";
import { canonicalSha256Hex } from "../../services/hashService";
import { assembleRenderedReport } from "./renderedReportAssembly";
import { buildReportPackageRequest, buildUserReleasedReportPackageRequest, reportPackageExactSubmission } from "./reportPackageRequest";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
afterEach(() => {
  invokeMock.mockReset();
  vi.restoreAllMocks();
  resetSessionResultCustodyForTests();
  delete (window as Json).__TAURI_INTERNALS__;
});
const root = resolve(__dirname, "../../../../../");
const text = (path: string) => readFileSync(resolve(root, path), "utf8");
const json = (path: string) => JSON.parse(text(path));
const DIR = "fixtures/reports/invented/report_standing_witnesses";
const GENERATOR = "apps/desktop/src/features/report/reportStandingWitness.test.tsx";
const WRITE = "OPENPIPESTRESS_WRITE_REPORT_STANDING_RENDER_INPUTS";

type Shape = { name: string; rawPath: string; requestPath: string };
const SHAPES: readonly Shape[] = [
  { name: "pressure_3_gap_unresolved", rawPath: `${DIR}/pressure_3_gap_unresolved-sparse_interactive.raw.json`, requestPath: "validation/references/rt_a3_s6/requests/c1_gap.json" },
  { name: "pressure_3_coupled_friction_sensitive", rawPath: `${DIR}/pressure_3_coupled_friction_sensitive-sparse_interactive.raw.json`, requestPath: `${DIR}/coupled_friction.request.json` }
];

/** The solver settings the desktop records in a session's input manifest, which the native
 * user-release derivation derives (`report_package::user_release::derive_input_manifest`). */
const DESKTOP_SOLVER_SETTINGS = { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: true };

/** The invented time of the user's statement (the record's `user_released_at`). */
const USER_RELEASED_AT = "2026-11-20T12:00:00Z";

type Session = { model: PreviewModel; result: MechanicsResult; analysisRun: Json; inputManifest: CurrentSessionInputManifestEvidence; intakeText: string };

/** One native-capture solve of the committed raw bytes for the shape's request model. */
async function solve(item: Shape): Promise<Session> {
  const model = structuredClone(json(item.requestPath).model) as PreviewModel;
  for (const loadCase of model.load_cases) if (loadCase.label === undefined) Object.assign(loadCase, { label: loadCase.id, kind: "invented_test_case", status: "invented_test_status" });
  if (typeof model.project.name !== "string") Object.assign(model.project, { name: `Invented report-standing witness ${item.name}` });
  (window as Json).__TAURI_INTERNALS__ = {};
  let intakeText = "";
  const rawText = text(item.rawPath);
  const transport = createSolveJobTransport((sent) => { intakeText = sent.text; return rawText; }, { jobIdPrefix: `invented-standing-${item.name}-job` });
  invokeMock.mockImplementation(transport.invoke);
  const result = await runPreviewMechanics(model, "sparse_interactive");
  const inputManifest = await buildCurrentSessionInputManifest({
    model,
    solver: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: `${result.producer!.component_name}@${result.producer!.component_version}`, solver_mode: "sparse_interactive", settings: DESKTOP_SOLVER_SETTINGS },
    active_rule_packs: [],
    external_assets: []
  });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
  return { model, result, inputManifest, analysisRun, intakeText };
}

describe("report standing witnesses (DEL-08-01 REQ-024)", () => {
  it("renders and packages each witness with its standing record, and the render and package arguments are current", async () => {
    const outDir = process.env[WRITE];
    for (const item of SHAPES) {
      const session = await solve(item);
      const { model, result, analysisRun, inputManifest } = session;
      expect(numericalResultStanding(result, model).eligible, item.name).toBe(false);
      const reading = reportStanding(result, model);
      expect(reading.readability, item.name).toEqual({ readable: true, solved: true, unreadable: [] });
      const assembly = await assembleRenderedReport({ model, result, analysisRun, inputManifest, projectSummary: null, ruleCheckRun: null, numericalHealth: null, context: "local_private", intent: true, generatedAt: null, libraryRefFindings: [] });
      const input = assembly.input as Json;
      expect(input.numerical_standing, item.name).toEqual(reading.numerical_standing);
      const cases = input.numerical_standing.cases as Json[];
      if (item.name === "pressure_3_gap_unresolved") {
        expect(cases.map((c) => [c.case_id, c.health, c.integrity_code, c.confirmation.outcome])).toEqual([
          ["load:PRESSURE", "quality_unresolved", "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED", "no_confirmation_record"],
          ["load:VACUUM", "quality_unresolved", "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED", "no_confirmation_record"]
        ]);
      } else {
        expect(cases.map((c) => [c.case_id, c.health, c.integrity_code, c.confirmation.outcome, c.confirmation.status])).toEqual([
          ["load:RS-SUS", "sensitive", "NUMERICAL_INTEGRITY_SENSITIVE", "solver_reported_established", "conditional"]
        ]);
      }
      // DEL-08-01 REQ-024, "Package and user release": the package request and the
      // user-released request admit the same result as the render, and state the same
      // standing record. Numerical standing is not a ground for refusing either.
      const localPrivate = await buildReportPackageRequest({ model, result, analysisRun, inputManifest, projectSummary: null, comparison: null, ruleCheckAggregate: null, ruleCheckRun: null }) as Json;
      expect(localPrivate.report.numerical_standing, item.name).toEqual(reading.numerical_standing);
      // The local-private package's report is the rendered report's input, byte for byte.
      expect(JSON.stringify(localPrivate.report), item.name).toBe(JSON.stringify(assembly.controlled.payload));
      const userReleased = await buildUserReleasedReportPackageRequest({ model, result, analysisRun, inputManifest, userReleasedAt: USER_RELEASED_AT }) as Json;
      expect(userReleased.report.numerical_standing, item.name).toEqual(reading.numerical_standing);
      const record = userReleased.report.report_context.user_release;
      expect(record.record_version, item.name).toBe("1.2.0");
      expect(record.scope.not_carried, item.name).toEqual(["rule_pack_references", "user_rule_outcomes", "run_comparisons", "handoff_records", "library_records", "audit_assets"]);
      const submission = assembly.exactSubmission!();
      expect(submission.exactResultText).toBe(text(item.rawPath));
      for (const request of [localPrivate, userReleased]) {
        const packaged = reportPackageExactSubmission(request)!;
        expect(packaged.exactResultText, item.name).toBe(submission.exactResultText);
        expect(packaged.exactIntakeText, item.name).toBe(submission.exactIntakeText);
      }
      const content = `${JSON.stringify({
        schema_version: "1.0.0",
        generator: GENERATOR,
        description: "DEL-08-01 REQ-024 connecting-path witness: the render arguments the desktop's report build step sends the native host for an exact-route result that is solved but not numerically eligible (the delivered result bytes are the committed raw file). Job ids are invented; unit transport replay of committed producer bytes, not a native UI qualification witness.",
        shape: item.name,
        raw_path: item.rawPath,
        request_path: item.requestPath,
        solver_mode: "sparse_interactive",
        exact_intake_text: submission.exactIntakeText,
        input_manifest_text: submission.inputManifestText,
        input: assembly.controlled.payload
      })}\n`;
      // What the native host's tests need besides the render arguments, to rebuild both
      // requests from the committed raw result and require them equal to these: the
      // run record and the user-released request are derived natively, the local-private
      // request is composed from them with the render input as its report, and each is
      // compared with the web view's by its canonical SHA-256. Only the gap witness also
      // carries its local-private section set (the friction witness's is 8.5 MB).
      expect(localPrivate.source_results[0], item.name).toBe(result);
      expect(userReleased.source_results[0], item.name).toBe(result);
      expect(userReleased.result_documents, item.name).toEqual(localPrivate.result_documents);
      expect(userReleased.audit_manifest, item.name).toEqual(localPrivate.audit_manifest);
      const packageContent = `${JSON.stringify({
        schema_version: "1.0.0",
        generator: GENERATOR,
        description: "DEL-08-01 REQ-024 connecting-path witness: what the native host's tests need, beside the render arguments and the committed raw result, to rebuild the report package request and the user-released package request the desktop builds for this result, and the canonical SHA-256 of each request and of the run record as the desktop built them. Job ids and the user-release time are invented; unit transport replay, not a native UI qualification witness.",
        shape: item.name,
        render_path: `${DIR}/${item.name}.render.json`,
        raw_path: item.rawPath,
        input_manifest: inputManifest,
        user_release: record,
        run_record_sha256: await canonicalSha256Hex(analysisRun),
        local_private_request_sha256: await canonicalSha256Hex(localPrivate),
        user_released_request_sha256: await canonicalSha256Hex(userReleased),
        local_private_sections: item.name === "pressure_3_gap_unresolved" ? localPrivate.state_comparison_handoff_records : null
      })}\n`;
      for (const [name, body] of [[`${item.name}.render.json`, content], [`${item.name}.package.json`, packageContent]] as const) {
        if (outDir) {
          mkdirSync(outDir, { recursive: true });
          writeFileSync(resolve(outDir, name), body);
          continue;
        }
        expect(text(`${DIR}/${name}`) === body, `${DIR}/${name} is stale; regenerate it with ${WRITE}=<dir>`).toBe(true);
      }
    }
  }, 120_000);
});
