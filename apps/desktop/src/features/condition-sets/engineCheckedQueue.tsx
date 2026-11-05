// Model document 0.9.0 S3b: queue a condition-set operation after the operation
// engine has checked it (validate-only, with the current model hash), as the
// load-state editor does (`load-cases/loadStateAuthoring.tsx`). The engine's
// outcome is shown in place exactly as it reports it: blocking refusals by name
// (nothing is queued), the solve's staged findings as warnings and permitted
// carried refusals as info (S3a's classified judgement, §11), with the code,
// message and affected references. The person then applies the queued change
// with Review/Apply; nothing here writes the model.
import { useEffect, useRef, useState } from "react";
import { computeModelHash, canonicalJsonString } from "../../services/hashService";
import { validateModelOperation } from "../../services/operationService";
import type { EditorOperationIntent, OperationOutcomeDiagnostic, PreviewModel } from "../../types";
import type { Canonicalize } from "../model-data/modelDataIntents";

export type EngineCheckedQueueProps = {
  model: PreviewModel;
  queuedIntents?: EditorOperationIntent[];
  onQueueIntent: (intent: EditorOperationIntent) => void;
  operationBusy?: boolean;
  preparationEpoch?: number;
  getPreparationEpoch?: () => number;
};

export const isBlocking = (item: Pick<OperationOutcomeDiagnostic, "severity">) => item.severity === "blocking" || item.severity === "error";

export function useEngineCheckedQueue(props: EngineCheckedQueueProps, canonicalize: Canonicalize = canonicalJsonString) {
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");
  const [notice, setNotice] = useState("");
  const [diagnostics, setDiagnostics] = useState<OperationOutcomeDiagnostic[]>([]);
  const latest = useRef(props);
  latest.current = props;
  const live = useRef(false);
  const pending = useRef(false);
  useEffect(() => { live.current = true; return () => { live.current = false; }; }, []);
  const epoch = (current: EngineCheckedQueueProps) => current.getPreparationEpoch?.() ?? current.preparationEpoch ?? 0;

  function reset() { setError(""); setNotice(""); setDiagnostics([]); }

  /** Build, check and queue. Returns true when the intent was queued. */
  async function submit(build: (canonicalize: Canonicalize) => Promise<EditorOperationIntent> | EditorOperationIntent): Promise<boolean> {
    if (pending.current || props.operationBusy) return false;
    const captured = props;
    const capturedEpoch = epoch(captured);
    pending.current = true;
    setBusy(true); reset();
    try {
      const intent = await build(canonicalize);
      if (captured.queuedIntents?.some((queued) => queued.target.object_type === intent.target.object_type && queued.target.ref === intent.target.ref && queued.change.field_path === intent.change.field_path)) {
        throw new Error("Review or remove the pending change for this record before queuing another.");
      }
      const outcome = await validateModelOperation(captured.model, intent, await computeModelHash(captured.model));
      const current = latest.current;
      if (!live.current) return false;
      if (epoch(current) !== capturedEpoch || current.model !== captured.model || current.operationBusy || current.queuedIntents !== captured.queuedIntents) {
        throw new Error("The model or pending changes changed. Review this draft and queue it again.");
      }
      setDiagnostics(outcome.diagnostics);
      if (outcome.diagnostics.some(isBlocking)) {
        setError(`${intent.change.field_label} was refused by the operation engine; nothing was queued.`);
        return false;
      }
      current.onQueueIntent(intent);
      setNotice(`${intent.change.field_label} checked by the operation engine and queued for Review/Apply.`);
      return true;
    } catch (caught) {
      if (live.current) setError(caught instanceof Error ? caught.message : String(caught));
      return false;
    } finally {
      pending.current = false;
      if (live.current) setBusy(false);
    }
  }

  return { busy: busy || Boolean(props.operationBusy), error, notice, diagnostics, submit, setError, reset };
}

const SEVERITY_TEXT: Record<string, string> = {
  blocking: "Refused",
  error: "Refused",
  warning: "Staged (the solve refuses this case until it is resolved)",
  info: "Carried (already refused before this edit, outside its scope; the solve still refuses it)"
};

/** The engine's outcome in place: each finding with its severity, code, message and references. */
export function EngineFeedback({ error, notice, diagnostics, testId }: { error: string; notice: string; diagnostics: readonly OperationOutcomeDiagnostic[]; testId: string }) {
  return <>
    {error ? <p role="alert" data-testid={`${testId}-error`}>{error}</p> : null}
    {diagnostics.length > 0 ? (
      <ul aria-label="Operation engine findings" data-testid={`${testId}-diagnostics`}>
        {diagnostics.map((item, index) => (
          <li key={`${item.id}:${index}`} data-testid={`${testId}-diagnostic-${item.code}`} data-severity={item.severity}>
            <strong>{SEVERITY_TEXT[item.severity] ?? item.severity}: {item.code}</strong> — {item.message}
            {item.affected_refs?.length ? <> <span>(refers to {item.affected_refs.join(", ")})</span></> : null}
          </li>
        ))}
      </ul>
    ) : null}
    {notice ? <p role="status" data-testid={`${testId}-notice`}>{notice}</p> : null}
  </>;
}
