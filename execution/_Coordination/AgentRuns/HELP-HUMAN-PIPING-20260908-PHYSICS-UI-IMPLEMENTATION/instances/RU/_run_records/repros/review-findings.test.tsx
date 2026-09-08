import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import React from "react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

import { App } from "../../../../../../../../apps/desktop/src/App";
import { loadPreviewModel } from "../../../../../../../../apps/desktop/src/services/previewService";
import type { EditorOperationIntent, PreviewModel } from "../../../../../../../../apps/desktop/src/types";
import { buildRouteSubmission, sameSubmission, type DraftSubmission, type RouteDraft } from "../../../../../../../../apps/desktop/src/features/viewport/routeDraft";

afterEach(() => {
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>((onResolve) => { resolve = onResolve; });
  return { promise, resolve };
}

function exactDiff(intent: EditorOperationIntent) {
  return [{
    entity_ref: intent.target.ref,
    object_type: intent.target.object_type,
    field_path: intent.change.field_path,
    before: intent.change.before,
    after: intent.change.after,
    unit: intent.change.unit,
    dimension: intent.change.dimension,
    change_kind: intent.change.change_kind
  }];
}

function validationFor(batch: { batch_id: string; operations: EditorOperationIntent[] }) {
  return {
    mode: "validate_only",
    batch_id: batch.batch_id,
    submitted_operations: structuredClone(batch.operations),
    validation: {
      schema_validation: "passed",
      batch_validation_status: "passed",
      diff_preview_status: "generated",
      application_status: "not_applied"
    },
    simulation_disposition: "validation_only_discarded",
    diagnostics: [],
    operation_outcomes: batch.operations.map((intent, index) => ({
      index,
      operation_id: intent.operation_id,
      change_id: intent.change.change_id,
      simulation_status: "validated_on_temporary_state",
      diff_preview: exactDiff(intent),
      diagnostics: []
    }))
  };
}

function singleValidation(intent: EditorOperationIntent) {
  return {
    mode: "validate_only",
    operation_id: intent.operation_id,
    change_id: intent.change.change_id,
    target_ref: intent.target.ref,
    validation: {
      schema_validation: "passed",
      diff_preview_status: "generated",
      application_status: "not_applied"
    },
    diff_preview: exactDiff(intent),
    diagnostics: []
  };
}

function appliedModelFor(basis: PreviewModel, operations: EditorOperationIntent[]) {
  const applied = structuredClone(basis);
  for (const intent of operations) {
    const record = JSON.parse(intent.change.after);
    if (intent.change.change_kind === "create_node") applied.nodes.push(record);
    if (intent.change.change_kind === "connect_pipe_run") applied.pipe_segments.push(record);
  }
  return applied;
}

async function fillExistingRoute(model: PreviewModel, id: string) {
  fireEvent.click(screen.getByTestId("command-pipe"));
  const panel = screen.getByTestId("viewport-editor-intents");
  const change = (field: string, value: string) => fireEvent.change(within(panel).getByTestId(field), { target: { value } });
  change("viewport-create-pipe-id", id);
  change("viewport-create-pipe-label", "Continuation repro");
  change("viewport-create-pipe-from", model.nodes[0].id);
  change("viewport-create-pipe-to", model.nodes[1].id);
  change("viewport-create-pipe-material", model.materials![0].id);
  change("viewport-create-pipe-od", "0.168");
  change("viewport-create-pipe-wall", "0.007");
  change("viewport-create-pipe-yref-x", "0");
  change("viewport-create-pipe-yref-y", "0");
  change("viewport-create-pipe-yref-z", "1");
  return panel;
}

async function fillNewRoute(model: PreviewModel, suffix: string) {
  fireEvent.click(screen.getByTestId("command-pipe"));
  const panel = screen.getByTestId("viewport-editor-intents");
  const change = (field: string, value: string) => fireEvent.change(within(panel).getByTestId(field), { target: { value } });
  change("viewport-create-pipe-id", `pipe:${suffix}`);
  change("viewport-create-pipe-label", "Cancel repro");
  change("viewport-create-pipe-from", model.nodes[0].id);
  fireEvent.click(within(panel).getByLabelText("New node"));
  change("viewport-route-end-id", `node:${suffix}`);
  change("viewport-route-end-label", "Canceled endpoint");
  change("viewport-route-end-x", "3.2");
  change("viewport-route-end-y", "0");
  change("viewport-route-end-z", "0");
  change("viewport-create-pipe-material", model.materials![0].id);
  change("viewport-create-pipe-od", "0.168");
  change("viewport-create-pipe-wall", "0.007");
  change("viewport-create-pipe-yref-x", "0");
  change("viewport-create-pipe-yref-y", "0");
  change("viewport-create-pipe-yref-z", "1");
  return panel;
}

describe("RU frozen U7 defect reproductions", () => {
  it("observes continuation cleared after a successful App commit", async () => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    const panel = await fillExistingRoute(model, "pipe:RU-continue");
    fireEvent.click(within(panel).getByTestId("continue-pipe-after-queue"));
    fireEvent.click(within(panel).getByTestId("queue-explicit-pipe-intent"));
    await waitFor(() => expect(within(panel).getByTestId("apply-reviewed-draft")).toBeEnabled());
    fireEvent.click(within(panel).getByTestId("apply-reviewed-draft"));
    await screen.findByTestId("tree-row-pipe:RU-continue");
    await waitFor(() => expect(within(panel).getByTestId("continue-pipe-after-queue")).not.toBeChecked());
    expect(within(panel).getByTestId("viewport-create-pipe-from")).toHaveValue("");
  });

  it("observes Cancel during delayed Apply still publishes the batch", async () => {
    const model = await loadPreviewModel();
    const apply = deferred<unknown>();
    let submittedBatch: { batch_id: string; operations: EditorOperationIntent[] } | null = null;
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    const panel = await fillNewRoute(model, "RU-cancel-apply");
    invokeMock.mockImplementation((command: string, args: { batch: { batch_id: string; operations: EditorOperationIntent[] } }) => {
      if (command === "validate_model_operation_batch") {
        submittedBatch = structuredClone(args.batch);
        return Promise.resolve(validationFor(args.batch));
      }
      if (command === "apply_model_operation_batch") return apply.promise;
      return Promise.reject(new Error(command));
    });
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    fireEvent.click(within(panel).getByTestId("queue-explicit-pipe-intent"));
    await waitFor(() => expect(within(panel).getByTestId("apply-reviewed-draft")).toBeEnabled());
    fireEvent.click(within(panel).getByTestId("apply-reviewed-draft"));
    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("apply_model_operation_batch", expect.any(Object)));
    fireEvent.click(within(panel).getByTestId("cancel-pipe-draft"));
    const batch = submittedBatch!;
    await act(async () => {
      const applied = validationFor(batch);
      apply.resolve({
        ...applied,
        mode: "apply",
        batch_id: batch.batch_id,
        validation: { ...applied.validation, application_status: "applied_to_session_model" },
        simulation_disposition: "committed_as_one_batch",
        applied_model: appliedModelFor(model, batch.operations),
        applied_model_backend_hash: "sha256:invented",
        acceptance: { acceptance_basis: "user_initiated_apply_in_local_session", acceptance_is_professional_approval: false, persistence_status: "session_only_until_saved" },
        diagnostics: [],
        application_route: "local_wasm_engine",
        professional_boundary: {}
      });
      await apply.promise;
    });
    expect(await screen.findByTestId("tree-row-node:RU-cancel-apply")).toBeInTheDocument();
    expect(screen.getByTestId("tree-row-pipe:RU-cancel-apply")).toBeInTheDocument();
    expect(screen.getByTestId("session-history-chip")).toHaveTextContent("1 undo / 0 redo");
  });

  it("observes a single reviewed Apply publish with an incomplete acceptance receipt", async () => {
    const model = await loadPreviewModel();
    let submitted: EditorOperationIntent | null = null;
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    fireEvent.click(screen.getByTestId("command-node"));
    const panel = screen.getByTestId("viewport-editor-intents");
    const change = (field: string, value: string) => fireEvent.change(within(panel).getByTestId(field), { target: { value } });
    change("viewport-create-node-id", "node:RU-incomplete-receipt");
    change("viewport-create-node-label", "Incomplete receipt repro");
    change("viewport-create-node-x", "0");
    change("viewport-create-node-y", "0");
    change("viewport-create-node-z", "0");
    invokeMock.mockImplementation((command: string, args: { intent: EditorOperationIntent }) => {
      if (command === "validate_model_operation") {
        submitted = structuredClone(args.intent);
        return Promise.resolve(singleValidation(args.intent));
      }
      if (command === "apply_model_operation") {
        const intent = args.intent;
        const applied = structuredClone(model);
        applied.nodes.push(JSON.parse(intent.change.after));
        return Promise.resolve({
          mode: "apply",
          operation_id: intent.operation_id,
          change_id: intent.change.change_id,
          target_ref: intent.target.ref,
          target_object_type: intent.target.object_type,
          change_kind: intent.change.change_kind,
          validation: { application_status: "applied_to_session_model" },
          applied_model: applied,
          applied_model_backend_hash: "sha256:invented",
          acceptance: {},
          diagnostics: [],
          application_route: "local_wasm_engine",
          professional_boundary: {}
        });
      }
      return Promise.reject(new Error(command));
    });
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    fireEvent.click(within(panel).getByTestId("queue-explicit-node-intent"));
    await waitFor(() => expect(within(panel).getByTestId("apply-reviewed-draft")).toBeEnabled());
    fireEvent.click(within(panel).getByTestId("apply-reviewed-draft"));
    expect(submitted).not.toBeNull();
    expect(await screen.findByTestId("tree-row-node:RU-incomplete-receipt")).toBeInTheDocument();
    expect(screen.getByTestId("operation-applied-ledger")).toHaveTextContent("Acceptance basis ; persistence");
  });

  it("observes malformed create metadata with no object type reserve an ID", async () => {
    const model = await loadPreviewModel();
    const draft: RouteDraft = {
      startNodeId: model.nodes[0].id,
      endMode: "existing",
      existingEndNodeId: model.nodes[1].id,
      newEnd: { id: "node:unused", label: "Unused", x: "0", y: "0", z: "0", coordinateUnit: "m", provenance: "invented" },
      pipe: { id: "pipe:RU-malformed-reservation", label: "Reservation repro", materialId: model.materials![0].id, outsideDiameter: "0.168", wallThickness: "0.007", lengthUnit: "m", yReferenceX: "0", yReferenceY: "0", yReferenceZ: "1", provenance: "invented" }
    };
    const malformed = [{ operation_kind: "create", target: { ref: draft.pipe.id } }];
    const built = buildRouteSubmission(model, malformed, draft, 1, "length=passed");
    expect(built.ok).toBe(false);
    if (built.ok) throw new Error("malformed reservation unexpectedly ignored");
    expect(built.errors).toContain("The pipe ID is already reserved.");
  });

  it("observes the new node and pipe in one atomic route may reuse the same ID", async () => {
    const model = await loadPreviewModel();
    const sharedId = "entity:RU-same-id";
    const draft: RouteDraft = {
      startNodeId: model.nodes[0].id,
      endMode: "new",
      existingEndNodeId: "",
      newEnd: { id: sharedId, label: "New endpoint", x: "1", y: "0", z: "0", coordinateUnit: "m", provenance: "invented" },
      pipe: { id: sharedId, label: "Same-ID pipe", materialId: model.materials![0].id, outsideDiameter: "0.168", wallThickness: "0.007", lengthUnit: "m", yReferenceX: "0", yReferenceY: "0", yReferenceZ: "1", provenance: "invented" }
    };
    const built = buildRouteSubmission(model, [], draft, 1, "length=passed");
    expect(built.ok).toBe(true);
    if (!built.ok || built.submission.kind !== "batch") throw new Error("expected accepted same-ID batch");
    expect(built.submission.batch.operations.map((intent) => intent.target.ref)).toEqual([sharedId, sharedId]);
  });

  it("independently confirms object-key order is ignored while array order and values remain exact", () => {
    const left = { kind: "batch", batch: { batch_id: "b", operations: [{ operation_id: "one" }, { operation_id: "two" }] } } as DraftSubmission;
    const reorderedKeys = { batch: { operations: [{ operation_id: "one" }, { operation_id: "two" }], batch_id: "b" }, kind: "batch" } as DraftSubmission;
    const reversedArray = { batch: { operations: [{ operation_id: "two" }, { operation_id: "one" }], batch_id: "b" }, kind: "batch" } as DraftSubmission;
    const changedValue = { batch: { operations: [{ operation_id: "one" }, { operation_id: "changed" }], batch_id: "b" }, kind: "batch" } as DraftSubmission;
    expect(sameSubmission(left, reorderedKeys)).toBe(true);
    expect(sameSubmission(left, reversedArray)).toBe(false);
    expect(sameSubmission(left, changedValue)).toBe(false);
  });
});
