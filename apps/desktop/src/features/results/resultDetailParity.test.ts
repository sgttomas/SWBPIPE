import { describe, expect, it } from "vitest";
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import type { MechanicsResult } from "../../types";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import {
  RESULT_DETAIL_FILTER_TYPE_ERROR,
  RESULT_DETAIL_KINDS,
  RESULT_DETAIL_MALFORMED,
  RESULT_DETAIL_REQUEST_INVALID,
  agentDetailFilter,
  decodeDetailResponse,
  decodeLookup,
  decodeRowSemantics,
  decodeRowsPage,
  decodeSourceRecovery,
  panelDetailFilter,
  queryResultRows,
  readResultRowSemantics,
  resultRowPages,
  type DetailFilter,
  type ResultDetailInvoke
} from "../../services/resultDetail";
import { caseText, resultDetailCases, resultDetailCasesText, type DetailCase } from "../../test-support/resultDetailCases";
import { ResultDetailRefusal, emulateDetailFilter, emulateLookup, emulateRowsQuery, emulateSourceRecovery, panelPage } from "../../test-support/resultDetailEmulation";
import { PhysicsSourceEvidencePanel } from "./PhysicsSourceEvidencePanel";
import { rowsUnderFilters } from "./ResultsPanel";
import { rememberReceivedText } from "./resultReader";
import { semanticLabels } from "./resultSemantics";

/** C1 slice 3 (DEL-04-05 `c1-native-custody-plan.md`): the TypeScript readers over the full rows
 * are the oracle of the native detail commands (`src-tauri/src/result_detail.rs`). For every case
 * of `resultDetailCases.ts` (committed as `fixtures/results/result_detail_cases.json`), each
 * outcome the Rust test commits (`result_detail_outcomes.json`) is compared with:
 * - the rows query: the panel's `rowsUnderFilters` and its kind and selected entity, the agent's
 *   `readResults` filter, and the exact members, paged by offset and limit (and, where the query is
 *   a page of the panel's, `paginateResults` itself), including the refusals where the TypeScript
 *   throws or a bound is exceeded;
 * - the lookups: `results.find` (first) and `new Map(results.map(...))` (last) by id and kind, and
 *   rows by ordinal;
 * - the recovery facts: `PhysicsSourceEvidencePanel`'s own rendering where the result is the
 *   composite route, and its computation otherwise.
 * Exact row bytes are the native side's (`result_detail_tests.rs`). */

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
type Row = MechanicsResult["results"][number];
type Outcomes = { cases: { name: string; queries: Json[]; lookups: Json[]; recoveries: Json[] }[] };

const ROOT = resolve(__dirname, "../../../../..");
const CASES_PATH = resolve(ROOT, "fixtures/results/result_detail_cases.json");
const OUTCOMES = JSON.parse(readFileSync(resolve(ROOT, "fixtures/results/result_detail_outcomes.json"), "utf8")) as Outcomes;
const CASES = resultDetailCases();

/** Ordinal runs `[[first, last], ...]`, as the outcomes state them. */
function expand(runs: [number, number][]): number[] {
  return runs.flatMap(([first, last]) => Array.from({ length: last - first + 1 }, (_, i) => first + i));
}
/** The code a TypeScript reading refuses with (a TypeError the readers throw is the filter's),
 * or its value. */
type Refused = { refused: string };
function attempt<T>(read: () => T): T | Refused {
  try {
    return read();
  } catch (error) {
    if (error instanceof ResultDetailRefusal) return { refused: error.code };
    if (error instanceof TypeError) return { refused: RESULT_DETAIL_FILTER_TYPE_ERROR };
    throw error;
  }
}
const refusedOf = (value: unknown): value is Refused => typeof value === "object" && value !== null && !Array.isArray(value) && "refused" in value;
/** The embedded agent's current filter (`embeddedAgentSession.ts` `readResults`), verbatim. */
function agentRows(rows: Row[], filter: { kinds: string[]; entityRefs: string[] } | undefined): Row[] {
  return filter ? rows.filter(row => (!filter.kinds.length || filter.kinds.includes(row.kind)) && (!filter.entityRefs.length || filter.entityRefs.includes(row.entity_ref))) : rows;
}
const PANEL_KEYS = new Set(["text", "family", "frame", "case", "kinds", "entities"]);
const AGENT_KEYS = new Set(["kinds", "entities"]);
const text = (v: unknown) => (typeof v === "string" && v ? v : "unavailable");
const escape = (s: string) => s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");

describe("C1 slice 3: the native detail is the TypeScript readers' output", () => {
  it("keeps the committed cases file equal to its generator", () => {
    const committed = resultDetailCasesText();
    if (process.env.RESULT_DETAIL_CASES_WRITE) writeFileSync(CASES_PATH, committed);
    expect(readFileSync(CASES_PATH, "utf8")).toBe(committed);
    expect(OUTCOMES.cases.map((entry) => entry.name)).toEqual(CASES.map((testCase) => testCase.name));
  });

  it.each(CASES.map((testCase, i) => [testCase.name, testCase, OUTCOMES.cases[i]] as [string, DetailCase, Outcomes["cases"][number]]))("%s", async (_name, testCase, outcome) => {
    await loadWasmEngine();
    const sourceText = caseText(testCase.result);
    const source = JSON.parse(sourceText) as MechanicsResult;
    rememberReceivedText(source, sourceText);
    const rows: Row[] = Array.isArray(source?.results) ? source.results : [];
    const labels = semanticLabels(source);

    testCase.queries.forEach((query, q) => {
      const native = outcome.queries[q];
      const where = `query ${q} ${JSON.stringify(query)}`;
      const expected = attempt(() => emulateRowsQuery(rows, labels, query.filter, query.offset, query.limit));
      if (refusedOf(expected)) {
        expect(native, where).toEqual(expected);
        return;
      }
      expect(native.refused, where).toBeUndefined();
      expect({ total: native.total, ordinals: expand(native.ordinal_runs) }, where).toEqual(expected);
      expect(native.truncated, where).toBe(false);
      const keys = Object.keys(query.filter);
      // A page of the panel's own: its filters, its kind and selected entity, `paginateResults`.
      if (keys.every((key) => PANEL_KEYS.has(key)) && (query.filter.kinds?.length ?? 0) <= 1 && (query.filter.entities?.length ?? 0) <= 1 && query.offset % query.limit === 0 && query.offset < Math.max(1, native.total)) {
        const filter = query.filter as DetailFilter;
        const state = { filterText: filter.text ?? "", familyFilter: filter.family ?? "all", frame: filter.frame ?? "all", caseKey: filter.case ?? "all", kind: filter.kinds?.[0] ?? null };
        const panel = attempt(() => {
          const navigatorRows = rowsUnderFilters(rows, labels, state as Parameters<typeof rowsUnderFilters>[2]);
          const entity = filter.entities?.[0];
          const filtered = navigatorRows.filter((item) => entity === undefined || item.entity_ref === entity);
          return filtered.length;
        });
        if (!refusedOf(panel)) {
          expect(panel, `${where}: the panel's count`).toBe(native.total);
          expect(panelPage(rows, labels, panelDetailFilter({ ...state, selectionEntity: filter.entities?.[0] ?? null }), query.offset / query.limit, query.limit), `${where}: paginateResults`)
            .toEqual(expand(native.ordinal_runs));
        }
      }
      // The agent's own filter.
      if (keys.every((key) => AGENT_KEYS.has(key))) {
        const agentFilter = keys.length ? { kinds: [...(query.filter.kinds ?? [])], entityRefs: [...(query.filter.entities ?? [])] } : undefined;
        const ordinal = new Map(rows.map((row, i) => [row, i] as const));
        const admitted = attempt(() => agentRows(rows, agentFilter).map((row) => ordinal.get(row)!));
        if (!refusedOf(admitted)) {
          expect(emulateDetailFilter(rows, labels, agentDetailFilter(agentFilter)), `${where}: the agent's filter`).toEqual(admitted);
          expect(admitted.slice(query.offset, query.offset + query.limit), `${where}: the agent's page`).toEqual(expand(native.ordinal_runs));
        }
      }
    });

    testCase.lookups.forEach((request, l) => {
      const native = outcome.lookups[l];
      const where = `lookup ${l} ${JSON.stringify(request)}`;
      const expected = attempt(() => emulateLookup(rows, request));
      if (refusedOf(expected)) {
        expect(native, where).toEqual(expected);
        return;
      }
      expect(native.row_count, where).toBe(rows.length);
      expect(native.match, where).toBe(request.match);
      const ordinals = (group: string) => (native[group] as Json[]).map((entry) => entry.row?.ordinal ?? null);
      expect({ ordinals: ordinals("ordinals"), ids: ordinals("ids"), kinds: ordinals("kinds") }, where).toEqual(expected);
      // The consumers' own reads: `find` by id, the `Map` by id.
      (request.ids ?? []).forEach((id, i) => {
        const row = request.match === "first" ? rows.find((candidate) => candidate.id === id) : new Map(rows.map((candidate) => [candidate.id, candidate])).get(id);
        expect(row === undefined ? null : rows.indexOf(row), `${where}: id ${id}`).toBe(native.ids[i].row?.ordinal ?? null);
      });
    });

    testCase.recoveries.forEach((request, r) => {
      const native = outcome.recoveries[r];
      const where = `recovery ${r} ${JSON.stringify(request)}`;
      const expected = attempt(() => emulateSourceRecovery(source, request.offset, request.limit));
      if (refusedOf(expected)) {
        expect(native, where).toEqual(expected);
        return;
      }
      expect({
        present: native.present, receiptKind: native.receipt_kind, bodyKind: native.body_kind, statusPresent: native.status_present, status: native.status,
        caseCount: native.case_count, physicalCaseCount: native.physical_case_count, receiptIdsDistinct: native.receipt_ids_distinct,
        coverageMatches: native.coverage_matches, offset: native.offset,
        cases: native.cases.map((item: Json) => ({ index: item.index, entry: item.entry, refId: item.ref_id, selectedMethod: item.selected_method, outcome: item.outcome }))
      }, where).toEqual(expected);
      expect(native.truncated, where).toBe(false);
      expect(native.texts_sha256.length, where).toBe(request.raw ? native.cases.length : 0);
      // The panel itself, where it renders (the composite route).
      const html = renderToStaticMarkup(createElement(PhysicsSourceEvidencePanel, { result: source }));
      // The retained-source pressure fixtures are the composite route: the panel renders for them.
      if (testCase.name.startsWith("pressure_source/")) expect(html, where).not.toBe("");
      if (html) {
        expect(html, where).toContain(escape(`Producer receipt: ${text(native.status)}.`));
        expect(html, where).toContain(escape(`Recorded cases: ${native.case_count}; physical cases: ${native.physical_case_count}. ${native.coverage_matches ? "Receipt and physical case identities agree." : "Case evidence is incomplete or inconsistent."}`));
        for (const item of native.cases as Json[]) {
          expect(html, where).toContain(`Selected method: <code>${escape(text(item.selected_method))}</code>. Producer outcome: ${escape(text(item.outcome))}.`);
        }
      }
    });
  });
});

/* -------------------------------------------------------------------------- *
 * The decoders and adapters
 * -------------------------------------------------------------------------- */

function response(kind: string, header: Json, texts: string[]): Uint8Array {
  const encoder = new TextEncoder();
  const head = encoder.encode(JSON.stringify({ document_kind: kind, version: 1, ...header, lengths: texts.map((t) => encoder.encode(t).length) }));
  const body = texts.map((t) => encoder.encode(t));
  const out = new Uint8Array(16 + head.length + body.reduce((n, b) => n + b.length, 0));
  out.set(encoder.encode("SWBDETv1"), 0);
  new DataView(out.buffer).setBigUint64(8, BigInt(head.length), true);
  out.set(head, 16);
  let at = 16 + head.length;
  for (const b of body) {
    out.set(b, at);
    at += b.length;
  }
  return out;
}

describe("C1 slice 3: the detail responses' reader", () => {
  const rowsHeader = { job_id: "job-1", total: 3, offset: 0, returned: 2, truncated: false, ordinals: [0, 2] };
  const good = () => response(RESULT_DETAIL_KINDS.rows, rowsHeader, ['{"id":"a","value":1.50}', '{"id":"Ω"}']);

  it("reads rows with their exact texts", () => {
    const page = decodeRowsPage(good());
    expect(page.total).toBe(3);
    expect(page.rows.map((row) => [row.ordinal, row.text, row.row.id])).toEqual([[0, '{"id":"a","value":1.50}', "a"], [2, '{"id":"Ω"}', "Ω"]]);
  });

  it("refuses every other layout by name", () => {
    const bytes = good();
    const malformed: [string, Uint8Array][] = [
      ["truncated", bytes.subarray(0, bytes.length - 1)],
      ["trailing", new Uint8Array([...bytes, 32])],
      ["magic", new Uint8Array([...new TextEncoder().encode("SWBSUMv1"), ...bytes.subarray(8)])],
      ["kind", response(RESULT_DETAIL_KINDS.lookup, rowsHeader, ["{}", "{}"])],
      ["descending", response(RESULT_DETAIL_KINDS.rows, { ...rowsHeader, ordinals: [2, 0] }, ["{}", "{}"])],
      ["counts", response(RESULT_DETAIL_KINDS.rows, { ...rowsHeader, returned: 1 }, ["{}", "{}"])],
      ["not json", response(RESULT_DETAIL_KINDS.rows, rowsHeader, ["{", "{}"])],
      ["header", new Uint8Array([...bytes.subarray(0, 8), ...new Uint8Array(8).fill(255), ...bytes.subarray(16)])]
    ];
    for (const [name, input] of malformed) expect(() => decodeRowsPage(input), name).toThrow(RESULT_DETAIL_MALFORMED);
    expect(() => decodeDetailResponse(new Uint8Array(4), RESULT_DETAIL_KINDS.rows)).toThrow(RESULT_DETAIL_MALFORMED);
  });

  it("reads lookups, semantics and recovery pages against their requests", () => {
    const lookup = response(RESULT_DETAIL_KINDS.lookup, {
      row_count: 3, match: "last", ordinals: [{ ordinal: 9, row: null }], ids: [{ id: "a", row: { ordinal: 1, text: 0 } }], kinds: []
    }, ['{"id":"a"}']);
    const read = decodeLookup(lookup, { ordinals: [9], ids: ["a"], match: "last" });
    expect([read.ordinals[0], read.ids[0]?.ordinal, read.ids[0]?.text]).toEqual([null, 1, '{"id":"a"}']);
    expect(() => decodeLookup(lookup, { ordinals: [9], ids: ["b"], match: "last" })).toThrow(RESULT_DETAIL_MALFORMED);
    expect(() => decodeLookup(lookup, { ordinals: [9], ids: ["a"], match: "first" })).toThrow(RESULT_DETAIL_MALFORMED);
    expect(decodeRowSemantics('[{"ok":true,"signature":null,"complete_metadata":false,"canonical_metadata":{}}]', [4])[0].ok).toBe(true);
    expect(() => decodeRowSemantics("[]", [4])).toThrow(RESULT_DETAIL_MALFORMED);
    const recovery = response(RESULT_DETAIL_KINDS.recovery, {
      present: true, receipt_kind: "object", body_kind: "object", status_present: true, status: "partial", receipt_sha256: "x", receipt_version: 2, publication_sha256: null,
      case_count: 2, physical_case_count: 1, receipt_ids_distinct: true, coverage_matches: false, offset: 1, returned: 1, truncated: false,
      cases: [{ index: 1, entry: 3, ref_id: "L-2", selected_method: "m", outcome: null, text: 0 }]
    }, ['{"basis_ref":{"ref_id":"L-2"}}']);
    const facts = decodeSourceRecovery(recovery);
    expect([facts.status, facts.cases[0].refId, facts.cases[0].text]).toEqual(["partial", "L-2", '{"basis_ref":{"ref_id":"L-2"}}']);
  });

  it("decodes the semantics ArrayBuffer delivered by Tauri's raw response body", async () => {
    const text = '[{"ok":true,"signature":null,"complete_metadata":false,"canonical_metadata":{}}]';
    const invoke: ResultDetailInvoke = async (command, args) => {
      expect(command).toBe("read_preview_mechanics_job_row_semantics");
      expect(args).toEqual({ jobId: "job-1", ordinals: [4] });
      return new TextEncoder().encode(text).buffer;
    };
    expect(await readResultRowSemantics(invoke, "job-1", [4])).toEqual(JSON.parse(text));
  });

  it("pages every row through an adapter and refuses nothing silently", async () => {
    const source = JSON.parse(caseText(CASES[0].result)) as MechanicsResult;
    const all = source.results.map((row) => JSON.stringify(row));
    const invoke: ResultDetailInvoke = async (command, args) => {
      expect(command).toBe("query_preview_mechanics_job_rows");
      const offset = args.offset as number, limit = args.limit as number;
      const ordinals = all.map((_, i) => i).slice(offset, offset + limit);
      return response(RESULT_DETAIL_KINDS.rows, { job_id: args.jobId, total: all.length, offset, returned: ordinals.length, truncated: false, ordinals }, ordinals.map((i) => all[i]));
    };
    const texts: string[] = [];
    for await (const page of resultRowPages(invoke, "job-1", {}, 7)) texts.push(...page.rows.map((row) => row.text));
    expect(texts).toEqual(all);
    await expect(queryResultRows(async () => "text", "job-1", {}, 0, 1)).rejects.toThrow(RESULT_DETAIL_MALFORMED);
    expect(panelDetailFilter({ filterText: "x", familyFilter: "all", frame: "all", caseKey: "all", kind: "", selectionEntity: null })).toEqual({ text: "x", family: "all", frame: "all", case: "all" });
    expect(agentDetailFilter({ kinds: ["k"], entityRefs: [] })).toEqual({ kinds: ["k"], entities: [] });
    expect(RESULT_DETAIL_FILTER_TYPE_ERROR).toBe("RESULT-DETAIL-FILTER-TYPE-ERROR");
    expect(RESULT_DETAIL_REQUEST_INVALID).toBe("RESULT-DETAIL-REQUEST-INVALID");
  });
});
