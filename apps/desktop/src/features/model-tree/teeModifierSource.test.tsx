// G11: the person enters the tee's modifier source through the existing
// inspector, operation engine and browser project store. No numerical change.
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import referenceJson from "../../../../../fixtures/reference_system/reference_system.json";
import { applyModelOperation } from "../../services/operationService";
import { applyOperationBatch } from "../../services/operationBatchService";
import { stampImportedBatch } from "../../services/operationAuthorship";
import { createLocalProject, openLocalProject } from "../../services/projectService";
import { computeModelHash } from "../../services/hashService";
import * as previewService from "../../services/previewService";
import { changeFormControl } from "../../test-support/workspaceTestControls";
import type { EditorOperationIntent, PreviewModel } from "../../types";
import { DisplayUnitsProvider } from "../display-units";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { PropertyInspector } from "./PropertyInspector";
import { OperationApplyPanel } from "../operations/OperationApplyPanel";

afterEach(() => { cleanup(); vi.restoreAllMocks(); });
const REF = "component:RS-T1";
function basis() {
  const model = structuredClone(referenceJson) as unknown as PreviewModel;
  model.diagnostics = [];
  return model;
}
const tee = (model: PreviewModel) => model.components.find((item) => item.id === REF)!;
function startTask(fieldPath = "modifiers.source_reference") {
  fireEvent.click(screen.getByRole("tab", { name: "Task" }));
  fireEvent.click(screen.getByTestId("inspector-start-task"));
  changeFormControl(screen.getByTestId("editor-intent-field"), { target: { value: fieldPath } });
}
function enter(value: string) {
  fireEvent.change(screen.getByRole("textbox", { name: "New modifier source" }), { target: { value } });
}

describe("tee Modifier source (G11)", () => {
  it("enters the missing fixture source, queues without mutation, explicitly applies and saves/reopens the exact tee", async () => {
    const model = basis();
    const expected = structuredClone(tee(model));
    delete tee(model).modifiers!.source_reference;
    const before = structuredClone(model);
    vi.spyOn(previewService, "loadPreviewModel").mockResolvedValue(model);
    let session: ReturnType<typeof useWorkspaceSession>;
    function Harness() {
      session = useWorkspaceSession();
      return session.model.model ? <DisplayUnitsProvider initialPreference="SI"><PropertyInspector
        model={session.model.model} selection={{ type: "component", id: REF }}
        onQueueIntent={session.operations.handleQueueEditorIntent}
        onApplyIntent={session.operations.handleApplyIntent}
      /><OperationApplyPanel
        queuedIntents={session.operations.editorIntents} outcomes={session.operations.operationOutcomes}
        appliedOperations={session.operations.appliedOperations}
        undoCount={session.operations.undoStack.length} redoCount={session.operations.redoStack.length}
        busy={session.operations.operationBusy} message={session.operations.operationMessage}
        engineStatus={{ state: "ready" } as never}
        onValidate={session.operations.handleValidateIntent} onApply={session.operations.handleApplyIntent}
        onUndo={session.operations.handleUndoSessionModelEdit} onRedo={session.operations.handleRedoSessionModelEdit}
      /></DisplayUnitsProvider> : null;
    }
    render(<Harness />);
    await screen.findByTestId("property-inspector");
    startTask();
    expect(screen.getByRole("textbox", { name: "New modifier source" })).toHaveValue("TBD");
    expect(screen.getByTestId("queue-editor-intent")).toBeDisabled();
    expect(screen.getByTestId("apply-editor-intent-inline")).toBeDisabled();
    const source = expected.modifiers!.source_reference!;
    enter(source);
    expect(within(screen.getByTestId("editor-intent-panel")).getByTestId("editor-operation-preview")).toHaveTextContent(source);
    fireEvent.click(screen.getByTestId("queue-editor-intent"));
    expect(session!.operations.editorIntents).toHaveLength(1);
    const intent = session!.operations.editorIntents[0];
    expect(intent).toMatchObject({
      author_type: "user", target: { object_type: "Component", ref: REF },
      change: { change_kind: "set_field", field_path: "modifiers.source_reference", before: "TBD", after: source, dimension: "dimensionless", unit: "none" }
    });
    expect(session!.model.model).toEqual(before);
    expect(model).toEqual(before);
    expect(session!.operations.undoStack).toEqual([]);

    fireEvent.click(screen.getByTestId(`apply-intent-${intent.queue_id}`));
    await waitFor(() => expect(session!.operations.undoStack).toHaveLength(1));
    expect(session!.operations.editorIntents).toEqual([]);
    expect(session!.operations.undoStack).toHaveLength(1);
    expect(tee(session!.model.model!)).toEqual(expected);
    expect(model).toEqual(before);
    await createLocalProject(session!.model.model!);
    const reopened = await openLocalProject(model.project.id);
    expect(tee(reopened!.model)).toEqual(expected);
  });

  it("changes only an explicitly edited source, preserving entered k/SIFs and refusing a stale before value", async () => {
    const model = basis();
    tee(model).modifiers!.flexibility_factor_user_value = { value: 1.75, unit: "none" };
    const before = structuredClone(model);
    const queue = vi.fn<(intent: EditorOperationIntent) => void>();
    render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector model={model} selection={{ type: "component", id: REF }} onQueueIntent={queue} /></DisplayUnitsProvider>);
    startTask();
    expect(screen.getByRole("textbox", { name: "New modifier source" })).toHaveValue(tee(before).modifiers!.source_reference);
    const source = "invented revised person-entered tee modifier source";
    enter(source);
    fireEvent.click(screen.getByTestId("queue-editor-intent"));
    const intent = queue.mock.calls[0][0];
    const result = await applyModelOperation(model, intent, null);
    expect(result.validation.application_status, JSON.stringify(result.diagnostics)).toBe("applied_to_session_model");
    const expected = structuredClone(before);
    tee(expected).modifiers!.source_reference = source;
    expect(result.applied_model).toEqual(expected);
    const stale = await applyModelOperation(result.applied_model!, intent, null);
    expect(stale.applied_model).toBeNull();
    expect(stale.diagnostics.some((item) => item.severity === "blocking" && item.code === "OP-STALE-BEFORE-VALUE")).toBe(true);
    expect(model).toEqual(before);
  });

  it.each([
    ["modifiers.source_reference", "person source", "OP-COMPONENT-MODIFIER-USER-ENTRY-REQUIRED"],
    ["modifiers.branch_header_sif_user_value.value", "2.6", "OP-COMPONENT-MODIFIER-USER-ENTRY-REQUIRED"],
    ["modifiers.flexibility_factor_user_value.value", "1.8", "OP-BEND-FLEXIBILITY-USER-ENTRY-REQUIRED"]
  ])("retains agent and imported-proposal refusals for %s", async (fieldPath, value, code) => {
    const model = basis();
    const before = structuredClone(model);
    const queue = vi.fn<(intent: EditorOperationIntent) => void>();
    render(<DisplayUnitsProvider initialPreference="SI"><PropertyInspector model={model} selection={{ type: "component", id: REF }} onQueueIntent={queue} /></DisplayUnitsProvider>);
    startTask(fieldPath);
    fireEvent.change(screen.getByTestId("editor-intent-value"), { target: { value } });
    fireEvent.click(screen.getByTestId("queue-editor-intent"));
    const user = queue.mock.calls[0][0];
    expect(user.author_type).toBe("user");
    const agent = await applyModelOperation(model, { ...structuredClone(user), author_type: "agent" }, null);
    expect(agent.applied_model).toBeNull();
    expect(agent.diagnostics.some((item) => item.code === code && item.severity === "blocking")).toBe(true);
    const imported = stampImportedBatch({ batch_id: "imported-modifier", operations: [user] });
    expect(imported.batch.operations[0].author_type).toBe("agent");
    const refused = await applyOperationBatch(model, imported.batch, await computeModelHash(model));
    expect(refused.applied_model).toBeNull();
    const codes = [...refused.diagnostics, ...refused.operation_outcomes.flatMap((item) => item.diagnostics)].map((item) => item.code);
    expect(codes).toContain(code);
    expect(model).toEqual(before);
  });
});
