import { useEffect, useRef, useState } from "react";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";
import { canonicalJsonString } from "../../services/hashService";
export type RichFormProps = {
  model: PreviewModel;
  selection: EntityRef;
  queuedIntents?: EditorOperationIntent[];
  onQueueIntent: (intent: EditorOperationIntent) => void;
  operationBusy?: boolean;
};
export type DraftRecord = {
  [key: string]: unknown;
};
export const clone = <T,>(value: T): T => JSON.parse(JSON.stringify(value)) as T;
export const record = (value: unknown): DraftRecord => value && typeof value === "object" && !Array.isArray(value) ? value as DraftRecord : {};
export function setMember(value: DraftRecord, key: string, next: unknown): DraftRecord {
  const result = { ...value };
  if (next === undefined || next === "")
    delete result[key];
  else
    result[key] = next;
  return result;
}
export function requireText(value: unknown, label: string): string {
  if (typeof value !== "string" || !value.trim())
    throw new Error(`${label} is required.`);
  return value;
}
/** Parse only explicit quantity slots. Blank input is never coerced to zero. */
export function parseQuantities(value: unknown, path = "Input"): unknown {
  if (Array.isArray(value))
    return value.map((v, i) => parseQuantities(v, `${path} ${i + 1}`));
  if (value && typeof value === "object") {
    const obj = record(value);
    if ("unit" in obj || ("value" in obj && !("dof" in obj))) {
      if ((typeof obj.value !== "number" && typeof obj.value !== "string") || String(obj.value).trim() === "" || !Number.isFinite(Number(obj.value)))
        throw new Error(`${path} requires a finite value.`);
      return {
        ...obj,
        value: Number(obj.value),
        unit: requireText(obj.unit, `${path} unit`)
      };
    }
    return Object.fromEntries(Object.entries(obj).map(([k, v]) => [k, parseQuantities(v, `${path} ${k.replaceAll("_", " ")}`)]));
  }
  return value;
}
export function TextField({ label, value, onChange, choices }: {
  label: string;
  value: unknown;
  onChange: (value: string) => void;
  choices?: readonly string[];
}) {
  const entered = typeof value === "string" ? value : "";
  return <label>{label}
    {choices ? <select
      aria-label={label}
      value={entered}
      onChange={e => onChange(e.target.value)}
    >
      <option value="">Not provided</option>{Array.from(new Set([...choices, ...(entered ? [entered] : [])])).map(v => <option key={v} value={v}>{v.replaceAll("_", " ")}</option>)}</select> : <input
      aria-label={label}
      value={entered}
      onChange={e => onChange(e.target.value)}
    />}</label>;
}
export function QuantityField({ label, value, onChange, dimension }: {
  label: string;
  value: unknown;
  onChange: (value: unknown) => void;
  dimension: string;
}) {
  const q = record(value);
  return <fieldset>
    <legend>{label}</legend>
    <p>Expected dimension: {dimension}. Enter an explicit compatible unit; validation checks compatibility.</p>
    <label>Value<input
      aria-label={`${label} value`}
      inputMode="decimal"
      value={q.value === undefined ? "" : String(q.value)}
      onChange={e => onChange({ ...q, value: e.target.value })}
    />
    </label>
    <label>Unit<input
      aria-label={`${label} unit`}
      value={typeof q.unit === "string" ? q.unit : ""}
      onChange={e => onChange({ ...q, unit: e.target.value })}
    />
    </label>{value !== undefined && <button type="button" onClick={() => onChange(undefined)}>Clear {label.toLowerCase()}</button>}</fieldset>;
}
export function makeRichIntent(target: EditorOperationIntent["target"], kind: EditorOperationIntent["change"]["change_kind"], path: string, before: string, after: unknown, label: string): EditorOperationIntent {
  const id = `${target.ref}:${path}`;
  return {
    operation_id: `op:rich:${id}`,
    operation_kind: kind === "create_support" ? "create" : "modify",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "apps/desktop/src/features/rich-authoring",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target,
    change: {
      change_id: `change:rich:${id}`,
      change_kind: kind,
      field_label: label,
      field_path: path,
      before,
      after: JSON.stringify(after),
      unit: "none",
      dimension: "dimensionless",
      source_note: "Explicit user-authored values and units; missing optional fields remain absent."
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: `User-authored ${label.toLowerCase()}; requires validation and review before application.`
  };
}
/** Parent props are consulted again after asynchronous Rust canonicalization. */
export function useRichQueue(props: RichFormProps) {
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");
  const [notice, setNotice] = useState("");
  const latest = useRef(props);
  latest.current = props;
  const live = useRef(false);
  const pending = useRef(false);
  useEffect(() => {
    live.current = true;
    return () => {
      live.current = false;
    };
  }, []);
  const queue = async (target: EditorOperationIntent["target"], kind: EditorOperationIntent["change"]["change_kind"], path: string, before: unknown, after: unknown, label: string, absent = false) => {
    if (pending.current || props.operationBusy)
      return;
    const captured = props;
    const snapshot = clone(after);
    const old = before === undefined ? undefined : clone(before);
    pending.current = true;
    setBusy(true);
    setError("");
    setNotice("");
    try {
      if (props.queuedIntents?.some(i => i.target.ref === target.ref && (i.change.field_path === path || i.change.change_kind === "create_support")))
        throw new Error("Review or remove the pending change for this record before queuing another.");
      const canonical = absent ? "not_present" : await canonicalJsonString(old);
      const current = latest.current;
      if (!live.current)
        return;
      if (current.model !== captured.model || current.selection.id !== captured.selection.id || current.selection.type !== captured.selection.type || current.operationBusy || current.queuedIntents !== captured.queuedIntents)
        throw new Error("The selection, model or pending changes changed. Review this draft and queue it again.");
      current.onQueueIntent(makeRichIntent(target, kind, path, canonical, snapshot, label));
      setNotice(`${label} queued for validation and review.`);
    }
    catch (e) {
      if (live.current)
        setError(e instanceof Error ? e.message : String(e));
    }
    finally {
      pending.current = false;
      if (live.current)
        setBusy(false);
    }
  };
  return {
    busy: busy || Boolean(props.operationBusy),
    error,
    notice,
    setError,
    queue
  };
}
export function QueueFeedback({ error, notice }: {
  error: string;
  notice: string;
}) {
  return <>{error && <p role="alert">{error}</p>}
    {notice && <p role="status">{notice}</p>}</>;
}
