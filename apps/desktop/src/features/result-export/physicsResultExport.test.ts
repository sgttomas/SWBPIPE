import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { createHash } from "node:crypto";
import { describe, expect, it } from "vitest";
import type { MechanicsResult, PreviewModel } from "../../types";
import { validatePhysicsEvidence } from "../results/physicsResultEvidence";
import { numericalResultStanding, sourceContract, sourceSemanticBinding, PHYSICS_CONTRACT_ID, PHYSICS_CONTRACT_SHA256, PRECISION_CONTRACT_ID, PRECISION_CONTRACT_SHA256 } from "../results/numericalResultQuality";
import { resultSemantics, canonicalResultMetadata } from "../results/resultSemantics";
import { buildAnalysisRunV03, validateAnalysisRunV03, modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";
import { deriveResultDocument, validateResultDocument, buildCurrentResultExport, resultDigest, ref, derivativeProvenance } from "./resultExportAdapter";

// Actual joined source bytes; the ENGINE_INTEGRATION execution receipt records their
// production. Loading them here does not create native invocation or Current proof.
const project = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(project, path), "utf8"));
const raw = (mode = "sparse"): MechanicsResult => json(`fixtures/results/physics_connected_mechanics_${mode}.json`);
const model = (): PreviewModel => json("core/product_physics/tests/fixtures/exact_pressure_connected_request.json").model;
const evidence = (source: MechanicsResult): any => source.contract_evidence;
function manifestArgs(source: MechanicsResult) {
  return { model: model(), solver: { solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: "test:consumer-received-source-not-native-attestation", solver_mode: "sparse_interactive", settings: {} }, active_rule_packs: [], external_assets: [] };
}
async function manifest(source: MechanicsResult) {
  // The core fixture omits UI load-case kind/status, so it cannot create a
  // Current-session manifest. This reference-only analysis basis preserves the
  // actual received model without inventing authored UI fields or Current proof.
  const args = manifestArgs(source), record = { model_basis: { model_ref: source.model_ref, model_payload: args.model }, solver_basis: args.solver };
  return { manifest: record, manifest_ref: { object_type: "InputManifest", ref: "test:reference-only-physics-analysis" }, manifest_sha256: await resultDigest(record) };
}
async function projectSource(source: MechanicsResult) {
  const origin = { origin_id: "test:received-source", origin_class: "received_current_dimension_absent", qualification_ref: ref("test_fixture", "unattested-source-consumer-projection"), authentic_producer_available: false,
    received_carrier_checksum: { algorithm: "sha256", canonicalization: "openpipestress_jcs_ijson_v1", payload_scope: "received_current_dimension_absent_carrier", payload_ref: ref("received_current_carrier", source.run_id), value: await resultDigest(source) }, original_producer_checksum: null,
    origin_limit: "Consumer projection only; no native producer or Current qualification", actual_model_ref: ref("model_payload", source.model_ref), mechanics_run_ref: ref("mechanics_run", source.run_id), request_model_ref: null, request_run_ref: null, request_alias_disclosure: null };
  return deriveResultDocument({ schema_version: "0.2.0", result_envelope: { schema_version: "0.2.0", envelope_id: "test:physics-projection", run_ref: ref("analysis_run", source.run_id), provenance: derivativeProvenance, unit_system_ref: ref("unit_system", "test:units"), result_sets: [{ values: [] }], reproducibility: {} } }, model(), source, origin);
}

describe("actual joined physics-1 source consumers", () => {
  it.each(["sparse", "dense"])("dispatches %s source with complete cases and preserves source bytes", mode => {
    const source = raw(mode), before = JSON.stringify(source);
    expect(sourceContract(source)).toBe("physics");
    expect(sourceSemanticBinding(source)).toEqual({ id: PHYSICS_CONTRACT_ID, sha256: PHYSICS_CONTRACT_SHA256 });
    expect(() => validatePhysicsEvidence(source, model())).not.toThrow();
    expect(numericalResultStanding(source, model()).eligible).toBe(true);
    for (const row of source.results.filter(r => r.kind.endsWith("_v2"))) {
      expect(resultSemantics(row, source)?.category).toBe("physical_quantity");
      expect(canonicalResultMetadata(row, source)).not.toBeNull();
    }
    expect(JSON.stringify(source)).toBe(before);
  });
  it("pins the additive table and retains frozen precision table bytes", () => {
    for (const [name, expected] of [["physics", PHYSICS_CONTRACT_SHA256], ["precision", PRECISION_CONTRACT_SHA256]])
      expect(createHash("sha256").update(readFileSync(resolve(project, `fixtures/results/semantic_contract_v0_3_${name}_1.json`))).digest("hex")).toBe(expected);
  });
  it("rejects unknown/reserved and mismatched profile dispatch without falling back", () => {
    for (const id of ["precision-2", "source-blocks-1", "physics-2"]) {
      const source = raw(); source.producer!.semantic_contract_id = `openpipestress.result_semantics/0.3.0/${id}`;
      expect(sourceContract(source)).toBe("unsupported");
    }
    for (const key of ["source_block_recovery", "carrier_evidence"]) {
      expect(sourceContract(Object.assign(raw(), { [key]: null }))).toBe("unsupported");
    }
    const ordinary = json("fixtures/product_preview/invented_mechanics_result_precision_1_sparse.json") as MechanicsResult;
    Object.assign(ordinary, { contract_evidence: null });
    expect(sourceContract(ordinary)).toBe("precision");
    const changed = raw(); changed.producer!.semantic_contract_id = PRECISION_CONTRACT_ID;
    expect(sourceContract(changed)).toBe("unsupported");
    changed.formulation_basis!.profile_id = "product_preview_mechanics_v1";
    expect(sourceContract(changed)).toBe("unsupported");
  });
  it.each([
    ["cross-case NUM evidence", (s: MechanicsResult) => { s.numerical_quality!.cases[0].evidence_refs = s.numerical_quality!.cases[1].evidence_refs; }],
    ["wrong force sign", (s: MechanicsResult) => { s.results.find(r => r.kind === "pipe_wall_axial_force_v2")!.metadata!.sign_convention = "compression-positive"; }],
    ["source bore contradiction", (s: MechanicsResult) => { evidence(s).exact_cases[0].pressure_rhs_assembly.groups[0].source_inner_radius_lo_bits = "0000000000000000"; }],
    ["missing support component", (s: MechanicsResult) => { const row = s.results.find(r => r.kind === "support_reaction_component_v2")!; s.results = s.results.filter(r => r.id !== row.id); }],
    ["missing evidence", (s: MechanicsResult) => { delete s.contract_evidence; }],
    ["unknown namespace", (s: MechanicsResult) => { evidence(s).source_block_recovery = {}; }],
    ["duplicate case", (s: MechanicsResult) => { evidence(s).exact_cases.push(evidence(s).exact_cases[0]); }],
    ["missing no-pressure material", (s: MechanicsResult) => { evidence(s).exact_cases[1].pipe_materials = []; }],
    ["contradictory duplicate material", (s: MechanicsResult) => { evidence(s).pressure[0].materials[0].nu = 0.2; }],
    ["contradictory duplicate geometry", (s: MechanicsResult) => { evidence(s).pressure[0].geometry[0].As_m2 *= 2; }],
    ["cross-case binding", (s: MechanicsResult) => { evidence(s).pressure[0].load_case_id = evidence(s).exact_cases[1].load_case_id; }],
    ["duplicate binding", (s: MechanicsResult) => { evidence(s).pressure[0].result_ids.push(evidence(s).pressure[0].result_ids[0]); }],
    ["missing binding", (s: MechanicsResult) => { evidence(s).pressure[0].result_ids.pop(); }],
    ["fabricated binding", (s: MechanicsResult) => { evidence(s).pressure[0].result_ids[0] = "missing"; }],
    ["missing row and its binding", (s: MechanicsResult) => { const id = evidence(s).pressure[0].result_ids.pop(); s.results = s.results.filter(r => r.id !== id); }],
    ["pressure bit contradiction", (s: MechanicsResult) => { evidence(s).pressure[0].p_pa += 1; }],
    ["unknown RHS method", (s: MechanicsResult) => { evidence(s).exact_cases[0].pressure_rhs_assembly.method = "unknown"; }],
    ["nonfinite physical input", (s: MechanicsResult) => { evidence(s).pressure[0].p_pa = Infinity; }],
    ["fake extrema midpoint", (s: MechanicsResult) => { const x = evidence(s).exact_cases[1].pipe_stress_extrema[0]; s.results.find(r => r.id === x.result_id)!.value = x.global_upper_bound_pa; }],
    ["derived pressure row", (s: MechanicsResult) => { s.results.find(r => r.kind === "pipe_wall_axial_force_v2")!.source_result_refs = [s.results[0].id]; }],
    ["misdimensioned reaction", (s: MechanicsResult) => { s.results.find(r => r.kind === "support_reaction_component_v2" && r.metadata?.component === "Mx")!.unit = "N"; }],
  ] as const)("rejects %s", (_, mutate) => {
    const source = raw(); mutate(source);
    expect(() => validatePhysicsEvidence(source, model())).toThrow();
    expect(numericalResultStanding(source, model()).eligible).toBe(false);
  });
  it("withholds a headline for incomplete extrema but retains valid signed rows", () => {
    const source = raw(), c = evidence(source).exact_cases[1], id = c.pipe_stress_extrema[0].result_id;
    c.stress_maximum_coverage = { complete: false, unavailable_pipe_ids: [c.pipe_materials[0].pipe_id] }; c.pipe_stress_extrema = [];
    source.results = source.results.filter(r => r.id !== id);
    expect(() => validatePhysicsEvidence(source)).toThrow("INCOMPLETE_MAXIMUM_HEADLINE");
    source.summary.max_open_formula_stress = null;
    expect(() => validatePhysicsEvidence(source)).not.toThrow();
  });
  it("keeps Sensitive evidence inspectable without promoting its accuracy claim", () => {
    const source = raw(); source.numerical_quality!.status = "sensitive"; source.numerical_quality!.cases[0].solve_quality = "sensitive"; source.numerical_quality!.cases[0].accuracy_evidence = "reference_verified";
    expect(sourceContract(source)).toBe("physics"); expect(numericalResultStanding(source, model()).eligible).toBe(false);
  });
  it.each(["sparse", "dense"])("preserves %s evidence through analysis/checksums, JSON reopen and canonical projection", async mode => {
    const source = raw(mode), before = JSON.stringify(source), inputManifest = await manifest(source);
    const analysis = await buildAnalysisRunV03(source, inputManifest, undefined, modelLoadBasisRefs(model()));
    await validateAnalysisRunV03(JSON.parse(JSON.stringify(analysis)), JSON.parse(JSON.stringify(source)), modelLoadBasisRefs(model()));
    expect(analysis.analysis_run.reproducibility.semantic_contract!.id).toBe(PHYSICS_CONTRACT_ID);
    const doc = await projectSource(source), reopened = JSON.parse(JSON.stringify(doc));
    expect(doc.result_envelope.contract_evidence).toEqual(source.contract_evidence);
    // JSON/JCS normalizes signed zero; the canonical evidence hash and explicit
    // source bit strings survive reopen without reconstructing any physical row.
    expect(await resultDigest(reopened.result_envelope.contract_evidence)).toBe(await resultDigest(source.contract_evidence));
    await validateResultDocument(reopened, source);
    expect(doc.result_envelope.reproducibility.source_origin_bindings[0].authentic_producer_available).toBe(false);
    expect(JSON.stringify(source)).toBe(before);
    delete reopened.result_envelope.contract_evidence;
    await expect(validateResultDocument(reopened, source)).rejects.toThrow("SOURCE_PHYSICAL_METADATA_MISMATCH");
    const changed = raw(mode); evidence(changed).pressure[0].provenance += ":changed";
    await expect(validateAnalysisRunV03(analysis, changed, modelLoadBasisRefs(model()))).rejects.toThrow("ANALYSIS_RECEIVED_SOURCE_MISMATCH");
  });
  it("requires existing Current manifest/source proof and retains Sensitive rejection", async () => {
    const source = raw(), inputManifest = await manifest(source), analysisRun = await buildAnalysisRunV03(source, inputManifest, undefined, modelLoadBasisRefs(model()));
    await expect(buildCurrentResultExport({ model: model(), result: source, analysisRun, inputManifest: null })).rejects.toThrow("CURRENT_INPUT_MANIFEST_UNAVAILABLE");
    await expect(buildCurrentSessionInputManifest(manifestArgs(source))).rejects.toThrow("INPUT-MANIFEST-LOAD-BASIS-INCOMPLETE");
    source.numerical_quality!.status = "sensitive";
    await expect(buildCurrentResultExport({ model: model(), result: source, analysisRun, inputManifest: null })).rejects.toThrow("CURRENT_NUMERICAL_INTEGRITY_NEEDS_RECOMPUTE");
  });
});
