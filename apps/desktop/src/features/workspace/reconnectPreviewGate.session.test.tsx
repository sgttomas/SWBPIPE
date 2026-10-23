/** W-10 (DEL-16-06 Design `pipe-reconnect.md` §3): any batch containing a
 * `reconnect_pipe_run` is applied only after a clean preview, whatever its
 * producer; the apply path enforces the same predicate as the button, and a
 * single reconnect intent is routed to batch review. The session runs the
 * actual wasm engine on an invented inline model. */
import { act, renderHook, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import * as previewService from "../../services/previewService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { buildReconnectBatch, emptyGeometryDraft, type GeometryDraft } from "../geometry-tools/geometryDraft";
import { parseOfflineBatch } from "../offline-proposal-intake/OfflineProposalIntakePanel";
import { buildGridOperationIntent, type GridColumn } from "./table/modelTableAdapter";
import { useWorkspaceSession, type WorkspaceSession } from "./workspaceSession";
type WorkspaceSessionApply = WorkspaceSession["operations"]["handleApplyIntent"];
import type { PreviewModel } from "../../types";
import { reconnectModel } from "../../test-support/reconnectModel";

function draft(over: Partial<GeometryDraft> = {}): GeometryDraft {
  return { ...emptyGeometryDraft(), action: "reconnect", pipeRefs: ["pipe:P"], reconnectEnd: "to", targetNode: "node:C", yReference: { x: "0", y: "0", z: "1" }, ...over };
}
function coordinateIntent(model: PreviewModel) {
  const target = model.nodes[2];
  const column: GridColumn = { key: "y", label: "Y", fieldPath: "position.y", objectType: "Node", changeKind: "set_field", dimension: "length", sourceNote: "entered coordinate", unit: () => model.project.units.length, value: () => String(target.position.y) };
  return buildGridOperationIntent({ column, model, row: { id: target.id, label: target.label, type: "node", raw: target, searchText: "" }, sequence: 1, value: "2.5", interaction: "cell" });
}
async function session() {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(reconnectModel());
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return hook.result;
}
afterEach(() => { vi.restoreAllMocks(); });

describe("reconnect preview gate (central, producer-independent)", () => {
  it("sets requiresPreview for builder, offline-intake and requeued reconnect batches, not for other batches", async () => {
    const result = await session();
    const model = structuredClone(result.current.model.model!);
    const built = await buildReconnectBatch(model, draft(), "batch:builder");
    await act(async () => { await result.current.operations.handleQueueOperationBatch(built); });
    await act(async () => { await result.current.operations.handleQueueOperationBatch(parseOfflineBatch(JSON.stringify({ ...built, batch_id: "batch:imported" }))); });
    await act(async () => { await result.current.operations.handleQueueOperationBatch({ batch_id: "requeue-0", operations: [structuredClone(built.operations[0])] }); });
    await act(async () => { await result.current.operations.handleQueueOperationBatch({ batch_id: "batch:other", operations: [coordinateIntent(model)] }); });
    expect(result.current.operations.queuedBatches.map((entry) => [entry.batch.batch_id, entry.requiresPreview])).toEqual([
      ["batch:builder", true], ["batch:imported", true], ["requeue-0", true], ["batch:other", false]]);
    // A batch without a reconnect applies as today, with no preview.
    const other = result.current.operations.queuedBatches[3];
    await act(async () => { await result.current.operations.handleRunOperationBatch(other, true); });
    expect(result.current.model.model!.nodes[2].position.y).toBe(2.5);
  });

  it("refuses an unpreviewed reconnect batch on the apply path, applies after a clean preview, and Undo restores the absent y_reference", async () => {
    const result = await session();
    const original = structuredClone(result.current.model.model!);
    const originalHash = (await computeModelHash(original))!.value;
    await act(async () => { await result.current.operations.handleQueueOperationBatch(await buildReconnectBatch(original, draft(), "batch:gate")); });
    const entry = result.current.operations.queuedBatches[0];
    await act(async () => { await result.current.operations.handleRunOperationBatch(entry, true); });
    expect(result.current.model.model).toEqual(original);
    expect(result.current.operations.undoStack).toHaveLength(0);
    expect(result.current.operations.batchMessage).toContain("was not applied. This batch contains a reconnect.");
    await act(async () => { await result.current.operations.handleRunOperationBatch(entry, false); });
    expect(result.current.operations.batchOutcomes[entry.key].validation.batch_validation_status).toBe("passed");
    await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], true); });
    const applied = result.current.model.model!.pipe_segments.find((pipe) => pipe.id === "pipe:P")!;
    expect(applied.to).toBe("node:C");
    expect(applied.y_reference).toEqual({ x: 0, y: 0, z: 1 });
    expect(result.current.operations.undoStack).toHaveLength(1);
    // The forward change added a y_reference: only the checkpoint Undo restores its absence.
    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect(result.current.model.model!.pipe_segments[0].y_reference).toBeUndefined();
    expect((await computeModelHash(result.current.model.model!))!.value).toBe(originalHash);
    // The retained intent requeues as a gated batch.
    const retained = result.current.operations.retainedReviewContext.find((intent) => intent.change.change_kind === "reconnect_pipe_run")!;
    await act(async () => { await result.current.operations.handleQueueOperationBatch({ batch_id: "requeue-1", operations: [retained] }); });
    expect(result.current.operations.queuedBatches.at(-1)!.requiresPreview).toBe(true);
  });

  it("(a) refuses a direct call whose entry omits requiresPreview, registered or not", async () => {
    const result = await session();
    const original = structuredClone(result.current.model.model!);
    const batch = await buildReconnectBatch(original, draft(), "batch:bypass-a");
    // Not registered at all: the handler refuses without running it.
    const forged = { key: "operation-batch-forged", batch, basisModel: original, basisHash: (await computeModelHash(original))!, basisRevision: result.current.operations.queuedBatches.length };
    await act(async () => { await result.current.operations.handleRunOperationBatch(forged, true); });
    expect(result.current.operations.batchMessage).toContain("is not registered in the review queue");
    await act(async () => { await result.current.operations.handleQueueOperationBatch(batch); });
    const registered = result.current.operations.queuedBatches[0];
    expect(Object.isFrozen(registered) && Object.isFrozen(registered.batch.operations[0].change)).toBe(true);
    // Registered, but the caller's copy drops the flag: the gate follows the registered operations.
    const { requiresPreview: _dropped, ...withoutFlag } = registered;
    await act(async () => { await result.current.operations.handleRunOperationBatch(withoutFlag as typeof registered, true); });
    expect(result.current.model.model).toEqual(original);
    expect(result.current.operations.undoStack).toHaveLength(0);
    expect(result.current.operations.batchMessage).toContain("This batch contains a reconnect.");
  });

  it("(b) applies only the registered batch that was previewed, never an altered batch under the same key", async () => {
    const result = await session();
    const original = structuredClone(result.current.model.model!);
    await act(async () => { await result.current.operations.handleQueueOperationBatch(await buildReconnectBatch(original, draft(), "batch:bypass-b")); });
    const registered = result.current.operations.queuedBatches[0];
    await act(async () => { await result.current.operations.handleRunOperationBatch(registered, false); });
    expect(result.current.operations.batchOutcomes[registered.key].validation.batch_validation_status).toBe("passed");
    // Same key and batch_id, P -> D instead of the previewed P -> C.
    const altered = structuredClone(await buildReconnectBatch(original, draft({ targetNode: "node:D" }), "batch:bypass-b"));
    const forged = { ...structuredClone(registered), batch: altered, requiresPreview: false };
    await act(async () => { await result.current.operations.handleRunOperationBatch(forged as typeof registered, true); });
    const pipe = result.current.model.model!.pipe_segments.find((p) => p.id === "pipe:P")!;
    expect(pipe.to).toBe("node:C");
    expect(result.current.operations.undoStack).toHaveLength(1);
  });

  it("(c) a retained run handler cannot apply a previewed batch after the queue is cleared", async () => {
    const result = await session();
    const original = structuredClone(result.current.model.model!);
    await act(async () => { await result.current.operations.handleQueueOperationBatch(await buildReconnectBatch(original, draft(), "batch:withdrawn")); });
    const registered = result.current.operations.queuedBatches[0];
    await act(async () => { await result.current.operations.handleRunOperationBatch(registered, false); });
    expect(result.current.operations.batchOutcomes[registered.key].validation.batch_validation_status).toBe("passed");
    // Keep the handler of this render (its closure still sees the queue and outcome).
    const retainedRun = result.current.operations.handleRunOperationBatch;
    act(() => result.current.operations.handleClearReviewQueue());
    expect(result.current.operations.queuedBatches).toHaveLength(0);
    await act(async () => { await retainedRun(registered, true); });
    expect(result.current.model.model).toEqual(original);
    expect(result.current.operations.undoStack).toHaveLength(0);
    expect(result.current.operations.batchMessage).toContain("is not registered in the review queue");
  });

  it("a selection change does not strand a queued, previewed batch", async () => {
    const result = await session();
    const original = structuredClone(result.current.model.model!);
    await act(async () => { await result.current.operations.handleQueueOperationBatch(await buildReconnectBatch(original, draft(), "batch:select")); });
    await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], false); });
    act(() => { result.current.selection.handleSelectEntity({ type: "node", id: "node:C" }); });
    await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], true); });
    expect(result.current.model.model!.pipe_segments.find((p) => p.id === "pipe:P")!.to).toBe("node:C");
  });

  it("keeps Apply refused after a blocking preview", async () => {
    const result = await session();
    const model = structuredClone(result.current.model.model!);
    // T = F: zero length, a blocking finding.
    await act(async () => { await result.current.operations.handleQueueOperationBatch(await buildReconnectBatch(model, draft({ targetNode: "node:A" }), "batch:blocked")); });
    const entry = result.current.operations.queuedBatches[0];
    await act(async () => { await result.current.operations.handleRunOperationBatch(entry, false); });
    expect(result.current.operations.batchOutcomes[entry.key].operation_outcomes[0].diagnostics.map((d) => d.code)).toContain("OP-RECONNECT-ZERO-LENGTH");
    await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], true); });
    expect(result.current.model.model).toEqual(model);
    expect(result.current.operations.undoStack).toHaveLength(0);
  });

  it("routes a single reconnect intent to batch review and applies other intents as today", async () => {
    const result = await session();
    const model = structuredClone(result.current.model.model!);
    const [intent] = (await buildReconnectBatch(model, draft(), "batch:single")).operations;
    let applied = true;
    await act(async () => { applied = await result.current.operations.handleApplyIntent(intent); });
    expect(applied).toBe(false);
    expect(result.current.model.model).toEqual(model);
    expect(result.current.operations.undoStack).toHaveLength(0);
    expect(result.current.operations.operationMessage).toBe("Reconnect is applied through batch review; it has been queued there.");
    expect(result.current.operations.queuedBatches).toHaveLength(1);
    expect(result.current.operations.queuedBatches[0].requiresPreview).toBe(true);
    expect(result.current.operations.queuedBatches[0].batch.operations).toEqual([intent]);
    await act(async () => { applied = await result.current.operations.handleApplyIntent(coordinateIntent(model)); });
    expect(applied).toBe(true);
  });
});

describe("delete_pipe_run dependents (W-10 §7)", () => {
  const boundary = {
    validation: { schema_validation: "not_run", constraint_validation: "not_run", unit_validation: "not_run", diff_preview_status: "not_generated", application_status: "not_applied" },
    audit_boundary: { mutation_route: "structured_operations_only", direct_model_mutation_allowed: false, requires_user_acceptance: true, mutates_accepted_model_state: false },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    }
  };
  const deletePipe = (model: PreviewModel) => {
    const pipe = model.pipe_segments.find((p) => p.id === "pipe:R")!;
    return { operation_id: "op:delete-pipe-R", operation_kind: "delete", operation_status: "proposed", author_type: "user", target: { object_type: "Element", ref: "pipe:R" },
      change: { change_id: "change:delete-pipe-R", change_kind: "delete_pipe_run", field_label: "Delete pipe", field_path: "pipe_segments", before: `${pipe.label}; ${pipe.from}->${pipe.to}; material=${pipe.material}`, after: "not_present", unit: "none", dimension: "dimensionless", source_note: "invented" },
      rationale: "invented W-10 delete", ...boundary } as unknown as Parameters<WorkspaceSessionApply>[0];
  };
  const deleteComponent = async (model: PreviewModel) => ({ operation_id: "op:delete-bend", operation_kind: "delete", operation_status: "proposed", author_type: "user", target: { object_type: "Component", ref: "component:bend-R" },
    change: { change_id: "change:delete-bend", change_kind: "delete_component", field_label: "Delete component", field_path: "components", before: await canonicalJsonString(model.components[0]), after: "deleted", unit: "none", dimension: "dimensionless", source_note: "invented" },
    rationale: "invented W-10 delete", ...boundary } as unknown as Parameters<WorkspaceSessionApply>[0]);
  it("refuses a pipe a bend names, naming the bend; [delete_component, delete_pipe_run] applies once and one Undo restores both", async () => {
    const model = reconnectModel();
    model.components = [{ id: "component:bend-R", kind: "bend", node: "node:C", geometry: { bend_pipe_ref: "pipe:R" }, provenance: "invented" } as unknown as PreviewModel["components"][number]];
    vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(model);
    const { result } = renderHook(() => useWorkspaceSession());
    await waitFor(() => expect(result.current.model.model).not.toBeNull());
    const basis = structuredClone(result.current.model.model!);
    let applied = true;
    await act(async () => { applied = await result.current.operations.handleApplyIntent(deletePipe(basis)); });
    expect(applied).toBe(false);
    expect(result.current.model.model).toEqual(basis);
    const refusal = Object.values(result.current.operations.operationOutcomes)[0].diagnostics.find((d) => d.code === "OP-PIPE-DELETE-REFERENCED")!;
    expect(refusal.message).toBe("Pipe segment pipe:R is still named by: component:bend-R (geometry.bend_pipe_ref).");
    expect(refusal.affected_refs).toEqual(["pipe:R", "component:bend-R"]);
    await act(async () => { await result.current.operations.handleQueueOperationBatch({ batch_id: "batch:delete-both", operations: [await deleteComponent(basis), deletePipe(basis)] }); });
    await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], true); });
    expect(result.current.model.model!.components).toEqual([]);
    expect(result.current.model.model!.pipe_segments.map((p) => p.id)).toEqual(["pipe:P", "pipe:Q"]);
    expect(result.current.operations.undoStack).toHaveLength(1);
    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect(result.current.model.model).toEqual(basis);
  });
});
