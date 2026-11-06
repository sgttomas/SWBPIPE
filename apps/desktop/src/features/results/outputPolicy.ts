/** T6S-3 (I74 PLAN 1.1; RR decision 2): the desktop output policy, per route and
 * per surface.
 *
 * `OUTPUT_POLICY` has one deliberate entry for every `SourceContract`; `tsc`
 * refuses a route without one (B3b gave `physics-retained-1`'s route,
 * `retained_physics`, its own entry). A gated route names every output surface: each surface
 * refuses with the route's shared reason, admits the route only at numerically
 * eligible standing with the live native capture (D2 4.9.4; D-U7-4), or adds no refusal
 * (`admitted`: the surface's own source and standing checks apply, as for an ungated
 * route). A surface left unadmitted refuses, and a route value missing from the table
 * fails closed.
 *
 * Only the Result Export and Stress-Neutral Export panels admit the F2a preview
 * successor (`retained_preview_physics`), and, likewise, B3b's exact successor
 * (`retained_physics`; B3-D §7, decision B3D-14), each with its own reason text. The other eighteen surfaces (T1's group
 * a, through `LoadReferenceOutputGate`) and the report package keep refusing it.
 * The report package reads its own `report-package` cell (`reportPackageUnavailableReason`,
 * W-18; DEL-08-01 Design `report-package-current-results.md` §2.1): the pressure-free
 * route is ungated there, and its Rust boundary admits preview-physics-2, pressure-2
 * and pressure-source-1.
 * Epoch 8 (DEL-04-07 Design `desktop-exact-outputs.md`; owner, 2026-10-21: "A accept
 * your recommendations for the entire set."): pressure-2 (`pressure`) and
 * pressure-source-1 (`pressure_source`) are `admitted` on the result JSON export, the
 * stress-neutral export and the rule check, where each surface takes the Rust reader's
 * verdict and standing. W-11 (DEL-08-01 Design `exact-route-report-sections.md` §5.1;
 * owner, 2026-10-26: "My two decisions are to approve your recommendations in both
 * cases. Proceed accordingly.") admits them on the rendered report and the report
 * package too, where each takes the Export Result JSON admission and the native host's
 * source binding and standing. Every other surface, the report JSON packet included,
 * refuses them. A load/reference-state result, published under one of these identities
 * since B7b, follows the reader with no source-level step (ruling 11B, design §5.4): an
 * eligible pressure-2 one is admitted there, and a pressure-source-1 one is never
 * eligible. Ungated routes add no refusal here; each surface's own source and standing
 * checks apply, including the refusal of an unsupported (for example relabelled)
 * statement.
 *
 * Text only: the policy never changes standing, rows or bytes, and admits no
 * producer-origin claim (D-U7-6). No product caller delivers an eligible successor
 * before B8, so the admitted panels stay dormant until then (RR decision 3).
 */
import type { SessionResult } from "../../services/nativeResult";
import type { SourceContract } from "./numericalResultQuality";
import { sourceContract } from "./resultReader";
import { numericalResultStanding } from "./resultStanding";
import type { MechanicsResult, PreviewModel } from "../../types";

export const PRESSURE_2_OUTPUT_NOT_YET_AVAILABLE = "PRESSURE-2-OUTPUT-NOT-YET-AVAILABLE";
/** Epoch 8 (design §5.3): reworded so that it no longer says every output is unavailable. */
export const N_PRESSURE_2_OUTPUT = "This output of pressure-2 results (3.0.0/exact_pressure_v3) is not available on the desktop; only the result JSON and stress-neutral exports, the rule check, the rendered report and the report package admit a numerically eligible result. The result remains readable here; this is not a finding about the result.";
export const PRESSURE_2_OUTPUT_REFUSAL = `${PRESSURE_2_OUTPUT_NOT_YET_AVAILABLE}: ${N_PRESSURE_2_OUTPUT}`;
/** b7-plan D6/MJ8: pressure-source-1 exports through the Rust derivative, which binds its
 * received v3 receipt. Epoch 8: the same surfaces as pressure-2 admit it, and every other
 * surface refuses under pressure-2's code. This policy adds no numerical standing. */
export const N_PRESSURE_SOURCE_OUTPUT = "This output of pressure-source-1 results (3.0.0/exact_pressure_v3) is not available on the desktop; only the result JSON and stress-neutral exports, the rule check, the rendered report and the report package admit a numerically eligible result. The result remains readable here; this is not a finding about the result.";
export const PRESSURE_SOURCE_OUTPUT_REFUSAL = `${PRESSURE_2_OUTPUT_NOT_YET_AVAILABLE}: ${N_PRESSURE_SOURCE_OUTPUT}`;
export const RETAINED_PRECISION_OUTPUT_NOT_YET_AVAILABLE = "RETAINED-PRECISION-OUTPUT-NOT-YET-AVAILABLE";
/** Decision 12: reworded so that it no longer says every output is unavailable. */
export const N_RETAINED_PRECISION_OUTPUT = "This output of retained-precision results (preview-physics-retained-1) is not available on the desktop; only the result JSON and stress-neutral exports admit a numerically eligible result. The result remains readable here; this is not a finding about the result.";
export const RETAINED_PRECISION_OUTPUT_REFUSAL = `${RETAINED_PRECISION_OUTPUT_NOT_YET_AVAILABLE}: ${N_RETAINED_PRECISION_OUTPUT}`;
/** B3b (B3-D §7; B3D-14): the exact successor's own reason; the preview text is untouched. */
export const N_RETAINED_PHYSICS_OUTPUT = "This output of retained-precision results (physics-retained-1) is not available on the desktop; only the result JSON and stress-neutral exports admit a numerically eligible result. The result remains readable here; this is not a finding about the result.";
export const RETAINED_PHYSICS_OUTPUT_REFUSAL = `${RETAINED_PRECISION_OUTPUT_NOT_YET_AVAILABLE}: ${N_RETAINED_PHYSICS_OUTPUT}`;
/** An admitted surface refuses a gated route that is not numerically eligible. The
 * reason starts with the standing's own finding code (for example
 * RETAINED_PRECISION_VALIDATION_REQUIRED or RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED). */
export const N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE = "This export admits a retained-precision result only at numerically eligible standing, with the live native capture of these exact bytes for the current model. The result remains readable here.";
/** Fail closed: a route value with no policy entry. */
export const OUTPUT_ROUTE_NOT_REGISTERED = "OUTPUT-ROUTE-NOT-REGISTERED";
export const OUTPUT_ROUTE_NOT_REGISTERED_REFUSAL = `${OUTPUT_ROUTE_NOT_REGISTERED}: This result's route has no desktop output policy entry, so no output is offered. The result remains readable here.`;

/** Every desktop surface whose download, package or handoff can carry result data.
 * The first eighteen are T1's group a (named by their test-id prefixes). */
export const OUTPUT_SURFACES = [
  "pcf-export", "caepipe-mbf", "caepipe-external", "export-adapter-sdk", "adapter-framework", "external-prover",
  "missing-data", "design-workspace", "rule-check", "report-lint", "solve-job", "headless-runner",
  "local-fea", "native-package", "handoff", "export-review", "report", "rendered-report",
  "report-package", "result-export", "stress-neutral",
] as const;
export type OutputSurface = (typeof OUTPUT_SURFACES)[number];
/** What one surface does with a gated route. */
export type SurfaceDecision = "refused" | "admitted_when_eligible" | "admitted";
export type RouteOutputPolicy =
  /** This policy adds no refusal; the surface's own checks apply. */
  | { readonly gate: "none" }
  /** Every surface is named: it refuses with `reason`, or admits only at eligible standing. */
  | { readonly gate: "per_surface"; readonly reason: string; readonly surfaces: Readonly<Record<OutputSurface, SurfaceDecision>> };

const UNGATED: RouteOutputPolicy = Object.freeze({ gate: "none" });
/** Epoch 8 (design §5.1): the exact routes' surfaces, `admitted` on the opened surfaces
 * and refused on every other. W-11 (DEL-08-01 Design `exact-route-report-sections.md`
 * §5.1; owner, 2026-10-26) adds the rendered report and the report package. */
const EXACT_ADMITTED_SURFACES: readonly OutputSurface[] = Object.freeze(["result-export", "stress-neutral", "rule-check", "rendered-report", "report-package"]);
function exactRouteSurfaces(): Readonly<Record<OutputSurface, SurfaceDecision>> {
  return Object.freeze(Object.fromEntries(OUTPUT_SURFACES.map(surface => [surface, EXACT_ADMITTED_SURFACES.includes(surface) ? "admitted" : "refused"])) as Record<OutputSurface, SurfaceDecision>);
}

export const OUTPUT_POLICY: Readonly<Record<SourceContract, RouteOutputPolicy>> = Object.freeze({
  // pressure-source-1 (with or without load/reference states): the five opened
  // surfaces add no refusal; their own checks take Rust's standing, which is never
  // eligible with load/reference states or without the registered invocation.
  pressure_source: Object.freeze({ gate: "per_surface", reason: PRESSURE_SOURCE_OUTPUT_REFUSAL, surfaces: exactRouteSurfaces() }),
  preview_physics: UNGATED,
  // Each surface refuses an unsupported statement through its own contract check.
  unsupported: UNGATED,
  // pressure-2 (likewise with or without load/reference states): the five opened
  // surfaces add no refusal; their own checks take Rust's standing.
  pressure: Object.freeze({ gate: "per_surface", reason: PRESSURE_2_OUTPUT_REFUSAL, surfaces: exactRouteSurfaces() }),
  // T6S (RR decisions 2 and 12): only the two T6 panels admit the preview successor.
  // Checklist item 1 opens the Rule-check panel by its own entry, under its own review.
  retained_preview_physics: Object.freeze({
    gate: "per_surface",
    reason: RETAINED_PRECISION_OUTPUT_REFUSAL,
    surfaces: Object.freeze({
      "pcf-export": "refused", "caepipe-mbf": "refused", "caepipe-external": "refused", "export-adapter-sdk": "refused",
      "adapter-framework": "refused", "external-prover": "refused", "missing-data": "refused", "design-workspace": "refused",
      "rule-check": "refused", "report-lint": "refused", "solve-job": "refused", "headless-runner": "refused",
      "local-fea": "refused", "native-package": "refused", "handoff": "refused", "export-review": "refused",
      "report": "refused", "rendered-report": "refused", "report-package": "refused",
      "result-export": "admitted_when_eligible", "stress-neutral": "admitted_when_eligible",
    }),
  }),
  // B3b (B3-D §7, decision 21): the exact successor, admitted by the same two T6 panels only.
  retained_physics: Object.freeze({
    gate: "per_surface",
    reason: RETAINED_PHYSICS_OUTPUT_REFUSAL,
    surfaces: Object.freeze({
      "pcf-export": "refused", "caepipe-mbf": "refused", "caepipe-external": "refused", "export-adapter-sdk": "refused",
      "adapter-framework": "refused", "external-prover": "refused", "missing-data": "refused", "design-workspace": "refused",
      "rule-check": "refused", "report-lint": "refused", "solve-job": "refused", "headless-runner": "refused",
      "local-fea": "refused", "native-package": "refused", "handoff": "refused", "export-review": "refused",
      "report": "refused", "rendered-report": "refused", "report-package": "refused",
      "result-export": "admitted_when_eligible", "stress-neutral": "admitted_when_eligible",
    }),
  }),
});

/** The policy entry for a route value; null fails closed (no entry). */
export function routeOutputPolicy(route: string): RouteOutputPolicy | null {
  return Object.hasOwn(OUTPUT_POLICY, route) ? OUTPUT_POLICY[route as SourceContract] : null;
}
function sourceRoute(source: SessionResult | null | undefined): string | null {
  if (!source) return null;
  try { return sourceContract(source); } catch { return null; }
}
/** The route's shared refusal, whatever the surface: the reason a gated route
 * carries, or null. This is `loadReferenceOutputRefusal`'s meaning, unchanged. */
export function routeOutputRefusal(source: SessionResult | null | undefined): string | null {
  const route = sourceRoute(source);
  if (route === null) return null;
  const policy = routeOutputPolicy(route);
  return policy === null ? OUTPUT_ROUTE_NOT_REGISTERED_REFUSAL : policy.gate === "none" ? null : policy.reason;
}
/** The decision of one surface for one route value, or null when the policy adds no
 * refusal (an ungated route, or an `admitted` cell). A route without an entry, or a
 * surface the entry does not name, refuses. */
export function routeSurfaceDecision(route: string, surface: string): { decision: SurfaceDecision; reason: string } | null {
  const policy = routeOutputPolicy(route);
  if (policy === null) return { decision: "refused", reason: OUTPUT_ROUTE_NOT_REGISTERED_REFUSAL };
  if (policy.gate === "none") return null;
  const decision = Object.hasOwn(policy.surfaces, surface) ? policy.surfaces[surface as OutputSurface] : "refused";
  if (decision === "admitted") return null;
  return { decision: decision === "admitted_when_eligible" ? decision : "refused", reason: policy.reason };
}
/** Route level only, for pure projections and validators: the surface's refusal of
 * this source's route, or null when the route is ungated or the surface admits it.
 * Eligibility is not checked here; `surfaceOutputRefusal` adds it. */
export function surfaceRouteRefusal(source: SessionResult | null | undefined, surface: OutputSurface): string | null {
  const route = sourceRoute(source);
  if (route === null) return null;
  const decided = routeSurfaceDecision(route, surface);
  return decided === null || decided.decision === "admitted_when_eligible" ? null : decided.reason;
}
/** Current level, for the panels and the Current builders: the route refusal, plus,
 * for a gated route the surface admits, numerically eligible standing with the live
 * native capture (`numericalResultStanding(source, model).eligible`). */
export function surfaceOutputRefusal(source: SessionResult | null | undefined, model: PreviewModel | null | undefined, surface: OutputSurface): string | null {
  const route = sourceRoute(source);
  if (route === null) return null;
  const decided = routeSurfaceDecision(route, surface);
  if (decided === null) return null;
  if (decided.decision !== "admitted_when_eligible") return decided.reason;
  let standing: { eligible: boolean; findings: string[] };
  try { standing = numericalResultStanding(source!, model); } catch { standing = { eligible: false, findings: [] }; }
  return standing.eligible ? null : `${standing.findings[0] ?? "RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE"}: ${N_OUTPUT_NOT_NUMERICALLY_ELIGIBLE}`;
}
/** Throws `surfaceRouteRefusal`. */
export function refuseSurfaceRoute(source: SessionResult | null | undefined, surface: OutputSurface): void {
  const refusal = surfaceRouteRefusal(source, surface);
  if (refusal) throw new Error(refusal);
}
/** Throws `surfaceOutputRefusal`. */
export function refuseSurfaceOutput(source: SessionResult | null | undefined, model: PreviewModel | null | undefined, surface: OutputSurface): void {
  const refusal = surfaceOutputRefusal(source, model, surface);
  if (refusal) throw new Error(refusal);
}
