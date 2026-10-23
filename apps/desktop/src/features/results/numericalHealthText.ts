/** Every user-visible literal of the numerical health indicator (DEL-07-05 Design
 * `numerical-health-indicator.md` §3-§5), copied from the design and kept in this one
 * module. Values in braces in the design are the parameters here. Nothing in this
 * module reads a result or decides a state: the states, the conditioning figure and the
 * verdict-only flag come from the Rust `numerical_health_json`; the callers pass them in. */

export type NumericalHealthState = "checks_passed" | "sensitive" | "quality_unresolved" | "solve_failed" | "not_assessed";

export const NUMERICAL_HEALTH_LABEL = "Numerical health";

/** The state names, as the headlines, badges and counts show them. */
export const NUMERICAL_HEALTH_STATE_TEXT: Readonly<Record<NumericalHealthState, string>> = Object.freeze({
  checks_passed: "checks passed",
  sensitive: "sensitive",
  quality_unresolved: "quality unresolved",
  solve_failed: "solve failed",
  not_assessed: "not assessed"
});

/** The pressure-source-1 headline prefix. */
export const ORDINARY_SOLVE_PREFIX = "Ordinary solve —";

/** "Numerical health: {state}", or with the "Ordinary solve —" prefix. */
export function numericalHealthHeadline(state: NumericalHealthState, ordinarySolvePrefix = false): string {
  const headline = `${NUMERICAL_HEALTH_LABEL}: ${NUMERICAL_HEALTH_STATE_TEXT[state]}`;
  return ordinarySolvePrefix ? `${ORDINARY_SOLVE_PREFIX} ${headline}` : headline;
}

/** The header's count form: non-zero counts in the order given; every case passing
 * reads "Numerical health: checks passed". */
export function numericalHealthCountHeadline(counts: ReadonlyArray<readonly [NumericalHealthState, number]>, ordinarySolvePrefix = false): string {
  const present = counts.filter(([, count]) => count > 0);
  if (present.length === 1 && present[0][0] === "checks_passed") return numericalHealthHeadline("checks_passed", ordinarySolvePrefix);
  const headline = `${NUMERICAL_HEALTH_LABEL}: ${present.map(([state, count]) => `${count} ${NUMERICAL_HEALTH_STATE_TEXT[state]}`).join(", ")}`;
  return ordinarySolvePrefix ? `${ORDINARY_SOLVE_PREFIX} ${headline}` : headline;
}

/** The pressure-source detail where the reader's standing is numerically eligible. */
export const ELIGIBLE_RECOVERY_TEXT =
  "The values shown come from the producer's recovery of this case, which passed its own checks. This badge describes the ordinary solve only.";

/** The header's output sentence, when the generic-route result is not Current because of the verdict. */
export function outputSentence(notPassedCount: number): string {
  return `No results of this run can be exported or used by rule checks while ${notPassedCount} load case(s) are not checks passed.`;
}

/** The boundary sentence (claims registry NQ-HEALTH, owner text pending). */
export const BOUNDARY_SENTENCE =
  "These checks look for numerical trouble; they do not measure the error, and they say nothing about whether the model, its inputs or its design are right.";

export function checksPassedText(caseId: string): string {
  return `The solver's residual and conditioning checks passed for ${caseId}. ${BOUNDARY_SENTENCE}`;
}

export function sensitiveLeadText(caseId: string): string {
  return `The solver's checks flagged ${caseId} as numerically sensitive.`;
}
export const KEPT_FOR_INSPECTION_TEXT = "Its values are kept for inspection.";
export const CONDITIONING_CAUSE_TEXT = "The scaled stiffness is poorly conditioned.";
/** The cause sentences name their locations in parentheses; with no named location
 * the parenthetical is left out. */
function named(locations: string): string {
  return locations ? ` (${locations})` : "";
}
export function loadFidelityCauseText(sources: string): string {
  return `The solver could not confirm that every applied load was carried at full precision${named(sources)}.`;
}
export function formationCauseText(rows: string): string {
  return `The solver's formation check could not confirm some stiffness or load terms at full precision${named(rows)}.`;
}
export function formationRecoveryCauseText(groups: string): string {
  return `The solver could not confirm some recovered member or support quantities at full precision${named(groups)}.`;
}
export const SEE_DIAGNOSTIC_TEXT = "See the solver diagnostic for the reason.";
export const SENSITIVE_CLOSING_TEXT =
  "Solving the unchanged model again in the same solver mode gives the same result. A very short or very stiff element next to flexible ones, stiffnesses that differ by many orders of magnitude, or a weakly restrained direction are common causes.";

export function qualityUnresolvedLeadText(caseId: string): string {
  return `The solver could not confirm the numerical quality of ${caseId} at working precision.`;
}
export const QUALITY_UNRESOLVED_CLOSING_TEXT = "A near-mechanism or extreme stiffness contrast are common causes.";
export const RECOVERY_BASIS_UNQUALIFIED_TEXT =
  "Its support-state solution is kept for inspection, but its numerical quality could not be confirmed.";

export function solveFailedLeadText(caseId: string): string {
  return `The solver could not complete a solution for ${caseId}.`;
}
export const MECHANISM_TEXT = "The model can move without resistance (a mechanism). Review connectivity, member releases and supports.";
export const NEGATIVE_ENERGY_TEXT = "The assembled stiffness is not positive in some direction. Review element and support properties.";

export function notAssessedText(caseId: string): string {
  return `The solver did not assess the numerical quality of ${caseId}.`;
}

/** "Inherited from its load cases: {case}: {state}; …". */
export function combinationText(cases: ReadonlyArray<readonly [string, NumericalHealthState]>): string {
  return `Inherited from its load cases: ${cases.map(([caseId, state]) => `${caseId}: ${NUMERICAL_HEALTH_STATE_TEXT[state]}`).join("; ")}.`;
}

/** The conditioning detail; `digits` is already formatted (`formatDigits`). */
export function conditioningDetailText(digits: string): string {
  return `Estimated sensitivity of the scaled linear solve: rounding can be magnified by roughly ${digits} of the about 16 decimal digits the computer carries (the solver marks a case sensitive at about 7.8). This is an estimate; it does not estimate the error in individual results.`;
}
/** The figure printed for d below 0.05 (including rcond = 1). */
export const DIGITS_BELOW_FIRST_DECIMAL = "below 0.1";

/** A K-D5 or S11-G row label, "node {id}, {direction}". */
export function nodeDofLocationText(node: string, direction: string): string {
  return `node ${node}, ${direction}`;
}

export const SHOW_SOLVER_DIAGNOSTIC = "Show solver diagnostic";
export const LOCATED_CAUSES_LABEL = "Named locations";
export const DETAIL_SECTION_LABEL = "Numerical health by load case";

/** The report and rendered-report line, one per load case. */
export function reportLineText(state: NumericalHealthState): string {
  return `Numerical health of the ordinary solve: ${NUMERICAL_HEALTH_STATE_TEXT[state]}.`;
}
