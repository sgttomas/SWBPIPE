from review_io import *
import ast,re,collections,datetime,stat
checks=collections.Counter();bad=[]
def eq(k,a,b):
 checks[k]+=1
 if a!=b:bad.append({'check':k,'actual':a,'expected':b})
def ok(k,v):eq(k,bool(v),True)
# Reuse only RV34's prior independent pure admission/metadata helpers.
s=get(R+'/REVIEW_RV34/t1_measurements_02/_run_records/check.py','468f14ec733026920729ca601c9d3243ff0f61aa').decode();names={'strict','loads','one','G','F','admit'}
exec(compile(ast.Module(body=[n for n in ast.parse(s).body if isinstance(n,ast.FunctionDef) and n.name in names],type_ignores=[]),'RV34 independent pure helpers','exec'),globals())
t3=get(R+'/REVIEW_RV34/t3_measurements_05/_run_records/check.py').decode();rep=next(ast.literal_eval(n.value) for n in ast.parse(t3).body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='replacement' for t in n.targets))
nodes=[n for n in ast.parse(rep).body if isinstance(n,ast.FunctionDef) and n.name in {'digit_bins','label_total','modelbytes'}];assert len(nodes)==3
exec(compile(ast.Module(body=nodes,type_ignores=[]),'RV34 accepted Builder scalar substitution','exec'),globals())
models_source=get(H+'/src/k6/models.rs',SOURCE).decode();r1_source=re.search(r'pub const R1_SOURCE: &str = "([^"]+)"',models_source)[1]
raw=archive(4);bind=packet('INPUT_BINDING.json');acq=packet('ACQUISITIONS.json');recipe=packet('EXECUTION_AND_COMPARISON_RECIPE.json');ready=packet('ADMISSION_READINESS.json')
seal=get(P+'/_run_records/SHA256SUMS');eq('author_seal',hashlib.sha256(seal).hexdigest(),'34e07f880cf196a94717d69429db85aa6e8db65e3e0c3520725f221d20c435f6')
for line in seal.decode().splitlines():
 h,n=line.split(None,1);eq('sealed_payload',hashlib.sha256(get(P+'/'+n.strip())).hexdigest(),h)
inv=packet('RAW_INVENTORY.json');eq('raw_exact_roster',sorted(raw),sorted(pathlib.Path(x['path']).name for x in inv['new_qualification_files']))
for x in inv['new_qualification_files']:
 b=raw[pathlib.Path(x['path']).name];eq('raw_hash',hashlib.sha256(b).hexdigest(),x['sha256']);eq('raw_size',len(b),x['bytes'])
capture=js(R+'/MEASUREMENTS/W1_T4_READINESS/_run_records/CAPTURE.json');eq('archive_hash',hashlib.sha256(get(R+'/MEASUREMENTS/W1_T4_READINESS/_run_records/qualification.tar.gz')).hexdigest(),capture['archive_sha256'])
def obs(rid):return list(map(loads,raw[rid+'.jsonl'].splitlines()))
seedbytes=get(H+'/observations/k6b/counts.jsonl',SOURCE);seed={r['model']:r for r in map(loads,seedbytes.splitlines())};eq('seed_hash',hashlib.sha256(seedbytes).hexdigest(),bind['seed']['sha256'])
launch=js(R+'/I26/measurement_plan_03/_run_records/LAUNCH_MANIFEST.json');planned=[p for p in launch['launches'] if p['tier']=='W1-T4'];plans={p['order']:p for p in planned};eq('planned24',len(planned),24);eq('author_full_schedule',packet('T4_LAUNCHES.json'),planned)
known=js(R+'/REVIEW_RV36/measurement_plan_02/_run_records/SCHEDULE_AND_COUNTS_AUDIT.json');consumed=list(known['complete_seed_consumed_fields']);invariant=list(known['also_compare_invariant_report_fields']);estfields=known['fresh_complete_context_estimate_required_fields']
for short,long in [('consumed','complete_seed_consumed_fields'),('invariants','also_compare_invariant_report_fields'),('estimates','fresh_complete_context_estimate_required_fields')]:eq('complete_field_roster',packet('FIELD_MAP.json')[short],list(known[long]))
H19={v['run_id']:(h,v) for h in js(R+'/I21/h_numeric_19/PHASES.json')['rows'] for v in h['launch_variants']}
models={mid:modelbytes(mid) for mid in {p['model'] for p in planned}}
eq('acquisition_count',len(acq['calls']),13);eq('acquisition_roster',[q['order'] for q in acq['calls']],[None,247,249,252,254,255,257,260,262,263,265,268,270])
normalize=lambda argv:[os.path.basename(a) if i==0 or os.path.isabs(a) else a for i,a in enumerate(argv)]
fresh={};pids=[];caller=[];base=None
for q in acq['calls']:
 rid=q['run_id'];r=loads(raw[rid+'.record.json']);oo=obs(rid);st=one(oo,'start');su=one(oo,'summary');pids.append(st['pid'])
 eq('process_record',r,q['record']);eq('process_PID',st['pid'],q['product_pid']);eq('exit0',r['exit_code'],0);eq('class_ok',r['classification'],'ok');eq('no_timeout',r['timed_out'],False);eq('no_RSS_kill',r['killed_by_rss_watchdog'],False);eq('stderr',raw[rid+'.stderr.txt'],b'');eq('stdout_hash',hashlib.sha256(raw[rid+'.jsonl']).hexdigest(),r['stdout_sha256']);eq('summary_match',r['summary'],su);eq('zero_repeats',su['repeats_completed'],0);eq('stop_none',su['stop_reason'],None)
 eq('argv_normalized',r['argv'],normalize(q['argv']));eq('actual_cwd',q['cwd'],launch['cwd']);eq('timeout',r['timeout_s'],60 if q['kind']=='noop' else 1800);eq('RSS_cap',r['rss_cap_bytes'],8589934592);eq('heap_cap',st['heap_cap_bytes'],8053063680);eq('wrapper',r['wrapper'],'/usr/bin/time -l')
 eq('guard_recorded','5387' in q['guard'] and '/guard/memguard.sh' in q['guard'],True);ok('quiet_wait',q['quiet']['waited_s']<=120)
 time=raw[rid+'.time.txt'].decode();rss=int(re.search(r'(\d+)\s+maximum resident set size',time)[1]);foot=int(re.search(r'(\d+)\s+peak memory footprint',time)[1]);eq('time_RSS',rss,r['peak_rss_bytes']);eq('time_footprint',foot,r['rss']['time_peak_footprint_bytes']);eq('time_wait4',rss,r['rss']['wait4_maxrss_bytes'])
 if q['kind']=='noop':
  eq('noop_kinds',[o['kind'] for o in oo],['start','summary']);eq('noop_flag',st['noop'],True);eq('noop_exact_argv',q['argv'],[bind['binary']['path'],'--noop','--heap-cap-bytes','8053063680']);base=r;continue
 p=plans[q['order']];ss=seed[q['model']];c=one(oo,'counts');eq('counts_kinds',[o['kind'] for o in oo],['start','counts','summary']);eq('counts_computed',c['counts_source'],'computed');eq('qualification_exact_argv',q['argv'],p['counts_binding_argv']);eq('exact_extension',q['argv'],p['argv']+['--counts-only'])
 for f in consumed:eq('consumed_field',c[f],ss[f])
 for f in invariant:eq('invariant_field',c[f],ss[f])
 for f in estfields:ok('fresh_estimate_type',type(c[f])is int and c[f]>=0)
 expected={'model':p['model'],'model_from_file':False,'mode':'w1a','counts_only':True,'repeats':5,'entry_repeats':5,'heap_cap_bytes':8053063680,'time_budget_s':1740,'first_repeat_limit_s':600,'allow_over_estimate':False,'schema':'k6-observe-v1','case_limit':2**64-1,'invocation_limit':2**64-1,'w1_prefixes':p['prefixes'],'dump_published':p['prefixes']}
 for k,v in expected.items():eq('start_context',st[k],v)
 h,v=H19[p['run_id']];args=sum(len(p['argv'][i+1].encode()) for i,f in enumerate(p['argv']) if f in ['--model','--model-file','--counts-file','--dump-solution','--dump-pattern','--dump-published']);eq('argument_bytes',args,p['H_persistent_arg_string_bytes']);fixed=models[p['model']]+1700+args+h['owners']['saved_attempts']+h['owners']['BASE']+(387 if p['prefixes'] else 0);eq('independent_fixed',fixed,c['estimate_w1_fixed_bytes']);delta=fixed-v['Hfixed']
 eq('independent_full',v['totals']['all']['moving_bytes']+delta,c['estimate_adm_bytes_w1a']);eq('independent_selected128',v['totals']['selected128']['moving_bytes']+delta,c['estimate_w1_sel128_bytes'])
 for f in estfields:
  if f not in ['estimate_adm_bytes_w1a','estimate_w1_sel128_bytes','estimate_w1_fixed_bytes']:eq('kernel_diagnostic_seed',c[f],ss[f])
 eq('sparse_seed_correspondence',c['estimate_adm_bytes_sparse'],ss['estimate_adm_bytes_sparse']);fresh[q['order']]=c;caller.append({'order':q['order'],'model':q['model'],'model_bytes':models[q['model']],'argument_bytes':args,'fixed':fixed,'max':c['estimate_adm_bytes_w1a'],'sel128':c['estimate_w1_sel128_bytes']})
eq('distinct_product_PIDs',len(set(pids)),13);eq('archive_has_no_dump',any(n.endswith(('.rows','.u')) for n in raw),False);eq('archive_has_no_normal_journal',any(n.endswith('records.jsonl') for n in raw),False)
BR=base['peak_rss_bytes'];BF=base['rss']['time_peak_footprint_bytes'];ok('valid_baseline_metrics',BR>0 and BF>0)
jbytes=archive(3)['records/records.jsonl'];journal=list(map(loads,jbytes.splitlines()));eq('historical_journal_hash',hashlib.sha256(jbytes).hexdigest(),bind['journal']['sha256']);eq('historical_journal_rows',len(journal),108)
# Here live file equality is a preservation check only. Projection uses immutable bytes.
eq('live_journal_unchanged',pathlib.Path(bind['journal']['path']).read_bytes(),jbytes)
runner=get(H+'/runner/k6_runner.py',SOURCE).decode();assignments={t.id:n.value for n in ast.parse(runner).body if isinstance(n,ast.Assign) for t in n.targets if isinstance(t,ast.Name)};p1=ast.literal_eval(assignments['P1_LINUX_PEAK_MIB']);holds=ast.literal_eval(assignments['CONDITIONAL_TIERS']);eq('sole_actual_hold',list(holds),['W1-T4'])
projected=[]
for row,p in zip(ready['rows'],planned):
 eq('projection_order',row['order'],p['order']);eq('hold_deferred',row['current_named_hold'],{'decision':'deferred','reason':holds['W1-T4']});eq('hypothetical_only',row['hypothetical_ROOT_named_release_only'],True)
 pop=[r for r in journal if r['family']==p['family'] and r['mode']==p['mode'] and 100<=r['members']<p['members'] and r['classification']=='ok'];eq('eight_eligible',len(pop),8);eq('T2_T3_only',all(r['tier'] in ['W1-T2','W1-T3'] for r in pop),True)
 roster=[{'run_id':r['run_id'],'tier':r['tier'],'members':r['members'],'own_actual_estimate_denominator':r['estimate_adm_bytes'],'footprint_rho_using_fresh_baseline':max(max(0,r['rss']['time_peak_footprint_bytes']-BF),r['repeats_heap_peak_move'])/r['estimate_adm_bytes'],'rss_rho_using_fresh_baseline':max(max(0,r['peak_rss_bytes']-BR),r['repeats_heap_peak_move'])/r['estimate_adm_bytes'],'rss_to_footprint':r['peak_rss_bytes']/r['rss']['time_peak_footprint_bytes']} for r in pop];eq('all_prior_denominators_ratios',roster,row['eligible_prior_calibration'])
 prev=p['model'].replace('-n10000-','-n01000-');ascent=[r['run_id'] for r in journal if r['model']==prev and r['mode']==p['mode'] and r['classification'] not in [None,'not_run']];eq('ascent_previous',prev,row['ascent_previous_model']);eq('ascent_roster',ascent,row['ascent_recorded_run_ids']);ok('ascent_exists',len(ascent)>0)
 e=fresh[p['order']]['estimate_adm_bytes_w1a'] if p['mode']=='w1a' else seed[p['model']]['estimate_adm_bytes_sparse'];d,popids=admit(p,e,journal);eq('independent_numeric_dictionary',d,row['prospective_numeric_admission']);eq('baseline_RSS_projection',row['baseline_rss_bytes'],BR);eq('baseline_footprint_projection',row['baseline_footprint_bytes'],BF);eq('hypothetical_eligibility',row['hypothetical_eligibility'],None)
 projected.append({'run_id':p['run_id'],'mode':p['mode'],'current_hold':row['current_named_hold'],'hypothetical_decision':d,'eligible_prior_ids':popids,'ascent':ascent})
eq('projection24',len(projected),24);eq('largest_W1',max(x['max'] for x in caller),ready['summary']['largest_W1_estimate']);eq('largest_projected_RSS',max(x['hypothetical_decision']['projected_rss_bytes'] for x in projected),ready['summary']['largest_projected_RSS'])
save('NUMERICAL_CHECKS.json',{'check_counts':dict(checks),'failures':bad,'actual_calls':13,'count_contexts':len(fresh),'baseline_RSS':BR,'baseline_footprint':BF,'global_hold_literal':holds,'hypothetical_counts':dict(collections.Counter(x['hypothetical_decision']['decision'] for x in projected)),'largest_W1':max(x['max'] for x in caller),'largest_projected_RSS':max(x['hypothetical_decision']['projected_rss_bytes'] for x in projected),'checked_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()});save('CALLER_RECOMPUTATION.json',caller);save('PROJECTIONS.json',projected)
print(json.dumps({'checks':dict(checks),'failures':bad,'hypothetical_decisions':dict(collections.Counter(x['hypothetical_decision']['decision'] for x in projected))},indent=2))
