import pathlib,json,hashlib,subprocess,sys,os,datetime
E=pathlib.Path(__file__).parent;P=E.parents[8];P=pathlib.Path('/Users/ryan/.codex/worktrees/8728/chirality-results-integrity-20260913/projects/chirality-piping')
pre=json.loads((E/'PRE_EFFECT_V1.json').read_text());d=E/sys.argv[1];d.mkdir()
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
argv=sys.argv[2:];paths=[x['path'] for x in pre['accepted39']+pre['tests']]
if (E/'NATIVE_PRE_EFFECT_V1.json').exists():paths+=['apps/desktop/src-tauri/Cargo.lock','apps/desktop/src-tauri/Cargo.toml']
cwd=P/'apps/desktop' if argv[0]=='node' else P
env=os.environ.copy();env['CARGO_NET_OFFLINE']='true'
if sys.argv[1]=='010-app-source-capture':env['DISPLAY_TEST_OUTPUT_DIR']=str(E/'actual_app_capture_v1')
command={'argv':argv,'cwd':str(cwd),'env_overrides':{k:env[k] for k in ['CARGO_NET_OFFLINE','DISPLAY_TEST_OUTPUT_DIR'] if k in env},'source_hashes':{x:sha(P/x) for x in paths},'driver_sha256':sha(pathlib.Path(__file__)),'prepared_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}
(d/'COMMAND.json').write_text(json.dumps(command,indent=2)+'\n')
with (d/'stdout.log').open('wb') as o,(d/'stderr.log').open('wb') as e:r=subprocess.run(argv,cwd=cwd,env=env,stdout=o,stderr=e)
result={'exit_code':r.returncode,'stdout_sha256':sha(d/'stdout.log'),'stderr_sha256':sha(d/'stderr.log'),'completed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()};(d/'RESULT.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result));print((d/'stdout.log').read_text()[-5500:]);print((d/'stderr.log').read_text()[-5500:]);sys.exit(r.returncode)
