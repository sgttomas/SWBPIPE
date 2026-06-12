import { CheckCheck, PlayCircle, Redo2, SearchCheck, Undo2 } from "lucide-react";
import type { OperationEngineStatus } from "../../services/operationService";
import type { AppliedOperationReceipt, EditorOperationIntent, OperationOutcome } from "../../types";

export function OperationApplyPanel({
  queuedIntents,
  outcomes,
  appliedOperations,
  undoCount,
  redoCount,
  busy,
  message,
  engineStatus,
  onValidate,
  onApply,
  onUndo,
  onRedo
}: {
  queuedIntents: EditorOperationIntent[];
  outcomes: Record<string, OperationOutcome>;
  appliedOperations: AppliedOperationReceipt[];
  undoCount: number;
  redoCount: number;
  busy: boolean;
  message: string | null;
  engineStatus: OperationEngineStatus;
  onValidate: (intent: EditorOperationIntent) => void;
  onApply: (intent: EditorOperationIntent) => void;
  onUndo: () => void;
  onRedo: () => void;
}) {
  return (
    <section className="panel operation-apply-panel" aria-label="Apply structured operations" data-testid="operation-apply-panel">
      <div className="panel-title">
        <CheckCheck size={16} aria-hidden="true" />
        Apply Operations
      </div>
      <p className="muted" data-testid="operation-apply-summary">
        {queuedIntents.length} queued; {appliedOperations.length} applied this session. Applying validates through the
        structured-operation seam and replaces the session model document; saving the project stores it locally.
      </p>
      <small className="muted" data-testid="operation-engine-status">
        engine_route={engineStatus.route}; engine_state={engineStatus.state}
        {engineStatus.detail ? `; ${engineStatus.detail}` : ""}
      </small>
      <div className="operation-history-actions" aria-label="Session edit history">
        <button
          data-testid="undo-session-model-edit"
          disabled={busy || undoCount === 0}
          onClick={onUndo}
          title="Undo last local session model edit"
          type="button"
        >
          <Undo2 size={14} aria-hidden="true" />
          Undo
        </button>
        <button
          data-testid="redo-session-model-edit"
          disabled={busy || redoCount === 0}
          onClick={onRedo}
          title="Redo last undone local session model edit"
          type="button"
        >
          <Redo2 size={14} aria-hidden="true" />
          Redo
        </button>
        <small data-testid="session-history-summary">
          undo={undoCount}; redo={redoCount}; local_session_only=true; saved_project_mutated=false
        </small>
      </div>
      {message ? (
        <p className="muted" data-testid="operation-apply-message">
          {message}
        </p>
      ) : null}
      {queuedIntents.length === 0 ? (
        <p className="muted" data-testid="operation-apply-empty">
          Queue a review intent from the property inspector or viewport to validate and apply it here.
        </p>
      ) : (
        <div className="operation-record-list" data-testid="operation-apply-queue">
          {queuedIntents.map((intent) => {
            const key = intentKey(intent);
            const outcome = outcomes[key] ?? null;
            return (
              <article className="operation-record" data-testid={`operation-apply-row-${key}`} key={key}>
                <strong>{intent.operation_id}</strong>
                <small>
                  {intent.target.object_type} {intent.target.ref}; {intent.change.field_path}:{" "}
                  {intent.change.before} to {intent.change.after}
                </small>
                <div className="operation-apply-actions">
                  <button
                    data-testid={`validate-intent-${key}`}
                    disabled={busy}
                    onClick={() => onValidate(intent)}
                    title="Validate without applying"
                    type="button"
                  >
                    <SearchCheck size={14} aria-hidden="true" />
                    Validate
                  </button>
                  <button
                    data-testid={`apply-intent-${key}`}
                    disabled={busy}
                    onClick={() => onApply(intent)}
                    title="Validate and apply to the session model"
                    type="button"
                  >
                    <PlayCircle size={14} aria-hidden="true" />
                    Apply
                  </button>
                </div>
                {outcome ? <OutcomeSummary outcome={outcome} testKey={key} /> : null}
              </article>
            );
          })}
        </div>
      )}
      {appliedOperations.length > 0 ? (
        <div className="operation-record-list" data-testid="operation-applied-ledger">
          <div className="panel-subtitle">Applied this session</div>
          {appliedOperations.map((receipt) => (
            <article className="operation-record" data-testid={`applied-operation-${receipt.receipt_id}`} key={receipt.receipt_id}>
              <strong>{receipt.operation_id}</strong>
              <small>
                {receipt.target_object_type} {receipt.target_ref}; {receipt.field_path}: {receipt.before} to{" "}
                {receipt.after}
              </small>
              <small data-testid={`applied-operation-route-${receipt.receipt_id}`}>
                route={receipt.application_route}; acceptance={receipt.acceptance.acceptance_basis};{" "}
                persistence={receipt.acceptance.persistence_status}; professional_approval=
                {String(receipt.acceptance.acceptance_is_professional_approval)}
              </small>
              <small>model_hash={shortHash(receipt.applied_model_hash)}</small>
            </article>
          ))}
        </div>
      ) : null}
      <small className="report-note">
        Applying a structured operation records a user-initiated local-session acceptance only. It is not professional
        approval, certification, sealing, authentication, or a code-compliance claim; results of earlier solves are
        cleared because they no longer describe the edited model.
      </small>
    </section>
  );
}

function OutcomeSummary({ outcome, testKey }: { outcome: OperationOutcome; testKey: string }) {
  return (
    <div className="operation-outcome" data-testid={`operation-outcome-${testKey}`}>
      <small data-testid={`operation-outcome-status-${testKey}`}>
        {outcome.mode}; application_status={outcome.validation.application_status}; schema=
        {outcome.validation.schema_validation}; unit={outcome.validation.unit_validation}; before_state=
        {outcome.validation.before_state_validation}; route={outcome.application_route}
      </small>
      {outcome.diff_preview.map((row) => (
        <small data-testid={`operation-outcome-diff-${testKey}`} key={`${row.field_path}-${row.after}`}>
          diff: {row.entity_ref} {row.field_path} {row.before} to {row.after} [{row.unit}]
        </small>
      ))}
      {outcome.diagnostics.map((diagnostic) => (
        <small data-testid={`operation-outcome-diagnostic-${diagnostic.code}`} key={diagnostic.id}>
          {diagnostic.severity}: {diagnostic.code} — {diagnostic.message}
        </small>
      ))}
    </div>
  );
}

export function intentKey(intent: EditorOperationIntent): string {
  return intent.queue_id ?? intent.operation_id;
}

function shortHash(value: string): string {
  return value.length > 24 ? `${value.slice(0, 24)}…` : value;
}
