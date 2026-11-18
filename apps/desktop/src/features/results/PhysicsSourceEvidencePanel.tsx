import type { MechanicsResult } from "../../types";
import { PRESSURE_SOURCE_CONTRACT_ID } from "./numericalResultQuality";

type RecordValue = Record<string, unknown>;
const record = (v: unknown): RecordValue | null => v && typeof v === "object" && !Array.isArray(v) ? v as RecordValue : null;
const rows = (v: unknown): RecordValue[] => Array.isArray(v) ? v.map(record).filter((x): x is RecordValue => x !== null) : [];
const text = (v: unknown) => typeof v === "string" && v ? v : "unavailable";
const number = (v: unknown) => typeof v === "number" && Number.isFinite(v) ? String(v) : "unavailable";

/** Presentation of a recorded location claim only; this is no eligibility test. */
export function endpointLocationDescription(value: unknown): string {
  const locations = record(value);
  if (locations?.kind === "strict_endpoint" && (locations.endpoint === "i" || locations.endpoint === "j")) {
    return `Strict endpoint ${locations.endpoint}; interior stations are below this maximum. This does not identify a unique circumferential fibre.`;
  }
  if (locations?.kind === "whole_span_constant") return "Whole span constant; every station attains the same maximum.";
  if (locations?.kind === "endpoint_candidates" && typeof locations.exact_tie_proven === "boolean" && typeof locations.interior_equal_possible === "boolean") {
    return `${locations.exact_tie_proven ? "Both endpoints tie by the recorded source evidence." : "Both endpoints remain candidates; equality is not proved."} ${locations.interior_equal_possible ? "Interior equality remains possible." : "Interior equality is excluded by the recorded bounds."}`;
  }
  return "Location evidence unavailable; no unique governing station is inferred.";
}

/** Every member the schema fixes to a constant for each of the two kinds of stress-maximum record
 * (`schemas/physics_source_recovery.schema.json`, the two variants of `pipe_stress_extrema`). These
 * name the method that formed the record and the scope of its interval. The panel test reads the
 * schema and fails when these differ from it. */
export const POLYNOMIAL_ENCLOSURE_FIXED: Readonly<Record<string, unknown>> = {
  approximation: "piecewise_quadratic_straight_section_statics",
  coefficient_basis: "j_side_section_equilibrium_binary64_enclosures",
  enclosure_scope: "supplied_binary64_polynomial_coefficient_enclosures; solution error is separate",
};
export const RETAINED_ENDPOINT_FIXED: Readonly<Record<string, unknown>> = {
  basis: "retained_source_endpoint_normal_max_v1",
  coefficient_basis: "retained_section_functionals_binary64",
  relative_limit: 1e-9,
  enclosure_scope: "retained_action_projection_and_ordered_binary64_recipe;represented_source_section_operands",
};

const finite = (v: unknown): v is number => typeof v === "number" && Number.isFinite(v);
/** The record states every member fixed for its kind, exactly, and none that only the other kind fixes. */
const states = (maximum: RecordValue, kind: Readonly<RecordValue>, other: Readonly<RecordValue>) =>
  Object.entries(kind).every(([key, value]) => maximum[key] === value)
  && Object.keys(other).every(key => key in kind || maximum[key] === undefined);

/** Which explanation of its interval a stress-maximum record supports. The panel's sentences rely
 * on these conditions, and each is checked here before a sentence is shown:
 * - the interval is two finite numbers, not negative, the lower not above the upper;
 * - the record states every schema-fixed member of one kind and none fixed only for the other;
 * - for a retained endpoint record, whose published value and error bounds are shown beside the
 *   bound: the published value is a finite number inside the interval, the error bounds are finite
 *   and not negative, and the relative error bound is not above the method's limit.
 * A record that fails any of them supports neither explanation, and its interval is shown as
 * recorded, not as a bound. Presentation only: this explains the scope a record declares and
 * re-establishes none of the engine's arithmetic; it is no validity or eligibility test. */
export function maximumIntervalReading(maximum: RecordValue): "retained_endpoint" | "polynomial_enclosure" | "uninterpreted" {
  const { value_lower_pa: lower, value_upper_pa: upper } = maximum;
  if (!finite(lower) || !finite(upper) || lower < 0 || lower > upper) return "uninterpreted";
  if (states(maximum, POLYNOMIAL_ENCLOSURE_FIXED, RETAINED_ENDPOINT_FIXED)) return "polynomial_enclosure";
  if (!states(maximum, RETAINED_ENDPOINT_FIXED, POLYNOMIAL_ENCLOSURE_FIXED)) return "uninterpreted";
  const { value_pa: value, absolute_error_bound_pa: absolute, relative_error_bound: relative } = maximum;
  const figuresAgree = finite(value) && lower <= value && value <= upper && finite(absolute) && absolute >= 0
    && finite(relative) && relative >= 0 && relative <= (RETAINED_ENDPOINT_FIXED.relative_limit as number);
  return figuresAgree ? "retained_endpoint" : "uninterpreted";
}

/** The interval of a record that is not shown as a retained endpoint maximum, with what the record
 * says it covers, as one sentence, so that the interval is never shown without its qualification.
 * The words are the record's own scope; the panel adds no account of how the coefficients were
 * formed. */
export function qualifiedIntervalDescription(maximum: RecordValue): string {
  const interval = `[${number(maximum.value_lower_pa)}, ${number(maximum.value_upper_pa)}] Pa`;
  if (maximumIntervalReading(maximum) === "polynomial_enclosure") return `Bound over the supplied binary64 polynomial coefficient enclosures: ${interval}. Solution error is separate.`;
  return `Recorded interval: ${interval}. The record's fields are not consistent with a method this panel explains, so the interval is not shown as a bound.`;
}

/** Render original method facts in both live and historical/reference contexts, for
 * the retained-source composite pressure-source-1 (B7b: the v3 twin of the retired
 * physics-source-1). The private invocation and method validators remain the
 * eligibility owners. */
export function PhysicsSourceEvidencePanel({ result }: { result: MechanicsResult }) {
  const physical = record(result.contract_evidence), receipt = record(result.source_block_recovery), body = record(receipt?.body);
  const cases = rows(body?.cases), exactCases = rows(physical?.exact_cases);
  const composite = result.producer?.semantic_contract_id === PRESSURE_SOURCE_CONTRACT_ID;
  if (!composite) return null;
  const receiptIds = cases.map(c => text(record(c.basis_ref)?.ref_id));
  const physicalIds = exactCases.map(c => text(c.load_case_id));
  const coverageMatches = (receiptIds.length === physicalIds.length && new Set(receiptIds).size === receiptIds.length && receiptIds.every(id => physicalIds.includes(id)));
  return <section aria-label="Recorded recovery evidence" data-testid="physics-source-evidence">
    <p>Physical profile: {text(result.formulation_basis?.profile_id)}. Ordinary numerical attempt: {text(result.numerical_quality?.status)}.</p>
    <p>Producer receipt: {text(body?.status)}. These recorded labels do not grant Current use; actual invocation and complete method evidence are checked separately.</p>
    <p data-testid="physics-source-case-coverage">Recorded cases: {cases.length}; physical cases: {exactCases.length}. {coverageMatches ? "Receipt and physical case identities agree." : "Case evidence is incomplete or inconsistent."}</p>
    {cases.map((c, index) => {
      const id = text(record(c.basis_ref)?.ref_id), exact = exactCases.find(x => x.load_case_id === id);
      const ordinary = result.numerical_quality?.cases.find(x => x.basis_ref.ref_id === id);
      return <article key={`${id}:${index}`} data-testid={`physics-source-case-${id}`}>
        <h4>{id}</h4>
        <p>Ordinary case attempt: {text(ordinary?.solve_quality)}. Selected method: <code>{text(c.selected_method)}</code>. Producer outcome: {text(c.outcome)}.</p>
        {exact ? <p>Material selection: {text(exact.material_basis)}; {rows(exact.pipe_materials).length} member material records; {rows(exact.pipe_sections).length} section records.</p> : null}
        {rows(exact?.pipe_stress_extrema).map((maximum, i) => { const reading = maximumIntervalReading(maximum); return <div key={`${text(maximum.result_id)}:${i}`} data-testid={`physics-source-maximum-${text(maximum.result_id)}`}>
          <strong>{text(maximum.pipe_id)} — elastic normal-stress maximum</strong>
          {reading === "retained_endpoint"
            ? <p>Method: <code>{text(maximum.basis)}</code>. Bound: [{number(maximum.value_lower_pa)}, {number(maximum.value_upper_pa)}] Pa.</p>
            : <p>{reading === "polynomial_enclosure" ? "Method" : "Recorded method"}: <code>{text(maximum.basis ?? maximum.coefficient_basis)}</code>. <span data-testid={`physics-source-maximum-interval-${text(maximum.result_id)}`}>{qualifiedIntervalDescription(maximum)}</span></p>}
          {reading === "retained_endpoint" ? <>
            <p>Published value: {number(maximum.value_pa)} Pa; absolute error bound: {number(maximum.absolute_error_bound_pa)} Pa; relative error bound: {number(maximum.relative_error_bound)}; relative limit: {number(maximum.relative_limit)}.</p>
            <p>{endpointLocationDescription(maximum.locations)}</p>
            <p>The bound covers retained-action projection and ordered arithmetic with the represented section operands. It does not certify material or primitive geometry accuracy.</p>
          </> : reading === "polynomial_enclosure" ? <p>Recorded station fraction: {number(maximum.station_fraction)}. A station witness alone does not establish uniqueness.</p> : null}
        </div>; })}
      </article>;
    })}
  </section>;
}
