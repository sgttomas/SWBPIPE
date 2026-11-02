/** RT-U4 (DEL-04-05, release target exit criterion 9): the latest solve's timeline,
 * from Analyze → Run to the first paint of its result. Observation only: nothing reads
 * it to decide anything, and it changes no behaviour.
 *
 * - Marks: each stage of the latest solve once, at `performance.now()` (a monotonic
 *   clock), with small details (byte and row counts, codes). `solve_requested` starts
 *   a new timeline and drops the previous one.
 * - Passes: every timed serialization, fingerprint or wasm reader call, aggregated by
 *   kind and by the last stage reached before it ran, so passes after the paint
 *   (Record Run, Save Project) are counted too. A pass whose text is at least half
 *   the received result's (or 1 MiB before it is known) counts as full-envelope.
 *   Nested passes are counted under each kind: a `native_fingerprint` includes one
 *   `checked_json_text`.
 * - Failure: the first error the solve reports, with the last stage reached, as its
 *   leading code and error class only.
 *
 * Timings, counts and codes only: no model data, path, identifier (job, run, model or
 * project) or message text is kept for reading. A detail string that is not a code is
 * dropped (`codeOrNull`), as is a detail under a key other than a measurement field
 * (`DETAIL_KEYS`) and an error class outside `ERROR_NAMES`. Kept small: a bounded mark list, one aggregate per (kind,
 * stage), and a WeakRef to the received result (never a copy of it).
 *
 * `readSolveTimeline` returns a fresh copy. The read-only global
 * `globalThis.__openPipeStressSolveTimelineV1` ([`solveTimelineGlobal`]) is installed by
 * `uiDiagnostics.ts` under the very condition that installs `__openPipeStressUiDiagnosticsV1`,
 * so it is present exactly where that one is; the diagnostics global itself is unchanged. */

import { LOCAL_REASON_CODES, READER_VERDICT_CODES, SOLVE_PATH_CODES } from "./solveTimelineCodes";

export const SOLVE_TIMELINE_SCHEMA = "swbpipe.solve-timeline/v1" as const;
export const SOLVE_TIMELINE_GLOBAL = "__openPipeStressSolveTimelineV1" as const;

export type SolveStage =
  | "solve_requested"
  | "job_started"
  | "job_done"
  | "take_start"
  | "take_end"
  | "decode_end"
  | "parse_end"
  | "reader_verdict_start"
  | "reader_verdict_end"
  | "analysis_run_built"
  | "result_set"
  | "result_committed"
  | "first_paint";

export type SolvePassKind =
  | "checked_json_text"
  | "native_fingerprint"
  | "reader_written_text"
  | "reader_wasm_dispatch"
  | "reader_wasm_validate"
  | "reader_wasm_transport"
  | "reader_wasm_row";

export type SolveOutcome = "running" | "completed" | "failed" | "refused" | "cancelled" | "superseded";
type Detail = Readonly<Record<string, string | number | boolean | null>>;

// The strings the timeline may keep, and nothing else: an EXACT allow-list, no pattern.
// - the lower-case values it records: solver modes and native job states, exactly;
// - the codes of `solveTimelineCodes.ts`, generated from the sources whose codes it can record
//   (`scripts/solveTimelineCodes.mjs`; `solveTimelineCodes.test.ts` fails when they differ).
// Anything else, however code-shaped (`SOURCE_CUSTOMER_ACME`, `ANALYSIS_PROJECT_SECRET`), an
// identifier (`secret_project`, `ALICE`, `RK-H0012`), a file name or message text is dropped.
export const LOWER_CODES: readonly string[] = [
  "sparse_interactive", "dense_scrutiny",
  "queued", "running", "completed", "cancelled", "failed", "refused"
];
const KEPT: ReadonlySet<string> = new Set([...LOWER_CODES, ...SOLVE_PATH_CODES, ...READER_VERDICT_CODES, ...LOCAL_REASON_CODES]);
/** `value` when it is one of the timeline's codes (see above), else null. */
export function codeOrNull(value: unknown): string | null {
  return typeof value === "string" && KEPT.has(value) ? value : null;
}
/** The leading code of an error's message (`CODE: …` or `CODE …`), else null. */
export function errorCode(error: unknown): string | null {
  const message = String((error as { message?: unknown })?.message ?? error ?? "");
  return codeOrNull(message.match(/^([A-Z][A-Z0-9_-]*)(?=[:\s]|$)/)?.[1] ?? null);
}
// The error classes a solve can raise, exactly: JavaScript's own, the DOM's that a take or a
// decode can raise, and the app's. Any other name (`ExampleCustomerError`) is dropped.
export const ERROR_NAMES: readonly string[] = [
  "Error", "TypeError", "RangeError", "SyntaxError", "ReferenceError", "EvalError", "URIError", "AggregateError",
  "AbortError", "DataCloneError", "QuotaExceededError",
  "ResultReaderError", "RetainedPrecisionError", "IngressRequestRetiredError", "ResourceUnavailableError"
];
const KEPT_ERROR_NAMES: ReadonlySet<string> = new Set(ERROR_NAMES);
function errorName(error: unknown): string | null {
  const name = (error as { name?: unknown })?.name;
  return typeof name === "string" && KEPT_ERROR_NAMES.has(name) ? name : null;
}
// The measurement fields the recording sites write, exactly. A detail under any other key is
// dropped with its key, so a key cannot carry an identifier (`EXAMPLE_CUSTOMER`) either.
export const DETAIL_KEYS: readonly string[] = [
  "solver_mode", "state", "result_bytes", "worker_peak_footprint_bytes", "worker_peak_grant_bytes",
  "bytes", "chars", "rows", "registered", "reason_code"
];
const KEPT_DETAIL_KEYS: ReadonlySet<string> = new Set(DETAIL_KEYS);
// Kept keys only; numbers and booleans as given, a string only if it is a code.
function cleanDetail(detail: Detail | null): Detail | null {
  if (!detail) return null;
  return Object.fromEntries(Object.entries(detail)
    .filter(([key]) => KEPT_DETAIL_KEYS.has(key))
    .map(([key, value]) => [key, typeof value === "string" ? codeOrNull(value) : value]));
}

export type SolveTimelineMark = Readonly<{ stage: SolveStage; at_ms: number; since_start_ms: number; detail: Detail | null }>;
export type SolveTimelinePass = Readonly<{
  kind: SolvePassKind;
  after_stage: SolveStage;
  count: number;
  total_ms: number;
  max_ms: number;
  full_count: number;
  full_ms: number;
  max_chars: number;
}>;
export type SolveTimelineSnapshot = Readonly<{
  schema: typeof SOLVE_TIMELINE_SCHEMA;
  sequence: number;
  time_origin_ms: number;
  outcome: SolveOutcome;
  marks: readonly SolveTimelineMark[];
  passes: readonly SolveTimelinePass[];
  dropped_marks: number;
  failure: Readonly<{ stage: SolveStage; code: string | null; error_name: string | null; at_ms: number }> | null;
}>;

const MAX_MARKS = 64;
const FULL_FLOOR_CHARS = 1 << 20;

type PassTotals = { count: number; total_ms: number; max_ms: number; full_count: number; full_ms: number; max_chars: number };
type Timeline = {
  sequence: number;
  started: number;
  outcome: SolveOutcome;
  jobId: string | null;
  marks: SolveTimelineMark[];
  dropped: number;
  passes: Map<string, PassTotals>;
  failure: { stage: SolveStage; code: string | null; error_name: string | null; at_ms: number } | null;
  result: WeakRef<object> | null;
  resultChars: number | null;
};

let sequence = 0;
let current: Timeline | null = null;
const now = () => performance.now();

/** The run that began a timeline: its marks and its end apply only while that
 * timeline is still the latest, so a superseded run cannot write into a newer one. */
export type SolveTimelineHandle = Readonly<{
  mark(stage: SolveStage, detail?: Detail | null): void;
  /** Marks `job_started` and remembers the job (kept internally, never read out). */
  jobStarted(jobId: string): void;
  end(outcome: Exclude<SolveOutcome, "running">, error?: unknown): void;
}>;

/** Starts the timeline of a new solve (Analyze → Run), replacing the previous one. */
export function beginSolveTimeline(detail: Detail | null = null): SolveTimelineHandle {
  const started = now();
  const timeline: Timeline = { sequence: ++sequence, started, outcome: "running", jobId: null, marks: [], dropped: 0, passes: new Map(), failure: null, result: null, resultChars: null };
  current = timeline;
  pushMark(timeline, "solve_requested", started, detail);
  return {
    mark: (stage, markDetail = null) => { if (current === timeline) markSolveStage(stage, markDetail); },
    jobStarted: (jobId) => { if (current === timeline) { timeline.jobId = jobId; markSolveStage("job_started"); } },
    end: (outcome, error) => { if (current === timeline) endSolveTimeline(outcome, error); }
  };
}

function pushMark(timeline: Timeline, stage: SolveStage, at: number, detail: Detail | null): void {
  if (timeline.marks.length >= MAX_MARKS) { timeline.dropped += 1; return; }
  timeline.marks.push({ stage, at_ms: at, since_start_ms: at - timeline.started, detail: cleanDetail(detail) });
}

/** Marks a stage of the latest solve; nothing without one. */
export function markSolveStage(stage: SolveStage, detail: Detail | null = null): void {
  if (!current) return;
  pushMark(current, stage, now(), detail);
}

/** The result read from the received bytes (by reference only), and its text length,
 * so full-envelope passes and the commit of this result can be recognised. */
export function noteSolveResult(result: object, chars: number): void {
  if (!current || typeof WeakRef === "undefined") return;
  current.result = new WeakRef(result);
  current.resultChars = chars;
}

/** Whether `jobId` is the running latest solve's job (a rule check's own solve is not). */
export function isSolveJob(jobId: string): boolean {
  return current !== null && current.outcome === "running" && current.jobId === jobId;
}

/** Whether `value` is the latest solve's received result. */
export function isSolveResult(value: unknown): boolean {
  return Boolean(current?.result && value !== null && typeof value === "object" && current.result.deref() === value);
}

/** Ends the latest solve's timeline as `outcome` (it stays readable and keeps counting
 * passes until the next solve). A failure records the last stage reached. */
function endSolveTimeline(outcome: Exclude<SolveOutcome, "running">, error?: unknown): void {
  if (!current || current.outcome !== "running") return;
  current.outcome = outcome;
  if (outcome === "failed" && error !== undefined && !current.failure) {
    const stage = current.marks.at(-1)?.stage ?? "solve_requested";
    current.failure = { stage, code: errorCode(error), error_name: errorName(error), at_ms: now() };
  }
}

/** Records one timed pass that started at `startedAt` (`performance.now()`) over a
 * text of `chars` characters. Cheap: one clock read and one map update. */
export function recordSolvePass(kind: SolvePassKind, startedAt: number, chars: number): void {
  const timeline = current;
  if (!timeline) return;
  const elapsed = now() - startedAt;
  const afterStage = timeline.marks.at(-1)?.stage ?? "solve_requested";
  const key = `${kind}\u0000${afterStage}`;
  let totals = timeline.passes.get(key);
  if (!totals) {
    totals = { count: 0, total_ms: 0, max_ms: 0, full_count: 0, full_ms: 0, max_chars: 0 };
    timeline.passes.set(key, totals);
  }
  totals.count += 1;
  totals.total_ms += elapsed;
  totals.max_ms = Math.max(totals.max_ms, elapsed);
  totals.max_chars = Math.max(totals.max_chars, chars);
  const full = timeline.resultChars === null ? FULL_FLOOR_CHARS : Math.max(FULL_FLOOR_CHARS, timeline.resultChars / 2);
  if (chars >= full) { totals.full_count += 1; totals.full_ms += elapsed; }
}

/** `run()` timed as one pass of `kind`; `chars` measures its text from its value. */
export function timedSolvePass<T>(kind: SolvePassKind, run: () => T, chars: (value: T) => number): T {
  if (!current) return run();
  const startedAt = now();
  const value = run();
  recordSolvePass(kind, startedAt, chars(value));
  return value;
}

/** Marks the first paint after the latest solve's result was committed: two animation
 * frames after the commit, the frame that shows it has been produced. */
export function markSolveCommittedAndPaint(result: object | null): void {
  if (!current || !result || !isSolveResult(result)) return;
  const timeline = current;
  if (timeline.marks.some((mark) => mark.stage === "result_committed")) return;
  markSolveStage("result_committed");
  if (typeof requestAnimationFrame !== "function") return;
  requestAnimationFrame(() => requestAnimationFrame(() => {
    if (current === timeline) markSolveStage("first_paint");
  }));
}

/** A fresh copy of the latest solve's timeline, or null before any solve. */
export function readSolveTimeline(): SolveTimelineSnapshot | null {
  const timeline = current;
  if (!timeline) return null;
  return {
    schema: SOLVE_TIMELINE_SCHEMA,
    sequence: timeline.sequence,
    time_origin_ms: performance.timeOrigin,
    outcome: timeline.outcome,
    marks: timeline.marks.map((mark) => ({ ...mark, detail: mark.detail ? { ...mark.detail } : null })),
    passes: [...timeline.passes].map(([key, totals]) => {
      const [kind, afterStage] = key.split("\u0000") as [SolvePassKind, SolveStage];
      return { kind, after_stage: afterStage, ...totals };
    }),
    dropped_marks: timeline.dropped,
    failure: timeline.failure ? { ...timeline.failure } : null
  };
}

// Objects and functions alike, through every own key (a function's own properties too), so
// nothing reachable from the global can be extended or changed.
function deepFreeze<T>(value: T): T {
  if (value && (typeof value === "object" || typeof value === "function") && !Object.isFrozen(value)) {
    Object.freeze(value);
    for (const key of Reflect.ownKeys(value)) {
      const descriptor = Object.getOwnPropertyDescriptor(value, key);
      if (descriptor && "value" in descriptor) deepFreeze(descriptor.value);
    }
  }
  return value;
}

export type SolveTimelineGlobal = Readonly<{
  schema: typeof SOLVE_TIMELINE_SCHEMA;
  /** A recursively frozen copy of the latest solve's timeline, or null before any solve. */
  readCurrent(): SolveTimelineSnapshot | null;
}>;

/** The value of the read-only global; installed by `uiDiagnostics.ts` (see the header). */
export const solveTimelineGlobal: SolveTimelineGlobal = deepFreeze({
  schema: SOLVE_TIMELINE_SCHEMA,
  readCurrent: () => deepFreeze(readSolveTimeline())
});

declare global {
  // RT-U4 (DEL-04-05): read-only timings and counts of the latest solve; installed with
  // __openPipeStressUiDiagnosticsV1 and under its condition.
  var __openPipeStressSolveTimelineV1: SolveTimelineGlobal | undefined;
}

/** Test support only. */
export function resetSolveTimelineForTests(): void {
  current = null;
  nativeReaderCalls = 0;
  delete globalThis.__SWBPIPE_NATIVE_SMOKE_READER_PHASE__;
}


/** C1's four native registration awaits, observed only by the smoke driver.
 * Fixed scalar events; no job/model/result identity or standing contents.
 * A missing exit remains missing after cancellation/disposal or process exit. */
export type NativeReaderPhase = "header" | "registration" | "standing" | "summary";
export type NativeReaderBoundary = Readonly<{ layer: "web"; call: number; phase: NativeReaderPhase; edge: "enter" | "exit"; status: "pending" | "ok" | "error"; mono_ms: number }>;
let nativeReaderCalls = 0;
declare global {
  var __SWBPIPE_NATIVE_SMOKE_READER_PHASE__: ((event: NativeReaderBoundary) => Promise<void>) | undefined;
}
export async function traceNativeReaderAwait<T>(phase: NativeReaderPhase, work: () => Promise<T>): Promise<T> {
  const reporter = globalThis.__SWBPIPE_NATIVE_SMOKE_READER_PHASE__;
  if (!reporter) return work();
  // A single native-smoke launch gets at most 32 observed awaits. Overflow
  // is a diagnostic failure, never a made-up successful/incomplete exit.
  const call = ++nativeReaderCalls;
  if (call > 32) throw new Error("C1-READER-DIAGNOSTIC-LIMIT");
  const emit = (edge: NativeReaderBoundary["edge"], status: NativeReaderBoundary["status"]) => reporter({ layer: "web", call, phase, edge, status, mono_ms: performance.now() });
  await emit("enter", "pending");
  let result: T;
  try { result = await work(); }
  catch (error) {
    await emit("exit", "error");
    throw error;
  }
  // A failed evidence write is not a failed product call, and must not
  // fabricate a second/conflicting exit for the same await.
  await emit("exit", "ok");
  return result;
}
