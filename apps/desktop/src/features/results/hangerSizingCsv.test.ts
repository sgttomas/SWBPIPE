// DEL-04-11 increment 3b: the hanger-sizing CSV adapters on the committed family fixtures
// (Design `hanger-sizing.md` §8.1; DEL-08-04 Design `csv-export.md` §3). Exact text for
// two_span; the absent-value statuses of four_span; the provenance and the file names.
import { describe, expect, it } from "vitest";
import twoSpanJson from "../../../../../fixtures/results/hanger_sizing_result_two_span.json";
import fourSpanJson from "../../../../../fixtures/results/hanger_sizing_result_four_span.json";
import twoSpanInput from "../../../../../fixtures/results/hanger_sizing_input_two_span.json";
import fourSpanInput from "../../../../../fixtures/results/hanger_sizing_input_four_span.json";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import { CSV_SOFTWARE_VERSION } from "../result-export/csvProvenance";
import { CSV_BOM, CSV_NOT_RECORDED, CSV_PREAMBLE_NOTICE, isCsvExportFileName, serializeCsvTable } from "../result-export/csvTable";
import { expectNoEmptyNumericField, preambleValue, readCsv, textField, type ReadCsv } from "../../test/csvRead";
import type { HangerSizingDocument, HangerSizingSizedDocument } from "./hangerSizingResult";
import {
  HANGER_SIZING_CONDITIONAL_TEXT,
  HANGER_SIZING_CSV_KIND,
  HANGER_SIZING_CSV_UNIT_BASIS,
  HANGER_SIZING_UNCONDITIONAL_TEXT,
  hangerSizingCandidatesCsv,
  hangerSizingCsvProvenance,
  hangerSizingSolvesCsv
} from "./hangerSizingCsv";
import { hangerSizingView, type HangerSizingSizedView } from "./hangerSizingView";

const clone = <T,>(value: T): T => structuredClone(value);
const twoSpan = () => clone(twoSpanJson) as unknown as HangerSizingSizedDocument;
const fourSpan = () => clone(fourSpanJson) as unknown as HangerSizingSizedDocument;

/** The committed two_span document's checked canonical hash. */
const TWO_SPAN_SHA256 = "a0d3764ae80d968cebd91c953412f287d869fba96dd631e671a29a3b4b653338";

async function sized(document: HangerSizingDocument, library?: unknown): Promise<HangerSizingSizedView> {
  const view = await hangerSizingView(document, null, library);
  if (view.status !== "sized") throw new Error("expected a sized view");
  return view;
}

async function exports(document: HangerSizingSizedDocument, library?: unknown, modelHash?: string | null) {
  const view = await sized(document, library);
  const context = { view, provenance: await hangerSizingCsvProvenance({ document, modelHash }), softwareVersion: CSV_SOFTWARE_VERSION };
  return { candidates: hangerSizingCandidatesCsv(context), solves: hangerSizingSolvesCsv(context) };
}

const crlf = (lines: readonly string[]) => lines.map((line) => `${line}\r\n`).join("");

function preamble(resultHash: string, filter: string, modelHash = CSV_NOT_RECORDED): string[] {
  return [
    `notice,"${CSV_PREAMBLE_NOTICE}"`,
    "export_context,local_private",
    "released_report,false",
    "table,hanger-sizing",
    `software_version,${CSV_SOFTWARE_VERSION}`,
    `model_hash,${modelHash}`,
    `result_hash,${resultHash}`,
    "analysis_route,openpipestress.result_semantics/hanger-sizing-1",
    `case_or_filter,${filter}`,
    `unit_basis,"${HANGER_SIZING_CSV_UNIT_BASIS}"`,
    ""
  ];
}

const CANDIDATE_HEADER = [
  "Support", "Node", "DOF", "Family policy", "Outcome", "Operating case", "Selected family", "Hanger id", "Record sha256", "Record name", "Record type",
  "Record working load min", "Record working load min unit", "Record working load max", "Record working load max unit",
  "Record travel range", "Record travel range unit", "Record stiffness", "Record stiffness unit",
  "W [N]", "F_cold: installed setting (at drawn position) [N]", "F_C: cold-condition load [N]", "k [N/m]", "Envelope low [N]", "Envelope high [N]", "Constant load [N]",
  "u_C [m]", "u_i [m]", "F_i [N]", "v_i [1]", "Span min [m]", "Span max [m]", "Required travel [m]", "Governing travel case", "Governing variation case",
  "Verification assumption", "Assumed k [N/m]", "Assumed F_cold: installed setting (at drawn position) [N]", "Assumed force [N]",
  "Verified u_C [m]", "Verified u_i [m]", "Verified F_i [N]", "Verified v_i [1]", "Verified span min [m]", "Verified span max [m]", "Verified required travel [m]",
  "Verified T pass", "Verified F_C: cold-condition load [N]", "Departure [N]", "Failed tests",
  "Variable records failing E1 [1]", "Variable records failing E2 [1]", "Variable records failing V0 [1]", "Variable records failing L [1]",
  "Variable records failing T [1]", "Variable records failing V [1]",
  "Constant records failing E1 [1]", "Constant records failing L [1]", "Constant records failing T [1]",
  "Verification standing", "Verification standing meaning", "Disclosures"
];
/** The qualification cells an unconditional document without disclosures carries on every row. */
const UNCONDITIONAL_CELLS = ["unconditional", `"${HANGER_SIZING_UNCONDITIONAL_TEXT}"`, "none"];

const RECORD = "008305de2dab833901ab4d9d4bd7bf29f7399670f12c9cedf7714502bb1683de";

function twoSpanRow(caseId: string, u: string, f: string, v: string, uv: string, fv: string, vv: string, record = "Invented VS-200-L,variable_spring_hanger,4000,N,6000,N,50,mm,200000,N/m"): string {
  return [
    "support:B", "node:B", "UZ", "variable", "selected", caseId, "variable_spring", "VS-200-L", RECORD, record,
    "5000", "5197.458880641297", "5197.458880641297", "200000", "4600", "5715.934209026074", "not applicable: variable spring",
    "5.942062022110673e-42", u, f, v, "-0.0005923766419238903", "0.000987294403206484", "0.005579671045130375", "load:O1", "load:O1",
    "spring", "200000", "5197.458880641297", "not applicable: spring assumption",
    "0.00027951475439142365", uv, fv, vv, "-0.00014515303489761239", "0.0009872944032064837", "0.0051324474381040965",
    "true", "5141.555929763012", "0", "none",
    "1", "1", "0", "1", "1", "0",
    "not tested", "not tested", "not tested",
    ...UNCONDITIONAL_CELLS
  ].join(",");
}

const rowFor = (csv: ReadCsv, support: string, caseId: string) => {
  const found = csv.rows.find((row) => textField(csv, row, "Support") === support && textField(csv, row, "Operating case") === caseId);
  if (!found) throw new Error(`no row ${support} ${caseId}`);
  return found;
};

describe("hanger-sizing candidates CSV", () => {
  it("writes the exact two_span text: preamble, fixed-unit headers, unrounded numbers and status cells", async () => {
    const document = twoSpan();
    expect(await canonicalSha256HexCheckedV1(document)).toBe(TWO_SPAN_SHA256);
    const { candidates } = await exports(document, clone(twoSpanInput.library));
    expect(candidates.fileName).toBe("swbpipe-hanger-sizing.csv");
    expect(serializeCsvTable(candidates.table)).toBe(CSV_BOM + crlf([
      ...preamble(`sha256:${TWO_SPAN_SHA256}`, "all candidates"),
      CANDIDATE_HEADER.join(","),
      twoSpanRow("load:O1", "0.000987294403206484", "5000", "0.03949177612825936", "0.0009872944032064837", "5000", "0.028311185952602402"),
      twoSpanRow("load:O2", "-0.0005923766419238903", "5315.934209026074", "0.022286831199606543", "-0.00014515303489761239", "5226.489487620819", "0.016250593837216405")
    ]));
  });

  it("parses every numeric field back to the published number exactly, and leaves no field empty", async () => {
    const document = twoSpan();
    const { candidates } = await exports(document, clone(twoSpanInput.library));
    const csv = readCsv(serializeCsvTable(candidates.table));
    expectNoEmptyNumericField(csv);
    for (const row of csv.rows) for (const field of row) expect(field).not.toBe("");
    const candidate = document.result.candidates[0];
    if (candidate.selected?.family !== "variable_spring" || candidate.verification?.family !== "variable_spring") throw new Error("variable expected");
    const o2 = rowFor(csv, "support:B", "load:O2");
    const at = (label: string) => Number(o2[csv.header.indexOf(label)]);
    expect(Object.is(at("u_C [m]"), candidate.u_cold_m)).toBe(true);
    expect(Object.is(at("F_i [N]"), candidate.selected.operating[1].f_n)).toBe(true);
    expect(Object.is(at("Verified u_i [m]"), candidate.verification.operating[1].u_m)).toBe(true);
    expect(Object.is(at("Verified v_i [1]"), candidate.verification.operating[1].variation)).toBe(true);
  });

  it("writes a null verified variation as no ratio formed, never 0", async () => {
    const document = twoSpan() as unknown as { result: { candidates: { verification: { operating: { variation: number | null }[] } }[] } };
    document.result.candidates[0].verification.operating[1].variation = null;
    const { candidates } = await exports(document as unknown as HangerSizingSizedDocument);
    const csv = readCsv(serializeCsvTable(candidates.table));
    expect(textField(csv, rowFor(csv, "support:B", "load:O2"), "Verified v_i [1]")).toBe("no ratio formed");
    expect(textField(csv, rowFor(csv, "support:B", "load:O1"), "Verified v_i [1]")).toBe("0.028311185952602402");
  });

  it("names the library gate's reason in the record cells when the library is not bound", async () => {
    const two = readCsv(serializeCsvTable((await exports(twoSpan())).candidates.table));
    expect(textField(two, two.rows[0], "Record name")).toBe("not shown: library_not_supplied");
    expect(textField(two, two.rows[0], "Record type")).toBe("not shown: library_not_supplied");
    const changed = clone(twoSpanInput.library) as { hanger_records: { name: string }[] };
    changed.hanger_records[0].name = "Changed elsewhere";
    const other = readCsv(serializeCsvTable((await exports(twoSpan(), changed)).candidates.table));
    expect(textField(other, other.rows[0], "Record name")).toBe("not shown: library_hash_mismatch");
    // The published identifiers stay: only the library's own details are withheld.
    expect(textField(other, other.rows[0], "Hanger id")).toBe("VS-200-L");
  });

  it("states four_span's absent values: constant selection, fallback, no fit and null counts", async () => {
    const { candidates } = await exports(fourSpan(), clone(fourSpanInput.library));
    const text = serializeCsvTable(candidates.table);
    const csv = readCsv(text);
    expectNoEmptyNumericField(csv);
    for (const row of csv.rows) for (const field of row) expect(field).not.toBe("");
    expect(csv.rows.map((row) => [textField(csv, row, "Support"), textField(csv, row, "Operating case"), textField(csv, row, "Outcome")])).toEqual([
      ["support:B", "load:O1", "selected_by_fallback"],
      ["support:C", "load:O1", "selected"],
      ["support:D", "load:O1", "no_record_fits"]
    ]);
    const field = (support: string, label: string) => textField(csv, rowFor(csv, support, "load:O1"), label);
    // The fallback: a constant selection, both families' counts.
    expect(field("support:B", "Selected family")).toBe("constant_effort");
    expect(field("support:B", "Constant load [N]")).toBe("4571.428571428572");
    for (const label of ["F_cold: installed setting (at drawn position) [N]", "F_C: cold-condition load [N]", "k [N/m]", "Envelope low [N]", "F_i [N]", "v_i [1]",
      "Governing variation case", "Verified F_i [N]", "Verified v_i [1]", "Verified F_C: cold-condition load [N]", "Departure [N]"]) {
      expect(field("support:B", label), label).toBe("not applicable: constant effort");
    }
    expect(field("support:B", "Verification assumption")).toBe("force");
    expect(field("support:B", "Assumed k [N/m]")).toBe("not applicable: force assumption");
    expect(field("support:B", "Assumed force [N]")).toBe("4571.428571428572");
    expect(field("support:B", "u_C [m]")).toBe("1.3469085432095433e-18");
    expect(field("support:B", "Variable records failing L [1]")).toBe("3");
    expect(field("support:B", "Constant records failing L [1]")).toBe("1");
    expect(field("support:B", "Record name")).not.toMatch(/^not shown/);
    // A null family count is "not tested", distinct from 0.
    expect(field("support:C", "Variable records failing E1 [1]")).toBe("not tested");
    expect(field("support:C", "Constant records failing E1 [1]")).toBe("0");
    // No fit: the outcome, and nothing put in place of a selection or verification.
    for (const label of ["Selected family", "Hanger id", "Record sha256", "Record name", "Record type", "F_cold: installed setting (at drawn position) [N]",
      "Constant load [N]", "F_i [N]", "v_i [1]", "Governing travel case", "Governing variation case", "Verified u_C [m]", "Verified u_i [m]",
      "Verified span min [m]", "Verified required travel [m]", "Verified T pass", "Failed tests"]) {
      expect(field("support:D", label), label).toBe("no fit");
    }
    expect(field("support:D", "W [N]")).toBe("4571.428571428572");
    expect(field("support:D", "u_i [m]")).toBe("0.0043440953741085316");
    expect(field("support:D", "Assumed force [N]")).toBe("4571.428571428572");
    expect(field("support:D", "Constant records failing T [1]")).toBe("not tested");
    expect(field("support:D", "Variable records failing L [1]")).toBe("3");
    expect(text).not.toMatch(/certif|approv|complian|seal/i);
  });
});

describe("hanger-sizing CSV: document-level qualifications and library quantities", () => {
  it("carries the conditional standing, its meaning and the no-fit candidates on every row of both files (four_span)", async () => {
    const { candidates, solves } = await exports(fourSpan(), clone(fourSpanInput.library));
    const meaning = `${HANGER_SIZING_CONDITIONAL_TEXT} Candidates with no fit: support:D.`;
    for (const table of [candidates.table, solves.table]) {
      const csv = readCsv(serializeCsvTable(table));
      expect(csv.rows.length).toBeGreaterThan(0);
      for (const row of csv.rows) {
        expect(textField(csv, row, "Verification standing")).toBe("conditional");
        expect(textField(csv, row, "Verification standing meaning")).toBe(meaning);
        expect(textField(csv, row, "Disclosures")).toBe("none");
      }
    }
  });

  it("carries every disclosure, with its code, names and message, on every row of both files", async () => {
    const document = twoSpan() as unknown as { result: { disclosures: { code: string; names: string[]; message: string }[] } };
    document.result.disclosures = [
      { code: "HANGER_SIZING_COLD_REFERENCE_STATE_DIFFERS", names: ["support:C-lift"], message: "the state differs in S_C" },
      { code: "HANGER_SIZING_SUPPORT_STATE_DIFFERS", names: ["support:C-lift"], message: "the state differs in S_O1" }
    ];
    const { candidates, solves } = await exports(document as unknown as HangerSizingSizedDocument);
    const expected = "HANGER_SIZING_COLD_REFERENCE_STATE_DIFFERS (support:C-lift): the state differs in S_C | HANGER_SIZING_SUPPORT_STATE_DIFFERS (support:C-lift): the state differs in S_O1";
    for (const table of [candidates.table, solves.table]) {
      const csv = readCsv(serializeCsvTable(table));
      for (const row of csv.rows) expect(textField(csv, row, "Disclosures")).toBe(expected);
    }
  });

  it("exports the bound record's ranges in the record's own units, unconverted, and states each absent case", async () => {
    const bound = readCsv(serializeCsvTable((await exports(twoSpan(), clone(twoSpanInput.library))).candidates.table));
    const row = bound.rows[0];
    expect([textField(bound, row, "Record working load min"), textField(bound, row, "Record working load min unit")]).toEqual(["4000", "N"]);
    expect([textField(bound, row, "Record travel range"), textField(bound, row, "Record travel range unit")]).toEqual(["50", "mm"]);
    const unbound = readCsv(serializeCsvTable((await exports(twoSpan())).candidates.table));
    for (const label of ["Record working load max", "Record working load max unit", "Record stiffness", "Record stiffness unit"]) {
      expect(textField(unbound, unbound.rows[0], label), label).toBe("not shown: library_not_supplied");
    }
    const four = readCsv(serializeCsvTable((await exports(fourSpan(), clone(fourSpanInput.library))).candidates.table));
    const field = (support: string, label: string) => textField(four, rowFor(four, support, "load:O1"), label);
    expect(field("support:C", "Record stiffness")).toBe("not applicable: constant effort");
    expect(field("support:C", "Record stiffness unit")).toBe("not applicable: constant effort");
    expect(field("support:C", "Record working load min")).not.toMatch(/^not /);
    expect(field("support:D", "Record travel range")).toBe("no fit");
    expect(field("support:D", "Record travel range unit")).toBe("no fit");
  });

  it("attaches per-case values by case id, not by list position", async () => {
    const document = twoSpan() as unknown as { result: { candidates: { selected: { operating: { case: string; f_n: number }[] }; verification: { operating: { case: string; u_m: number }[] } }[] } };
    const candidate = document.result.candidates[0];
    const f = Object.fromEntries(candidate.selected.operating.map((entry) => [entry.case, entry.f_n]));
    const u = Object.fromEntries(candidate.verification.operating.map((entry) => [entry.case, entry.u_m]));
    candidate.selected.operating.reverse();
    candidate.verification.operating.reverse();
    const csv = readCsv(serializeCsvTable((await exports(document as unknown as HangerSizingSizedDocument)).candidates.table));
    for (const caseId of ["load:O1", "load:O2"]) {
      const row = rowFor(csv, "support:B", caseId);
      expect(Number(row[csv.header.indexOf("F_i [N]")])).toBe(f[caseId]);
      expect(Number(row[csv.header.indexOf("Verified u_i [m]")])).toBe(u[caseId]);
    }
    expect(f["load:O1"]).not.toBe(f["load:O2"]);
  });
});

describe("hanger-sizing solves CSV", () => {
  it("writes the exact two_span text: one row per solve and nonlinear support", async () => {
    const { solves } = await exports(twoSpan());
    expect(solves.fileName).toBe("swbpipe-hanger-sizing-solves.csv");
    const document = twoSpan();
    expect(serializeCsvTable(solves.table)).toBe(CSV_BOM + crlf([
      ...preamble(`sha256:${TWO_SPAN_SHA256}`, "all solves"),
      "Solve,Case,Derived document sha256,Nonlinear support,State,Verification standing,Verification standing meaning,Disclosures",
      ...document.result.solves.map((solve) => [solve.label, solve.case, solve.derived_document_sha256, "support:C-lift", "active", ...UNCONDITIONAL_CELLS].join(","))
    ]));
    expect(document.result.solves.map((solve) => solve.label)).toEqual(["S_W", "S_C", "S_O1", "S_O2", "V_C", "V_O1", "V_O2"]);
  });

  it("states a solve of a model with no nonlinear support (four_span)", async () => {
    const csv = readCsv(serializeCsvTable((await exports(fourSpan())).solves.table));
    expect(csv.rows.map((row) => row[0])).toEqual(["S_W", "S_C", "S_O1", "V_C", "V_O1"]);
    for (const row of csv.rows) expect(row.slice(3)).toEqual(["no nonlinear support", "no nonlinear support"]);
    expect(csv.rows[2].slice(0, 3)).toEqual(["S_O1", "load:O1", "00e44382ceae8d90e5f4d3cd234489c8867e8279c1ee1acdaddac43386cfce5c"]);
  });
});

describe("hanger-sizing CSV provenance and file names", () => {
  it("states a supplied model hash, and not_recorded for none or a value that is not a hash", async () => {
    const hash = `sha256:${"ab".repeat(32)}`;
    const supplied = readCsv(serializeCsvTable((await exports(twoSpan(), undefined, hash)).candidates.table));
    expect(preambleValue(supplied, "model_hash")).toBe(hash);
    for (const absent of [undefined, null, "not a hash"]) {
      const csv = readCsv(serializeCsvTable((await exports(twoSpan(), undefined, absent)).solves.table));
      expect(preambleValue(csv, "model_hash")).toBe(CSV_NOT_RECORDED);
    }
    expect((await hangerSizingCsvProvenance({ document: twoSpan(), modelHash: "ab".repeat(32) })).modelHash).toBe(`sha256:${"ab".repeat(32)}`);
  });

  it("binds result_hash to the exact document, and states not_recorded when it cannot be hashed", async () => {
    const changed = twoSpan() as unknown as { result: { candidates: { w_n: number }[] } };
    changed.result.candidates[0].w_n = 5000.000000000001;
    const provenance = await hangerSizingCsvProvenance({ document: changed as unknown as HangerSizingSizedDocument });
    expect(provenance.resultHash).toBe(`sha256:${await canonicalSha256HexCheckedV1(changed)}`);
    expect(provenance.resultHash).not.toBe(`sha256:${TWO_SPAN_SHA256}`);
    changed.result.candidates[0].w_n = Number.NaN;
    expect((await hangerSizingCsvProvenance({ document: changed as unknown as HangerSizingSizedDocument })).resultHash).toBe(CSV_NOT_RECORDED);
    expect(provenance.analysisRoutes).toEqual(["openpipestress.result_semantics/hanger-sizing-1"]);
  });

  it("suggests names under the export file-name rule", async () => {
    const { candidates, solves } = await exports(fourSpan());
    expect(HANGER_SIZING_CSV_KIND).toBe("hanger-sizing");
    expect([candidates.fileName, solves.fileName]).toEqual(["swbpipe-hanger-sizing.csv", "swbpipe-hanger-sizing-solves.csv"]);
    for (const name of [candidates.fileName, solves.fileName]) expect(isCsvExportFileName(name)).toBe(true);
    expect(candidates.table.preamble.table).toBe("hanger-sizing");
    expect(solves.table.preamble.table).toBe("hanger-sizing");
  });
});
