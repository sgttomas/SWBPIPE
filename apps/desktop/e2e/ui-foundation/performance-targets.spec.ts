import { expect, test } from "@playwright/test";
import { scorePerformanceRun, scorePerformanceCohort, orbitBoundaryPopulations, SAME_TRACE_EXPORT_PROFILE, type PerformanceRun, type RunExpectation,
  type FixtureSize, type OrbitEvidence, type PerformanceBindings } from "./performance-targets";

import { constructOrbitEvidence, conservativePresentedGap } from "./full-cohort-controller";

const bindings = (size: FixtureSize): PerformanceBindings => ({ validationStatus: "PASS_CALLER_VALIDATED_BINDINGS",
  sourceSha256: "a".repeat(64), buildSha256: "b".repeat(64), fixtureSha256: (size === 1000 ? "c" : "d").repeat(64),
  browserSha256: "e".repeat(64), oracleSha256: "f".repeat(64), methodSha256: "1".repeat(64), samplesSha256: "2".repeat(64) });
const expectation = (size: FixtureSize = 1000, number = 1): RunExpectation => ({ fixtureSize: size,
  runNumber: number, runId: `run-${size}-${number}`, sessionId: `fresh-${size}-${number}`, bindings: bindings(size) });
const duration = (upper: number, lower = 0) => ({ qualification: "PASS_QUALIFIED_CAUSAL_EVIDENCE" as const,
  durationIntervalMs: { lower, upper } });
const actions = (count: number, upper: number) => Array.from({ length: count }, (_, i) => ({ sampleId: i + 1, ...duration(upper) }));
function withEndpoints(e: any): OrbitEvidence {
  // Synthetic reported coordinates use the same binary64 microsecond-to-ms mapping as the caller.
  const times = [e.gaps[0].fromMs, ...e.gaps.map((g: any) => g.toMs)].map((t: number) => (t * 1000) / 1000);
  return { ...e, durationBasis: { kind: "independent-page-interval/v1" }, traceToPageOffsetMs: 0, endpoints: times.map((coordinateMs: number, sourceIndex: number) => ({ sourceIndex,
    coordinateMs, reportedTimestamp: coordinateMs * 1000, intervalMs: { lower: coordinateMs, upper: coordinateMs } })),
    envelope: { first: 0, last: times.length - 1 },
    gaps: e.gaps.map((g: any, i: number) => ({ ...g, fromMs: times[i], toMs: times[i + 1], fromSourceIndex: i, toSourceIndex: i + 1 })) };
}
const orbit = (mode: OrbitEvidence["mode"], upper: number): OrbitEvidence => withEndpoints({ mode,
  qualification: "PASS_QUALIFIED_CAUSAL_EVIDENCE", coverageStatus: "PASS_COMPLETE_BRACKETED_PRESENTATIONS", labelsOn: true,
  warmup: { startMs: 0, endMs: 2000 }, measured: { startMs: 2000, endMs: 12000 },
  gaps: Array.from({ length: 1001 }, (_, i) => ({ sampleId: i + 1, fromMs: 1990 + i * 10, toMs: 2000 + i * 10,
    ...duration(upper) })), odConversion: { status: "PASS_CURRENT_CONVERSION", cache: "warm" } });
// First endpoint is strictly before the measured start; last strictly after.
const validOrbit = (mode: OrbitEvidence["mode"], upper: number): OrbitEvidence => {
  const e = orbit(mode, upper);
  return withEndpoints({ ...e, gaps: e.gaps.map((gap) => ({ ...gap, fromMs: gap.fromMs + 1, toMs: gap.toMs + 1 })) });
};
const fixture = (expected = expectation()): PerformanceRun => ({ ...expected, freshSession: true,
  qualification: "PASS_QUALIFIED_RUN", assignment: duration(2000), points: actions(200, 100), boxes: actions(20, 200),
  filters: actions(20, 200), centerline: validOrbit("centerline", 16.7), actualOd: validOrbit("actual-od", 33.3),
  settled: { status: "PASS_OBSERVED_SETTLED", ownedRafCount: 0 },
  observerDiagnostics: { evidenceRef: "run-owned/observer.json", status: "EXCEEDS_RAW_REFERENCES_TRUE_COST_UNPROVED" } });
const score = (run: PerformanceRun) => scorePerformanceRun(run, expectation());

test("all exact owner boundaries pass despite raw observer reference exceedance", () => {
  const result = score(fixture());
  expect(result.status).toBe("PASS_METRIC_ACCEPTANCE");
  expect(result.scores).toEqual({ assignment: 2000, pointP95: 100, boxP95: 200, filterP95: 200, centerlineP95: 16.7, actualOdP95: 33.3 });
  expect(result.rawObserverReferences).toEqual({ p95Ms: 0.5, maximumMs: 2, totalPercent: 3 });
});

test("each target rejects tiny excess and scores upper rather than lower endpoint", () => {
  for (const key of ["assignment", "points", "boxes", "filters", "centerline", "actualOd"] as const) {
    const run = fixture();
    const changed = key === "assignment" ? { ...run, assignment: duration(2000.0000001, 1999) }
      : key === "points" ? { ...run, points: actions(200, 100.0000001) }
      : key === "boxes" ? { ...run, boxes: actions(20, 200.0000001) }
      : key === "filters" ? { ...run, filters: actions(20, 200.0000001) }
      : key === "centerline" ? { ...run, centerline: validOrbit("centerline", 16.7000001) }
      : { ...run, actualOd: validOrbit("actual-od", 33.3000001) };
    const result = score(changed);
    expect(result.validityFailures).toEqual([]);
    expect(result.status).toBe("FAIL_TARGETS");
    expect(result.targetFailures).toHaveLength(1);
  }
});

test("nearest rank uses the 190th of 200 and retains all tail samples", () => {
  const rows = actions(200, 100).map((row, index) => index >= 190 ? { ...row, ...duration(500) } : row);
  expect(score({ ...fixture(), points: rows }).scores.pointP95).toBe(100);
  const eleven = rows.map((row, index) => index === 189 ? { ...row, ...duration(500) } : row);
  expect(score({ ...fixture(), points: eleven }).scores.pointP95).toBe(500);
  expect(score({ ...fixture(), points: eleven }).status).toBe("FAIL_TARGETS");
});

test("all twenty box and filter rows are required and nineteenth upper is scored", () => {
  for (const key of ["boxes", "filters"] as const) {
    const rows = actions(20, 200).map((row, index) => index === 19 ? { ...row, ...duration(900) } : row);
    expect(score({ ...fixture(), [key]: rows }).status).toBe("PASS_METRIC_ACCEPTANCE");
    expect(score({ ...fixture(), [key]: rows.map((row, index) => index === 18 ? { ...row, ...duration(900) } : row) }).status).toBe("FAIL_TARGETS");
    expect(score({ ...fixture(), [key]: rows.slice(0, 19) }).status).toBe("FAIL_INVALID_EVIDENCE");
  }
});

test("missing duplicate reordered sparse or unqualified action populations fail without survivor p95", () => {
  for (const key of ["points", "boxes", "filters"] as const) {
    const rows = fixture()[key];
    for (const bad of [undefined, rows.slice(1), [...rows, rows[0]], [rows[1], rows[0], ...rows.slice(2)],
      [rows[0], rows[0], ...rows.slice(2)], [undefined, ...rows.slice(1)],
      [{ ...rows[0], qualification: "FAIL" }, ...rows.slice(1)]]) {
      const result = score({ ...fixture(), [key]: bad } as any);
      expect(result.status).toBe("FAIL_INVALID_EVIDENCE");
      expect(result.scores[key === "points" ? "pointP95" : key === "boxes" ? "boxP95" : "filterP95"]).toBeNull();
    }
  }
});

test("invalid duration intervals and absent assignment never qualify", () => {
  for (const interval of [undefined, { lower: -1, upper: 100 }, { lower: 101, upper: 100 },
    { lower: 0, upper: Infinity }, { lower: NaN, upper: 100 }, { lower: 0, upper: "100" }]) {
    expect(score({ ...fixture(), assignment: { qualification: "PASS_QUALIFIED_CAUSAL_EVIDENCE", durationIntervalMs: interval } } as any).status).toBe("FAIL_INVALID_EVIDENCE");
    const run = fixture();
    expect(score({ ...run, points: [{ ...run.points[0], durationIntervalMs: interval }, ...run.points.slice(1)] } as any).status).toBe("FAIL_INVALID_EVIDENCE");
  }
  expect(score({ ...fixture(), assignment: undefined } as any).status).toBe("FAIL_INVALID_EVIDENCE");
});

test("settled owned RAF must be observed exact zero", () => {
  for (const settled of [undefined, { status: "PASS_OBSERVED_SETTLED" }, { status: "UNKNOWN", ownedRafCount: 0 },
    ...[1, -1, NaN, "0"].map((ownedRafCount) => ({ status: "PASS_OBSERVED_SETTLED", ownedRafCount }))]) {
    expect(score({ ...fixture(), settled } as any).validityFailures).toContain("SETTLED_OWNED_RAF_NOT_EXACT_ZERO");
  }
});

test("orbit requires exact windows complete contiguous brackets correct modes and current OD", () => {
  const e = fixture().centerline;
  for (const bad of [undefined, { ...e, mode: "actual-od" }, { ...e, labelsOn: false },
    { ...e, coverageStatus: "PARTIAL" }, { ...e, warmup: { startMs: 1, endMs: 2000 } },
    { ...e, measured: { startMs: 2000, endMs: 11999 } }, { ...e, gaps: e.gaps.slice(1) },
    { ...e, gaps: e.gaps.slice(0, -1) }, { ...e, gaps: [e.gaps[0], ...e.gaps.slice(2)] },
    { ...e, gaps: [e.gaps[0], e.gaps[0], ...e.gaps.slice(2)] },
    { ...e, gaps: e.gaps.map((gap) => ({ ...gap, durationIntervalMs: { lower: 0, upper: 9 } })) }]) {
    expect(score({ ...fixture(), centerline: bad } as any).status).toBe("FAIL_INVALID_EVIDENCE");
  }
  const run = fixture();
  expect(score({ ...run, centerline: run.actualOd, actualOd: run.centerline }).status).toBe("FAIL_INVALID_EVIDENCE");
  expect(score({ ...run, actualOd: { ...run.actualOd, odConversion: undefined } }).status).toBe("FAIL_INVALID_EVIDENCE");
});

test("run identity bindings freshness and qualification are mandatory", () => {
  for (const patch of [{ runId: "other" }, { fixtureSize: 10000 }, { runNumber: 2 }, { sessionId: "other" },
    { freshSession: false }, { qualification: "FAIL" }, { bindings: undefined }, { observerDiagnostics: undefined },
    { bindings: { ...bindings(1000), methodSha256: "3".repeat(64) } }]) {
    expect(score({ ...fixture(), ...patch } as any).status).toBe("FAIL_INVALID_EVIDENCE");
  }
});
const cohort = () => {
  const expected = ([1000, 10000] as const).flatMap((size) => [1, 2, 3, 4, 5].map((i) => expectation(size, i)));
  return { expected, runs: expected.map(fixture) };
};
test("exact ten-run cohort preserves each result without pooling sizes or modes", () => {
  const f = cohort();
  expect(scorePerformanceCohort(f.runs, f.expected).status).toBe("PASS_COHORT_METRICS");
  const runs = f.runs.map((run, index) => index === 9 ? { ...run, assignment: duration(2001) } : run);
  const result = scorePerformanceCohort(runs, f.expected);
  expect(result.status).toBe("FAIL_COHORT");
  expect(result.outcomes).toHaveLength(10);
  expect(result.outcomes[9].status).toBe("FAIL_TARGETS");
  expect(result.outcomes.filter((o) => o.status === "PASS_METRIC_ACCEPTANCE")).toHaveLength(9);
});
test("missing repeated mixed-binding or nonfresh cohort cannot pass", () => {
  const f = cohort();
  for (const runs of [f.runs.slice(1), [...f.runs, f.runs[0]], [f.runs[0], f.runs[0], ...f.runs.slice(2)],
    f.runs.map((r, i) => i === 9 ? { ...r, sessionId: f.runs[0].sessionId } : r),
    f.runs.map((r, i) => i === 9 ? { ...r, bindings: { ...r.bindings, browserSha256: "9".repeat(64) } } : r),
    f.runs.map((r, i) => i === 9 ? { ...r, freshSession: false } : r)]) {
    expect(scorePerformanceCohort(runs as any, f.expected).status).toBe("FAIL_COHORT");
  }
  expect(scorePerformanceCohort(f.runs, f.expected.slice(1)).status).toBe("FAIL_COHORT");
  const mixed = f.expected.map((r, i) => i === 9 ? { ...r, bindings: { ...r.bindings, methodSha256: "9".repeat(64) } } : r);
  expect(scorePerformanceCohort(mixed.map(fixture), mixed).status).toBe("FAIL_COHORT");
});

for (const mode of ["centerline", "actual-od"] as const) {
  test(`outside orbit brackets survive exact left right and both boundary alignment: ${mode}`, () => {
    for (const alignment of ["left", "right", "both"] as const) {
      const times = Array.from({ length: 1003 }, (_, i) => 1990 + i * 10);
      if (alignment === "left") times[times.length - 2] = 11999;
      if (alignment === "right") times[1] = 2001;
      const gaps = times.slice(1).map((toMs, i) => ({ sampleId: i + 1, fromMs: times[i], toMs,
        ...duration(toMs - times[i], toMs - times[i]) }));
      const key = mode === "centerline" ? "centerline" : "actualOd";
      const e = withEndpoints({ ...validOrbit(mode, 10), gaps });
      const result = score({ ...fixture(), [key]: e });
      expect(result.status).toBe("PASS_METRIC_ACCEPTANCE");
      expect(result.scores[mode === "centerline" ? "centerlineP95" : "actualOdP95"]).toBe(10);
      expect(gaps).toHaveLength(1002);
    }
  });
  test(`boundary endpoints cannot replace missing left right or both outside brackets: ${mode}`, () => {
    const times = Array.from({ length: 1003 }, (_, i) => 1990 + i * 10);
    for (const missing of ["left", "right", "both"] as const) {
      const retained = times.slice(missing === "right" ? 0 : 1, missing === "left" ? undefined : -1);
      const gaps = retained.slice(1).map((toMs, i) => ({ sampleId: i + 1, fromMs: retained[i], toMs, ...duration(10, 10) }));
      const key = mode === "centerline" ? "centerline" : "actualOd";
      const result = score({ ...fixture(), [key]: { ...validOrbit(mode, 10), gaps } });
      expect(result.status).toBe("FAIL_INVALID_EVIDENCE");
      expect(result.scores[mode === "centerline" ? "centerlineP95" : "actualOdP95"]).toBeNull();
    }
  });
}

function tiedOrbit(mode: OrbitEvidence["mode"], origin: number, ties: number): OrbitEvidence {
  const start = origin + 2000, end = start + 10000, interior = 1199 - ties;
  const times = [start - 1, start, ...Array(ties).fill(start),
    ...Array.from({ length: interior }, (_, i) => start + 10000 * (i + 1) / interior), end + 1];
  return withEndpoints({ ...validOrbit(mode, 10), warmup: { startMs: origin, endMs: start }, measured: { startMs: start, endMs: end },
    gaps: times.slice(1).map((toMs, i) => ({ sampleId: i + 1, fromMs: times[i], toMs, ...duration(10, 0) })) });
}
for (const [mode, origin, ties, target] of [["centerline", 9635.5, 30, 16.7], ["actual-od", 42020.800000190735, 46, 33.3]] as const) {
  const key = mode === "centerline" ? "centerline" : "actualOd", metric = mode === "centerline" ? "centerlineP95" : "actualOdP95";
  test(`qualified tied timestamps preserve all1201 upper endpoints and nearest rank1141: ${mode}`, () => {
    const e = tiedOrbit(mode, origin, ties);
    expect(e.gaps).toHaveLength(1201);
    expect(e.gaps.filter(g => g.toMs === g.fromMs)).toHaveLength(ties);
    expect(e.warmup.endMs - e.warmup.startMs).toBe(2000);
    expect(e.measured.endMs - e.measured.startMs).toBe(10000);
    // Exactly61 upper endpoints exceed the low population; ties are among them.
    // Dropping tied rows would wrongly lower p95 to10; the 1141st of1201 is target.
    const high = new Set(e.gaps.flatMap((g, i) => g.toMs === g.fromMs ? [i] : []));
    for (let i = e.gaps.length - 1; high.size < 61; i--) high.add(i);
    const critical = [...high][0];
    const gaps = e.gaps.map((g, i) => ({ ...g, ...duration(high.has(i) ? i === critical ? target : target + 1 : 10) }));
    const evidence = { ...e, gaps }, before = structuredClone(evidence);
    const result = score({ ...fixture(), [key]: evidence });
    expect(result.status).toBe("PASS_METRIC_ACCEPTANCE");
    expect(result.scores[metric]).toBe(target);
    expect(evidence).toEqual(before);
    const exceeded = { ...evidence, gaps: gaps.map((g, i) => i === critical ? { ...g, ...duration(target + 0.0000001) } : g) };
    const failed = score({ ...fixture(), [key]: exceeded });
    expect(failed.validityFailures).toEqual([]);
    expect(failed.status).toBe("FAIL_TARGETS");
    expect(failed.scores[metric]).toBe(target + 0.0000001);
    expect(failed.targetFailures).toHaveLength(1);
  });
  test(`tied coordinates do not waive identity population interval or window guards: ${mode}`, () => {
    const e = tiedOrbit(mode, origin, ties), tie = e.gaps.findIndex(g => g.toMs === g.fromMs);
    const variants: ((value: any) => void)[] = [
      v => { v.gaps[tie].toMs = v.gaps[tie].fromMs - 1; },
      v => { v.gaps[tie].sampleId = v.gaps[tie - 1].sampleId; },
      v => { v.gaps.splice(tie, 1); },
      v => { [v.gaps[tie], v.gaps[tie + 1]] = [v.gaps[tie + 1], v.gaps[tie]]; },
      v => { v.gaps[tie].fromMs += .1; v.gaps[tie].toMs += .1; },
      v => { v.gaps.shift(); v.gaps.forEach((g: any, i: number) => g.sampleId = i + 1); },
      v => { v.gaps.pop(); },
      v => { v.gaps[tie].qualification = "FAIL"; },
      v => { v.gaps[tie].durationIntervalMs = { lower: 0, upper: Infinity }; },
      v => { v.gaps[tie].durationIntervalMs = { lower: .01, upper: 1 }; },
      v => { v.gaps[tie].durationIntervalMs = null; },
      v => { v.gaps[tie] = undefined; },
      v => { v.warmup.startMs += 1; },
      v => { v.measured.endMs -= 1; },
      v => { v.gaps.forEach((g: any) => { g.fromMs = v.measured.startMs; g.toMs = v.measured.startMs; }); }
    ];
    for (const mutate of variants) {
      const bad = structuredClone(e); mutate(bad);
      const result = score({ ...fixture(), [key]: bad });
      expect(result.status).toBe("FAIL_INVALID_EVIDENCE");
      expect(result.scores[metric]).toBeNull();
    }
  });
}

function constructed(times: number[], radius = .125, actionAt = 1000, mode: OrbitEvidence["mode"] = "centerline") {
  const endpoints=times.map((time,sourceIndex)=>({sourceIndex,reportedTimestamp:time*1000,coordinateMs:time,
    intervalMs:{lower:time-radius,upper:time+radius}}));
  const first=endpoints.reduce((found,p,i)=>p.intervalMs.upper<actionAt+2000?i:found,-1),last=endpoints.findIndex(p=>p.intervalMs.lower>actionAt+12000);
  return { ...validOrbit(mode,10),durationBasis:{kind:"independent-page-interval/v1"} as const,
    warmup:{startMs:actionAt,endMs:actionAt+2000},measured:{startMs:actionAt+2000,endMs:actionAt+12000},endpoints,envelope:{first,last},
    gaps:endpoints.slice(first+1,last+1).map((p,i)=>({sampleId:i+1,fromSourceIndex:first+i,toSourceIndex:first+i+1,
      fromMs:endpoints[first+i].coordinateMs,toMs:p.coordinateMs,qualification:"PASS_QUALIFIED_CAUSAL_EVIDENCE" as const,
      durationIntervalMs:conservativePresentedGap(endpoints[first+i].intervalMs,p.intervalMs)})) };
}
test("uncertain right boundary is qualified rather than nominally rejected", () => {
  const times = [2999, ...Array.from({ length: 1000 }, (_, i) => 3009 + i * 10), 13000.0625, 13009];
  const e = constructed(times);
  const result = score({ ...fixture(), centerline: e });
  expect(result.validityFailures).toEqual([]);
  expect(result.orbitPopulations.centerline?.map(c => c.gapCount)).toEqual([1001, 1002]);
});
test("independent dyadic1199 versus1200 dilution must fail using maximum population p95", () => {
  const times = [2999.21875];
  for (let i = 0; i < 1199; i++) times.push(times.at(-1)! + (i >= 500 && i < 560 ? 16.59375 : 7.90625));
  expect(times.at(-1)).toBe(13000.0625); times.push(times.at(-1)! + 7.90625);
  const e = constructed(times), before = structuredClone(e);
  const result = score({ ...fixture(), centerline: e });
  expect(result.validityFailures).toEqual([]); expect(result.status).toBe("FAIL_TARGETS");
  expect(result.targetFailures).toEqual(["TARGET_EXCEEDED:centerlineP95"]);
  expect(result.orbitPopulations.centerline?.map(c => c.gapCount)).toEqual([1199,1200]);
  expect(result.orbitPopulations.centerline![0].p95UpperMs).toBeGreaterThan(16.84375);
  expect(result.orbitPopulations.centerline![1].p95UpperMs).toBeLessThan(8.1562500001);
  expect(result.scores.centerlineP95).toBe(result.orbitPopulations.centerline![0].p95UpperMs);
  expect(result.scores.assignment).toBe(2000); expect(result.scores.pointP95).toBe(100);
  expect(e).toEqual(before);
});
test("all left right both-edge cuts retain ties and no omitted candidates", () => {
  for (const [times, expected] of [
    [[2999,3000.0625,5000,9000,13001], [[0,4],[1,4]]],
    [[2999,5000,9000,13000.0625,13001], [[0,3],[0,4]]],
    [[2999,3000.0625,5000,9000,13000.0625,13001], [[0,4],[0,5],[1,4],[1,5]]],
    [[2999,3000.0625,3000.0625,3000.125,5000,9000,13000,13000,13000.0625,13001], [[0,6],[0,8],[0,9],[2,6],[2,8],[2,9],[3,6],[3,8],[3,9]]]
  ] as [number[], number[][]][]) {
    const e = constructed(times, .25);
    expect(orbitBoundaryPopulations(e).map(c => [c.left,c.right])).toEqual(expected);
    const cuts = orbitBoundaryPopulations(e), upper = (cut: typeof cuts[number]) => {
      const values=e.gaps.slice(cut.left,cut.right).map(g=>g.durationIntervalMs.upper).sort((a,b)=>a-b);
      return values[Math.ceil(.95*values.length)-1];
    };
    expect(Math.max(...cuts.map(upper))).toBe(Math.max(...[...cuts].reverse().map(upper)));
    expect(Math.max(...cuts.map(upper))).toBeGreaterThanOrEqual(upper(cuts[0]));
  }
});
test("endpoint and envelope defects never become a favorable boundary population", () => {
  const e=constructed([2999,3000.0625,5000,9000,13000.0625,13001]);
  for(const mutate of [
    (v:any)=>{delete v.endpoints;}, (v:any)=>{delete v.endpoints[2].intervalMs;},
    (v:any)=>{delete v.endpoints[2].sourceIndex;}, (v:any)=>{v.endpoints[2].reportedTimestamp=NaN;},
    (v:any)=>{[v.endpoints[1],v.endpoints[2]]=[v.endpoints[2],v.endpoints[1]];}, (v:any)=>{v.traceToPageOffsetMs+=1;},
    (v:any)=>{v.endpoints[2].reportedTimestamp+=1;}, (v:any)=>{v.endpoints[2].intervalMs.lower=Infinity;},
    (v:any)=>{v.endpoints[2].intervalMs.lower=v.endpoints[2].intervalMs.upper+1;},
    (v:any)=>{v.endpoints[2].sourceIndex++;}, (v:any)=>{v.endpoints.splice(2,1);},
    (v:any)=>{v.endpoints[2]=v.endpoints[1];}, (v:any)=>{v.endpoints[2].reportedTimestamp=v.endpoints[1].reportedTimestamp;},
    (v:any)=>{v.gaps[1].fromSourceIndex=0;},(v:any)=>{v.gaps[1].toMs++;},
    (v:any)=>{v.envelope.first++;},(v:any)=>{v.envelope.last--;},
    (v:any)=>{v.endpoints[0].intervalMs.upper=v.measured.startMs;},
    (v:any)=>{v.endpoints.at(-1).intervalMs.lower=v.measured.endMs;},
    (v:any)=>{v.gaps.splice(1,1);},(v:any)=>{v.endpoints.forEach((p:any)=>p.reportedTimestamp=1);}
  ]) {const bad=structuredClone(e);mutate(bad);expect(()=>orbitBoundaryPopulations(bad)).toThrow();}
});

// Synthetic caller-qualified records exercise the actual constructor, while expected
// bounds below are frozen rational/binary64 values, not this implementation's output.
function sameTraceConstructed(times: number[], mode: OrbitEvidence["mode"] = "centerline") {
  const refs=times.map((t,i)=>({rawCaptureSha256:"a".repeat(64),markerIdentity:`m-${i}`,reportedTimestamp:Math.round(t*1000),
    reporterOccurrence:'[41,9,"id2.local","0x8","cc,benchmark","renderer","PipelineReporter"]',reporterBeginEventIndex:2*i,reporterEndEventIndex:2*i+1}));
  const basis:any={kind:"same-trace-integer-us/v1",qualification:"PASS_CALLER_BOUND_SAME_TRACE",rawCaptureSha256:"a".repeat(64),
    profile:SAME_TRACE_EXPORT_PROFILE,documentFrame:"document",documentTimeOrigin:10000,documentEvidenceEpoch:0,actionListenerObservedAt:1000,
    crossOriginIsolated:false,actionToken:"orbit",actionTraceTimestamp:1000000,canvasEpoch:1,contextEpoch:2,modelGeneration:3,
    rendererProcessId:41,rendererMainThreadId:7,rendererCompositorThreadId:9,layerTreeId:"23",references:refs};
  const results=refs.map(r=>({presentationTraceTimestamp:r.reportedTimestamp,markerIdentity:r.markerIdentity,token:"orbit",feedbackKind:"orbit",
    actionTraceTimestamp:1000000,canvasEpoch:1,contextEpoch:2,modelGeneration:3,rendererProcessId:41,rendererMainThreadId:7,
    rendererCompositorThreadId:9,layerTreeId:"23",status:"PASS_EXACT_CAUSAL_CHROMIUM_REPORTED_PRESENTATION",
    pipelineReporterOccurrence:r.reporterOccurrence,reporterBeginEventIndex:r.reporterBeginEventIndex,reporterEndEventIndex:r.reporterEndEventIndex,
    actionToPresentationIntervalMs:{lower:r.reportedTimestamp/1000-1000-.125,upper:r.reportedTimestamp/1000-1000+.125},
    pageToTraceOffsetIntervalMs:{minimum:0,maximum:0}}));
  return constructOrbitEvidence(results,1000,mode,basis);
}
test("same-trace scorer independently rejects narrow bounds and false carried context",()=>{
  const times=[2999,...Array.from({length:1000},(_,i)=>3009+i*10),13000.062,13009];
  const e=sameTraceConstructed(times),result=score({...fixture(),centerline:e});
  expect(result.validityFailures).toEqual([]);
  expect(result.orbitPopulations.centerline?.map(c=>c.gapCount)).toEqual([1001,1002]);
  expect(e.gaps[0].durationIntervalMs).toEqual({lower:9.998999999999999,upper:10.001000000000001});
  expect(e.endpoints[1].intervalMs.upper-e.endpoints[1].intervalMs.lower).toBeGreaterThan(.25);
  for(const mutate of [
    (v:any)=>{v.gaps[0].durationIntervalMs.upper=10;},(v:any)=>{v.gaps[0].durationIntervalMs.lower=10;},
    (v:any)=>{delete v.durationBasis;},(v:any)=>{v.durationBasis.qualification="FAIL";},
    (v:any)=>{v.durationBasis.references[0].rawCaptureSha256="b".repeat(64);},
    (v:any)=>{v.durationBasis.profile={...v.durationBasis.profile,exporterSha256:"0".repeat(64)};},
    (v:any)=>{v.durationBasis.references.pop();},(v:any)=>{v.durationBasis.references[1].reportedTimestamp++;},
    (v:any)=>{v.durationBasis.references[1].reporterBeginEventIndex=v.durationBasis.references[0].reporterBeginEventIndex;}
  ]){const bad=structuredClone(e);mutate(bad);expect(score({...fixture(),centerline:bad}).status).toBe("FAIL_INVALID_EVIDENCE");}
});
test("same-trace complete populations retain ties and prevent boundary dilution",()=>{
  // 60 high gaps: rank1139 of1199 is high; rank1140 of1200 is low.
  const deltas=Array.from({length:1199},(_,i)=>i>=500&&i<560?17000:7885);
  const start=13000062-deltas.reduce((a,b)=>a+b,0),us=[start];
  for(const delta of deltas)us.push(us.at(-1)!+delta);
  us.push(us.at(-1)!+7885);
  const e=sameTraceConstructed(us.map(t=>t/1000)),result=score({...fixture(),centerline:e});
  expect(result.validityFailures).toEqual([]);expect(result.status).toBe("FAIL_TARGETS");
  expect(result.orbitPopulations.centerline?.map(c=>c.gapCount)).toEqual([1199,1200]);
  expect(result.scores.centerlineP95).toBe(17.001000000000005);
  expect(result.orbitPopulations.centerline![1].p95UpperMs).toBe(7.886000000000001);
  const tied=sameTraceConstructed([2999,3000.062,3000.062,5000,9000,13000.062,13001]);
  expect(tied.gaps.filter(g=>g.fromMs===g.toMs)).toHaveLength(1);
  expect(tied.gaps.find(g=>g.fromMs===g.toMs)!.durationIntervalMs).toEqual({lower:0,upper:.0010000000000000002});
  expect(orbitBoundaryPopulations(tied).map(c=>[c.left,c.right])).toEqual([[0,5],[0,6],[2,5],[2,6]]);
  expect(result.scores.assignment).toBe(2000);expect(result.scores.pointP95).toBe(100);
});

test("same capture forbids every raw event reuse while tracks and raw index order may repeat",()=>{
  const e=sameTraceConstructed([2999,3000.062,5000,9000,13000.062,13001]);
  const basis:any=e.durationBasis;
  expect(new Set(basis.references.map((r:any)=>r.reporterOccurrence)).size).toBe(1);
  expect(orbitBoundaryPopulations(e)).toHaveLength(4);
  const reversed=structuredClone(e);(reversed.durationBasis as any).references.forEach((r:any)=>{[r.reporterBeginEventIndex,r.reporterEndEventIndex]=[r.reporterEndEventIndex,r.reporterBeginEventIndex];});
  expect(orbitBoundaryPopulations(reversed)).toEqual(orbitBoundaryPopulations(e));
  for(const [begin,end] of [[0,1],[0,3],[2,1],[1,3],[2,2]]) {
    const bad=structuredClone(e),refs=(bad.durationBasis as any).references;
    refs[1].reporterOccurrence="different-track";refs[1].reporterBeginEventIndex=begin;refs[1].reporterEndEventIndex=end;
    expect(()=>orbitBoundaryPopulations(bad)).toThrow("invalid orbit duration basis");
  }
});
