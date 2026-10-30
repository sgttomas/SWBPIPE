import { afterEach, describe, expect, it, vi } from 'vitest';
import { EmbeddedAgentSession, type WorkspaceExecution, type AgentRunReceipt } from './embeddedAgentSession';
import type { PreviewModel, ModelHashEvidence } from '../types';
import type { AgentBasis } from './embeddedAgentBridge';
const mocks=vi.hoisted(()=>({validate:vi.fn()}));
vi.mock('./operationBatchService',()=>({validateOperationBatch:mocks.validate}));
const hash={algorithm:'sha256',canonicalization:'rfc8785_jcs',payload_scope:'model_payload',payload_ref:'invented',value:'a'.repeat(64),hash_status:'computed_local_preview'} as ModelHashEvidence;
const basis:AgentBasis={workspaceHandle:'workspace-invented',applicationGeneration:1,modelRevision:1,modelHash:hash,stateId:null,runId:null};
function deferred<T>(){let resolve!:(v:T)=>void;const promise=new Promise<T>(r=>{resolve=r;});return {resolve,promise};}
function setup(){
 let revision=1,generation=1,tool='swbpipe_read_model_summary',args:Record<string,unknown>={};
 const model={project:{id:'invented',units:{length:'m'}},nodes:[{id:'node-1',position:{x:-0,y:1,z:2}}]} as unknown as PreviewModel;
 const owner:WorkspaceExecution={snapshot:()=>({model,hash,revision,generation,result:null,standing:null}),register:vi.fn(async()=> 'batch-key'),proposal:vi.fn(()=>({status:'pending_review' as const,outcomeModelHash:null})),solve:vi.fn(async()=>undefined),withdraw:vi.fn()};
 const sent:Record<string,unknown>[]=[];
 const invoke=vi.fn(async(value:Record<string,unknown>):Promise<unknown>=>{
  sent.push(value);
  switch(value.kind){
   case 'register':return {...basis,ownerCapability:'private-owner-capability'};
   case 'start':return {status:'starting'};
   case 'poll':return {notices:['invented-session:1'],messages:[],closed:false};
   case 'claim':return {raw:JSON.stringify({requestRef:'invented-session:1',threadId:'thread-1',turnId:'turn-1',callId:'call-1',basis:{...basis,modelHash:Object.fromEntries(Object.entries(hash).reverse())},tool,arguments:args})};
   case 'publish_actual':return {recorded:true};
   case 'complete':return {delivery:'detached'};
   default:return {};
  }
 });
 const session=new EmbeddedAgentSession(owner,invoke);
 return {session,owner,invoke,sent,edit:()=>{revision++;},retire:()=>{generation++;},call:(name:string,a:Record<string,unknown>={})=>{tool=name;args=a;}};
}
afterEach(()=>{vi.useRealTimers();vi.clearAllMocks();});
async function drain(){for(let i=0;i<16;i++)await Promise.resolve();}
describe('workspace-owned live embedded agent adapter',()=>{
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
  vi.useFakeTimers();const {session,owner,call,edit,sent}=setup();const gate=deferred<AgentRunReceipt|undefined>();vi.mocked(owner.solve).mockReturnValueOnce(gate.promise);
  call('swbpipe_solve');await session.start('revision');await session.poll();await drain();edit();
  gate.resolve({jobId:'job-1',runId:'run-1',modelHash:hash,solverMode:'sparse_interactive',health:'ok'});await drain();
  const actual=JSON.parse(session.actual('invented-session:1')!.outcome);expect(actual.basis.modelRevision).toBe(1);expect(actual.data.runId).toBe('run-1');
  expect(sent.find(v=>v.kind==='publish_actual')!.proof).toMatchObject({nativeJobId:'job-1',status:'completed'});await session.close();
 });
 it('never retries a repeated notice and closes on project replacement',async()=>{
  vi.useFakeTimers();const {session,owner,retire,call}=setup();call('swbpipe_solve');await session.start('revision');await session.poll();await drain();await session.poll();await drain();
  expect(owner.solve).toHaveBeenCalledOnce();retire();await session.poll();await expect(session.text('invented')).rejects.toThrow('AGENT-CLOSED');
 });
});
