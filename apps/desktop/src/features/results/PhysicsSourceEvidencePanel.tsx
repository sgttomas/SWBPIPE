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
  if (locations?.kind === "strict_endpoint" && ["i", "j"].includes(String(locations.endpoint))) {
    return `Strict endpoint ${locations.endpoint}; interior stations are below this maximum. This does not identify a unique circumferential fibre.`;
  }
  if (locations?.kind === "whole_span_constant") return "Whole span constant; every station attains the same maximum.";
  if (locations?.kind === "endpoint_candidates" && typeof locations.exact_tie_proven === "boolean" && typeof locations.interior_equal_possible === "boolean") {
    return `${locations.exact_tie_proven ? "Both endpoints tie by the recorded source evidence." : "Both endpoints remain candidates; equality is not proved."} ${locations.interior_equal_possible ? "Interior equality remains possible." : "Interior equality is excluded by the recorded bounds."}`;
  }
  return "Location evidence unavailable; no unique governing station is inferred.";
}

/** The scope an ordinary stress-maximum record states for its own interval, and the retained
 * endpoint method's basis and scope: the schemas' constants. */
const POLYNOMIAL_ENCLOSURE_SCOPE = "supplied_binary64_polynomial_coefficient_enclosures; solution error is separate";
const RETAINED_ENDPOINT_BASIS = "retained_source_endpoint_normal_max_v1";
const RETAINED_ENDPOINT_SCOPE = "retained_action_projection_and_ordered_binary64_recipe;represented_source_section_operands";

/** Which explanation of its interval a stress-maximum record supports: the one whose scope it
 * states exactly. A record whose basis and scope disagree, or whose scope the panel does not
 * know, supports neither. Presentation only; this is no validity or eligibility test. */
export function maximumIntervalReading(maximum: Record<string, unknown>): "retained_endpoint" | "polynomial_enclosure" | "uninterpreted" {
  const retained = maximum.basis === RETAINED_ENDPOINT_BASIS;
  if (retained && maximum.enclosure_scope === RETAINED_ENDPOINT_SCOPE) return "retained_endpoint";
  if (!retained && maximum.enclosure_scope === POLYNOMIAL_ENCLOSURE_SCOPE) return "polynomial_enclosure";
  return "uninterpreted";
}

/** The interval of a record that is not a retained endpoint maximum, with what the record says
 * it covers, as one sentence, so that the interval is never shown without its qualification.
 * The words are the record's own scope; the panel adds no account of how the coefficients
 * were formed. */
export function qualifiedIntervalDescription(maximum: Record<string, unknown>): string {
  const interval = `[${number(maximum.value_lower_pa)}, ${number(maximum.value_upper_pa)}] Pa`;
  if (maximumIntervalReading(maximum) === "polynomial_enclosure") return `Bound over the supplied binary64 polynomial coefficient enclosures: ${interval}. Solution error is separate.`;
  return `Recorded interval: ${interval}. This panel does not interpret the scope this record gives for the interval, so it is not shown as a bound.`;
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
            : <p>Method: <code>{text(maximum.basis ?? maximum.coefficient_basis)}</code>. <span data-testid={`physics-source-maximum-interval-${text(maximum.result_id)}`}>{qualifiedIntervalDescription(maximum)}</span></p>}
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
