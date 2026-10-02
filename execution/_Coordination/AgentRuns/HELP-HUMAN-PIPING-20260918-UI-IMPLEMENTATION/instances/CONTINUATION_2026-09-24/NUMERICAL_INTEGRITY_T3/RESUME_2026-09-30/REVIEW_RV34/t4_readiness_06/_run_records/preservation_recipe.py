from review_io import *
import collections,stat,ast,datetime
checks=collections.Counter();bad=[]
def eq(k,a,b):
 checks[k]+=1
 if a!=b:bad.append({'check':k,'actual':a,'expected':b})
bind=packet('INPUT_BINDING.json');recipe=packet('EXECUTION_AND_COMPARISON_RECIPE.json');src=js(R+'/I28/ordinary_artifacts_02/_run_records/SOURCE_AFTER.json');root=pathlib.Path(bind['source_root'])
eq('source_set',sorted(str(p.relative_to(root)) for p in root.rglob('*') if p.is_file()),sorted(x['relative'] for x in src));eq('source_no_symlinks',sum(p.is_symlink() for p in root.rglob('*')),0)
for x in src:
 p=pathlib.Path(x['path']);b=p.read_bytes();eq('current_source_sha',hashlib.sha256(b).hexdigest(),x['sha256']);eq('Git_source_sha',hashlib.sha256(get(x['relative'],SOURCE)).hexdigest(),x['sha256']);eq('source_mode',oct(stat.S_IMODE(p.stat().st_mode)),x['mode']);eq('source_size',len(b),x['bytes'])
for k in ['binary','seed']:eq(k+'_identity',hashlib.sha256(pathlib.Path(bind[k]['path']).read_bytes()).hexdigest(),bind[k]['sha256'])
for x in bind['preexisting_files']:eq('prior_file_sha',hashlib.sha256(pathlib.Path(x['path']).read_bytes()).hexdigest(),x['sha256'])
for x in bind['references']:
 p=x['path']
 if '/source/projects/' in p:b=get('projects/'+p.split('/source/projects/',1)[1],SOURCE)
 else:
  num='/numerics/' in p;b=get(p.split('/numerics/' if num else '/k6c/',1)[1],NUM if num else REV)
 eq('reference_sha',hashlib.sha256(b).hexdigest(),x['sha256'])
for tier in [1,2,3]:
 data=archive(tier);cap=js(R+'/MEASUREMENTS/W1_T'+str(tier)+'/_run_records/CAPTURE.json');name='records.tar.gz' if tier==1 else 'delta_records.tar.gz';eq('prior_archive_identity',hashlib.sha256(get(R+'/MEASUREMENTS/W1_T'+str(tier)+'/_run_records/'+name)).hexdigest(),cap['archive_sha256'])
launch=js(R+'/I26/measurement_plan_03/_run_records/LAUNCH_MANIFEST.json');planned=[p for p in launch['launches'] if p['tier']=='W1-T4'];t3=list(map(json.loads,archive(3)['records/records.jsonl'].splitlines()))[84:]
eq('future_wrapper_argv',recipe['original_T4_wrapper_argv'],next(t['argv'] for t in launch['tier_cli'] if t['tier']=='W1-T4'))
for argkey,hashkey in [('original_T4_wrapper_argv','wrapper_sha256'),('R1_argv','R1_script_sha256')]:
 p=pathlib.Path(recipe[argkey][1]);rel=str(p).split('/k6c/',1)[1];b=get(rel);eq('recipe_script_Git_hash',hashlib.sha256(b).hexdigest(),recipe[hashkey]);eq('recipe_script_current_hash',hashlib.sha256(p.read_bytes()).hexdigest(),recipe[hashkey])
refpath=recipe['reference_path'].split('/k6c/',1)[1];refs=get(refpath);eq('reference_script_input_hash',hashlib.sha256(refs).hexdigest(),recipe['reference_sha256']);eq('reference_current_hash',hashlib.sha256(pathlib.Path(recipe['reference_path']).read_bytes()).hexdigest(),recipe['reference_sha256'])
eq('R1_argument_binding',recipe['R1_argv'][2:],[str(root/H/'runner'),recipe['reference_path'],recipe['prospective_input_directory']]);eq('comparison_directory_not_created',pathlib.Path(recipe['prospective_input_directory']).exists(),False)
for p in planned:
 if '--dump-published' in p['argv']:eq('no_future_publication_dump',pathlib.Path(p['argv'][p['argv'].index('--dump-published')+1]).exists(),False)
 eq('named_hold_required',p['conditional_ROOT_release_required'],True)
eq('recipe_orders',recipe['schedule']['orders'],[p['order'] for p in planned]);eq('first_pass_orders',recipe['schedule']['first_pass_W1_dump_orders'],[p['order'] for p in planned if p['mode']=='w1a' and p['pass']==1]);eq('normal24',recipe['schedule']['normal_processes'],24);eq('conditional_product37',recipe['schedule']['product_count_if_all_rows_reached'],24+12+1)
for mode in ['w1a','sparse']:
 rows=[r for r in t3 if r['mode']==mode];times=[r['wall_s'] for r in rows];expected=recipe['runtime_estimate_basis'][mode];eq('observed_time_processes',expected['observed_processes'],len(rows));eq('observed_time_min',expected['min_wall_s'],min(times));eq('observed_time_max',expected['max_wall_s'],max(times));eq('observed_time_sum',round(expected['sum_wall_s'],4),round(sum(times),4))
acq=packet('ACQUISITIONS.json');raw=archive(4);pids=[next(json.loads(x)['pid'] for x in raw[c['run_id']+'.jsonl'].splitlines() if json.loads(x)['kind']=='start') for c in acq['calls']];cleanup=packet('RUNTIME_CLEANUP.json');inventory=packet('RAW_INVENTORY.json')
eq('cleanup_known13',sorted(cleanup['known_product_PIDs']),sorted(pids));eq('cleanup_empty',cleanup['remaining'],[]);eq('cleanup_reaped',cleanup['outer_session_exit0'],True);eq('cleanup_default_credit',cleanup['normal_return_survivor_scan_credit'],False);eq('final_known13',sorted(inventory['known_product_pids']),sorted(pids));eq('final_owned_empty',inventory['remaining_known_or_exact_owned_commands'],[])
eq('calls_sequential',all(a['end_utc']<=b['start_utc'] for a,b in zip(acq['calls'],acq['calls'][1:])),True);eq('runtime_before_cleanup',acq['runtime_end_utc']<=cleanup['checked_utc'],True);eq('no_normal_or_R1',acq['no_normal_run_or_R1'],True)
# This is a static check of the preserved scripts; neither script is executed.
wrapper=get(recipe['original_T4_wrapper_argv'][1].split('/k6c/',1)[1]).decode();comp=get(recipe['R1_argv'][1].split('/k6c/',1)[1]).decode();runner=get(H+'/runner/k6_runner.py',SOURCE).decode()
eq('unchanged_wrapper_named_hold',"r.CONDITIONAL_TIERS = {}" in wrapper and "r.run_tier('W1-T4'" in wrapper,True)
eq('R1_exact_fraction_tolerance',"TOL = Fr(1, 10 ** 9)" in comp and "bound = TOL * max(abs(exp), scales.get(kind, Fr(0)))" in comp and "diff = abs(Fr(o) - exp)" in comp,True)
eq('R1_missing_expected_fails',"failures += fails + missing" in comp and "return 1 if failures else 0" in comp,True)
cases=json.loads(refs)['cases'];expected_counts={}
for model in sorted({p['model'] for p in planned}):
 n=len(cases[model]['expected']);wanted=103 if 'CHAIN' in model else 194 if 'TREE' in model else 215;eq('unchanged_analytic_subset',n,wanted);expected_counts[model]=n
save('PRESERVATION_AND_RECIPE_CHECKS.json',{'check_counts':dict(checks),'failures':bad,'analytic_subset_sizes':expected_counts,'static_recipe_disposition':{'wrapper_unchanged':True,'sole_named_hold_future_release_only':True,'R1_script_reference_predicate_unchanged':True,'mandatory_preflight':'Every first-pass outcome must be accounted for; a Selected result requires its complete hash-verified dump; absent/unpublished cases receive no passing value check. The comparator does not detect an entirely missing file by itself, so this recipe preflight remains mandatory.','TREE_scope':'Expected-unpublished TREE is accounted separately. Changed supported outcome/precision/row facts stop for investigation and produced dumps are preserved.','hash_scope':'FNV and SHA identities remain distinct; reviewed available-field schema mapping only. Full historical published-value equality remains unestablished.','duration_scope':'1000-member observed wall times verified; no10000-member ETA or memory guarantee inferred.'},'checked_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()});print(json.dumps({'check_counts':dict(checks),'failures':bad},indent=2))
