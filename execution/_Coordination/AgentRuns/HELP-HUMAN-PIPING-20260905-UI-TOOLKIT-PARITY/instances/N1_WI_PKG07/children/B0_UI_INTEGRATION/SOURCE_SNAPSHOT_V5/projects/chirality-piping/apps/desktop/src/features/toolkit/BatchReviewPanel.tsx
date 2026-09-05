import type { OperationBatch, OperationBatchOutcome } from "../../services/operationBatchService";
import type { ModelHashEvidence, PreviewModel } from "../../types";

export type QueuedBatch = {
  key: string;
  batch: OperationBatch;
  basisModel: PreviewModel;
  basisHash: ModelHashEvidence;
  basisRevision: number;
};
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
};

export function BatchReviewPanel({ batches, outcomes, receipts, revision, busy, message, onClear, onValidate, onApply }: Props) {
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
        return (
          <article key={entry.key} data-testid={`batch-${entry.key}`}>
            <h4>{entry.batch.batch_id}</h4>
            <p>{entry.batch.operations.length} proposed changes. {stale ? reason : "Bound to the model used to prepare this batch."}</p>
            <button type="button" data-testid={`validate-batch-${entry.key}`} disabled={stale || busy} title={reason} onClick={() => onValidate(entry)}>Validate batch</button>
            <button type="button" data-testid={`apply-batch-${entry.key}`} disabled={stale || busy} title={reason} onClick={() => onApply(entry)}>Apply entire batch</button>
            <details><summary>Submitted changes and source metadata</summary><pre>{JSON.stringify(entry.batch, null, 2)}</pre></details>
            {outcome ? <BatchOutcome outcome={outcome} /> : null}
          </article>
        );
      })}
      {receipts.map((receipt, index) => (
        <details key={`${index}:${receipt.batch.batch_id}`} data-testid="batch-receipt">
          <summary>Applied batch: {receipt.batch.batch_id}</summary>
          <BatchOutcome outcome={receipt.outcome} />
          <pre>{JSON.stringify({ submitted: receipt.batch, outcome: receipt.outcome }, null, 2)}</pre>
        </details>
      ))}
    </section>
  );
}

function BatchOutcome({ outcome }: { outcome: OperationBatchOutcome }) {
  return (
    <div>
      <p>{outcome.validation.application_status === "applied_to_session_model"
        ? "Applied as one batch. One session checkpoint was created."
        : outcome.simulation_disposition === "rolled_back_no_model_published"
          ? "Blocked. Temporary changes were rolled back; the model was not changed."
          : "Preview only. Temporary state was discarded; no change was applied or accepted."}</p>
      {outcome.diagnostics.map((item, index) => <p key={`${index}:${item.code}`}>{item.message} {item.remediation}</p>)}
      {outcome.operation_outcomes.map((step) => (
        <div key={step.index}>
          <h5>Temporary step {step.index + 1}: {step.target_ref}</h5>
          {step.diff_preview.map((diff, index) => <p key={index}>{diff.field_path}: {diff.before} → {diff.after}</p>)}
          {step.diagnostics.map((item, index) => <p key={`${index}:${item.code}`}>{item.message} {item.remediation}</p>)}
        </div>
      ))}
    </div>
  );
}
