/** Desktop output availability for gated result routes (T1 WP2).
 *
 * ROOT deferred every desktop output of load/reference-state results to T6
 * (T1_WAVE1_RULINGS.md section 12): no file, package, handoff or external
 * request may carry their result data yet. B7b retired load-reference-1 and
 * load-reference-source-1 with the v2 contract: a load/reference-state result is
 * now published under pressure-2 or pressure-source-1, both of which refuse on
 * every surface, each with its own reason. Every such surface calls this one
 * function and shows the route's one reason; the report package keeps T0R's own
 * fresh-result refusal (REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE). The statement
 * is about the desktop route only: the result stays readable, and nothing here
 * is a finding about the result or its evidence. Text only; it never changes
 * standing, rows or bytes.
 *
 * U6d (D-U6-8; D2 4.9.6): the F2a preview successor's desktop outputs are
 * likewise T6's. The same shared function returns its own reason for it.
 *
 * T6S-3 (RR decisions 2 and 12): the reasons come from the per-route output
 * policy (`outputPolicy.ts`), and this function's meaning is unchanged: it returns
 * the route's shared refusal on every surface that calls it. The eighteen surfaces
 * behind `LoadReferenceOutputGate` keep calling it. The Result Export and
 * Stress-Neutral Export panels no longer call it: they read the policy's
 * per-surface decision, which admits an eligible successor there only.
 */
import { routeOutputRefusal } from "./outputPolicy";
import type { MechanicsResult } from "../../types";

export {
  PRESSURE_2_OUTPUT_REFUSAL, PRESSURE_SOURCE_OUTPUT_REFUSAL,
  RETAINED_PRECISION_OUTPUT_NOT_YET_AVAILABLE, N_RETAINED_PRECISION_OUTPUT, RETAINED_PRECISION_OUTPUT_REFUSAL,
  N_RETAINED_PHYSICS_OUTPUT, RETAINED_PHYSICS_OUTPUT_REFUSAL,
} from "./outputPolicy";

/** The shared output refusal: the reason for a gated route's result (pressure-2,
 * pressure-source-1 or a retained-precision successor), else null. */
export function loadReferenceOutputRefusal(source: MechanicsResult | null | undefined): string | null {
  return routeOutputRefusal(source);
}
/** Throws the shared refusal for a gated route's result. */
export function refuseLoadReferenceOutput(source: MechanicsResult | null | undefined): void {
  const refusal = loadReferenceOutputRefusal(source);
  if (refusal) throw new Error(refusal);
}
