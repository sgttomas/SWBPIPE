/** The hanger-sizing display model (DEL-04-11 Design `hanger-sizing.md` §8.1, from the document
 * contract of §8.3). Increment 3a: a pure adapter, tested on the committed family fixtures. It is
 * not a desktop feature by itself; the table component and CSV (3b) and the native sizing job
 * (3c) consume it.
 *
 * What it does:
 * - Passes every value the document publishes through exactly, in document order. Nothing is
 *   rounded, converted or recomputed; quantities are shown in SI with their unit, as the
 *   document's `basis_statement` states them. A null variation is "no ratio formed", never 0.
 * - Shows a record's name, type, k, working load range and travel range only from a supplied
 *   library whose checked canonical hash (`canonicalSha256HexCheckedV1`, the desktop's one
 *   checked hash) equals `inputs.library.sha256`, and only for a record found by `hanger_id`
 *   whose own checked hash equals the selection's `record_sha256`. Every other case is a named
 *   not-shown state; record quantities are shown as the library states them, unconverted.
 * - Shows the readers' standings only from supplied evidence that carries the checked
 *   hash of the exact document it was computed on, recomputed here and compared. Absent,
 *   malformed or differently bound evidence shows every standing as `not_checked`.
 *
 * - Refuses to build a sized view (it throws) whose `integrity_bound`, or any solve's
 *   `solver_standing` or `support_state_confirmation`, is outside its closed vocabulary, whose
 *   integrity disclosures are not the engine's exact messages, one per non-passing solve, or whose
 *   standings are not what those disclosures state (`hangerSizingResultFault`).
 * - Reads its inputs once, synchronously, before its first await, and builds the view from those
 *   copies; a change to an input while the view is being built does not reach it.
 *
 * What it does not do: validate the rest of the document, mint Current or acceptance, state that
 * typed results are trusted, compute anything per record, or fall back to a native or `evaluate`
 * path. It never mutates its inputs. */

import { canonicalSha256HexCheckedV1, checkedJsonText } from "../../services/hashService";
import type {
  HangerSizingCandidate,
  HangerSizingConfirmationOutcome,
  HangerSizingDocument,
  HangerSizingFamilyPolicy,
  HangerSizingIntegrityBound,
  HangerSizingOutcome,
  HangerSizingRefusedDocument,
  HangerSizingRequest,
  HangerSizingSizedDocument,
  HangerSizingSolverStanding,
  HangerSizingSupportStateConfirmation
} from "./hangerSizingResult";
import { supportStatesLine, type SupportStatesLine } from "./integrityStatus";
import { BOUNDARY_SENTENCE } from "./numericalHealthText";

// ---------------------------------------------------------------------------------------------
// Labels and fixed display text

/** Exact labels of Design §8.1: F_cold and F_C are never read as one where u_C is not zero. */
export const F_COLD_LABEL = "installed setting (at drawn position)";
export const F_COLD_CONDITION_LABEL = "cold-condition load";
/** A variation the document publishes as null. */
export const NO_RATIO_FORMED = "no ratio formed";

// ---------------------------------------------------------------------------------------------
// The integrity bound and each solve's standing and confirmation: closed, and shown only when well formed
//
// Every surface (the native reply's reader, this view, the panel and both CSV files) checks these members with
// the predicates below and words them with the formatters below. A value outside them fails closed: it is never
// shown, exported or re-worded as something else.

/** A derived solve's `solver_standing`: `solver_checks_passed`, then the four non-passing tokens. */
export const HANGER_SIZING_SOLVER_STANDINGS: readonly HangerSizingSolverStanding[] = Object.freeze([
  "solver_checks_passed", "not_assessed", "sensitive", "unresolved", "failed"
]);
/** A derived solve's `support_state_confirmation.outcome`. */
export const HANGER_SIZING_CONFIRMATION_OUTCOMES: readonly HangerSizingConfirmationOutcome[] = Object.freeze([
  "solver_reported_established", "not_attempted_reason_not_recorded", "not_established_detail_quoted", "no_confirmation_record", "not_applicable"
]);
const DERIVED_SOLVE_INTEGRITY = "HANGER_SIZING_DERIVED_SOLVE_INTEGRITY";
const CORRECTION_UNAVAILABLE = "DISPLACEMENT_CORRECTION_UNAVAILABLE";

/** The support-state line of a solve that publishes no nonlinear-support state (`not_applicable`). */
export const HANGER_SIZING_CONFIRMATION_NOT_APPLICABLE = "Support states: not applicable; this solve publishes no nonlinear-support state.";

/** Exactly `{status: "not_verified", reason: "inverse_norm_bound_not_verified"}`, as the document and the reader state it. */
export function wellFormedIntegrityBound(value: unknown): value is HangerSizingIntegrityBound {
  return isPlainObject(value)
    && Object.keys(value).length === 2
    && value.status === "not_verified"
    && value.reason === "inverse_norm_bound_not_verified";
}

/** One of `HANGER_SIZING_SOLVER_STANDINGS`. */
export function wellFormedSolverStanding(value: unknown): value is HangerSizingSolverStanding {
  return typeof value === "string" && (HANGER_SIZING_SOLVER_STANDINGS as readonly string[]).includes(value);
}

/** The four standings other than `solver_checks_passed`, each disclosed by `HANGER_SIZING_DERIVED_SOLVE_INTEGRITY`. */
export const HANGER_SIZING_NONPASSING_STANDINGS: readonly Exclude<HangerSizingSolverStanding, "solver_checks_passed">[] = Object.freeze([
  "not_assessed", "sensitive", "unresolved", "failed"
]);

/** The fixed message of a `HANGER_SIZING_DERIVED_SOLVE_INTEGRITY` disclosure, exactly as the engine forms it
 * (`core/analysis/hanger_sizing/src/derived.rs` `integrity_message`) and the schema enumerates it. */
export function integrityDisclosureMessage(token: string): string {
  return `${token}: the solver's numerical-integrity status for this derived solve is ${token}, not solver_checks_passed; every value sizing takes from it carries that standing (integrity_bound: not_verified, inverse_norm_bound_not_verified)`;
}

/** The 30 code points a quoted `detail` may not consist of alone: the union of Rust `char::is_whitespace`, Python
 * `str.isspace` and ECMA-262 `\s` (`String.prototype.trim`). The same set is the engine's `is_blank_detail`, the
 * Python verifier's `_is_blank_detail` and the schema's `SupportStateConfirmation` detail pattern. */
const BLANK_DETAIL_CHAR = /^[\u0009-\u000D\u001C-\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF]$/u;

/** True when every code point of `s` is a blank-detail character (an empty string included). */
export function isBlankDetail(s: string): boolean {
  return [...s].every((c) => BLANK_DETAIL_CHAR.test(c));
}

/** Exactly `{outcome, detail}`: the outcome one of `HANGER_SIZING_CONFIRMATION_OUTCOMES`, and `detail` a non-blank
 * string (`isBlankDetail`) for `not_established_detail_quoted` and null otherwise. With `stateless` given, the outcome
 * is also `not_applicable` exactly when the solve publishes no nonlinear-support state. */
export function wellFormedSupportStateConfirmation(value: unknown, stateless?: boolean): value is HangerSizingSupportStateConfirmation {
  if (!isPlainObject(value) || Object.keys(value).length !== 2 || !Object.hasOwn(value, "outcome") || !Object.hasOwn(value, "detail")) return false;
  const { outcome, detail } = value;
  if (typeof outcome !== "string" || !(HANGER_SIZING_CONFIRMATION_OUTCOMES as readonly string[]).includes(outcome)) return false;
  if (stateless !== undefined && stateless !== (outcome === "not_applicable")) return false;
  return outcome === "not_established_detail_quoted" ? typeof detail === "string" && !isBlankDetail(detail) : detail === null;
}

type SolveFacts = Readonly<{ label: string; stateless: boolean; standing: unknown; confirmation: unknown }>;
type FindingFacts = Readonly<{ code: unknown; names: unknown; message: unknown }>;

/** The first fault of a result's solves, or null:
 * - two solves with one label;
 * - a `HANGER_SIZING_DERIVED_SOLVE_INTEGRITY` disclosure that does not name exactly one known solve, or whose message is
 *   not `integrityDisclosureMessage` of a non-passing token, or a second one for the same solve;
 * - a `DISPLACEMENT_CORRECTION_UNAVAILABLE` disclosure that does not name exactly one known solve, or whose message is
 *   not a non-empty string, or a second one for the same solve;
 * - a standing or confirmation that is not well formed;
 * - a standing that is not exactly what the integrity disclosures state of its solve (a non-passing standing has one
 *   disclosure naming it with its token; `solver_checks_passed` has none);
 * - a solve whose confirmation is `solver_reported_established` beside its `DISPLACEMENT_CORRECTION_UNAVAILABLE`
 *   disclosure: the producer withdraws the confirmed state when the displacement correction fails. */
function solvesFault(solves: readonly SolveFacts[], disclosures: readonly FindingFacts[]): string | null {
  const labels = new Set<string>();
  for (const solve of solves) {
    if (labels.has(solve.label)) return `HANGER_SIZING_SOLVE_LABEL_DUPLICATE: ${solve.label}`;
    labels.add(solve.label);
  }
  /** The one known solve label a per-solve disclosure names, or null. */
  const solveLabel = (names: unknown): string | null =>
    Array.isArray(names) && names.length === 1 && typeof names[0] === "string" && labels.has(names[0]) ? names[0] : null;
  const stated = new Map<string, string>();
  const correctionUnavailable = new Set<string>();
  for (const [index, d] of disclosures.entries()) {
    if (d.code === CORRECTION_UNAVAILABLE) {
      const label = solveLabel(d.names);
      if (label === null || typeof d.message !== "string" || d.message === "") {
        return `HANGER_SIZING_CORRECTION_UNAVAILABLE_DISCLOSURE_MALFORMED: disclosures[${index}]`;
      }
      if (correctionUnavailable.has(label)) return `HANGER_SIZING_CORRECTION_UNAVAILABLE_DISCLOSURE_DUPLICATE: ${label}`;
      correctionUnavailable.add(label);
      continue;
    }
    if (d.code !== DERIVED_SOLVE_INTEGRITY) continue;
    const malformed = `HANGER_SIZING_SOLVE_INTEGRITY_DISCLOSURE_MALFORMED: disclosures[${index}]`;
    const label = solveLabel(d.names);
    if (label === null) return malformed;
    if (stated.has(label)) return `HANGER_SIZING_SOLVE_INTEGRITY_DISCLOSURE_DUPLICATE: ${label}`;
    const message = d.message;
    if (typeof message !== "string") return malformed;
    const colon = message.indexOf(":");
    const token = colon < 0 ? "" : message.slice(0, colon);
    if (!(HANGER_SIZING_NONPASSING_STANDINGS as readonly string[]).includes(token) || message !== integrityDisclosureMessage(token)) return malformed;
    stated.set(label, token);
  }
  for (const solve of solves) {
    if (!wellFormedSolverStanding(solve.standing)) return `HANGER_SIZING_SOLVE_STANDING_MALFORMED: ${solve.label}`;
    if (!wellFormedSupportStateConfirmation(solve.confirmation, solve.stateless)) return `HANGER_SIZING_SOLVE_CONFIRMATION_MALFORMED: ${solve.label}`;
    const token = stated.get(solve.label);
    if (solve.standing === "solver_checks_passed" ? token !== undefined : token !== solve.standing) {
      return `HANGER_SIZING_SOLVE_STANDING_UNDISCLOSED: ${solve.label}`;
    }
  }
  for (const solve of solves) {
    const { outcome } = solve.confirmation as HangerSizingSupportStateConfirmation;
    if (outcome === "solver_reported_established" && correctionUnavailable.has(solve.label)) {
      return `HANGER_SIZING_SOLVE_CONFIRMATION_CONTRADICTION: ${solve.label}`;
    }
  }
  return null;
}

/** The first fault of a sized document's `result` in these members, or null. Anything that is not a result with
 * a well-formed bound, solves and disclosures is a fault. */
export function hangerSizingResultFault(result: unknown): string | null {
  if (!isPlainObject(result)) return "HANGER_SIZING_RESULT_MALFORMED";
  if (!wellFormedIntegrityBound(result.integrity_bound)) return "HANGER_SIZING_INTEGRITY_BOUND_MALFORMED";
  const solves = result.solves;
  const disclosures = result.disclosures;
  if (!Array.isArray(solves) || !Array.isArray(disclosures) || !disclosures.every(isPlainObject)) return "HANGER_SIZING_RESULT_MALFORMED";
  const facts: SolveFacts[] = [];
  for (const solve of solves) {
    if (!isPlainObject(solve) || typeof solve.label !== "string" || !isPlainObject(solve.states)) return "HANGER_SIZING_RESULT_MALFORMED";
    facts.push({ label: solve.label, stateless: Object.keys(solve.states).length === 0, standing: solve.solver_standing, confirmation: solve.support_state_confirmation });
  }
  return solvesFault(facts, disclosures as FindingFacts[]);
}

/** The first fault of a sized view in these members, or null: what a direct caller of the CSV builders hands them is
 * checked by the same rule as the document. */
export function hangerSizingViewFault(view: HangerSizingSizedView): string | null {
  if (!wellFormedIntegrityBound(view.integrityBound)) return "HANGER_SIZING_INTEGRITY_BOUND_MALFORMED";
  if (!Array.isArray(view.solves) || !Array.isArray(view.disclosures)) return "HANGER_SIZING_RESULT_MALFORMED";
  return solvesFault(
    view.solves.map((solve) => ({ label: solve.label, stateless: solve.states.length === 0, standing: solve.solverStanding, confirmation: solve.supportStateConfirmation })),
    view.disclosures
  );
}

/** The bound in words: `not_verified (inverse_norm_bound_not_verified)`. Throws for anything else. */
export function formatHangerSizingIntegrityBound(bound: unknown): string {
  if (!wellFormedIntegrityBound(bound)) throw new Error("HANGER_SIZING_INTEGRITY_BOUND_MALFORMED");
  return `${bound.status} (${bound.reason})`;
}

/** The reader verdict's bound beside its standings: `Integrity bound: … .` when the evidence is bound to this
 * document, null when it is not (the standings then read not checked). Throws for a malformed bound. */
export function formatHangerSizingStandingsBound(standings: Pick<StandingsView, "integrityBound">): string | null {
  return standings.integrityBound === null ? null : `Integrity bound: ${formatHangerSizingIntegrityBound(standings.integrityBound)}.`;
}

/** A solve's standing in words: `solver_checks_passed` is the solver's report, not a verified bound, and carries the
 * desktop's boundary sentence beside it wherever it is shown or exported; any other token is disclosed. Throws for
 * anything outside `HANGER_SIZING_SOLVER_STANDINGS`. */
export function formatHangerSizingSolverStanding(standing: unknown): string {
  if (!wellFormedSolverStanding(standing)) throw new Error("HANGER_SIZING_SOLVE_STANDING_MALFORMED");
  return standing === "solver_checks_passed"
    ? `solver_checks_passed: the solver's own checks found no problem; this is the solver's report, not a verified error bound. ${BOUNDARY_SENTENCE}`
    : `${standing}: not solver_checks_passed, disclosed as ${DERIVED_SOLVE_INTEGRITY}; every value taken from this solve carries this standing`;
}

/** A solve's support-state line and, for `not_established_detail_quoted`, the solver's quoted reason, as the
 * desktop states a case's confirmation elsewhere (`supportStatesLine`). Throws for a malformed confirmation. */
export function formatHangerSizingConfirmation(confirmation: unknown): SupportStatesLine {
  if (!wellFormedSupportStateConfirmation(confirmation)) throw new Error("HANGER_SIZING_SOLVE_CONFIRMATION_MALFORMED");
  if (confirmation.outcome === "not_applicable") return { text: HANGER_SIZING_CONFIRMATION_NOT_APPLICABLE, detail: null };
  const line = supportStatesLine(confirmation.outcome, confirmation.detail);
  if (line === null) throw new Error("HANGER_SIZING_SOLVE_CONFIRMATION_MALFORMED");
  return line;
}

// ---------------------------------------------------------------------------------------------
// Values

export type SiUnit = "N" | "m" | "N/m";
/** A published SI quantity: `value` is the document's number, unchanged; `text` is its shortest
 * round-trip decimal (`String`) with the unit. */
export type SiQuantity = Readonly<{ value: number; unit: SiUnit; text: string }>;
/** A published ratio (a variation or the variation limit): dimensionless. */
export type RatioValue = Readonly<{ value: number; unit: "ratio"; text: string }>;
export type VariationView =
  | Readonly<{ state: "ratio"; ratio: RatioValue }>
  | Readonly<{ state: "no_ratio_formed"; text: typeof NO_RATIO_FORMED }>;
export type LabelledQuantity = Readonly<{ label: string; quantity: SiQuantity }>;

const si = (value: number, unit: SiUnit): SiQuantity => ({ value, unit, text: `${String(value)} ${unit}` });
const ratio = (value: number): RatioValue => ({ value, unit: "ratio", text: String(value) });
const variation = (value: number | null): VariationView =>
  value === null ? { state: "no_ratio_formed", text: NO_RATIO_FORMED } : { state: "ratio", ratio: ratio(value) };
const pair = (values: readonly [number, number], unit: SiUnit): readonly [SiQuantity, SiQuantity] => [si(values[0], unit), si(values[1], unit)];

// ---------------------------------------------------------------------------------------------
// Basis block

export type SizingRequestView = Readonly<{
  weightCase: string;
  weightSupportState: "own_solve";
  designOperatingCase: string;
  checkOperatingCases: readonly string[];
  candidates: readonly Readonly<{ support: string; familyPolicy: HangerSizingFamilyPolicy }>[];
  method: Readonly<{
    loadDesign: "hot" | "cold";
    variationReference: "operating_load" | "design_load";
    variationLimit: RatioValue;
    travelAllowance: SiQuantity;
    selectionRule: "smallest_capacity" | "least_variation";
  }>;
  solverMode: "sparse_interactive" | "dense_scrutiny";
}>;

export type CaseView = Readonly<{
  case: string;
  role: "weight" | "operating";
  loadCategories: readonly string[];
  usesConditionSet: boolean;
}>;

export type BasisView = Readonly<{
  documentKind: string;
  schemaVersion: string;
  semanticContractId: string;
  route: string | null;
  canonicalProfile: string;
  /** Null exactly when a refused document publishes it as null. */
  sizingRequest: SizingRequestView | null;
  sizingRequestSha256: string | null;
  solveRequestSha256: string | null;
  library: Readonly<{ libraryId: string | null; sha256: string | null }>;
  /** The document's own strings, verbatim. */
  basisStatement: readonly string[];
  limitations: readonly string[];
}>;

const requestView = (request: HangerSizingRequest): SizingRequestView => ({
  weightCase: request.weight_case,
  weightSupportState: request.weight_support_state,
  designOperatingCase: request.design_operating_case,
  checkOperatingCases: [...request.check_operating_cases],
  candidates: request.candidates.map((candidate) => ({ support: candidate.support, familyPolicy: candidate.family_policy })),
  method: {
    loadDesign: request.method.load_design,
    variationReference: request.method.variation_reference,
    variationLimit: ratio(request.method.variation_limit),
    travelAllowance: si(request.method.travel_allowance_m, "m"),
    selectionRule: request.method.selection_rule
  },
  solverMode: request.solver_mode
});

const basisView = (document: HangerSizingDocument): BasisView => ({
  documentKind: document.document_kind,
  schemaVersion: document.schema_version,
  semanticContractId: document.semantic_contract_id,
  route: document.route,
  canonicalProfile: document.inputs.canonical_profile,
  sizingRequest: document.inputs.sizing_request === null ? null : requestView(document.inputs.sizing_request),
  sizingRequestSha256: document.inputs.sizing_request_sha256,
  solveRequestSha256: document.inputs.solve_request.sha256,
  library: { libraryId: document.inputs.library.library_id, sha256: document.inputs.library.sha256 },
  basisStatement: [...document.basis_statement],
  limitations: [...document.limitations]
});

// ---------------------------------------------------------------------------------------------
// Library detail gate

export type LibraryGateReason = "library_not_supplied" | "library_not_hashable" | "library_hash_mismatch" | "library_id_mismatch";
export type RecordNotShownReason = LibraryGateReason | "record_not_found" | "record_ambiguous" | "record_hash_mismatch" | "record_not_readable";

export type LibraryGateView =
  | Readonly<{ state: "bound"; libraryId: string; sha256: string }>
  | Readonly<{ state: "not_shown"; reason: LibraryGateReason }>;

/** A quantity as the library record states it: magnitude and unit unconverted. */
export type StatedQuantity = Readonly<{ magnitude: number; unit: string; text: string }>;

export type RecordDetailView =
  | Readonly<{
      state: "shown";
      hangerId: string;
      recordSha256: string;
      name: string;
      hangerType: string;
      /** The record's stated stiffness, for a variable selection only; null for a constant one. */
      stiffness: StatedQuantity | null;
      workingLoadRange: Readonly<{ minimum: StatedQuantity; maximum: StatedQuantity }>;
      travelRange: StatedQuantity;
    }>
  | Readonly<{ state: "not_shown"; hangerId: string; reason: RecordNotShownReason }>;

type BoundLibrary = { state: "bound"; libraryId: string; sha256: string; records: readonly unknown[] | null };
type LibraryGate = BoundLibrary | { state: "not_shown"; reason: LibraryGateReason };

const isPlainObject = (value: unknown): value is Record<string, unknown> =>
  typeof value === "object" && value !== null && !Array.isArray(value);

async function checkedHash(value: unknown): Promise<string | null> {
  try {
    return await canonicalSha256HexCheckedV1(value);
  } catch {
    return null;
  }
}

async function libraryGate(boundId: string, boundSha256: string, supplied: unknown): Promise<LibraryGate> {
  if (supplied === undefined || supplied === null) return { state: "not_shown", reason: "library_not_supplied" };
  // The exact full supplied value is read once, synchronously, as its checked JSON text; a value the checked profile
  // refuses is not hashable. The hash and every record detail shown come from that one private copy, so a change the
  // caller makes to the value while this gate awaits reaches neither.
  let library: unknown;
  try {
    library = JSON.parse(checkedJsonText(supplied));
  } catch {
    return { state: "not_shown", reason: "library_not_hashable" };
  }
  const sha256 = await checkedHash(library);
  if (sha256 === null) return { state: "not_shown", reason: "library_not_hashable" };
  if (sha256 !== boundSha256) return { state: "not_shown", reason: "library_hash_mismatch" };
  const header = isPlainObject(library) ? library.hanger_library : undefined;
  const libraryId = isPlainObject(header) ? header.library_id : undefined;
  if (libraryId !== boundId) return { state: "not_shown", reason: "library_id_mismatch" };
  const records = isPlainObject(library) && Array.isArray(library.hanger_records) ? library.hanger_records : null;
  return { state: "bound", libraryId: boundId, sha256, records };
}

const stated = (value: unknown): StatedQuantity | null =>
  isPlainObject(value) && typeof value.magnitude === "number" && typeof value.unit === "string"
    ? { magnitude: value.magnitude, unit: value.unit, text: `${String(value.magnitude)} ${value.unit}` }
    : null;

async function recordDetail(gate: LibraryGate, selection: RecordSelection): Promise<RecordDetailView> {
  const hangerId = selection.hanger_id;
  if (gate.state === "not_shown") return { state: "not_shown", hangerId, reason: gate.reason };
  const matches = (gate.records ?? []).filter((record) => isPlainObject(record) && record.hanger_id === hangerId);
  if (matches.length === 0) return { state: "not_shown", hangerId, reason: "record_not_found" };
  if (matches.length > 1) return { state: "not_shown", hangerId, reason: "record_ambiguous" };
  const record = matches[0] as Record<string, unknown>;
  if ((await checkedHash(record)) !== selection.record_sha256) return { state: "not_shown", hangerId, reason: "record_hash_mismatch" };
  const hanger: Record<string, unknown> = isPlainObject(record.hanger) ? record.hanger : {};
  const range: Record<string, unknown> = isPlainObject(hanger.working_load_range) ? hanger.working_load_range : {};
  const minimum = stated(range.minimum);
  const maximum = stated(range.maximum);
  const travelRange = stated(hanger.travel_range);
  const stiffness = selection.family === "variable_spring"
    ? stated(isPlainObject(hanger.stiffness) ? hanger.stiffness.value : undefined)
    : null;
  const name = record.name;
  const hangerType = hanger.hanger_type;
  // No partial display: every detail the row shows must be readable, or none is shown.
  if (typeof name !== "string" || typeof hangerType !== "string" || !minimum || !maximum || !travelRange
    || (selection.family === "variable_spring" && !stiffness)) {
    return { state: "not_shown", hangerId, reason: "record_not_readable" };
  }
  return {
    state: "shown",
    hangerId,
    recordSha256: selection.record_sha256,
    name,
    hangerType,
    stiffness,
    workingLoadRange: { minimum, maximum },
    travelRange
  };
}

// ---------------------------------------------------------------------------------------------
// Candidate rows

export type FailureCountsView =
  | Readonly<{ state: "not_tested" }>
  | Readonly<{ state: "tested"; counts: Readonly<Record<string, number>> }>;

export type SelectionView =
  | Readonly<{
      family: "variable_spring";
      hangerId: string;
      recordSha256: string;
      k: SiQuantity;
      fCold: LabelledQuantity;
      fColdCondition: LabelledQuantity;
      envelope: readonly [SiQuantity, SiQuantity];
      /** Per operating case, in the document's order. */
      operating: readonly Readonly<{ case: string; force: SiQuantity; variation: VariationView }>[];
    }>
  | Readonly<{ family: "constant_effort"; hangerId: string; recordSha256: string; load: SiQuantity }>;

export type VerificationAssumptionView =
  | Readonly<{ role: "spring"; k: SiQuantity; fCold: LabelledQuantity }>
  | Readonly<{ role: "force"; force: SiQuantity }>;

export type VerificationView =
  | Readonly<{
      family: "variable_spring";
      uCold: SiQuantity;
      operating: readonly Readonly<{ case: string; position: SiQuantity; force: SiQuantity; variation: VariationView }>[];
      span: readonly [SiQuantity, SiQuantity];
      requiredTravel: SiQuantity;
      tPass: boolean;
      fColdCondition: LabelledQuantity;
      departure: SiQuantity;
      failedTests: readonly string[];
    }>
  | Readonly<{
      family: "constant_effort";
      uCold: SiQuantity;
      operating: readonly Readonly<{ case: string; position: SiQuantity }>[];
      span: readonly [SiQuantity, SiQuantity];
      requiredTravel: SiQuantity;
      tPass: boolean;
      failedTests: readonly string[];
    }>;

export type CandidateRowView = Readonly<{
  support: string;
  node: string;
  dof: string;
  familyPolicy: HangerSizingFamilyPolicy;
  outcome: HangerSizingOutcome;
  w: SiQuantity;
  uCold: SiQuantity;
  /** The candidate's position per operating case, in the document's order. */
  operating: readonly Readonly<{ case: string; position: SiQuantity }>[];
  span: readonly [SiQuantity, SiQuantity];
  requiredTravel: SiQuantity;
  /** Null for no fit: the outcome is shown and nothing is put in its place. */
  selection: SelectionView | null;
  governing: Readonly<{ travelCase: string; variationCase: string | null }> | null;
  verificationAssumption: VerificationAssumptionView;
  verification: VerificationView | null;
  failureCounts: Readonly<{ variable: FailureCountsView; constant: FailureCountsView }>;
  /** Null for no fit (no record is selected); otherwise the gated record detail. */
  record: RecordDetailView | null;
}>;

const countsView = (counts: Readonly<Record<string, number>> | null): FailureCountsView =>
  counts === null ? { state: "not_tested" } : { state: "tested", counts: { ...counts } };

const fCold = (value: number): LabelledQuantity => ({ label: F_COLD_LABEL, quantity: si(value, "N") });
const fColdCondition = (value: number): LabelledQuantity => ({ label: F_COLD_CONDITION_LABEL, quantity: si(value, "N") });

function selectionView(candidate: HangerSizingCandidate): SelectionView | null {
  const selected = candidate.selected;
  if (selected === null) return null;
  if (selected.family === "constant_effort") {
    return { family: "constant_effort", hangerId: selected.hanger_id, recordSha256: selected.record_sha256, load: si(selected.load_n, "N") };
  }
  return {
    family: "variable_spring",
    hangerId: selected.hanger_id,
    recordSha256: selected.record_sha256,
    k: si(selected.k_n_per_m, "N/m"),
    fCold: fCold(selected.f_cold_n),
    fColdCondition: fColdCondition(selected.f_cold_condition_n),
    envelope: pair(selected.envelope_n, "N"),
    operating: selected.operating.map((entry) => ({ case: entry.case, force: si(entry.f_n, "N"), variation: variation(entry.variation) }))
  };
}

function verificationView(candidate: HangerSizingCandidate): VerificationView | null {
  const verification = candidate.verification;
  if (verification === null) return null;
  if (verification.family === "constant_effort") {
    return {
      family: "constant_effort",
      uCold: si(verification.u_cold_m, "m"),
      operating: verification.operating.map((entry) => ({ case: entry.case, position: si(entry.u_m, "m") })),
      span: pair(verification.span_m, "m"),
      requiredTravel: si(verification.required_travel_m, "m"),
      tPass: verification.t_pass,
      failedTests: [...verification.failed_tests]
    };
  }
  return {
    family: "variable_spring",
    uCold: si(verification.u_cold_m, "m"),
    operating: verification.operating.map((entry) => ({
      case: entry.case,
      position: si(entry.u_m, "m"),
      force: si(entry.f_n, "N"),
      variation: variation(entry.variation)
    })),
    span: pair(verification.span_m, "m"),
    requiredTravel: si(verification.required_travel_m, "m"),
    tPass: verification.t_pass,
    fColdCondition: fColdCondition(verification.f_cold_condition_n),
    departure: si(verification.departure_n, "N"),
    failedTests: [...verification.failed_tests]
  };
}

const assumptionView = (candidate: HangerSizingCandidate): VerificationAssumptionView => {
  const assumption = candidate.verification_assumption;
  return assumption.role === "spring"
    ? { role: "spring", k: si(assumption.k_n_per_m, "N/m"), fCold: fCold(assumption.f_cold_n) }
    : { role: "force", force: si(assumption.force_n, "N") };
};

type RecordSelection = { family: "variable_spring" | "constant_effort"; hanger_id: string; record_sha256: string };
type PendingRow = Readonly<{ row: Omit<CandidateRowView, "record">; selection: RecordSelection | null }>;

/** A candidate's row, read synchronously, and a copy of the selection its record detail is gated on. */
function candidateRow(candidate: HangerSizingCandidate): PendingRow {
  const selected = candidate.selected;
  const row = {
    support: candidate.support,
    node: candidate.node,
    dof: candidate.dof,
    familyPolicy: candidate.family_policy,
    outcome: candidate.outcome,
    w: si(candidate.w_n, "N"),
    uCold: si(candidate.u_cold_m, "m"),
    operating: candidate.operating.map((entry) => ({ case: entry.case, position: si(entry.u_m, "m") })),
    span: pair(candidate.span_m, "m"),
    requiredTravel: si(candidate.required_travel_m, "m"),
    selection: selectionView(candidate),
    governing: candidate.governing === null
      ? null
      : { travelCase: candidate.governing.travel_case, variationCase: candidate.governing.variation_case },
    verificationAssumption: assumptionView(candidate),
    verification: verificationView(candidate),
    failureCounts: { variable: countsView(candidate.failure_counts.variable), constant: countsView(candidate.failure_counts.constant) }
  };
  return {
    row,
    selection: selected === null ? null : { family: selected.family, hanger_id: selected.hanger_id, record_sha256: selected.record_sha256 }
  };
}

// ---------------------------------------------------------------------------------------------
// Reader standings, shown only as evidence bound to this exact document

export const HANGER_SIZING_STANDING_NAMES = [
  "closure",
  "binding",
  "arithmetic_consistent_with_published_positions",
  "derivation",
  "reproduction"
] as const;
export type HangerSizingStandingName = (typeof HANGER_SIZING_STANDING_NAMES)[number];
export type HangerSizingStandingStatus = "passed" | "failed" | "not_checked";
type ReaderStanding = { readonly status: HangerSizingStandingStatus; readonly difference: string | null };

/** The readers' verdict (the shape of the Rust reader's `Verdict::to_json`), with the checked
 * canonical sha256 (`openpipestress_jcs_ijson_v1`) of the document it was computed on. The hash is
 * of the canonical parsed value, so whitespace, member order and equivalent number spellings do
 * not distinguish documents. This binds evidence to a document; it does not authenticate the
 * evidence. The caller is responsible for supplying the reader's actual output, and an object
 * built to this shape with the right hash is displayed as supplied. */
export type HangerSizingReaderEvidence = {
  readonly document_sha256: string;
  readonly verdict: {
    readonly closure: ReaderStanding;
    readonly binding: ReaderStanding & { readonly not_checked: readonly string[] };
    readonly arithmetic_consistent_with_published_positions: ReaderStanding;
    readonly derivation: ReaderStanding;
    readonly reproduction: ReaderStanding;
    readonly integrity_bound: HangerSizingIntegrityBound;
  };
};

export type StandingsUnboundReason = "evidence_not_supplied" | "evidence_malformed" | "document_not_hashable" | "evidence_bound_to_other_document";

export type StandingView = Readonly<{
  name: HangerSizingStandingName;
  status: HangerSizingStandingStatus;
  difference: string | null;
  /** Binding only: the inputs the reader did not have. */
  notChecked?: readonly string[];
}>;

export type StandingsView = Readonly<{
  binding: Readonly<{ state: "bound"; documentSha256: string }> | Readonly<{ state: "unbound"; reason: StandingsUnboundReason }>;
  standings: readonly StandingView[];
  /** The verdict's `integrity_bound` when the evidence is bound to this document; otherwise null. */
  integrityBound: HangerSizingIntegrityBound | null;
}>;

const STANDING_STATUSES: ReadonlySet<string> = new Set(["passed", "failed", "not_checked"]);
const SHA256 = /^[0-9a-f]{64}$/;

const wellFormedStanding = (value: unknown): value is ReaderStanding =>
  isPlainObject(value)
  && typeof value.status === "string" && STANDING_STATUSES.has(value.status)
  && (value.difference === null || typeof value.difference === "string");

function wellFormedEvidence(value: unknown): value is HangerSizingReaderEvidence {
  if (!isPlainObject(value) || typeof value.document_sha256 !== "string" || !SHA256.test(value.document_sha256)) return false;
  const verdict = value.verdict;
  if (!isPlainObject(verdict) || !HANGER_SIZING_STANDING_NAMES.every((name) => wellFormedStanding(verdict[name]))) return false;
  const notChecked = (verdict.binding as Record<string, unknown>).not_checked;
  if (!Array.isArray(notChecked) || !notChecked.every((item) => typeof item === "string")) return false;
  return wellFormedIntegrityBound(verdict.integrity_bound);
}

const unbound = (reason: StandingsUnboundReason): StandingsView => ({
  binding: { state: "unbound", reason },
  standings: HANGER_SIZING_STANDING_NAMES.map((name) => ({ name, status: "not_checked" as const, difference: null })),
  integrityBound: null
});

/** The readers' standings bound to the document whose checked hash `documentHash` resolves to. The caller starts that
 * hash (`checkedHash`) synchronously, before its own first await, so the document's checked text is taken as it was
 * read; everything else this returns is read synchronously, before its one await: the evidence is checked and copied.
 * A change either value undergoes while the hash is computed reaches neither the binding nor what is shown. */
async function standingsView(documentHash: Promise<string | null>, evidence: unknown): Promise<StandingsView> {
  if (evidence === undefined || evidence === null) return unbound("evidence_not_supplied");
  if (!wellFormedEvidence(evidence)) return unbound("evidence_malformed");
  const boundSha256 = evidence.document_sha256;
  const standings: StandingView[] = HANGER_SIZING_STANDING_NAMES.map((name) => {
    const standing = evidence.verdict[name];
    return name === "binding"
      ? { name, status: standing.status, difference: standing.difference, notChecked: [...evidence.verdict.binding.not_checked] }
      : { name, status: standing.status, difference: standing.difference };
  });
  const integrityBound = { status: evidence.verdict.integrity_bound.status, reason: evidence.verdict.integrity_bound.reason };
  if (!wellFormedIntegrityBound(integrityBound) || !standings.every(wellFormedStanding)) return unbound("evidence_malformed");
  const documentSha256 = await documentHash;
  if (documentSha256 === null) return unbound("document_not_hashable");
  if (documentSha256 !== boundSha256) return unbound("evidence_bound_to_other_document");
  return { binding: { state: "bound", documentSha256 }, standings, integrityBound };
}

// ---------------------------------------------------------------------------------------------
// The view

export type SolveView = Readonly<{
  label: string;
  case: string;
  derivedDocumentSha256: string;
  /** Each nonlinear support's state, in the document's key order. */
  states: readonly Readonly<{ support: string; state: string }>[];
  /** The solver's standing of this solve (`formatHangerSizingSolverStanding`). */
  solverStanding: HangerSizingSolverStanding;
  /** What the solver's integrity record states of the support-state confirmation (`formatHangerSizingConfirmation`). */
  supportStateConfirmation: HangerSizingSupportStateConfirmation;
}>;

export type NamedFindingView = Readonly<{ code: string; names: readonly string[]; message: string }>;

export type HangerSizingSizedView = Readonly<{
  status: "sized";
  basis: BasisView;
  /** Each named case with its role, in the document's order. */
  cases: readonly CaseView[];
  verificationStanding: "unconditional" | "conditional";
  /** The document's `result.integrity_bound`: the ordinary derived solves' error bound is not verified. */
  integrityBound: HangerSizingIntegrityBound;
  library: LibraryGateView;
  rows: readonly CandidateRowView[];
  solves: readonly SolveView[];
  disclosures: readonly NamedFindingView[];
  readerStandings: StandingsView;
  /** The checked canonical SHA-256 of the document as it was read when the view was formed (before its first await),
   * or null when it cannot be hashed. The CSV exports record it as `result_hash`, so a later change to the document
   * reaches neither the view nor its exports. */
  documentSha256: string | null;
}>;

export type HangerSizingRefusedView = Readonly<{
  status: "refused";
  basis: BasisView;
  /** Every refusal, in the engine's order. */
  refusals: readonly NamedFindingView[];
  readerStandings: StandingsView;
}>;

export type HangerSizingView = HangerSizingSizedView | HangerSizingRefusedView;

const findingView = (finding: { code: string; names: readonly string[]; message: string }): NamedFindingView =>
  ({ code: finding.code, names: [...finding.names], message: finding.message });

async function sizedView(document: HangerSizingSizedDocument, evidence: unknown, library: unknown): Promise<HangerSizingSizedView> {
  // Fail closed: a bound, standing or confirmation outside the closed vocabulary, or a standing its disclosures do
  // not state, is never shown.
  const fault = hangerSizingResultFault(document.result);
  if (fault !== null) throw new Error(fault);
  // Everything the view shows is read here, synchronously, before the first await: the document's checked text is
  // taken for its hash, the validated members are copied, and the library gate and reader standings take their
  // checked text as they are called. A change the caller makes to the document, the evidence or the library while the
  // hashes are computed reaches none of it, nor the hash the view and its exports record.
  const documentHashPromise = checkedHash(document);
  const result = document.result;
  const basis = basisView(document);
  const cases: CaseView[] = result.cases.map((entry) => ({
    case: entry.case,
    role: entry.role,
    loadCategories: [...entry.load_categories],
    usesConditionSet: entry.uses_condition_set
  }));
  const verificationStanding = result.verification_standing;
  const integrityBound = { status: result.integrity_bound.status, reason: result.integrity_bound.reason };
  const pending = result.candidates.map(candidateRow);
  const solves: SolveView[] = result.solves.map((solve) => ({
    label: solve.label,
    case: solve.case,
    derivedDocumentSha256: solve.derived_document_sha256,
    states: Object.keys(solve.states).map((support) => ({ support, state: solve.states[support] })),
    solverStanding: solve.solver_standing,
    supportStateConfirmation: { ...solve.support_state_confirmation }
  }));
  const disclosures = result.disclosures.map(findingView);
  const gatePromise = libraryGate(document.inputs.library.library_id, document.inputs.library.sha256, library);
  const standingsPromise = standingsView(documentHashPromise, evidence);
  const gate = await gatePromise;
  const rows: CandidateRowView[] = [];
  for (const { row, selection } of pending) rows.push({ ...row, record: selection === null ? null : await recordDetail(gate, selection) });
  const view: HangerSizingSizedView = {
    status: "sized",
    basis,
    cases,
    verificationStanding,
    integrityBound,
    library: gate.state === "bound" ? { state: "bound", libraryId: gate.libraryId, sha256: gate.sha256 } : gate,
    rows,
    solves,
    disclosures,
    readerStandings: await standingsPromise,
    documentSha256: await documentHashPromise
  };
  // The copies are checked again by the rule the CSV builders apply, so a value read differently the second time is
  // never shown either.
  const copied = hangerSizingViewFault(view);
  if (copied !== null) throw new Error(copied);
  return view;
}

async function refusedView(document: HangerSizingRefusedDocument, evidence: unknown): Promise<HangerSizingRefusedView> {
  const documentHashPromise = checkedHash(document);
  return {
    status: "refused",
    basis: basisView(document),
    refusals: document.refusals.map(findingView),
    readerStandings: await standingsView(documentHashPromise, evidence)
  };
}

/** The display model of one hanger-sizing result document.
 *
 * `evidence` is the readers' verdict bound to the exact document by its checked hash; anything
 * else shows every standing as `not_checked`. `library` is the full library document the result
 * binds; record details are shown only when its checked hash equals `inputs.library.sha256`. */
export async function hangerSizingView(
  document: HangerSizingDocument,
  evidence?: HangerSizingReaderEvidence | null,
  library?: unknown
): Promise<HangerSizingView> {
  if (document.status === "sized") return sizedView(document, evidence, library);
  if (document.status === "refused") return refusedView(document, evidence);
  throw new Error("HANGER_SIZING_VIEW_STATUS_UNKNOWN");
}
