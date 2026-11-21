// Release exit criterion 18 (LIM-026): the desktop's words for the solver's integrity verdict.
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import * as status from "./integrityStatus";

const STANDING_RS = resolve(__dirname, "../../../../../core/reporting/report_renderer/src/standing.rs");

describe("the solver report note and its predicate", () => {
  it("names the note once and needs it for the solver's checks-passed code and its established line", () => {
    expect(status.SOLVER_REPORT_NOTE).toBe("Reported by the solver; not verified by SWBPIPE.");
    expect(status.needsSolverReportNote({ code: "NUMERICAL_INTEGRITY_CHECKS_PASSED", message: "x" })).toBe(true);
    expect(status.needsSolverReportNote({ code: "MODEL_INCOMPLETE", message: "Inspection only (prior_outcome=NUMERICAL_INTEGRITY_CHECKS_PASSED): blocked" })).toBe(true);
    expect(status.needsSolverReportNote({ code: "OTHER", message: "… Converged support state: established at 128-bit precision by one formation-checked solve; …" })).toBe(true);
    expect(status.needsSolverReportNote({ code: "NUMERICAL_INTEGRITY_SENSITIVE", message: "the converged support state is not established at 128-bit precision (x)" })).toBe(false);
    expect(status.needsSolverReportNote({ code: "NUMERICAL_INTEGRITY_SENSITIVE", message: "sensitive" })).toBe(false);
    expect(status.needsSolverReportNote(null)).toBe(false);
    const coded = "Member actions … of a load case reported NUMERICAL_INTEGRITY_CHECKS_PASSED are recovered at 128-bit precision.";
    expect(status.limitationsNote(["Plain.", coded])).toBe(status.SOLVER_REPORT_NOTE);
    expect(status.limitationsNote(["Plain."])).toBeNull();
    expect(status.withLimitationsNote(["Plain.", coded])).toBe(`Plain. ${coded} ${status.SOLVER_REPORT_NOTE}`);
    expect(status.withLimitationsNote(["Plain."])).toBe("Plain.");
  });
});

describe("standing and results-line words", () => {
  it("states the reader's status and the solver's tag as not verified, and every other status as before", () => {
    expect(status.standingStatusText("solver_checks_passed")).toBe("solver checks passed, not verified");
    expect(status.standingStatusText("needs_recompute")).toBe("needs_recompute");
    expect(status.numericalQualityStatusText("checks_passed")).toBe("not verified");
    for (const other of ["sensitive", "unresolved", "failed", "not_assessed"]) expect(status.numericalQualityStatusText(other)).toBe(other);
    expect(status.numericalHealthLead({ numerical_quality: { status: "checks_passed" } } as never)).toBe("Numerical health: not verified.");
    expect(status.numericalHealthLead({ numerical_quality: { status: "sensitive" } } as never)).toBe("Numerical health: sensitive.");
    expect(status.ordinarySolveLead({ numerical_quality: { status: "checks_passed" } } as never)).toBe("Ordinary solve: not verified.");
  });
});

describe("the support-state line", () => {
  it("is the report renderer's reviewed statement, with the desktop's words for the inverse-norm condition", () => {
    const rust = readFileSync(STANDING_RS, "utf8");
    const body = rust.slice(rust.indexOf("pub fn confirmation_statement"), rust.indexOf("/// One support's statement"));
    const arm = (variant: string) => {
      const match = new RegExp(`HistoricalConfirmation::${variant} => \\{?\\s*"([^"]+)"`).exec(body);
      expect(match, variant).not.toBeNull();
      return match![1];
    };
    const desktop = (text: string) => text.replace("conditional on the solver's inverse-norm step", "reported by the solver; not verified by SWBPIPE");
    expect(status.CONFIRMATION_STATEMENTS).toEqual({
      solver_reported_established: desktop(arm("SolverReportedEstablished")),
      not_attempted_reason_not_recorded: arm("NotAttemptedReasonNotRecorded"),
      not_established_detail_quoted: arm("NotEstablishedDetailQuoted"),
      no_confirmation_record: arm("NoConfirmationRecord")
    });
    for (const text of Object.values(status.CONFIRMATION_STATEMENTS)) expect(text).not.toMatch(/inverse-norm/i);
    expect(status.supportStatesText("solver_reported_established")).toBe(`Support states: not confirmed. ${status.CONFIRMATION_STATEMENTS.solver_reported_established}`);
    expect(status.supportStatesText("not_applicable")).toBeNull();
    expect(status.supportStatesText("invented")).toBeNull();
  });

  it("reads a standing record's and a native summary's outcomes by case", () => {
    const record = { cases: [{ case_id: "L1", confirmation: { outcome: "solver_reported_established" } }, { case_id: "L2", confirmation: { outcome: "not_applicable" } }] };
    expect([...status.supportStatesByCase(record).keys()]).toEqual(["L1"]);
    expect([...status.supportStatesByCase([{ case_id: "L3", outcome: "no_confirmation_record" }]).entries()]).toEqual([["L3", `Support states: not confirmed. ${status.CONFIRMATION_STATEMENTS.no_confirmation_record}`]]);
    expect(status.supportStatesByCase(null).size).toBe(0);
  });
});
