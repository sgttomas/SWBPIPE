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
export const SAME_TRACE_EXPORT_PROFILE = Object.freeze({
  clockDomain: "MAC_MACH_ABSOLUTE_TIME", chromiumRevision: "507c6ee3e2f3b2ca0e660547e5b9ea4820c67f4c",
  perfettoRevision: "da65f7e907e0caf473ddec16e15427465f503d05",
  exporterSha256: "dc3a3b53cc2df3cd66be8b331b805d31331c2a8dbcf42e3f58d6fe8a05377667",
  traceTimeSha256: "6a5fa3cf626a07016997c3bdaf32610577c45c9e0fccd60f174205793da7fe2b",
  exportRule: "positive-integer-nanoseconds-truncated-to-microseconds", differenceAllowanceUs: 1
} as const);
export type SameTraceDurationBasis = Readonly<{
  kind: "same-trace-integer-us/v1"; qualification: "PASS_CALLER_BOUND_SAME_TRACE";
  rawCaptureSha256: string; profile: typeof SAME_TRACE_EXPORT_PROFILE;
  documentFrame: string; documentTimeOrigin: number; documentEvidenceEpoch: number; actionListenerObservedAt: number; crossOriginIsolated: boolean; actionToken: string; actionTraceTimestamp: number;
  canvasEpoch: number; contextEpoch: number; modelGeneration: number;
  rendererProcessId: number; rendererMainThreadId: number; rendererCompositorThreadId: number; layerTreeId: string;
  references: readonly Readonly<{ rawCaptureSha256: string; markerIdentity: string; reportedTimestamp: number;
    reporterOccurrence: string; reporterBeginEventIndex: number; reporterEndEventIndex: number }>[];
}>;
export type OrbitEndpoint = Readonly<{ sourceIndex: number; reportedTimestamp: number; coordinateMs: number; intervalMs: DurationInterval }>;
export type OrbitPopulation = Readonly<{ left: number; right: number; gapCount: number }>;
export type OrbitEvidence = Readonly<{
  mode: "centerline" | "actual-od";
  qualification: "PASS_QUALIFIED_CAUSAL_EVIDENCE";
  coverageStatus: "PASS_COMPLETE_BRACKETED_PRESENTATIONS";
  labelsOn: true;
  warmup: Readonly<{ startMs: number; endMs: number }>;
  measured: Readonly<{ startMs: number; endMs: number }>;
  // Full caller-qualified source sequence plus exact conservative enclosing slice.
  durationBasis: SameTraceDurationBasis | Readonly<{ kind: "independent-page-interval/v1" }>;
  traceToPageOffsetMs: number;
  endpoints: readonly OrbitEndpoint[];
  envelope: Readonly<{ first: number; last: number }>;
  // Consecutive qualified presentation gaps, including both window brackets.
  gaps: readonly (QualifiedDuration & Readonly<{ sampleId: number; fromMs: number; toMs: number; fromSourceIndex: number; toSourceIndex: number }>)[];
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

const directed = (value: number, up: boolean) => {
  if (value === 0) return up ? Number.MIN_VALUE : -Number.MIN_VALUE;
  const view = new DataView(new ArrayBuffer(8)); view.setFloat64(0, value);
  view.setBigUint64(0, view.getBigUint64(0) + ((value > 0) === up ? 1n : -1n)); return view.getFloat64(0);
};
// Independent scorer enclosure, deliberately not imported from controller arithmetic.
const sameTracePairBounds = (a: number, b: number) => {
  if (![a,b].every(t=>Number.isSafeInteger(t)&&t>=0) || b<a) throw new Error("invalid same-trace integer pair");
  const delta=BigInt(b)-BigInt(a);
  return { lower: Math.max(0,directed(Number(delta-1n)/1000,false)), upper: directed(Number(delta+1n)/1000,true) };
};
export function validSameTraceBasis(b: SameTraceDurationBasis | undefined, count: number): boolean {
  return b?.kind === "same-trace-integer-us/v1" && b.qualification === "PASS_CALLER_BOUND_SAME_TRACE" &&
    /^[a-f0-9]{64}$/.test(b.rawCaptureSha256) && Object.entries(SAME_TRACE_EXPORT_PROFILE).every(([k,v]) => (b.profile as any)?.[k]===v) &&
    Number.isSafeInteger(b.documentEvidenceEpoch) && b.documentEvidenceEpoch>=0 && finite(b.actionListenerObservedAt) && b.actionListenerObservedAt>=0 && typeof b.crossOriginIsolated==="boolean" &&
    text(b.documentFrame) && finite(b.documentTimeOrigin) && b.documentTimeOrigin>0 && text(b.actionToken) &&
    Number.isSafeInteger(b.actionTraceTimestamp) && b.actionTraceTimestamp>=0 && text(b.layerTreeId) &&
    [b.canvasEpoch,b.contextEpoch,b.modelGeneration,b.rendererProcessId,b.rendererMainThreadId,b.rendererCompositorThreadId].every(n=>Number.isSafeInteger(n)&&n>0) &&
    Array.isArray(b.references) && b.references.length===count && Array.from(b.references).every(r=>r && r.rawCaptureSha256===b.rawCaptureSha256 && text(r.markerIdentity) && text(r.reporterOccurrence) &&
      Number.isSafeInteger(r.reportedTimestamp) && r.reportedTimestamp>=0 && Number.isSafeInteger(r.reporterBeginEventIndex) && r.reporterBeginEventIndex>=0 &&
      Number.isSafeInteger(r.reporterEndEventIndex) && r.reporterEndEventIndex>=0 && r.reporterEndEventIndex!==r.reporterBeginEventIndex) &&
    new Set(b.references.map(r=>r.reporterOccurrence)).size===count;
}

// Necessary local cut conditions conservatively over-approximate shared-clock feasibility.
// Callers authenticate observations; this validates the complete carried source/slice linkage.
export function orbitBoundaryPopulations(e: OrbitEvidence): OrbitPopulation[] {
  const points = e?.endpoints, envelope = e?.envelope, m = e?.measured;
  if (!finite(e?.traceToPageOffsetMs) || !m || !finite(m.startMs) || !finite(m.endMs) || m.startMs >= m.endMs || !Array.isArray(points) || points.length < 3 ||
      Array.from(points).some((p, i) => !p || p.sourceIndex !== i || !finite(p.reportedTimestamp) || !finite(p.coordinateMs) ||
        p.coordinateMs !== p.reportedTimestamp / 1000 - e.traceToPageOffsetMs || !finite(p.intervalMs?.lower) || !finite(p.intervalMs?.upper) || p.intervalMs.lower > p.intervalMs.upper ||
        p.coordinateMs < p.intervalMs.lower || p.coordinateMs > p.intervalMs.upper ||
        (i > 0 && (p.reportedTimestamp < points[i - 1].reportedTimestamp || p.coordinateMs < points[i - 1].coordinateMs ||
          (p.reportedTimestamp === points[i - 1].reportedTimestamp) !== (p.coordinateMs === points[i - 1].coordinateMs))))) throw new Error("invalid orbit endpoint population");
  const sameTrace = e.durationBasis?.kind === "same-trace-integer-us/v1";
  if (sameTrace ? !validSameTraceBasis(e.durationBasis as SameTraceDurationBasis, points.length) : e.durationBasis?.kind !== "independent-page-interval/v1") throw new Error("invalid orbit duration basis");
  if (sameTrace && points.some((p,i)=>p.reportedTimestamp!==(e.durationBasis as SameTraceDurationBasis).references[i].reportedTimestamp)) throw new Error("orbit endpoint reference mismatch");
  const first = points.reduce((found: number, p: OrbitEndpoint, i: number) => p.intervalMs.upper < m.startMs ? i : found, -1);
  const last = points.findIndex(p => p.intervalMs.lower > m.endMs);
  if (first < 0 || last <= first + 1 || envelope?.first !== first || envelope?.last !== last ||
      !Array.isArray(e.gaps) || e.gaps.length !== last - first) throw new Error("incomplete orbit envelope");
  if (Array.from(e.gaps).some((g, i) => {
    const a = points[first + i], b = points[first + i + 1];
    const required = sameTrace ? sameTracePairBounds(a.reportedTimestamp,b.reportedTimestamp)
      : { lower: Math.max(0,b.intervalMs.lower-a.intervalMs.upper), upper:b.intervalMs.upper-a.intervalMs.lower };
    return !g || g.sampleId !== i + 1 || g.fromSourceIndex !== a.sourceIndex || g.toSourceIndex !== b.sourceIndex ||
      !qualified(g) || g.fromMs !== a.coordinateMs || g.toMs !== b.coordinateMs || g.fromMs < 0 || g.toMs < g.fromMs ||
      g.durationIntervalMs.lower > g.toMs - g.fromMs || g.durationIntervalMs.upper < g.toMs - g.fromMs ||
      g.durationIntervalMs.lower > required.lower || g.durationIntervalMs.upper < required.upper;
  })) throw new Error("invalid orbit gap association");
  const left: number[] = [], right: number[] = [];
  for (let i = first; i < last; i++) {
    const a = points[i], b = points[i + 1];
    if (a.reportedTimestamp >= b.reportedTimestamp) continue; // Never split an exact timestamp group.
    if (a.intervalMs.lower < m.startMs && b.intervalMs.upper >= m.startMs) left.push(i);
    if (b.intervalMs.upper > m.endMs && a.intervalMs.lower <= m.endMs) right.push(i + 1);
  }
  const populations = left.flatMap(a => right.filter(b => a < b).map(b => ({ left: a, right: b, gapCount: b - a })));
  if (!populations.length) throw new Error("no admissible orbit boundary population");
  return populations;
}

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
  const orbitPopulations: Partial<Record<OrbitEvidence["mode"], (OrbitPopulation & { p95UpperMs: number })[]>> = {};
  const orbit = (e: OrbitEvidence | undefined, mode: OrbitEvidence["mode"]): number | null => {
    const w = e?.warmup, m = e?.measured, gaps = e?.gaps;
    const windowValid = w && m && [w.startMs, w.endMs, m.startMs, m.endMs].every(finite) &&
      w.startMs >= 0 && w.endMs - w.startMs === 2000 && m.endMs - m.startMs === 10000 && w.endMs === m.startMs;
    if (e?.mode !== mode || e?.qualification !== "PASS_QUALIFIED_CAUSAL_EVIDENCE" ||
        e?.coverageStatus !== "PASS_COMPLETE_BRACKETED_PRESENTATIONS" || e?.labelsOn !== true || !windowValid ||
        (mode === "actual-od" && (e.odConversion?.status !== "PASS_CURRENT_CONVERSION" ||
          (e.odConversion?.cache !== "cold" && e.odConversion?.cache !== "warm"))) ||
        !Array.isArray(gaps)) {
      validityFailures.push(`INCOMPLETE_OR_INVALID_ORBIT:${mode}`); return null;
    }
    try {
      const populations = orbitBoundaryPopulations(e!);
      const candidates = populations.map(cut => ({ ...cut, p95UpperMs: p95(gaps.slice(cut.left - e!.envelope.first,
        cut.right - e!.envelope.first).map(g => g.durationIntervalMs.upper)) }));
      orbitPopulations[mode] = candidates;
      return Math.max(...candidates.map(c => c.p95UpperMs));
    } catch {
      validityFailures.push(`INCOMPLETE_OR_INVALID_ORBIT:${mode}`); return null;
    }
  };
  scores.centerlineP95 = orbit(run?.centerline, "centerline");
  scores.actualOdP95 = orbit(run?.actualOd, "actual-od");
  if (run?.settled?.status !== "PASS_OBSERVED_SETTLED" || !Number.isSafeInteger(run.settled?.ownedRafCount) ||
      run.settled.ownedRafCount !== 0) validityFailures.push("SETTLED_OWNED_RAF_NOT_EXACT_ZERO");
  for (const key of Object.keys(scores) as (keyof typeof scores)[]) {
    if (scores[key] !== null && scores[key]! > PERFORMANCE_TARGETS_MS[key]) targetFailures.push(`TARGET_EXCEEDED:${key}`);
  }
  return { status: validityFailures.length ? "FAIL_INVALID_EVIDENCE" : targetFailures.length ? "FAIL_TARGETS" : "PASS_METRIC_ACCEPTANCE",
    validityFailures, targetFailures, scores, orbitPopulations, runId: run?.runId, fixtureSize: run?.fixtureSize,
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
