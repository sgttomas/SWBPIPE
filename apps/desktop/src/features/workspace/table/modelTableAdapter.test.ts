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
