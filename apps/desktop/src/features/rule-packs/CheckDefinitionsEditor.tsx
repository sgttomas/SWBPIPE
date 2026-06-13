import { draftPlaceholderProvenance } from "../../services/rulePackService";
import {
  EnumSelect,
  Field,
  TextField,
  readRequiredInputIds,
  readValueSlotIds,
  uniqueDeclarationId,
  type DeclarationDocument
} from "./DeclarationsEditor";

// Rule-pack check-definitions editor (Phase C2 slice 5, TP-C2-CHECKDEF-001).
//
// PRD §14.5 requires the rule-pack editor to let a user author the pack's
// acceptability checks. Slices 1-4 authored the metadata, the formula
// expression AST, and the variable declarations (`required_inputs` /
// `value_slots`); `check_definitions` — the member that binds those
// declarations and a formula into a check — was the last document-structure
// member still hand-edited as raw document JSON. This component replaces that
// hand-editing with structured form controls: add / remove / edit each check,
// binding its `required_input_refs` + `value_slot_refs` to the declared ids and
// its `formula_ref` to a declared formula, then choosing the
// `acceptability_basis`, the `result_statuses` it may emit, and the
// `diagnostic_policy` code for each failure condition.
//
// Boundary (unchanged from the prior C2 slices): purely structured form
// controls — no writable expression text syntax (D-02b AWAITING_RULING); a
// check binds *references* and selects closed-vocabulary tokens, never any
// expression text. Private-by-default placeholder provenance; no silent
// defaults (an out-of-vocabulary stored enum surfaces as a "(current)" option,
// an unresolved reference as an "(unresolved)" option — never snapped). The
// software authors the check's *shape*; it never decides pass/fail acceptance
// (the engine independently blocks incomplete checks — C4) and never fills in a
// code or standards value. Lossless: a patched check keeps every member it
// already carried (provenance, description, any extra ref fields), and
// untouched sibling checks round-trip verbatim.

// Editable enum vocabularies, copied verbatim from rule_pack.schema.yaml so the
// controls can only emit schema-valid tokens (the codec rejects any other).
// Exported so a schema-conformance test can assert set equality against the
// schema.
export const ACCEPTABILITY_BASES = [
  "user_supplied_rule_pack",
  "owner_design_basis",
  "private_project_basis",
  "invented_non_engineering_example",
  "TBD"
] as const;

export const ANALYSIS_STATUSES = [
  "MODEL_INCOMPLETE",
  "MECHANICS_SOLVED",
  "RULE_INPUTS_INCOMPLETE",
  "USER_RULE_CHECKED",
  "USER_RULE_FAILED",
  "HUMAN_REVIEW_REQUIRED"
] as const;

export const DIAGNOSTIC_CODES = [
  "RULE_CHECK_BLOCKING",
  "RULE_INPUT_MISSING",
  "RULE_UNIT_MISMATCH",
  "RULE_PROVENANCE_WARNING",
  "RULE_REDISTRIBUTION_WARNING",
  "RULE_CHECKSUM_MISMATCH",
  "RULE_PROTECTED_CONTENT_WARNING",
  "RULE_EVALUATOR_ERROR",
  "RULE_INCOMPLETE_DATA",
  "RULE_PROFESSIONAL_BOUNDARY_NOTICE"
] as const;

// The eight DiagnosticPolicy fields, in schema order. Each maps a rule-check
// condition to the diagnostic code emitted for it. Exported so a
// schema-conformance test can assert set equality against the schema's
// DiagnosticPolicy required members.
export const DIAGNOSTIC_POLICY_FIELDS = [
  "rule_check_blocking",
  "missing_input",
  "unit_mismatch",
  "provenance_gap",
  "redistribution_gap",
  "checksum_mismatch",
  "protected_content_suspected",
  "evaluator_error"
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

function asStringArray(value: unknown): string[] {
  return asArray(value).filter((entry): entry is string => typeof entry === "string");
}

function isKnownStatus(status: string): boolean {
  return (ANALYSIS_STATUSES as readonly string[]).includes(status);
}

/** The declared `check_id`s in document order (empty when none/malformed). */
export function readCheckIds(document: DeclarationDocument): string[] {
  return asArray(document.check_definitions)
    .map((entry) => asString(asObject(entry)?.check_id))
    .filter((id): id is string => id !== null);
}

/** The declared `formula_id`s in document order (empty when none/malformed). */
export function readFormulaDeclarationIds(document: DeclarationDocument): string[] {
  return asArray(document.formula_declarations)
    .map((entry) => asString(asObject(entry)?.formula_id))
    .filter((id): id is string => id !== null);
}

/**
 * A schema-valid default DiagnosticPolicy (all eight condition→code bindings),
 * byte-identical to the draft template's policy in rulePackService. Returns a
 * fresh object each call.
 */
export function defaultDiagnosticPolicy(): Record<string, string> {
  return {
    rule_check_blocking: "RULE_CHECK_BLOCKING",
    missing_input: "RULE_INPUT_MISSING",
    unit_mismatch: "RULE_UNIT_MISMATCH",
    provenance_gap: "RULE_PROVENANCE_WARNING",
    redistribution_gap: "RULE_REDISTRIBUTION_WARNING",
    checksum_mismatch: "RULE_CHECKSUM_MISMATCH",
    protected_content_suspected: "RULE_PROTECTED_CONTENT_WARNING",
    evaluator_error: "RULE_EVALUATOR_ERROR"
  };
}

/**
 * A schema-valid default CheckDefinition: a fresh unique id, its single-ref
 * arrays bound to the first declared input / slot / formula (or the uppercase
 * `"TBD"` placeholder the schema `Id` pattern accepts when nothing is declared
 * yet — surfaced as "(unresolved)", never a silent binding), the
 * user-rule-pack acceptability basis, the user-rule result statuses, the
 * default diagnostic policy, and private-by-default placeholder provenance. No
 * numeric value, code, or standards content is invented.
 */
export function defaultCheckDefinition(
  existingCheckIds: string[],
  inputIds: string[],
  slotIds: string[],
  formulaIds: string[]
): Record<string, unknown> {
  return {
    check_id: uniqueDeclarationId("user_check", existingCheckIds),
    name: "User-defined check (placeholder)",
    description: "Replace with the user-defined acceptability check.",
    required_input_refs: [{ ref_id: inputIds[0] ?? "TBD", ref_type: "required_input" }],
    value_slot_refs: [{ ref_id: slotIds[0] ?? "TBD", ref_type: "value_slot" }],
    formula_ref: { ref_id: formulaIds[0] ?? "TBD", ref_type: "formula" },
    acceptability_basis: "user_supplied_rule_pack",
    result_statuses: [
      "RULE_INPUTS_INCOMPLETE",
      "USER_RULE_CHECKED",
      "USER_RULE_FAILED",
      "HUMAN_REVIEW_REQUIRED"
    ],
    diagnostic_policy: defaultDiagnosticPolicy(),
    provenance: draftPlaceholderProvenance()
  };
}

/** Return a new document with the `check_definitions` array replaced. */
export function setCheckDefinitions(
  document: DeclarationDocument,
  checks: unknown[]
): DeclarationDocument {
  return { ...document, check_definitions: checks };
}

// ---------------------------------------------------------------------------
// Field primitives specific to this editor (reference picker + status toggles)
// ---------------------------------------------------------------------------

function RefSelect({
  testId,
  options,
  value,
  disabled,
  onChange
}: {
  testId: string;
  options: string[];
  value: string;
  disabled: boolean;
  onChange: (next: string) => void;
}) {
  // A ref may point at an id that is not (or no longer) declared — a
  // declaration was renamed/removed, or the default "TBD" placeholder stands in
  // because nothing is declared yet. Surface it as an "(unresolved)" option
  // rather than silently snapping the ref to the first declared id; the backend
  // validation independently flags dangling references.
  const known = options.includes(value);
  return (
    <select
      data-testid={testId}
      value={value}
      disabled={disabled}
      onChange={(event) => onChange(event.target.value)}
    >
      {!known ? <option value={value}>(unresolved) {value || "(empty)"}</option> : null}
      {options.map((option) => (
        <option key={option} value={option}>
          {option}
        </option>
      ))}
    </select>
  );
}

function ResultStatusToggles({
  selected,
  disabled,
  disabledReason,
  onToggle
}: {
  selected: string[];
  disabled: boolean;
  disabledReason?: string;
  onToggle: (status: string, on: boolean) => void;
}) {
  // One checkbox per known AnalysisStatus, plus any stored token outside the
  // current vocabulary (kept visible and removable as a "(current)" option,
  // never silently dropped). The minItems:1 schema floor blocks unchecking the
  // last selected status.
  const unknown = selected.filter((status) => !isKnownStatus(status));
  const options = [...ANALYSIS_STATUSES, ...unknown];
  const floorReason = "A check needs at least one result status.";
  return (
    <div className="rule-pack-check-statuses" data-testid="rule-pack-check-result-statuses">
      <span className="rule-pack-node-label">result_statuses</span>
      {options.map((status) => {
        const checked = selected.includes(status);
        const blockUncheck = checked && selected.length <= 1;
        return (
          <label className="rule-pack-node-field" key={status}>
            <input
              type="checkbox"
              data-testid={`rule-pack-check-status-${status}`}
              checked={checked}
              disabled={disabled || blockUncheck}
              title={disabled ? disabledReason : blockUncheck ? floorReason : undefined}
              onChange={(event) => onToggle(status, event.target.checked)}
            />
            <span>{isKnownStatus(status) ? status : `(current) ${status}`}</span>
          </label>
        );
      })}
    </div>
  );
}

// ---------------------------------------------------------------------------
// Editor
// ---------------------------------------------------------------------------

export type CheckDefinitionsEditorProps = {
  document: DeclarationDocument;
  onChange: (next: DeclarationDocument) => void;
  disabled: boolean;
  disabledReason?: string;
};

/**
 * Structured editor for the rule pack's `check_definitions`. Edits are written
 * back into a structurally-copied document and handed to `onChange`; the parent
 * owns serialization and persistence (the same contract the declarations editor
 * and expression composer use).
 */
export function CheckDefinitionsEditor({
  document,
  onChange,
  disabled,
  disabledReason
}: CheckDefinitionsEditorProps) {
  const rawChecks = asArray(document.check_definitions);
  const checkIds = readCheckIds(document);
  const inputIds = readRequiredInputIds(document);
  const slotIds = readValueSlotIds(document);
  const formulaIds = readFormulaDeclarationIds(document);

  // Patch one check by index; untouched checks round-trip verbatim.
  const updateCheck = (index: number, patch: Record<string, unknown>) =>
    onChange(
      setCheckDefinitions(
        document,
        rawChecks.map((entry, position) =>
          position === index ? { ...(asObject(entry) ?? {}), ...patch } : entry
        )
      )
    );

  const addCheck = () =>
    onChange(
      setCheckDefinitions(document, [
        ...rawChecks,
        defaultCheckDefinition(checkIds, inputIds, slotIds, formulaIds)
      ])
    );

  const removeCheck = (index: number) =>
    onChange(
      setCheckDefinitions(
        document,
        rawChecks.filter((_, position) => position !== index)
      )
    );

  // Reference-array editing (required_input_refs / value_slot_refs). A ref edit
  // preserves any sibling members the ref already carried (e.g. a Reference
  // `version`) via structural spread; untouched refs round-trip verbatim.
  const readRefs = (index: number, member: string): unknown[] =>
    asArray(asObject(rawChecks[index])?.[member]);

  const updateRef = (index: number, member: string, refIndex: number, refId: string) =>
    updateCheck(index, {
      [member]: readRefs(index, member).map((ref, position) =>
        position === refIndex ? { ...(asObject(ref) ?? {}), ref_id: refId } : ref
      )
    });

  const addRef = (index: number, member: string, refType: string, optionIds: string[]) =>
    updateCheck(index, {
      [member]: [...readRefs(index, member), { ref_id: optionIds[0] ?? "TBD", ref_type: refType }]
    });

  const removeRef = (index: number, member: string, refIndex: number) => {
    const refs = readRefs(index, member);
    if (refs.length <= 1) return;
    updateCheck(index, { [member]: refs.filter((_, position) => position !== refIndex) });
  };

  const updateFormulaRef = (index: number, refId: string) => {
    const existing = asObject(asObject(rawChecks[index])?.formula_ref) ?? {};
    updateCheck(index, { formula_ref: { ...existing, ref_id: refId } });
  };

  const updateDiagnosticPolicy = (index: number, field: string, code: string) => {
    const policy = asObject(asObject(rawChecks[index])?.diagnostic_policy) ?? {};
    updateCheck(index, { diagnostic_policy: { ...policy, [field]: code } });
  };

  const toggleResultStatus = (index: number, status: string, on: boolean) => {
    const current = asStringArray(asObject(rawChecks[index])?.result_statuses);
    if (on) {
      if (current.includes(status)) return;
      updateCheck(index, { result_statuses: [...current, status] });
    } else {
      if (current.length <= 1) return; // minItems:1 schema floor
      updateCheck(index, { result_statuses: current.filter((entry) => entry !== status) });
    }
  };

  const checkRemoveReason =
    rawChecks.length <= 1 ? "A rule pack needs at least one check definition." : undefined;

  return (
    <div
      className="report-list rule-pack-check-definitions-editor"
      data-testid="rule-pack-check-definitions-editor"
    >
      <strong>Acceptability checks (structured; no text syntax — D-02b)</strong>
      <small className="rule-pack-node-readonly" data-testid="rule-pack-check-definitions-guidance">
        Bind each check to the required inputs and value slots it consumes and the formula that
        computes it, then choose the acceptability basis, the result statuses the check may emit, and
        the diagnostic code for each failure condition. Pickers bind to the ids declared above; the
        software authors the check shape only — it never decides pass/fail acceptance or fills in a
        code or standards value.
      </small>

      <div data-testid="rule-pack-check-definitions">
        <small>Check definitions ({rawChecks.length})</small>
        {rawChecks.map((entry, index) => {
          const record = asObject(entry) ?? {};
          const inputRefs = asArray(record.required_input_refs);
          const slotRefs = asArray(record.value_slot_refs);
          const formulaRef = asObject(record.formula_ref) ?? {};
          const policy = asObject(record.diagnostic_policy) ?? {};
          const resultStatuses = asStringArray(record.result_statuses);
          return (
            <div
              className="rule-pack-node operation-record"
              data-testid="rule-pack-check-row"
              key={`check-${index}`}
            >
              <TextField
                testId="rule-pack-check-id"
                label="check_id"
                value={asString(record.check_id) ?? ""}
                disabled={disabled}
                onChange={(next) => updateCheck(index, { check_id: next })}
              />
              <TextField
                testId="rule-pack-check-name"
                label="name"
                value={asString(record.name) ?? ""}
                disabled={disabled}
                onChange={(next) => updateCheck(index, { name: next })}
              />
              <TextField
                testId="rule-pack-check-description"
                label="description"
                value={asString(record.description) ?? ""}
                disabled={disabled}
                onChange={(next) => updateCheck(index, { description: next })}
              />

              <Field label="formula_ref">
                <RefSelect
                  testId="rule-pack-check-formula-ref"
                  options={formulaIds}
                  value={asString(formulaRef.ref_id) ?? ""}
                  disabled={disabled}
                  onChange={(next) => updateFormulaRef(index, next)}
                />
              </Field>

              <div className="rule-pack-check-subgroup" data-testid="rule-pack-check-input-refs">
                <span className="rule-pack-node-label">required_input_refs</span>
                {inputRefs.map((ref, refIndex) => (
                  <div className="rule-pack-node-field" key={`input-ref-${refIndex}`}>
                    <RefSelect
                      testId="rule-pack-check-input-ref"
                      options={inputIds}
                      value={asString(asObject(ref)?.ref_id) ?? ""}
                      disabled={disabled}
                      onChange={(next) => updateRef(index, "required_input_refs", refIndex, next)}
                    />
                    <button
                      type="button"
                      data-testid="rule-pack-check-input-ref-remove"
                      disabled={disabled || inputRefs.length <= 1}
                      title={
                        disabled
                          ? disabledReason
                          : inputRefs.length <= 1
                            ? "A check needs at least one required-input ref."
                            : undefined
                      }
                      onClick={() => removeRef(index, "required_input_refs", refIndex)}
                    >
                      Remove
                    </button>
                  </div>
                ))}
                <button
                  type="button"
                  data-testid="rule-pack-check-input-ref-add"
                  disabled={disabled}
                  title={disabled ? disabledReason : undefined}
                  onClick={() => addRef(index, "required_input_refs", "required_input", inputIds)}
                >
                  Add input ref
                </button>
              </div>

              <div className="rule-pack-check-subgroup" data-testid="rule-pack-check-slot-refs">
                <span className="rule-pack-node-label">value_slot_refs</span>
                {slotRefs.map((ref, refIndex) => (
                  <div className="rule-pack-node-field" key={`slot-ref-${refIndex}`}>
                    <RefSelect
                      testId="rule-pack-check-slot-ref"
                      options={slotIds}
                      value={asString(asObject(ref)?.ref_id) ?? ""}
                      disabled={disabled}
                      onChange={(next) => updateRef(index, "value_slot_refs", refIndex, next)}
                    />
                    <button
                      type="button"
                      data-testid="rule-pack-check-slot-ref-remove"
                      disabled={disabled || slotRefs.length <= 1}
                      title={
                        disabled
                          ? disabledReason
                          : slotRefs.length <= 1
                            ? "A check needs at least one value-slot ref."
                            : undefined
                      }
                      onClick={() => removeRef(index, "value_slot_refs", refIndex)}
                    >
                      Remove
                    </button>
                  </div>
                ))}
                <button
                  type="button"
                  data-testid="rule-pack-check-slot-ref-add"
                  disabled={disabled}
                  title={disabled ? disabledReason : undefined}
                  onClick={() => addRef(index, "value_slot_refs", "value_slot", slotIds)}
                >
                  Add slot ref
                </button>
              </div>

              <EnumSelect
                testId="rule-pack-check-acceptability-basis"
                label="acceptability_basis"
                options={ACCEPTABILITY_BASES}
                value={asString(record.acceptability_basis) ?? "TBD"}
                disabled={disabled}
                onChange={(next) => updateCheck(index, { acceptability_basis: next })}
              />

              <ResultStatusToggles
                selected={resultStatuses}
                disabled={disabled}
                disabledReason={disabledReason}
                onToggle={(status, on) => toggleResultStatus(index, status, on)}
              />

              <div className="rule-pack-check-subgroup" data-testid="rule-pack-check-diagnostic-policy">
                <span className="rule-pack-node-label">diagnostic_policy</span>
                {DIAGNOSTIC_POLICY_FIELDS.map((field) => (
                  <EnumSelect
                    key={field}
                    testId={`rule-pack-check-policy-${field}`}
                    label={field}
                    options={DIAGNOSTIC_CODES}
                    value={asString(policy[field]) ?? ""}
                    disabled={disabled}
                    onChange={(next) => updateDiagnosticPolicy(index, field, next)}
                  />
                ))}
              </div>

              <button
                type="button"
                data-testid="rule-pack-check-remove"
                disabled={disabled || rawChecks.length <= 1}
                title={disabled ? disabledReason : checkRemoveReason}
                onClick={() => removeCheck(index)}
              >
                Remove check
              </button>
            </div>
          );
        })}
        <button
          type="button"
          data-testid="rule-pack-check-add"
          disabled={disabled}
          title={disabled ? disabledReason : undefined}
          onClick={addCheck}
        >
          Add check
        </button>
      </div>
    </div>
  );
}
