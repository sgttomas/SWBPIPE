// Release exit criterion 18 (LIM-026): the desktop's words for what the solver's checks found.
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import * as status from "./integrityStatus";

const STANDING_RS = resolve(__dirname, "../../../../../core/reporting/report_renderer/src/standing.rs");

describe("standing and results-line words", () => {
  it("states the reader's status as an admission standing and the solver's tag as what its checks found", () => {
    expect(status.standingStatusText("solver_checks_passed")).toBe("numerically eligible");
    expect(status.standingStatusText("needs_recompute")).toBe("needs recompute");
    expect(status.numericalQualityStatusText("checks_passed")).toBe("solver checks found no problem");
    for (const other of ["sensitive", "unresolved", "failed", "not_assessed"]) expect(status.numericalQualityStatusText(other)).toBe(other);
    // Where the gated state agrees with the status, the line reads as it always has.
    expect(status.numericalHealthLead({ numerical_quality: { status: "checks_passed" } } as never, "checks_passed")).toBe("Numerical health: solver checks found no problem.");
    expect(status.numericalHealthLead({ numerical_quality: { status: "sensitive" } } as never, "sensitive")).toBe("Numerical health: sensitive.");
    expect(status.numericalHealthLead({ numerical_quality: { status: "unresolved" } } as never, "quality_unresolved")).toBe("Numerical health: unresolved.");
    expect(status.ordinarySolveLead({ numerical_quality: { status: "checks_passed" } } as never, "checks_passed")).toBe("Ordinary solve: solver checks found no problem.");
  });

  it("takes the gated state where the status disagrees with it, and never falls back to the status", () => {
    // An unsolved document whose own status says checks_passed: the gate assesses nothing.
    expect(status.numericalHealthLead({ numerical_quality: { status: "checks_passed" } } as never, "not_assessed")).toBe("Numerical health: not assessed.");
    expect(status.ordinarySolveLead({ numerical_quality: { status: "checks_passed" } } as never, "not_assessed")).toBe("Ordinary solve: not assessed.");
    expect(status.numericalHealthLead({ numerical_quality: { status: "checks_passed" } } as never, "sensitive")).toBe("Numerical health: sensitive.");
    expect(status.numericalHealthLead({ numerical_quality: { status: "invented" } } as never, "checks_passed")).toBe("Numerical health: solver checks found no problem.");
    expect(status.numericalHealthLead({} as never, "not_assessed")).toBe("Numerical health: not assessed.");
  });

  it("reads an absent confirmations member as it reads null: every case's line not read", () => {
    expect(status.supportStatesByCase(undefined)).toBe(status.supportStatesByCase(null));
  });

  it("pins the heading above the solver's own messages", () => {
    expect(status.SOLVER_MESSAGES_HEADING).toBe("Messages from the solver are the solver's own statements. Where one rests on the solver's error estimate, SWBPIPE has not independently checked that estimate.");
  });
});

describe("the support-state line", () => {
  it("has one line for each of the report renderer's confirmation outcomes", () => {
    const rust = readFileSync(STANDING_RS, "utf8");
    const body = rust.slice(rust.indexOf("pub fn confirmation_statement"), rust.indexOf("/// One support's statement"));
    for (const variant of ["SolverReportedEstablished", "NotAttemptedReasonNotRecorded", "NotEstablishedDetailQuoted", "NoConfirmationRecord"]) {
      expect(body, variant).toMatch(new RegExp(`HistoricalConfirmation::${variant} =>`));
    }
    expect(status.CONFIRMATION_STATEMENTS).toEqual({
      solver_reported_established:
        "Support states: reported established by the solver. Its confirmation rests on the solver's own error estimate, which SWBPIPE has not independently checked. This result does not publish the deciding quantities or their intervals.",
      not_attempted_reason_not_recorded: "Support states: not confirmed. No confirmation was attempted for this load case; the result does not record why.",
      not_established_detail_quoted:
        "Support states: not confirmed. The solver's confirmation did not establish the support states of this load case. The result publishes one reason only, quoted below; it does not show that any other support's state was established.",
      no_confirmation_record: "Support states: not confirmed. The result records no support-state confirmation for this load case."
    });
    expect(status.supportStatesText("solver_reported_established")).toBe(status.CONFIRMATION_STATEMENTS.solver_reported_established);
    // The quoted reason is carried and shown beneath; where it is not carried the statement points to where it is.
    expect(status.supportStatesLine("not_established_detail_quoted", "formation check: stage B: x")).toEqual({ text: status.CONFIRMATION_STATEMENTS.not_established_detail_quoted, detail: "formation check: stage B: x" });
    expect(status.supportStatesLine("not_established_detail_quoted", null)!.text).toBe(status.CONFIRMATION_STATEMENTS.not_established_detail_quoted.replace("quoted below", "stated in this load case's solver diagnostic"));
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
    expect(status.supportStatesOf(status.supportStatesByCase([{ case_id: "L3", outcome: "no_confirmation_record", detail: null }]), "L3")).toEqual({ text: status.CONFIRMATION_STATEMENTS.no_confirmation_record, detail: null });
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
