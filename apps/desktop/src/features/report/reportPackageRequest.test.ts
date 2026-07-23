import { describe, expect, it } from "vitest";
import { buildAnalysisRunPreview, buildPreviewComparison, loadPreviewModel, runPreviewMechanics } from "../../services/previewService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { buildReportPackageRequest } from "./reportPackageRequest";

async function currentSession() {
  const model = await loadPreviewModel();
  const result = await runPreviewMechanics(model);
  const inputManifest = await buildCurrentSessionInputManifest({
    model,
    solver: {
      solver_name: "open_pipe_stress_product_physics",
      solver_version: "0.1.0",
      solver_build_ref: "open_pipe_stress_product_physics@0.1.0",
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

describe("report-package current-session request", () => {
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
      (item) => item.payload_scope === "result_envelope"
    )?.value;
    expect(inputManifest.manifest_sha256).not.toBe(resultEnvelopeHash);
    expect(request.audit_manifest.rule_pack_refs).toEqual([]);
    expect(request.result_envelopes[0].run_ref.ref_id).toBe(result.run_id);
    expect(request.result_envelopes[0].result_sets.flatMap((set) => set.values)).toHaveLength(result.results.length);
    expect(request.result_envelopes[0].provenance).toMatchObject({
      source_location: "local desktop session",
      contributor: "user_local_session",
      contributor_certification: "not_asserted",
      redistribution_status: "private_only",
      review_status: "pending"
    });
    const resultValues = request.result_envelopes[0].result_sets.flatMap(
      (set) => set.values
    );
    expect(
      resultValues.find(
        (item) =>
          item.result_id ===
          "result:component-stiffness:component-C-150:axial"
      )?.dimension
    ).toBe("linear_stiffness");
    expect(
      resultValues.find(
        (item) =>
          item.result_id ===
          "result:component-stiffness:component-C-150:torsional"
      )?.dimension
    ).toBe("rotational_stiffness");
    const reportValues = request.report.report_sections.user_supplied_values;
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
    const analysisRun = await buildAnalysisRunPreview(deceptive, {
      inputManifest
    });
    const request = await buildReportPackageRequest({
      model,
      result: deceptive,
      analysisRun,
      inputManifest,
      projectSummary: null,
      comparison: null,
      ruleCheckAggregate: null
    });
    const mapped = request.result_envelopes[0].result_sets
      .flatMap((set) => set.values)
      .find((item) => item.result_id === target!.id);
    expect(mapped).toMatchObject({
      family: "force",
      dimension: "force",
      unit: "MPa"
    });
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
});
