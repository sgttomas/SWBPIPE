/** How the desktop words what the solver's checks found (release exit criterion 18, LIM-026).
 *
 * The solver's integrity check rests on its own error estimate, which SWBPIPE does not check, so the
 * desktop says what the checks found and never calls the result passed or bounded: the standing
 * status words, the results line, the support-state line of each case with nonlinear supports, and
 * one heading sentence above the solver's own messages. Every literal here is presentation only;
 * nothing here reads into standing, admission or an output gate. */
import type { MechanicsResult } from "../../types";
import type { StandingStatus } from "./resultReader";
import { NUMERICAL_HEALTH_STATE_TEXT, type NumericalHealthState } from "./numericalHealthText";

/** Shown once where the desktop lists the solver's own messages: at the head of the list, or in
 * Design Knowledge directly before the first record made from one. */
export const SOLVER_MESSAGES_HEADING = "Messages from the solver are the solver's own statements. Where one rests on the solver's error estimate, SWBPIPE has not independently checked that estimate.";

/** The words for the reader's standing status where the desktop shows it to a person: an admission
 * standing, which asserts no verified error bound. */
export function standingStatusText(status: StandingStatus): string {
  return status === "solver_checks_passed" ? "numerically eligible" : "needs recompute";
}
/** The words of a `numerical_quality.status` token: the solver's `checks_passed` reads as its health
 * state words ("solver checks found no problem"); every other status is shown as it is. */
export function numericalQualityStatusText(status: string): string {
  return status === "checks_passed" ? NUMERICAL_HEALTH_STATE_TEXT.checks_passed : status;
}

/** The indicator's states from best to worst: no line and no headline reads better than the worse
 * of the document's own status and the gated health. */
export const HEALTH_STATES_BEST_FIRST: readonly NumericalHealthState[] = Object.freeze(["checks_passed", "not_assessed", "sensitive", "quality_unresolved", "solve_failed"]);
/** The indicator's state for a document's `numerical_quality.status` token, read as the indicator
 * reads a case's `solve_quality` (report_standing `historical_health.rs` `state_of`); a token it has
 * no state for, or none, is not assessed. */
export function documentStatusState(status: unknown): NumericalHealthState {
  switch (status) {
    case "checks_passed": return "checks_passed";
    case "sensitive": return "sensitive";
    case "unresolved": return "quality_unresolved";
    case "failed": return "solve_failed";
    default: return "not_assessed";
  }
}
/** The worse of two states (`HEALTH_STATES_BEST_FIRST`). */
export function worseState(a: NumericalHealthState, b: NumericalHealthState): NumericalHealthState {
  return HEALTH_STATES_BEST_FIRST.indexOf(a) >= HEALTH_STATES_BEST_FIRST.indexOf(b) ? a : b;
}
/** The results line's state words: the worse of the document's own status and the gated health's
 * worst state (`gatedState`; not assessed where the gate yields no health), in the indicator's
 * words. It reads "solver checks found no problem" only where both say checks passed, and never
 * shows a raw token. */
export function leadStateText(result: Pick<MechanicsResult, "numerical_quality">, gated: NumericalHealthState): string {
  return NUMERICAL_HEALTH_STATE_TEXT[worseState(documentStatusState(result.numerical_quality?.status), gated)];
}
/** "Numerical health: {state}." on the results line. */
export function numericalHealthLead(result: Pick<MechanicsResult, "numerical_quality">, gated: NumericalHealthState): string {
  return `Numerical health: ${leadStateText(result, gated)}.`;
}
/** The pressure-source route's "Ordinary solve: {state}." lead. */
export function ordinarySolveLead(result: Pick<MechanicsResult, "numerical_quality">, gated: NumericalHealthState): string {
  return `Ordinary solve: ${leadStateText(result, gated)}.`;
}

/** The reader's standing record's confirmation outcomes (`cases[].confirmation.outcome`,
 * open_pipe_stress_numerical_vocabulary `historical_confirmation`). */
export type ConfirmationOutcome =
  | "solver_reported_established" | "not_attempted_reason_not_recorded" | "not_established_detail_quoted"
  | "no_confirmation_record" | "not_applicable";

/** The support-state line of each confirmation outcome, as the desktop states it. The report
 * renderer keeps its own reviewed wording (report_renderer `standing.rs` `confirmation_statement`). */
export const CONFIRMATION_STATEMENTS: Readonly<Record<Exclude<ConfirmationOutcome, "not_applicable">, string>> = Object.freeze({
  solver_reported_established:
    "Support states: reported established by the solver. Its confirmation rests on the solver's own error estimate, which SWBPIPE has not independently checked. This result does not publish the deciding quantities or their intervals.",
  not_attempted_reason_not_recorded: "Support states: not confirmed. No confirmation was attempted for this load case; the result does not record why.",
  not_established_detail_quoted:
    "Support states: not confirmed. The solver's confirmation did not establish the support states of this load case. The result publishes one reason only, quoted below; it does not show that any other support's state was established.",
  no_confirmation_record: "Support states: not confirmed. The result records no support-state confirmation for this load case."
});

/** The support-state line of a case with nonlinear supports; null for a case without them or an
 * outcome outside the vocabulary. */
export function supportStatesText(outcome: unknown): string | null {
  if (typeof outcome !== "string" || !Object.hasOwn(CONFIRMATION_STATEMENTS, outcome)) return null;
  return CONFIRMATION_STATEMENTS[outcome as keyof typeof CONFIRMATION_STATEMENTS];
}

/** Where the quoted reason of `not_established_detail_quoted` was not carried to this surface, the
 * statement points to where it is instead of "quoted below". */
export const DETAIL_NOT_CARRIED_TEXT = "stated in this load case's solver diagnostic";

/** A case's support-state line and, for `not_established_detail_quoted`, the solver's quoted reason
 * shown directly beneath it. */
export type SupportStatesLine = Readonly<{ text: string; detail: string | null }>;

/** The line of one outcome and its quoted detail; null for a case without nonlinear supports or an
 * outcome outside the vocabulary. */
export function supportStatesLine(outcome: unknown, detail: unknown): SupportStatesLine | null {
  const text = supportStatesText(outcome);
  if (text === null) return null;
  if (outcome !== "not_established_detail_quoted") return { text, detail: null };
  if (typeof detail !== "string" || detail.length === 0) return { text: text.replace("quoted below", DETAIL_NOT_CARRIED_TEXT), detail: null };
  return { text, detail };
}

/** The desktop's own line for a case whose confirmation outcome could not be read here: not one of
 * the reader's outcomes (`ConfirmationOutcome`, `CONFIRMATION_STATEMENTS`) and kept apart from that
 * vocabulary. It stands where the desktop has not positively established that a case has nothing to
 * confirm: the confirmation read refused or failed, was not made, the record is unreadable under
 * today's reader, or no model is available to tell whether the model has a nonlinear support. */
export const SUPPORT_STATES_UNREAD_LINE: SupportStatesLine = Object.freeze({
  text: "Support states: not read here. This result's record of what the solver reported about this load case's support states could not be read.",
  detail: null
});

/** What the desktop knows of a result's support-state confirmations, for the indicator's cases: the
 * line (or, where the reader's record gives the case an outcome that takes none, null) of each case
 * the record names, and the line of every other case. Fail closed: a case shows no line only where
 * the record gives it an outcome that takes none, or the model the result is read with is known to
 * have no nonlinear support. */
export type SupportStates = Readonly<{ byCase: ReadonlyMap<string, SupportStatesLine | null>; otherwise: SupportStatesLine | null }>;
/** The model the result is read with is known and has no nonlinear support: no case has a line. */
export const SUPPORT_STATES_NONE: SupportStates = Object.freeze({ byCase: new Map<string, SupportStatesLine | null>(), otherwise: null });
/** No confirmation outcome could be read: every case the indicator lists has the could-not-be-read line. */
export const SUPPORT_STATES_UNREAD: SupportStates = Object.freeze({ byCase: new Map<string, SupportStatesLine | null>(), otherwise: SUPPORT_STATES_UNREAD_LINE });
/** A case's line under what the desktop knows. */
export function supportStatesOf(states: SupportStates, caseId: string): SupportStatesLine | null {
  return states.byCase.has(caseId) ? states.byCase.get(caseId)! : states.otherwise;
}

/** What a model says about nonlinear supports, as the reader's standing record reads it
 * (`supports[].nonlinear`): `"none"` only for a model whose `supports` is a list naming none;
 * `"unknown"` for no model or one whose supports cannot be read. */
export type NonlinearSupportKnowledge = "nonlinear" | "none" | "unknown";
export function nonlinearSupportKnowledge(model: unknown): NonlinearSupportKnowledge {
  if (!model || typeof model !== "object" || Array.isArray(model)) return "unknown";
  const supports = (model as { supports?: unknown }).supports;
  if (!Array.isArray(supports)) return "unknown";
  return supports.some((support) => (support as { nonlinear?: unknown } | null)?.nonlinear != null) ? "nonlinear" : "none";
}

/** The support-state line of each case a standing record (`numerical_standing`:
 * `cases[].confirmation.{outcome, detail}`) or a native summary's `support_confirmations`
 * (`[{case_id, outcome, detail}]`) names: the reader's statement, or null where the outcome takes no
 * line (`not_applicable`). An outcome outside the vocabulary has the could-not-be-read line, and so
 * has every case the record does not name. Anything that is not a record is none read. */
export function supportStatesByCase(record: unknown): SupportStates {
  const cases = Array.isArray(record) ? record : (record as { cases?: unknown } | null)?.cases;
  if (!Array.isArray(cases)) return SUPPORT_STATES_UNREAD;
  const byCase = new Map<string, SupportStatesLine | null>();
  for (const item of cases) {
    const caseId = (item as { case_id?: unknown })?.case_id;
    const confirmation = (item as { confirmation?: { outcome?: unknown; detail?: unknown } })?.confirmation;
    const outcome = (item as { outcome?: unknown })?.outcome ?? confirmation?.outcome;
    const detail = (item as { detail?: unknown })?.detail ?? confirmation?.detail;
    if (typeof caseId !== "string") continue;
    byCase.set(caseId, outcome === "not_applicable" ? null : supportStatesLine(outcome, detail) ?? SUPPORT_STATES_UNREAD_LINE);
  }
  return Object.freeze({ byCase, otherwise: SUPPORT_STATES_UNREAD_LINE });
}

/** What the desktop knows for a result read with `model` (null where none is available), where
 * `read` gives the reader's standing record or a native summary's confirmations, or throws, or gives
 * null where none could be read: no line only where the model is known to have no nonlinear support,
 * or the record gives the case an outcome that takes none; otherwise the could-not-be-read line. */
export function supportStatesFor(model: unknown, read: () => unknown): SupportStates {
  if (nonlinearSupportKnowledge(model) === "none") return SUPPORT_STATES_NONE;
  try { return supportStatesByCase(read()); } catch { return SUPPORT_STATES_UNREAD; }
}

/** What a native summary says of the confirmations: its `support_confirmations` (`confirmations`)
 * and, for a restored run, how they were read (`NativeCustody.supportConfirmations`); absent for a
 * solve, whose S is read with its own model, so its record stands, decided with `model` (null where
 * none is available) as `supportStatesFor` decides. */
export function nativeSupportStates(confirmations: unknown, read: "read" | "no_nonlinear_support" | "not_read" | undefined, model: unknown = null): SupportStates {
  if (read === "no_nonlinear_support") return SUPPORT_STATES_NONE;
  if (read === "not_read") return SUPPORT_STATES_UNREAD;
  if (read === "read") return supportStatesByCase(confirmations);
  return supportStatesFor(model, () => confirmations);
}

/** The method `retained_source_blocks_exact_v1` a pressure-source case's values are recovered by
 * (source_recovery.rs). */
export const SOURCE_RECOVERY_METHOD = "retained_source_blocks_exact_v1";
/** The load cases of a pressure-source result whose published values come from the producer's
 * recovery: the method each case's `contract_evidence.exact_cases[].recovery_method` records, which
 * the pressure-source reader requires to equal the receipt's `selected_method` for that case. A case
 * the ordinary solve alone produced (an ordinary companion) records an ordinary method and is not
 * among them. The member is carried whole in a native summary's envelope. */
export function recoveredCaseIds(result: Readonly<{ contract_evidence?: unknown }> | null | undefined): ReadonlySet<string> {
  const cases = (result?.contract_evidence as { exact_cases?: unknown } | null | undefined)?.exact_cases;
  const out = new Set<string>();
  if (!Array.isArray(cases)) return out;
  for (const item of cases) {
    const id = (item as { load_case_id?: unknown })?.load_case_id, method = (item as { recovery_method?: unknown })?.recovery_method;
    if (typeof id === "string" && method === SOURCE_RECOVERY_METHOD) out.add(id);
  }
  return out;
}
