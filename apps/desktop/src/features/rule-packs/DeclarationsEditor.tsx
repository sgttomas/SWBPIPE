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
//
// Library references (TP-C3-LIBREFAUTHOR-001): a required input whose
// source_kind is `private_library_value` additionally authors a
// `library_value_ref` (library_kind / library_id / record_id / slot_id) — the
// pointer the C4 rule-check runner resolves from the local private-library
// store at run time (TP-C3C4-LIBREF-001). The reference is carried; the private
// value is NEVER embedded in the rule-pack document (IP boundary, PRD
// §13/§17.3), so authoring a reference here never redistributes a private value.
//
// Solver-result references (TP-C4-SOLVERREFAUTHOR-001): a required input whose
// source_kind is `solver_result` additionally authors a `solver_result_ref`
// (a single `result_id`) — the pointer the C4 rule-check runner resolves from
// the solved result envelope's `results[]` by id at run time
// (TP-C4-SOLVERREF-001). It is the in-pack, canonical form of the previously
// caller-supplied `{input_id, result_id}` solver binding (PRD §12.5): an
// authored reference governs its input alone and supersedes the run panel's
// per-input result selector. Removing it returns the input to the run-panel
// binding path. An unfilled/unresolvable reference leaves the input unsupplied
// so the check blocks at RULE_INPUTS_INCOMPLETE — never a silent pass.
//
// Both reference members are additive and optional; they change nothing for
// packs that do not use them. Both are now ratified parts of the schema:
// `library_value_ref` (DEC-038) and `solver_result_ref` (DEC-039), carried at
// rule-pack `schema_version` 0.4.0.

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
// LibraryValueRef.library_kind vocabulary, copied verbatim from
// rule_pack.schema.yaml. Unlike the other declaration enums this one has NO
// "TBD" member (the schema offers only the three concrete kinds), so a seeded
// reference defaults library_kind to the first kind and leaves the *ids* as
// visible "TBD" placeholders instead.
export const LIBRARY_KINDS = ["material", "section", "component"] as const;

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

/**
 * A schema-valid default `library_value_ref` (LibraryValueRef): the reference a
 * `private_library_value` required input uses to point at a value held in a
 * saved private library record. All four members are required by the schema, so
 * the placeholder carries them all: `library_kind` defaults to the first kind
 * (the only kind the runner resolves today — material allowable slots), and the
 * three ids are the visible uppercase `"TBD"` placeholder the `Id` pattern
 * accepts, so an unfilled reference resolves to nothing and the input blocks
 * (never a silent pass). The private value itself is NEVER stored here — only
 * the reference; it is read from the local library store at run time (IP
 * boundary, PRD §13/§17.3).
 */
export function defaultLibraryValueRef(): Record<string, unknown> {
  return {
    library_kind: LIBRARY_KINDS[0],
    library_id: "TBD",
    record_id: "TBD",
    slot_id: "TBD"
  };
}

/**
 * A schema-shaped default `solver_result_ref` (SolverResultRef): the single
 * required `result_id` member as the visible uppercase `"TBD"` placeholder the
 * `Id` pattern accepts. An unfilled reference matches no result row, so the
 * input stays unsupplied and the check blocks (never a silent pass). The
 * authored reference is the canonical, in-pack form of the previously
 * caller-supplied `{input_id, result_id}` solver binding (PRD §12.5) and
 * supersedes the run panel's per-input result selector; no result *value* is
 * ever stored here, only the row id resolved at run time from the solved
 * envelope.
 */
export function defaultSolverResultRef(): Record<string, unknown> {
  return {
    result_id: "TBD"
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
  // Change a required input's source_kind. Switching TO a kind that uses a
  // reference seeds a placeholder reference when none exists yet, so its
  // sub-form opens with a complete, schema-valid (but unresolved, hence
  // blocking) reference rather than a partial one: private_library_value seeds
  // a library_value_ref, solver_result seeds a solver_result_ref. Switching
  // away leaves any existing reference in place (lossless); its sub-form stays
  // visible so a now-meaningless reference is never silently hidden, and the
  // explicit "Remove ... reference" control lets the user drop it.
  const changeInputSourceKind = (index: number, next: string) => {
    const record = asObject(rawInputs[index]) ?? {};
    const patch: Record<string, unknown> = { source_kind: next };
    if (next === "private_library_value" && asObject(record.library_value_ref) === null) {
      patch.library_value_ref = defaultLibraryValueRef();
    }
    if (next === "solver_result" && asObject(record.solver_result_ref) === null) {
      patch.solver_result_ref = defaultSolverResultRef();
    }
    updateInput(index, patch);
  };
  // Patch one input's library_value_ref, merging into the existing reference
  // (or a fresh default when none exists yet — so the first field edit always
  // produces a complete four-member reference, never a schema-invalid partial).
  const updateInputLibraryRef = (index: number, patch: Record<string, unknown>) => {
    const ref = asObject(asObject(rawInputs[index])?.library_value_ref) ?? defaultLibraryValueRef();
    updateInput(index, { library_value_ref: { ...ref, ...patch } });
  };
  // Drop the library_value_ref member entirely — the only member-removing edit
  // the editor exposes, used when an input no longer references a library. Every
  // other member of the input round-trips verbatim.
  const removeInputLibraryRef = (index: number) =>
    onChange(
      setRequiredInputs(
        document,
        rawInputs.map((entry, position) => {
          if (position !== index) return entry;
          const rest = { ...(asObject(entry) ?? {}) };
          delete rest.library_value_ref;
          return rest;
        })
      )
    );
  // Patch one input's solver_result_ref, merging into the existing reference
  // (or a fresh default when none exists yet — so the first field edit always
  // produces a complete reference, never a schema-invalid empty object).
  const updateInputSolverRef = (index: number, patch: Record<string, unknown>) => {
    const ref = asObject(asObject(rawInputs[index])?.solver_result_ref) ?? defaultSolverResultRef();
    updateInput(index, { solver_result_ref: { ...ref, ...patch } });
  };
  // Drop the solver_result_ref member entirely — returns the input to the
  // run-panel caller-supplied binding path. Every other member round-trips
  // verbatim.
  const removeInputSolverRef = (index: number) =>
    onChange(
      setRequiredInputs(
        document,
        rawInputs.map((entry, position) => {
          if (position !== index) return entry;
          const rest = { ...(asObject(entry) ?? {}) };
          delete rest.solver_result_ref;
          return rest;
        })
      )
    );
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
          const sourceKind = asString(record.source_kind) ?? "TBD";
          const libraryRef = asObject(record.library_value_ref);
          const solverRef = asObject(record.solver_result_ref);
          // Show the private-library reference sub-form whenever the input draws
          // from a private library, OR whenever a reference is already present
          // (so a reference left over from a since-changed source_kind stays
          // visible and removable rather than silently hidden).
          const showLibraryRef = sourceKind === "private_library_value" || libraryRef !== null;
          // Same rule for the solver-result reference sub-form: shown for a
          // solver_result input, or whenever a reference lingers from a
          // since-changed source_kind (visible and removable, never hidden).
          const showSolverRef = sourceKind === "solver_result" || solverRef !== null;
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
                onChange={(next) => changeInputSourceKind(index, next)}
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
              {showLibraryRef ? (
                <div className="rule-pack-node" data-testid="rule-pack-input-library-ref">
                  <small
                    className="rule-pack-node-readonly"
                    data-testid="rule-pack-input-library-ref-note"
                  >
                    Private library reference. The referenced value is resolved at check-run time
                    from the local private library store; it stays in the library and is never
                    embedded in the rule pack.
                  </small>
                  <EnumSelect
                    testId="rule-pack-input-library-kind"
                    label="library_kind"
                    options={LIBRARY_KINDS}
                    value={asString(libraryRef?.library_kind) ?? LIBRARY_KINDS[0]}
                    disabled={disabled}
                    onChange={(next) => updateInputLibraryRef(index, { library_kind: next })}
                  />
                  <TextField
                    testId="rule-pack-input-library-id"
                    label="library_id"
                    value={asString(libraryRef?.library_id) ?? ""}
                    disabled={disabled}
                    onChange={(next) => updateInputLibraryRef(index, { library_id: next })}
                  />
                  <TextField
                    testId="rule-pack-input-library-record"
                    label="record_id"
                    value={asString(libraryRef?.record_id) ?? ""}
                    disabled={disabled}
                    onChange={(next) => updateInputLibraryRef(index, { record_id: next })}
                  />
                  <TextField
                    testId="rule-pack-input-library-slot"
                    label="slot_id"
                    value={asString(libraryRef?.slot_id) ?? ""}
                    disabled={disabled}
                    onChange={(next) => updateInputLibraryRef(index, { slot_id: next })}
                  />
                  <button
                    type="button"
                    data-testid="rule-pack-input-library-remove"
                    disabled={disabled || libraryRef === null}
                    title={
                      disabled
                        ? disabledReason
                        : libraryRef === null
                          ? "No library reference to remove."
                          : undefined
                    }
                    onClick={() => removeInputLibraryRef(index)}
                  >
                    Remove library reference
                  </button>
                </div>
              ) : null}
              {showSolverRef ? (
                <div className="rule-pack-node" data-testid="rule-pack-input-solver-ref">
                  <small
                    className="rule-pack-node-readonly"
                    data-testid="rule-pack-input-solver-ref-note"
                  >
                    Solver-result reference. Binds this input to a specific row of the solved result
                    envelope by result id, resolved at check-run time. An authored reference is the
                    canonical binding — it supersedes the run panel&apos;s per-input result selector;
                    remove it to bind from the run panel instead.
                  </small>
                  <TextField
                    testId="rule-pack-input-solver-result-id"
                    label="result_id"
                    value={asString(solverRef?.result_id) ?? ""}
                    disabled={disabled}
                    onChange={(next) => updateInputSolverRef(index, { result_id: next })}
                  />
                  <button
                    type="button"
                    data-testid="rule-pack-input-solver-remove"
                    disabled={disabled || solverRef === null}
                    title={
                      disabled
                        ? disabledReason
                        : solverRef === null
                          ? "No solver-result reference to remove."
                          : undefined
                    }
                    onClick={() => removeInputSolverRef(index)}
                  >
                    Remove solver-result reference
                  </button>
                </div>
              ) : null}
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
