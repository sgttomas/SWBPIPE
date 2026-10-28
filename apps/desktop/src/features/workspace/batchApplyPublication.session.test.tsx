/** Batch apply is ordered Prepare -> Publish -> Notify. A failure before or
 * inside publication leaves no undo checkpoint, redo change, receipt, review
 * context or applied-operation record, and keeps the batch registered and
 * queued; a failure of an external callback after publication is reported and
 * does not roll the published batch back. The session runs the actual wasm
 * engine on an invented inline model. */
import { act, render, renderHook, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import * as previewService from "../../services/previewService";
import { BatchReviewPanel } from "../toolkit/BatchReviewPanel";
import { buildReconnectBatch, emptyGeometryDraft } from "../geometry-tools/geometryDraft";
import * as sessionModel from "./sessionModel";
import { buildGridOperationIntent, type GridColumn } from "./table/modelTableAdapter";
import { useWorkspaceSession } from "./workspaceSession";
import type { PreviewModel } from "../../types";
import { reconnectModel } from "../../test-support/reconnectModel";

const APPLIED = "Batch applied once. Previous solve results were cleared; one undo checkpoint is available.";

function coordinateIntent(model: PreviewModel, value: string, sequence: number) {
  const target = model.nodes[2];
  const column: GridColumn = { key: "y", label: "Y", fieldPath: "position.y", objectType: "Node", changeKind: "set_field", dimension: "length", sourceNote: "entered coordinate", unit: () => model.project.units.length, value: () => String(target.position.y) };
  return buildGridOperationIntent({ column, model, row: { id: target.id, label: target.label, type: "node", raw: target, searchText: "" }, sequence, value, interaction: "cell" });
}

async function session() {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(reconnectModel());
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return hook.result;
}

type Session = Awaited<ReturnType<typeof session>>;

async function queueCoordinate(result: Session, batchId: string, value: string, sequence: number) {
  const model = structuredClone(result.current.model.model!);
  await act(async () => { await result.current.operations.handleQueueOperationBatch({ batch_id: batchId, operations: [coordinateIntent(model, value, sequence)] }); });
  return result.current.operations.queuedBatches.find((entry) => entry.batch.batch_id === batchId)!;
}

type Queued = Session["current"]["operations"]["queuedBatches"][number];

async function apply(result: Session, entry: Queued) {
  await act(async () => { await result.current.operations.handleRunOperationBatch(entry, true); });
}

async function validate(result: Session, entry: Queued) {
  await act(async () => { await result.current.operations.handleRunOperationBatch(entry, false); });
  const outcome = result.current.operations.batchOutcomes[entry.key];
  expect(outcome.mode).toBe("validate_only");
  expect(outcome.validation.batch_validation_status).toBe("passed");
  return structuredClone(outcome);
}

const queued = (result: Session, key: string) => result.current.operations.queuedBatches.find((entry) => entry.key === key)!;

/** The identity hash of a model commit fails only for the model `applied` recognises. */
function failIdentityHashFor(applied: (model: PreviewModel) => boolean) {
  const identityHash = sessionModel.uiModelIdentityHash;
  return vi.spyOn(sessionModel, "uiModelIdentityHash").mockImplementation((model) => {
    if (applied(model)) throw new Error("injected identity-hash failure");
    return identityHash(model);
  });
}

/** The batch review panel as App renders it from the session. */
function renderBatchReview(result: Session) {
  const operations = result.current.operations;
  return render(<BatchReviewPanel batches={operations.queuedBatches} outcomes={operations.batchOutcomes} receipts={operations.batchReceipts}
    revision={result.current.model.modelRevision.current} busy={operations.operationBusy} message={operations.batchMessage}
    onClear={vi.fn()} onValidate={vi.fn()} onApply={vi.fn()} />);
}

/** Applies one batch, then undoes it, so a redo entry exists for the next apply to clear. */
async function withRedoEntry(result: Session) {
  await apply(result, await queueCoordinate(result, "batch:first", "1.5", 1));
  expect(result.current.model.model!.nodes[2].position.y).toBe(1.5);
  act(() => result.current.operations.handleUndoSessionModelEdit());
  expect(result.current.operations.undoStack).toHaveLength(0);
  expect(result.current.operations.redoStack).toHaveLength(1);
}

function historySnapshot(result: Session) {
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

afterEach(() => { vi.restoreAllMocks(); });

describe("batch apply: Prepare -> Publish -> Notify", () => {
  it("a failure before publication changes no history, receipt, review context or applied operation, and the batch can be applied again", async () => {
    const result = await session();
    await withRedoEntry(result);
    const entry = await queueCoordinate(result, "batch:second", "2.5", 2);
    const validated = await validate(result, entry);
    const before = historySnapshot(result);
    // The UI identity hash of the applied model is part of the model commit's
    // own fallible work; it fails only for the model this batch returns.
    const failure = failIdentityHashFor((model) => model.nodes[2]?.position.y === 2.5);

    await apply(result, queued(result, entry.key));

    expect(historySnapshot(result)).toEqual(before);
    expect(result.current.operations.batchMessage).toBe("Batch request failed: Error: injected identity-hash failure");
    expect(result.current.operations.queuedBatches.map((candidate) => candidate.key)).toContain(entry.key);
    // The earlier preview stands; no applied outcome was published.
    expect(result.current.operations.batchOutcomes[entry.key]).toEqual(validated);

    failure.mockRestore();
    await apply(result, queued(result, entry.key));
    expect(result.current.model.model!.nodes[2].position.y).toBe(2.5);
    expect(result.current.operations.undoStack).toHaveLength(1);
    expect(result.current.operations.redoStack).toHaveLength(0);
    expect(result.current.operations.batchMessage).toBe(APPLIED);
    expect(result.current.operations.queuedBatches.map((queued) => queued.key)).not.toContain(entry.key);
  });

  it("after a failed preparation a reconnect batch keeps its preview, Apply stays enabled and a retry applies", async () => {
    const result = await session();
    const basis = structuredClone(result.current.model.model!);
    const draft = { ...emptyGeometryDraft(), action: "reconnect" as const, pipeRefs: ["pipe:P"], reconnectEnd: "to" as const, targetNode: "node:C", yReference: { x: "0", y: "0", z: "1" } };
    await act(async () => { await result.current.operations.handleQueueOperationBatch(await buildReconnectBatch(basis, draft, "batch:reconnect")); });
    const entry = result.current.operations.queuedBatches[0];
    expect(entry.requiresPreview).toBe(true);
    const validated = await validate(result, entry);
    const failure = failIdentityHashFor((model) => model.pipe_segments.find((pipe) => pipe.id === "pipe:P")?.to === "node:C");

    await apply(result, queued(result, entry.key));

    expect(result.current.model.model).toEqual(basis);
    expect(result.current.operations.undoStack).toHaveLength(0);
    expect(result.current.operations.batchReceipts).toHaveLength(0);
    expect(result.current.operations.batchMessage).toBe("Batch request failed: Error: injected identity-hash failure");
    expect(result.current.operations.batchOutcomes[entry.key]).toEqual(validated);
    const panel = renderBatchReview(result);
    expect((screen.getByTestId(`apply-batch-${entry.key}`) as HTMLButtonElement).disabled).toBe(false);
    expect(screen.getByTestId(`batch-${entry.key}`).textContent).not.toContain("Applied as one batch.");
    panel.unmount();

    failure.mockRestore();
    await apply(result, queued(result, entry.key));
    expect(result.current.model.model!.pipe_segments.find((pipe) => pipe.id === "pipe:P")!.to).toBe("node:C");
    expect(result.current.operations.undoStack).toHaveLength(1);
    expect(result.current.operations.batchOutcomes[entry.key].validation.application_status).toBe("applied_to_session_model");
    expect(result.current.operations.batchMessage).toBe(APPLIED);
  });

  it("a viewport callback failure after publication is reported; the model and history advance and are not rolled back", async () => {
    const result = await session();
    await withRedoEntry(result);
    const entry = await queueCoordinate(result, "batch:second", "2.5", 2);
    const before = historySnapshot(result);
    const commands: string[] = [];
    result.current.chrome.viewportViewCommandRef.current = (command) => {
      commands.push(command.type);
      if (command.type === "retire-box-gesture") throw new Error("injected viewport failure");
    };

    await apply(result, entry);

    const operations = result.current.operations;
    expect(commands).toContain("retire-box-gesture");
    expect(result.current.model.model!.nodes[2].position.y).toBe(2.5);
    expect(operations.undoStack).toHaveLength(1);
    expect(operations.undoStack[0].checkpoint_id).toBe(`undo-${entry.key}`);
    expect(operations.undoStack[0].model).toEqual(before.model);
    expect(operations.redoStack).toHaveLength(0);
    expect(operations.batchReceipts).toHaveLength(before.batchReceipts.length + 1);
    expect(operations.appliedOperations).toHaveLength(before.appliedOperations.length + 1);
    expect(operations.appliedOperations.at(-1)!.sequence).toBe(before.appliedOperations.length + 1);
    expect(operations.retainedReviewContext).toHaveLength(before.retainedReviewContext.length + 1);
    expect(operations.queuedBatches.map((queued) => queued.key)).not.toContain(entry.key);
    expect(operations.batchMessage).toBe(`${APPLIED} The viewport did not retire its box gesture: Error: injected viewport failure`);
  });

  it("a normal apply adds one undo checkpoint, clears redo and reports the applied batch", async () => {
    const result = await session();
    await withRedoEntry(result);
    const entry = await queueCoordinate(result, "batch:second", "2.5", 2);
    const before = historySnapshot(result);

    await apply(result, entry);

    const operations = result.current.operations;
    expect(result.current.model.model!.nodes[2].position.y).toBe(2.5);
    expect(operations.undoStack).toHaveLength(1);
    expect(operations.undoStack[0]).toMatchObject({ checkpoint_id: `undo-${entry.key}`, operation_id: "batch:second", model: before.model });
    expect(operations.redoStack).toHaveLength(0);
    expect(operations.batchReceipts.at(-1)!.batch.batch_id).toBe("batch:second");
    expect(operations.appliedOperations.at(-1)).toMatchObject({ receipt_id: `applied-${entry.key}`, sequence: before.appliedOperations.length + 1, operation_id: "batch:second" });
    expect(operations.queuedBatches.map((queued) => queued.key)).not.toContain(entry.key);
    expect(operations.batchMessage).toBe(APPLIED);
  });
});
