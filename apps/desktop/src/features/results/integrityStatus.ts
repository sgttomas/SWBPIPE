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
import type { IntegrityBound, StandingStatus } from "./resultReader";
import solverTextClaims from "./solverTextClaims.json";
import { NUMERICAL_HEALTH_STATE_TEXT } from "./numericalHealthText";

/** The solver's integrity code for a case its own checks found nothing against. */
export const CHECKS_PASSED_CODE = "NUMERICAL_INTEGRITY_CHECKS_PASSED";
/** The start of the solver's confirmed support-state line in a nonlinear case's integrity message. */
export const SUPPORT_STATE_ESTABLISHED_TEXT = "Converged support state: established at 128-bit precision";

/** Shown beside the solver's own text wherever that text reports a check passed, a bound or a state
 * established on the ordinary solve's unverified bound: the solver's text stays the solver's, and is
 * never shown alone. */
export const SOLVER_REPORT_NOTE = "Reported by the solver; not verified by SWBPIPE.";

/** The closed classification (`solverTextClaims.json`) of every diagnostic code the solve crates
 * publish (`fixtures/diagnostics/exact_route_codes.json`): whether its text states a pass, a bound
 * or an established state that rests on the ordinary solve's integrity bound. */
export type SolverTextClaim = "claim" | "quotes_claim" | "no_claim";
const CLAIM_CODES: ReadonlySet<string> = new Set(Object.keys(solverTextClaims.claim));
const QUOTING_CODES: ReadonlySet<string> = new Set(Object.keys(solverTextClaims.quotes_claim));
const NO_CLAIM_CODES: ReadonlySet<string> = new Set(solverTextClaims.no_claim);
/** The solver's fixed phrases that state such a claim. */
export const SOLVER_CLAIM_PHRASES: readonly string[] = Object.freeze(solverTextClaims.phrases.map((phrase) => phrase.text));

/** A code's class; null for a code outside the table. */
export function solverTextClaim(code: unknown): SolverTextClaim | null {
  if (typeof code !== "string") return null;
  return CLAIM_CODES.has(code) ? "claim" : QUOTING_CODES.has(code) ? "quotes_claim" : NO_CLAIM_CODES.has(code) ? "no_claim" : null;
}

/** Whether a text contains one of the solver's fixed claim phrases. */
function quotesClaim(text: unknown): boolean {
  return typeof text === "string" && SOLVER_CLAIM_PHRASES.some((phrase) => text.includes(phrase));
}

/** Whether a solver diagnostic, or a published text with no code (a limitation, a confirmation's
 * quoted detail), needs `SOLVER_REPORT_NOTE` beside it: it states a pass, a bound or an established
 * state that rests on the ordinary solve's unverified integrity bound. Decided by its code where the
 * code is in the table (`claim`: always; `no_claim`: never, which keeps the retained-precision and
 * pressure-source recovery certificates' own texts without it), and by the solver's fixed phrases for
 * a `quotes_claim` code, a code outside the table, and a text without a code. */
export function needsSolverReportNote(text: Readonly<{ code?: unknown; message?: unknown }> | null | undefined): boolean {
  if (!text) return false;
  switch (solverTextClaim(text.code)) {
    case "claim": return true;
    case "no_claim": return false;
    default: return quotesClaim(text.message);
  }
}

/** Each published limitation with the note directly after it where it states such a claim
 * (`needsSolverReportNote` on the text alone), in order and whole. */
export function limitationsWithNotes(limitations: readonly string[]): readonly Readonly<{ text: string; note: string | null }>[] {
  return limitations.map((text) => ({ text, note: needsSolverReportNote({ message: text }) ? SOLVER_REPORT_NOTE : null }));
}
/** The limitations joined by single spaces, each followed by the note where it applies. */
export function withLimitationsNote(limitations: readonly string[]): string {
  return limitationsWithNotes(limitations).map(({ text, note }) => (note ? `${text} ${note}` : text)).join(" ");
}

/** The words for the reader's standing status where the desktop shows it to a person: an admission
 * standing, which asserts no verified error bound. */
export function standingStatusText(status: StandingStatus): string {
  return status === "solver_checks_passed" ? "numerically eligible" : "needs recompute";
}
/** The reader verdict's `integrity_bound` member in words, for the ordinary solve. */
export const INTEGRITY_BOUND_NOT_VERIFIED_TEXT = "the ordinary solve's error bound is not verified";
/** The bound member's words; null where no reader verdict was read (the member is null). */
export function integrityBoundText(bound: IntegrityBound | null | undefined): string | null {
  return bound?.status === "not_verified" ? INTEGRITY_BOUND_NOT_VERIFIED_TEXT : null;
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

/** Where the quoted reason of `not_established_detail_quoted` was not carried to this surface, the
 * statement points to where it is instead of "quoted below". */
export const DETAIL_NOT_CARRIED_TEXT = "stated in this load case's solver diagnostic";

/** A case's support-state line and, for `not_established_detail_quoted`, the solver's quoted reason
 * shown directly beneath it, with the note where the reason states a claim (`needsSolverReportNote`). */
export type SupportStatesLine = Readonly<{ text: string; detail: string | null; detailNote: string | null }>;

/** The line of one outcome and its quoted detail; null for a case without nonlinear supports or an
 * outcome outside the vocabulary. */
export function supportStatesLine(outcome: unknown, detail: unknown): SupportStatesLine | null {
  const text = supportStatesText(outcome);
  if (text === null) return null;
  if (outcome !== "not_established_detail_quoted") return { text, detail: null, detailNote: null };
  if (typeof detail !== "string" || detail.length === 0) return { text: text.replace("quoted below", DETAIL_NOT_CARRIED_TEXT), detail: null, detailNote: null };
  return { text, detail, detailNote: needsSolverReportNote({ message: detail }) ? SOLVER_REPORT_NOTE : null };
}

/** The support-state line of each case with nonlinear supports, by case id, from a standing record
 * (`numerical_standing`: `cases[].confirmation.{outcome, detail}`) or a native summary's
 * `support_confirmations` (`[{case_id, outcome, detail}]`). Anything else gives no line. */
export function supportStatesByCase(record: unknown): ReadonlyMap<string, SupportStatesLine> {
  const out = new Map<string, SupportStatesLine>();
  const cases = Array.isArray(record) ? record : (record as { cases?: unknown } | null)?.cases;
  if (!Array.isArray(cases)) return out;
  for (const item of cases) {
    const caseId = (item as { case_id?: unknown })?.case_id;
    const confirmation = (item as { confirmation?: { outcome?: unknown; detail?: unknown } })?.confirmation;
    const outcome = (item as { outcome?: unknown })?.outcome ?? confirmation?.outcome;
    const detail = (item as { detail?: unknown })?.detail ?? confirmation?.detail;
    const line = supportStatesLine(outcome, detail);
    if (typeof caseId === "string" && line) out.set(caseId, line);
  }
  return out;
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
