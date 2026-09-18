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
export const continuationExternalBindings=seed=>Object.fromEntries(Object.entries(seed.frozenEnvironment).filter(([key])=>!excludedExternalKeys.has(key)));
const jsonEqual=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
async function boundJson(ref) {
  if(!ref || !path.isAbsolute(ref.path) || !/^[a-f0-9]{64}$/.test(ref.sha256??""))throw new Error("absolute hash-bound JSON reference required");
  const bytes=await readFile(ref.path);if(hash(bytes)!==ref.sha256)throw new Error(`bound JSON changed: ${ref.path}`);return JSON.parse(bytes.toString());
}
async function writeOnce(file,value){await writeFile(file,`${JSON.stringify(value,null,2)}\n`,{flag:"wx"});return {path:file,sha256:hash(await readFile(file))};}
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
  const directory=path.join(policy.ledgerRoot,"claims");await mkdir(directory,{recursive:true});
  const names=(await readdir(directory)).sort();
  if(names.some(n=>!CONTINUATION_SLOTS.some(slot=>n===`${slot}.json`)))throw new Error("unknown ledger claim");
  const claims=[];
  for(const slot of CONTINUATION_SLOTS) {
    const file=path.join(directory,`${slot}.json`);
    if(!names.includes(`${slot}.json`)) {
      if(names.length!==claims.length)throw new Error("ledger has gap/reordered slot");break;
    }
    const bytes=await readFile(file),claim=JSON.parse(bytes.toString());
    if(claim.slot!==slot || claim.seedSha256!==ORIGINAL_RETURN_SHA256 || claim.cohortId!==policy.cohortId || claim.methodSha256!==policy.method.sha256 ||
      claim.instrumentRevision!==policy.instrumentRevision || claim.evidenceRoot!==policy.attemptRoots[slot])throw new Error("untracked method or claim identity drift");
    claims.push({claim,path:file,sha256:hash(bytes)});
  }
  return claims;
}
export async function validateContinuationReceipt(receipt,policy,slot,previous,seed) {
  const external=continuationExternalBindings(seed);
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
  const env={...seed.frozenEnvironment,UI_FOUNDATION_METHOD_MANIFEST_PATH:policy.method.path,UI_FOUNDATION_METHOD_MANIFEST_SHA256:policy.method.sha256};
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
  const slots=[{slot:"1000.1",consumed:true,instrumentRevision:seed.instrumentRevision,methodSha256:seed.originalScores.bindings.methodSha256,
    evidenceValidity:"INVALID",collectionCompleteness:"INCOMPLETE",originalScored:seed.originalScores,source:policy.seed}];
  for(const slot of CONTINUATION_SLOTS) {
    const entry=claims.find(c=>c.claim.slot===slot);
    if(!entry){slots.push({slot,consumed:false,disposition:"UNATTEMPTED"});continue;}
    const [size,ordinal]=slot.split("."),resultFile=path.join(entry.claim.evidenceRoot,"raw",`run-${ordinal.padStart(2,"0")}`,size,"result.json");
    let result=null,terminal=null;
    try{result=JSON.parse(await readFile(resultFile,"utf8"));}catch(error){if(error.code!=="ENOENT")throw error;}
    try{terminal=JSON.parse(await readFile(path.join(policy.ledgerRoot,`terminal-${slot}.json`),"utf8"));}catch(error){if(error.code!=="ENOENT")throw error;}
    slots.push({slot,consumed:true,instrumentRevision:entry.claim.instrumentRevision,methodSha256:entry.claim.methodSha256,claimSha256:entry.sha256,
      evidenceValidity:result?.collection?.evidenceValidity??"INVALID_OR_UNAVAILABLE",collectionCompleteness:result?.collection?.collectionCompleteness??"INCOMPLETE",
      originalScored:result?.scored??null,runtimeDisposition:terminal?.exitCode===0&&!terminal.signal&&!terminal.launchError?"PASSED":"FAILED_OR_UNAVAILABLE",terminal,resultFile});
  }
  return {schema:"ui-foundation.lifetime-attempt-budget-report/v1",seed:policy.seed,slots,consumed:slots.filter(s=>s.consumed).length,
    validCompleteBySize:Object.fromEntries(["1000","10000"].map(size=>[size,slots.filter(s=>s.slot.startsWith(`${size}.`)&&s.runtimeDisposition==="PASSED"&&s.evidenceValidity==="VALID"&&s.collectionCompleteness==="COMPLETE").length])),
    originalAggregateFailure:seed.collection.targetOutcome,qualificationCohort:false,comparison:"Original and successor methods remain separate; attempt budget completion is not valid-workload or metric acceptance."};
}
export async function launchContinuationSlot(policyRef,slot,receiptRef,operations={}) {
  const policy=await boundJson(policyRef),seed=await boundJson(policy.seed);validateContinuationPolicy(policy,seed);
  if(!CONTINUATION_SLOTS.includes(slot))throw new Error("consumed/unknown/sixth slot rejected");
  await mkdir(policy.ledgerRoot,{recursive:true});
  // Registry next to immutable seed prevents silently resetting the ledger path.
  const registryFile=`${await realpath(policy.seed.path)}.continuation-ledger.json`;
  const registry={seedSha256:policy.seed.sha256,cohortId:policy.cohortId,ledgerRoot:policy.ledgerRoot,policySha256:policyRef.sha256,methodSha256:policy.method.sha256,instrumentRevision:policy.instrumentRevision};
  if(operations.bindRegistry)await operations.bindRegistry(registryFile,registry);
  else {try{await writeOnce(registryFile,registry);}catch(error){if(error.code!=="EEXIST")throw error;if(!jsonEqual(JSON.parse(await readFile(registryFile,"utf8")),registry))throw new Error("approved ledger relocation/reset rejected");}}
  const claims=await continuationClaims(policy);
  if(slot!==CONTINUATION_SLOTS[claims.length])throw new Error("slot consumed or out of order");
  const previous=claims.at(-1)??{sha256:policy.seed.sha256};
  const receipt=await boundJson(receiptRef);await validateContinuationReceipt(receipt,policy,slot,previous,seed);
  const env=await (operations.verifyFiles??verifyContinuationFiles)(policy,seed);
  const evidenceRoot=policy.attemptRoots[slot];await mkdir(evidenceRoot); // exclusive new output; never reuse
  const token=createHash("sha256").update(`${policyRef.sha256}:${slot}:${Date.now()}:${process.pid}`).digest("hex");
  const claim={schema:"ui-foundation.started-slot/v1",slot,cohortId:policy.cohortId,seedSha256:policy.seed.sha256,policySha256:policyRef.sha256,
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
  if(process.argv[2]==="launch-slot") {
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
