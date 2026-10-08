/** Desktop output availability for load/reference-state results (T1 WP2).
 *
 * ROOT deferred every desktop output of load-reference-1 and
 * load-reference-source-1 results to T6 (T1_WAVE1_RULINGS.md section 12): no
 * file, package, handoff or external request may carry their result data yet.
 * Every such surface calls this one function and shows this one reason; the
 * report package keeps T0R's own fresh-result refusal
 * (REPORT-PACKAGE-FRESH-RESULT-UNAVAILABLE). The statement is about the
 * desktop route only: the result stays readable, and nothing here is a
 * finding about the result or its evidence. Text only; it never changes
 * standing, rows or bytes.
 *
 * U6d (D-U6-8; D2 4.9.6): the F2a preview successor's desktop outputs are
 * likewise T6's. The same shared function returns its own reason for it;
 * `isLoadReferenceRoute` is unchanged, so the load-reference text is never
 * shown for a successor.
 *
 * T6S-3 (RR decisions 2 and 12): the reasons now come from the per-route output
 * policy (`outputPolicy.ts`), and this function's meaning is unchanged: it returns
 * the route's shared refusal on every surface that calls it. The eighteen surfaces
 * behind `LoadReferenceOutputGate` keep calling it. The Result Export and
 * Stress-Neutral Export panels no longer call it: they read the policy's
 * per-surface decision, which admits an eligible successor there only.
 */
import { routeOutputRefusal } from "./outputPolicy";
import { sourceContract } from "./numericalResultQuality";
import type { MechanicsResult } from "../../types";

export {
  LOAD_REFERENCE_OUTPUT_NOT_YET_AVAILABLE, N_LOAD_REFERENCE_OUTPUT, LOAD_REFERENCE_OUTPUT_REFUSAL,
  RETAINED_PRECISION_OUTPUT_NOT_YET_AVAILABLE, N_RETAINED_PRECISION_OUTPUT, RETAINED_PRECISION_OUTPUT_REFUSAL,
  N_RETAINED_PHYSICS_OUTPUT, RETAINED_PHYSICS_OUTPUT_REFUSAL,
} from "./outputPolicy";

/** Whether header dispatch selects a load/reference-state route. */
export function isLoadReferenceRoute(source: MechanicsResult | null | undefined): boolean {
  if (!source) return false;
  try {
    const route = sourceContract(source);
    return route === "load_reference" || route === "load_reference_source";
  } catch { return false; }
}
/** The shared output refusal: the reason for a load/reference-state or
 * retained-precision successor result, else null. */
export function loadReferenceOutputRefusal(source: MechanicsResult | null | undefined): string | null {
  return routeOutputRefusal(source);
}
/** Throws the shared refusal for a load/reference-state or successor result. */
export function refuseLoadReferenceOutput(source: MechanicsResult | null | undefined): void {
  const refusal = loadReferenceOutputRefusal(source);
  if (refusal) throw new Error(refusal);
}
