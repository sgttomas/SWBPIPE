import { describe, expect, it } from "vitest";
import { buildGeometryBatch, emptyGeometryDraft, geometryPayload, orphanCleanupNode, reconnectPayload, type GeometryDraft } from "./geometryDraft";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { validateOperationBatch } from "../../services/operationBatchService";
import { reconnectModel } from "../../test-support/reconnectModel";

const draft = (over: Partial<GeometryDraft> = {}): GeometryDraft =>
  ({ ...emptyGeometryDraft(), action: "reconnect", pipeRefs: ["pipe:P"], reconnectEnd: "to", targetNode: "node:C", ...over });

describe("reconnect geometry draft (W-10)", () => {
  it("builds the exact payload and refuses incomplete drafts", () => {
    const model = reconnectModel();
    expect(geometryPayload(model, draft())).toEqual({ pipe_ref: "pipe:P", end: "to", target_node_ref: "node:C" });
    expect(reconnectPayload(model, draft({ yReference: { x: "0", y: "1", z: "0" } }))).toEqual({ pipe_ref: "pipe:P", end: "to", target_node_ref: "node:C", y_reference: { x: 0, y: 1, z: 0 } });
    expect(() => reconnectPayload(model, draft({ pipeRefs: ["pipe:P", "pipe:Q"] }))).toThrow("exactly one source pipe");
    expect(() => reconnectPayload(model, draft({ reconnectEnd: "" }))).toThrow("Choose which end");
    expect(() => reconnectPayload(model, draft({ targetNode: "node:absent" }))).toThrow("is not a node of the model");
    expect(() => reconnectPayload(model, draft({ yReference: { x: "1", y: "", z: "" } }))).toThrow("all three y_reference components");
  });
  it("queues one reconnect intent on the exact wire", async () => {
    const model = reconnectModel();
    const batch = await buildGeometryBatch(model, draft(), "batch:one");
    expect(batch.operations).toHaveLength(1);
    const [intent] = batch.operations;
    expect(intent).toMatchObject({ operation_kind: "modify", author_type: "user", target: { object_type: "Element", ref: "pipe:P" },
      change: { change_kind: "reconnect_pipe_run", field_path: "pipe_segments", unit: "none", dimension: "dimensionless", before: await canonicalJsonString(model.pipe_segments[0]) } });
    expect(JSON.parse(intent.change.after)).toEqual({ pipe_ref: "pipe:P", end: "to", target_node_ref: "node:C" });
  });
  it("queues [create_node, reconnect] for a new point and the batch validates on the engine", async () => {
    const model = reconnectModel();
    const batch = await buildGeometryBatch(model, draft({ toNewPoint: true, newNode: { id: "node:N", label: "New end", provenance: "invented UI test" }, newPoint: { x: "0", y: "5", z: "0" }, yReference: { x: "0", y: "0", z: "1" } }), "batch:new-point");
    expect(batch.operations.map((op) => op.change.change_kind)).toEqual(["create_node", "reconnect_pipe_run"]);
    expect(JSON.parse(batch.operations[1].change.after).target_node_ref).toBe("node:N");
    const outcome = await validateOperationBatch(model, batch, await computeModelHash(model));
    expect(outcome.validation.batch_validation_status, JSON.stringify(outcome.operation_outcomes.map((s) => s.diagnostics))).toBe("passed");
  });
  it("offers and appends delete_node only when nothing else names the node left", async () => {
    const model = reconnectModel();
    // B keeps Q: no cleanup. A is P's other end, with an anchor: no cleanup either.
    expect(orphanCleanupNode(model, draft())).toBeNull();
    expect(orphanCleanupNode(model, draft({ reconnectEnd: "from" }))).toBeNull();
    model.supports = model.supports.filter((s) => s.node !== "node:A");
    expect(orphanCleanupNode(model, draft({ reconnectEnd: "from" }))).toBe("node:A");
    const batch = await buildGeometryBatch(model, draft({ reconnectEnd: "from", deleteOrphan: true }), "batch:cleanup");
    expect(batch.operations.map((op) => op.change.change_kind)).toEqual(["reconnect_pipe_run", "delete_node"]);
    const outcome = await validateOperationBatch(model, batch, await computeModelHash(model));
    expect(outcome.validation.batch_validation_status, JSON.stringify(outcome.operation_outcomes.map((s) => s.diagnostics))).toBe("passed");
  });
});
