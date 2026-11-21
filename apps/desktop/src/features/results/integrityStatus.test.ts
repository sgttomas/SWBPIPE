// Release exit criterion 18 (LIM-026): the desktop's words for the solver's integrity verdict.
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import * as status from "./integrityStatus";

const STANDING_RS = resolve(__dirname, "../../../../../core/reporting/report_renderer/src/standing.rs");

describe("standing and results-line words", () => {
  it("states the reader's status and the solver's tag as not verified, and every other status as before", () => {
    expect(status.standingStatusText("solver_checks_passed")).toBe("numerically eligible");
    expect(status.standingStatusText("needs_recompute")).toBe("needs recompute");
    expect(status.integrityBoundText({ status: "not_verified", reason: "inverse_norm_bound_not_verified" })).toBe("the ordinary solve's error bound is not verified");
    expect(status.integrityBoundText(null)).toBeNull();
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
    // The quoted reason is carried and shown beneath; where it is not carried the statement points to where it is.
    expect(status.supportStatesLine("not_established_detail_quoted", "formation check: stage B: x")).toEqual({ text: `Support states: not confirmed. ${status.CONFIRMATION_STATEMENTS.not_established_detail_quoted}`, detail: "formation check: stage B: x", detailNote: null });
    expect(status.supportStatesLine("not_established_detail_quoted", null)!.text).toBe(`Support states: not confirmed. ${status.CONFIRMATION_STATEMENTS.not_established_detail_quoted.replace("quoted below", "stated in this load case's solver diagnostic")}`);
    expect(status.supportStatesLine("not_established_detail_quoted", "separate zero count/cap/contact/sliding checks passed")!.detailNote).toBe(status.SOLVER_REPORT_NOTE);
    expect(status.supportStatesText("not_applicable")).toBeNull();
    expect(status.supportStatesText("invented")).toBeNull();
  });

  it("reads a standing record's and a native summary's outcomes by case", () => {
    const record = { cases: [{ case_id: "L1", confirmation: { outcome: "solver_reported_established" } }, { case_id: "L2", confirmation: { outcome: "not_applicable" } }] };
    const read = status.supportStatesByCase(record);
    expect([...read.byCase.entries()]).toEqual([["L1", status.supportStatesLine("solver_reported_established", null)], ["L2", null]]);
    // A case the record does not name, and an outcome outside the vocabulary, could not be read.
    expect(status.supportStatesOf(read, "L9")).toEqual(status.SUPPORT_STATES_UNREAD_LINE);
    expect(status.supportStatesOf(status.supportStatesByCase([{ case_id: "L5", outcome: "invented" }]), "L5")).toEqual(status.SUPPORT_STATES_UNREAD_LINE);
    expect(status.supportStatesOf(status.supportStatesByCase([{ case_id: "L3", outcome: "no_confirmation_record", detail: null }]), "L3")).toEqual({ text: `Support states: not confirmed. ${status.CONFIRMATION_STATEMENTS.no_confirmation_record}`, detail: null, detailNote: null });
    expect(status.supportStatesOf(status.supportStatesByCase({ cases: [{ case_id: "L4", confirmation: { outcome: "not_established_detail_quoted", detail: "why" } }] }), "L4")!.detail).toBe("why");
    // Nothing read: every case says so.
    expect(status.supportStatesByCase(null)).toBe(status.SUPPORT_STATES_UNREAD);
  });

  it("hides the line only where the model is known to have no nonlinear support or the record gives none", () => {
    expect(status.nonlinearSupportKnowledge({ supports: [] })).toBe("none");
    expect(status.nonlinearSupportKnowledge({ supports: [{ id: "s", nonlinear: { kind: "one_way" } }] })).toBe("nonlinear");
    for (const unknown of [null, undefined, {}, { supports: null }, [], "x"]) expect(status.nonlinearSupportKnowledge(unknown)).toBe("unknown");
    expect(status.supportStatesFor({ supports: [] }, () => { throw new Error("not read"); })).toBe(status.SUPPORT_STATES_NONE);
    expect(status.supportStatesFor(null, () => null)).toBe(status.SUPPORT_STATES_UNREAD);
    expect(status.supportStatesFor({ supports: [{ nonlinear: {} }] }, () => { throw new Error("refused"); })).toBe(status.SUPPORT_STATES_UNREAD);
    expect(status.nativeSupportStates([{ case_id: "L1", outcome: "not_applicable" }], "no_nonlinear_support")).toBe(status.SUPPORT_STATES_NONE);
    expect(status.nativeSupportStates([{ case_id: "L1", outcome: "not_applicable" }], "not_read")).toBe(status.SUPPORT_STATES_UNREAD);
    expect(status.supportStatesOf(status.nativeSupportStates([{ case_id: "L1", outcome: "not_applicable" }], "read"), "L1")).toBeNull();
    expect(status.nativeSupportStates(null, "read")).toBe(status.SUPPORT_STATES_UNREAD);
    // A solve's S, read with its own model, keeps its record; with no record, a model with none still hides it.
    expect(status.supportStatesOf(status.nativeSupportStates([{ case_id: "L1", outcome: "not_applicable" }], undefined, null), "L1")).toBeNull();
    expect(status.nativeSupportStates(null, undefined, { supports: [] })).toBe(status.SUPPORT_STATES_NONE);
  });
});

describe("the cases whose values come from the producer's recovery", () => {
  it("reads each case's recorded method: the recovered case, not the ordinary companion", () => {
    const mixed = JSON.parse(readFileSync(resolve(__dirname, "../../../../../fixtures/product_preview/pressure_source/physics_source/mixed-sparse_interactive.raw.json"), "utf8"));
    expect([...status.recoveredCaseIds(mixed)]).toEqual(["case"]);
    expect(status.recoveredCaseIds(null).size).toBe(0);
  });
});
