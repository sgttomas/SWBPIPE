// The numerical health the Analyze page's status line reports for a held result
// (DEL-07-05 REQ-023): the worse of the gated reading and the document's own quality
// status, worst first in NUMERICAL_HEALTH_ORDER. Display only; it decides nothing.
import { NUMERICAL_HEALTH_ORDER, worstState, type NumericalHealthState, type NumericalHealthView } from "../results/numericalHealth";

export type StatusLineHealthInput = Readonly<{
  /** The result's recorded mechanics status. */
  mechanics: string;
  /** The document's `numerical_quality.status`; undefined where the document carries none. */
  documentStatus?: string;
  /** A native handle carries no document status: its gated reading alone stands. */
  native: boolean;
}>;

/** Unless the mechanics are solved, no reading may say checks passed or sensitive: a failed
 * or unresolved verdict stands, anything else reads not assessed. Applied to both inputs. */
function gate(state: NumericalHealthState, solved: boolean): NumericalHealthState {
  if (solved) return state;
  return state === "solve_failed" || state === "quality_unresolved" ? state : "not_assessed";
}

/** The document's quality status in the indicator's states; anything else, or none, is not assessed. */
export function documentHealthState(status: string | undefined): NumericalHealthState {
  switch (status) {
    case "checks_passed": return "checks_passed";
    case "sensitive": return "sensitive";
    case "unresolved": return "quality_unresolved";
    case "solve_failed":
    case "failed": return "solve_failed";
    default: return "not_assessed";
  }
}

export function statusLineHealth(input: StatusLineHealthInput, health: NumericalHealthView | null): NumericalHealthState {
  const solved = input.mechanics === "MECHANICS_SOLVED";
  const gated = gate(health && health.cases.length > 0 ? worstState(health.cases) : "not_assessed", solved);
  if (input.native) return gated;
  const documented = gate(documentHealthState(input.documentStatus), solved);
  return NUMERICAL_HEALTH_ORDER.indexOf(documented) < NUMERICAL_HEALTH_ORDER.indexOf(gated) ? documented : gated;
}
