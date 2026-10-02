from review_io import *
import collections,stat,datetime,shlex
checks=collections.Counter();bad=[]
def eq(name,a,b):
 checks[name]+=1
 if a!=b:bad.append({'check':name,'actual':a,'expected':b})
bind=packet('INPUT_BINDING.json');source=js(R+'/I28/ordinary_artifacts_02/_run_records/SOURCE_AFTER.json');root=pathlib.Path(bind['source_root'])
eq('fixed_source_set',sorted(str(p.relative_to(root)) for p in root.rglob('*') if p.is_file()),sorted(x['relative'] for x in source));eq('fixed_source_no_symlinks',sum(p.is_symlink() for p in root.rglob('*')),0)
for x in source:
 p=pathlib.Path(x['path']);b=p.read_bytes();eq('fixed_source_hash',hashlib.sha256(b).hexdigest(),x['sha256']);eq('frozen_source_git',hashlib.sha256(get(x['relative'],SOURCE)).hexdigest(),x['sha256']);eq('fixed_source_size',len(b),x['bytes']);eq('fixed_source_mode',oct(stat.S_IMODE(p.stat().st_mode)),x['mode'])
for k in ['binary','seed']:eq(k+'_current_hash',hashlib.sha256(pathlib.Path(bind[k]['path']).read_bytes()).hexdigest(),bind[k]['sha256'])
for x in bind['preexisting_files']:eq('earlier_file_hash',hashlib.sha256(pathlib.Path(x['path']).read_bytes()).hexdigest(),x['sha256'])
for x in bind['references']:
 path=x['path'];num='/numerics/' in path;rel=path.split('/numerics/' if num else '/k6c/',1)[1];eq('referenced_file_hash',hashlib.sha256(get(rel,NUM if num else REV)).hexdigest(),x['sha256'])
t2=archive(2);journal=list(map(json.loads,t2['records/records.jsonl'].splitlines()));runs=journal[60:]
old={r['order']:r for r in map(json.loads,get(R.rsplit('/',1)[0]+'/IMPLEMENTATION/K6B/_run_records/b3/records/records.jsonl').splitlines()) if r.get('order') in range(199,223)}
fields=['run_id','model','family','members','mode','pass','repeats','entry_repeats','rss_cap_bytes','heap_cap_bytes','timeout_s','time_budget_s','first_repeat_limit_s','conditional']
groups=collections.defaultdict(list)
for r in runs:
 for k in fields:eq('original_schedule_field',r[k],old[r['order']][k])
 oo=list(map(json.loads,t2['records/'+r['run_id']+'.jsonl'].splitlines()));oc=next(x for x in oo if x['kind']=='outcome');groups[(r['model'],r['mode'])].append({k:v for k,v in oc.items() if k!='repeat'})
 seedrow=next(json.loads(x) for x in get(H+'/observations/k6b/counts.jsonl',SOURCE).splitlines() if json.loads(x)['model']==r['model'])
 for x in oo:
  if x.get('kind')=='parity' and x.get('item')=='w1_source_encoding_equals_evidence':eq('source_digest_join',x['evidence_source_fnv64'],seedrow['w1_source_encoding_fnv64'])
  if x.get('kind')=='parity' and x.get('item')=='rcm_count_equals_ordering':eq('RCM_profile_join',x['ordering_profile_entries'],seedrow['rcm_profile_entries']);eq('RCM_bandwidth_join',x['ordering_half_bandwidth'],seedrow['rcm_half_bandwidth'])
for key,values in groups.items():eq('two_pass_outcome_identity',values[0],values[1])
inv=packet('ACTUAL_INVOCATION.json');base=packet('BASELINE_CHECK.json');progress=packet('PROGRESS_CHECK_01.json');timeline=packet('TIMELINE.json');qual=packet('QUALIFICATIONS.json');cleanup=packet('FINAL_PRESERVATION_AND_CLEANUP.json');driver=t2['records/tier_W1-T2.driver.log'].decode()
eq('baseline_active_command_count',len(base['active_owned_commands']),1)
cmd=base['active_owned_commands'][0].split(None,3);eq('baseline_observed_PID',int(cmd[0]),inv['outer_PID_PGID']);eq('baseline_observed_PGID',int(cmd[2]),inv['outer_PID_PGID']);eq('baseline_observed_argv',shlex.split(cmd[3]),inv['argv'])
eq('baseline_tail_is_actual_prefix',driver.startswith(base['tier_log_tail']),True);eq('baseline_two_completed_rows',sum(': ok ' in l for l in base['tier_log_tail'].splitlines()),2)
eq('baseline_time_record',base['checked_utc'],timeline['baseline_checked_utc']);eq('baseline_check_within_driver_file_interval',timeline['driver_birth_utc']<base['checked_utc']<timeline['driver_last_write_utc'],True)
eq('progress_after_driver_lastwrite',progress['checked_utc']>timeline['driver_last_write_utc'],True);eq('progress_24_rows',progress['completed_T2_rows_checked'],24);eq('progress_ids',[r['run_id'] for r in runs],progress['checked_run_ids']);eq('progress_no_faults',progress['faults'],[]);eq('progress_no_owned_commands',progress['owned_active_commands'],[])
eq('outer_exit',inv['outer_exit_code'],0);eq('final_outer_exit',cleanup['outer_exit_code'],0);eq('final_session',cleanup['outer_exec_session'],inv['outer_exec_session_id']);eq('final_owned_commands',cleanup['owned_runner_wrapper_product_matches'],[])
eq('driver_stop_result',json.loads(driver.splitlines()[-1]),{'stop':[],'stop_after_tier':[]});eq('driver_no_retry',any('already recorded' in l or l.startswith('STOP:') for l in driver.splitlines()),False)
for q in qual['calls']:
 eq('qualification_before_tier',q['end_utc']<inv['dispatch_intent_utc'],True);eq('qualification_guard_named','5387' in q['guard'] and '/guard/memguard.sh' in q['guard'],True);eq('qualification_quiet_bound',q['quiet']['waited_s']<=120,True)
populations=[]
for family in ['CHAIN','TREE','CONT']:
 for mode in ['w1a','sparse']:
  eligible=[r for r in journal if r['family']==family and r['mode']==mode and 100<=r['members']<1000 and r['classification']=='ok'];eq('size100_available_population',len(eligible),4)
  populations.append({'family':family,'mode':mode,'eligible_existing_rows_for_size1000_predicate':[r['run_id'] for r in eligible],'denominators':[r['estimate_adm_bytes'] for r in eligible],'policy':'Existing process-classification ok predicate. Sparse Sensitive is preserved as its numerical outcome and is not relabeled Passed. No future admission evaluated.'})
poll=collections.Counter((r['binary_pid_found'],r['watchdog_polls']) for r in [json.loads(b) for n,b in t2.items() if n.endswith('.record.json')])
save('PRESERVATION_TIMING_AND_READINESS.json',{'check_counts':dict(checks),'failures':bad,'available_calibration_populations':populations,'baseline_observed_while_active_after_two_rows':True,'full_per_row_checks_post_run':True,'T2_grant_does_not_require_pre_first_row_live_gate':True,'successful_survivor_arrays_are_defaults_not_scans':True,'cleanup_credit':'Source-supported wrapper waits, recorded outer exit0, observed empty endpoint command matches. No reviewer process scan or universal absence/intervention claim.','filesystem_times_remain_filesystem_provenance':True,'watchdog_poll_distribution':[{'pid_found':k[0],'polls':k[1],'records':v} for k,v in sorted(poll.items())],'checked_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()});print(json.dumps({'check_counts':dict(checks),'failures':bad},indent=2))
