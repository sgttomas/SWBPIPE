/** B2 (DEL-07-02 Design `bulk-change.md`): each generated batch equals a hand-built batch of
 * the existing operations, and the family refusals hold. The canonicalizer is a stand-in that
 * labels its input, so the expected `before` values show what each operation is given. */
import { describe, expect, it } from "vitest";
import { bulkChangeModel } from "../../test-support/bulkChangeModel";
import type { EditorOperationIntent, EntityRef } from "../../types";
import { stableJson } from "../model-data/modelDataIntents";
import { buildBulkChangeBatch, planBlockedReason, planBulkChange, type BulkChangeRequest } from "./bulkChange";

const canon = async (value: unknown) => `canon:${stableJson(value)}`;
const pipe = (id: string): EntityRef => ({ type: "pipe", id });
const support = (id: string): EntityRef => ({ type: "support", id });
const node = (id: string): EntityRef => ({ type: "node", id });
const component = (id: string): EntityRef => ({ type: "component", id });
const BULK_SOURCE = { source_ref: "apps/desktop/src/features/bulk-change", source_channel: "local_desktop_preview", source_role: "gui_editor" };
const AUDIT = { mutation_route: "structured_operations_only", direct_model_mutation_allowed: false, requires_user_acceptance: true, mutates_accepted_model_state: false };
const PROFESSIONAL = { human_review_required: true, software_makes_compliance_claim: false, software_makes_certification_claim: false, software_makes_sealing_claim: false, software_makes_approval_claim: false, software_makes_authentication_claim: false };
const notRun = (unit: string) => ({ schema_validation: "not_run", constraint_validation: "not_run", unit_validation: unit, diff_preview_status: "not_generated", application_status: "not_applied" });

async function batch(scope: EntityRef[], request: BulkChangeRequest) {
  return buildBulkChangeBatch(bulkChangeModel(), scope, request, canon, "bulk-test-1");
}

describe("bulk change batches equal hand-built batches of the existing operations", () => {
  it("assigns a shared section with assign_section, before the canonical pipe", async () => {
    const model = bulkChangeModel();
    const expected: EditorOperationIntent[] = ["pipe:P1", "pipe:P2"].map((id) => ({
      operation_id: `op:rich:${id}:section_ref`, operation_kind: "modify", operation_status: "proposed", author_type: "user", source: BULK_SOURCE,
      target: { object_type: "Element", ref: id },
      change: { change_id: `change:rich:${id}:section_ref`, change_kind: "assign_section", field_label: "Assign shared section", field_path: "section_ref",
        before: `canon:${stableJson(model.pipe_segments.find((p) => p.id === id))}`, after: JSON.stringify({ section_ref: "section:S1" }), unit: "none", dimension: "dimensionless",
        source_note: "Explicit user-authored values and units; missing optional fields remain absent." },
      validation: notRun("not_run"), audit_boundary: AUDIT, professional_boundary: PROFESSIONAL,
      rationale: "User-authored assign shared section; requires validation and review before application."
    } as EditorOperationIntent));
    expect(await batch([pipe("pipe:P1"), pipe("pipe:P2")], { kind: "section", sectionId: "section:S1" })).toEqual({ batch_id: "bulk-test-1", operations: expected });
  });

  it("reassigns the material with set_field material, before the current material id", async () => {
    const expected = ["pipe:P2", "pipe:P3"].map((id) => ({
      operation_id: `op:bulk-${id}-material`, operation_kind: "modify", operation_status: "proposed", author_type: "user", source: BULK_SOURCE,
      target: { object_type: "Element", ref: id },
      change: { change_id: `change:bulk-${id}-material`, change_kind: "set_field", field_label: "Material", field_path: "material", before: "material:steel", after: "material:alloy",
        unit: "none", dimension: "dimensionless", source_note: "material reference" },
      validation: notRun("not_required_dimensionless"), audit_boundary: AUDIT, professional_boundary: PROFESSIONAL,
      rationale: "Bulk change: material material:alloy; requires validation and review before application."
    }));
    expect(await batch([pipe("pipe:P2"), pipe("pipe:P3")], { kind: "material", materialId: "material:alloy" })).toEqual({ batch_id: "bulk-test-1", operations: expected });
  });

  it("replaces a support kind with update_support configuration and writes the entered restraints", async () => {
    const config = { family: "guide", restraints: ["UY", "UZ"], provenance: "invented B2 bulk-change test input" };
    const expected = ["support:B", "support:C"].map((id) => ({
      operation_id: `op:rich:${id}:configuration`, operation_kind: "modify", operation_status: "proposed", author_type: "user", source: BULK_SOURCE,
      target: { object_type: "Support", ref: id },
      change: { change_id: `change:rich:${id}:configuration`, change_kind: "update_support", field_label: "Support configuration", field_path: "configuration",
        before: `canon:${stableJson(config)}`,
        after: JSON.stringify({ family: "line_stop", restraints: ["UX"], provenance: "invented B2 bulk-change test input" }),
        unit: "none", dimension: "dimensionless", source_note: "Explicit user-authored values and units; missing optional fields remain absent." },
      validation: notRun("not_run"), audit_boundary: AUDIT, professional_boundary: PROFESSIONAL,
      rationale: "User-authored support configuration; requires validation and review before application."
    }));
    // A run range: the pipes P1 and P2 reach the guides at B and C, and the anchor at A is not matched.
    const built = await batch([pipe("pipe:P1"), pipe("pipe:P2")], { kind: "support-kind", fromFamily: "guide", toFamily: "line_stop", restraints: ["UX"] });
    expect(built).toEqual({ batch_id: "bulk-test-1", operations: expected });
  });

  it("keeps each support's own restraints when asked, writing only the family", async () => {
    const built = await batch([support("support:B")], { kind: "support-kind", fromFamily: "guide", toFamily: "vertical_support", restraints: "keep" });
    expect(JSON.parse(built.operations[0].change.after)).toEqual({ family: "vertical_support", restraints: ["UY", "UZ"], provenance: "invented B2 bulk-change test input" });
  });

  it("removes a support kind with the inspector's delete_support intent, before the label", async () => {
    const expected = [["support:B", "Guide B"], ["support:C", "Guide C"]].map(([id, label]) => ({
      operation_id: `op:delete-support-${id}`, operation_kind: "delete", operation_status: "proposed", author_type: "user", source: BULK_SOURCE,
      target: { object_type: "Support", ref: id },
      change: { change_id: `change:delete-support:${id}`, change_kind: "delete_support", field_label: "Explicit support deletion", field_path: "supports", before: label, after: "not_present",
        unit: "none", dimension: "dimensionless", source_note: "explicit user-entered support deletion; reference integrity required" },
      validation: notRun("not_required_dimensionless"), audit_boundary: AUDIT, professional_boundary: PROFESSIONAL,
      rationale: "explicit user-entered support deletion for project:b2-bulk; requires reference validation before durable model change."
    }));
    expect(await batch([node("node:B"), node("node:C")], { kind: "support-remove", family: "guide" })).toEqual({ batch_id: "bulk-test-1", operations: expected });
  });

  it("sets an unrealized bend's radius with set_field geometry.bend_radius.value, before the current value", async () => {
    const expected = [{
      operation_id: "op:bulk-component:BEND-B-geometry.bend_radius.value", operation_kind: "modify", operation_status: "proposed", author_type: "user", source: BULK_SOURCE,
      target: { object_type: "Component", ref: "component:BEND-B" },
      change: { change_id: "change:bulk-component:BEND-B-geometry.bend_radius.value", change_kind: "set_field", field_label: "Bend radius", field_path: "geometry.bend_radius.value",
        before: "0.15", after: JSON.stringify({ value: 0.229, unit: "m" }), unit: "m", dimension: "length", source_note: "unit metadata required; entered unit captured explicitly" },
      validation: notRun("not_run"), audit_boundary: AUDIT, professional_boundary: PROFESSIONAL,
      rationale: "Bulk change: bend radius 0.229 m; requires validation and review before application."
    }];
    expect(await batch([component("component:BEND-B"), component("component:VALVE-C")], { kind: "bend-radius", value: "0.229", unit: "m" })).toEqual({ batch_id: "bulk-test-1", operations: expected });
  });

  it("adds a tag with each family's annotation kind, appending to the current tags", async () => {
    const expected = [
      ["Element", "pipe:P1", "set_field", `canon:${stableJson(["rack-1"])}`, ["rack-1", "what-if"]],
      ["Element", "pipe:P2", "set_field", "not_present", ["what-if"]],
      ["Support", "support:B", "update_support", "not_present", ["what-if"]]
    ].map(([objectType, ref, kind, before, tags]) => ({
      operation_id: `op:model-data:${kind}:${ref}:tags`, operation_kind: "modify", operation_status: "proposed", author_type: "user", source: BULK_SOURCE,
      target: { object_type: objectType, ref },
      change: { change_id: `change:model-data:${kind}:${ref}:tags`, change_kind: kind, field_label: "Tags", field_path: "tags", before, after: JSON.stringify(tags),
        unit: "none", dimension: "dimensionless", source_note: "User-entered model data; the record is stored with the model and is not evaluated." },
      validation: notRun("not_required_dimensionless"), audit_boundary: AUDIT, professional_boundary: PROFESSIONAL,
      rationale: "User-authored tags; requires validation and review before application."
    }));
    // support:C already carries the tag and is unchanged.
    expect(await batch([pipe("pipe:P1"), pipe("pipe:P2"), support("support:B"), support("support:C")], { kind: "tag", tag: "what-if" }))
      .toEqual({ batch_id: "bulk-test-1", operations: expected });
  });
});

describe("family-aware validation", () => {
  const model = bulkChangeModel();
  const refusedIds = (scope: EntityRef[], request: BulkChangeRequest) => planBulkChange(model, scope, request).refused.map((entry) => entry.ref.id);

  it("refuses a section or material change on a non-pipe", () => {
    expect(refusedIds([pipe("pipe:P1"), support("support:B"), node("node:A")], { kind: "section", sectionId: "section:S1" })).toEqual(["support:B", "node:A"]);
    expect(planBulkChange(model, [support("support:B")], { kind: "section", sectionId: "section:S1" }).refused[0].reason).toBe("A section is assigned to pipes only; support support:B is not one.");
    expect(refusedIds([component("component:VALVE-C")], { kind: "material", materialId: "material:alloy" })).toEqual(["component:VALVE-C"]);
  });

  it("refuses a non-pipe section and an unknown material as requests", () => {
    expect(planBulkChange(model, [pipe("pipe:P1")], { kind: "section", sectionId: "section:RIGID" }).requestError).toBe("Section section:RIGID is not a pipe section.");
    expect(planBulkChange(model, [pipe("pipe:P1")], { kind: "material", materialId: "material:none" }).requestError).toBe("Choose a material.");
  });

  it("refuses supports carrying data a kind replacement would discard, and non-support families", () => {
    const plan = planBulkChange(model, [support("support:D"), { type: "material", id: "material:steel" }], { kind: "support-kind", fromFamily: "spring", toFamily: "anchor", restraints: "keep" });
    expect(plan.refused.map((entry) => entry.reason)).toEqual([
      "A support change acts on supports, and on the supports at selected nodes and pipe ends; material material:steel is not one.",
      "Support support:D carries stiffness data that a kind replacement would discard; change it in Support configuration."
    ]);
  });

  it("never infers restraints: an entered set must be ticked", () => {
    const plan = planBulkChange(model, [support("support:B")], { kind: "support-kind", fromFamily: "guide", toFamily: "line_stop", restraints: [] });
    expect(plan.requestError).toBe("Tick the restraints to write; none are inferred from the kind.");
    expect(planBlockedReason(plan)).toBe(plan.requestError);
  });

  it("refuses removal of a normal-reaction source and a realized bend's radius", () => {
    const withSource = bulkChangeModel();
    withSource.supports.push({ id: "support:F", label: "Friction", node: "node:B", restraints: [], nonlinear: { behavior: "friction", dof: "UX", normal_reaction_source: { support_ref: "support:B", dof: "UY" } }, provenance: "x" } as never);
    expect(planBulkChange(withSource, [support("support:B")], { kind: "support-remove", family: "guide" }).refused[0].reason).toBe("Support support:B is still referenced by support:F; delete or retarget them first.");
    const radius = planBulkChange(model, [pipe("pipe:P3")], { kind: "bend-radius", value: "3", unit: "m" });
    expect(radius.refused.map((entry) => entry.ref.id)).toEqual(["component:BEND-R"]);
    expect(radius.notMatched.map((entry) => entry.ref.id)).toEqual(["component:VALVE-C"]);
  });

  it("refuses removal on the applier's whole reference set, in its order", () => {
    const referenced = bulkChangeModel();
    referenced.load_cases[0] = { ...referenced.load_cases[0],
      primitive_loads: [{ id: "load:settle-B", target: { type: "support", support: "support:B", dof: "UZ" } }],
      analysis_state: { support_states: [{ support_ref: "support:C" }, { support_ref: "support:B" }] } } as never;
    referenced.supports.push({ id: "support:F", label: "Friction", node: "node:B", restraints: [], nonlinear: { behavior: "friction", dof: "UX", normal_reaction_source: { support_ref: "support:B", dof: "UY" } }, provenance: "x" } as never);
    expect(planBulkChange(referenced, [node("node:B"), node("node:C")], { kind: "support-remove", family: "guide" }).refused.map((entry) => entry.reason)).toEqual([
      "Support support:B is still referenced by load:settle-B, support:F, case:w.analysis_state.support_states.1.support_ref; delete or retarget them first.",
      "Support support:C is still referenced by case:w.analysis_state.support_states.0.support_ref; delete or retarget them first."
    ]);
  });

  it("sorts reached entities into unchanged and not matched without refusing them", () => {
    const plan = planBulkChange(model, [node("node:A"), node("node:B")], { kind: "support-kind", fromFamily: "guide", toFamily: "guide", restraints: ["UZ", "UY"] });
    expect(plan.unchanged.map((entry) => entry.ref.id)).toEqual(["support:B"]);
    expect(plan.notMatched.map((entry) => entry.ref.id)).toEqual(["support:A"]);
    expect(planBlockedReason(plan)).toBe("No entity in the selection would change.");
  });

  it("builds nothing when any entity is refused", async () => {
    await expect(batch([pipe("pipe:P1"), support("support:B")], { kind: "material", materialId: "material:alloy" }))
      .rejects.toThrow("1 entity is refused; narrow the selection or change the request. Nothing is queued while any entity is refused.");
  });
});
