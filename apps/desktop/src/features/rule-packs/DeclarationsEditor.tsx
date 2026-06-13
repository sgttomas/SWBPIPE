import type { ReactNode } from "react";
import { DimensionSelect } from "./ExpressionComposer";
import { draftPlaceholderProvenance } from "../../services/rulePackService";

// Rule-pack declarations editor (Phase C2 slice 4, TP-C2-DECLEDITOR-001).
//
// PRD §14.5 requires the rule-pack editor to let a user declare the rule
// pack's inputs and user-supplied value slots — the named variables a
// formula's `variable_ref` binds to. Slices 1-3 authored the document metadata
// and the formula expression AST, but `required_inputs` and `value_slots`
// (the variable declarations) still had to be hand-edited as raw document
// JSON; the expression composer's variable picker had nothing to offer until
// they were. This component replaces that hand-editing with structured form
// controls: add / remove / edit each required input and value slot, with the
// composer's variable browser reflecting them immediately.
//
// Boundary (unchanged from the composer slices): purely structured form
// controls — no writable expression text syntax (D-02b AWAITING_RULING).
// Private-by-default placeholders; no silent defaults (the unknown-dimension
// and unknown-slot-kind placeholder is the uppercase `"TBD"` token the codec
// and schema accept — CONTRACT no-silent-defaults). Lossless: a patched entry
// keeps every member it already carried (provenance, the const-true relaxation
// flags, missing-value diagnostic), and untouched sibling entries round-trip
// verbatim — the editor never silently drops or rewrites a member it does not
// expose. The actual private allowable *value* of a slot is not an authored
// member of this schema version (UserSuppliedValueSlot carries a dimension /
// unit intent and a value_status, not a number); the honest value_status
// marker is exposed instead of inventing a value field.

export type DeclarationDocument = Record<string, unknown>;

// Editable enum vocabularies, copied verbatim from rule_pack.schema.yaml so
// the controls can only emit schema-valid tokens (the codec rejects any
// other). Kept as exported consts (mirroring the composer's operator consts)
// so a schema-conformance test can assert set equality against the schema.
export const SOURCE_KINDS = [
  "solver_result",
  "model_input",
  "user_supplied_rule_value",
  "private_library_value",
  "owner_design_basis",
  "TBD"
] as const;
export const REQUIRED_FOR_TARGETS = ["rule_check", "reporting", "diagnostic", "TBD"] as const;
export const COMPLETENESS_STATUSES = [
  "complete",
  "missing_required_value",
  "missing_unit",
  "missing_provenance",
  "missing_redistribution_status",
  "review_pending",
  "TBD"
] as const;
export const SLOT_KINDS = [
  "allowable",
  "stress_limit",
  "ratio_limit",
  "coefficient",
  "category_threshold",
  "source_reference",
  "TBD"
] as const;
export const VALUE_STATUSES = [
  "not_provided",
  "private_user_supplied",
  "public_permissive_reviewed",
  "invented_non_engineering_example",
  "protected_suspected",
  "TBD"
] as const;

function asObject(value: unknown): Record<string, unknown> | null {
  if (typeof value !== "object" || value === null || Array.isArray(value)) return null;
  return value as Record<string, unknown>;
}

function asString(value: unknown): string | null {
  return typeof value === "string" ? value : null;
}

function asArray(value: unknown): unknown[] {
  return Array.isArray(value) ? value : [];
}

/**
 * Generate a unique declaration id of the form `<prefix>_<n>`, skipping any id
 * already present. Matches the schema `Id` pattern and the draft template's
 * `user_required_input_1` / `user_limit_slot_1` naming.
 */
export function uniqueDeclarationId(prefix: string, existingIds: string[]): string {
  let n = existingIds.length + 1;
  let candidate = `${prefix}_${n}`;
  while (existingIds.includes(candidate)) {
    n += 1;
    candidate = `${prefix}_${n}`;
  }
  return candidate;
}

/** The declared `input_id`s in document order (empty when none/malformed). */
export function readRequiredInputIds(document: DeclarationDocument): string[] {
  return asArray(document.required_inputs)
    .map((entry) => asString(asObject(entry)?.input_id))
    .filter((id): id is string => id !== null);
}

/** The declared `slot_id`s in document order (empty when none/malformed). */
export function readValueSlotIds(document: DeclarationDocument): string[] {
  return asArray(document.value_slots)
    .map((entry) => asString(asObject(entry)?.slot_id))
    .filter((id): id is string => id !== null);
}

/**
 * A schema-valid default required input (RequiredInput): a private placeholder
 * with a fresh unique id and the uppercase `"TBD"` dimension placeholder. The
 * const-true relaxation flags are emitted true (the schema and codec require
 * them); no value is invented.
 */
export function defaultRequiredInput(existingIds: string[]): Record<string, unknown> {
  return {
    input_id: uniqueDeclarationId("user_required_input", existingIds),
    name: "User-defined required input (placeholder)",
    source_kind: "user_supplied_rule_value",
    quantity_intent: {
      dimension: "TBD",
      unit_ref: "TBD",
      unit_required: true,
      dimension_check_required: true
    },
    required_for: "rule_check",
    completeness_status: "missing_required_value",
    missing_value_diagnostic: "RULE_INPUT_MISSING",
    provenance: draftPlaceholderProvenance(),
    redistribution_status: "private_only",
    provenance_required: true,
    redistribution_status_required: true
  };
}

/**
 * A schema-valid default user-supplied value slot (UserSuppliedValueSlot): a
 * private placeholder with a fresh unique id, `"TBD"` slot kind/dimension, and
 * `not_provided` value status (no value is invented).
 */
export function defaultValueSlot(existingIds: string[]): Record<string, unknown> {
  return {
    slot_id: uniqueDeclarationId("user_limit_slot", existingIds),
    slot_kind: "TBD",
    quantity_intent: {
      dimension: "TBD",
      unit_ref: "TBD",
      unit_required: true,
      dimension_check_required: true
    },
    value_status: "not_provided",
    required_for: "rule_check",
    missing_value_diagnostic: "RULE_INPUT_MISSING",
    provenance: draftPlaceholderProvenance(),
    redistribution_status: "private_only",
    provenance_required: true,
    redistribution_status_required: true,
    review_status: "pending",
    completeness_status: "missing_required_value"
  };
}

/** Return a new document with the `required_inputs` array replaced. */
export function setRequiredInputs(
  document: DeclarationDocument,
  inputs: unknown[]
): DeclarationDocument {
  return { ...document, required_inputs: inputs };
}

/** Return a new document with the `value_slots` array replaced. */
export function setValueSlots(
  document: DeclarationDocument,
  slots: unknown[]
): DeclarationDocument {
  return { ...document, value_slots: slots };
}

// ---------------------------------------------------------------------------
// Field primitives
// ---------------------------------------------------------------------------

export function Field({ label, children }: { label: string; children: ReactNode }) {
  return (
    <label className="rule-pack-node-field">
      <span>{label}</span>
      {children}
    </label>
  );
}

export function TextField({
  testId,
  label,
  value,
  disabled,
  onChange
}: {
  testId: string;
  label: string;
  value: string;
  disabled: boolean;
  onChange: (next: string) => void;
}) {
  return (
    <Field label={label}>
      <input
        type="text"
        data-testid={testId}
        value={value}
        disabled={disabled}
        onChange={(event) => onChange(event.target.value)}
      />
    </Field>
  );
}

export function EnumSelect({
  testId,
  label,
  options,
  value,
  disabled,
  onChange
}: {
  testId: string;
  label: string;
  options: readonly string[];
  value: string;
  disabled: boolean;
  onChange: (next: string) => void;
}) {
  // An opened document may carry a token outside the current vocabulary (e.g.
  // from a future schema). Surface it as a "(current)" option rather than
  // silently snapping it to a default — the user sees exactly what is stored.
  const known = options.includes(value);
  return (
    <Field label={label}>
      <select
        data-testid={testId}
        value={value}
        disabled={disabled}
        onChange={(event) => onChange(event.target.value)}
      >
        {!known ? <option value={value}>(current) {value}</option> : null}
        {options.map((option) => (
          <option key={option} value={option}>
            {option}
          </option>
        ))}
      </select>
    </Field>
  );
}

// ---------------------------------------------------------------------------
// Editor
// ---------------------------------------------------------------------------

export type DeclarationsEditorProps = {
  document: DeclarationDocument;
  onChange: (next: DeclarationDocument) => void;
  disabled: boolean;
  disabledReason?: string;
};

/**
 * Structured editor for the rule pack's variable declarations: its
 * `required_inputs` and user-supplied `value_slots`. Edits are written back
 * into a structurally-copied document and handed to `onChange`; the parent
 * owns serialization and persistence (the same contract the expression
 * composer uses).
 */
export function DeclarationsEditor({
  document,
  onChange,
  disabled,
  disabledReason
}: DeclarationsEditorProps) {
  const rawInputs = asArray(document.required_inputs);
  const rawSlots = asArray(document.value_slots);
  const inputIds = readRequiredInputIds(document);
  const slotIds = readValueSlotIds(document);

  // Patch one required input by index; untouched entries round-trip verbatim.
  const updateInput = (index: number, patch: Record<string, unknown>) =>
    onChange(
      setRequiredInputs(
        document,
        rawInputs.map((entry, position) =>
          position === index ? { ...(asObject(entry) ?? {}), ...patch } : entry
        )
      )
    );
  const updateInputQuantity = (index: number, patch: Record<string, unknown>) => {
    const quantity = asObject(asObject(rawInputs[index])?.quantity_intent) ?? {};
    updateInput(index, { quantity_intent: { ...quantity, ...patch } });
  };
  const addInput = () =>
    onChange(setRequiredInputs(document, [...rawInputs, defaultRequiredInput(inputIds)]));
  const removeInput = (index: number) =>
    onChange(
      setRequiredInputs(
        document,
        rawInputs.filter((_, position) => position !== index)
      )
    );

  const updateSlot = (index: number, patch: Record<string, unknown>) =>
    onChange(
      setValueSlots(
        document,
        rawSlots.map((entry, position) =>
          position === index ? { ...(asObject(entry) ?? {}), ...patch } : entry
        )
      )
    );
  const updateSlotQuantity = (index: number, patch: Record<string, unknown>) => {
    const quantity = asObject(asObject(rawSlots[index])?.quantity_intent) ?? {};
    updateSlot(index, { quantity_intent: { ...quantity, ...patch } });
  };
  const addSlot = () =>
    onChange(setValueSlots(document, [...rawSlots, defaultValueSlot(slotIds)]));
  const removeSlot = (index: number) =>
    onChange(
      setValueSlots(
        document,
        rawSlots.filter((_, position) => position !== index)
      )
    );

  const inputRemoveReason =
    rawInputs.length <= 1 ? "A rule pack needs at least one required input." : undefined;
  const slotRemoveReason =
    rawSlots.length <= 1 ? "A rule pack needs at least one value slot." : undefined;

  return (
    <div className="report-list rule-pack-declarations-editor" data-testid="rule-pack-declarations-editor">
      <strong>Variable declarations (structured; no text syntax — D-02b)</strong>
      <small className="rule-pack-node-readonly" data-testid="rule-pack-declarations-guidance">
        Declare the rule pack&apos;s required inputs (bound from solver results or model inputs at
        check time) and user-supplied value slots (private allowables/limits). The expression
        composer&apos;s <code>variable_ref</code> picker binds to these ids. Values stay private; the
        software never fills in code or standards values.
      </small>

      <div data-testid="rule-pack-required-inputs">
        <small>Required inputs ({rawInputs.length})</small>
        {rawInputs.map((entry, index) => {
          const record = asObject(entry) ?? {};
          const quantity = asObject(record.quantity_intent) ?? {};
          return (
            <div
              className="rule-pack-node operation-record"
              data-testid="rule-pack-required-input-row"
              key={`input-${index}`}
            >
              <TextField
                testId="rule-pack-input-id"
                label="input_id"
                value={asString(record.input_id) ?? ""}
                disabled={disabled}
                onChange={(next) => updateInput(index, { input_id: next })}
              />
              <TextField
                testId="rule-pack-input-name"
                label="name"
                value={asString(record.name) ?? ""}
                disabled={disabled}
                onChange={(next) => updateInput(index, { name: next })}
              />
              <EnumSelect
                testId="rule-pack-input-source-kind"
                label="source_kind"
                options={SOURCE_KINDS}
                value={asString(record.source_kind) ?? "TBD"}
                disabled={disabled}
                onChange={(next) => updateInput(index, { source_kind: next })}
              />
              <Field label="dimension">
                <DimensionSelect
                  testId="rule-pack-input-dimension"
                  value={asString(quantity.dimension) ?? "TBD"}
                  disabled={disabled}
                  onChange={(next) => updateInputQuantity(index, { dimension: next })}
                />
              </Field>
              <TextField
                testId="rule-pack-input-unit"
                label="unit_ref"
                value={asString(quantity.unit_ref) ?? "TBD"}
                disabled={disabled}
                onChange={(next) => updateInputQuantity(index, { unit_ref: next })}
              />
              <EnumSelect
                testId="rule-pack-input-required-for"
                label="required_for"
                options={REQUIRED_FOR_TARGETS}
                value={asString(record.required_for) ?? "TBD"}
                disabled={disabled}
                onChange={(next) => updateInput(index, { required_for: next })}
              />
              <EnumSelect
                testId="rule-pack-input-completeness"
                label="completeness_status"
                options={COMPLETENESS_STATUSES}
                value={asString(record.completeness_status) ?? "TBD"}
                disabled={disabled}
                onChange={(next) => updateInput(index, { completeness_status: next })}
              />
              <button
                type="button"
                data-testid="rule-pack-input-remove"
                disabled={disabled || rawInputs.length <= 1}
                title={disabled ? disabledReason : inputRemoveReason}
                onClick={() => removeInput(index)}
              >
                Remove input
              </button>
            </div>
          );
        })}
        <button
          type="button"
          data-testid="rule-pack-input-add"
          disabled={disabled}
          title={disabled ? disabledReason : undefined}
          onClick={addInput}
        >
          Add required input
        </button>
      </div>

      <div data-testid="rule-pack-value-slots">
        <small>Value slots ({rawSlots.length})</small>
        {rawSlots.map((entry, index) => {
          const record = asObject(entry) ?? {};
          const quantity = asObject(record.quantity_intent) ?? {};
          return (
            <div
              className="rule-pack-node operation-record"
              data-testid="rule-pack-value-slot-row"
              key={`slot-${index}`}
            >
              <TextField
                testId="rule-pack-slot-id"
                label="slot_id"
                value={asString(record.slot_id) ?? ""}
                disabled={disabled}
                onChange={(next) => updateSlot(index, { slot_id: next })}
              />
              <EnumSelect
                testId="rule-pack-slot-kind"
                label="slot_kind"
                options={SLOT_KINDS}
                value={asString(record.slot_kind) ?? "TBD"}
                disabled={disabled}
                onChange={(next) => updateSlot(index, { slot_kind: next })}
              />
              <Field label="dimension">
                <DimensionSelect
                  testId="rule-pack-slot-dimension"
                  value={asString(quantity.dimension) ?? "TBD"}
                  disabled={disabled}
                  onChange={(next) => updateSlotQuantity(index, { dimension: next })}
                />
              </Field>
              <TextField
                testId="rule-pack-slot-unit"
                label="unit_ref"
                value={asString(quantity.unit_ref) ?? "TBD"}
                disabled={disabled}
                onChange={(next) => updateSlotQuantity(index, { unit_ref: next })}
              />
              <EnumSelect
                testId="rule-pack-slot-value-status"
                label="value_status"
                options={VALUE_STATUSES}
                value={asString(record.value_status) ?? "TBD"}
                disabled={disabled}
                onChange={(next) => updateSlot(index, { value_status: next })}
              />
              <EnumSelect
                testId="rule-pack-slot-required-for"
                label="required_for"
                options={REQUIRED_FOR_TARGETS}
                value={asString(record.required_for) ?? "TBD"}
                disabled={disabled}
                onChange={(next) => updateSlot(index, { required_for: next })}
              />
              <EnumSelect
                testId="rule-pack-slot-completeness"
                label="completeness_status"
                options={COMPLETENESS_STATUSES}
                value={asString(record.completeness_status) ?? "TBD"}
                disabled={disabled}
                onChange={(next) => updateSlot(index, { completeness_status: next })}
              />
              <button
                type="button"
                data-testid="rule-pack-slot-remove"
                disabled={disabled || rawSlots.length <= 1}
                title={disabled ? disabledReason : slotRemoveReason}
                onClick={() => removeSlot(index)}
              >
                Remove slot
              </button>
            </div>
          );
        })}
        <button
          type="button"
          data-testid="rule-pack-slot-add"
          disabled={disabled}
          title={disabled ? disabledReason : undefined}
          onClick={addSlot}
        >
          Add value slot
        </button>
      </div>
    </div>
  );
}
