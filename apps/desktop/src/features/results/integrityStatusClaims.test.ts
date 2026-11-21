// Release exit criterion 18 (LIM-026): the one predicate that decides whether a solver text shown to
// a person needs "Reported by the solver; not verified by SWBPIPE." beside it, and its closed table.
// The table (`solverTextClaims.json`) classifies every code of the repository's diagnostic code table
// (`fixtures/diagnostics/exact_route_codes.json`, written by tools/validation/diagnostic_codes.py from
// the solve crates' sources); a code added there and not classified here fails the first test. A
// `claim` or `quotes_claim` code always takes the note; the phrases decide a text with no code or a
// code outside the table, and each phrase's test below says where it is found.
import { readdirSync, readFileSync } from "node:fs";
import { join, relative, resolve } from "node:path";
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

/** Every `{code, message}` anywhere in a committed JSON fixture whose message quotes a structural
 * report's quality (`quality: {word}`), with the words it quotes. A file that does not contain the
 * text is not parsed. */
function quotedQualities(): { code: string; message: string; file: string; qualities: string[] }[] {
  const out: { code: string; message: string; file: string; qualities: string[] }[] = [];
  const visit = (value: unknown, file: string) => {
    if (Array.isArray(value)) { for (const item of value) visit(item, file); return; }
    if (!value || typeof value !== "object") return;
    const record = value as Record<string, unknown>;
    if (typeof record.code === "string" && typeof record.message === "string") {
      const qualities = [...new Set([...record.message.matchAll(/\bquality: (\w+)/g)].map((m) => m[1]))];
      if (qualities.length > 0) out.push({ code: record.code, message: record.message, file, qualities });
    }
    for (const item of Object.values(record)) visit(item, file);
  };
  const walk = (dir: string): string[] => readdirSync(dir, { withFileTypes: true }).flatMap((entry) => entry.isDirectory() ? walk(join(dir, entry.name)) : entry.name.endsWith(".json") ? [join(dir, entry.name)] : []);
  for (const path of walk(resolve(root, "fixtures"))) {
    const text = readFileSync(path, "utf8");
    if (!text.includes("quality: ")) continue;
    visit(JSON.parse(text), relative(root, path));
  }
  return out;
}

describe("the census of committed results that quote the ordinary solve's structural report", () => {
  const found = quotedQualities();

  it("finds the report's quality only as Passed or Sensitive, the two variants of the solver's SolveQuality", () => {
    expect(found.length).toBeGreaterThan(100);
    // A spelling of a pass the phrases do not know fails here, not in review.
    expect([...new Set(found.flatMap((d) => d.qualities))].sort()).toEqual(["Passed", "Sensitive"]);
  });

  it("marks every diagnostic that states a pass under a code that is not no_claim", () => {
    const passing = found.filter((d) => d.qualities.includes("Passed") && solverTextClaim(d.code) !== "no_claim");
    expect(passing.filter((d) => !needsSolverReportNote(d)).map((d) => `${d.code} (${d.file})`)).toEqual([]);
    // The two witnesses round 2 named: a sensitive case and an unqualified recovery basis whose quoted report passed.
    expect(passing.some((d) => d.code === "NUMERICAL_INTEGRITY_SENSITIVE" && d.file === "fixtures/results/pressure_v3_cap_remainder_halfway_force.json")).toBe(true);
    expect(passing.some((d) => d.code === "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED" && d.file === "fixtures/results/preview_physics_nonlinear_operands_corpus.json")).toBe(true);
    // No code the table classifies as stating no claim quotes a passed report.
    expect(found.filter((d) => d.qualities.includes("Passed") && solverTextClaim(d.code) === "no_claim").map((d) => `${d.code} (${d.file})`)).toEqual([]);
  });
});

describe("where each of the solver's fixed phrases is found", () => {
  const source = (path: string) => readFileSync(resolve(root, path), "utf8");
  /** For each phrase, the producer source this test reads and what it asserts there. */
  const FOUND: Record<string, () => void> = {
    // The code, as a literal in the producer's integrity report.
    NUMERICAL_INTEGRITY_CHECKS_PASSED: () => expect(source("core/product_physics/src/lib.rs")).toContain('"NUMERICAL_INTEGRITY_CHECKS_PASSED"'),
    // The constant NONLINEAR_CONFIRMED_EVIDENCE_LINE.
    "Converged support state: established at 128-bit precision": () => expect(source("core/product_physics/src/lib.rs")).toContain('const NONLINEAR_CONFIRMED_EVIDENCE_LINE: &str = "Converged support state: established at 128-bit precision'),
    // The equilibrium sentence of append_integrity_report, and the inspection-only unqualified message.
    "separate zero count/cap/contact/sliding checks passed": () => expect(source("core/product_physics/src/lib.rs")).toContain("separate zero count/cap/contact/sliding checks passed"),
    // The not-joined message's attempt token.
    not_required_ordinary_checks_passed: () => expect(source("core/product_physics/src/lib.rs")).toContain('"not_required_ordinary_checks_passed"'),
    "derived row group(s) are established within their stated bound": () => expect(source("core/product_physics/src/combination_certificate.rs")).toContain("derived row group(s) are established within their stated bound"),
    "the case is stress-free to within numerical precision": () => expect(source("core/product_physics/src/formation_guard.rs")).toContain("the case is stress-free to within numerical precision"),
    "only the derived row groups that contract_evidence.combination_certificate establishes carry a stated bound": () => expect(source("core/product_physics/src/exact_combinations.rs")).toContain("only the derived row groups that contract_evidence.combination_certificate establishes carry a stated bound"),
    "Established within its stated numerical bound for the explicitly held system": () => expect(source("core/product_physics/src/held_status.rs")).toContain("Established within its stated numerical bound for the explicitly held system"),
    // Not a literal: the Debug form of StructuralReport's `quality` field. Asserted from the type (its
    // derive, the field and the enum's two variants), the integrity report formatting the whole report
    // with `{:?}`, and the report standing reader that parses the same text.
    "quality: Passed": () => {
      const structural = source("core/solver/frame_kernel/src/structural.rs");
      expect(structural).toMatch(/#\[derive\(Debug, Clone, Copy, PartialEq, Eq\)\]\npub enum SolveQuality \{\n {4}Passed,\n {4}Sensitive,\n\}/);
      expect(structural).toMatch(/#\[derive\(Debug, Clone, PartialEq\)\]\npub struct StructuralReport \{\n {4}pub policy: &'static str,\n {4}pub quality: SolveQuality,/);
      expect(source("core/product_physics/src/lib.rs")).toContain('"{} represented original-equation structural evidence for load case {}: {:?}; global_dof_map={:?}.');
      expect(source("core/reporting/report_standing/src/historical_health.rs")).toContain('pub(crate) const LEAD_B: &str = ": StructuralReport { policy: \\"M03-INTEGRITY-v1\\", quality: ";');
    }
  };

  it("names a source for each phrase of the table, and only those", () => {
    expect(Object.keys(FOUND).sort()).toEqual([...SOLVER_CLAIM_PHRASES].sort());
  });
  it.each(Object.keys(FOUND))("%s", (phrase) => FOUND[phrase]());
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
    // A text with no code: a limitation, a held-status outcome, a confirmation's quoted detail.
    expect(needsSolverReportNote({ message: "Member actions … of a load case reported NUMERICAL_INTEGRITY_CHECKS_PASSED are recovered at 128-bit precision." })).toBe(true);
    expect(needsSolverReportNote({ message: "Established within its stated numerical bound for the explicitly held system at this case's loads. The verdict does not establish this case's own support state." })).toBe(true);
    expect(needsSolverReportNote(null)).toBe(false);
  });

  it("notes every message of a quoting code, whatever its words", () => {
    // A quoting code's wording is not closed: a message that quotes no claim takes the note too.
    expect(needsSolverReportNote({ code: "NUMERICAL_INTEGRITY_SENSITIVE", message: "the converged support state is not established at 128-bit precision (x)" })).toBe(true);
    expect(needsSolverReportNote({ code: "NUMERICAL_INTEGRITY_SENSITIVE", message: "M03-INTEGRITY-v1 represented original-equation structural evidence for load case c: StructuralReport { policy: \"M03-INTEGRITY-v1\", quality: Sensitive, factorization: \"x\" }" })).toBe(true);
    expect(needsSolverReportNote({ code: "LOAD_STATE_SOURCE_RECOVERY_NOT_JOINED", message: "requested_mode=sparse_interactive; …; retained_source_attempt=unavailable" })).toBe(true);
    for (const code of Object.keys(claims.quotes_claim)) expect(needsSolverReportNote({ code, message: "" }), code).toBe(true);
  });

  it("decides a text the code does not decide by the phrases: the structural report's pass, and no other quality", () => {
    const report = (quality: string) => `M03-INTEGRITY-v1 represented original-equation structural evidence for load case c: StructuralReport { policy: "M03-INTEGRITY-v1", quality: ${quality}, factorization: "x" }; global_dof_map=[]`;
    expect(needsSolverReportNote({ message: report("Passed") })).toBe(true);
    expect(needsSolverReportNote({ code: "OUTSIDE_THE_TABLE", message: report("Passed") })).toBe(true);
    expect(needsSolverReportNote({ message: report("Sensitive") })).toBe(false);
    expect(needsSolverReportNote({ code: "OUTSIDE_THE_TABLE", message: report("Sensitive") })).toBe(false);
    // A no-claim code is decided by its code, never by the phrases.
    expect(needsSolverReportNote({ code: "RETAINED_PRECISION_SELECTED", message: report("Passed") })).toBe(false);
  });

  it("notes no denial, and nothing another certificate establishes", () => {
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
