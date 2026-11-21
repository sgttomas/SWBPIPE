// Release exit criterion 18 (LIM-026): the one predicate that decides whether a solver text shown to
// a person needs "Reported by the solver; not verified by SWBPIPE." beside it, and its closed table.
// The table (`solverTextClaims.json`) classifies every code of the repository's diagnostic code table
// (`fixtures/diagnostics/exact_route_codes.json`, written by tools/validation/diagnostic_codes.py from
// the solve crates' sources); a code added there and not classified here fails the first test.
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import claims from "./solverTextClaims.json";
import { limitationsWithNotes, needsSolverReportNote, SOLVER_CLAIM_PHRASES, SOLVER_REPORT_NOTE, solverTextClaim, withLimitationsNote } from "./integrityStatus";
import { ELIGIBLE_RECOVERY_TEXT } from "./numericalHealthText";

const root = resolve(__dirname, "../../../../..");
const json = <T,>(path: string) => JSON.parse(readFileSync(resolve(root, path), "utf8")) as T;
const CODE_TABLE = json<{ codes: { code: string; messages: string[] }[] }>("fixtures/diagnostics/exact_route_codes.json");

/** Every `{code, message}` of every result document under a committed fixture tree. */
function publishedDiagnostics(): { code: string; message: string; file: string }[] {
  const out: { code: string; message: string; file: string }[] = [];
  const visit = (value: unknown, file: string) => {
    if (Array.isArray(value)) { for (const item of value) visit(item, file); return; }
    if (!value || typeof value !== "object") return;
    const record = value as Record<string, unknown>;
    if (Array.isArray(record.diagnostics) && "results" in record) {
      for (const d of record.diagnostics as { code?: unknown; message?: unknown }[]) {
        if (typeof d?.code === "string" && typeof d.message === "string") out.push({ code: d.code, message: d.message, file });
      }
    }
    for (const item of Object.values(record)) visit(item, file);
  };
  for (const file of [
    "fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json",
    "fixtures/results/preview_physics_connected_sparse.json",
    "fixtures/results/rt_a3_s6_c1_correction_unavailable.json",
    "fixtures/results/combination_certificate_corpus.json",
    "fixtures/results/preview_physics_nonlinear_operands_corpus.json",
    "fixtures/results/preview_physics_held_status_corpus.json",
    "fixtures/results/retained_precision_cases.json",
    "fixtures/results/invented/agent_solve_witness/blocked_retained_evaluation-sparse_interactive.raw.json",
    "fixtures/product_preview/pressure_source/physics_source/mixed-sparse_interactive.raw.json",
    "fixtures/product_preview/pressure/load_reference/pressure-sparse_interactive.raw.json"
  ]) visit(json(file), file);
  return out;
}

describe("the closed classification of the solver's codes", () => {
  it("classifies every code of the diagnostic code table once, and names no other", () => {
    const table = CODE_TABLE.codes.map((c) => c.code);
    const classified = [...Object.keys(claims.claim), ...Object.keys(claims.quotes_claim), ...claims.no_claim];
    expect(table.filter((code) => solverTextClaim(code) === null), "codes of the table with no classification").toEqual([]);
    expect(classified.filter((code) => !table.includes(code)), "classified codes the table does not have").toEqual([]);
    expect(new Set(classified).size, "each code in one class only").toBe(classified.length);
  });

  it("holds the claims the census found, by code", () => {
    for (const code of ["NUMERICAL_INTEGRITY_CHECKS_PASSED", "COMBINATION_ROWS_WITHIN_STATED_BOUND", "STRESS_FREE_WITHIN_NUMERICAL_PRECISION", "EXACT_COMBINATION_DERIVED_FROM_CASE_RESULTS"]) expect(solverTextClaim(code), code).toBe("claim");
    for (const code of ["NUMERICAL_INTEGRITY_SENSITIVE", "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED", "NUMERICAL_INTEGRITY_INSPECTION_ONLY", "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED"]) expect(solverTextClaim(code), code).toBe("quotes_claim");
    // Another certificate's own texts: the retained-precision route and the pressure-source recovery.
    for (const code of ["RETAINED_PRECISION_SELECTED", "SOURCE_BLOCK_RECOVERY_SELECTED", "COMBINATION_ROWS_NOT_ASSESSED", "COMBINATION_OPERAND_CHECKS_NOT_PASSED"]) expect(solverTextClaim(code), code).toBe("no_claim");
  });

  it("finds no claim phrase in a published message of a code it classifies as stating none", () => {
    const found = publishedDiagnostics();
    expect(found.length).toBeGreaterThan(100);
    const missed = found.filter((d) => solverTextClaim(d.code) === "no_claim" && SOLVER_CLAIM_PHRASES.some((phrase) => d.message.includes(phrase)));
    expect(missed.map((d) => `${d.code} (${d.file})`)).toEqual([]);
    // And every published message of a claim code is noted.
    for (const d of found.filter((x) => solverTextClaim(x.code) === "claim")) expect(needsSolverReportNote(d), d.code).toBe(true);
  });
});

describe("needsSolverReportNote", () => {
  const STRESS_FREE = "Load case case:A: every element-local member force row and support force row is bounded by 1.2e-12 N and every element-local member moment row and support moment row by 3.4e-12 N*m at 128-bit precision; the case is stress-free to within numerical precision. The standing remains NUMERICAL_INTEGRITY_SENSITIVE.";
  const WITHIN = "combination comb:o-s: 9 derived row group(s) are established within their stated bound against the operand cases' intended systems; 0 are not established (the bound exceeds the 1e-9 criterion, as where the operands nearly cancel, or the comparison is unavailable); 19 are not assessed (below the floor); 0 of its rows are covered by no group and remain inspection results.";
  const SLIDING = "Load case load:VACUUM: ... observed maximum only, exact represented maximum not claimed; general-energy historical alias is residual work, not total energy balance; separate zero count/cap/contact/sliding checks passed The formation check ...";

  it("notes each claim the census found, by code or by the solver's phrase", () => {
    expect(needsSolverReportNote({ code: "NUMERICAL_INTEGRITY_CHECKS_PASSED", message: "x" })).toBe(true);
    expect(needsSolverReportNote({ code: "COMBINATION_ROWS_WITHIN_STATED_BOUND", message: WITHIN })).toBe(true);
    expect(needsSolverReportNote({ code: "STRESS_FREE_WITHIN_NUMERICAL_PRECISION", message: STRESS_FREE })).toBe(true);
    expect(needsSolverReportNote({ code: "NUMERICAL_INTEGRITY_SENSITIVE", message: SLIDING })).toBe(true);
    expect(needsSolverReportNote({ code: "NUMERICAL_INTEGRITY_SENSITIVE", message: "… Converged support state: established at 128-bit precision by one formation-checked solve; …" })).toBe(true);
    expect(needsSolverReportNote({ code: "NUMERICAL_INTEGRITY_INSPECTION_ONLY", message: "Inspection only (prior_outcome=NUMERICAL_INTEGRITY_CHECKS_PASSED): blocked" })).toBe(true);
    expect(needsSolverReportNote({ code: "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED", message: "requested_mode=sparse_interactive; …; retained_source_attempt=not_required_ordinary_checks_passed" })).toBe(true);
    expect(needsSolverReportNote({ code: "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED", message: "requested_mode=sparse_interactive; …; retained_source_attempt=unavailable" })).toBe(false);
    // A text with no code: a limitation, a held-status outcome, a confirmation's quoted detail.
    expect(needsSolverReportNote({ message: "Member actions … of a load case reported NUMERICAL_INTEGRITY_CHECKS_PASSED are recovered at 128-bit precision." })).toBe(true);
    expect(needsSolverReportNote({ message: "Established within its stated numerical bound for the explicitly held system at this case's loads. The verdict does not establish this case's own support state." })).toBe(true);
    expect(needsSolverReportNote(null)).toBe(false);
  });

  it("notes no denial, and nothing another certificate establishes", () => {
    expect(needsSolverReportNote({ code: "NUMERICAL_INTEGRITY_SENSITIVE", message: "the converged support state is not established at 128-bit precision (x)" })).toBe(false);
    expect(needsSolverReportNote({ code: "COMBINATION_ROWS_NOT_ASSESSED", message: "combination c: its rows are not assessed against a stated bound (x)" })).toBe(false);
    expect(needsSolverReportNote({ code: "COMBINATION_OPERAND_CHECKS_NOT_PASSED", message: "combination c uses operand case(s) whose numerical integrity checks did not pass" })).toBe(false);
    expect(needsSolverReportNote({ code: "DISPLACEMENT_CORRECTION_UNAVAILABLE", message: "Load case l: the corrected displacement of node:A:UX could not be certified (x)" })).toBe(false);
    // The retained-precision route's and the pressure-source recovery's own texts.
    const retained = json<{ cases: unknown[] }>("fixtures/results/retained_precision_cases.json");
    const retainedTexts = JSON.stringify(retained).match(/Retained-precision recovery \(contribution_preserving_multiprecision_v1\) is selected[^"]*/g) ?? [];
    expect(retainedTexts.length).toBeGreaterThan(0);
    for (const message of retainedTexts) expect(needsSolverReportNote({ code: "RETAINED_PRECISION_SELECTED", message }), message.slice(0, 60)).toBe(false);
    for (const message of retainedTexts) expect(needsSolverReportNote({ message }), message.slice(0, 60)).toBe(false);
    expect(needsSolverReportNote({ code: "SOURCE_BLOCK_RECOVERY_SELECTED", message: "requested_mode=sparse_interactive; selected_method=retained_source_blocks_exact_v1; complete source-bound displacement/member/station/device projections precede binary64 publication; ordinary-attempt evidence remains separate" })).toBe(false);
    expect(needsSolverReportNote({ message: ELIGIBLE_RECOVERY_TEXT })).toBe(false);
    for (const text of ["Uncovered quantity: verified only to an absolute bound of ±1 Pa, below the relative accuracy floor for this body.", "The circular normal-stress maximum bounds the supplied binary64 polynomial coefficient enclosures; solution error is separate, and incomplete coverage withholds the headline."]) expect(needsSolverReportNote({ message: text })).toBe(false);
  });

  it("puts the note directly after each limitation it marks, keeping every text and the order", () => {
    const coded = "Member actions … of a load case reported NUMERICAL_INTEGRITY_CHECKS_PASSED are recovered at 128-bit precision.";
    expect(limitationsWithNotes(["Plain.", coded, "Last."])).toEqual([{ text: "Plain.", note: null }, { text: coded, note: SOLVER_REPORT_NOTE }, { text: "Last.", note: null }]);
    expect(withLimitationsNote(["Plain.", coded, "Last."])).toBe(`Plain. ${coded} ${SOLVER_REPORT_NOTE} Last.`);
    expect(withLimitationsNote(["Plain."])).toBe("Plain.");
  });
});
