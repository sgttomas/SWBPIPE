/** The desktop's result-route vocabulary: the semantic contract identities its notices
 * and policies name, its route names, and the successor carrier's status mapping.
 * R2 (b7-plan Unit R): the route of a statement, its table binding, its downgrade
 * refusal and its standing are the Rust reader's verdict (`resultReader.ts`,
 * `resultStanding.ts`); nothing here reads a statement.
 *
 * B7b (b7-plan, v2 deletion): the exact contract is 3.0.0/exact_pressure_v3 only. Its
 * identities are pressure-2 (the ordinary exact route) and pressure-source-1 (the
 * retained-source composite); a model with the load/reference-state group publishes
 * its `load_reference_states` under either. A stored result naming a retired
 * 2.0.0/exact_straight_pressure_v2 identity (physics-1, physics-source-1,
 * load-reference-1, load-reference-source-1) reads unsupported through Rust's header
 * (SOURCE_EXACT_PRESSURE_V2_RETIRED); the desktop names none of them. */
/** J1/J3: the v3 retained-source identity. */
export const PRESSURE_SOURCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/pressure-source-1";
/** T4-U2: the ordinary exact identity (3.0.0/exact_pressure_v3). */
export const PRESSURE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/pressure-2";
export const PREVIEW_PHYSICS_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-2";
/** DEL-05-03 Design `pf-timoshenko.md` §4: the pressure-free Timoshenko identity, a second
 * complete identity on the `preview_physics` route (a document whose
 * `model_settings.shear_model` selects `timoshenko_annulus_energy_v1`). Its table is
 * preview-physics-2's with the identity, the profile and limitation 0 replaced. */
export const PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1";
/** The `preview_physics` route's identities, Euler–Bernoulli first. */
export const PREVIEW_PHYSICS_CONTRACT_IDS: readonly string[] = Object.freeze([PREVIEW_PHYSICS_CONTRACT_ID, PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID]);
/** U6d (D-U6-6; D2 4.7 S-1, 4.9.6): the F2a preview successor. Its statement is checked
 * only by the accepted reader (the Rust one, through `retainedPrecisionReader.ts`), and
 * standing comes only from that reader's registered validation
 * (`retainedPrecisionStanding.ts`). */
export const PREVIEW_PHYSICS_RETAINED_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1";
/** B3b (B3-D §6.1, §7): the exact successor `<physics-retained>`, likewise. */
export const PHYSICS_RETAINED_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/physics-retained-1";
/** Either successor identity: only these may carry a `retained_precision` receipt. */
export function isRetainedIdentity(id: unknown): boolean {
  return id === PREVIEW_PHYSICS_RETAINED_CONTRACT_ID || id === PHYSICS_RETAINED_CONTRACT_ID;
}
/** Either successor route: the preview successor or the exact one. */
export function isRetainedRoute(route: string): route is "retained_preview_physics" | "retained_physics" {
  return route === "retained_preview_physics" || route === "retained_physics";
}
/** A receipt member, or a raw row with the W1 method token, offered under any
 * other identity (I66 F-5: the base readers' closed lists do not name it). */
export const RETAINED_PRECISION_DOWNGRADE_FORBIDDEN = "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN";
/** The desktop's route names, one per readable identity (Rust `reader_verdict::ROUTES`),
 * and `unsupported` for a statement the header dispatch refuses. */
export type SourceContract = "pressure_source" | "preview_physics" | "pressure" | "retained_preview_physics" | "retained_physics" | "unsupported";
/** TS's standing status for each carrier token of the successor (RV92 N-8: carriers
 * compare the token; this mapping is pinned separately). TS has no `unsupported`
 * status: a refused statement reads `needs_recompute` with the reader's code. */
export const RETAINED_STANDING_STATUS = Object.freeze({ numerically_eligible: "integrity_checked", needs_recompute: "needs_recompute", unsupported: "needs_recompute" } as const);
