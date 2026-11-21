/** How the numerical-health indicator and the result's standing look, state by state. The words are
 * the indicator's own (`numericalHealthText.ts`, `integrityStatus.ts`); this file forms none. */
import { Circle } from "lucide-react";
import type { NumericalHealthState } from "./numericalHealthText";

/** How a numerical-health state looks: checked with nothing found (a hollow ring, ordinary text),
 * a check that found trouble (the warning or blocking badge, as built), not checked (no mark, muted
 * words). Never a tick, a shield, green or "passed". */
export type NumericalHealthLook = "checked" | "trouble" | "unchecked";
export function healthLookOf(state: NumericalHealthState): NumericalHealthLook {
  if (state === "checks_passed") return "checked";
  return state === "not_assessed" ? "unchecked" : "trouble";
}
/** The mark of a look, where it has one: the hollow ring of a check that found nothing. */
export function HealthLookMark({ look }: { look: NumericalHealthLook | null }) {
  return look === "checked" ? <Circle className="numerical-health-ring" size={10} strokeWidth={2.25} aria-hidden="true" /> : null;
}

