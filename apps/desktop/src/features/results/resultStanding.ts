/** R2 (b7-plan Unit R): the desktop's numerical standing, presented over the Rust verdict
 * (`resultReader.ts`), and the session state it needs.
 *
 * G1: standing and its findings are Rust's (`reader_verdict::standing`); the desktop
 * displays them. G4: as R1's retained carrier (`retainedPrecisionStanding.ts`), the
 * desktop keeps the source routes' registrations: a completed validation of these exact
 * bytes with the native invocation previewService captured
 * (physics-source-1, pressure-source-1), or a completed reading of these exact bytes (load-reference-source-1).
 * A registration reads nothing: it holds the invocation, and the standing of a
 * registered source is Rust's with that invocation. A copy, a header, a hash or a saved
 * record never registers, and any later byte change voids it. G3 (the caller's model is
 * the dispatched one) is previewService's capture check. */
import { readerDispatch, readerSourceText, readerValidate, ResultReaderError, type ReaderStanding, type ReaderValidation } from "./resultReader";
import { retainedPrecisionStanding } from "./retainedPrecisionStanding";
import { isRetainedRoute, RETAINED_STANDING_STATUS, type SourceContract } from "./numericalResultQuality";
import type { MechanicsResult, PreviewModel } from "../../types";

/** No registered validation with the captured invocation holds for these bytes and model. */
export const SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED = "SOURCE_BLOCKS_VALIDATED_INVOCATION_REQUIRED";
/** The joined reader's admission; joined evidence is never numerically eligible in T1. */
export const LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE = "LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1";
/** No completed reading of these exact bytes is registered. */
export const LOAD_REFERENCE_SOURCE_VALIDATION_REQUIRED = "SOURCE_LOAD_REFERENCE_SOURCE_VALIDATION_REQUIRED";

export type SourceRecoveryInvocation = Readonly<{ request: unknown; solver_mode: "dense_scrutiny" | "sparse_interactive" }>;
export type NumericalStanding = { contract: SourceContract; status: "integrity_checked" | "needs_recompute"; eligible: boolean; findings: string[] };
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

/** Structural JSON-data equality with signed zeros distinct (the desktop's model
 * comparison: a caller's model against the captured one). */
export function sameModelData(a: Json, b: Json): boolean {
  if (Object.is(a, b)) return true;
  if (!a || !b || typeof a !== "object" || typeof b !== "object" || Array.isArray(a) !== Array.isArray(b)) return false;
  const keys = Object.keys(a);
  return keys.length === Object.keys(b).length && keys.every(k => Object.hasOwn(b, k) && sameModelData(a[k], b[k]));
}
function textOf(value: unknown): string | null {
  try { return readerSourceText(value); } catch { return null; }
}

/* -------------------------------------------------------------------------- *
 * G4: physics-source-1 and pressure-source-1
 * -------------------------------------------------------------------------- */
type SourceRegistration = {
  route: "physics_source" | "pressure_source";
  sourceText: string;
  invocation: SourceRecoveryInvocation;
  callerModel?: PreviewModel;
  callerSnapshot?: PreviewModel;
  live?: (model: unknown) => boolean;
};
const sourceRegistrations = new WeakMap<object, SourceRegistration>();

/** Registration after actual direct IPC or a completed known job only
 * (`previewService.validateCapturedSource`, after its G3 check): the Rust reader of the
 * source's route validates these bytes with the captured invocation, and only a pass
 * registers. A refusal clears any earlier registration and is thrown. `callerModel` is
 * the caller's model the invocation was captured from; a later edit of it voids the
 * registration. `live`, when supplied by previewService, keeps the invocation
 * unusable until final native registration succeeds and while that capture is live.
 * Direct registrations without this guard retain their existing behavior. */
export async function registerSourceRecovery(source: MechanicsResult, invocation: SourceRecoveryInvocation, callerModel?: PreviewModel, live?: (model: unknown) => boolean): Promise<ReaderValidation> {
  sourceRegistrations.delete(source);
  const route = readerDispatch(source).route;
  if (route !== "physics_source" && route !== "pressure_source") throw new ResultReaderError(route === "unsupported" ? "SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED" : "SOURCE_BLOCKS_CONTRACT");
  const sourceText = readerSourceText(source);
  const captured = structuredClone(invocation);
  const callerSnapshot = callerModel ? structuredClone(callerModel) : undefined;
  const validation = readerValidate(route === "pressure_source" ? "pressure-source" : "physics-source", source, undefined, captured);
  sourceRegistrations.set(source, { route, sourceText, invocation: captured, callerModel, callerSnapshot, live });
  return validation;
}
/** The registered invocation of these exact bytes for this model, or null: the bytes
 * are unchanged, the caller's model is unchanged, and `model` is the captured model or
 * the caller's (the desktop's capture rules, unchanged). */
function registeredInvocation(source: MechanicsResult, model: Pick<PreviewModel, "load_cases"> | null | undefined, route: SourceContract): SourceRecoveryInvocation | null {
  const registration = sourceRegistrations.get(source);
  if (!registration || registration.route !== route || !model) return null;
  // The reader can pass before previewService finishes its awaited hash and
  // final capture checks. Its invocation is usable only with live native custody.
  try { if (registration.live && registration.live(model) !== true) return null; } catch { return null; }
  if (textOf(source) !== registration.sourceText || textOf(model) === null) return null;
  if (registration.callerModel && (textOf(registration.callerModel) === null || !sameModelData(registration.callerModel, registration.callerSnapshot))) return null;
  const captured = (registration.invocation.request as { model: PreviewModel }).model;
  return sameModelData(model, captured) || (registration.callerSnapshot && sameModelData(model, registration.callerSnapshot)) ? registration.invocation : null;
}
/** The registered invocation, for the rule-check backend's `sourceBlockInvocation`
 * argument; null unless the standing with it is eligible. */
export function retainedSourceRecoveryInvocation(source: MechanicsResult, model: PreviewModel): SourceRecoveryInvocation | null {
  const route = readerDispatch(source).route;
  if (route !== "physics_source" && route !== "pressure_source") return null;
  const invocation = registeredInvocation(source, model, route);
  return invocation && readerDispatch(source, model, invocation).standing.eligible ? structuredClone(invocation) : null;
}
/** Whether the eligible registered invocation was solved in this mode. */
export function sourceRecoveryModeMatches(source: MechanicsResult, model: PreviewModel, solverMode: string): boolean {
  return retainedSourceRecoveryInvocation(source, model)?.solver_mode === solverMode;
}

/* -------------------------------------------------------------------------- *
 * G4: load-reference-source-1
 * -------------------------------------------------------------------------- */
const joinedRegistrations = new WeakMap<object, { sourceText: string; error: string | null }>();
/** The joined reader on these exact bytes. Resolves to false: admitted, never
 * numerically eligible. Registers the outcome (admitted, or its refusal) for standing
 * and rethrows a refusal. */
export async function validateLoadReferenceSourceEvidence(source: MechanicsResult): Promise<false> {
  const sourceText = textOf(source);
  try {
    readerValidate("load-reference-source", source);
    if (sourceText !== null && source && typeof source === "object") joinedRegistrations.set(source, { sourceText, error: null });
    return false;
  } catch (error) {
    if (sourceText !== null && source && typeof source === "object") joinedRegistrations.set(source, { sourceText, error: error instanceof Error ? error.message : "SOURCE_LOAD_REFERENCE_MALFORMED" });
    throw error;
  }
}
function joinedRegistered(source: MechanicsResult): boolean {
  const registration = joinedRegistrations.get(source);
  return !!registration && textOf(source) === registration.sourceText;
}
/** The joined route's standing findings: the missing-reading reason, or (read) Rust's:
 * the reader's refusal or the declared T1 reason. */
export function loadReferenceSourceStanding(source: MechanicsResult): { eligible: false; findings: string[] } {
  if (!joinedRegistered(source)) return { eligible: false, findings: [LOAD_REFERENCE_SOURCE_VALIDATION_REQUIRED] };
  return { eligible: false, findings: [...readerDispatch(source).standing.findings] };
}

/* -------------------------------------------------------------------------- *
 * Standing
 * -------------------------------------------------------------------------- */
function presented(standing: ReaderStanding): NumericalStanding {
  return { contract: standing.route, status: standing.status, eligible: standing.eligible, findings: [...standing.findings] };
}
/** The numerical standing of a statement for the current model: Rust's, with the
 * registered invocation for a source route (G4). A successor's standing is R1's
 * registered reader validation with the live native capture. */
export function numericalResultStanding(source: MechanicsResult, model?: (Pick<PreviewModel, "load_cases"> & Partial<Pick<PreviewModel, "pipe_segments" | "supports">>) | null): NumericalStanding {
  const route = readerDispatch(source).route;
  if (route === "physics_source" || route === "pressure_source") {
    return presented(readerDispatch(source, model, registeredInvocation(source, model, route) ?? undefined).standing);
  }
  if (route === "load_reference_source") return { contract: route, status: "needs_recompute", eligible: false, findings: loadReferenceSourceStanding(source).findings };
  if (isRetainedRoute(route)) {
    const standing = retainedPrecisionStanding(source, model);
    return { contract: route, status: RETAINED_STANDING_STATUS[standing.standing], eligible: standing.eligible, findings: standing.findings };
  }
  return presented(readerDispatch(source, model).standing);
}
