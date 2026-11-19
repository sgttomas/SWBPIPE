import { afterEach, describe, expect, it, vi } from 'vitest';
import { EmbeddedAgentSession, boundedFindings, type WorkspaceExecution, type AgentSolveOutcome, type AgentSolverMode } from './embeddedAgentSession';
import { agentDetailFilter } from './resultDetail';
import { PERSON_ENTERED_CHECK } from './operationAuthorship';
import type { PreviewModel, ModelHashEvidence, MechanicsResult, Diagnostic } from '../types';
import type { AgentBasis } from './embeddedAgentBridge';
import type { SessionResult } from './nativeResult';
import catalog from '../../src-tauri/resources/agent-capability-catalog.json';
import referenceJson from '../../../../fixtures/reference_system/reference_system.json';
import { canonicalJsonString } from './hashService';
const mocks=vi.hoisted(()=>({validate:vi.fn()}));
vi.mock('./operationBatchService',()=>({validateOperationBatch:mocks.validate}));
// A native read, scripted only where a test turns it on: the native rows query's page and its ordinals.
const native=vi.hoisted(()=>({on:false,rows:[] as {ordinal:number;row:unknown}[]}));
vi.mock('./nativeResult',async(importOriginal)=>{
 const actual=await importOriginal<typeof import('./nativeResult')>();
 return {...actual,
  isNativeResult:(value:unknown)=>native.on||actual.isNativeResult(value),
  nativeCustody:(value:unknown)=>native.on?{jobId:'job-1',summary:{header:{omitted_members:[]}}}:actual.nativeCustody(value as never),
  queryNativeRows:async(...args:Parameters<typeof actual.queryNativeRows>)=>native.on?{rows:native.rows,total:native.rows.length}:actual.queryNativeRows(...args)};
});
const hash={algorithm:'sha256',canonicalization:'rfc8785_jcs',payload_scope:'model_payload',payload_ref:'invented',value:'a'.repeat(64),hash_status:'computed_local_preview'} as ModelHashEvidence;
// The workspace and the engine write `sha256:` hex; the agent's catalog form is the hex alone.
const engineHash={...hash,value:`sha256:${hash.value}`} as ModelHashEvidence;
const basis:AgentBasis={workspaceHandle:'workspace-invented',applicationGeneration:1,modelRevision:1,modelHash:hash,stateId:null,runId:null};
function deferred<T>(){let resolve!:(v:T)=>void;const promise=new Promise<T>(r=>{resolve=r;});return {resolve,promise};}
// The result the session publishes for a completed solve: a completed outcome carries it, and it
// is the run's identity (with the native job), never the run id.
let heldResult:MechanicsResult|null=null;
const held=()=>heldResult as unknown as SessionResult;
function setup(workspaceHash:ModelHashEvidence=engineHash){
 heldResult=null;
 let revision=1,generation=1,tool='swbpipe_read_model_summary',args:Record<string,unknown>={},reference='invented-session:1',turn='turn-1';
 const model={schema_version:'0.9.0',project:{id:'invented',units:{length:'m'}},nodes:[{id:'node-1',label:'N1',position:{x:-0,y:1,z:2}}]} as unknown as PreviewModel;
 let result:MechanicsResult|null=null;
 const owner:WorkspaceExecution={snapshot:()=>({model,hash:workspaceHash,revision,generation,result,standing:null}),register:vi.fn(async()=> 'batch-key'),proposal:vi.fn(()=>({status:'pending_review' as const,outcomeModelHash:null})),solve:vi.fn(async(_mode?:AgentSolverMode):Promise<AgentSolveOutcome>=>({kind:'not_started',message:'Invented: nothing was started.'})),withdraw:vi.fn()};
 const sent:Record<string,unknown>[]=[];
 const invoke=vi.fn(async(value:Record<string,unknown>):Promise<unknown>=>{
  sent.push(value);
  switch(value.kind){
   case 'register':return {...basis,ownerCapability:'private-owner-capability'};
   case 'start':return {status:'starting'};
   case 'poll':return {status:'ready',notices:[reference],messages:[],closed:false,turnActive:false,turnStatus:'none'};
   case 'claim':return {raw:JSON.stringify({requestRef:reference,threadId:'thread-1',turnId:turn,callId:'call-1',basis:{...basis,modelHash:Object.fromEntries(Object.entries(hash).reverse())},tool,arguments:args})};
   case 'interrupt':return {status:'queued',turnId:turn};
   case 'publish_actual':return {recorded:true};
   case 'complete':return {delivery:'detached'};
   default:return {};
  }
 });
 const session=new EmbeddedAgentSession(owner,invoke);
 return {session,owner,invoke,sent,model,setResult:(value:MechanicsResult)=>{result=value;heldResult=value;},edit:()=>{revision++;},retire:()=>{generation++;},call:(name:string,a:Record<string,unknown>={},ref='invented-session:1',turnId='turn-1')=>{tool=name;args=a;reference=ref;turn=turnId;}};
}
afterEach(()=>{vi.useRealTimers();vi.clearAllMocks();});
async function drain(){for(let i=0;i<16;i++)await Promise.resolve();}
// A record page's own sentence, and the notice that leads its message when it left out a record.
const PAGE_SENTENCE='Bounded record page at the captured basis; before texts are exact for that basis only.';
const UNBOUND="Not returned, over this catalog's record bound: ";
const UNNAMED='Not returned, with an identifier this catalog cannot carry: ';
describe('workspace-owned live embedded agent adapter',()=>{
 it('gives the agent the catalog form of the workspace hash in its basis, updates and solve output',async()=>{
  vi.useFakeTimers();const {session,sent,owner,call}=setup();
  await session.start('revision');await session.poll();await drain();
  const bases=sent.filter(v=>v.kind==='register'||v.kind==='basis').map(v=>(v.basis as {modelHash:ModelHashEvidence}).modelHash);
  expect(sent.some(v=>v.kind==='basis')).toBe(true);expect(bases.every(item=>JSON.stringify(item)===JSON.stringify(hash))).toBe(true);
  expect(JSON.parse(session.actual('invented-session:1')!.outcome).status).toBe('ok');
  vi.mocked(owner.solve).mockResolvedValueOnce({kind:'completed',jobId:'job-1',runId:'run-1',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED',result:held()});
  call('swbpipe_solve',{},'invented-session:2');await session.poll();await drain();
  const solved=JSON.parse(session.actual('invented-session:2')!.outcome);expect(solved.status).toBe('completed');expect(solved.data.evaluatedModelHash).toEqual(hash);
  await session.close();
 });
 it('refuses a person-entered member at the top level of any operation before the engine, naming it and echoing no value',async()=>{
  const SECRET='SECRET-4.2-person-value';
  const op=(id:string,after:unknown)=>({operation_id:id,change:{after:JSON.stringify(after)}});
  const members:[string,Record<string,unknown>,RegExp][]=[
   ['modifiers',{id:'component:c',kind:'tee',modifiers:{flexibility_factor_user_value:{value:4.2,unit:'none'},source_reference:SECRET}},/modifiers \(k, SIFs, stiffness and their source\)/],
   ['modifiers on a material',{id:'material:m',label:SECRET,modifiers:null},/modifiers \(k, SIFs, stiffness and their source\)/],
   ['bend_realization',{id:'component:b',kind:'bend',bend_realization:{radius:SECRET}},/bend_realization/],
   ['curved interface',{id:'component:v',kind:'valve',label:SECRET,mechanics_interface:{solver_consumption:'curved_bend_macro_element'}},/a curved-bend mechanics_interface \(a bend realization\)/],
  ];
  let n=0;
  for(const [what,after,named] of members)for(const position of [0,2]){
   vi.useFakeTimers();const {session,owner,call}=setup();n++;
   const operations=[op('operation-1',{id:'node:a'}),op('operation-2',{id:'node:b'}),op('operation-3',{id:'node:c'})];operations[position]=op(`operation-${position+1}`,after);
   call('swbpipe_propose_operations',{operations,assumptions:[]});await session.start('revision');await session.poll();await drain();
   const outcome=JSON.parse(session.actual('invented-session:1')!.outcome);
   expect(outcome.status,`${what} at ${position}`).toBe('unavailable');expect(outcome.data).toBeNull();
   expect(outcome.message).toMatch(named);expect(outcome.message).toContain(`Operation ${position+1} (operation-${position+1})`);
   expect(outcome.message).toContain('which only the person enters. A value naming it cannot be told apart from it, even as text, so nothing was queued.');
   expect(session.actual('invented-session:1')!.outcome).not.toContain(SECRET);
   expect(mocks.validate).not.toHaveBeenCalled();expect(owner.register).not.toHaveBeenCalled();
   // The turn stays usable: the next call in the same turn is answered.
   call('swbpipe_read_model_summary',{},'invented-session:2');await session.poll();await drain();
   expect(JSON.parse(session.actual('invented-session:2')!.outcome).status).toBe('ok');
   await session.close();vi.clearAllMocks();vi.useRealTimers();
  }
  expect(n).toBe(8);
 });
 it('refuses a person-entered member at any depth, naming its path, and an after value past the walk bound, before validation',async()=>{
  const review='{"id":"combination:review-nested","label":"Review example","basis":"mechanics","terms":[{"load_case":"load:RS-SUS","factor":1,"modifiers":{"sif_user_value":{"value":2.4,"unit":"1"}}}],"provenance":"invented review example"}';
  let deep:unknown={leaf:1};for(let i=0;i<40;i++)deep=[deep];
  for(const [after,expected] of [[review,/has a value that names modifiers \(k, SIFs, stiffness and their source\) three levels inside the value, in item 1 of a list, which only the person enters\. A value naming it cannot be told apart from it, even as text, so nothing was queued\./],
   [JSON.stringify({id:'node:n',position:{x:0,y:0,z:0,bend_realization:{}}}),/has a value that names bend_realization \(a bend realization\) two levels inside the value,/],
   [JSON.stringify(deep),/carries an after value nested deeper than 32 levels, which is not checked for values only the person enters, so nothing was queued\. Propose it in smaller operations\./]] as [string,RegExp][]){
   vi.useFakeTimers();const {session,owner,call}=setup();
   call('swbpipe_propose_operations',{operations:[{operation_id:'operation-1',change:{change_kind:'create_combination',after}}],assumptions:[]});await session.start('revision');await session.poll();await drain();
   const outcome=JSON.parse(session.actual('invented-session:1')!.outcome);
   expect(outcome.status).toBe('unavailable');expect(outcome.message).toMatch(expected);expect(outcome.message).not.toContain('2.4');
   expect(mocks.validate).not.toHaveBeenCalled();expect(owner.register).not.toHaveBeenCalled();
   await session.close();vi.clearAllMocks();vi.useRealTimers();
  }
 });
 it('cannot be reached past the member-count bound or the length bound through the agent: such arguments exceed the host argument bound first',()=>{
  // The smallest after value with more members than the walk reads, and any after text longer
  // than the check reads, is far larger than the 28,672 argument bytes the host admits
  // (protocol.rs refuses larger arguments before the webview; the catalog caps an after at
  // 28,672 characters), and the bridge refuses a claim over 64 KiB; these bounds are for intake
  // and requeue (operationAuthorship.bounds.test.ts).
  const smallest=`[${'0,'.repeat(PERSON_ENTERED_CHECK.maxMembers)}0]`;
  expect(new TextEncoder().encode(smallest).byteLength).toBeGreaterThan(64*1024);
  expect(PERSON_ENTERED_CHECK.maxAfterChars).toBeGreaterThan(64*1024);
 });
 it('reads an after value at exactly the depth bound and refuses one level past it, before validation',async()=>{
  const deep=(depth:number)=>`${'['.repeat(depth)}0${']'.repeat(depth)}`;
  for(const [after,registers] of [[deep(PERSON_ENTERED_CHECK.maxDepth),true],[deep(PERSON_ENTERED_CHECK.maxDepth+1),false]] as [string,boolean][]){
   vi.useFakeTimers();const {session,owner,call}=setup();if(registers)mocks.validate.mockResolvedValueOnce({validation:{batch_validation_status:'passed'}});
   call('swbpipe_propose_operations',{operations:[{operation_id:'operation-1',change:{change_kind:'create_material',after}}],assumptions:[]});await session.start('revision');await session.poll();await drain();
   const outcome=JSON.parse(session.actual('invented-session:1')!.outcome);
   if(registers){expect(outcome.status).toBe('pending_review');expect(owner.register).toHaveBeenCalledTimes(1);}
   else{expect(outcome.status).toBe('unavailable');expect(outcome.message).toBe(`Operation 1 (operation-1) carries an after value nested deeper than ${PERSON_ENTERED_CHECK.maxDepth} levels, which is not checked for values only the person enters, so nothing was queued. Propose it in smaller operations.`);
    expect(mocks.validate).not.toHaveBeenCalled();expect(owner.register).not.toHaveBeenCalled();}
   await session.close();vi.clearAllMocks();vi.useRealTimers();
  }
 });
 it('refuses a label whose text reads as JSON naming a person-entered member, in words that claim no member was found',async()=>{
  vi.useFakeTimers();const {session,owner,call}=setup();
  call('swbpipe_propose_operations',{operations:[{operation_id:'operation-1',operation_kind:'modify',target:{object_type:'Material',ref:'material:m'},change:{change_kind:'set_field',field_path:'label',before:'M',after:'{"modifiers":{}}'}}],assumptions:[]});
  await session.start('revision');await session.poll();await drain();
  const outcome=JSON.parse(session.actual('invented-session:1')!.outcome);
  expect(outcome.status).toBe('unavailable');
  expect(outcome.message).toBe('Operation 1 (operation-1) has a value that names modifiers (k, SIFs, stiffness and their source) at the top level of the value, which only the person enters. A value naming it cannot be told apart from it, even as text, so nothing was queued. Propose it without that name and ask the person to enter the value.');
  expect(mocks.validate).not.toHaveBeenCalled();expect(owner.register).not.toHaveBeenCalled();
  await session.close();
 });
 it('reads an after given as an object, an array or a scalar, and names no key the agent chose, before validation',async()=>{
  const SECRET='Zq7SECRETAGENTTEXTxv';
  for(const [after,expected] of [
   [{id:'m',[SECRET]:{modifiers:{}}},'Operation 1 (operation-1) has a value that names modifiers (k, SIFs, stiffness and their source) two levels inside the value, which only the person enters. A value naming it cannot be told apart from it, even as text, so nothing was queued. Propose it without that name and ask the person to enter the value.'],
   [[{bend_realization:1}],'Operation 1 (operation-1) has a value that names bend_realization (a bend realization) two levels inside the value, in item 1 of a list, which only the person enters. A value naming it cannot be told apart from it, even as text, so nothing was queued. Propose it without that name and ask the person to enter the value.'],
   [{id:SECRET},'Operation 1 (operation-1) has an after value that is not text, which the operation engine does not read, so nothing was queued. Propose it with the after value as text.'],
   [7,'Operation 1 (operation-1) has an after value that is not text, which the operation engine does not read, so nothing was queued. Propose it with the after value as text.'],
   [true,'Operation 1 (operation-1) has an after value that is not text, which the operation engine does not read, so nothing was queued. Propose it with the after value as text.'],
   [null,'Operation 1 (operation-1) has an after value that is not text, which the operation engine does not read, so nothing was queued. Propose it with the after value as text.'],
   [JSON.stringify({modifiers_note:{[`modifiers_${SECRET}`]:{modifiers:1}}}),'Operation 1 (operation-1) has a value that names modifiers (k, SIFs, stiffness and their source) three levels inside the value, which only the person enters. A value naming it cannot be told apart from it, even as text, so nothing was queued. Propose it without that name and ask the person to enter the value.'],
   [JSON.stringify({mechanics_interface:{[SECRET]:SECRET,solver_consumption:'not_solver_consumed'},x:{[SECRET]:{modifiers:1}}}),'Operation 1 (operation-1) has a value that names modifiers (k, SIFs, stiffness and their source) three levels inside the value, which only the person enters. A value naming it cannot be told apart from it, even as text, so nothing was queued. Propose it without that name and ask the person to enter the value.'],
   [JSON.stringify({[SECRET]:[{[SECRET]:{modifiers:1}}]}),'Operation 1 (operation-1) has a value that names modifiers (k, SIFs, stiffness and their source) four levels inside the value, in item 1 of a list, which only the person enters. A value naming it cannot be told apart from it, even as text, so nothing was queued. Propose it without that name and ask the person to enter the value.'],
  ] as [unknown,string][]){
   vi.useFakeTimers();const {session,owner,call}=setup();
   call('swbpipe_propose_operations',{operations:[{operation_id:'operation-1',change:{change_kind:'create_material',after}}],assumptions:[]});await session.start('revision');await session.poll();await drain();
   const outcome=JSON.parse(session.actual('invented-session:1')!.outcome);
   expect(outcome.status).toBe('unavailable');expect(outcome.message).toBe(expected);
   expect(session.actual('invented-session:1')!.outcome).not.toContain(SECRET);
   expect(outcome.message).not.toContain('modifiers_note');
   expect(outcome.message).not.toContain('mechanics_interface');
   expect(mocks.validate).not.toHaveBeenCalled();expect(owner.register).not.toHaveBeenCalled();
   await session.close();vi.clearAllMocks();vi.useRealTimers();
  }
 });
 it('still registers a non-curved interface, a payload that is not JSON and one with no such member',async()=>{
  for(const after of [JSON.stringify({id:'component:v',mechanics_interface:{solver_consumption:'not_solver_consumed'}}),'not_present','{"modifiers"',JSON.stringify([1,{id:'x'}])]){
   vi.useFakeTimers();const {session,owner,call}=setup();mocks.validate.mockResolvedValueOnce({validation:{batch_validation_status:'passed'}});
   call('swbpipe_propose_operations',{operations:[{operation_id:'operation-1',change:{after}}],assumptions:[]});await session.start('revision');await session.poll();await drain();
   expect(JSON.parse(session.actual('invented-session:1')!.outcome).status,after).toBe('pending_review');expect(owner.register).toHaveBeenCalledTimes(1);
   await session.close();vi.clearAllMocks();vi.useRealTimers();
  }
 });
 it('validates the batch on the engine form and reports an applied outcome in the catalog form',async()=>{
  vi.useFakeTimers();const {session,sent,owner,call}=setup();mocks.validate.mockResolvedValueOnce({validation:{batch_validation_status:'passed'}});
  call('swbpipe_propose_operations',{operations:[{operation_id:'operation-1'}],assumptions:[]});
  await session.start('revision');await session.poll();await drain();
  const [,batch,validationHash]=mocks.validate.mock.calls[0];expect(batch.source_model_hash).toBe(engineHash.value);expect(validationHash).toEqual(engineHash);
  const handle=JSON.parse(session.actual('invented-session:1')!.outcome).data.proposalHandle;
  vi.mocked(owner.proposal).mockReturnValueOnce({status:'applied',outcomeModelHash:{...engineHash,value:`sha256:${'b'.repeat(64)}`}});
  call('swbpipe_observe_proposal',{proposalHandle:handle},'invented-session:2');await session.poll();await drain();
  const observed=JSON.parse(session.actual('invented-session:2')!.outcome);expect(observed.status).toBe('applied');expect(observed.data.outcomeModelHash.value).toBe('b'.repeat(64));
  expect((sent.filter(v=>v.kind==='publish_actual').at(-1)!.proof as {outcomeModelHash:ModelHashEvidence}).outcomeModelHash.value).toBe('b'.repeat(64));
  await session.close();
 });
 it('refuses a workspace hash in any form but sha256: and 64 lowercase hex digits',async()=>{
  for(const value of [hash.value,`sha256:${hash.value.toUpperCase()}`,`sha256:${hash.value.slice(1)}`,`SHA256:${hash.value}`]){
   const {session,invoke}=setup({...hash,value});
   await expect(session.start('revision')).rejects.toThrow('AGENT-ENGINE-HASH-FORM-REFUSED');
   expect(invoke.mock.calls.some(([request])=>request.kind==='register')).toBe(false);
  }
 });
 it('refuses an agent basis hash in any form but 64 lowercase hex digits, by name',async()=>{
  const {agentHash,catalogHash}=await import('./embeddedAgentSession');
  expect(agentHash(engineHash)).toEqual(hash);expect(catalogHash(hash)).toBe(hash);expect(catalogHash(null)).toBeNull();
  for(const value of [engineHash.value,hash.value.toUpperCase(),hash.value.slice(1)])expect(()=>catalogHash({...hash,value})).toThrow('AGENT-CATALOG-HASH-FORM-REFUSED');
 });
 it('constructs without any native call or timer',()=>{const {invoke}=setup();expect(invoke).not.toHaveBeenCalled();});
 it('keeps private authority outside tool responses and records actual before delivery',async()=>{
  vi.useFakeTimers();const {session,sent}=setup();await session.start('revision');await session.poll();await drain();
  const publish=sent.find(v=>v.kind==='publish_actual')!;expect(publish).toBeDefined();
  expect(String(publish.outcome)).not.toContain('private-owner-capability');
  expect(sent.findIndex(v=>v.kind==='publish_actual')).toBeLessThan(sent.findIndex(v=>v.kind==='complete'));
  expect(session.actual('invented-session:1')).toBeDefined();await session.close();
 });
 it('returns bounded source-unit pages with negative zero preserved',async()=>{
  vi.useFakeTimers();const {session,sent,call}=setup();call('swbpipe_read_nodes',{page:{cursor:null,limit:50}});
  await session.start('revision');await session.poll();await drain();
  const raw=sent.find(v=>v.kind==='publish_actual')!.outcome as string;
  expect(raw).toContain('"x":-0');expect(JSON.parse(raw).data.nodes).toHaveLength(1);await session.close();
 });
 it('rechecks the workspace after asynchronous validation before queue registration',async()=>{
  vi.useFakeTimers();const {session,owner,call,edit}=setup();const gate=deferred<unknown>();mocks.validate.mockReturnValueOnce(gate.promise);
  call('swbpipe_propose_operations',{operations:[{operation_id:'operation-1'}],assumptions:[]});
  await session.start('revision');await session.poll();await drain();edit();gate.resolve({validation:{batch_validation_status:'passed'}});await drain();
  expect(owner.register).not.toHaveBeenCalled();expect(JSON.parse(session.actual('invented-session:1')!.outcome).status).toBe('stale_basis');await session.close();
 });
 it('stamps proposal attribution and retains late actual after close during registration',async()=>{
  vi.useFakeTimers();const {session,owner,call}=setup();mocks.validate.mockResolvedValueOnce({validation:{batch_validation_status:'passed'}});
  const gate=deferred<string|null>();vi.mocked(owner.register).mockReturnValueOnce(gate.promise);
  call('swbpipe_propose_operations',{operations:[{operation_id:'operation-1',author_type:'human',source:{source_ref:'forged'},validation:{application_status:'applied'}}],assumptions:[]});
  await session.start('revision');await session.poll();await drain();const closing=session.close();gate.resolve('actual-batch-key');await closing;
  const [batch,context]=vi.mocked(owner.register).mock.calls[0];
  expect(batch.operations[0].author_type).toBe('agent');expect(batch.operations[0].source?.source_ref).toBe('invented-session:1');
  expect(context.threadId).toBe('thread-1');expect(JSON.parse(session.actual('invented-session:1')!.outcome).status).toBe('pending_review');
 });
 it('binds the actual solve receipt to its evaluated basis after a live edit',async()=>{
  vi.useFakeTimers();const {session,owner,call,edit,sent}=setup();const gate=deferred<AgentSolveOutcome>();vi.mocked(owner.solve).mockReturnValueOnce(gate.promise);
  call('swbpipe_solve');await session.start('revision');await session.poll();await drain();edit();
  gate.resolve({kind:'completed',jobId:'job-1',runId:'run-1',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED',result:held()});await drain();
  const actual=JSON.parse(session.actual('invented-session:1')!.outcome);expect(actual.basis.modelRevision).toBe(1);expect(actual.data.runId).toBe('run-1');
  expect(sent.find(v=>v.kind==='publish_actual')!.proof).toMatchObject({nativeJobId:'job-1',status:'completed'});await session.close();
 });
 it('tells the agent when a solve treated bends awaiting the person\'s k as straight chords',async()=>{
  vi.useFakeTimers();const {session,owner,call,model}=setup();
  (model as unknown as Record<string,unknown>).components=[{id:'component:b1',kind:'bend'},{id:'component:t1',kind:'tee'}];
  vi.mocked(owner.solve).mockResolvedValueOnce({kind:'completed',jobId:'job-1',runId:'run-1',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED',result:held()});
  call('swbpipe_solve');await session.start('revision');await session.poll();await drain();
  const message=JSON.parse(session.actual('invented-session:1')!.outcome).message;
  expect(message).toContain('component:b1');expect(message).toContain('straight chords');expect(message).toContain('k does not affect chord stiffness');expect(message).not.toContain('component:t1');
  expect(message).toContain('None has a supplied scalar SIF, so no intensified measure is published');expect(message).not.toContain('applies to');
  await session.close();
 });
 it('says a scalar SIF applies only to chords that have one',async()=>{
  vi.useFakeTimers();const {session,owner,call,model}=setup();
  (model as unknown as Record<string,unknown>).components=[{id:'component:b1',kind:'bend',modifiers:{sif_user_value:{value:1.5,unit:'none'}}},{id:'component:b2',kind:'bend',modifiers:{sif_user_value:{value:0,unit:'none'}}}];
  vi.mocked(owner.solve).mockResolvedValue({kind:'completed',jobId:'job-1',runId:'run-1',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED',result:held()});
  call('swbpipe_solve');await session.start('revision');await session.poll();await drain();
  let message:string=JSON.parse(session.actual('invented-session:1')!.outcome).message;
  expect(message).toContain('A supplied scalar SIF applies to the other chords; no scalar SIF was supplied for 1 of them (component:b2)');
  await session.close();
  (model as unknown as Record<string,unknown>).components=[{id:'component:b1',kind:'bend',modifiers:{sif_user_value:{value:1.5,unit:'none'}}}];
  call('swbpipe_solve',{},'invented-session:2');await session.start('revision');await session.poll();await drain();
  message=JSON.parse(session.actual('invented-session:2')!.outcome).message;
  expect(message).toContain('Each has a supplied scalar SIF, which applies to its chord.');await session.close();
 });
 it('keeps summary and both solve messages inside the catalog bounds for long identifiers',async()=>{
  vi.useFakeTimers();const {session,owner,call,model}=setup();
  const long=(i:number)=>`component:${String(i).padStart(3,'0')}${'x'.repeat(147)}`;
  (model as unknown as Record<string,unknown>).components=[{id:`component:${'y'.repeat(200)}`,kind:'tee'},...Array.from({length:40},(_,i)=>({id:long(i),kind:'bend'}))];
  await session.start('revision');await session.poll();await drain();
  const summary=JSON.parse(session.actual('invented-session:1')!.outcome).data.componentsAwaitingUserModifiers;
  expect(summary.count).toBe(41);expect(summary.ids).toHaveLength(32);expect(summary.ids.every((id:string)=>id.length<=160)).toBe(true);
  await session.close();
  vi.mocked(owner.solve).mockResolvedValueOnce({kind:'completed',jobId:'job-1',runId:'run-1',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED',result:held()});
  call('swbpipe_solve',{},'invented-session:2');await session.start('revision');await session.poll();await drain();
  const chord=JSON.parse(session.actual('invented-session:2')!.outcome);
  expect(chord.status).toBe('completed');expect(chord.message.length).toBeLessThanOrEqual(1000);expect(chord.message).toContain(long(0));expect(chord.message).toContain('…');
  await session.close();
  (model as unknown as Record<string,unknown>).pressure_contract={version:'3.0.0',mode:'exact_pressure_v3',shear_model:'euler_bernoulli'};
  call('swbpipe_solve',{},'invented-session:3');await session.start('revision');await session.poll();await drain();
  const exact=JSON.parse(session.actual('invented-session:3')!.outcome);
  expect(exact.status).toBe('unavailable');expect(exact.message.length).toBeLessThanOrEqual(1000);expect(exact.message).toContain('…');
  await session.close();
 });
 it('refuses to solve on the exact route while a bend is unrealized, naming it',async()=>{
  vi.useFakeTimers();const {session,owner,call,model}=setup();
  Object.assign(model as unknown as Record<string,unknown>,{pressure_contract:{version:'3.0.0',mode:'exact_pressure_v3',shear_model:'euler_bernoulli'},components:[{id:'component:b1',kind:'bend'},{id:'component:b2',kind:'bend',mechanics_interface:{solver_consumption:'curved_bend_macro_element'}}]});
  call('swbpipe_solve');await session.start('revision');await session.poll();await drain();
  const outcome=JSON.parse(session.actual('invented-session:1')!.outcome);
  expect(outcome.status).toBe('unavailable');expect(outcome.message).toContain('component:b1');expect(outcome.message).not.toContain('component:b2');
  expect(owner.solve).not.toHaveBeenCalled();await session.close();
 });
 it('never retries a repeated notice and closes on project replacement',async()=>{
  vi.useFakeTimers();const {session,owner,retire,call}=setup();call('swbpipe_solve');await session.start('revision');await session.poll();await drain();await session.poll();await drain();
  expect(owner.solve).toHaveBeenCalledOnce();retire();await session.poll();await expect(session.text('invented')).rejects.toThrow('AGENT-CLOSED');
 });
 it('interrupts only the current native turn without withdrawing an older pending proposal',async()=>{
  vi.useFakeTimers();const {session,owner,call}=setup();mocks.validate.mockResolvedValue({validation:{batch_validation_status:'passed'}});
  vi.mocked(owner.register).mockResolvedValueOnce('first-key').mockResolvedValueOnce('second-key');
  call('swbpipe_propose_operations',{operations:[{operation_id:'operation-1'}],assumptions:[]});
  await session.start('revision');await session.poll();await drain();
  call('swbpipe_propose_operations',{operations:[{operation_id:'operation-2'}],assumptions:[]},'invented-session:2','turn-2');
  await session.poll();await drain();await session.interrupt();
  expect(owner.withdraw).toHaveBeenCalledExactlyOnceWith('second-key');await session.close();
 });

 it('does not let a poll answered after Stop reopen the closed view',async()=>{
  vi.useFakeTimers();const {session,invoke}=setup();const baseline=invoke.getMockImplementation()!;
  const late=deferred<unknown>();
  invoke.mockImplementation(async request=>request.kind==='poll'?late.promise:baseline(request));
  let last:import('./embeddedAgentSession').AgentSessionView|undefined;const remove=session.subscribe(view=>{last=view;});
  await session.start('revision');
  const pending=session.poll();await drain();
  await session.close();
  expect(last).toMatchObject({closed:true,turnActive:false});
  late.resolve({status:'ready',notices:[],messages:[{itemId:'late',kind:'final',text:'late'}],closed:false,turnActive:true,turnStatus:'in_progress'});
  expect(await pending).toBeNull();
  expect(last).toMatchObject({closed:true,turnActive:false});expect(last!.messages).toEqual([]);
  remove();
 });
 it('uses final message standing to replace deltas and keeps transcript eviction separate',async()=>{
  vi.useFakeTimers();const {session,invoke}=setup();const baseline=invoke.getMockImplementation()!;
  let frame:Record<string,unknown>={status:'ready',closed:false,notices:[],messages:[]};
  invoke.mockImplementation(async request=>request.kind==='poll'?frame:baseline(request));
  let last:import('./embeddedAgentSession').AgentSessionView|undefined;const remove=session.subscribe(view=>{last=view;});
  await session.start('revision');
  for(const [kind,text] of [['delta','Hello'],['delta',' model'],['final','Hello model']]){
   frame={...frame,messages:[{itemId:'answer',kind,text}]};await session.poll();
  }
  expect(last!.messages.map(message=>message.text)).toEqual(['Hello model']);
  frame={...frame,messages:[{itemId:'large',kind:'final',text:'x'.repeat(65000)}]};await session.poll();
  frame={...frame,messages:[{itemId:'new',kind:'final',text:'y'.repeat(1000)}]};await session.poll();
  expect(last!.truncated).toBe(true);expect(last!.messages).toHaveLength(1);remove();await session.close();
 });

 it('reports the admitted kinds, project identity and the exact pressure-profile before-text',async()=>{
  vi.useFakeTimers();const {session,sent,model}=setup();await session.start('revision');await session.poll();await drain();
  let data=JSON.parse(sent.filter(v=>v.kind==='publish_actual').at(-1)!.outcome as string).data;
  expect(data).toMatchObject({projectId:'invented',schemaVersion:'0.9.0',pressureContract:null,pressureProfileBefore:'{"schema_version":"0.9.0"}'});
  expect(data.admittedOperationKinds).toContain('insert/Component/insert_component_symbol');expect(data.admittedOperationKinds).toHaveLength(41);expect(data.admittedOperationKinds).toEqual(catalog.definitions.summaryData.properties.admittedOperationKinds.items.enum);expect(data.componentsAwaitingUserModifiers).toEqual({count:0,ids:[]});
  await session.close();
  (model as unknown as Record<string,unknown>).pressure_contract={version:'3.0.0',mode:'exact_pressure_v3',shear_model:'euler_bernoulli'};
  await session.start('revision');await session.poll();await drain();
  data=JSON.parse(sent.filter(v=>v.kind==='publish_actual').at(-1)!.outcome as string).data;
  expect(data.pressureProfileBefore).toBe('{"pressure_contract":{"mode":"exact_pressure_v3","shear_model":"euler_bernoulli","version":"3.0.0"},"schema_version":"0.9.0"}');
  await session.close();
 });
 it('pages records with exact before texts and binds the cursor to its family',async()=>{
  vi.useFakeTimers();const {session,call,model}=setup();
  (model as unknown as Record<string,unknown>).supports=[{id:'support:a',label:' Anchor A ',node:'node-1',restraints:['UX'],family:'anchor',provenance:'p'},{id:'support:b',label:'Guide B',node:'node-1',restraints:['UZ'],provenance:'p'}];
  call('swbpipe_read_records',{family:'supports',page:{cursor:null,limit:1}});await session.start('revision');await session.poll();await drain();
  const first=JSON.parse(session.actual('invented-session:1')!.outcome).data;
  expect(first.records).toHaveLength(1);expect(first.records[0].id).toBe('support:a');
  expect(first.records[0].before.delete).toBe('Anchor A');
  expect(first.records[0].before.configuration).toBe('{"family":"anchor","provenance":"p","restraints":["UX"]}');
  expect(first.nextCursor).toEqual(expect.any(String));
  call('swbpipe_read_records',{family:'supports',page:{cursor:first.nextCursor,limit:1}},'invented-session:2');await session.poll();await drain();
  const second=JSON.parse(session.actual('invented-session:2')!.outcome).data;expect(second.records[0].id).toBe('support:b');expect(second.nextCursor).toBeNull();
  call('swbpipe_read_records',{family:'nodes',page:{cursor:first.nextCursor,limit:1}},'invented-session:3');await session.poll();await drain();
  expect(JSON.parse(session.actual('invented-session:3')!.outcome).status).toBe('execution_error');
  call('swbpipe_read_records',{family:'nodes',page:{cursor:null,limit:5}},'invented-session:4');await session.poll();await drain();
  const nodes=JSON.parse(session.actual('invented-session:4')!.outcome).data;expect(Object.keys(nodes.records[0].before)).toEqual(['delete']);expect(nodes.records[0].before.delete).toBe('N1; x=0; y=1; z=2');
  await session.close();
 });
 it('bounds record pages in UTF-8 bytes and leaves out, never cuts, an oversized entry',async()=>{
  vi.useFakeTimers();const {session,call,model}=setup();
  const records=model as unknown as Record<string,unknown>;
  // 10000 three-byte characters: under 28672 UTF-16 units, over 28672 bytes.
  records.supports=[{id:'support:wide',label:'€'.repeat(10000),node:'node-1',restraints:['UX'],provenance:'p'}];
  call('swbpipe_read_records',{family:'supports',page:{cursor:null,limit:5}});await session.start('revision');await session.poll();await drain();
  let outcome=JSON.parse(session.actual('invented-session:1')!.outcome);expect(outcome.status).toBe('ok');expect(outcome.data.records).toEqual([]);expect(outcome.data.nextCursor).toBeNull();
  expect(outcome.message).toBe(`${UNBOUND}"support:wide". ${PAGE_SENTENCE}`);
  // Two records of about 30 KB each: the page holds one and resumes at the next.
  records.supports=[0,1].map(i=>({id:`support:${i}`,label:'€'.repeat(5000),node:'node-1',restraints:['UX'],provenance:'p'}));
  call('swbpipe_read_records',{family:'supports',page:{cursor:null,limit:5}},'invented-session:2');await session.poll();await drain();
  outcome=JSON.parse(session.actual('invented-session:2')!.outcome);expect(outcome.data.records.map((r:{id:string})=>r.id)).toEqual(['support:0']);expect(outcome.data.nextCursor).toEqual(expect.any(String));
  expect(outcome.data.records[0].before.delete).toBe('€'.repeat(5000));
  // Each text fits the record text bound, but escaping makes the single entry larger than a page.
  records.supports=[{id:'support:quoted',label:'"'.repeat(3900),node:'node-1',restraints:['UX'],provenance:'"'.repeat(3900)}];
  call('swbpipe_read_records',{family:'supports',page:{cursor:null,limit:5}},'invented-session:3');await session.poll();await drain();
  outcome=JSON.parse(session.actual('invented-session:3')!.outcome);expect(outcome.status).toBe('ok');expect(outcome.data.records).toEqual([]);expect(outcome.data.nextCursor).toBeNull();
  expect(outcome.message).toBe(`${UNBOUND}"support:quoted". ${PAGE_SENTENCE}`);
  await session.close();
 });
 it('reports the catalog limits and reads back a stored state up to the argument bound, one byte over by name',async()=>{
  // One figure bounds a text the agent writes (the argument bytes, a change's before and after)
  // and the same text read back: a state the agent could enter and not read could never be removed,
  // since its removal names the stored state as its before.
  vi.useFakeTimers();const {session,call,model}=setup();
  const bound=catalog.limits.argumentBytes;expect(bound).toBe(28672);
  await session.start('revision');await session.poll();await drain();
  expect(JSON.parse(session.actual('invented-session:1')!.outcome).data.limits).toEqual(catalog.limits);
  const records=model as unknown as Record<string,unknown>;
  // {"provenance":"…"} is 17 bytes around its text.
  const loadCase=(fill:number)=>[{id:'load:L1',label:'L1',kind:'sustained',status:'active',primitive_loads:[],analysis_state:{provenance:'x'.repeat(fill)}}];
  records.load_cases=loadCase(bound-17);
  call('swbpipe_read_records',{family:'load_cases',page:{cursor:null,limit:5}},'invented-session:2');await session.poll();await drain();
  let outcome=JSON.parse(session.actual('invented-session:2')!.outcome);expect(outcome.status).toBe('ok');
  expect(new TextEncoder().encode(outcome.data.records[0].before.analysis_state).byteLength).toBe(bound);
  records.load_cases=loadCase(bound-16);
  call('swbpipe_read_records',{family:'load_cases',page:{cursor:null,limit:5}},'invented-session:3');await session.poll();await drain();
  outcome=JSON.parse(session.actual('invented-session:3')!.outcome);expect(outcome.status).toBe('ok');expect(outcome.data.records).toEqual([]);
  expect(outcome.message).toBe(`${UNBOUND}"load:L1". ${PAGE_SENTENCE}`);
  await session.close();
 });
 it('reads the reference system\'s load cases back whole across pages, each state and its pressure regions as stored',async()=>{
  // A read-back that succeeds: the committed reference system's own load cases, through the
  // session's pager. Each entry carries the case's analysis state and pressure regions as before
  // texts; a page holds what fits and the cursor resumes at the next case.
  vi.useFakeTimers();const {session,call,model}=setup();
  const settle=async()=>{for(let i=0;i<8;i++)await drain();};
  const cases=structuredClone((referenceJson as unknown as {load_cases:Record<string,any>[]}).load_cases);
  expect(cases.length).toBeGreaterThanOrEqual(3);
  (model as unknown as Record<string,unknown>).load_cases=cases;await session.start('revision');await session.poll();await settle();
  const seen:Record<string,any>[]=[];let cursor:string|null=null,pages=0;
  for(;pages<cases.length;){
   call('swbpipe_read_records',{family:'load_cases',page:{cursor,limit:5}},`invented-session:${pages+2}`);await session.poll();await settle();
   const outcome=JSON.parse(session.actual(`invented-session:${pages+2}`)!.outcome);expect(outcome.status).toBe('ok');pages++;
   seen.push(...outcome.data.records);cursor=outcome.data.nextCursor;if(cursor===null)break;
  }
  expect(cursor).toBeNull();expect(pages).toBeGreaterThan(1);
  // Every case exactly once, in the document's order.
  expect(seen.map(record=>record.id)).toEqual(cases.map(item=>item.id));
  for(const [index,record] of seen.entries()){
   expect(record.before.analysis_state).toBe(await canonicalJsonString(cases[index].analysis_state));
   expect(record.before.pressure_regions).toBe(await canonicalJsonString(cases[index].pressure_regions));
  }
  await session.close();
 });
 it('returns the ordinary cases before and after a case whose two held texts each fit and together exceed a record page, each once, names that case as not returned and resumes past it at the same basis (issue 401); that case itself stays unread (issue 403)',async()=>{
  // The record is installed directly, as a person's edit in the desktop can leave it. Its two
  // texts are also of a size the agent's own proposals carry: the reach test
  // (agent_catalog_reach.rs) writes an analysis state and pressure regions of 25,000 canonical
  // bytes each in separate proposals within the argument bound. Each text is within the record
  // text bound; the entry is over the 48 KiB record page and is never cut. The read leaves the
  // wide case out, names it first in its message and returns the ordinary cases on either side of
  // it, each the same entry it returns when read without the wide case. The wide case itself is
  // still not read: what the agent can write is not always what it can read (issue 403).
  vi.useFakeTimers();const {session,call,model}=setup();
  const settle=async()=>{for(let i=0;i<8;i++)await drain();};
  const bytes=(text:string)=>new TextEncoder().encode(text).byteLength;
  // {"provenance":"…"} is 17 bytes around its text; [{"id":"region:wide","provenance":"…"}] is 38.
  const ordinary=(name:string)=>({id:`load:${name}`,label:name,kind:'primitive_user_load',status:'draft',provenance:'invented',primitive_loads:[],analysis_state:{provenance:'invented'},pressure_regions:[]});
  const wide={id:'load:wide',label:'wide',kind:'primitive_user_load',status:'draft',provenance:'invented',primitive_loads:[],
   analysis_state:{provenance:'s'.repeat(25000-17)},pressure_regions:[{id:'region:wide',provenance:'r'.repeat(25000-38)}]};
  expect(bytes(await canonicalJsonString(wide.analysis_state))).toBe(25000);expect(bytes(await canonicalJsonString(wide.pressure_regions))).toBe(25000);
  expect(25000).toBeLessThanOrEqual(catalog.definitions.record.properties.before.additionalProperties.maxLength);
  (model as unknown as Record<string,unknown>).load_cases=[ordinary('before'),wide,ordinary('after')];await session.start('revision');await session.poll();await settle();
  let n=1;const read=async(cursor:string|null,limit:number)=>{n++;call('swbpipe_read_records',{family:'load_cases',page:{cursor,limit}},`invented-session:${n}`);await session.poll();await settle();return JSON.parse(session.actual(`invented-session:${n}`)!.outcome);};
  const idsOf=(page:{data:{records:{id:string}[]}})=>page.data.records.map(record=>record.id);
  // A page of two rows examines the first ordinary case and the wide one; its cursor resumes past the wide case.
  const first=await read(null,2);
  expect(first.status).toBe('ok');expect(first.message).toBe(`${UNBOUND}"load:wide". ${PAGE_SENTENCE}`);expect(idsOf(first)).toEqual(['load:before']);expect(first.data.nextCursor).toEqual(expect.any(String));
  const second=await read(first.data.nextCursor,2);
  expect(second.status).toBe('ok');expect(second.message).toBe(PAGE_SENTENCE);expect(idsOf(second)).toEqual(['load:after']);expect(second.data.nextCursor).toBeNull();
  expect(second.basis).toEqual(first.basis);
  // One page of five rows returns both and names the wide case between them.
  const whole=await read(null,5);
  expect(whole.status).toBe('ok');expect(whole.message).toBe(`${UNBOUND}"load:wide". ${PAGE_SENTENCE}`);expect(idsOf(whole)).toEqual(['load:before','load:after']);expect(whole.data.nextCursor).toBeNull();
  // Without the wide case the two ordinary cases are read as the same entries.
  (model as unknown as Record<string,unknown>).load_cases=[ordinary('before'),ordinary('after')];
  const alone=await read(null,5);
  expect(alone.status).toBe('ok');expect(alone.message).toBe(PAGE_SENTENCE);expect(idsOf(alone)).toEqual(['load:before','load:after']);
  expect([...first.data.records,...second.data.records]).toEqual(alone.data.records);expect(whole.data.records).toEqual(alone.data.records);
  await session.close();
 });
 describe('a record the page cannot carry is left out by name and the page continues past it (issue 401)',()=>{
  // A support whose label (its delete before text) is 10000 three-byte characters is over the
  // record text bound; an ordinary support is well inside a page.
  const wide=(id:string)=>({id,label:'€'.repeat(10000),node:'node-1',restraints:['UX'],provenance:'p'});
  const fits=(id:string)=>({id,label:id,node:'node-1',restraints:['UX'],provenance:'p'});
  type Page={status:string;basis:AgentBasis;message:string;data:{records:{id:string}[];nextCursor:string|null}};
  /** A session over these supports, and a reader that issues one read call per page. */
  async function reader(supports:unknown[]){
   vi.useFakeTimers();const {session,call,model}=setup();
   (model as unknown as Record<string,unknown>).supports=supports;
   const settle=async()=>{for(let i=0;i<8;i++)await drain();};
   await session.start('revision');await session.poll();await settle();let n=1;
   const read=async(cursor:string|null,limit:number):Promise<Page>=>{
    n++;call('swbpipe_read_records',{family:'supports',page:{cursor,limit}},`invented-session:${n}`);await session.poll();await settle();
    const raw=session.actual(`invented-session:${n}`)!.outcome;
    expect(new TextEncoder().encode(raw).byteLength).toBeLessThanOrEqual(catalog.limits.resultBytes);
    const page=JSON.parse(raw) as Page;expect(page.status).toBe('ok');expect(page.basis).toEqual(basis);
    expect([...page.message].length).toBeLessThanOrEqual(catalog.definitions.recordsOutput.properties.message.maxLength);
    return page;
   };
   return {session,read};
  }
  const ids=(page:Page)=>page.data.records.map(record=>record.id);
  /** The records a page's message names as not returned for size, in the order named. */
  const named=(page:Page):string[]=>{
   if(!page.message.startsWith(UNBOUND))return [];
   // The names are JSON strings separated by ", "; the list ends at the first "." outside a string.
   const rest=page.message.slice(UNBOUND.length);let end=0,inside=false;
   for(;end<rest.length;end++){const c=rest[end];if(inside){if(c==='\\')end++;else if(c==='"')inside=false;}else if(c==='"')inside=true;else if(c==='.')break;}
   return JSON.parse(`[${rest.slice(0,end)}]`) as string[];
  };
  it('returns both ordinary cases beside an oversized one once, names it first, and the cursor resumes past it at the same basis',async()=>{
   const {session,read}=await reader([fits('support:a'),wide('support:W'),fits('support:b')]);
   // A page of two rows: the ordinary record and the oversized one, which is named; the cursor follows it.
   const first=await read(null,2);
   expect(ids(first)).toEqual(['support:a']);expect(first.message).toBe(`${UNBOUND}"support:W". ${PAGE_SENTENCE}`);expect(first.data.nextCursor).toEqual(expect.any(String));
   const second=await read(first.data.nextCursor,2);
   expect(ids(second)).toEqual(['support:b']);expect(second.message).toBe(PAGE_SENTENCE);expect(second.data.nextCursor).toBeNull();
   expect(second.basis).toEqual(first.basis);
   // One page of five rows: both ordinary records and the oversized one named between them.
   const whole=await read(null,5);
   expect(ids(whole)).toEqual(['support:a','support:b']);expect(named(whole)).toEqual(['support:W']);expect(whole.message.startsWith(UNBOUND)).toBe(true);expect(whole.data.nextCursor).toBeNull();
   await session.close();
  });
  it('leads the message with the notice whenever something is left out, and leaves it exactly the page sentence when nothing is',async()=>{
   const {session,read}=await reader([fits('support:a'),fits('support:b')]);
   const page=await read(null,5);expect(ids(page)).toEqual(['support:a','support:b']);expect(page.message).toBe(PAGE_SENTENCE);
   await session.close();
   const left=await reader([fits('support:a'),{id:'',label:'x'},wide('support:W')]);
   const both=await left.read(null,5);
   expect(both.message).toBe(`${UNBOUND}"support:W". ${UNNAMED}1 record. ${PAGE_SENTENCE}`);
   await left.session.close();
  });
  it('names every record left out for size: the page stops before a name that would not fit and the cursor resumes at it',async()=>{
   // Identifiers of 160 characters, the catalog's most: only a few fit one message.
   const long=(i:number)=>`support:${'w'.repeat(151)}${i}`;expect(long(0)).toHaveLength(160);
   const rows=[fits('support:o0'),wide(long(0)),wide(long(1)),fits('support:o1'),wide(long(2)),wide(long(3)),wide(long(4)),wide(long(5)),fits('support:o2'),wide(long(6)),wide(long(7)),wide(long(8)),fits('support:o3')];
   const all=rows.map(row=>row.id);
   const {session,read}=await reader(rows);
   const pages:Page[]=[];let cursor:string|null=null;
   do{const page=await read(cursor,50);pages.push(page);cursor=page.data.nextCursor;}while(cursor!==null&&pages.length<rows.length);
   expect(cursor).toBeNull();expect(pages.length).toBeGreaterThan(1);
   // The first page names fewer than all nine, and returns exactly the ordinary records before the first unnamed one.
   const firstNamed=named(pages[0]);
   // Exactly as many names as the message has room for, no fewer: each is 162 characters as a JSON
   // string and ", " apart, beside the notice's opening, its full stop, a count clause reserved for
   // the page's limit of 50 and the page sentence, in 1000 characters.
   const room=catalog.definitions.recordsOutput.properties.message.maxLength-(UNBOUND.length+1+1+UNNAMED.length+'50 records.'.length+1+PAGE_SENTENCE.length);
   expect(firstNamed.length).toBe(Math.floor((room+2)/(162+2)));expect(firstNamed.length).toBe(4);
   const resume=all.indexOf(long(firstNamed.length));
   expect(ids(pages[0])).toEqual(all.slice(0,resume).filter(id=>id.length<160));
   expect(named(pages[1])[0]).toBe(long(firstNamed.length));
   // Across the pages every record is examined exactly once, in order: returned or named, never both, never lost.
   const seen=pages.flatMap(page=>[...ids(page),...named(page)]);
   expect(new Set(seen).size).toBe(seen.length);expect([...seen].sort()).toEqual([...all].sort());
   expect(pages.flatMap(named)).toEqual(all.filter(id=>id.length===160));
   expect(pages.flatMap(ids)).toEqual(all.filter(id=>id.length<160));
   await session.close();
  });
  it('leaves out an oversized record first on a page, last on a page, and two on one page',async()=>{
   // First on a later page: that page carries nothing but the notice, and the cursor still advances.
   let {session,read}=await reader([fits('support:a'),wide('support:W'),fits('support:b')]);
   const one=await read(null,1);expect(ids(one)).toEqual(['support:a']);expect(one.message).toBe(PAGE_SENTENCE);
   const two=await read(one.data.nextCursor,1);expect(ids(two)).toEqual([]);expect(two.message).toBe(`${UNBOUND}"support:W". ${PAGE_SENTENCE}`);expect(two.data.nextCursor).toEqual(expect.any(String));
   const three=await read(two.data.nextCursor,1);expect(ids(three)).toEqual(['support:b']);expect(three.message).toBe(PAGE_SENTENCE);expect(three.data.nextCursor).toBeNull();
   await session.close();
   // First on the first page, then last on it.
   ({session,read}=await reader([wide('support:W'),fits('support:a')]));
   let page=await read(null,5);expect(ids(page)).toEqual(['support:a']);expect(named(page)).toEqual(['support:W']);expect(page.data.nextCursor).toBeNull();
   await session.close();
   ({session,read}=await reader([fits('support:a'),wide('support:W')]));
   page=await read(null,5);expect(ids(page)).toEqual(['support:a']);expect(named(page)).toEqual(['support:W']);expect(page.data.nextCursor).toBeNull();
   await session.close();
   // Two on one page, named in their order.
   ({session,read}=await reader([fits('support:a'),wide('support:W1'),fits('support:b'),wide('support:W2'),fits('support:c')]));
   page=await read(null,5);expect(ids(page)).toEqual(['support:a','support:b','support:c']);expect(page.message).toBe(`${UNBOUND}"support:W1", "support:W2". ${PAGE_SENTENCE}`);expect(page.data.nextCursor).toBeNull();
   await session.close();
  });
  it('names each record as a JSON string, so that one name holding a comma is not read as two records',async()=>{
   // A model identifier may hold a comma, a quote or a line break. One record named "support:a, support:b"
   // and two records named "support:a" and "support:b" are told apart, and each name reads back exactly.
   let {session,read}=await reader([wide('support:a, support:b'),fits('support:c')]);
   const one=await read(null,5);expect(ids(one)).toEqual(['support:c']);
   expect(one.message).toBe(`${UNBOUND}"support:a, support:b". ${PAGE_SENTENCE}`);expect(named(one)).toEqual(['support:a, support:b']);
   await session.close();
   ({session,read}=await reader([wide('support:a'),wide('support:b'),fits('support:c')]));
   const two=await read(null,5);expect(ids(two)).toEqual(['support:c']);
   expect(two.message).toBe(`${UNBOUND}"support:a", "support:b". ${PAGE_SENTENCE}`);expect(named(two)).toEqual(['support:a','support:b']);
   expect(two.message).not.toBe(one.message);
   await session.close();
   // A quote, a backslash, a line break and a sentence end inside a name stay inside its string.
   const awkward=['support:"q", "r"','support:back\\slash','support:line\nbreak. Bounded record page','support:é. Not returned'];
   ({session,read}=await reader([...awkward.map(wide),fits('support:c')]));
   const page=await read(null,50);expect(ids(page)).toEqual(['support:c']);expect(named(page)).toEqual(awkward);
   expect(page.message).toBe(`${UNBOUND}${awkward.map(id=>JSON.stringify(id)).join(', ')}. ${PAGE_SENTENCE}`);expect(page.message).not.toContain('\n');
   await session.close();
  });
  it('names the longest name a notice can carry alone, counts one whose JSON string cannot fit even alone, and always advances',async()=>{
   // 160 quotes are 322 characters as a JSON string: named, alone on a fresh page. 160 control
   // characters are 962 as a JSON string, more than a notice has room for with its count and the
   // page sentence: that record is counted like one whose identifier the catalog cannot carry, so
   // the page examines it and moves on instead of stopping at it for ever.
   const quotes='"'.repeat(160),controls='\u0001'.repeat(160);
   expect(JSON.stringify(quotes)).toHaveLength(322);expect(JSON.stringify(controls)).toHaveLength(962);
   let {session,read}=await reader([wide(quotes),fits('support:a')]);
   let page=await read(null,1);expect(ids(page)).toEqual([]);expect(named(page)).toEqual([quotes]);expect(page.data.nextCursor).toEqual(expect.any(String));
   page=await read(page.data.nextCursor,1);expect(ids(page)).toEqual(['support:a']);expect(page.data.nextCursor).toBeNull();
   await session.close();
   ({session,read}=await reader([wide(controls),fits('support:a')]));
   page=await read(null,1);expect(ids(page)).toEqual([]);expect(page.message).toBe(`${UNNAMED}1 record. ${PAGE_SENTENCE}`);expect(page.data.nextCursor).toEqual(expect.any(String));
   page=await read(page.data.nextCursor,1);expect(ids(page)).toEqual(['support:a']);expect(page.message).toBe(PAGE_SENTENCE);expect(page.data.nextCursor).toBeNull();
   await session.close();
  });
  it('counts, without a name, a record whose identifier the catalog cannot carry, and continues past it',async()=>{
   // Empty, over 160 characters, not a string, and not well-formed text.
   let {session,read}=await reader([{id:'',label:'e'},fits('support:a'),{id:'x'.repeat(161),label:'l'},{id:42,label:'n'},wide('support:W'),{id:'\ud800x',label:'s'},fits('support:b')]);
   let page=await read(null,50);
   expect(ids(page)).toEqual(['support:a','support:b']);expect(page.message).toBe(`${UNBOUND}"support:W". ${UNNAMED}4 records. ${PAGE_SENTENCE}`);expect(page.data.nextCursor).toBeNull();
   await session.close();
   // The rows counted take their place in the page's limit, and the cursor resumes after them.
   ({session,read}=await reader([{id:'',label:'e'},{id:42,label:'n'},fits('support:a')]));
   page=await read(null,2);expect(ids(page)).toEqual([]);expect(page.message).toBe(`${UNNAMED}2 records. ${PAGE_SENTENCE}`);expect(page.data.nextCursor).toEqual(expect.any(String));
   page=await read(page.data.nextCursor,2);expect(ids(page)).toEqual(['support:a']);expect(page.message).toBe(PAGE_SENTENCE);expect(page.data.nextCursor).toBeNull();
   await session.close();
  });
 });
 it('filters result pages by kind and entity and binds the filter to the cursor',async()=>{
  vi.useFakeTimers();const {session,owner,call,sent,setResult}=setup();
  const rows=Array.from({length:60},(_,i)=>({id:`r${i}`,kind:i%2?'support_reaction_component_v2':'global_nodal_displacement_x',entity_ref:i%4<2?'support:a':'node:c',unit:'N',value:i}));
  setResult({run_id:'run-1',results:rows} as unknown as MechanicsResult);
  vi.mocked(owner.solve).mockResolvedValueOnce({kind:'completed',jobId:'job-1',runId:'run-1',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED',result:held()});
  call('swbpipe_solve');await session.start('revision');await session.poll();await drain();
  const runHandle=JSON.parse(session.actual('invented-session:1')!.outcome).data.runHandle;
  const filter={kinds:['support_reaction_component_v2'],entityRefs:['support:a']};
  call('swbpipe_read_results',{runHandle,page:{cursor:null,limit:50},filter},'invented-session:2');await session.poll();await drain();
  const page=JSON.parse(session.actual('invented-session:2')!.outcome).data;
  expect(page.values.map((v:{id:string})=>v.id)).toEqual(rows.filter(r=>r.kind==='support_reaction_component_v2'&&r.entity_ref==='support:a').map(r=>r.id));
  expect(page.nextCursor).toBeNull();
  call('swbpipe_read_results',{runHandle,page:{cursor:null,limit:10}},'invented-session:3');await session.poll();await drain();
  const first=JSON.parse(session.actual('invented-session:3')!.outcome).data;expect(first.values).toHaveLength(10);expect(first.nextCursor).toEqual(expect.any(String));
  call('swbpipe_read_results',{runHandle,page:{cursor:first.nextCursor,limit:10},filter},'invented-session:4');await session.poll();await drain();
  expect(JSON.parse(session.actual('invented-session:4')!.outcome).status).toBe('execution_error');
  expect(sent.some(v=>v.kind==='complete')).toBe(true);await session.close();
 });

});

describe('the solve tool reports what the solve route says happened (scripted owner outcomes)',()=>{
 const finding=(i:number,extra:Partial<Diagnostic>={}):Diagnostic=>({code:`INVENTED_FINDING_${i}`,severity:'blocking',message:`Invented finding ${i}.`,affected_refs:[`node:${i}`],...extra});
 const outcome=(session:EmbeddedAgentSession,ref='invented-session:1')=>JSON.parse(session.actual(ref)!.outcome);
 const proof=(sent:Record<string,unknown>[])=>sent.filter(v=>v.kind==='publish_actual').at(-1)!.proof as Record<string,unknown>;
 it('reports a refused solve as rejected with its bounded findings and no run, never as completed',async()=>{
  vi.useFakeTimers();const {session,owner,call,sent}=setup();
  const diagnostics=[finding(0,{message:'m'.repeat(900),affected_refs:Array.from({length:10},(_,i)=>`node:${i}`)}),finding(1,{severity:'fatal' as Diagnostic['severity']}),...Array.from({length:18},(_,i)=>finding(i+2))];
  vi.mocked(owner.solve).mockResolvedValueOnce({kind:'rejected',jobId:'job-1',modelHash:engineHash,solverMode:'sparse_interactive',health:'MODEL_INCOMPLETE',diagnostics,notListed:3});
  call('swbpipe_solve');await session.start('revision');await session.poll();await drain();
  const refused=outcome(session);
  expect(refused.status).toBe('rejected');expect(refused.message).not.toMatch(/completed/i);expect(refused.message).toContain('Not solved');
  expect(refused.data).toMatchObject({runHandle:null,runId:null,evaluatedModelHash:hash,health:'MODEL_INCOMPLETE',failure:null});
  expect(refused.data.diagnostics).toHaveLength(16);
  // The finding with a severity outside the catalog is counted, not altered: 3 + 20 - 16.
  expect(refused.data.diagnosticsNotListed).toBe(7);expect(refused.data.diagnostics.map((d:{code:string})=>d.code)).not.toContain('INVENTED_FINDING_1');
  expect(Array.from(refused.data.diagnostics[0].message as string)).toHaveLength(500);expect(refused.data.diagnostics[0].affectedRefs).toHaveLength(8);
  expect(proof(sent)).toMatchObject({kind:'run',handle:null,runId:null,status:'rejected',nativeJobId:'job-1'});
  // No run is retained: there is nothing to read.
  call('swbpipe_read_results',{runHandle:'r'.repeat(48),page:{cursor:null,limit:5}},'invented-session:2');await session.poll();await drain();
  expect(outcome(session,'invented-session:2').status).toBe('unavailable');expect(owner.solve).toHaveBeenCalledOnce();
  await session.close();
 });
 it('reports a refusal before any job (a solve gate) in the same form, bound to the call while the model is unchanged',async()=>{
  vi.useFakeTimers();const {session,owner,call,sent}=setup();
  vi.mocked(owner.solve).mockResolvedValueOnce({kind:'rejected',jobId:null,modelHash:null,solverMode:'sparse_interactive',health:'MODEL_INCOMPLETE',diagnostics:[finding(0)],notListed:2});
  call('swbpipe_solve');await session.start('revision');await session.poll();await drain();
  const refused=outcome(session);
  expect(refused.status).toBe('rejected');expect(refused.data).toMatchObject({evaluatedModelHash:hash,diagnosticsNotListed:2});
  expect(proof(sent)).toMatchObject({status:'rejected',nativeJobId:null,handle:null});
  await session.close();
 });
 it('reports a failed solve with its kind and bounded message, no run and no job claim',async()=>{
  vi.useFakeTimers();const {session,owner,call,sent}=setup();
  vi.mocked(owner.solve).mockResolvedValueOnce({kind:'failed',failure:'result_too_large',message:'x'.repeat(800),modelHash:engineHash,solverMode:'sparse_interactive'});
  call('swbpipe_solve');await session.start('revision');await session.poll();await drain();
  const failed=outcome(session);
  expect(failed.status).toBe('failed');expect(failed.message).not.toMatch(/completed/i);
  expect(failed.data).toMatchObject({runHandle:null,runId:null,diagnostics:[],diagnosticsNotListed:0,failure:{kind:'result_too_large'}});
  expect(Array.from(failed.data.failure.message as string)).toHaveLength(500);
  expect(proof(sent)).toMatchObject({kind:'run',handle:null,runId:null,status:'failed',nativeJobId:null});
  await session.close();
 });
 it('reports nothing started as unavailable and an outcome for another model state as stale',async()=>{
  vi.useFakeTimers();const {session,owner,call}=setup();
  call('swbpipe_solve');await session.start('revision');await session.poll();await drain();
  expect(outcome(session).status).toBe('unavailable');
  vi.mocked(owner.solve).mockResolvedValueOnce({kind:'failed',failure:'job_failed',message:'Invented.',modelHash:{...engineHash,value:`sha256:${'c'.repeat(64)}`},solverMode:'sparse_interactive'});
  call('swbpipe_solve',{},'invented-session:2');await session.poll();await drain();
  expect(outcome(session,'invented-session:2').status).toBe('stale_basis');
  await session.close();
 });
 it('solves in the mode the call names, and reports the mode that ran',async()=>{
  vi.useFakeTimers();const {session,owner,call}=setup();
  vi.mocked(owner.solve).mockImplementation(async(mode)=>({kind:'completed',jobId:'job-1',runId:'run-1',modelHash:engineHash,solverMode:mode??'sparse_interactive',health:'MECHANICS_SOLVED',result:held()}));
  call('swbpipe_solve',{solverMode:'dense_scrutiny'});await session.start('revision');await session.poll();await drain();
  expect(vi.mocked(owner.solve).mock.calls[0]).toEqual(['dense_scrutiny']);expect(outcome(session).data.solverMode).toBe('dense_scrutiny');
  call('swbpipe_solve',{},'invented-session:2');await session.poll();await drain();
  expect(vi.mocked(owner.solve).mock.calls[1]).toEqual([undefined]);expect(outcome(session,'invented-session:2').data.solverMode).toBe('sparse_interactive');
  // A route that ran another mode than the one named is not reported as the named mode's run.
  vi.mocked(owner.solve).mockResolvedValueOnce({kind:'completed',jobId:'job-1',runId:'run-1',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED',result:held()});
  call('swbpipe_solve',{solverMode:'dense_scrutiny'},'invented-session:3');await session.poll();await drain();
  expect(outcome(session,'invented-session:3').status).toBe('unknown');
  await session.close();
 });
});

describe('result pages know their load case or combination',()=>{
 const row=(i:number,ref:{ref_type:string;ref_id:string}|null,extra:Record<string,unknown>={})=>({id:`r${i}`,kind:'global_nodal_displacement_x',entity_ref:'node:c',unit:'mm',value:i,basis_ref:ref,metadata:{component:'ux',coordinate_system:'global',location:'node',basis:'b',sign_convention:'positive_global'},...extra});
 const sustained={ref_type:'load_case',ref_id:'lc-sustained'},expansion={ref_type:'load_case',ref_id:'lc-expansion'},combo={ref_type:'combination',ref_id:'lc-sustained'};
 async function solved(rows:unknown[]){
  vi.useFakeTimers();const harness=setup();harness.setResult({run_id:'run-1',results:rows} as unknown as MechanicsResult);
  vi.mocked(harness.owner.solve).mockResolvedValueOnce({kind:'completed',jobId:'job-1',runId:'run-1',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED',result:held()});
  harness.call('swbpipe_solve');await harness.session.start('revision');await harness.session.poll();await drain();
  return {...harness,runHandle:JSON.parse(harness.session.actual('invented-session:1')!.outcome).data.runHandle as string};
 }
 const read=async(h:Awaited<ReturnType<typeof solved>>,ref:string,args:Record<string,unknown>)=>{h.call('swbpipe_read_results',{runHandle:h.runHandle,...args},ref);await h.session.poll();await drain();return JSON.parse(h.session.actual(ref)!.outcome);};
 it('selects one load case or combination by type and id, and names each value\'s case and metadata',async()=>{
  const h=await solved([row(0,sustained),row(1,expansion),row(2,combo),row(3,null),row(4,sustained,{metadata:undefined})]);
  const page=await read(h,'invented-session:2',{page:{cursor:null,limit:50},filter:{kinds:[],entityRefs:[],case:{refType:'load_case',refId:'lc-sustained'}}});
  expect(page.data.values.map((v:{id:string})=>v.id)).toEqual(['r0','r4']);
  expect(page.data.values[0]).toMatchObject({case:{refType:'load_case',refId:'lc-sustained'},component:'ux',coordinateSystem:'global',location:'node',signConvention:'positive_global'});
  expect(page.data.values[1]).toMatchObject({case:{refType:'load_case',refId:'lc-sustained'},component:null,coordinateSystem:null,location:null,signConvention:null});
  // The combination sharing the load case's id stays apart.
  const combination=await read(h,'invented-session:3',{page:{cursor:null,limit:50},filter:{kinds:[],entityRefs:[],case:{refType:'combination',refId:'lc-sustained'}}});
  expect(combination.data.values.map((v:{id:string})=>v.id)).toEqual(['r2']);
  const all=await read(h,'invented-session:4',{page:{cursor:null,limit:50}});
  expect(all.data.values.map((v:{case:unknown})=>v.case)).toEqual([{refType:'load_case',refId:'lc-sustained'},{refType:'load_case',refId:'lc-expansion'},{refType:'combination',refId:'lc-sustained'},null,{refType:'load_case',refId:'lc-sustained'}]);
  await h.session.close();
 });
 it('binds the selected case to the cursor',async()=>{
  const h=await solved(Array.from({length:6},(_,i)=>row(i,i%2?expansion:sustained)));
  const first=await read(h,'invented-session:2',{page:{cursor:null,limit:2},filter:{kinds:[],entityRefs:[],case:{refType:'load_case',refId:'lc-sustained'}}});
  expect(first.data.values.map((v:{id:string})=>v.id)).toEqual(['r0','r2']);expect(first.data.nextCursor).toEqual(expect.any(String));
  const other=await read(h,'invented-session:3',{page:{cursor:first.data.nextCursor,limit:2},filter:{kinds:[],entityRefs:[],case:{refType:'load_case',refId:'lc-expansion'}}});
  expect(other.status).toBe('execution_error');
  const next=await read(h,'invented-session:4',{page:{cursor:first.data.nextCursor,limit:2},filter:{kinds:[],entityRefs:[],case:{refType:'load_case',refId:'lc-sustained'}}});
  expect(next.data.values.map((v:{id:string})=>v.id)).toEqual(['r4']);expect(next.data.nextCursor).toBeNull();
  await h.session.close();
 });
 it('never cuts a case or metadata member, and stops a page before the result byte cap',async()=>{
  let h=await solved([row(0,sustained,{metadata:{location:'l'.repeat(161)}})]);
  expect((await read(h,'invented-session:2',{page:{cursor:null,limit:5}})).status).toBe('unavailable');await h.session.close();
  const wide=(i:number)=>row(i,{ref_type:'load_case',ref_id:`${i}`.padEnd(160,'c')},{id:`r${i}`.padEnd(160,'i'),kind:'k'.repeat(160),entity_ref:'e'.repeat(160),metadata:{component:'c'.repeat(160),coordinate_system:'s'.repeat(160),location:'l'.repeat(160),sign_convention:'g'.repeat(160)}});
  h=await solved(Array.from({length:50},(_,i)=>wide(i)));
  const page=await read(h,'invented-session:2',{page:{cursor:null,limit:50}});
  expect(page.status).toBe('ok');expect(page.data.values.length).toBeLessThan(50);expect(page.data.nextCursor).toEqual(expect.any(String));
  expect(new TextEncoder().encode(h.session.actual('invented-session:2')!.outcome).byteLength).toBeLessThanOrEqual(65536);
  // Following the cursor reaches every row once, each page inside the cap.
  let seen=page.data.values.length,cursor=page.data.nextCursor,n=3;
  while(cursor){const ref=`invented-session:${n++}`;const next=await read(h,ref,{page:{cursor,limit:50}});
   expect(next.status).toBe('ok');expect(new TextEncoder().encode(h.session.actual(ref)!.outcome).byteLength).toBeLessThanOrEqual(65536);
   seen+=next.data.values.length;cursor=next.data.nextCursor;}
  expect(seen).toBe(50);
  await h.session.close();
 });
 it('cuts a page by its rows\' text as the host does: no more than 32 KiB of row text, at least one row',async()=>{
  // Four rows of about 12 KiB of text each, limit 4: the page holds two (a third would pass 32 KiB).
  const h=await solved(Array.from({length:4},(_,i)=>row(i,sustained,{note:'p'.repeat(12*1024)})));
  const first=await read(h,'invented-session:2',{page:{cursor:null,limit:4}});
  expect(first.data.values.map((v:{id:string})=>v.id)).toEqual(['r0','r1']);expect(first.data.nextCursor).toEqual(expect.any(String));
  // One row larger than the bound still makes a page of one.
  await h.session.close();
  const g=await solved([row(0,sustained,{note:'p'.repeat(40*1024)}),row(1,sustained)]);
  const single=await read(g,'invented-session:2',{page:{cursor:null,limit:4}});
  expect(single.data.values.map((v:{id:string})=>v.id)).toEqual(['r0']);
  await g.session.close();
 });
 it('states the selected case to the native query as the results panel keys it',()=>{
  expect(agentDetailFilter({kinds:['k'],entityRefs:[]})).toEqual({kinds:['k'],entities:[]});
  expect(agentDetailFilter({kinds:[],entityRefs:['node:c'],case:{refType:'combination',refId:'c-1'}})).toEqual({kinds:[],entities:['node:c'],case:'combination:c-1'});
 });
 it('lists the model\'s load case and combination ids in the summary, bounded',async()=>{
  vi.useFakeTimers();const {session,model}=setup();
  Object.assign(model as unknown as Record<string,unknown>,{load_cases:Array.from({length:40},(_,i)=>({id:`lc-${i}`})),combinations:[{id:'c-1'},{id:'x'.repeat(161)}]});
  await session.start('revision');await session.poll();await drain();
  const data=JSON.parse(session.actual('invented-session:1')!.outcome).data;
  expect(data.loadCases.count).toBe(40);expect(data.loadCases.ids).toHaveLength(32);expect(data.combinations).toEqual({count:2,ids:['c-1']});
  await session.close();
 });
});

describe('a refusal stays informative inside the result cap',()=>{
 const outcomeOf=(session:EmbeddedAgentSession,ref='invented-session:1')=>JSON.parse(session.actual(ref)!.outcome);
 const loneSurrogate=/[\ud800-\udbff](?![\udc00-\udfff])|(?<![\ud800-\udbff])[\udc00-\udfff]/;
 async function refuse(rejected:Partial<Extract<AgentSolveOutcome,{kind:'rejected'}>>){
  vi.useFakeTimers();const harness=setup();
  vi.mocked(harness.owner.solve).mockResolvedValueOnce({kind:'rejected',jobId:'job-1',modelHash:engineHash,solverMode:'sparse_interactive',health:'MODEL_INCOMPLETE',diagnostics:[],notListed:0,...rejected});
  harness.call('swbpipe_solve');await harness.session.start('revision');await harness.session.poll();await drain();
  return harness;
 }
 it('lists as many findings as fit a refusal larger than the cap and counts the rest, never an execution error',async()=>{
  const wide=(i:number):Diagnostic=>({code:`INVENTED_WIDE_${i}`,severity:'blocking',message:'€'.repeat(500),affected_refs:Array.from({length:8},(_,j)=>`${i}:${j}:`.padEnd(160,'€'))});
  const {session}=await refuse({diagnostics:Array.from({length:20},(_,i)=>wide(i)),notListed:5});
  const raw=session.actual('invented-session:1')!.outcome,refused=JSON.parse(raw);
  expect(refused.status).toBe('rejected');expect(new TextEncoder().encode(raw).byteLength).toBeLessThanOrEqual(65536);
  const listed=refused.data.diagnostics.length;expect(listed).toBeGreaterThan(0);expect(listed).toBeLessThan(16);
  expect(refused.data.diagnosticsNotListed).toBe(5+20-listed);
  expect(refused.message).toContain(`25 findings, ${5+20-listed} not listed`);
  await session.close();
 });
 it('cuts a multi-megabyte message to a bounded outcome and never ends a cut in half a surrogate pair',async()=>{
  const huge='😀'.repeat(2_000_000);
  const {session,owner,call}=await refuse({diagnostics:[{code:'INVENTED_HUGE',severity:'error',message:huge,affected_refs:['x'.repeat(5_000_000),'node:a']}],message:`a${huge}`});
  const refused=outcomeOf(session);
  const text:string=refused.data.diagnostics[0].message;
  expect(Array.from(text)).toHaveLength(500);expect(text.endsWith('😀…')).toBe(true);expect(loneSurrogate.test(text)).toBe(false);
  expect(refused.data.diagnostics[0].affectedRefs).toEqual(['node:a']);
  expect(Array.from(refused.message as string).length).toBeLessThanOrEqual(1000);expect(loneSurrogate.test(refused.message)).toBe(false);
  // An odd cut position: the prefix before the ellipsis is whole characters.
  vi.mocked(owner.solve).mockResolvedValueOnce({kind:'failed',failure:'error',message:`ab${'😀'.repeat(1_000_000)}`,modelHash:engineHash,solverMode:'sparse_interactive'});
  call('swbpipe_solve',{},'invented-session:2');await session.poll();await drain();
  const failed:string=outcomeOf(session,'invented-session:2').data.failure.message;
  expect(Array.from(failed)).toHaveLength(500);expect(loneSurrogate.test(failed)).toBe(false);expect(failed.startsWith('ab😀')).toBe(true);
  await session.close();
 });
 it('says how many findings and how many are not listed, with the refusal\'s own text when it gives one',async()=>{
  const {session}=await refuse({diagnostics:[{code:'INVENTED_A',severity:'blocking',message:'Invented.'}],notListed:1,message:'Invented refusal text.'});
  const refused=outcomeOf(session);
  expect(refused.message).toContain('2 findings, 1 not listed');expect(refused.message).toContain('Invented refusal text.');
  await session.close();
 });
 it('carries a refusal\'s text when it gives no findings, and invents none',async()=>{
  const {session}=await refuse({jobId:null,modelHash:null,message:'The model could not be read.'});
  const refused=outcomeOf(session);
  expect(refused.status).toBe('rejected');expect(refused.message).toContain('Not solved: the solve was refused. The model could not be read.');
  expect(refused.message).not.toMatch(/finding/);expect(refused.data.diagnostics).toEqual([]);expect(refused.data.diagnosticsNotListed).toBe(0);
  await session.close();
 });
 it('says plainly that no reason was reported when a refusal has neither findings nor text',async()=>{
  const {session}=await refuse({});
  const refused=outcomeOf(session);
  expect(refused.status).toBe('rejected');expect(refused.message).toContain('Not solved: the solve was refused. No reason was reported.');
  expect(refused.data.diagnostics).toEqual([]);
  await session.close();
 });
 it('lists summary case and combination ids within their byte budget, counting all',async()=>{
  vi.useFakeTimers();const {session,model}=setup();
  Object.assign(model as unknown as Record<string,unknown>,{load_cases:Array.from({length:40},(_,i)=>({id:`${i}:`.padEnd(160,'€')})),combinations:[]});
  await session.start('revision');await session.poll();await drain();
  const data=JSON.parse(session.actual('invented-session:1')!.outcome).data;
  expect(data.loadCases.count).toBe(40);expect(data.loadCases.ids.length).toBeLessThan(32);
  expect(new TextEncoder().encode(JSON.stringify(data.loadCases.ids)).byteLength).toBeLessThanOrEqual(8*1024+64);
  await session.close();
 });
});

describe('a run handle reads only its own solve\'s result',()=>{
 it('refuses a handle whose result was replaced by a later solve under the same run id',async()=>{
  vi.useFakeTimers();const {session,owner,call,setResult}=setup();
  const rows=(tag:string)=>[{id:`r-${tag}`,kind:'global_nodal_displacement_x',entity_ref:'node:c',unit:'mm',value:1}];
  const first={run_id:'run:preview-linear-static-001',results:rows('a')} as unknown as MechanicsResult;
  setResult(first);
  vi.mocked(owner.solve).mockResolvedValueOnce({kind:'completed',jobId:'job-a',runId:'run:preview-linear-static-001',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED',result:held()});
  call('swbpipe_solve');await session.start('revision');await session.poll();await drain();
  const a=JSON.parse(session.actual('invented-session:1')!.outcome).data.runHandle;
  // A second solve publishes another result under the producer's one run id.
  const second={run_id:'run:preview-linear-static-001',results:rows('b')} as unknown as MechanicsResult;
  setResult(second);
  vi.mocked(owner.solve).mockResolvedValueOnce({kind:'completed',jobId:'job-b',runId:'run:preview-linear-static-001',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED',result:held()});
  call('swbpipe_solve',{},'invented-session:2');await session.poll();await drain();
  const b=JSON.parse(session.actual('invented-session:2')!.outcome).data.runHandle;
  call('swbpipe_read_results',{runHandle:a,page:{cursor:null,limit:5}},'invented-session:3');await session.poll();await drain();
  const stale=JSON.parse(session.actual('invented-session:3')!.outcome);
  expect(stale.status).toBe('unavailable');expect(stale.data).toBeNull();
  call('swbpipe_read_results',{runHandle:b,page:{cursor:null,limit:5}},'invented-session:4');await session.poll();await drain();
  const current=JSON.parse(session.actual('invented-session:4')!.outcome);
  expect(current.status).toBe('ok');expect(current.data.values.map((v:{id:string})=>v.id)).toEqual(['r-b']);
  await session.close();
 });
});

describe('the byte budgets count the encoded array, brackets and commas included',()=>{
 const bytes=(value:unknown)=>new TextEncoder().encode(JSON.stringify(value)).byteLength;
 const finding=(message:string,ref:string):Diagnostic=>({code:'CC',severity:'blocking',message,affected_refs:[ref]});
 it('lists fifteen of the reviewer\'s sixteen findings whose array would be 32,785 bytes, and all sixteen at exactly 32,768',()=>{
  const wide=finding('€'.repeat(500),'€'.repeat(160));
  const sixteen=Array.from({length:16},()=>wide);
  const all=boundedFindings(sixteen,0);
  expect(bytes(boundedFindings([wide],0).diagnostics[0])).toBe(2048);
  expect(all.diagnostics).toHaveLength(15);expect(all.notListed).toBe(1);expect(bytes(all.diagnostics)).toBeLessThanOrEqual(32768);
  // The sixteenth shortened so the whole array is exactly 32,768 bytes: then all fit, and one byte more does not.
  // The listed form's bytes with an empty reference text (its key is affectedRefs).
  const fixed=bytes({code:'CC',severity:'blocking',message:'€'.repeat(500),affectedRefs:['']});
  const room=32768-(2+15*2048+15)-fixed;
  const exact=finding('€'.repeat(500),'€'.repeat(Math.floor(room/3))+'a'.repeat(room%3));
  const atLimit=[...Array.from({length:15},()=>wide),exact];
  expect(bytes(boundedFindings(atLimit,0).diagnostics)).toBe(32768);
  expect(boundedFindings(atLimit,0).diagnostics).toHaveLength(16);
  const over=[...Array.from({length:15},()=>wide),finding(exact.message,`${exact.affected_refs![0]}a`)];
  expect(boundedFindings(over,0)).toMatchObject({notListed:1});expect(boundedFindings(over,0).diagnostics).toHaveLength(15);
 });
 it('lists fifteen of the reviewer\'s sixteen ids whose array would be 8,193 bytes, and all sixteen at exactly 8,192',async()=>{
  vi.useFakeTimers();const {session,model}=setup();
  // The reviewer's id: 29 emoji and 131 euro signs, 160 characters, 511 bytes with its quotes.
  const reviewer=`${'😀'.repeat(29)}${'€'.repeat(131)}`;
  const tooMany=Array.from({length:16},()=>reviewer);
  expect(bytes(tooMany)).toBe(8193);
  // One emoji fewer and one euro sign more in one id (160 characters, one byte less): exactly 8,192.
  const atLimit=[...tooMany.slice(0,15),`${'😀'.repeat(28)}${'€'.repeat(132)}`];
  expect(bytes(atLimit)).toBe(8192);
  Object.assign(model as unknown as Record<string,unknown>,{load_cases:tooMany.map(value=>({id:value})),combinations:atLimit.map(value=>({id:value}))});
  await session.start('revision');await session.poll();await drain();
  const data=JSON.parse(session.actual('invented-session:1')!.outcome).data;
  expect(data.loadCases.count).toBe(16);expect(data.loadCases.ids).toHaveLength(15);expect(bytes(data.loadCases.ids)).toBeLessThanOrEqual(8192);
  expect(data.combinations.count).toBe(16);expect(data.combinations.ids).toHaveLength(16);expect(bytes(data.combinations.ids)).toBe(8192);
  await session.close();
 });
});

describe('a native result read tells the host which rows its values are',()=>{
 afterEach(()=>{native.on=false;native.rows=[];});
 it('carries each value\'s native row ordinal in the read\'s proof, in page order',async()=>{
  vi.useFakeTimers();const {session,owner,call,sent,setResult}=setup();
  setResult({run_id:'run:preview-linear-static-001'} as unknown as MechanicsResult);
  vi.mocked(owner.solve).mockResolvedValueOnce({kind:'completed',jobId:'job-1',runId:'run:preview-linear-static-001',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED',result:held()});
  call('swbpipe_solve');await session.start('revision');await session.poll();await drain();
  const runHandle=JSON.parse(session.actual('invented-session:1')!.outcome).data.runHandle;
  native.on=true;
  native.rows=[7,9].map(ordinal=>({ordinal,row:{id:`r${ordinal}`,kind:'k',entity_ref:'node:a',unit:'mm',value:1}}));
  call('swbpipe_read_results',{runHandle,page:{cursor:null,limit:5}},'invented-session:2');await session.poll();await drain();
  expect(JSON.parse(session.actual('invented-session:2')!.outcome).status).toBe('ok');
  expect(sent.filter(v=>v.kind==='publish_actual').at(-1)!.proof).toMatchObject({kind:'run',handle:runHandle,nativeJobId:'job-1',ordinals:[7,9]});
  await session.close();
 });
});
