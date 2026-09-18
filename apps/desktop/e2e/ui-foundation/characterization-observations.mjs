import { spawn, execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { readFile, writeFile, mkdir, readdir, realpath } from "node:fs/promises";
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

export const CONTINUATION_SLOTS=Object.freeze(["1000.2","1000.3","1000.4","1000.5","10000.1","10000.2","10000.3","10000.4","10000.5"]);
export const ORIGINAL_RETURN_SHA256="b9cd6955f39698ee611e1375153b81a9714e41c9320de745e099f93b5d91fee6";
const excludedExternalKeys=new Set(["UI_FOUNDATION_EVIDENCE_DIR","UI_FOUNDATION_METHOD_MANIFEST_PATH","UI_FOUNDATION_METHOD_MANIFEST_SHA256"]);
export const continuationExternalBindings=(seed,policy)=>({...Object.fromEntries(Object.entries(seed.frozenEnvironment).filter(([key])=>!excludedExternalKeys.has(key))),
  ...(policy?.displayTransition?{UI_FOUNDATION_REFERENCE_PROFILE:policy.displayTransition.profile.path,UI_FOUNDATION_REFERENCE_PROFILE_SHA256:policy.displayTransition.profile.sha256}:{})});
const jsonEqual=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
async function boundJson(ref) {
  if(!ref || !path.isAbsolute(ref.path) || !/^[a-f0-9]{64}$/.test(ref.sha256??""))throw new Error("absolute hash-bound JSON reference required");
  const bytes=await readFile(ref.path);if(hash(bytes)!==ref.sha256)throw new Error(`bound JSON changed: ${ref.path}`);return JSON.parse(bytes.toString());
}
async function writeOnce(file,value){await writeFile(file,`${JSON.stringify(value,null,2)}\n`,{flag:"wx"});return {path:file,sha256:hash(await readFile(file))};}
const ONE_SUCCESS_AUTHORITY_SHA256="ba5e8bceea55838cc0d23e815cc9a890ed543534085a9e9bdd932d133fb37fa2";
export const INTERNAL120_AUTHORITY_SHA256="6df272b7af0367b0083302229018143f8fa397a6bde0f5e368c6ec1e454157b9";
export async function validateDisplayTransition(policy,seed) {
  const t=policy.displayTransition;if(!t)return null;
  if(!policy.ownerTransition || t.schema!=="ui-foundation.internal120-transition/v1" || t.authority?.sha256!==INTERNAL120_AUTHORITY_SHA256 ||
    t.authority.path!==path.resolve(path.dirname(policy.seed.path),"../../OWNER_DIRECTION_INTERNAL120_20260917.md") ||
    hash(await readFile(t.authority.path))!==t.authority.sha256 ||
    !jsonEqual(t.previousProfile,{path:seed.frozenEnvironment.UI_FOUNDATION_REFERENCE_PROFILE,sha256:seed.frozenEnvironment.UI_FOUNDATION_REFERENCE_PROFILE_SHA256}))throw new Error("internal120 owner/profile lineage mismatch");
  const previous=await boundJson(t.previousProfile),record=await boundJson(t.profile);
  if(previous.refreshHz!==60 || record.schema!==previous.schema || record.refreshHz!==120 || record.cohortId!==policy.cohortId || record.productRevision!==previous.productRevision ||
    !["hostModel","memoryBytes","viewport","browserDpr","effectiveDprCap"].every(k=>jsonEqual(record[k],previous[k])) || record.externallyVerified!==true ||
    typeof record.verificationEvidence!=="string" || !record.verificationEvidence || !Number.isFinite(Date.parse(record.verifiedAt)) || record.display?.name!=="Color LCD" || record.display?.connection!=="spdisplays_internal" ||
    !/@ 120\.00Hz$/.test(record.display.resolution??"") || record.display.mirror!=="spdisplays_off" ||
    !["vendor","product","serial","pixels","resolution"].every(k=>typeof record.display[k]==="string"&&record.display[k]))throw new Error("authorized internal120 reference profile mismatch");
  return {authority:t.authority,previousProfile:t.previousProfile,profile:t.profile,record};
}
export async function continuationProfileAuthorization(env) {
  const policyRef={path:env.UI_FOUNDATION_CONTINUATION_POLICY,sha256:env.UI_FOUNDATION_CONTINUATION_POLICY_SHA256};
  if(!policyRef.path)return null;
  const policy=await boundJson(policyRef),seed=await boundJson(policy.seed),transition=await validateDisplayTransition(policy,seed);
  if(!transition)return null;
  const claim=await boundJson({path:env.UI_FOUNDATION_CONTINUATION_CLAIM,sha256:env.UI_FOUNDATION_CONTINUATION_CLAIM_SHA256});
  if(env.UI_FOUNDATION_PHASE!=="candidate" || env.UI_FOUNDATION_COLLECTION_MODE!=="characterization" || !/^10000\.[1-5]$/.test(claim.slot) ||
    claim.policySha256!==policyRef.sha256 || claim.methodSha256!==policy.method.sha256 || claim.executionToken!==env.UI_FOUNDATION_CONTINUATION_TOKEN ||
    env.UI_FOUNDATION_REFERENCE_PROFILE!==transition.profile.path || env.UI_FOUNDATION_REFERENCE_PROFILE_SHA256!==transition.profile.sha256)throw new Error("internal120 profile requires matching authorized10000 claim");
  return {refreshHz:120,authoritySha256:transition.authority.sha256,profileSha256:transition.profile.sha256};
}
const historicalSlots=["1000.2","1000.3","1000.4"];
const eligibleSlots=policy=>policy.ownerTransition?CONTINUATION_SLOTS.filter(s=>s!=="1000.5"):CONTINUATION_SLOTS;
const registryValue=(policy,policySha256)=>({seedSha256:policy.seed.sha256,cohortId:policy.cohortId,ledgerRoot:policy.ledgerRoot,policySha256,methodSha256:policy.method.sha256,instrumentRevision:policy.instrumentRevision});
const resultPath=entry=>{const [size,ordinal]=entry.claim.slot.split(".");return path.join(entry.claim.evidenceRoot,"raw",`run-${ordinal.padStart(2,"0")}`,size,"result.json");};
async function optionalJson(file){try{return JSON.parse(await readFile(file,"utf8"));}catch(error){if(error.code!=="ENOENT")throw error;return null;}}
function validCompleteResult(result,entry) {
  const [size,ordinal]=entry.claim.slot.split("."),expected=result?.expected,evidence=result?.evidence;
  return result?.collection?.evidenceValidity==="VALID" && result.collection.collectionCompleteness==="COMPLETE" &&
    expected?.runId===`${entry.claim.cohortId}.${entry.claim.slot}` && expected.fixtureSize===Number(size) && expected.runNumber===Number(ordinal) &&
    expected.bindings?.methodSha256===entry.claim.methodSha256 && typeof expected.sessionId==="string" && expected.sessionId.length>0 && evidence?.runId===expected.runId && evidence.sessionId===expected.sessionId &&
    evidence.freshSession===true && evidence.qualification==="PASS_QUALIFIED_RUN" && result.segmentCount===243 &&
    evidence.points?.length===200 && evidence.boxes?.length===20 && evidence.filters?.length===20 &&
    result.errors?.length===0 && result.scored?.validityFailures?.length===0 && ["PASS_METRIC_ACCEPTANCE","FAIL_TARGETS"].includes(result.scored?.status);
}
async function successfulEntry(policy,entry) {
  const terminal=await optionalJson(path.join(policy.ledgerRoot,`terminal-${entry.claim.slot}.json`)),file=resultPath(entry),result=await optionalJson(file);
  if(terminal?.exitCode!==0 || terminal.signal || terminal.launchError || terminal.claim?.sha256!==entry.sha256 || terminal.claim?.path!==entry.path || !validCompleteResult(result,entry))return null;
  return {slot:entry.claim.slot,claim:{path:entry.path,sha256:entry.sha256},result:{path:file,sha256:hash(await readFile(file))},terminal:{path:path.join(policy.ledgerRoot,`terminal-${entry.claim.slot}.json`),sha256:hash(await readFile(path.join(policy.ledgerRoot,`terminal-${entry.claim.slot}.json`)))}};
}
async function oneSuccessHistory(policy) {
  const t=policy.ownerTransition;if(!t)return null;
  if(t.schema!=="ui-foundation.one-success-transition/v1" || t.authority?.sha256!==ONE_SUCCESS_AUTHORITY_SHA256 ||
    t.authority.path!==path.resolve(path.dirname(policy.seed.path),"../../OWNER_DIRECTION_ONE_SUCCESS_20260917.md") ||
    hash(await readFile(t.authority.path))!==t.authority.sha256)throw new Error("owner waiver authority mismatch");
  const prior=await boundJson(t.previousPolicy),seed=await boundJson(policy.seed);validateContinuationPolicy(prior,seed);
  const displayTransition=await validateDisplayTransition(policy,seed);
  if(prior.ownerTransition || !["seed","cohortId","ledgerRoot","instrumentProjectRoot","attemptRoots"].every(k=>jsonEqual(prior[k],policy[k])) ||
    !jsonEqual(await boundJson(t.registry),registryValue(prior,t.previousPolicy.sha256)))throw new Error("historical policy/registry lineage or ledger relocation mismatch");
  const oldMethod=await boundJson(prior.method),newMethod=await boundJson(policy.method);
  const orchestration=new Set(["characterization-observations.mjs","characterization-observations.d.mts","characterization-mode.ts","full-cohort-controller.spec.ts","README.md"]);
  if(displayTransition) {
    const relative="apps/desktop/e2e/ui-foundation/characterization-commands.ts";
    const prefix=execFileSync("git",["rev-parse","--show-prefix"],{cwd:policy.instrumentProjectRoot,encoding:"utf8"}).trim();
    const oldSource=execFileSync("git",["show",`${prior.instrumentRevision}:${prefix}${relative}`],{cwd:policy.instrumentProjectRoot,encoding:"utf8"});
    const newSource=await readFile(path.join(policy.instrumentProjectRoot,relative),"utf8");
    const outsideProfileValidator=source=>source.replace(/export function validateDisplayProfile\([\s\S]*?(?=export async function captureDisplayProfile\()/,"PROFILE_VALIDATOR_ONLY\n");
    if(!oldSource.includes("export function validateDisplayProfile(") || outsideProfileValidator(oldSource)!==outsideProfileValidator(newSource))throw new Error("display transition changed commands outside profile validator");
    orchestration.add("characterization-commands.ts");
  }
  if(oldMethod.files?.length!==34 || newMethod.files?.length!==34 || !jsonEqual(oldMethod.files.map(f=>f.path),newMethod.files.map(f=>f.path)) ||
    oldMethod.files.some((f,i)=>!orchestration.has(path.basename(f.path))&&f.sha256!==newMethod.files[i].sha256))throw new Error("measurement method changed across accounting transition");
  if(!Array.isArray(t.history)||t.history.length!==3)throw new Error("exact three historical claims required");
  let previous=policy.seed.sha256;
  const history=[];
  for(const [i,record] of t.history.entries()) {
    const slot=historicalSlots[i],claim=await boundJson(record.claim),terminal=await boundJson(record.terminal),returned=await boundJson(record.return);
    if(record.claim.path!==path.join(policy.ledgerRoot,"claims",`${slot}.json`) || record.terminal.path!==path.join(policy.ledgerRoot,`terminal-${slot}.json`) ||
      claim.slot!==slot || claim.previousClaimSha256!==previous || claim.policySha256!==t.previousPolicy.sha256 || claim.seedSha256!==policy.seed.sha256 ||
      claim.cohortId!==policy.cohortId || claim.methodSha256!==prior.method.sha256 || claim.instrumentRevision!==prior.instrumentRevision ||
      claim.evidenceRoot!==prior.attemptRoots[slot] || terminal.slot!==slot || !jsonEqual(terminal.claim,record.claim) || returned.slot!==slot ||
      !Array.isArray(returned.evidence) || !returned.evidence.some(e=>jsonEqual(e,record.claim)) || !returned.evidence.some(e=>jsonEqual(e,record.terminal)))throw new Error("historical claim/terminal/return identity mismatch");
    for(const ref of returned.evidence)await boundJson(ref);
    if(i===0 && (returned.collection?.evidenceValidity!=="VALID" || returned.collection?.collectionCompleteness!=="COMPLETE" || returned.processExit!==0 ||
      terminal.exitCode!==0 || terminal.signal || terminal.launchError || returned.cleanup?.verificationStatus!=="VERIFIED" || returned.cleanup.browserProcessesRemaining!==0 ||
      returned.cleanup.serverListening!==false || returned.externalBindings!=="PASS_INDEPENDENT_EXTERNAL_REVALIDATION"))throw new Error("historical1000 success not verified");
    if(i===1 && (returned.collection?.evidenceValidity!=="INVALID" || returned.collection.collectionCompleteness!=="INCOMPLETE" || returned.processExit!==1 || terminal.exitCode!==1))throw new Error("historical1000.3 invalid outcome changed");
    if(i===2 && (returned.status!=="OWNER_CANCELLED_STARTED_SLOT_INTERRUPTED" || returned.startedBeforeSteering!==true || terminal.exitCode!==null || terminal.signal!=="SIGTERM" ||
      returned.externalRecovery?.verificationStatus!=="VERIFIED" || returned.externalRecovery.browserProcessesRemaining!==0 || returned.externalRecovery.serverListening!==false))throw new Error("historical interrupted1000.4 recovery not verified");
    const entry={claim,path:record.claim.path,sha256:record.claim.sha256,terminal,returned};
    if(i===0 && (!returned.evidence.some(e=>e.path===resultPath(entry)) || !await successfulEntry(prior,entry)))throw new Error("historical success result does not match actual claim/workload");
    history.push(entry);previous=record.claim.sha256;
  }
  return {prior,history};
}
export function validateContinuationPolicy(policy,seed) {
  if(policy.seed?.path!==path.join(path.dirname(seed.frozenEnvironment?.UI_FOUNDATION_REFERENCE_PROFILE??""),"RETURN.json"))throw new Error("canonical original seed path required");
  if(policy?.schema!=="ui-foundation.continuation-policy/v1" || policy.seed?.sha256!==ORIGINAL_RETURN_SHA256 || seed.counts?.attempted!==1 || seed.counts?.invalidFailed!==1 || seed.counts?.unattempted!==9 ||
    seed.collection?.runs?.length!==10 || !seed.collection.runs[0].runId.endsWith(".1000.1") || seed.collection.runs[0].disposition.evidenceValidity!=="INVALID" ||
    seed.collection.runs.slice(1).some(r=>r.disposition.attemptDisposition!=="UNATTEMPTED") || policy.cohortId!==seed.frozenEnvironment.UI_FOUNDATION_COHORT_ID ||
    !path.isAbsolute(policy.ledgerRoot??"") || !path.isAbsolute(policy.instrumentProjectRoot??"") || !/^[a-f0-9]{40}$/.test(policy.instrumentRevision??"") ||
    !jsonEqual(Object.keys(policy.attemptRoots??{}),CONTINUATION_SLOTS) || new Set(Object.values(policy.attemptRoots)).size!==9 ||
    Object.values(policy.attemptRoots).some(p=>!path.isAbsolute(p)||p===seed.frozenEnvironment.UI_FOUNDATION_EVIDENCE_DIR) || !/^[a-f0-9]{64}$/.test(policy.method?.sha256??""))throw new Error("invalid continuation policy/seed/ten lifetime slot budget");
}
export async function continuationClaims(policy) {
  const transition=await oneSuccessHistory(policy),slots=eligibleSlots(policy);
  const directory=path.join(policy.ledgerRoot,"claims");await mkdir(directory,{recursive:true});
  const names=(await readdir(directory)).sort();
  if(names.some(n=>!slots.some(slot=>n===`${slot}.json`)))throw new Error("unknown or owner-waived ledger claim");
  const claims=[];
  for(const slot of slots) {
    const file=path.join(directory,`${slot}.json`);
    if(!names.includes(`${slot}.json`)) {
      if(names.length!==claims.length)throw new Error("ledger has gap/reordered slot");break;
    }
    const bytes=await readFile(file),claim=JSON.parse(bytes.toString());
    const attribution=transition&&historicalSlots.includes(slot)?transition.prior:policy;
    if(claim.slot!==slot || claim.seedSha256!==ORIGINAL_RETURN_SHA256 || claim.cohortId!==policy.cohortId || claim.methodSha256!==attribution.method.sha256 ||
      claim.instrumentRevision!==attribution.instrumentRevision || claim.evidenceRoot!==attribution.attemptRoots[slot])throw new Error("untracked method or claim identity drift");
    if(transition && (claim.previousClaimSha256!==(claims.at(-1)?.sha256??policy.seed.sha256) ||
      (historicalSlots.includes(slot)?hash(bytes)!==transition.history[claims.length]?.sha256:
       claim.policySha256!==(await optionalJson(path.join(policy.ledgerRoot,"one-success-policy.json")))?.policySha256)))throw new Error("claim chain or historical/successor policy mismatch");
    if(slot.startsWith("10000.") && !jsonEqual(claim.displayTransition,policy.displayTransition))throw new Error("claim display profile attribution mismatch");
    claims.push({claim,path:file,sha256:hash(bytes)});
  }
  if(transition&&claims.length<3)throw new Error("historical consumed claims missing");
  return claims;
}
export async function validateContinuationReceipt(receipt,policy,slot,previous,seed) {
  await validateDisplayTransition(policy,seed);
  const external=continuationExternalBindings(seed,policy);
  if(receipt?.schema!=="ui-foundation.continuation-preconditions/v1" || receipt.slot!==slot || receipt.previousClaimSha256!==previous.sha256 ||
    receipt.cleanup?.status!=="VERIFIED_NO_REMAINING_BROWSER_OR_SERVER" || receipt.bindingsStatus!=="VERIFIED_UNCHANGED" ||
    !jsonEqual(receipt.externalBindings,external) || !Number.isFinite(Date.parse(receipt.verifiedAt)) || Date.parse(receipt.verifiedAt)<Date.parse(previous.claim?.claimedAt??seed.at) ||
    !Array.isArray(receipt.evidence) || !receipt.evidence.length)throw new Error("cleanup/external precondition receipt missing, stale or mismatched");
  for(const ref of receipt.evidence)await boundJson(ref);
  const proof=await boundJson(receipt.cleanup.evidence);
  if(proof.previousClaimSha256!==previous.sha256 || proof.browserProcessesRemaining!==0 || proof.serverListening!==false || proof.verificationStatus!=="VERIFIED" ||
    !["NORMAL_EXIT","EXTERNAL_RECOVERY_VERIFIED"].includes(proof.processDisposition))throw new Error("prior actual cleanup evidence not verified");
  if(previous.claim) {
    let terminal=null;try{terminal=JSON.parse(await readFile(path.join(policy.ledgerRoot,`terminal-${previous.claim.slot}.json`),"utf8"));}catch(error){if(error.code!=="ENOENT")throw error;}
    if((!terminal || terminal.exitCode===null || terminal.signal) && proof.processDisposition!=="EXTERNAL_RECOVERY_VERIFIED")throw new Error("interrupted/unknown prior process requires external recovery evidence");
  }
  const bindingProof=await boundJson(receipt.bindingEvidence);
  if(bindingProof.status!=="PASS_INDEPENDENT_EXTERNAL_REVALIDATION" || !jsonEqual(bindingProof.externalBindings,external))throw new Error("independent external binding proof mismatch");
}
async function verifyContinuationFiles(policy,seed) {
  await validateDisplayTransition(policy,seed);
  const env={...seed.frozenEnvironment,...continuationExternalBindings(seed,policy),UI_FOUNDATION_METHOD_MANIFEST_PATH:policy.method.path,UI_FOUNDATION_METHOD_MANIFEST_SHA256:policy.method.sha256};
  const head=execFileSync("git",["rev-parse","HEAD"],{cwd:policy.instrumentProjectRoot,encoding:"utf8"}).trim();
  if(head!==policy.instrumentRevision)throw new Error("instrument revision drift");
  const method=await boundJson(policy.method);
  if(method.files?.length!==34 || new Set(method.files.map(f=>f.path)).size!==34)throw new Error("complete34 method inventory required");
  for(const file of method.files){if(typeof file.path!=="string"||!file.path.startsWith("apps/desktop/e2e/ui-foundation/")||file.path.includes("..")||hash(await readFile(path.join(policy.instrumentProjectRoot,file.path)))!==file.sha256)throw new Error("method source bytes drift");}
  for(const [file,expected] of [[env.UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST,env.UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST_SHA256],
    [env.UI_FOUNDATION_REFERENCE_PROFILE,env.UI_FOUNDATION_REFERENCE_PROFILE_SHA256],[env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH,env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_SHA256],
    [path.join(env.UI_FOUNDATION_CANDIDATE_ORACLE_DIR,"CANDIDATE_POINT_ORACLE_MANIFEST.json"),env.UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256],
    [path.join(policy.instrumentProjectRoot,"apps/desktop/e2e/ui-foundation/fixture-manifest.json"),env.UI_FOUNDATION_MANIFEST_SHA256]]) {
    if(!path.isAbsolute(file)||hash(await readFile(file))!==expected)throw new Error(`external file drift: ${file}`);
  }
  const bundle=JSON.parse(await readFile(env.UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST,"utf8"));
  if(bundle.productRevision!=="8468a33c86adb622b25e98f98b0eaf28c7e9fa0e"||bundle.sourceStage!=="final")throw new Error("product provenance mismatch");
  for(const entry of [...bundle.source,...bundle.mutableTestOnlySourceSnapshot,...bundle.files]) {
    if(typeof entry.path!=="string"||path.isAbsolute(entry.path)||entry.path.split(/[\\/]/).includes(".."))throw new Error("product path escapes");
    const bytes=await readFile(path.join(env.UI_FOUNDATION_CANDIDATE_SOURCE_ROOT,entry.path));if(bytes.length!==entry.bytes||hash(bytes)!==entry.sha256)throw new Error("product source/build drift");
  }
  // Existing driver repeats the complete fixture/oracle/source/browser/profile checks.
  return env;
}
export async function validateClaimedContinuation(policyRef,claimRef,executionToken,evidenceRoot) {
  const policy=await boundJson(policyRef),seed=await boundJson(policy.seed);validateContinuationPolicy(policy,seed);
  const claim=await boundJson(claimRef),claims=await continuationClaims(policy),last=claims.at(-1);
  if(!last || last.sha256!==claimRef.sha256 || last.path!==claimRef.path || claim.policySha256!==policyRef.sha256 ||
    claim.executionToken!==executionToken || claim.evidenceRoot!==evidenceRoot)throw new Error("selected slot is not the current claimed launch");
  // Single driver entry claim blocks replay even if Playwright is invoked outside launcher.
  await writeOnce(path.join(policy.ledgerRoot,`entered-${claim.slot}.json`),{slot:claim.slot,claimSha256:claimRef.sha256,executionToken,enteredAt:new Date().toISOString()});
  return {policy,claim,fixtureSize:Number(claim.slot.split(".")[0]),runNumber:Number(claim.slot.split(".")[1])};
}
async function bindContinuationRegistry(policyRef,policy,operations={}) {
  const registryFile=`${await realpath(policy.seed.path)}.continuation-ledger.json`;
  if(policy.ownerTransition) {
    await oneSuccessHistory(policy);
    if(operations.bindRegistry)await operations.bindRegistry(registryFile,await boundJson(policy.ownerTransition.registry));
    else if(policy.ownerTransition.registry.path!==registryFile)throw new Error("canonical original registry path required");
    const seal={...registryValue(policy,policyRef.sha256),previousRegistry:policy.ownerTransition.registry,authority:policy.ownerTransition.authority,waivedUnattempted:["1000.5"]};
    const file=path.join(policy.ledgerRoot,"one-success-policy.json");
    try{await writeOnce(file,seal);}catch(error){if(error.code!=="EEXIST")throw error;if(!jsonEqual(await optionalJson(file),seal))throw new Error("owner transition already sealed with different policy");}
    return;
  }
  if(await optionalJson(path.join(policy.ledgerRoot,"one-success-policy.json")))throw new Error("superseded full-budget policy cannot launch");
  const registry=registryValue(policy,policyRef.sha256);
  if(operations.bindRegistry)await operations.bindRegistry(registryFile,registry);
  else {try{await writeOnce(registryFile,registry);}catch(error){if(error.code!=="EEXIST")throw error;if(!jsonEqual(JSON.parse(await readFile(registryFile,"utf8")),registry))throw new Error("approved ledger relocation/reset rejected");}}
}
async function oneSuccessState(policy,claims) {
  if(!policy.ownerTransition)return {candidate:null,completion:null};
  let candidate=null;
  for(const entry of claims.filter(e=>e.claim.slot.startsWith("10000."))) {
    candidate=await successfulEntry(policy,entry);if(candidate)break;
  }
  const completion=await optionalJson(path.join(policy.ledgerRoot,"one-success-complete.json"));
  if(completion) {
    const seal=await optionalJson(path.join(policy.ledgerRoot,"one-success-policy.json"));
    if(!candidate || !jsonEqual(candidate,completion.success) || completion.policySha256!==seal?.policySha256 ||
      !jsonEqual(completion.waivedUnattempted,CONTINUATION_SLOTS.filter(s=>s.startsWith("10000.")&&!claims.some(e=>e.claim.slot===s))))throw new Error("completion identity or waiver mismatch");
    for(const ref of [completion.success.result,completion.success.terminal])await boundJson(ref);
    const seed=await boundJson(policy.seed),entry=claims.find(e=>e.claim.slot===candidate.slot);
    await validateContinuationReceipt(await boundJson(completion.receipt),policy,candidate.slot,entry,seed);
  }
  return {candidate,completion};
}
export async function closeOneSuccess(policyRef,receiptRef,operations={}) {
  const policy=await boundJson(policyRef),seed=await boundJson(policy.seed);validateContinuationPolicy(policy,seed);
  if(!policy.ownerTransition)throw new Error("owner one-success transition required");
  await bindContinuationRegistry(policyRef,policy,operations);
  const claims=await continuationClaims(policy),state=await oneSuccessState(policy,claims),entry=claims.at(-1);
  if(!state.candidate || state.candidate.slot!==entry?.claim.slot || state.completion)throw new Error("no unmatched valid complete successful actual10000 run to close");
  await validateContinuationReceipt(await boundJson(receiptRef),policy,entry.claim.slot,entry,seed);
  await (operations.verifyFiles??verifyContinuationFiles)(policy,seed);
  const completion={schema:"ui-foundation.one-success-complete/v1",policySha256:policyRef.sha256,authority:policy.ownerTransition.authority,success:state.candidate,receipt:receiptRef,
    waivedUnattempted:CONTINUATION_SLOTS.filter(s=>s.startsWith("10000.")&&!claims.some(e=>e.claim.slot===s)),completedAt:new Date().toISOString(),targetAcceptanceClaim:false};
  await writeOnce(path.join(policy.ledgerRoot,"one-success-complete.json"),completion);
  await writeOnce(path.join(policy.ledgerRoot,"one-success-final-budget.json"),await continuationBudgetReport(policy,seed));
  return completion;
}
async function persistedProgress(root,emit) {
  let fingerprint="";
  return setInterval(async()=>{
    try {
      const raw=path.join(root,"raw"),runs=await readdir(raw),states=[];
      for(const run of runs)for(const size of await readdir(path.join(raw,run))) {
        const dir=path.join(raw,run,size),dirents=await readdir(dir,{withFileTypes:true}),names=dirents.map(e=>e.name);
        for(const name of dirents.filter(e=>e.isDirectory()&&/^(assignment|point-selection|box-selection|tree-filter|orbit)-/.test(e.name)).map(e=>e.name)) {
          const entries=await readdir(path.join(dir,name));states.push(`${run}/${size}/${name}:${entries.includes("derived.json")?"derived":entries.includes("action-stopped.json")?"stopped":entries.includes("prepared.json")?"prepared":"created"}`);
        }
        for(const name of ["result.json","run-failure.json","restoration.json"])if(names.includes(name))states.push(`${run}/${size}/${name}`);
      }
      const next=states.join("|");if(next!==fingerprint){fingerprint=next;emit(`persisted progress: ${states.slice(-3).join(", ")} (${states.length} existing records)`);}
    }catch{/* Only already persisted optional progress; never affects measurement or substitutes for status. */}
  },10000);
}
async function spawnOne(policy,env) {
  return new Promise((resolve,reject)=>{
    const cleanHostEnv=Object.fromEntries(Object.entries(process.env).filter(([key])=>!key.startsWith("UI_FOUNDATION_")&&!key.startsWith("PLAYWRIGHT_")));
    const child=spawn(process.execPath,["node_modules/@playwright/test/cli.js","test","--config","apps/desktop/e2e/ui-foundation/playwright.candidate-performance.config.ts","--headed","--workers=1","--retries=0","--repeat-each=1"],{cwd:policy.instrumentProjectRoot,env:{...cleanHostEnv,...env},stdio:"inherit"});
    child.once("error",reject);child.once("exit",(exitCode,signal)=>resolve({exitCode,signal}));
  });
}
export async function continuationBudgetReport(policy,seed) {
  const claims=await continuationClaims(policy);
  const state=await oneSuccessState(policy,claims);
  const slots=[{slot:"1000.1",consumed:true,instrumentRevision:seed.instrumentRevision,methodSha256:seed.originalScores.bindings.methodSha256,
    evidenceValidity:"INVALID",collectionCompleteness:"INCOMPLETE",originalScored:seed.originalScores,source:policy.seed,
    referenceProfile:{path:seed.frozenEnvironment.UI_FOUNDATION_REFERENCE_PROFILE,sha256:seed.frozenEnvironment.UI_FOUNDATION_REFERENCE_PROFILE_SHA256}}];
  for(const slot of CONTINUATION_SLOTS) {
    const entry=claims.find(c=>c.claim.slot===slot);
    if(!entry){slots.push({slot,consumed:false,disposition:policy.ownerTransition&&(slot==="1000.5"||state.completion?.waivedUnattempted.includes(slot))?"WAIVED_UNATTEMPTED_BY_OWNER":"UNATTEMPTED"});continue;}
    const [size,ordinal]=slot.split("."),resultFile=path.join(entry.claim.evidenceRoot,"raw",`run-${ordinal.padStart(2,"0")}`,size,"result.json");
    let result=null,terminal=null;
    try{result=JSON.parse(await readFile(resultFile,"utf8"));}catch(error){if(error.code!=="ENOENT")throw error;}
    try{terminal=JSON.parse(await readFile(path.join(policy.ledgerRoot,`terminal-${slot}.json`),"utf8"));}catch(error){if(error.code!=="ENOENT")throw error;}
    slots.push({slot,consumed:true,instrumentRevision:entry.claim.instrumentRevision,methodSha256:entry.claim.methodSha256,claimSha256:entry.sha256,
      referenceProfile:entry.claim.displayTransition?.profile??{path:seed.frozenEnvironment.UI_FOUNDATION_REFERENCE_PROFILE,sha256:seed.frozenEnvironment.UI_FOUNDATION_REFERENCE_PROFILE_SHA256},
      evidenceValidity:result?.collection?.evidenceValidity??"INVALID_OR_UNAVAILABLE",collectionCompleteness:result?.collection?.collectionCompleteness??"INCOMPLETE",
      originalScored:result?.scored??null,runtimeDisposition:terminal?.exitCode===0&&!terminal.signal&&!terminal.launchError?"PASSED":"FAILED_OR_UNAVAILABLE",terminal,resultFile,
      ...(policy.ownerTransition?{matchedIdentityAndWorkload:validCompleteResult(result,entry)}:{}),
      ...(policy.ownerTransition&&slot==="1000.4"?{ownerDisposition:"OWNER_INTERRUPTED_CONSUMED",historicalReturn:policy.ownerTransition.history[2].return}:{})});
  }
  return {schema:"ui-foundation.lifetime-attempt-budget-report/v1",seed:policy.seed,slots,consumed:slots.filter(s=>s.consumed).length,
    validCompleteBySize:Object.fromEntries(["1000","10000"].map(size=>[size,slots.filter(s=>s.slot.startsWith(`${size}.`)&&s.runtimeDisposition==="PASSED"&&s.evidenceValidity==="VALID"&&s.collectionCompleteness==="COMPLETE"&&(!policy.ownerTransition||s.matchedIdentityAndWorkload)).length])),
    ...(policy.ownerTransition?{confirmedSuccessBySize:{1000:1,10000:state.completion?1:0},
      interrupted:slots.filter(s=>s.consumed&&s.terminal&&(s.terminal.signal||s.terminal.launchError||s.terminal.exitCode===null)).length,
      invalidFailed:slots.filter(s=>s.consumed&&s.ownerDisposition!=="OWNER_INTERRUPTED_CONSUMED"&&!(s.terminal&&(s.terminal.signal||s.terminal.launchError||s.terminal.exitCode===null))&&!(s.matchedIdentityAndWorkload&&s.runtimeDisposition==="PASSED")).length}:{}),
    waived:slots.filter(s=>s.disposition==="WAIVED_UNATTEMPTED_BY_OWNER").length,eligible:slots.filter(s=>s.disposition==="UNATTEMPTED").length,
    ownerStoppingStatus:policy.ownerTransition?(state.completion?"SUCCESS_CONFIRMED_AND_REMAINDER_WAIVED":state.candidate?"VALID_COMPLETE_PROCESS_SUCCESS_PENDING_CLEANUP":claims.filter(e=>e.claim.slot.startsWith("10000.")).length===5?"FIVE_ATTEMPTS_EXHAUSTED_NO_SUCCESS":"CONTINUE_NEXT10000_AFTER_PRECONDITIONS"):null,
    originalAggregateFailure:seed.collection.targetOutcome,qualificationCohort:false,comparison:"Original and successor methods remain separate; attempt budget completion is not valid-workload or metric acceptance."};
}
export async function launchContinuationSlot(policyRef,slot,receiptRef,operations={}) {
  const policy=await boundJson(policyRef),seed=await boundJson(policy.seed);validateContinuationPolicy(policy,seed);
  if(!CONTINUATION_SLOTS.includes(slot) || (policy.ownerTransition&&!slot.startsWith("10000.")))throw new Error("consumed/waived/unknown/sixth slot rejected");
  await mkdir(policy.ledgerRoot,{recursive:true});
  await bindContinuationRegistry(policyRef,policy,operations);
  const claims=await continuationClaims(policy);
  if((await oneSuccessState(policy,claims)).candidate)throw new Error("valid complete10000 process success already exists; close with verified cleanup, no further launch");
  if(slot!==eligibleSlots(policy)[claims.length])throw new Error("slot consumed or out of order");
  const previous=claims.at(-1)??{sha256:policy.seed.sha256};
  const receipt=await boundJson(receiptRef);await validateContinuationReceipt(receipt,policy,slot,previous,seed);
  const env=await (operations.verifyFiles??verifyContinuationFiles)(policy,seed);
  if(policy.ownerTransition) {
    const current=await continuationClaims(policy);
    if(current.length!==claims.length || (await oneSuccessState(policy,current)).candidate)throw new Error("stale concurrent launch or successful run already present");
  }
  const evidenceRoot=policy.attemptRoots[slot];await mkdir(evidenceRoot); // exclusive new output; never reuse
  const token=createHash("sha256").update(`${policyRef.sha256}:${slot}:${Date.now()}:${process.pid}`).digest("hex");
  const claim={schema:"ui-foundation.started-slot/v1",slot,cohortId:policy.cohortId,seedSha256:policy.seed.sha256,policySha256:policyRef.sha256,
    ...(policy.displayTransition?{displayTransition:policy.displayTransition}:{}),
    previousClaimSha256:previous.sha256,receipt:receiptRef,methodSha256:policy.method.sha256,instrumentRevision:policy.instrumentRevision,
    evidenceRoot,executionToken:token,claimedAt:new Date().toISOString(),disposition:"CONSUMED_BEFORE_SUBPROCESS_LAUNCH"};
  const claimRef=await writeOnce(path.join(policy.ledgerRoot,"claims",`${slot}.json`),claim);
  const launchEnv={...env,UI_FOUNDATION_EVIDENCE_DIR:evidenceRoot,UI_FOUNDATION_COLLECTION_MODE:"characterization",
    UI_FOUNDATION_CONTINUATION_POLICY:policyRef.path,UI_FOUNDATION_CONTINUATION_POLICY_SHA256:policyRef.sha256,
    UI_FOUNDATION_CONTINUATION_CLAIM:claimRef.path,UI_FOUNDATION_CONTINUATION_CLAIM_SHA256:claimRef.sha256,UI_FOUNDATION_CONTINUATION_TOKEN:token};
  delete launchEnv.UI_FOUNDATION_SMOKE;delete launchEnv.UI_FOUNDATION_DIAGNOSTIC_MODE;
  const timer=operations.spawn?null:await persistedProgress(evidenceRoot,line=>process.stdout.write(`${line}\n`));
  let outcome;
  try{outcome=await (operations.spawn??spawnOne)(policy,launchEnv);}
  catch(error){outcome={exitCode:null,signal:null,launchError:String(error)};}
  finally{if(timer)clearInterval(timer);}
  const terminal={...outcome,slot,claim:claimRef,finishedAt:new Date().toISOString(),cleanup:"UNKNOWN_REQUIRES_INDEPENDENT_REVALIDATION",comparison:"mixed-method attempt budget; not same-method cohort qualification"};
  await writeOnce(path.join(policy.ledgerRoot,`terminal-${slot}.json`),terminal);
  await writeOnce(path.join(policy.ledgerRoot,`budget-after-${slot}.json`),await continuationBudgetReport(policy,seed));
  return terminal;
}

if(process.argv[1] && path.resolve(process.argv[1])===fileURLToPath(import.meta.url)) {
  if(process.argv[2]==="close-one-success") {
    if(process.argv.length!==7)throw new Error("usage: node characterization-observations.mjs close-one-success POLICY_JSON POLICY_SHA256 RECEIPT_JSON RECEIPT_SHA256");
    process.stdout.write(`${JSON.stringify(await closeOneSuccess({path:path.resolve(process.argv[3]),sha256:process.argv[4]},{path:path.resolve(process.argv[5]),sha256:process.argv[6]}))}\n`);
  } else if(process.argv[2]==="launch-slot") {
    if(process.argv.length!==8)throw new Error("usage: node characterization-observations.mjs launch-slot POLICY_JSON POLICY_SHA256 SLOT RECEIPT_JSON RECEIPT_SHA256");
    const result=await launchContinuationSlot({path:path.resolve(process.argv[3]),sha256:process.argv[4]},process.argv[5],{path:path.resolve(process.argv[6]),sha256:process.argv[7]});
    process.stdout.write(`${JSON.stringify(result)}\n`);
    process.exitCode=Number.isInteger(result.exitCode)?result.exitCode:1;
  } else {
    const [, , attemptFile, outputFile]=process.argv;
    if(!attemptFile||!outputFile||process.argv.length!==4)throw new Error("usage: node characterization-observations.mjs CLOSED_ATTEMPT_JSON OUTPUT_JSON");
    await observeAttempt(path.resolve(attemptFile),path.resolve(outputFile));
  }
}
