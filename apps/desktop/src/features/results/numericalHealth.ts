/** The numerical health indicator's one derived presentation object (DEL-07-05 Design
 * `numerical-health-indicator.md` §2.6, §3, §5).
 *
 * The Rust `numerical_health_json` (wasm, `core/model_operations/operation_applier`)
 * decides every state, the conditioning figure, its sensitivity flag, the located causes
 * and `verdict_only`. This module carries the result text and the reader's findings to
 * it, checks the shape of what comes back and fails closed: an absent export, a refusal
 * or any unexpected shape gives no indicator, and every surface keeps today's text. It
 * then only arranges and formats. It never takes part in admission, standing or an
 * output gate. */
import { loadedWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { readerSourceText } from "./resultReader";
import {
  CONDITIONING_CAUSE_TEXT,
  DIGITS_BELOW_FIRST_DECIMAL,
  ELIGIBLE_RECOVERY_TEXT,
  KEPT_FOR_INSPECTION_TEXT,
  MECHANISM_TEXT,
  NEGATIVE_ENERGY_TEXT,
  QUALITY_UNRESOLVED_CLOSING_TEXT,
  RECOVERY_BASIS_UNQUALIFIED_TEXT,
  SEE_DIAGNOSTIC_TEXT,
  SENSITIVE_CLOSING_TEXT,
  checksPassedText,
  combinationText,
  conditioningDetailText,
  formationCauseText,
  formationRecoveryCauseText,
  loadFidelityCauseText,
  nodeDofLocationText,
  notAssessedText,
  numericalHealthCountHeadline,
  numericalHealthHeadline,
  outputSentence,
  qualityUnresolvedLeadText,
  sensitiveLeadText,
  solveFailedLeadText,
  type NumericalHealthState
} from "./numericalHealthText";
import type { MechanicsResult, ResultBasisRef } from "../../types";

export type { NumericalHealthState } from "./numericalHealthText";
export type NumericalHealthRoute = "generic" | "pressure_source" | "retained" | "unsupported";
export type NumericalHealthCauseKind =
  | "conditioning" | "load_fidelity" | "formation" | "formation_recovery" | "curved_bend_short"
  | "recovery_basis_unqualified" | "mechanism" | "negative_energy";
export type NumericalHealthLocation =
  | Readonly<{ kind: "node_dof"; node: string; dof: string }>
  | Readonly<{ kind: "ref"; ref: string }>
  | Readonly<{ kind: "text"; text: string }>;
export type NumericalHealthCause = Readonly<{ kind: NumericalHealthCauseKind; diagnostic_id: string | null; locations: readonly NumericalHealthLocation[] }>;
export type NumericalHealthCase = Readonly<{
  case_id: string;
  state: NumericalHealthState;
  code: string | null;
  integrity_diagnostic_id: string | null;
  rcond: number | null;
  digits: number | null;
  conditioning_sensitive: boolean;
  causes: readonly NumericalHealthCause[];
}>;
/** A combination and the load cases it names. */
export type NumericalHealthCombination = Readonly<{
  combination_id: string;
  /** Null for a withheld combination: it has no health of its own. */
  state: NumericalHealthState | null;
  cases: readonly string[];
  withheld: boolean;
}>;
/** The Rust parse output, shape-checked. */
export type NumericalHealth = Readonly<{
  route: NumericalHealthRoute;
  verdict_only: boolean;
  cases: readonly NumericalHealthCase[];
  combinations: readonly NumericalHealthCombination[];
}>;

const ROUTES: readonly string[] = ["generic", "pressure_source", "retained", "unsupported"];
const STATES: readonly string[] = ["checks_passed", "sensitive", "quality_unresolved", "solve_failed", "not_assessed"];
const CAUSE_KINDS: readonly string[] = [
  "conditioning", "load_fidelity", "formation", "formation_recovery", "curved_bend_short",
  "recovery_basis_unqualified", "mechanism", "negative_energy"
];

class Shape extends Error {}
function fail(): never { throw new Shape(); }
/** Every member of the response is required: nothing is defaulted, so a missing or
 * ill-typed member means no indicator at all (fail closed). */
function field(o: Record<string, unknown>, key: string): unknown {
  return Object.hasOwn(o, key) ? o[key] : fail();
}
function obj(value: unknown): Record<string, unknown> {
  return value !== null && typeof value === "object" && !Array.isArray(value) ? value as Record<string, unknown> : fail();
}
function arr(value: unknown): unknown[] {
  return Array.isArray(value) ? value : fail();
}
function str(value: unknown): string {
  return typeof value === "string" && value.length > 0 ? value : fail();
}
function strOrNull(value: unknown): string | null {
  return value === null ? null : str(value);
}
function numOrNull(value: unknown): number | null {
  if (value === null) return null;
  return typeof value === "number" && Number.isFinite(value) ? value : fail();
}
function bool(value: unknown): boolean {
  return typeof value === "boolean" ? value : fail();
}
function state(value: unknown): NumericalHealthState {
  return typeof value === "string" && STATES.includes(value) ? value as NumericalHealthState : fail();
}
function location(value: unknown): NumericalHealthLocation {
  const o = obj(value);
  switch (field(o, "kind")) {
    case "node_dof": return { kind: "node_dof", node: str(field(o, "node")), dof: str(field(o, "dof")) };
    case "ref": return { kind: "ref", ref: str(field(o, "ref")) };
    case "text": return { kind: "text", text: str(field(o, "text")) };
    default: return fail();
  }
}
function cause(value: unknown): NumericalHealthCause {
  const o = obj(value);
  const kind = field(o, "kind");
  if (typeof kind !== "string" || !CAUSE_KINDS.includes(kind)) fail();
  return { kind: kind as NumericalHealthCauseKind, diagnostic_id: strOrNull(field(o, "diagnostic_id")), locations: arr(field(o, "locations")).map(location) };
}
function healthCase(value: unknown): NumericalHealthCase {
  const o = obj(value);
  const digits = numOrNull(field(o, "digits"));
  if (digits !== null && digits < 0) fail();
  return {
    case_id: str(field(o, "case_id")),
    state: state(field(o, "state")),
    code: strOrNull(field(o, "code")),
    integrity_diagnostic_id: strOrNull(field(o, "integrity_diagnostic_id")),
    rcond: numOrNull(field(o, "rcond")),
    digits,
    conditioning_sensitive: bool(field(o, "conditioning_sensitive")),
    causes: arr(field(o, "causes")).map(cause)
  };
}
function combination(value: unknown): NumericalHealthCombination {
  const o = obj(value);
  const combinationState = field(o, "state");
  return {
    combination_id: str(field(o, "combination_id")),
    state: combinationState === null ? null : state(combinationState),
    cases: arr(field(o, "cases")).map(str),
    withheld: bool(field(o, "withheld"))
  };
}

/** The shape check of `numerical_health_json`'s output; null for anything else. */
export function parseNumericalHealth(value: unknown): NumericalHealth | null {
  try {
    const o = obj(value);
    const routeValue = field(o, "route");
    const route = typeof routeValue === "string" && ROUTES.includes(routeValue) ? routeValue as NumericalHealthRoute : fail();
    const verdictOnly = bool(field(o, "verdict_only"));
    // Rust gives verdict_only only on a generic route; anything else is not its output.
    if (verdictOnly && route !== "generic") fail();
    const cases = arr(field(o, "cases")).map(healthCase);
    if (new Set(cases.map((c) => c.case_id)).size !== cases.length) fail();
    const combinations = arr(field(o, "combinations")).map(combination);
    if (new Set(combinations.map((c) => c.combination_id)).size !== combinations.length) fail();
    return Object.freeze({ route, verdict_only: verdictOnly, cases: Object.freeze(cases), combinations: Object.freeze(combinations) });
  } catch (error) {
    if (error instanceof Shape) return null;
    throw error;
  }
}

const CACHE_LIMIT = 8;
/** Readings by source text, then by the findings' text: a lookup keys on the source
 * text itself, never on a copy of it joined to the findings. */
const cache = new Map<string, Map<string, NumericalHealth | null>>();
let cacheOwner: unknown = null;

/** The Rust indicator for this result and the reader's findings for it (the standing
 * call's `findings`), or null when the engine, its export or a readable output is
 * missing (fail closed). */
export function readNumericalHealth(result: MechanicsResult, readerFindings?: readonly string[] | null): NumericalHealth | null {
  const engine = loadedWasmEngine();
  if (!engine?.numericalHealthJson) return null;
  // Readings are kept per engine binding: another binding reads afresh.
  if (cacheOwner !== engine.numericalHealthJson) { cache.clear(); cacheOwner = engine.numericalHealthJson; }
  let sourceJson: string;
  try { sourceJson = readerSourceText(result); } catch { return null; }
  const findingsJson = readerFindings ? JSON.stringify([...readerFindings]) : undefined;
  const key = findingsJson ?? "";
  let readings = cache.get(sourceJson);
  if (readings) { cache.delete(sourceJson); cache.set(sourceJson, readings); }
  if (readings?.has(key)) return readings.get(key)!;
  let health: NumericalHealth | null;
  try { health = parseNumericalHealth(JSON.parse(engine.numericalHealthJson(sourceJson, findingsJson))); } catch { health = null; }
  if (!readings) {
    readings = new Map();
    cache.set(sourceJson, readings);
    while (cache.size > CACHE_LIMIT) cache.delete(cache.keys().next().value!);
  }
  readings.set(key, health);
  return health;
}

/* -------------------------------------------------------------------------- *
 * Presentation
 * -------------------------------------------------------------------------- */
export type NumericalHealthBadge = "green" | "red" | "grey";
/** The header's count order, which is also the order of the worst state. */
export const NUMERICAL_HEALTH_ORDER: readonly NumericalHealthState[] = ["solve_failed", "quality_unresolved", "sensitive", "not_assessed", "checks_passed"];

export function badgeOf(state: NumericalHealthState): NumericalHealthBadge {
  return state === "checks_passed" ? "green" : state === "not_assessed" ? "grey" : "red";
}
/** Red if any case is red, otherwise grey if any is grey, green only if all pass. No case reads grey. */
export function headerBadge(cases: readonly Pick<NumericalHealthCase, "state">[]): NumericalHealthBadge {
  if (cases.some((c) => badgeOf(c.state) === "red")) return "red";
  if (cases.length === 0 || cases.some((c) => badgeOf(c.state) === "grey")) return "grey";
  return "green";
}
export function worstState(cases: readonly Pick<NumericalHealthCase, "state">[]): NumericalHealthState {
  return NUMERICAL_HEALTH_ORDER.find((s) => cases.some((c) => c.state === s)) ?? "not_assessed";
}
/** The header's counts, load cases each counted once. */
export function stateCounts(cases: readonly Pick<NumericalHealthCase, "state">[]): [NumericalHealthState, number][] {
  return NUMERICAL_HEALTH_ORDER.map((s) => [s, cases.filter((c) => c.state === s).length]);
}

/** The conditioning figure: one decimal, two when 7.7 ≤ d ≤ 7.9; any d below 0.05
 * (including d = 0, rcond = 1) is "below 0.1", so "0.0" never appears. */
export function formatDigits(d: number): string {
  if (d < 0.05) return DIGITS_BELOW_FIRST_DECIMAL;
  return d >= 7.7 && d <= 7.9 ? d.toFixed(2) : d.toFixed(1);
}

export function locationText(location: NumericalHealthLocation): string {
  switch (location.kind) {
    case "node_dof": return nodeDofLocationText(location.node, location.dof);
    case "ref": return location.ref;
    case "text": return location.text;
  }
}
function locationsText(causes: readonly NumericalHealthCause[]): string {
  return causes.flatMap((c) => c.locations).map(locationText).join("; ");
}

/** The detail sentences of one case, in the design's order. `eligible` is the reader's
 * standing (the recovery passed, on the pressure-source route). */
export function caseDetailSentences(c: NumericalHealthCase, route: NumericalHealthRoute, eligible: boolean): string[] {
  const of = (kind: NumericalHealthCauseKind) => c.causes.filter((x) => x.kind === kind);
  const sentences: string[] = [];
  switch (c.state) {
    case "checks_passed":
      sentences.push(checksPassedText(c.case_id));
      break;
    case "sensitive": {
      sentences.push(sensitiveLeadText(c.case_id));
      if (!eligible) sentences.push(KEPT_FOR_INSPECTION_TEXT);
      const reasons: string[] = [];
      if (c.conditioning_sensitive || of("conditioning").length > 0) reasons.push(CONDITIONING_CAUSE_TEXT);
      if (of("load_fidelity").length > 0) reasons.push(loadFidelityCauseText(locationsText(of("load_fidelity"))));
      if (of("formation").length > 0) reasons.push(formationCauseText(locationsText(of("formation"))));
      if (of("formation_recovery").length > 0) reasons.push(formationRecoveryCauseText(locationsText(of("formation_recovery"))));
      sentences.push(...(reasons.length > 0 ? reasons : [SEE_DIAGNOSTIC_TEXT]), SENSITIVE_CLOSING_TEXT);
      break;
    }
    case "quality_unresolved":
      sentences.push(qualityUnresolvedLeadText(c.case_id));
      if (of("recovery_basis_unqualified").length > 0 || c.code === "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED") sentences.push(RECOVERY_BASIS_UNQUALIFIED_TEXT);
      sentences.push(SEE_DIAGNOSTIC_TEXT, QUALITY_UNRESOLVED_CLOSING_TEXT);
      break;
    case "solve_failed":
      sentences.push(solveFailedLeadText(c.case_id));
      sentences.push(of("mechanism").length > 0 ? MECHANISM_TEXT : of("negative_energy").length > 0 ? NEGATIVE_ENERGY_TEXT : SEE_DIAGNOSTIC_TEXT);
      break;
    case "not_assessed":
      sentences.push(notAssessedText(c.case_id));
      break;
  }
  if (route === "pressure_source" && eligible) sentences.push(ELIGIBLE_RECOVERY_TEXT);
  return sentences;
}

export type NumericalHealthCaseView = Readonly<{
  caseId: string;
  state: NumericalHealthState;
  badge: NumericalHealthBadge;
  headline: string;
  sentences: readonly string[];
  /** The conditioning detail, for checks passed and sensitive cases with a figure. */
  conditioning: string | null;
  locations: readonly NumericalHealthLocation[];
  integrityDiagnosticId: string | null;
}>;
export type NumericalHealthCombinationView = Readonly<{
  combinationId: string;
  state: NumericalHealthState;
  badge: NumericalHealthBadge;
  text: string;
}>;
/** The one derived presentation object every surface reads. */
export type NumericalHealthView = Readonly<{
  route: "generic" | "pressure_source";
  badge: NumericalHealthBadge;
  /** The header (count form; "Ordinary solve —" on pressure-source). */
  headline: string;
  /** The rail's short form, "Numerical health: {worst state}". */
  shortHeadline: string;
  /** Present only when the generic result is held back by its verdict alone. */
  outputSentence: string | null;
  /** No row badges on the pressure-source route: its values are the producer's recovery. */
  rowBadges: boolean;
  cases: readonly NumericalHealthCaseView[];
  combinations: readonly NumericalHealthCombinationView[];
  caseState: (caseId: string) => NumericalHealthState | null;
  combinationState: (combinationId: string) => NumericalHealthCombinationView | null;
}>;

/** The presentation of the generic and pressure-source routes; null for the retained and
 * unsupported routes, whose texts are unchanged. `verdictHold` is `numericalHealthHold`'s
 * condition: the generic result is not Current because of its verdict alone. */
export function numericalHealthView(health: NumericalHealth | null, options: { eligible: boolean; verdictHold: boolean }): NumericalHealthView | null {
  if (!health || (health.route !== "generic" && health.route !== "pressure_source")) return null;
  const route = health.route;
  const prefix = route === "pressure_source";
  const caseStates = new Map(health.cases.map((c) => [c.case_id, c.state]));
  const cases = health.cases.map((c): NumericalHealthCaseView => ({
    caseId: c.case_id,
    state: c.state,
    badge: badgeOf(c.state),
    headline: numericalHealthHeadline(c.state, prefix),
    sentences: caseDetailSentences(c, route, options.eligible),
    conditioning: c.digits !== null && (c.state === "checks_passed" || c.state === "sensitive") ? conditioningDetailText(formatDigits(c.digits)) : null,
    locations: c.causes.flatMap((x) => x.locations),
    integrityDiagnosticId: c.integrity_diagnostic_id
  }));
  // A withheld combination (or one with no state) keeps its existing finding and gets
  // no badge or detail of its own.
  const combinations = health.combinations.flatMap((m): NumericalHealthCombinationView[] => m.withheld || m.state === null ? [] : [{
    combinationId: m.combination_id,
    state: m.state,
    badge: badgeOf(m.state),
    // A case missing from numerical_quality reads "not assessed".
    text: combinationText(m.cases.map((caseId) => [caseId, caseStates.get(caseId) ?? "not_assessed"] as const))
  }]);
  const notPassed = health.cases.filter((c) => c.state !== "checks_passed").length;
  const byCombination = new Map(combinations.map((m) => [m.combinationId, m]));
  return Object.freeze({
    route,
    badge: headerBadge(health.cases),
    headline: health.cases.length === 0 ? numericalHealthHeadline("not_assessed", prefix) : numericalHealthCountHeadline(stateCounts(health.cases), prefix),
    shortHeadline: numericalHealthHeadline(worstState(health.cases)),
    outputSentence: route === "generic" && options.verdictHold && notPassed > 0 ? outputSentence(notPassed) : null,
    rowBadges: route === "generic",
    cases,
    combinations,
    caseState: (caseId: string) => caseStates.get(caseId) ?? null,
    combinationState: (combinationId: string) => byCombination.get(combinationId) ?? null
  });
}

/** What the shell chip and the rail show instead of "Needs recompute". */
export type NumericalHealthHold = Readonly<{ headline: string; shortHeadline: string; outputSentence: string | null }>;

/** The chip and rail replacement predicate (§3 "Replacing 'Needs recompute'"): every
 * Current requirement other than numerical eligibility holds, the Rust `verdict_only` is
 * true, and some load case is red or grey. Anything else, including missing or
 * malformed evidence, keeps "Needs recompute". Admission is not read or changed here. */
export function numericalHealthHold(input: { isCurrent: boolean; otherCurrentRequirementsHold: boolean; health: NumericalHealth | null }): boolean {
  const { isCurrent, otherCurrentRequirementsHold, health } = input;
  return !isCurrent && otherCurrentRequirementsHold && health !== null && health.verdict_only
    && health.cases.some((c) => badgeOf(c.state) !== "green");
}

/** The state the results table badges a row with: its load case's state, or its combination's
 * inherited state. Null where the table shows no row badge: the pressure-source route (its rows
 * are the producer's recovery), a row with no basis, or a basis with no state. */
export function rowHealthState(health: NumericalHealthView | null, row: Readonly<{ basis_ref?: ResultBasisRef }>): NumericalHealthState | null {
  if (!health || !health.rowBadges || !row.basis_ref) return null;
  const { ref_type: refType, ref_id: refId } = row.basis_ref;
  return refType === "load_case" ? health.caseState(refId)
    : refType === "combination" ? health.combinationState(refId)?.state ?? null : null;
}

/** What a result overlay in the viewport (result colour, reaction arrows) is given: the reader's
 * numerical standing of the result and the session's numerical health presentation of it. */
export type ResultOverlayStanding = Readonly<{ eligible: boolean; health: NumericalHealthView | null }>;

/**
 * Whether a row's value may drive a result overlay. It reuses, and adds nothing to, the rules the
 * results surface already applies: the reader's numerical standing admits the result (when it does
 * not, the header says that no results of the run can be exported or used by rule checks), the
 * numerical health indicator exists for the result's route, and the results table gives the row no
 * badge other than "checks passed". Anything missing fails closed. Presentation only.
 */
export function overlayMayShowRow(standing: ResultOverlayStanding | null | undefined, row: Readonly<{ basis_ref?: ResultBasisRef }>): boolean {
  if (!standing || !standing.eligible || !standing.health) return false;
  if (!standing.health.rowBadges) return true;
  return rowHealthState(standing.health, row) === "checks_passed";
}
