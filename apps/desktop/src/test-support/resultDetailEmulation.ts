/** C1 slice 3 (DEL-04-05 `c1-native-custody-plan.md`): the native detail commands
 * (`services/resultDetail.ts`) emulated over the full rows by the TypeScript readers they replace:
 * the results panel's `rowsUnderFilters` and `paginateResults`, the embedded agent's current
 * `readResults` filter, `results.find` and `new Map(results.map(...))`, and
 * `PhysicsSourceEvidencePanel`'s recovery facts. The parity test checks the native outcomes against
 * these; slice 4's fixture replay answers the commands with them. Ordinals, not bytes: exact row
 * bytes are the native side's, checked in `result_detail_tests.rs`. */
import type { MechanicsResult } from "../types";
import { paginateResults, rowsUnderFilters } from "../features/results/ResultsPanel";
import type { RowLabels } from "../features/results/resultSemantics";
import {
  RESULT_DETAIL_FILTER_TYPE_ERROR,
  RESULT_DETAIL_MAX_KEYS,
  RESULT_DETAIL_MAX_PAGE_ROWS,
  RESULT_DETAIL_REQUEST_INVALID,
  type DetailFilter,
  type LookupRequest,
  type RecoveryCase,
  type SourceRecovery
} from "../services/resultDetail";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
type Row = MechanicsResult["results"][number];

export class ResultDetailRefusal extends Error {
  constructor(readonly code: string, detail: string) {
    super(`${code}: ${detail}`);
    this.name = "ResultDetailRefusal";
  }
}
function pageLimit(limit: number): void {
  if (!Number.isSafeInteger(limit) || limit < 1 || limit > RESULT_DETAIL_MAX_PAGE_ROWS) throw new ResultDetailRefusal(RESULT_DETAIL_REQUEST_INVALID, `limit ${limit} is outside 1..=${RESULT_DETAIL_MAX_PAGE_ROWS}`);
}
function list(values: readonly string[] | undefined, name: string): readonly string[] | null {
  if (values && values.length > RESULT_DETAIL_MAX_KEYS) throw new ResultDetailRefusal(RESULT_DETAIL_REQUEST_INVALID, `filter.${name} states ${values.length} values`);
  return values && values.length > 0 ? values : null;
}
/** A TypeError the readers throw, as the native refusal names it. */
function typed<T>(read: () => T): T {
  try {
    return read();
  } catch (error) {
    if (error instanceof TypeError) throw new ResultDetailRefusal(RESULT_DETAIL_FILTER_TYPE_ERROR, error.message);
    throw error;
  }
}

/** The rows a filter admits, in order, by ordinal: the results panel's `rowsUnderFilters` (text,
 * family, frame and case), then its kind and selected entity, the agent's lists, and the exact
 * members. */
export function emulateDetailFilter(rows: readonly Row[], labels: RowLabels, filter: DetailFilter): number[] {
  const kinds = list(filter.kinds, "kinds"), entities = list(filter.entities, "entities"), ids = list(filter.ids, "ids");
  const ordinal = new Map(rows.map((row, i) => [row, i] as const));
  return typed(() => rowsUnderFilters(rows as Row[], labels, {
    filterText: filter.text ?? "",
    familyFilter: (filter.family ?? "all") as Parameters<typeof rowsUnderFilters>[2]["familyFilter"],
    frame: filter.frame ?? "all",
    caseKey: filter.case ?? "all",
    kind: null
  }).filter((row) =>
    (!kinds || kinds.includes(row.kind)) &&
    (!entities || entities.includes(row.entity_ref)) &&
    (!ids || ids.includes(row.id)) &&
    (filter.set === undefined || (row.basis_ref as Json)?.ref_id === filter.set) &&
    (filter.station === undefined || (row.metadata as Json)?.location === filter.station)
  ).map((row) => ordinal.get(row)!));
}

/** A rows query over the full rows: its total and the page's ordinals. */
export function emulateRowsQuery(rows: readonly Row[], labels: RowLabels, filter: DetailFilter, offset: number, limit: number): { total: number; ordinals: number[] } {
  pageLimit(limit);
  const admitted = emulateDetailFilter(rows, labels, filter);
  return { total: admitted.length, ordinals: admitted.slice(offset, offset + limit) };
}

/** The results panel's own page of a filter: `paginateResults` at a page index and size. */
export function panelPage(rows: readonly Row[], labels: RowLabels, filter: DetailFilter, pageIndex: number, pageSize: number): number[] {
  const ordinal = new Map(rows.map((row, i) => [row, i] as const));
  const admitted = emulateDetailFilter(rows, labels, filter).map((i) => rows[i]);
  return paginateResults(admitted as Row[], pageIndex, pageSize).items.map((row) => ordinal.get(row)!);
}

/** A lookup over the full rows: the ordinal of each requested row, or `null`. */
export function emulateLookup(rows: readonly Row[], request: LookupRequest): { ordinals: (number | null)[]; ids: (number | null)[]; kinds: (number | null)[] } {
  const keys = (request.ordinals?.length ?? 0) + (request.ids?.length ?? 0) + (request.kinds?.length ?? 0);
  if (keys === 0 || keys > RESULT_DETAIL_MAX_KEYS) throw new ResultDetailRefusal(RESULT_DETAIL_REQUEST_INVALID, `a lookup states ${keys} keys`);
  const by = (member: "id" | "kind", values: readonly string[] | undefined) => typed(() => {
    if (!values?.length) return [];
    if (request.match === "first") return values.map((value) => { const at = rows.findIndex((row) => row[member] === value); return at < 0 ? null : at; });
    const last = new Map(rows.map((row, i) => [row[member], i] as const));
    return values.map((value) => last.get(value) ?? null);
  });
  return {
    ordinals: (request.ordinals ?? []).map((ordinal) => (ordinal < rows.length ? ordinal : null)),
    ids: by("id", request.ids),
    kinds: by("kind", request.kinds)
  };
}

const record = (v: unknown): Record<string, unknown> | null => (v && typeof v === "object" && !Array.isArray(v) ? v as Record<string, unknown> : null);
const records = (v: unknown): [number, Record<string, unknown>][] => (Array.isArray(v) ? v.map((item, i) => [i, record(item)] as const).filter((x): x is [number, Record<string, unknown>] => x[1] !== null) : []);
const label = (v: unknown) => (typeof v === "string" && v ? v : "unavailable");

/** The recovery facts over the full envelope, as `PhysicsSourceEvidencePanel` computes them. */
export function emulateSourceRecovery(source: MechanicsResult, offset: number, limit: number): Omit<SourceRecovery, "truncated" | "cases" | "receiptSha256" | "receiptVersion" | "publicationSha256"> & { cases: Omit<RecoveryCase, "text">[] } {
  pageLimit(limit);
  const envelope = source as unknown as Record<string, unknown>;
  const present = Object.hasOwn(envelope, "source_block_recovery");
  const receipt = envelope.source_block_recovery;
  const body = record(receipt)?.body;
  const cases = records(record(body)?.cases);
  const exactCases = records(record(envelope.contract_evidence)?.exact_cases);
  const receiptIds = cases.map(([, c]) => label(record(c.basis_ref)?.ref_id));
  const physicalIds = exactCases.map(([, c]) => label(c.load_case_id));
  const kind = (v: unknown, here: boolean) => (!here ? "absent" : v === null ? "null" : Array.isArray(v) ? "array" : typeof v === "object" ? "object" : typeof v);
  const status = record(body) ? record(body)! : null;
  return {
    present,
    receiptKind: kind(receipt, present),
    bodyKind: kind(body, record(receipt) !== null && Object.hasOwn(record(receipt)!, "body")),
    statusPresent: status !== null && Object.hasOwn(status, "status"),
    status: status?.status ?? null,
    caseCount: cases.length,
    physicalCaseCount: exactCases.length,
    receiptIdsDistinct: new Set(receiptIds).size === receiptIds.length,
    coverageMatches: receiptIds.length === physicalIds.length && new Set(receiptIds).size === receiptIds.length && receiptIds.every((id) => physicalIds.includes(id)),
    offset,
    cases: cases.slice(offset, offset + limit).map(([entry, c], i) => ({ index: offset + i, entry, refId: receiptIds[offset + i], selectedMethod: c.selected_method ?? null, outcome: c.outcome ?? null }))
  };
}
