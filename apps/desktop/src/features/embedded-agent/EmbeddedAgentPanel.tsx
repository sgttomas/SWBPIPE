import { useEffect, useState } from 'react';
import { CSV_PREAMBLE_NOTICE } from '../result-export/csvTable';
import { EmbeddedAgentSession, type AgentSessionView } from '../../services/embeddedAgentSession';

type Provider = {revision:string;endpoint:string;model:string;contextWindow:number|null;credentialMode:'none'|'keychain'};
/** The person's provider context window, in tokens; the native store enforces the same bounds. */
const CONTEXT_WINDOW={min:8192,max:131072};
type Snapshot = {provider:Provider|null;cleanupPending:number|null;cleanupError:unknown};
const object=(value:unknown):value is Record<string,unknown>=>!!value&&typeof value==='object'&&!Array.isArray(value);
function snapshot(value:unknown):Snapshot {
 if(!object(value)||!(value.provider===null||object(value.provider)))throw new Error('INVALID-SETTINGS');
 const provider=value.provider;
 if(provider&&!(typeof provider.revision==='string'&&typeof provider.endpoint==='string'&&typeof provider.model==='string'&&(provider.contextWindow===null||Number.isInteger(provider.contextWindow))&&['none','keychain'].includes(String(provider.credentialMode))))throw new Error('INVALID-SETTINGS');
 return {provider:provider as Provider|null,cleanupPending:typeof value.cleanupPending==='number'?value.cleanupPending:null,cleanupError:value.cleanupError};
}
function settingsError(error:unknown):string {
 if(object(error)&&object(error.reenter_key)){
  if(error.reenter_key.reason==='missing')return 'The saved key is missing. Re-enter your key.';
  if(error.reenter_key.reason==='denied')return 'This build cannot use the saved key. Re-enter your key.';
 }
 switch(error){
  case 'invalid_input':return `Use a local HTTP endpoint or a hosted HTTPS endpoint, a non-empty model identifier and a context window from ${CONTEXT_WINDOW.min} to ${CONTEXT_WINDOW.max} tokens.`;
  case 'context_window_required':return "Enter the model's context window in tokens and save the provider before starting.";
  case 'corrupt':return 'Provider settings could not be verified.';
  case 'stale':return 'Settings changed. Refresh them before saving.';
  case 'new_credential_required':return 'Enter a key for this provider, or select no API key.';
  case 'busy':return 'Wait for agent cleanup before changing the provider.';
  case 'recovery_required':return 'Credential cleanup is pending. Restore Keychain access and retry cleanup.';
  case 'admission':return 'Resources are unavailable. Try again when memory is available.';
  case 'not_configured':return 'Save a provider and model before starting.';
  default:return 'Provider settings could not be used. Check Keychain access and refresh settings.';
 }
}
function failureText(failure:string|null):string|null {
 switch(failure){
  case null:return null;
  case 'stale_runtime_scope':return 'Previous session cleanup needs recovery. The agent remains unavailable.';
  case 'observation':return 'Session isolation could not be verified. The agent stopped.';
  case 'supplier_resource_unavailable':case 'supplier_pin_mismatch':case 'supplier_resource_invalid':return 'The bundled agent runtime could not be verified.';
  case 'message_limit':return 'Conversation output exceeded the session limit. The agent stopped.';
  case 'protocol':return 'The agent protocol session ended with an error.';
  default:return 'The agent session stopped. Check its configuration before starting again.';
 }
}
const initial:AgentSessionView={status:'closed',closed:true,turnActive:false,turnStatus:'none',failure:null,messages:[]};
/** Minimal explicit-start UI. Only native custody stores keys; this component
 * never writes them to a browser store, log, file or conversation. */
export function EmbeddedAgentPanel({session,active,onReview}:{session:EmbeddedAgentSession;active:boolean;onReview:()=>void}) {
 const [available,setAvailable]=useState<boolean|null>(null),[unavailable,setUnavailable]=useState('');
 const [saved,setSaved]=useState<Snapshot|null>(null),[endpoint,setEndpoint]=useState(''),[model,setModel]=useState(''),[contextWindow,setContextWindow]=useState('');
 const [credentialMode,setCredentialMode]=useState<'keep'|'replace'|'no_key'>('no_key'),[key,setKey]=useState('');
 const [view,setView]=useState<AgentSessionView>(initial),[text,setText]=useState(''),[busy,setBusy]=useState(false),[message,setMessage]=useState<string|null>(null);
 const [settingsOpen,setSettingsOpen]=useState(true);
 useEffect(()=>session.subscribe(setView),[session]);
 function adopt(next:Snapshot){setSaved(next);if(next.provider){setEndpoint(next.provider.endpoint);setModel(next.provider.model);setContextWindow(next.provider.contextWindow===null?'':String(next.provider.contextWindow));setCredentialMode(next.provider.credentialMode==='keychain'?'keep':'no_key');}}
 async function settings(request:Record<string,unknown>){
  const reply=await session.provider(request);
  if(!object(reply)||reply.status!=='ok'){setMessage(settingsError(object(reply)?reply.error:null));return null;}
  const next=snapshot(reply.snapshot);adopt(next);return next;
 }
 useEffect(()=>{
  if(!active||available!==null)return;
  let disposed=false,qualified=false;
  void session.availability().then(async result=>{
   if(disposed)return;
   if(!object(result)||result.available!==true){setAvailable(false);setUnavailable(object(result)&&result.reason==='qualified_isolation_unavailable'?'Agent isolation is not qualified for this platform.':'Use a qualified native Mac build to run the embedded agent.');return;}
   qualified=true;const reply=await session.provider({kind:'provider_snapshot'});
   if(disposed)return;
   setAvailable(true);
   if(object(reply)&&reply.status==='ok'){const next=snapshot(reply.snapshot);adopt(next);setSettingsOpen(!next.provider);}else setMessage(settingsError(object(reply)?reply.error:null));
  }).catch(()=>{if(!disposed){if(qualified){setAvailable(true);setMessage('Provider settings are unavailable. Stop the agent and refresh settings.');}else{setAvailable(false);setUnavailable('The embedded agent is available in the qualified native Mac app.');}}});
  return ()=>{disposed=true;};
 },[active,available,session]);
 async function perform(action:()=>Promise<unknown>){if(busy)return;setBusy(true);setMessage(null);try{await action();}catch(error){setMessage(error instanceof Error&&error.message==='AGENT-WORKSPACE-NOT-READY'?'Open a model and wait for its fingerprint before starting.':'The request did not complete. Check the session status before trying again.');}finally{setBusy(false);}}
 async function save(){
  const entered=key;setKey('');
  const tokens=Number(contextWindow);
  if(!/^[0-9]+$/.test(contextWindow)||tokens<CONTEXT_WINDOW.min||tokens>CONTEXT_WINDOW.max){setKey(entered);setMessage(settingsError('invalid_input'));return;}
  const result=await settings({kind:'provider_save',expected:saved?.provider?.revision??null,endpoint,model,contextWindow:tokens,credentialMode,...(credentialMode==='replace'?{key:entered}:{})});
  if(result){setSettingsOpen(false);setMessage(result.cleanupPending===0?'Provider saved.':result.cleanupPending===null?'Provider saved; credential cleanup status is unavailable.':'Provider saved; stale credential cleanup is pending.');}
 }
 async function start(){
  if(!saved?.provider)return;
  const reply=await session.start(saved.provider.revision);
  if(object(reply)&&reply.status==='credential_error'){setSettingsOpen(true);if(object(reply.error)&&object(reply.error.reenter_key)&&['missing','denied'].includes(String(reply.error.reenter_key.reason)))setCredentialMode('replace');setMessage(settingsError(reply.error));}
 }
 const running=!view.closed;
 if(!active&&available===null)return null;
 return <section className="panel embedded-agent-panel" aria-label="Embedded agent">
  <h3>Agent</h3>
  <p>Inspect the workspace and propose supported changes. Changes enter Batch review for your explicit Apply; sending a message does not apply them.</p>
  {available===null?<p role="status">Checking availability…</p>:available===false?<p role="status">{unavailable}</p>:<>
   <details open={settingsOpen} onToggle={event=>setSettingsOpen(event.currentTarget.open)}>
    <summary>Provider settings</summary>
    <p>The selected endpoint receives the conversation and tool results. Use a local OpenAI-compatible endpoint or a hosted HTTPS provider.</p>
    <form onSubmit={event=>{event.preventDefault();void perform(save);}} autoComplete="off">
     <label>Endpoint<input aria-label="Agent endpoint" type="url" value={endpoint} maxLength={2048} placeholder="https://provider.example/v1" onChange={event=>setEndpoint(event.target.value)} disabled={busy||running} required/></label>
     <label>Model<input aria-label="Agent model" value={model} maxLength={1024} onChange={event=>setModel(event.target.value)} disabled={busy||running} required/></label>
     <label>Context window (tokens)<input aria-label="Agent context window" type="number" inputMode="numeric" min={CONTEXT_WINDOW.min} max={CONTEXT_WINDOW.max} step={1} value={contextWindow} placeholder="the window your provider serves" onChange={event=>setContextWindow(event.target.value)} disabled={busy||running} required/></label>
     <label>Authentication<select aria-label="Agent authentication" value={credentialMode} onChange={event=>{setCredentialMode(event.target.value as typeof credentialMode);setKey('');}} disabled={busy||running}>
      {saved?.provider?.credentialMode==='keychain'?<option value="keep">Keep saved key</option>:null}
      <option value="replace">Enter a key</option><option value="no_key">No API key</option>
     </select></label>
     {credentialMode==='replace'?<label>API key<input aria-label="Agent API key" type="password" value={key} maxLength={4096} autoComplete="off" onChange={event=>setKey(event.target.value)} disabled={busy||running} required/></label>:null}
     <button type="submit" disabled={busy||running}>Save provider</button>
     <button type="button" disabled={busy||running} onClick={()=>void perform(async()=>{await settings({kind:'provider_snapshot'});})}>Refresh settings</button>
     {saved?.provider?<button type="button" disabled={busy||running} onClick={()=>void perform(async()=>{const result=await settings({kind:'provider_clear',expected:saved.provider!.revision});if(result){setEndpoint('');setModel('');setContextWindow('');setKey('');setCredentialMode('no_key');setMessage('Provider cleared.');}})}>Clear provider</button>:null}
     {saved?.cleanupPending!==0&&saved!==null?<button type="button" disabled={busy||running} onClick={()=>void perform(async()=>{await settings({kind:'provider_recover'});})}>Retry credential cleanup</button>:null}
    </form>
   </details>
   {saved?.provider?<p>{saved.provider.model} · {saved.provider.contextWindow===null?'context window not entered':`${saved.provider.contextWindow.toLocaleString('en-US')}-token context window`} · {saved.provider.endpoint}</p>:<p>Save a provider and model to begin.</p>}
   <div>
    <button type="button" disabled={busy||running||!saved?.provider} onClick={()=>void perform(start)}>Start agent</button>
    <button type="button" disabled={busy||!running||!view.turnActive||view.turnStatus!=='in_progress'} onClick={()=>void perform(()=>session.interrupt())}>Interrupt turn</button>
    <button type="button" disabled={busy||!running} onClick={()=>void perform(()=>session.close())}>Stop agent</button>
    <button type="button" onClick={onReview}>Open Batch review</button>
   </div>
   <p role="status" data-testid="embedded-agent-standing">{view.closed?'Closed':view.status==='starting'?'Starting':view.turnActive?'Working':view.status==='ready'?'Ready':view.status}. {view.turnStatus==='failed'?'The last turn failed.':view.turnStatus==='interrupted'?'Turn interrupted. Applied changes remain applied.':''}</p>
   {failureText(view.failure)?<p role="alert">{failureText(view.failure)}</p>:null}
   {message?<p role="status">{message}</p>:null}
   <p>{CSV_PREAMBLE_NOTICE}</p>
   {view.truncated?<p>Earlier conversation text was omitted from this bounded view.</p>:null}
   <div className="embedded-agent-conversation" aria-label="Draft agent conversation" aria-live="polite" style={{maxHeight:'20rem',overflow:'auto'}}>
    {view.messages.map(item=><p key={item.itemId} style={{whiteSpace:'pre-wrap'}}><strong>{item.role==='user'?'You':'Agent'}:</strong> {item.text}</p>)}
   </div>
   <form onSubmit={event=>{event.preventDefault();const submitted=text;void perform(async()=>{await session.text(submitted);setText('');});}}>
    <label>Message<textarea aria-label="Message to agent" value={text} maxLength={8192} rows={3} onChange={event=>setText(event.target.value)} disabled={view.closed||busy}/></label>
    <button type="submit" disabled={busy||view.closed||view.status!=='ready'||view.turnActive||!text.trim()}>Send</button>
   </form>
  </>}
 </section>;
}
