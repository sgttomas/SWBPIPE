from inspect import *
import ast,re,types,copy,datetime
H_ACTUAL = '--h-actual' in sys.argv
BINDINGS={x['run_id']:x for x in js(REC/'H_ACTUAL_LAUNCH_BINDING.json')['rows']} if H_ACTUAL else {}
PREFIX='ACTUAL_H_' if H_ACTUAL else ''
# Load only named pure admission functions from immutable Git-show snapshots.
s=read(REC/'k6_runner_908.py'); tree=ast.parse(s)
names=['members_of','previous_size','run_entries','refusal_by_name','estimate_key','measured_ratio','peak_footprint','measured_footprint_ratio','rss_to_footprint','admission']
const=['RF_LARGE_SIZES','Q5_GRID_SIDES','N2_MODES','N2_REFUSAL_MEMBERS','RHO_DEFAULT','RSS_TO_FOOTPRINT_DEFAULT','PROJECTED_RSS_FRACTION','RHO_MIN_MEMBERS_FOR_LARGE','LARGE_MEMBERS','P1_LINUX_PEAK_MIB']
ns={'re':re,'MIB':1024**2}
selected=[x for x in tree.body if isinstance(x,ast.FunctionDef) and x.name in names or isinstance(x,ast.Assign) and any(isinstance(t,ast.Name) and t.id in const for t in x.targets)]
assert sum(isinstance(x,ast.FunctionDef) for x in selected)==len(names)
exec(compile(ast.Module(body=selected,type_ignores=[]),str(REC/'k6_runner_908.py'), 'exec'),ns)
k6=types.SimpleNamespace(**ns)
vns={'k6':k6}; vt=ast.parse(read(REC/'vk_scale_runner_908.py')); vf=[x for x in vt.body if isinstance(x,ast.FunctionDef) and x.name=='admission'];assert len(vf)==1
exec(compile(ast.Module(body=vf,type_ignores=[]),str(REC/'vk_scale_runner_908.py'),'exec'),vns)
(REC/'PURE_FUNCTIONS.py.txt').write_text('\n\n'.join(ast.get_source_segment(s,x) for x in selected)+'\n\n'+ast.get_source_segment(read(REC/'vk_scale_runner_908.py'),vf[0]))
HNUM=js(R/'I21/h_numeric_19/PHASES.json')['rows']; HROWS={x['model']:x for x in HNUM}
VRNUM=js(R/'metric_design_15_result5_join/RESULT5_CLI24.json')['rows'];VROWS={x['id']:x for x in VRNUM}
manifest=js(R/'I21/k0_assembly_16/HISTORICAL_COMPARISON.json')
original={(x['dataset'],x['run_id']):x for x in manifest['rows']}
files={'K6B':T3/'IMPLEMENTATION/K6B/_run_records/b3/records/records.jsonl','VK':T3/'IMPLEMENTATION/VK/_run_records/b/runs/records.jsonl','KF3':T3/'IMPLEMENTATION/KF3/_run_records/b/runs/records.jsonl'}
all_rows={d:[json.loads(line) for line in read(p).splitlines() if line.strip()] for d,p in files.items()}
summary={}; replay=[]; peaks=[]; cf=[]; check=[]
def decision(dataset,run,estimate,prior):
 counts={run['model']:{'estimate_adm_bytes_w1a':estimate,'estimate_max_bytes':estimate}}
 br,bf=run.get('baseline_rss_bytes'),run.get('baseline_footprint_bytes')
 # Historical W1-T4 approval is evidenced by the b3 override; VR V3 approval
 # evidenced by actual admitted records. This is retrospective, no new grant.
 if dataset=='K6B':return k6.admission(run,counts,prior,br,baseline_footprint_bytes=bf)
 return vns['admission'](run,counts,prior,br,bf,True)
def corrected(dataset,run):
 if dataset=='K6B':
  h=HROWS[run['model']];variants=[v for v in h['launch_variants'] if v['run_id']==run['run_id']];assert len(variants)==1
  return variants[0]['totals']['all']['moving_bytes'] + (BINDINGS[run['run_id']]['actual_minus_reference_bytes'] if H_ACTUAL else 0)
 return VROWS[run['model']]['metrics']['moving']['fields']['max']
def eligible(run,prior):
 floor=100 if run['members']>=1000 else 0
 return [r for r in prior if 'family' in r and r['family']==run['family'] and r['mode']==run['mode'] and floor<=r['members']<run['members'] and r.get('classification')=='ok']
for dataset,rows in all_rows.items():
 # Use file chronology, never sort away the supplied observation order.
 orders=[x['order'] for x in rows if 'order'in x];assert orders==sorted(orders)
 old_prior=[]; new_prior=[]; cf_prior=[]; diffs=[];old_mismatch=[]
 for run in rows:
  if run.get('mode')!='w1a':
   old_prior.append(run);new_prior.append(copy.deepcopy(run));cf_prior.append(copy.deepcopy(run));continue
  mid=run['model'];rid=run['run_id'];e=corrected(dataset,run);historical=original[(dataset,rid)]
  qual='conditional ordinary-production profile; actual H historical retained arguments bound' if H_ACTUAL and dataset=='K6B' else 'conditional reference-launch arithmetic; actual historical launch/profile qualification unresolved'
  assert historical['old_estimate_bytes']==run['estimate_adm_bytes']
  assert historical['old_admission']==run['admission']
  assert historical['recorded_heap_bytes']==run['repeats_heap_peak']
  assert historical['recorded_move_bytes']==run['repeats_heap_peak_move']
  old=decision(dataset,run,run['estimate_adm_bytes'],old_prior)
  if old!=run['admission']:old_mismatch.append({'run_id':rid,'reproduced':old,'recorded':run['admission']})
  new=decision(dataset,run,e,new_prior); counter=decision(dataset,run,e,cf_prior)
  population=eligible(run,new_prior)
  pop=[]
  for p in population:
   assert p['order']<run['order']
   assert p['estimate_adm_bytes']==corrected(dataset,p)
   pop.append({'run_id':p['run_id'],'order':p['order'],'members':p['members'],'estimate_adm_bytes':p['estimate_adm_bytes'],'footprint_ratio':k6.measured_footprint_ratio(p,run.get('baseline_footprint_bytes')),'rss_ratio':k6.measured_ratio(p,run.get('baseline_rss_bytes')),'rss_to_footprint':k6.rss_to_footprint(p),'net_footprint':max(0,(k6.peak_footprint(p) or 0)-(run.get('baseline_footprint_bytes') or 0)),'heap_move':p['repeats_heap_peak_move']})
  row={'dataset':dataset,'run_id':rid,'order':run['order'],'model':mid,'members':run['members'],'pass':run.get('pass'),'status':qual,'old_estimate_bytes':run['estimate_adm_bytes'],'new_estimate_bytes':e,'estimate_delta_bytes':e-run['estimate_adm_bytes'],'recorded_decision':run['admission'],'old_reproduced':old,'new_as_recorded_history':new,'decision_changed':new['decision']!=run['admission']['decision'],'baseline_rss_bytes':run.get('baseline_rss_bytes'),'baseline_footprint_bytes':run.get('baseline_footprint_bytes'),'calibration_population':pop,'prior_observation_count':len(new_prior),'prior_recorded_ascent_ids':[p['run_id'] for p in new_prior if p.get('model')==k6.previous_size(mid) and p.get('mode')=='w1a' and p.get('classification')not in(None,'not_run')],'historical_approval_preserved':True}
  replay.append(row)
  cf.append({'dataset':dataset,'run_id':rid,'new_as_recorded_history':new,'counterfactual_with_removed_nonadmitted_observations':counter,'lost_calibration_run_ids':[p['run_id'] for p in eligible(run,new_prior) if p['run_id']not in {q['run_id']for q in cf_prior}],'different_decision':counter!=new})
  if row['decision_changed']:diffs.append(rid)
  new_record=copy.deepcopy(run);new_record['estimate_adm_bytes']=e
  old_prior.append(run);new_prior.append(new_record)
  if counter['decision']=='admitted':cf_prior.append(copy.deepcopy(new_record))
  p={'dataset':dataset,'run_id':rid,'model':mid,'old_estimate_bytes':run['estimate_adm_bytes'],'status':qual + '; not historical executable attestation','historical_requested':run['repeats_heap_peak'],'historical_moving':run['repeats_heap_peak_move']}
  if dataset=='K6B':
   variant=copy.deepcopy(next(v for v in HROWS[mid]['launch_variants'] if v['run_id']==rid));t=variant['totals']['all']
   if H_ACTUAL:
    delta=BINDINGS[rid]['actual_minus_reference_bytes']
    variant['Hfixed']+=delta
    for key in t:
     if t[key]!=0:t[key]+=delta
    p['historical_retained_argument_bytes']=BINDINGS[rid]['retained_string_utf8_bytes']
    p['actual_minus_reference_bytes']=delta
   p.update(metric_window='H source/solve staged summary; separate outer prefix',requested_bound=max(t['source_requested'],t['solve_requested']),moving_bound=max(t['source_moving'],t['solve_moving']),fixed=variant['Hfixed'],reference_persistent_argument_bytes=variant['persistent_argument_bytes'])
   for suffix,metric in [('requested','heap_peak'),('moving','heap_peak_move')]:
    p['prefix_'+suffix+'_observed']=run.get('summary',{}).get('prefix_phase_'+metric)
    p['prefix_'+suffix+'_bound']=t['prefix_'+suffix] if variant['prefix'] else None
    prefix=p['prefix_'+suffix+'_observed']
    p['prefix_'+suffix+'_slack']=None if prefix is None else p['prefix_'+suffix+'_bound']-prefix
   p['stage_checks']={name:{'requested_observed':st['heap_peak'],'moving_observed':st['heap_peak_move'],'requested_bound':t['source_requested' if name=='w1_source' else 'solve_requested' if name=='w1_solve' else 'prefix_requested'],'moving_bound':t['source_moving' if name=='w1_source' else 'solve_moving' if name=='w1_solve' else 'prefix_moving']}for name,st in run['stages'].items() if name.startswith('w1_')}
  else:
   v=VROWS[mid];p.update(metric_window='VR global summary including parse/output/RCM/parity',requested_bound=v['metrics']['requested']['fields']['max'],moving_bound=v['metrics']['moving']['fields']['max'],fixed=v['metrics']['moving']['fields']['fixed'],model_component=v['metrics']['moving']['fields']['model'])
  p['requested_slack']=p['requested_bound']-p['historical_requested'];p['moving_slack']=p['moving_bound']-p['historical_moving'];peaks.append(p)
 summary[dataset]={'rows':sum(x.get('mode')=='w1a'for x in rows),'old_exact_reproduction_mismatches':old_mismatch,'changed_decision_run_ids':diffs,'new_decisions':{d:sum(x['dataset']==dataset and x['new_as_recorded_history']['decision']==d for x in replay)for d in ['admitted','deferred','never']}}
assert len(replay)==102 and len(peaks)==102
assert all(p['requested_slack']>=0 and p['moving_slack']>=0 for p in peaks)
assert all(p.get('prefix_requested_slack')is None or p['prefix_requested_slack']>=0 for p in peaks)
assert all(p.get('prefix_moving_slack')is None or p['prefix_moving_slack']>=0 for p in peaks)
assert all(st['requested_observed']<=st['requested_bound'] and st['moving_observed']<=st['moving_bound']for p in peaks for st in p.get('stage_checks',{}).values())
check={'time_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'rows':102,'all_summary_and_prefix_peaks_below_reference_bounds':True,'all_eligible_denominators_replaced':True,'only_prior_observations_used':True,'independent_old_replay_mismatch_count':sum(len(x['old_exact_reproduction_mismatches'])for x in summary.values()),'counterfactual_decision_differences':sum(x['different_decision']for x in cf),'counterfactual_lost_calibration_rows':sum(bool(x['lost_calibration_run_ids'])for x in cf)}
save(PREFIX+'REPLAY.json',replay);save(PREFIX+'PEAK_COMPARISONS.json',peaks);save(PREFIX+'COUNTERFACTUAL.json',cf);save(PREFIX+'SUMMARY.json',summary);save(PREFIX+'CHECKS.json',check)
print(json.dumps({'summary':summary,'checks':check},indent=2))
