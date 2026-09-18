import { createHash } from "node:crypto";
import { readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { parseTraceEventsWithUnsafeIntegersAsDecimalStrings } from "./lossless-trace-json.mjs";

const hash = bytes => createHash("sha256").update(bytes).digest("hex");
const integer = value => typeof value === "number" && Number.isSafeInteger(value) ? BigInt(value)
  : typeof value === "string" && /^-?\d+$/.test(value) ? BigInt(value) : null;
const same = (a,b) => integer(a) !== null && integer(a) === integer(b);
const statistics = values => {
  if (!values.length) return { status:"UNAVAILABLE_EMPTY_DURATION_POPULATION", count:0, p50Us:null, p95Us:null, maxUs:null };
  const sorted = values.map(BigInt).sort((a,b)=>a<b?-1:a>b?1:0);
  return { status:"AVAILABLE_TRACE_DURATION_STATISTICS",count:sorted.length,p50Us:String(sorted[Math.ceil(sorted.length*.5)-1]),
    p95Us:String(sorted[Math.ceil(sorted.length*.95)-1]),maxUs:String(sorted.at(-1)) };
};
function population(events, nameMatches, pid, tid, start, end) {
  const rows=[], rejected={wrongProcessOrThread:0,invalidTimestamp:0,unknownPhase:0};
  for(let index=0;index<events.length;index++) {
    const e=events[index]; if(!nameMatches(e.name) || e.ph==="E")continue;
    if(!same(e.pid,pid)||!same(e.tid,tid)){rejected.wrongProcessOrThread++;continue;}
    const ts=integer(e.ts); if(ts===null){rejected.invalidTimestamp++;continue;}
    if(ts<start||ts>end)continue;
    let duration=null,endEventIndex=null,durationStatus="MISSING_DURATION";
    if(e.ph==="X") {
      const d=integer(e.dur); if(d!==null && d>=0n){duration=String(d);durationStatus="AVAILABLE";}
      else if(e.dur!==undefined)durationStatus="INVALID_DURATION";
    } else if(e.ph==="B") {
      let depth=1;
      for(let j=index+1;j<events.length;j++) {
        const candidate=events[j]; if(!same(candidate.pid,pid)||!same(candidate.tid,tid))continue;
        if(candidate.ph==="B")depth++;
        if(candidate.ph==="E" && --depth===0) {
          const endTs=integer(candidate.ts);
          if(endTs!==null && endTs>=ts && (candidate.name===undefined || candidate.name===e.name)) {duration=String(endTs-ts);durationStatus="AVAILABLE";endEventIndex=j;}
          else durationStatus="AMBIGUOUS_OR_INVALID_END";
          break;
        }
      }
    } else {durationStatus="AMBIGUOUS_UNSUPPORTED_PHASE";rejected.unknownPhase++;}
    rows.push({eventIndex:index,endEventIndex,name:e.name,phase:e.ph,pid:String(pid),tid:String(tid),timestampUs:String(ts),durationUs:duration,durationStatus,
      traceId:e.id??e.id2??null});
  }
  return {populationCount:rows.length,knownDurationCount:rows.filter(r=>r.durationUs!==null).length,
    missingDurationCount:rows.filter(r=>r.durationStatus==="MISSING_DURATION").length,
    ambiguousOrInvalidDurationCount:rows.filter(r=>!["MISSING_DURATION","AVAILABLE"].includes(r.durationStatus)).length,
    rejected,events:rows,statistics:statistics(rows.filter(r=>r.durationUs!==null).map(r=>r.durationUs))};
}
export function observeOrbitTrace(rawBytes, derived, finalized) {
  if(!Buffer.isBuffer(rawBytes))throw new Error("canonical raw Buffer required");
  const rawSha256=hash(rawBytes), basis=derived?.orbitDurationBasis;
  if(rawBytes.length>268435456 || !basis || basis.kind!=="same-trace-integer-us/v1" || basis.qualification!=="PASS_CALLER_BOUND_SAME_TRACE" ||
      basis.rawCaptureSha256!==rawSha256 || finalized?.traceDataLossOccurred!==false ||
      finalized?.rawTraceTransport?.rawCompleteThroughEof!==true || finalized.rawTraceTransport.rawSha256!==rawSha256)
    throw new Error("incomplete, unbound or changed canonical orbit trace");
  const {events,convertedCount}=parseTraceEventsWithUnsafeIntegersAsDecimalStrings(rawBytes.toString("utf8"));
  const pid=integer(basis.rendererProcessId),tid=integer(basis.rendererMainThreadId),action=integer(basis.actionTraceTimestamp);
  if(pid===null||tid===null||action===null)throw new Error("required exact renderer identity/window unavailable");
  const actions=events.map((event,index)=>({event,index})).filter(({event:e})=>e.name==="TimeStamp" && e.args?.data?.message===`UIF_CAUSAL_V1:ACTION:${basis.actionToken}`);
  if(actions.length!==1 || !same(actions[0].event.pid,pid.toString()) || !same(actions[0].event.tid,tid.toString()) ||
      integer(actions[0].event.ts)!==action || actions[0].event.args?.data?.frame!==basis.documentFrame)
    throw new Error("missing or ambiguous exact action/process/thread binding");
  const occurrenceKey=e=>JSON.stringify([e.pid,e.tid,"id2.local",e.id2?.local,e.cat??"",e.scope??"",e.name]);
  const refs=basis.references;
  if(!Array.isArray(refs)||!refs.length)throw new Error("same-trace presentation references missing");
  for(const ref of refs) {
    const begin=events[ref.reporterBeginEventIndex],end=events[ref.reporterEndEventIndex];
    if(ref.rawCaptureSha256!==rawSha256 || begin?.name!=="PipelineReporter" || begin.ph!=="b" || end?.name!=="PipelineReporter" || end.ph!=="e" ||
      !same(begin.pid,basis.rendererProcessId)||!same(end.pid,basis.rendererProcessId)||!same(begin.tid,basis.rendererCompositorThreadId)||!same(end.tid,basis.rendererCompositorThreadId)||!same(end.ts,ref.reportedTimestamp) || typeof begin.id2?.local!=="string" ||
      occurrenceKey(begin)!==ref.reporterOccurrence || occurrenceKey(end)!==ref.reporterOccurrence || integer(begin.ts)===null || integer(end.ts)===null || integer(begin.ts)>integer(end.ts))
      throw new Error("same-trace presentation occurrence binding drift");
  }
  const start=action+2000000n,end=action+12000000n;
  const mainFrame=population(events,n=>n==="ProxyMain::BeginMainFrame",pid.toString(),tid.toString(),start,end);
  const gpuProcesses=[...new Set(events.filter(e=>e.ph==="M"&&e.name==="process_name"&&/^(GPU Process|Gpu|GPU)$/.test(e.args?.name??"")).map(e=>String(e.pid)))];
  const gpuThreads=events.filter(e=>e.ph==="M"&&e.name==="thread_name"&&e.args?.name==="CrGpuMain"&&gpuProcesses.includes(String(e.pid)));
  const gpu= gpuProcesses.length===1 && gpuThreads.length===1
    ? {status:"AVAILABLE_UNBOUND_GPU_THREAD_CONTEXT",pid:gpuProcesses[0],tid:String(gpuThreads[0].tid),
      ...population(events,n=>typeof n==="string"&&/(Gpu|GPU|gpu|SwapBuffers|Present|Fence)/.test(n),gpuProcesses[0],String(gpuThreads[0].tid),start,end)}
    : {status:gpuProcesses.length>1||gpuThreads.length>1?"UNAVAILABLE_AMBIGUOUS_GPU_IDENTITY":"UNAVAILABLE_GPU_IDENTITY",processCandidates:gpuProcesses,threadCandidates:gpuThreads.map(e=>({pid:String(e.pid),tid:String(e.tid)})),populationCount:null,statistics:null};
  if(gpu.events) {
    gpu.byEventName=Object.fromEntries([...new Set(gpu.events.map(e=>e.name))].map(name=>{
      const rows=gpu.events.filter(e=>e.name===name);
      return [name,{populationCount:rows.length,missingDurationCount:rows.filter(e=>e.durationUs===null).length,
        statistics:statistics(rows.filter(e=>e.durationUs!==null).map(e=>e.durationUs))}];
    }));
    gpu.statistics=null; // Different trace span names are never pooled into one GPU quantity.
  }
  return {schema:"ui-foundation.offline-orbit-observations/v1",rawSha256,rawBytes:rawBytes.length,losslessUnsafeIntegerConversions:convertedCount,
    actionEventIndex:actions[0].index,window:{startUs:String(start),endUs:String(end),inclusive:true,definition:"action trace timestamp + 2000000 through + 12000000 us; start-event membership"},
    attribution:{pid:String(pid),tid:String(tid),documentFrame:basis.documentFrame,actionToken:basis.actionToken,status:"EXACT_SAME_TRACE_RENDERER_MAIN_THREAD"},
    mainFrame:{eventName:"ProxyMain::BeginMainFrame",...mainFrame},gpuRelatedTraceSpans:gpu,
    originalPresentationReferences:refs,originalExtractionSha256:hash(JSON.stringify(derived.extraction)),
    limits:["All attributable main-frame occurrences, including ties; not feedback-selected frames.","Durations are recorded trace spans, not hardware GPU time; missing duration is never zero.",
      "GPU spans are same-window thread context, never causal per-frame joins or nearest-time cross-process attribution.","Original input latency and Chromium-reported presentation scores are unchanged; presentation is not scanout."]};
}
export async function observeAttempt(attemptFile, outputFile) {
  const attemptBytes=await readFile(attemptFile),attempt=JSON.parse(attemptBytes.toString());
  if(attempt.timedAttemptEnded!==true || !["COMPLETED","ABORTED"].includes(attempt.collection?.attemptDisposition) || !Array.isArray(attempt.runRecords))
    throw new Error("offline analysis requires a closed timed attempt report");
  const observations=[],unavailable=[];
  for(const run of attempt.runRecords) {
    if(!run.resultFile) {for(const mode of ["orbit-001","orbit-002"])unavailable.push({runId:run.expected.runId,mode,status:"UNAVAILABLE_NOT_ZERO",error:"run result/capture path unavailable"});continue;}
    for(const name of ["orbit-001","orbit-002"]) {
      const directory=path.join(path.dirname(run.resultFile),name);
      try {
        const [raw,derivedBytes,finalizedBytes]=await Promise.all([readFile(path.join(directory,"trace-events.raw.json")),readFile(path.join(directory,"derived.json")),readFile(path.join(directory,"trace-finalized.json"))]);
        observations.push({runId:run.expected.runId,mode:name,rawPath:path.join(directory,"trace-events.raw.json"),derivedSha256:hash(derivedBytes),finalizedSha256:hash(finalizedBytes),
          ...observeOrbitTrace(raw,JSON.parse(derivedBytes.toString()),JSON.parse(finalizedBytes.toString()))});
      } catch(error){unavailable.push({runId:run.expected.runId,mode:name,error:String(error),status:"UNAVAILABLE_NOT_ZERO"});}
    }
  }
  const report={schema:"ui-foundation.offline-attempt-observations/v1",attemptFile,attemptSha256:hash(attemptBytes),observations,unavailable,
    derivative:true,canonicalRawPolicy:"References only; canonical raw bytes never rewritten",qualificationClaim:false};
  await writeFile(outputFile,`${JSON.stringify(report,null,2)}\n`,{flag:"wx"}); return report;
}
if(process.argv[1] && path.resolve(process.argv[1])===fileURLToPath(import.meta.url)) {
  const [, , attemptFile, outputFile]=process.argv;
  if(!attemptFile||!outputFile||process.argv.length!==4)throw new Error("usage: node characterization-observations.mjs CLOSED_ATTEMPT_JSON OUTPUT_JSON");
  await observeAttempt(path.resolve(attemptFile),path.resolve(outputFile));
}
