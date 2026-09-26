import { useEffect, useRef, useState } from "react";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { validateModelOperation } from "../../services/operationService";
import type { EditorOperationIntent, OperationOutcomeDiagnostic, PreviewModel } from "../../types";
import { makeRichIntent, record, setMember, type DraftRecord, type RichFormProps } from "../rich-authoring/formSupport";

/* Plain input fields for the model 0.4.0 load/reference-state records
 * (`openpipestress.load_reference_state/1.0.0`). Every edit is one whole-record
 * replacement submitted through the typed operations in
 * core/model_operations/operation_applier (load_state_authoring.rs): the draft
 * is first checked by the same engine the Review/Apply queue uses
 * (validateModelOperation, validate-only, with the current model hash), then
 * queued for Review/Apply. Nothing here mutates the model, sets a default or
 * judges the record: an empty field is an absent key, and the engine decides. */

export const LOAD_STATE_MODEL_VERSION = "0.4.0";
export const LOAD_STATE_CONTRACT = "openpipestress.load_reference_state/1.0.0";
export const LOAD_STATE_NEEDS_040 = "Load/reference state needs a 0.4.0 model.";
export const NOT_PRESENT = "not_present";

export function isLoadStateModel(model: Pick<PreviewModel, "schema_version">): boolean {
  return model.schema_version === LOAD_STATE_MODEL_VERSION;
}

/** The one line shown instead of the fields on a pre-0.4 model (D3: no upgrade). */
export function LoadStateNeeds040({ testId }: { testId: string }) {
  return <p className="muted" data-testid={testId}>{LOAD_STATE_NEEDS_040}</p>;
}

export const clone = <T,>(value: T): T => value === undefined ? value : JSON.parse(JSON.stringify(value)) as T;

/** Converts entered text in quantity slots (`{value, unit}`) and in the named
 * plain-number keys to JSON numbers. A blank entry is an absent key; an empty
 * quantity is an absent quantity. Text that is not a finite number cannot be a
 * JSON number, so it is reported instead of being sent. Every other key is
 * passed through unchanged. */
export function toPayload(value: unknown, numberKeys: readonly string[] = ["factor"], path = "Input"): unknown {
  if (Array.isArray(value)) return value.map((item, index) => toPayload(item, numberKeys, `${path} ${index + 1}`));
  if (!value || typeof value !== "object") return value;
  const entries = Object.entries(value as DraftRecord);
  if (entries.length > 0 && entries.every(([key]) => key === "value" || key === "unit")) {
    const out: DraftRecord = {};
    for (const [key, item] of entries) {
      if (key === "value") {
        const number = numeric(item, `${path} value`);
        if (number !== undefined) out.value = number;
      } else if (typeof item === "string" ? item !== "" : item !== undefined) out[key] = item;
    }
    return Object.keys(out).length ? out : undefined;
  }
  const out: DraftRecord = {};
  for (const [key, item] of entries) {
    const label = `${path} ${key.replaceAll("_", " ")}`;
    const next = numberKeys.includes(key) ? numeric(item, label) : toPayload(item, numberKeys, label);
    if (next !== undefined) out[key] = next;
  }
  return out;
}

function numeric(value: unknown, label: string): number | undefined {
  if (typeof value === "number") return value;
  if (value === undefined || value === null || (typeof value === "string" && value.trim() === "")) return undefined;
  const text = String(value).trim();
  const number = Number(text);
  if (!Number.isFinite(number)) throw new Error(`${label} must be a finite number.`);
  return number;
}

export type LoadStateTarget = {
  target: EditorOperationIntent["target"];
  kind: "set_field" | "update_load";
  path: "reference_configurations" | "expansion_laws" | "analysis_state";
  label: string;
};

/** Builds the intent for a whole-record replacement, or for removal of the
 * key (`not_present`, the operation's explicit absence value). */
export async function buildLoadStateIntent(spec: LoadStateTarget, before: unknown, after: unknown | typeof NOT_PRESENT): Promise<EditorOperationIntent> {
  const canonicalBefore = before === undefined ? NOT_PRESENT : await canonicalJsonString(before);
  const intent = makeRichIntent(spec.target, spec.kind, spec.path, canonicalBefore, after, spec.label);
  if (after === NOT_PRESENT) intent.change = { ...intent.change, after: NOT_PRESENT };
  return intent;
}

export function blockingDiagnostics(diagnostics: OperationOutcomeDiagnostic[]): OperationOutcomeDiagnostic[] {
  return diagnostics.filter(item => item.severity === "blocking" || item.severity === "error");
}

/** Queue a load/reference-state edit: engine validate-only first, then the
 * ordinary Review/Apply queue. Engine refusals stay in the form as targeted
 * diagnostics; the draft and the model are unchanged and remain editable. */
export function useLoadStateQueue(props: RichFormProps) {
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
  async function submit(spec: LoadStateTarget, before: unknown, after: () => unknown | typeof NOT_PRESENT) {
    if (pending.current || props.operationBusy) return;
    const captured = props;
    const capturedEpoch = epoch(captured);
    pending.current = true;
    setBusy(true); setError(""); setNotice(""); setDiagnostics([]);
    try {
      if (captured.queuedIntents?.some(i => i.target.ref === spec.target.ref && i.change.field_path === spec.path))
        throw new Error("Review or remove the pending change for this record before queuing another.");
      const payload = after();
      const intent = await buildLoadStateIntent(spec, clone(before), clone(payload));
      const hash = await computeModelHash(captured.model);
      const outcome = await validateModelOperation(captured.model, intent, hash);
      const current = latest.current;
      if (!live.current) return;
      if (epoch(current) !== capturedEpoch || current.model !== captured.model || current.operationBusy || current.queuedIntents !== captured.queuedIntents)
        throw new Error("The model or pending changes changed. Review this draft and queue it again.");
      const blocking = blockingDiagnostics(outcome.diagnostics);
      setDiagnostics(outcome.diagnostics);
      if (blocking.length) {
        setError(`${spec.label} was refused by the operation engine; nothing was queued. Correct the fields and queue again.`);
        return;
      }
      current.onQueueIntent(intent);
      setNotice(`${spec.label} checked by the operation engine and queued for Review/Apply.`);
    } catch (e) {
      if (live.current) setError(e instanceof Error ? e.message : String(e));
    } finally {
      pending.current = false;
      if (live.current) setBusy(false);
    }
  }
  return { busy: busy || Boolean(props.operationBusy), error, notice, diagnostics, submit };
}

export function LoadStateFeedback({ error, notice, diagnostics, testId }: { error: string; notice: string; diagnostics: OperationOutcomeDiagnostic[]; testId: string }) {
  return <>
    {error && <p role="alert" data-testid={`${testId}-error`}>{error}</p>}
    {diagnostics.length > 0 && <ul aria-label="Operation engine diagnostics" data-testid={`${testId}-diagnostics`}>
      {diagnostics.map(item => <li key={item.id} data-testid={`${testId}-diagnostic-${item.code}`}>{item.severity}: {item.code} — {item.message}</li>)}
    </ul>}
    {notice && <p role="status" data-testid={`${testId}-notice`}>{notice}</p>}
  </>;
}

/** Discriminant change keeps only the keys that belong to the new variant. */
export function switchVariant(current: unknown, tag: string, value: string, variants: Record<string, readonly string[]>): DraftRecord | undefined {
  if (!value) return undefined;
  const allowed = variants[value] ?? [];
  const source = record(current);
  const next: DraftRecord = { [tag]: value };
  for (const key of allowed) if (source[key] !== undefined) next[key] = source[key];
  return next;
}

export function Text({ label, value, onChange, choices }: { label: string; value: unknown; onChange: (value: string) => void; choices?: readonly string[] }) {
  const entered = typeof value === "string" ? value : "";
  return <label>{label}
    {choices ? <select aria-label={label} value={entered} onChange={e => onChange(e.target.value)}>
      <option value="">Not provided</option>
      {Array.from(new Set([...choices, ...(entered ? [entered] : [])])).map(item => <option key={item} value={item}>{item}</option>)}
    </select> : <input aria-label={label} value={entered} onChange={e => onChange(e.target.value)} />}
  </label>;
}

export function NumberText({ label, value, onChange }: { label: string; value: unknown; onChange: (value: string) => void }) {
  return <label>{label}<input aria-label={label} inputMode="decimal" value={value === undefined || value === null ? "" : String(value)} onChange={e => onChange(e.target.value)} /></label>;
}

/** A quantity slot: an explicit value and an explicit unit, no default unit. */
export function Quantity({ label, dimension, value, onChange }: { label: string; dimension: string; value: unknown; onChange: (value: unknown) => void }) {
  const q = record(value);
  const edit = (key: "value" | "unit", text: string) => {
    const next = setMember(q, key, text);
    onChange(Object.keys(next).length ? next : undefined);
  };
  return <fieldset>
    <legend>{label} ({dimension}; unit required)</legend>
    <label>Value<input aria-label={`${label} value`} inputMode="decimal" value={q.value === undefined ? "" : String(q.value)} onChange={e => edit("value", e.target.value)} /></label>
    <label>Unit<input aria-label={`${label} unit`} value={typeof q.unit === "string" ? q.unit : ""} onChange={e => edit("unit", e.target.value)} /></label>
  </fieldset>;
}

export function member(value: DraftRecord, key: string, next: unknown): DraftRecord {
  return setMember(value, key, next);
}

export function rows(value: unknown): DraftRecord[] | undefined {
  return Array.isArray(value) ? value as DraftRecord[] : undefined;
}

export function replaceAt<T>(list: T[], index: number, next: T): T[] {
  return list.map((item, i) => i === index ? next : item);
}
