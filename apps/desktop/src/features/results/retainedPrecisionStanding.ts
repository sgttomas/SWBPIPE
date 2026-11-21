/** Unit R: retained-source session capture and presentation. Rust owns the
 * receipt reading, standing predicates and classification summary. This module
 * binds the accepted validation to the captured source and invocation, retains
 * reader errors and checks the desktop's live native capture (D-U7-4). */
import { retainedPrecisionSemantics, validateRetainedPrecision, validateRetainedPrecisionBytes, type RetainedClassificationSummary } from "./retainedPrecisionReader";
import { RetainedPrecisionError, type RetainedPrecisionValidation, type RowClassification } from "./retainedPrecisionTypes";
import { checkedJsonText } from "../../services/hashService";
import type { MechanicsResult, PreviewModel } from "../../types";

/** No completed reader validation is registered for these exact bytes. */
export const RETAINED_PRECISION_VALIDATION_REQUIRED = "RETAINED_PRECISION_VALIDATION_REQUIRED";
/** The registered validation does not make the successor numerically eligible. */
export const RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE = "RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE";
/** U7 slice T (RV91 N-2 = RV88 U6d S-1; D-U7-4): a statement that would be
 * eligible, but without the live native capture of these bytes for this model. */
export const RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED = "RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED";

export type RetainedStandingToken = "numerically_eligible" | "needs_recompute" | "unsupported";
export type { RetainedClassificationSummary } from "./retainedPrecisionReader";
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
type Fingerprint = { text: string; negativeZeros: string };
type Outcome = { validation: RetainedPrecisionValidation; error: null } | { validation: null; error: string };
/** Whether the live native capture still holds for these bytes and a given model:
 * previewService's `hasNativeMechanicsInvocation` (an unchanged, not invalidated
 * capture; unchanged source bytes; that model equal to the captured one). */
export type LiveNativeCapture = (model: unknown) => boolean;
type Registration = { source: Fingerprint; invocation: unknown; outcome: Outcome; live: LiveNativeCapture | null };
const registrations = new WeakMap<object, Registration>();

function negativeZeroPaths(value: unknown): string {
  const paths: string[] = [];
  const visit = (item: unknown, path: string) => {
    if (Object.is(item, -0)) paths.push(path);
    else if (item && typeof item === "object") for (const [key, child] of Object.entries(item)) visit(child, `${path}/${key.replaceAll("~", "~0").replaceAll("/", "~1")}`);
  };
  visit(value, "");
  return JSON.stringify(paths);
}
function fingerprint(value: unknown): Fingerprint | null {
  try { return { text: checkedJsonText(value), negativeZeros: negativeZeroPaths(value) }; } catch { return null; }
}
function receiptCases(source: MechanicsResult): Json[] {
  const cases = (source as Json)?.retained_precision?.body?.cases;
  return Array.isArray(cases) ? cases : [];
}

/** The recorded code when the reader could not run (the wasm engine is unavailable or failed). */
export const RETAINED_PRECISION_READER_UNAVAILABLE = "RETAINED_PRECISION_READER_UNAVAILABLE";

/** Registration after actual direct IPC or a completed known job only
 * (`previewService.validateCapturedSource`). Runs the accepted reader with the
 * captured invocation and records its outcome, validation or first code,
 * against the exact bytes it validated. A refusal is rethrown after it is
 * recorded, so the caller does not register the native invocation.
 * `live` is the native capture's liveness for these bytes; only previewService
 * passes it. Without it the registration can never stand eligible (U7 slice T).
 * `sourceBytes`, when given, are the producer's bytes `source` was read from: the
 * reader then reads those bytes (b7-plan R1, "Exact bytes to the desktop"). */
export async function registerRetainedPrecision(source: MechanicsResult, invocation: unknown, live: LiveNativeCapture | null = null, sourceBytes: string | null = null): Promise<RetainedPrecisionValidation> {
  // Captured before the reader's first await, as the reader snapshots its inputs.
  // Bytes outside the checked JSON profile never register (the reader refuses them).
  const sourcePrint = fingerprint(source), captured = structuredClone(invocation);
  const record = (outcome: Outcome) => { if (sourcePrint) registrations.set(source, { source: sourcePrint, invocation: captured, outcome, live }); };
  try {
    const validation = sourceBytes === null ? await validateRetainedPrecision(source, invocation) : await validateRetainedPrecisionBytes(sourceBytes, invocation);
    record({ validation, error: null });
    return validation;
  } catch (error) {
    // A reader refusal records its code. Any other failure (the wasm engine did not load, or
    // failed other than by reading) records RETAINED_PRECISION_READER_UNAVAILABLE: the
    // statement was not read, so it stays refused (fail closed) without a reader code.
    record({ validation: null, error: error instanceof RetainedPrecisionError ? error.code : RETAINED_PRECISION_READER_UNAVAILABLE });
    throw error;
  }
}
/** The registration of these exact bytes, or null (absent, or bytes changed). The
 * comparison is the exact checked JSON text, so an in-place edit that only reorders
 * a member's keys also voids it (fail closed; RV88 U6d N-2). */
function registered(source: MechanicsResult | null | undefined): Registration | null {
  const registration = registrations.get(source as object);
  if (!registration) return null;
  const current = fingerprint(source);
  return current && current.text === registration.source.text && current.negativeZeros === registration.source.negativeZeros ? registration : null;
}
/** The registered validation outcome for these exact bytes: a validation, the
 * reader's first code, or null when nothing is registered. Query only. */
export function retainedPrecisionRegistration(source: MechanicsResult | null | undefined): Readonly<Outcome> | null {
  return registered(source)?.outcome ?? null;
}

/** Transport-only seam for Rust's standing over an accepted validation. */
export function retainedStandingFrom(validation: RetainedPrecisionValidation, source: MechanicsResult, requested: readonly unknown[]): "numerically_eligible" | "needs_recompute" {
  return retainedPrecisionSemantics(validation, source, requested).standing;
}
function requestedRefs(model: Pick<PreviewModel, "load_cases"> | null | undefined): Json[] {
  return (model?.load_cases ?? []).map(c => ({ ref_type: "load_case", ref_id: c.id }));
}
/** Synchronous successor standing (plan 3, rule 1-3):
 * - nothing registered for these bytes: `needs_recompute`, VALIDATION_REQUIRED;
 * - the registered reader refused: `unsupported`, with the reader's first code;
 * - otherwise D2 4.9.4 with the model's requested load cases; not eligible gives
 *   `needs_recompute`, NOT_NUMERICALLY_ELIGIBLE;
 * - eligible by D2 4.9.4, but the live native capture does not hold for these
 *   bytes and this model (a stale model, an invalidated or raced capture, or a
 *   registration without a capture): `needs_recompute`, NATIVE_CAPTURE_REQUIRED
 *   (U7 slice T; D-U7-4's declared difference: Rust and Python require the actual
 *   invocation argument, TS the live capture). */
export function retainedPrecisionStanding(source: MechanicsResult, model?: Pick<PreviewModel, "load_cases"> | null): { standing: RetainedStandingToken; eligible: boolean; findings: string[] } {
  const registration = registered(source);
  if (!registration) return { standing: "needs_recompute", eligible: false, findings: [RETAINED_PRECISION_VALIDATION_REQUIRED] };
  if (registration.outcome.error !== null) return { standing: "unsupported", eligible: false, findings: [registration.outcome.error] };
  let standing: "numerically_eligible" | "needs_recompute";
  try { standing = retainedStandingFrom(registration.outcome.validation, source, requestedRefs(model)); }
  catch { return { standing: "unsupported", eligible: false, findings: [RETAINED_PRECISION_READER_UNAVAILABLE] }; }
  if (standing !== "numerically_eligible") return { standing: "needs_recompute", eligible: false, findings: [RETAINED_PRECISION_NOT_NUMERICALLY_ELIGIBLE] };
  if (registration.live?.(model) !== true) return { standing: "needs_recompute", eligible: false, findings: [RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED] };
  return { standing: "numerically_eligible", eligible: true, findings: [] };
}
/** The registered invocation, for the rule-check backend's existing
 * `sourceBlockInvocation` argument; null unless the standing is eligible. */
export function retainedPrecisionInvocation(source: MechanicsResult, model: PreviewModel): unknown {
  if (!retainedPrecisionStanding(source, model).eligible) return null;
  return structuredClone(registered(source)!.invocation);
}
/** Validated G5c classes by result id from the registration; null when nothing
 * valid is registered (no validated class exists for any row). */
export function retainedRowClasses(source: MechanicsResult | null | undefined): ReadonlyMap<string, RowClassification> | null {
  const validation = registered(source)?.outcome.validation;
  return validation ? new Map(validation.classifications.map(row => [row.result_id, row])) : null;
}
/** Transport-only seam for Rust's summary over an accepted validation. */
export function classificationSummaryFrom(validation: RetainedPrecisionValidation, source: MechanicsResult, requested: readonly unknown[]): RetainedClassificationSummary[] {
  return retainedPrecisionSemantics(validation, source, requested).classification_summary;
}
/** The per-case summary over the registered validation; empty when nothing
 * valid is registered for these bytes. `withheld` counts as Current only when
 * the standing is eligible, the live native capture included (D-U7-4);
 * otherwise no requested refs are passed, which gives the not-Current count. */
export function classificationSummary(source: MechanicsResult | null | undefined, model?: Pick<PreviewModel, "load_cases"> | null): RetainedClassificationSummary[] {
  const validation = registered(source)?.outcome.validation;
  if (!validation) return [];
  try { return classificationSummaryFrom(validation, source!, retainedPrecisionStanding(source!, model).eligible ? requestedRefs(model) : []); }
  catch { return []; } // The semantic binding did not read a verdict: fail closed.
}
/** Results-panel standing text from the registered receipt only, never from the
 * ordinary `numerical_quality` (D2 4.9.2). The receipt's case count is shown only
 * from a validated registration, never from an unvalidated or refused receipt
 * (RV91 N-4). Every branch carries the published `formulation_basis.limitations`,
 * in order, as every other route's standing text does (DEL-07-04: mechanics outputs
 * are shown only with their limitations visible); the retained route's header check
 * admits only a non-empty list of them. Text only; it changes no standing. */
export function retainedPrecisionStandingText(source: MechanicsResult): string {
  const outcome = retainedPrecisionRegistration(source);
  // D-U7-6: eligibility is a property of the supplied statement and its invocation.
  const tail = `${source.formulation_basis!.limitations.join(" ")} Current use is checked separately against the actual invocation and the requested cases. The reader checks these bytes and their invocation; it does not establish which producer made them. Numerical checks do not establish engineering correctness.`;
  if (!outcome) return `Retained precision: receipt not validated for these exact bytes in this session (saved, reference or copied data never register); needs recompute. ${tail}`;
  if (outcome.error !== null) return `Retained precision: receipt refused by the retained-precision reader (${outcome.error}); unsupported, values shown for inspection only. ${tail}`;
  const cases = receiptCases(source);
  const selected = cases.filter(c => c?.status === "selected").length;
  return `Retained precision: receipt validated by the retained-precision reader ${outcome.validation.invocation_bound ? "against the actual invocation" : "without an invocation"} for these exact bytes. Selected cases: ${selected} of ${cases.length}. ${tail}`;
}
