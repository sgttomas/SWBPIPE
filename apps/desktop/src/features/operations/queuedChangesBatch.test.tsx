// G9 (DEL-07-02 `Work/queued-changes-batch.md`): the person's queued single changes
// move into batch review as one batch, in queue order, against the workspace session
// and the actual wasm engine. Apply entire batch validates and applies all or none;
// a change whose before-value depends on another queued change is
// refused by name and nothing applies. Invented run-sheet fixture model.
import { act, cleanup, fireEvent, render, renderHook, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import fixtureJson from "../../../../../fixtures/model_operations/run_sheet_authoring.json";
import * as previewService from "../../services/previewService";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { buildGridOperationIntent, type GridColumn } from "../workspace/table/modelTableAdapter";
import { inspectorAuthoringPaths } from "../model-tree/PropertyInspector";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { OperationApplyPanel } from "./OperationApplyPanel";

const fixture = fixtureJson as unknown as { basis_model: PreviewModel };
afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
});

async function harness() {
  vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(structuredClone(fixture.basis_model));
  const hook = renderHook(() => useWorkspaceSession());
  await waitFor(() => expect(hook.result.current.model.model).not.toBeNull());
  return hook.result;
}
type Session = Awaited<ReturnType<typeof harness>>;

/** A coordinate edit of a stored node, as the engineering table queues it. */
function coordinateIntent(model: PreviewModel, nodeId: string, axis: "x" | "y", value: string, sequence: number): EditorOperationIntent {
  const target = model.nodes.find((node) => node.id === nodeId)!;
  const column: GridColumn = { key: axis, label: axis.toUpperCase(), fieldPath: `position.${axis}`, objectType: "Node", changeKind: "set_field", dimension: "length", sourceNote: "entered coordinate", unit: () => "m", value: () => String(target.position[axis]) };
  return buildGridOperationIntent({ column, model, row: { id: target.id, label: target.label, type: "node", raw: target, searchText: "" }, sequence, value, interaction: "cell" });
}
function queue(session: Session, intents: EditorOperationIntent[]) {
  act(() => { for (const intent of intents) session.current.operations.handleQueueEditorIntent(intent); });
}
async function reviewAllAsBatch(session: Session) {
  await act(async () => { await session.current.operations.handleReviewQueuedIntentsAsBatch(); });
}
async function run(session: Session, apply: boolean) {
  await act(async () => { await session.current.operations.handleRunOperationBatch(session.current.operations.queuedBatches[0], apply); });
}

describe("review all queued changes as one batch (G9)", () => {
  it("moves the queued changes in queue order and applies them only as one validated batch", async () => {
    const session = await harness();
    const model = session.current.model.model!;
    const first = coordinateIntent(model, "node:A", "y", "0.5", 1);
    const second = coordinateIntent(model, "node:C", "y", "-0.25", 2);
    queue(session, [first, second]);
    expect(session.current.operations.editorIntents).toHaveLength(2);

    await reviewAllAsBatch(session);
    expect(session.current.operations.editorIntents).toEqual([]);
    const [entry] = session.current.operations.queuedBatches;
    expect(entry.batch.operations.map((op) => op.operation_id)).toEqual([first.operation_id, second.operation_id]);
    expect(session.current.model.model).toEqual(model);

    await run(session, false);
    expect(session.current.model.model).toEqual(model);
    const undoBefore = session.current.operations.undoStack.length;
    const appliedBefore = session.current.operations.appliedOperations.length;
    await run(session, true);
    const applied = session.current.model.model!;
    expect(applied.nodes.find((node) => node.id === "node:A")!.position.y).toBe(0.5);
    expect(applied.nodes.find((node) => node.id === "node:C")!.position.y).toBe(-0.25);
    expect(session.current.operations.undoStack.length).toBe(undoBefore + 1);
    expect(session.current.operations.appliedOperations.length).toBe(appliedBefore + 1);
    expect(session.current.operations.batchReceipts.at(-1)!.batch.operations).toHaveLength(2);
  });

  it("refuses a change whose before-value another queued change makes stale, and applies nothing", async () => {
    const session = await harness();
    const model = session.current.model.model!;
    queue(session, [coordinateIntent(model, "node:A", "y", "0.5", 1), coordinateIntent(model, "node:A", "y", "0.75", 2)]);
    await reviewAllAsBatch(session);
    await run(session, false);
    const outcome = Object.values(session.current.operations.batchOutcomes)[0]!;
    expect(outcome.validation.batch_validation_status).not.toBe("passed");
    const refused = outcome.operation_outcomes.flatMap((step) => step.diagnostics.filter((d) => d.severity === "blocking").map((d) => d.code));
    expect(refused).toContain("OP-STALE-BEFORE-VALUE");
    await run(session, true);
    expect(session.current.model.model).toEqual(model);
  });

  it("does nothing with fewer than two of the person's queued changes", async () => {
    const session = await harness();
    const model = session.current.model.model!;
    queue(session, [coordinateIntent(model, "node:A", "y", "0.5", 1)]);
    await reviewAllAsBatch(session);
    expect(session.current.operations.editorIntents).toHaveLength(1);
    expect(session.current.operations.queuedBatches).toEqual([]);
  });

  it("names duplicate operation ids before moving same-field edits and leaves the single queue unchanged", async () => {
    const session = await harness();
    const model = session.current.model.model!;
    const node = model.nodes.find((item) => item.id === "node:A")!;
    const edits = ["First label", "Second label"].map((proposedValue) => inspectorAuthoringPaths.buildOperationIntent({
      field: { label: "Label", fieldPath: "label", before: node.label, objectType: "Node", changeKind: "set_field", dimension: "dimensionless", unit: "1", sourceNote: "entered label", unitEditable: false, valueKind: "text" },
      model, proposedValue, proposedUnit: "1", rationale: "entered label", selection: { type: "node", id: node.id }, unitCatalogRoute: null
    }));
    expect(edits[0].operation_id).toBe(edits[1].operation_id);
    queue(session, [edits[0]]);
    queue(session, [edits[1]]);
    const queuedBefore = structuredClone(session.current.operations.editorIntents);
    expect(new Set(queuedBefore.map((intent) => intent.queue_id)).size).toBe(2);
    const panel = () => <OperationApplyPanel
      queuedIntents={session.current.operations.editorIntents}
      outcomes={session.current.operations.operationOutcomes}
      appliedOperations={session.current.operations.appliedOperations}
      undoCount={session.current.operations.undoStack.length}
      redoCount={session.current.operations.redoStack.length}
      busy={false} message={session.current.operations.operationMessage} engineStatus={{ state: "ready" } as never}
      onValidate={() => {}} onApply={() => {}} onUndo={() => {}} onRedo={() => {}}
      onReviewAllAsBatch={session.current.operations.handleReviewQueuedIntentsAsBatch}
    />;
    const view = render(panel());
    await act(async () => { fireEvent.click(screen.getByRole("button", { name: "Review all 2 queued changes as one batch" })); });
    view.rerender(panel());

    expect(screen.getByTestId("operation-apply-message")).toHaveTextContent(`2 changes share the operation id ${edits[0].operation_id}`);
    expect(screen.getByTestId("operation-apply-message")).toHaveTextContent("every change stays queued");
    expect(session.current.operations.editorIntents).toEqual(queuedBefore);
    expect(session.current.operations.queuedBatches).toEqual([]);
    expect(session.current.operations.operationOutcomes).toEqual({});
    expect(session.current.operations.undoStack).toEqual([]);
    expect(session.current.operations.batchReceipts).toEqual([]);
    expect(session.current.model.model).toEqual(model);
  });
});

describe("the apply panel's batch control (G9)", () => {
  const props = { outcomes: {}, appliedOperations: [], undoCount: 0, redoCount: 0, busy: false, message: null, engineStatus: { state: "ready" } as never, onValidate: () => {}, onApply: () => {}, onUndo: () => {}, onRedo: () => {} };
  const intent = (n: number, author = "user") => ({ ...coordinateIntent(fixture.basis_model, "node:A", "y", String(n), n), queue_id: `editor-intent-${n}`, author_type: author }) as EditorOperationIntent;

  it("offers the person's queued changes as one batch, counts only theirs, and waits while busy", () => {
    const onReviewAllAsBatch = vi.fn();
    const { rerender } = render(<OperationApplyPanel {...props} queuedIntents={[intent(1)]} onReviewAllAsBatch={onReviewAllAsBatch} />);
    expect(screen.queryByTestId("review-all-as-batch")).toBeNull();
    rerender(<OperationApplyPanel {...props} queuedIntents={[intent(1), intent(2), intent(3, "agent")]} onReviewAllAsBatch={onReviewAllAsBatch} />);
    const button = screen.getByRole("button", { name: "Review all 2 queued changes as one batch" });
    expect(button).toHaveAccessibleDescription(/refused by name, never adjusted/);
    fireEvent.click(button);
    expect(onReviewAllAsBatch).toHaveBeenCalledOnce();
    rerender(<OperationApplyPanel {...props} busy queuedIntents={[intent(1), intent(2)]} onReviewAllAsBatch={onReviewAllAsBatch} />);
    expect(screen.getByRole("button", { name: "Review all 2 queued changes as one batch" })).toBeDisabled();
  });
});
