from pathlib import Path
import subprocess,json,hashlib,os,datetime,sys
base=Path('/Users/ryan/.codex/worktrees/8728/chirality-result-compatibility-pressure-20260914')
project=base/'projects/chirality-piping'
run=project/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE'
out=Path('/private/tmp/piping-foundation-dec025-20260915-v2')
assert not out.exists(), 'Preserve any earlier attempt; choose a new output version.'
assert subprocess.check_output(['git','status','--porcelain'],cwd=base)==b'', 'Clean committed candidate required.'
head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=base,text=True).strip()
release=json.loads((run/'instances/ROOT/_run_records/FINAL_SOURCE_RELEASE_V6.json').read_text())
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
release['frozen_source'].append({'path':'projects/chirality-piping/tests/test_analysis_run_records.py','sha256':'b40df20cceac6ebcf85ec7111b005fd57cfcad64b46947897ddcabaa91c15a04'})
assert sha(run/'instances/ROOT/_run_records/F12_REVIEW_V1/REVIEW_RETURN_V1.json')=='4bdd5d1bfb5fce919d2b8c2f30598893625c6f7ccf7ef0e96af7e723f3ef8817'
assert all(sha(base/f['path'])==f['sha256'] for f in release['frozen_source'])
out.mkdir()
started=datetime.datetime.now(datetime.timezone.utc).isoformat()
env=dict(os.environ)
env.update({'PYTHONDONTWRITEBYTECODE':'1','PLAYWRIGHT_WORKERS':'1','PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH':'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome','CARGO_NET_OFFLINE':'true'})
py='/private/tmp/piping-foundation-20260914-venv/bin/python'
setup=[py,'tools/serialization/build_checked_json.py']
with (out/'checked_json_setup.log').open('wb') as f:
 result=subprocess.run(setup,cwd=project,env=env,stdout=f,stderr=subprocess.STDOUT)
assert result.returncode==0, 'Explicit checked-JSON setup failed; preserve its log.'
cli=project/'core/serialization/canonical_json/target/checked-json/release/openpipestress_jcs_ijson'
assert cli.is_file()
env['OPENPIPESTRESS_CHECKED_JSON_BIN']=str(cli)
cmd=[py,'tools/release/run_evidence_sweep.py','--execute','--require-capability','host','--output-dir',str(out/'summaries')]
print(json.dumps({'stage':'STARTED','candidate':head,'output':str(out),'checked_cli_sha256':sha(cli),'command':cmd}),flush=True)
with (out/'full_sweep.log').open('wb') as f:
 result=subprocess.run(cmd,cwd=project,env=env,stdout=f,stderr=subprocess.STDOUT)
post_status=subprocess.check_output(['git','status','--porcelain'],cwd=base,text=True)
post_head=subprocess.check_output(['git','rev-parse','HEAD'],cwd=base,text=True).strip()
source_mismatches=[f['path'] for f in release['frozen_source'] if sha(base/f['path'])!=f['sha256']]
summaries=[{'path':str(p),'sha256':sha(p)} for p in sorted((out/'summaries').glob('*.json'))]
record={'schema':'root-complete-dec025-execution-v2','started_utc':started,'finished_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'candidate_commit':head,'working_tree_clean_before':True,'source_review_commit':release['candidate_commit'],'source_files_before_exact':len(release['frozen_source']),'explicit_setup_command':setup,'explicit_setup_exit_code':0,'checked_cli':{'path':str(cli),'sha256':sha(cli)},'command':cmd,'exit_code':result.returncode,'head_after':post_head,'status_after':post_status,'source_mismatches_after':source_mismatches,'summaries':summaries,'logs':{name:sha(out/name) for name in ['checked_json_setup.log','full_sweep.log']},'status':'PASS' if result.returncode==0 and not post_status and post_head==head and not source_mismatches else 'FAIL_OR_REQUIRES_ADJUDICATION'}
(out/'EXECUTION_RESULT_V2.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps({'status':record['status'],'exit_code':result.returncode,'candidate':head,'record':str(out/'EXECUTION_RESULT_V2.json'),'sha256':sha(out/'EXECUTION_RESULT_V2.json')}),flush=True)
sys.exit(0 if record['status']=='PASS' else 1)
