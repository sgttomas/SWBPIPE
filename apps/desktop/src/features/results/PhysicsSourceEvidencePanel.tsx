import type { MechanicsResult } from "../../types";
import { PRESSURE_SOURCE_CONTRACT_ID } from "./numericalResultQuality";

type RecordValue = Record<string, unknown>;
const record = (v: unknown): RecordValue | null => v && typeof v === "object" && !Array.isArray(v) ? v as RecordValue : null;
const rows = (v: unknown): RecordValue[] => Array.isArray(v) ? v.map(record).filter((x): x is RecordValue => x !== null) : [];
const text = (v: unknown) => typeof v === "string" && v ? v : "unavailable";
const number = (v: unknown) => typeof v === "number" && Number.isFinite(v) ? String(v) : "unavailable";

const flag = (v: unknown) => typeof v === "boolean" ? String(v) : "unavailable";

/** Said once before a case's stress-maximum records. The panel shows what each record holds and
 * asserts nothing of its own about it; it does not say that any check ran. */
export const RECORDED_FIELDS_NOTICE = "These are the record's fields as recorded. Whether they agree with each other and with the result is for the result's reader; this panel does not check it.";

/** The recorded interval of a stress-maximum record and the scope the record states for it, as
 * one sentence, so that the interval is never shown without that scope. Both are the record's own
 * fields, shown as recorded. */
export function recordedIntervalDescription(maximum: RecordValue): string {
  return `Recorded interval: [${number(maximum.value_lower_pa)}, ${number(maximum.value_upper_pa)}] Pa. Recorded scope: ${text(maximum.enclosure_scope)}.`;
}

/** The other fields a stress-maximum record may carry: member, label, how its value is read and
 * its unit, in a fixed order. A value of another kind than its field's is shown as unavailable, as
 * an interval endpoint is: text is not shown as a number, nor a number as text. */
const RECORDED_FIELDS: ReadonlyArray<readonly [string, string, (v: unknown) => string, string]> = [
  ["basis", "Recorded basis", text, ""],
  ["coefficient_basis", "Recorded coefficient basis", text, ""],
  ["approximation", "Recorded approximation", text, ""],
  ["value_pa", "Recorded value", number, " Pa"],
  ["absolute_error_bound_pa", "Recorded absolute error bound", number, " Pa"],
  ["relative_error_bound", "Recorded relative error bound", number, ""],
  ["relative_limit", "Recorded relative limit", number, ""],
  ["station_fraction", "Recorded station fraction", number, ""],
];
const RECORDED_LOCATION_FIELDS: ReadonlyArray<readonly [string, (v: unknown) => string]> = [
  ["kind", text], ["endpoint", text], ["exact_tie_proven", flag], ["interior_equal_possible", flag],
];

/** One sentence for each of those fields the record has, and one for its location record when it
 * has one, each naming the field and giving what is recorded. Nothing is inferred from them. */
export function recordedFieldLines(maximum: RecordValue): string[] {
  const lines: string[] = [];
  for (const [member, label, read, unit] of RECORDED_FIELDS) {
    if (maximum[member] === undefined) continue;
    const value = read(maximum[member]);
    lines.push(`${label}: ${value}${value === "unavailable" ? "" : unit}.`);
  }
  if (maximum.locations !== undefined) {
    const locations = record(maximum.locations);
    const fields = locations ? RECORDED_LOCATION_FIELDS.filter(([member]) => locations[member] !== undefined).map(([member, read]) => `${member} ${read(locations[member])}`) : [];
    lines.push(`Recorded location: ${fields.length ? fields.join(", ") : "unavailable"}.`);
  }
  return lines;
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
        {rows(exact?.pipe_stress_extrema).length > 0 ? <p data-testid={`physics-source-recorded-fields-notice-${id}`}>{RECORDED_FIELDS_NOTICE}</p> : null}
        {rows(exact?.pipe_stress_extrema).map((maximum, i) => <div key={`${text(maximum.result_id)}:${i}`} data-testid={`physics-source-maximum-${text(maximum.result_id)}`}>
          <strong>Recorded elastic normal-stress maximum</strong>
          <p>Recorded pipe: <code>{text(maximum.pipe_id)}</code>. Recorded result: <code>{text(maximum.result_id)}</code>.</p>
          <p><span data-testid={`physics-source-maximum-interval-${text(maximum.result_id)}`}>Recorded interval: [{number(maximum.value_lower_pa)}, {number(maximum.value_upper_pa)}] Pa. Recorded scope: <code>{text(maximum.enclosure_scope)}</code>.</span></p>
          <p data-testid={`physics-source-maximum-fields-${text(maximum.result_id)}`}>{recordedFieldLines(maximum).join(" ")}</p>
        </div>)}
      </article>;
    })}
  </section>;
}
