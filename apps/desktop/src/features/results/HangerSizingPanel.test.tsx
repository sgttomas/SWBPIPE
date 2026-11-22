// DEL-04-11 increment 3b: the hanger-sizing table component on the committed family fixtures
// (Design `hanger-sizing.md` §8.1, §8.3), with the inputs' libraries. Read-only rendering of 3a's
// display model; its CSV controls save the adapters' files.
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import twoSpanJson from "../../../../../fixtures/results/hanger_sizing_result_two_span.json";
import fourSpanJson from "../../../../../fixtures/results/hanger_sizing_result_four_span.json";
import refusedJson from "../../../../../fixtures/results/hanger_sizing_result_refused.json";
import corpusJson from "../../../../../fixtures/results/hanger_sizing_corpus.json";
import twoSpanInput from "../../../../../fixtures/results/hanger_sizing_input_two_span.json";
import fourSpanInput from "../../../../../fixtures/results/hanger_sizing_input_four_span.json";
import exPressurisedJson from "../../../../../fixtures/results/hanger_sizing_result_ex_pressurised.json";
import exPressurisedInput from "../../../../../fixtures/results/hanger_sizing_input_ex_pressurised.json";
import { canonicalSha256HexCheckedV1 } from "../../services/hashService";
import { CSV_SOFTWARE_VERSION } from "../result-export/csvProvenance";
import { serializeCsvTable } from "../result-export/csvTable";
import { preambleValue, readCsv } from "../../test/csvRead";
import type { HangerSizingDisclosure, HangerSizingDisclosureCode, HangerSizingDocument, HangerSizingSizedDocument } from "./hangerSizingResult";
import { hangerSizingCandidatesCsv, hangerSizingCsvProvenance, hangerSizingSolvesCsv } from "./hangerSizingCsv";
import { CONFIRMATION_STATEMENTS } from "./integrityStatus";
import { BOUNDARY_SENTENCE } from "./numericalHealthText";
import {
  HANGER_SIZING_CONFIRMATION_NOT_APPLICABLE,
  HANGER_SIZING_STANDING_NAMES,
  formatHangerSizingSolverStanding,
  hangerSizingView,
  integrityDisclosureMessage,
  type HangerSizingReaderEvidence
} from "./hangerSizingView";
import {
  HANGER_SIZING_CANDIDATES_TABLE_LABEL,
  HANGER_SIZING_CONDITIONAL_TEXT,
  HANGER_SIZING_SOLVES_TABLE_LABEL,
  HANGER_SIZING_UNCONDITIONAL_TEXT,
  HangerSizingPanel
} from "./HangerSizingPanel";

// Compile-time controls on the disclosure-code union (types only; the readers validate documents): every
// code the schema admits is a member, and an unknown code is not.
const DISCLOSURE_CODES_TYPED: readonly HangerSizingDisclosureCode[] = [
  "HANGER_SIZING_SUPPORT_STATE_DIFFERS",
  "HANGER_SIZING_COLD_REFERENCE_STATE_DIFFERS",
  "HANGER_SIZING_DERIVED_SOLVE_INTEGRITY",
  "DISPLACEMENT_CORRECTION_UNAVAILABLE"
];
// @ts-expect-error an unknown disclosure code is not a member of the union.
const UNKNOWN_DISCLOSURE_CODE: HangerSizingDisclosureCode = "HANGER_SIZING_UNKNOWN_DISCLOSURE";
void DISCLOSURE_CODES_TYPED;
void UNKNOWN_DISCLOSURE_CODE;

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
  reproduction: { status: "failed", difference: "V_O1: reproduced value differs" },
  integrity_bound: { status: "not_verified", reason: "inverse_norm_bound_not_verified" }
});

async function renderSized(props: Parameters<typeof HangerSizingPanel>[0]) {
  const view = render(<HangerSizingPanel {...props} />);
  await screen.findByTestId("hanger-sizing-sized");
  return view;
}

const byId = (id: string) => screen.getByTestId(`hanger-sizing-${id}`);
const SENSITIVE_MESSAGE = "sensitive: the solver's numerical-integrity status for this derived solve is sensitive, not solver_checks_passed; every value sizing takes from it carries that standing (integrity_bound: not_verified, inverse_norm_bound_not_verified)";

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
    expect(within(table).getAllByRole("columnheader").map((cell) => cell.textContent))
      .toEqual(["Solve", "Case", "Derived document sha256", "Nonlinear support states", "Solver standing", "Support-state confirmation"]);
    expect([...byId("solve-S_O2").querySelectorAll("td")].map((cell) => cell.textContent))
      .toEqual([
        "load:O2", "590df0f06c498e3e017fcf697b75b6e2be034bf4ad931367b3b37731f7e4a615", "support:C-lift: active",
        formatHangerSizingSolverStanding("solver_checks_passed"), CONFIRMATION_STATEMENTS.solver_reported_established
      ]);
    cleanup();
    await renderSized({ document: fourSpan() });
    expect(byId("solve-S_W").textContent).toContain("no nonlinear support");
    expect(byId("solve-S_W-standing").textContent).toBe(formatHangerSizingSolverStanding("solver_checks_passed"));
    expect(byId("solve-S_W-confirmation").textContent).toBe(HANGER_SIZING_CONFIRMATION_NOT_APPLICABLE);
  });

  it("shows a non-passing solve's standing and a quoted confirmation detail beside the solve", async () => {
    const document = twoSpan() as unknown as { result: { solves: Record<string, unknown>[]; disclosures: unknown[] } };
    document.result.disclosures = [{ code: "HANGER_SIZING_DERIVED_SOLVE_INTEGRITY", names: ["S_O1"], message: SENSITIVE_MESSAGE }];
    document.result.solves[2].solver_standing = "sensitive";
    document.result.solves[4].support_state_confirmation = { outcome: "not_established_detail_quoted", detail: "margin below its bound" };
    await renderSized({ document: document as unknown as HangerSizingDocument });
    expect(byId("solve-S_O1-standing").textContent).toBe(formatHangerSizingSolverStanding("sensitive"));
    expect(byId("solve-V_C-confirmation").textContent).toBe(`${CONFIRMATION_STATEMENTS.not_established_detail_quoted} margin below its bound`);
    expect(byId("solve-V_C-confirmation-detail").tagName).toBe("Q");
    expect(byId("solve-V_C-confirmation-detail").textContent).toBe("margin below its bound");
    expect(screen.queryByTestId("hanger-sizing-solve-S_O2-confirmation-detail")).toBeNull();
  });

  it("renders no sized view of a document whose bound, standing or confirmation is tampered", async () => {
    const tampered: [string, (d: { result: { integrity_bound: unknown; solves: Record<string, unknown>[] } }) => void, string][] = [
      ["bound verified", (d) => { d.result.integrity_bound = { status: "verified", reason: "inverse_norm_bound_not_verified" }; }, "HANGER_SIZING_INTEGRITY_BOUND_MALFORMED"],
      ["standing verified", (d) => { d.result.solves[0].solver_standing = "verified"; }, "HANGER_SIZING_SOLVE_STANDING_MALFORMED: S_W"],
      ["standing undisclosed", (d) => { d.result.solves[0].solver_standing = "failed"; }, "HANGER_SIZING_SOLVE_STANDING_UNDISCLOSED: S_W"],
      ["confirmation spurious detail", (d) => { d.result.solves[0].support_state_confirmation = { outcome: "no_confirmation_record", detail: "invented" }; }, "HANGER_SIZING_SOLVE_CONFIRMATION_MALFORMED: S_W"]
    ];
    for (const [name, change, fault] of tampered) {
      const document = twoSpan() as unknown as { result: { integrity_bound: unknown; solves: Record<string, unknown>[] } };
      change(document);
      render(<HangerSizingPanel document={document as unknown as HangerSizingDocument} />);
      expect((await screen.findByTestId("hanger-sizing-error")).textContent, name).toBe(`The sizing result could not be displayed: ${fault}`);
      expect(screen.queryByTestId("hanger-sizing-sized"), name).toBeNull();
      expect(screen.queryByTestId("hanger-sizing-integrity-bound"), name).toBeNull();
      cleanup();
    }
  });

  it("renders no sized view of a document whose integrity disclosures are orphaned, multi-label, duplicated or forged", async () => {
    type Disclosing = { result: { solves: Record<string, unknown>[]; disclosures: unknown[] } };
    const own = { code: "HANGER_SIZING_DERIVED_SOLVE_INTEGRITY", names: ["S_W"], message: SENSITIVE_MESSAGE };
    const tampered: [string, unknown[], string][] = [
      ["orphan label", [own, { ...own, names: ["S_O99"] }], "HANGER_SIZING_SOLVE_INTEGRITY_DISCLOSURE_MALFORMED: disclosures[1]"],
      ["multi-label", [{ ...own, names: ["S_W", "S_C"] }], "HANGER_SIZING_SOLVE_INTEGRITY_DISCLOSURE_MALFORMED: disclosures[0]"],
      ["duplicate", [own, own], "HANGER_SIZING_SOLVE_INTEGRITY_DISCLOSURE_DUPLICATE: S_W"],
      ["forged message", [{ ...own, message: "sensitive: a verified error bound for this solve" }], "HANGER_SIZING_SOLVE_INTEGRITY_DISCLOSURE_MALFORMED: disclosures[0]"]
    ];
    for (const [name, disclosures, fault] of tampered) {
      const document = twoSpan() as unknown as Disclosing;
      document.result.solves[0].solver_standing = "sensitive";
      document.result.disclosures = disclosures;
      render(<HangerSizingPanel document={document as unknown as HangerSizingDocument} />);
      expect((await screen.findByTestId("hanger-sizing-error")).textContent, name).toBe(`The sizing result could not be displayed: ${fault}`);
      expect(screen.queryByTestId("hanger-sizing-sized"), name).toBeNull();
      cleanup();
    }
  });

  it("renders no sized view of a solve reported established beside its unavailable correction, and renders the withdrawn one", async () => {
    // S_O1 (solves[2]) sensitive with its integrity and correction-unavailable disclosures: the producer withdraws a
    // confirmed state when the correction fails, so solver_reported_established is a contradiction.
    const withS_O1 = (outcome: string) => {
      const document = twoSpan() as unknown as { result: { solves: Record<string, unknown>[]; disclosures: unknown[] } };
      document.result.solves[2].solver_standing = "sensitive";
      document.result.disclosures = [
        { code: "HANGER_SIZING_DERIVED_SOLVE_INTEGRITY", names: ["S_O1"], message: SENSITIVE_MESSAGE },
        { code: "DISPLACEMENT_CORRECTION_UNAVAILABLE", names: ["S_O1"], message: "Load case load:OPERATING: the corrected displacement of node:A:UX could not be certified (invented); no published result of this case relies on the member-relative formation check." }
      ];
      document.result.solves[2].support_state_confirmation = { outcome, detail: null };
      return document as unknown as HangerSizingDocument;
    };
    render(<HangerSizingPanel document={withS_O1("solver_reported_established")} />);
    expect((await screen.findByTestId("hanger-sizing-error")).textContent)
      .toBe("The sizing result could not be displayed: HANGER_SIZING_SOLVE_CONFIRMATION_CONTRADICTION: S_O1");
    expect(screen.queryByTestId("hanger-sizing-sized")).toBeNull();
    expect(screen.queryByRole("button", { name: /^Export CSV/ })).toBeNull();
    cleanup();
    await renderSized({ document: withS_O1("no_confirmation_record") });
    expect(byId("solve-S_O1-standing").textContent).toBe(formatHangerSizingSolverStanding("sensitive"));
    fireEvent.click(screen.getByRole("button", { name: "Export CSV: Hanger sizing candidates" }));
    fireEvent.click(screen.getByRole("button", { name: "Export CSV: Hanger sizing derived solves" }));
    await waitFor(() => expect(saved.length).toBe(2));
    const solves = readCsv(saved.find((item) => item.fileName === "swbpipe-hanger-sizing-solves.csv")!.text);
    expect(solves.rows[2][solves.header.indexOf("Support-state confirmation")]).toBe(CONFIRMATION_STATEMENTS.no_confirmation_record);
  });

  it("shows a sensitive weight solve with the engine's exact integrity message, disclosed and not refused, and exports it", async () => {
    expect(integrityDisclosureMessage("sensitive")).toBe(SENSITIVE_MESSAGE);
    const document = twoSpan() as unknown as { result: { solves: Record<string, unknown>[]; disclosures: unknown[] } };
    document.result.solves[0].solver_standing = "sensitive";
    document.result.disclosures = [{ code: "HANGER_SIZING_DERIVED_SOLVE_INTEGRITY", names: ["S_W"], message: integrityDisclosureMessage("sensitive") }];
    await renderSized({ document: document as unknown as HangerSizingDocument });
    expect(byId("solve-S_W-standing").textContent).toBe(formatHangerSizingSolverStanding("sensitive"));
    expect(byId("solve-S_C-standing").textContent).toBe(formatHangerSizingSolverStanding("solver_checks_passed"));
    expect(byId("solve-S_C-standing").textContent).toContain(BOUNDARY_SENTENCE);
    expect(byId("disclosure-0").textContent).toBe(`HANGER_SIZING_DERIVED_SOLVE_INTEGRITY (S_W): ${SENSITIVE_MESSAGE}`);
    fireEvent.click(screen.getByRole("button", { name: "Export CSV: Hanger sizing candidates" }));
    fireEvent.click(screen.getByRole("button", { name: "Export CSV: Hanger sizing derived solves" }));
    await waitFor(() => expect(saved.length).toBe(2));
    const solves = readCsv(saved.find((item) => item.fileName === "swbpipe-hanger-sizing-solves.csv")!.text);
    expect(solves.rows[0][solves.header.indexOf("Solver standing")]).toBe(formatHangerSizingSolverStanding("sensitive"));
    for (const item of saved) expect(item.text).toContain(`HANGER_SIZING_DERIVED_SOLVE_INTEGRITY (S_W): ${SENSITIVE_MESSAGE}`);
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

  it("lists each derived solve's integrity and correction-unavailable disclosures by solve label", async () => {
    // The messages are the product's own forms (the integrity message of a sensitive standing; the
    // DISPLACEMENT_CORRECTION_UNAVAILABLE warning of the forced-failure fixture rt_a3_s6_c1_correction_unavailable).
    const disclosures: HangerSizingDisclosure[] = [
      { code: "HANGER_SIZING_DERIVED_SOLVE_INTEGRITY", names: ["S_O1"], message: "sensitive: the solver's numerical-integrity status for this derived solve is sensitive, not solver_checks_passed; every value sizing takes from it carries that standing (integrity_bound: not_verified, inverse_norm_bound_not_verified)" },
      { code: "DISPLACEMENT_CORRECTION_UNAVAILABLE", names: ["S_O1"], message: "Load case load:VACUUM: the corrected displacement of node:A:UX could not be certified (forced by test); no published result of this case relies on the member-relative formation check." },
      { code: "HANGER_SIZING_DERIVED_SOLVE_INTEGRITY", names: ["V_O1"], message: "sensitive: the solver's numerical-integrity status for this derived solve is sensitive, not solver_checks_passed; every value sizing takes from it carries that standing (integrity_bound: not_verified, inverse_norm_bound_not_verified)" }
    ];
    const document = twoSpan() as unknown as { result: { disclosures: unknown[]; solves: Record<string, unknown>[] } };
    document.result.disclosures = disclosures;
    // Each integrity disclosure states its solve's own standing, and S_O1's confirmation is withdrawn beside its
    // unavailable correction, as the producer withdraws it.
    for (const label of ["S_O1", "V_O1"]) document.result.solves.find((solve) => solve.label === label)!.solver_standing = "sensitive";
    document.result.solves[2].support_state_confirmation = { outcome: "no_confirmation_record", detail: null };
    await renderSized({ document: document as unknown as HangerSizingDocument });
    expect(byId("disclosure-0").textContent).toBe("HANGER_SIZING_DERIVED_SOLVE_INTEGRITY (S_O1): sensitive: the solver's numerical-integrity status for this derived solve is sensitive, not solver_checks_passed; every value sizing takes from it carries that standing (integrity_bound: not_verified, inverse_norm_bound_not_verified)");
    expect(byId("disclosure-1").textContent).toBe("DISPLACEMENT_CORRECTION_UNAVAILABLE (S_O1): Load case load:VACUUM: the corrected displacement of node:A:UX could not be certified (forced by test); no published result of this case relies on the member-relative formation check.");
    expect(byId("disclosure-2").textContent).toBe("HANGER_SIZING_DERIVED_SOLVE_INTEGRITY (V_O1): sensitive: the solver's numerical-integrity status for this derived solve is sensitive, not solver_checks_passed; every value sizing takes from it carries that standing (integrity_bound: not_verified, inverse_norm_bound_not_verified)");
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
    expect(byId("standings-integrity-bound").textContent).toBe("Integrity bound: not_verified (inverse_norm_bound_not_verified).");
    expect(byId("integrity-bound").textContent).toBe(
      `Integrity bound: not_verified (inverse_norm_bound_not_verified) — SWBPIPE does not verify an error bound for the ordinary derived solves (LIM-026). ${BOUNDARY_SENTENCE}`
    );
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
      expect(screen.queryByTestId("hanger-sizing-standings-integrity-bound")).toBeNull();
      expect(byId("integrity-bound").textContent).toContain("not_verified (inverse_norm_bound_not_verified)");
      cleanup();
    }
  });
});

describe("HangerSizingPanel: refused documents", () => {
  it("lists every refusal in order with the basis, and renders no table and no export", async () => {
    const document = refused() as unknown as Record<string, unknown> & { refusals: unknown[] };
    document.refusals = [
      { code: "HANGER_SIZING_CANDIDATE_UNKNOWN", names: ["support:missing"], message: "the named support is not a support of the model" },
      { code: "HANGER_SIZING_LIBRARY_NOT_ADMITTED", names: [], message: "the library is not admitted" }
    ];
    render(<HangerSizingPanel document={document as unknown as HangerSizingDocument} />);
    await screen.findByTestId("hanger-sizing-refused");
    expect(byId("refusal-0").textContent).toBe("HANGER_SIZING_CANDIDATE_UNKNOWN (support:missing): the named support is not a support of the model");
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

  it("shows the null route of a refused document whose input was not admitted as not published", async () => {
    // The committed corpus entry: the refused fixture with the corpus's replacements applied.
    const entry = corpusJson.entries.find((candidate) => candidate.id === "refused-route-null-not-admitted");
    if (!entry) throw new Error("corpus entry refused-route-null-not-admitted is missing");
    const document = refused() as unknown as Record<string, unknown>;
    for (const mutation of entry.mutations) {
      expect(mutation.op).toBe("replace");
      if (!("value" in mutation)) throw new Error("a replace mutation carries its value");
      const parts = mutation.path.split("/").slice(1);
      let parent = document as Record<string, unknown>;
      for (const part of parts.slice(0, -1)) parent = parent[part] as Record<string, unknown>;
      parent[parts[parts.length - 1]] = clone(mutation.value);
    }
    expect(document.route).toBeNull();
    const view = await hangerSizingView(document as unknown as HangerSizingDocument);
    expect(view.basis.route).toBeNull();
    render(<HangerSizingPanel document={document as unknown as HangerSizingDocument} />);
    await screen.findByTestId("hanger-sizing-refused");
    expect(byId("basis-route").textContent).toBe("not published");
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

  it("exports the provenance of the document as the view read it, whatever the document undergoes after rendering", async () => {
    const document = twoSpan();
    render(<HangerSizingPanel document={document} />);
    // Changed synchronously after rendering, while the view awaits its hashes; the export clicks come after.
    (document as unknown as { result: { integrity_bound: { status: string } } }).result.integrity_bound.status = "verified";
    (document as unknown as { semantic_contract_id: string }).semantic_contract_id = "mutated";
    await screen.findByTestId("hanger-sizing-sized");
    expect(byId("basis-document").textContent).toContain("(openpipestress.result_semantics/hanger-sizing-1)");
    fireEvent.click(screen.getByRole("button", { name: "Export CSV: Hanger sizing candidates" }));
    fireEvent.click(screen.getByRole("button", { name: "Export CSV: Hanger sizing derived solves" }));
    await waitFor(() => expect(saved.length).toBe(2));
    for (const item of saved) {
      const csv = readCsv(item.text);
      expect(preambleValue(csv, "result_hash"), item.fileName).toBe("sha256:0264fd8e5d22c82815486fd143ae6c4cc6fcd5da6da12c3a156e7b39acc4133a");
      expect(preambleValue(csv, "analysis_route"), item.fileName).toBe("openpipestress.result_semantics/hanger-sizing-1");
      for (const row of csv.rows) expect(row[csv.header.indexOf("Integrity bound")], item.fileName).toBe("not_verified (inverse_norm_bound_not_verified)");
    }
  });
});
