from review_io import *
import ast,re,collections,statistics,datetime
checks=collections.Counter();bad=[]
def eq(k,a,b):
 checks[k]+=1
 if a!=b:bad.append({'check':k,'actual':a,'expected':b})
def ok(k,v):eq(k,bool(v),True)
old=get(R+'/REVIEW_RV34/t1_measurements_02/_run_records/check.py','468f14ec733026920729ca601c9d3243ff0f61aa').decode();names={'strict','loads','one','G','F','admit'}
exec(compile(ast.Module(body=[n for n in ast.parse(old).body if isinstance(n,ast.FunctionDef) and n.name in names],type_ignores=[]),'RV34 independent helpers','exec'),globals())
t3code=get(R+'/REVIEW_RV34/t3_measurements_05/_run_records/check.py').decode();replacement=next(ast.literal_eval(n.value) for n in ast.parse(t3code).body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='replacement' for t in n.targets));exec(compile(ast.Module(body=[n for n in ast.parse(replacement).body if isinstance(n,ast.FunctionDef) and n.name in ['digit_bins','label_total','modelbytes']],type_ignores=[]),'RV34 scalar Builder method','exec'),globals())
models_source=get(H+'/src/k6/models.rs',SOURCE).decode();r1_source=re.search(r'pub const R1_SOURCE: &str = "([^"]+)"',models_source)[1]
raw=archive(4);prior=archive(3);seedbytes=get(H+'/observations/k6b/counts.jsonl',SOURCE);seed={r['model']:r for r in map(loads,seedbytes.splitlines())};bind=packet('INPUT_BINDING.json')
seal=get(P+'/_run_records/SHA256SUMS');eq('seal',hashlib.sha256(seal).hexdigest(),'9d331a4f6cc9ea637fe24d42a07cf62c69cd4859d0a01f2367ab46ec44d10f84')
for line in seal.decode().splitlines():
 h,n=line.split(None,1);eq('payload_hash',hashlib.sha256(get(P+'/'+n.strip())).hexdigest(),h)
eq('archive_hash',hashlib.sha256(get(R+'/MEASUREMENTS/W1_T4/_run_records/delta_records.tar.gz')).hexdigest(),'97e0fa638b6e2c997f6c3d2ca7f907935daf030348230d8d0c0dd13f043f5720')
inventory=packet('RAW_INVENTORY.json');rootprefix=str(pathlib.Path(bind['records']).parent)+'/'
eq('archive_exact_set',sorted(raw),sorted([x['path'].removeprefix(rootprefix) for x in inventory['new_files']]+['records/records.jsonl']))
for x in inventory['new_files']:
 b=raw[x['path'].removeprefix(rootprefix)];eq('raw_hash',hashlib.sha256(b).hexdigest(),x['sha256']);eq('raw_size',len(b),x['bytes'])
jbytes=raw['records/records.jsonl'];oldbytes=prior['records/records.jsonl'];eq('prior_exact_prefix',jbytes[:len(oldbytes)],oldbytes);eq('prior_bytes',len(oldbytes),404251);eq('journal_hash',hashlib.sha256(jbytes).hexdigest(),inventory['journal']['sha256']);eq('suffix_hash',hashlib.sha256(jbytes[len(oldbytes):]).hexdigest(),inventory['journal']['new_suffix_sha256'])
journal=list(map(loads,jbytes.splitlines()));runs=journal[108:];eq('journal132',len(journal),132);eq('actual_orders',[r['order'] for r in runs],list(range(247,271)));eq('prior_dictionaries',journal[:108],list(map(loads,oldbytes.splitlines())))
launch=js(R+'/I26/measurement_plan_03/_run_records/LAUNCH_MANIFEST.json');plans={p['run_id']:p for p in launch['launches'] if p['tier']=='W1-T4'};eq('launch_table',packet('T4_LAUNCHES.json'),list(plans.values()));known=js(R+'/REVIEW_RV36/measurement_plan_02/_run_records/SCHEDULE_AND_COUNTS_AUDIT.json');consumed=list(known['complete_seed_consumed_fields']);invariants=list(known['also_compare_invariant_report_fields']);estfields=known['fresh_complete_context_estimate_required_fields']
H19={v['run_id']:(h,v) for h in js(R+'/I21/h_numeric_19/PHASES.json')['rows'] for v in h['launch_variants']};models={m:modelbytes(m) for m in {p['model'] for p in plans.values()}}
def obs(rid):return list(map(loads,raw['records/'+rid+'.jsonl'].splitlines()))
records={n[len('records/'):-len('.record.json')]:loads(b) for n,b in raw.items() if n.startswith('records/') and n.endswith('.record.json')};eq('product_calls',len(records),37)
base=records['baseline_W1-T4'];BR=base['peak_rss_bytes'];BF=base['rss']['time_peak_footprint_bytes'];ok('baseline_positive',BR>0 and BF>0);eq('baseline_zero',one(obs('baseline_W1-T4'),'summary')['repeats_completed'],0);eq('baseline_noop',one(obs('baseline_W1-T4'),'start')['noop'],True)
pids=[]
for rid,r in records.items():
 oo=obs(rid);pids.append(one(oo,'start')['pid']);eq('exit0',r['exit_code'],0);eq('process_ok',r['classification'],'ok');eq('no_timeout',r['timed_out'],False);eq('no_RSS_kill',r['killed_by_rss_watchdog'],False);eq('stderr_empty',raw['records/'+rid+'.stderr.txt'],b'');eq('raw_stdout_hash',hashlib.sha256(raw['records/'+rid+'.jsonl']).hexdigest(),r['stdout_sha256']);eq('process_summary',one(oo,'summary'),r['summary']);eq('RSS_cap',r['rss_cap_bytes'],8589934592);eq('timeout_bound',r['timeout_s'],60 if rid=='baseline_W1-T4' else 1800)
 tt=raw['records/'+rid+'.time.txt'].decode();rss=int(re.search(r'(\d+)\s+maximum resident set size',tt)[1]);ft=int(re.search(r'(\d+)\s+peak memory footprint',tt)[1]);eq('raw_time_RSS',rss,r['peak_rss_bytes']);eq('raw_time_footprint',ft,r['rss']['time_peak_footprint_bytes']);eq('wait4_RSS',rss,r['rss']['wait4_maxrss_bytes'])
eq('distinct_product_PIDs',len(set(pids)),37)
s=get(H+'/runner/k6_runner.py',SOURCE).decode();p1=next(ast.literal_eval(n.value) for n in ast.parse(s).body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='P1_LINUX_PEAK_MIB' for t in n.targets))
normalize=lambda a:[os.path.basename(v) if i==0 or os.path.isabs(v) else v for i,v in enumerate(a)]
matched=[];excluded=[];prefixes=[];outcomes=[];admissions=[];high=[];first_attempts=[];pairs=collections.defaultdict(list);caller=[];paritycounts=collections.Counter();numerical_stops=[]
for i,r in enumerate(runs,start=108):
 rid=r['run_id'];p=plans[rid];oo=obs(rid);st=one(oo,'start');c=one(oo,'counts');su=one(oo,'summary');ss=seed[r['model']]
 for k in ['order','model','mode','family','members','pass','tier','heap_cap_bytes','rss_cap_bytes','timeout_s','time_budget_s','first_repeat_limit_s']:eq('schedule_'+k,r[k],p[k])
 eq('named_hold_lift_only',r['conditional'],False);eq('normal_argv',r['argv'],normalize(p['argv']));eq('normal_counts_file',c['counts_source'],'file');eq('baseline_RSS',r['baseline_rss_bytes'],BR);eq('baseline_footprint',r['baseline_footprint_bytes'],BF);eq('completed5',su['repeats_completed'],5);eq('no_stop_reason',su['stop_reason'],None)
 for k in consumed:eq('normal_consumed',c[k],ss[k])
 for k in invariants:eq('normal_invariant',c[k],ss[k])
 expected={'schema':'k6-observe-v1','model':r['model'],'mode':r['mode'],'model_from_file':False,'counts_only':False,'repeats':5,'entry_repeats':5,'heap_cap_bytes':8053063680,'time_budget_s':1740,'first_repeat_limit_s':600,'allow_over_estimate':False}
 if r['mode']=='w1a':expected.update(case_limit=2**64-1,invocation_limit=2**64-1,w1_prefixes=p['prefixes'],dump_published=p['prefixes'])
 for k,v in expected.items():eq('normal_start',st[k],v)
 for o in oo:
  if o['kind']=='stage':
   ot=next((x for x in oo if x['kind']=='outcome' and x['repeat']==o['repeat']),None)
   numerical_stop=(r['mode']=='sparse' and ot is not None and ot['class']=='NumericallyUnresolved' and o['stage'] in [ot['failed_stage'],'entry_checked','entry_plain'])
   eq('stage_status_matches_numerical_outcome',o['ok'],not numerical_stop)
   if numerical_stop:eq('stage_numerical_error_identity',o['error'],ot['error']);numerical_stops.append({'run_id':rid,'repeat':o['repeat'],'stage':o['stage'],'ok':False,'error':o['error'],'outcome':ot['class']})
  if o['kind']=='parity':eq('parity',o['equal'],True);paritycounts[o['item']]+=1
 for name,agg in r['stages'].items():
  rr=[o for o in oo if o['kind']=='stage' and o['stage']==name];eq('stage_samples',len(rr),agg['samples']);eq('stage_median_ns',int(statistics.median(x['elapsed_ns'] for x in rr)),agg['median_ns'])
  for k in ['heap_peak','heap_peak_move']:eq('stage_max',max(x[k] for x in rr),agg[k])
 outs=[o for o in oo if o['kind']=='outcome'];eq('five_outcome_records',[o['repeat'] for o in outs],list(range(5)));eq('repeat_outcome_identity',[{k:v for k,v in o.items() if k!='repeat'} for o in outs],[{k:v for k,v in outs[0].items() if k!='repeat'}]*5);pairs[(r['model'],r['mode'])].append({k:v for k,v in outs[0].items() if k!='repeat'})
 tree='TREE' in r['model'];outcomes.append({'run_id':rid,'model':r['model'],'mode':r['mode'],'pass':r['pass'],'outcome':outs[0],'repeats':5})
 if r['mode']=='w1a':
  bn=rid+'.counts-binding';bo=obs(bn);bc=one(bo,'counts');eq('binding_argv',records[bn]['argv'],normalize(p['counts_binding_argv']));eq('binding_kinds',[x['kind'] for x in bo],['start','counts','summary']);eq('binding_zero',one(bo,'summary')['repeats_completed'],0);eq('binding_computed',bc['counts_source'],'computed')
  for k,v in expected.items():eq('binding_start',one(bo,'start')[k],True if k=='counts_only' else v)
  for k in consumed:eq('binding_consumed',bc[k],ss[k])
  for k in invariants:eq('binding_invariant',bc[k],ss[k])
  for k in estfields:eq('fresh_context_identity',bc[k],c[k]);ok('estimate_type',type(c[k])is int and c[k]>=0)
  h,v=H19[rid];args=sum(len(p['argv'][j+1].encode()) for j,f in enumerate(p['argv']) if f in ['--model','--model-file','--counts-file','--dump-solution','--dump-pattern','--dump-published']);fixed=models[r['model']]+1700+args+h['owners']['saved_attempts']+h['owners']['BASE']+(387 if p['prefixes'] else 0);eq('caller_fixed',fixed,c['estimate_w1_fixed_bytes']);delta=fixed-v['Hfixed'];win={sch:{k:value+delta if value else 0 for k,value in t.items()} for sch,t in v['totals'].items()};eq('caller_full',win['all']['moving_bytes'],c['estimate_adm_bytes_w1a']);eq('caller_selected128',win['selected128']['moving_bytes'],c['estimate_w1_sel128_bytes'])
  for k in estfields:
   if k not in ['estimate_adm_bytes_w1a','estimate_w1_sel128_bytes','estimate_w1_fixed_bytes']:eq('kernel_diagnostics',c[k],ss[k])
  caller.append({'run_id':rid,'model_bytes':models[r['model']],'args':args,'fixed':fixed,'delta':delta,'windows':win})
  attempts=[o for o in oo if o['kind']=='attempt'];first_attempts.extend({'run_id':rid,**o} for o in attempts if o['repeat']==0)
  for ot in outs:
   eq('W1_class',ot['class'],'Unresolved' if tree else 'Selected');eq('W1_reason',ot['reason'],'Ceiling' if tree else None);eq('selected_precision',ot['selected_precision'],None if tree else 128);eq('verification_precision',ot['verification_precision'],None if tree else 256)
   if not tree:eq('published_rows',ot['rows'],ss['w1_rows']);eq('row_class_sum',sum(ot[k] for k in ['rows_relative_verified','rows_absolute_verified','rows_input_derived','rows_unpublishable']),ot['rows'])
   else:eq('unpublished_fields_absent','rows' in ot,False)
   aa=[a for a in attempts if a['repeat']==ot['repeat']];eq('attempt_count',len(aa),ot['attempts']);eq('attempt_precisions',[a['precision'] for a in aa],[128,256,512,1024] if tree else [128,256]);eq('work_meter',sum(a['charged_by'] for a in aa),ot['meter_charged'])
   for a in aa:
    eq('own_stage_sum',sum(v for k,v in a.items() if k.startswith('own_') and k not in ['own_total','own_unstaged']),a['own_total']);eq('shared_stage_sum',sum(v for k,v in a.items() if k.startswith('shared_') and k not in ['shared_work','shared_built_here','shared_unstaged']),a['shared_work']+a['verification_shared_work']);eq('work_charge',a['charged_by'],a['own_total']+a['shared_work']+a['verification_shared_work']);eq('work_stages_complete',a['stages_complete'],True);eq('own_unstaged',a['own_unstaged'],0);eq('shared_unstaged',a['shared_unstaged'],0)
    for k,sk in [('storage_pattern_entries','w1_pattern_entries'),('storage_profile_entries','w1_profile_entries'),('storage_limbs_per_entry','w1_limbs_per_entry_'+str(a['precision']))]:eq('attempt_storage',a[k],ss[sk])
    pr=str(a['precision']);eq('precision_own',a['own_total'],ot['work_'+pr+'_own']);eq('precision_shared',a['shared_work']+a['verification_shared_work'],ot['work_'+pr+'_shared'])
    if tree and a['precision']>=512:high.append({'run_id':rid,'repeat':a['repeat'],'precision':a['precision'],'role':a['role'],'outcome':a['outcome'],'own_work':a['own_total'],'shared_work':a['shared_work']+a['verification_shared_work'],'pattern':a['storage_pattern_entries'],'profile':a['storage_profile_entries'],'limbs':a['storage_limbs_per_entry'],'per_precision_heap_available':False})
  aa=[a for a in attempts if a['repeat']==4];segments=[];total=0
  for j,a in enumerate(aa):
   total+=a['shared_work']+a['own_total']-a['verification_work']-a['stop_rule_work'];segments.append(('solve_'+str(a['precision']),total))
   if a['verification_work'] or a['verification_shared_work']:
    total+=a['verification_work']+a['verification_shared_work'];segments.append(('verify_'+str(a['precision']),total))
    if j and aa[j-1]['stop_rule_work']:total+=aa[j-1]['stop_rule_work'];segments.append(('decide_'+str(aa[j-1]['precision']),total))
  pp=[o for o in oo if o['kind']=='prefix'];required=segments[:-1] if p['prefixes'] else [];eq('prefix_exact_endpoints',[(o['through_segment'],o['case_limit']) for o in pp],required);eq('last_segment_meter',total,outs[-1]['meter_charged'])
  for o in pp:eq('prefix_unresolved',o['class'],'Unresolved');eq('prefix_budget_case',o['reason'],'Budget(Case)')
  if p['prefixes']:prefixes.append({'run_id':rid,'full_segments':segments,'actual':pp,'complete':True})
  eq('dump_presence',('records/'+rid+'.rows') in raw,p['prefixes'] and not tree)
  for o in oo:
   for f,n in o.items():
    if 'heap' not in f or type(n)is not int:continue
    if o['kind']=='stage':phase='source' if o['stage']=='w1_source' else 'solve' if o['stage']=='w1_solve' else 'prefix';bound=win['all'][phase+('_moving' if 'move' in f else '_requested')]
    elif o['kind']=='summary' and f.startswith('repeats_'):met='moving' if 'move' in f else 'requested';bound=max(win['all']['source_'+met],win['all']['solve_'+met])
    elif o['kind']=='summary' and f.startswith('prefix_'):bound=win['all']['prefix_'+('moving' if 'move' in f else 'requested')]
    else:excluded.append({'run_id':rid,'kind':o['kind'],'field':f,'value':n});continue
    ok('matched_heap_bound',n<=bound);matched.append({'run_id':rid,'kind':o['kind'],'stage':o.get('stage'),'repeat':o.get('repeat'),'field':f,'observed':n,'bound':bound,'slack':bound-n})
 else:
  for ot in outs:eq('sparse_numerical_class',ot['class'],'Sensitive' if 'CONT' in r['model'] else 'NumericallyUnresolved')
  eq('no_unrequested_sparse_dump','records/'+rid+'.u' in raw,False);eq('sparse_seed_estimate',r['estimate_adm_bytes'],ss['estimate_adm_bytes_sparse'])
 estimate=c['estimate_adm_bytes_'+r['mode']];eq('own_estimate_denominator',estimate,r['estimate_adm_bytes']);decision,pop=admit(r,estimate,journal[:i]);eq('independent_admission',decision,r['admission']);eq('population_eight',len(pop),8);eq('current_size_never_calibrates',all(next(x for x in journal if x['run_id']==pid)['tier'] in ['W1-T2','W1-T3'] for pid in pop),True)
 prev=r['model'].replace('-n10000-','-n01000-');ascent=[x['run_id'] for x in journal[:i] if x['model']==prev and x['mode']==r['mode'] and x['classification'] not in [None,'not_run']];ok('recorded_ascent',bool(ascent));ok('RSS_under_cap',r['peak_rss_bytes']<=r['rss_cap_bytes']);admissions.append({'run_id':rid,'mode':r['mode'],'decision':decision,'calibration':pop,'ascent':ascent,'observed_RSS':r['peak_rss_bytes'],'projected_RSS':decision['projected_rss_bytes'],'projection_missed':r['peak_rss_bytes']>decision['projected_rss_bytes'],'miss_bytes':r['peak_rss_bytes']-decision['projected_rss_bytes']})
save('NUMERICAL_STAGE_STOPS.json',numerical_stops)
for k,vals in pairs.items():eq('two_pass_outcome_identity',vals[0],vals[1])
aw=packet('METRIC_WINDOW_CHECKS.json')['windows'];eq('matched_field_count',len(matched),len(aw))
for x,y in zip(matched,aw):
 for k in ['run_id','field','observed','bound','slack']:eq('author_window_value',x[k],y[k])
for ours,other in zip(admissions,packet('ADMISSION_AND_RSS.json')):eq('author_RSS_observation',ours['observed_RSS'],other['actual_RSS']);eq('author_RSS_projection',ours['projected_RSS'],other['projected_RSS']);eq('author_miss_flag',ours['projection_missed'],other['actual_over_projected']);eq('author_calibration',ours['calibration'],other['calibration_ids'])
save('MEASUREMENT_CHECKS.json',{'check_counts':dict(checks),'failures':bad,'new_products':len(records),'normal_rows':len(runs),'completed_repeats':sum(r['summary']['repeats_completed'] for r in runs),'W1_classes':dict(collections.Counter(o['outcome']['class'] for o in outcomes if o['mode']=='w1a')),'sparse_classes':dict(collections.Counter(o['outcome']['class'] for o in outcomes if o['mode']=='sparse')),'actual_prefix_calls':sum(len(x['actual']) for x in prefixes),'matched_fields':len(matched),'minimum_slack':min(x['slack'] for x in matched),'largest_matched_observation':max(x['observed'] for x in matched),'unclaimed_or_configuration_fields':len(excluded),'TREE_high_precision_attempt_records':len(high),'projection_misses':dict(collections.Counter(x['mode'] for x in admissions if x['projection_missed'])),'checked_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()});save('ADMISSIONS_AND_RSS_MISSES.json',admissions);save('INDEPENDENT_CALLERS.json',caller);save('PREFIX_COMPLETENESS.json',prefixes);save('HIGH_PRECISION_COVERAGE.json',high);save('OUTCOMES.json',outcomes);save('MINIMUM_SLACK.json',min(matched,key=lambda x:x['slack']));print(json.dumps({'failures':bad,'checks':sum(checks.values()),'matched':len(matched),'min_slack':min(x['slack'] for x in matched),'high_precision_records':len(high)},indent=2))
