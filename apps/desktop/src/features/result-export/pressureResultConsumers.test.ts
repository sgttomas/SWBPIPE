import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { createHash } from "node:crypto";
import { describe, expect, it } from "vitest";
import type { MechanicsResult, PreviewModel } from "../../types";
import { readerTransport, readerValidate } from "../results/resultReader";
import { PRESSURE_CONTRACT_ID } from "../results/numericalResultQuality";
import { RETIRED_PRECISION_CONTRACT_ID, retiredPrecisionStatement } from "../../test-support/retiredPrecision";
import { PRESSURE_CONTRACT_SHA256 } from "../../test-support/contractBindings";
import { sourceContract, sourceSemanticBinding } from "../results/resultReader";
import { numericalResultStanding } from "../results/resultStanding";
import { readerRowSemantics } from "../results/resultReader";
import { PRESSURE_1_OUTPUT_REFUSAL } from "../results/outputPolicy";
const resultSemantics = (row: Parameters<ReturnType<typeof readerRowSemantics>>[0], source?: Parameters<typeof readerRowSemantics>[0]) => readerRowSemantics(source)(row).signature;
const canonicalResultMetadata = (row: Parameters<ReturnType<typeof readerRowSemantics>>[0], source?: Parameters<typeof readerRowSemantics>[0]) => readerRowSemantics(source)(row).canonical_metadata;
import { buildAnalysisRunV03, modelLoadBasisRefs } from "../../services/analysisRunCompatibility";
import { deriveResultDocument, buildCurrentResultExport, resultDigest, ref, derivativeProvenance } from "./resultExportAdapter";

/** B7b: the v3 twin of the retired physics-1 consumer suite. The exact route's ordinary
 * identity is pressure-1 (3.0.0/exact_pressure_v3); the desktop reads it through the Rust
 * reader's own route and offers no output of it: no analysis-run record
 * (ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH) and every surface refused
 * (PRESSURE-1-OUTPUT-NOT-YET-AVAILABLE). Actual committed producer bytes; loading them
 * here creates no native invocation or Current proof. */
const project = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(project, path), "utf8"));
const raw = (mode = "sparse"): MechanicsResult => json(`fixtures/results/pressure_connected_mechanics_${mode}.json`);
const model = (): PreviewModel => json("core/product_physics/tests/fixtures/exact_pressure_connected_request.json").model;
const evidence = (source: MechanicsResult): any => source.contract_evidence; // eslint-disable-line @typescript-eslint/no-explicit-any

describe("actual pressure-1 source consumers (B7b: the v3 twin of physics-1)", () => {
  it.each(["sparse", "dense"])("dispatches %s source with complete cases and preserves source bytes", mode => {
    const source = raw(mode), before = JSON.stringify(source);
    expect(sourceContract(source)).toBe("pressure");
    expect(sourceSemanticBinding(source)).toEqual({ id: PRESSURE_CONTRACT_ID, sha256: PRESSURE_CONTRACT_SHA256 });
    expect(() => readerValidate("pressure", source, model())).not.toThrow();
    expect(numericalResultStanding(source, model()).eligible).toBe(true);
    for (const row of source.results.filter(r => r.kind.endsWith("_v2"))) {
      expect(resultSemantics(row, source)?.category).toBe("physical_quantity");
      expect(canonicalResultMetadata(row, source)).not.toBeNull();
    }
    expect(JSON.stringify(source)).toBe(before);
  });
  it("pins the pressure-1 table bytes", () => {
    expect(createHash("sha256").update(readFileSync(resolve(project, "fixtures/results/semantic_contract_v0_3_pressure_1.json"))).digest("hex")).toBe(PRESSURE_CONTRACT_SHA256);
  });
  it("rejects unknown, reserved, retired and mismatched profile dispatch without falling back", () => {
    for (const id of ["precision-2", "source-blocks-1", "physics-2", "physics-1", "physics-source-1", "load-reference-1", "load-reference-source-1"]) {
      const source = raw(); source.producer!.semantic_contract_id = `openpipestress.result_semantics/0.3.0/${id}`;
      expect(sourceContract(source), id).toBe("unsupported");
    }
    for (const key of ["source_block_recovery", "carrier_evidence"]) {
      expect(sourceContract(Object.assign(raw(), { [key]: null }))).toBe("unsupported");
    }
    // B1: a stored precision-1 statement is refused by name (unsupported here).
    expect(sourceContract(retiredPrecisionStatement())).toBe("unsupported");
    const changed = raw(); changed.producer!.semantic_contract_id = RETIRED_PRECISION_CONTRACT_ID;
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
    expect(() => readerValidate("pressure", source, model())).toThrow();
    expect(numericalResultStanding(source, model()).eligible).toBe(false);
  });
  it("withholds a headline for incomplete extrema but retains valid signed rows", () => {
    const source = raw(), c = evidence(source).exact_cases[1], id = c.pipe_stress_extrema[0].result_id;
    c.stress_maximum_coverage = { complete: false, unavailable_pipe_ids: [c.pipe_materials[0].pipe_id] }; c.pipe_stress_extrema = [];
    source.results = source.results.filter(r => r.id !== id);
    expect(() => readerValidate("pressure", source)).toThrow("SOURCE_PHYSICS_INCOMPLETE_STRESS_HEADLINE");
    source.summary.max_open_formula_stress = null;
    expect(() => readerValidate("pressure", source)).not.toThrow();
  });
  it("keeps Sensitive evidence inspectable without promoting its accuracy claim", () => {
    const source = raw(); source.numerical_quality!.status = "sensitive"; source.numerical_quality!.cases[0].solve_quality = "sensitive"; source.numerical_quality!.cases[0].accuracy_evidence = "reference_verified";
    expect(sourceContract(source)).toBe("pressure"); expect(numericalResultStanding(source, model()).eligible).toBe(false);
  });
  it("has no transport reader: a transported pressure-1 statement is refused by name, and a retired reader name is unknown", () => {
    expect(() => readerTransport("pressure", raw())).toThrow("SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED");
    for (const retired of ["physics", "physics-source", "load-reference", "load-reference-source"]) {
      expect(() => readerValidate(retired as never, raw()), retired).toThrow(`READER_UNKNOWN: ${retired}`);
    }
  });
  it.each(["sparse", "dense"])("%s: builds no analysis-run record and offers no result export or projection", async mode => {
    const source = raw(mode), before = JSON.stringify(source);
    const record = { model_basis: { model_ref: source.model_ref, model_payload: model() }, solver_basis: { solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: "test:reference-only", solver_mode: "sparse_interactive", settings: {} } };
    const inputManifest = { manifest: record, manifest_ref: { object_type: "InputManifest", ref: "test:reference-only-pressure-analysis" }, manifest_sha256: await resultDigest(record) };
    await expect(buildAnalysisRunV03(source, inputManifest as never, undefined, modelLoadBasisRefs(model()))).rejects.toThrow("ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH");
    await expect(buildCurrentResultExport({ model: model(), result: source, analysisRun: {} as never, inputManifest: null })).rejects.toThrow(PRESSURE_1_OUTPUT_REFUSAL);
    const origin = { origin_id: "test:received-source", origin_class: "received_current_dimension_absent", qualification_ref: ref("test_fixture", "unattested-source-consumer-projection"), authentic_producer_available: false,
      received_carrier_checksum: { algorithm: "sha256", canonicalization: "openpipestress_jcs_ijson_v1", payload_scope: "received_current_dimension_absent_carrier", payload_ref: ref("received_current_carrier", source.run_id), value: await resultDigest(source) }, original_producer_checksum: null,
      origin_limit: "Consumer projection only; no native producer or Current qualification", actual_model_ref: ref("model_payload", source.model_ref), mechanics_run_ref: ref("mechanics_run", source.run_id), request_model_ref: null, request_run_ref: null, request_alias_disclosure: null };
    await expect(deriveResultDocument({ schema_version: "0.2.0", result_envelope: { schema_version: "0.2.0", envelope_id: "test:pressure-projection", run_ref: ref("analysis_run", source.run_id), provenance: derivativeProvenance, unit_system_ref: ref("unit_system", "test:units"), result_sets: [{ values: [] }], reproducibility: {} } }, model(), source, origin)).rejects.toThrow(PRESSURE_1_OUTPUT_REFUSAL);
    expect(JSON.stringify(source)).toBe(before);
  });
});
