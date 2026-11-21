// A configuration that gives a directed support a hanger. A hanger takes no direction, and a stored direction is never
// dropped silently, so the form queues one batch of two changes: the explicit removal of the stored direction first, the
// configuration second. Apply entire batch applies both or neither, and one Undo reverts both; nothing is queued as a
// single change. The operation engine checks that exact batch on the current model before it is queued; its refusals of
// the pair are shown by the form's own statements. The removal is built by hanger selection's builder, so both paths
// write it alike.
import { useEffect, useRef, useState } from "react";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { validateOperationBatch, type OperationBatch } from "../../services/operationBatchService";
import type { OperationOutcomeDiagnostic } from "../../types";
import { isBlocking } from "../condition-sets/engineCheckedQueue";
import { buildDirectionRemovalChange, SUPPORT_DIRECTION_FIELD_PATH } from "../hanger-selection/hangerSelection";
import { clone, makeRichIntent, type RichFormProps } from "../rich-authoring/formSupport";
import { directionRemovalRefusedText, directionRemovalStatement, STORED_DIRECTION_CHANGED_TEXT } from "./supportDirection";

/** The engine's codes this pair answers with its own statement: the hanger on a record that still holds the
 * direction, and a removal whose before is no longer the stored direction. */
export const RECORD_UNSUPPORTED_CODE = "SUPPORT_DIRECTION_RECORD_UNSUPPORTED";
export const BEFORE_MISMATCH_CODE = "OP-BEFORE-VALUE-MISMATCH";

/** The form's button while the configuration gives a directed support a hanger. */
export const DIRECTION_REMOVAL_BUTTON_TEXT = "Queue the removal and the configuration as one batch";
/** What the person reads once the batch is queued. */
export const DIRECTION_REMOVAL_QUEUED_TEXT = "One batch of two changes was queued for review under Batch review: the removal of the stored direction, then the support configuration with the hanger. " +
  "Apply entire batch applies both, the removal first, or neither; one Undo reverts both.";
/** Without a batch queue the pair cannot be queued at all: never as two single changes. */
export const DIRECTION_REMOVAL_NO_BATCH_TEXT = "This support's stored direction and its new hanger configuration must be queued together as one batch, and batches cannot be queued from here; nothing was queued.";

/** A form that can queue a batch. */
export type BatchFormProps = RichFormProps & { onQueueBatch?: (batch: OperationBatch) => void | Promise<void> };

export type DirectionRemovalRequest = {
  supportId: string;
  /** The stored direction value, as the record holds it. */
  direction: unknown;
  /** The stored direction as the form prints it. */
  storedText: string;
  /** The stored configuration (the configuration's before) and the edited one (its after). */
  before: unknown;
  after: unknown;
};

/** The one batch: the removal, then the configuration. */
export async function buildDirectionRemovalBatch(request: DirectionRemovalRequest): Promise<OperationBatch> {
  const statement = directionRemovalStatement(request.storedText);
  const removal = buildDirectionRemovalChange(request.supportId, await canonicalJsonString(request.direction),
    `${statement} Queued by the support configuration form in one batch with the configuration that adds the hanger, ahead of it.`);
  const configuration = makeRichIntent({ object_type: "Support", ref: request.supportId }, "update_support", "configuration",
    await canonicalJsonString(clone(request.before)), clone(request.after), "Support configuration");
  configuration.rationale = `User-authored support configuration that adds a hanger. ${statement} The removal is the change before this one in the same batch.`;
  return { batch_id: `batch:support-configuration:hanger-removes-direction:${crypto.randomUUID()}`, operations: [removal, configuration] };
}

export function useDirectionRemovalQueue(props: BatchFormProps) {
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");
  const [notice, setNotice] = useState("");
  const [diagnostics, setDiagnostics] = useState<OperationOutcomeDiagnostic[]>([]);
  const latest = useRef(props);
  latest.current = props;
  const live = useRef(false);
  const pending = useRef(false);
  useEffect(() => { live.current = true; return () => { live.current = false; }; }, []);
  const epoch = (current: RichFormProps) => current.getPreparationEpoch?.() ?? current.preparationEpoch ?? 0;

  function reset() { setError(""); setNotice(""); setDiagnostics([]); }

  /** Check the batch with the engine, then queue that same batch. True when it was queued. */
  async function submit(request: DirectionRemovalRequest): Promise<boolean> {
    if (pending.current || props.operationBusy) return false;
    const captured = props;
    const capturedEpoch = epoch(captured);
    reset();
    if (!captured.onQueueBatch) { setError(DIRECTION_REMOVAL_NO_BATCH_TEXT); return false; }
    pending.current = true;
    setBusy(true);
    try {
      // A single change already queued for this support (the review list's own queue). A queued batch is not visible
      // here; a second batch for the same support is retired by the session when the first is applied.
      if (captured.queuedIntents?.some(i => i.target.ref === request.supportId
        && ([SUPPORT_DIRECTION_FIELD_PATH, "configuration"].includes(i.change.field_path) || i.change.change_kind === "create_support")))
        throw new Error("Review or remove the pending change for this record before queuing another.");
      const batch = await buildDirectionRemovalBatch(request);
      const outcome = await validateOperationBatch(captured.model, batch, await computeModelHash(captured.model));
      const current = latest.current;
      if (!live.current) return false;
      if (epoch(current) !== capturedEpoch || current.model !== captured.model || current.operationBusy || current.queuedIntents !== captured.queuedIntents
        || current.selection.id !== captured.selection.id || current.selection.type !== captured.selection.type || !current.onQueueBatch)
        throw new Error("The selection, model or pending changes changed. Review this draft and queue it again.");
      const found = [...outcome.diagnostics, ...(outcome.operation_outcomes ?? []).flatMap(step => step.diagnostics ?? [])]
        .filter((item, index, all) => all.findIndex(other => other.id === item.id && other.code === item.code) === index);
      const answered = (code: string) => found.some(item => item.code === code && isBlocking(item));
      // The applier's message and remediation for these two are not shown: the form states what happened.
      setDiagnostics(found.filter(item => item.code !== RECORD_UNSUPPORTED_CODE && item.code !== BEFORE_MISMATCH_CODE));
      if (answered(BEFORE_MISMATCH_CODE)) { setError(STORED_DIRECTION_CHANGED_TEXT); return false; }
      if (answered(RECORD_UNSUPPORTED_CODE)) { setError(directionRemovalRefusedText(request.storedText)); return false; }
      if (found.some(isBlocking) || outcome.validation.batch_validation_status === "blocked") {
        setError("The removal of the stored direction and the support configuration were refused by the operation engine; nothing was queued.");
        return false;
      }
      // The batch the engine checked, unchanged.
      await current.onQueueBatch(batch);
      if (live.current) setNotice(DIRECTION_REMOVAL_QUEUED_TEXT);
      return true;
    } catch (caught) {
      if (live.current) setError(caught instanceof Error ? caught.message : String(caught));
      return false;
    } finally {
      pending.current = false;
      if (live.current) setBusy(false);
    }
  }

  return { busy: busy || Boolean(props.operationBusy), error, notice, diagnostics, submit, reset };
}
