from review_io import *
import collections,stat,datetime,shlex
checks=collections.Counter();bad=[]
def eq(k,a,b):
 checks[k]+=1
 if a!=b:bad.append({'check':k,'actual':a,'expected':b})
bind=packet('INPUT_BINDING.json');source=js(R+'/I28/ordinary_artifacts_02/_run_records/SOURCE_AFTER.json');root=pathlib.Path(bind['source_root'])
eq('source_exact_set',sorted(str(p.relative_to(root)) for p in root.rglob('*') if p.is_file()),sorted(x['relative'] for x in source));eq('source_no_symlinks',sum(p.is_symlink() for p in root.rglob('*')),0)
for x in source:
 p=pathlib.Path(x['path']);b=p.read_bytes();eq('source_current_sha',hashlib.sha256(b).hexdigest(),x['sha256']);eq('source_frozen_sha',hashlib.sha256(get(x['relative'],SOURCE)).hexdigest(),x['sha256']);eq('source_size',len(b),x['bytes']);eq('source_mode',oct(stat.S_IMODE(p.stat().st_mode)),x['mode'])
for k in ['binary','seed']:eq(k+'_hash',hashlib.sha256(pathlib.Path(bind[k]['path']).read_bytes()).hexdigest(),bind[k]['sha256'])
for x in bind['preexisting_files']:eq('prior_nonjournal_file_hash',hashlib.sha256(pathlib.Path(x['path']).read_bytes()).hexdigest(),x['sha256'])
for x in bind['references']:
 path=x['path']
 if '/source/projects/' in path:raw=get('projects/'+path.split('/source/projects/',1)[1],SOURCE)
 else:
  num='/numerics/' in path;raw=get(path.split('/numerics/' if num else '/k6c/',1)[1],NUM if num else REV)
 eq('referenced_hash',hashlib.sha256(raw).hexdigest(),x['sha256'])
t1=archive(1);t2=archive(2);t3=archive(3)
for x in js(R+'/I26/t2_measurements_07/_run_records/RAW_INVENTORY.json')['new_files']:eq('all_T2_raw_hashes',hashlib.sha256(t2[x['relative_to_measurement_root']]).hexdigest(),x['sha256'])
eq('T2_exact_T1_prefix',t2['records/records.jsonl'][:len(t1['records.jsonl'])],t1['records.jsonl'])
journal=list(map(json.loads,t3['records/records.jsonl'].splitlines()));runs=journal[84:];byrid={r['run_id']:r for r in journal};plan={p['run_id']:p for p in packet('T3_LAUNCHES.json')};author={r['run_id']:r for r in packet('FULL_CHECKS.json')['runs']}
old={r['order']:r for r in map(json.loads,get(R.rsplit('/',1)[0]+'/IMPLEMENTATION/K6B/_run_records/b3/records/records.jsonl').splitlines()) if r.get('order') in range(223,247)}
fields=['run_id','model','family','members','mode','pass','repeats','entry_repeats','rss_cap_bytes','heap_cap_bytes','timeout_s','time_budget_s','first_repeat_limit_s','conditional']
independent=json.loads((REC/'ADMISSIONS.json').read_bytes());groups=collections.defaultdict(list);prefix_evidence=[];populations=[];productpids=[]
def observations(rid):return list(map(json.loads,t3['records/'+rid+'.jsonl'].splitlines()))
seed={r['model']:r for r in map(json.loads,get(H+'/observations/k6b/counts.jsonl',SOURCE).splitlines())}
for i,r in enumerate(runs,start=84):
 for k in fields:eq('original_schedule',r[k],old[r['order']][k])
 pop=[q for q in journal[:i] if q['family']==r['family'] and q['mode']==r['mode'] and 100<=q['members']<r['members'] and q['classification']=='ok']
 eq('eligible_size100_rows',len(pop),4);eq('all_from_T2',all(q['tier']=='W1-T2' and q['members']==100 for q in pop),True);eq('independent_population_ids',[q['run_id'] for q in pop],independent[i-84]['population'])
 expected=[{'run_id':q['run_id'],'members':q['members'],'own_estimate_denominator':q['estimate_adm_bytes']} for q in pop];eq('author_denominator_roster',expected,author[r['run_id']]['calibration_population'])
 prev=r['model'].replace('-n01000-','-n00100-');ascent=[q['run_id'] for q in journal[:i] if q['model']==prev and q['mode']==r['mode'] and q['classification'] not in [None,'not_run']];eq('ascent_exact_ids',ascent,author[r['run_id']]['recorded_ascent_run_ids'])
 populations.append({'run_id':r['run_id'],'population':expected,'ascent':ascent})
 oo=observations(r['run_id']);outs=[x for x in oo if x['kind']=='outcome'];groups[(r['model'],r['mode'])].append({k:v for k,v in outs[0].items() if k!='repeat'})
 for x in oo:
  if x.get('kind')=='parity' and x.get('item')=='w1_source_encoding_equals_evidence':eq('source_encoding',x['evidence_source_fnv64'],seed[r['model']]['w1_source_encoding_fnv64'])
  if x.get('kind')=='parity' and x.get('item')=='rcm_count_equals_ordering':eq('RCM_profile',x['ordering_profile_entries'],seed[r['model']]['rcm_profile_entries']);eq('RCM_bandwidth',x['ordering_half_bandwidth'],seed[r['model']]['rcm_half_bandwidth'])
 if r['mode']=='w1a' and r['pass']==1:
  at=[x for x in oo if x['kind']=='attempt' and x['repeat']==4];charge=0;segments=[]
  for j,a in enumerate(at):
   charge+=a['shared_work']+a['own_total']-a['verification_work']-a['stop_rule_work'];segments.append({'label':'solve_'+str(a['precision']),'end':charge})
   if a['verification_work'] or a['verification_shared_work']:
    charge+=a['verification_work']+a['verification_shared_work'];segments.append({'label':'verify_'+str(a['precision']),'end':charge})
    if j and at[j-1]['stop_rule_work']:charge+=at[j-1]['stop_rule_work'];segments.append({'label':'decide_'+str(at[j-1]['precision']),'end':charge})
  actual=[x for x in oo if x['kind']=='prefix'];eq('prefix_complete_exact_order',[(x['through_segment'],x['case_limit']) for x in actual],[(x['label'],x['end']) for x in segments[:-1]]);eq('final_segment_work',segments[-1]['end'],outs[-1]['meter_charged'])
  eq('prefix_stage_ids',[x['stage'] for x in oo if x['kind']=='stage' and x['stage'].startswith('w1_prefix_')],['w1_prefix_'+str(i+1) for i in range(len(segments)-1)])
  prefix_evidence.append({'run_id':r['run_id'],'model':r['model'],'final_call_segments':segments,'required_prefixes':segments[:-1],'actual_prefixes':actual,'complete':True})
for key,outs in groups.items():eq('two_pass_outcome',outs[0],outs[1])
inv=packet('ACTUAL_INVOCATION.json');base=packet('BASELINE_CHECK.json');progress=packet('PROGRESS_CHECK_01.json');end=packet('PROGRESS_CHECK_02.json');timeline=packet('TIMELINE.json');cleanup=packet('FINAL_PRESERVATION_AND_CLEANUP.json');driver=t3['records/tier_W1-T3.driver.log'].decode()
observedgroups=set()
for snap,rid in [(base,runs[0]['run_id']),(progress,next(r['run_id'] for r in runs if r['order']==238))]:
 commands=[x.split(None,3) for x in snap['active_owned_commands']];outer=next(x for x in commands if int(x[0])==inv['observed_outer_PID_PGID']);eq('active_runner_argv',shlex.split(outer[3]),inv['argv'])
 product=next(x for x in commands if shlex.split(x[3])[0]==bind['binary']['path']);eq('active_product_argv',shlex.split(product[3]),plan[rid]['argv']);eq('active_product_PID',int(product[0]),next(x['pid'] for x in observations(rid) if x['kind']=='start'))
 wrapper=next(x for x in commands if int(x[0])==int(product[1]));eq('product_wrapper_group',int(product[2]),int(wrapper[2]));eq('snapshot_group_roster',sorted({int(x[2]) for x in commands}),snap['observed_group_ids']);observedgroups.update(snap['observed_group_ids'])
 eq('snapshot_driver_tail_exists',snap['driver_tail'] in driver,True)
eq('baseline_no_completed_normal_line',sum(': ok ' in l for l in base['driver_tail'].splitlines()),0)
eq('live_completed15',sum(x['complete'] for x in progress['visible_runs']),15);eq('live_ongoing238',[x['order'] for x in progress['visible_runs'] if not x['complete']],[238]);eq('live_peak_count_arithmetic',sum(x['normal_stage_records']*2 for x in progress['visible_runs'] if x['mode']=='w1a'),progress['matched_live_peak_fields'])
for visible in progress['visible_runs']:
 r=next(r for r in runs if r['order']==visible['order']);n=sum(x['kind']=='stage' for x in observations(r['run_id']));eq('snapshot_stage_count_supported',visible['normal_stage_records']==n if visible['complete'] else visible['normal_stage_records']<=n,True)
eq('snapshot_no_faults',progress['faults'],[]);eq('completion_snapshot_ids',[x['order'] for x in end['seen']],list(range(223,247)));eq('completion_rows_full',all(x['complete'] and x['repeats']==5 for x in end['seen']),True);eq('completion_snapshot_no_owned',end['active_owned_commands'],[]);eq('completion_no_faults',end['faults'],[])
eq('time_sequence',timeline['driver_birth_utc']<base['checked_utc']<progress['checked_utc']<timeline['driver_last_write_utc']<end['checked_utc'],True)
for name,bb in t3.items():
 if name.endswith('.jsonl') and name!='records/records.jsonl':
  ob=list(map(json.loads,bb.splitlines()));productpids.extend(o['pid'] for o in ob if o.get('kind')=='start')
eq('all43_products_scanned',sorted(cleanup['all43_raw_product_PIDs_scanned']),sorted(productpids));eq('observed_groups_scanned',sorted(observedgroups),cleanup['previously_observed_group_ids_scanned']);eq('known_groups_remaining',cleanup['observed_groups_remaining'],[]);eq('known_products_remaining',cleanup['known_products_remaining'],[]);eq('owned_commands_remaining',cleanup['exact_owned_runner_wrapper_binary_matches'],[]);eq('outer_reaped',cleanup['outer_exit_code'],0);eq('outer_session',cleanup['outer_session'],inv['outer_session_id']);eq('no_survivor_array_scan_credit',cleanup['default_survivor_array_scan_credit'],False)
eq('driver_stop_result',json.loads(driver.splitlines()[-1]),{'stop':[],'stop_after_tier':[]});eq('no_retry_or_skip',any('already recorded' in x or x.startswith('STOP:') for x in driver.splitlines()),False)
qual=packet('QUALIFICATIONS.json')
for q in qual['calls']:eq('qualification_before_tier',q['end_utc']<inv['dispatch_intent_utc'],True);eq('recorded_guard','5387' in q['guard'] and '/guard/memguard.sh' in q['guard'],True);eq('quiet_bound',q['quiet']['waited_s']<=120,True)
available=[]
for fam in ['CHAIN','TREE','CONT']:
 for mode in ['w1a','sparse']:
  rs=[r for r in journal if r['family']==fam and r['mode']==mode and 100<=r['members']<10000 and r['classification']=='ok'];eq('available_later_roster',len(rs),8);available.append({'family':fam,'mode':mode,'existing_eligible_rows':[r['run_id'] for r in rs],'no_future_admission_evaluated':True})
save('PRESERVATION_AND_SNAPSHOT_CHECKS.json',{'check_counts':dict(checks),'failures':bad,'live_peak_field_count_consistency':progress['matched_live_peak_fields'],'live_scope':'Two retained finite active snapshots; final-data stage counts corroborate the reported182-peak subset. No independent replay of snapshot time or continuous/per-row monitoring asserted.','cleanup_scope':'Recorded final checks of the union of three observed groups, all43 raw product PIDs and exact owned commands; source wrapper waits and outer exit0. No default survivor-array scan credit.','future_available_calibration':available,'checked_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()});save('PREFIX_COMPLETENESS.json',prefix_evidence);save('CALIBRATION_ROSTERS.json',populations);print(json.dumps({'check_counts':dict(checks),'failures':bad},indent=2))
