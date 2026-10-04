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
 */
import { sourceContract } from "./numericalResultQuality";
import type { MechanicsResult } from "../../types";

export const LOAD_REFERENCE_OUTPUT_NOT_YET_AVAILABLE = "LOAD-REFERENCE-OUTPUT-NOT-YET-AVAILABLE";
export const N_LOAD_REFERENCE_OUTPUT = "Output of load/reference-state results (load-reference-1 and load-reference-source-1) is not yet available on the desktop; it is routed to T6. The result remains readable here; this is not a finding about the result.";
export const LOAD_REFERENCE_OUTPUT_REFUSAL = `${LOAD_REFERENCE_OUTPUT_NOT_YET_AVAILABLE}: ${N_LOAD_REFERENCE_OUTPUT}`;
export const RETAINED_PRECISION_OUTPUT_NOT_YET_AVAILABLE = "RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE";
export const N_RETAINED_PRECISION_OUTPUT = "Output of retained-precision results (preview-physics-retained-1) is not yet available on the desktop; it is routed to T6. The result remains readable here; this is not a finding about the result.";
export const RETAINED_PRECISION_OUTPUT_REFUSAL = `${RETAINED_PRECISION_OUTPUT_NOT_YET_AVAILABLE}: ${N_RETAINED_PRECISION_OUTPUT}`;

/** Whether header dispatch selects a load/reference-state route. */
export function isLoadReferenceRoute(source: MechanicsResult | null | undefined): boolean {
  if (!source) return false;
  try {
    const route = sourceContract(source);
    return route === "load_reference" || route === "load_reference_source";
  } catch { return false; }
}
function isRetainedPrecisionRoute(source: MechanicsResult | null | undefined): boolean {
  try { return sourceContract(source!) === "retained_preview_physics"; } catch { return false; }
}
/** The shared output refusal: the reason for a load/reference-state or
 * retained-precision successor result, else null. */
export function loadReferenceOutputRefusal(source: MechanicsResult | null | undefined): string | null {
  return isLoadReferenceRoute(source) ? LOAD_REFERENCE_OUTPUT_REFUSAL : isRetainedPrecisionRoute(source) ? RETAINED_PRECISION_OUTPUT_REFUSAL : null;
}
/** Throws the shared refusal for a load/reference-state or successor result. */
export function refuseLoadReferenceOutput(source: MechanicsResult | null | undefined): void {
  const refusal = loadReferenceOutputRefusal(source);
  if (refusal) throw new Error(refusal);
}
