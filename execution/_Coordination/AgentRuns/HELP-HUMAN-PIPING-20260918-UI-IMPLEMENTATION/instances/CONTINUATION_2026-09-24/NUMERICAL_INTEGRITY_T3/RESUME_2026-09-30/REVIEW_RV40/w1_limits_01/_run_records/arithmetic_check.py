"""One-off RV40 standard-library extraction/arithmetic; no solver execution."""
import pathlib,json,tarfile,statistics,collections,subprocess,os,hashlib
NUM=pathlib.Path(__file__).resolve().parents[0]
while NUM.name!='numerics': NUM=NUM.parent
K6C=NUM.parent/'k6c'
R=pathlib.Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30')
OUT=NUM/R/'REVIEW_RV40/w1_limits_01/_run_records'
inputs=[]
def read(p):
 b=p.read_bytes(); inputs.append({'path':str(p),'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()});return b
proposal=json.loads(read(NUM/R/'I29/w1_limits_preparation_01/_run_records/CALCULATIONS.json'))
prows={r['run_id']:r for r in proposal['W1_process_rows']}
rows=[];raw_records={};attempt_checks=0;member_origins=[];mismatches=[]
for tier in range(1,5):
 rel=R/f'MEASUREMENTS/W1_T{tier}/_run_records'/('records.tar.gz' if tier==1 else 'delta_records.tar.gz')
 read(K6C/rel)
 with tarfile.open(K6C/rel) as t:
  for n in t.getnames():
   if n.endswith('.record.json') and '.counts-binding.' not in n and (tier==1 or n.startswith('records/')):
    z=json.load(t.extractfile(n));raw_records[z['run_id']]=z
  for n in sorted(t.getnames()):
   if (tier!=1 and not n.startswith('records/')) or not n.endswith('_w1a.jsonl'):continue
   payload=t.extractfile(n).read(); rr=[json.loads(line) for line in payload.splitlines() if line.strip()]
   start=next(x for x in rr if x['kind']=='start')
   summary=next(x for x in rr if x['kind']=='summary')
   if start['counts_only'] or summary['repeats_completed']==0:continue
   name=n.removeprefix('records/').removesuffix('.jsonl');rec=raw_records[name]
   member_origins.append({'archive':str(rel),'member':n,'sha256':hashlib.sha256(payload).hexdigest()})
   outcomes=[x for x in rr if x['kind']=='outcome'];assert len(outcomes)==summary['repeats_completed']==5
   counts=next(x for x in rr if x['kind']=='counts')
   attempts=[x for x in rr if x['kind']=='attempt']
   stages=[x for x in rr if x['kind']=='stage']
   solve=[x for x in stages if x['stage']=='w1_solve']; assert len(solve)==5
   o=outcomes[0]; first=[x for x in attempts if x['repeat']==o['repeat']]
   for o2 in outcomes:
    aa=[x for x in attempts if x['repeat']==o2['repeat']]
    full=sum(x['own_total']+x['shared_work']+x['verification_shared_work'] for x in aa)
    invoked=sum(x['own_total']+(x['shared_work'] if x['shared_built_here'] else 0)+(x['verification_shared_work'] if x['verification_shared_built_here'] else 0) for x in aa)
    assert full==invoked==o2['meter_charged']==sum(x['charged_by'] for x in aa)
    assert o2['meter_charged']==o['meter_charged']
    for x in aa:
     assert x['charged_by']==x['own_total']+x['shared_work']+x['verification_shared_work']
     assert x['stages_complete']
     attempt_checks+=1
   d={'tier':f'T{tier}','model':start['model'],'run_id':name,'class':o['class'],'reason':o['reason'],
      'selected_precision':o['selected_precision'],'verification_precision':o['verification_precision'],
      'repeats':len(outcomes),'lme_per_repeat':o['meter_charged'],
      'own_lme':sum(x['own_total'] for x in first),'shared_lme':sum(x['shared_work']+x['verification_shared_work'] for x in first),
      'solve_median_s':statistics.median(x['elapsed_ns'] for x in solve)/1e9,
      'solve_heap_requested_max_B':max(x['heap_peak'] for x in solve),
      'solve_heap_moving_max_B':max(x['heap_peak_move'] for x in solve),
      'stage_requested_max_B':max(x['heap_peak'] for x in stages),
      'stage_moving_max_B':max(x['heap_peak_move'] for x in stages),
      'whole_process_RSS_B':rec['peak_rss_bytes'],'whole_process_footprint_B':rec['rss']['time_peak_footprint_bytes'],
      'whole_process_wall_s':rec['wall_s'],'conditional_full_H_estimate_B':counts['estimate_adm_bytes_w1a'],
      'heap_cap_B':start['heap_cap_bytes'],'rss_cap_B':rec['rss_cap_bytes'],'load_before':rec['load_before'],'load_after':rec['load_after'],
      'prefixes':[x for x in rr if x['kind']=='prefix'],
      'first_attempts':[{k:x[k] for k in ['precision','role','outcome','own_total','shared_work','verification_shared_work','storage_pattern_entries','storage_profile_entries','storage_limbs_per_entry']} for x in first]}
   for k,v in d.items():
    if k in prows[name] and k not in ['prefixes'] and prows[name][k]!=v:mismatches.append([name,k,prows[name][k],v])
   rows.append(d)
assert len(rows)==66 and sum(x['repeats'] for x in rows)==330
groups=[]
for g in proposal['coverage_groups']:
 rr=[x for x in rows if x['tier']==g['tier'] and (x['model'].startswith(g['family']) if g['family']!='DEC053' else not x['model'].startswith('RF-LARGE'))]
 result={'tier':g['tier'],'family':g['family'],'models':len(set(x['model'] for x in rr)),
         'processes':len(rr),'repeats':sum(x['repeats'] for x in rr)}
 for k in ['lme_per_repeat','solve_median_s','solve_heap_requested_max_B','solve_heap_moving_max_B','stage_requested_max_B','stage_moving_max_B','whole_process_RSS_B','whole_process_footprint_B','whole_process_wall_s','conditional_full_H_estimate_B']:
  result[k]=[min(x[k] for x in rr),max(x[k] for x in rr)]
 for k,v in result.items():
  if g[k]!=v:mismatches.append([g['tier']+' '+g['family'],k,g[k],v])
 groups.append(result)
vr=[];vr_attempt_checks=0;a1_deltas=[];historical_origins=[]
env={**os.environ,'GIT_OPTIONAL_LOCKS':'0'}
for p in sorted((K6C/'projects/chirality-piping/validation/benchmarks/numerical_robustness/observations/kernel_lane').glob('rf_*.json')):
 rr=json.loads(read(p));rel=p.relative_to(K6C)
 oldbytes=subprocess.check_output(['git','show',f'37bff17808:{rel}'],cwd=K6C,env=env)
 historical_origins.append({'revision':'37bff17808','path':str(rel),'sha256':hashlib.sha256(oldbytes).hexdigest()})
 old={x['id']:x for x in json.loads(oldbytes)}
 for x in rr:
  charge=sum(a['own_work']+a['exact_sum_work']+(a['shared_work'] if a['shared_built_here'] else 0)+(a['verification_shared_work'] if a['verification_shared_built_here'] else 0) for a in x['attempts'])
  assert charge==x['invocation_charged'];vr_attempt_checks+=len(x['attempts'])
  assert old[x['id']]['outcome']==x['outcome']
  delta=charge-old[x['id']]['invocation_charged']
  if delta:a1_deltas.append({'id':x['id'],'before_LME':old[x['id']]['invocation_charged'],'current_LME':charge,'delta_LME':delta})
 result={'family':rr[0]['family'],'cases':len(rr),'selected':sum(x.get('selected_precision') is not None for x in rr),
         'lme_min':min(x['invocation_charged'] for x in rr),'lme_max':max(x['invocation_charged'] for x in rr),
         'precisions_reached':sorted(set(a['precision'] for x in rr for a in x['attempts'])),
         'outcomes':dict(collections.Counter(x['outcome'] for x in rr))}
 pg=next(x for x in proposal['current_VK_records'] if x['family']==result['family'])
 for k,v in result.items():
  if k in pg and pg[k]!=v:mismatches.append([result['family'],k,pg[k],v])
 vr.append(result)
options=[]
for p in proposal['options']:
 bymodel={x['model']:x for x in rows if x['model'] in p['basis_models']}
 vals=[x['lme_per_repeat'] for x in bymodel.values()]
 cq=p['case_rounding_quantum_LME'];iq=p['invocation_rounding_quantum_LME']
 result={'id':p['id'],'measured_case_max_LME':max(vals),'arithmetic_portfolio_sum_LME':sum(vals),
         'proposed_case_LME':((max(vals)+cq-1)//cq)*cq,'proposed_invocation_LME':((sum(vals)+iq-1)//iq)*iq,
         'max_conditional_single_H_estimate_B':max(x['conditional_full_H_estimate_B'] for x in rows if x['model'] in p['basis_models'])}
 for k,v in result.items():
  if p[k]!=v:mismatches.append([p['id'],k,p[k],v])
 result['case_headroom_LME']=p['proposed_case_LME']-max(vals)
 result['portfolio_headroom_LME']=p['proposed_invocation_LME']-sum(vals)
 result['case_work_covered_models']=[x for x in sorted({x['model'] for x in rows}) if next(y['lme_per_repeat'] for y in rows if y['model']==x)<=p['proposed_case_LME']]
 options.append(result)
admissions=json.loads(read(K6C/R/'REVIEW_RV34/t4_measurements_07/_run_records/ADMISSIONS_AND_RSS_MISSES.json'))
misses=[]
for a in admissions:
 rec=raw_records[a['run_id']];assert a['observed_RSS']==rec['peak_rss_bytes']
 if rec['peak_rss_bytes']>a['projected_RSS']:
  misses.append({'run_id':a['run_id'],'observed_RSS':rec['peak_rss_bytes'],'projected_RSS':a['projected_RSS'],'ratio':rec['peak_rss_bytes']/a['projected_RSS']})
assert len(misses)==8
prefixes=[{'run_id':x['run_id'],**p,'overshoot_LME':p['meter_charged']-p['case_limit']} for x in rows for p in x['prefixes']]
summary={'normal_processes':len(rows),'normal_repeats':sum(x['repeats'] for x in rows),'models':len(set(x['model'] for x in rows)),
 'H_attempt_charge_checks':attempt_checks,'VR_attempt_charge_checks':vr_attempt_checks,'VR_cases':sum(x['cases'] for x in vr),'VR_Selected':sum(x['selected'] for x in vr),
 'A1_increased_case_count':len(a1_deltas),'A1_min_positive_delta':min(x['delta_LME'] for x in a1_deltas),'A1_max_positive_delta':max(x['delta_LME'] for x in a1_deltas),
 'all_A1_deltas_positive':all(x['delta_LME']>0 for x in a1_deltas),
 'proposal_mismatches':mismatches,'options':options,'rss_miss_count':len(misses),
 'prefix_count':len(prefixes),'T4_max_matched_window_peak':max(x['stage_moving_max_B'] for x in rows if x['tier']=='T4')}
OUT.mkdir(parents=True,exist_ok=True)
for name,val in [('ARITHMETIC.json',{'summary':summary,'groups':groups,'current_VR':vr,'rss_misses':misses,'prefixes':prefixes,'A1_deltas':a1_deltas,'H_processes':rows}),('ARITHMETIC_ORIGINS.json',{'inputs':inputs,'members':member_origins,'historical_read_only_git_inputs':historical_origins})]:
 (OUT/name).write_text(json.dumps(val,indent=2)+'\n')
print(json.dumps(summary,indent=2))

# Supplemental outcome and window arithmetic against accepted caller expressions.
sparse_classes={}
all_stages=[]
for tier in range(2,5):
 p=K6C/R/f'MEASUREMENTS/W1_T{tier}/_run_records/delta_records.tar.gz'
 with tarfile.open(p) as t:
  sparse=collections.Counter()
  for name in t.getnames():
   if name.startswith('records/') and name.endswith('_sparse.jsonl'):
    rr=[json.loads(line) for line in t.extractfile(name)]
    oo=[x for x in rr if x['kind']=='outcome']
    assert len(oo)==5 and len({x['class'] for x in oo})==1
    sparse[oo[0]['class']]+=1
   if tier==4 and name.startswith('records/') and name.endswith('_w1a.jsonl'):
    all_stages.extend((name.removeprefix('records/').removesuffix('.jsonl'),x)
        for x in map(json.loads,t.extractfile(name)) if x['kind']=='stage')
  sparse_classes[f'T{tier}_sparse_processes']=dict(sparse)
callers={x['run_id']:x for x in json.loads(read(K6C/R/'REVIEW_RV34/t4_measurements_07/_run_records/INDEPENDENT_CALLERS.json'))}
matched=[]
for run,x in all_stages:
 c=callers[run]['windows']['all']
 name='source' if x['stage']=='w1_source' else 'solve' if x['stage']=='w1_solve' else 'prefix'
 for field,kind in [('heap_peak','requested'),('heap_peak_move','moving')]:
  bound=c[f'{name}_{kind}'];assert x[field]<=bound
  matched.append({'run_id':run,'stage':x['stage'],'repeat':x['repeat'],'field':field,
                  'observed':x[field],'bound':bound,'slack':bound-x[field]})
minimum=min(matched,key=lambda x:x['slack'])
accepted_slack=json.loads(read(K6C/R/'REVIEW_RV34/t4_measurements_07/_run_records/MINIMUM_SLACK.json'))
assert minimum['slack']==accepted_slack['slack']==469832
aux={**sparse_classes,'all_schedule_bound_stage_fields':len(matched),'minimum_slack':minimum,
     'scope':'Arithmetic against accepted RV34 caller expressions and independently extracted raw observations; no full source re-proof or compiler execution.'}
(OUT/'AUXILIARY_CHECKS.json').write_text(json.dumps(aux,indent=2)+'\n')
# Rebind every full input byte stream to the supplied frozen revisions.
frozen_checks=[]
for inp in inputs:
 p=pathlib.Path(inp['path']);root=NUM if p.is_relative_to(NUM) else K6C
 rel=p.relative_to(root);rev='8fada07f2c975cd20e52856a3d4fb589b05767e5' if root==NUM else '9994462204231fb92073e17eb09775756bec22f3'
 blob=subprocess.check_output(['git','show',f'{rev}:{rel}'],cwd=root,env=env)
 ok=hashlib.sha256(blob).hexdigest()==inp['sha256'];assert ok
 frozen_checks.append({'path':str(rel),'revision':rev,'matches':ok})
(OUT/'ARITHMETIC_ORIGINS.json').write_text(json.dumps({'inputs':inputs,'members':member_origins,
    'historical_read_only_git_inputs':historical_origins,'frozen_input_checks':frozen_checks},indent=2)+'\n')
