// W-15 slice d (DEL-03-05 Design `rigid-component-stiffness-scaling.md` §5, §6): the
// desktop's reading of a component body's reference-section rows. The source is the
// bundled demo's regenerated preview-physics-2 result, whose valve C-130 (scale 15)
// maps pipe:P-130; the export rows are projected through the pure desktop exporter.
import { describe, expect, it } from "vitest";
import demoJson from "../../../../../fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json";
import demoModelJson from "../../../../../fixtures/product_preview/invented_demo_model.json";
import type { MechanicsResult, PreviewModel } from "../../types";
import { deriveResultDocument, derivativeProvenance, ref, resultDigest, validateResultDocument } from "../result-export/resultExportAdapter";
import { componentBodies, componentBodyNotices } from "./componentBody";
import { COMPONENT_BODY_REFERENCE_SECTION, ruleBindingRefusal } from "./knownSemanticLimitations";

const demo = () => structuredClone(demoJson) as unknown as MechanicsResult;
const RECORD = { component_ref: "component:C-130", stiffness_scale: 15, section_role: "reference_pipe_section" };
const MAXIMUM = "pipe_elastic_normal_stress_maximum_v2";

describe("component-body identification (W-15 slice d)", () => {
  it("names the demo's one body and never puts the headline on it", () => {
    const result = demo();
    const bodies = componentBodies(result);
    expect([...bodies.keys()]).toEqual(["pipe:P-130"]);
    expect(bodies.get("pipe:P-130")).toEqual(RECORD);
    expect(result.summary.max_open_formula_stress?.location_ref).not.toBe("pipe:P-130");
    const bodyRow = result.results.find(row => row.entity_ref === "pipe:P-130" && row.kind === MAXIMUM)!;
    const pipeRow = result.results.find(row => row.entity_ref !== "pipe:P-130" && row.kind === MAXIMUM)!;
    expect(componentBodyNotices(result, { resultIds: [bodyRow.id, pipeRow.id] })).toEqual([
      "pipe:P-130 is the body of component component:C-130 (stiffness scale 15); its stress rows are those of the mapped pipe section as a reference section, not a stress of the component body",
    ]);
    expect(componentBodyNotices(result, { resultIds: [pipeRow.id] })).toEqual([]);
    expect(componentBodyNotices(result, { pipeIds: ["pipe:P-130"] })).toHaveLength(1);
  });

  it("refuses a rule bound to a body row with the reference-section reason", () => {
    const result = demo();
    for (const row of result.results.filter(candidate => candidate.kind === MAXIMUM)) {
      expect(ruleBindingRefusal(result, row)).toBe(row.entity_ref === "pipe:P-130" ? COMPONENT_BODY_REFERENCE_SECTION : null);
    }
  });

  it("the desktop exporter copies the record verbatim and validates it against the source row", async () => {
    const source = demo();
    // The model-bound reader derives the expected body from the model (C-130 on pipe:P-130).
    const model = structuredClone(demoModelJson) as unknown as PreviewModel;
    const origin = { origin_id: "synthetic-projection-unattested", origin_class: "received_current_dimension_absent", qualification_ref: ref("synthetic_fixture", "not-Current"), authentic_producer_available: false, received_carrier_checksum: { algorithm: "sha256", canonicalization: "openpipestress_jcs_ijson_v1", payload_scope: "received_current_dimension_absent_carrier", payload_ref: ref("synthetic_received_carrier", "fixture"), value: await resultDigest(source) }, original_producer_checksum: null, origin_limit: "pure projection not authenticated Current", actual_model_ref: ref("model_payload", source.model_ref), mechanics_run_ref: ref("mechanics_run", source.run_id), request_model_ref: null, request_run_ref: null, request_alias_disclosure: null };
    const base = { schema_version: "0.3.0", result_envelope: { schema_version: "0.3.0", envelope_id: "synthetic", run_ref: ref("analysis_run", source.run_id), provenance: derivativeProvenance, unit_system_ref: ref("unit_system", "fixture"), result_sets: [{ values: [] }], reproducibility: {} } };
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const doc: any = await deriveResultDocument(base as any, model, source, origin as any);
    await validateResultDocument(doc, source);
    // A model without the body (its scale not consumed) does not admit the result's records.
    const unscaled = structuredClone(model) as any; // eslint-disable-line @typescript-eslint/no-explicit-any
    unscaled.components.find((c: { id: string }) => c.id === "component:C-130").mechanics_interface.solver_consumption = "not_solver_consumed";
    await expect(deriveResultDocument(base as any, unscaled, source, origin as any)).rejects.toThrow("SOURCE_COMPONENT_BODY_MODEL_MISMATCH"); // eslint-disable-line @typescript-eslint/no-explicit-any
    const accounting: Array<{ source_row_index: number; disposition: string; target_field_path: string }> = doc.result_envelope.row_accounting;
    const target = (copy: typeof doc, index: number) => accounting[index].target_field_path.split("/").slice(1).reduce((node: any, key: string) => node[key], copy); // eslint-disable-line @typescript-eslint/no-explicit-any
    let carried = 0;
    source.results.forEach((row, index) => {
      expect(doc.result_envelope.source_annotations[index].component_body).toEqual(row.component_body);
      if (accounting[index].disposition === "exported_quantity") {
        expect(target(doc, index).component_body).toEqual(row.component_body);
        if (row.component_body) carried += 1;
      }
    });
    expect(carried).toBeGreaterThan(0);
    const body = source.results.findIndex((row, index) => row.component_body && accounting[index].disposition === "exported_quantity");
    const pipe = source.results.findIndex((row, index) => !row.component_body && row.kind === MAXIMUM && accounting[index].disposition === "exported_quantity");
    const tampered = (edit: (copy: typeof doc) => void) => { const copy = structuredClone(doc); edit(copy); return validateResultDocument(copy, source); };
    await expect(tampered(copy => { delete target(copy, body).component_body; })).rejects.toThrow("SOURCE_COMPONENT_BODY_MISMATCH");
    await expect(tampered(copy => { target(copy, body).component_body.stiffness_scale = 16; })).rejects.toThrow("SOURCE_COMPONENT_BODY_MISMATCH");
    await expect(tampered(copy => { target(copy, pipe).component_body = RECORD; })).rejects.toThrow("SOURCE_COMPONENT_BODY_MISMATCH");
    await expect(tampered(copy => { delete copy.result_envelope.source_annotations[body].component_body; })).rejects.toThrow("SOURCE_ANNOTATION");
    // The coverage-only body list on a quantity row or a source annotation (round-2 scrutiny of #1604).
    await expect(tampered(copy => { target(copy, body).component_body_pipe_ids = ["nonexistent"]; })).rejects.toThrow("SOURCE_COMPONENT_BODY_MISMATCH");
    await expect(tampered(copy => { target(copy, pipe).component_body_pipe_ids = ["nonexistent"]; })).rejects.toThrow("SOURCE_COMPONENT_BODY_MISMATCH");
    await expect(tampered(copy => { copy.result_envelope.source_annotations[body].component_body_pipe_ids = ["nonexistent"]; })).rejects.toThrow("SOURCE_ANNOTATION");
    const other = accounting.findIndex(a => a.disposition !== "exported_quantity");
    expect(other).toBeGreaterThanOrEqual(0);
    await expect(tampered(copy => { target(copy, other).component_body_pipe_ids = ["nonexistent"]; })).rejects.toThrow("SOURCE_COMPONENT_BODY_MISMATCH");
  });
});
