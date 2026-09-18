"""One explicit operation per invocation; no retries, loops over slots or overall timeout.
prepare SLOT: read-only host checks plus immutable receipt; no runtime.
launch SLOT RECEIPT SHA RELEASE SHA: exactly one reviewed launcher invocation.
prepare-close SLOT: fresh same-slot cleanup and binding receipt.
close SLOT RECEIPT SHA RELEASE SHA: append successful completion/waivers, no browser.
ROOT must separately authorize execution of each command; a hash is not authorization.
"""
from pathlib import Path
import json,hashlib,subprocess,os,sys,datetime
HERE=Path(__file__).resolve().parent
POLICY=HERE/'POLICY_V6.json'
POLICY_SHA='c608c66ad9b04fb1a74018168ff3936b76b7cc6f1d6980e0fe2b58f0090b8163'
def sha(p):return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def now():return datetime.datetime.now(datetime.timezone.utc).isoformat()
def bound(p,h):
 assert sha(p)==h, 'bound file drift: '+str(p)
 return json.loads(Path(p).read_text())
def save(p,d):
 p=Path(p);p.parent.mkdir(parents=True,exist_ok=True)
 with p.open('x') as f:f.write(json.dumps(d,indent=2)+'\n')
 return {'path':str(p),'sha256':sha(p)}
def read(p):return json.loads(Path(p).read_text())
def command(args,cwd=None):return subprocess.check_output(args,cwd=cwd,text=True)
policy=bound(POLICY,POLICY_SHA);seed=bound(policy['seed']['path'],policy['seed']['sha256'])
external={k:v for k,v in seed['frozenEnvironment'].items() if k not in ['UI_FOUNDATION_EVIDENCE_DIR','UI_FOUNDATION_METHOD_MANIFEST_PATH','UI_FOUNDATION_METHOD_MANIFEST_SHA256']}
transition=policy['displayTransition'];assert transition['schema']=='ui-foundation.internal120-transition/v1'
assert transition['authority']['sha256']=='6df272b7af0367b0083302229018143f8fa397a6bde0f5e368c6ec1e454157b9'
assert transition['authority']['path']==str(Path(policy['seed']['path']).parent.parent.parent/'OWNER_DIRECTION_INTERNAL120_20260917.md')
assert sha(transition['authority']['path'])==transition['authority']['sha256']
assert transition['previousProfile']=={'path':external['UI_FOUNDATION_REFERENCE_PROFILE'],'sha256':external['UI_FOUNDATION_REFERENCE_PROFILE_SHA256']}
bound(transition['previousProfile']['path'],transition['previousProfile']['sha256'])
assert transition['profile']['sha256']=='6de03bed447bc236b34295eabcdcd3957434aada022e368f1a4559e5f839ad8e'
bound(transition['profile']['path'],transition['profile']['sha256'])
external.update({'UI_FOUNDATION_REFERENCE_PROFILE':transition['profile']['path'],'UI_FOUNDATION_REFERENCE_PROFILE_SHA256':transition['profile']['sha256']})
project=Path(policy['instrumentProjectRoot']);ledger=Path(policy['ledgerRoot']);slots=list(policy['attemptRoots']);mode=sys.argv[1]
def verify():
 assert command(['git','rev-parse','HEAD'],project).strip()==policy['instrumentRevision']
 assert not command(['git','status','--porcelain'],project)
 method=bound(policy['method']['path'],policy['method']['sha256']);assert len(method['files'])==34
 for e in method['files']:assert sha(project/e['path'])==e['sha256']
 env=external
 for p,h in [(env['UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST'],env['UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST_SHA256']),(env['PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH'],env['PLAYWRIGHT_CHROMIUM_EXECUTABLE_SHA256']),(env['UI_FOUNDATION_REFERENCE_PROFILE'],env['UI_FOUNDATION_REFERENCE_PROFILE_SHA256']),(project/'apps/desktop/e2e/ui-foundation/fixture-manifest.json',env['UI_FOUNDATION_MANIFEST_SHA256']),(Path(env['UI_FOUNDATION_CANDIDATE_ORACLE_DIR'])/'CANDIDATE_POINT_ORACLE_MANIFEST.json',env['UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256'])]:assert sha(p)==h
 product=Path(env['UI_FOUNDATION_CANDIDATE_SOURCE_ROOT']);assert command(['git','rev-parse','HEAD'],product).strip()=='8468a33c86adb622b25e98f98b0eaf28c7e9fa0e';assert not command(['git','status','--porcelain'],product)
 bundle=read(env['UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST'])
 for group in ['source','mutableTestOnlySourceSnapshot','files']:
  for e in bundle[group]:assert sha(product/e['path'])==e['sha256'] and (product/e['path']).stat().st_size==e['bytes']
 oracle=Path(env['UI_FOUNDATION_CANDIDATE_ORACLE_DIR'])
 for e in read(oracle/'CANDIDATE_POINT_ORACLE_MANIFEST.json')['files']:assert sha(oracle/e['path'])==e['sha256'] and (oracle/e['path']).stat().st_size==e['bytes']
 profile=read(env['UI_FOUNDATION_REFERENCE_PROFILE']);assert int(command(['sysctl','-n','hw.memsize']))==profile['memoryBytes']
 display=json.loads(command(['/usr/sbin/system_profiler','SPDisplaysDataType','-json']));main=[(g,v) for g in display['SPDisplaysDataType'] for v in g.get('spdisplays_ndrvs',[]) if v.get('spdisplays_main')=='spdisplays_yes' and v.get('spdisplays_online')=='spdisplays_yes'];assert len(main)==1 and main[0][0]['_name']==profile['hostModel'];mapping={'name':'_name','vendor':'_spdisplays_display-vendor-id','product':'_spdisplays_display-product-id','serial':'_spdisplays_display-serial-number','pixels':'_spdisplays_pixels','resolution':'_spdisplays_resolution','mirror':'spdisplays_mirror','connection':'spdisplays_connection_type'};assert {k:main[0][1][v] for k,v in mapping.items()}==profile['display'];assert profile['refreshHz']==120 and profile['display']['name']=='Color LCD' and profile['display']['connection']=='spdisplays_internal' and profile['display']['mirror']=='spdisplays_off';assert '120.00Hz' in main[0][1]['_spdisplays_resolution']
 return {'status':'PASS_INDEPENDENT_EXTERNAL_REVALIDATION','externalBindings':external,'verifiedAt':now(),'instrumentRevision':policy['instrumentRevision'],'method':policy['method'],'display':display}
def cleanup():
 ps=command(['ps','-axo','pid,ppid,comm']);remaining=[s for s in ps.splitlines() if 'Google Chrome for Testing' in s]
 server=subprocess.run(['lsof','-nP','-iTCP:5176','-sTCP:LISTEN'],capture_output=True,text=True);assert server.returncode in [0,1]
 assert not remaining and server.returncode==1 and not server.stdout,'browser or server remains'
 return {'browserProcessesRemaining':len(remaining),'serverListening':False,'verificationStatus':'VERIFIED','verifiedAt':now(),'processInventory':ps,'portReadout':{'exit':server.returncode,'stdout':server.stdout,'stderr':server.stderr}}
def historical():
 authority=policy['ownerTransition']['authority'];assert authority['path']==str(Path(policy['seed']['path']).parent.parent.parent/'OWNER_DIRECTION_ONE_SUCCESS_20260917.md');assert authority['sha256']=='ba5e8bceea55838cc0d23e815cc9a890ed543534085a9e9bdd932d133fb37fa2' and sha(authority['path'])==authority['sha256']
 for key in ['previousPolicy','registry']:bound(**{'p':policy['ownerTransition'][key]['path'],'h':policy['ownerTransition'][key]['sha256']})
 for row in policy['ownerTransition']['history']:
  for ref in row.values():bound(ref['path'],ref['sha256'])
 assert not (ledger/'claims/1000.5.json').exists(),'waived1000.5 cannot be claimed'
def success(slot):
 terminal=ledger/f'terminal-{slot}.json';size,ordinal=slot.split('.');result=Path(policy['attemptRoots'][slot])/f'raw/run-{int(ordinal):02d}/{size}/result.json'
 if not terminal.exists() or not result.exists():return False
 t=read(terminal);r=read(result);return t.get('exitCode')==0 and not t.get('signal') and not t.get('launchError') and r.get('collection',{}).get('evidenceValidity')=='VALID' and r.get('collection',{}).get('collectionCompleteness')=='COMPLETE'
def previous(slot,closing=False):
 historical();eligible=[f'10000.{n}' for n in range(1,6)];assert slot in eligible,'no further1000 or sixth slot'
 claims=[v for v in eligible if (ledger/'claims'/f'{v}.json').exists()];assert claims==eligible[:len(claims)]
 assert not (ledger/'one-success-complete.json').exists(),'already closed'
 if closing:
  assert claims and claims[-1]==slot and success(slot),'no matching complete successful actual run to close'
  prior=slot
 else:
  assert len(claims)<5 and eligible[len(claims)]==slot,'consumed/out-of-order slot'
  assert not any(success(v) for v in claims),'success already achieved; close instead'
  prior=claims[-1] if claims else '1000.4'
 path=ledger/'claims'/f'{prior}.json';return path,sha(path),prior
if mode in ['prepare','prepare-close']:
 assert len(sys.argv)==3;slot=sys.argv[2];prev,prevsha,prevslot=previous(slot,mode=='prepare-close');proof=cleanup();bindings=verify();proof['previousClaimSha256']=prevsha
 terminal=read(ledger/f'terminal-{prevslot}.json') if prevslot and (ledger/f'terminal-{prevslot}.json').exists() else (read(HERE.parent/'COHORT_V1_PROCESS_RETURN.json') if not prevslot else None)
 normal=terminal is not None and isinstance(terminal.get('exitCode'),int) and terminal['exitCode']>=0 and not terminal.get('signal') and not terminal.get('launchError')
 proof['processDisposition']='NORMAL_EXIT' if normal else 'EXTERNAL_RECOVERY_VERIFIED';proof['priorTerminal']=terminal;proof['recoveryBasis']='Actual full process inventory and vacant5176 after prior invocation; all external bytes/live display freshly revalidated'
 folder=HERE/('close-receipts' if mode=='prepare-close' else 'receipts')/slot;assert not folder.exists();cref=save(folder/'cleanup.json',proof);bref=save(folder/'bindings.json',bindings)
 receipt={'schema':'ui-foundation.continuation-preconditions/v1','slot':slot,'previousClaimSha256':prevsha,'verifiedAt':now(),'cleanup':{'status':'VERIFIED_NO_REMAINING_BROWSER_OR_SERVER','evidence':cref},'bindingsStatus':'VERIFIED_UNCHANGED','externalBindings':external,'bindingEvidence':bref,'evidence':[cref,bref]};print(json.dumps(save(folder/'receipt.json',receipt)));sys.exit(0)
assert mode in ['launch','close']
if mode in ['launch','close']:
 assert len(sys.argv)==7;slot,rpath,rsha,release,rh=sys.argv[2:];previous(slot,mode=='close');receipt=bound(rpath,rsha);assert receipt['slot']==slot;authority=bound(release,rh);proof=cleanup();bindings=verify();assert (datetime.datetime.now(datetime.timezone.utc)-datetime.datetime.fromisoformat(receipt['verifiedAt'])).total_seconds()<300,'receipt stale; ROOT must disposition fresh proof'
 argv=['node','apps/desktop/e2e/ui-foundation/characterization-observations.mjs']+(['launch-slot',str(POLICY),POLICY_SHA,slot,rpath,rsha] if mode=='launch' else ['close-one-success',str(POLICY),POLICY_SHA,rpath,rsha]);folder=HERE/('processes' if mode=='launch' else 'close-processes')/slot;assert not folder.exists();Path(policy['attemptRoots'][slot]).parent.mkdir(parents=True,exist_ok=True);env={k:v for k,v in os.environ.items() if not k.startswith(('UI_FOUNDATION_','PLAYWRIGHT_'))}
launch={'at':now(),'argv':argv,'cwd':str(project),'release':{'path':release,'sha256':rh},'scriptSha256':sha(__file__),'policySha256':POLICY_SHA,'environmentOverrides':{k:v for k,v in env.items() if k.startswith(('UI_FOUNDATION_','PLAYWRIGHT_'))},'beforeCleanup':proof,'beforeBindings':bindings};save(folder/'launch.json',launch)
with (folder/'stdout.log').open('x') as stdout,(folder/'stderr.log').open('x') as stderr:
 p=subprocess.Popen(argv,cwd=project,env=env,stdout=stdout,stderr=stderr);save(folder/'started.json',{'pid':p.pid,'at':now()});print('START',mode,str(folder),p.pid,flush=True);rc=p.wait()
end={'at':now(),'exitCode':rc,'signal':-rc if rc<0 else None}
try:end['afterCleanup']=cleanup();end['afterBindings']=verify()
except Exception as error:end['postconditionError']=repr(error)
save(folder/'process-return.json',end);print('END',mode,'exit',rc,'postconditionError',end.get('postconditionError'),flush=True);sys.exit(rc if rc>=0 else 1)
