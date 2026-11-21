// DEL-07-05 numerical health §3, §5 and §7: every literal of numericalHealthText.ts
// pinned to the design's wording, and the module's vocabulary fence.
import { describe, expect, it } from "vitest";
import * as text from "./numericalHealthText";
import * as status from "./integrityStatus";

describe("numerical health text pins", () => {
  it("pins the label, states, headlines and prefix", () => {
    expect(text.NUMERICAL_HEALTH_LABEL).toBe("Numerical health");
    expect(text.NUMERICAL_HEALTH_STATE_TEXT).toEqual({
      checks_passed: "not verified",
      sensitive: "sensitive",
      quality_unresolved: "quality unresolved",
      solve_failed: "solve failed",
      not_assessed: "not assessed"
    });
    expect(text.numericalHealthHeadline("checks_passed")).toBe("Numerical health: not verified");
    expect(text.numericalHealthHeadline("sensitive")).toBe("Numerical health: sensitive");
    expect(text.numericalHealthHeadline("quality_unresolved")).toBe("Numerical health: quality unresolved");
    expect(text.numericalHealthHeadline("solve_failed")).toBe("Numerical health: solve failed");
    expect(text.numericalHealthHeadline("not_assessed")).toBe("Numerical health: not assessed");
    expect(text.ORDINARY_SOLVE_PREFIX).toBe("Ordinary solve —");
    expect(text.numericalHealthHeadline("sensitive", true)).toBe("Ordinary solve — Numerical health: sensitive");
  });

  it("pins the header count form", () => {
    expect(text.numericalHealthCountHeadline([["solve_failed", 0], ["quality_unresolved", 0], ["sensitive", 2], ["not_assessed", 0], ["checks_passed", 1]]))
      .toBe("Numerical health: 2 sensitive, 1 not verified");
    expect(text.numericalHealthCountHeadline([["solve_failed", 0], ["quality_unresolved", 0], ["sensitive", 0], ["not_assessed", 0], ["checks_passed", 3]]))
      .toBe("Numerical health: not verified");
    expect(text.numericalHealthCountHeadline([["solve_failed", 1], ["quality_unresolved", 1], ["sensitive", 1], ["not_assessed", 1], ["checks_passed", 1]], true))
      .toBe("Ordinary solve — Numerical health: 1 solve failed, 1 quality unresolved, 1 sensitive, 1 not assessed, 1 not verified");
  });

  it("pins the pressure-source, output and boundary sentences", () => {
    expect(text.ELIGIBLE_RECOVERY_TEXT).toBe(
      "The values shown come from the producer's recovery of this case, which passed its own checks. This badge describes the ordinary solve only."
    );
    expect(text.outputSentence(2)).toBe(
      "Rule checks, the result JSON and stress-neutral exports and the report package do not use this run while 2 load case(s) are sensitive, quality unresolved, solve failed or not assessed; results tables, CSV export and, for an exact-route result, the rendered report carry each case's standing."
    );
    expect(text.BOUNDARY_SENTENCE).toBe(
      "These checks look for numerical trouble; they do not measure the error, and they say nothing about whether the model, its inputs or its design are right."
    );
  });

  it("pins every detail text", () => {
    expect(text.checksPassedText("LC1")).toBe("The solver's own checks found no problem with LC1, but SWBPIPE has not verified an error bound for its values.");
    expect(text.sensitiveLeadText("LC1")).toBe("The solver's checks flagged LC1 as numerically sensitive.");
    expect(text.KEPT_FOR_INSPECTION_TEXT).toBe("Its values are kept for inspection.");
    expect(text.CONDITIONING_CAUSE_TEXT).toBe("The scaled stiffness is poorly conditioned.");
    expect(text.loadFidelityCauseText("load:a")).toBe(
      "The solver could not confirm that every applied load was carried at full precision (load:a)."
    );
    expect(text.formationCauseText("node N1, UX")).toBe(
      "The solver's formation check could not confirm some stiffness or load terms at full precision (node N1, UX)."
    );
    expect(text.formationRecoveryCauseText("member forces")).toBe(
      "The solver could not confirm some recovered member or support quantities at full precision (member forces)."
    );
    expect(text.loadFidelityCauseText("")).toBe("The solver could not confirm that every applied load was carried at full precision.");
    expect(text.formationCauseText("")).toBe("The solver's formation check could not confirm some stiffness or load terms at full precision.");
    expect(text.formationRecoveryCauseText("")).toBe("The solver could not confirm some recovered member or support quantities at full precision.");
    expect(text.SEE_DIAGNOSTIC_TEXT).toBe("See the solver diagnostic for the reason.");
    expect(text.SENSITIVE_CLOSING_TEXT).toBe(
      "Solving the unchanged model again in the same solver mode gives the same result. A very short or very stiff element next to flexible ones, stiffnesses that differ by many orders of magnitude, or a weakly restrained direction are common causes."
    );
    expect(text.qualityUnresolvedLeadText("LC1")).toBe("The solver could not confirm the numerical quality of LC1 at working precision.");
    expect(text.QUALITY_UNRESOLVED_CLOSING_TEXT).toBe("A near-mechanism or extreme stiffness contrast are common causes.");
    expect(text.RECOVERY_BASIS_UNQUALIFIED_TEXT).toBe(
      "Its support-state solution is kept for inspection, but its numerical quality could not be confirmed."
    );
    expect(text.solveFailedLeadText("LC1")).toBe("The solver could not complete a solution for LC1.");
    expect(text.MECHANISM_TEXT).toBe("The model can move without resistance (a mechanism). Review connectivity, member releases and supports.");
    expect(text.NEGATIVE_ENERGY_TEXT).toBe("The assembled stiffness is not positive in some direction. Review element and support properties.");
    expect(text.notAssessedText("LC1")).toBe("No numerical quality is stated for LC1.");
    expect(text.combinationText([["LC1", "sensitive"], ["LC2", "checks_passed"]])).toBe(
      "Inherited from its load cases: LC1: sensitive; LC2: not verified."
    );
    expect(text.conditioningDetailText("10.5")).toBe(
      "Estimated sensitivity of the scaled linear solve: rounding can be magnified by roughly 10.5 of the about 16 decimal digits the computer carries (the solver marks a case sensitive at about 7.8). This is an estimate; it does not estimate the error in individual results."
    );
    expect(text.DIGITS_BELOW_FIRST_DECIMAL).toBe("below 0.1");
    expect(text.nodeDofLocationText("N1", "UX")).toBe("node N1, UX");
    expect(text.SHOW_SOLVER_DIAGNOSTIC).toBe("Show solver diagnostic");
    expect(text.LOCATED_CAUSES_LABEL).toBe("Named locations");
    expect(text.DETAIL_SECTION_LABEL).toBe("Numerical health by load case");
    expect(text.reportLineText("checks_passed")).toBe("Numerical health of the ordinary solve: not verified.");
  });

  it("pins the module's exports, so a new literal comes with a pin", () => {
    expect(Object.keys(text).sort()).toEqual([
      "BOUNDARY_SENTENCE", "CONDITIONING_CAUSE_TEXT", "DETAIL_SECTION_LABEL", "DIGITS_BELOW_FIRST_DECIMAL",
      "ELIGIBLE_RECOVERY_TEXT", "KEPT_FOR_INSPECTION_TEXT", "LOCATED_CAUSES_LABEL", "MECHANISM_TEXT",
      "NEGATIVE_ENERGY_TEXT", "NUMERICAL_HEALTH_LABEL", "NUMERICAL_HEALTH_STATE_TEXT", "ORDINARY_SOLVE_PREFIX",
      "QUALITY_UNRESOLVED_CLOSING_TEXT", "RECOVERY_BASIS_UNQUALIFIED_TEXT", "SEE_DIAGNOSTIC_TEXT",
      "SENSITIVE_CLOSING_TEXT", "SHOW_SOLVER_DIAGNOSTIC", "checksPassedText", "combinationText",
      "conditioningDetailText", "formationCauseText", "formationRecoveryCauseText", "loadFidelityCauseText",
      "nodeDofLocationText", "notAssessedText", "numericalHealthCountHeadline", "numericalHealthHeadline",
      "outputSentence", "qualityUnresolvedLeadText", "reportLineText", "sensitiveLeadText", "solveFailedLeadText"
    ].sort());
  });
});

describe("numerical health vocabulary", () => {
  // DEL-07-05 REQ-023 and AC-008, as changed for release exit criterion 18 (LIM-026): no text of the
  // indicator uses certified, guaranteed, valid, approved, compliant, correct or reliable; it uses
  // verified and bound only to deny them, in the phrases "not verified" and "has not verified an error
  // bound". Each permitted denial is taken out only as a whole phrase at word boundaries, so "a knot
  // verified" or "unbound" keeps its word. The note ("Reported by the solver; not verified by
  // SWBPIPE.") denies by the first phrase.
  const DENIALS = [/\bhas not verified an error bound\b/gi, /\bnot verified\b/gi];
  const FENCED = ["verified", "bound", "certified", "guaranteed", "valid", "approved", "compliant", "correct", "reliable"];
  function fencedWords(value: string): string[] {
    const rest = DENIALS.reduce((left, denial) => left.replace(denial, " "), value);
    return FENCED.filter((word) => new RegExp(`\\b${word}\\b`, "i").test(rest));
  }
  const STATES = Object.keys(text.NUMERICAL_HEALTH_STATE_TEXT) as text.NumericalHealthState[];
  const OUTCOMES = Object.keys(status.CONFIRMATION_STATEMENTS);
  const shown = (value: unknown): string[] => typeof value === "string" ? [value] : value && typeof value === "object" ? Object.values(value).flatMap(shown) : [];
  /** Every string each export of the indicator's two text modules can show, by export: a constant
   * as it is, a record's values, a function called over every state, outcome and form it takes. */
  const INDICATOR: Record<string, () => string[]> = {
    // numericalHealthText.ts: every export is indicator text.
    ...Object.fromEntries(Object.entries(text).filter(([, value]) => typeof value !== "function").map(([name, value]) => [`text.${name}`, () => shown(value)])),
    "text.numericalHealthHeadline": () => STATES.flatMap((state) => [text.numericalHealthHeadline(state), text.numericalHealthHeadline(state, true)]),
    "text.numericalHealthCountHeadline": () => [true, false].flatMap((prefix) => [text.numericalHealthCountHeadline(STATES.map((state) => [state, 1] as const), prefix), text.numericalHealthCountHeadline([["checks_passed", 2]], prefix)]),
    "text.outputSentence": () => [text.outputSentence(2)],
    "text.checksPassedText": () => [text.checksPassedText("LC1")],
    "text.sensitiveLeadText": () => [text.sensitiveLeadText("LC1")],
    "text.loadFidelityCauseText": () => [text.loadFidelityCauseText(""), text.loadFidelityCauseText("load:a")],
    "text.formationCauseText": () => [text.formationCauseText(""), text.formationCauseText("node N1, UX")],
    "text.formationRecoveryCauseText": () => [text.formationRecoveryCauseText(""), text.formationRecoveryCauseText("member forces")],
    "text.qualityUnresolvedLeadText": () => [text.qualityUnresolvedLeadText("LC1")],
    "text.solveFailedLeadText": () => [text.solveFailedLeadText("LC1")],
    "text.notAssessedText": () => [text.notAssessedText("LC1")],
    "text.combinationText": () => [text.combinationText(STATES.map((state) => ["LC1", state] as const))],
    "text.conditioningDetailText": () => [text.conditioningDetailText("10.5")],
    "text.nodeDofLocationText": () => [text.nodeDofLocationText("N1", "UX")],
    "text.reportLineText": () => STATES.map(text.reportLineText),
    // integrityStatus.ts: the indicator's note, support-state lines and results-line leads.
    "status.SOLVER_REPORT_NOTE": () => [status.SOLVER_REPORT_NOTE],
    "status.CONFIRMATION_STATEMENTS": () => shown(status.CONFIRMATION_STATEMENTS),
    "status.SUPPORT_STATES_LEAD": () => [status.SUPPORT_STATES_LEAD],
    "status.DETAIL_NOT_CARRIED_TEXT": () => [status.DETAIL_NOT_CARRIED_TEXT],
    "status.supportStatesText": () => OUTCOMES.flatMap((outcome) => shown(status.supportStatesText(outcome))),
    "status.supportStatesLine": () => OUTCOMES.flatMap((outcome) => [null, "x"].flatMap((detail) => { const line = status.supportStatesLine(outcome, detail); return line ? [line.text, ...shown(line.detailNote)] : []; })),
    "status.numericalQualityStatusText": () => [...STATES, "checks_passed", "unresolved", "failed"].map(status.numericalQualityStatusText),
    "status.numericalHealthLead": () => ["checks_passed", "sensitive"].map((value) => status.numericalHealthLead({ numerical_quality: { status: value } } as never)),
    "status.ordinarySolveLead": () => ["checks_passed", "sensitive"].map((value) => status.ordinarySolveLead({ numerical_quality: { status: value } } as never))
  };
  /** The exports of integrityStatus.ts that are not indicator text, with what they are. */
  const NOT_INDICATOR: Record<string, string> = {
    CHECKS_PASSED_CODE: "the solver's code, matched, never shown as the desktop's words",
    SUPPORT_STATE_ESTABLISHED_TEXT: "the solver's own sentence, matched in its text",
    SOLVER_CLAIM_PHRASES: "the solver's own phrases, matched in its text",
    solverTextClaim: "the classification of a code",
    needsSolverReportNote: "the predicate",
    limitationsWithNotes: "the solver's limitation texts with the note",
    withLimitationsNote: "the solver's limitation texts with the note",
    standingStatusText: "the reader's standing status words, outside the indicator",
    INTEGRITY_BOUND_NOT_VERIFIED_TEXT: "the reader verdict's bound member in words, outside the indicator",
    integrityBoundText: "the reader verdict's bound member in words, outside the indicator",
    supportStatesByCase: "reads records into supportStatesLine",
    SOURCE_RECOVERY_METHOD: "a method token, matched",
    recoveredCaseIds: "reads a result's recorded methods"
  };

  it("classifies every export of both modules, so a new string comes with a check", () => {
    const named = new Set([...Object.keys(INDICATOR), ...Object.keys(NOT_INDICATOR).map((name) => `status.${name}`)]);
    const exported = [...Object.keys(text).map((name) => `text.${name}`), ...Object.keys(status).map((name) => `status.${name}`)];
    expect(exported.filter((name) => !named.has(name)), "unclassified exports").toEqual([]);
    expect([...named].filter((name) => !exported.includes(name)), "classified names that are not exports").toEqual([]);
  });

  it.each(Object.keys(INDICATOR))("%s uses no fenced word but in the two denials", (name) => {
    const strings = INDICATOR[name]();
    expect(strings.length, name).toBeGreaterThan(0);
    for (const value of strings) expect(fencedWords(value), `${name}: ${value}`).toEqual([]);
  });

  it("finds a fenced word that only looks like a denial", () => {
    expect(fencedWords("a knot verified")).toEqual(["verified"]);
    expect(fencedWords("not verifiedly bound")).toEqual(["bound"]);
    expect(fencedWords("an error bound is verified")).toEqual(["verified", "bound"]);
    expect(fencedWords("checks passed and verified")).toEqual(["verified"]);
    expect(fencedWords("it is correct")).toEqual(["correct"]);
    expect(fencedWords("Reported by the solver; not verified by SWBPIPE.")).toEqual([]);
    expect(fencedWords("SWBPIPE has not verified an error bound for its values.")).toEqual([]);
  });
});
