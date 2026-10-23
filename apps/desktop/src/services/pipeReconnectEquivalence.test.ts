// W-10 runtime-equivalence witness (DEL-16-06 CLM-003; Design
// `pipe-reconnect.md` §4): the same inline model and reconnect batch as the
// native `runtime_equivalence_witness_native` in
// `core/model_operations/operation_applier/tests/pipe_reconnect.rs`, built from
// this language's MODEL_SCHEMA_VERSION and run through the wasm engine. Both
// tests state the same codes, diff rows, warnings and applied document.
import { describe, expect, it } from "vitest";
import type { EditorOperationIntent, PreviewModel } from "../types";
import { applyOperationBatch } from "./operationBatchService";
import { canonicalJsonString, computeModelHash } from "./hashService";
import { MODEL_SCHEMA_VERSION } from "./projectService";

const PROV = "invented W-10 reconnect test input";
const node = (id: string, x: number, y: number, z: number) => ({ id, label: id.replace("node:", ""), position: { x, y, z }, provenance: PROV });
function equivalenceModel(): PreviewModel {
  return {
    schema_version: MODEL_SCHEMA_VERSION, document_kind: "openpipestress.product_preview.model",
    project: { id: "project:w10-equivalence", units: { length: "m", force: "N", angle: "rad", pressure: "Pa", stress: "Pa", temperature: "degC" } },
    analysis_status: { mechanics: "ready_for_preview_diagnostics", rule_check: "not_performed_user_rule_inputs_missing", professional_acceptance: "not_provided" },
    nodes: [node("node:A", 0, 0, 0), node("node:B", 2, 0, 0), node("node:C", 2, 2, 0)],
    pipe_segments: [{ id: "pipe:P", label: "P", from: "node:A", to: "node:B",
      section: { outside_diameter: { value: 0.1, unit: "m" }, wall_thickness: { value: 0.005, unit: "m" } },
      material: "material:steel", provenance: PROV, y_reference: { x: 0, y: 0, z: 1 } }],
    supports: [{ id: "support:A", node: "node:A", family: "anchor", restraints: ["UX", "UY", "UZ", "RX", "RY", "RZ"], provenance: PROV }],
    components: [],
    materials: [{ id: "material:steel", elastic_modulus: { value: 200e9, unit: "Pa" }, shear_modulus: { value: 80e9, unit: "Pa" }, provenance: PROV }],
    load_cases: [{ id: "case:w", primitive_loads: [], provenance: PROV }],
    combinations: []
  } as unknown as PreviewModel;
}
async function reconnect(model: PreviewModel): Promise<EditorOperationIntent> {
  const id = "reconnect:pipe:P:to:node:C";
  return {
    operation_id: `op:${id}`, operation_kind: "modify", operation_status: "proposed", author_type: "user",
    target: { object_type: "Element", ref: "pipe:P" },
    change: { change_id: `change:${id}`, change_kind: "reconnect_pipe_run", field_label: "pipe_segments", field_path: "pipe_segments",
      before: await canonicalJsonString(model.pipe_segments[0]), after: JSON.stringify({ pipe_ref: "pipe:P", end: "to", target_node_ref: "node:C" }),
      unit: "none", dimension: "dimensionless", source_note: "invented explicit test entry" },
    validation: { schema_validation: "not_run", constraint_validation: "not_run", unit_validation: "not_run", diff_preview_status: "not_generated", application_status: "not_applied" },
    rationale: "invented W-10 reconnect test operation",
    audit_boundary: { mutation_route: "structured_operations_only", direct_model_mutation_allowed: false, requires_user_acceptance: true, mutates_accepted_model_state: false },
    professional_boundary: { human_review_required: true, software_makes_compliance_claim: false, software_makes_certification_claim: false, software_makes_sealing_claim: false, software_makes_approval_claim: false, software_makes_authentication_claim: false }
  } as EditorOperationIntent;
}
const RECORD = (to: string) => `{"from":"node:A","id":"pipe:P","label":"P","material":"material:steel","provenance":"invented W-10 reconnect test input","section":{"outside_diameter":{"unit":"m","value":0.1},"wall_thickness":{"unit":"m","value":0.005}},"to":"${to}","y_reference":{"x":0,"y":0,"z":1}}`;

describe("W-10 reconnect: wasm runtime equivalence witness", () => {
  it("accepts with the stated diff row, warning and applied document", async () => {
    const model = equivalenceModel();
    const outcome = await applyOperationBatch(model, { batch_id: "batch:w10-equivalence-accept", operations: [await reconnect(model)] }, await computeModelHash(model));
    expect(outcome.validation.application_status, JSON.stringify(outcome.diagnostics)).toBe("applied_to_session_model");
    const step = outcome.operation_outcomes[0];
    expect(step.diff_preview).toEqual([{ entity_ref: "pipe:P", object_type: "Element", field_path: "pipe_segments",
      before: RECORD("node:B"), after: RECORD("node:C"), unit: "none", dimension: "dimensionless", change_kind: "reconnect_pipe_run" }]);
    expect(step.diagnostics.map(d => [d.code, d.severity, d.affected_refs])).toEqual([["OP-RECONNECT-NODE-ORPHANED", "warning", ["pipe:P", "node:B"]]]);
    const expected = equivalenceModel();
    expected.pipe_segments[0].to = "node:C";
    expect(outcome.applied_model).toEqual(expected);
  });
  it("refuses a tee at the node the pipe leaves with the stated blocker", async () => {
    const model = equivalenceModel();
    model.components = [{ id: "component:tee", kind: "tee", node: "node:B", provenance: PROV } as unknown as PreviewModel["components"][number]];
    const outcome = await applyOperationBatch(model, { batch_id: "batch:w10-equivalence-refuse", operations: [await reconnect(model)] }, await computeModelHash(model));
    expect(outcome.applied_model).toBeNull();
    const step = outcome.operation_outcomes[0];
    expect(step.diff_preview).toEqual([]);
    expect(step.diagnostics.map(d => [d.code, d.message, d.affected_refs])).toEqual([[
      "OP-RECONNECT-COMPONENT-ATTACHED",
      "Cannot reconnect pipe:P: component component:tee (tee) sits at node node:B, where this reconnect removes the pipe.",
      ["pipe:P", "component:tee", "node:B"]
    ]]);
  });
});
