/** Sorted stress and ratio table (survey item A3; DEL-07-05 Design `stress-ratio-table.md`).
 *
 * Per stress category, the result locations sorted largest first: the user's rule-check outcomes
 * of each of the pack's stress categories by their ratio (computed value / limit) or by the
 * computed value's magnitude, and each emitted stress kind by the magnitude of its rows. A
 * threshold shows only the rows at or above it.
 *
 * Pure post-processing of emitted rows and of the session's rule-check run: nothing here forms an
 * engineering quantity, a limit or a status. A ratio is formed only from the outcome's own computed
 * value and limit, stated in one unit, under an upper-bound relation (§3.1). The status is the
 * runner's own. FR-RULE-003: an incomplete outcome is pinned with its reason, never sorted and never
 * hidden by the threshold; a withheld or failed outcome is never hidden either. A row the numerical
 * standing withholds (the viewport overlays' rule, `overlayMayShowRow`) carries no value in the
 * view. Stored values and units are kept unchanged for an export (D2). */

import type { MechanicsResult, PreviewModel, ResultBasisRef } from "../../types";
import type { SessionRuleCheckRun } from "../report/renderableReportInput";
import { PASCAL } from "../report/reportQuantityDisplay";
import { RANGE_ENVELOPE_BASIS, basisKey } from "../viewport/resultColourOverlay";
import {
  SOLVER_SELECTOR_SECTION_BASES,
  deriveRuleCheckBindingPlan,
  ruleCheckOutcomeIdentity,
  type RuleCheckOutcome,
  type RuleCheckStatus,
  type SolverResultRef
} from "../../services/ruleCheckService";
import { overlayMayShowRow, type ResultOverlayStanding } from "./numericalHealth";
import { caseWithheldReason, supportLoadCases, type SupportLoadCase } from "./supportLoadSummary";

type ResultRow = MechanicsResult["results"][number];

export type StressKind = Readonly<{ kind: string; label: string }>;

/** The emitted stress kinds, in display order: every kind a pinned semantic contract gives the
 * `stress` family (`fixtures/results/semantic_contract_v0_3_*.json`; a test keeps the two equal). */
export const STRESS_KINDS: readonly StressKind[] = Object.freeze([
  { kind: "pipe_elastic_normal_stress_maximum_v2", label: "Normal-stress maximum" },
  { kind: "element_local_normal_stress_extreme", label: "Section normal-stress extreme σ±" },
  { kind: "component_equal_factor_intensified_bending_stress_v1", label: "Intensified bending stress (component)" },
  { kind: "curved_bend_directional_intensified_bending_maximum_v1", label: "Curved-bend intensified bending maximum" },
  { kind: "curved_bend_directional_intensified_bending_stress_v1", label: "Curved-bend intensified bending stress" },
  { kind: "element_local_axial_normal_stress", label: "Axial normal stress" },
  { kind: "element_local_bending_normal_stress_y", label: "Bending normal stress, local y" },
  { kind: "element_local_bending_normal_stress_z", label: "Bending normal stress, local z" },
  { kind: "pipe_axial_membrane_stress_v2", label: "Axial membrane stress" },
  { kind: "element_local_torsional_shear_stress", label: "Torsional shear stress" },
  { kind: "element_local_transverse_shear_stress_maximum", label: "Transverse shear stress maximum" },
  { kind: "pipe_section_pressure_hoop_stress", label: "Pressure hoop stress" },
  { kind: "pipe_section_pressure_longitudinal_stress", label: "Pressure longitudinal stress" },
  { kind: "pipe_lame_hoop_stress_v2", label: "Lamé hoop stress" },
  { kind: "pipe_lame_radial_stress_v2", label: "Lamé radial stress" },
  { kind: "pipe_toroidal_membrane_hoop_stress_v1", label: "Toroidal membrane hoop stress" },
  { kind: "pipe_toroidal_radial_surface_stress_v1", label: "Toroidal radial surface stress" }
].map((item) => Object.freeze(item)));

export type StressRatioMode = "ratio" | "stress";

/** A stored value and its stored unit, unchanged. */
export type StressRatioValue = Readonly<{ value: number; unit: string }>;

/** Where a row is: an emitted row's entity, station, component and section basis, or a rule
 * input's selector when the run bound no row of this result. */
export type StressRatioLocation = Readonly<{
  entityRef: string;
  station: string | null;
  component: string | null;
  sectionBasis: string | null;
  /** The emitted row it is read from, or null when it is the pack's selector. */
  resultId: string | null;
  /** The rule input it locates, for an outcome. */
  inputId: string | null;
}>;

/** What highlighting a row selects: an emitted row (the results table's own path) or an entity. */
export type StressRatioTarget = Readonly<{ resultId: string | null; entityRef: string | null }>;

export type StressRatioOutcome = Readonly<{
  /** `ruleCheckOutcomeIdentity`: (check_id, basis ref type, basis ref id). */
  identity: string;
  checkId: string;
  status: RuleCheckStatus;
  relation: string;
  /** Null for a withheld row, and for an incomplete row whose case is withheld. */
  computed: StressRatioValue | null;
  limit: StressRatioValue | null;
  /** computed / limit (§3.1), or null with `ratioNote` saying why none is formed. */
  ratio: number | null;
  ratioNote: string | null;
  /** For an incomplete outcome: the missing inputs and the run's findings. */
  incompleteReason: string | null;
}>;

/** Whether the numerical standing admits a row's values (§4). */
export type StressRatioStanding = "admitted" | "withheld";

export type StressRatioRow = Readonly<{
  /** The result-row id (stress row) or the outcome identity. */
  key: string;
  /** Source order: result-row order, or run order. */
  order: number;
  case: SupportLoadCase | null;
  caseLabel: string;
  locations: readonly StressRatioLocation[];
  target: StressRatioTarget;
  standing: StressRatioStanding;
  withheldReason: string | null;
  /** An emitted stress row's value (admitted rows only). */
  stress: Readonly<StressRatioValue & { resultId: string }> | null;
  outcome: StressRatioOutcome | null;
}>;

/** A case whose stress rows the numerical standing withholds, stated once. */
export type StressRatioWithheldCase = Readonly<{ case: SupportLoadCase; rowCount: number; reason: string }>;

export type StressRatioGroup = Readonly<{
  id: string;
  source: "rule_category" | "emitted_stress";
  label: string;
  /** The emitted kind, or the pack's stress category id. */
  ref: string;
  modes: readonly StressRatioMode[];
  /** Admitted stress rows, or every outcome (admitted, withheld and incomplete) of the category. */
  rows: readonly StressRatioRow[];
  /** Stress groups only: the withheld cases, in case order. */
  withheldCases: readonly StressRatioWithheldCase[];
}>;

export type StressRatioRunState = "none" | "used" | "not_this_result" | "no_outcomes";

export type StressRatioTable = Readonly<{
  state: "available" | "unavailable";
  reason: string | null;
  run: Readonly<{ state: StressRatioRunState; rulePackId: string | null; outcomeCount: number; note: string }>;
  groups: readonly StressRatioGroup[];
}>;

export const STRESS_RATIO_NO_RESULT = "No result";
export const STRESS_RATIO_NOT_SOLVED = "Mechanics not solved";
export const STRESS_RATIO_NOTHING = "No emitted stress rows and no rule-check outcomes";
export const STRESS_RATIO_NO_RUN = "No rule-check run in this session; emitted stresses are sorted by magnitude.";
export const STRESS_RATIO_RUN_NOT_THIS_RESULT = "The rule-check run does not belong to this result; its outcomes are not shown.";
export const STRESS_RATIO_RUN_NO_OUTCOMES = "The rule-check run has no outcomes; emitted stresses are sorted by magnitude.";
export function stressRatioRunText(rulePackId: string, outcomeCount: number): string {
  return `Rule-check run of pack ${rulePackId}: ${outcomeCount} outcome${outcomeCount === 1 ? "" : "s"}, one per check and load case or combination.`;
}

export const RATIO_NOTE_LOWER_BOUND = "Lower-bound relation; no ratio is formed";
export const RATIO_NOTE_PREDICATE = "Predicate check; no ratio is formed";
export const RATIO_NOTE_MISSING = "No computed value or limit; no ratio is formed";
export const RATIO_NOTE_LIMIT_NOT_POSITIVE = "Limit is not positive; no ratio is formed";
export const RATIO_NOTE_NOT_FINITE = "Ratio is not finite; none is formed";
export function ratioNoteUnitsDiffer(computed: string, limit: string): string {
  return `Computed value and limit in different units (${computed}, ${limit}); no ratio is formed`;
}
export const INCOMPLETE_NO_REASON = "The run states no reason.";
/** Said after the findings: their messages are the runner's free text and are not shown here (§3.2). */
export const INCOMPLETE_MESSAGES_NOTE = "The findings' messages are shown with the rule-check run.";
/** Shown in place of a `STATUS_NOT_DECLARED` finding's subject, which is the evaluated status the
 * pack did not declare (§3.2). */
export const SUBJECT_UNDECLARED_STATUS = "undeclared status";
/** Shown in place of any other finding subject that reads as a rule-check status (§3.2). */
export const SUBJECT_NOT_SHOWN = "subject not shown";

/** The rule-check runner's whole status vocabulary (`RuleCheckStatus` in
 * `core/rules/rule_check_runner`): the only values it emits as an outcome status, and so the only
 * pass or fail tokens a finding subject can carry. Keyed by the type, so a new status must be listed. */
const RULE_CHECK_STATUS_TOKENS: readonly string[] = Object.freeze(Object.keys({
  RULE_INPUTS_INCOMPLETE: true,
  USER_RULE_CHECKED: true,
  USER_RULE_FAILED: true
} satisfies Record<RuleCheckStatus, true>));
const STATUS_NOT_DECLARED = "STATUS_NOT_DECLARED";

const UPPER_BOUND_RELATIONS = new Set(["less_than", "less_than_or_equal"]);
const LOWER_BOUND_RELATIONS = new Set(["greater_than", "greater_than_or_equal"]);
const SECTION_BASIS_SEPARATOR = "; section_basis=";

function metadataText(row: ResultRow, key: "location" | "component" | "basis"): string | null {
  const value = (row.metadata as Record<string, unknown> | undefined)?.[key];
  return typeof value === "string" ? value : null;
}

/** The section basis a row's `metadata.basis` names (result_export::section_basis), or null. */
function sectionBasisOf(row: ResultRow): string | null {
  const token = metadataText(row, "basis");
  if (!token) return null;
  const at = token.lastIndexOf(SECTION_BASIS_SEPARATOR);
  if (at < 0) return null;
  const basis = token.slice(at + SECTION_BASIS_SEPARATOR.length);
  return (SOLVER_SELECTOR_SECTION_BASES as readonly string[]).includes(basis) ? basis : null;
}

function rowLocation(row: ResultRow, inputId: string | null = null): StressRatioLocation {
  return Object.freeze({
    entityRef: row.entity_ref,
    station: metadataText(row, "location"),
    component: metadataText(row, "component"),
    sectionBasis: sectionBasisOf(row),
    resultId: row.id,
    inputId
  });
}

function selectorLocation(selector: SolverResultRef, inputId: string): StressRatioLocation {
  return Object.freeze({
    entityRef: selector.entity_ref,
    station: selector.location ?? null,
    component: selector.component ?? null,
    sectionBasis: selector.section_basis ?? null,
    resultId: null,
    inputId
  });
}

/** One line for a location: entity, station, component and section basis, as stated. */
export function stressRatioLocationText(location: StressRatioLocation): string {
  return [location.entityRef, location.station, location.component, location.sectionBasis?.replace("_", " ") ?? null]
    .filter((part): part is string => part !== null && part !== "")
    .join(" · ");
}

function caseIndex(model: PreviewModel, result: MechanicsResult): Map<string, SupportLoadCase> {
  return new Map(supportLoadCases(model, result).map((item) => [item.key, item] as const));
}

function caseOf(cases: ReadonlyMap<string, SupportLoadCase>, basis: ResultBasisRef | null | undefined): { case: SupportLoadCase | null; label: string } {
  if (!basis) return { case: null, label: "No load case or combination" };
  const key = basisKey(basis)!;
  const known = cases.get(key);
  return known ? { case: known, label: known.label } : { case: null, label: basis.ref_id };
}

// --- Emitted stress groups ------------------------------------------------------------------

function stressGroups(result: MechanicsResult, cases: ReadonlyMap<string, SupportLoadCase>, standing: ResultOverlayStanding | null | undefined): StressRatioGroup[] {
  type KindEntry = { rows: StressRatioRow[]; withheld: Map<string, { case: SupportLoadCase; count: number; reason: string }> };
  const byKind = new Map<string, KindEntry>();
  const wanted = new Set(STRESS_KINDS.map((item) => item.kind));
  result.results.forEach((row, order) => {
    if (!wanted.has(row.kind) || !Number.isFinite(row.value)) return;
    const basis = row.basis_ref;
    if (!basis || (basis.ref_type !== "load_case" && basis.ref_type !== "combination")) return;
    const { case: rowCase, label } = caseOf(cases, basis);
    if (!rowCase) return;
    const entry: KindEntry = byKind.get(row.kind) ?? { rows: [], withheld: new Map() };
    byKind.set(row.kind, entry);
    if (!overlayMayShowRow(standing, row)) {
      const held = entry.withheld.get(rowCase.key) ?? { case: rowCase, count: 0, reason: caseWithheldReason(standing, rowCase.basis) };
      held.count += 1;
      entry.withheld.set(rowCase.key, held);
      return;
    }
    const caseLabel = row.metadata?.basis === RANGE_ENVELOPE_BASIS ? `${label} (range envelope)` : label;
    entry.rows.push(Object.freeze({
      key: row.id,
      order,
      case: rowCase,
      caseLabel,
      locations: Object.freeze([rowLocation(row)]),
      target: Object.freeze({ resultId: row.id, entityRef: row.entity_ref }),
      standing: "admitted",
      withheldReason: null,
      stress: Object.freeze({ value: row.value, unit: row.unit, resultId: row.id }),
      outcome: null
    }));
  });
  const caseOrder = [...cases.keys()];
  return STRESS_KINDS.flatMap((item) => {
    const entry = byKind.get(item.kind);
    if (!entry) return [];
    const withheldCases = [...entry.withheld.values()]
      .sort((a, b) => caseOrder.indexOf(a.case.key) - caseOrder.indexOf(b.case.key))
      .map((held) => Object.freeze({ case: held.case, rowCount: held.count, reason: held.reason }));
    return [Object.freeze({
      id: `stress:${item.kind}`,
      source: "emitted_stress" as const,
      label: item.label,
      ref: item.kind,
      modes: Object.freeze(["stress"] as StressRatioMode[]),
      rows: Object.freeze(entry.rows),
      withheldCases: Object.freeze(withheldCases)
    })];
  });
}

// --- Rule-check groups ----------------------------------------------------------------------

function valueOf(quantity: RuleCheckOutcome["computed_value"]): StressRatioValue | null {
  return quantity && Number.isFinite(quantity.value) ? Object.freeze({ value: quantity.value, unit: quantity.unit_ref }) : null;
}

/** The ratio of §3.1, or why none is formed. */
export function outcomeRatio(outcome: Pick<RuleCheckOutcome, "acceptability_relation" | "computed_value" | "limit_value">): { ratio: number | null; note: string | null } {
  const relation = outcome.acceptability_relation;
  if (relation === "formula_predicate") return { ratio: null, note: RATIO_NOTE_PREDICATE };
  if (LOWER_BOUND_RELATIONS.has(relation)) return { ratio: null, note: RATIO_NOTE_LOWER_BOUND };
  const computed = outcome.computed_value;
  const limit = outcome.limit_value;
  if (!UPPER_BOUND_RELATIONS.has(relation) || !computed || !limit || !Number.isFinite(computed.value) || !Number.isFinite(limit.value)) {
    return { ratio: null, note: RATIO_NOTE_MISSING };
  }
  if (computed.unit_ref !== limit.unit_ref || computed.dimension !== limit.dimension) {
    return { ratio: null, note: ratioNoteUnitsDiffer(computed.unit_ref, limit.unit_ref) };
  }
  if (!(limit.value > 0)) return { ratio: null, note: RATIO_NOTE_LIMIT_NOT_POSITIVE };
  const ratio = computed.value / limit.value;
  return Number.isFinite(ratio) ? { ratio, note: null } : { ratio: null, note: RATIO_NOTE_NOT_FINITE };
}

/** §3.2: a finding subject as shown for an admitted row. The runner puts the evaluated status in
 * a `STATUS_NOT_DECLARED` finding's subject before downgrading the outcome to incomplete, so a
 * subject that contains a status token (in any case) is never echoed. */
function shownSubject(finding: { code: string; subject_id: string }): string {
  const subject = finding.subject_id.toUpperCase();
  if (!RULE_CHECK_STATUS_TOKENS.some((token) => subject.includes(token))) return finding.subject_id;
  return finding.code === STATUS_NOT_DECLARED ? SUBJECT_UNDECLARED_STATUS : SUBJECT_NOT_SHOWN;
}

/** §3.2: the missing inputs, then the run's findings by kind, code and subject, else its diagnostic
 * codes. Built from structured fields only: a finding's message is the runner's free text and can
 * carry a quantity (a table argument, a bound) with no unit to display it by, so it is never copied
 * here, whatever the row's standing. A withheld row's findings carry no subject at all, and an
 * admitted row's never echo a status token (`shownSubject`): a subject can carry the evaluated pass
 * or fail status of a downgraded check. */
export function incompleteReason(outcome: RuleCheckOutcome, standing: StressRatioStanding): string {
  const parts: string[] = [];
  const missing = outcome.bound_inputs.filter((input) => !input.supplied).map((input) => input.input_id);
  if (missing.length > 0) parts.push(`Missing input${missing.length === 1 ? "" : "s"}: ${missing.join(", ")}.`);
  const finding = (kind: string, item: { code: string; subject_id: string }) =>
    standing === "withheld" ? `${kind} finding ${item.code}.` : `${kind} finding ${item.code} (${shownSubject(item)}).`;
  const findings = [
    ...outcome.completeness_findings.map((item) => finding("Completeness", item)),
    ...outcome.evaluator_findings.map((item) => finding("Evaluator", item))
  ];
  parts.push(...findings);
  if (findings.length > 0) parts.push(INCOMPLETE_MESSAGES_NOTE);
  if (parts.length === 0 && outcome.diagnostic_codes.length > 0) parts.push(outcome.diagnostic_codes.join(", "));
  return parts.length > 0 ? parts.join(" ") : INCOMPLETE_NO_REASON;
}

function ruleGroups(
  run: SessionRuleCheckRun,
  result: MechanicsResult,
  cases: ReadonlyMap<string, SupportLoadCase>,
  standing: ResultOverlayStanding | null | undefined
): StressRatioGroup[] | "not_this_result" {
  const plan = deriveRuleCheckBindingPlan(run.rulePackDocument);
  const rowsById = new Map(result.results.map((row) => [row.id, row] as const));
  const solverInputs = new Map(plan.solverInputs.map((input) => [input.input_id, input] as const));
  const checkInputs = new Map(plan.checks.map((check) => [check.check_id, check.input_refs.filter((id) => solverInputs.has(id))] as const));
  // The guard of §2.2: a bound row that is not a row of this result means the run is another result's.
  for (const outcome of run.result.checks) {
    for (const input of outcome.bound_inputs) {
      if (solverInputs.has(input.input_id) && input.result_id && !rowsById.has(input.result_id)) return "not_this_result";
    }
  }
  const byCategory = new Map<string, StressRatioRow[]>();
  run.result.checks.forEach((outcome, order) => {
    const category = outcome.category_ref ?? "";
    const basis = outcome.basis_ref ?? null;
    const { case: outcomeCase, label } = caseOf(cases, basis);
    const locations = (checkInputs.get(outcome.check_id) ?? []).flatMap((inputId): StressRatioLocation[] => {
      const bound = outcome.bound_inputs.find((input) => input.input_id === inputId)?.result_id;
      const row = bound ? rowsById.get(bound) : undefined;
      if (row) return [rowLocation(row, inputId)];
      const selector = solverInputs.get(inputId)?.solver_result_ref;
      return selector ? [selectorLocation(selector, inputId)] : [];
    });
    const firstRow = locations.find((location) => location.resultId !== null);
    const target = Object.freeze({ resultId: firstRow?.resultId ?? null, entityRef: firstRow?.entityRef ?? locations[0]?.entityRef ?? null });
    const admitted = overlayMayShowRow(standing, { basis_ref: basis ?? undefined });
    const incomplete = outcome.status === "RULE_INPUTS_INCOMPLETE";
    // A withheld row carries no value; an incomplete one carries no ratio.
    const showValues = admitted;
    const { ratio, note } = showValues && !incomplete ? outcomeRatio(outcome) : { ratio: null, note: null };
    const rowOutcome: StressRatioOutcome = Object.freeze({
      identity: ruleCheckOutcomeIdentity(outcome),
      checkId: outcome.check_id,
      status: outcome.status,
      relation: outcome.acceptability_relation,
      computed: showValues ? valueOf(outcome.computed_value) : null,
      limit: showValues ? valueOf(outcome.limit_value) : null,
      ratio,
      ratioNote: note,
      incompleteReason: incomplete ? incompleteReason(outcome, admitted ? "admitted" : "withheld") : null
    });
    const rows = byCategory.get(category) ?? [];
    byCategory.set(category, rows);
    rows.push(Object.freeze({
      key: rowOutcome.identity,
      order,
      case: outcomeCase,
      caseLabel: label,
      locations: Object.freeze(locations),
      target,
      standing: admitted ? "admitted" : "withheld",
      withheldReason: admitted ? null : caseWithheldReason(standing, basis),
      stress: null,
      outcome: rowOutcome
    }));
  });
  const declared = plan.stressCategories.map((category) => category.category_id);
  const order = [...declared.filter((id) => byCategory.has(id)), ...[...byCategory.keys()].filter((id) => !declared.includes(id))];
  return order.map((id) => {
    const name = plan.stressCategories.find((category) => category.category_id === id)?.name;
    return Object.freeze({
      id: `rule:${id}`,
      source: "rule_category" as const,
      label: id === "" ? "No stress category" : name ? `${name} (${id})` : id,
      ref: id,
      modes: Object.freeze(["ratio", "stress"] as StressRatioMode[]),
      rows: Object.freeze(byCategory.get(id)!),
      withheldCases: Object.freeze([])
    });
  });
}

function unavailable(reason: string): StressRatioTable {
  return Object.freeze({
    state: "unavailable", reason,
    run: Object.freeze({ state: "none" as const, rulePackId: null, outcomeCount: 0, note: STRESS_RATIO_NO_RUN }),
    groups: Object.freeze([])
  });
}

/**
 * The rule groups (one per stress category of the session's rule-check run, in the pack's order)
 * and the stress groups (one per emitted stress kind, in `STRESS_KINDS` order). `standing` is the
 * viewport overlays' input. `ruleCheckRun` must be a run of this result (App passes it only while
 * `result` is the current solved result); a run that binds a row this result does not hold is not
 * used.
 */
export function buildStressRatioTable(input: {
  model: PreviewModel;
  result: MechanicsResult | null;
  standing: ResultOverlayStanding | null | undefined;
  ruleCheckRun?: SessionRuleCheckRun | null;
}): StressRatioTable {
  const { model, result, standing, ruleCheckRun = null } = input;
  if (!result) return unavailable(STRESS_RATIO_NO_RESULT);
  if (result.status.mechanics !== "MECHANICS_SOLVED") return unavailable(STRESS_RATIO_NOT_SOLVED);
  const cases = caseIndex(model, result);
  let run: StressRatioTable["run"] = Object.freeze({ state: "none", rulePackId: null, outcomeCount: 0, note: STRESS_RATIO_NO_RUN });
  let rules: StressRatioGroup[] = [];
  if (ruleCheckRun) {
    const rulePackId = ruleCheckRun.result.rule_pack_id;
    const outcomeCount = ruleCheckRun.result.checks.length;
    const built = outcomeCount === 0 ? [] : ruleGroups(ruleCheckRun, result, cases, standing);
    if (built === "not_this_result") {
      run = Object.freeze({ state: "not_this_result", rulePackId, outcomeCount, note: STRESS_RATIO_RUN_NOT_THIS_RESULT });
    } else if (outcomeCount === 0) {
      run = Object.freeze({ state: "no_outcomes", rulePackId, outcomeCount, note: STRESS_RATIO_RUN_NO_OUTCOMES });
    } else {
      rules = built;
      run = Object.freeze({ state: "used", rulePackId, outcomeCount, note: stressRatioRunText(rulePackId, outcomeCount) });
    }
  }
  const groups = [...rules, ...stressGroups(result, cases, standing)];
  if (groups.length === 0) return Object.freeze({ state: "unavailable", reason: STRESS_RATIO_NOTHING, run, groups: Object.freeze([]) });
  return Object.freeze({ state: "available", reason: null, run, groups: Object.freeze(groups) });
}

// --- The view: sort, threshold, toggle (§5) -------------------------------------------------

export type StressRatioThreshold = Readonly<
  | { state: "none" }
  | { state: "applied"; text: string; /** In the key's stored unit. */ value: number }
  | { state: "invalid"; text: string }
  | { state: "disabled"; reason: string }
>;

export type StressRatioRankedRow = Readonly<{ row: StressRatioRow; key: number; hidden: boolean }>;

export type StressRatioView = Readonly<{
  mode: StressRatioMode;
  /** The stored unit of the key (stress mode), or null (ratio, or units differ). */
  keyUnit: string | null;
  /** False when the keyed rows' units differ: source order, and no threshold. */
  sorted: boolean;
  unitNote: string | null;
  threshold: StressRatioThreshold;
  /** Incomplete outcomes, then withheld outcomes, in run order; never sorted or hidden. */
  pinned: readonly StressRatioRow[];
  /** Keyed rows, largest key first (or source order when not sorted), each marked hidden or not. */
  ranked: readonly StressRatioRankedRow[];
  /** Admitted outcomes with no key in this mode, in run order; never hidden. */
  unranked: readonly StressRatioRow[];
  hiddenCount: number;
  shownCount: number;
  totalCount: number;
}>;

export const THRESHOLD_INVALID_NOTE = "Not a number; no threshold is applied.";
export function unitsDifferNote(units: readonly string[]): string {
  return `Units differ (${units.join(", ")}); not sorted and no threshold is applied.`;
}

const THRESHOLD_TEXT = /^([+-]?)(\d+(?:\.\d*)?|\.\d+)(?:[eE]([+-]?\d+))?$/;

/** The entered threshold in the key's stored unit. The text is in the display unit; a value stored
 * in Pa is displayed in MPa, so the text's decimal exponent is shifted by six (1 MPa = 10^6 Pa,
 * exact) and the shifted decimal is read once. Null when the text is not a number. */
export function thresholdInStoredUnit(text: string, storedUnit: string | null): number | null {
  const match = THRESHOLD_TEXT.exec(text.trim());
  if (!match) return null;
  const [, sign, digits, exponent = "0"] = match;
  const shift = storedUnit === PASCAL ? 6 : 0;
  const value = Number(`${sign}${digits}e${Number.parseInt(exponent, 10) + shift}`);
  return Number.isFinite(value) ? value : null;
}

function keyOf(row: StressRatioRow, mode: StressRatioMode): { key: number; unit: string | null } | null {
  if (row.stress) return { key: Math.abs(row.stress.value), unit: row.stress.unit };
  const outcome = row.outcome;
  if (!outcome || row.standing !== "admitted" || outcome.status === "RULE_INPUTS_INCOMPLETE") return null;
  if (mode === "ratio") return outcome.ratio === null ? null : { key: outcome.ratio, unit: null };
  return outcome.computed ? { key: Math.abs(outcome.computed.value), unit: outcome.computed.unit } : null;
}

/** Only a ranked stress row, or a ranked outcome the rule check reports as checked, can be hidden
 * by the threshold or left to a later page. A failed outcome never is. */
function mayBeOmitted(row: StressRatioRow): boolean {
  return row.stress !== null || row.outcome?.status === "USER_RULE_CHECKED";
}

const STATUS_RANK: Readonly<Record<string, number>> = Object.freeze({ USER_RULE_FAILED: 0, USER_RULE_CHECKED: 1 });

/** The view of one group under a mode and a threshold text (§5). Pure; frozen. */
export function stressRatioView(group: StressRatioGroup, options: { mode: StressRatioMode; threshold: string }): StressRatioView {
  const mode = group.modes.includes(options.mode) ? options.mode : group.modes[0];
  const pinned = [
    ...group.rows.filter((row) => row.outcome?.status === "RULE_INPUTS_INCOMPLETE"),
    ...group.rows.filter((row) => row.standing === "withheld" && row.outcome?.status !== "RULE_INPUTS_INCOMPLETE")
  ];
  const keyed: Array<{ row: StressRatioRow; key: number; unit: string | null }> = [];
  const unranked: StressRatioRow[] = [];
  for (const row of group.rows) {
    if (pinned.includes(row)) continue;
    const key = keyOf(row, mode);
    if (key) keyed.push({ row, ...key });
    else unranked.push(row);
  }
  const units = [...new Set(keyed.map((item) => item.unit).filter((unit): unit is string => unit !== null))].sort();
  const sorted = units.length <= 1;
  const keyUnit = mode === "stress" && units.length === 1 ? units[0] : null;
  const ordered = sorted
    ? [...keyed].sort((a, b) =>
        (b.key - a.key) ||
        ((STATUS_RANK[a.row.outcome?.status ?? ""] ?? 0) - (STATUS_RANK[b.row.outcome?.status ?? ""] ?? 0)) ||
        (a.row.order - b.row.order))
    : keyed;
  const text = options.threshold.trim();
  let threshold: StressRatioThreshold;
  if (!sorted) threshold = Object.freeze({ state: "disabled", reason: unitsDifferNote(units) });
  else if (text === "") threshold = Object.freeze({ state: "none" });
  else {
    const value = thresholdInStoredUnit(text, keyUnit);
    threshold = value === null ? Object.freeze({ state: "invalid", text }) : Object.freeze({ state: "applied", text, value });
  }
  const ranked = ordered.map((item) => Object.freeze({
    row: item.row,
    key: item.key,
    hidden: threshold.state === "applied" && mayBeOmitted(item.row) && item.key < threshold.value
  }));
  const hiddenCount = ranked.filter((item) => item.hidden).length;
  const withheldRowCount = group.withheldCases.reduce((sum, item) => sum + item.rowCount, 0);
  const totalCount = group.rows.length + withheldRowCount;
  return Object.freeze({
    mode,
    keyUnit,
    sorted,
    unitNote: sorted ? null : unitsDifferNote(units),
    threshold,
    pinned: Object.freeze(pinned),
    ranked: Object.freeze(ranked),
    unranked: Object.freeze(unranked),
    hiddenCount,
    shownCount: totalCount - hiddenCount,
    totalCount
  });
}

// --- The page (§7) ---------------------------------------------------------------------------

export type StressRatioPage = Readonly<{
  /** The ranked rows listed, each with its rank: its position among the rows at or above the
   * threshold, so a failed outcome listed beyond the page keeps its own rank. */
  ranked: readonly Readonly<{ row: StressRatioRow; rank: number }>[];
  /** Rows listed: pinned (and withheld stress rows), the listed ranked rows and the not-ranked rows. */
  listedCount: number;
  /** Rows at or above the threshold that a later page lists. */
  notListedCount: number;
}>;

/** The ranked rows one page lists: the first `limit` stress rows or checked outcomes at or above the
 * threshold, and every failed outcome at its rank whatever its position, the mode, the threshold or
 * the page length (FR-RULE-003 keeps incomplete ones pinned; this keeps failed ones listed). */
export function stressRatioPage(view: StressRatioView, limit: number): StressRatioPage {
  const ranked: Array<Readonly<{ row: StressRatioRow; rank: number }>> = [];
  let paged = 0;
  let notListedCount = 0;
  view.ranked.filter((item) => !item.hidden).forEach((item, index) => {
    if (mayBeOmitted(item.row)) {
      paged += 1;
      if (paged > limit) {
        notListedCount += 1;
        return;
      }
    }
    ranked.push(Object.freeze({ row: item.row, rank: index + 1 }));
  });
  return Object.freeze({ ranked: Object.freeze(ranked), listedCount: view.shownCount - notListedCount, notListedCount });
}
