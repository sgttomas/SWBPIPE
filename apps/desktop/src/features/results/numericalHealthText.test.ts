// DEL-07-05 numerical health §3, §5 and §7: every literal of numericalHealthText.ts
// pinned to the design's wording, and the module's vocabulary fence.
import { readFileSync } from "node:fs";
import path from "node:path";
import { describe, expect, it } from "vitest";
import * as text from "./numericalHealthText";

describe("numerical health text pins", () => {
  it("pins the label, states, headlines and prefix", () => {
    expect(text.NUMERICAL_HEALTH_LABEL).toBe("Numerical health");
    expect(text.NUMERICAL_HEALTH_STATE_TEXT).toEqual({
      checks_passed: "checks passed",
      sensitive: "sensitive",
      quality_unresolved: "quality unresolved",
      solve_failed: "solve failed",
      not_assessed: "not assessed"
    });
    expect(text.numericalHealthHeadline("checks_passed")).toBe("Numerical health: checks passed");
    expect(text.numericalHealthHeadline("sensitive")).toBe("Numerical health: sensitive");
    expect(text.numericalHealthHeadline("quality_unresolved")).toBe("Numerical health: quality unresolved");
    expect(text.numericalHealthHeadline("solve_failed")).toBe("Numerical health: solve failed");
    expect(text.numericalHealthHeadline("not_assessed")).toBe("Numerical health: not assessed");
    expect(text.ORDINARY_SOLVE_PREFIX).toBe("Ordinary solve —");
    expect(text.numericalHealthHeadline("sensitive", true)).toBe("Ordinary solve — Numerical health: sensitive");
  });

  it("pins the header count form", () => {
    expect(text.numericalHealthCountHeadline([["solve_failed", 0], ["quality_unresolved", 0], ["sensitive", 2], ["not_assessed", 0], ["checks_passed", 1]]))
      .toBe("Numerical health: 2 sensitive, 1 checks passed");
    expect(text.numericalHealthCountHeadline([["solve_failed", 0], ["quality_unresolved", 0], ["sensitive", 0], ["not_assessed", 0], ["checks_passed", 3]]))
      .toBe("Numerical health: checks passed");
    expect(text.numericalHealthCountHeadline([["solve_failed", 1], ["quality_unresolved", 1], ["sensitive", 1], ["not_assessed", 1], ["checks_passed", 1]], true))
      .toBe("Ordinary solve — Numerical health: 1 solve failed, 1 quality unresolved, 1 sensitive, 1 not assessed, 1 checks passed");
  });

  it("pins the pressure-source, output and boundary sentences", () => {
    expect(text.ELIGIBLE_RECOVERY_TEXT).toBe(
      "The values shown come from the producer's recovery of this case, which passed its own checks. This badge describes the ordinary solve only."
    );
    expect(text.outputSentence(2)).toBe(
      "No results of this run can be exported or used by rule checks while 2 load case(s) are not checks passed."
    );
    expect(text.BOUNDARY_SENTENCE).toBe(
      "These checks look for numerical trouble; they do not measure the error, and they say nothing about whether the model, its inputs or its design are right."
    );
  });

  it("pins every detail text", () => {
    expect(text.checksPassedText("LC1")).toBe(
      "The solver's residual and conditioning checks passed for LC1. These checks look for numerical trouble; they do not measure the error, and they say nothing about whether the model, its inputs or its design are right."
    );
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
    expect(text.notAssessedText("LC1")).toBe("The solver did not assess the numerical quality of LC1.");
    expect(text.combinationText([["LC1", "sensitive"], ["LC2", "checks_passed"]])).toBe(
      "Inherited from its load cases: LC1: sensitive; LC2: checks passed."
    );
    expect(text.conditioningDetailText("10.5")).toBe(
      "Estimated sensitivity of the scaled linear solve: rounding can be magnified by roughly 10.5 of the about 16 decimal digits the computer carries (the solver marks a case sensitive at about 7.8). This is an estimate; it does not estimate the error in individual results."
    );
    expect(text.DIGITS_BELOW_FIRST_DECIMAL).toBe("below 0.1");
    expect(text.nodeDofLocationText("N1", "UX")).toBe("node N1, UX");
    expect(text.SHOW_SOLVER_DIAGNOSTIC).toBe("Show solver diagnostic");
    expect(text.LOCATED_CAUSES_LABEL).toBe("Named locations");
    expect(text.DETAIL_SECTION_LABEL).toBe("Numerical health by load case");
    expect(text.reportLineText("checks_passed")).toBe("Numerical health of the ordinary solve: checks passed.");
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
  it("never uses a fenced word anywhere in the text module", () => {
    const source = readFileSync(path.resolve(__dirname, "numericalHealthText.ts"), "utf8");
    for (const word of ["verified", "certified", "bound", "guaranteed", "valid", "approved", "compliant", "correct", "reliable"]) {
      expect(source, word).not.toMatch(new RegExp(`\\b${word}\\b`, "i"));
    }
  });
});
