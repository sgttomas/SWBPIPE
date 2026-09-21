import { act, fireEvent, render, renderHook, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { App } from "./App";
import * as operationService from "./services/operationService";
import { useWorkspaceSession } from "./features/workspace/workspaceSession";
import { buildGridOperationIntent, type GridColumn } from "./features/workspace/table/modelTableAdapter";
import type { PreviewModel } from "./types";
import { computeModelHash } from "./services/hashService";

afterEach(() => { vi.restoreAllMocks(); });
function coordinateIntent(model: PreviewModel, value = "0.5") {
  const node = model.nodes[0];
  const column: GridColumn = { key: "y", label: "Y", fieldPath: "position.y", objectType: "Node", changeKind: "set_field", dimension: "length", sourceNote: "entered coordinate", unit: () => model.project.units.length, value: () => String(node.position.y) };
  return buildGridOperationIntent({ column, model, row: { id: node.id, label: node.label, type: "node", raw: node, searchText: "" }, sequence: 1, value, interaction: "cell" });
}

describe("table operations through the existing session controller", () => {
  it("connects the primary table pointer Apply to one actual engine operation and toolbar history", async () => {
    const apply = vi.spyOn(operationService, "applyModelOperation");
    render(<App />); await screen.findByTestId("desktop-preview-shell");
    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    const cell = () => screen.getByTestId("table-cell-node:N-100-y");
    fireEvent.click(cell()); fireEvent.click(cell());
    fireEvent.change(screen.getByRole("textbox", { name: "node:N-100 Y [m]" }), { target: { value: "0.5" } });
    fireEvent.click(within(screen.getByTestId("engineering-table")).getByRole("button", { name: "Apply" }));
    await waitFor(() => expect(cell()).toHaveTextContent("0.5"));
    expect(apply).toHaveBeenCalledTimes(1);
    expect(apply.mock.calls[0][1]).toMatchObject({ author_type: "user", target: { object_type: "Node", ref: "node:N-100" }, change: { field_path: "position.y", before: "0", after: '{"value":0.5,"unit":"m"}', unit: "m", dimension: "length" } });
    await screen.findByTestId("project-edited");
    fireEvent.click(screen.getByTestId("workspace-undo")); expect(cell()).toHaveTextContent("0"); expect(screen.getByTestId("workspace-undo")).toBeDisabled();
    fireEvent.click(screen.getByTestId("workspace-redo")); expect(cell()).toHaveTextContent("0.5");
  });
  it("creates one real checkpoint, preserves selection, Undo/Redo/Edited and Save/Open, and matches a reviewed offline agent batch", async () => {
    const { result } = renderHook(() => useWorkspaceSession());
    await waitFor(() => expect(result.current.model.model).not.toBeNull());
    const basis = structuredClone(result.current.model.model!); const beforeHash = await computeModelHash(basis);
    const intent = coordinateIntent(basis);
    act(() => result.current.selection.handleSelectEntity({ type: "node", id: basis.nodes[0].id }));
    const selection = result.current.selection.orderedSelection;
    let accepted = false;
    await act(async () => { accepted = await result.current.operations.handleApplyIntent(intent); });
    expect(accepted).toBe(true); expect(result.current.model.model!.nodes[0].position.y).toBe(0.5);
    expect(result.current.operations.undoStack).toHaveLength(1); expect(result.current.operations.appliedOperations).toHaveLength(1);
    expect(result.current.selection.orderedSelection.orderedKeys).toEqual(selection.orderedKeys);
    await waitFor(() => expect(result.current.project.modelEdited).toBe(true));
    const humanModel = structuredClone(result.current.model.model!);
    const userRecord = result.current.operations.retainedReviewContext.at(-1)!;
    expect(userRecord.author_type).toBe("user"); expect(userRecord.change).toEqual(intent.change);
    act(() => result.current.operations.handleUndoSessionModelEdit());
    expect(result.current.operations.undoStack).toHaveLength(0); expect(result.current.operations.redoStack).toHaveLength(1);
    expect((await computeModelHash(result.current.model.model!))?.value).toBe(beforeHash?.value);
    act(() => result.current.operations.handleRedoSessionModelEdit()); expect(result.current.model.model).toEqual(humanModel);
    await act(async () => { await result.current.project.handleSaveProject(); });
    await waitFor(() => expect(result.current.project.modelEdited).toBe(false));
    act(() => result.current.operations.handleUndoSessionModelEdit());
    await waitFor(() => expect(result.current.project.modelEdited).toBe(true));
    const agent = structuredClone(intent); agent.author_type = "agent"; agent.operation_id = "op:table-offline-equivalent"; agent.change.change_id = "change:table-offline-equivalent";
    agent.source = { source_ref: "offline:table-test", source_channel: "offline_user_supplied", source_role: "agent" }; agent.rationale = "Explicitly reviewed offline node-coordinate change.";
    await act(async () => { await result.current.operations.handleQueueOperationBatch({ batch_id: "batch:table-offline-equivalent", operations: [agent] }); });
    const queued = result.current.operations.queuedBatches[0]; expect(queued.batch.operations[0]).toEqual(agent);
    await act(async () => { await result.current.operations.handleRunOperationBatch(queued, false); });
    expect(result.current.operations.undoStack).toHaveLength(0); expect(result.current.model.model).toEqual(basis);
    await act(async () => { await result.current.operations.handleRunOperationBatch(queued, true); });
    expect(result.current.model.model).toEqual(humanModel); expect(result.current.operations.undoStack).toHaveLength(1);
    expect(result.current.operations.retainedReviewContext.at(-1)).toEqual(agent);
    await waitFor(() => expect(result.current.project.modelEdited).toBe(false));
    await act(async () => { await result.current.project.handleOpenProject(); });
    expect(result.current.model.model).toEqual(humanModel); expect(result.current.operations.undoStack).toHaveLength(0);
    expect(result.current.results.currentSolvedResult).toBeNull();
  });
  it("keeps an actual engine rejection out of model history", async () => {
    const { result } = renderHook(() => useWorkspaceSession());
    await waitFor(() => expect(result.current.model.model).not.toBeNull());
    const basis = structuredClone(result.current.model.model!); const intent = coordinateIntent(basis);
    intent.change.unit = "kg"; intent.change.after = JSON.stringify({ value: 0.5, unit: "kg" });
    let accepted = true;
    await act(async () => { accepted = await result.current.operations.handleApplyIntent(intent); });
    expect(accepted).toBe(false); expect(result.current.model.model).toEqual(basis); expect(result.current.operations.undoStack).toHaveLength(0);
    const outcome = Object.values(result.current.operations.operationOutcomes)[0]; expect(outcome.validation.application_status).not.toBe("applied_to_session_model"); expect(outcome.diagnostics.length).toBeGreaterThan(0);
  });
});
