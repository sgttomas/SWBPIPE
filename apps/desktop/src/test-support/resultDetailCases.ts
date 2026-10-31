// The cases of C1 slice 3's parity (DEL-04-05 `c1-native-custody-plan.md`, slice 3): each result
// of slice 2's summary cases (the fixtures the builders' tests, the demos and the native replay use,
// and their invented results), two retained-source pressure results with recorded recovery evidence,
// and invented results for the paths those do not reach (search over every member type, JavaScript
// number text, Unicode case mapping, duplicate ids, rows the filter throws on, empty and absent
// results, a duplicated `results` member, whitespace around rows, and incoherent recovery evidence).
// Each case states its rows queries, lookups and recovery pages, generated from its rows. Written to
// `fixtures/results/result_detail_cases.json`, which the desktop's Rust test reads to run and commit
// the native outcomes (`result_detail_tests.rs`). Invented data; no engineering meaning.

import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { jsonTextKeepingNegativeZero } from "../services/jsonText";
import { resultSummaryCases, type CaseText } from "./resultSummaryCases";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

export type DetailQuery = Readonly<{ filter: Json; offset: number; limit: number }>;
export type DetailLookup = Readonly<{ ordinals?: number[]; ids?: Json[]; kinds?: Json[]; match: "first" | "last" }>;
export type DetailRecovery = Readonly<{ offset: number; limit: number; raw?: boolean }>;
export type DetailCase = Readonly<{ name: string; result: CaseText; queries: DetailQuery[]; lookups: DetailLookup[]; recoveries: DetailRecovery[] }>;

const ROOT = resolve(__dirname, "../../../..");
const FAMILIES = ["displacement", "rotation", "reaction", "force", "moment", "stress", "ratio", "other"];
const inline = (value: unknown): CaseText => ({ text: jsonTextKeepingNegativeZero(value) });
const file = (path: string, pointer = ""): CaseText => ({ path, pointer });

function pointer(value: Json, path: string): Json {
  return path === "" ? value : path.slice(1).split("/").map((token) => token.replace(/~1/g, "/").replace(/~0/g, "~")).reduce((at, token) => at[token], value);
}
/** A case's text, as the tests read it (a member of a file is written by `JSON.stringify`, which
 * serde_json's compact writer equals on these fixtures; only the rows are read here). */
export function caseText(reference: CaseText): string {
  if ("text" in reference) return reference.text;
  const text = readFileSync(resolve(ROOT, reference.path), "utf8");
  return reference.pointer === "" ? text : JSON.stringify(pointer(JSON.parse(text), reference.pointer));
}

const distinct = <T,>(values: T[]): T[] => [...new Set(values)];
const frameOf = (row: Json) => {
  const frame = row?.metadata?.coordinate_system;
  return typeof frame === "string" && frame.trim() ? frame : "unstated";
};
const caseKeyOf = (row: Json) => (row?.basis_ref ? `${row.basis_ref.ref_type}:${row.basis_ref.ref_id}` : "unstated");
const strings = (values: unknown[]): string[] => distinct(values.filter((value): value is string => typeof value === "string"));

/** The queries of a result, from its rows: paging bounds, each family, frame and case, text
 * queries from the rows' own members, the kind, entity and id lists, the exact members, and the
 * panel's and the agent's combined filters. */
function queries(rows: Json[], extraText: readonly string[]): DetailQuery[] {
  const n = rows.length;
  const objects = rows.filter((row) => row && typeof row === "object" && !Array.isArray(row));
  const pick = [objects[0], objects[Math.floor(objects.length / 2)], objects[objects.length - 1]].filter(Boolean);
  const out: DetailQuery[] = [];
  const page = (filter: Json, offset = 0, limit = 200) => out.push({ filter, offset, limit });
  for (const [offset, limit] of [[0, 1], [0, 7], [0, 1000], [Math.max(0, n - 1), 50], [n, 5], [n + 10, 5], [1e15, 1], [0, 0], [0, 1001]]) page({}, offset, limit);
  for (const family of ["all", ...FAMILIES]) page({ family }, 0, 1000);
  page({ family: "stress" }, 3, 2);
  const frames = distinct(objects.map(frameOf));
  for (const frame of ["all", ...frames, "no-such-frame"]) page({ frame });
  const cases = distinct(objects.map(caseKeyOf));
  for (const key of ["all", ...cases.slice(0, 6)]) page({ case: key });
  const tokens = strings(pick.flatMap((row) => [
    row.id, typeof row.id === "string" ? row.id.toUpperCase().slice(2, 14) : null, row.unit, String(row.value), row.kind?.slice?.(0, 8), row.entity_ref,
    row.metadata?.location, row.basis_ref?.ref_id
  ]));
  for (const text of distinct(["", "   ", "undefined", "null", "e+", "[object", ...tokens, ...extraText])) page({ text });
  const kinds = strings(pick.map((row) => row.kind));
  const entities = strings(pick.map((row) => row.entity_ref));
  const ids = strings(objects.map((row) => row.id));
  const duplicate = ids.find((id) => objects.filter((row) => row.id === id).length > 1);
  for (const list of [kinds.slice(0, 1), kinds, [], ["no-such-kind"]]) page({ kinds: list });
  for (const list of [entities.slice(0, 1), entities]) page({ entities: list });
  for (const list of [ids.slice(0, 1), duplicate ? [duplicate] : [], ["no-such-id"], ids.slice(0, 3)]) page({ ids: list });
  const set = strings(objects.map((row) => row.basis_ref?.ref_id))[0];
  const station = strings(objects.map((row) => row.metadata?.location))[0];
  if (set !== undefined) page({ set });
  if (station !== undefined) page({ station });
  page({ set: "no-such-set", station: "no-such-station" });
  // The results panel's filters together, and the embedded agent's.
  const first = objects[0];
  if (first) {
    page({ text: first.unit ?? "", family: "all", frame: frameOf(first), case: caseKeyOf(first), kinds: kinds.slice(0, 1), entities: entities.slice(0, 1) });
    page({ text: "pipe", family: "stress", frame: "all", case: caseKeyOf(first) }, 0, 50);
    page({ text: "pipe", family: "stress", frame: "all", case: caseKeyOf(first) }, 50, 50);
    page({ text: first.unit ?? "", family: "displacement", frame: frameOf(first), case: "all" }, 2, 3);
  }
  page({ kinds, entities }, 1, 2);
  page({ kinds: kinds.slice(0, 1), entities: [] }, 0, 50);
  return out;
}

function lookups(rows: Json[]): DetailLookup[] {
  const n = rows.length;
  const objects = rows.filter((row) => row && typeof row === "object" && !Array.isArray(row));
  const ids = strings(objects.map((row) => row.id));
  const duplicate = ids.find((id) => objects.filter((row) => row.id === id).length > 1);
  const kinds = strings(objects.map((row) => row.kind));
  const keyIds = distinct([ids[0], duplicate, ids[ids.length - 1], "no-such-id", ids[0]].filter((id): id is string => id !== undefined));
  const keyKinds = [kinds[0], kinds[kinds.length - 1], "no-such-kind"].filter((kind): kind is string => kind !== undefined);
  return [
    { ordinals: [0, Math.floor(n / 2), n - 1, n, 1e15].filter((ordinal) => ordinal >= 0), match: "first" },
    { ids: [...keyIds, ids[0] ?? "x"], kinds: keyKinds, match: "first" },
    { ids: [...keyIds], kinds: keyKinds, match: "last" },
    { ordinals: [0], ids: ["no-such-id"], match: "last" },
    { ordinals: [], match: "first" }
  ];
}

const RECOVERIES: DetailRecovery[] = [{ offset: 0, limit: 1000, raw: true }, { offset: 1, limit: 1, raw: false }, { offset: 1e15, limit: 5, raw: true }, { offset: 0, limit: 0 }];

/* Invented results */

const T = (id: Json, kind: Json, entity: Json, unit: Json, value: Json, extra: Json = {}) => ({ id, kind, entity_ref: entity, unit, value, ...extra });
const TEXT_ROWS = [
  T("ΟΔΟΣ-1", "k_alpha", "pipe:İ", "MPa", 1e21, { basis_ref: { ref_type: "load_case", ref_id: "L-1" }, metadata: { coordinate_system: "global", location: "midspan", component: "Fx", basis: "nominal", sign_convention: "tension_positive" }, source_result_refs: ["src:A", "", 0, null, false, ["x", "y"]] }),
  T("straße", "k_beta", "node:N1", "mm", 1e-7, { basis_ref: { ref_type: "combination", ref_id: 7 }, metadata: { coordinate_system: "  " } }),
  T("dup", "k_alpha", "node:N1", "mm", -0, { basis_ref: null, metadata: null }),
  T("dup", "k_gamma", "node:N2", "", "text value", { basis_ref: "loose", metadata: { coordinate_system: "Ünicode_frame", location: "end-j" } }),
  T(7, "k_beta", "node:N2", "kN", true, { basis_ref: [], metadata: "flat" }),
  T("", "k_alpha", "emoji:😀", "N·m", null, { source_result_refs: "refs" }),
  { id: "no-value", kind: "K_ALPHA", entity_ref: "node:N3", unit: "mm", basis_ref: { ref_type: "load_case" }, metadata: { coordinate_system: "pipe_section" } },
  T("big", "k_beta", "node:N3", "mm", 123456789012345680000, { metadata: { location: "midspan" } }),
  T("tiny", "k_beta", "node:N3", "mm", 5e-324, { basis_ref: { ref_type: "load_case", ref_id: "L-1" } }),
  T("arr", "k_beta", "node:N4", "mm", [1, null, [2, 3]]),
  T("obj", "k_beta", "node:N4", "mm", { a: 1 }),
  T("ΣΊΣΥΦΟΣ", "k_delta", "node:N5", "MPa", 0.1, { basis_ref: { ref_type: "combination", ref_id: "C-1" }, metadata: { coordinate_system: "global", basis: "explicit_user_range_envelope" } }),
  T("dup", "k_alpha", "node:N6", "mm", 2.5, { source_result_refs: [] }),
  T("NaN-ish", 5, "node:N6", 3, -1.5e-300, { metadata: { component: 1, location: 0 } })
];
const TEXT_QUERIES = [
  "ς", "σ", "σίσυφος", "οδος", "i̇", "straße", "STRASSE", "1e+21", "1e-7", " 0 ", "123456789012345680000", "5e-324", "-1.5e-300", "true",
  "1,,2,3", "[object object]", "text value", "x,y", "r e f s", "src:a", "load_case", "7", "😀", "n·m", "  MPA ", " mpa ", "﻿mpa", "k_alpha",
  "end-j", "ünicode", "undefined"
];

function invented(): [string, CaseText, string[], boolean][] {
  const recovery = {
    results: [T("r1", "k", "node:A", "mm", 1)],
    contract_evidence: { exact_cases: [{ load_case_id: "L-1" }, { load_case_id: "L-2" }, 5, { load_case_id: "" }] },
    source_block_recovery: {
      receipt_sha256: "abc",
      body: {
        status: "partial", receipt_version: 2, publication_sha256: "def",
        cases: [
          { basis_ref: { ref_id: "L-1" }, selected_method: "m1", outcome: "qualified", rows: [1, 2, 3] },
          "not a record",
          { basis_ref: { ref_id: "L-1" }, selected_method: null },
          { basis_ref: [1], outcome: 3 },
          { basis_ref: { ref_id: "L-2" }, selected_method: "", outcome: "refused" }
        ]
      }
    }
  };
  return [
    ["invented/detail_text", inline({ document_kind: "invented", results: TEXT_ROWS }), TEXT_QUERIES, false],
    ["invented/detail_null_row", inline({ results: [T("a", "k", "node:A", "mm", 1), null, T("b", "k", "node:B", "mm", 2)] }), ["a"], false],
    ["invented/detail_refs_not_iterable", inline({ results: [T("b", "k", "node:B", "mm", 2), T("a", "k", "node:A", "mm", 1, { source_result_refs: 7 })] }), ["a", "b"], false],
    ["invented/detail_empty", inline({ results: [] }), [], false],
    ["invented/detail_no_results", inline({ document_kind: "invented" }), [], false],
    // Raw texts: a duplicated member (the last is read) and whitespace around the rows.
    ["invented/detail_duplicate_results", { text: '{"results":[{"id":"x","kind":"k"},{"id":"y","kind":"k"}],"results":[ {"id":"z","kind":"k","value":1E+2} ]}' }, ["z", "100"], false],
    ["invented/detail_whitespace", { text: '{\n  "results" : [\n    {"id" : "a", "kind": "k", "value": 1.50},\n\t{"id":"b","kind":"k","value":-0.0e0}  ,  "s" , [ ]\n  ]\n}\n' }, ["1.5", "0"], false],
    ["invented/detail_recovery", inline(recovery), [], true],
    ["invented/detail_recovery_null", inline({ results: [], source_block_recovery: null }), [], true],
    ["invented/detail_recovery_no_body", inline({ source_block_recovery: { receipt_sha256: "x" }, contract_evidence: { exact_cases: [] } }), [], true]
  ];
}

/** Every case, in the committed order. */
export function resultDetailCases(): DetailCase[] {
  const seen = new Set<string>();
  const results: [string, CaseText, string[], boolean][] = [];
  for (const summary of resultSummaryCases()) {
    const key = JSON.stringify(summary.result);
    if (seen.has(key)) continue;
    seen.add(key);
    results.push([summary.name.replace(/#.*$/, ""), summary.result, [], summary.name.startsWith("pressure_source/")]);
  }
  for (const name of ["load_reference_source/fields-dense_scrutiny", "load_reference_source/n06-sparse_interactive"]) {
    results.push([`pressure_source/${name}`, file(`fixtures/product_preview/pressure_source/${name}.raw.json`), [], true]);
  }
  results.push(...invented());
  return results.map(([name, result, extraText, recovers]) => {
    const source = JSON.parse(caseText(result));
    const rows: Json[] = Array.isArray(source?.results) ? source.results : [];
    return { name, result, queries: queries(rows, extraText), lookups: lookups(rows), recoveries: recovers ? RECOVERIES : [RECOVERIES[0]] };
  });
}

/** The cases file's text, as committed. */
export function resultDetailCasesText(): string {
  return `${JSON.stringify({
    document_kind: "swbpipe.test.result_detail_cases",
    note: "Written by resultDetailParity.test.ts (RESULT_DETAIL_CASES_WRITE=1) from test-support/resultDetailCases.ts; read by result_detail_tests.rs.",
    cases: resultDetailCases()
  }, null, 1)}\n`;
}
