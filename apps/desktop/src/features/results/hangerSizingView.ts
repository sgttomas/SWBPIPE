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
 * - Shows the readers' standings (§8.3.7) only from supplied evidence that carries the checked
 *   hash of the exact document it was computed on, recomputed here and compared. Absent,
 *   malformed or differently bound evidence shows every standing as `not_checked`.
 *
 * What it does not do: validate the document, mint Current or acceptance, state that typed
 * results are trusted, compute anything per record, or fall back to a native or `evaluate`
 * path. It never mutates its inputs. */

import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import type {
  HangerSizingCandidate,
  HangerSizingDocument,
  HangerSizingFamilyPolicy,
  HangerSizingOutcome,
  HangerSizingRefusedDocument,
  HangerSizingRequest,
  HangerSizingSizedDocument
} from "./hangerSizingResult";

// ---------------------------------------------------------------------------------------------
// Labels and fixed display text

/** Exact labels of Design §8.1: F_cold and F_C are never read as one where u_C is not zero. */
export const F_COLD_LABEL = "installed setting (at drawn position)";
export const F_COLD_CONDITION_LABEL = "cold-condition load";
/** A variation the document publishes as null. */
export const NO_RATIO_FORMED = "no ratio formed";

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
  route: string;
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

async function libraryGate(boundId: string, boundSha256: string, library: unknown): Promise<LibraryGate> {
  if (library === undefined || library === null) return { state: "not_shown", reason: "library_not_supplied" };
  // The exact full supplied value is hashed; a value the checked profile refuses is not hashable.
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

async function recordDetail(
  gate: LibraryGate,
  selection: { family: "variable_spring" | "constant_effort"; hanger_id: string; record_sha256: string }
): Promise<RecordDetailView> {
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

async function candidateRow(candidate: HangerSizingCandidate, gate: LibraryGate): Promise<CandidateRowView> {
  return {
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
    failureCounts: { variable: countsView(candidate.failure_counts.variable), constant: countsView(candidate.failure_counts.constant) },
    record: candidate.selected === null ? null : await recordDetail(gate, candidate.selected)
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
  return Array.isArray(notChecked) && notChecked.every((item) => typeof item === "string");
}

const unbound = (reason: StandingsUnboundReason): StandingsView => ({
  binding: { state: "unbound", reason },
  standings: HANGER_SIZING_STANDING_NAMES.map((name) => ({ name, status: "not_checked" as const, difference: null }))
});

async function standingsView(document: HangerSizingDocument, evidence: unknown): Promise<StandingsView> {
  if (evidence === undefined || evidence === null) return unbound("evidence_not_supplied");
  if (!wellFormedEvidence(evidence)) return unbound("evidence_malformed");
  const documentSha256 = await checkedHash(document);
  if (documentSha256 === null) return unbound("document_not_hashable");
  if (documentSha256 !== evidence.document_sha256) return unbound("evidence_bound_to_other_document");
  return {
    binding: { state: "bound", documentSha256 },
    standings: HANGER_SIZING_STANDING_NAMES.map((name) => {
      const standing = evidence.verdict[name];
      return name === "binding"
        ? { name, status: standing.status, difference: standing.difference, notChecked: [...evidence.verdict.binding.not_checked] }
        : { name, status: standing.status, difference: standing.difference };
    })
  };
}

// ---------------------------------------------------------------------------------------------
// The view

export type SolveView = Readonly<{
  label: string;
  case: string;
  derivedDocumentSha256: string;
  /** Each nonlinear support's state, in the document's key order. */
  states: readonly Readonly<{ support: string; state: string }>[];
}>;

export type NamedFindingView = Readonly<{ code: string; names: readonly string[]; message: string }>;

export type HangerSizingSizedView = Readonly<{
  status: "sized";
  basis: BasisView;
  /** Each named case with its role, in the document's order. */
  cases: readonly CaseView[];
  verificationStanding: "unconditional" | "conditional";
  library: LibraryGateView;
  rows: readonly CandidateRowView[];
  solves: readonly SolveView[];
  disclosures: readonly NamedFindingView[];
  readerStandings: StandingsView;
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
  const gate = await libraryGate(document.inputs.library.library_id, document.inputs.library.sha256, library);
  const rows: CandidateRowView[] = [];
  for (const candidate of document.result.candidates) rows.push(await candidateRow(candidate, gate));
  return {
    status: "sized",
    basis: basisView(document),
    cases: document.result.cases.map((entry) => ({
      case: entry.case,
      role: entry.role,
      loadCategories: [...entry.load_categories],
      usesConditionSet: entry.uses_condition_set
    })),
    verificationStanding: document.result.verification_standing,
    library: gate.state === "bound" ? { state: "bound", libraryId: gate.libraryId, sha256: gate.sha256 } : gate,
    rows,
    solves: document.result.solves.map((solve) => ({
      label: solve.label,
      case: solve.case,
      derivedDocumentSha256: solve.derived_document_sha256,
      states: Object.keys(solve.states).map((support) => ({ support, state: solve.states[support] }))
    })),
    disclosures: document.result.disclosures.map(findingView),
    readerStandings: await standingsView(document, evidence)
  };
}

async function refusedView(document: HangerSizingRefusedDocument, evidence: unknown): Promise<HangerSizingRefusedView> {
  return {
    status: "refused",
    basis: basisView(document),
    refusals: document.refusals.map(findingView),
    readerStandings: await standingsView(document, evidence)
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
