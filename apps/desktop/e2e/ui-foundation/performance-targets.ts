// Strict metric acceptance only. The driver must establish the carried bindings
// and qualification from canonical evidence; this module cannot authenticate them.
export type FixtureSize = 1000 | 10000;
export type DurationInterval = Readonly<{ lower: number; upper: number }>;
export type QualifiedDuration = Readonly<{
  qualification: "PASS_QUALIFIED_CAUSAL_EVIDENCE";
  durationIntervalMs: DurationInterval;
}>;
export type ActionDuration = QualifiedDuration & Readonly<{ sampleId: number }>;
export type PerformanceBindings = Readonly<{
  validationStatus: "PASS_CALLER_VALIDATED_BINDINGS";
  sourceSha256: string; buildSha256: string; fixtureSha256: string;
  browserSha256: string; oracleSha256: string; methodSha256: string; samplesSha256: string;
}>;
export type RunExpectation = Readonly<{
  fixtureSize: FixtureSize; runNumber: number; runId: string; sessionId: string;
  bindings: PerformanceBindings;
}>;
export type OrbitEvidence = Readonly<{
  mode: "centerline" | "actual-od";
  qualification: "PASS_QUALIFIED_CAUSAL_EVIDENCE";
  coverageStatus: "PASS_COMPLETE_BRACKETED_PRESENTATIONS";
  labelsOn: true;
  warmup: Readonly<{ startMs: number; endMs: number }>;
  measured: Readonly<{ startMs: number; endMs: number }>;
  // Consecutive qualified presentation gaps, including both window brackets.
  gaps: readonly (QualifiedDuration & Readonly<{ sampleId: number; fromMs: number; toMs: number }>)[];
  odConversion?: Readonly<{ status: "PASS_CURRENT_CONVERSION"; cache: "cold" | "warm" }>;
}>;
export type PerformanceRun = RunExpectation & Readonly<{
  freshSession: true;
  qualification: "PASS_QUALIFIED_RUN";
  assignment: QualifiedDuration;
  points: readonly ActionDuration[]; boxes: readonly ActionDuration[]; filters: readonly ActionDuration[];
  centerline: OrbitEvidence; actualOd: OrbitEvidence;
  settled: Readonly<{ status: "PASS_OBSERVED_SETTLED"; ownedRafCount: number }>;
  // Raw references are disclosed, never subtracted or used as target gates.
  observerDiagnostics: Readonly<{ evidenceRef: string; status: string }>;
}>;
export const PERFORMANCE_TARGETS_MS = Object.freeze({ assignment: 2000, pointP95: 100,
  boxP95: 200, filterP95: 200, centerlineP95: 16.7, actualOdP95: 33.3 });
export const RAW_OBSERVER_REFERENCES = Object.freeze({ p95Ms: 0.5, maximumMs: 2, totalPercent: 3 });
const bindingKeys = ["sourceSha256", "buildSha256", "fixtureSha256", "browserSha256", "oracleSha256",
  "methodSha256", "samplesSha256"] as const;
const sharedKeys = ["sourceSha256", "buildSha256", "browserSha256", "methodSha256"] as const;
const finite = (n: unknown): n is number => typeof n === "number" && Number.isFinite(n);
const text = (s: unknown): s is string => typeof s === "string" && s.trim().length > 0;
const validBindings = (b: PerformanceBindings | undefined) => b?.validationStatus === "PASS_CALLER_VALIDATED_BINDINGS" &&
  bindingKeys.every((key) => typeof b[key] === "string" && /^[a-f0-9]{64}$/.test(b[key]));
const qualified = (v: QualifiedDuration | undefined) => v?.qualification === "PASS_QUALIFIED_CAUSAL_EVIDENCE" &&
  finite(v.durationIntervalMs?.lower) && finite(v.durationIntervalMs?.upper) &&
  v.durationIntervalMs.lower >= 0 && v.durationIntervalMs.upper >= v.durationIntervalMs.lower;
const p95 = (values: readonly number[]) => [...values].sort((a, b) => a - b)[Math.ceil(values.length * 0.95) - 1];
const identityValid = (e: RunExpectation | undefined) => (e?.fixtureSize === 1000 || e?.fixtureSize === 10000) &&
  Number.isSafeInteger(e.runNumber) && e.runNumber >= 1 && e.runNumber <= 5 && text(e.runId) && text(e.sessionId);

export function scorePerformanceRun(run: PerformanceRun, expected: RunExpectation) {
  const validityFailures: string[] = [];
  const targetFailures: string[] = [];
  const scores: Record<keyof typeof PERFORMANCE_TARGETS_MS, number | null> = {
    assignment: null, pointP95: null, boxP95: null, filterP95: null, centerlineP95: null, actualOdP95: null
  };
  if (!identityValid(expected) || !identityValid(run) ||
      ["fixtureSize", "runNumber", "runId", "sessionId"].some((key) =>
        run?.[key as keyof RunExpectation] !== expected?.[key as keyof RunExpectation])) validityFailures.push("RUN_IDENTITY_MISMATCH");
  if (!validBindings(expected?.bindings) || !validBindings(run?.bindings) ||
      bindingKeys.some((key) => run?.bindings?.[key] !== expected?.bindings?.[key])) validityFailures.push("INVALID_OR_MISMATCHED_BINDINGS");
  if (run?.freshSession !== true || run?.qualification !== "PASS_QUALIFIED_RUN") validityFailures.push("RUN_NOT_FRESH_AND_QUALIFIED");
  if (!text(run?.observerDiagnostics?.evidenceRef) || !text(run?.observerDiagnostics?.status)) validityFailures.push("MISSING_OBSERVER_DIAGNOSTICS");
  if (qualified(run?.assignment)) scores.assignment = run.assignment.durationIntervalMs.upper;
  else validityFailures.push("INVALID_ASSIGNMENT_EVIDENCE");
  const actions = (rows: readonly ActionDuration[] | undefined, count: number, label: string): number | null => {
    if (!Array.isArray(rows) || rows.length !== count || Array.from(rows).some((row, index) =>
      row?.sampleId !== index + 1 || !qualified(row))) {
      validityFailures.push(`INCOMPLETE_OR_INVALID_${label}`); return null;
    }
    return p95(rows.map((row) => row.durationIntervalMs.upper));
  };
  scores.pointP95 = actions(run?.points, 200, "POINTS");
  scores.boxP95 = actions(run?.boxes, 20, "BOXES");
  scores.filterP95 = actions(run?.filters, 20, "FILTERS");
  const orbit = (e: OrbitEvidence | undefined, mode: OrbitEvidence["mode"]): number | null => {
    const w = e?.warmup, m = e?.measured, gaps = e?.gaps;
    const windowValid = w && m && [w.startMs, w.endMs, m.startMs, m.endMs].every(finite) &&
      w.startMs >= 0 && w.endMs - w.startMs === 2000 && m.endMs - m.startMs === 10000 && w.endMs === m.startMs;
    if (e?.mode !== mode || e?.qualification !== "PASS_QUALIFIED_CAUSAL_EVIDENCE" ||
        e?.coverageStatus !== "PASS_COMPLETE_BRACKETED_PRESENTATIONS" || e?.labelsOn !== true || !windowValid ||
        (mode === "actual-od" && (e.odConversion?.status !== "PASS_CURRENT_CONVERSION" ||
          (e.odConversion?.cache !== "cold" && e.odConversion?.cache !== "warm"))) ||
        !Array.isArray(gaps) || gaps.length < 2 || Array.from(gaps).some((gap, index) =>
          gap?.sampleId !== index + 1 || !qualified(gap) || !finite(gap.fromMs) || !finite(gap.toMs) ||
          // Distinct caller-qualified occurrences may share a reported timestamp; retain every uncertainty interval.
          gap.fromMs < 0 || gap.toMs < gap.fromMs ||
          gap.durationIntervalMs.lower > gap.toMs - gap.fromMs || gap.durationIntervalMs.upper < gap.toMs - gap.fromMs ||
          (index > 0 && gaps[index - 1]?.toMs !== gap.fromMs)) ||
        gaps[0].fromMs >= m!.startMs || gaps[0].toMs < m!.startMs ||
        gaps.at(-1)!.fromMs > m!.endMs || gaps.at(-1)!.toMs <= m!.endMs ||
        gaps.slice(1, -1).some((gap) => gap.fromMs < m!.startMs || gap.toMs > m!.endMs)) {
      validityFailures.push(`INCOMPLETE_OR_INVALID_ORBIT:${mode}`); return null;
    }
    return p95(gaps.map((gap) => gap.durationIntervalMs.upper));
  };
  scores.centerlineP95 = orbit(run?.centerline, "centerline");
  scores.actualOdP95 = orbit(run?.actualOd, "actual-od");
  if (run?.settled?.status !== "PASS_OBSERVED_SETTLED" || !Number.isSafeInteger(run.settled?.ownedRafCount) ||
      run.settled.ownedRafCount !== 0) validityFailures.push("SETTLED_OWNED_RAF_NOT_EXACT_ZERO");
  for (const key of Object.keys(scores) as (keyof typeof scores)[]) {
    if (scores[key] !== null && scores[key]! > PERFORMANCE_TARGETS_MS[key]) targetFailures.push(`TARGET_EXCEEDED:${key}`);
  }
  return { status: validityFailures.length ? "FAIL_INVALID_EVIDENCE" : targetFailures.length ? "FAIL_TARGETS" : "PASS_METRIC_ACCEPTANCE",
    validityFailures, targetFailures, scores, runId: run?.runId, fixtureSize: run?.fixtureSize,
    bindings: run?.bindings, observerDiagnostics: run?.observerDiagnostics, rawObserverReferences: RAW_OBSERVER_REFERENCES,
    scope: "Metric acceptance for supplied qualified instrumented evidence only; no resource/native/product/project closure." } as const;
}

export function scorePerformanceCohort(runs: readonly PerformanceRun[], expected: readonly RunExpectation[]) {
  const validityFailures: string[] = [];
  const expectations = Array.isArray(expected) ? Array.from(expected) : [];
  const population = Array.isArray(runs) ? Array.from(runs) : [];
  if (expectations.length !== 10 || population.length !== 10) validityFailures.push("COHORT_REQUIRES_TEN_RUNS");
  for (const [label, rows] of [["EXPECTED", expectations], ["ACTUAL", population]] as const) {
    if (rows.some((row) => !identityValid(row) || !validBindings(row?.bindings)) ||
        new Set(rows.map((row) => row?.runId)).size !== rows.length ||
        new Set(rows.map((row) => row?.sessionId)).size !== rows.length ||
        [1000, 10000].some((size) => [1, 2, 3, 4, 5].some((number) =>
          rows.filter((row) => row?.fixtureSize === size && row?.runNumber === number).length !== 1))) {
      validityFailures.push(`INVALID_OR_REPEATED_${label}_RUNS`);
    }
    if (rows.some((row) => sharedKeys.some((key) => row?.bindings?.[key] !== rows[0]?.bindings?.[key]))) {
      validityFailures.push(`MIXED_${label}_CANDIDATE_METHOD_BROWSER_BINDINGS`);
    }
    for (const size of [1000, 10000]) {
      const group = rows.filter((row) => row?.fixtureSize === size);
      if (group.some((row) => bindingKeys.some((key) => row.bindings?.[key] !== group[0]?.bindings?.[key]))) {
        validityFailures.push(`MIXED_${label}_FIXTURE_BINDINGS:${size}`);
      }
    }
  }
  const outcomes = population.map((run) => {
    const matches = expectations.filter((e) => e?.runId === run?.runId);
    if (matches.length !== 1) validityFailures.push(`UNEXPECTED_OR_AMBIGUOUS_RUN:${run?.runId}`);
    return scorePerformanceRun(run, matches[0]);
  });
  return { status: validityFailures.length || outcomes.some((o) => o.status !== "PASS_METRIC_ACCEPTANCE")
    ? "FAIL_COHORT" : "PASS_COHORT_METRICS", validityFailures, outcomes,
    scope: "Five fresh runs per fixture; every outcome retained, no size/mode pooling. Resource/native acceptance remains separate." } as const;
}
