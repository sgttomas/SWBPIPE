// The Loads page's case sections (Wind, Seismic, Hydrostatic test): queue the section's changes
// after the operation engine has checked them, validate-only on the current model, as the
// Conditions control does (`condition-sets/engineCheckedQueue.tsx`). One change is checked
// alone, so the engine's own classification of that intent is shown; several are checked as
// one ordered sequence (`validateOperationBatch`), so a change that relies on the one before it
// (the test contents on a case that becomes a test case; the contents cleared before the test)
// is judged on the state the earlier one leaves. Nothing is queued while the engine refuses any
// change; the engine's findings are shown in place, by code with its message. Nothing here
// writes the model: the person applies the queued changes with Review/Apply.
import { useEffect, useRef, useState } from "react";
import { computeModelHash, canonicalJsonString } from "../../services/hashService";
import { validateModelOperation } from "../../services/operationService";
import { validateOperationBatch } from "../../services/operationBatchService";
import type { EditorOperationIntent, OperationOutcomeDiagnostic, PreviewModel } from "../../types";
import type { Canonicalize } from "../model-data/modelDataIntents";
import { isBlocking, type EngineCheckedQueueProps } from "../condition-sets/engineCheckedQueue";

export function useCheckedSectionQueue(props: EngineCheckedQueueProps, canonicalize: Canonicalize = canonicalJsonString) {
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

  /** Build, check and queue, in order. Returns true when every change was queued. */
  async function submit(label: string, build: (canonicalize: Canonicalize) => Promise<EditorOperationIntent[]>): Promise<boolean> {
    if (pending.current || props.operationBusy) return false;
    const captured = props;
    const capturedEpoch = epoch(captured);
    pending.current = true;
    setBusy(true); reset();
    try {
      const intents = await build(canonicalize);
      if (!intents.length) return false;
      const clash = intents.find((intent) => captured.queuedIntents?.some((queued) => queued.target.object_type === intent.target.object_type && queued.target.ref === intent.target.ref && queued.change.field_path === intent.change.field_path));
      if (clash) throw new Error(`${clash.change.field_label} already has a change waiting in Review changes; apply or remove it first.`);
      const found = await check(captured.model, intents);
      const current = latest.current;
      if (!live.current) return false;
      if (epoch(current) !== capturedEpoch || current.model !== captured.model || current.operationBusy || current.queuedIntents !== captured.queuedIntents) {
        throw new Error("The model or the waiting changes changed meanwhile; nothing was queued. Queue again.");
      }
      setDiagnostics(found);
      if (found.some(isBlocking)) {
        setError(`${label}: refused by the engine; nothing was queued.`);
        return false;
      }
      for (const intent of intents) current.onQueueIntent(intent);
      setNotice(`${label}: queued. Apply it in Review changes.`);
      return true;
    } catch (caught) {
      if (live.current) setError(caught instanceof Error ? caught.message : String(caught));
      return false;
    } finally {
      pending.current = false;
      if (live.current) setBusy(false);
    }
  }

  return { busy: busy || Boolean(props.operationBusy), error, notice, diagnostics, submit };
}

/** The engine's findings for the changes. A sequence reports every refusal on its way, and the
 * warnings and notes of the state its last change leaves (those of an earlier step that the
 * later ones resolve are not the person's to act on). */
async function check(model: PreviewModel, intents: EditorOperationIntent[]): Promise<OperationOutcomeDiagnostic[]> {
  const hash = await computeModelHash(model);
  if (intents.length === 1) return (await validateModelOperation(model, intents[0], hash)).diagnostics;
  const outcome = await validateOperationBatch(model, { batch_id: `loads-case-section-${intents[0].target.ref}`, operations: intents }, hash);
  const steps = outcome.operation_outcomes ?? [];
  const last = steps.length === intents.length ? steps[steps.length - 1].diagnostics : [];
  const refusals = [...outcome.diagnostics.filter(isBlocking)];
  if (outcome.validation.batch_validation_status !== "passed" && !refusals.length) {
    refusals.push(...outcome.diagnostics);
  }
  const seen = new Set<string>();
  return [...refusals, ...last.filter((item) => !isBlocking(item))].filter((item) => {
    const key = `${item.code}:${item.message}`;
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
}

const SEVERITY_WORD: Record<string, string> = { blocking: "Refused", error: "Refused", warning: "Warning", info: "Already refused" };

/** The engine's findings beside a section: what it is in plain words where the code is known
 * here, then the code and the engine's own message. */
export function SectionFindings({ error, notice, diagnostics, testId, plain = {} }: {
  error: string;
  notice: string;
  diagnostics: readonly OperationOutcomeDiagnostic[];
  testId: string;
  plain?: Readonly<Record<string, string>>;
}) {
  return <>
    {diagnostics.length > 0 ? (
      <ul className="loads-findings" aria-label="Engine findings" data-testid={`${testId}-findings`}>
        {diagnostics.map((item, index) => (
          <li key={`${item.id}:${index}`} data-testid={`${testId}-finding-${item.code}`} data-severity={item.severity}>
            <strong>{SEVERITY_WORD[item.severity] ?? item.severity}{plain[item.code] ? `: ${plain[item.code]}` : ""}</strong>{" "}
            <code>{item.code}</code>
            <span className="loads-finding-message">{item.message}</span>
          </li>
        ))}
      </ul>
    ) : null}
    {error ? <p role="alert" data-testid={`${testId}-error`}>{error}</p> : null}
    {notice ? <p role="status" className="loads-status" data-testid={`${testId}-notice`}>{notice}</p> : null}
  </>;
}
