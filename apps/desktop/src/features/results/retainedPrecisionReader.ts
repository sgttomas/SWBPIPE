/** R1 (b7-plan Unit R): the desktop's retained-precision reader is the Rust reader
 * (`core/reporting/result_export/src/retained_precision.rs`), compiled to wasm and loaded
 * through `loadWasmEngine`. This module carries JSON text to it and its outcome back; it
 * reads no receipt itself.
 *
 * - `validateRetainedPrecisionBytes` reads the exact source bytes the caller holds (a
 *   source received from the native solver).
 * - `validateRetainedPrecision` and `validateRetainedPrecisionTransport` read a parsed
 *   value, written as JSON text that keeps its negative zeros: `JSON.stringify` writes -0
 *   as 0, and the reader refuses a -0 in a receipt (G2), so dropping the sign would read
 *   another statement.
 *
 * A pass is the reader's `Validation`, frozen, with the standing it implies. A refusal
 * throws `RetainedPrecisionError` with the reader's gate, code and detail. A value that is
 * not checked JSON, or text that is not JSON, cannot be the receipt's statement: it is
 * refused at G1 with RETAINED_PRECISION_RECEIPT_MISMATCH. Hashes bind the supplied
 * statements; they do not establish producer origin (D-U7-6). */
import { checkedJsonText } from "../../services/hashService";
import { jsonTextKeepingNegativeZero } from "../../services/jsonText";
import { loadedWasmEngine, loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import { RetainedPrecisionError, type RetainedPrecisionValidation, type RowClassification } from "./retainedPrecisionTypes";

type ReaderOutcome =
  | { ok: true; validation: Omit<RetainedPrecisionValidation, "standing"> }
  | { ok: false; error: { gate: string; code: string; detail: string | null } };

function unreadable(): RetainedPrecisionError {
  return new RetainedPrecisionError("G1", "RETAINED_PRECISION_RECEIPT_MISMATCH");
}

/** JSON text of a checked JSON value (`checkedJsonText`'s profile), with -0 written as -0. */
export function retainedJsonText(value: unknown): string {
  checkedJsonText(value);
  return jsonTextKeepingNegativeZero(value);
}

function freeze<T>(value: T): T {
  if (value && typeof value === "object") {
    for (const item of Object.values(value)) freeze(item);
    Object.freeze(value);
  }
  return value;
}

function readOutcome(outcomeJson: string): RetainedPrecisionValidation {
  const outcome = JSON.parse(outcomeJson) as ReaderOutcome;
  if (!outcome.ok) throw new RetainedPrecisionError(outcome.error.gate, outcome.error.code, outcome.error.detail);
  const v = outcome.validation;
  return freeze({
    invocation_bound: v.invocation_bound,
    numerical_eligible: v.numerical_eligible,
    standing: v.numerical_eligible ? "eligible" : "needs_recompute",
    publication_sha256: v.publication_sha256,
    classifications: v.classifications.map((row): RowClassification => ({
      result_id: row.result_id, basis_ref: row.basis_ref, normalized_bits: row.normalized_bits,
      scale_bits: row.scale_bits, bound_bits: row.bound_bits, class: row.class,
    })),
  });
}

async function run(read: () => string): Promise<RetainedPrecisionValidation> {
  let outcome: string;
  try { outcome = read(); } catch (error) {
    // Only text that is not JSON is the statement's refusal; any other engine failure is loud.
    if (error instanceof Error && error.message.startsWith("WASM-ENGINE-INPUT-JSON-INVALID")) throw unreadable();
    throw error;
  }
  return readOutcome(outcome);
}

/** The reader (G0-G8) on these exact source bytes, bound to the invocation when one is
 * given. The invocation is written to text before the first await, so later caller edits
 * cannot alter what is read. */
export async function validateRetainedPrecisionBytes(sourceJson: string, invocation?: unknown): Promise<RetainedPrecisionValidation> {
  let invocationJson: string | undefined;
  try { invocationJson = invocation == null ? undefined : retainedJsonText(invocation); } catch { throw unreadable(); }
  const engine = await loadWasmEngine();
  return run(() => engine.validateRetainedPrecisionJson(sourceJson, invocationJson));
}

/** The reader (G0-G8) on a parsed statement, bound to the invocation when one is given. */
export async function validateRetainedPrecision(source: unknown, invocation?: unknown): Promise<RetainedPrecisionValidation> {
  let sourceJson: string;
  try { sourceJson = retainedJsonText(source); } catch { throw unreadable(); }
  return validateRetainedPrecisionBytes(sourceJson, invocation);
}

/** The reader's metadata-only transport checks (G0-G2 and the base's transport metadata
 * at G7): omitted raw rows are never reconstructed or verified, and the result is never
 * numerically eligible. */
export async function validateRetainedPrecisionTransport(source: unknown): Promise<RetainedPrecisionValidation> {
  let sourceJson: string;
  try { sourceJson = retainedJsonText(source); } catch { throw unreadable(); }
  const engine = await loadWasmEngine();
  return run(() => engine.validateRetainedPrecisionTransportJson(sourceJson));
}

export type RetainedClassificationSummary = {
  case_id: unknown;
  relative_verified: number;
  absolute_verified: number;
  interval_bindable: number;
  not_covered: number;
  input_derived: number;
  non_quantity: number;
  withheld: number;
};
type RetainedSemantics = Readonly<{
  standing: "numerically_eligible" | "needs_recompute";
  classification_summary: RetainedClassificationSummary[];
}>;
const semanticVerdicts = new WeakMap<RetainedPrecisionValidation, Map<string, RetainedSemantics>>();
/** Rust's semantic helpers over the registration's accepted reader verdict. The
 * registration binds it to this source; this function only carries their JSON.
 * The engine is already loaded by validation. A missing or failed engine is loud,
 * so the registration carrier can retain its named unavailable error state. */
export function retainedPrecisionSemantics(validation: RetainedPrecisionValidation, source: unknown, requested: readonly unknown[]): RetainedSemantics {
  const engine = loadedWasmEngine();
  if (!engine) throw new Error("RETAINED_PRECISION_READER_UNAVAILABLE");
  const sourceJson = retainedJsonText(source), requestedJson = retainedJsonText(requested);
  const key = `${sourceJson}\u0000${requestedJson}`;
  let entries = semanticVerdicts.get(validation);
  if (!entries) { entries = new Map(); semanticVerdicts.set(validation, entries); }
  if (!entries.has(key)) {
    entries.set(key, freeze(JSON.parse(engine.retainedPrecisionSemanticsJson(sourceJson, retainedJsonText(validation), requestedJson)) as RetainedSemantics));
    while (entries.size > 24) entries.delete(entries.keys().next().value!);
  }
  return entries.get(key)!;
}
