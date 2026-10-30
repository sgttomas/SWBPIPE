/** Desktop output availability for gated result routes (T1 WP2).
 *
 * ROOT deferred every desktop output of load/reference-state results to T6
 * (T1_WAVE1_RULINGS.md section 12). The owner's ruling of 2026-10-21 (epoch 8, ruling
 * 11B: "A accept your recommendations for the entire set.") ends that deferral for the
 * current exact identities: a load/reference-state result is published under
 * pressure-2 or pressure-source-1 since B7b, and follows the Rust reader there (DEL-04-07
 * Design `desktop-exact-outputs.md` §5.4).
 *
 * This function returns the route's shared refusal, whatever the surface: the reason a
 * gated route carries, or null. It has no remaining product caller: the rendered
 * report reads its own `rendered-report` cell, which admits pressure-2 and
 * pressure-source-1 since W-11 (DEL-08-01 Design `exact-route-report-sections.md`
 * §5.1; owner, 2026-10-26). The surfaces behind `LoadReferenceOutputGate` read
 * their own per-surface decision (`surfaceRouteRefusal`), and the Result Export,
 * Stress-Neutral Export and Rendered Report panels read theirs (`surfaceOutputRefusal`),
 * as does the report package (its `report-package` cell,
 * `reportPackageUnavailableReason`). The
 * statement is about the desktop route only: the result stays readable, and nothing
 * here is a finding about the result or its evidence. Text only; it never changes
 * standing, rows or bytes.
 *
 * U6d (D-U6-8; D2 4.9.6): the F2a preview successor's desktop outputs are
 * likewise T6's. The same shared function returns its own reason for it.
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
