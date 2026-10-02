"""Independent T2 byte/metadata/arithmetic audit reusing RV34's sealed T1 method.
No author checker, runner module, product/model/count/graph algorithm or solver.
"""
from review_io import *
import ast,collections,re,statistics,datetime
checks=collections.Counter();bad=[]
def eq(k,a,b):
 checks[k]+=1
 if a!=b:bad.append({'check':k,'actual':a,'expected':b})
def ok(k,v):eq(k,bool(v),True)
# Deliberate reuse of seven already reviewed, side-effect-free RV34 helper
# definitions. The previous packet is referenced/read in memory, not copied.
priorpath=R+'/REVIEW_RV34/t1_measurements_02/_run_records/check.py'
old=get(priorpath,'468f14ec733026920729ca601c9d3243ff0f61aa').decode()
names={'strict','loads','one','G','F','modelbytes','admit'}
nodes=[n for n in ast.parse(old).body if isinstance(n,ast.FunctionDef) and n.name in names]
assert len(nodes)==len(names)
exec(compile(ast.Module(body=nodes,type_ignores=[]),'RV34 frozen T1 pure helpers','exec'),globals())
t1=archive(1);t2=archive(2);raw={'records/'+n:b for n,b in t1.items()};raw.update(t2)
def obs(name):return [loads(x) for x in raw[name+'.jsonl'].splitlines() if x.strip()]
def seal_check(file,expected):
 b=get(P+'/_run_records/'+file);eq('seal_identity',hashlib.sha256(b).hexdigest(),expected)
 for line in b.decode().splitlines():
  h,n=line.split(None,1);eq('sealed_payload',hashlib.sha256(get(P+'/'+n.strip())).hexdigest(),h)
seal_check('SHA256SUMS','1f1e7f70e6970687d6a13e3c4ccfd61e77b47935591d8e1266a720f71acf9883')
seal_check('OUTCOME_CORRECTION_SHA256SUMS','3d0b208fde5ea6ca4501bbdec6bbf6d719d8e2bbfb127aeee7001ca770406a7a')
inv=packet('RAW_INVENTORY.json');eq('T2_archive_exact_set',sorted(t2),sorted([x['relative_to_measurement_root'] for x in inv['new_files']]+['records/records.jsonl']))
for x in inv['new_files']:
 bb=t2[x['relative_to_measurement_root']];eq('T2_raw_hash',hashlib.sha256(bb).hexdigest(),x['sha256']);eq('T2_raw_bytes',len(bb),x['bytes'])
for x in js(R+'/I26/t1_measurements_06/_run_records/RAW_RECORDS_MANIFEST.json')['files']:eq('T1_raw_hash',hashlib.sha256(t1[x['relative']]).hexdigest(),x['sha256'])
jbytes=t2['records/records.jsonl'];oldbytes=t1['records.jsonl'];jr=inv['journal']
eq('T1_prefix_bytes',jbytes[:len(oldbytes)],oldbytes);eq('prefix_length',len(oldbytes),223099);eq('prefix_hash',hashlib.sha256(oldbytes).hexdigest(),jr['original_T1_prefix_sha256']);eq('journal_hash',hashlib.sha256(jbytes).hexdigest(),jr['sha256']);eq('appended_hash',hashlib.sha256(jbytes[len(oldbytes):]).hexdigest(),jr['appended_sha256']);eq('appended_bytes',len(jbytes)-len(oldbytes),jr['appended_bytes'])
for tier in [1,2]:
 cap=js(R+'/MEASUREMENTS/W1_T'+str(tier)+'/_run_records/CAPTURE.json');name='records.tar.gz' if tier==1 else 'delta_records.tar.gz';eq('container_hash',hashlib.sha256(get(R+'/MEASUREMENTS/W1_T'+str(tier)+'/_run_records/'+name)).hexdigest(),cap['archive_sha256'])
bind=packet('INPUT_BINDING.json');manifest=js(R+'/I26/measurement_plan_03/_run_records/LAUNCH_MANIFEST.json');planned=[x for x in manifest['launches'] if x['tier']=='W1-T2'];plans={p['run_id']:p for p in planned}
eq('author_plan',packet('T2_LAUNCHES.json'),planned);eq('actual_CLI',packet('ACTUAL_INVOCATION.json')['argv'],next(x['argv'] for x in manifest['tier_cli'] if x['tier']=='W1-T2'));eq('actual_cwd',packet('ACTUAL_INVOCATION.json')['cwd'],manifest['cwd'])
seedbytes=get(H+'/observations/k6b/counts.jsonl',SOURCE);eq('seed_hash',hashlib.sha256(seedbytes).hexdigest(),bind['seed']['sha256']);seed={r['model']:r for r in map(loads,seedbytes.splitlines())}
known=js(R+'/REVIEW_RV36/measurement_plan_02/_run_records/SCHEDULE_AND_COUNTS_AUDIT.json')
for short,k in [('consumed','complete_seed_consumed_fields'),('invariants','also_compare_invariant_report_fields'),('estimates','fresh_complete_context_estimate_required_fields')]:eq('complete_field_map',packet('FIELD_MAP.json')[short],list(known[k]))
consumed=list(known['complete_seed_consumed_fields']);invariant=list(known['also_compare_invariant_report_fields']);estfields=known['fresh_complete_context_estimate_required_fields']
H19={v['run_id']:(h,v) for h in js(R+'/I21/h_numeric_19/PHASES.json')['rows'] for v in h['launch_variants']};modelmeta={x['model']:x for x in js(R+'/I26/measurement_plan_03/_run_records/MODEL_INPUTS.json')['models']}
models={mid:modelbytes(mid) for mid in sorted({p['model'] for p in planned})}
normalize=lambda argv:[os.path.basename(a) if i==0 or os.path.isabs(a) else a for i,a in enumerate(argv)]
meta=loads(raw['records/metadata.json']);eq('binary_metadata',meta['binary_sha256'],bind['binary']['sha256']);eq('source_metadata',meta['source_commit'],SOURCE);eq('tree_metadata',meta['source_tree'],bind['source_tree'])
base=loads(t2['records/baseline_W1-T2.record.json']);bo=obs('records/baseline_W1-T2');BR=base['peak_rss_bytes'];BF=base['rss']['time_peak_footprint_bytes']
eq('baseline_argv',base['argv'],['k6_observe','--noop','--heap-cap-bytes','8053063680'])
eq('baseline_kinds',[o['kind'] for o in bo],['start','summary']);eq('baseline_noop',bo[0]['noop'],True);eq('baseline_zero_repeats',bo[1]['repeats_completed'],0);eq('baseline_cap',bo[0]['heap_cap_bytes'],8053063680);ok('baseline_positive',BR>0 and BF>0);ok('baseline_fresh_pid',bo[0]['pid']!=obs('records/baseline_W1-T1')[0]['pid'])
journal=list(map(loads,jbytes.splitlines()));oldjournal=list(map(loads,oldbytes.splitlines()));runs=journal[60:];eq('T1_dictionaries_preserved',journal[:60],oldjournal);eq('total_journal_rows',len(journal),84);eq('T2_orders',[r['order'] for r in runs],list(range(199,223)));eq('T2_ids',[r['run_id'] for r in runs],[p['run_id'] for p in planned])
processes={n[:-12]:loads(bb) for n,bb in t2.items() if n.endswith('.record.json')};eq('new_processes',len(processes),43);pids=[]
for name,r in processes.items():
 eq('process_RSS_cap',r['rss_cap_bytes'],8589934592);eq('process_timeout_bound',r['timeout_s'],60 if name=='records/baseline_W1-T2' else 600);eq('process_time_wrapper',r['wrapper'],'/usr/bin/time -l')
 oo=obs(name);pids.append(one(oo,'start')['pid']);eq('exit',r['exit_code'],0);eq('classification',r['classification'],'ok');eq('survivor_array',r['survivors'],[]);eq('timed_out',r['timed_out'],False);eq('watchdog_kill',r['killed_by_rss_watchdog'],False);eq('stderr',raw[name+'.stderr.txt'],b'');eq('stdout_hash',hashlib.sha256(raw[name+'.jsonl']).hexdigest(),r['stdout_sha256']);eq('summary_record',one(oo,'summary'),r['summary'])
 time=raw[name+'.time.txt'].decode();rss=int(re.search(r'(\d+)\s+maximum resident set size',time)[1]);ft=int(re.search(r'(\d+)\s+peak memory footprint',time)[1]);eq('time_rss',rss,r['peak_rss_bytes']);eq('time_footprint',ft,r['rss']['time_peak_footprint_bytes']);eq('time_wait4',rss,r['rss']['wait4_maxrss_bytes'])
eq('distinct_new_pids',len(set(pids)),43);ok('positive_pids',all(p>0 for p in pids))
quals=packet('QUALIFICATIONS.json');eq('qualification_orders',[q['order'] for q in quals['calls']],[199,204,207,212,215,220]);eq('qualification_scope',quals['normal_solve_or_admission'],False)
qualified={};qchron=[]
for q in quals['calls']:
 p=next(p for p in planned if p['order']==q['order']);name='qualification/'+q['run_id'];oo=obs(name);c=one(oo,'counts');st=one(oo,'start');ss=seed[q['model']]
 eq('qualification_argv',q['argv'],p['counts_binding_argv']);eq('qualification_record',q['record'],processes[name]);eq('qualification_kinds',[x['kind'] for x in oo],['start','counts','summary']);eq('qualification_zero_repeats',one(oo,'summary')['repeats_completed'],0);eq('qualification_source',c['counts_source'],'computed')
 for k in consumed:eq('qualification_consumed',c[k],ss[k])
 for k in invariant:eq('qualification_invariant',c[k],ss[k])
 for k in estfields:ok('qualification_integer_estimate',type(c[k])is int and c[k]>=0)
 ok('qualification_positive_max',c['estimate_adm_bytes_w1a']>0);eq('qualification_normalized_argv',processes[name]['argv'],normalize(q['argv']));eq('qualification_cwd',q['cwd'],manifest['cwd'])
 qualified[p['run_id']]=c;qchron.append({'order':q['order'],'start_utc':q['start_utc'],'end_utc':q['end_utc']})
ok('qualification_before_tier',quals['end_utc']<packet('ACTUAL_INVOCATION.json')['dispatch_intent_utc']);ok('qualification_sequential',all(a['end_utc']<=b['start_utc'] for a,b in zip(qchron,qchron[1:])))
source=get(H+'/runner/k6_runner.py',SOURCE).decode();p1=next(ast.literal_eval(n.value) for n in ast.parse(source).body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='P1_LINUX_PEAK_MIB' for t in n.targets))
ar={r['run_id']:r for r in packet('FULL_CHECKS.json')['runs']};outcomes=[];windows=[];metrics=[];admissions=[];prefixes_total=0;normal_repeats=0;parities=collections.Counter()
for i,r in enumerate(runs,start=60):
 rid=r['run_id'];name='records/'+rid;p=plans[rid];oo=obs(name);c=one(oo,'counts');st=one(oo,'start');su=one(oo,'summary');ss=seed[r['model']]
 for k in ['order','model','mode','family','members','pass','tier','heap_cap_bytes','rss_cap_bytes','timeout_s','time_budget_s','first_repeat_limit_s']:eq('schedule_'+k,r[k],p[k])
 eq('normal_argv',r['argv'],normalize(p['argv']));eq('not_conditional',r['conditional'],False);eq('normal_count_source',c['counts_source'],'file');eq('row_source',r['source_commit'],SOURCE);eq('row_tree',r['source_tree'],bind['source_tree']);eq('current_baseline_rss',r['baseline_rss_bytes'],BR);eq('current_baseline_footprint',r['baseline_footprint_bytes'],BF)
 for k in consumed:eq('normal_consumed',c[k],ss[k])
 for k in invariant:eq('normal_invariant',c[k],ss[k])
 expected={'model':r['model'],'model_from_file':False,'mode':r['mode'],'counts_only':False,'repeats':5,'entry_repeats':5,'heap_cap_bytes':8053063680,'time_budget_s':540,'first_repeat_limit_s':600,'allow_over_estimate':False,'schema':'k6-observe-v1'}
 if r['mode']=='w1a':expected.update(case_limit=2**64-1,invocation_limit=2**64-1,w1_prefixes=p['prefixes'],dump_published=p['prefixes'])
 for k,v in expected.items():eq('normal_start',st[k],v)
 eq('completed_repeats',su['repeats_completed'],5);eq('normal_stop',su['stop_reason'],None);normal_repeats+=su['repeats_completed']
 out=[x for x in oo if x['kind']=='outcome'];eq('outcome_repeats',[x['repeat'] for x in out],list(range(5)));eq('outcome_repeat_identity',[{k:v for k,v in x.items() if k!='repeat'} for x in out],[{k:v for k,v in out[0].items() if k!='repeat'}]*5)
 for o in oo:
  if o['kind']=='parity':eq('parity',o['equal'],True);parities[o['item']]+=1
  if o['kind']=='stage':eq('stage_ok',o['ok'],True)
  if o['kind']=='attempt':
   eq('own_stage_sum',sum(v for k,v in o.items() if k.startswith('own_') and k not in ['own_total','own_unstaged']),o['own_total']);eq('shared_stage_sum',sum(v for k,v in o.items() if k.startswith('shared_') and k not in ['shared_work','shared_built_here','shared_unstaged']),o['shared_work']+o['verification_shared_work']);eq('work_closes',o['own_total']+o['shared_work']+o['verification_shared_work'],o['charged_by']);eq('work_complete',o['stages_complete'],True);eq('own_unstaged',o['own_unstaged'],0);eq('shared_unstaged',o['shared_unstaged'],0)
   for key,skey in [('storage_pattern_entries','w1_pattern_entries'),('storage_profile_entries','w1_profile_entries'),('storage_limbs_per_entry','w1_limbs_per_entry_'+str(o['precision']))]:eq('attempt_storage',o[key],ss[skey])
 for stage,agg in r['stages'].items():
  sr=[o for o in oo if o['kind']=='stage' and o['stage']==stage];eq('stage_samples',len(sr),agg['samples'])
  for k in ['heap_peak','heap_peak_move']:eq('stage_maximum',max(x[k] for x in sr),agg[k])
  eq('stage_min_time',min(x['elapsed_ns'] for x in sr),agg['min_ns']);eq('stage_median_time',int(statistics.median(x['elapsed_ns'] for x in sr)),agg['median_ns'])
 if r['mode']=='w1a':
  bn=name+'.counts-binding';bo=obs(bn);bc=one(bo,'counts');bs=one(bo,'start');eq('binding_kinds',[o['kind'] for o in bo],['start','counts','summary']);eq('binding_source',bc['counts_source'],'computed');eq('binding_zero_repeats',one(bo,'summary')['repeats_completed'],0);eq('binding_argv',processes[bn]['argv'],normalize(p['counts_binding_argv']));eq('prepass_extension',p['counts_binding_argv'],p['argv']+['--counts-only'])
  for k,v in expected.items():eq('binding_start',bs[k],True if k=='counts_only' else v)
  for k in consumed:eq('binding_consumed',bc[k],ss[k])
  for k in invariant:eq('binding_invariant',bc[k],ss[k])
  for k in estfields:eq('all22_estimate_identities',bc[k],c[k]);ok('estimates_integer_nonnegative',type(c[k])is int and c[k]>=0)
  if rid in qualified:
   for k in expected:eq('qualification_start',one(obs('qualification/'+str(r['order'])+'_counts_binding_qualification'),'start')[k],True if k=='counts_only' else expected[k])
   for k in estfields:eq('qualification_to_actual_estimate',qualified[rid][k],c[k])
  h,v=H19[rid];args=sum(len(p['argv'][j+1].encode()) for j,f in enumerate(p['argv']) if f in ['--model','--model-file','--counts-file','--dump-solution','--dump-pattern','--dump-published']);eq('actual_argument_bytes',args,p['H_persistent_arg_string_bytes']);fixed=models[r['model']]+1700+args+h['owners']['saved_attempts']+h['owners']['BASE']+(387 if p['prefixes'] else 0);eq('independent_fixed',fixed,c['estimate_w1_fixed_bytes']);delta=fixed-v['Hfixed'];ww={sch:{k:n+delta if n else 0 for k,n in t.items()} for sch,t in v['totals'].items()};eq('independent_full',ww['all']['moving_bytes'],c['estimate_adm_bytes_w1a']);eq('independent_selected128',ww['selected128']['moving_bytes'],c['estimate_w1_sel128_bytes']);eq('author_window_values',ww,ar[rid]['H_windows'])
  for k in estfields:
   if k not in ['estimate_adm_bytes_w1a','estimate_w1_sel128_bytes','estimate_w1_fixed_bytes']:eq('kernel_diagnostic_seed',c[k],ss[k])
  windows.append({'run_id':rid,'model_bytes':models[r['model']],'argument_bytes':args,'fixed':fixed,'delta':delta,'moving_full':ww['all']['moving_bytes'],'moving_selected128':ww['selected128']['moving_bytes']})
  for ot in out:
   eq('W1_class',ot['class'],'Selected');eq('W1_selected',ot['selected_precision'],128);eq('W1_verified',ot['verification_precision'],256);eq('W1_publication_rows',ot['rows'],ss['w1_rows']);eq('W1_row_class_sum',sum(ot[k] for k in ['rows_relative_verified','rows_absolute_verified','rows_input_derived','rows_unpublishable']),ot['rows']);eq('W1_no_budget',ot['budget_reached'],False)
   aa=[o for o in oo if o['kind']=='attempt' and o['repeat']==ot['repeat']];eq('attempt_number',len(aa),ot['attempts']);eq('attempt_precision',[a['precision'] for a in aa],[128,256]);eq('meter_work',sum(a['charged_by'] for a in aa),ot['meter_charged'])
   for prec in [128,256]:
    ap=[a for a in aa if a['precision']==prec];eq('outcome_precision_attempts',len(ap),ot['work_'+str(prec)+'_attempts']);eq('outcome_precision_own',sum(a['own_total'] for a in ap),ot['work_'+str(prec)+'_own']);eq('outcome_precision_shared',sum(a['shared_work']+a['verification_shared_work'] for a in ap),ot['work_'+str(prec)+'_shared'])
  pref=[o for o in oo if o['kind']=='prefix'];eq('prefix_count',len(pref),3 if p['prefixes'] else 0);prefixes_total+=len(pref);aa=[o for o in oo if o['kind']=='attempt' and o['repeat']==4];segments=[];charge=0
  for j,a in enumerate(aa):
   charge+=a['shared_work']+a['own_total']-a['verification_work']-a['stop_rule_work'];segments.append(('solve_'+str(a['precision']),charge))
   if a['verification_work'] or a['verification_shared_work']:
    charge+=a['verification_shared_work']+a['verification_work'];segments.append(('verify_'+str(a['precision']),charge))
    if j and aa[j-1]['stop_rule_work']:charge+=aa[j-1]['stop_rule_work'];segments.append(('decide_'+str(aa[j-1]['precision']),charge))
  if pref:eq('prefix_limit_segments',[(x['through_segment'],x['case_limit']) for x in pref],segments[:-1])
  for pre in pref:eq('prefix_class',pre['class'],'Unresolved');eq('prefix_budget',pre['reason'],'Budget(Case)')
  eq('publication_file_scope',name+'.rows' in raw,p['prefixes'])
  if p['prefixes']:
   lines=raw[name+'.rows'].decode().splitlines();eq('publication_header',lines[:2],['k6b-rows v1','model '+r['model']]);data=[x.split() for x in lines[2:]];eq('publication_rowcount',len(data),out[0]['rows']);eq('publication_unique_ids',len({x[0] for x in data}),len(data))
   for cl in ['relative_verified','absolute_verified','input_derived','unpublishable']:eq('publication_class_count',sum(x[3]==cl for x in data),out[0]['rows_'+cl])
  for o in oo:
   for field,n in o.items():
    if 'heap' not in field or type(n)is not int:continue
    met='moving' if 'move' in field else 'requested';bound=None;cl='outside_window'
    if field=='heap_cap_bytes':cl='configuration'
    elif o['kind']=='stage':phase='source' if o['stage']=='w1_source' else 'solve' if o['stage']=='w1_solve' else 'prefix';bound=ww['all'][phase+'_'+met];cl='matched'
    elif o['kind']=='summary' and field.startswith('repeats_'):bound=max(ww['all']['source_'+met],ww['all']['solve_'+met]);cl='matched'
    elif o['kind']=='summary' and field.startswith('prefix_'):bound=ww['all']['prefix_'+met];cl='matched'
    if bound is not None:ok('matched_H_bound',n<=bound)
    metrics.append({'run_id':rid,'kind':o['kind'],'stage':o.get('stage'),'repeat':o.get('repeat'),'field':field,'observed':n,'metric':met,'bound':bound,'slack':None if bound is None else bound-n,'class':cl})
 else:
  eq('sparse_seed_admission',r['estimate_adm_bytes'],ss['estimate_adm_bytes_sparse']);words=raw[name+'.u'].decode().splitlines();eq('sparse_dump_dofs',len(words),ss['dofs']);ok('sparse_bit_words',all(re.fullmatch('[0-9a-f]{16}',x) for x in words))
  for ot in out:ok('sparse_actual_class',ot['class'] in ['Sensitive','Passed'])
 prev=r['model'].replace('-n00100-','-n00010-');ok('existing_ascent',any(q['model']==prev and q['mode']==r['mode'] and q['classification'] not in [None,'not_run'] for q in journal[:i]))
 estimate=c['estimate_adm_bytes_'+r['mode']];eq('normal_journal_denominator',estimate,r['estimate_adm_bytes']);decision,pop=admit(r,estimate,journal[:i]);eq('chronological_admission',decision,r['admission']);admissions.append({'run_id':rid,'population':pop,'estimate':estimate,'decision':decision})
 outcomes.append({'run_id':rid,'model':r['model'],'mode':r['mode'],'class':out[0]['class'],'precision':out[0].get('selected_precision'),'all_repeat_classes':[x['class'] for x in out],'process_classification':r['classification'],'completed_repeats':len(out)})
author=packet('METRIC_WINDOW_CHECKS.json');eq('metric_count',len(metrics),len(author))
for a,b in zip(metrics,author):eq('metric_values',{k:v for k,v in a.items() if k!='class'},{k:v for k,v in b.items() if k!='disposition'})
classes=collections.Counter(x['class'] for x in outcomes if x['mode']=='sparse');eq('correction_raw_sparse_tally',dict(classes),{'Sensitive':8,'Passed':4});eq('structured_sparse_summary',dict(classes),packet('RESULTS_SUMMARY.json')['sparse_outcomes'])
summary={'normal_rows':len(runs),'new_product_calls':len(processes),'qualifications':len(quals['calls']),'completed_repeats':normal_repeats,'W1_classes':dict(collections.Counter(x['class'] for x in outcomes if x['mode']=='w1a')),'sparse_classes':dict(classes),'prefix_calls':prefixes_total,'metric_counts':dict(collections.Counter(m['class'] for m in metrics)),'min_slack':min(m['slack'] for m in metrics if m['slack'] is not None),'max_W1_requested':max(r['repeats_heap_peak'] for r in runs if r['mode']=='w1a'),'max_W1_moving':max(r['repeats_heap_peak_move'] for r in runs if r['mode']=='w1a'),'max_product_RSS':max(r['peak_rss_bytes'] for r in processes.values()),'baseline_RSS':BR,'baseline_footprint':BF,'all_T2_members100':all(r['members']==100 for r in runs),'parity_items':dict(parities),'failures':bad,'check_counts':dict(checks),'checked_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}
save('CHECKS.json',summary);save('OUTCOMES.json',outcomes);save('CONTEXT_BOUNDS.json',windows);save('ADMISSIONS.json',admissions);save('MINIMUM_SLACK.json',min((m for m in metrics if m['slack'] is not None),key=lambda m:m['slack']));print(json.dumps(summary,indent=2))
