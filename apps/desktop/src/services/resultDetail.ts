/** C1 native custody, slice 3 (DEL-04-05 `c1-native-custody-plan.md`): the webview's side of the
 * paged detail of a retained result (`src-tauri/src/result_detail.rs`). Nothing calls these
 * commands yet: the panels, the session and the embedded agent switch in slice 4. Until then the
 * TypeScript filters stay the oracle (`resultDetailParity.test.ts`); `resultDetailEmulation.ts`
 * reads the full rows as those filters do, for the fixtures' replay.
 *
 * - `query_preview_mechanics_job_rows {jobId, filter, offset, limit}`: the rows `filter` admits, in
 *   publication order, `limit` (1–1000) from the `offset`-th, with their total; each row the exact
 *   bytes the producer published. A page stops early (`truncated`) rather than exceed the response
 *   limit; paging on from `offset + returned` reaches every row.
 * - `lookup_preview_mechanics_job_rows {jobId, ordinals?, ids?, kinds?, match}`: rows by exact
 *   ordinal, and the first (`results.find`) or last (`new Map(results.map(r => [r.id, r]))`) row
 *   with an id or kind. Every row with an id is the query with `filter.ids`.
 * - `read_preview_mechanics_job_row_semantics {jobId, ordinals}`: the reader's row semantics of
 *   those rows, as `readerRowSemantics` reads them (one outcome per ordinal).
 * - `query_preview_mechanics_job_source_recovery {jobId, offset, limit, raw?}`: the recorded
 *   source-block recovery facts `ResultsPanel` and `PhysicsSourceEvidencePanel` show, and a page of
 *   the recorded cases (with `raw`, each case's exact recorded text). Recorded labels only: they
 *   grant no eligibility.
 *
 * - `query_preview_mechanics_job_navigator {jobId, filter, probe}` (slice 4's results navigator):
 *   the entity axis over the rows `filter` admits, as `navigatorEntities` and `firstRowOf` read the
 *   panel's `navigatorRows`: each distinct `entity_ref` once in first-publication order, with its
 *   first row's position among the admitted rows and its ordinal; the admitted total; and, for a
 *   probed ordinal, whether the filter admits it and where. It scales with entities, not rows.
 *
 * The row, lookup, recovery and navigator responses are one buffer: the magic `SWBDETv1`, the header's length
 * (u64, little-endian), the JSON header, then the texts it lists by `lengths`, concatenated, with
 * nothing after the last. `decodeDetailResponse` refuses anything else with
 * `RESULT-DETAIL-MALFORMED`. */
import type { MechanicsResult } from "../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
type Row = MechanicsResult["results"][number];

export const RESULT_DETAIL_MALFORMED = "RESULT-DETAIL-MALFORMED";
export const RESULT_DETAIL_REQUEST_INVALID = "RESULT-DETAIL-REQUEST-INVALID";
export const RESULT_DETAIL_RESPONSE_LIMIT = "RESULT-DETAIL-RESPONSE-LIMIT";
export const RESULT_DETAIL_FILTER_TYPE_ERROR = "RESULT-DETAIL-FILTER-TYPE-ERROR";
export const RESULT_DETAIL_NAVIGATOR_ENTITY_TYPE = "RESULT-DETAIL-NAVIGATOR-ENTITY-TYPE";
export const RESULT_DETAIL_VERSION = 1;
export const RESULT_DETAIL_KINDS = Object.freeze({
  rows: "swbpipe.desktop.result_detail_rows",
  lookup: "swbpipe.desktop.result_detail_lookup",
  recovery: "swbpipe.desktop.result_source_recovery",
  navigator: "swbpipe.desktop.result_detail_navigator"
} as const);
/** The most rows (or recorded cases) one page returns; the most keys one lookup, semantics batch
 * or filter list states. */
export const RESULT_DETAIL_MAX_PAGE_ROWS = 1000;
export const RESULT_DETAIL_MAX_KEYS = 1000;
export const RESULT_DETAIL_COMMANDS = Object.freeze([
  "query_preview_mechanics_job_rows",
  "lookup_preview_mechanics_job_rows",
  "read_preview_mechanics_job_row_semantics",
  "query_preview_mechanics_job_source_recovery",
  "query_preview_mechanics_job_navigator"
] as const);
export type ResultDetailCommand = (typeof RESULT_DETAIL_COMMANDS)[number];
const MAGIC = "SWBDETv1";

/** A rows query's filter: each member optional; all present must admit a row. `family`, `frame`
 * and `case` equal to `"all"`, and empty lists, admit every row (as the panel's and the agent's
 * filters do). `text`, `family`, `frame`, `case`, `kinds` and `entities` read the row as the
 * results panel and the embedded agent do; `ids`, `set` (`basis_ref.ref_id`) and `station`
 * (`metadata.location`) are exact. */
export type DetailFilter = Readonly<{
  text?: string;
  family?: string;
  frame?: string;
  case?: string;
  set?: string;
  station?: string;
  kinds?: readonly string[];
  entities?: readonly string[];
  ids?: readonly string[];
}>;

/** The results panel's filters as the native query states them: its text, family, frame and case,
 * its kind request and, with "Selected entity only", the selected entity (`null`: none selected,
 * which the panel shows as no rows without asking). */
export function panelDetailFilter(state: Readonly<{ filterText: string; familyFilter: string; frame: string; caseKey: string; kind?: string | null; selectionEntity?: string | null }>): DetailFilter {
  return {
    text: state.filterText,
    family: state.familyFilter,
    frame: state.frame,
    case: state.caseKey,
    ...(state.kind ? { kinds: [state.kind] } : {}),
    ...(state.selectionEntity != null ? { entities: [state.selectionEntity] } : {})
  };
}

/** The embedded agent's current `swbpipe_read_results` filter as the native query states it. */
export function agentDetailFilter(filter?: Readonly<{ kinds: readonly string[]; entityRefs: readonly string[] }>): DetailFilter {
  return filter ? { kinds: filter.kinds, entities: filter.entityRefs } : {};
}

export class ResultDetailMalformed extends Error {
  constructor(reason: string) {
    super(`${RESULT_DETAIL_MALFORMED}: ${reason}`);
    this.name = "ResultDetailMalformed";
  }
}
function fail(reason: string): never {
  throw new ResultDetailMalformed(reason);
}
const count = (value: unknown, what: string): number => (Number.isSafeInteger(value) && (value as number) >= 0 ? value as number : fail(`${what} is not a count`));

export type DetailResponse = Readonly<{ header: Json; texts: readonly string[] }>;

/** Reads a row, lookup or recovery response, checking its layout. */
export function decodeDetailResponse(input: ArrayBuffer | Uint8Array, kind: string): DetailResponse {
  const bytes = input instanceof Uint8Array ? input : new Uint8Array(input);
  if (bytes.length < 16 || String.fromCharCode(...bytes.subarray(0, 8)) !== MAGIC) fail("not a result detail response");
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const headerLength = view.getBigUint64(8, true);
  if (headerLength > BigInt(bytes.length - 16)) fail("the header runs past the end");
  const decoder = new TextDecoder("utf-8", { fatal: true });
  let header: Json;
  let at = 16 + Number(headerLength);
  try {
    header = JSON.parse(decoder.decode(bytes.subarray(16, at)));
  } catch (error) {
    fail(`the header is not JSON: ${(error as Error).message}`);
  }
  if (header === null || typeof header !== "object" || Array.isArray(header)) fail("the header is not an object");
  if (header.document_kind !== kind || header.version !== RESULT_DETAIL_VERSION) fail("another document kind or version");
  if (!Array.isArray(header.lengths)) fail("no text lengths");
  const texts = (header.lengths as unknown[]).map((length, i) => {
    const end = at + count(length, `text ${i}'s length`);
    if (end > bytes.length) fail(`text ${i} runs past the end`);
    let text: string;
    try {
      text = decoder.decode(bytes.subarray(at, end));
    } catch {
      fail(`text ${i} is not UTF-8`);
    }
    at = end;
    return text;
  });
  if (at !== bytes.length) fail("bytes follow the last text");
  return Object.freeze({ header, texts: Object.freeze(texts) });
}

/** A row as a page returns it: its ordinal in `results`, its exact text and its parsed value. */
export type DetailRow = Readonly<{ ordinal: number; text: string; row: Row }>;
export type RowsPage = Readonly<{ total: number; offset: number; truncated: boolean; rows: readonly DetailRow[] }>;

function detailRow(ordinal: number, text: string, rowCount: number | null): DetailRow {
  if (!Number.isSafeInteger(ordinal) || ordinal < 0 || (rowCount !== null && ordinal >= rowCount)) fail(`row ordinal ${ordinal} is outside the result`);
  let row: Row;
  try {
    row = JSON.parse(text);
  } catch {
    fail(`row ${ordinal} is not JSON`);
  }
  return Object.freeze({ ordinal, text, row });
}

/** Reads a rows response: ordinals ascending, each with its text. */
export function decodeRowsPage(input: ArrayBuffer | Uint8Array): RowsPage {
  const { header, texts } = decodeDetailResponse(input, RESULT_DETAIL_KINDS.rows);
  const total = count(header.total, "total");
  const ordinals = Array.isArray(header.ordinals) ? header.ordinals as number[] : fail("no ordinals");
  if (ordinals.length !== texts.length || count(header.returned, "returned") !== texts.length) fail("ordinals and texts differ in number");
  if (typeof header.truncated !== "boolean") fail("no truncation flag");
  const rows = ordinals.map((ordinal, i) => {
    if (i > 0 && !(ordinal > ordinals[i - 1])) fail("ordinals do not ascend");
    return detailRow(ordinal, texts[i], null);
  });
  if (rows.length > total) fail("more rows than the total");
  return Object.freeze({ total, offset: count(header.offset, "offset"), truncated: header.truncated, rows: Object.freeze(rows) });
}

export type LookupMatch = "first" | "last";
export type LookupRequest = Readonly<{ ordinals?: readonly number[]; ids?: readonly string[]; kinds?: readonly string[]; match: LookupMatch }>;
export type Lookup = Readonly<{
  rowCount: number;
  ordinals: readonly (DetailRow | null)[];
  ids: readonly (DetailRow | null)[];
  kinds: readonly (DetailRow | null)[];
}>;

/** Reads a lookup response: one entry per requested key, in order, each a row or `null`. */
export function decodeLookup(input: ArrayBuffer | Uint8Array, request: LookupRequest): Lookup {
  const { header, texts } = decodeDetailResponse(input, RESULT_DETAIL_KINDS.lookup);
  const rowCount = count(header.row_count, "row_count");
  if (header.match !== request.match) fail("another match");
  const entries = (group: "ordinals" | "ids" | "kinds", keys: readonly (number | string)[] | undefined, member: string) => {
    const list = Array.isArray(header[group]) ? header[group] as Json[] : fail(`no ${group}`);
    if (list.length !== (keys ?? []).length) fail(`${group} are not the requested keys`);
    return Object.freeze(list.map((entry, i) => {
      if (entry?.[member] !== keys![i]) fail(`${group} entry ${i} is not the requested key`);
      if (entry.row === null) return null;
      const at = count(entry.row?.text, `${group} entry ${i}'s text`);
      if (at >= texts.length) fail(`${group} entry ${i} names no text`);
      return detailRow(entry.row.ordinal, texts[at], rowCount);
    }));
  };
  return Object.freeze({
    rowCount,
    ordinals: entries("ordinals", request.ordinals, "ordinal"),
    ids: entries("ids", request.ids, "id"),
    kinds: entries("kinds", request.kinds, "kind")
  });
}

/** One row's semantics outcome, as `reader_row_semantics_json` returns it. */
export type RowSemanticsOutcome = Readonly<{ ok: true; signature: Json; complete_metadata: boolean; canonical_metadata: Json } | { ok: false; error: { code: string; detail: string | null } }>;

/** Reads a semantics response: one outcome per requested ordinal. */
export function decodeRowSemantics(text: string, ordinals: readonly number[]): readonly RowSemanticsOutcome[] {
  let outcomes: Json;
  try {
    outcomes = JSON.parse(text);
  } catch {
    fail("the semantics are not JSON");
  }
  if (!Array.isArray(outcomes) || outcomes.length !== ordinals.length) fail("not one outcome per row");
  if (outcomes.some((outcome: Json) => typeof outcome?.ok !== "boolean")) fail("an outcome without `ok`");
  return Object.freeze(outcomes);
}

export type RecoveryCase = Readonly<{ index: number; entry: number; refId: string; selectedMethod: Json; outcome: Json; text: string | null }>;
export type SourceRecovery = Readonly<{
  present: boolean;
  receiptKind: string;
  bodyKind: string;
  statusPresent: boolean;
  status: Json;
  receiptSha256: Json;
  receiptVersion: Json;
  publicationSha256: Json;
  caseCount: number;
  physicalCaseCount: number;
  receiptIdsDistinct: boolean;
  coverageMatches: boolean;
  offset: number;
  truncated: boolean;
  cases: readonly RecoveryCase[];
}>;

/** Reads a recovery response. */
export function decodeSourceRecovery(input: ArrayBuffer | Uint8Array): SourceRecovery {
  const { header, texts } = decodeDetailResponse(input, RESULT_DETAIL_KINDS.recovery);
  const flag = (value: unknown, what: string) => (typeof value === "boolean" ? value : fail(`${what} is not a flag`));
  const caseCount = count(header.case_count, "case_count");
  const cases = (Array.isArray(header.cases) ? header.cases as Json[] : fail("no cases")).map((item, i): RecoveryCase => {
    const index = count(item?.index, `case ${i}'s index`);
    if (index >= caseCount) fail(`case ${i} is outside the recorded cases`);
    const at = item.text === null ? null : count(item.text, `case ${i}'s text`);
    if (at !== null && at >= texts.length) fail(`case ${i} names no text`);
    return Object.freeze({ index, entry: count(item.entry, `case ${i}'s entry`), refId: typeof item.ref_id === "string" ? item.ref_id : fail(`case ${i}'s id`), selectedMethod: item.selected_method, outcome: item.outcome, text: at === null ? null : texts[at] });
  });
  if (count(header.returned, "returned") !== cases.length) fail("returned is not the cases' number");
  return Object.freeze({
    present: flag(header.present, "present"),
    receiptKind: String(header.receipt_kind),
    bodyKind: String(header.body_kind),
    statusPresent: flag(header.status_present, "status_present"),
    status: header.status,
    receiptSha256: header.receipt_sha256,
    receiptVersion: header.receipt_version,
    publicationSha256: header.publication_sha256,
    caseCount,
    physicalCaseCount: count(header.physical_case_count, "physical_case_count"),
    receiptIdsDistinct: flag(header.receipt_ids_distinct, "receipt_ids_distinct"),
    coverageMatches: flag(header.coverage_matches, "coverage_matches"),
    offset: count(header.offset, "offset"),
    truncated: flag(header.truncated, "truncated"),
    cases: Object.freeze(cases)
  });
}

/** The navigator's entity axis over the rows a filter admits. */
export type NavigatorAxisRead = Readonly<{
  total: number;
  entities: readonly string[];
  /** Each entity's first row: its position among the admitted rows, and its ordinal. */
  firstIndices: readonly number[];
  firstOrdinals: readonly number[];
  probe: Readonly<{ ordinal: number; admitted: boolean; index: number | null }> | null;
}>;

/** Reads a navigator response: one first position and ordinal per entity, entities distinct,
 * positions ascending from 0 within the total, ordinals ascending; no texts. */
export function decodeNavigator(input: ArrayBuffer | Uint8Array, probe: number | null): NavigatorAxisRead {
  const { header, texts } = decodeDetailResponse(input, RESULT_DETAIL_KINDS.navigator);
  if (texts.length !== 0) fail("a navigator response carries no texts");
  const total = count(header.total, "total");
  const entities = Array.isArray(header.entities) ? header.entities as unknown[] : fail("no entities");
  const firstIndices = Array.isArray(header.first_indices) ? header.first_indices as unknown[] : fail("no first indices");
  const firstOrdinals = Array.isArray(header.first_ordinals) ? header.first_ordinals as unknown[] : fail("no first ordinals");
  if (firstIndices.length !== entities.length || firstOrdinals.length !== entities.length) fail("entities and first rows differ in number");
  const seen = new Set<string>();
  entities.forEach((entity, i) => {
    if (typeof entity !== "string" || seen.has(entity)) fail(`entity ${i} is not a distinct text`);
    seen.add(entity);
    const index = count(firstIndices[i], `entity ${i}'s first index`);
    count(firstOrdinals[i], `entity ${i}'s first ordinal`);
    if (index >= total || (i === 0 ? index !== 0 : !(index > (firstIndices[i - 1] as number)))) fail(`entity ${i}'s first index is out of order`);
    if (i > 0 && !((firstOrdinals[i] as number) > (firstOrdinals[i - 1] as number))) fail(`entity ${i}'s first ordinal is out of order`);
  });
  if (total > 0 && entities.length === 0) fail("admitted rows without an entity");
  let read: NavigatorAxisRead["probe"] = null;
  if (probe === null) {
    if (header.probe !== null) fail("a probe answer without a probe");
  } else {
    const answer = header.probe;
    if (!answer || answer.ordinal !== probe || typeof answer.admitted !== "boolean") fail("not the probe's answer");
    const index = answer.index === null ? null : count(answer.index, "the probe's index");
    if ((index !== null) !== answer.admitted || (index !== null && index >= total)) fail("the probe's index is not its admission");
    read = Object.freeze({ ordinal: probe, admitted: answer.admitted, index });
  }
  return Object.freeze({ total, entities: Object.freeze(entities as string[]), firstIndices: Object.freeze(firstIndices as number[]), firstOrdinals: Object.freeze(firstOrdinals as number[]), probe: read });
}

/* -------------------------------------------------------------------------- *
 * Adapters (slice 4 binds `invoke` to the raw-body ingress)
 * -------------------------------------------------------------------------- */

/** Sends a detail command's arguments and returns its response. */
export type ResultDetailInvoke = (command: ResultDetailCommand, args: Record<string, unknown>) => Promise<ArrayBuffer | Uint8Array | string>;

const binary = (response: ArrayBuffer | Uint8Array | string): ArrayBuffer | Uint8Array => (typeof response === "string" ? fail("a text response where bytes were expected") : response);

export async function queryResultRows(invoke: ResultDetailInvoke, jobId: string, filter: DetailFilter, offset: number, limit: number): Promise<RowsPage> {
  return decodeRowsPage(binary(await invoke("query_preview_mechanics_job_rows", { jobId, filter, offset, limit })));
}
export async function lookupResultRows(invoke: ResultDetailInvoke, jobId: string, request: LookupRequest): Promise<Lookup> {
  return decodeLookup(binary(await invoke("lookup_preview_mechanics_job_rows", { jobId, ...request })), request);
}
export async function readResultRowSemantics(invoke: ResultDetailInvoke, jobId: string, ordinals: readonly number[]): Promise<readonly RowSemanticsOutcome[]> {
  const response = await invoke("read_preview_mechanics_job_row_semantics", { jobId, ordinals });
  return decodeRowSemantics(typeof response === "string" ? response : new TextDecoder("utf-8", { fatal: true }).decode(response), ordinals);
}
export async function queryResultSourceRecovery(invoke: ResultDetailInvoke, jobId: string, offset: number, limit: number, raw = false): Promise<SourceRecovery> {
  return decodeSourceRecovery(binary(await invoke("query_preview_mechanics_job_source_recovery", { jobId, offset, limit, raw })));
}

export async function queryResultNavigator(invoke: ResultDetailInvoke, jobId: string, filter: DetailFilter, probe: number | null = null): Promise<NavigatorAxisRead> {
  return decodeNavigator(binary(await invoke("query_preview_mechanics_job_navigator", { jobId, filter, probe })), probe);
}

/** Every row a filter admits, page by page (each within the response limit). */
export async function* resultRowPages(invoke: ResultDetailInvoke, jobId: string, filter: DetailFilter, limit = RESULT_DETAIL_MAX_PAGE_ROWS): AsyncGenerator<RowsPage> {
  for (let offset = 0; ;) {
    const page = await queryResultRows(invoke, jobId, filter, offset, limit);
    if (page.rows.length === 0) return;
    yield page;
    offset += page.rows.length;
  }
}
