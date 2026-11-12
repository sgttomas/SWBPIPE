// DEL-04-11 REQ-008: the session's registration gate. A batch that carries a sized hanger selection is registered only
// with the precondition its sizing result supplies, whatever its producer; a hand-picked hanger selection is unaffected.
// The session is the desktop's own, loaded with the invented two-span sizing input's model (with the session's
// `diagnostics: []`); the sized run identity and loads are invented, as in `hangerSelection.sized.test.ts`.
import { act, renderHook, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import twoSpanInput from "../../../../../fixtures/results/hanger_sizing_input_two_span.json";
import * as previewService from "../../services/previewService";
import type { OperationBatch } from "../../services/operationBatchService";
import type { PreviewModel } from "../../types";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import {
  buildHangerSelectionBatch,
  carriesSizedHangerSelection,
  SIZED_HANGER_SELECTION_FIELD_LABEL,
  SIZED_HANGER_SELECTION_UNREGISTERED_TEXT,
  type HangerDocument,
  type HangerSizingApplication
} from "./hangerSelection";

const model = () => ({ ...structuredClone(twoSpanInput.request.model), diagnostics: [] }) as unknown as PreviewModel;
const library = () => structuredClone(twoSpanInput.library) as unknown as HangerDocument;
const TARGET = { supportId: "support:B", restraints: ["UZ"] };
const SIZED: HangerSizingApplication = {
  run: { sizingRequestSha256: "1".repeat(64), solveRequestSha256: "2".repeat(64), libraryId: "invented.hangers.sizing", librarySha256: "3".repeat(64), hangerId: "VS-200-L", recordSha256: "4".repeat(64) },
  loads: { family: "variable_spring", coldLoadN: 5197.5, hotLoadN: 5000, designOperatingCase: "load:O1" }
};

async function session() {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(model());
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return hook.result;
}
type Session = Awaited<ReturnType<typeof session>>;

async function register(result: Session, batch: OperationBatch, options: Parameters<Session["current"]["operations"]["registerOperationBatch"]>[1] = {}) {
  let key: string | null = null;
  await act(async () => { key = await result.current.operations.registerOperationBatch(batch, options); });
  return key as string | null;
}

async function requeue(result: Session, intent: unknown) {
  let refused: string | null = null;
  await act(async () => { refused = await result.current.operations.handleRequeueRetainedRecord(structuredClone(intent), 0); });
  return refused as string | null;
}

afterEach(() => { vi.restoreAllMocks(); });

describe("the sized hanger selection predicate", () => {
  it("is true for the builder's sized batch and false for a hand-picked one", async () => {
    const source = model();
    expect(carriesSizedHangerSelection(await buildHangerSelectionBatch(source, library(), "VS-200-L", TARGET, SIZED))).toBe(true);
    expect(carriesSizedHangerSelection(await buildHangerSelectionBatch(source, library(), "VS-200-L", TARGET))).toBe(false);
    expect(carriesSizedHangerSelection({ batch_id: "empty", operations: [] })).toBe(false);
  });

  it("does not catch a sized batch whose label was altered (the stated limit: it reads a label until a structured member exists)", async () => {
    const batch = await buildHangerSelectionBatch(model(), library(), "VS-200-L", TARGET, SIZED);
    expect(batch.operations[0].change.field_label).toBe(SIZED_HANGER_SELECTION_FIELD_LABEL);
    batch.operations[0].change.field_label = "Imported hanger selection";
    expect(carriesSizedHangerSelection(batch)).toBe(false);
  });
});

describe("the session's registration gate", () => {
  it("refuses a sized batch without a precondition by name, and admits it with one", async () => {
    const result = await session();
    const batch = await buildHangerSelectionBatch(result.current.model.model!, library(), "VS-200-L", TARGET, SIZED);
    expect(await register(result, structuredClone(batch))).toBeNull();
    expect(result.current.operations.batchMessage).toBe(SIZED_HANGER_SELECTION_UNREGISTERED_TEXT);
    expect(result.current.operations.queuedBatches).toHaveLength(0);
    const key = await register(result, structuredClone(batch), { precondition: async () => null });
    expect(key).not.toBeNull();
    expect(result.current.operations.queuedBatches.map((entry) => entry.key)).toEqual([key]);
  });

  it("refuses a sized batch from the queue's own producer, and from an agent's register, when no precondition comes", async () => {
    const result = await session();
    const batch = await buildHangerSelectionBatch(result.current.model.model!, library(), "VS-200-L", TARGET, SIZED);
    await act(async () => { await result.current.operations.handleQueueOperationBatch(structuredClone(batch)); });
    expect(result.current.operations.queuedBatches).toHaveLength(0);
    expect(result.current.operations.batchMessage).toBe(SIZED_HANGER_SELECTION_UNREGISTERED_TEXT);
    let key: string | null = "not run";
    await act(async () => {
      key = await result.current.operations.registerOperationBatch(structuredClone(batch), {
        agentContext: { requestRef: "r", threadId: "t", turnId: "u", callId: "c", basis: { workspaceHandle: "w", applicationGeneration: 0, modelRevision: 0, modelHash: null, stateId: null, runId: null }, assumptions: [] }
      });
    });
    expect(key).toBeNull();
    expect(result.current.operations.queuedBatches).toHaveLength(0);
  });

  it("refuses to requeue a stored sized-selection record, as requeue reports its refusals", async () => {
    const result = await session();
    const batch = await buildHangerSelectionBatch(result.current.model.model!, library(), "VS-200-L", TARGET, SIZED);
    expect(await requeue(result, batch.operations[0])).toBe(SIZED_HANGER_SELECTION_UNREGISTERED_TEXT);
    expect(result.current.operations.batchMessage).toBe(`Record could not be requeued: ${SIZED_HANGER_SELECTION_UNREGISTERED_TEXT}`);
    expect(result.current.operations.queuedBatches).toHaveLength(0);
  });

  it("admits and requeues a hand-picked hanger selection as today", async () => {
    const result = await session();
    const batch = await buildHangerSelectionBatch(result.current.model.model!, library(), "VS-200-L", TARGET);
    expect(await register(result, structuredClone(batch))).not.toBeNull();
    expect(await requeue(result, batch.operations[0])).toBeNull();
    expect(result.current.operations.queuedBatches).toHaveLength(2);
    expect(result.current.operations.queuedBatches[1].batch.batch_id).toBe("requeue-0");
  });
});
