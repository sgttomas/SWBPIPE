/** C1 native custody, slice 4 (DEL-04-05 `c1-native-custody-plan.md`): the webview's handle on a
 * fresh solve result held natively (`solve_jobs::HeldResult`), and the one access layer to it.
 *
 * - **The handle.** A `NativeResult` carries the envelope members S states (`header.envelope`:
 *   identity, status, summary, numerical quality, formulation basis, professional boundary,
 *   diagnostics and `contract_evidence` without its per-row combination evidence), S itself, the
 *   native verdict and the native analysis-run preview, each bound to the exact texts it was read
 *   with. It has no `results` and no row-scale member: the type omits them, so every full-row
 *   consumer fails to compile against it, and at run time `results`, `source_block_recovery`,
 *   `retained_precision`, `toJSON` (so `JSON.stringify`) and `structuredClone` throw
 *   `NATIVE-RESULT-ROWS-GUARDED` rather than read a header as a result. Reader text
 *   (`readerSourceText`) refuses it too.
 * - **Access.** Rows come by bounded native page (`query`/`lookup`/`semantics`/`recovery`,
 *   `resultDetail.ts`), never as the envelope. A C2 consumer (reports, comparisons, rule checks,
 *   derivative exports, handoffs) may load the full result lazily through `loadFullResult`, only
 *   at or under 64 MiB; above it the native host refuses `RESULT-DETAIL-WEBVIEW-LIMIT` and
 *   `FullResultRefused` carries the sizes and the advice. No C1 surface calls it.
 * - **Delivery kinds.** The verdict command returns JSON through Tauri, so the webview receives an
 *   already parsed value (`decodeNativeVerdict` accepts exactly that); the summary, detail, row
 *   semantics and analysis-run preview commands return raw bytes (an ArrayBuffer), decoded by
 *   their readers. A text where bytes are expected, or bytes where a value is expected, is refused.
 * - **Binding.** A verdict is cached by (job, reader or dispatch, exact model text, exact
 *   invocation text); S by (job, exact model text, exact standing); the preview by (job,
 *   manifest, aggregate, exclusions). Disposal drops every entry of the job and marks the handle
 *   disposed, so a reply that arrives afterwards is discarded by its caller (`isLive`). */
import { invoke } from "@tauri-apps/api/core";
import { invokeWithIngress, type NativeIngressCommand } from "./nativeIngress";
import { nativeInvokeError } from "./memoryPool";
import { decodeResultSummary, type ResultSummary } from "./resultSummary";
import {
  lookupResultRows,
  queryResultRows,
  queryResultSourceRecovery,
  readResultRowSemantics,
  type DetailFilter,
  type Lookup,
  type LookupRequest,
  type ResultDetailCommand,
  type RowSemanticsOutcome,
  type RowsPage,
  type SourceRecovery
} from "./resultDetail";
import { ANALYSIS_RUN_PREVIEW_COMMAND, readAnalysisRunPreview, type AnalysisRunPreview } from "./analysisRunPreview";
import type { ReaderDispatch, ReaderName } from "../features/results/resultReader";
import type { AnalysisRunEnvelope, MechanicsResult } from "../types";

type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any

export const NATIVE_RESULT_ROWS_GUARDED = "NATIVE-RESULT-ROWS-GUARDED";
export const NATIVE_VERDICT_COMMAND = "read_preview_mechanics_job_verdict";
export const NATIVE_SUMMARY_COMMAND = "read_preview_mechanics_job_summary";
export const NATIVE_FULL_COPY_COMMAND = "read_preview_mechanics_job_rows_all";
export const NATIVE_FULL_COPY_RELEASE_COMMAND = "release_preview_mechanics_job_full_copy";
export const NATIVE_VERDICT_MALFORMED = "NATIVE-VERDICT-MALFORMED";
export const NATIVE_RESULT_DISPOSED = "NATIVE-RESULT-DISPOSED";
export const RESULT_DETAIL_WEBVIEW_LIMIT = "RESULT-DETAIL-WEBVIEW-LIMIT";
/** The limit the native host applies to a C2 full copy (PR #51's 64 MiB), for messages only. */
export const RESULT_DETAIL_WEBVIEW_LIMIT_BYTES = 64 * 1024 * 1024;

/** Members of the envelope a native handle never carries (row-scale, or omitted from S). */
export const NATIVE_GUARDED_MEMBERS = Object.freeze(["results", "source_block_recovery", "retained_precision"] as const);

/** What a native handle states of the envelope: S's `header.envelope` members. */
export type NativeResultHeader = Readonly<Omit<MechanicsResult, (typeof NATIVE_GUARDED_MEMBERS)[number]>>;

/** The bound inputs a native reading was read with. */
export type NativeBinding = Readonly<{
  /** The model text the verdict (standing) was read with: the solve's manifest model. */
  standingModelText: string;
  /** The invocation text the pressure-source standing was read with, when registered. */
  invocationText: string | null;
  /** The model text S was built with (the panels' model). */
  summaryModelText: string;
  /** The standing S was built with. */
  standing: Readonly<{ eligible: boolean; findings: readonly string[] }> | null;
}>;

/** Native custody of one fresh result. */
export type NativeCustody = Readonly<{
  jobId: string;
  /** The held result's exact length in bytes (the job's status). */
  resultBytes: number;
  summary: ResultSummary;
  /** The header dispatch and the standing, read natively with `binding`. */
  verdict: ReaderDispatch;
  binding: NativeBinding;
  /** The pressure-source recovery facts (page 0, no case texts), for routes that carry them. */
  recovery: SourceRecovery | null;
}>;

const NATIVE = Symbol("swbpipe.nativeResult");
export type NativeResult = NativeResultHeader & { readonly [NATIVE]: NativeCustody };

/** A full-row access on a native handle. */
export class NativeResultRowsGuarded extends Error {
  constructor(readonly member: string) {
    super(`${NATIVE_RESULT_ROWS_GUARDED}: a native result holds no ${member} in the webview; read it through S or a bounded native page.`);
    this.name = "NativeResultRowsGuarded";
  }
}

export function isNativeResult(value: unknown): value is NativeResult {
  return !!value && typeof value === "object" && NATIVE in (value as object);
}
/** The custody of a native handle. */
export function nativeCustody(result: NativeResult): NativeCustody {
  return result[NATIVE];
}
/** The custody when `value` is a native handle, else null. */
export function nativeCustodyOf(value: unknown): NativeCustody | null {
  return isNativeResult(value) ? value[NATIVE] : null;
}

/** Builds a handle from S and the readings. The envelope members come from S's header only. */
export function createNativeResult(custody: NativeCustody): NativeResult {
  const handle: Record<string | symbol, unknown> = {};
  for (const [key, value] of Object.entries(custody.summary.header.envelope)) handle[key] = value;
  for (const member of NATIVE_GUARDED_MEMBERS) {
    Object.defineProperty(handle, member, { enumerable: true, get() { throw new NativeResultRowsGuarded(member); } });
  }
  Object.defineProperty(handle, "toJSON", { enumerable: false, value() { throw new NativeResultRowsGuarded("serializable envelope"); } });
  Object.defineProperty(handle, NATIVE, { enumerable: false, value: custody });
  return Object.freeze(handle) as unknown as NativeResult;
}

/* -------------------------------------------------------------------------- *
 * Transport
 * -------------------------------------------------------------------------- */

/** The native result commands that take a raw-body ingress. */
export type NativeResultCommand =
  | typeof NATIVE_VERDICT_COMMAND
  | typeof NATIVE_SUMMARY_COMMAND
  | typeof ANALYSIS_RUN_PREVIEW_COMMAND
  | ResultDetailCommand;
/** Sends a native result command's arguments; returns what Tauri delivers. */
export type NativeResultInvoke = (command: NativeResultCommand, args: Record<string, unknown>) => Promise<unknown>;

/** The production transport: the raw-body ingress (`nativeIngress.ts`). */
export const tauriNativeResultInvoke: NativeResultInvoke = (command, args) => invokeWithIngress<unknown>(command as NativeIngressCommand, args);

let transport: NativeResultInvoke = tauriNativeResultInvoke;
/** Test support: the transport the access layer uses. */
export function setNativeResultTransportForTests(next: NativeResultInvoke | null): void {
  transport = next ?? tauriNativeResultInvoke;
}
function send(command: NativeResultCommand, args: Record<string, unknown>): Promise<unknown> {
  return transport(command, args);
}
/** The byte-response commands' adapter: an ArrayBuffer or a typed array, never text. */
const bytesInvoke = (command: NativeResultCommand, args: Record<string, unknown>) => send(command, args).then((delivered) => {
  if (delivered instanceof ArrayBuffer || ArrayBuffer.isView(delivered)) return delivered as ArrayBuffer | Uint8Array;
  if (Object.prototype.toString.call(delivered) === "[object ArrayBuffer]") return delivered as ArrayBuffer;
  if (typeof delivered === "string") return delivered;
  throw new Error(`NATIVE-RESULT-RESPONSE-KIND: ${command} delivered a ${delivered === null ? "null" : typeof delivered}, not bytes`);
});

/* -------------------------------------------------------------------------- *
 * The verdict (a parsed JSON value through Tauri)
 * -------------------------------------------------------------------------- */

function plain(value: unknown): value is Record<string, unknown> {
  return !!value && typeof value === "object" && !Array.isArray(value) && !(value instanceof ArrayBuffer) && !ArrayBuffer.isView(value);
}
function freeze<T>(value: T): T {
  if (value && typeof value === "object") {
    for (const item of Object.values(value)) freeze(item);
    Object.freeze(value);
  }
  return value;
}
function malformed(reason: string): never {
  throw new Error(`${NATIVE_VERDICT_MALFORMED}: ${reason}`);
}

/** Reads the verdict command's delivery: Tauri delivers its JSON body already parsed, so an object
 * with the dispatch's members. A string or bytes is not the command's delivery and is refused. */
export function decodeNativeVerdict(delivered: unknown): ReaderDispatch {
  if (!plain(delivered)) malformed(`the delivery is ${typeof delivered === "string" ? "text" : delivered === null ? "null" : "not an object"}, not the parsed dispatch`);
  const standing = delivered.standing;
  if (typeof delivered.ok !== "boolean" || typeof delivered.route !== "string" || !plain(standing)) malformed("not a dispatch outcome");
  if (typeof standing.eligible !== "boolean" || !Array.isArray(standing.findings)) malformed("the standing is not readable");
  return freeze(delivered as unknown as ReaderDispatch);
}

/** Reads a reader validation delivery (`reader` given): `{ok, ...}` as `readerValidate`'s outcome. */
export function decodeNativeValidation(delivered: unknown): { ok: true; eligible?: boolean; findings?: string[] } | { ok: false; error: { code: string; detail: string | null } } {
  if (!plain(delivered) || typeof delivered.ok !== "boolean") malformed("not a reader validation outcome");
  if (delivered.ok === false && !(plain(delivered.error) && typeof delivered.error.code === "string")) malformed("a refusal without a code");
  return freeze(delivered as Json);
}

export type VerdictRequest = Readonly<{ modelText?: string | null; invocationText?: string | null; reader?: ReaderName | null }>;

/* -------------------------------------------------------------------------- *
 * Per-job caches, bound by exact inputs, dropped on disposal
 * -------------------------------------------------------------------------- */

type JobEntries = { disposed: boolean; readings: Map<string, Promise<unknown>>; fullCopy: Promise<MechanicsResult> | null };
const jobs = new Map<string, JobEntries>();
function entries(jobId: string): JobEntries {
  let job = jobs.get(jobId);
  if (!job) {
    job = { disposed: false, readings: new Map(), fullCopy: null };
    jobs.set(jobId, job);
  }
  return job;
}
function live(jobId: string): JobEntries {
  const job = entries(jobId);
  if (job.disposed) throw new Error(`${NATIVE_RESULT_DISPOSED}: ${jobId}'s native custody has ended.`);
  return job;
}
function cached<T>(jobId: string, key: string, read: () => Promise<T>): Promise<T> {
  const job = live(jobId);
  const existing = job.readings.get(key);
  if (existing) return existing as Promise<T>;
  const pending = read().catch((error) => {
    // A refusal is not kept: a later request reads afresh (memory may permit it then).
    if (job.readings.get(key) === pending) job.readings.delete(key);
    throw error;
  });
  job.readings.set(key, pending);
  return pending;
}
const keyOf = (...parts: readonly (string | null | undefined)[]) => parts.map((part) => part ?? "\u0001").join("\u0000");

/** Whether the job's native custody is still held by this webview (no disposal seen). */
export function isNativeJobLive(jobId: string): boolean {
  return !jobs.get(jobId)?.disposed;
}
/** Marks the job disposed and drops every reading; a later reply is discarded by its caller. */
export function forgetNativeJob(jobId: string): void {
  const job = entries(jobId);
  job.disposed = true;
  job.readings.clear();
  job.fullCopy = null;
}
/** Test support: forget every job. */
export function resetNativeResultsForTests(): void {
  jobs.clear();
}

/** The verdict (dispatch, or one reader's validation with `reader`), cached by its exact inputs. */
export function readNativeVerdict(jobId: string, request: VerdictRequest): Promise<ReaderDispatch> {
  if (request.reader) throw new Error("readNativeVerdict: use readNativeValidation for a reader");
  return cached(jobId, keyOf("dispatch", request.modelText, request.invocationText), async () =>
    decodeNativeVerdict(await send(NATIVE_VERDICT_COMMAND, {
      jobId,
      ...(request.modelText != null ? { modelJson: request.modelText } : {}),
      ...(request.invocationText != null ? { invocationJson: request.invocationText } : {})
    })));
}
export function readNativeValidation(jobId: string, reader: ReaderName, request: Omit<VerdictRequest, "reader">) {
  return cached(jobId, keyOf("validate", reader, request.modelText, request.invocationText), async () =>
    decodeNativeValidation(await send(NATIVE_VERDICT_COMMAND, {
      jobId,
      reader,
      ...(request.modelText != null ? { modelJson: request.modelText } : {}),
      ...(request.invocationText != null ? { invocationJson: request.invocationText } : {})
    })));
}

/** S for the model text and standing, cached by both (`null`: no standing). */
export function readNativeSummary(jobId: string, modelText: string, standing: NativeBinding["standing"]): Promise<ResultSummary> {
  const standingArg = standing ? { eligible: standing.eligible, findings: [...standing.findings] } : null;
  return cached(jobId, keyOf("summary", modelText, JSON.stringify(standingArg)), async () => {
    const delivered = await bytesInvoke(NATIVE_SUMMARY_COMMAND, { jobId, modelJson: modelText, standing: standingArg });
    if (typeof delivered === "string") throw new Error("RESULT-SUMMARY-MALFORMED: a text response where bytes were expected");
    return decodeResultSummary(delivered);
  });
}

/** The native analysis-run preview (no record tree in the webview), cached by its inputs. */
export function readNativeRunPreview(
  jobId: string,
  inputs: Parameters<typeof readAnalysisRunPreview>[2]
): Promise<AnalysisRunPreview> {
  const key = keyOf("preview", inputs.inputManifest.manifest_sha256, inputs.inputManifest.canonical_bytes, inputs.ruleCheckAggregate ?? null, JSON.stringify(inputs.rulePackExclusions ?? []));
  return cached(jobId, key, () => readAnalysisRunPreview(bytesInvoke as never, jobId, inputs));
}

/* -------------------------------------------------------------------------- *
 * Bounded pages (C1 surfaces)
 * -------------------------------------------------------------------------- */

/** A native query the host refused for its arguments (for example a lone surrogate in a search,
 * which serde refuses as `INGRESS-ARGS-INVALID`): shown as a typed refusal, never as no rows. */
export class NativeQueryRefused extends Error {
  constructor(readonly code: string, message: string) {
    super(message);
    this.name = "NativeQueryRefused";
  }
}
function refusalOf(error: unknown): unknown {
  const read = nativeInvokeError(error);
  const text = read instanceof Error ? read.message : typeof read === "string" ? read : null;
  if (text) {
    const match = /(?:INVALID_REQUEST: )?([A-Z][A-Z0-9-]+(?:-[A-Z0-9]+)*):/.exec(text);
    if (match && (match[1] === "INGRESS-ARGS-INVALID" || match[1].startsWith("RESULT-DETAIL-"))) return new NativeQueryRefused(match[1], text);
  }
  return read;
}
async function page<T>(jobId: string, read: () => Promise<T>): Promise<T> {
  live(jobId);
  try {
    return await read();
  } catch (error) {
    throw refusalOf(error);
  }
}
const detailInvoke = bytesInvoke as unknown as Parameters<typeof queryResultRows>[0];

export function queryNativeRows(result: NativeResult, filter: DetailFilter, offset: number, limit: number): Promise<RowsPage> {
  const { jobId } = nativeCustody(result);
  return page(jobId, () => queryResultRows(detailInvoke, jobId, filter, offset, limit));
}
export function lookupNativeRows(result: NativeResult, request: LookupRequest): Promise<Lookup> {
  const { jobId } = nativeCustody(result);
  return page(jobId, () => lookupResultRows(detailInvoke, jobId, request));
}
export function readNativeRowSemantics(result: NativeResult, ordinals: readonly number[]): Promise<readonly RowSemanticsOutcome[]> {
  const { jobId } = nativeCustody(result);
  return page(jobId, () => readResultRowSemantics(detailInvoke, jobId, ordinals));
}
export function queryNativeRecovery(jobId: string, offset: number, limit: number, raw = false): Promise<SourceRecovery> {
  return page(jobId, () => queryResultSourceRecovery(detailInvoke, jobId, offset, limit, raw));
}
/** The id of the row at each ordinal, by bounded lookup (pages of at most 1,000 keys). */
export async function nativeIdsOf(result: NativeResult, ordinals: readonly number[]): Promise<Map<number, string>> {
  const ids = new Map<number, string>();
  const unique = [...new Set(ordinals)];
  for (let i = 0; i < unique.length; i += 1000) {
    const keys = unique.slice(i, i + 1000);
    const found = await lookupNativeRows(result, { ordinals: keys, match: "first" });
    found.ordinals.forEach((row, at) => {
      if (row) ids.set(keys[at], row.row.id);
    });
  }
  return ids;
}

/* -------------------------------------------------------------------------- *
 * C2's bounded full copy
 * -------------------------------------------------------------------------- */

export const C2_RESULT_ADVICE = "remove the load cases and combinations this review does not need (Load Cases section); or solve the cases in smaller groups, each in its own copy of the model document (File > Save Model Document…), with the combinations in a copy holding only the cases they use; or split a large model into parts.";

/** A C2 consumer's full copy refused for its size (`RESULT-DETAIL-WEBVIEW-LIMIT`). */
export class FullResultRefused extends Error {
  constructor(readonly code: string, readonly resultBytes: number | null, readonly limitBytes: number | null, readonly nativeMessage: string) {
    super(nativeMessage);
    this.name = "FullResultRefused";
  }
}

const MIB = 1024 * 1024;
function size(bytes: number): string {
  const mib = bytes / MIB;
  return `${bytes} bytes (${mib >= 1024 ? `${(mib / 1024).toFixed(2).replace(/\.?0+$/, "")} GiB` : `${mib.toFixed(2).replace(/\.?0+$/, "")} MiB`})`;
}
/** The refusal a C2 feature shows: the sizes, what still works, and #51's advice. */
export function describeFullResultRefused(refusal: FullResultRefused, feature: string): string {
  const sizes = refusal.resultBytes !== null && refusal.limitBytes !== null
    ? `This result is ${size(refusal.resultBytes)}, above the ${size(refusal.limitBytes)} the webview can hold whole`
    : "This result is above the size the webview can hold whole";
  return `${refusal.code}: ${sizes}, so ${feature} cannot read it until its native path lands. The results themselves are still available: the summary, the tables, the viewport and the paged result detail read the result natively. To use ${feature} now: ${C2_RESULT_ADVICE}`;
}

/** Parses the native refusal text `RESULT-DETAIL-WEBVIEW-LIMIT (result_bytes=…, limit_bytes=…): …`. */
export function parseFullResultRefusal(error: unknown): FullResultRefused | null {
  const read = nativeInvokeError(error);
  const text = read instanceof Error ? read.message : typeof read === "string" ? read : "";
  const match = /RESULT-DETAIL-WEBVIEW-LIMIT \(result_bytes=(\d+), limit_bytes=(\d+)\)/.exec(text);
  if (!match) return null;
  return new FullResultRefused(RESULT_DETAIL_WEBVIEW_LIMIT, Number(match[1]), Number(match[2]), text.replace(/^INVALID_REQUEST: /, ""));
}

/** Reads the delivered bytes of a full copy into text. */
function copyText(delivered: unknown): string {
  let bytes: Uint8Array;
  if (Object.prototype.toString.call(delivered) === "[object ArrayBuffer]") bytes = new Uint8Array(delivered as ArrayBuffer);
  else if (ArrayBuffer.isView(delivered)) bytes = new Uint8Array(delivered.buffer, delivered.byteOffset, delivered.byteLength);
  else throw new Error("RESULT-DETAIL-FULL-COPY-KIND: the full copy is delivered as bytes");
  return new TextDecoder("utf-8", { fatal: true }).decode(bytes);
}

/** The full-copy transport (a plain command with a small JSON argument; the response is bytes). */
export type FullCopyInvoke = (command: typeof NATIVE_FULL_COPY_COMMAND | typeof NATIVE_FULL_COPY_RELEASE_COMMAND, args: { jobId: string }) => Promise<unknown>;
let fullCopyTransport: FullCopyInvoke = (command, args) => invoke<unknown>(command, args);
export function setFullCopyTransportForTests(next: FullCopyInvoke | null): void {
  fullCopyTransport = next ?? ((command, args) => invoke<unknown>(command, args));
}

/** C2 only (reports, comparisons, rule checks, derivative exports, handoffs): the full result,
 * copied once per job at or under 64 MiB and registered by `register` (the legacy reader path on
 * the exact copied bytes). Above the limit, rejects with `FullResultRefused`. The copy's custody
 * lease is released with the job's disposal. */
export function loadFullResult(result: NativeResult, register: (text: string) => Promise<MechanicsResult>): Promise<MechanicsResult> {
  const { jobId } = nativeCustody(result);
  const job = live(jobId);
  if (job.fullCopy) return job.fullCopy;
  const pending = (async () => {
    let delivered: unknown;
    try {
      delivered = await fullCopyTransport(NATIVE_FULL_COPY_COMMAND, { jobId });
    } catch (error) {
      throw parseFullResultRefusal(error) ?? nativeInvokeError(error);
    }
    if (job.disposed) throw new Error(`${NATIVE_RESULT_DISPOSED}: ${jobId}'s native custody ended while its full copy was read.`);
    return register(copyText(delivered));
  })();
  job.fullCopy = pending;
  pending.catch(() => {
    if (job.fullCopy === pending) job.fullCopy = null;
  });
  return pending;
}
/** The C2 full copy already loaded for this handle, without loading it. */
export function loadedFullResult(result: NativeResult): Promise<MechanicsResult> | null {
  return jobs.get(nativeCustody(result).jobId)?.fullCopy ?? null;
}

/* -------------------------------------------------------------------------- *
 * The native analysis-run record (its preview header)
 * -------------------------------------------------------------------------- */

const NATIVE_RUN = Symbol("swbpipe.nativeAnalysisRun");
type RowScaleRunMember = "diagnostics" | "result_refs" | "contract_evidence" | "source_block_recovery" | "retained_precision";
/** The record the session holds for a native result: the preview's record header (every member
 * but the row-scale ones), branded. Its row-scale members are not in the type, and at run time
 * reading them, `JSON.stringify` or `structuredClone` throws `NATIVE-RESULT-ROWS-GUARDED`; the full
 * record stays native, for Record Run and Save Project. */
export type NativeAnalysisRun = Readonly<Omit<AnalysisRunEnvelope, "analysis_run">> & {
  readonly analysis_run: Readonly<Omit<AnalysisRunEnvelope["analysis_run"], RowScaleRunMember>>;
  readonly [NATIVE_RUN]: AnalysisRunPreview;
};

export function isNativeAnalysisRun(value: unknown): value is NativeAnalysisRun {
  return !!value && typeof value === "object" && NATIVE_RUN in (value as object);
}
/** The native preview a native record was made from. */
export function nativeRunPreview(run: NativeAnalysisRun): AnalysisRunPreview {
  return run[NATIVE_RUN];
}
/** The record header of a native preview, branded and guarded. */
export function createNativeAnalysisRun(preview: AnalysisRunPreview): NativeAnalysisRun {
  const run: Record<string | symbol, unknown> = { ...preview.record.analysis_run };
  for (const member of ["diagnostics", "result_refs", "contract_evidence", "source_block_recovery", "retained_precision"] as const) {
    Object.defineProperty(run, member, { enumerable: true, get() { throw new NativeResultRowsGuarded(`analysis_run.${member}`); } });
  }
  const record: Record<string | symbol, unknown> = { ...preview.record, analysis_run: Object.freeze(run) };
  Object.defineProperty(record, "toJSON", { enumerable: false, value() { throw new NativeResultRowsGuarded("serializable analysis-run record"); } });
  Object.defineProperty(record, NATIVE_RUN, { enumerable: false, value: preview });
  return Object.freeze(record) as unknown as NativeAnalysisRun;
}

/** The session's result: a full result in the webview (historical, browser reference, legacy) or a
 * native handle (a fresh desktop solve). */
export type SessionResult = MechanicsResult | NativeResult;
/** The session's record: the full record, or a native result's record header. */
export type SessionAnalysisRun = AnalysisRunEnvelope | NativeAnalysisRun;

/** The result's row count: S's for a native handle, else its rows'. */
export function resultRowCount(result: SessionResult): number {
  return isNativeResult(result) ? result[NATIVE].summary.header.row_count : result.results.length;
}
/** The result's distinct units in first-seen order: S's for a native handle, else its rows'. */
export function resultUnits(result: SessionResult): string[] {
  return isNativeResult(result) ? result[NATIVE].summary.header.units.map((unit) => unit as string) : [...new Set(result.results.map((row) => row.unit))];
}
