// The report-package request (W-18 (b2); DEL-08-01 Design `report-package-current-results.md`
// §2.1-§2.5): the route gate per identity, admission through the result-export builder,
// rule-check threading and the solve/rule-check rule-pack agreement, and the request's
// members. Unit transport replay of committed producer bytes through mocked IPC; NOT a
// native UI qualification witness. Every rule pack and rule-check outcome here is invented.
import { describe, expect, it, afterEach, vi } from "vitest";
import { createNativeMechanicsReplay, createSolveJobTransport, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
afterEach(() => { invokeMock.mockReset(); vi.restoreAllMocks(); delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__; });
import { buildAnalysisRunPreview, buildPreviewComparison, runPreviewMechanics } from "../../services/previewService";
import { canonicalJsonString, canonicalSha256Hex } from "../../services/hashService";
import { buildCurrentSessionInputManifest, type InputManifestRulePack } from "../../services/inputManifestService";
import type { RuleCheckStatus } from "../../services/ruleCheckService";
import previewConnectedSparse from "../../../../../fixtures/results/preview_physics_connected_sparse.json";
import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import inventedPreviewModel from "../../../../../fixtures/product_preview/invented_preview_model.json";
import { buildAnalysisRunV03 } from "../../services/analysisRunCompatibility";
import type { MechanicsResult, PreviewModel, RulePackReference } from "../../types";
import { sourceContract } from "../results/resultReader";
import { PREVIEW_PHYSICS_CONTRACT_ID } from "../results/numericalResultQuality";
import { PRESSURE_2_OUTPUT_REFUSAL, PRESSURE_SOURCE_OUTPUT_REFUSAL, surfaceRouteRefusal } from "../results/outputPolicy";
import * as resultExportAdapter from "../result-export/resultExportAdapter";
import {
  buildReportPackageRequest,
  checkRulePacksAgree,
  reportPackageUnavailableReason,
  REPORT_PACKAGE_DESKTOP_PROFILE,
  REPORT_PACKAGE_ROUTE_NOT_ADMITTED,
  REPORT_PACKAGE_RULE_CHECK_BINDING_MISMATCH,
  REPORT_PACKAGE_RULE_PACK_CHECKSUM_BASIS_UNSUPPORTED,
  REPORT_PACKAGE_RULE_PACKS_DISAGREE,
  type ReportPackageRequestInput
} from "./reportPackageRequest";
import { numericalResultStanding } from "../results/resultStanding";
import { buildRenderableReportInput, type SessionRuleCheckRun } from "./renderableReportInput";
import { reportResultRows } from "./reportResultRows";
import componentProvenanceProjection from "../../../../../fixtures/reports/invented/component_provenance_cross_layer_projection.json";

const SOLVER = {
  solver_name: "open_pipe_stress_product_physics",
  solver_version: "0.2.0",
  solver_mode: "sparse_interactive",
  settings: { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: true }
};
const hex = (digit: string) => digit.repeat(64);

function inventedReference(id: string, checksum: string, version = "1.0.0"): RulePackReference {
  return {
    rule_pack_id: id,
    name: `Invented pack ${id}`,
    rule_pack_version: version,
    checksum: { algorithm: "sha256", canonicalization: "JCS", payload_scope: "rule_pack_payload", payload_excludes: ["checksums"], value: checksum },
    redistribution_status: "invented_non_engineering_example",
    source_notice: `Invented notice for ${id}; not an engineering basis.`
  };
}
function activePack(id: string, checksum: string, version = "1.0.0"): InputManifestRulePack {
  return { rule_pack_id: id, version, checksum_sha256: checksum, source_notice: `Invented notice for ${id}; not an engineering basis.` };
}
/** An invented rule-check run over an invented loaded pack (unit fixture, not rule evaluation). */
function inventedRun(id: string, aggregate: RuleCheckStatus, checksum: Record<string, unknown> | null = { algorithm: "sha256", canonicalization: "JCS", value: hex("a") }, version = "1.0.0"): SessionRuleCheckRun {
  return {
    rulePackDocument: {
      metadata: { rule_pack_id: id, name: `Invented pack ${id}`, rule_pack_version: version, source_notice: `Invented notice for ${id}; not an engineering basis.` },
      classification: { redistribution_status: "invented_non_engineering_example" },
      ...(checksum ? { checksums: { rule_pack_checksum: checksum } } : {})
    },
    result: {
      document_kind: "openpipestress.rule_check.run",
      rule_pack_id: id,
      grammar_version: "1.0.0",
      aggregate_status: aggregate,
      checks: [{ check_id: "invented_check", status: aggregate, acceptability_relation: "invented", bound_inputs: [], completeness_findings: [], evaluator_findings: [], diagnostic_codes: [] }],
      professional_boundary_notice: "Invented notice."
    }
  };
}

type SessionOptions = { solverBuildRef?: string; references?: RulePackReference[]; activeRulePacks?: InputManifestRulePack[]; ruleCheckAggregate?: RuleCheckStatus | null };

/** A Current preview-physics-2 session through the unit replay: the captured pair's model
 * (with `references`, when given), its native solve, manifest and 0.3.0 record. */
async function currentSession(options: SessionOptions = {}) {
  const pair = nativeMechanicsReplayPair("sparse_interactive", { profile: "preview" });
  const model = structuredClone(pair.model);
  if (options.references) model.rule_pack_refs = options.references;
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(options.references
    ? createSolveJobTransport(() => pair.bytes, { jobIdPrefix: "unit-report-package-references" }).invoke
    : createNativeMechanicsReplay({ profile: "preview" }).invoke);
  const result = await runPreviewMechanics(model);
  const inputManifest = await buildCurrentSessionInputManifest({
    model,
    solver: { ...SOLVER, solver_build_ref: options.solverBuildRef ?? "open_pipe_stress_product_physics@0.2.0" },
    active_rule_packs: options.activeRulePacks ?? [],
    external_assets: []
  });
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest, ruleCheckAggregate: options.ruleCheckAggregate ?? null });
  return { model, result, inputManifest, analysisRun };
}

function input(session: Awaited<ReturnType<typeof currentSession>>, extra: Partial<ReportPackageRequestInput> = {}): ReportPackageRequestInput {
  return { ...session, projectSummary: null, comparison: null, ruleCheckAggregate: null, ruleCheckRun: null, ...extra };
}

describe("the route gate (§2.1): the output policy's report-package cell", () => {
  it("admits the pressure-free route to the builder's own checks", async () => {
    const { result } = await currentSession();
    expect(sourceContract(result)).toBe("preview_physics");
    expect(reportPackageUnavailableReason(result)).toBeNull();
    expect(reportPackageUnavailableReason(null)).toBeNull();
  });

  it("keeps a genuine exact-route source (pressure-2) refused with PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE (ruling 10A)", async () => {
    const { model } = nativeMechanicsReplayPair("sparse_interactive", { profile: "pressure" });
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(createNativeMechanicsReplay({ profile: "pressure" }).invoke);
    const result = await runPreviewMechanics(model);
    expect(sourceContract(result)).toBe("pressure");
    expect(numericalResultStanding(result, model).eligible).toBe(true);
    const before = JSON.stringify(result);
    expect(reportPackageUnavailableReason(result)).toBe(PRESSURE_2_OUTPUT_REFUSAL);
    const exportSpy = vi.spyOn(resultExportAdapter, "buildCurrentResultExport");
    await expect(buildReportPackageRequest({ model, result, analysisRun: null as never, inputManifest: null as never, projectSummary: null, comparison: null, ruleCheckAggregate: null, ruleCheckRun: null }))
      .rejects.toThrow(new Error(PRESSURE_2_OUTPUT_REFUSAL));
    expect(exportSpy).not.toHaveBeenCalled();
    expect(JSON.stringify(result)).toBe(before);
  });

  it("refuses an eligible registered pressure-source-1 source by its cell", async () => {
    const root = resolve(__dirname, "../../../../../");
    const folder = "fixtures/product_preview/pressure_source/physics_source";
    const model = JSON.parse(readFileSync(resolve(root, `${folder}/n05.request.json`), "utf8")).model as PreviewModel;
    const bytes = readFileSync(resolve(root, `${folder}/n05-sparse_interactive.raw.json`), "utf8");
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(createSolveJobTransport((sent) => {
      expect(sent.model).toEqual(model);
      return bytes;
    }).invoke);
    const result = await runPreviewMechanics(model);
    expect(sourceContract(result)).toBe("pressure_source");
    expect(numericalResultStanding(result, model).eligible).toBe(true);
    expect(reportPackageUnavailableReason(result)).toBe(PRESSURE_SOURCE_OUTPUT_REFUSAL);
    expect(PRESSURE_SOURCE_OUTPUT_REFUSAL.startsWith("PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE: ")).toBe(true);
    await expect(buildReportPackageRequest({ model, result, analysisRun: null as never, inputManifest: null as never, projectSummary: null, comparison: null, ruleCheckAggregate: null, ruleCheckRun: null }))
      .rejects.toThrow(new Error(PRESSURE_SOURCE_OUTPUT_REFUSAL));
  });

  it.each([
    "fixtures/product_preview/pressure/load_reference/connected-sparse_interactive.raw.json",
    "fixtures/product_preview/pressure/load_reference/pressure-dense_scrutiny.raw.json",
    "fixtures/product_preview/pressure_source/load_reference_source/mixed-sparse_interactive.raw.json",
    "fixtures/product_preview/pressure_source/load_reference_source/eigen_motion-dense_scrutiny.raw.json",
    "core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-sparse_interactive.raw.json",
  ])("refuses the load/reference-state result %s with its route's refusal, whatever its standing", path => {
    const result = JSON.parse(readFileSync(resolve(__dirname, "../../../../../", path), "utf8")) as MechanicsResult;
    expect(["pressure", "pressure_source"]).toContain(sourceContract(result));
    const refusal = reportPackageUnavailableReason(result);
    expect(refusal).toBe(surfaceRouteRefusal(result, "report-package"));
    expect(refusal?.startsWith("PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE: ")).toBe(true);
  });
});

describe("admission and the request's members (§2.1-§2.4)", () => {
  it("builds the 2.0.0 request whose result member is the Export Result JSON document", async () => {
    const session = await currentSession();
    const comparison = buildPreviewComparison({ result: session.result, analysisRun: session.analysisRun });
    const before = JSON.stringify(session);
    const request = await buildReportPackageRequest(input(session, { comparison }));
    const { model, result, inputManifest, analysisRun } = session;

    // The result member is canonically equal to the document Export Result JSON saves.
    const document = await resultExportAdapter.buildCurrentResultExport({ model, result, analysisRun, inputManifest });
    expect(request.result_documents).toHaveLength(1);
    expect(await canonicalJsonString(request.result_documents[0])).toBe(await canonicalJsonString(document));
    expect(request.result_documents[0].result_envelope.producer.semantic_contract_id).toBe(PREVIEW_PHYSICS_CONTRACT_ID);
    expect(request.source_results).toEqual([result]);
    expect(request.export_profile_id).toBe(REPORT_PACKAGE_DESKTOP_PROFILE);
    expect(request.source_model_ref).toEqual({ ref_type: "model", ref_id: model.project.id });
    expect(request.source_basis_refs[0]).toEqual({ ref_type: "analysis_run", ref_id: result.run_id });
    expect(request.source_basis_refs.slice(1)).toEqual(document.result_envelope.load_basis_refs);

    // §2.4: the audit manifest is named by the input-manifest reference, its unit system is
    // the document's, and both hashes carry the basis they were computed on.
    const modelHash = await canonicalSha256Hex(model);
    expect(request.audit_manifest).toMatchObject({
      manifest_id: inputManifest.manifest_ref.ref,
      model_hash: { algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_kind: "model_json", payload_ref: model.project.id, value: modelHash },
      input_manifest_hash: { algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_kind: "input_manifest_json", payload_ref: inputManifest.manifest_ref.ref, value: inputManifest.manifest_sha256 },
      solver_version: { solver_name: "open_pipe_stress_product_physics", solver_version: "0.2.0", solver_build_ref: "open_pipe_stress_product_physics@0.2.0" },
      unit_system_ref: `${model.project.id}:units`,
      rule_pack_refs: [],
      assets: []
    });
    expect(document.result_envelope.reproducibility.audit_manifest_ref.ref_id).toBe(request.audit_manifest.manifest_id);
    expect(document.result_envelope.reproducibility.model_hash.value).toBe(modelHash);

    // The report renders under local_private with no generation time, its rows the
    // source rows' projection, and no rule check.
    expect(request.report.report_context.export_context).toBe("local_private");
    expect(request.report.report_context.generated_at).toBeNull();
    expect(request.report.result_rows).toEqual(reportResultRows(result));
    expect(request.report.user_rule_checks).toBeNull();
    expect(request.report.calculation_report.rule_pack_refs).toEqual([]);
    expect(request.rule_check_aggregate).toBeNull();
    expect(request.solve_rule_check_status).toBe(result.status.rule_check);
    expect(request.state_comparison_handoff_records).toHaveLength(1);
    expect(request.state_comparison_handoff_records[0]).toMatchObject({ deliverable_id: "DEL-08-06", section_set_id: `desktop-current-session:${result.run_id}` });

    const packageJson = JSON.stringify(request.report);
    expect(packageJson).not.toContain("invented_public_example");
    expect(packageJson).toContain('"privacy_classification":"private_project_data"');
    expect(JSON.stringify(session)).toBe(before);
  });

  it("refuses a document whose identity is not preview-physics-2 (REPORT-PACKAGE-ROUTE-NOT-ADMITTED)", async () => {
    const session = await currentSession();
    const real = resultExportAdapter.buildCurrentResultExport;
    vi.spyOn(resultExportAdapter, "buildCurrentResultExport").mockImplementation(async (args) => {
      const document = await real(args);
      document.result_envelope.producer.semantic_contract_id = "openpipestress.result_semantics/0.3.0/pressure-2";
      return document;
    });
    await expect(buildReportPackageRequest(input(session))).rejects.toThrow(REPORT_PACKAGE_ROUTE_NOT_ADMITTED);
  });

  it("surfaces each result-export builder refusal with its own code", async () => {
    const session = await currentSession();
    // A copy carries no registered native invocation.
    await expect(buildReportPackageRequest(input(session, { result: structuredClone(session.result) })))
      .rejects.toThrow("CURRENT_NATIVE_INVOCATION_REQUIRED");
    await expect(buildReportPackageRequest(input(session, { inputManifest: null as never })))
      .rejects.toThrow("CURRENT_INPUT_MANIFEST_UNAVAILABLE");
    const changedModel = structuredClone(session.model);
    changedModel.project.description = `${changedModel.project.description} changed after manifest`;
    await expect(buildReportPackageRequest(input(session, { model: changedModel }))).rejects.toThrow("CURRENT_MODEL_PAYLOAD_MISMATCH");
  });

  it("preserves a consistent recorded build reference in the document and the audit manifest", async () => {
    const recordedBuild = "build:captured-historical-product-physics";
    const session = await currentSession({ solverBuildRef: recordedBuild });
    const request = await buildReportPackageRequest(input(session));
    expect(request.result_documents[0].result_envelope.solver_version.solver_build_ref).toBe(recordedBuild);
    expect(request.audit_manifest.solver_version.solver_build_ref).toBe(recordedBuild);
    expect(request.report.report_context.solver_build_ref).toBe(recordedBuild);
  });
});

describe("rule-check threading (§2.5)", () => {
  it("refuses an aggregate without its run, and a run without its aggregate", async () => {
    const session = await currentSession({ ruleCheckAggregate: "RULE_INPUTS_INCOMPLETE" });
    await expect(buildReportPackageRequest(input(session, { ruleCheckAggregate: "RULE_INPUTS_INCOMPLETE" })))
      .rejects.toThrow(REPORT_PACKAGE_RULE_CHECK_BINDING_MISMATCH);
    const plain = await currentSession();
    await expect(buildReportPackageRequest(input(plain, { ruleCheckRun: inventedRun("invented-pack-a", "USER_RULE_CHECKED") })))
      .rejects.toThrow(REPORT_PACKAGE_RULE_CHECK_BINDING_MISMATCH);
  });

  it("refuses a run whose aggregate is not the session's, and a record whose rule token is not the aggregate", async () => {
    const session = await currentSession({ references: [inventedReference("invented-pack-a", hex("a"))], activeRulePacks: [activePack("invented-pack-a", hex("a"))], ruleCheckAggregate: "USER_RULE_CHECKED" });
    await expect(buildReportPackageRequest(input(session, { ruleCheckAggregate: "USER_RULE_CHECKED", ruleCheckRun: inventedRun("invented-pack-a", "USER_RULE_FAILED") })))
      .rejects.toThrow(REPORT_PACKAGE_RULE_CHECK_BINDING_MISMATCH);
    // The record still carries the solve's status while the session names an aggregate.
    const unrecorded = await currentSession({ references: [inventedReference("invented-pack-a", hex("a"))], activeRulePacks: [activePack("invented-pack-a", hex("a"))] });
    await expect(buildReportPackageRequest(input(unrecorded, { ruleCheckAggregate: "USER_RULE_CHECKED", ruleCheckRun: inventedRun("invented-pack-a", "USER_RULE_CHECKED") })))
      .rejects.toThrow(/REPORT-PACKAGE-RULE-CHECK-BINDING-MISMATCH: the analysis record's rule status/);
  });

  it("refuses a run whose outcomes name another pack than its loaded document", async () => {
    const session = await currentSession({ references: [inventedReference("invented-pack-a", hex("a"))], activeRulePacks: [activePack("invented-pack-a", hex("a"))], ruleCheckAggregate: "USER_RULE_CHECKED" });
    const run = inventedRun("invented-pack-a", "USER_RULE_CHECKED");
    run.result.rule_pack_id = "invented-pack-other";
    await expect(buildReportPackageRequest(input(session, { ruleCheckAggregate: "USER_RULE_CHECKED", ruleCheckRun: run })))
      .rejects.toThrow("REPORT-RULE-CHECK-PACK-MISMATCH");
  });
});

describe("solve and rule-check rule packs agree (§2.5, decided)", () => {
  it("saves equal sets: the rule check of the model's resolved pack, carried in the report and the audit projection", async () => {
    const session = await currentSession({ references: [inventedReference("invented-pack-a", hex("a"))], activeRulePacks: [activePack("invented-pack-a", hex("a"))], ruleCheckAggregate: "USER_RULE_CHECKED" });
    const run = inventedRun("invented-pack-a", "USER_RULE_CHECKED");
    const request = await buildReportPackageRequest(input(session, { ruleCheckAggregate: "USER_RULE_CHECKED", ruleCheckRun: run }));
    expect(request.rule_check_aggregate).toBe("USER_RULE_CHECKED");
    expect(request.report.user_rule_checks).toMatchObject({ rule_pack_id: "invented-pack-a", aggregate_status: "USER_RULE_CHECKED" });
    expect(request.report.calculation_report.rule_pack_refs).toEqual([expect.objectContaining({
      rule_pack_id: "invented-pack-a", version: "1.0.0",
      checksum: { algorithm: "sha256", canonicalization: "JCS", payload_ref: { ref_type: "rule_pack", ref_id: "invented-pack-a" }, value: hex("a") }
    })]);
    expect(request.audit_manifest.rule_pack_refs).toEqual([{
      rule_pack_id: "invented-pack-a",
      rule_pack_version: "1.0.0",
      source_notice: "Invented notice for invented-pack-a; not an engineering basis.",
      redistribution_status: "invented_non_engineering_example",
      checksum: { algorithm: "sha256", canonicalization: "rule_pack_payload_jcs", payload_kind: "rule_pack_reference", payload_ref: "invented-pack-a", value: hex("a") },
      private_payload_redacted: true
    }]);
    expect(request.result_documents[0].result_envelope.analysis_status).toContain("USER_RULE_CHECKED");
  });

  it("refuses a rule check of a different pack than the solve recorded, naming both sets", async () => {
    const session = await currentSession({ references: [inventedReference("invented-pack-a", hex("a"))], activeRulePacks: [activePack("invented-pack-a", hex("a"))], ruleCheckAggregate: "USER_RULE_CHECKED" });
    const other = inventedRun("invented-pack-b", "USER_RULE_CHECKED", { algorithm: "sha256", canonicalization: "JCS", value: hex("b") });
    const refusal = buildReportPackageRequest(input(session, { ruleCheckAggregate: "USER_RULE_CHECKED", ruleCheckRun: other }));
    await expect(refusal).rejects.toThrow(REPORT_PACKAGE_RULE_PACKS_DISAGREE);
    await expect(refusal).rejects.toThrow(`invented-pack-b@1.0.0 (sha256 ${hex("b")})`);
    await expect(refusal).rejects.toThrow(`invented-pack-a@1.0.0 (sha256 ${hex("a")})`);
  });

  it("refuses a solve pack with no rule check", async () => {
    const session = await currentSession({ references: [inventedReference("invented-pack-a", hex("a"))], activeRulePacks: [activePack("invented-pack-a", hex("a"))] });
    await expect(buildReportPackageRequest(input(session))).rejects.toThrow(
      new RegExp(`^${REPORT_PACKAGE_RULE_PACKS_DISAGREE}: the package's rule packs \\(the rule check's: none\\)`)
    );
  });

  it("refuses a rule check with no solve pack", async () => {
    const session = await currentSession({ ruleCheckAggregate: "USER_RULE_CHECKED" });
    await expect(buildReportPackageRequest(input(session, { ruleCheckAggregate: "USER_RULE_CHECKED", ruleCheckRun: inventedRun("invented-pack-a", "USER_RULE_CHECKED") })))
      .rejects.toThrow(/REPORT-PACKAGE-RULE-PACKS-DISAGREE: .*are not the packs the solve recorded \(none\)/);
  });

  it("compares as sets of {rule_pack_id, version, checksum value}", () => {
    const ref = (id: string, version: string, value: string) => ({ rule_pack_id: id, version, checksum: { value } });
    expect(() => checkRulePacksAgree([], [])).not.toThrow();
    expect(() => checkRulePacksAgree([ref("p", "1", hex("a"))], [activePack("p", hex("a"), "1")])).not.toThrow();
    expect(() => checkRulePacksAgree([ref("p", "2", hex("a"))], [activePack("p", hex("a"), "1")])).toThrow(REPORT_PACKAGE_RULE_PACKS_DISAGREE);
    expect(() => checkRulePacksAgree([ref("p", "1", hex("b"))], [activePack("p", hex("a"), "1")])).toThrow(REPORT_PACKAGE_RULE_PACKS_DISAGREE);
    expect(() => checkRulePacksAgree([ref("p", "1", hex("a"))], [activePack("p", hex("a"), "1"), activePack("q", hex("c"), "1")])).toThrow(REPORT_PACKAGE_RULE_PACKS_DISAGREE);
  });

  it("compares as sets: a repeated pack counts once and does not stand in for another", () => {
    const ref = (id: string, version: string, value: string) => ({ rule_pack_id: id, version, checksum: { value } });
    expect(() => checkRulePacksAgree([ref("p", "1", hex("a")), ref("p", "1", hex("a"))], [activePack("p", hex("a"), "1")])).not.toThrow();
    expect(() => checkRulePacksAgree([ref("p", "1", hex("a"))], [activePack("p", hex("a"), "1"), activePack("p", hex("a"), "1")])).not.toThrow();
    expect(() => checkRulePacksAgree([ref("p", "1", hex("a")), ref("p", "1", hex("a"))], [activePack("p", hex("a"), "1"), activePack("q", hex("c"), "1")]))
      .toThrow(REPORT_PACKAGE_RULE_PACKS_DISAGREE);
    expect(() => checkRulePacksAgree([ref("p", "1", hex("a")), ref("q", "1", hex("c"))], [activePack("p", hex("a"), "1"), activePack("p", hex("a"), "1")]))
      .toThrow(REPORT_PACKAGE_RULE_PACKS_DISAGREE);
  });

  it("keys a reference without a checksum value as not recorded, which no recorded SHA-256 equals", () => {
    expect(() => checkRulePacksAgree([{ rule_pack_id: "p", version: "1" }], [activePack("p", hex("a"), "1")]))
      .toThrow(/the rule check's: p@1 \(sha256 not recorded\)/);
    expect(() => checkRulePacksAgree([{ rule_pack_id: "p", version: "1", checksum: null }], [activePack("p", hex("a"), "1")]))
      .toThrow(REPORT_PACKAGE_RULE_PACKS_DISAGREE);
  });

  it("compares a pack with no declared checksum by its loaded document's SHA-256 (the fallback)", async () => {
    const run = inventedRun("invented-pack-a", "USER_RULE_CHECKED", null);
    const fallback = await canonicalSha256Hex(run.rulePackDocument);
    // The solve recorded the pack's stamped checksum, which is not the fallback.
    const stamped = await currentSession({ references: [inventedReference("invented-pack-a", hex("a"))], activeRulePacks: [activePack("invented-pack-a", hex("a"))], ruleCheckAggregate: "USER_RULE_CHECKED" });
    const refusal = buildReportPackageRequest(input(stamped, { ruleCheckAggregate: "USER_RULE_CHECKED", ruleCheckRun: run }));
    await expect(refusal).rejects.toThrow(REPORT_PACKAGE_RULE_PACKS_DISAGREE);
    await expect(refusal).rejects.toThrow(`the rule check's: invented-pack-a@1.0.0 (sha256 ${fallback})`);
    // A solve that recorded that value agrees, and the fallback is projected as rfc8785_jcs.
    const recorded = await currentSession({ references: [inventedReference("invented-pack-a", fallback)], activeRulePacks: [activePack("invented-pack-a", fallback)], ruleCheckAggregate: "USER_RULE_CHECKED" });
    const request = await buildReportPackageRequest(input(recorded, { ruleCheckAggregate: "USER_RULE_CHECKED", ruleCheckRun: run }));
    expect(request.report.calculation_report.rule_pack_refs[0].checksum).toEqual({
      algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_ref: { ref_type: "rule_pack_document", ref_id: "invented-pack-a" }, value: fallback
    });
    expect(request.audit_manifest.rule_pack_refs[0].checksum).toMatchObject({ canonicalization: "rfc8785_jcs", payload_ref: "rule_pack_document:invented-pack-a", value: fallback });
  });

  it("refuses a rule-pack checksum pair the audit manifest cannot name", async () => {
    const session = await currentSession({ references: [inventedReference("invented-pack-a", hex("a"))], activeRulePacks: [activePack("invented-pack-a", hex("a"))], ruleCheckAggregate: "USER_RULE_CHECKED" });
    const run = inventedRun("invented-pack-a", "USER_RULE_CHECKED", { algorithm: "sha512", canonicalization: "JCS", value: hex("a") });
    await expect(buildReportPackageRequest(input(session, { ruleCheckAggregate: "USER_RULE_CHECKED", ruleCheckRun: run })))
      .rejects.toThrow(`${REPORT_PACKAGE_RULE_PACK_CHECKSUM_BASIS_UNSUPPORTED}: the rule pack's checksum sha512/JCS`);
  });
});

// Pure report projection (no package): the component-provenance and hanger oracles of
// the rendered report input the package carries.
describe("the report input the package carries", () => {
  async function syntheticPreviewSession(model: PreviewModel) {
    const result = structuredClone(previewConnectedSparse) as unknown as MechanicsResult;
    result.model_ref = model.project.id;
    const inputManifest = await buildCurrentSessionInputManifest({
      model,
      solver: { solver_name: "open_pipe_stress_product_physics", solver_version: "0.2.0", solver_build_ref: "open_pipe_stress_product_physics@0.2.0", solver_mode: "sparse_interactive", settings: {} },
      active_rule_packs: [], external_assets: []
    });
    const analysisRun = await buildAnalysisRunV03(result, inputManifest);
    return { model, result, inputManifest, analysisRun };
  }

  it("preserves the component-provenance oracle", async () => {
    const modelWithMissingProvenance = structuredClone(inventedPreviewModel) as unknown as PreviewModel;
    const missingComponent = modelWithMissingProvenance.components.find((component) => component.id === "component:C-140");
    expect(missingComponent).toBeDefined();
    missingComponent!.provenance = "";
    const { model, result, analysisRun } = await syntheticPreviewSession(modelWithMissingProvenance);
    const projected = await buildRenderableReportInput({ model, result, analysisRun, projectSummary: null, renderContext: { exportContext: "local_private", generatedAt: null } });
    const sections = projected.report_sections;
    const projection = {
      schema_version: "1.0.0",
      present_component: {
        value: sections.user_supplied_values.find((value) => value.value_id === "component-provenance:component:C-110"),
        provenance_note: sections.provenance_notes.find((note) => note.source_name === "Invented elbow marker component provenance")
      },
      missing_component: {
        value: sections.user_supplied_values.find((value) => value.value_id === "component-provenance:component:C-140"),
        provenance_note: sections.provenance_notes.find((note) => note.source_name === "Invented tie-in marker component provenance"),
        diagnostic: sections.diagnostics.find((diagnostic) => diagnostic.code === "COMPONENT_PROVENANCE_MISSING" && diagnostic.affected_object.ref_id === "component:C-140")
      }
    };
    // The shared projection the Rust consumer reads (`report_package/src/wire.rs`). Written
    // only with OPENPIPESTRESS_WRITE_COMPONENT_PROVENANCE_PROJECTION=<dir>; otherwise stale fails.
    const outDir = process.env.OPENPIPESTRESS_WRITE_COMPONENT_PROVENANCE_PROJECTION;
    if (outDir) {
      mkdirSync(outDir, { recursive: true });
      writeFileSync(resolve(outDir, "component_provenance_cross_layer_projection.json"), `${JSON.stringify(projection, null, 2)}\n`);
    }
    expect(projection).toEqual(componentProvenanceProjection);
  });

  it("preserves the hanger input oracles", async () => {
    const session = await syntheticPreviewSession(structuredClone(demoModel) as PreviewModel);
    const before = JSON.stringify(session);
    const report = await buildRenderableReportInput({ ...session, projectSummary: null, renderContext: { exportContext: "local_private", generatedAt: null } });
    const reportValues = report.report_sections.user_supplied_values;
    expect(reportValues.find((item) => item.value_id === "spring-hanger:support:SH-140")).toMatchObject({
      quantity: { magnitude: 390, unit: "N", dimension: "force" }, required_for: ["reporting", "human_review"], missing_data_finding: false
    });
    expect(reportValues.find((item) => item.value_id === "spring-hanger:support:CE-120")).toMatchObject({
      quantity: { magnitude: 375, unit: "N", dimension: "force" }, required_for: ["reporting", "human_review"], missing_data_finding: false
    });
    expect(JSON.stringify(session)).toBe(before);
  });
});
