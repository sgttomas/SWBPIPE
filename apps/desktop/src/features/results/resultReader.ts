/** R2 (b7-plan Unit R, "All readers."): the desktop's base-route readers are the Rust
 * readers (`core/reporting/result_export/src/reader_verdict.rs`), compiled to wasm and
 * loaded through `loadWasmEngine`. This module carries JSON text to them and their
 * verdicts back; it reads nothing itself. As R1's `retainedPrecisionReader.ts`, it is
 * the only desktop path to these readers.
 *
 * - The text a reader reads is the text the desktop received: `rememberReceivedText`
 *   keeps a native solve's bytes beside the result read from them (previewService), and
 *   `readerSourceText` hands them over while that result is unchanged. A value with no
 *   received text (a saved record, a copy, an edited or built value) is written with
 *   `jsonTextKeepingNegativeZero`, after a check that every number is finite. The Rust
 *   readers compare some numbers by their JSON form, so a re-serialization can read as
 *   another statement; only received text is the statement as published.
 * - The verdicts are synchronous: the engine is loaded before the desktop renders
 *   (`main.tsx`) and before every test (`src/test/setup.ts`). Without it nothing is
 *   read: a dispatch is unsupported and a reading refused, with
 *   RESULT_READER_UNAVAILABLE (fail closed).
 * - A value that is not JSON (a non-finite number, a lone surrogate) is refused at the
 *   text boundary with JSON_PARSE_REJECTED, as the native corpus states such texts.
 *
 * Verdicts are cached by the exact texts read. */
import { jsonTextKeepingNegativeZero } from "../../services/jsonText";
import { loadedWasmEngine, type WasmOperationEngine } from "../../services/wasmEngine/loadWasmEngine";
import { timedSolvePass } from "../../services/solveTimeline";
import type { SourceContract } from "./numericalResultQuality";
import type { MechanicsResult } from "../../types";
import { isNativeResult, nativeCustody, NativeResultRowsGuarded, type NativeResult } from "../../services/nativeResult";

/** A reader refusal: its code (the error message's leading token) and its whole text. */
export class ResultReaderError extends Error {
  constructor(readonly code: string, readonly detail: string | null = null) { super(detail ?? code); }
}
/** The engine is not loaded (or failed), so nothing was read. */
export const RESULT_READER_UNAVAILABLE = "RESULT_READER_UNAVAILABLE";
/** The value has no JSON text a reader can read. */
export const JSON_PARSE_REJECTED = "JSON_PARSE_REJECTED";

/** Rust `reader_verdict::READERS`. pressure-2 has no transport reader (Rust refuses it
 * as SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED); a retired reader name is READER_UNKNOWN. */
export type ReaderName = "preview-physics" | "pressure-source" | "source-recovery" | "pressure";
export type TransportReaderName = Exclude<ReaderName, "source-recovery">;
export type StandingToken = "numerically_eligible" | "needs_recompute" | "unsupported";
/** G1: the desktop's numerical standing, as Rust computes it. */
export type ReaderStanding = Readonly<{ route: SourceContract; token: StandingToken; status: "integrity_checked" | "needs_recompute"; eligible: boolean; findings: readonly string[] }>;
export type ContractBinding = Readonly<{ id: string; sha256: string }>;
/** G5: the header dispatch's route, its table binding, its refusal and the standing. */
export type ReaderDispatch = Readonly<{ ok: boolean; route: SourceContract; binding: ContractBinding | null; error: Readonly<{ code: string; detail: string | null }> | null; standing: ReaderStanding }>;
/** A reader's pass: `eligible` from the readers that grant it, and the source readers' findings. */
export type ReaderValidation = Readonly<{ eligible?: boolean; findings?: readonly string[] }>;
/** One table row of the source's semantic table (its signature). */
export type SemanticSignature = Readonly<{
  signature_id: string; kind: string; unit: string; component: string | null; category: string; family: string | null;
  derivative_target_dimension: string | null; source_physical_semantic_dimension: string | null; canonical_disposition: string;
  legacy_declared_dimension: string | null; legacy_run_creation_admission: unknown; governing_ratio_eligible: unknown; source_basis?: string;
}>;
/** One row's semantics under the source's table. */
export type RowSemantics = Readonly<{ signature: SemanticSignature | null; complete_metadata: boolean; canonical_metadata: Readonly<Record<string, string>> | null }>;
type Refusal = { ok: false; error: { code: string; detail: string | null } };
type Outcome<T> = ({ ok: true } & T) | Refusal;
type SourceRow = MechanicsResult["results"][number];

function freeze<T>(value: T): T {
  if (value && typeof value === "object") {
    for (const item of Object.values(value)) freeze(item);
    Object.freeze(value);
  }
  return value;
}

/* -------------------------------------------------------------------------- *
 * Text
 * -------------------------------------------------------------------------- */
const received = new WeakMap<object, { text: string; written: string }>();
class NotJson extends Error {}
function assertFinite(value: unknown): void {
  if (typeof value === "number") { if (!Number.isFinite(value)) throw new NotJson(); return; }
  if (Array.isArray(value)) { for (const item of value) assertFinite(item); return; }
  if (value !== null && typeof value === "object") for (const item of Object.values(value)) assertFinite(item);
}
/** JSON text of a value with no received text: `jsonTextKeepingNegativeZero`, when
 * every number is finite (JSON text has no other numbers). */
function writtenText(value: unknown): string {
  // RT-U4: timed for the latest solve's timeline (solveTimeline.ts); unchanged otherwise.
  return timedSolvePass("reader_written_text", () => {
    assertFinite(value);
    return jsonTextKeepingNegativeZero(value);
  }, (text) => text.length);
}
/** b7-plan R2 (as R1's "Exact bytes to the desktop"): `text` is the received JSON text
 * `source` was read from. The readers read it while `source` is unchanged. */
export function rememberReceivedText(source: object, text: string): void {
  try { received.set(source, { text, written: writtenText(source) }); } catch { /* not JSON: nothing is kept */ }
}
/** The text a reader reads for this value (a statement, a model or an invocation): its
 * received text while the value is unchanged since it was read from it, otherwise the
 * value written as JSON text. Throws for a value that has no JSON text. */
export function readerSourceText(source: unknown): string {
  // A native handle has no text in the webview: its readers run natively.
  if (isNativeResult(source)) throw new NativeResultRowsGuarded("reader text");
  let written: string;
  try { written = writtenText(source); } catch { throw new ResultReaderError(JSON_PARSE_REJECTED); }
  const kept = source && typeof source === "object" ? received.get(source) : undefined;
  return kept && kept.written === written ? kept.text : written;
}
function optionalText(value: unknown): string | undefined {
  return value === undefined || value === null ? undefined : readerSourceText(value);
}

/* -------------------------------------------------------------------------- *
 * The bindings, cached by the texts read
 * -------------------------------------------------------------------------- */
const CACHE_TEXTS = 24;
const cache = new Map<string, Map<string, unknown>>();
function cached<T>(sourceJson: string, key: string, read: () => T): T {
  let entries = cache.get(sourceJson);
  if (entries) { cache.delete(sourceJson); cache.set(sourceJson, entries); }
  else {
    entries = new Map();
    cache.set(sourceJson, entries);
    while (cache.size > CACHE_TEXTS) cache.delete(cache.keys().next().value!);
  }
  if (!entries.has(key)) entries.set(key, read());
  return entries.get(key) as T;
}
function engine(): WasmOperationEngine | null {
  return loadedWasmEngine();
}
/** One binding call: its outcome JSON, or the refusal when the text is not JSON. The
 * engine is loaded (`unavailable` is checked before any reading is cached). */
function call<T>(read: (e: WasmOperationEngine) => string): Outcome<T> {
  const e = engine()!;
  try { return freeze(JSON.parse(read(e)) as Outcome<T>); } catch (error) {
    if (error instanceof Error && error.message.startsWith("WASM-ENGINE-INPUT-JSON-INVALID")) return { ok: false, error: { code: JSON_PARSE_REJECTED, detail: null } };
    throw error;
  }
}
/** Refuses the reading when the engine is not loaded. */
function unavailable(): void {
  if (!engine()) throw new ResultReaderError(RESULT_READER_UNAVAILABLE);
}
function refuse(outcome: Refusal): never {
  throw new ResultReaderError(outcome.error.code, outcome.error.detail);
}
function unsupportedDispatch(code: string): ReaderDispatch {
  return freeze({ ok: false, route: "unsupported", binding: null, error: { code, detail: null },
    standing: { route: "unsupported", token: "unsupported", status: "needs_recompute", eligible: false, findings: [code] } });
}

/** G5 and G1: the header dispatch of this statement (route, table binding, refusal)
 * and its standing with the model's load cases and, for a source route, the
 * invocation it was captured with. */
export function readerDispatch(source: unknown, model?: unknown, invocation?: unknown): ReaderDispatch {
  if (isNativeResult(source)) return nativeDispatch(source, model, invocation);
  let sourceJson: string, modelJson: string | undefined, invocationJson: string | undefined;
  try { sourceJson = readerSourceText(source); modelJson = optionalText(model); invocationJson = optionalText(invocation); }
  catch (error) { if (error instanceof ResultReaderError) return unsupportedDispatch(error.code); throw error; }
  const e = engine();
  // Nothing is read, and nothing is cached, without the engine.
  if (!e) return unsupportedDispatch(RESULT_READER_UNAVAILABLE);
  return cached(sourceJson, `dispatch\u0000${modelJson ?? ""}\u0000${invocationJson ?? ""}`, () => {
    try {
      const verdict = timedSolvePass("reader_wasm_dispatch", () => e.readerDispatchJson(sourceJson, modelJson, invocationJson), () => sourceJson.length);
      return freeze(JSON.parse(verdict) as ReaderDispatch);
    } catch (error) {
      if (error instanceof Error && error.message.startsWith("WASM-ENGINE-INPUT-JSON-INVALID")) return unsupportedDispatch(JSON_PARSE_REJECTED);
      throw error;
    }
  });
}
/** C1: the native verdict of a held result. The header dispatch (route, binding, refusal) is the
 * same whatever the model and invocation; the standing is the one read natively with the handle's
 * bound texts, so a request with other texts reads as not bound (fail closed), never as another
 * model's standing. */
const NATIVE_VERDICT_NOT_BOUND = "NATIVE_VERDICT_NOT_BOUND";
function nativeDispatch(source: NativeResult, model?: unknown, invocation?: unknown): ReaderDispatch {
  const { verdict, binding } = nativeCustody(source);
  if (model === undefined && invocation === undefined) return verdict;
  let modelText: string | undefined, invocationText: string | undefined;
  try { modelText = optionalText(model); invocationText = optionalText(invocation); } catch { return unsupportedDispatch(JSON_PARSE_REJECTED); }
  if (modelText === binding.standingModelText && (invocationText ?? null) === binding.invocationText) return verdict;
  return freeze({ ...verdict, standing: { ...verdict.standing, token: "needs_recompute", status: "needs_recompute", eligible: false, findings: [NATIVE_VERDICT_NOT_BOUND] } }) as ReaderDispatch;
}
/** One reader's validation of this statement (G2: with the current model; the source
 * readers with their captured invocation). Throws the reader's refusal. */
export function readerValidate(reader: ReaderName, source: unknown, model?: unknown, invocation?: unknown): ReaderValidation {
  const sourceJson = readerSourceText(source), modelJson = optionalText(model), invocationJson = optionalText(invocation);
  unavailable();
  const outcome = cached(sourceJson, `validate\u0000${reader}\u0000${modelJson ?? ""}\u0000${invocationJson ?? ""}`,
    () => call<{ eligible?: boolean; findings?: string[] }>(e => timedSolvePass("reader_wasm_validate", () => e.readerValidateJson(reader, sourceJson, modelJson, invocationJson), () => sourceJson.length)));
  if (!outcome.ok) refuse(outcome);
  const { ok: _, ...validation } = outcome;
  return validation;
}
/** One reader's transport (metadata-only) checks of a transported statement; never
 * eligible. Throws the reader's refusal. */
export function readerTransport(reader: TransportReaderName, source: unknown): void {
  const sourceJson = readerSourceText(source);
  unavailable();
  const outcome = cached(sourceJson, `transport\u0000${reader}`, () => call<object>(e => timedSolvePass("reader_wasm_transport", () => e.readerTransportJson(reader, sourceJson), () => sourceJson.length)));
  if (!outcome.ok) refuse(outcome);
}
/** A row's semantics read only its kind, unit and the five metadata members
 * (`semantic_contract::signature_in`, `complete_metadata`, `canonical_metadata_in`:
 * each vocabulary key is one of the five), so the row read is those members alone,
 * and rows that agree in them share one reading. */
const ROW_METADATA = ["component", "coordinate_system", "location", "basis", "sign_convention"] as const;
function rowText(row: SourceRow): string {
  const metadata = row?.metadata as Record<string, unknown> | null | undefined;
  const picked = metadata && typeof metadata === "object" && !Array.isArray(metadata)
    ? Object.fromEntries(ROW_METADATA.filter(k => Object.hasOwn(metadata, k)).map(k => [k, metadata[k]])) : null;
  return writtenText({ kind: row?.kind, unit: row?.unit, metadata: picked });
}
/** The readings of the source's own rows, read in one call with the source read
 * once (the outcomes a call per row returns), once per source text. Anything that
 * call does not read (a source that is not JSON, a row outside the source) is
 * left to the call per row. */
function readSourceRows(source: MechanicsResult, sourceJson: string): void {
  cached(sourceJson, "rows\u0000", () => {
    const texts = new Set<string>();
    for (const row of Array.isArray(source.results) ? source.results : []) {
      try { texts.add(rowText(row)); } catch { /* read per row, as before */ }
    }
    const rows = [...texts];
    let outcomes: unknown;
    try { outcomes = JSON.parse(engine()!.readerRowSemanticsEachJson(`[${rows.join(",")}]`, sourceJson)); } catch { return true; }
    const read = Array.isArray(outcomes) && outcomes.length === rows.length ? outcomes : null;
    read?.forEach((outcome, i) => cached(sourceJson, `row\u0000${rows[i]}`, () => freeze(outcome) as Outcome<RowSemantics>));
    return true;
  });
}
/** The row semantics of this source's table (no source is refused): a
 * function from a row to its signature, complete-metadata flag and canonical metadata.
 * It throws the reader's refusal: a source the dispatch refuses
 * (SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED), or a contradictory row. Call it once per
 * source and read every row through it. */
export function readerRowSemantics(source?: MechanicsResult | null): (row: SourceRow) => RowSemantics {
  let sourceJson: string | undefined, failure: ResultReaderError | null = null;
  try { sourceJson = source == null ? undefined : readerSourceText(source); }
  catch (error) { if (!(error instanceof ResultReaderError)) throw error; failure = new ResultReaderError("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED"); }
  return row => {
    if (failure) throw failure;
    let text: string;
    try { text = rowText(row); } catch { throw new ResultReaderError(JSON_PARSE_REJECTED); }
    unavailable();
    if (source != null && sourceJson !== undefined) readSourceRows(source, sourceJson);
    const outcome = cached(sourceJson ?? "", `row\u0000${text}`,
      () => call<RowSemantics>(e => timedSolvePass("reader_wasm_row", () => e.readerRowSemanticsJson(text, sourceJson), () => sourceJson?.length ?? 0)));
    if (!outcome.ok) refuse(outcome);
    return { signature: outcome.signature, complete_metadata: outcome.complete_metadata, canonical_metadata: outcome.canonical_metadata };
  };
}

/* -------------------------------------------------------------------------- *
 * The route, presented
 * -------------------------------------------------------------------------- */
/** The desktop route of a statement: Rust's header dispatch (G5). A refused header is
 * `unsupported`. A header never authenticates its claimed producer. */
export function sourceContract(source: MechanicsResult | NativeResult): SourceContract {
  return readerDispatch(source).route;
}
/** The route's pinned table binding; throws SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED for a
 * statement the dispatch refuses. */
export function sourceSemanticBinding(source: MechanicsResult | NativeResult): ContractBinding {
  const binding = readerDispatch(source).binding;
  if (!binding) throw new Error("SOURCE_SEMANTIC_CONTRACT_UNSUPPORTED");
  return { id: binding.id, sha256: binding.sha256 };
}
/** Exact known (readable) contract binding, or null. This does not authenticate a
 * producer, mint numerical eligibility or admit a historical identity as fresh. */
export function currentSemanticContract(source: MechanicsResult | NativeResult | null | undefined): ContractBinding | null {
  if (!source) return null;
  try { return sourceSemanticBinding(source); } catch { return null; }
}
export function hasCurrentSourceContract(source: MechanicsResult | NativeResult | null | undefined): boolean {
  return currentSemanticContract(source) !== null;
}
/** The header's downgrade refusal: a `retained_precision` member, or a raw row with the
 * W1 method token, under an identity other than a successor's. */
export function retainedPrecisionDowngrade(source: MechanicsResult | NativeResult): boolean {
  return readerDispatch(source).error?.code === "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN";
}
