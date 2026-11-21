/** Release exit criterion 18 (LIM-026): how the desktop states the solver's integrity verdict.
 *
 * The solver's integrity check rests on a step the product does not verify for any result, so
 * the reader's verdict carries `integrity_bound` as not verified on every route (Rust
 * `reader_verdict::integrity_bound`). Wherever a person reads the solver's verdict, the desktop
 * states it as the solver's report, not verified: the health state word (`numericalHealthText.ts`),
 * the standing status words, the results line, the note beside the solver's own text, and the
 * support-state line. Admission is unchanged: a result the reader admits stays admitted, and the
 * admission asserts no bound. Every literal here is presentation only; nothing here reads into
 * standing, admission or an output gate. */
import type { MechanicsResult } from "../../types";
import type { StandingStatus } from "./resultReader";
import { NUMERICAL_HEALTH_STATE_TEXT } from "./numericalHealthText";

/** The solver's integrity code for a case its own checks found nothing against. */
export const CHECKS_PASSED_CODE = "NUMERICAL_INTEGRITY_CHECKS_PASSED";
/** The start of the solver's confirmed support-state line in a nonlinear case's integrity message. */
export const SUPPORT_STATE_ESTABLISHED_TEXT = "Converged support state: established at 128-bit precision";

/** Shown beside the solver's own text wherever that text reports a check passed or a state
 * established: the solver's text stays the solver's, and is never shown alone. */
export const SOLVER_REPORT_NOTE = "Reported by the solver; not verified by SWBPIPE.";

/** Whether a diagnostic shown to a person needs `SOLVER_REPORT_NOTE` beside it: its code is the
 * solver's checks-passed code, or its message states that code or the solver's confirmed
 * support-state line. */
export function needsSolverReportNote(diagnostic: Readonly<{ code?: unknown; message?: unknown }> | null | undefined): boolean {
  if (!diagnostic) return false;
  if (diagnostic.code === CHECKS_PASSED_CODE) return true;
  const message = typeof diagnostic.message === "string" ? diagnostic.message : "";
  return message.includes(CHECKS_PASSED_CODE) || message.includes(SUPPORT_STATE_ESTABLISHED_TEXT);
}


/** The note for a result's published limitations, when one states the solver's checks-passed code; null
 * otherwise. The limitations are shown whole and in order, and the note follows them. */
export function limitationsNote(limitations: readonly string[]): string | null {
  return limitations.some((text) => needsSolverReportNote({ message: text })) ? SOLVER_REPORT_NOTE : null;
}
/** The limitations joined by single spaces, followed by `limitationsNote` where it applies. */
export function withLimitationsNote(limitations: readonly string[]): string {
  const note = limitationsNote(limitations);
  return note ? `${limitations.join(" ")} ${note}` : limitations.join(" ");
}

/** The words for the reader's standing status where the desktop shows it to a person. */
export function standingStatusText(status: StandingStatus): string {
  return status === "solver_checks_passed" ? "solver checks passed, not verified" : status;
}


/** The status word of `numerical_quality.status` on the results line: the solver's `checks_passed`
 * reads as its health state word ("not verified"); every other status is shown as it is. */
export function numericalQualityStatusText(status: string): string {
  return status === "checks_passed" ? NUMERICAL_HEALTH_STATE_TEXT.checks_passed : status;
}

/** "Numerical health: {status}." on the results line. */
export function numericalHealthLead(result: Pick<MechanicsResult, "numerical_quality">): string {
  return `Numerical health: ${numericalQualityStatusText(String(result.numerical_quality!.status))}.`;
}
/** The pressure-source route's "Ordinary solve: {status}." lead. */
export function ordinarySolveLead(result: Pick<MechanicsResult, "numerical_quality">): string {
  return `Ordinary solve: ${numericalQualityStatusText(String(result.numerical_quality!.status))}.`;
}

/** The reader's standing record's confirmation outcomes (`cases[].confirmation.outcome`,
 * open_pipe_stress_numerical_vocabulary `historical_confirmation`). */
export type ConfirmationOutcome =
  | "solver_reported_established" | "not_attempted_reason_not_recorded" | "not_established_detail_quoted"
  | "no_confirmation_record" | "not_applicable";

/** The report renderer's reviewed statement for each outcome (report_renderer `standing.rs`
 * `confirmation_statement`), as the desktop states it: where the report says "conditional on the
 * solver's inverse-norm step", the desktop says "reported by the solver; not verified by SWBPIPE".
 * The report keeps its own wording. */
export const CONFIRMATION_STATEMENTS: Readonly<Record<Exclude<ConfirmationOutcome, "not_applicable">, string>> = Object.freeze({
  solver_reported_established:
    "Not confirmed: the solver's confirmation reports every support state of this load case established, reported by the solver; not verified by SWBPIPE. This result does not publish the deciding quantities or their intervals.",
  not_attempted_reason_not_recorded: "Not confirmed: no confirmation was attempted for this load case; the result does not record why.",
  not_established_detail_quoted:
    "Not confirmed: the solver's confirmation did not establish the support states of this load case. The result publishes one reason only, quoted below; it does not show that any other support's state was established.",
  no_confirmation_record: "Not confirmed: the result records no support-state confirmation for this load case."
});
export const SUPPORT_STATES_LEAD = "Support states: not confirmed";

/** The support-state line of a case with nonlinear supports; null for a case without them or an
 * outcome outside the vocabulary. */
export function supportStatesText(outcome: unknown): string | null {
  if (typeof outcome !== "string" || !Object.hasOwn(CONFIRMATION_STATEMENTS, outcome)) return null;
  return `${SUPPORT_STATES_LEAD}. ${CONFIRMATION_STATEMENTS[outcome as keyof typeof CONFIRMATION_STATEMENTS]}`;
}

/** The support-state line of each case with nonlinear supports, by case id, from a standing record
 * (`numerical_standing`: `cases[].confirmation.outcome`) or a native summary's
 * `support_confirmations` (`[{case_id, outcome}]`). Anything else gives no line. */
export function supportStatesByCase(record: unknown): ReadonlyMap<string, string> {
  const out = new Map<string, string>();
  const cases = Array.isArray(record) ? record : (record as { cases?: unknown } | null)?.cases;
  if (!Array.isArray(cases)) return out;
  for (const item of cases) {
    const caseId = (item as { case_id?: unknown })?.case_id;
    const outcome = (item as { outcome?: unknown })?.outcome ?? (item as { confirmation?: { outcome?: unknown } })?.confirmation?.outcome;
    const text = supportStatesText(outcome);
    if (typeof caseId === "string" && text) out.set(caseId, text);
  }
  return out;
}
