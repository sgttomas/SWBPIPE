/** RT-U4 (DEL-04-05, release target exit criterion 9): the latest solve's timeline,
 * from Analyze → Run to the first paint of its result. Observation only: nothing reads
 * it to decide anything, and it changes no behaviour.
 *
 * - Marks: each stage of the latest solve once, at `performance.now()` (a monotonic
 *   clock), with small details (job id, byte counts, outcomes). `solve_requested`
 *   starts a new timeline and drops the previous one.
 * - Passes: every timed serialization, fingerprint or wasm reader call, aggregated by
 *   kind and by the last stage reached before it ran, so passes after the paint
 *   (Record Run, Save Project) are counted too. A pass whose text is at least half
 *   the received result's (or 1 MiB before it is known) counts as full-envelope.
 *   Nested passes are counted under each kind: a `native_fingerprint` includes one
 *   `checked_json_text`.
 * - Failure: the first error the solve reports, with the last stage reached.
 *
 * Kept small: a bounded mark list, one aggregate per (kind, stage), and a WeakRef to
 * the received result (never a copy of it). `readSolveTimeline` returns a fresh copy;
 * `uiDiagnostics.readCurrent()` exposes it as `solveTimeline`. */

export const SOLVE_TIMELINE_SCHEMA = "swbpipe.solve-timeline/v1" as const;

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
  job_id: string | null;
  marks: readonly SolveTimelineMark[];
  passes: readonly SolveTimelinePass[];
  dropped_marks: number;
  failure: Readonly<{ stage: SolveStage; message: string; at_ms: number }> | null;
}>;

const MAX_MARKS = 64;
const MAX_FAILURE_CHARS = 2000;
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
  failure: { stage: SolveStage; message: string; at_ms: number } | null;
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
    end: (outcome, error) => { if (current === timeline) endSolveTimeline(outcome, error); }
  };
}

function pushMark(timeline: Timeline, stage: SolveStage, at: number, detail: Detail | null): void {
  if (timeline.marks.length >= MAX_MARKS) { timeline.dropped += 1; return; }
  timeline.marks.push({ stage, at_ms: at, since_start_ms: at - timeline.started, detail });
}

/** Marks a stage of the latest solve; nothing without one. */
export function markSolveStage(stage: SolveStage, detail: Detail | null = null): void {
  if (!current) return;
  if (stage === "job_started" && typeof detail?.job_id === "string") current.jobId = detail.job_id;
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
    const message = String((error as { message?: unknown })?.message ?? error).slice(0, MAX_FAILURE_CHARS);
    const stage = current.marks.at(-1)?.stage ?? "solve_requested";
    current.failure = { stage, message, at_ms: now() };
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
    job_id: timeline.jobId,
    marks: timeline.marks.map((mark) => ({ ...mark, detail: mark.detail ? { ...mark.detail } : null })),
    passes: [...timeline.passes].map(([key, totals]) => {
      const [kind, afterStage] = key.split("\u0000") as [SolvePassKind, SolveStage];
      return { kind, after_stage: afterStage, ...totals };
    }),
    dropped_marks: timeline.dropped,
    failure: timeline.failure ? { ...timeline.failure } : null
  };
}

/** Test support only. */
export function resetSolveTimelineForTests(): void {
  current = null;
}
