import { describe, expect, it, afterEach, vi } from "vitest";
import { createNativeMechanicsReplay, nativeMechanicsReplayPair } from "../../test/nativeMechanicsReplay";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));
afterEach(() => { invokeMock.mockReset(); delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__; });
import { buildAnalysisRunPreview, buildPreviewComparison, loadPreviewModel, runPreviewMechanics } from "../../services/previewService";
import { canonicalSha256Hex, canonicalSha256HexCheckedV1 } from "../../services/hashService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import historicalResult from "../../../../../fixtures/product_preview/invented_mechanics_result.json";
import { analysisRecordProjection, buildAnalysisRunV02, verifyAnalysisRunRecord } from "../../services/analysisRunCompatibility";
import type { MechanicsResult, PreviewModel } from "../../types";
import {resultSemantics} from "../results/resultSemantics";
import { buildReportPackageRequest, projectReceivedReportResults } from "./reportPackageRequest";
import { buildRenderableReportInput } from "./renderableReportInput";
import componentProvenanceProjection from "../../../../../fixtures/reports/invented/component_provenance_cross_layer_projection.json";

// Actual captured full-UI pair through unit IPC replay, NOT native UI qualification.
async function currentSession(profile: "precision" | "physics" = "precision") {
  const { model } = nativeMechanicsReplayPair("sparse_interactive", { profile });
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  invokeMock.mockImplementation(createNativeMechanicsReplay({ profile }).invoke);
  const result = await runPreviewMechanics(model);
  const inputManifest = await buildCurrentSessionInputManifest({
    model,
    solver: {
      solver_name: "open_pipe_stress_product_physics",
      solver_version: "0.2.0",
      solver_build_ref: "open_pipe_stress_product_physics@0.2.0",
      solver_mode: "sparse_interactive",
      settings: {
        nonlinear_iteration_policy:
          "DEC-046-CV-B-product-preview-active-set-count-v1",
        sparse_evidence_lane: true
      }
    },
    active_rule_packs: [],
    external_assets: []
  });
  const analysisRun = await buildAnalysisRunPreview(result, {
    inputManifest
  });
  return { model, result, inputManifest, analysisRun };
}

// Historical projection compatibility only: no fresh solve is requested for
// the provenance-edited model, and the received legacy carrier stays unchanged.
async function legacyProvenanceSession(model: PreviewModel) {
  const result = structuredClone(historicalResult) as MechanicsResult;
  const inputManifest = await buildCurrentSessionInputManifest({
    model,
    solver: { solver_name: "open_pipe_stress_product_physics", solver_version: "0.1.0", solver_build_ref: "open_pipe_stress_product_physics@0.1.0", solver_mode: "sparse_interactive", settings: {} },
    active_rule_packs: [], external_assets: []
  });
  const analysisRun = await buildAnalysisRunV02(result, inputManifest);
  return { model, result, inputManifest, analysisRun };
}

describe("report-package current-session request", () => {
  it("preserves the legacy component-provenance oracle through pure report projection", async () => {
    const modelWithMissingProvenance = structuredClone(await loadPreviewModel());
    const missingComponent = modelWithMissingProvenance.components.find(
      (component) => component.id === "component:C-140"
    );
    expect(missingComponent).toBeDefined();
    missingComponent!.provenance = "";
    const { model, result, inputManifest, analysisRun } = await legacyProvenanceSession(
      modelWithMissingProvenance
    );
    const manifestComponent = inputManifest.manifest.model_basis.model_payload.components.find(
      (component) => component.id === "component:C-140"
    );

    expect(inputManifest.manifest.model_basis.model_ref).toBe(model.project.id);
    expect(inputManifest.manifest.model_basis.model_payload).toEqual(model);
    expect(manifestComponent).toMatchObject({
      id: "component:C-140",
      provenance: ""
    });

    const projected = await buildRenderableReportInput({ model, result, analysisRun, projectSummary: null });
    const historicalIdentity = { solver_name: "open_pipe_stress_product_physics", solver_version: "0.1.0", solver_build_ref: "open_pipe_stress_product_physics@0.1.0" };
    expect(analysisRun.analysis_run.solver_version).toMatchObject({ solver_name: historicalIdentity.solver_name, solver_version: historicalIdentity.solver_version, build_ref: { ref: historicalIdentity.solver_build_ref } });
    expect(result).toEqual(historicalResult);
    const sections = projected.report_sections;
    const presentId = componentProvenanceProjection.present_component.value.value_id;
    const missingId = componentProvenanceProjection.missing_component.value.value_id;

    const actualProjection = {
      schema_version: "1.0.0",
      present_component: {
        value: sections.user_supplied_values.find((value) => value.value_id === presentId),
        provenance_note: sections.provenance_notes.find(
          (note) => note.source_name === componentProvenanceProjection.present_component.provenance_note.source_name
        )
      },
      missing_component: {
        value: sections.user_supplied_values.find((value) => value.value_id === missingId),
        provenance_note: sections.provenance_notes.find(
          (note) => note.source_name === componentProvenanceProjection.missing_component.provenance_note.source_name
        ),
        diagnostic: sections.diagnostics.find(
          (diagnostic) =>
            diagnostic.code === "COMPONENT_PROVENANCE_MISSING" &&
            diagnostic.affected_object.ref_id === "component:C-140"
          )
      }
    };

    expect(componentProvenanceProjection.schema_version).toBe("1.0.0");
    expect(actualProjection).toEqual(componentProvenanceProjection);
  });

  it("maps the actual manifest, source dimensions, private copies, and DEL-08-06 records without mutating sources", async () => {
    const { model, result, inputManifest, analysisRun } =
      await currentSession();
    const comparison = buildPreviewComparison({ result, analysisRun });
    const modelSnapshot = structuredClone(model);
    const resultSnapshot = structuredClone(result);
    const runSnapshot = structuredClone(analysisRun);

    const request = await buildReportPackageRequest({
      model,
      result,
      analysisRun,
      inputManifest,
      projectSummary: null,
      comparison,
      ruleCheckAggregate: null
    });

    const sourceIdentity = { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: inputManifest.manifest.solver_basis.solver_build_ref };
    expect(sourceIdentity).toEqual({ solver_name: "open_pipe_stress_product_physics", solver_version: "0.2.0", solver_build_ref: "open_pipe_stress_product_physics@0.2.0" });
    expect(request.result_envelopes[0]).toMatchObject(sourceIdentity);
    expect(request.audit_manifest.solver_version).toEqual(sourceIdentity);
    expect(result.schema_version).toBe("0.2.0");
    expect(analysisRun.schema_version).toBe("0.3.0");
    expect(result.results.every(row => row.dimension === undefined)).toBe(true);
    const dimensions = request.result_envelopes[0].result_sets.flatMap(set => set.values).map(value => value.dimension);
    expect(dimensions).toEqual(expect.arrayContaining(["force", "moment"]));
    expect(request.export_profile_id).toBe("desktop_local_private_report_package_1");
    expect(request.source_model_ref.ref_id).toBe(result.model_ref);
    expect(request.audit_manifest.model_hash?.value).toMatch(/^[a-f0-9]{64}$/);
    expect(request.audit_manifest.input_manifest_hash?.value).toMatch(/^[a-f0-9]{64}$/);
    expect(request.audit_manifest.input_manifest_hash?.value).toBe(
      inputManifest.manifest_sha256
    );
    expect(request.audit_manifest.input_manifest_hash?.payload_ref).toBe(
      inputManifest.manifest_ref.ref
    );
    const resultEnvelopeHash = analysisRun.analysis_run.hashes.find(
      (item) => item.payload_scope === "received_result"
    )?.value;
    expect(inputManifest.manifest_sha256).not.toBe(resultEnvelopeHash);
    expect(request.audit_manifest.rule_pack_refs).toEqual([]);
    expect(request.result_envelopes[0].run_ref.ref_id).toBe(result.run_id);
    expect(request.result_envelopes[0].result_sets.flatMap((set) => set.values).length).toBeLessThan(result.results.length);
    for(const value of request.result_envelopes[0].result_sets.flatMap(set=>set.values)){const row=result.results.find(row=>row.id===value.result_id)!;expect(resultSemantics(row, result)?.category).toBe("physical_quantity");expect(value.magnitude).toBe(row.value);expect(value.unit).toBe(row.unit);expect(value.dimension).toBe(resultSemantics(row, result)?.derivative_target_dimension);}
    expect(request.result_envelopes[0].diagnostics.some(d=>d.code==="REPORT_SOURCE_EVIDENCE_DISCLOSED")).toBe(true);
    expect(request.result_envelopes[0].provenance).toMatchObject({
      source_location: "local desktop session",
      contributor: "user_local_session",
      contributor_certification: "not_asserted",
      redistribution_status: "private_only",
      review_status: "pending"
    });
    expect(request.state_comparison_handoff_records[0]).toMatchObject({
      deliverable_id: "DEL-08-06",
      section_set_id: `desktop-current-session:${result.run_id}`,
      diagnostics: [],
      professional_boundary: expect.objectContaining({ human_review_required: true })
    });
    const packageJson = JSON.stringify(request);
    expect(packageJson).not.toContain("invented_public_example");
    expect(packageJson).not.toContain('"redistribution_status":"public_permissive"');
    expect(packageJson).toContain('"privacy_classification":"private_project_data"');
    expect(packageJson).toContain('"redistribution_status":"private_only"');
    expect(packageJson).toContain('"review_status":"pending"');
    expect(model).toEqual(modelSnapshot);
    expect(result).toEqual(resultSnapshot);
    expect(analysisRun).toEqual(runSnapshot);
  });

  it("preserves historical fixture component and hanger input oracles as an unqualified projection", async () => {
    const session = await legacyProvenanceSession(await loadPreviewModel());
    const before = JSON.stringify(session);
    const projection = projectReceivedReportResults(session.result, session.analysisRun);
    const report = await buildRenderableReportInput({ ...session, projectSummary: null });
    const resultValues = projection.resultSets.flatMap(
      (set) => set.values
    );
    expect(
      resultValues.find(
        (item) =>
          item.result_id ===
          "result:component-stiffness:component-C-150:axial"
      )?.dimension
    ).toBeUndefined();
    expect(
      resultValues.find(
        (item) =>
          item.result_id ===
          "result:component-stiffness:component-C-150:torsional"
      )?.dimension
    ).toBeUndefined();
    const reportValues = report.report_sections.user_supplied_values;
    expect(
      reportValues.find((item) => item.value_id === "spring-hanger:support:SH-140")
    ).toMatchObject({
      quantity: { magnitude: 390, unit: "N", dimension: "force" },
      required_for: ["reporting", "human_review"],
      missing_data_finding: false
    });
    expect(
      reportValues.find((item) => item.value_id === "spring-hanger:support:CE-120")
    ).toMatchObject({
      quantity: { magnitude: 375, unit: "N", dimension: "force" },
      required_for: ["reporting", "human_review"],
      missing_data_finding: false
    });
    expect(JSON.stringify(session)).toBe(before);
  });

  it("blocks a same-ID model whose canonical payload differs from the verified manifest", async () => {
    const { model, result, inputManifest, analysisRun } = await currentSession();
    const changedModel = structuredClone(model);
    changedModel.project.description = `${changedModel.project.description} changed after manifest`;

    expect(changedModel.project.id).toBe(inputManifest.manifest.model_basis.model_ref);
    expect(await canonicalSha256Hex(changedModel)).not.toBe(
      await canonicalSha256Hex(inputManifest.manifest.model_basis.model_payload)
    );
    await expect(
      buildReportPackageRequest({
        model: changedModel,
        result,
        analysisRun,
        inputManifest,
        projectSummary: null,
        comparison: null,
        ruleCheckAggregate: null
      })
    ).rejects.toThrow("REPORT-PACKAGE-INPUT-MANIFEST-MODEL-PAYLOAD-MISMATCH");
  });

  it("accepts canonically equal model payloads with reordered object keys", async () => {
    const { model, result, inputManifest, analysisRun } = await currentSession();
    const reorderedModel = structuredClone(model);
    reorderedModel.project.units = Object.fromEntries(
      Object.entries(reorderedModel.project.units).reverse()
    );
    const canonicalModelHash = await canonicalSha256Hex(model);

    expect(JSON.stringify(reorderedModel)).not.toBe(JSON.stringify(model));
    expect(await canonicalSha256Hex(reorderedModel)).toBe(canonicalModelHash);
    const request = await buildReportPackageRequest({
      model: reorderedModel,
      result,
      analysisRun,
      inputManifest,
      projectSummary: null,
      comparison: null,
      ruleCheckAggregate: null
    });

    expect(request.audit_manifest.model_hash?.value).toBe(canonicalModelHash);
  });

  it("blocks every non-null rule-check aggregate before assembly", async () => {
    const { model, result, inputManifest } = await currentSession();
    const analysisRun = await buildAnalysisRunPreview(result, {
      inputManifest,
      ruleCheckAggregate: "RULE_INPUTS_INCOMPLETE"
    });

    await expect(
      buildReportPackageRequest({
        model,
        result,
        analysisRun,
        inputManifest,
        projectSummary: null,
        comparison: null,
        ruleCheckAggregate: "RULE_INPUTS_INCOMPLETE"
      })
    ).rejects.toThrow("REPORT-PACKAGE-RULE-PACK-BINDING-UNAVAILABLE");
  });

  it("maps family from exact kind and source dimension without reading deceptive unit text", async () => {
    const { model, result, inputManifest } = await currentSession();
    const deceptive = structuredClone(result);
    const target = deceptive.results.find(
      (item) => item.kind === "element_local_axial_force"
    );
    expect(target).toBeDefined();
    target!.unit = "MPa";
    await expect(buildAnalysisRunPreview(deceptive, {
      inputManifest
    })).rejects.toThrow("SOURCE_UNIT_CONTRADICTION");
  });

  it("blocks a source dimension that contradicts exact result kind semantics", async () => {
    const { model, result, inputManifest, analysisRun } =
      await currentSession();
    const mismatched = structuredClone(analysisRun);
    const target = mismatched.analysis_run.result_refs.find(
      (item) =>
        result.results.find(
          (source) =>
            source.id === item.result_ref.ref &&
            source.kind === "element_local_axial_force"
        )
    );
    expect(target).toBeDefined();
    target!.source_dimension = "stress";

    await expect(
      buildReportPackageRequest({
        model,
        result,
        analysisRun: mismatched,
        inputManifest,
        projectSummary: null,
        comparison: null,
        ruleCheckAggregate: null
      })
    ).rejects.toThrow("REPORT-PACKAGE-SOURCE-DIMENSION-MISMATCH");
  });

  it("blocks missing or mismatched manifest evidence and malformed SHA-256 before assembly", async () => {
    const { model, result, inputManifest, analysisRun } =
      await currentSession();
    const missing = structuredClone(analysisRun);
    missing.analysis_run.reproducibility.input_manifest_refs = [];
    missing.analysis_run.reproducibility.input_manifest_hashes = [];
    await expect(
      buildReportPackageRequest({
        model,
        result,
        analysisRun: missing,
        inputManifest,
        projectSummary: null,
        comparison: null,
        ruleCheckAggregate: null
      })
    ).rejects.toThrow("REPORT-PACKAGE-HASH-BINDING-INCOMPLETE");

    const malformed = structuredClone(analysisRun);
    malformed.analysis_run.hashes[0].value = "A".repeat(64);
    await expect(
      buildReportPackageRequest({
        model,
        result,
        analysisRun: malformed,
        inputManifest,
        projectSummary: null,
        comparison: null,
        ruleCheckAggregate: null
      })
    ).rejects.toThrow("REPORT-PACKAGE-SHA256-INVALID");

    const mismatched = structuredClone(inputManifest);
    mismatched.manifest.model_basis.model_ref = "project:different";
    await expect(
      buildReportPackageRequest({
        model,
        result,
        analysisRun,
        inputManifest: mismatched,
        projectSummary: null,
        comparison: null,
        ruleCheckAggregate: null
      })
    ).rejects.toThrow("INPUT-MANIFEST-HASH-MISMATCH");

    for (const ref of [
      `result-envelope:project-invented-loop-01:${inputManifest.manifest_sha256}`,
      `input-manifest:project-different:${inputManifest.manifest_sha256}`
    ]) {
      const wrongIdentity = structuredClone(inputManifest);
      wrongIdentity.manifest_ref.ref = ref;
      await expect(
        buildReportPackageRequest({
          model,
          result,
          analysisRun,
          inputManifest: wrongIdentity,
          projectSummary: null,
          comparison: null,
          ruleCheckAggregate: null
        })
      ).rejects.toThrow("INPUT-MANIFEST-HASH-MISMATCH");
    }
  });
  it("rejects future analysis versions and missing current received-result hashes", async () => {
    const session = await currentSession();
    const future = structuredClone(session.analysisRun);
    future.schema_version = "0.4.0";
    await expect(buildReportPackageRequest({ ...session, analysisRun: future, projectSummary: null, comparison: null, ruleCheckAggregate: null }))
      .rejects.toThrow("REPORT-PACKAGE-ANALYSIS-VERSION-UNSUPPORTED");
    const futureSource = structuredClone(session.result);
    futureSource.schema_version = "0.4.0";
    await expect(buildReportPackageRequest({ ...session, result: futureSource, projectSummary: null, comparison: null, ruleCheckAggregate: null }))
      .rejects.toThrow("REPORT-PACKAGE-SOURCE-CONTRACT-MISMATCH");
    const missing = structuredClone(session.analysisRun);
    missing.analysis_run.hashes = missing.analysis_run.hashes.filter(hash => hash.payload_scope !== "received_result");
    await expect(buildReportPackageRequest({ ...session, analysisRun: missing, projectSummary: null, comparison: null, ruleCheckAggregate: null }))
      .rejects.toThrow("REPORT-PACKAGE-HASH-BINDING-INCOMPLETE");
  });

  it("discloses a synthetic unknown row on the unqualified historical projection without inferring force from N", async () => {
    const { model, result, inputManifest } = await legacyProvenanceSession(await loadPreviewModel());
    const annotated = structuredClone(result);
    const unknownId = "result:test:unregistered-source";
    annotated.results.push({ id: unknownId, entity_ref: model.project.id, kind: "unregistered_source_kind", value: 1, unit: "N" });
    const analysisRun = await buildAnalysisRunV02(annotated, inputManifest);
    const before = JSON.stringify(annotated);
    const projection = projectReceivedReportResults(annotated, analysisRun);
    expect(projection.resultSets.flatMap(set => set.values).some(value => value.result_id === unknownId)).toBe(false);
    expect(JSON.stringify(projection.semanticDisclosures)).toContain(unknownId);
    expect(JSON.stringify(annotated)).toBe(before);
  });

  it.each(["solver_name", "solver_version", "build_ref"] as const)("rejects recorded %s that differs from the verified manifest", async field => {
    const session = await currentSession();
    const analysisRun = structuredClone(session.analysisRun);
    const solver = analysisRun.analysis_run.solver_version!;
    if (field === "build_ref") solver.build_ref.ref = "build:conflicting-record";
    else solver[field] = "conflicting-record";
    await expect(buildReportPackageRequest({ ...session, analysisRun, projectSummary: null, comparison: null, ruleCheckAggregate: null }))
      .rejects.toThrow("REPORT-PACKAGE-SOLVER-IDENTITY-MISMATCH");
  });

  it.each(["solver_name", "solver_version"] as const)("rejects a hash-valid manifest and coherent record with the wrong producer %s", async field => {
    const session = await currentSession();
    const receivedSourceBefore = JSON.stringify(session.result);
    const solver = { ...session.inputManifest.manifest.solver_basis, [field]: field === "solver_version" ? "0.1.0" : "different_solver" };
    const inputManifest = await buildCurrentSessionInputManifest({ model: session.model, solver, active_rule_packs: [], external_assets: [] });
    await expect(buildAnalysisRunPreview(session.result, { inputManifest })).rejects.toThrow("ANALYSIS_SOURCE_PRODUCER_MISMATCH");
    // Deliberately adversarial, test-only record: do not bypass the strict
    // builder by changing its validator or pretending this is admitted output.
    const analysisRun = structuredClone(session.analysisRun);
    const run = analysisRun.analysis_run;
    run.solver_version = { solver_name: solver.solver_name, solver_version: solver.solver_version, build_ref: { object_type: "ExternalReference", ref: solver.solver_build_ref } };
    run.settings_ref = { object_type: "SolverSettings", ref: `solver-settings:${inputManifest.manifest_ref.ref}:${inputManifest.manifest_sha256}` };
    run.unit_system_ref = { object_type: "UnitSystem", ref: `unit-system:${session.result.model_ref}:${inputManifest.manifest_sha256}` };
    run.reproducibility.input_manifest_refs = [structuredClone(inputManifest.manifest_ref)];
    run.reproducibility.input_manifest_hashes = [{ ...run.reproducibility.input_manifest_hashes[0], payload_ref: structuredClone(inputManifest.manifest_ref), value: inputManifest.manifest_sha256 }];
    run.hashes.find(hash => hash.payload_scope === "analysis_run_record")!.value = await canonicalSha256HexCheckedV1(analysisRecordProjection(analysisRun));
    expect(await verifyAnalysisRunRecord(analysisRun)).toBe("match");
    expect(run.hashes.find(hash => hash.payload_scope === "received_result"))
      .toEqual(session.analysisRun.analysis_run.hashes.find(hash => hash.payload_scope === "received_result"));
    await expect(buildReportPackageRequest({ ...session, inputManifest, analysisRun, projectSummary: null, comparison: null, ruleCheckAggregate: null }))
      .rejects.toThrow("REPORT-PACKAGE-SOLVER-IDENTITY-MISMATCH");
    expect(JSON.stringify(session.result)).toBe(receivedSourceBefore);
  });

  it("preserves a consistent recorded build reference without inventing one from producer version", async () => {
    const session = await currentSession();
    const recordedBuild = "build:captured-current-product-physics";
    const inputManifest = await buildCurrentSessionInputManifest({ model: session.model, solver: { ...session.inputManifest.manifest.solver_basis, solver_build_ref: recordedBuild }, active_rule_packs: [], external_assets: [] });
    const analysisRun = await buildAnalysisRunPreview(session.result, { inputManifest });
    const request = await buildReportPackageRequest({ ...session, inputManifest, analysisRun, projectSummary: null, comparison: null, ruleCheckAggregate: null });
    expect(request.result_envelopes[0].solver_build_ref).toBe(recordedBuild);
    expect(request.audit_manifest.solver_version.solver_build_ref).toBe(recordedBuild);
    expect(request.audit_manifest.solver_version.solver_version).toBe(session.result.producer!.component_version);
  });

});


it("refuses an imported or cloned precision source even when its manifest and analysis hashes match", async () => {
  const session = await currentSession();
  const clone = structuredClone(session.result), before = JSON.stringify(clone);
  await expect(buildReportPackageRequest({ ...session, result: clone, projectSummary: null, comparison: null, ruleCheckAggregate: null }))
    .rejects.toThrow("REPORT-PACKAGE-NATIVE-INVOCATION-REQUIRED");
  expect(JSON.stringify(clone)).toBe(before);
});

it("keeps genuine physics source inspectable while explicitly withholding this report transport", async () => {
  const session = await currentSession("physics"), before = JSON.stringify(session);
  await expect(buildReportPackageRequest({ ...session, projectSummary: null, comparison: null, ruleCheckAggregate: null }))
    .rejects.toThrow("REPORT-PACKAGE-PHYSICS-PROJECTION-UNAVAILABLE");
  expect(JSON.stringify(session)).toBe(before);
});
