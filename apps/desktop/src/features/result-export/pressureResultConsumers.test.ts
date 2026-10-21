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
import { surfaceOutputRefusal } from "../results/outputPolicy";
const resultSemantics = (row: Parameters<ReturnType<typeof readerRowSemantics>>[0], source?: Parameters<typeof readerRowSemantics>[0]) => readerRowSemantics(source)(row).signature;
const canonicalResultMetadata = (row: Parameters<ReturnType<typeof readerRowSemantics>>[0], source?: Parameters<typeof readerRowSemantics>[0]) => readerRowSemantics(source)(row).canonical_metadata;
import { buildAnalysisRunV03, modelLoadBasisRefs, validateAnalysisRunV03 } from "../../services/analysisRunCompatibility";
import { deriveResultDocument, buildCurrentResultExport, resultDigest, ref, derivativeProvenance, validateResultDocument } from "./resultExportAdapter";

/** B7b: the v3 twin of the retired physics-1 consumer suite. The exact route's ordinary
 * identity is pressure-2 (3.0.0/exact_pressure_v3); the desktop reads it through the Rust
 * reader's own route and records it as Python's builder does (DEL-14-02: the record binds
 * pressure-2's table and embeds no evidence). Epoch 8 (DEL-04-07 Design
 * `desktop-exact-outputs.md` §4.2): the result JSON export admits it at the Rust reader's
 * standing; the Current builder still requires its record, manifest and live native
 * capture, and the pure projection copies `contract_evidence` whole, as Rust's
 * `derive_document` does. Actual committed producer bytes; loading them here creates no
 * native invocation or Current proof. */
const project = resolve(__dirname, "../../../../../");
const json = (path: string) => JSON.parse(readFileSync(resolve(project, path), "utf8"));
const raw = (mode = "sparse"): MechanicsResult => json(`fixtures/results/pressure_connected_mechanics_${mode}.json`);
const model = (): PreviewModel => json("core/product_physics/tests/fixtures/exact_pressure_connected_request.json").model;
const evidence = (source: MechanicsResult): any => source.contract_evidence; // eslint-disable-line @typescript-eslint/no-explicit-any

describe("actual pressure-2 source consumers (B7b: the v3 twin of physics-1)", () => {
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
  it("pins the pressure-2 table bytes", () => {
    expect(createHash("sha256").update(readFileSync(resolve(project, "fixtures/results/semantic_contract_v0_3_pressure_2.json"))).digest("hex")).toBe(PRESSURE_CONTRACT_SHA256);
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
    // T4-U6 (D-5 sections 2.2, 2.5): the member's maximum is withdrawn on both section bases (each
    // basis has its own maximum record, coverage record and headline), so both headlines are withheld.
    const source = raw(), c = evidence(source).exact_cases[1], pipe = c.pipe_materials[0].pipe_id;
    const removed: string[] = c.pipe_stress_extrema.filter((x: { pipe_id: string }) => x.pipe_id === pipe).map((x: { result_id: string }) => x.result_id);
    expect(removed).toHaveLength(2);
    c.stress_maximum_coverage = ["nominal_section", "reduced_section"].map(section_basis => ({ section_basis, complete: false, unavailable_pipe_ids: [pipe] }));
    c.pipe_stress_extrema = c.pipe_stress_extrema.filter((x: { pipe_id: string }) => x.pipe_id !== pipe);
    source.results = source.results.filter(r => !removed.includes(r.id));
    expect(source.summary.max_open_formula_stress).toBeNull();
    expect(() => readerValidate("pressure", source)).toThrow("SOURCE_PHYSICS_INCOMPLETE_STRESS_HEADLINE");
    // Either basis's headline, kept over its incomplete coverage, is refused.
    for (const [withheld, kept] of [["max_open_formula_stress_nominal_section", "max_open_formula_stress_reduced_section"], ["max_open_formula_stress_reduced_section", "max_open_formula_stress_nominal_section"]] as const) {
      const one = structuredClone(source); one.summary[withheld] = null;
      expect(one.summary[kept]).toBeTruthy();
      expect(() => readerValidate("pressure", one)).toThrow("SOURCE_PHYSICS_INCOMPLETE_STRESS_HEADLINE");
    }
    source.summary.max_open_formula_stress_nominal_section = null;
    source.summary.max_open_formula_stress_reduced_section = null;
    expect(() => readerValidate("pressure", source)).not.toThrow();
  });
  it("keeps Sensitive evidence inspectable without promoting its accuracy claim", () => {
    const source = raw(); source.numerical_quality!.status = "sensitive"; source.numerical_quality!.cases[0].solve_quality = "sensitive"; source.numerical_quality!.cases[0].accuracy_evidence = "reference_verified";
    expect(sourceContract(source)).toBe("pressure"); expect(numericalResultStanding(source, model()).eligible).toBe(false);
  });
  it("has no transport reader: a transported pressure-2 statement is refused by name, and a retired reader name is unknown", () => {
    expect(() => readerTransport("pressure", raw())).toThrow("SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED");
    for (const retired of ["physics", "physics-source", "load-reference", "load-reference-source"]) {
      expect(() => readerValidate(retired as never, raw()), retired).toThrow(`READER_UNKNOWN: ${retired}`);
    }
  });
  it.each(["sparse", "dense"])("%s: builds its analysis-run record; the Current builder still needs its manifest, and the projection copies the evidence (epoch 8, §4.2)", async mode => {
    const source = raw(mode), before = JSON.stringify(source);
    const record = { model_basis: { model_ref: source.model_ref, model_payload: model() }, solver_basis: { solver_name: source.producer!.component_name, solver_version: source.producer!.component_version, solver_build_ref: "test:reference-only", solver_mode: "sparse_interactive", settings: {} } };
    const inputManifest = { manifest: record, manifest_ref: { object_type: "InputManifest", ref: "test:reference-only-pressure-analysis" }, manifest_sha256: await resultDigest(record) };
    const analysisRun = await buildAnalysisRunV03(source, inputManifest as never, undefined, modelLoadBasisRefs(model()));
    expect(analysisRun.analysis_run.reproducibility.semantic_contract).toStrictEqual({ id: PRESSURE_CONTRACT_ID, sha256: PRESSURE_CONTRACT_SHA256 });
    expect(Object.hasOwn(analysisRun.analysis_run, "contract_evidence")).toBe(false);
    // The record and its source validate after a JSON reopen; a changed source no longer validates against it.
    await validateAnalysisRunV03(JSON.parse(JSON.stringify(analysisRun)), JSON.parse(JSON.stringify(source)), modelLoadBasisRefs(model()));
    const changed = raw(mode); evidence(changed).pressure[0].provenance += ":changed";
    await expect(validateAnalysisRunV03(analysisRun, changed, modelLoadBasisRefs(model()))).rejects.toThrow("ANALYSIS_RECEIVED_SOURCE_MISMATCH");
    // Epoch 8 (§4.2): the surface admits pressure-2 at Rust's standing (no policy refusal),
    // and the Current builder keeps its refusal of a result without its manifest.
    expect(surfaceOutputRefusal(source, model(), "result-export")).toBeNull();
    await expect(buildCurrentResultExport({ model: model(), result: source, analysisRun, inputManifest: null })).rejects.toThrow("CURRENT_INPUT_MANIFEST_UNAVAILABLE");
    const origin = { origin_id: "test:received-source", origin_class: "received_current_dimension_absent", qualification_ref: ref("test_fixture", "unattested-source-consumer-projection"), authentic_producer_available: false,
      received_carrier_checksum: { algorithm: "sha256", canonicalization: "openpipestress_jcs_ijson_v1", payload_scope: "received_current_dimension_absent_carrier", payload_ref: ref("received_current_carrier", source.run_id), value: await resultDigest(source) }, original_producer_checksum: null,
      origin_limit: "Consumer projection only; no native producer or Current qualification", actual_model_ref: ref("model_payload", source.model_ref), mechanics_run_ref: ref("mechanics_run", source.run_id), request_model_ref: null, request_run_ref: null, request_alias_disclosure: null };
    // The pure projection derives the 0.3 document through the full pressure-2 reader, with
    // contract_evidence copied whole, no receipt, and pressure-2's contract reference.
    const doc = await deriveResultDocument({ schema_version: "0.2.0", result_envelope: { schema_version: "0.2.0", envelope_id: "test:pressure-projection", run_ref: ref("analysis_run", source.run_id), provenance: derivativeProvenance, unit_system_ref: ref("unit_system", "test:units"), result_sets: [{ values: [] }], reproducibility: {} } }, model(), source, origin);
    expect(doc.schema_version).toBe("0.3.0");
    expect(doc.result_envelope.contract_evidence).toStrictEqual(source.contract_evidence);
    expect(Object.hasOwn(doc.result_envelope, "source_block_recovery")).toBe(false);
    expect(doc.result_envelope.semantic_contract_ref).toStrictEqual(ref("semantic_contract", PRESSURE_CONTRACT_ID));
    expect(doc.result_envelope.row_accounting).toHaveLength(source.results.length);
    await expect(validateResultDocument(JSON.parse(JSON.stringify(doc)), source)).resolves.toBeUndefined();
    // The validator binds the copy to its source: a changed source no longer validates it.
    await expect(validateResultDocument(doc, changed)).rejects.toThrow("SOURCE_PHYSICAL_METADATA_MISMATCH");
    expect(JSON.stringify(source)).toBe(before);
  });
});
