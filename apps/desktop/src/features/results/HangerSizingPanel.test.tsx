// DEL-04-11 increment 3b: the hanger-sizing table component on the committed family fixtures
// (Design `hanger-sizing.md` §8.1, §8.3), with the inputs' libraries. Read-only rendering of 3a's
// display model; its CSV controls save the adapters' files.
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import twoSpanJson from "../../../../../fixtures/results/hanger_sizing_result_two_span.json";
import fourSpanJson from "../../../../../fixtures/results/hanger_sizing_result_four_span.json";
import refusedJson from "../../../../../fixtures/results/hanger_sizing_result_refused.json";
import twoSpanInput from "../../../../../fixtures/results/hanger_sizing_input_two_span.json";
import fourSpanInput from "../../../../../fixtures/results/hanger_sizing_input_four_span.json";
import exPressurisedJson from "../../../../../fixtures/results/hanger_sizing_result_ex_pressurised.json";
import exPressurisedInput from "../../../../../fixtures/results/hanger_sizing_input_ex_pressurised.json";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import { CSV_SOFTWARE_VERSION } from "../result-export/csvProvenance";
import { serializeCsvTable } from "../result-export/csvTable";
import { preambleValue, readCsv } from "../../test/csvRead";
import type { HangerSizingDocument, HangerSizingSizedDocument } from "./hangerSizingResult";
import { hangerSizingCandidatesCsv, hangerSizingCsvProvenance, hangerSizingSolvesCsv } from "./hangerSizingCsv";
import { HANGER_SIZING_STANDING_NAMES, hangerSizingView, type HangerSizingReaderEvidence } from "./hangerSizingView";
import {
  HANGER_SIZING_CANDIDATES_TABLE_LABEL,
  HANGER_SIZING_CONDITIONAL_TEXT,
  HANGER_SIZING_SOLVES_TABLE_LABEL,
  HANGER_SIZING_UNCONDITIONAL_TEXT,
  HangerSizingPanel
} from "./HangerSizingPanel";

const saved: Array<{ text: string; fileName: string; explicitLocalPrivateIntent: boolean }> = [];
vi.mock("../result-export/csvSave", () => ({
  saveCsvTable: vi.fn(async (input: { text: string; fileName: string; explicitLocalPrivateIntent: boolean }) => {
    saved.push(input);
    return { mode: "browser", file_name: input.fileName, byte_count: new TextEncoder().encode(input.text).length };
  })
}));
afterEach(() => { cleanup(); saved.length = 0; });

const clone = <T,>(value: T): T => structuredClone(value);
const asDocument = (value: unknown) => clone(value) as HangerSizingDocument;
const twoSpan = () => asDocument(twoSpanJson);
const fourSpan = () => asDocument(fourSpanJson);
const refused = () => asDocument(refusedJson);
const library = () => clone(twoSpanInput.library) as Record<string, unknown>;
const fourLibrary = () => clone(fourSpanInput.library) as Record<string, unknown>;

const verdict = (): HangerSizingReaderEvidence["verdict"] => ({
  closure: { status: "passed", difference: null },
  binding: { status: "passed", difference: null, not_checked: ["solve_request"] },
  arithmetic_consistent_with_published_positions: { status: "passed", difference: null },
  derivation: { status: "not_checked", difference: null },
  reproduction: { status: "failed", difference: "V_O1: reproduced value differs" }
});

async function renderSized(props: Parameters<typeof HangerSizingPanel>[0]) {
  const view = render(<HangerSizingPanel {...props} />);
  await screen.findByTestId("hanger-sizing-sized");
  return view;
}

const byId = (id: string) => screen.getByTestId(`hanger-sizing-${id}`);

describe("HangerSizingPanel: the candidate table", () => {
  it("labels F_cold and F_C exactly and shows the published SI values as 3a gives them", async () => {
    await renderSized({ document: twoSpan(), library: library() });
    const table = screen.getByRole("table", { name: HANGER_SIZING_CANDIDATES_TABLE_LABEL });
    const headers = within(table).getAllByRole("columnheader").map((cell) => cell.textContent);
    expect(headers).toContain("F_cold: installed setting (at drawn position)");
    expect(headers).toContain("F_C: cold-condition load");
    const rows = within(table).getAllByRole("row").slice(1);
    expect(rows).toHaveLength(1);
    expect(within(rows[0]).getByRole("rowheader").textContent).toBe("support:B");
    expect(byId("f-cold-support:B").textContent).toBe("5197.458880641297 N");
    expect(byId("f-c-support:B").textContent).toBe("5197.458880641297 N");
    expect(byId("outcome-support:B").textContent).toBe("selected");
    expect(rows[0].textContent).toContain("5.942062022110673e-42 m");
    expect(rows[0].textContent).toContain("[4600 N, 5715.934209026074 N]");
    expect(rows[0].textContent).toContain("200000 N/m");
    expect(byId("per-case-support:B").textContent).toBe(
      "load:O1: u_i 0.000987294403206484 m; F_i 5000 N; v_i 0.03949177612825936"
      + "load:O2: u_i -0.0005923766419238903 m; F_i 5315.934209026074 N; v_i 0.022286831199606543"
    );
    expect(byId("record-support:B").textContent).toBe("invented.hangers.sizing; variable_spring; VS-200-L; Invented VS-200-L; variable_spring_hanger");
    expect(byId("record-ranges-support:B").textContent).toBe("[4000 N, 6000 N]; travel 50 mm");
  });

  it("shows a null variation as no ratio formed, never 0", async () => {
    const document = twoSpan() as unknown as { result: { candidates: { verification: { operating: { variation: number | null }[] } }[] } };
    document.result.candidates[0].verification.operating[1].variation = null;
    await renderSized({ document: document as unknown as HangerSizingDocument });
    const verification = byId("detail-verification-support:B").textContent;
    expect(verification).toContain("load:O2: u_i -0.00014515303489761239 m; F_i 5226.489487620819 N; v_i no ratio formed");
    expect(verification).toContain("load:O1: u_i 0.0009872944032064837 m; F_i 5000 N; v_i 0.028311185952602402");
  });

  it("shows the constant, fallback and no-fit rows with nothing put in place of a missing selection (four_span)", async () => {
    await renderSized({ document: fourSpan(), library: fourLibrary() });
    const table = screen.getByRole("table", { name: HANGER_SIZING_CANDIDATES_TABLE_LABEL });
    expect(within(table).getAllByRole("rowheader").map((cell) => cell.textContent)).toEqual(["support:B", "support:C", "support:D"]);
    expect(["support:B", "support:C", "support:D"].map((support) => byId(`outcome-${support}`).textContent))
      .toEqual(["selected_by_fallback", "selected", "no_record_fits"]);
    expect(byId("f-cold-support:B").textContent).toBe("not applicable: constant effort");
    expect(byId("row-support:B").textContent).toContain("4571.428571428572 N (constant load 4571.428571428572 N)");
    expect(byId("record-support:C").textContent).toContain("CE-1");
    for (const id of ["f-cold-support:D", "f-c-support:D", "record-support:D", "record-ranges-support:D"]) expect(byId(id).textContent, id).toBe("no fit");
    expect(byId("per-case-support:D").textContent).toBe("load:O1: u_i 0.0043440953741085316 m; F_i no fit; v_i no fit");
    expect(byId("detail-verification-support:D").textContent).toBe("no fit");
    expect(byId("detail-failed-tests-support:D").textContent).toBe("no fit");
    expect(byId("detail-assumption-support:D").textContent).toBe("force: 4571.428571428572 N");
    expect(byId("detail-record-support:D").textContent).toBe("no fit: no record is selected");
  });

  it("shows null failure counts as not tested, and a fallback's two families' counts", async () => {
    await renderSized({ document: fourSpan() });
    expect(byId("detail-counts-variable-support:B").textContent).toBe("E1 1, E2 1, V0 0, L 3, T 1, V 0");
    expect(byId("detail-counts-constant-support:B").textContent).toBe("E1 0, L 1, T 1");
    expect(byId("detail-counts-variable-support:C").textContent).toBe("not tested");
    expect(byId("detail-counts-constant-support:C").textContent).toBe("E1 0, L 2, T 0");
    expect(byId("detail-counts-constant-support:D").textContent).toBe("not tested");
  });

  it("lists each candidate's details: assumption, verification, failed tests, governing cases, span and travel", async () => {
    await renderSized({ document: twoSpan(), library: library() });
    const detail = byId("detail-support:B");
    expect(detail.tagName).toBe("DETAILS");
    expect(within(detail).getByText("Details: support:B (selected)").tagName).toBe("SUMMARY");
    expect(byId("detail-assumption-support:B").textContent).toBe("spring: k 200000 N/m; F_cold (installed setting (at drawn position)) 5197.458880641297 N");
    expect(byId("detail-verification-support:B").textContent).toContain("F_C (cold-condition load) 5141.555929763012 N; departure 0 N");
    expect(byId("detail-verification-support:B").textContent).toContain("span [-0.00014515303489761239 m, 0.0009872944032064837 m]; required travel 0.0051324474381040965 m; T pass: yes");
    expect(byId("detail-failed-tests-support:B").textContent).toBe("none");
    expect(detail.textContent).toContain("travel load:O1; variation load:O1");
    expect(detail.textContent).toContain("[-0.0005923766419238903 m, 0.000987294403206484 m]; required travel 0.005579671045130375 m");
    expect(byId("detail-record-support:B").textContent).toContain("Invented VS-200-L (variable_spring_hanger)");
    expect(byId("detail-record-support:B").textContent).toContain("record sha256 008305de2dab833901ab4d9d4bd7bf29f7399670f12c9cedf7714502bb1683de");
    expect(byId("detail-record-support:B").textContent).toContain("k 200000 N/m");
  });
});

describe("HangerSizingPanel: library gate", () => {
  it("names the gate's reason instead of any record detail", async () => {
    for (const [lib, reason] of [[undefined, "library_not_supplied"], [{ hanger_records: [] }, "library_hash_mismatch"]] as const) {
      await renderSized({ document: twoSpan(), library: lib });
      expect(byId("basis-library-gate").textContent).toBe(`not shown: ${reason}`);
      expect(byId("record-support:B").textContent).toBe(`invented.hangers.sizing; variable_spring; VS-200-L; record details not shown: ${reason}`);
      expect(byId("record-ranges-support:B").textContent).toBe(`not shown: ${reason}`);
      expect(byId("detail-record-support:B").textContent).toBe(`VS-200-L: record details not shown: ${reason}`);
      expect(screen.queryByText(/Invented VS-200-L/)).toBeNull();
      cleanup();
    }
  });

  it("names library_id_mismatch and record_hash_mismatch", async () => {
    const renamed = twoSpan() as unknown as { inputs: { library: { library_id: string } } };
    renamed.inputs.library.library_id = "another.library";
    await renderSized({ document: renamed as unknown as HangerSizingDocument, library: library() });
    expect(byId("basis-library-gate").textContent).toBe("not shown: library_id_mismatch");
    cleanup();
    const mismatched = twoSpan() as unknown as { result: { candidates: { selected: { record_sha256: string } }[] } };
    mismatched.result.candidates[0].selected.record_sha256 = "0".repeat(64);
    await renderSized({ document: mismatched as unknown as HangerSizingDocument, library: library() });
    expect(byId("basis-library-gate").textContent).toMatch(/^shown from the supplied library invented\.hangers\.sizing/);
    expect(byId("detail-record-support:B").textContent).toBe("VS-200-L: record details not shown: record_hash_mismatch");
  });
});

describe("HangerSizingPanel: basis, solves, standing and disclosures", () => {
  it("gives every request input, the bound hashes, route, profile, cases and the document's own strings verbatim", async () => {
    const document = twoSpan() as HangerSizingSizedDocument;
    await renderSized({ document });
    expect(byId("basis-route").textContent).toBe("pressure_free");
    expect(byId("basis-profile").textContent).toBe("openpipestress_jcs_ijson_v1");
    expect(byId("basis-library-id").textContent).toBe("invented.hangers.sizing");
    expect(byId("basis-library-sha256").textContent).toBe("e904866d00f0b665b9f4d7fc078b95fd583418d5b6174d09eb3bca0adfdc7861");
    expect(byId("basis-solve-request-sha256").textContent).toBe("966be96d808dae54a17143264c4648477647ad76c2b5d4ef44c61212bbbe6ac4");
    expect(byId("basis-sizing-request-sha256").textContent).toBe(document.inputs.sizing_request_sha256);
    const request = byId("basis-request").textContent;
    for (const text of ["load:W", "own_solve", "load:O1", "support:B: variable", "hot", "operating_load", "smallest_capacity", "dense_scrutiny"]) expect(request).toContain(text);
    expect(byId("basis-check-cases").textContent).toBe("load:O2");
    expect(byId("basis-variation-limit").textContent).toBe("0.25");
    expect(byId("basis-travel-allowance").textContent).toBe("0.002 m");
    expect(byId("basis-case-load:W").textContent).toBe("load:W: role weight; load categories weight; uses an operating-condition set: no");
    expect(byId("basis-case-load:O2").textContent).toBe("load:O2: role operating; load categories concentrated_force, weight; uses an operating-condition set: no");
    expect([...byId("basis-statement").querySelectorAll("li")].map((item) => item.textContent)).toEqual(twoSpanJson.basis_statement);
    expect([...byId("limitations").querySelectorAll("li")].map((item) => item.textContent)).toEqual(twoSpanJson.limitations);
  });

  it("reads the exact route from a committed exact-route document and renders its table", async () => {
    const document = asDocument(exPressurisedJson) as HangerSizingSizedDocument;
    await renderSized({ document, library: clone(exPressurisedInput.library) as Record<string, unknown> });
    expect(byId("basis-route").textContent).toBe("exact_pressure");
    const table = screen.getByRole("table", { name: HANGER_SIZING_CANDIDATES_TABLE_LABEL });
    const rows = within(table).getAllByRole("row").slice(1);
    expect(rows.map((row) => within(row).getByRole("rowheader").textContent)).toEqual(["H-3", "H-5"]);
    expect(byId("outcome-H-3").textContent).toBe("selected");
    expect(byId("outcome-H-5").textContent).toBe("selected");
    expect(byId("record-H-3").textContent).toContain("VS-B");
    expect(byId("record-H-5").textContent).toContain("CE-1");
    expect([...byId("limitations").querySelectorAll("li")].map((item) => item.textContent)).toEqual(exPressurisedJson.limitations);
    expect(screen.getByRole("table", { name: HANGER_SIZING_SOLVES_TABLE_LABEL })).toBeTruthy();
  });

  it("lists every solve with its label, case, derived hash and each nonlinear support's state", async () => {
    await renderSized({ document: twoSpan() });
    const table = screen.getByRole("table", { name: HANGER_SIZING_SOLVES_TABLE_LABEL });
    expect(within(table).getAllByRole("rowheader").map((cell) => cell.textContent)).toEqual(["S_W", "S_C", "S_O1", "S_O2", "V_C", "V_O1", "V_O2"]);
    expect([...byId("solve-S_O2").querySelectorAll("td")].map((cell) => cell.textContent))
      .toEqual(["load:O2", "590df0f06c498e3e017fcf697b75b6e2be034bf4ad931367b3b37731f7e4a615", "support:C-lift: active"]);
    cleanup();
    await renderSized({ document: fourSpan() });
    expect(byId("solve-S_W").textContent).toContain("no nonlinear support");
  });

  it("states a conditional standing prominently, with what it means and the candidates with no fit", async () => {
    await renderSized({ document: fourSpan() });
    const standing = byId("verification-standing");
    expect(standing).toHaveAttribute("role", "note");
    expect(standing.querySelector("strong")?.textContent).toBe(HANGER_SIZING_CONDITIONAL_TEXT);
    expect(standing.textContent).toContain("Candidates with no fit: support:D.");
    expect(byId("sized")).toHaveAttribute("data-standing", "conditional");
    cleanup();
    await renderSized({ document: twoSpan() });
    expect(byId("verification-standing").textContent).toBe(HANGER_SIZING_UNCONDITIONAL_TEXT);
    expect(byId("disclosures").textContent).toContain("No disclosures.");
  });

  it("lists disclosures with their code, names and message", async () => {
    const document = twoSpan() as unknown as { result: { disclosures: unknown[] } };
    document.result.disclosures = [{ code: "HANGER_SIZING_SUPPORT_STATE_DIFFERS", names: ["support:C-lift"], message: "invented message" }];
    await renderSized({ document: document as unknown as HangerSizingDocument });
    expect(byId("disclosure-0").textContent).toBe("HANGER_SIZING_SUPPORT_STATE_DIFFERS (support:C-lift): invented message");
  });
});

describe("HangerSizingPanel: reader standings", () => {
  it("shows bound evidence exactly as 3a returns it", async () => {
    const document = twoSpan();
    const hash = await canonicalSha256HexCheckedV1(document);
    await renderSized({ document, evidence: { document_sha256: hash, verdict: verdict() } });
    expect(byId("standings-binding").textContent).toBe(`Reader evidence is bound to this document by its checked hash (sha256 ${hash}).`);
    expect(byId("standing-closure").textContent).toBe("closure: passed");
    expect(byId("standing-binding").textContent).toBe("binding: passed; inputs not checked: solve_request");
    expect(byId("standing-derivation").textContent).toBe("derivation: not checked");
    expect(byId("standing-reproduction").textContent).toBe("reproduction: failed (first difference: V_O1: reproduced value differs)");
  });

  it("shows the unbound reason and every standing as not checked for absent or mismatched evidence", async () => {
    const twoHash = await canonicalSha256HexCheckedV1(twoSpan());
    for (const [evidence, reason] of [
      [undefined, "evidence_not_supplied"],
      [{ document_sha256: twoHash, verdict: verdict() }, "evidence_bound_to_other_document"]
    ] as const) {
      await renderSized({ document: fourSpan(), evidence });
      expect(byId("standings-binding").textContent).toBe(`Reader evidence not bound: ${reason}. Every standing reads not checked.`);
      for (const name of HANGER_SIZING_STANDING_NAMES) expect(byId(`standing-${name}`).textContent).toBe(`${name}: not checked`);
      expect(byId("standings").textContent).not.toMatch(/passed|valid|accepted/i);
      cleanup();
    }
  });
});

describe("HangerSizingPanel: refused documents", () => {
  it("lists every refusal in order with the basis, and renders no table and no export", async () => {
    const document = refused() as unknown as Record<string, unknown> & { refusals: unknown[] };
    document.refusals = [
      { code: "HANGER_SIZING_CANDIDATE_UNKNOWN", names: ["support:missing"], message: "the candidate does not meet the sizing rules of Design §3" },
      { code: "HANGER_SIZING_LIBRARY_NOT_ADMITTED", names: [], message: "the library is not admitted" }
    ];
    render(<HangerSizingPanel document={document as unknown as HangerSizingDocument} />);
    await screen.findByTestId("hanger-sizing-refused");
    expect(byId("refusal-0").textContent).toBe("HANGER_SIZING_CANDIDATE_UNKNOWN (support:missing): the candidate does not meet the sizing rules of Design §3");
    expect(byId("refusal-1").textContent).toBe("HANGER_SIZING_LIBRARY_NOT_ADMITTED: the library is not admitted");
    expect(screen.queryAllByRole("table")).toHaveLength(0);
    expect(screen.queryAllByRole("button", { name: /^Export CSV/ })).toHaveLength(0);
    expect(byId("basis-sizing-request-sha256").textContent).toBe("b5343f6afddd1716902e57ce0c59359768ecd1ce89442b460e2a6fdde2c40b47");
    expect(byId("standings-binding").textContent).toBe("Reader evidence not bound: evidence_not_supplied. Every standing reads not checked.");
  });

  it("shows null bound inputs as not published", async () => {
    const document = refused() as unknown as Record<string, unknown> & { inputs: Record<string, unknown> };
    document.inputs.sizing_request = null;
    document.inputs.sizing_request_sha256 = null;
    document.inputs.solve_request = { sha256: null };
    document.inputs.library = { library_id: null, sha256: null };
    render(<HangerSizingPanel document={document as unknown as HangerSizingDocument} />);
    await screen.findByTestId("hanger-sizing-refused");
    for (const id of ["basis-library-id", "basis-library-sha256", "basis-solve-request-sha256", "basis-sizing-request-sha256"]) expect(byId(id).textContent, id).toBe("not published");
    expect(byId("basis-request").textContent).toBe("Sizing request: not published (the input was not admitted).");
  });
});

describe("HangerSizingPanel: accessibility and CSV controls", () => {
  it("gives both tables accessible names, column and row headers, and a named Export CSV control each", async () => {
    await renderSized({ document: fourSpan(), library: fourLibrary() });
    const tables = screen.getAllByRole("table");
    expect(tables.map((table) => table.getAttribute("aria-label"))).toEqual([HANGER_SIZING_CANDIDATES_TABLE_LABEL, HANGER_SIZING_SOLVES_TABLE_LABEL]);
    for (const table of tables) {
      expect(table).toHaveAttribute("data-csv-export", "hanger-sizing");
      for (const header of within(table).getAllByRole("columnheader")) expect(header).toHaveAttribute("scope", "col");
      expect(within(table).getAllByRole("rowheader").length).toBeGreaterThan(0);
    }
    expect(screen.getByRole("region", { name: "Hanger sizing" })).toBeTruthy();
    expect(screen.getByRole("region", { name: "Sizing basis" })).toBeTruthy();
    expect(screen.getByRole("region", { name: "Reader standings" })).toBeTruthy();
    expect(screen.getAllByRole("button", { name: /^Export CSV/ }).map((button) => button.getAttribute("aria-label")))
      .toEqual(["Export CSV: Hanger sizing candidates", "Export CSV: Hanger sizing derived solves"]);
  });

  it("saves each table's adapter file with explicit local-private intent", async () => {
    const document = fourSpan() as HangerSizingSizedDocument;
    const modelHash = `sha256:${"cd".repeat(32)}`;
    await renderSized({ document, library: fourLibrary(), modelHash });
    fireEvent.click(screen.getByRole("button", { name: "Export CSV: Hanger sizing candidates" }));
    fireEvent.click(screen.getByRole("button", { name: "Export CSV: Hanger sizing derived solves" }));
    await waitFor(() => expect(saved.length).toBe(2));
    const view = await hangerSizingView(document, undefined, fourLibrary());
    if (view.status !== "sized") throw new Error("expected a sized view");
    const context = { view, provenance: await hangerSizingCsvProvenance({ document, modelHash }), softwareVersion: CSV_SOFTWARE_VERSION };
    const byName = new Map(saved.map((item) => [item.fileName, item]));
    expect([...byName.keys()].sort()).toEqual(["swbpipe-hanger-sizing-solves.csv", "swbpipe-hanger-sizing.csv"]);
    expect(byName.get("swbpipe-hanger-sizing.csv")!.text).toBe(serializeCsvTable(hangerSizingCandidatesCsv(context).table));
    expect(byName.get("swbpipe-hanger-sizing-solves.csv")!.text).toBe(serializeCsvTable(hangerSizingSolvesCsv(context).table));
    for (const item of saved) {
      expect(item.explicitLocalPrivateIntent).toBe(true);
      const csv = readCsv(item.text);
      expect(preambleValue(csv, "model_hash")).toBe(modelHash);
      expect(preambleValue(csv, "result_hash")).toBe(`sha256:${await canonicalSha256HexCheckedV1(document)}`);
    }
    await waitFor(() => expect(byId("export-csv-status").textContent).toMatch(/^Downloaded swbpipe-hanger-sizing\.csv \(\d+ bytes\)\.$/));
  });
});
