// A configuration that gives a directed support a hanger. A hanger takes no direction, and a stored direction is never
// dropped silently, so the form queues the explicit removal of the stored direction first and the configuration
// second, never the configuration alone. The operation engine checks the two together, as one batch on the current
// model, before either is queued; its refusals of this pair are shown by the form's own statements. The field path,
// label and reason are hanger selection's exports, so both paths name the removal alike.
import { useEffect, useRef, useState } from "react";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { validateOperationBatch } from "../../services/operationBatchService";
import type { EditorOperationIntent, OperationOutcomeDiagnostic } from "../../types";
import { isBlocking } from "../condition-sets/engineCheckedQueue";
import { SUPPORT_DIRECTION_FIELD_PATH } from "../hanger-selection/hangerSelection";
import { clone, makeRichIntent, type RichFormProps } from "../rich-authoring/formSupport";
import { buildDirectionRemovalIntent, directionRemovalRefusedText, directionRemovalStatement, STORED_DIRECTION_CHANGED_TEXT } from "./supportDirection";

/** The engine's codes this pair answers with its own statement: the hanger on a record that still holds the
 * direction, and a removal whose before is no longer the stored direction. */
export const RECORD_UNSUPPORTED_CODE = "SUPPORT_DIRECTION_RECORD_UNSUPPORTED";
export const BEFORE_MISMATCH_CODE = "OP-BEFORE-VALUE-MISMATCH";

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

export function useDirectionRemovalQueue(props: RichFormProps) {
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

  /** Check the removal and the configuration together, then queue them in that order. True when both were queued. */
  async function submit(request: DirectionRemovalRequest): Promise<boolean> {
    if (pending.current || props.operationBusy) return false;
    const captured = props;
    const capturedEpoch = epoch(captured);
    pending.current = true;
    setBusy(true); reset();
    try {
      if (captured.queuedIntents?.some(i => i.target.ref === request.supportId
        && ([SUPPORT_DIRECTION_FIELD_PATH, "configuration"].includes(i.change.field_path) || i.change.change_kind === "create_support")))
        throw new Error("Review or remove the pending change for this record before queuing another.");
      const statement = directionRemovalStatement(request.storedText);
      const removal = buildDirectionRemovalIntent(request.supportId, await canonicalJsonString(request.direction),
        `${statement} Queued by the support configuration form ahead of the configuration that adds the hanger.`);
      const configuration: EditorOperationIntent = makeRichIntent({ object_type: "Support", ref: request.supportId }, "update_support", "configuration",
        await canonicalJsonString(clone(request.before)), clone(request.after), "Support configuration");
      const outcome = await validateOperationBatch(captured.model,
        { batch_id: `batch:support-configuration:${crypto.randomUUID()}`, operations: [removal, configuration] },
        await computeModelHash(captured.model));
      const current = latest.current;
      if (!live.current) return false;
      if (epoch(current) !== capturedEpoch || current.model !== captured.model || current.operationBusy || current.queuedIntents !== captured.queuedIntents
        || current.selection.id !== captured.selection.id || current.selection.type !== captured.selection.type)
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
      current.onQueueIntent(removal);
      current.onQueueIntent(configuration);
      setNotice("The removal of the stored direction and the support configuration were checked together by the operation engine and queued for Review/Apply, the removal first.");
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
