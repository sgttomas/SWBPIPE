import { useEffect, useMemo, useState, type ReactNode } from "react";
import { DimensionSelect } from "./ExpressionComposer";
import {
  BASIS_TYPE_WORDS,
  COMPLETENESS_WORDS,
  LIBRARY_KIND_WORDS,
  PART_WORDS,
  REQUIRED_FOR_WORDS,
  SLOT_KIND_WORDS,
  SOURCE_KIND_WORDS,
  VALUE_STATUS_WORDS,
  fieldWord,
  wordFor,
  type Words
} from "./rulePackWords";
import { draftPlaceholderProvenance } from "../../services/rulePackService";
import {
  SOLVER_SELECTOR_SECTION_BASES,
  describeSolverSelector,
  solverSelectorForRow,
  type SolverResultRef
} from "../../services/ruleCheckService";
import type { MechanicsResult, PreviewModel } from "../../types";
import {
  acceptedUnits,
  describeUnitBasis,
  loadUnitCatalog,
  unitDimensionValidationStatus,
  unitEntryMatchesDimension,
  type UnitCatalogRoute
} from "../../services/unitCatalogService";

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
// Solver-row selectors (rule-pack format 0.5.0; DEL-06-01 Design
// `rule-pack-0.5.0.md` §2.5, §3): a required input whose source_kind is
// `solver_result` authors a `solver_result_ref` row selector — result_kind,
// entity_ref and the optional location, component, section_basis and tie_rod —
// that the run command resolves on each of a check's required load cases
// against the solved envelope. It is the one binding route (the run panel's
// caller selector is retired); a solver_result input without it is refused by
// validation (RULE_SOLVER_RESULT_REF_MISSING). The members are typed; when the
// session has a current solved result the editor also offers the selectors of
// its rows. An unfilled selector names no row, so the check blocks at
// RULE_INPUTS_INCOMPLETE — never a silent pass.
//
// Required load cases, stress categories and load-case mappings are
// authored by `LoadCaseCategoriesEditor` below: the categories and their
// descriptions are the user's own text, with no built-in meaning.
//
// Unit I/O (TP-UNITS-B2-RULEPACKUNITS-001): declaration `quantity_intent`
// unit refs use the reviewed DEC-018 catalog when the desktop backend exposes
// it. Browser preview and backend-unavailable routes keep the prior text input
// rather than synthesizing unit options. The picker filters catalog entries by
// the selected dimension, keeps any stored out-of-catalog value visible as the
// current value, and never silently rewrites a dimension/unit pair.

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

type UnitOption = {
  symbol: string;
  label: string;
};

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
 * A schema-shaped default `solver_result_ref` row selector (format 0.5.0): the
 * two required members as the visible uppercase `"TBD"` placeholder. An
 * unfilled selector matches no result row, so the input stays unsupplied and
 * the check blocks (never a silent pass). No result *value* is ever stored
 * here, only the selector resolved at run time on each basis.
 */
export function defaultSolverResultRef(): Record<string, unknown> {
  return {
    result_kind: "TBD",
    entity_ref: "TBD"
  };
}

/** The selector members, in schema order (rule_pack.schema.yaml SolverResultRef). */
export const SOLVER_SELECTOR_TEXT_MEMBERS = ["result_kind", "entity_ref", "location", "component", "tie_rod"] as const;

/**
 * The distinct selectors of a solved result's rows (rows without basis_ref, a
 * malformed section basis or an unnamed rod have none), each with the number
 * of bases it names a row on. Order follows the result's rows.
 */
export function solverSelectorOffers(result: MechanicsResult | null | undefined): Array<{ key: string; selector: SolverResultRef; bases: number }> {
  if (!result) return [];
  const offers = new Map<string, { key: string; selector: SolverResultRef; bases: number }>();
  for (const row of result.results) {
    const selector = solverSelectorForRow(result, row);
    if (!selector) continue;
    const key = describeSolverSelector(selector);
    const entry = offers.get(key);
    if (entry) entry.bases += 1;
    else offers.set(key, { key, selector, bases: 1 });
  }
  return [...offers.values()];
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

/**
 * How a reference names each declaration: by its entered name, falling back
 * to its identifier where no name was entered, and with the identifier beside
 * a name two declarations share, so different declarations never look the same.
 */
export function distinctNames(ids: readonly string[], names?: Record<string, string>): Record<string, string> {
  const named = ids.map((id) => names?.[id] || id);
  const shown: Record<string, string> = {};
  ids.forEach((id, index) => {
    const name = named[index];
    const shared = named.filter((other) => other === name).length > 1;
    shown[id] = shared && name !== id ? `${name} (${id})` : name;
  });
  return shown;
}

// ---------------------------------------------------------------------------
// Field primitives
// ---------------------------------------------------------------------------

/** The title that keeps a field's format name findable. */
export function formatNameTitle(formatName: string | undefined): string | undefined {
  return formatName ? `Format name: ${formatName}` : undefined;
}

export function Field({
  label,
  formatName,
  children
}: {
  label: string;
  /** The format's own name of the field, kept as the label's title. */
  formatName?: string;
  children: ReactNode;
}) {
  return (
    <label className="rule-pack-node-field" title={formatNameTitle(formatName)}>
      <span>{label}</span>
      {children}
    </label>
  );
}

export function TextField({
  testId,
  label,
  formatName,
  value,
  disabled,
  onChange
}: {
  testId: string;
  label: string;
  formatName?: string;
  value: string;
  disabled: boolean;
  onChange: (next: string) => void;
}) {
  return (
    <Field label={label} formatName={formatName}>
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

function hasTauriInternals(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

function unitOptions(route: UnitCatalogRoute | null, dimensionId: string, currentSymbol: string): UnitOption[] {
  const current = currentSymbol.trim() || "TBD";
  const basis = describeUnitBasis(route, current, dimensionId);
  const fallback = { symbol: basis.symbol, label: basis.label };
  if (route?.route !== "tauri_unit_catalog") return [fallback];

  const options = acceptedUnits(route.catalog)
    .filter((entry) => unitEntryMatchesDimension(entry, dimensionId))
    .sort((left, right) => Number(right.canonical) - Number(left.canonical) || left.symbol.localeCompare(right.symbol))
    .map((entry) => {
      const entryBasis = describeUnitBasis(route, entry.symbol, dimensionId);
      return { symbol: entry.symbol, label: entryBasis.label };
    });

  if (!options.some((option) => option.symbol === fallback.symbol)) {
    options.unshift(fallback);
  }
  return options.length > 0 ? options : [fallback];
}

function UnitRefField({
  testId,
  label,
  formatName,
  value,
  dimension,
  disabled,
  unitCatalogRoute,
  onChange
}: {
  testId: string;
  label: string;
  formatName?: string;
  value: string;
  dimension: string;
  disabled: boolean;
  unitCatalogRoute: UnitCatalogRoute | null;
  onChange: (next: string) => void;
}) {
  // Until the desktop catalog is available, preserve the previous free-text
  // behavior so browser preview does not invent a fallback catalog.
  if (unitCatalogRoute?.route !== "tauri_unit_catalog") {
    return (
      <TextField testId={testId} label={label} formatName={formatName} value={value} disabled={disabled} onChange={onChange} />
    );
  }

  const options = unitOptions(unitCatalogRoute, dimension, value);
  return (
    <Field label={label} formatName={formatName}>
      <select
        data-testid={testId}
        value={value.trim() || "TBD"}
        disabled={disabled}
        onChange={(event) => onChange(event.target.value)}
      >
        {options.map((option) => (
          <option key={option.symbol} value={option.symbol}>
            {option.label}
          </option>
        ))}
      </select>
    </Field>
  );
}

export function EnumSelect({
  testId,
  label,
  formatName,
  options,
  words,
  value,
  disabled,
  onChange
}: {
  testId: string;
  label: string;
  formatName?: string;
  options: readonly string[];
  /** The visible word of each token; the token stays the option's value and title. */
  words?: Words;
  value: string;
  disabled: boolean;
  onChange: (next: string) => void;
}) {
  // An opened document may carry a token outside the current vocabulary (e.g.
  // from a future schema). Surface it as a "(current)" option rather than
  // silently snapping it to a default — the user sees exactly what is stored.
  const known = options.includes(value);
  return (
    <Field label={label} formatName={formatName}>
      <select
        data-testid={testId}
        value={value}
        disabled={disabled}
        onChange={(event) => onChange(event.target.value)}
      >
        {!known ? <option value={value}>(current) {value}</option> : null}
        {options.map((option) => (
          <option key={option} value={option} title={words ? option : undefined}>
            {words ? wordFor(words, option) : option}
          </option>
        ))}
      </select>
    </Field>
  );
}

/**
 * A closed disclosure inside a row or part ("More", "Details"). Native
 * <details>: every control inside stays in the document, reachable by
 * keyboard once opened.
 */
export function Disclosure({
  summary,
  testId,
  children
}: {
  summary: ReactNode;
  testId?: string;
  children: ReactNode;
}) {
  return (
    <details className="rule-pack-disclosure" data-testid={testId}>
      <summary>{summary}</summary>
      {children}
    </details>
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
  /** The session's current solved result, whose rows' selectors are offered. */
  result?: MechanicsResult | null;
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
  disabledReason,
  result = null
}: DeclarationsEditorProps) {
  const [unitCatalogRoute, setUnitCatalogRoute] = useState<UnitCatalogRoute | null>(null);
  const selectorOffers = useMemo(() => solverSelectorOffers(result), [result]);
  const rawInputs = asArray(document.required_inputs);
  const rawSlots = asArray(document.value_slots);
  const inputIds = readRequiredInputIds(document);
  const slotIds = readValueSlotIds(document);

  useEffect(() => {
    if (!hasTauriInternals()) {
      setUnitCatalogRoute({
        route: "unavailable_browser_preview",
        diagnostic:
          "UNIT-CATALOG-DESKTOP-ONLY: browser preview preserves stored rule-pack unit_ref text " +
          "and does not synthesize a DEC-018 fallback catalog."
      });
      return undefined;
    }
    let active = true;
    loadUnitCatalog()
      .then((route) => {
        if (active) setUnitCatalogRoute(route);
      })
      .catch((error) => {
        if (!active) return;
        setUnitCatalogRoute({
          route: "unavailable_browser_preview",
          diagnostic: `UNIT-CATALOG-LOAD-FAILED: ${String(error)}`
        });
      });
    return () => {
      active = false;
    };
  }, []);

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
  // Patch one member of an input's selector, merging into the existing selector
  // (or a fresh default when none exists yet). An emptied optional member is
  // removed, since an absent member matches only a row without it; an emptied
  // required member stays (validation names it).
  const updateInputSolverRef = (index: number, member: string, value: string) => {
    const ref = { ...(asObject(asObject(rawInputs[index])?.solver_result_ref) ?? defaultSolverResultRef()) };
    if (value === "" && member !== "result_kind" && member !== "entity_ref") delete ref[member];
    else ref[member] = value;
    updateInput(index, { solver_result_ref: ref });
  };
  // Replace the selector with one offered from the current solved result.
  const applySolverOffer = (index: number, key: string) => {
    const offer = selectorOffers.find((entry) => entry.key === key);
    if (offer) updateInput(index, { solver_result_ref: { ...offer.selector } });
  };
  // Drop the solver_result_ref member entirely. A solver_result input without a
  // selector is refused by validation (RULE_SOLVER_RESULT_REF_MISSING); the
  // control stays so a selector left on another source kind can be removed.
  // Every other member round-trips verbatim.
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
  const unitPolicySummary = declarationUnitPolicySummary(unitCatalogRoute, rawInputs, rawSlots);

  return (
    <div className="report-list rule-pack-declarations-editor" data-testid="rule-pack-declarations-editor">
      <div data-testid="rule-pack-required-inputs">
        <strong className="rule-pack-subheading">{PART_WORDS.inputsHeading} ({rawInputs.length})</strong>
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
                testId="rule-pack-input-name"
                label={fieldWord("name")}
                formatName="name"
                value={asString(record.name) ?? ""}
                disabled={disabled}
                onChange={(next) => updateInput(index, { name: next })}
              />
              <EnumSelect
                testId="rule-pack-input-source-kind"
                label={fieldWord("source_kind")}
                formatName="source_kind"
                options={SOURCE_KINDS}
                words={SOURCE_KIND_WORDS}
                value={asString(record.source_kind) ?? "TBD"}
                disabled={disabled}
                onChange={(next) => changeInputSourceKind(index, next)}
              />
              <Field label={fieldWord("dimension")} formatName="quantity_intent.dimension">
                <DimensionSelect
                  testId="rule-pack-input-dimension"
                  value={asString(quantity.dimension) ?? "TBD"}
                  disabled={disabled}
                  onChange={(next) => updateInputQuantity(index, { dimension: next })}
                />
              </Field>
              <UnitRefField
                testId="rule-pack-input-unit"
                label={fieldWord("unit_ref")}
                formatName="quantity_intent.unit_ref"
                value={asString(quantity.unit_ref) ?? "TBD"}
                dimension={asString(quantity.dimension) ?? "TBD"}
                disabled={disabled}
                unitCatalogRoute={unitCatalogRoute}
                onChange={(next) => updateInputQuantity(index, { unit_ref: next })}
              />
              {showLibraryRef ? (
                <div className="rule-pack-node" data-testid="rule-pack-input-library-ref">
                  <small
                    className="rule-pack-node-note"
                    data-testid="rule-pack-input-library-ref-note"
                    title="The referenced value is resolved at check-run time from the local private library store; it stays in the library and is never embedded in the rule pack."
                  >
                    The value stays in your private library; the pack keeps only where to find it.
                  </small>
                  <EnumSelect
                    testId="rule-pack-input-library-kind"
                    label={fieldWord("library_kind")}
                    formatName="library_value_ref.library_kind"
                    options={LIBRARY_KINDS}
                    words={LIBRARY_KIND_WORDS}
                    value={asString(libraryRef?.library_kind) ?? LIBRARY_KINDS[0]}
                    disabled={disabled}
                    onChange={(next) => updateInputLibraryRef(index, { library_kind: next })}
                  />
                  <TextField
                    testId="rule-pack-input-library-id"
                    label={fieldWord("library_id")}
                    formatName="library_value_ref.library_id"
                    value={asString(libraryRef?.library_id) ?? ""}
                    disabled={disabled}
                    onChange={(next) => updateInputLibraryRef(index, { library_id: next })}
                  />
                  <TextField
                    testId="rule-pack-input-library-record"
                    label={fieldWord("record_id")}
                    formatName="library_value_ref.record_id"
                    value={asString(libraryRef?.record_id) ?? ""}
                    disabled={disabled}
                    onChange={(next) => updateInputLibraryRef(index, { record_id: next })}
                  />
                  <TextField
                    testId="rule-pack-input-library-slot"
                    label={fieldWord("library slot_id")}
                    formatName="library_value_ref.slot_id"
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
                    className="rule-pack-node-note"
                    data-testid="rule-pack-input-solver-ref-note"
                    title="On each required load case of a check, this input binds the one solved row of this kind and entity whose location, component, section basis and tie rod equal the members below; an empty optional member matches only a row without it. No row, or more than one, leaves the check incomplete there."
                  >
                    Picks one solved row on each load case; none or several leaves the check incomplete there.
                  </small>
                  {SOLVER_SELECTOR_TEXT_MEMBERS.map((member) => (
                    <TextField
                      key={member}
                      testId={`rule-pack-input-solver-${member.replace("_", "-")}`}
                      label={fieldWord(member)}
                      formatName={`solver_result_ref.${member}`}
                      value={asString(solverRef?.[member]) ?? ""}
                      disabled={disabled}
                      onChange={(next) => updateInputSolverRef(index, member, next)}
                    />
                  ))}
                  <Field label={fieldWord("section_basis")} formatName="solver_result_ref.section_basis">
                    <select
                      data-testid="rule-pack-input-solver-section-basis"
                      value={asString(solverRef?.section_basis) ?? ""}
                      disabled={disabled}
                      onChange={(event) => updateInputSolverRef(index, "section_basis", event.target.value)}
                    >
                      <option value="">(none: a row without a section basis)</option>
                      {(() => {
                        const current = asString(solverRef?.section_basis) ?? "";
                        return current && !(SOLVER_SELECTOR_SECTION_BASES as readonly string[]).includes(current)
                          ? <option value={current}>(current) {current}</option>
                          : null;
                      })()}
                      {SOLVER_SELECTOR_SECTION_BASES.map((basis) => (
                        <option key={basis} value={basis}>
                          {basis}
                        </option>
                      ))}
                    </select>
                  </Field>
                  {selectorOffers.length > 0 ? (
                    <Field label={fieldWord("solver_offer")}>
                      <select
                        data-testid="rule-pack-input-solver-offer"
                        value=""
                        disabled={disabled}
                        onChange={(event) => applySolverOffer(index, event.target.value)}
                      >
                        <option value="">-- use a selector of the current result --</option>
                        {selectorOffers.map((offer) => (
                          <option key={offer.key} value={offer.key}>
                            {offer.key} ({offer.bases} load case(s) or combination(s))
                          </option>
                        ))}
                      </select>
                    </Field>
                  ) : (
                    <small className="rule-pack-node-note" data-testid="rule-pack-input-solver-offer-none">
                      No solved result in this session; type the selector members.
                    </small>
                  )}
                  <button
                    type="button"
                    data-testid="rule-pack-input-solver-remove"
                    disabled={disabled || solverRef === null}
                    title={
                      disabled
                        ? disabledReason
                        : solverRef === null
                          ? "No solver-row selector to remove."
                          : undefined
                    }
                    onClick={() => removeInputSolverRef(index)}
                  >
                    Remove solver-row selector
                  </button>
                </div>
              ) : null}
              <Disclosure summary="More" testId="rule-pack-input-more">
                <TextField
                  testId="rule-pack-input-id"
                  label={fieldWord("input_id")}
                  formatName="input_id"
                  value={asString(record.input_id) ?? ""}
                  disabled={disabled}
                  onChange={(next) => updateInput(index, { input_id: next })}
                />
                <EnumSelect
                  testId="rule-pack-input-required-for"
                  label={fieldWord("required_for")}
                  formatName="required_for"
                  options={REQUIRED_FOR_TARGETS}
                  words={REQUIRED_FOR_WORDS}
                  value={asString(record.required_for) ?? "TBD"}
                  disabled={disabled}
                  onChange={(next) => updateInput(index, { required_for: next })}
                />
                <EnumSelect
                  testId="rule-pack-input-completeness"
                  label={fieldWord("completeness_status")}
                  formatName="completeness_status"
                  options={COMPLETENESS_STATUSES}
                  words={COMPLETENESS_WORDS}
                  value={asString(record.completeness_status) ?? "TBD"}
                  disabled={disabled}
                  onChange={(next) => updateInput(index, { completeness_status: next })}
                />
              </Disclosure>
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
          Add input
        </button>
      </div>

      <div data-testid="rule-pack-value-slots">
        <strong className="rule-pack-subheading">{PART_WORDS.limitsHeading} ({rawSlots.length})</strong>
        {rawSlots.map((entry, index) => {
          const record = asObject(entry) ?? {};
          const quantity = asObject(record.quantity_intent) ?? {};
          return (
            <div
              className="rule-pack-node operation-record"
              data-testid="rule-pack-value-slot-row"
              key={`slot-${index}`}
            >
              {/* A limit has no name member: its identifier is the name it has. */}
              <TextField
                testId="rule-pack-slot-id"
                label={fieldWord("slot_id")}
                formatName="slot_id"
                value={asString(record.slot_id) ?? ""}
                disabled={disabled}
                onChange={(next) => updateSlot(index, { slot_id: next })}
              />
              <EnumSelect
                testId="rule-pack-slot-kind"
                label={fieldWord("slot_kind")}
                formatName="slot_kind"
                options={SLOT_KINDS}
                words={SLOT_KIND_WORDS}
                value={asString(record.slot_kind) ?? "TBD"}
                disabled={disabled}
                onChange={(next) => updateSlot(index, { slot_kind: next })}
              />
              <Field label={fieldWord("dimension")} formatName="quantity_intent.dimension">
                <DimensionSelect
                  testId="rule-pack-slot-dimension"
                  value={asString(quantity.dimension) ?? "TBD"}
                  disabled={disabled}
                  onChange={(next) => updateSlotQuantity(index, { dimension: next })}
                />
              </Field>
              <UnitRefField
                testId="rule-pack-slot-unit"
                label={fieldWord("unit_ref")}
                formatName="quantity_intent.unit_ref"
                value={asString(quantity.unit_ref) ?? "TBD"}
                dimension={asString(quantity.dimension) ?? "TBD"}
                disabled={disabled}
                unitCatalogRoute={unitCatalogRoute}
                onChange={(next) => updateSlotQuantity(index, { unit_ref: next })}
              />
              <Disclosure summary="More" testId="rule-pack-slot-more">
                <EnumSelect
                  testId="rule-pack-slot-value-status"
                  label={fieldWord("value_status")}
                  formatName="value_status"
                  options={VALUE_STATUSES}
                  words={VALUE_STATUS_WORDS}
                  value={asString(record.value_status) ?? "TBD"}
                  disabled={disabled}
                  onChange={(next) => updateSlot(index, { value_status: next })}
                />
                <EnumSelect
                  testId="rule-pack-slot-required-for"
                  label={fieldWord("required_for")}
                  formatName="required_for"
                  options={REQUIRED_FOR_TARGETS}
                  words={REQUIRED_FOR_WORDS}
                  value={asString(record.required_for) ?? "TBD"}
                  disabled={disabled}
                  onChange={(next) => updateSlot(index, { required_for: next })}
                />
                <EnumSelect
                  testId="rule-pack-slot-completeness"
                  label={fieldWord("completeness_status")}
                  formatName="completeness_status"
                  options={COMPLETENESS_STATUSES}
                  words={COMPLETENESS_WORDS}
                  value={asString(record.completeness_status) ?? "TBD"}
                  disabled={disabled}
                  onChange={(next) => updateSlot(index, { completeness_status: next })}
                />
              </Disclosure>
              <button
                type="button"
                data-testid="rule-pack-slot-remove"
                disabled={disabled || rawSlots.length <= 1}
                title={disabled ? disabledReason : slotRemoveReason}
                onClick={() => removeSlot(index)}
              >
                Remove limit
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
          Add limit
        </button>
      </div>

      <Disclosure summary="Details" testId="rule-pack-declarations-details">
        <small className="rule-pack-node-readonly" data-testid="rule-pack-declarations-unit-policy">
          {unitPolicySummary}
        </small>
      </Disclosure>
    </div>
  );
}

function declarationUnitPolicySummary(
  route: UnitCatalogRoute | null,
  rawInputs: unknown[],
  rawSlots: unknown[]
): string {
  const inputStatuses = rawInputs.map((entry, index) => {
    const record = asObject(entry) ?? {};
    const quantity = asObject(record.quantity_intent) ?? {};
    const id = asString(record.input_id) ?? `input_${index + 1}`;
    return `required_input:${id}=${unitIntentValidationStatus(route, quantity)}`;
  });
  const slotStatuses = rawSlots.map((entry, index) => {
    const record = asObject(entry) ?? {};
    const quantity = asObject(record.quantity_intent) ?? {};
    const id = asString(record.slot_id) ?? `slot_${index + 1}`;
    return `value_slot:${id}=${unitIntentValidationStatus(route, quantity)}`;
  });
  return [
    "unit_policy=stored_unit_refs_preserved",
    `catalog_route=${unitCatalogRouteStatus(route)}`,
    "conversion=false",
    `required_inputs=${inputStatuses.length ? inputStatuses.join(",") : "none"}`,
    `value_slots=${slotStatuses.length ? slotStatuses.join(",") : "none"}`
  ].join("; ");
}

function unitIntentValidationStatus(route: UnitCatalogRoute | null, quantity: Record<string, unknown>): string {
  const dimension = asString(quantity.dimension) ?? "TBD";
  const unit = asString(quantity.unit_ref) ?? "TBD";
  return `${unitDimensionValidationStatus(route, unit, dimension)}(unit=${unit};dimension=${dimension})`;
}

function unitCatalogRouteStatus(route: UnitCatalogRoute | null): string {
  if (!route) return "loading";
  if (route.route === "unavailable_browser_preview") return "browser_preview_manual_entry";
  return "tauri_dec018_catalog";
}

// ---------------------------------------------------------------------------
// Required load cases, stress categories and load-case mappings
// (rule-pack format 0.5.0)
// ---------------------------------------------------------------------------
//
// A check names one stress category and is evaluated once for each required
// load case that category's one mapping lists. Category names and descriptions
// are the user's own text: the format gives a category no built-in meaning and
// no fixed list, and nothing here names a category of any standard. A required
// load case names the model's own load case or combination id; with a model
// open it is chosen from the model's load cases and combinations, otherwise
// typed. Whether it exists in a model is checked at run time, never here.

/** The declared `required_load_case_id`s in document order. */
export function readRequiredLoadCaseIds(document: DeclarationDocument): string[] {
  return asArray(document.required_load_cases)
    .map((entry) => asString(asObject(entry)?.required_load_case_id))
    .filter((id): id is string => id !== null);
}

/** The declared stress `category_id`s in document order. */
export function readStressCategoryIds(document: DeclarationDocument): string[] {
  return asArray(document.stress_categories)
    .map((entry) => asString(asObject(entry)?.category_id))
    .filter((id): id is string => id !== null);
}

/** A new required load case: a fresh id, empty text, the given basis. */
export function defaultRequiredLoadCase(
  existingIds: string[],
  basis: { ref_type: string; ref_id: string }
): Record<string, unknown> {
  return {
    required_load_case_id: uniqueDeclarationId("user_required_load_case", existingIds),
    name: "",
    description: "",
    basis_ref: { ref_type: basis.ref_type, ref_id: basis.ref_id },
    provenance: draftPlaceholderProvenance()
  };
}

/** A new stress category: a fresh id and empty text for the user to fill. */
export function defaultStressCategory(existingIds: string[]): Record<string, unknown> {
  return {
    category_id: uniqueDeclarationId("user_stress_category", existingIds),
    name: "",
    description: "",
    provenance: draftPlaceholderProvenance()
  };
}

type BasisOption = { value: string; ref_type: "load_case" | "combination"; ref_id: string; label: string };

const basisOptionValue = (refType: string, refId: string) => `${refType}|${refId}`;

/** The current model's load cases and combinations, by id and label. */
export function modelBasisOptions(model: PreviewModel | null | undefined): BasisOption[] {
  if (!model) return [];
  return [
    ...(model.load_cases ?? []).map((entry) => ({
      value: basisOptionValue("load_case", entry.id),
      ref_type: "load_case" as const,
      ref_id: entry.id,
      label: `Load case ${entry.id}${entry.label ? ` — ${entry.label}` : ""}`
    })),
    ...(model.combinations ?? []).map((entry) => ({
      value: basisOptionValue("combination", entry.id),
      ref_type: "combination" as const,
      ref_id: entry.id,
      label: `Combination ${entry.id}${entry.label ? ` — ${entry.label}` : ""}`
    }))
  ];
}

function referenceId(value: unknown, refType: string): string | null {
  const reference = asObject(value);
  return reference && reference.ref_type === refType ? asString(reference.ref_id) : null;
}

export type LoadCaseCategoriesEditorProps = {
  document: DeclarationDocument;
  onChange: (next: DeclarationDocument) => void;
  disabled: boolean;
  disabledReason?: string;
  model?: PreviewModel | null;
  /** Which half to show: the page shows each in its own part; both by default. */
  show?: "both" | "load-cases" | "categories";
};

export function LoadCaseCategoriesEditor({
  document,
  onChange,
  disabled,
  disabledReason,
  model = null,
  show = "both"
}: LoadCaseCategoriesEditorProps) {
  const rawLoadCases = asArray(document.required_load_cases);
  const rawCategories = asArray(document.stress_categories);
  const rawMappings = asArray(document.load_case_mappings);
  const loadCaseIds = readRequiredLoadCaseIds(document);
  const categoryIds = readStressCategoryIds(document);
  const basisOptions = modelBasisOptions(model);
  // Each load case by its name, with its identifier beside a name two share.
  const loadCaseNames = distinctNames(
    loadCaseIds,
    Object.fromEntries(
      rawLoadCases
        .map((entry) => asObject(entry))
        .filter((entry): entry is Record<string, unknown> => entry !== null)
        .map((entry) => [asString(entry.required_load_case_id) ?? "", asString(entry.name) ?? ""])
    )
  );

  const patchItem = (items: unknown[], index: number, patch: Record<string, unknown>) =>
    items.map((entry, position) => (position === index ? { ...(asObject(entry) ?? {}), ...patch } : entry));

  // --- required load cases ---
  const updateLoadCase = (index: number, patch: Record<string, unknown>) =>
    onChange({ ...document, required_load_cases: patchItem(rawLoadCases, index, patch) });
  // Renaming an id carries the mappings' references with it.
  const renameLoadCase = (index: number, nextId: string) => {
    const previous = asString(asObject(rawLoadCases[index])?.required_load_case_id);
    onChange({
      ...document,
      required_load_cases: patchItem(rawLoadCases, index, { required_load_case_id: nextId }),
      load_case_mappings: rawMappings.map((mapping) => {
        const record = asObject(mapping);
        if (!record || previous === null) return mapping;
        return {
          ...record,
          required_load_case_refs: asArray(record.required_load_case_refs).map((reference) =>
            referenceId(reference, "required_load_case") === previous
              ? { ...(asObject(reference) ?? {}), ref_id: nextId }
              : reference
          )
        };
      })
    });
  };
  const setLoadCaseBasis = (index: number, refType: string, refId: string) =>
    updateLoadCase(index, { basis_ref: { ref_type: refType, ref_id: refId } });
  const addLoadCase = () => {
    const used = new Set(
      rawLoadCases.map((entry) => {
        const basis = asObject(asObject(entry)?.basis_ref);
        return basisOptionValue(asString(basis?.ref_type) ?? "", asString(basis?.ref_id) ?? "");
      })
    );
    const unused = basisOptions.find((option) => !used.has(option.value));
    onChange({
      ...document,
      required_load_cases: [
        ...rawLoadCases,
        defaultRequiredLoadCase(loadCaseIds, unused ?? { ref_type: "load_case", ref_id: "" })
      ]
    });
  };
  // Removing a required load case also removes the mappings' references to it.
  const removeLoadCase = (index: number) => {
    const removed = asString(asObject(rawLoadCases[index])?.required_load_case_id);
    onChange({
      ...document,
      required_load_cases: rawLoadCases.filter((_, position) => position !== index),
      load_case_mappings: rawMappings.map((mapping) => {
        const record = asObject(mapping);
        if (!record) return mapping;
        return {
          ...record,
          required_load_case_refs: asArray(record.required_load_case_refs).filter(
            (reference) => referenceId(reference, "required_load_case") !== removed
          )
        };
      })
    });
  };

  // --- stress categories and their mapping ---
  const updateCategory = (index: number, patch: Record<string, unknown>) =>
    onChange({ ...document, stress_categories: patchItem(rawCategories, index, patch) });
  // Renaming a category id carries its mapping and its checks with it.
  const renameCategory = (index: number, nextId: string) => {
    const previous = asString(asObject(rawCategories[index])?.category_id);
    const retarget = (entry: unknown) => {
      const record = asObject(entry);
      if (!record || previous === null || referenceId(record.category_ref, "stress_category") !== previous) return entry;
      return { ...record, category_ref: { ...(asObject(record.category_ref) ?? {}), ref_id: nextId } };
    };
    onChange({
      ...document,
      stress_categories: patchItem(rawCategories, index, { category_id: nextId }),
      load_case_mappings: rawMappings.map(retarget),
      check_definitions: asArray(document.check_definitions).map(retarget)
    });
  };
  const addCategory = () => {
    const category = defaultStressCategory(categoryIds);
    const mappingIds = rawMappings
      .map((entry) => asString(asObject(entry)?.mapping_id))
      .filter((id): id is string => id !== null);
    onChange({
      ...document,
      stress_categories: [...rawCategories, category],
      // Every category has exactly one mapping; it starts without load cases
      // (validation names that until the user ticks one).
      load_case_mappings: [
        ...rawMappings,
        {
          mapping_id: uniqueDeclarationId("user_load_case_mapping", mappingIds),
          category_ref: { ref_id: category.category_id, ref_type: "stress_category" },
          required_load_case_refs: []
        }
      ]
    });
  };
  const removeCategory = (index: number) => {
    const removed = asString(asObject(rawCategories[index])?.category_id);
    onChange({
      ...document,
      stress_categories: rawCategories.filter((_, position) => position !== index),
      load_case_mappings: rawMappings.filter(
        (mapping) => referenceId(asObject(mapping)?.category_ref, "stress_category") !== removed
      )
    });
  };
  const mappedLoadCases = (categoryId: string): string[] => {
    const mapping = rawMappings.map(asObject).find((entry) => referenceId(entry?.category_ref, "stress_category") === categoryId);
    return asArray(mapping?.required_load_case_refs)
      .map((reference) => referenceId(reference, "required_load_case"))
      .filter((id): id is string => id !== null);
  };
  // Tick or untick a required load case in the category's mapping (the first
  // mapping naming the category; one is created when there is none). A ticked
  // load case is appended, so the mapping's order is the order of ticking.
  const toggleMapping = (categoryId: string, loadCaseId: string, on: boolean) => {
    const index = rawMappings.findIndex((entry) => referenceId(asObject(entry)?.category_ref, "stress_category") === categoryId);
    if (index < 0) {
      if (!on) return;
      const mappingIds = rawMappings
        .map((entry) => asString(asObject(entry)?.mapping_id))
        .filter((id): id is string => id !== null);
      onChange({
        ...document,
        load_case_mappings: [
          ...rawMappings,
          {
            mapping_id: uniqueDeclarationId("user_load_case_mapping", mappingIds),
            category_ref: { ref_id: categoryId, ref_type: "stress_category" },
            required_load_case_refs: [{ ref_id: loadCaseId, ref_type: "required_load_case" }]
          }
        ]
      });
      return;
    }
    const record = asObject(rawMappings[index]) ?? {};
    const refs = asArray(record.required_load_case_refs);
    const present = refs.some((reference) => referenceId(reference, "required_load_case") === loadCaseId);
    const nextRefs = on
      ? present ? refs : [...refs, { ref_id: loadCaseId, ref_type: "required_load_case" }]
      : refs.filter((reference) => referenceId(reference, "required_load_case") !== loadCaseId);
    onChange({ ...document, load_case_mappings: patchItem(rawMappings, index, { required_load_case_refs: nextRefs }) });
  };

  const showLoadCases = show !== "categories";
  const showCategories = show !== "load-cases";
  return (
    <div
      className="report-list rule-pack-load-case-categories-editor"
      data-testid={show === "categories" ? "rule-pack-stress-categories-editor" : "rule-pack-load-case-categories-editor"}
    >
      {showLoadCases ? (
      <small className="rule-pack-node-note" data-testid="rule-pack-load-case-categories-guidance">
        A check runs once on each load case its stress category lists.
      </small>
      ) : null}

      {showLoadCases ? (
      <div data-testid="rule-pack-required-load-cases">
        <strong className="rule-pack-subheading">{PART_WORDS.loadCases} ({rawLoadCases.length})</strong>
        {rawLoadCases.map((entry, index) => {
          const record = asObject(entry) ?? {};
          const basis = asObject(record.basis_ref) ?? {};
          const refType = asString(basis.ref_type) ?? "";
          const refId = asString(basis.ref_id) ?? "";
          const current = basisOptionValue(refType, refId);
          const known = basisOptions.some((option) => option.value === current);
          return (
            <div
              className="rule-pack-node operation-record"
              data-testid="rule-pack-required-load-case-row"
              key={`load-case-${index}`}
            >
              <TextField
                testId="rule-pack-load-case-name"
                label={fieldWord("name")}
                formatName="name"
                value={asString(record.name) ?? ""}
                disabled={disabled}
                onChange={(next) => updateLoadCase(index, { name: next })}
              />
              {basisOptions.length > 0 ? (
                <Field label={fieldWord("basis_ref")} formatName="basis_ref">
                  <select
                    data-testid="rule-pack-load-case-basis"
                    value={current}
                    disabled={disabled}
                    onChange={(event) => {
                      const option = basisOptions.find((candidate) => candidate.value === event.target.value);
                      if (option) setLoadCaseBasis(index, option.ref_type, option.ref_id);
                    }}
                  >
                    {!known ? (
                      <option value={current}>
                        (not in the current model) {refType || "?"} {refId || "(empty)"}
                      </option>
                    ) : null}
                    {basisOptions.map((option) => (
                      <option key={option.value} value={option.value}>
                        {option.label}
                      </option>
                    ))}
                  </select>
                </Field>
              ) : (
                <>
                  <EnumSelect
                    testId="rule-pack-load-case-basis-type"
                    label={fieldWord("basis ref_type")}
                    formatName="basis_ref.ref_type"
                    options={["load_case", "combination"]}
                    words={BASIS_TYPE_WORDS}
                    value={refType}
                    disabled={disabled}
                    onChange={(next) => setLoadCaseBasis(index, next, refId)}
                  />
                  <TextField
                    testId="rule-pack-load-case-basis-id"
                    label={fieldWord("basis ref_id")}
                    formatName="basis_ref.ref_id"
                    value={refId}
                    disabled={disabled}
                    onChange={(next) => setLoadCaseBasis(index, refType || "load_case", next)}
                  />
                </>
              )}
              <TextField
                testId="rule-pack-load-case-description"
                label={fieldWord("description")}
                formatName="description"
                value={asString(record.description) ?? ""}
                disabled={disabled}
                onChange={(next) => updateLoadCase(index, { description: next })}
              />
              <Disclosure summary="More" testId="rule-pack-load-case-more">
                <TextField
                  testId="rule-pack-load-case-id"
                  label={fieldWord("required_load_case_id")}
                  formatName="required_load_case_id"
                  value={asString(record.required_load_case_id) ?? ""}
                  disabled={disabled}
                  onChange={(next) => renameLoadCase(index, next)}
                />
              </Disclosure>
              <button
                type="button"
                data-testid="rule-pack-load-case-remove"
                disabled={disabled || rawLoadCases.length <= 1}
                title={
                  disabled
                    ? disabledReason
                    : rawLoadCases.length <= 1
                      ? "A rule pack needs at least one required load case."
                      : undefined
                }
                onClick={() => removeLoadCase(index)}
              >
                Remove load case
              </button>
            </div>
          );
        })}
        <button
          type="button"
          data-testid="rule-pack-load-case-add"
          disabled={disabled}
          title={disabled ? disabledReason : undefined}
          onClick={addLoadCase}
        >
          Add load case
        </button>
      </div>
      ) : null}

      {showCategories ? (
      <div data-testid="rule-pack-stress-categories">
        <strong className="rule-pack-subheading">{PART_WORDS.categories} ({rawCategories.length})</strong>
        {rawCategories.map((entry, index) => {
          const record = asObject(entry) ?? {};
          const categoryId = asString(record.category_id) ?? "";
          const mapped = mappedLoadCases(categoryId);
          return (
            <div
              className="rule-pack-node operation-record"
              data-testid="rule-pack-stress-category-row"
              key={`category-${index}`}
            >
              <TextField
                testId="rule-pack-category-name"
                label={fieldWord("name")}
                formatName="name"
                value={asString(record.name) ?? ""}
                disabled={disabled}
                onChange={(next) => updateCategory(index, { name: next })}
              />
              <TextField
                testId="rule-pack-category-description"
                label={fieldWord("description")}
                formatName="description"
                value={asString(record.description) ?? ""}
                disabled={disabled}
                onChange={(next) => updateCategory(index, { description: next })}
              />
              <div
                className="rule-pack-check-subgroup"
                data-testid="rule-pack-category-mapping"
                title="Format name: load_case_mappings[].required_load_case_refs"
              >
                <span className="rule-pack-node-label">{fieldWord("required_load_case_refs")}</span>
                {loadCaseIds.map((loadCaseId) => {
                  const name = loadCaseNames[loadCaseId];
                  return (
                    <label className="rule-pack-node-field" key={loadCaseId}>
                      <input
                        type="checkbox"
                        data-testid={`rule-pack-category-mapping-${loadCaseId}`}
                        checked={mapped.includes(loadCaseId)}
                        disabled={disabled || !categoryId}
                        title={disabled ? disabledReason : !categoryId ? "Give the category an identifier first." : undefined}
                        onChange={(event) => toggleMapping(categoryId, loadCaseId, event.target.checked)}
                      />
                      <span title={loadCaseId}>{name}</span>
                    </label>
                  );
                })}
              </div>
              <Disclosure summary="More" testId="rule-pack-category-more">
                <TextField
                  testId="rule-pack-category-id"
                  label={fieldWord("category_id")}
                  formatName="category_id"
                  value={categoryId}
                  disabled={disabled}
                  onChange={(next) => renameCategory(index, next)}
                />
              </Disclosure>
              <button
                type="button"
                data-testid="rule-pack-category-remove"
                disabled={disabled || rawCategories.length <= 1}
                title={
                  disabled
                    ? disabledReason
                    : rawCategories.length <= 1
                      ? "A rule pack needs at least one stress category."
                      : undefined
                }
                onClick={() => removeCategory(index)}
              >
                Remove category
              </button>
            </div>
          );
        })}
        <button
          type="button"
          data-testid="rule-pack-category-add"
          disabled={disabled}
          title={disabled ? disabledReason : undefined}
          onClick={addCategory}
        >
          Add stress category
        </button>
      </div>
      ) : null}
    </div>
  );
}
