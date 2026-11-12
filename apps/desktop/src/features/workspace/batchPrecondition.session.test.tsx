/** A queued batch's host precondition, asked at the final Apply (DEL-04-11 REQ-008; the queue shape agreed by the
 * lanes that share the review queue). The session runs the actual wasm operation engine on the invented W-10 inline
 * model; the batch is one entered node coordinate. A precondition is kept beside the registration in this session
 * only: it is asked only when the batch is applied, immediately before the host is asked; a refusal, a throw or no
 * answer within the bound applies nothing, leaves the batch queued and the entry's last-apply refusal shown; the
 * run's standing is read again after the precondition answers. */
import { act, render, renderHook, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import * as operationBatchService from "../../services/operationBatchService";
import * as previewService from "../../services/previewService";
import { canonicalJsonString } from "../../services/hashService";
import type { AgentContext } from "../../services/embeddedAgentSession";
import { reconnectModel } from "../../test-support/reconnectModel";
import type { PreviewModel } from "../../types";
import { BatchReviewPanel } from "../toolkit/BatchReviewPanel";
import type { SessionEvent } from "./sessionEvents";
import { buildGridOperationIntent, type GridColumn } from "./table/modelTableAdapter";
import { PRECONDITION_TIMEOUT_MS, PRECONDITION_UNCHECKED_REASON, useWorkspaceSession, type BatchPrecondition } from "./workspaceSession";

const REFUSAL = "The condition this batch was prepared under no longer holds.";

function coordinateIntent(model: PreviewModel, value: string, sequence: number) {
  const target = model.nodes[2];
  const column: GridColumn = { key: "y", label: "Y", fieldPath: "position.y", objectType: "Node", changeKind: "set_field", dimension: "length", sourceNote: "entered coordinate", unit: () => model.project.units.length, value: () => String(target.position.y) };
  return buildGridOperationIntent({ column, model, row: { id: target.id, label: target.label, type: "node", raw: target, searchText: "" }, sequence, value, interaction: "cell" });
}

function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((yes) => { resolve = yes; });
  return { promise, resolve };
}

async function session() {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(reconnectModel());
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return hook.result;
}

type Session = Awaited<ReturnType<typeof session>>;
type Options = Parameters<Session["current"]["operations"]["registerOperationBatch"]>[1];

const batchOf = (result: Session, batchId: string, value: string, sequence: number) =>
  ({ batch_id: batchId, operations: [coordinateIntent(structuredClone(result.current.model.model!), value, sequence)] });

async function queue(result: Session, batchId: string, value: string, sequence: number, options: Options = {}) {
  let key: string | null = null;
  await act(async () => { key = await result.current.operations.registerOperationBatch(batchOf(result, batchId, value, sequence), options); });
  expect(key).not.toBeNull();
  return entryOf(result, key!);
}

const entryOf = (result: Session, key: string) => result.current.operations.queuedBatches.find((entry) => entry.key === key)!;
type Queued = ReturnType<typeof entryOf>;

async function apply(result: Session, entry: Queued) {
  await act(async () => { await result.current.operations.handleRunOperationBatch(entry, true); });
}

/** Everything a refused Apply must leave as it was. */
function history(result: Session) {
  const operations = result.current.operations;
  return structuredClone({
    model: result.current.model.model,
    undoStack: operations.undoStack,
    redoStack: operations.redoStack,
    batchReceipts: operations.batchReceipts,
    retainedReviewContext: operations.retainedReviewContext,
    appliedOperations: operations.appliedOperations
  });
}

function renderReview(result: Session) {
  const operations = result.current.operations;
  return render(<BatchReviewPanel batches={operations.queuedBatches} outcomes={operations.batchOutcomes} receipts={operations.batchReceipts}
    revision={result.current.model.modelRevision.current} busy={operations.operationBusy} message={operations.batchMessage}
    onClear={vi.fn()} onValidate={vi.fn()} onApply={vi.fn()} />);
}

afterEach(() => {
  vi.useRealTimers();
  vi.restoreAllMocks();
});

describe("a queued batch's precondition", () => {
  it("that answers null applies the batch as today, asked once at the Apply", async () => {
    const result = await session();
    const precondition = vi.fn<BatchPrecondition>(async () => null);
    const entry = await queue(result, "batch:pass", "1.5", 1, { precondition });
    await apply(result, entry);
    expect(precondition).toHaveBeenCalledTimes(1);
    expect(result.current.model.model!.nodes[2].position.y).toBe(1.5);
    expect(result.current.operations.undoStack).toHaveLength(1);
    expect(result.current.operations.queuedBatches).toHaveLength(0);
  });

  it("that refuses applies nothing, keeps the batch queued with its last-apply refusal shown, and applies once the condition holds", async () => {
    const result = await session();
    const holds = { now: false };
    const precondition = vi.fn<BatchPrecondition>(async () => (holds.now ? null : REFUSAL));
    const entry = await queue(result, "batch:refused", "2.5", 1, { precondition });
    const before = history(result);
    const applySpy = vi.spyOn(operationBatchService, "applyOperationBatch");

    await apply(result, entry);
    expect(history(result)).toEqual(before);
    expect(applySpy).not.toHaveBeenCalled();
    expect(result.current.operations.batchMessage).toBe(`Batch batch:refused was not applied. ${REFUSAL}`);
    expect(entryOf(result, entry.key).lastApplyRefusal).toEqual({ kind: "precondition", reason: REFUSAL });
    expect(result.current.operations.operationBusy).toBe(false);
    expect(result.current.operations.operationRequest.current.busy).toBe(false);
    const { unmount } = renderReview(result);
    expect(screen.getByTestId(`apply-refused-${entry.key}`)).toHaveTextContent(`Last apply refused: ${REFUSAL}`);
    // Apply stays offered while the condition fails, and refuses again.
    expect(screen.getByTestId(`apply-batch-${entry.key}`)).not.toBeDisabled();
    unmount();
    await apply(result, entryOf(result, entry.key));
    expect(precondition).toHaveBeenCalledTimes(2);
    expect(history(result)).toEqual(before);

    holds.now = true;
    await apply(result, entryOf(result, entry.key));
    expect(applySpy).toHaveBeenCalledTimes(1);
    expect(result.current.model.model!.nodes[2].position.y).toBe(2.5);
    expect(result.current.operations.queuedBatches).toHaveLength(0);
  });

  it("refusal is cleared when the entry is next run to an outcome", async () => {
    const result = await session();
    const entry = await queue(result, "batch:cleared", "3.5", 1, { precondition: async () => REFUSAL });
    await apply(result, entry);
    expect(entryOf(result, entry.key).lastApplyRefusal?.kind).toBe("precondition");
    await act(async () => { await result.current.operations.handleRunOperationBatch(entryOf(result, entry.key), false); });
    expect(result.current.operations.batchOutcomes[entry.key].mode).toBe("validate_only");
    expect(entryOf(result, entry.key).lastApplyRefusal).toBeUndefined();
  });

  it("that throws refuses with the fixed reason", async () => {
    const result = await session();
    const entry = await queue(result, "batch:throws", "4.5", 1, { precondition: async () => { throw new Error("library read failed"); } });
    const before = history(result);
    await apply(result, entry);
    expect(history(result)).toEqual(before);
    expect(result.current.operations.batchMessage).toBe(`Batch batch:throws was not applied. ${PRECONDITION_UNCHECKED_REASON}`);
    expect(entryOf(result, entry.key).lastApplyRefusal).toEqual({ kind: "precondition", reason: PRECONDITION_UNCHECKED_REASON });
  });

  it("that never settles refuses at the bound, and the route is free afterwards", async () => {
    const result = await session();
    const called = deferred<void>();
    const entry = await queue(result, "batch:silent", "5.5", 1, { precondition: () => { called.resolve(); return new Promise<string | null>(() => {}); } });
    const before = history(result);
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });
    let running!: Promise<void>;
    await act(async () => {
      running = result.current.operations.handleRunOperationBatch(entry, true);
      await called.promise;
    });
    expect(result.current.operations.operationRequest.current.busy).toBe(true);
    await act(async () => {
      await vi.advanceTimersByTimeAsync(PRECONDITION_TIMEOUT_MS);
      await running;
    });
    vi.useRealTimers();
    expect(history(result)).toEqual(before);
    expect(result.current.operations.batchMessage).toBe(`Batch batch:silent was not applied. ${PRECONDITION_UNCHECKED_REASON}`);
    expect(result.current.operations.operationRequest.current.busy).toBe(false);
    // A second operation can run.
    const other = await queue(result, "batch:after", "6.5", 2);
    await apply(result, other);
    expect(result.current.model.model!.nodes[2].position.y).toBe(6.5);
  });

  it("does not start a second run when Apply is pressed again during the wait", async () => {
    const result = await session();
    const called = deferred<void>();
    const answer = deferred<string | null>();
    const precondition = vi.fn<BatchPrecondition>(() => { called.resolve(); return answer.promise; });
    const entry = await queue(result, "batch:twice", "7.5", 1, { precondition });
    const applySpy = vi.spyOn(operationBatchService, "applyOperationBatch");
    let running!: Promise<void>;
    await act(async () => {
      running = result.current.operations.handleRunOperationBatch(entry, true);
      await called.promise;
    });
    await act(async () => { await result.current.operations.handleRunOperationBatch(entryOf(result, entry.key), true); });
    expect(precondition).toHaveBeenCalledTimes(1);
    await act(async () => { answer.resolve(null); await running; });
    expect(applySpy).toHaveBeenCalledTimes(1);
    expect(result.current.operations.undoStack).toHaveLength(1);
  });

  it("is not asked by validation or preview", async () => {
    const result = await session();
    const precondition = vi.fn<BatchPrecondition>(async () => REFUSAL);
    const entry = await queue(result, "batch:validate", "8.5", 1, { precondition });
    await act(async () => { await result.current.operations.handleRunOperationBatch(entry, false); });
    expect(result.current.operations.batchOutcomes[entry.key].mode).toBe("validate_only");
    expect(precondition).not.toHaveBeenCalled();
  });
});

/** A batch queued with a precondition held open by the test, and an Apply started and waiting on it. */
async function waitingApply(result: Session, batchId: string, value: string, options: Options = {}) {
  const called = deferred<void>();
  const answer = deferred<string | null>();
  const entry = await queue(result, batchId, value, 1, { ...options, precondition: () => { called.resolve(); return answer.promise; } });
  const before = history(result);
  const applySpy = vi.spyOn(operationBatchService, "applyOperationBatch");
  let running!: Promise<void>;
  await act(async () => {
    running = result.current.operations.handleRunOperationBatch(entry, true);
    await called.promise;
  });
  expect(result.current.operations.operationRequest.current.busy).toBe(true);
  /** Answers null, as a precondition that holds does, and waits for the run to end. */
  const holds = async () => { await act(async () => { answer.resolve(null); await running; }); };
  return { entry, before, applySpy, holds };
}

describe("the gates after the precondition's wait (each with a held answer that then holds)", () => {
  it("the model's revision changed during the wait: nothing applied, ending as a superseded run does", async () => {
    const result = await session();
    const { applySpy, holds } = await waitingApply(result, "batch:revised", "9.5");
    const revision = result.current.model.modelRevision.current;
    // A model publication moves the revision (and, like every publication, supersedes the running request). With the
    // operation route busy, the only publications the session admits during the wait are project operations.
    await act(async () => { await result.current.project.handleCreateBlankProject(); });
    await waitFor(() => expect(result.current.model.modelRevision.current).not.toBe(revision));
    await holds();
    expect(applySpy).not.toHaveBeenCalled();
    expect(result.current.operations.batchReceipts).toHaveLength(0);
    expect(result.current.operations.batchMessage ?? "").not.toMatch(/^Batch applied once/);
  });

  it("the request superseded during the wait: nothing applied, the batch still queued", async () => {
    const result = await session();
    const { entry, before, applySpy, holds } = await waitingApply(result, "batch:superseded", "10.5");
    // A superseding request takes the next request sequence number, as every model publication and the queue's clear do.
    result.current.operations.operationRequest.current.sequence += 1;
    await holds();
    expect(applySpy).not.toHaveBeenCalled();
    expect(history(result)).toEqual(before);
    expect(result.current.operations.queuedBatches.map((queued) => queued.key)).toEqual([entry.key]);
  });

  it("the registration withdrawn during the wait: nothing applied (an agent owner's withdraw is refused while its batch applies)", async () => {
    // The agent owner's withdraw (workspaceSession's agent owner, `withdraw`) refuses a key whose batch is applying, so
    // it cannot withdraw during the wait; the withdrawal the session makes during a wait is the queue's clear, which
    // revokes every registration.
    const result = await session();
    const { before, applySpy, holds } = await waitingApply(result, "batch:withdrawn", "11.5");
    act(() => result.current.operations.handleClearReviewQueue());
    await holds();
    expect(applySpy).not.toHaveBeenCalled();
    expect(history(result)).toEqual(before);
    expect(result.current.operations.queuedBatches).toHaveLength(0);
  });

  it("the queue cleared or the entry retired during the wait: nothing applied", async () => {
    const result = await session();
    const { entry, applySpy, holds } = await waitingApply(result, "batch:retired", "12.5");
    // A new project retires every registration prepared on the earlier model and clears the queue.
    await act(async () => { await result.current.project.handleCreateBlankLoadStateProject(); });
    await waitFor(() => expect(result.current.model.model!.project.id).not.toBe(reconnectModel().project.id));
    await holds();
    expect(applySpy).not.toHaveBeenCalled();
    expect(result.current.operations.queuedBatches.some((queued) => queued.key === entry.key && !queued.retired)).toBe(false);
  });

  it("the project session replaced during the wait: nothing applied to either project", async () => {
    const result = await session();
    const { applySpy, holds } = await waitingApply(result, "batch:replaced", "13.5");
    await act(async () => { await result.current.project.handleCreateBlankProject(); });
    await waitFor(() => expect(result.current.model.model!.project.id).not.toBe(reconnectModel().project.id));
    const blank = structuredClone(result.current.model.model);
    await holds();
    expect(applySpy).not.toHaveBeenCalled();
    expect(result.current.model.model).toEqual(blank);
    expect(result.current.operations.undoStack).toHaveLength(0);
  });

  // No run is cancelled: the session has no cancel for a batch apply (cancellation exists for solves only).

  it("the late answer: a precondition that answers null after the bound has refused applies nothing, then or afterwards", async () => {
    const result = await session();
    const called = deferred<void>();
    const answer = deferred<string | null>();
    const entry = await queue(result, "batch:late", "14.5", 1, { precondition: () => { called.resolve(); return answer.promise; } });
    const before = history(result);
    const applySpy = vi.spyOn(operationBatchService, "applyOperationBatch");
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout"] });
    let running!: Promise<void>;
    await act(async () => {
      running = result.current.operations.handleRunOperationBatch(entry, true);
      await called.promise;
    });
    await act(async () => {
      await vi.advanceTimersByTimeAsync(PRECONDITION_TIMEOUT_MS + 1);
      await running;
    });
    vi.useRealTimers();
    expect(result.current.operations.batchMessage).toBe(`Batch batch:late was not applied. ${PRECONDITION_UNCHECKED_REASON}`);
    expect(entryOf(result, entry.key).lastApplyRefusal).toEqual({ kind: "precondition", reason: PRECONDITION_UNCHECKED_REASON });

    // The answer arrives late, saying the condition holds.
    await act(async () => {
      answer.resolve(null);
      for (let flush = 0; flush < 20; flush += 1) await Promise.resolve();
    });
    await new Promise((settle) => setTimeout(settle, 50));
    expect(applySpy).not.toHaveBeenCalled();
    expect(history(result)).toEqual(before);
    expect(result.current.operations.operationRequest.current.busy).toBe(false);
    expect(entryOf(result, entry.key).lastApplyRefusal?.reason).toBe(PRECONDITION_UNCHECKED_REASON);
  });
});

describe("what a precondition leaves out", () => {
  it("is in no batch document or receipt: both are byte-identical with and without one", async () => {
    const withIt = await session();
    const batch = batchOf(withIt, "batch:same", "11.5", 1);
    let key: string | null = null;
    await act(async () => { key = await withIt.current.operations.registerOperationBatch(structuredClone(batch), { precondition: async () => null }); });
    const queuedWith = await canonicalJsonString(entryOf(withIt, key!).batch);
    expect(Object.keys(entryOf(withIt, key!))).not.toContain("precondition");
    await apply(withIt, entryOf(withIt, key!));
    const receiptWith = withIt.current.operations.batchReceipts[0];
    vi.restoreAllMocks();

    const without = await session();
    await act(async () => { key = await without.current.operations.registerOperationBatch(structuredClone(batch)); });
    expect(await canonicalJsonString(entryOf(without, key!).batch)).toBe(queuedWith);
    await apply(without, entryOf(without, key!));
    const receiptWithout = without.current.operations.batchReceipts[0];
    expect(Object.keys(receiptWith).sort()).toEqual(Object.keys(receiptWithout).sort());
    expect(await canonicalJsonString(receiptWith.batch)).toBe(await canonicalJsonString(receiptWithout.batch));
    expect(JSON.stringify(receiptWith)).not.toContain("precondition");
  });

  it("refused leaves an agent proposal's standing pending review: no withdrawal, abandonment, invalidation or application", async () => {
    const result = await session();
    const events: SessionEvent[] = [];
    const stop = result.current.operations.sessionEvents.listen((event) => { events.push(event); });
    const context: AgentContext = {
      requestRef: "request:test", threadId: "thread:test", turnId: "turn:test", callId: "call:test",
      basis: { workspaceHandle: "workspace:test", applicationGeneration: 0, modelRevision: 0, modelHash: null, stateId: null, runId: null },
      assumptions: []
    };
    // No agent batch carries a precondition today; the session's register admits both together, as a test of the
    // standing path (the agent's own register member passes none).
    const entry = await queue(result, "batch:agent", "12.5", 1, { agentContext: context, precondition: async () => REFUSAL });
    events.length = 0;
    await apply(result, entry);
    stop();
    expect(entryOf(result, entry.key).lastApplyRefusal?.kind).toBe("precondition");
    // The standing moves only on these events (to withdrawn or unknown) or on publication (to applied); the hook does
    // not expose the standing itself, so its movers are what is observed.
    for (const type of ["withdrawn", "abandoned", "invalidated"] as const) expect(events.some((event) => event.type === type)).toBe(false);
    expect(events.some((event) => event.type === "checkpoint")).toBe(false);
    expect(result.current.operations.batchReceipts).toHaveLength(0);
  });
});
