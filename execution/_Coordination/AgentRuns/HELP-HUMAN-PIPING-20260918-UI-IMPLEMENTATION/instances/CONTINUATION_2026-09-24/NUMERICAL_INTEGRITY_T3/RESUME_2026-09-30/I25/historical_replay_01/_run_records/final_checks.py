from inspect import *
import datetime,collections
# Hash-only check of 12 retained external model inputs; no parsing/generation.
WT=pathlib.Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3')
xs=js(R/'metric_design_14_reference_callers/_run_records/CLI_EXTERNAL_INPUTS.json');checks=[]
for x in xs:
 p=WT/'scratch/i17/b_models'/(x['id']+'.json'); b=p.read_bytes();got=hashlib.sha256(b).hexdigest(); assert got==x['sha256'] and len(b)==x['bytes'];checks.append({'id':x['id'],'path':str(p),'sha256':got,'bytes':len(b)})
save('EXTERNAL_INPUT_HASH_CHECK.json',checks)
for pre in ['','ACTUAL_H_']:
 x=js(REC/(pre+'REPLAY.json'))
 for r in x:
  r['calibration_evaluated_by_admission']=r['new_as_recorded_history'].get('rho') is not None
  r['calibration_population_note']='Eligible prior-only population is shown for audit; on VR half-cap early return it is not evaluated or used by admission.'
 save(pre+'REPLAY.json',x)
# Freeze source origins by checking every snapshot against its exact Git-show source,
# without importing or executing full runners or touching the index.
specs=[('k6_runner_908.py',H/'runner/k6_runner.py','9086964a1fb656a76cda6d1002d8594efa636fdc'),('vk_scale_runner_908.py',VR/'runner/vk_scale_runner.py','9086964a1fb656a76cda6d1002d8594efa636fdc'),('h_envelope_908.rs',H/'src/k6/w1/h_envelope.rs','9086964a1fb656a76cda6d1002d8594efa636fdc'),('vr_envelope_908.rs',VR/'src/envelope.rs','9086964a1fb656a76cda6d1002d8594efa636fdc'),('h_main_908.rs',H/'src/bin/k6_observe/main.rs','9086964a1fb656a76cda6d1002d8594efa636fdc'),('k6_runner_historical_4eeb.py.txt',H/'runner/k6_runner.py','4eeb206c09b67b9c5f83ca89f5e66d188ad5a02b'),('h_main_historical_4eeb.rs.txt',H/'src/bin/k6_observe/main.rs','4eeb206c09b67b9c5f83ca89f5e66d188ad5a02b')]
origins=[]
for local,path,sha in specs:
 cmd=['git','show',sha+':'+str(path)];r=subprocess.run(cmd,env=dict(os.environ,GIT_OPTIONAL_LOCKS='0'),capture_output=True,check=True);b=(REC/local).read_bytes();assert r.stdout==b
 origins.append({'source_commit':sha,'source_path':str(path),'local_snapshot':local,'sha256':hashlib.sha256(b).hexdigest(),'command':cmd,'environment':{'GIT_OPTIONAL_LOCKS':'0'},'exit_code':r.returncode,'stderr':r.stderr.decode()})
save('SOURCE_ORIGINS.json',origins)
x=js(REC/'ACTUAL_H_REPLAY.json'); p=js(REC/'ACTUAL_H_PEAK_COMPARISONS.json')
summary={'status':'conditional source/records replay complete with finite VR historical-launch gaps','source_candidate':'9086964a1fb656a76cda6d1002d8594efa636fdc','original_rows':102,'old_admission_exact_reproductions':102,'H_historical_path_bound_rows':66,'VR_reference_only_rows':36,'admission_decision_changes':12,'VR10000_deferrals':12,'H_decision_changes':0,'counterfactual_cascade_count':0,'external_input_hashes':12,'requested_and_moving_row_comparisons':204,'prefix_row_comparisons':66,'minimum_slack_by_dataset':{d:{m:min(r[m+'_slack']for r in p if r['dataset']==d)for m in ['requested','moving']}for d in ['K6B','VK','KF3']},'calculation_completed_at':datetime.datetime.now(datetime.timezone.utc).isoformat()}
save('FINAL_CHECKS.json',summary)
print(json.dumps(summary,indent=2))
