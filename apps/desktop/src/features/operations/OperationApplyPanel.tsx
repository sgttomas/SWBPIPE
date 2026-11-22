import { PlayCircle, SearchCheck, X } from "lucide-react";
import type { OperationEngineStatus } from "../../services/operationService";
import type { AppliedOperationReceipt, EditorOperationIntent, OperationOutcome, PreviewModel } from "../../types";

/**
 * The queued single changes of the Review changes tab: one row per change, read
 * in words (what changes, on which entity, from what to what, in its unit), with
 * Apply and Remove on the row, and the operation as recorded, Validate and the
 * engine's outcome in the row's closed details. The tab (`ReviewChangesTab`)
 * lays these out; `OperationApplyRecord` is the tab's record of what was applied.
 */
export function PendingChangeRows({
  queuedIntents,
  outcomes,
  model,
  busy,
  onValidate,
  onApply,
  onRemove
}: {
  queuedIntents: EditorOperationIntent[];
  outcomes: Record<string, OperationOutcome>;
  model: PreviewModel | null;
  busy: boolean;
  onValidate: (intent: EditorOperationIntent) => void;
  onApply: (intent: EditorOperationIntent) => unknown;
  /** Removes one queued change; the row offers Remove only when given. */
  onRemove?: (key: string) => void;
}) {
  return (
    <div className="operation-record-list" data-testid="operation-apply-queue">
      {queuedIntents.map((intent) => {
        const key = intentKey(intent);
        const outcome = outcomes[key] ?? null;
        const state = outcome ? outcomeLine(outcome) : null;
        return (
          <article className="operation-record pending-change" data-testid={`operation-apply-row-${key}`} key={key}>
            <p className="pending-change-line" data-testid={`operation-apply-line-${key}`}>{queuedChangeLine(intent, model)}</p>
            {state ? (
              <p className={state.refused ? "pending-change-state refused" : "pending-change-state"} data-testid={`operation-apply-state-${key}`} role={state.refused ? "alert" : undefined}>
                {state.text}
              </p>
            ) : null}
            <div className="operation-apply-actions">
              <button
                data-testid={`apply-intent-${key}`}
                disabled={busy}
                onClick={() => onApply(intent)}
                title="Check this change and apply it"
                type="button"
              >
                <PlayCircle size={14} aria-hidden="true" />
                Apply
              </button>
              {onRemove ? (
                <button
                  data-testid={`remove-intent-${key}`}
                  disabled={busy}
                  onClick={() => onRemove(key)}
                  title="Take this change off the list without applying it"
                  type="button"
                >
                  <X size={14} aria-hidden="true" />
                  Remove
                </button>
              ) : null}
            </div>
            <details className="pending-change-details" data-testid={`operation-apply-details-${key}`}>
              <summary>Details</summary>
              <strong>{intent.operation_id}</strong>
              <small>
                {intent.target.object_type} {intent.target.ref}; {intent.change.field_path}:{" "}
                {intent.change.before} to {intent.change.after}
              </small>
              <small data-testid={`operation-apply-author-${key}`}>
                {intent.author_type === "agent" ? "Proposed by the agent" : "Entered by you"}; stage{" "}
                {outcome?.mode === "validate_only" && !outcome.diagnostics.some(isBlocking) ? "validated" : "proposed"}.
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
              </div>
              {outcome ? <OutcomeSummary outcome={outcome} testKey={key} /> : null}
            </details>
          </article>
        );
      })}
    </div>
  );
}

/**
 * The record of this session's single changes, unchanged in content and test id:
 * the counts, the engine's route and readiness, the unit validations, the undo
 * and redo depths, the applied receipts and the acceptance boundary. The tab
 * keeps it in its closed "Record" disclosure.
 */
export function OperationApplyRecord({
  queuedIntents,
  outcomes,
  appliedOperations,
  undoCount,
  redoCount,
  engineStatus
}: {
  queuedIntents: EditorOperationIntent[];
  outcomes: Record<string, OperationOutcome>;
  appliedOperations: AppliedOperationReceipt[];
  undoCount: number;
  redoCount: number;
  engineStatus: OperationEngineStatus;
}) {
  return (
    <>
      <p className="muted" data-testid="operation-apply-summary">
        {queuedIntents.length} queued; {appliedOperations.length} applied this session. Applying validates through the
        structured-operation seam and replaces the session model document; saving the project stores it locally.
      </p>
      <div className="operation-status-chips" aria-label="Operation status">
        <span data-testid="operation-engine-chip">Engine {engineStatus.state}</span>
        <span data-testid="operation-unit-policy-chip">
          {operationApplyUnitPolicySummary(queuedIntents, outcomes, appliedOperations)}
        </span>
        <span className="operation-history-chip" data-testid="session-history-chip">
          {undoCount} undo / {redoCount} redo
        </span>
      </div>
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
                Applied through {receipt.application_route}. Acceptance basis {receipt.acceptance.acceptance_basis};
                persistence {receipt.acceptance.persistence_status}; professional approval{" "}
                {receipt.acceptance.acceptance_is_professional_approval ? "recorded" : "not recorded"}.
              </small>
              <small>model_hash={shortHash(receipt.applied_model_hash)}</small>
            </article>
          ))}
        </div>
      ) : null}
      <small className="report-note">
        Applying a structured operation records a user-initiated local-session acceptance only. Results of earlier
        solves are cleared because they no longer describe the edited model.
      </small>
    </>
  );
}

function OutcomeSummary({ outcome, testKey }: { outcome: OperationOutcome; testKey: string }) {
  return (
    <div className="operation-outcome" data-testid={`operation-outcome-${testKey}`}>
      <small data-testid={`operation-outcome-status-${testKey}`}>
        {outcome.mode} outcome: {outcome.validation.application_status}. Schema {outcome.validation.schema_validation};
        units {outcome.validation.unit_validation}; prior state {outcome.validation.before_state_validation}. Route{" "}
        {outcome.application_route}.
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

const isBlocking = (diagnostic: OperationOutcome["diagnostics"][number]) => diagnostic.severity === "blocking";

/**
 * A queued change's state in words, only when it is not simply waiting: the
 * engine's own reason when it cannot be applied (its code stays in the row's
 * details), or that a check passed, with any warning in the engine's words.
 */
export function outcomeLine(outcome: OperationOutcome): { text: string; refused: boolean } | null {
  const blocking = outcome.diagnostics.filter(isBlocking);
  if (blocking.length > 0) return { text: `Cannot be applied: ${blocking.map((item) => item.message).join(" ")}`, refused: true };
  if (outcome.mode === "apply" && outcome.validation.application_status !== "applied_to_session_model") {
    return { text: "Not applied. The engine's outcome is in Details.", refused: true };
  }
  const warnings = outcome.diagnostics.filter((item) => item.severity === "warning");
  if (outcome.mode === "validate_only") {
    return { text: `Checked; it can be applied.${warnings.length ? ` Warning: ${warnings.map((item) => item.message).join(" ")}` : ""}`, refused: false };
  }
  return null;
}

/** The record family each operation target lives in, and what an engineer calls one. */
const TARGETS: Record<string, { collection: string; noun: string }> = {
  Node: { collection: "nodes", noun: "node" },
  Element: { collection: "pipe_segments", noun: "pipe" },
  Component: { collection: "components", noun: "component" },
  Support: { collection: "supports", noun: "support" },
  Load: { collection: "load_cases", noun: "load case" },
  Combination: { collection: "combinations", noun: "combination" },
  Material: { collection: "materials", noun: "material" },
  Section: { collection: "sections", noun: "section" },
  Line: { collection: "lines", noun: "line" },
  Assumption: { collection: "assumptions", noun: "assumption" },
  Constraint: { collection: "constraints", noun: "constraint" },
  EquipmentInterface: { collection: "equipment_interfaces", noun: "equipment interface" },
  ConditionSet: { collection: "condition_sets", noun: "condition set" },
  RulePackRef: { collection: "rule_pack_refs", noun: "rule pack" }
};
/** Change kinds whose target is the entity they delete. */
const DELETES_TARGET: ReadonlySet<string> = new Set([
  "delete_node", "delete_pipe_run", "delete_support", "delete_material", "delete_section", "delete_component",
  "delete_load_case", "delete_combination", "delete_line", "delete_assumption", "delete_constraint",
  "delete_equipment_interface", "delete_condition_set"
]);
const ABSENT = "not_present";

type Named = { id?: unknown; label?: unknown; name?: unknown };
function findRecord(model: PreviewModel | null, collection: string, ref: string): Named | undefined {
  const records = model ? (model as unknown as Record<string, unknown>)[collection] : undefined;
  return Array.isArray(records) ? (records as Named[]).find((record) => record?.id === ref) : undefined;
}
function displayName(record: Named | undefined, fallback: string): string {
  for (const value of [record?.label, record?.name]) if (typeof value === "string" && value.trim()) return value;
  return fallback;
}
function parsedRecord(value: string): Record<string, unknown> | null {
  if (!/^\s*[[{]/.test(value)) return null;
  try {
    const parsed: unknown = JSON.parse(value);
    return parsed && typeof parsed === "object" && !Array.isArray(parsed) ? (parsed as Record<string, unknown>) : {};
  } catch {
    return null;
  }
}
const unitText = (unit: unknown) => (typeof unit === "string" && unit && unit !== "none" && unit !== "1" && unit !== "TBD" ? ` ${unit}` : "");
/**
 * One side of a change as an engineer reads it: a number or text with its unit,
 * a quantity record (`{"value": 0.5, "unit": "m"}`) as its value and unit, a
 * condition-set reference by the set's name, or null for any other record, whose
 * members are shown in the row's details.
 */
function sideText(text: string, unit: string, model: PreviewModel | null): string | null {
  if (text === "") return "not stated";
  if (text === ABSENT) return "none";
  const record = parsedRecord(text);
  if (record === null) return /^\s*[[{]/.test(text) ? null : `${text}${unitText(unit)}`;
  const keys = Object.keys(record);
  if ("value" in record && keys.every((key) => key === "value" || key === "unit") && (typeof record.value === "number" || typeof record.value === "string")) {
    return `${record.value}${unitText(record.unit ?? unit)}`;
  }
  if (keys.length === 1 && typeof record.condition_set_ref === "string") {
    return displayName(findRecord(model, "condition_sets", record.condition_set_ref), record.condition_set_ref);
  }
  return null;
}

/**
 * One line for a queued change, in an engineer's words: what changes, on which
 * entity by the name the tree shows, from what to what in its unit. Identifiers
 * of the operation stay in the row's details.
 */
export function queuedChangeLine(intent: EditorOperationIntent, model: PreviewModel | null): string {
  const { change, target } = intent;
  const known = TARGETS[target.object_type];
  const projectTarget = target.object_type === "Model" || target.object_type === "Project";
  const noun = projectTarget ? "project" : known?.noun ?? target.object_type.toLowerCase();
  const existing = known ? findRecord(model, known.collection, target.ref) : undefined;
  const payload = parsedRecord(change.after);
  const name = projectTarget
    ? model?.project.name || target.ref
    : displayName(existing ?? (payload as Named | null) ?? undefined, target.ref);
  const nameOf = (collection: string, ref: unknown) => (typeof ref === "string" ? displayName(findRecord(model, collection, ref), ref) : null);
  const author = intent.author_type === "agent" ? " (proposed by the agent)" : "";
  const kind = String(change.change_kind);

  if (!existing && !projectTarget && change.before === ABSENT && (kind.startsWith("create_") || kind === "connect_pipe_run" || kind === "insert_component_symbol")) {
    const position = (payload?.position ?? payload) as Record<string, unknown> | null;
    const at = position && [position.x, position.y, position.z].every((value) => typeof value === "number")
      ? ` at (${position.x}, ${position.y}, ${position.z})${unitText(change.unit)}`
      : "";
    const from = nameOf("nodes", payload?.from);
    const to = nameOf("nodes", payload?.to);
    return `Add ${noun} ${name}${at}${from && to ? ` from ${from} to ${to}` : ""}${author}`;
  }
  if (DELETES_TARGET.has(kind) && change.after === ABSENT) return `Delete ${noun} ${name}${author}`;
  const subject = `${change.field_label || change.field_path} of ${noun} ${name}`;
  const before = sideText(change.before, change.unit, model);
  const after = sideText(change.after, change.unit, model);
  if (before === null || after === null) return `${subject}: changed (values in Details)${author}`;
  return `${subject}: ${before} → ${after}${author}`;
}

function operationApplyUnitPolicySummary(
  queuedIntents: EditorOperationIntent[],
  outcomes: Record<string, OperationOutcome>,
  appliedOperations: AppliedOperationReceipt[]
) {
  const queuedUnitBearing = queuedIntents.filter((intent) => intent.change.unit !== "none").length;
  const queuedDimensionless = queuedIntents.length - queuedUnitBearing;
  const unitValidationStatuses = Array.from(
    new Set(
      Object.values(outcomes)
        .map((outcome) => outcome.validation.unit_validation)
        .filter((status) => status.trim().length > 0)
    )
  ).sort();
  return `${queuedUnitBearing} unit-bearing queued, ${queuedDimensionless} dimensionless queued; ${
    unitValidationStatuses.length > 0 ? unitValidationStatuses.join(", ") : "no unit validations yet"
  }; ${appliedOperations.length} applied receipts.`;
}

export function intentKey(intent: EditorOperationIntent): string {
  return intent.queue_id ?? intent.operation_id;
}

function shortHash(value: string): string {
  return value.length > 24 ? `${value.slice(0, 24)}…` : value;
}

/** The queued changes the person authored; another author's stay single (G9). */
export function batchableCount(intents: EditorOperationIntent[]): number {
  return intents.filter((intent) => intent.author_type === "user").length;
}
