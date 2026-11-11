/** B2 (DEL-07-02 Design `bulk-change.md`): every bulk change, built with the engine's own
 * canonical JSON, queues as one batch through the session's existing registration, validates
 * with no blocking finding, applies once, and one Undo restores the original model. The session
 * runs the actual wasm engine on an invented inline model. */
import { act, renderHook, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import * as previewService from "../../services/previewService";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { bulkChangeModel } from "../../test-support/bulkChangeModel";
import type { EntityRef, PreviewModel } from "../../types";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { buildBulkChangeBatch, type BulkChangeRequest } from "./bulkChange";

async function session() {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(bulkChangeModel());
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return hook.result;
}
afterEach(() => { vi.restoreAllMocks(); });

const cases: Array<[string, EntityRef[], BulkChangeRequest, (model: PreviewModel) => unknown, unknown]> = [
  ["section", [{ type: "pipe", id: "pipe:P1" }, { type: "pipe", id: "pipe:P2" }], { kind: "section", sectionId: "section:S1" },
    (model) => model.pipe_segments.map((pipe) => [pipe.section_ref ?? null, pipe.section.outside_diameter.value]), [["section:S1", 0.1683], ["section:S1", 0.1683], [null, 0.1]]],
  ["material", [{ type: "pipe", id: "pipe:P2" }, { type: "pipe", id: "pipe:P3" }], { kind: "material", materialId: "material:alloy" },
    (model) => model.pipe_segments.map((pipe) => pipe.material), ["material:steel", "material:alloy", "material:alloy"]],
  ["support kind", [{ type: "pipe", id: "pipe:P1" }, { type: "pipe", id: "pipe:P2" }], { kind: "support-kind", fromFamily: "guide", toFamily: "line_stop", restraints: ["UX"] },
    (model) => model.supports.map((support) => [support.id, support.family, support.restraints]),
    [["support:A", "anchor", ["UX", "UY", "UZ", "RX", "RY", "RZ"]], ["support:B", "line_stop", ["UX"]], ["support:C", "line_stop", ["UX"]], ["support:D", "spring", []]]],
  ["support removal", [{ type: "node", id: "node:B" }, { type: "node", id: "node:C" }], { kind: "support-remove", family: "guide" },
    (model) => model.supports.map((support) => support.id), ["support:A", "support:D"]],
  ["bend radius", [{ type: "node", id: "node:B" }], { kind: "bend-radius", value: "0.229", unit: "m" },
    (model) => model.components.map((component) => component.geometry?.bend_radius?.value ?? null), [0.229, 2.5, null]],
  ["tag", [{ type: "pipe", id: "pipe:P1" }, { type: "support", id: "support:B" }, { type: "component", id: "component:VALVE-C" }], { kind: "tag", tag: "what-if" },
    (model) => [model.pipe_segments[0].tags, model.supports[1].tags, model.components[2].tags], [["rack-1", "what-if"], ["what-if"], ["what-if"]]]
];

describe("bulk change through the session (queue, review, apply, Undo)", () => {
  it.each(cases)("%s: one batch validates, applies once and is undone at once", async (_name, scope, request, read, expected) => {
    const result = await session();
    const original = structuredClone(result.current.model.model!);
    const originalHash = (await computeModelHash(original))!.value;
    const batch = await buildBulkChangeBatch(original, scope, request, canonicalJsonString, `bulk-${request.kind}-1`);
    await act(async () => { await result.current.operations.handleQueueOperationBatch(batch); });
    expect(result.current.operations.queuedBatches).toHaveLength(1);
    const entry = result.current.operations.queuedBatches[0];
    expect(entry.batch.operations).toHaveLength(batch.operations.length);

    await act(async () => { await result.current.operations.handleRunOperationBatch(entry, false); });
    const preview = result.current.operations.batchOutcomes[entry.key];
    expect(preview.diagnostics.filter((item) => item.severity === "blocking")).toEqual([]);
    expect(preview.operation_outcomes.flatMap((step) => step.diagnostics.filter((item) => item.severity === "blocking"))).toEqual([]);
    expect(preview.validation.batch_validation_status).toBe("passed");
    expect(preview.operation_outcomes).toHaveLength(batch.operations.length);
    expect(result.current.model.model).toEqual(original);

    await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], true); });
    expect(result.current.operations.batchMessage).toBe("Batch applied once. Previous solve results were cleared; one undo checkpoint is available.");
    expect(read(result.current.model.model!)).toEqual(expected);
    expect(result.current.operations.undoStack).toHaveLength(1);

    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect((await computeModelHash(result.current.model.model!))!.value).toBe(originalHash);
    expect(read(result.current.model.model!)).toEqual(read(original));
  });
});
