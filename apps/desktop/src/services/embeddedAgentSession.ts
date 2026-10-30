/** Explicit workspace-owned live binding. The private execution capability is
 * kept in closures, never tool data, an event, a log, storage or a provider call.
 * No invocation, timer, listener or launcher runs at module construction. */
import { jsonTextKeepingNegativeZero } from './jsonText';
import { invokeWithIngress } from './nativeIngress';
import { EmbeddedAgentBridge, type AgentActual, type AgentBasis, type AgentCall, type AgentDelivery, type AgentScope } from './embeddedAgentBridge';
import type { PreviewModel, ModelHashEvidence, EditorOperationIntent, MechanicsResult } from '../types';
import type { OperationBatch } from './operationBatchService';
import { validateOperationBatch } from './operationBatchService';
import { overlayMayShowRow, type ResultOverlayStanding } from '../features/results/numericalHealth';

export type AgentContext = Readonly<{requestRef:string;threadId:string;turnId:string;callId:string;basis:AgentBasis;assumptions:readonly unknown[]}>;
export type AgentRunReceipt = Readonly<{jobId:string;runId:string;modelHash:ModelHashEvidence;solverMode:string;health:string}>;
export interface WorkspaceExecution {
 snapshot(): {model:PreviewModel|null;hash:ModelHashEvidence|null;revision:number;generation:number;result:MechanicsResult|null;standing:ResultOverlayStanding|null};
 register(batch:OperationBatch,context:AgentContext):Promise<string|null>;
 proposal(key:string):{status:'pending_review'|'withdrawn'|'applied'|'unknown';outcomeModelHash:ModelHashEvidence|null}|null;
 solve():Promise<AgentRunReceipt|undefined>;
 withdraw(key:string):void;
}
type Authority=AgentScope&{ownerCapability:string};
type Invoke=(request:Record<string,unknown>)=>Promise<unknown>;
type Proposal={basis:AgentBasis;key:string;context:AgentContext};
type Run={basis:AgentBasis;receipt:AgentRunReceipt};
type Proof=Record<string,unknown>;
const LIMITS={argumentBytes:16384,resultBytes:65536,pageRows:50,pendingCalls:16,proposalOperations:16};
const plain=(x:unknown):x is Record<string,unknown>=>!!x&&typeof x==='object'&&!Array.isArray(x);
function same(a:unknown,b:unknown):boolean {
 if(Object.is(a,b))return true;
 if(Array.isArray(a)&&Array.isArray(b))return a.length===b.length&&a.every((value,i)=>same(value,b[i]));
 if(!plain(a)||!plain(b))return false;
 const keys=Object.keys(a);return keys.length===Object.keys(b).length&&keys.every(key=>Object.hasOwn(b,key)&&same(a[key],b[key]));
}
const defaultInvoke:Invoke=request=>invokeWithIngress('embedded_agent_host',{request});
function opaque():string {const bytes=new Uint8Array(24);crypto.getRandomValues(bytes);return Array.from(bytes,byte=>byte.toString(16).padStart(2,'0')).join('');}
function small(value:unknown):string {const raw=jsonTextKeepingNegativeZero(value);if(new TextEncoder().encode(raw).byteLength>64*1024)throw new Error('AGENT-OUTPUT-LIMIT');return raw;}
/** Only the workspace session constructs this class with its actual getters and
 * handlers. A model tool call cannot register or replace that execution owner. */
export class EmbeddedAgentSession {
 #authority:Authority|null=null;#scope:AgentScope|null=null;#generation=-1;#bridge:EmbeddedAgentBridge|null=null;
 #actual=new Map<string,AgentActual>();#proposals=new Map<string,Proposal>();#runs=new Map<string,Run>();
 #cursors=new Map<string,{tool:string;basis:AgentBasis;offset:number;run?:string}>();
 #interrupted=new Set<string>();#seenTurns=new Set<string>();
 #starting=false;#inflight=new Set<Promise<unknown>>();
 #timer:ReturnType<typeof setTimeout>|null=null;#closed=true;#polling=false;
 constructor(private readonly owner:WorkspaceExecution,private readonly invoke:Invoke=defaultInvoke){}
 async availability():Promise<unknown>{return this.invoke({kind:'availability'});}
 async provider(request:Record<string,unknown>):Promise<unknown>{
  if(!['provider_snapshot','provider_save','provider_clear','provider_recover'].includes(String(request.kind)))throw new Error('AGENT-CONFIG-REQUEST');
  return this.invoke(request);
 }
 async start(providerRevision:string):Promise<unknown>{
  if(this.#starting)throw new Error('AGENT-START-IN-PROGRESS');
  this.#starting=true;
  try{return await this.startOwned(providerRevision);}finally{this.#starting=false;}
 }
 private async startOwned(providerRevision:string):Promise<unknown>{
  if(this.#authority)await this.close();
  const before=this.owner.snapshot();if(!before.model||!before.hash)throw new Error('AGENT-WORKSPACE-NOT-READY');
  const registered=await this.invoke({kind:'register',basis:{modelRevision:before.revision,modelHash:before.hash,stateId:null,runId:null}});
  if(!plain(registered)||typeof registered.workspaceHandle!=='string'||typeof registered.applicationGeneration!=='number'||typeof registered.ownerCapability!=='string')throw new Error('AGENT-REGISTRATION-REFUSED');
  this.#authority={workspaceHandle:registered.workspaceHandle,applicationGeneration:registered.applicationGeneration,ownerCapability:registered.ownerCapability};
  this.#scope={workspaceHandle:registered.workspaceHandle,applicationGeneration:registered.applicationGeneration};this.#generation=before.generation;
  this.#closed=false;
  const authority=this.#authority;
  this.#bridge=new EmbeddedAgentBridge(this.#scope,{
   claim:async requestRef=>{const value=await this.invoke({kind:'claim',authority,requestRef});if(!plain(value)||typeof value.raw!=='string')throw new Error('AGENT-CLAIM-REFUSED');return value.raw;},
   complete:async(requestRef,outcome)=>{
    const actual=this.#actual.get(requestRef);if(!actual||actual.outcome!==outcome)throw new Error('AGENT-OWNER-RECORD-MISSING');
    await this.invoke({kind:'publish_actual',authority,requestRef,outcome,proof:actual.record});
    const result=await this.invoke({kind:'complete',authority,requestRef,outcome});
    if(!plain(result)||!['queued','detached','lost','unconfirmed'].includes(String(result.delivery)))return 'unconfirmed';
    return result.delivery as AgentDelivery;
   }
  },{isCurrent:call=>!this.#closed&&this.owner.snapshot().generation===this.#generation&&call.basis.workspaceHandle===this.#scope?.workspaceHandle&&call.basis.applicationGeneration===this.#scope?.applicationGeneration,execute:async call=>{
   this.#seenTurns.add(call.turnId);let actual:AgentActual;
   try{actual=await this.execute(call);}catch{actual=this.failure(call,'Outcome unavailable; inspect the retained queue or job record.','execution_error');}
   this.#actual.set(call.requestRef,Object.freeze(actual));return actual;
  }});
  if(this.owner.snapshot().generation!==before.generation||this.owner.snapshot().revision!==before.revision){await this.close();throw new Error('AGENT-WORKSPACE-RETIRED');}
  try {const response=await this.invoke({kind:'start',authority,providerRevision});
   if(plain(response)&&response.status==='starting')this.schedule();else await this.close();return response;
  }catch(error){await this.close();throw error;}
 }
 private schedule(){if(this.#closed)return;this.#timer=setTimeout(()=>{void this.poll().catch(()=>{void this.close();}).finally(()=>this.schedule());},40);}
 async poll():Promise<unknown>{
  if(!this.#authority||this.#polling)return null;this.#polling=true;
  try{
   const snap=this.owner.snapshot();if(snap.generation!==this.#generation){await this.close();return null;}
   if(snap.hash)await this.invoke({kind:'basis',authority:this.#authority,basis:{modelRevision:snap.revision,modelHash:snap.hash,stateId:null,runId:null}});
   const result=await this.invoke({kind:'poll',authority:this.#authority});
   if(plain(result)&&Array.isArray(result.notices))for(const reference of result.notices){
    if(typeof reference==='string'&&this.#bridge){const task=this.#bridge.notify({requestRef:reference});this.#inflight.add(task);void task.finally(()=>this.#inflight.delete(task));}
   }
   if(plain(result)&&result.closed===true){this.#closed=true;this.#bridge?.close();}
   return result;
  }finally{this.#polling=false;}
 }
 async text(text:string){if(!this.#authority||this.#closed)throw new Error('AGENT-CLOSED');return this.invoke({kind:'text',authority:this.#authority,text});}
 async interrupt(){if(!this.#authority)return;for(const turn of this.#seenTurns)this.#interrupted.add(turn);for(const proposal of this.#proposals.values())this.owner.withdraw(proposal.key);return this.invoke({kind:'interrupt',authority:this.#authority});}
 async close(){this.#closed=true;if(this.#timer)clearTimeout(this.#timer);this.#timer=null;this.#bridge?.close();const response=this.#authority?await this.invoke({kind:'close',authority:this.#authority}):null;await Promise.allSettled([...this.#inflight]);return response;}
 actual(reference:string){return this.#actual.get(reference);}
 private current(call:AgentCall){
  const live=this.owner.snapshot();
  if(this.#closed||live.generation!==this.#generation||call.basis.workspaceHandle!==this.#scope?.workspaceHandle||call.basis.applicationGeneration!==this.#scope?.applicationGeneration)return false;
  if(call.tool==='swbpipe_observe_proposal')return same(this.#proposals.get(String(call.arguments.proposalHandle))?.basis,call.basis);
  if(call.tool==='swbpipe_read_results')return same(this.#runs.get(String(call.arguments.runHandle))?.basis,call.basis);
  return !!live.hash&&call.basis.modelRevision===live.revision&&same(call.basis.modelHash,live.hash);
 }
 private failure(call:AgentCall,message:string,status='unavailable'):AgentActual{return {outcome:small({status,basis:call.basis,message,data:null}),record:{kind:'failure'}};}
 private result(call:AgentCall,status:string,message:string,data:unknown,proof:Proof):AgentActual{return {outcome:small({status,basis:call.basis,message,data}),record:proof};}
 private offset(call:AgentCall,run?:string):number {
  const page=call.arguments.page as {cursor:string|null;limit:number};if(page.cursor===null)return 0;
  const saved=this.#cursors.get(page.cursor);if(!saved||saved.tool!==call.tool||!same(saved.basis,call.basis)||saved.run!==run)throw new Error('AGENT-CURSOR-REFUSED');return saved.offset;
 }
 private cursor(call:AgentCall,offset:number,more:boolean,run?:string):string|null {
  if(!more)return null;if(this.#cursors.size>=64)throw new Error('AGENT-CURSOR-LIMIT');const ref=opaque();this.#cursors.set(ref,{tool:call.tool,basis:call.basis,offset,run});return ref;
 }
 private async execute(call:AgentCall):Promise<AgentActual>{
  if(call.tool==='swbpipe_observe_proposal'){
   const handle=String(call.arguments.proposalHandle),record=this.#proposals.get(handle);if(!record||!same(record.basis,call.basis))return this.failure(call,'Proposal is unavailable.');
   const standing=this.owner.proposal(record.key);const status=standing?.status??'unknown';const hash=standing?.outcomeModelHash??null;
   return this.result(call,status,'Recorded proposal standing.',{proposalHandle:handle,proposalBasis:record.basis,outcomeModelHash:hash,author_type:'agent',assumptions:record.context.assumptions},{kind:'proposal',handle,basis:record.basis,status,outcomeModelHash:hash});
  }
  if(call.tool==='swbpipe_read_results')return this.readResults(call);
  if(!this.current(call))return this.failure(call,'Workspace basis changed.','stale_basis');
  const snap=this.owner.snapshot();const model=snap.model!;
  if(call.tool==='swbpipe_read_model_summary'){
   const counts:Record<string,number>={};for(const key of ['nodes','pipe_segments','supports','materials','sections','load_cases','primitive_loads','combinations','components']){
    const value=(model as unknown as Record<string,unknown>)[key];if(Array.isArray(value))counts[key]=value.length;
   }
   return this.result(call,'ok','Current admitted workspace; changes require human review.',{workspaceHandle:call.basis.workspaceHandle,projectUnits:model.project.units,objectCounts:counts,admittedOperationKinds:['modify/Node/set_field/position'],limits:LIMITS},{kind:'read'});
  }
  if(call.tool==='swbpipe_read_nodes'){
   const offset=this.offset(call),page=call.arguments.page as {limit:number};const end=Math.min(model.nodes.length,offset+page.limit);
   const nodes=[];for(let i=offset;i<end;i++){const node=model.nodes[i];if(node.id.length>160)return this.failure(call,'Node identifier exceeds this catalog page.');nodes.push({id:node.id,position:{x:node.position.x,y:node.position.y,z:node.position.z},unit:model.project.units.length});}
   return this.result(call,'ok','Bounded node page at the captured basis.',{nodes,projectUnits:model.project.units,nextCursor:this.cursor(call,end,end<model.nodes.length)},{kind:'read'});
  }
  if(call.tool==='swbpipe_propose_operations'){
   if(this.#proposals.size>=64)return this.failure(call,'Proposal retention is full.');
   const handle=opaque();const context:AgentContext=Object.freeze({requestRef:call.requestRef,threadId:call.threadId,turnId:call.turnId,callId:call.callId,basis:structuredClone(call.basis),assumptions:structuredClone(call.arguments.assumptions as unknown[])});
   const operations=(call.arguments.operations as EditorOperationIntent[]).map(operation=>({...structuredClone(operation),author_type:'agent' as const,operation_status:'proposed' as const,queue_id:handle,source:{source_ref:call.requestRef,source_channel:'embedded_agent',source_role:'agent'},validation:{schema_validation:'not_run',constraint_validation:'not_run',unit_validation:'not_run',diff_preview_status:'not_run',application_status:'not_applied'}}));
   const batch:OperationBatch={batch_id:`agent-${handle}`,operations,source_model_hash:snap.hash!.value};
   const validation=await validateOperationBatch(model,batch,snap.hash);if(!this.current(call))return this.failure(call,'Workspace changed during proposal preparation.','stale_basis');
   if(validation.validation.batch_validation_status!=='passed')return this.failure(call,'The operation engine refused the proposal; nothing was queued.');
   if(this.#interrupted.has(call.turnId))return this.failure(call,'Turn interrupted before registration.');
   const key=await this.owner.register(batch,context);if(!key)return this.failure(call,'Proposal registration was not confirmed.');
   this.#proposals.set(handle,{basis:structuredClone(call.basis),key,context});
   if(this.#interrupted.has(call.turnId)){this.owner.withdraw(key);return this.failure(call,'Turn interrupted; inspect the retained queue record.','unknown');}
   return this.result(call,'pending_review','Immutable batch registered for explicit human review.',{proposalHandle:handle,author_type:'agent'},{kind:'proposal',handle,basis:call.basis,status:'pending_review',outcomeModelHash:null});
  }
  if(call.tool==='swbpipe_solve'){
   if(this.#runs.size>=64)return this.failure(call,'Run retention is full.');
   const receipt=await this.owner.solve();if(!receipt||!same(receipt.modelHash,call.basis.modelHash))return this.failure(call,'No matching completed native solve is available.');
   const handle=opaque();this.#runs.set(handle,{basis:structuredClone(call.basis),receipt});
   return this.result(call,'completed','Native solve completed; applicability and professional acceptance remain separate.',{runHandle:handle,runId:receipt.runId,evaluatedModelHash:receipt.modelHash,health:receipt.health,solverMode:receipt.solverMode},{kind:'run',handle,runId:receipt.runId,basis:call.basis,status:'completed',nativeJobId:receipt.jobId});
  }
  return this.failure(call,'Tool is unavailable.');
 }
 private readResults(call:AgentCall):AgentActual{
  const handle=String(call.arguments.runHandle),run=this.#runs.get(handle);if(!run||!same(run.basis,call.basis))return this.failure(call,'Run is unavailable.');
  const snap=this.owner.snapshot(),result=snap.result;if(!result||result.run_id!==run.receipt.runId)return this.failure(call,'The retained run is no longer held by the result owner.');
  const offset=this.offset(call,handle),page=call.arguments.page as {limit:number},end=Math.min(result.results.length,offset+page.limit);const values=[];
  for(let i=offset;i<end;i++){
   const row=result.results[i];if([row.id,row.kind,row.entity_ref].some(v=>v.length>160)||row.unit.length>40)return this.failure(call,'Result identifiers exceed this catalog page.');
   const permitted=overlayMayShowRow(snap.standing,row)&&!result.retained_precision&&Number.isFinite(row.value);
   values.push({id:row.id,kind:row.kind,entityRef:row.entity_ref,value:permitted?row.value:null,exactDecimal:null,unit:row.unit,availability:permitted?'available':'withheld',reason:permitted?null:'The existing numerical view does not permit this binary64 value.'});
  }
  return this.result(call,'ok','Permitted bounded source-unit values; withheld values have no substitute number.',{runHandle:handle,runId:run.receipt.runId,evaluatedModelHash:run.basis.modelHash,values,nextCursor:this.cursor(call,end,end<result.results.length,handle),standing:{route:result.producer?.semantic_contract_id??'unavailable',health:run.receipt.health,checks:snap.standing?.eligible?['numerical_view_eligible']:['numerical_view_withheld']}},{kind:'run',handle,runId:run.receipt.runId,basis:run.basis,status:'completed',nativeJobId:run.receipt.jobId});
 }
}
