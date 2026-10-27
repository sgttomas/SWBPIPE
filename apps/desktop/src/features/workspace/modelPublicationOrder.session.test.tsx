/** Single-intent apply, draft-review apply (batch and single) and undo/redo
 * are ordered Prepare -> Publish -> Notify, like batch apply
 * (`batchApplyPublication.session.test.tsx`). A failure before publication
 * leaves the model, both stacks, receipts, review context, applied operations,
 * the earlier outcome, the queued intent and the registered draft review as
 * they were, and the action can be retried; a failure of an external callback
 * after publication is reported and does not roll the publication back. Undo
 * and redo never throw. The session runs the actual wasm engine on an invented
 * inline model. */
import { act, renderHook, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import * as previewService from "../../services/previewService";
import { buildRouteSubmission, type DraftSubmission, type FrozenDraftReview, type RouteDraft } from "../viewport/routeDraft";
import * as selectionState from "./selectionState";
import * as sessionModel from "./sessionModel";
import { buildGridOperationIntent, type GridColumn } from "./table/modelTableAdapter";
import { useWorkspaceSession } from "./workspaceSession";
import type { OperationOutcome, PreviewModel } from "../../types";
import { reconnectModel } from "../../test-support/reconnectModel";

const VIEWPORT_FAILURE = "The viewport did not retire its box gesture: Error: injected viewport failure";
const PREPARE_FAILURE = "Error: injected identity-hash failure";

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

const nodeCY = (result: Session) => result.current.model.model!.nodes[2].position.y;

// Captured before any spy, so a spy never wraps another.
const identityHash = sessionModel.uiModelIdentityHash;

/** The identity hash of a model commit fails only for the model `applied` recognises. */
function failIdentityHashFor(applied: (model: PreviewModel) => boolean) {
  return vi.spyOn(sessionModel, "uiModelIdentityHash").mockImplementation((model) => {
    if (applied(model)) throw new Error("injected identity-hash failure");
    return identityHash(model);
  });
}

/** The identity hash fails for the next model commit only. */
function failNextIdentityHash() {
  let failed = false;
  return failIdentityHashFor(() => !failed && (failed = true));
}

function failViewportRetire(result: Session) {
  const commands: string[] = [];
  result.current.chrome.viewportViewCommandRef.current = (command) => {
    commands.push(command.type);
    if (command.type === "retire-box-gesture") throw new Error("injected viewport failure");
  };
  return commands;
}

function snapshot(result: Session) {
  const operations = result.current.operations;
  return structuredClone({
    model: result.current.model.model,
    undoStack: operations.undoStack,
    redoStack: operations.redoStack,
    batchReceipts: operations.batchReceipts,
    retainedReviewContext: operations.retainedReviewContext,
    appliedOperations: operations.appliedOperations,
    operationOutcomes: operations.operationOutcomes,
    editorIntents: operations.editorIntents
  });
}

/** Applies one coordinate edit (y = 1.5), then undoes it, so a redo entry exists. */
async function withRedoEntry(result: Session) {
  await act(async () => { await result.current.operations.handleApplyIntent(coordinateIntent(result.current.model.model!, "1.5", 1)); });
  expect(nodeCY(result)).toBe(1.5);
  act(() => result.current.operations.handleUndoSessionModelEdit());
  expect(nodeCY(result)).toBe(2);
  expect(result.current.operations.undoStack).toHaveLength(0);
  expect(result.current.operations.redoStack).toHaveLength(1);
}

afterEach(() => { vi.restoreAllMocks(); });

// ---- single-intent apply ---------------------------------------------------

/** Queues a coordinate edit (y = 2.5) and validates it, so a queue entry and an earlier outcome exist. */
async function queuedValidatedIntent(result: Session) {
  act(() => result.current.operations.handleQueueEditorIntent(coordinateIntent(result.current.model.model!, "2.5", 2)));
  const intent = result.current.operations.editorIntents[0];
  await act(async () => { await result.current.operations.handleValidateIntent(intent); });
  expect(result.current.operations.operationOutcomes[intent.queue_id!].mode).toBe("validate_only");
  return intent;
}

async function applyIntent(result: Session, intent: Parameters<Session["current"]["operations"]["handleApplyIntent"]>[0], owned?: { current?: Readonly<OperationOutcome> }) {
  let applied = false;
  await act(async () => { applied = await result.current.operations.handleApplyIntent(intent, owned); });
  return applied;
}

const INTENT_APPLIED = (operationId: string) =>
  `Applied ${operationId} to the session model; previous solve results were cleared. Run a new solve, then save the project to store the edited model locally.`;

describe("single-intent apply: Prepare -> Publish -> Notify", () => {
  it("a failure before publication changes nothing, keeps the earlier outcome and the queued intent, and a retry applies", async () => {
    const result = await session();
    await withRedoEntry(result);
    const intent = await queuedValidatedIntent(result);
    const before = snapshot(result);
    const failure = failIdentityHashFor((model) => model.nodes[2]?.position.y === 2.5);
    const owned: { current?: Readonly<OperationOutcome> } = {};

    expect(await applyIntent(result, intent, owned)).toBe(false);

    expect(snapshot(result)).toEqual(before);
    expect(owned.current).toBeUndefined();
    expect(result.current.operations.operationMessage).toBe(`Operation apply failed to run: ${PREPARE_FAILURE}`);

    failure.mockRestore();
    expect(await applyIntent(result, intent, owned)).toBe(true);
    expect(nodeCY(result)).toBe(2.5);
    expect(result.current.operations.undoStack).toHaveLength(1);
    expect(result.current.operations.redoStack).toHaveLength(0);
    expect(result.current.operations.editorIntents).toHaveLength(0);
    expect(owned.current?.validation.application_status).toBe("applied_to_session_model");
  });

  it("a viewport callback failure after publication is reported; the model and history advance", async () => {
    const result = await session();
    await withRedoEntry(result);
    const intent = await queuedValidatedIntent(result);
    const before = snapshot(result);
    const commands = failViewportRetire(result);
    const owned: { current?: Readonly<OperationOutcome> } = {};

    expect(await applyIntent(result, intent, owned)).toBe(true);

    const operations = result.current.operations;
    expect(commands).toContain("retire-box-gesture");
    expect(nodeCY(result)).toBe(2.5);
    expect(operations.undoStack).toHaveLength(1);
    expect(operations.undoStack[0].model).toEqual(before.model);
    expect(operations.redoStack).toHaveLength(0);
    expect(operations.appliedOperations).toHaveLength(before.appliedOperations.length + 1);
    expect(operations.retainedReviewContext).toHaveLength(before.retainedReviewContext.length + 1);
    expect(operations.operationOutcomes[intent.queue_id!].validation.application_status).toBe("applied_to_session_model");
    expect(operations.editorIntents).toHaveLength(0);
    expect(owned.current?.validation.application_status).toBe("applied_to_session_model");
    expect(operations.operationMessage).toBe(`${INTENT_APPLIED(intent.operation_id)} ${VIEWPORT_FAILURE}`);
  });

  it("an owned-outcome accessor that throws after publication is reported; the model and history advance", async () => {
    const result = await session();
    const intent = await queuedValidatedIntent(result);
    const owned = { set current(_value: Readonly<OperationOutcome> | undefined) { throw new Error("injected accessor failure"); } } as { current?: Readonly<OperationOutcome> };

    expect(await applyIntent(result, intent, owned)).toBe(true);

    expect(nodeCY(result)).toBe(2.5);
    expect(result.current.operations.undoStack).toHaveLength(1);
    expect(result.current.operations.appliedOperations).toHaveLength(1);
    expect(result.current.operations.operationMessage).toBe(
      `${INTENT_APPLIED(intent.operation_id)} The caller did not receive the applied outcome: Error: injected accessor failure`
    );
  });

  it("a normal apply adds one undo checkpoint, clears redo, records the outcome and reports the applied operation", async () => {
    const result = await session();
    await withRedoEntry(result);
    const intent = await queuedValidatedIntent(result);
    const before = snapshot(result);

    expect(await applyIntent(result, intent)).toBe(true);

    const operations = result.current.operations;
    expect(nodeCY(result)).toBe(2.5);
    expect(operations.undoStack).toHaveLength(1);
    expect(operations.undoStack[0]).toMatchObject({ operation_id: intent.operation_id, model: before.model });
    expect(operations.redoStack).toHaveLength(0);
    expect(operations.appliedOperations[0]).toMatchObject({ operation_id: intent.operation_id, sequence: before.appliedOperations.length + 1 });
    expect(operations.operationOutcomes[intent.queue_id!].validation.application_status).toBe("applied_to_session_model");
    expect(operations.editorIntents).toHaveLength(0);
    expect(operations.operationMessage).toBe(INTENT_APPLIED(intent.operation_id));
  });
});

// ---- draft-review apply ----------------------------------------------------

function routeDraft(endMode: "existing" | "new"): RouteDraft {
  return {
    startNodeId: endMode === "existing" ? "node:A" : "node:D",
    endMode,
    existingEndNodeId: "node:C",
    newEnd: { id: "node:E", label: "E", x: "6", y: "0", z: "0", coordinateUnit: "m", provenance: "invented session test input" },
    pipe: {
      id: "pipe:S", label: "S", materialId: "material:steel", outsideDiameter: "0.1", wallThickness: "0.005", lengthUnit: "m",
      yReferenceX: "0", yReferenceY: "0", yReferenceZ: "1", provenance: "invented session test input"
    }
  };
}

async function frozenReview(result: Session, kind: DraftSubmission["kind"]) {
  const built = buildRouteSubmission(result.current.model.model!, [], routeDraft(kind === "single" ? "existing" : "new"), 1, "length=passed");
  if (!built.ok || built.submission.kind !== kind) throw new Error(`expected a ${kind} route submission`);
  let review: FrozenDraftReview | null = null;
  await act(async () => { review = await result.current.operations.handleAddDraftReview(built.submission, 1); });
  expect(review).not.toBeNull();
  return review!;
}

async function applyDraft(result: Session, review: FrozenDraftReview) {
  let applied = false;
  await act(async () => { applied = await result.current.operations.handleApplyDraftReview(review); });
  return applied;
}

const hasPipeS = (model: PreviewModel) => model.pipe_segments.some((pipe) => pipe.id === "pipe:S");
const DRAFT_BATCH_APPLIED = "Applied the reviewed route as one atomic batch; one undo checkpoint is available.";
const DRAFT_SINGLE_APPLIED = (operationId: string) => `Applied reviewed ${operationId}; previous solve results were cleared.`;
const expected = {
  batch: { message: () => DRAFT_BATCH_APPLIED, selection: { type: "node", id: "node:E" }, receipts: 1 },
  single: { message: (review: FrozenDraftReview) => DRAFT_SINGLE_APPLIED(review.submission.kind === "single" ? review.submission.intent.operation_id : ""), selection: { type: "pipe", id: "pipe:S" }, receipts: 0 }
} as const;

describe.each(["batch", "single"] as const)("draft-review apply (%s): Prepare -> Publish -> Notify", (kind) => {
  it("a failure before publication changes nothing and keeps the review registered, so a retry applies", async () => {
    const result = await session();
    await withRedoEntry(result);
    const review = await frozenReview(result, kind);
    const before = snapshot(result);
    const failure = failIdentityHashFor(hasPipeS);

    expect(await applyDraft(result, review)).toBe(false);

    expect(snapshot(result)).toEqual(before);
    expect(result.current.operations.operationMessage).toBe(`Draft apply failed to run: ${PREPARE_FAILURE}`);

    failure.mockRestore();
    expect(await applyDraft(result, review)).toBe(true);
    expect(hasPipeS(result.current.model.model!)).toBe(true);
    expect(result.current.operations.undoStack).toHaveLength(1);
    expect(result.current.operations.redoStack).toHaveLength(0);
    expect(result.current.operations.operationMessage).toBe(expected[kind].message(review));
  });

  it("a review kept after a failed preparation is refused once another edit has changed the model", async () => {
    const result = await session();
    const review = await frozenReview(result, kind);
    const failure = failIdentityHashFor(hasPipeS);
    expect(await applyDraft(result, review)).toBe(false);
    failure.mockRestore();
    await act(async () => { await result.current.operations.handleApplyIntent(coordinateIntent(result.current.model.model!, "2.5", 2)); });
    expect(nodeCY(result)).toBe(2.5);
    const edited = snapshot(result);

    expect(await applyDraft(result, review)).toBe(false);

    expect(snapshot(result)).toEqual(edited);
    expect(hasPipeS(result.current.model.model!)).toBe(false);
  });

  it("a viewport callback failure after publication is reported; the model and history advance", async () => {
    const result = await session();
    await withRedoEntry(result);
    const review = await frozenReview(result, kind);
    const before = snapshot(result);
    const commands = failViewportRetire(result);

    expect(await applyDraft(result, review)).toBe(true);

    const operations = result.current.operations;
    expect(commands).toContain("retire-box-gesture");
    expect(hasPipeS(result.current.model.model!)).toBe(true);
    expect(operations.undoStack).toHaveLength(1);
    expect(operations.undoStack[0].model).toEqual(before.model);
    expect(operations.redoStack).toHaveLength(0);
    expect(operations.appliedOperations).toHaveLength(before.appliedOperations.length + 1);
    expect(operations.batchReceipts).toHaveLength(before.batchReceipts.length + expected[kind].receipts);
    expect(result.current.selection.selection).toEqual(expected[kind].selection);
    expect(operations.operationMessage).toBe(`${expected[kind].message(review)} ${VIEWPORT_FAILURE}`);
  });

  it("a normal apply adds one undo checkpoint, clears redo, selects the route and the review is used once", async () => {
    const result = await session();
    await withRedoEntry(result);
    const review = await frozenReview(result, kind);
    const before = snapshot(result);

    expect(await applyDraft(result, review)).toBe(true);

    const operations = result.current.operations;
    expect(hasPipeS(result.current.model.model!)).toBe(true);
    expect(operations.undoStack).toHaveLength(1);
    expect(operations.undoStack[0]).toMatchObject({ checkpoint_id: `undo-${review.reviewId}`, model: before.model });
    expect(operations.redoStack).toHaveLength(0);
    expect(operations.appliedOperations).toHaveLength(before.appliedOperations.length + 1);
    expect(operations.batchReceipts).toHaveLength(before.batchReceipts.length + expected[kind].receipts);
    expect(result.current.selection.selection).toEqual(expected[kind].selection);
    expect(operations.operationMessage).toBe(expected[kind].message(review));
    const applied = snapshot(result);
    expect(await applyDraft(result, review)).toBe(false);
    expect(snapshot(result)).toEqual(applied);
  });
});

// ---- undo and redo ---------------------------------------------------------

const UNDID = (operationId: string) =>
  `Undid ${operationId} in the local session; previous solve results were cleared. Save is still required to persist the current session model.`;
const REDID = (operationId: string) =>
  `Redid ${operationId} in the local session; previous solve results were cleared. Save is still required to persist the current session model.`;

/** One applied edit (y = 1.5) with its undo checkpoint. */
async function withUndoEntry(result: Session) {
  await act(async () => { await result.current.operations.handleApplyIntent(coordinateIntent(result.current.model.model!, "1.5", 1)); });
  expect(nodeCY(result)).toBe(1.5);
  return result.current.operations.undoStack[0].operation_id;
}

describe("undo and redo: Prepare -> Publish -> Notify", () => {
  it("an undo failure before publication is reported, does not throw, changes nothing, and a retry undoes", async () => {
    const result = await session();
    const operationId = await withUndoEntry(result);
    const before = snapshot(result);
    failNextIdentityHash();

    expect(() => act(() => result.current.operations.handleUndoSessionModelEdit())).not.toThrow();

    expect(snapshot(result)).toEqual(before);
    expect(result.current.operations.operationMessage).toBe(`Undo failed to run: ${PREPARE_FAILURE}`);
    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect(nodeCY(result)).toBe(2);
    expect(result.current.operations.undoStack).toHaveLength(0);
    expect(result.current.operations.redoStack).toHaveLength(1);
    expect(result.current.operations.operationMessage).toBe(UNDID(operationId));
  });

  it("a redo failure before publication is reported, does not throw, changes nothing, and a retry redoes", async () => {
    const result = await session();
    await withRedoEntry(result);
    const operationId = result.current.operations.redoStack[0].operation_id;
    const before = snapshot(result);
    failNextIdentityHash();

    expect(() => act(() => result.current.operations.handleRedoSessionModelEdit())).not.toThrow();

    expect(snapshot(result)).toEqual(before);
    expect(result.current.operations.operationMessage).toBe(`Redo failed to run: ${PREPARE_FAILURE}`);
    act(() => result.current.operations.handleRedoSessionModelEdit());
    expect(nodeCY(result)).toBe(1.5);
    expect(result.current.operations.undoStack).toHaveLength(1);
    expect(result.current.operations.redoStack).toHaveLength(0);
    expect(result.current.operations.operationMessage).toBe(REDID(operationId));
  });

  it.each([
    ["the restored-model clone", () => vi.spyOn(sessionModel, "clonePreviewModel").mockImplementationOnce(() => { throw new Error("injected preparation failure"); })],
    ["the restored selection", () => vi.spyOn(selectionState, "singletonSelection").mockImplementationOnce(() => { throw new Error("injected preparation failure"); })]
  ])("an undo failure while preparing %s is reported, does not throw and changes nothing", async (_label, inject) => {
    const result = await session();
    await withUndoEntry(result);
    const before = snapshot(result);
    inject();

    expect(() => act(() => result.current.operations.handleUndoSessionModelEdit())).not.toThrow();

    expect(snapshot(result)).toEqual(before);
    expect(result.current.operations.operationMessage).toBe("Undo failed to run: Error: injected preparation failure");
    expect(result.current.operations.historyFailure).toBe("Undo failed to run: Error: injected preparation failure");
  });

  it("a viewport callback failure after an undo is reported and does not throw; the model and stacks move", async () => {
    const result = await session();
    const operationId = await withUndoEntry(result);
    const commands = failViewportRetire(result);

    expect(() => act(() => result.current.operations.handleUndoSessionModelEdit())).not.toThrow();

    expect(commands).toContain("retire-box-gesture");
    expect(nodeCY(result)).toBe(2);
    expect(result.current.operations.undoStack).toHaveLength(0);
    expect(result.current.operations.redoStack).toHaveLength(1);
    expect(result.current.operations.operationMessage).toBe(`${UNDID(operationId)} ${VIEWPORT_FAILURE}`);
  });

  it("a viewport callback failure after a redo is reported and does not throw; the model and stacks move", async () => {
    const result = await session();
    await withRedoEntry(result);
    const operationId = result.current.operations.redoStack[0].operation_id;
    const commands = failViewportRetire(result);

    expect(() => act(() => result.current.operations.handleRedoSessionModelEdit())).not.toThrow();

    expect(commands).toContain("retire-box-gesture");
    expect(nodeCY(result)).toBe(1.5);
    expect(result.current.operations.undoStack).toHaveLength(1);
    expect(result.current.operations.redoStack).toHaveLength(0);
    expect(result.current.operations.operationMessage).toBe(`${REDID(operationId)} ${VIEWPORT_FAILURE}`);
  });

  it("a normal undo then redo restores each model, moves one checkpoint each way and restores the selection", async () => {
    const result = await session();
    act(() => result.current.selection.handleSelectEntity({ type: "node", id: "node:C" }));
    const operationId = await withUndoEntry(result);
    const applied = structuredClone(result.current.model.model);
    act(() => result.current.selection.handleSelectEntity({ type: "node", id: "node:B" }));

    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect(nodeCY(result)).toBe(2);
    expect(result.current.operations.undoStack).toHaveLength(0);
    expect(result.current.operations.redoStack[0]).toMatchObject({ checkpoint_id: `redo-undo-1-${operationId}`, model: applied, selection: { type: "node", id: "node:B" } });
    expect(result.current.selection.selection).toEqual({ type: "node", id: "node:C" });
    expect(result.current.operations.operationMessage).toBe(UNDID(operationId));

    act(() => result.current.operations.handleRedoSessionModelEdit());
    expect(result.current.model.model).toEqual(applied);
    expect(result.current.operations.redoStack).toHaveLength(0);
    expect(result.current.operations.undoStack).toHaveLength(1);
    expect(result.current.selection.selection).toEqual({ type: "node", id: "node:B" });
    expect(result.current.operations.operationMessage).toBe(REDID(operationId));
  });
});
