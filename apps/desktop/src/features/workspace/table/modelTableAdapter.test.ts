import { describe, expect, it } from "vitest";
import { loadPreviewModel } from "../../../services/previewService";
import { computeModelHash } from "../../../services/hashService";
import { applyModelOperation } from "../../../services/operationService";
import { applyOperationBatch, validateOperationBatch } from "../../../services/operationBatchService";
import { buildGridOperationIntent, type GridColumn } from "./modelTableAdapter";

describe("shared table operation adapter", () => {
  it("preserves exact typed coordinates and yields the same engine model for an explicitly reviewed offline agent batch", async () => {
    const model = await loadPreviewModel(); const original = JSON.stringify(model);
    const node = model.nodes[0];
    const column: GridColumn = { key: "y", label: "Y", fieldPath: "position.y", objectType: "Node", changeKind: "set_field", dimension: "length", sourceNote: "unit metadata required; entered unit captured explicitly", unit: () => model.project.units.length, value: () => String(node.position.y) };
    const intent = buildGridOperationIntent({ column, model, row: { id: node.id, label: node.label, type: "node", searchText: "", raw: node }, sequence: 1, value: "0.5", interaction: "cell" });
    expect(intent.target).toEqual({ object_type: "Node", ref: node.id });
    expect(intent.change).toMatchObject({ field_path: "position.y", before: String(node.position.y), after: JSON.stringify({ value: 0.5, unit: model.project.units.length }), unit: model.project.units.length, dimension: "length" });
    const hash = await computeModelHash(model); const human = await applyModelOperation(model, intent, hash);
    expect(human.validation.application_status).toBe("applied_to_session_model");
    const agent = structuredClone(intent); agent.operation_id = "op:offline-table-equivalence"; agent.change.change_id = "change:offline-table-equivalence"; agent.author_type = "agent";
    agent.source = { source_ref: "offline:table-equivalence-fixture", source_channel: "offline_user_supplied", source_role: "agent" }; agent.rationale = "Explicitly reviewed offline coordinate proposal.";
    const batch = { batch_id: "batch:table-equivalence", operations: [agent] };
    const reviewed = await validateOperationBatch(model, batch, hash);
    expect(reviewed.validation.batch_validation_status).toBe("passed"); expect(reviewed.applied_model).toBeNull();
    const applied = await applyOperationBatch(model, batch, hash);
    expect(applied.validation.application_status).toBe("applied_to_session_model"); expect(applied.applied_model).toEqual(human.applied_model);
    expect(applied.submitted_operations[0]).toEqual(agent); expect(agent.author_type).toBe("agent"); expect(intent.author_type).toBe("user");
    expect(JSON.stringify(model)).toBe(original);
  });
});

describe("Materials existing quantity contract", () => {
  it.each([
    ["elastic_modulus.value", "2", true], ["shear_modulus.value", "2", true],
    ["elastic_modulus.value", "0", false], ["shear_modulus.value", "-1", false],
    ["thermal_expansion_coefficient.value", "0", true], ["thermal_expansion_coefficient.value", "-0.00001", true],
    ["elastic_modulus.value", "TBD", false], ["elastic_modulus.value", "", false], ["elastic_modulus.value", "Infinity", false]
  ] as const)("%s = %s retains unit and receives engine acceptance=%s", async (fieldPath, value, accepted) => {
    const model = await loadPreviewModel(); const material = model.materials![0];
    const quantity = material[fieldPath.split(".")[0] as "elastic_modulus" | "shear_modulus" | "thermal_expansion_coefficient"]!;
    const column: GridColumn = { key: fieldPath, label: fieldPath, fieldPath, objectType: "Material", changeKind: "set_field", dimension: fieldPath.startsWith("thermal") ? "thermal_expansion_coefficient" : "stress", sourceNote: "actual sibling unit", unit: () => quantity.unit, value: () => String(quantity.value) };
    const intent = buildGridOperationIntent({ column, model, row: { id: material.id, label: material.label, type: "material", searchText: "", raw: material }, sequence: 1, value, interaction: "cell" });
    const result = await applyModelOperation(model, intent, await computeModelHash(model));
    expect(result.validation.application_status === "applied_to_session_model").toBe(accepted);
    expect(intent.change.before).toBe(String(quantity.value)); expect(intent.change.unit).toBe(quantity.unit);
    if (accepted) expect((result.applied_model!.materials![0] as unknown as Record<string, { value: number; unit: string }>)[fieldPath.split(".")[0]]).toMatchObject({ value: Number(value), unit: quantity.unit });
  });
});
