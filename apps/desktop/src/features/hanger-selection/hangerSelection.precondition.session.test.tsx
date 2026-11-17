// DEL-04-11 REQ-008: a sized run's batch is applied only while the stored hanger library is the one its result binds,
// checked again at the final Apply. The session is the desktop's own (`useWorkspaceSession`) with the wasm operation
// engine, loaded with the four-span input's model and the session's `diagnostics: []`. The run is the product's
// four-span result (CE-2 on support:B by fallback, CE-1 on support:C, support:D unfitted) with its solve-request binding
// replaced by the checked hash of the session's model, which the headless input's own binding cannot match because of
// that member. The stored library is a test reader over a value the test replaces.
import { act, renderHook, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import fourSpanInput from "../../../../../fixtures/results/hanger_sizing_input_four_span.json";
import fourSpanResult from "../../../../../fixtures/results/hanger_sizing_result_four_span.json";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import * as operationBatchService from "../../services/operationBatchService";
import * as previewService from "../../services/previewService";
import type { PreviewModel } from "../../types";
import { prepareSizedApplication, type StoredHangerLibraryReader } from "../hanger-sizing/sizedApplication";
import { HANGER_SIZING_LIBRARY_CHANGED_TEXT, HANGER_SIZING_LIBRARY_UNAVAILABLE_TEXT } from "../hanger-sizing/sizedSelection";
import type { HangerSizingSizedDocument } from "../results/hangerSizingResult";
import { useWorkspaceSession } from "../workspace/workspaceSession";

const sessionModel = () => ({ ...structuredClone(fourSpanInput.request.model), diagnostics: [] }) as unknown as PreviewModel;
const library = () => structuredClone(fourSpanInput.library) as Record<string, unknown>;
const revised = () => {
  const changed = library();
  (changed.hanger_library as Record<string, unknown>).name = "Invented hanger sizing fixture, revised";
  return changed;
};

async function session() {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(sessionModel());
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return hook.result;
}
type Session = Awaited<ReturnType<typeof session>>;

async function boundRun(model: PreviewModel): Promise<HangerSizingSizedDocument> {
  const document = structuredClone(fourSpanResult) as unknown as HangerSizingSizedDocument & { inputs: { solve_request: { sha256: string } } };
  document.inputs.solve_request.sha256 = await canonicalSha256HexCheckedV1({ model, materials: [] });
  return document;
}

function history(result: Session) {
  const operations = result.current.operations;
  return structuredClone({ model: result.current.model.model, undoStack: operations.undoStack, batchReceipts: operations.batchReceipts, appliedOperations: operations.appliedOperations });
}

afterEach(() => { vi.restoreAllMocks(); });

describe("a queued sized run and its stored library", () => {
  it("applies nothing when the library is replaced after queueing, keeps the batch queued with its reason, and applies once it is restored", async () => {
    const result = await session();
    const stored: { document: Record<string, unknown> | null } = { document: library() };
    const read: StoredHangerLibraryReader = async () => (stored.document === null ? { kind: "unavailable" } : { kind: "read", document: structuredClone(stored.document) });
    const model = result.current.model.model!;
    const prepared = await prepareSizedApplication(await boundRun(model), model, model.project.id, read);
    if (prepared.kind !== "prepared") throw new Error(prepared.reason);
    await act(async () => { await result.current.operations.handleQueueOperationBatch(prepared.batch, { precondition: prepared.precondition }); });
    const [entry] = result.current.operations.queuedBatches;
    expect(entry.batch.batch_id).toBe(prepared.batch.batch_id);
    const before = history(result);
    const applySpy = vi.spyOn(operationBatchService, "applyOperationBatch");

    for (const [replacement, reason] of [[revised(), HANGER_SIZING_LIBRARY_CHANGED_TEXT], [null, HANGER_SIZING_LIBRARY_UNAVAILABLE_TEXT]] as const) {
      stored.document = replacement;
      await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], true); });
      expect(applySpy).not.toHaveBeenCalled();
      expect(history(result)).toEqual(before);
      expect(result.current.operations.batchMessage).toBe(`Batch ${entry.batch.batch_id} was not applied. ${reason}`);
      expect(result.current.operations.queuedBatches.map((queued) => [queued.key, queued.lastApplyRefusal])).toEqual([[entry.key, { kind: "precondition", reason }]]);
    }

    stored.document = library();
    await act(async () => { await result.current.operations.handleRunOperationBatch(result.current.operations.queuedBatches[0], true); });
    expect(applySpy).toHaveBeenCalledTimes(1);
    expect(result.current.operations.queuedBatches).toHaveLength(0);
    expect(result.current.operations.undoStack).toHaveLength(1);
    const applied = result.current.model.model!;
    expect(applied.supports.find((support) => support.id === "support:B")!.family).toBe("constant_effort_support");
    expect(applied.supports.find((support) => support.id === "support:C")!.family).toBe("constant_effort_support");
  });
});
