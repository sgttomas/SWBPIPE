import type { OperationBatchOutcome } from "../../services/operationBatchService";
import type { OperationEngineStatus } from "../../services/operationService";
import type { AppliedOperationReceipt, EditorOperationIntent, OperationOutcome } from "../../types";
import { OperationApplyPanel } from "../operations/OperationApplyPanel";
import { BatchReviewPanel, type BatchReceipt, type QueuedBatch } from "./BatchReviewPanel";

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
  onDismissBatch: (entry: QueuedBatch) => void;
  onSelectRef: (ref: string) => void;
  queuedIntents: EditorOperationIntent[];
  outcomes: Record<string, OperationOutcome>;
  appliedOperations: AppliedOperationReceipt[];
  undoCount: number;
  redoCount: number;
  message: string | null;
  engineStatus: OperationEngineStatus;
  onValidate: (intent: EditorOperationIntent) => void;
  onApply: (intent: EditorOperationIntent) => void;
  onReviewAllAsBatch: () => void;
  onUndo: () => void;
  onRedo: () => void;
};

/** The Review changes tab of the Model stage's operations strip. */
export function ReviewChangesTab(props: Props) {
  return (
    <>
              <BatchReviewPanel
                onClear={props.onClear}
                batches={props.batches} outcomes={props.batchOutcomes} receipts={props.batchReceipts}
                revision={props.revision} busy={props.busy} message={props.batchMessage}
                onValidate={props.onValidateBatch}
                onApply={props.onApplyBatch}
                onDismiss={props.onDismissBatch}
                onSelectRef={props.onSelectRef}
              />
              <OperationApplyPanel
                queuedIntents={props.queuedIntents}
                outcomes={props.outcomes}
                appliedOperations={props.appliedOperations}
                undoCount={props.undoCount}
                redoCount={props.redoCount}
                busy={props.busy}
                message={props.message}
                engineStatus={props.engineStatus}
                onValidate={props.onValidate}
                onApply={props.onApply}
                onReviewAllAsBatch={props.onReviewAllAsBatch}
                onUndo={props.onUndo}
                onRedo={props.onRedo}
              />
    </>
  );
}
