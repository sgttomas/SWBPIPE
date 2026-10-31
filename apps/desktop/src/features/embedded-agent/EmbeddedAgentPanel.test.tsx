import { fireEvent, render, screen, waitFor } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { EmbeddedAgentPanel } from './EmbeddedAgentPanel';
import type { EmbeddedAgentSession, AgentSessionView } from '../../services/embeddedAgentSession';
const metadata={revision:'a'.repeat(48),endpoint:'http://127.0.0.1:12345/v1',model:'invented-model',contextWindow:32768,credentialMode:'keychain'};
const settings={provider:metadata,cleanupPending:0,cleanupError:null};
function setup(configured=true){
 let listener:(view:AgentSessionView)=>void=()=>{};
 const closed:AgentSessionView={status:'closed',closed:true,turnActive:false,turnStatus:'none',failure:null,messages:[]};
 const port={
  subscribe:vi.fn((fn:typeof listener)=>{listener=fn;fn(closed);return ()=>{};}),
  availability:vi.fn(async()=>({available:true})),
  provider:vi.fn(async(_request:Record<string,unknown>):Promise<unknown>=>({status:'ok',snapshot:configured?settings:{provider:null,cleanupPending:0,cleanupError:null}})),
  start:vi.fn(async(_revision:string):Promise<unknown>=>{listener({...closed,status:'ready',closed:false});return {status:'starting'};}),
  close:vi.fn(async()=>{listener(closed);}),text:vi.fn(async(_text:string)=>({status:'queued'})),interrupt:vi.fn(async()=>({status:'queued'}))
 };
 const session=port as unknown as EmbeddedAgentSession;
 const emit=(changes:Partial<AgentSessionView>)=>listener({...closed,...changes});
 return {session,port,emit};
}
describe('minimal embedded agent panel',()=>{
 it('does no availability or provider IO until its page is opened',async()=>{
  const {session,port}=setup();const page=render(<EmbeddedAgentPanel session={session} active={false} onReview={()=>{}}/>);
  expect(port.availability).not.toHaveBeenCalled();page.rerender(<EmbeddedAgentPanel session={session} active onReview={()=>{}}/>);
  await screen.findByText('Start agent');expect(port.start).not.toHaveBeenCalled();
 });
 it('shows unavailable isolation and never asks for a key',async()=>{
  const {session,port}=setup();port.availability.mockResolvedValueOnce({available:false});
  render(<EmbeddedAgentPanel session={session} active onReview={()=>{}}/>);
  await screen.findByText('Use a qualified native Mac build to run the embedded agent.');
  expect(port.provider).not.toHaveBeenCalled();expect(screen.queryByLabelText('Agent API key')).toBeNull();
 });
 it('saves a key only through native custody and never puts it in a conversation',async()=>{
  const {session,port}=setup(false);render(<EmbeddedAgentPanel session={session} active onReview={()=>{}}/>);
  await screen.findByText('Save a provider and model to begin.');
  fireEvent.change(screen.getByLabelText('Agent endpoint'),{target:{value:metadata.endpoint}});
  fireEvent.change(screen.getByLabelText('Agent model'),{target:{value:metadata.model}});
  fireEvent.change(screen.getByLabelText('Agent context window'),{target:{value:'32768'}});
  fireEvent.change(screen.getByLabelText('Agent authentication'),{target:{value:'replace'}});
  fireEvent.change(screen.getByLabelText('Agent API key'),{target:{value:'invented-fixture-credential'}});
  port.provider.mockResolvedValueOnce({status:'ok',snapshot:settings});
  fireEvent.click(screen.getByText('Save provider'));
  await screen.findByText('Provider saved.');
  expect(port.provider).toHaveBeenLastCalledWith({kind:'provider_save',expected:null,endpoint:metadata.endpoint,model:metadata.model,contextWindow:32768,credentialMode:'replace',key:'invented-fixture-credential'});
  expect(screen.queryByLabelText('Agent API key')).toBeNull();
  fireEvent.click(screen.getByText('Start agent'));await screen.findByText('Ready.');
  fireEvent.change(screen.getByLabelText('Message to agent'),{target:{value:'Read the workspace summary.'}});fireEvent.click(screen.getByText('Send'));
  await waitFor(()=>expect(port.text).toHaveBeenCalledWith('Read the workspace summary.'));
  expect(JSON.stringify(port.text.mock.calls)).not.toContain('invented-fixture-credential');
 });
 it.each(['missing','denied'] as const)('keeps %s as typed recoverable re-entry',async reason=>{
  const {session,port}=setup();port.start.mockResolvedValueOnce({status:'credential_error',error:{reenter_key:{reason}}});
  render(<EmbeddedAgentPanel session={session} active onReview={()=>{}}/>);
  await screen.findByText('Start agent');fireEvent.click(screen.getByText('Start agent'));
  await screen.findByText(reason==='missing'?'The saved key is missing. Re-enter your key.':'This build cannot use the saved key. Re-enter your key.');
  expect(screen.getByLabelText('Agent authentication')).toHaveValue('replace');expect(screen.getByLabelText('Agent API key')).toHaveValue('');
 });
 it('leaves other keychain errors as errors without switching to re-entry',async()=>{
  const {session,port}=setup();port.start.mockResolvedValueOnce({status:'credential_error',error:{keychain:'unknown'}});
  render(<EmbeddedAgentPanel session={session} active onReview={()=>{}}/>);await screen.findByText('Start agent');fireEvent.click(screen.getByText('Start agent'));
  await screen.findByText('Provider settings could not be used. Check Keychain access and refresh settings.');
  expect(screen.getByLabelText('Agent authentication')).toHaveValue('keep');
 });
 it('requires an entered context window in range and sends no default',async()=>{
  const {session,port}=setup(false);render(<EmbeddedAgentPanel session={session} active onReview={()=>{}}/>);
  await screen.findByText('Save a provider and model to begin.');
  fireEvent.change(screen.getByLabelText('Agent endpoint'),{target:{value:metadata.endpoint}});
  fireEvent.change(screen.getByLabelText('Agent model'),{target:{value:metadata.model}});
  for(const value of ['8191','131073','32768.5']){
   fireEvent.change(screen.getByLabelText('Agent context window'),{target:{value}});
   fireEvent.submit(screen.getByText('Save provider').closest('form')!);
   await screen.findByText(/context window from 8192 to 131072 tokens/);
  }
  expect(port.provider.mock.calls.filter(([request])=>request.kind==='provider_save')).toHaveLength(0);
 });
 it('shows the saved window and refuses Start for settings saved before the field existed',async()=>{
  const {session,port}=setup();port.provider.mockResolvedValue({status:'ok',snapshot:{...settings,provider:{...metadata,contextWindow:null}}});
  port.start.mockResolvedValueOnce({status:'credential_error',error:'context_window_required'});
  render(<EmbeddedAgentPanel session={session} active onReview={()=>{}}/>);
  await screen.findByText(/context window not entered/);
  fireEvent.click(screen.getByText('Start agent'));
  await screen.findByText("Enter the model's context window in tokens and save the provider before starting.");
  expect(screen.getByLabelText('Agent context window')).toHaveValue(null);
 });
 it('shows a saved window beside the model',async()=>{
  const {session}=setup();render(<EmbeddedAgentPanel session={session} active onReview={()=>{}}/>);
  await screen.findByText(/invented-model · 32,768-token context window ·/);
 });
 it('retains the committed save when cleanup standing is unknown',async()=>{
  const {session,port}=setup();render(<EmbeddedAgentPanel session={session} active onReview={()=>{}}/>);await screen.findByText('Start agent');
  fireEvent.click(screen.getByText('Provider settings'));port.provider.mockResolvedValueOnce({status:'ok',snapshot:{...settings,cleanupPending:null,cleanupError:'unknown'}});
  fireEvent.click(screen.getByText('Save provider'));await screen.findByText('Provider saved; credential cleanup status is unavailable.');
  expect(port.provider).toHaveBeenCalledTimes(2);
 });
});
