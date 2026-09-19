import { useCallback, useRef, useState } from "react";
import type { OperationBatchOutcome } from "../../services/operationBatchService";
import { initialOperationEngineStatus } from "../../services/operationService";
import type { OperationEngineStatus } from "../../services/operationService";
import type { AppliedOperationReceipt, EditorOperationIntent, OperationOutcome } from "../../types";
import type { BatchReceipt, QueuedBatch } from "../toolkit/BatchReviewPanel";
import type { FrozenDraftReview } from "../viewport/routeDraft";
import type { SessionModelCheckpoint } from "./sessionModel";

/**
 * The operations cells of the workspace session: the queued editor intents and
 * the retained review context; the operation outcomes and the applied receipts;
 * the undo and redo stacks; the queued batches with their outcomes, receipts
 * and message; the request epoch with its ref and getter; the direct-draft
 * commit token; the busy flag, the message and the engine status; and the
 * sequence, request and draft-review refs that the apply paths guard on. It
 * declares no effect and no handler.
 *
 * Called only by the session, which is `AppSession` in `App.tsx`. Its setters
 * are not for components: the queue, the stacks and the receipts change only in
 * the session's operation, undo, redo and project handlers.
 */
export function useOperationsSessionState() {
  const [editorIntents, setEditorIntents] = useState<EditorOperationIntent[]>([]);
  const [retainedReviewContext, setRetainedReviewContext] = useState<EditorOperationIntent[]>([]);
  const [operationOutcomes, setOperationOutcomes] = useState<Record<string, OperationOutcome>>({});
  const [appliedOperations, setAppliedOperations] = useState<AppliedOperationReceipt[]>([]);
  const [undoStack, setUndoStack] = useState<SessionModelCheckpoint[]>([]);
  const [redoStack, setRedoStack] = useState<SessionModelCheckpoint[]>([]);
  const [queuedBatches, setQueuedBatches] = useState<QueuedBatch[]>([]);
  const [batchOutcomes, setBatchOutcomes] = useState<Record<string, OperationBatchOutcome>>({});
  const [batchReceipts, setBatchReceipts] = useState<BatchReceipt[]>([]);
  const [batchMessage, setBatchMessage] = useState<string | null>(null);
  const [requestEpoch, setRequestEpoch] = useState(0);
  const requestEpochRef = useRef(0);
  const getPreparationEpoch = useCallback(() => requestEpochRef.current, []);
  const [directDraftCommitToken, setDirectDraftCommitToken] = useState<string | null>(null);
  const batchSequence = useRef(0);
  const [operationBusy, setOperationBusy] = useState(false);
  const [operationMessage, setOperationMessage] = useState<string | null>(null);
  const [operationEngineStatus, setOperationEngineStatus] = useState<OperationEngineStatus>(() =>
    initialOperationEngineStatus()
  );
  const intentSequence = useRef(0);
  const operationRequest = useRef({ sequence: 0, busy: false });
  const directDraftReviews = useRef(new Map<string, FrozenDraftReview>());
  const directDraftReviewSequence = useRef(0);
  return {
    editorIntents, setEditorIntents,
    retainedReviewContext, setRetainedReviewContext,
    operationOutcomes, setOperationOutcomes,
    appliedOperations, setAppliedOperations,
    undoStack, setUndoStack,
    redoStack, setRedoStack,
    queuedBatches, setQueuedBatches,
    batchOutcomes, setBatchOutcomes,
    batchReceipts, setBatchReceipts,
    batchMessage, setBatchMessage,
    requestEpoch, setRequestEpoch,
    requestEpochRef,
    getPreparationEpoch,
    directDraftCommitToken, setDirectDraftCommitToken,
    batchSequence,
    operationBusy, setOperationBusy,
    operationMessage, setOperationMessage,
    operationEngineStatus, setOperationEngineStatus,
    intentSequence,
    operationRequest,
    directDraftReviews,
    directDraftReviewSequence
  };
}
