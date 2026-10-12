/** The desktop's result-route vocabulary: the semantic contract identities its notices
 * and policies name, its route names, and the successor carrier's status mapping.
 * R2 (b7-plan Unit R): the route of a statement, its table binding, its downgrade
 * refusal and its standing are the Rust reader's verdict (`resultReader.ts`,
 * `resultStanding.ts`); nothing here reads a statement. */
export const PRECISION_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/precision-1";
export const PHYSICS_SOURCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/physics-source-1";
/** J1/J3: the v3 retained-source identity, shared by 0.3.0 and 0.4.0 inputs. */
export const PRESSURE_SOURCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/pressure-source-1";
export const PHYSICS_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/physics-1";
export const PREVIEW_PHYSICS_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-1";
export const SOURCE_BLOCKS_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/source-blocks-1";
/** T1 load/reference-state identities (0.4.0 exact route only). */
export const LOAD_REFERENCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/load-reference-1";
export const LOAD_REFERENCE_SOURCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/load-reference-source-1";
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
export type SourceContract = "legacy" | "precision" | "physics" | "source_blocks" | "physics_source" | "pressure_source" | "preview_physics" | "load_reference" | "load_reference_source" | "retained_preview_physics" | "retained_physics" | "unsupported";
/** TS's standing status for each carrier token of the successor (RV92 N-8: carriers
 * compare the token; this mapping is pinned separately). TS has no `unsupported`
 * status: a refused statement reads `needs_recompute` with the reader's code. */
export const RETAINED_STANDING_STATUS = Object.freeze({ numerically_eligible: "integrity_checked", needs_recompute: "needs_recompute", unsupported: "needs_recompute" } as const);
