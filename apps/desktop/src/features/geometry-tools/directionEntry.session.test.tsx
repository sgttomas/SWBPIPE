/** B5 (DEL-07-01 Design `split-and-slope-entry.md` §3): the computed end point through the
 * session and the actual wasm engine, on both routes that take it: a new route end (create_node +
 * connect_pipe_run, the viewport builder) and a reconnect to a new point (create_node +
 * reconnect_pipe_run, the Geometry builder). The stored coordinates are bit-identical to the
 * computed ones (Object.is per component), including a length typed in another unit and a −0. */
import { act, renderHook, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import * as previewService from "../../services/previewService";
import { computeModelHash } from "../../services/hashService";
import type { OperationBatch } from "../../services/operationBatchService";
import { splitPartsModel } from "../../test-support/splitPartsModel";
import type { PreviewModel } from "../../types";
import { buildRouteSubmission } from "../viewport/routeDraft";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { computeDirectionEnd, emptyDirectionDraft, exactCoordinateText, type DirectionDraft, type Vec3 } from "./directionArithmetic";
import { buildReconnectBatch, emptyGeometryDraft } from "./geometryDraft";

async function session(model: PreviewModel) {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(model);
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return hook.result;
}
afterEach(() => { vi.restoreAllMocks(); });

async function applyBatch(model: PreviewModel, batch: OperationBatch) {
  const result = await session(model);
  const originalHash = (await computeModelHash(result.current.model.model!))!.value;
  await act(async () => { await result.current.operations.handleQueueOperationBatch(batch); });
  await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], false); });
  const entry = result.current.operations.queuedBatches[0];
  const preview = result.current.operations.batchOutcomes[entry.key];
  expect(preview.operation_outcomes.flatMap((step) => step.diagnostics.filter((item) => item.severity === "blocking"))).toEqual([]);
  await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], true); });
  expect(result.current.operations.batchMessage).toBe("Batch applied once. Previous solve results were cleared; one undo checkpoint is available.");
  return { result, originalHash };
}

const text = (p: Vec3) => ({ x: exactCoordinateText(p.x), y: exactCoordinateText(p.y), z: exactCoordinateText(p.z) });
const expectBits = (stored: Vec3, computed: Vec3) => {
  for (const axis of ["x", "y", "z"] as const) expect(Object.is(stored[axis], computed[axis]), `${axis}: ${stored[axis]} vs ${computed[axis]}`).toBe(true);
};

const slope: DirectionDraft = { ...emptyDirectionDraft("in"), mode: "slope", length: "137.3", direction: { x: "3", y: "-1", z: "0" }, up: "+z", slopeN: "96", slopeSense: "falls" };
const angle: DirectionDraft = { ...emptyDirectionDraft("m"), mode: "angle", length: "2.71", direction: { x: "0", y: "1", z: "0" }, up: "+z", angle: "7.5", angleUnit: "deg" };

describe("computed end points through the wasm engine", () => {
  it.each([["a slope typed in inches (project ft)", slope], ["an angle with the length in metres (project ft)", angle]])("route end: %s is stored bit-identical, and one Undo reverts it", async (_name, entry) => {
    const model = splitPartsModel();
    const start = model.nodes.find((n) => n.id === "node:E")!;
    const computed = (await computeDirectionEnd(entry, start.position, model.project.units.length)).position;
    const shown = text(computed);
    const built = buildRouteSubmission(model, [], {
      startNodeId: start.id, endMode: "new", existingEndNodeId: "",
      newEnd: { id: "node:N1", label: "N1", ...shown, coordinateUnit: "ft", provenance: "invented" },
      pipe: { id: "pipe:N1", label: "N1", materialId: "material:steel", outsideDiameter: "4.5", wallThickness: "0.237", lengthUnit: "in", yReferenceX: "0", yReferenceY: "0", yReferenceZ: "1", provenance: "invented" }
    }, 1, "not_run");
    expect(built.ok).toBe(true);
    if (!built.ok || built.submission.kind !== "batch") return;
    // Emitted = displayed: the create_node payload carries the shown text's numbers, in ft.
    const createNode = built.submission.batch.operations[0];
    expect(createNode.change.unit).toBe("ft");
    expectBits(JSON.parse(createNode.change.after).position, computed);
    const { result, originalHash } = await applyBatch(model, built.submission.batch);
    expectBits(result.current.model.model!.nodes.find((n) => n.id === "node:N1")!.position, computed);
    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect((await computeModelHash(result.current.model.model!))!.value).toBe(originalHash);
  });

  it("reconnect to a new point from the fixed end, with a −0 start coordinate: +0 is stored, bit-identical", async () => {
    const model = splitPartsModel();
    const fixed = model.nodes.find((n) => n.id === "node:D")!;
    fixed.position = { x: 20, y: -0, z: 9 };
    const entry: DirectionDraft = { ...emptyDirectionDraft("ft"), mode: "vector", length: "6.5", direction: { x: "1", y: "-0", z: "0" } };
    const result0 = await computeDirectionEnd(entry, fixed.position, "ft");
    // −0 + 6.5 · (−0 / 1) is −0 in binary64; the builder normalizes it to +0.
    expect(Object.is(fixed.position.y + result0.increment.y, -0)).toBe(true);
    const computed = result0.position;
    expect(Object.is(computed.y, 0)).toBe(true);
    expect(computed).toEqual({ x: 26.5, y: 0, z: 9 });
    const draft = { ...emptyGeometryDraft(), action: "reconnect" as const, pipeRefs: ["pipe:H"], reconnectEnd: "to" as const, toNewPoint: true,
      newNode: { id: "node:R1", label: "R1", provenance: "invented" }, newPoint: text(computed) };
    const batch = await buildReconnectBatch(model, draft, "reconnect-1");
    const { result } = await applyBatch(model, batch);
    const applied = result.current.model.model!;
    expectBits(applied.nodes.find((n) => n.id === "node:R1")!.position, computed);
    expect(applied.pipe_segments.find((p) => p.id === "pipe:H")!.to).toBe("node:R1");
  });
});
