import { afterEach, describe, expect, it, vi } from 'vitest';
import { EmbeddedAgentSession, type WorkspaceExecution, type AgentSolveOutcome, type AgentSolverMode } from './embeddedAgentSession';
import { agentDetailFilter } from './resultDetail';
import type { PreviewModel, ModelHashEvidence, MechanicsResult, Diagnostic } from '../types';
import type { AgentBasis } from './embeddedAgentBridge';
const mocks=vi.hoisted(()=>({validate:vi.fn()}));
vi.mock('./operationBatchService',()=>({validateOperationBatch:mocks.validate}));
const hash={algorithm:'sha256',canonicalization:'rfc8785_jcs',payload_scope:'model_payload',payload_ref:'invented',value:'a'.repeat(64),hash_status:'computed_local_preview'} as ModelHashEvidence;
// The workspace and the engine write `sha256:` hex; the agent's catalog form is the hex alone.
const engineHash={...hash,value:`sha256:${hash.value}`} as ModelHashEvidence;
const basis:AgentBasis={workspaceHandle:'workspace-invented',applicationGeneration:1,modelRevision:1,modelHash:hash,stateId:null,runId:null};
function deferred<T>(){let resolve!:(v:T)=>void;const promise=new Promise<T>(r=>{resolve=r;});return {resolve,promise};}
function setup(workspaceHash:ModelHashEvidence=engineHash){
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
 return {session,owner,invoke,sent,model,setResult:(value:MechanicsResult)=>{result=value;},edit:()=>{revision++;},retire:()=>{generation++;},call:(name:string,a:Record<string,unknown>={},ref='invented-session:1',turnId='turn-1')=>{tool=name;args=a;reference=ref;turn=turnId;}};
}
afterEach(()=>{vi.useRealTimers();vi.clearAllMocks();});
async function drain(){for(let i=0;i<16;i++)await Promise.resolve();}
describe('workspace-owned live embedded agent adapter',()=>{
 it('gives the agent the catalog form of the workspace hash in its basis, updates and solve output',async()=>{
  vi.useFakeTimers();const {session,sent,owner,call}=setup();
  await session.start('revision');await session.poll();await drain();
  const bases=sent.filter(v=>v.kind==='register'||v.kind==='basis').map(v=>(v.basis as {modelHash:ModelHashEvidence}).modelHash);
  expect(sent.some(v=>v.kind==='basis')).toBe(true);expect(bases.every(item=>JSON.stringify(item)===JSON.stringify(hash))).toBe(true);
  expect(JSON.parse(session.actual('invented-session:1')!.outcome).status).toBe('ok');
  vi.mocked(owner.solve).mockResolvedValueOnce({kind:'completed',jobId:'job-1',runId:'run-1',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED'});
  call('swbpipe_solve',{},'invented-session:2');await session.poll();await drain();
  const solved=JSON.parse(session.actual('invented-session:2')!.outcome);expect(solved.status).toBe('completed');expect(solved.data.evaluatedModelHash).toEqual(hash);
  await session.close();
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
  gate.resolve({kind:'completed',jobId:'job-1',runId:'run-1',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED'});await drain();
  const actual=JSON.parse(session.actual('invented-session:1')!.outcome);expect(actual.basis.modelRevision).toBe(1);expect(actual.data.runId).toBe('run-1');
  expect(sent.find(v=>v.kind==='publish_actual')!.proof).toMatchObject({nativeJobId:'job-1',status:'completed'});await session.close();
 });
 it('tells the agent when a solve treated bends awaiting the person\'s k as straight chords',async()=>{
  vi.useFakeTimers();const {session,owner,call,model}=setup();
  (model as unknown as Record<string,unknown>).components=[{id:'component:b1',kind:'bend'},{id:'component:t1',kind:'tee'}];
  vi.mocked(owner.solve).mockResolvedValueOnce({kind:'completed',jobId:'job-1',runId:'run-1',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED'});
  call('swbpipe_solve');await session.start('revision');await session.poll();await drain();
  const message=JSON.parse(session.actual('invented-session:1')!.outcome).message;
  expect(message).toContain('component:b1');expect(message).toContain('straight chords');expect(message).toContain('k does not affect chord stiffness');expect(message).not.toContain('component:t1');
  expect(message).toContain('None has a supplied scalar SIF, so no intensified measure is published');expect(message).not.toContain('applies to');
  await session.close();
 });
 it('says a scalar SIF applies only to chords that have one',async()=>{
  vi.useFakeTimers();const {session,owner,call,model}=setup();
  (model as unknown as Record<string,unknown>).components=[{id:'component:b1',kind:'bend',modifiers:{sif_user_value:{value:1.5,unit:'none'}}},{id:'component:b2',kind:'bend',modifiers:{sif_user_value:{value:0,unit:'none'}}}];
  vi.mocked(owner.solve).mockResolvedValue({kind:'completed',jobId:'job-1',runId:'run-1',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED'});
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
  vi.mocked(owner.solve).mockResolvedValueOnce({kind:'completed',jobId:'job-1',runId:'run-1',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED'});
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
  expect(data.admittedOperationKinds).toContain('insert/Component/insert_component_symbol');expect(data.admittedOperationKinds).toHaveLength(16);expect(data.componentsAwaitingUserModifiers).toEqual({count:0,ids:[]});
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
 it('bounds record pages in UTF-8 bytes and never cuts an oversized first entry',async()=>{
  vi.useFakeTimers();const {session,call,model}=setup();
  const records=model as unknown as Record<string,unknown>;
  // 6000 three-byte characters: under 16384 UTF-16 units, over 16384 bytes.
  records.supports=[{id:'support:wide',label:'€'.repeat(6000),node:'node-1',restraints:['UX'],provenance:'p'}];
  call('swbpipe_read_records',{family:'supports',page:{cursor:null,limit:5}});await session.start('revision');await session.poll();await drain();
  let outcome=JSON.parse(session.actual('invented-session:1')!.outcome);expect(outcome.status).toBe('unavailable');expect(outcome.message).toContain('support:wide');
  // Two records of about 30 KB each: the page holds one and resumes at the next.
  records.supports=[0,1].map(i=>({id:`support:${i}`,label:'€'.repeat(5000),node:'node-1',restraints:['UX'],provenance:'p'}));
  call('swbpipe_read_records',{family:'supports',page:{cursor:null,limit:5}},'invented-session:2');await session.poll();await drain();
  outcome=JSON.parse(session.actual('invented-session:2')!.outcome);expect(outcome.data.records.map((r:{id:string})=>r.id)).toEqual(['support:0']);expect(outcome.data.nextCursor).toEqual(expect.any(String));
  expect(outcome.data.records[0].before.delete).toBe('€'.repeat(5000));
  // Each text fits 16 KiB, but escaping makes the single entry larger than a page.
  records.supports=[{id:'support:quoted',label:'"'.repeat(3900),node:'node-1',restraints:['UX'],provenance:'"'.repeat(3900)}];
  call('swbpipe_read_records',{family:'supports',page:{cursor:null,limit:5}},'invented-session:3');await session.poll();await drain();
  outcome=JSON.parse(session.actual('invented-session:3')!.outcome);expect(outcome.status).toBe('unavailable');expect(outcome.message).toContain('support:quoted');
  await session.close();
 });
 it('filters result pages by kind and entity and binds the filter to the cursor',async()=>{
  vi.useFakeTimers();const {session,owner,call,sent,setResult}=setup();
  const rows=Array.from({length:60},(_,i)=>({id:`r${i}`,kind:i%2?'support_reaction_component_v2':'global_nodal_displacement_x',entity_ref:i%4<2?'support:a':'node:c',unit:'N',value:i}));
  setResult({run_id:'run-1',results:rows} as unknown as MechanicsResult);
  vi.mocked(owner.solve).mockResolvedValueOnce({kind:'completed',jobId:'job-1',runId:'run-1',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED'});
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
  vi.mocked(owner.solve).mockImplementation(async(mode)=>({kind:'completed',jobId:'job-1',runId:'run-1',modelHash:engineHash,solverMode:mode??'sparse_interactive',health:'MECHANICS_SOLVED'}));
  call('swbpipe_solve',{solverMode:'dense_scrutiny'});await session.start('revision');await session.poll();await drain();
  expect(vi.mocked(owner.solve).mock.calls[0]).toEqual(['dense_scrutiny']);expect(outcome(session).data.solverMode).toBe('dense_scrutiny');
  call('swbpipe_solve',{},'invented-session:2');await session.poll();await drain();
  expect(vi.mocked(owner.solve).mock.calls[1]).toEqual([undefined]);expect(outcome(session,'invented-session:2').data.solverMode).toBe('sparse_interactive');
  // A route that ran another mode than the one named is not reported as the named mode's run.
  vi.mocked(owner.solve).mockResolvedValueOnce({kind:'completed',jobId:'job-1',runId:'run-1',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED'});
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
  vi.mocked(harness.owner.solve).mockResolvedValueOnce({kind:'completed',jobId:'job-1',runId:'run-1',modelHash:engineHash,solverMode:'sparse_interactive',health:'MECHANICS_SOLVED'});
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
  const rest=await read(h,'invented-session:3',{page:{cursor:page.data.nextCursor,limit:50}});
  expect(page.data.values.length+rest.data.values.length).toBe(50);
  await h.session.close();
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
