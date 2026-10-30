import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {observeOrbitTrace} from "./characterization-observations.mjs";
const hash=b=>createHash("sha256").update(b).digest("hex");
function fixture(extra=[]) {
  const pair={name:"PipelineReporter",pid:1,tid:3,id2:{local:"0x1"},cat:"cc"};
  const events=[{name:"TimeStamp",ph:"I",pid:1,tid:2,ts:1000000,args:{data:{message:"UIF_CAUSAL_V1:ACTION:orbit",frame:"doc"}}},
    {...pair,ph:"b",ts:2900000},{...pair,ph:"e",ts:3000000},...extra];
  const raw=Buffer.from(JSON.stringify({traceEvents:events}));
  const sha=hash(raw),key=JSON.stringify([1,3,"id2.local","0x1","cc","","PipelineReporter"]);
  const derived={extraction:{presentations:[{originalScore:33.334}]},orbitDurationBasis:{kind:"same-trace-integer-us/v1",qualification:"PASS_CALLER_BOUND_SAME_TRACE",rawCaptureSha256:sha,
    rendererProcessId:1,rendererMainThreadId:2,rendererCompositorThreadId:3,actionTraceTimestamp:1000000,actionToken:"orbit",documentFrame:"doc",
    references:[{rawCaptureSha256:sha,reporterBeginEventIndex:1,reporterEndEventIndex:2,reportedTimestamp:3000000,reporterOccurrence:key}]}};
  return {raw,derived,finalized:{traceDataLossOccurred:false,rawTraceTransport:{rawCompleteThroughEof:true,rawSha256:sha}}};
}
const bmf=(ts,other={})=>({name:"ProxyMain::BeginMainFrame",ph:"X",pid:1,tid:2,ts,dur:10,...other});
const check=f=>observeOrbitTrace(f.raw,f.derived,f.finalized);
const f=fixture([bmf(3000000),bmf(3000000,{dur:0}),bmf(3000001,{dur:undefined}),bmf(13000000,{dur:30}),bmf(2999999),bmf(13000001),bmf(4000000,{pid:9}),bmf(4000000,{tid:9})]);
const original=JSON.stringify(f.derived),report=check(f);
assert.equal(report.mainFrame.populationCount,4);assert.equal(report.mainFrame.missingDurationCount,1);assert.equal(report.mainFrame.knownDurationCount,3);
assert.deepEqual(report.mainFrame.events.map(e=>e.eventIndex),[3,4,5,6]);assert.equal(report.mainFrame.events[1].durationUs,"0");assert.equal(report.mainFrame.events[2].durationUs,null);
assert.equal(report.mainFrame.statistics.p95Us,"30");assert.equal(report.mainFrame.rejected.wrongProcessOrThread,2);assert.equal(JSON.stringify(f.derived),original);
assert.equal(check(fixture()).mainFrame.statistics.p95Us,null);
const nested=fixture([bmf(4000000,{ph:"B",dur:undefined}),{name:"nested",ph:"B",pid:1,tid:2,ts:4000001},{name:"nested",ph:"E",pid:1,tid:2,ts:4000002},bmf(4000020,{ph:"E",dur:undefined}),bmf(5000000,{ph:"B",dur:undefined})]);
assert.equal(check(nested).mainFrame.events[0].durationUs,"20");assert.equal(check(nested).mainFrame.events[1].durationStatus,"MISSING_DURATION");
const ambiguous=fixture([bmf(4000000,{ph:"B",dur:undefined}),{name:"wrong",ph:"E",pid:1,tid:2,ts:4000020}]);assert.equal(check(ambiguous).mainFrame.ambiguousOrInvalidDurationCount,1);
for(const mutate of [f=>f.finalized.traceDataLossOccurred=true,f=>f.finalized.rawTraceTransport.rawCompleteThroughEof=false,
  f=>f.derived.orbitDurationBasis.rendererMainThreadId=9,f=>f.derived.orbitDurationBasis.documentFrame="other",
  f=>f.derived.orbitDurationBasis.references[0].reporterOccurrence="wrong",f=>f.derived.orbitDurationBasis.references[0].reporterEndEventIndex=1]){const bad=fixture();mutate(bad);assert.throws(()=>check(bad));}
const duplicate=fixture([{name:"TimeStamp",ph:"I",pid:1,tid:2,ts:1000000,args:{data:{message:"UIF_CAUSAL_V1:ACTION:orbit",frame:"doc"}}}]);assert.throws(()=>check(duplicate));
const huge=fixture([bmf(3000000,{id:"PLACEHOLDER"})]);huge.raw=Buffer.from(huge.raw.toString().replace('"PLACEHOLDER"','18446744073709551615'));
const sha=hash(huge.raw);huge.derived.orbitDurationBasis.rawCaptureSha256=sha;huge.derived.orbitDurationBasis.references[0].rawCaptureSha256=sha;huge.finalized.rawTraceTransport.rawSha256=sha;
assert.equal(check(huge).mainFrame.events[0].traceId,"18446744073709551615");assert.equal(check(huge).losslessUnsafeIntegerConversions,1);
const gpu=fixture([{name:"process_name",ph:"M",pid:8,args:{name:"GPU Process"}},{name:"thread_name",ph:"M",pid:8,tid:9,args:{name:"CrGpuMain"}},
  {name:"GpuWork",ph:"X",pid:8,tid:9,ts:4000000},{name:"GpuWork",ph:"X",pid:8,tid:9,ts:4000000,dur:0}]);
assert.equal(check(gpu).gpuRelatedTraceSpans.missingDurationCount,1);assert.equal(check(gpu).gpuRelatedTraceSpans.byEventName.GpuWork.statistics.p95Us,"0");
const ambiguousGpu=fixture([{name:"process_name",ph:"M",pid:8,args:{name:"GPU Process"}},{name:"process_name",ph:"M",pid:9,args:{name:"GPU Process"}}]);
assert.equal(check(ambiguousGpu).gpuRelatedTraceSpans.status,"UNAVAILABLE_AMBIGUOUS_GPU_IDENTITY");assert.equal(check(fixture()).gpuRelatedTraceSpans.status,"UNAVAILABLE_GPU_IDENTITY");
console.log("PASS offline populations: ties, extra frames, boundaries, lossless IDs, missing/zero spans, nested B/E, exact and ambiguous attribution, incomplete trace, stable score bytes");
