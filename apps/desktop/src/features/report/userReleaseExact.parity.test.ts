// D-6 / RT-R5. Invented fixture replay; no user release or product release.
import { readFileSync, writeFileSync } from "node:fs";
import path from "node:path";
import { afterEach, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import { createSolveJobTransport } from "../../test/nativeMechanicsReplay";
import { buildAnalysisRunPreview, resetSessionResultCustodyForTests, runPreviewMechanics } from "../../services/previewService";
import { canonicalSha256Hex } from "../../services/hashService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { buildCurrentResultExport } from "../result-export/resultExportAdapter";
import { assembleUserReleasedReportPackageRequest, reportPackageExactSubmission } from "./reportPackageRequest";

const root = path.resolve(__dirname, "../../../../..");
const load = (p: string) => JSON.parse(readFileSync(path.join(root, p), "utf8"));
afterEach(() => { invokeMock.mockReset(); resetSessionResultCustodyForTests(); delete (window as any).__TAURI_INTERNALS__; });

for (const shape of ["pressure_2_straight", "pressure_source_1"]) {
  it(`D-6 derives ${shape} with exact binding, Diagnostics and disclosures`, async () => {
    const fixture = load(`fixtures/reports/invented/exact_report_package_requests/${shape}.json`);
    const model = fixture.model;
    const bytes = readFileSync(path.join(root, fixture.raw_path), "utf8");
    (window as any).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(createSolveJobTransport(() => bytes, { jobIdPrefix: `invented-d6-${shape}` }).invoke);
    const result = await runPreviewMechanics(model);
    const producer = result.producer!;
    const inputManifest = await buildCurrentSessionInputManifest({ model, solver: {
      solver_name: producer.component_name, solver_version: producer.component_version,
      solver_build_ref: `${producer.component_name}@${producer.component_version}`, solver_mode: "sparse_interactive",
      settings: { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: true }
    }, active_rule_packs: [], external_assets: [] });
    const analysisRun = await buildAnalysisRunPreview(result, { inputManifest });
    const document = await buildCurrentResultExport({ model, result, analysisRun, inputManifest });
    const userRelease = structuredClone(load("fixtures/reports/invented/user_release_record.json"));
    userRelease.record_version = "1.1.0";
    userRelease.scope.not_carried.push("ineligible_exact_route_results");
    userRelease.scope.report_id = `report:${result.run_id}`;
    userRelease.scope.package_id = `desktop-user-released-report-${result.run_id.replace(/[^A-Za-z0-9._-]+/g, "-").replace(/^-+|-+$/g, "")}`;
    userRelease.bound_to = {
      model_ref: model.project.id, model_document_sha256: await canonicalSha256Hex(model), analysis_run_ref: result.run_id,
      input_manifest_ref: inputManifest.manifest_ref.ref, input_manifest_sha256: inputManifest.manifest_sha256,
      result_envelope_id: (document as any).result_envelope.envelope_id,
      result_document_derivative_sha256: (document as any).result_envelope.reproducibility.derivative_hash.value
    };
    const request = await assembleUserReleasedReportPackageRequest({ model, result, analysisRun, inputManifest, userRelease });
    const report = request.report as any;
    expect(report.exact_route.admission.semantic_contract_id).toBe(result.producer!.semantic_contract_id);
    expect(report.exact_route.admission.received_bytes_sha256).toMatch(/^[0-9a-f]{64}$/);
    expect(report.exact_route.headlines).toHaveLength(2);
    expect(report.report_sections.diagnostics).toHaveLength(result.diagnostics.length);
    expect(report.report_sections.limitations).toHaveLength(result.formulation_basis!.limitations.length + 2);
    expect(report.report_sections.limitations[0].limitation_id).toMatch(/:technical-preview$/);
    expect(report.report_context.user_release).toEqual(userRelease);
    expect(reportPackageExactSubmission(request)).not.toBeNull();
    const vector = { shape, model, source: result, solver_mode: "sparse_interactive", software: { name: "SWBPIPE", version: "0.1.0" },
      run_record: analysisRun, input_manifest: inputManifest.manifest, user_release: userRelease, request,
      exact_intake_text: fixture.exact_intake_text, exact_result_text: bytes };
    const target = process.env.OPENPIPESTRESS_WRITE_D6_VECTORS;
    if (target) writeFileSync(path.join(target, `${shape}.json`), JSON.stringify(vector) + "\n");
    else expect(JSON.parse(JSON.stringify(vector))).toEqual(load(`fixtures/reports/invented/user_release_exact/${shape}.json`));
  });
}
