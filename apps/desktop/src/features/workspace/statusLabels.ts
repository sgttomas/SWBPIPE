// Registered display forms for the software status and evidence tokens.
//
// This table is the display-form table of docs/claims_registry.md §2
// (DEC-102): exactly eight rows, each with the authority domain that owns the
// token (docs/SPEC.md §4.3). Rules carried here:
//   - a label is never shown without its token reachable in place;
//   - a label is always shown with its authority domain;
//   - no registered label exists outside this table (ENGINEER_ACCEPTED stays reserved and
//     has no display form).
// Values that are not among the eight tokens are not status labels. They are
// made readable by `readableStatusValue` and never given a registered form.
// The owner-adopted Solver fallback below is display-only, not a registry entry.

export type StatusAuthorityDomain = "Solver" | "Rule pack" | "Human" | "Evidence";

export type RegisteredStatusToken =
  | "MODEL_INCOMPLETE"
  | "MECHANICS_SOLVED"
  | "RULE_INPUTS_INCOMPLETE"
  | "USER_RULE_CHECKED"
  | "USER_RULE_FAILED"
  | "HUMAN_REVIEW_REQUIRED"
  | "INTERNALLY_VERIFIED"
  | "PROVER_CORRELATED";

export type RegisteredStatusLabel = {
  token: RegisteredStatusToken;
  label: string;
  domain: StatusAuthorityDomain;
  kind: "status" | "evidence";
};

export const REGISTERED_STATUS_LABELS: Readonly<Record<RegisteredStatusToken, RegisteredStatusLabel>> = Object.freeze({
  MODEL_INCOMPLETE: { token: "MODEL_INCOMPLETE", label: "Model incomplete", domain: "Solver", kind: "status" },
  MECHANICS_SOLVED: { token: "MECHANICS_SOLVED", label: "Mechanics solved", domain: "Solver", kind: "status" },
  RULE_INPUTS_INCOMPLETE: {
    token: "RULE_INPUTS_INCOMPLETE",
    label: "Rule inputs incomplete",
    domain: "Rule pack",
    kind: "status"
  },
  USER_RULE_CHECKED: { token: "USER_RULE_CHECKED", label: "User-rule checked", domain: "Rule pack", kind: "status" },
  USER_RULE_FAILED: { token: "USER_RULE_FAILED", label: "User-rule failed", domain: "Rule pack", kind: "status" },
  HUMAN_REVIEW_REQUIRED: {
    token: "HUMAN_REVIEW_REQUIRED",
    label: "Human review required",
    domain: "Human",
    kind: "status"
  },
  INTERNALLY_VERIFIED: {
    token: "INTERNALLY_VERIFIED",
    label: "Internally verified",
    domain: "Evidence",
    kind: "evidence"
  },
  PROVER_CORRELATED: { token: "PROVER_CORRELATED", label: "Prover correlated", domain: "Evidence", kind: "evidence" }
});

/** The registered row for a recorded value, or null when the value is not one of the eight tokens. */
export function registeredStatusLabel(value: string): RegisteredStatusLabel | null {
  const token = value.trim().toUpperCase();
  return Object.prototype.hasOwnProperty.call(REGISTERED_STATUS_LABELS, token)
    ? REGISTERED_STATUS_LABELS[token as RegisteredStatusToken]
    : null;
}

/**
 * Domain and label ("Solver · Mechanics solved"). Use only where the raw token
 * is reachable in the same place (for example the status pill, whose body
 * shows the recorded token). Unregistered values fall back to readable text.
 */
export function statusDisplay(value: string): string {
  const row = registeredStatusLabel(value);
  return row ? `${row.domain} · ${row.label}` : readableStatusValue(value);
}

/**
 * Domain, label and the token itself ("Solver · Mechanics solved
 * (MECHANICS_SOLVED)"), for text runs that have no other place for the token.
 */
export function statusDisplayWithToken(value: string): string {
  const row = registeredStatusLabel(value);
  return row ? `${row.domain} · ${row.label} (${row.token})` : readableStatusValue(value);
}

/** Readable text for a value that is not a registered token. Never a registered form. */
export function readableStatusValue(value: string): string {
  const token = value.toLowerCase();
  if (token === "not_run" || token === "not_computed") return "Not run";
  if (token === "computed_for_invented_demo") return "Demo computed";
  return value.replace(/_/g, " ").toLowerCase();
}

// The preview fixtures record two legacy enum values where the product has
// always displayed a software status token. These map the recorded value to
// that token; they add no label.
export function ruleCheckStatusToken(value: string): string {
  return value.toLowerCase() === "not_performed_user_rule_inputs_missing" ? "RULE_INPUTS_INCOMPLETE" : value;
}

export function professionalStatusToken(value: string): string {
  return value.toLowerCase() === "not_provided" ? "HUMAN_REVIEW_REQUIRED" : value;
}

/** Owner-adopted display fallback, not a ninth registered authority token. */
export const SOLVER_NOT_SOLVED = Object.freeze({ label: "Not solved", domain: "Solver" as const });

export function solverDisplayWithToken(value: string): string {
  const row = registeredStatusLabel(value);
  return row?.domain === "Solver" && row.kind === "status"
    ? `${row.domain} · ${row.label} (${value})`
    : `${SOLVER_NOT_SOLVED.domain} · ${SOLVER_NOT_SOLVED.label} (${value})`;
}

/** Known absence/readiness values do not assert a failed or blocked solve. */
export function hasRecordedUnsolvedModelStatus(value: string | null): value is string {
  return value !== null && value.trim() !== "" &&
    !["NOT_RUN", "NOT_COMPUTED", "READY", "READY_FOR_PREVIEW_DIAGNOSTICS", "MECHANICS_SOLVED"].includes(value.trim().toUpperCase());
}
