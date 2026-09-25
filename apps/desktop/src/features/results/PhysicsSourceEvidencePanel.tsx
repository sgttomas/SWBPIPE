import type { MechanicsResult } from "../../types";

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

/** Render original method facts in both live and historical/reference contexts.
 * The private invocation and method validators remain the eligibility owners. */
export function PhysicsSourceEvidencePanel({ result }: { result: MechanicsResult }) {
  const physical = record(result.contract_evidence), receipt = record(result.source_block_recovery), body = record(receipt?.body);
  const cases = rows(body?.cases), exactCases = rows(physical?.exact_cases);
  const composite = result.producer?.semantic_contract_id === "openpipestress.result_semantics/0.3.0/physics-source-1";
  const source = result.producer?.semantic_contract_id === "openpipestress.result_semantics/0.3.0/source-blocks-1";
  if (!composite && !source) return null;
  const receiptIds = cases.map(c => text(record(c.basis_ref)?.ref_id));
  const physicalIds = exactCases.map(c => text(c.load_case_id));
  const coverageMatches = !composite || (receiptIds.length === physicalIds.length && new Set(receiptIds).size === receiptIds.length && receiptIds.every(id => physicalIds.includes(id)));
  return <section aria-label="Recorded recovery evidence" data-testid="physics-source-evidence">
    <p>Physical profile: {text(result.formulation_basis?.profile_id)}. Ordinary numerical attempt: {text(result.numerical_quality?.status)}.</p>
    <p>Producer receipt: {text(body?.status)}. These recorded labels do not grant Current use; actual invocation and complete method evidence are checked separately.</p>
    <p data-testid="physics-source-case-coverage">Recorded cases: {cases.length}{composite ? `; physical cases: ${exactCases.length}` : ""}. {composite ? (coverageMatches ? "Receipt and physical case identities agree." : "Case evidence is incomplete or inconsistent.") : "Source method evidence is recorded per case."}</p>
    {cases.map((c, index) => {
      const id = text(record(c.basis_ref)?.ref_id), exact = exactCases.find(x => x.load_case_id === id);
      const ordinary = result.numerical_quality?.cases.find(x => x.basis_ref.ref_id === id);
      return <article key={`${id}:${index}`} data-testid={`physics-source-case-${id}`}>
        <h4>{id}</h4>
        <p>Ordinary case attempt: {text(ordinary?.solve_quality)}. Selected method: <code>{text(c.selected_method)}</code>. Producer outcome: {text(c.outcome)}.</p>
        {exact ? <p>Material selection: {text(exact.material_basis)}; {rows(exact.pipe_materials).length} member material records; {rows(exact.pipe_sections).length} section records.</p> : null}
        {rows(exact?.pipe_stress_extrema).map((maximum, i) => <div key={`${text(maximum.result_id)}:${i}`} data-testid={`physics-source-maximum-${text(maximum.result_id)}`}>
          <strong>{text(maximum.pipe_id)} — elastic normal-stress maximum</strong>
          <p>Method: <code>{text(maximum.basis ?? maximum.coefficient_basis)}</code>. Bound: [{number(maximum.value_lower_pa)}, {number(maximum.value_upper_pa)}] Pa.</p>
          {maximum.basis === "retained_source_endpoint_normal_max_v1" ? <>
            <p>Published value: {number(maximum.value_pa)} Pa; absolute error bound: {number(maximum.absolute_error_bound_pa)} Pa; relative error bound: {number(maximum.relative_error_bound)}; relative limit: {number(maximum.relative_limit)}.</p>
            <p>{endpointLocationDescription(maximum.locations)}</p>
            <p>The bound covers retained-action projection and ordered arithmetic with the represented section operands. It does not certify material or primitive geometry accuracy.</p>
          </> : <p>Ordinary physical enclosure; recorded station fraction: {number(maximum.station_fraction)}. A station witness alone does not establish uniqueness.</p>}
        </div>)}
      </article>;
    })}
  </section>;
}
