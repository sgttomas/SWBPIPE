import type { MechanicsResult } from "../../types";
import { PRESSURE_SOURCE_CONTRACT_ID } from "./numericalResultQuality";

type RecordValue = Record<string, unknown>;
const record = (v: unknown): RecordValue | null => v && typeof v === "object" && !Array.isArray(v) ? v as RecordValue : null;
const rows = (v: unknown): RecordValue[] => Array.isArray(v) ? v.map(record).filter((x): x is RecordValue => x !== null) : [];
const text = (v: unknown) => typeof v === "string" && v ? v : "unavailable";

/** The panel's own marker for a field it cannot display: a member the row always lists that the
 * record lacks, or a value of another kind than its field's. It is never quoted, so it cannot be
 * taken for a recorded text. */
const UNAVAILABLE_TO_DISPLAY = "Unavailable to display";
/** A recorded text, quoted as JSON text so that its extent is unmistakable: no text value can
 * read as a label, as another field's value, or as the panel's marker. */
const recordedText = (v: unknown) => typeof v === "string" ? JSON.stringify(v) : UNAVAILABLE_TO_DISPLAY;
/** A recorded number as the parsed number prints (a recorded 0.0 shows as 0). */
const recordedNumber = (v: unknown) => typeof v === "number" && Number.isFinite(v) ? String(v) : UNAVAILABLE_TO_DISPLAY;
const recordedFlag = (v: unknown) => typeof v === "boolean" ? String(v) : UNAVAILABLE_TO_DISPLAY;

/** Said once before a case's stress-maximum entries, and about those entries only. It says that no
 * check of their agreement ran in this view. */
const RECORDED_FIELDS_NOTICE = "These entries show selected fields from the stress-maximum record. This view does not run the whole-result reader to check whether those fields agree with each other or with the linked result.";

type Reader = (v: unknown) => string;
type Entry = readonly [label: string, value: string];
/** The two endpoints and the scope the record states for them: always listed, and rendered as
 * one element so that the endpoints are never shown without that scope. */
const INTERVAL_FIELDS: ReadonlyArray<readonly [string, string, Reader]> = [
  ["value_lower_pa", "Recorded lower endpoint (Pa)", recordedNumber],
  ["value_upper_pa", "Recorded upper endpoint (Pa)", recordedNumber],
  ["enclosure_scope", "Recorded scope", recordedText],
];
const IDENTITY_FIELDS: ReadonlyArray<readonly [string, string, Reader]> = [
  ["pipe_id", "Recorded pipe", recordedText],
  ["result_id", "Recorded result", recordedText],
];
/** The other members shown, each only when the record has it, in a fixed order. */
const OPTIONAL_FIELDS: ReadonlyArray<readonly [string, string, Reader]> = [
  ["basis", "Recorded basis", recordedText],
  ["coefficient_basis", "Recorded coefficient basis", recordedText],
  ["approximation", "Recorded approximation", recordedText],
  ["value_pa", "Recorded value (Pa)", recordedNumber],
  ["absolute_error_bound_pa", "Recorded absolute error bound (Pa)", recordedNumber],
  ["relative_error_bound", "Recorded relative error bound", recordedNumber],
  ["relative_limit", "Recorded relative limit", recordedNumber],
  ["station_fraction", "Recorded station fraction", recordedNumber],
];
/** The members of a location record that are shown, each under its own label and only when the
 * location record has it. The two flags keep their member names. */
const LOCATION_FIELDS: ReadonlyArray<readonly [string, string, Reader]> = [
  ["kind", "Recorded location kind", recordedText],
  ["endpoint", "Recorded location endpoint", recordedText],
  ["exact_tie_proven", "Recorded location exact_tie_proven", recordedFlag],
  ["interior_equal_possible", "Recorded location interior_equal_possible", recordedFlag],
];
const always = (source: RecordValue, fields: typeof INTERVAL_FIELDS): Entry[] => fields.map(([member, label, read]) => [label, read(source[member])]);
const present = (source: RecordValue, fields: typeof INTERVAL_FIELDS): Entry[] => fields.filter(([member]) => source[member] !== undefined).map(([member, label, read]) => [label, read(source[member])]);
/** The optional members of a stress-maximum record as label and value pairs. A location that is
 * not a record, or holds none of the members shown, is one entry with the panel's marker. */
function optionalEntries(maximum: RecordValue): Entry[] {
  const entries = present(maximum, OPTIONAL_FIELDS);
  if (maximum.locations !== undefined) {
    const locations = record(maximum.locations), members = locations ? present(locations, LOCATION_FIELDS) : [];
    entries.push(...(members.length ? members : [["Recorded location", UNAVAILABLE_TO_DISPLAY] as const]));
  }
  return entries;
}
/** Label and value pairs as a definition list: each label and each value is its own element. */
function Entries({ entries, testId }: { entries: Entry[]; testId: string }) {
  return <dl data-testid={testId}>{entries.map(([label, value]) => <div key={label}><dt>{label}</dt><dd>{value}</dd></div>)}</dl>;
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
          <strong>Elastic normal-stress maximum evidence entry</strong>
          <Entries entries={always(maximum, IDENTITY_FIELDS)} testId={`physics-source-maximum-identity-${text(maximum.result_id)}`} />
          <Entries entries={always(maximum, INTERVAL_FIELDS)} testId={`physics-source-maximum-interval-${text(maximum.result_id)}`} />
          <Entries entries={optionalEntries(maximum)} testId={`physics-source-maximum-fields-${text(maximum.result_id)}`} />
        </div>)}
      </article>;
    })}
  </section>;
}
