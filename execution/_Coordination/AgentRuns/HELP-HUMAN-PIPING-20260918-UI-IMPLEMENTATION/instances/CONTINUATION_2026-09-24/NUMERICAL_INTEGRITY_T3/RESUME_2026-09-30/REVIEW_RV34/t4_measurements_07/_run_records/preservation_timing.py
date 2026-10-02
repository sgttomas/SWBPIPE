from review_io import *
import stat,datetime,collections,shlex
checks=collections.Counter();bad=[]
def eq(k,a,b):
 checks[k]+=1
 if a!=b:bad.append({'check':k,'actual':a,'expected':b})
def moment(s):return datetime.datetime.fromisoformat(s.replace('Z','+00:00'))
bind=packet('INPUT_BINDING.json');source=js(R+'/I28/ordinary_artifacts_02/_run_records/SOURCE_AFTER.json');root=pathlib.Path(bind['source_root'])
eq('source_exact_set',sorted(str(p.relative_to(root)) for p in root.rglob('*') if p.is_file()),sorted(x['relative'] for x in source));eq('source_no_symlink',sum(p.is_symlink() for p in root.rglob('*')),0)
for x in source:
 p=pathlib.Path(x['path']);b=p.read_bytes();eq('source_sha',hashlib.sha256(b).hexdigest(),x['sha256']);eq('source_Git_sha',hashlib.sha256(get(x['relative'],SOURCE)).hexdigest(),x['sha256']);eq('source_size',len(b),x['bytes']);eq('source_mode',oct(stat.S_IMODE(p.stat().st_mode)),x['mode'])
for k in ['binary','seed']:eq(k+'_current_sha',hashlib.sha256(pathlib.Path(bind[k]['path']).read_bytes()).hexdigest(),bind[k]['sha256'])
for x in bind['preexisting_files']:eq('prior_file_sha',hashlib.sha256(pathlib.Path(x['path']).read_bytes()).hexdigest(),x['sha256'])
for x in bind['references']:
 path=x['path']
 if '/source/projects/' in path:b=get('projects/'+path.split('/source/projects/',1)[1],SOURCE)
 else:b=get(path.split('/numerics/' if '/numerics/' in path else '/k6c/',1)[1])
 eq('reference_preserved_through_placement',hashlib.sha256(b).hexdigest(),x['sha256'])
raw=archive(4);inv=packet('ACTUAL_INVOCATION.json');plans={p['run_id']:p for p in packet('T4_LAUNCHES.json')};journal=list(map(json.loads,raw['records/records.jsonl'].splitlines()));runs=journal[108:];byorder={r['order']:r for r in runs}
expected={x['path'] for x in bind['preexisting_files']}|{x['path'] for x in packet('RAW_INVENTORY.json')['new_files']}|{bind['journal_prefix']['path']};actual={str(p) for p in pathlib.Path(bind['records']).parent.rglob('*') if p.is_file()};eq('measurement_exact_file_set',sorted(actual),sorted(expected))
eq('actual_journal_equals_snapshot',pathlib.Path(bind['journal_prefix']['path']).read_bytes(),raw['records/records.jsonl'])
for tier in [1,2,3]:
 cap=js(R+'/MEASUREMENTS/W1_T'+str(tier)+'/_run_records/CAPTURE.json');name='records.tar.gz' if tier==1 else 'delta_records.tar.gz';eq('prior_archive_hash',hashlib.sha256(get(R+'/MEASUREMENTS/W1_T'+str(tier)+'/_run_records/'+name)).hexdigest(),cap['archive_sha256'])
recipe=js(R+'/I26/t4_readiness_09/_run_records/EXECUTION_AND_COMPARISON_RECIPE.json');eq('exact_wrapper_argv',inv['argv'],recipe['original_T4_wrapper_argv']);eq('wrapper_sha',hashlib.sha256(get(inv['argv'][1].split('/k6c/',1)[1])).hexdigest(),inv['wrapper_sha256']);eq('one_wrapper',inv['tier_cli_launches'],1);eq('hold_release_record',inv['named_hold_release_authorized'],True);eq('no_numeric_override',inv['numeric_override'],False)
driver=raw['records/tier_W1-T4.driver.log'].decode();eq('override_logged_once',driver.count('override: CONDITIONAL_TIERS emptied for W1-T4 (ROOT approval)'),1);eq('normal_stop_result',json.loads(driver.splitlines()[-1]),{'stop':[],'stop_after_tier':[]});eq('no_repeat_or_skip',('already recorded' in driver or 'STOP:' in driver),False)
snapshots=[packet('BASELINE_CHECK.json')]+[packet('PROGRESS_CHECK_%02d.json'%i) for i in range(1,20)];seen_groups=set();lastcomplete=set();snapshot_summary=[]
for snap in snapshots:
 complete={x['order'] for x in snap.get('seen',[]) if x['completed']};eq('completion_monotone',lastcomplete<=complete,True) if 'seen' in snap else None
 if 'seen' in snap:lastcomplete=complete
 for seen in snap.get('seen',[]):
  r=byorder[seen['order']];oo=list(map(json.loads,raw['records/'+r['run_id']+'.jsonl'].splitlines()));stages=sum(x['kind']=='stage' for x in oo);eq('snapshot_stages_supported',seen['stage_records']==stages if seen['completed'] else seen['stage_records']<=stages,True)
 for o in snap.get('outcomes_available',[]):
  r=byorder[o['order']];oo=list(map(json.loads,raw['records/'+r['run_id']+'.jsonl'].splitlines()));ot=next(x for x in oo if x['kind']=='outcome' and x['repeat']==o['repeat'])
  for k,rk in [('class','class'),('reason','reason'),('selected','selected_precision'),('verification','verification_precision'),('rows','rows')]:eq('snapshot_outcome_supported',o.get(k),ot.get(rk))
 seen_groups.update(snap.get('observed_group_ids',[]));commands=[x.split(None,3) for x in snap.get('active_owned_commands',[])];eq('observed_group_ids',sorted({int(x[2]) for x in commands}),snap.get('observed_group_ids',[]))
 for command in commands:
  argv=shlex.split(command[3])
  if argv and argv[0]==bind['binary']['path']:
   # Both normal/count binding have the same model with an optional counts-only flag.
   model=argv[argv.index('--model')+1] if '--model' in argv else None
   candidates=[p for p in plans.values() if p['model']==model]
   eq('observed_actual_product_argv',any(argv in [p['argv'],p['counts_binding_argv']] for p in candidates),True)
 if 'faults' in snap:eq('snapshot_no_faults',snap['faults'],[])
 snapshot_summary.append({'checked_utc':snap['checked_utc'],'completed_rows':len(complete),'active_commands':len(commands),'observed_groups':snap.get('observed_group_ids',[])})
continuation=packet('CONTINUATION_AUTHORITY.json');ruling=get(T3+'/ROOT_RULINGS_V1.md',continuation['ruling_commit']).decode();section=ruling[ruling.index('## Explicit final-family continuation boundary'):];eq('extension_named_commit',continuation['ruling_commit'],'bf234cf6d253692cfd3451033d2932709c04fcd4');eq('extension_CONT_only',continuation['only_remaining_authorized_orders'],list(range(263,271)));eq('extension_not_automatic',continuation['automatic_extension'],False);eq('extension_no_retry',continuation['retry_authorized'],False)
p12=packet('PROGRESS_CHECK_12.json');eq('through262_before_old_cutoff',moment(p12['checked_utc'])<moment(continuation['original_normal_cutoff']) and all(any(x['order']==order and x['completed'] for x in p12['seen']) for order in range(247,263)),True);eq('continuation_received_before_old_cutoff',moment(continuation['received_utc_approx'])<moment(continuation['original_normal_cutoff']),True)
timeline=packet('TIMELINE.json');normal=moment(timeline['normal_completion_first_observed']);eq('normal_within_revised_cutoff',normal<=moment(continuation['new_normal_cutoff']),True);eq('normal_after_old_cutoff',normal>moment(continuation['original_normal_cutoff']),True)
r1inv=packet('R1_INVOCATION.json');r1done=packet('R1_COMPLETION.json');eq('critical_checks_before_R1',moment(packet('FULL_CHECKS.json')['summary']['checked_utc'])<moment(packet('R1_INPUT_VIEW.json')['created_utc'])<moment(r1inv['dispatch_intent_utc']),True);eq('R1_after_complete_normal',normal<moment(r1inv['dispatch_intent_utc']),True);eq('R1_within_revised_cutoff',moment(r1done['observed_complete_utc'])<=moment(continuation['new_comparator_cutoff']),True);eq('R1_under_10minute_bound',(moment(r1done['observed_complete_utc'])-moment(r1inv['dispatch_intent_utc'])).total_seconds()<600,True)
eq('review_dispatch_packet_available_before_return_deadline',moment('2026-10-02T15:48:22Z')<moment(continuation['new_return_deadline']),True)
pids=[next(x['pid'] for x in map(json.loads,b.splitlines()) if x['kind']=='start') for n,b in raw.items() if n.startswith('records/') and n.endswith('.jsonl') and n!='records/records.jsonl'];clean=packet('FINAL_PRESERVATION_AND_CLEANUP.json');eq('37_product_PID_set',sorted(clean['known_product_PIDs_checked']),sorted(pids));eq('observed_group_union_scanned',clean['observed_groups_checked'],sorted(seen_groups));eq('known_remaining',clean['known_or_observed_remaining'],[]);eq('owned_remaining',clean['exact_owned_commands_remaining'],[]);eq('normal_outer_exit',clean['normal_outer_exit'],0);eq('R1_exit',clean['R1_exit'],0);eq('default_arrays_not_scans',clean['default_success_survivors_arrays_are_not_scans'],True)
save('PRESERVATION_TIMING_CHECKS.json',{'check_counts':dict(checks),'failures':bad,'snapshot_summary':snapshot_summary,'continuation_ruling_sha256':hashlib.sha256(ruling.encode()).hexdigest(),'continuation_section_sha256':hashlib.sha256(section.encode()).hexdigest(),'continuation':continuation,'normal_completion_observed':timeline['normal_completion_first_observed'],'R1_completion_observed':r1done['observed_complete_utc'],'timing_limit':'Finite observations and explicitly labelled file mtimes; no exact unobserved process endpoints or continuous supervision inferred. Continuation receipt is labelled approximate.','relocation_ledger_sha256':hashlib.sha256(gitraw(R+'/verification/k6c_record_placement_01/_run_records/RELOCATION.json',REV)).hexdigest(),'checked_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()});print(json.dumps({'checks':sum(checks.values()),'failures':bad},indent=2))
