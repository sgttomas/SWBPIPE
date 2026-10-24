import { Fragment } from "react";
import type { OperationBatch, OperationBatchOutcome } from "../../services/operationBatchService";
import type { ModelHashEvidence, OperationOutcomeDiagnostic, PreviewModel } from "../../types";

export type QueuedBatch = {
  key: string;
  batch: OperationBatch;
  basisModel: PreviewModel;
  basisHash: ModelHashEvidence;
  basisRevision: number;
  /** Desktop-side only (never on the batch wire): set centrally when any
   * submitted operation is a `reconnect_pipe_run` (W-10). Such a batch is
   * applied only after a clean validation-only preview on the same basis. */
  requiresPreview?: boolean;
};
/** True when any submitted operation is a reconnect (DEL-16-06 Design `pipe-reconnect.md` §3). */
export function batchRequiresPreview(batch: OperationBatch): boolean {
  return Array.isArray(batch.operations) && batch.operations.some((operation) => operation?.change?.change_kind === "reconnect_pipe_run");
}
const blocking = (items: readonly OperationOutcomeDiagnostic[] | undefined) => (items ?? []).some((item) => item.severity === "blocking");
/** The one preview gate, checked by the Apply button and the apply path alike. */
export function previewSatisfied(entry: QueuedBatch, outcome: OperationBatchOutcome | undefined, revision: number): boolean {
  // Derived from the batch's operations; the flag can only add the requirement.
  if (!entry.requiresPreview && !batchRequiresPreview(entry.batch)) return true;
  return Boolean(outcome)
    && outcome!.mode === "validate_only"
    && outcome!.batch_id === entry.batch.batch_id
    && outcome!.validation.batch_validation_status === "passed"
    && !blocking(outcome!.diagnostics)
    && !outcome!.operation_outcomes.some((step) => blocking(step.diagnostics))
    && outcome!.initial_model_hash?.value === entry.basisHash.value
    && entry.basisRevision === revision;
}
export const PREVIEW_REQUIRED_REASON = "This batch contains a reconnect. Validate it on the current model first; Apply is enabled only after a preview with no blocking finding.";
export type BatchReceipt = { batch: OperationBatch; outcome: OperationBatchOutcome };
type Props = {
  batches: QueuedBatch[];
  outcomes: Record<string, OperationBatchOutcome>;
  receipts: BatchReceipt[];
  revision: number;
  busy: boolean;
  message: string | null;
  onClear: () => void;
  onValidate: (entry: QueuedBatch) => void;
  onApply: (entry: QueuedBatch) => void;
  /** Select an affected ref through the existing selection. */
  onSelectRef?: (ref: string) => void;
};

export function BatchReviewPanel({ batches, outcomes, receipts, revision, busy, message, onClear, onValidate, onApply, onSelectRef }: Props) {
  return (
    <section className="panel" id="batch-review" tabIndex={-1} aria-label="Batch review">
      <h3>Batch review</h3>
      <p data-testid="batch-review-summary">{batches.length} queued batches; {receipts.length} batches applied this session.</p>
      <p>Validate to preview all steps on temporary state. Apply commits the entire batch once or leaves the model unchanged. Batch drafts and receipts shown here belong to this session.</p>
      <button
        type="button"
        data-testid="clear-pending-batches"
        onClick={onClear}
        disabled={batches.length === 0 && !busy}
        title={batches.length === 0 && !busy ? "No pending batches or requests to clear." : undefined}
      >
        Clear pending changes and batches
      </button>
      {message ? <p role="status" data-testid="batch-review-message">{message}</p> : null}
      {batches.map((entry) => {
        const outcome = outcomes[entry.key];
        const stale = entry.basisRevision !== revision;
        const reason = stale ? "The model changed. Prepare a new batch from the current model." : busy ? "Wait for the current request to finish." : undefined;
        const gated = !previewSatisfied(entry, outcome, revision);
        const applyReason = reason ?? (gated ? PREVIEW_REQUIRED_REASON : undefined);
        return (
          <article key={entry.key} data-testid={`batch-${entry.key}`}>
            <h4>{entry.batch.batch_id}</h4>
            <p>{entry.batch.operations.length} proposed changes. {stale ? reason : "Bound to the model used to prepare this batch."}</p>
            <button type="button" data-testid={`validate-batch-${entry.key}`} disabled={stale || busy} title={reason} onClick={() => onValidate(entry)}>Validate batch</button>
            <button type="button" data-testid={`apply-batch-${entry.key}`} disabled={stale || busy || gated} title={applyReason} onClick={() => onApply(entry)}>Apply entire batch</button>
            {gated && !stale ? <p data-testid={`preview-required-${entry.key}`}>{PREVIEW_REQUIRED_REASON}</p> : null}
            <details><summary>Submitted changes and source metadata</summary><pre>{JSON.stringify(entry.batch, null, 2)}</pre></details>
            {outcome ? <BatchOutcome outcome={outcome} onSelectRef={onSelectRef} /> : null}
          </article>
        );
      })}
      {receipts.map((receipt, index) => (
        <details key={`${index}:${receipt.batch.batch_id}`} data-testid="batch-receipt">
          <summary>Applied batch: {receipt.batch.batch_id}</summary>
          <BatchOutcome outcome={receipt.outcome} onSelectRef={onSelectRef} />
          <pre>{JSON.stringify({ submitted: receipt.batch, outcome: receipt.outcome }, null, 2)}</pre>
        </details>
      ))}
    </section>
  );
}

function Diagnostic({ item, onSelectRef }: { item: OperationOutcomeDiagnostic; onSelectRef?: (ref: string) => void }) {
  return <div data-testid={`batch-diagnostic-${item.code}`}>
    <p><code>{item.code}</code> ({item.severity}): {item.message} {item.remediation}</p>
    {item.affected_refs?.length ? <p>Affected: {item.affected_refs.map((ref, index) => <Fragment key={`${index}:${ref}`}>
      {index > 0 ? ", " : null}
      {onSelectRef ? <button type="button" className="link-button" onClick={() => onSelectRef(ref)}>{ref}</button> : <code>{ref}</code>}
    </Fragment>)}</p> : null}
  </div>;
}

function BatchOutcome({ outcome, onSelectRef }: { outcome: OperationBatchOutcome; onSelectRef?: (ref: string) => void }) {
  return (
    <div>
      <p>{outcome.validation.application_status === "applied_to_session_model"
        ? "Applied as one batch. One session checkpoint was created."
        : outcome.simulation_disposition === "rolled_back_no_model_published"
          ? "Blocked. Temporary changes were rolled back; the model was not changed."
          : "Preview only. Temporary state was discarded; no change was applied or accepted."}</p>
      {outcome.diagnostics.map((item, index) => <Diagnostic key={`${index}:${item.code}`} item={item} onSelectRef={onSelectRef} />)}
      {outcome.operation_outcomes.map((step) => (
        <div key={step.index}>
          <h5>Temporary step {step.index + 1}: {step.target_ref}</h5>
          {step.diff_preview.map((diff, index) => <p key={index}>{diff.field_path}: {diff.before} → {diff.after}</p>)}
          {step.diagnostics.map((item, index) => <Diagnostic key={`${index}:${item.code}`} item={item} onSelectRef={onSelectRef} />)}
        </div>
      ))}
    </div>
  );
}
