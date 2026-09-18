from pathlib import Path
import json,hashlib,subprocess,os,sys,datetime
out=Path(__file__).parent;phase=out.parent.parent;sha=lambda p:hashlib.sha256(Path(p).read_bytes()).hexdigest();commands=out/'RUNTIME_COMMANDS_PREPARED_V4.json';assert sha(commands)=='04bd20156fefb44163d4d9143a50b2c966d9a1eca73fa7e734c4aa268d986d08';assert sha(phase/'PRE_RUNTIME_ACCEPTANCE.json')=='ef7fe72aedb65687a3468e6cdf81b0785eaeadfd478ea2badaeebea32025044a';d=json.loads(commands.read_text());stage=sys.argv[1];assert stage == 'smoke';spec=d[stage];env={k:v for k,v in os.environ.items() if not k.startswith(('UI_FOUNDATION_','PLAYWRIGHT_'))};env.update(spec['environment']);base=d['baseEnvironment'];raw=Path('/Users/ryan/.codex/task-runtime-cache/chirality-d70-baseline-20260917/instances/VERIFY')
if stage in ['smoke','cohort']:
 oracle=Path(env['UI_FOUNDATION_CANDIDATE_ORACLE_DIR'])/'CANDIDATE_POINT_ORACLE_MANIFEST.json';assert oracle.is_file();assert sha(oracle)==env['UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256'], 'frozen oracle manifest drift'
def check():
 assert sha(oracle)==env['UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256'], 'frozen oracle manifest drift'
 for entry in json.loads(oracle.read_text())['files']:
  member=oracle.parent/entry['path'];assert member.resolve().is_relative_to(oracle.parent.resolve());assert member.stat().st_size==entry['bytes'] and sha(member)==entry['sha256'], 'frozen oracle member drift'
 for a,b in [('UI_FOUNDATION_METHOD_MANIFEST_PATH','UI_FOUNDATION_METHOD_MANIFEST_SHA256'),('UI_FOUNDATION_REFERENCE_PROFILE','UI_FOUNDATION_REFERENCE_PROFILE_SHA256'),('UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST','UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST_SHA256'),('PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH','PLAYWRIGHT_CHROMIUM_EXECUTABLE_SHA256')]:assert sha(base[a])==base[b],a
 for name,root,groups in [(base['UI_FOUNDATION_METHOD_MANIFEST_PATH'],Path(d['cwd']),['files']),(base['UI_FOUNDATION_CANDIDATE_BUNDLE_MANIFEST'],Path(base['UI_FOUNDATION_CANDIDATE_SOURCE_ROOT']),['source','files'])]:
  m=json.loads(Path(name).read_text())
  for group in groups:
   for e in m[group]:assert sha(root/e['path'])==e['sha256'],e['path']
 assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=d['cwd'],text=True).strip()==d['instrumentRevision']
 root=base['UI_FOUNDATION_CANDIDATE_SOURCE_ROOT'];assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip()=='8468a33c86adb622b25e98f98b0eaf28c7e9fa0e';assert not subprocess.check_output(['git','status','--porcelain'],cwd=root,text=True)
assert subprocess.check_output(['git','rev-parse','HEAD'],cwd=d['cwd'],text=True).strip()==d['instrumentRevision'];check();destination=Path(env['UI_FOUNDATION_EVIDENCE_DIR']) if stage!='freezeOracle' else raw/'oracle-freeze-process-01';assert not destination.exists();destination.mkdir(parents=True);record={'stage':stage,'instrumentRevision':d['instrumentRevision'], 'launcherSha256':sha(Path(__file__)),'at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'argv':spec['argv'],'cwd':d['cwd'],'environmentOverrides':{k:v for k,v in env.items() if k.startswith(('UI_FOUNDATION_','PLAYWRIGHT_'))},'clearedInheritedKeys':[k for k in os.environ if k.startswith(('UI_FOUNDATION_','PLAYWRIGHT_'))],'cohortContribution':0 if stage=='smoke' else 'single planned characterization cohort','beforeBindings':'PASS'}
(destination/'process-before.txt').write_text(subprocess.check_output(['ps','-axo','pid,ppid,command'],text=True));launch=out/(stage.upper()+'_V5_LAUNCH.json');assert not launch.exists();launch.write_text(json.dumps(record,indent=2)+'\n');print('START',stage,str(destination),flush=True)
with (destination/'process.stdout.log').open('w') as stdout,(destination/'process.stderr.log').open('w') as stderr:
 p=subprocess.Popen(spec['argv'],cwd=d['cwd'],env=env,stdout=stdout,stderr=stderr);print('PID',p.pid,flush=True);rc=p.wait()
record.update({'exitCode':rc,'endedAt':datetime.datetime.now(datetime.timezone.utc).isoformat()});(destination/'process-after.txt').write_text(subprocess.check_output(['ps','-axo','pid,ppid,command'],text=True))
try:check();record['afterBindings']='PASS'
except Exception as e:record['afterBindings']='FAIL';record['bindingError']=str(e)
(out/(stage.upper()+'_V5_PROCESS_RETURN.json')).write_text(json.dumps(record,indent=2)+'\n');print('END',stage,rc,record['afterBindings'],flush=True);sys.exit(rc or (0 if record['afterBindings']=='PASS' else 2))
