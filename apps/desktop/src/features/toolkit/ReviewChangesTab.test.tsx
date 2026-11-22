// The Review changes tab: one line at rest; each pending change one row in words
// with Apply and Remove; the engine's refusal in words on the row and its code in
// the row's details; Apply all and Discard all; the session's record in a closed
// "Record"; batches only while one exists. Invented run-sheet fixture model.
import { act, cleanup, fireEvent, render, renderHook, screen, waitFor, within } from "@testing-library/react";
import { useState } from "react";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import fixtureJson from "../../../../../fixtures/model_operations/run_sheet_authoring.json";
import * as previewService from "../../services/previewService";
import type { OperationBatch } from "../../services/operationBatchService";
import type { AppliedOperationReceipt, EditorOperationIntent, ModelHashEvidence, OperationOutcome, PreviewModel } from "../../types";
import { buildGridOperationIntent, type GridColumn } from "../workspace/table/modelTableAdapter";
import { useWorkspaceSession } from "../workspace/workspaceSession";
import { queuedChangeLine } from "../operations/OperationApplyPanel";
import { ReviewChangesTab } from "./ReviewChangesTab";
import type { BatchReceipt, QueuedBatch } from "./BatchReviewPanel";

const fixture = fixtureJson as unknown as { basis_model: PreviewModel };
afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
});

/** A coordinate edit of a stored node, as the engineering table queues it. */
function coordinateIntent(model: PreviewModel, nodeId: string, value: string, sequence: number): EditorOperationIntent {
  const target = model.nodes.find((node) => node.id === nodeId)!;
  const column: GridColumn = { key: "y", label: "Y", fieldPath: "position.y", objectType: "Node", changeKind: "set_field", dimension: "length", sourceNote: "entered coordinate", unit: () => "m", value: () => String(target.position.y) };
  return { ...buildGridOperationIntent({ column, model, row: { id: target.id, label: target.label, type: "node", raw: target, searchText: "" }, sequence, value, interaction: "cell" }), queue_id: `editor-intent-${sequence}` };
}

type TabProps = Parameters<typeof ReviewChangesTab>[0];
function props(over: Partial<TabProps> = {}): TabProps {
  return {
    batches: [], batchOutcomes: {}, batchReceipts: [], revision: 1, busy: false, batchMessage: null,
    onClear: vi.fn(), onValidateBatch: vi.fn(), onApplyBatch: vi.fn(), model: fixture.basis_model, hasResults: false,
    queuedIntents: [], outcomes: {}, appliedOperations: [], undoCount: 0, redoCount: 0, message: null,
    engineStatus: { route: "local_wasm_engine", state: "ready", detail: null },
    onValidate: vi.fn(), onApply: vi.fn(async () => true), onRemove: vi.fn(),
    ...over
  };
}

/** The text a person sees without a press: nothing hidden, nothing in a closed disclosure but its summary. */
function shownLines(root: HTMLElement): string[] {
  const shown = (element: Element) => {
    for (let node: Element | null = element; node && node !== root.parentElement; node = node.parentElement) {
      if (node.hasAttribute("hidden")) return false;
      const details = node.parentElement;
      if (details?.tagName === "DETAILS" && !(details as HTMLDetailsElement).open && node.tagName !== "SUMMARY") return false;
    }
    return true;
  };
  return Array.from(root.querySelectorAll("p, button, summary, h3, h4"))
    .filter((element) => !element.parentElement?.closest("button, summary") && shown(element))
    .map((element) => (element.textContent ?? "").trim())
    .filter(Boolean);
}

const tab = () => screen.getByTestId("operation-apply-panel");
const model = fixture.basis_model;
const nodeA = model.nodes.find((node) => node.id === "node:A")!;

describe("the Review changes tab at rest", () => {
  it("reads exactly one line, with the session's record closed at the foot", () => {
    render(<ReviewChangesTab {...props()} />);
    expect(shownLines(tab())).toEqual(["No pending changes.", "Record"]);
    expect(screen.getByTestId("review-record")).not.toHaveAttribute("open");
    expect(screen.queryByTestId("review-engine-not-ready")).toBeNull();
    expect(document.getElementById("batch-review")).toBeNull();
  });

  it("says the engine is not ready only while it is not, and what a person meets", () => {
    const view = render(<ReviewChangesTab {...props({ engineStatus: { route: "local_wasm_engine", state: "loading", detail: null } })} />);
    expect(screen.getByTestId("review-engine-not-ready")).toHaveTextContent("The editing engine is still loading. Apply waits until it is ready.");
    view.rerender(<ReviewChangesTab {...props({ engineStatus: { route: "local_wasm_engine", state: "unavailable", detail: "WASM-ENGINE-ASSET-ABSENT" } })} />);
    expect(screen.getByTestId("review-engine-not-ready")).toHaveTextContent("Changes cannot be checked or applied: the editing engine did not load (WASM-ENGINE-ASSET-ABSENT).");
    view.rerender(<ReviewChangesTab {...props()} />);
    expect(screen.queryByTestId("review-engine-not-ready")).toBeNull();
  });
});

describe("a pending change", () => {
  it("shows one row in words, with Apply and Remove and no identifier on the line", () => {
    const intent = coordinateIntent(model, "node:A", "0.5", 1);
    const onApply = vi.fn(async () => true);
    const onRemove = vi.fn();
    render(<ReviewChangesTab {...props({ queuedIntents: [intent], onApply, onRemove })} />);
    const line = screen.getByTestId("operation-apply-line-editor-intent-1");
    // The table records the new value as a quantity, `{"value":0.5,"unit":"m"}`, and the old one as text.
    expect(intent.change.after).toBe(JSON.stringify({ value: 0.5, unit: "m" }));
    expect(line).toHaveTextContent(`Y of node ${nodeA.label}: ${nodeA.position.y} m → 0.5 m`);
    expect(line.textContent).not.toContain(intent.operation_id);
    expect(line.textContent).not.toContain("node:A");
    const row = screen.getByTestId("operation-apply-row-editor-intent-1");
    expect(within(row).getByRole("button", { name: "Apply" })).toBe(screen.getByTestId("apply-intent-editor-intent-1"));
    fireEvent.click(within(row).getByRole("button", { name: "Remove" }));
    expect(onRemove).toHaveBeenCalledWith("editor-intent-1");
    fireEvent.click(screen.getByTestId("apply-intent-editor-intent-1"));
    expect(onApply).toHaveBeenCalledWith(intent);
    // The operation as recorded and Validate are in the row's details, closed by default.
    const details = screen.getByTestId("operation-apply-details-editor-intent-1");
    expect(details).not.toHaveAttribute("open");
    expect(details).toHaveTextContent(intent.operation_id);
    expect(details).toContainElement(screen.getByTestId("validate-intent-editor-intent-1"));
    expect(shownLines(tab())).toEqual(["Apply all", "Discard all", line.textContent, "Apply", "Remove", "Details", "Record"]);
  });

  it("shows the engine's reason in words when it cannot be applied, and its code only in the details", () => {
    const intent = coordinateIntent(model, "node:A", "0.5", 1);
    const reason = "The before value no longer matches the model: node:A position.y is 1.5, not 0.";
    const outcome = {
      mode: "validate_only", application_route: "local_wasm_engine", operation_id: intent.operation_id,
      validation: { schema_validation: "passed", reference_validation: "passed", unit_validation: "passed", before_state_validation: "blocked", diff_preview_status: "blocked_by_validation", application_status: "not_applied" },
      diff_preview: [], diagnostics: [{ id: "d1", code: "OP-STALE-BEFORE-VALUE", severity: "blocking", message: reason, remediation: "Queue it again.", affected_refs: ["node:A"], source: "engine" }]
    } as unknown as OperationOutcome;
    render(<ReviewChangesTab {...props({ queuedIntents: [intent], outcomes: { "editor-intent-1": outcome } })} />);
    const state = screen.getByTestId("operation-apply-state-editor-intent-1");
    expect(state).toHaveTextContent(`Cannot be applied: ${reason}`);
    expect(state.textContent).not.toContain("OP-STALE-BEFORE-VALUE");
    const code = screen.getByTestId("operation-outcome-diagnostic-OP-STALE-BEFORE-VALUE");
    expect(screen.getByTestId("operation-apply-details-editor-intent-1")).toContainElement(code);
    expect(code).toHaveTextContent("blocking: OP-STALE-BEFORE-VALUE");
  });

  it("names an added node and a deleted one by their names", () => {
    const create = { ...coordinateIntent(model, "node:A", "0.5", 1), target: { object_type: "Node", ref: "node:N-9" },
      change: { change_id: "c", change_kind: "create_node", field_label: "Explicit node geometry", field_path: "nodes", before: "not_present",
        after: JSON.stringify({ id: "node:N-9", label: "Anchor", position: { x: 0, y: 2.4, z: 0 } }), unit: "m", dimension: "length" } } as EditorOperationIntent;
    expect(queuedChangeLine(create, model)).toBe("Add node Anchor at (0, 2.4, 0) m");
    const remove = { ...create, target: { object_type: "Node", ref: "node:A" },
      change: { ...create.change, change_kind: "delete_node", before: "stored", after: "not_present" } } as EditorOperationIntent;
    expect(queuedChangeLine(remove, model)).toBe(`Delete node ${nodeA.label}`);
  });

  it("says applying clears the current results only while results are shown", () => {
    const intent = coordinateIntent(model, "node:A", "0.5", 1);
    const view = render(<ReviewChangesTab {...props({ queuedIntents: [intent], hasResults: true })} />);
    expect(screen.getByTestId("apply-clears-results")).toHaveTextContent("Applying a change clears the current results.");
    view.rerender(<ReviewChangesTab {...props({ queuedIntents: [intent], hasResults: false })} />);
    expect(screen.queryByTestId("apply-clears-results")).toBeNull();
  });
});

describe("Apply all and Discard all", () => {
  it("Apply all runs each row's own Apply in queue order and stops at the first not applied", async () => {
    // The queue lists the newest first, as the session prepends.
    const [first, second, third] = [1, 2, 3].map((n) => coordinateIntent(model, "node:A", String(n), n));
    const onApply = vi.fn<(intent: EditorOperationIntent) => boolean>((intent) => intent !== second);
    // As the session does: an applied change leaves the list in the same update.
    function Queue() {
      const [queued, setQueued] = useState([third, second, first]);
      return <ReviewChangesTab {...props({ queuedIntents: queued, onApply: async (intent) => {
        const applied = onApply(intent);
        if (applied) setQueued((current) => current.filter((item) => item !== intent));
        return applied;
      } })} />;
    }
    render(<Queue />);
    await act(async () => { fireEvent.click(screen.getByTestId("apply-all-intents")); });
    await waitFor(() => expect(screen.getByTestId("apply-all-intents")).toHaveTextContent("Apply all"));
    expect(onApply.mock.calls.map(([intent]) => intent)).toEqual([first, second]);
    expect(screen.queryByTestId("operation-apply-row-editor-intent-1")).toBeNull();
    expect(screen.getByTestId("operation-apply-row-editor-intent-2")).toBeInTheDocument();
    expect(screen.getByTestId("operation-apply-row-editor-intent-3")).toBeInTheDocument();
  });

  it("Apply all leaves the session as the rows' Apply, one after another, leaves it", { timeout: 30_000 }, async () => {
    vi.spyOn(previewService, "loadPreviewModel").mockImplementation(async () => structuredClone(fixture.basis_model));
    const sessions = [renderHook(() => useWorkspaceSession()).result, renderHook(() => useWorkspaceSession()).result];
    await waitFor(() => expect(sessions.every((session) => session.current.model.model)).toBe(true));
    function Tab({ index }: { index: number }) {
      const ops = sessions[index].current.operations;
      return <div data-testid={`tab-${index}`}><ReviewChangesTab {...props({
        queuedIntents: ops.editorIntents, outcomes: ops.operationOutcomes, appliedOperations: ops.appliedOperations,
        busy: ops.operationBusy, onApply: ops.handleApplyIntent, model: sessions[index].current.model.model
      })} /></div>;
    }
    for (const session of sessions) {
      const basis = session.current.model.model!;
      // One act each: the session numbers a queued change when its update runs.
      act(() => { session.current.operations.handleQueueEditorIntent(coordinateIntent(basis, "node:A", "0.5", 1)); });
      act(() => { session.current.operations.handleQueueEditorIntent(coordinateIntent(basis, "node:C", "-0.25", 2)); });
    }
    const view = render(<><Tab index={0} /><Tab index={1} /></>);
    const rerender = () => view.rerender(<><Tab index={0} /><Tab index={1} /></>);
    // Session 0: Apply all.
    await act(async () => { fireEvent.click(within(screen.getByTestId("tab-0")).getByTestId("apply-all-intents")); });
    // The two sessions are separate roots, so the tab is re-rendered with each session's state as App would.
    for (let step = 0; step < 300 && (sessions[0].current.operations.editorIntents.length > 0 || sessions[0].current.operations.operationBusy); step += 1) {
      await act(async () => { rerender(); await new Promise((resolve) => setTimeout(resolve, 20)); });
    }
    // Session 1: each row's Apply, oldest first.
    for (const remaining of [1, 0]) {
      rerender();
      const rows = within(screen.getByTestId("tab-1")).getAllByTestId(/^apply-intent-/);
      await act(async () => { fireEvent.click(rows.at(-1)!); });
      await waitFor(() => expect(sessions[1].current.operations.editorIntents.length === remaining && !sessions[1].current.operations.operationBusy).toBe(true), { timeout: 10_000 });
    }
    const [all, each] = sessions.map((session) => session.current);
    expect(all.operations.editorIntents).toEqual([]);
    expect(each.operations.editorIntents).toEqual([]);
    expect(all.model.model).toEqual(each.model.model);
    expect(all.model.model!.nodes.find((node) => node.id === "node:C")!.position.y).toBe(-0.25);
    expect(all.operations.undoStack.length).toBe(each.operations.undoStack.length);
    expect(all.operations.appliedOperations).toEqual(each.operations.appliedOperations);
  });

  it("Discard all is the present clear, offered only while something is pending or running", () => {
    const onClear = vi.fn();
    const view = render(<ReviewChangesTab {...props({ onClear })} />);
    const discard = screen.getByTestId("clear-pending-batches");
    expect(discard).toBeDisabled();
    expect(discard.closest("[hidden]")).not.toBeNull();
    view.rerender(<ReviewChangesTab {...props({ onClear, queuedIntents: [coordinateIntent(model, "node:A", "0.5", 1)] })} />);
    expect(screen.getByRole("button", { name: "Discard all" })).toBeEnabled();
    fireEvent.click(screen.getByRole("button", { name: "Discard all" }));
    expect(onClear).toHaveBeenCalledOnce();
    view.rerender(<ReviewChangesTab {...props({ onClear, busy: true })} />);
    expect(screen.getByRole("button", { name: "Discard all" })).toBeEnabled();
  });
});

describe("the Record disclosure", () => {
  it("holds every moved line with its test id, closed by default", () => {
    const intent = coordinateIntent(model, "node:A", "0.5", 1);
    const receipt = { receipt_id: "applied-1-editor-intent-0", sequence: 1, operation_id: "op:earlier", change_id: "c", target_object_type: "Node", target_ref: "node:A",
      field_path: "position.y", before: "0", after: "1", application_route: "local_wasm_engine", applied_model_hash: "sha256:abc",
      acceptance: { acceptance_basis: "user_initiated_apply_in_local_session", acceptance_is_professional_approval: false, persistence_status: "session_state_only_not_yet_saved" },
      diagnostics: [], professional_boundary: {} } as AppliedOperationReceipt;
    const batch = { batch_id: "batch:applied", operations: [] } as unknown as OperationBatch;
    const batchReceipt = { batch, outcome: { batch_id: "batch:applied", mode: "apply", diagnostics: [], operation_outcomes: [],
      validation: { application_status: "applied_to_session_model" } } } as unknown as BatchReceipt;
    render(<ReviewChangesTab {...props({ queuedIntents: [intent], appliedOperations: [receipt], undoCount: 1, batchReceipts: [batchReceipt] })} />);
    const record = screen.getByTestId("review-record");
    expect(record.tagName).toBe("DETAILS");
    expect(record).not.toHaveAttribute("open");
    for (const [id, text] of [
      ["operation-apply-summary", "1 queued; 1 applied this session. Applying validates through the structured-operation seam"],
      ["operation-engine-chip", "Engine ready"],
      ["operation-unit-policy-chip", "1 unit-bearing queued, 0 dimensionless queued; no unit validations yet; 1 applied receipts."],
      ["session-history-chip", "1 undo / 0 redo"],
      ["operation-applied-ledger", "Applied this session"],
      ["applied-operation-route-applied-1-editor-intent-0", "Applied through local_wasm_engine. Acceptance basis user_initiated_apply_in_local_session"],
      ["batch-review-summary", "0 queued batches; 1 batches applied this session."],
      ["batch-receipt", "Applied batch: batch:applied"]
    ] as const) {
      expect(record, id).toContainElement(screen.getByTestId(id));
      expect(screen.getByTestId(id), id).toHaveTextContent(text);
    }
    expect(record).toHaveTextContent("Applying a structured operation records a user-initiated local-session acceptance only.");
  });
});

describe("batches", () => {
  it("appear only while one exists, with Validate and Apply entire batch", () => {
    const intent = coordinateIntent(model, "node:A", "0.5", 1);
    const entry: QueuedBatch = { key: "operation-batch-1", batch: { batch_id: "batch:one", operations: [intent] } as unknown as OperationBatch,
      basisModel: model, basisHash: { value: "sha256:basis" } as ModelHashEvidence, basisRevision: 1 };
    const onValidateBatch = vi.fn();
    const view = render(<ReviewChangesTab {...props()} />);
    expect(document.getElementById("batch-review")).toBeNull();
    view.rerender(<ReviewChangesTab {...props({ batches: [entry], onValidateBatch })} />);
    expect(screen.queryByTestId("operation-apply-empty")).toBeNull();
    fireEvent.click(screen.getByTestId("validate-batch-operation-batch-1"));
    expect(onValidateBatch).toHaveBeenCalledWith(entry);
    expect(screen.getByTestId("apply-batch-operation-batch-1")).toBeEnabled();
    expect(screen.getByRole("button", { name: "Discard all" })).toBeEnabled();
  });
});
