import { describe, expect, it } from "vitest";
import { buildNativePackageReview, buildUnitPreservationEvidence } from "./NativePackagePanel";
import { loadPreviewModel, loadBundledMechanicsReference, hasNativeMechanicsInvocation } from "../../services/previewService";
import { buildAnalysisRunV03, modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";

// Pure received-hash inspection of preserved reference data. This constructs
// an analysis record, never a native invocation or Current/qualified export.
async function referencePrecisionAnalysis() {
  const model = await loadPreviewModel(), reference = await loadBundledMechanicsReference();
  const result = reference.source;
  const manifest = {
    model_basis: { model_ref: model.project.id, model_payload: model },
    solver_basis: { solver_name: result.producer!.component_name, solver_version: result.producer!.component_version, solver_build_ref: "test:preserved-reference-not-native-invocation" },
  };
  const basis = { manifest, manifest_ref: { object_type: "InputManifest", ref: "test:reference-analysis-inputs" }, manifest_sha256: await canonicalSha256HexCheckedV1(manifest) };
  const analysisRun = await buildAnalysisRunV03(result, basis, undefined, modelLoadBasisRefs(model));
  const receivedHash = analysisRun.analysis_run.hashes.find(hash => hash.payload_scope === "received_result")!.value;
  expect(reference.standing).toBe("reference_only");
  expect(reference.provenance.current_use_eligible).toBe(false);
  expect(hasNativeMechanicsInvocation(result, model)).toBe(false);
  expect(analysisRun.schema_version).toBe("0.3.0");
  return { model, result, analysisRun, receivedHash };
}

describe("native review package received-result hash binding", () => {
  it("uses the preserved reference received hash and discloses unsupported future versions", async () => {
    const session = await referencePrecisionAnalysis();
    const input = { ...session, editorIntents: [], modelHash: null, projectSummary: null, proposal: null, selectedReviewTarget: null, storageCapability: null };
    const before = JSON.stringify(input);
    const packet = buildNativePackageReview(input);
    const member = packet.manifest.package_members.find(item => item.path === "results/result_envelope_ref.json");
    expect(member?.hash_status).toBe(`sha256:${session.receivedHash}`);
    expect(JSON.stringify(input)).toBe(before);
    const future = structuredClone(session.analysisRun);
    future.schema_version = "0.4.0";
    const unsupported = buildNativePackageReview({ ...input, analysisRun: future });
    expect(unsupported.manifest.package_members.find(item => item.path === "results/result_envelope_ref.json")?.hash_status).toBe("TBD_analysis_record_version_unsupported");
  });
});


it("retains explicit exact-profile quantities without inventing an authored shear modulus", async () => {
  const modelModule = await import("../../../../../fixtures/model_operations/exact_pressure_authoring_model.json");
  const resultModule = await import("../../../../../fixtures/results/physics_connected_mechanics_sparse.json");
  const model = structuredClone(modelModule.default) as import("../../types").PreviewModel;
  const result = resultModule.default as unknown as import("../../types").MechanicsResult;
  model.pipe_segments[0].section.material_density = { value: 1000.123456789, unit: "kg/m³" };
  const before = JSON.stringify(model);
  const evidence = buildUnitPreservationEvidence({ model, result });
  const rows = evidence.model_quantity_witnesses;
  expect(rows.some(row => row.source_ref.field_path === "shear_modulus")).toBe(false);
  expect(rows.find(row => row.source_ref.field_path === "poisson_ratio")?.source_quantity)
    .toEqual({ value: 0.3, unit: "1", dimension: "ratio" });
  expect(rows.find(row => row.source_ref.field_path === "section.material_density")?.source_quantity)
    .toEqual({ value: 1000.123456789, unit: "kg/m³", dimension: "density" });
  expect(rows.find(row => row.source_ref.field_path === "pressure_regions.region:fixture-pressure.pressure")?.source_quantity)
    .toEqual({ value: 2000, unit: "kPa", dimension: "pressure" });
  expect(JSON.stringify(model)).toBe(before);
});
