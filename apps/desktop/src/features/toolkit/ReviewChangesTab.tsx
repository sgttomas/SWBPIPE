import { CheckCheck, Layers, Trash2 } from "lucide-react";
import { useEffect, useRef, useState } from "react";
import type { OperationBatchOutcome } from "../../services/operationBatchService";
import type { OperationEngineStatus } from "../../services/operationService";
import type { AppliedOperationReceipt, EditorOperationIntent, OperationOutcome, PreviewModel } from "../../types";
import { batchableCount, intentKey, OperationApplyRecord, PendingChangeRows } from "../operations/OperationApplyPanel";
import { BatchReviewPanel, BatchReviewRecord, type BatchReceipt, type QueuedBatch } from "./BatchReviewPanel";

type Props = {
  batches: QueuedBatch[];
  batchOutcomes: Record<string, OperationBatchOutcome>;
  batchReceipts: BatchReceipt[];
  revision: number;
  busy: boolean;
  batchMessage: string | null;
  onClear: () => void;
  onValidateBatch: (entry: QueuedBatch) => void;
  onApplyBatch: (entry: QueuedBatch) => void;
  onDismissBatch?: (entry: QueuedBatch) => void;
  onSelectRef?: (ref: string) => void;
  /** The working model, for naming each change's entity as the tree names it. */
  model: PreviewModel | null;
  /** True while a solved result is shown that applying a change would clear. */
  hasResults: boolean;
  queuedIntents: EditorOperationIntent[];
  outcomes: Record<string, OperationOutcome>;
  appliedOperations: AppliedOperationReceipt[];
  undoCount: number;
  redoCount: number;
  message: string | null;
  engineStatus: OperationEngineStatus;
  onValidate: (intent: EditorOperationIntent) => void;
  /** The single-change apply; resolves false when the change was not applied. */
  onApply: (intent: EditorOperationIntent) => Promise<boolean> | void;
  /** Removes one queued change; each row offers Remove only when given. */
  onRemove?: (key: string) => void;
  /** G9 (DEL-07-02 REQ-017): moves the person's queued changes into one batch. */
  onReviewAllAsBatch?: () => void;
};

/**
 * The Review changes tab of the Model stage's operations strip. At rest it reads
 * one line. With changes queued it lists them, one row each in words, under
 * Apply all and Discard all; batches appear only while one exists; the session's
 * counts, receipts and acceptance boundary are in the closed "Record" at the foot.
 */
export function ReviewChangesTab(props: Props) {
  const { batches, queuedIntents, busy, engineStatus, hasResults } = props;
  const applyAll = useApplyAll(queuedIntents, busy, props.onApply);
  const pending = queuedIntents.length > 0 || batches.length > 0;
  const batchable = batchableCount(queuedIntents);
  return (
    <section className="panel operation-apply-panel review-changes" aria-label="Review changes" data-testid="operation-apply-panel">
      {engineStatus.state === "loading" ? (
        <p className="review-changes-status" role="status" data-testid="review-engine-not-ready">
          The editing engine is still loading. Apply waits until it is ready.
        </p>
      ) : engineStatus.state === "unavailable" ? (
        <p className="review-changes-status" role="alert" data-testid="review-engine-not-ready">
          Changes cannot be checked or applied: the editing engine did not load ({engineStatus.detail}).
        </p>
      ) : null}
      {busy ? (
        <p className="review-changes-status" role="status" data-testid="review-busy">
          Working on a change. The buttons return when it finishes.
        </p>
      ) : null}
      {props.message ? <p className="review-changes-status" role="status" data-testid="operation-apply-message">{props.message}</p> : null}
      {props.batchMessage ? <p className="review-changes-status" role="status" data-testid="batch-review-message">{props.batchMessage}</p> : null}
      {!pending ? <p className="review-changes-empty" data-testid="operation-apply-empty">No pending changes.</p> : null}
      {/* Discard all keeps its element at rest, hidden and disabled as the clear was with nothing to clear;
          while a request runs it stays offered, as before, to withdraw it. */}
      <div className="review-changes-actions" role="group" aria-label="All pending changes" hidden={!pending && !busy}>
          {queuedIntents.length > 0 ? (
            <button
              type="button"
              className="review-apply-all"
              data-testid="apply-all-intents"
              disabled={busy || applyAll.running}
              onClick={applyAll.start}
              title="Apply each change in the order it was queued, stopping at the first that cannot be applied"
            >
              <CheckCheck size={14} aria-hidden="true" />
              {applyAll.running ? "Applying…" : "Apply all"}
            </button>
          ) : null}
          <button
            type="button"
            data-testid="clear-pending-batches"
            disabled={!pending && !busy}
            onClick={() => { applyAll.stop(); props.onClear(); }}
            title="Remove every pending change and batch without applying any"
          >
            <Trash2 size={14} aria-hidden="true" />
            Discard all
          </button>
          {props.onReviewAllAsBatch && batchable >= 2 ? (
            <span className="operation-batch-all" data-testid="operation-review-all-as-batch">
              <button
                aria-describedby="operation-review-all-as-batch-help"
                type="button"
                data-testid="review-all-as-batch"
                disabled={busy || applyAll.running}
                onClick={props.onReviewAllAsBatch}
              >
                <Layers size={14} aria-hidden="true" />
                Review all {batchable} queued changes as one batch
              </button>
              <small hidden id="operation-review-all-as-batch-help">
                The changes move to batch review in the order they were queued. Apply entire batch validates and
                applies them all or none, as one undo step; Validate batch checks them first without applying. A
                change whose before-value depends on another queued change is refused by name, never adjusted.
              </small>
            </span>
          ) : null}
          {hasResults && pending ? <p className="review-changes-note" data-testid="apply-clears-results">Applying a change clears the current results.</p> : null}
      </div>
      {queuedIntents.length > 0 ? (
        <PendingChangeRows
          queuedIntents={queuedIntents}
          outcomes={props.outcomes}
          model={props.model}
          busy={busy || applyAll.running}
          onValidate={props.onValidate}
          onApply={props.onApply}
          onRemove={props.onRemove}
        />
      ) : null}
      <BatchReviewPanel
        batches={batches} outcomes={props.batchOutcomes}
        revision={props.revision} busy={busy}
        onValidate={props.onValidateBatch}
        onApply={props.onApplyBatch}
        onDismiss={props.onDismissBatch}
        onSelectRef={props.onSelectRef}
      />
      <details className="review-changes-record" data-testid="review-record">
        <summary>Record</summary>
        <OperationApplyRecord
          queuedIntents={queuedIntents}
          outcomes={props.outcomes}
          appliedOperations={props.appliedOperations}
          undoCount={props.undoCount}
          redoCount={props.redoCount}
          engineStatus={engineStatus}
        />
        <BatchReviewRecord batches={batches} receipts={props.batchReceipts} onSelectRef={props.onSelectRef} />
      </details>
    </section>
  );
}

/**
 * Apply all: the row's own Apply for each queued change in turn, oldest first
 * (queue order). Each starts only after the previous one has finished and has
 * left the list, so it runs on the model that one published; the run stops at
 * the first change that is not applied. It adds no path of its own: every step
 * is the single-change apply a row's button runs.
 */
function useApplyAll(queuedIntents: EditorOperationIntent[], busy: boolean, onApply: Props["onApply"]) {
  const [run, setRun] = useState<{ tried: Set<string> } | null>(null);
  const [step, setStep] = useState(0);
  const inFlight = useRef(false);
  const awaiting = useRef<string | null>(null);
  useEffect(() => {
    if (!run || busy || inFlight.current) return;
    if (awaiting.current && queuedIntents.some((intent) => intentKey(intent) === awaiting.current)) return;
    awaiting.current = null;
    const next = [...queuedIntents].reverse().find((intent) => !run.tried.has(intentKey(intent)));
    if (!next) {
      setRun(null);
      return;
    }
    const key = intentKey(next);
    run.tried.add(key);
    inFlight.current = true;
    Promise.resolve(onApply(next)).then(
      (applied) => {
        inFlight.current = false;
        if (applied === false) {
          setRun(null);
        } else {
          awaiting.current = key;
          setStep((value) => value + 1);
        }
      },
      () => {
        inFlight.current = false;
        setRun(null);
      }
    );
  }, [run, step, busy, queuedIntents, onApply]);
  return {
    running: run !== null,
    start: () => { awaiting.current = null; setRun({ tried: new Set() }); },
    stop: () => { awaiting.current = null; setRun(null); }
  };
}
