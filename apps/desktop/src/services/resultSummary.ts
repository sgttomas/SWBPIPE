/** C1 native custody, slice 2 (DEL-04-05 `c1-native-custody-plan.md`): the reader of S, the
 * summary `read_preview_mechanics_job_summary` builds natively from a retained result
 * (`src-tauri/src/result_summary.rs`, `result_planes.rs`). No panel reads it yet: the webview
 * switches in slice 4, and until then the TypeScript builders stay the oracle of S's tables
 * (`resultSummaryParity.test.ts`).
 *
 * The response is one buffer: the magic `SWBSUMv1`, the header's length (u64, little-endian),
 * the header (S's JSON), zero padding to 8, then little-endian f64 planes at the offsets and
 * counts the header's descriptors state, with nothing after the last. `decodeResultSummary`
 * refuses anything else, and any plane whose codes, ordinals or offsets are not the header's,
 * with `RESULT-SUMMARY-MALFORMED`. Plane codes index S's value table `values` (-1: absent);
 * ordinals index the envelope's `results`, the exact row a pick opens (`resolveId`). */
import type { DisplacementExtremes } from "../features/results/displacementExtremes";
import type { StressRatioRow, StressRatioTable } from "../features/results/stressRatioTable";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

export const RESULT_SUMMARY_MALFORMED = "RESULT-SUMMARY-MALFORMED";
export const RESULT_SUMMARY_DOCUMENT_KIND = "swbpipe.desktop.result_summary";
export const RESULT_SUMMARY_VERSION = 1;
const MAGIC = "SWBSUMv1";

export const DISPLAY_COLUMNS = Object.freeze([
  "display.ordinal", "display.value", "display.kind", "display.entity_ref", "display.basis_ref", "display.unit",
  "display.coordinate_system", "display.component", "display.location", "display.metadata_basis", "display.sign_convention", "display.dimension"
] as const);
export const SOURCE_OFFSETS = "display.source_offsets";
export const SOURCE_REFS = "display.source_refs";
export const STRESS_COLUMNS = Object.freeze([
  "stress.ordinal", "stress.value", "stress.case", "stress.case_label", "stress.entity_ref", "stress.station", "stress.component",
  "stress.section_basis", "stress.unit"
] as const);
const DISPLAY_CODES = DISPLAY_COLUMNS.slice(2);
const STRESS_CODES = ["stress.case_label", "stress.entity_ref", "stress.station", "stress.component", "stress.section_basis", "stress.unit"] as const;
const PLANE_NAMES: readonly string[] = [...DISPLAY_COLUMNS, SOURCE_OFFSETS, SOURCE_REFS, ...STRESS_COLUMNS];

export class ResultSummaryMalformed extends Error {
  constructor(reason: string) {
    super(`${RESULT_SUMMARY_MALFORMED}: ${reason}`);
    this.name = "ResultSummaryMalformed";
  }
}
function fail(reason: string): never {
  throw new ResultSummaryMalformed(reason);
}

export type ResultSummaryHeader = Readonly<{
  document_kind: string;
  version: number;
  job_id: string;
  envelope: Readonly<Record<string, Json>>;
  omitted_members: readonly string[];
  combination_evidence_counts: readonly Json[];
  row_count: number;
  units: readonly Json[];
  numerical_health: Json;
  standing: Readonly<{ eligible: boolean; health_view: boolean }> | null;
  tables: Readonly<Record<string, Json>>;
  display: Readonly<{ rows: number; source_refs: number }>;
  stress: Readonly<{ rows: number }>;
  values: readonly Json[];
  planes: readonly Readonly<{ name: string; offset: number; count: number }>[];
  planes_bytes: number;
}>;
export type ResultSummary = Readonly<{ header: ResultSummaryHeader; planes: ReadonlyMap<string, Float64Array> }>;

const count = (value: unknown, what: string): number => (Number.isSafeInteger(value) && (value as number) >= 0 ? value as number : fail(`${what} is not a count`));

/** Reads a summary response, checking its layout and its planes against its header. */
export function decodeResultSummary(input: ArrayBuffer | Uint8Array): ResultSummary {
  const bytes = input instanceof Uint8Array ? input : new Uint8Array(input);
  if (bytes.length < 16 || String.fromCharCode(...bytes.subarray(0, 8)) !== MAGIC) fail("not a result summary");
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const headerLength = view.getBigUint64(8, true);
  if (headerLength > BigInt(bytes.length - 16)) fail("the header runs past the end");
  const head = 16 + Number(headerLength);
  let header: Json;
  try {
    header = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(16, head)));
  } catch (error) {
    fail(`the header is not JSON: ${(error as Error).message}`);
  }
  if (header === null || typeof header !== "object" || Array.isArray(header)) fail("the header is not an object");
  const start = head + ((8 - (head % 8)) % 8);
  if (start > bytes.length) fail("the padding runs past the end");
  if (bytes.subarray(head, start).some((byte) => byte !== 0)) fail("the padding is not zero");
  const total = count(header.planes_bytes, "planes_bytes");
  if (bytes.length - start !== total) fail("the planes are not the response's remaining bytes");
  if (!Array.isArray(header.planes)) fail("no plane descriptors");
  if (header.planes.length !== PLANE_NAMES.length || header.planes.some((descriptor: Json, i: number) => descriptor?.name !== PLANE_NAMES[i])) {
    fail("plane descriptors do not match the summary schema");
  }
  const planes = new Map<string, Float64Array>();
  let expected = 0;
  for (const descriptor of header.planes as Json[]) {
    const name = typeof descriptor?.name === "string" ? descriptor.name as string : fail("a plane without a name");
    const offset = count(descriptor.offset, `plane ${name}'s offset`);
    const length = count(descriptor.count, `plane ${name}'s count`);
    if (offset !== expected || offset % 8 !== 0) fail(`plane ${name} is not where the previous one ends`);
    const end = offset + length * 8;
    if (end > total) fail(`plane ${name} runs past the planes`);
    if (planes.has(name)) fail(`plane ${name} is stated twice`);
    const values = new Float64Array(length);
    for (let i = 0; i < length; i += 1) values[i] = view.getFloat64(start + offset + i * 8, true);
    planes.set(name, values);
    expected = end;
  }
  if (expected !== total) fail("bytes follow the last plane");
  checkSummary(header as ResultSummaryHeader, planes);
  return Object.freeze({ header: header as ResultSummaryHeader, planes });
}

function plane(planes: ReadonlyMap<string, Float64Array>, name: string, length: number): Float64Array {
  const values = planes.get(name) ?? fail(`no plane ${name}`);
  if (values.length !== length) fail(`plane ${name} holds ${values.length} values, not ${length}`);
  return values;
}

/** Every plane the header implies, of its length; codes in the value table; ordinals rows of the
 * envelope (the display rows ascending); source offsets ascending to the source references; stress
 * rows in their groups, each group ascending, with a known case. */
function checkSummary(header: ResultSummaryHeader, planes: ReadonlyMap<string, Float64Array>): void {
  if (header.document_kind !== RESULT_SUMMARY_DOCUMENT_KIND || header.version !== RESULT_SUMMARY_VERSION) fail("another document kind or version");
  const rows = count(header.row_count, "row_count");
  const values = Array.isArray(header.values) ? header.values : fail("no value table");
  const display = count(header.display?.rows, "display.rows");
  const sources = count(header.display?.source_refs, "display.source_refs");
  const stress = count(header.stress?.rows, "stress.rows");
  const integer = (x: number, low: number, high: number, what: string) => {
    if (!Number.isInteger(x) || x < low || x >= high) fail(`${what} ${x} is outside ${low}..${high}`);
  };
  const ordinals = plane(planes, "display.ordinal", display);
  ordinals.forEach((x, i) => {
    integer(x, 0, rows, "display ordinal");
    if (i > 0 && !(x > ordinals[i - 1])) fail("display ordinals do not ascend");
  });
  plane(planes, "display.value", display);
  for (const name of DISPLAY_CODES) plane(planes, name, display).forEach((x) => integer(x, -1, values.length, `${name} code`));
  const offsets = plane(planes, SOURCE_OFFSETS, display + 1);
  offsets.forEach((x, i) => {
    integer(x, 0, sources + 1, "source offset");
    if (i > 0 && x < offsets[i - 1]) fail("source offsets descend");
  });
  if (offsets[0] !== 0 || offsets[display] !== sources) fail("source offsets do not span the source references");
  plane(planes, SOURCE_REFS, sources).forEach((x) => integer(x, -1, rows, "source reference"));
  const cases = Array.isArray(header.tables?.support_load_cases) ? header.tables.support_load_cases.length : fail("no cases");
  const stressOrdinals = plane(planes, "stress.ordinal", stress);
  stressOrdinals.forEach((x) => integer(x, 0, rows, "stress ordinal"));
  plane(planes, "stress.value", stress);
  plane(planes, "stress.case", stress).forEach((x) => integer(x, 0, cases, "stress case"));
  for (const name of STRESS_CODES) plane(planes, name, stress).forEach((x) => integer(x, -1, values.length, `${name} code`));
  let covered = 0;
  for (const group of (header.tables?.stress_ratio?.groups ?? []) as Json[]) {
    const first = count(group?.rows?.first, "a group's first row");
    const length = count(group?.rows?.count, "a group's row count");
    if (first !== covered) fail("stress groups are not contiguous");
    for (let i = first + 1; i < first + length; i += 1) if (!(stressOrdinals[i] > stressOrdinals[i - 1])) fail("a group's rows do not ascend");
    covered += length;
  }
  if (covered !== stress) fail("stress groups do not cover the stress rows");
}

/** A code's value (`undefined` for -1). */
export function summaryValue(summary: ResultSummary, code: number): Json {
  return code < 0 ? undefined : summary.header.values[code];
}

/** A group's rows, as `stressGroups` builds them; `resolveId` gives a row's id from its ordinal. */
export function stressRatioRows(summary: ResultSummary, group: Json, resolveId: (ordinal: number) => Json): StressRatioRow[] {
  const at = (name: string) => summary.planes.get(name)!;
  const cases = summary.header.tables.support_load_cases as Json[];
  const rows: StressRatioRow[] = [];
  for (let i = group.rows.first; i < group.rows.first + group.rows.count; i += 1) {
    const ordinal = at("stress.ordinal")[i];
    const id = resolveId(ordinal);
    const entityRef = summaryValue(summary, at("stress.entity_ref")[i]);
    rows.push({
      key: id,
      order: ordinal,
      case: cases[at("stress.case")[i]],
      caseLabel: summaryValue(summary, at("stress.case_label")[i]),
      locations: [{
        entityRef,
        station: summaryValue(summary, at("stress.station")[i]),
        component: summaryValue(summary, at("stress.component")[i]),
        sectionBasis: summaryValue(summary, at("stress.section_basis")[i]),
        resultId: id,
        inputId: null
      }],
      target: { resultId: id, entityRef },
      standing: "admitted",
      withheldReason: null,
      stress: { value: at("stress.value")[i], unit: summaryValue(summary, at("stress.unit")[i]), resultId: id },
      outcome: null
    });
  }
  return rows;
}

/** S's stress and ratio table (no rule-check run) with each group's rows. */
export function stressRatioTableFromSummary(summary: ResultSummary, resolveId: (ordinal: number) => Json): StressRatioTable {
  const table = summary.header.tables.stress_ratio;
  return {
    ...table,
    groups: (table.groups as Json[]).map((group) => ({ ...group, rows: stressRatioRows(summary, group, resolveId) }))
  };
}

/** S's displacement extremes, an absent slot (`null`) read as the builder's `undefined`. */
export function displacementExtremesFromSummary(summary: ResultSummary): DisplacementExtremes {
  const data = summary.header.tables.displacement_extremes;
  return {
    ...data,
    nodes: (data.nodes as Json[]).map((node) => ({
      ...node,
      values: Object.fromEntries(Object.entries(node.values as Record<string, Json[]>).map(([id, slots]) => [id, slots.map((slot) => slot ?? undefined)]))
    }))
  };
}
