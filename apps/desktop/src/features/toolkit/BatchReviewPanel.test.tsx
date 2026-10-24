import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { BatchReviewPanel, batchRequiresPreview, previewSatisfied, PREVIEW_REQUIRED_REASON, type QueuedBatch } from "./BatchReviewPanel";
import type { OperationBatch, OperationBatchOutcome } from "../../services/operationBatchService";
import type { EditorOperationIntent, ModelHashEvidence, PreviewModel } from "../../types";

afterEach(cleanup);
const hash = { value: "sha256:basis" } as ModelHashEvidence;
const intent = (kind: string) => ({ operation_id: `op:${kind}`, change: { change_kind: kind } }) as unknown as EditorOperationIntent;
function entry(key: string, kinds: string[], revision = 1): QueuedBatch {
  const batch: OperationBatch = { batch_id: `batch:${key}`, operations: kinds.map(intent) };
  return { key, batch, basisModel: {} as PreviewModel, basisHash: hash, basisRevision: revision, requiresPreview: batchRequiresPreview(batch) };
}
function outcome(batchId: string, over: { mode?: "validate_only" | "apply"; blocking?: boolean; hashValue?: string } = {}): OperationBatchOutcome {
  const diagnostics = [{ id: "d1", code: over.blocking ? "OP-RECONNECT-COMPONENT-ATTACHED" : "OP-RECONNECT-NODE-ORPHANED", severity: over.blocking ? "blocking" : "warning",
    message: "Cannot reconnect pipe:P: component component:tee (tee) sits at node node:B, where this reconnect removes the pipe.",
    remediation: "Delete or re-author component:tee, reconnect, then author the component again.", affected_refs: ["pipe:P", "component:tee", "node:B"], source: "engine" }];
  return {
    batch_id: batchId, mode: over.mode ?? "validate_only", diagnostics: [],
    validation: { schema_validation: "passed", batch_validation_status: over.blocking ? "blocked" : "passed", diff_preview_status: "generated", application_status: "not_applied" },
    simulation_disposition: over.blocking ? "rolled_back_no_model_published" : "validation_only_discarded",
    operation_outcomes: [{ index: 0, operation_id: "op:reconnect_pipe_run", change_id: "c", target_object_type: "Element", target_ref: "pipe:P", change_kind: "reconnect_pipe_run",
      validation: {} as never, diff_preview: [], diagnostics, simulation_status: over.blocking ? "blocked" : "validated_on_temporary_state" }],
    initial_model_hash: { value: over.hashValue ?? hash.value } as ModelHashEvidence
  } as unknown as OperationBatchOutcome;
}
function renderPanel(batches: QueuedBatch[], outcomes: Record<string, OperationBatchOutcome>, revision = 1, onSelectRef = vi.fn(), onApply = vi.fn()) {
  render(<BatchReviewPanel batches={batches} outcomes={outcomes} receipts={[]} revision={revision} busy={false} message={null}
    onClear={vi.fn()} onValidate={vi.fn()} onApply={onApply} onSelectRef={onSelectRef} />);
  return { onSelectRef, onApply };
}
const apply = (key: string) => screen.getByTestId(`apply-batch-${key}`);

describe("reconnect preview gate in batch review", () => {
  it("disables Apply for a reconnect batch before Validate and enables it after a clean Validate", () => {
    const reconnect = entry("r", ["create_node", "reconnect_pipe_run"]);
    expect(reconnect.requiresPreview).toBe(true);
    renderPanel([reconnect], {});
    expect(apply("r")).toBeDisabled();
    expect(apply("r")).toHaveAttribute("title", PREVIEW_REQUIRED_REASON);
    expect(screen.getByTestId("preview-required-r")).toHaveTextContent("Validate it on the current model first");
    cleanup();
    const { onApply } = renderPanel([reconnect], { r: outcome("batch:r") });
    expect(apply("r")).toBeEnabled();
    fireEvent.click(apply("r"));
    expect(onApply).toHaveBeenCalledOnce();
  });
  it("keeps Apply disabled after a blocking Validate, an apply outcome, another basis, or a model change", () => {
    const reconnect = entry("r", ["reconnect_pipe_run"]);
    for (const [label, result, revision] of [
      ["blocking", outcome("batch:r", { blocking: true }), 1],
      ["apply-mode", outcome("batch:r", { mode: "apply" }), 1],
      ["other basis", outcome("batch:r", { hashValue: "sha256:other" }), 1],
      ["other batch", outcome("batch:other"), 1],
      ["model changed", outcome("batch:r"), 2]
    ] as const) {
      expect(previewSatisfied(reconnect, result, revision), label).toBe(false);
      renderPanel([reconnect], { r: result }, revision);
      expect(apply("r"), label).toBeDisabled();
      cleanup();
    }
  });
  it("leaves batches without a reconnect unchanged", () => {
    const other = entry("o", ["set_field", "split_pipe_run"]);
    expect(other.requiresPreview).toBe(false);
    expect(previewSatisfied(other, undefined, 1)).toBe(true);
    renderPanel([other], {});
    expect(apply("o")).toBeEnabled();
    expect(screen.queryByTestId("preview-required-o")).toBeNull();
  });
  it("shows each diagnostic's code and selectable affected refs", () => {
    const { onSelectRef } = renderPanel([entry("r", ["reconnect_pipe_run"])], { r: outcome("batch:r", { blocking: true }) });
    const shown = screen.getByTestId("batch-diagnostic-OP-RECONNECT-COMPONENT-ATTACHED");
    expect(shown).toHaveTextContent("OP-RECONNECT-COMPONENT-ATTACHED (blocking): Cannot reconnect pipe:P: component component:tee (tee) sits at node node:B");
    expect(shown).toHaveTextContent("Affected: pipe:P, component:tee, node:B");
    fireEvent.click(within(shown).getByRole("button", { name: "component:tee" }));
    expect(onSelectRef).toHaveBeenCalledWith("component:tee");
  });
});
