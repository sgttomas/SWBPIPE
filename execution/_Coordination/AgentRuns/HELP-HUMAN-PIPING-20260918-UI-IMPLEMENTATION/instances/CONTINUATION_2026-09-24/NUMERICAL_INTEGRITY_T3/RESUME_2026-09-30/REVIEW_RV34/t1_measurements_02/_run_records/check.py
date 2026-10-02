"""RV34 T1 independent read-only metadata, byte and arithmetic review.
No product/runner module, author checker, model/count builder or solver is run.
"""
from review_io import *
import collections,re,ast,datetime,statistics
checks=collections.Counter();bad=[]
def eq(k,a,b):
    checks[k]+=1
    if a!=b:bad.append({'check':k,'actual':a,'expected':b})
def ok(k,v):eq(k,bool(v),True)
def strict(pairs):
    d={}
    for k,v in pairs:
        if k in d:raise ValueError('duplicate JSON key '+k)
        d[k]=v
    return d
def loads(b):return json.loads(b,object_pairs_hook=strict)
raw=archive()
def obs(name):return [loads(x) for x in raw[name+'.jsonl'].splitlines() if x.strip()]
def one(rows,kind):
    r=[o for o in rows if o.get('kind')==kind];assert len(r)==1,(kind,len(r));return r[0]

seal=get(P+'/_run_records/SHA256SUMS');eq('author_seal',hashlib.sha256(seal).hexdigest(),'ae41f9d742aa34f9d228bfe335da910ea66fc536ec904cbce760340e75d1d59d')
for line in seal.decode().splitlines():
    h,n=line.split(None,1);eq('author_payload',hashlib.sha256(get(P+'/'+n.strip())).hexdigest(),h)
manifest=packet('RAW_RECORDS_MANIFEST.json');eq('archive_exact_members',sorted(raw),sorted(x['relative'] for x in manifest['files']))
for x in manifest['files']:
    eq('archive_payload_hash',hashlib.sha256(raw[x['relative']]).hexdigest(),x['sha256']);eq('archive_payload_size',len(raw[x['relative']]),x['bytes'])
capture=js(R+'/MEASUREMENTS/W1_T1/_run_records/CAPTURE.json');eq('archive_container_hash',hashlib.sha256(get(R+'/MEASUREMENTS/W1_T1/_run_records/records.tar.gz')).hexdigest(),capture['archive_sha256'])
bind=packet('INPUT_BINDING.json');launch=js(R+'/I26/measurement_plan_03/_run_records/LAUNCH_MANIFEST.json');planned=[x for x in launch['launches'] if x['tier']=='W1-T1']
eq('author_launch_plan',packet('T1_LAUNCHES.json'),planned)
eq('actual_tier_cli',packet('ACTUAL_INVOCATION.json')['argv'],next(x['argv'] for x in launch['tier_cli'] if x['tier']=='W1-T1'))
eq('actual_cwd',packet('ACTUAL_INVOCATION.json')['cwd'],launch['cwd'])
seedbytes=get(H+'/observations/k6b/counts.jsonl',SOURCE);eq('frozen_seed_hash',hashlib.sha256(seedbytes).hexdigest(),bind['seed']['sha256']);seed={r['model']:r for r in map(loads,seedbytes.splitlines())}
known=js(R+'/REVIEW_RV36/measurement_plan_02/_run_records/SCHEDULE_AND_COUNTS_AUDIT.json')
for k in ['complete_seed_consumed_fields','also_compare_invariant_report_fields','fresh_complete_context_estimate_required_fields']:eq('field_map_bound',packet('FIELD_MAP.json')[k],known[k])
consumed=list(known['complete_seed_consumed_fields']);invariant=list(known['also_compare_invariant_report_fields']);estfields=known['fresh_complete_context_estimate_required_fields']
H19={v['run_id']:(h,v) for h in js(R+'/I21/h_numeric_19/PHASES.json')['rows'] for v in h['launch_variants']}
authorruns={r['run_id']:r for r in packet('FULL_CHECKS.json')['runs']}
baseline=loads(raw['baseline_W1-T1.record.json']);baseobs=obs('baseline_W1-T1');meta=loads(raw['metadata.json'])
eq('baseline_kinds',[o['kind'] for o in baseobs],['start','summary']);eq('baseline_noop',baseobs[0]['noop'],True);eq('baseline_repeats',baseobs[1]['repeats_completed'],0)
eq('baseline_cap',baseobs[0]['heap_cap_bytes'],8053063680);eq('baseline_timeout',baseline['timeout_s'],60)
eq('metadata_binary',meta['binary_sha256'],bind['binary']['sha256']);eq('metadata_source',meta['source_commit'],SOURCE);eq('metadata_tree',meta['source_tree'],bind['source_tree'])
BR=baseline['peak_rss_bytes'];BF=baseline['rss']['time_peak_footprint_bytes'];ok('baseline_usable',type(BR)is int and BR>0 and type(BF)is int and BF>0)
journal=[loads(x) for x in raw['records.jsonl'].splitlines()];eq('exact_schedule_order',[r['order'] for r in journal],list(range(139,199)));eq('exact_scheduled_ids',[r['run_id'] for r in journal],[p['run_id'] for p in planned])
eq('journal_no_qualification_rows',len(journal),60)
plans={p['run_id']:p for p in planned};product_records={n[:-12]:loads(b) for n,b in raw.items() if n.endswith('.record.json')}
eq('process_count',len(product_records),91);pids=[]
for name,r in product_records.items():
    oo=obs(name);pids.append(one(oo,'start')['pid'])
    eq('process_exit',r['exit_code'],0);eq('process_class',r['classification'],'ok');eq('process_survivors',r['survivors'],[])
    eq('process_timeout',r['timed_out'],False);eq('process_rss_kill',r['killed_by_rss_watchdog'],False);eq('process_stderr',raw[name+'.stderr.txt'],b'')
    eq('stdout_identity',hashlib.sha256(raw[name+'.jsonl']).hexdigest(),r['stdout_sha256']);eq('stdout_summary',one(oo,'summary'),r['summary'])
    time=raw[name+'.time.txt'].decode()
    rss=int(re.search(r'(\d+)\s+maximum resident set size',time)[1]);foot=int(re.search(r'(\d+)\s+peak memory footprint',time)[1])
    eq('raw_time_rss',rss,r['peak_rss_bytes']);eq('raw_time_footprint',foot,r['rss']['time_peak_footprint_bytes'])
    eq('rss_wrapper_wait',rss,r['rss']['wait4_maxrss_bytes'])
eq('distinct_product_pids',len(set(pids)),91);ok('positive_product_pids',all(x>0 for x in pids))

# Reconstruct H caller bytes from already frozen canonical label metadata.
# Only labels/counted lines are read; no coordinates, coefficients, graph,
# source constructor, model algorithm or production parser is evaluated.
modelmeta={x['model']:x for x in js(R+'/I26/measurement_plan_03/_run_records/MODEL_INPUTS.json')['models']}
def G(s,n):return 0 if n==0 else s*max(4,1<<(n-1).bit_length())
def F(n):return 0 if n==0 else max(8,2*n)
def modelbytes(mid):
    name=mid.replace(':','_');path=H+'/observations/k6/models/'+name+'.k6model';b=get(path,SOURCE)
    eq('canonical_model_hash',hashlib.sha256(b).hexdigest(),modelmeta[mid]['canonical_model_sha256_from_sealed_catalog'])
    eq('canonical_model_len',len(b),seed[mid]['model_canonical_len'])
    grouped=collections.defaultdict(list)
    for l in b.decode().splitlines():
        t=l.split();grouped[t[0]].append(t)
    n=len(grouped['node']);m=len(grouped['member']);rs=len(grouped['restraint']);l=len(grouped['load'])
    eq('metadata_nodes',n,seed[mid]['nodes']);eq('metadata_members',m,seed[mid]['members']);eq('metadata_loads',l,seed[mid]['w1_loads'])
    dec=mid.startswith('DEC053:')
    return max(G(48,n),32*n if dec else 0)+64*m+G(16,rs)+G(16,l)+G(136,m)+sum(F(len(x[2].encode())) for x in grouped['node'])+sum(F(len(x[2].encode())) if dec else len(x[2].encode()) for x in grouped['member'])+len(mid.encode())+len(grouped['source'][0][1].encode())
models={mid:modelbytes(mid) for mid in sorted({p['model'] for p in planned})}

# Exact source policy constants; no function/module is imported.
runner=get(H+'/runner/k6_runner.py',SOURCE).decode();tree=ast.parse(runner)
p1=next(ast.literal_eval(n.value) for n in tree.body if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=='P1_LINUX_PEAK_MIB' for t in n.targets))
def admit(run,e,prior):
    same=[p for p in prior if p['family']==run['family'] and p['mode']==run['mode'] and (100 if run['members']>=1000 else 0)<=p['members']<run['members'] and p['classification']=='ok']
    fps=[max(max(0,p['rss']['time_peak_footprint_bytes']-BF),p['repeats_heap_peak_move'])/p['estimate_adm_bytes'] for p in same]
    rrs=[max(max(0,p['peak_rss_bytes']-BR),p['repeats_heap_peak_move'])/p['estimate_adm_bytes'] for p in same]
    rho=max(fps,default=2.0);rr=max(rrs,default=2.0);rf=max((p['peak_rss_bytes']/p['rss']['time_peak_footprint_bytes'] for p in same),default=1.45)
    half=run['rss_cap_bytes']//2;linux=p1.get((run['model'],run['mode']))
    if linux is not None and linux*1048576<=half:fp=max(int(linux*1048576),e);branch='p1_linux_peak %.1f MiB <= C/2'%linux
    else:
        assert e*rho<=half;fp=int(e*rho);branch='estimate %d B x rho %.3f <= C/2'%(e,rho)
    projected=int(fp*rf);limit=int(.8*run['rss_cap_bytes']);assert e<=run['heap_cap_bytes']//2 and projected<=limit
    return {'decision':'admitted','reason':branch+'; projected RSS %d B <= 0.8C'%projected,'estimate_adm_bytes':e,'rho':rho,'rho_rss':rr,'half_cap_bytes':half,'footprint_estimate_bytes':fp,'rss_to_footprint':rf,'projected_rss_bytes':projected,'projected_rss_limit_bytes':limit},[p['run_id'] for p in same]

counts=[];windows=[];metrics=[];admissions=[];outcomes=[];prefixcount=0;normalrepeats=0
normalize=lambda argv:[os.path.basename(x) if i==0 or os.path.isabs(x) else x for i,x in enumerate(argv)]
for i,r in enumerate(journal):
    rid=r['run_id'];p=plans[rid];o=obs(rid);c=one(o,'counts');st=one(o,'start');su=one(o,'summary');ss=seed[r['model']]
    for key in ['order','model','mode','family','members','pass','tier','heap_cap_bytes','rss_cap_bytes','timeout_s','time_budget_s','first_repeat_limit_s']:eq('schedule_'+key,r[key],p[key])
    eq('normal_argv',r['argv'],normalize(p['argv']));eq('normal_not_conditional',r['conditional'],False);eq('normal_counts_source',c['counts_source'],'file')
    for field in consumed:eq('normal_consumed',c[field],ss[field])
    for field in invariant:eq('normal_invariant',c[field],ss[field])
    eq('normal_repeats',su['repeats_completed'],5);eq('normal_stop',su['stop_reason'],None);normalrepeats+=su['repeats_completed']
    eq('baseline_rss_all',r['baseline_rss_bytes'],BR);eq('baseline_footprint_all',r['baseline_footprint_bytes'],BF)
    eq('row_source',r['source_commit'],SOURCE);eq('row_tree',r['source_tree'],bind['source_tree'])
    expectedstart={'model':r['model'],'model_from_file':False,'mode':r['mode'],'counts_only':False,'repeats':5,'entry_repeats':5,'heap_cap_bytes':8053063680,'time_budget_s':540,'first_repeat_limit_s':600,'allow_over_estimate':False,'schema':'k6-observe-v1'}
    if r['mode']=='w1a':expectedstart.update(case_limit=2**64-1,invocation_limit=2**64-1,w1_prefixes=p['prefixes'],dump_published=p['prefixes'])
    for k,v in expectedstart.items():eq('start_context',st[k],v)
    for par in [x for x in o if x['kind']=='parity']:eq('parity_true',par['equal'],True)
    for stage in [x for x in o if x['kind']=='stage']:eq('stage_ok',stage['ok'],True)
    outs=[x for x in o if x['kind']=='outcome'];eq('outcome_repeats',[x['repeat'] for x in outs],list(range(5)))
    eq('repeat_outcome_identity',[{k:v for k,v in x.items() if k!='repeat'} for x in outs],[{k:v for k,v in outs[0].items() if k!='repeat'}]*5)
    for stage,agg in r['stages'].items():
        rows=[x for x in o if x.get('kind')=='stage' and x['stage']==stage]
        eq('stage_samples',len(rows),agg['samples'])
        for k in ['heap_peak','heap_peak_move']:eq('stage_max',max(x[k] for x in rows),agg[k])
        eq('stage_min_time',min(x['elapsed_ns'] for x in rows),agg['min_ns']);eq('stage_median_time',int(statistics.median(x['elapsed_ns'] for x in rows)),agg['median_ns'])
    if r['mode']=='w1a':
        bn=rid+'.counts-binding';bo=obs(bn);bc=one(bo,'counts');bst=one(bo,'start');bs=one(bo,'summary')
        eq('prepass_kinds',[x['kind'] for x in bo],['start','counts','summary']);eq('prepass_source',bc['counts_source'],'computed');eq('prepass_repeats',bs['repeats_completed'],0)
        eq('prepass_argv',product_records[bn]['argv'],normalize(p['counts_binding_argv']));eq('prepass_exact_extension',p['counts_binding_argv'],p['argv']+['--counts-only'])
        for k,v in expectedstart.items():eq('prepass_start_context',bst[k],True if k=='counts_only' else v)
        for field in consumed:eq('prepass_consumed',bc[field],ss[field])
        for field in invariant:eq('prepass_invariant',bc[field],ss[field])
        for field in estfields:eq('prepass_normal_estimate',bc[field],c[field]);ok('estimate_integer',type(c[field])is int and c[field]>=0)
        h,v=H19[rid];args=sum(len(p['argv'][j+1].encode()) for j,x in enumerate(p['argv']) if x in ['--model','--model-file','--counts-file','--dump-solution','--dump-pattern','--dump-published'])
        eq('retained_args_bytes',args,p['H_persistent_arg_string_bytes'])
        C=models[r['model']]+1700+args+h['owners']['saved_attempts'];fixed=C+h['owners']['BASE']+(387 if p['prefixes'] else 0)
        eq('independent_actual_fixed',fixed,c['estimate_w1_fixed_bytes']);delta=fixed-v['Hfixed'];w={}
        for schedule,t in v['totals'].items():w[schedule]={k:value+delta if value else 0 for k,value in t.items()}
        eq('independent_full',w['all']['moving_bytes'],c['estimate_adm_bytes_w1a']);eq('independent_selected128',w['selected128']['moving_bytes'],c['estimate_w1_sel128_bytes'])
        eq('author_H_windows',w,authorruns[rid]['H_windows']);eq('author_fixed_delta',delta,authorruns[rid]['Hfixed_delta'])
        for field in estfields:
            if field not in ['estimate_adm_bytes_w1a','estimate_w1_fixed_bytes','estimate_w1_sel128_bytes']:eq('kernel_diagnostic_seed',c[field],ss[field])
        windows.append({'run_id':rid,'model_bytes':models[r['model']],'args_bytes':args,'fixed_bytes':fixed,'fixed_delta':delta,'max_bytes':c['estimate_adm_bytes_w1a'],'sel128_bytes':c['estimate_w1_sel128_bytes']})
        for out in outs:
            eq('W1_selected',out['class'],'Selected');eq('W1_precision',out['selected_precision'],128);eq('W1_verification',out['verification_precision'],256);eq('published_row_total',sum(out[k] for k in ['rows_relative_verified','rows_absolute_verified','rows_input_derived','rows_unpublishable']),out['rows']);eq('publication_count',out['rows'],ss['w1_rows']);eq('budget_not_reached',out['budget_reached'],False)
            attempts=[x for x in o if x['kind']=='attempt' and x['repeat']==out['repeat']];eq('attempt_count',len(attempts),out['attempts'])
            eq('attempt_precision',[a['precision'] for a in attempts],[128,256]);eq('charged_work',sum(a['charged_by'] for a in attempts),out['meter_charged'])
            for a in attempts:
                eq('attempt_storage_pattern',a['storage_pattern_entries'],ss['w1_pattern_entries']);eq('attempt_storage_profile',a['storage_profile_entries'],ss['w1_profile_entries']);eq('attempt_storage_limbs',a['storage_limbs_per_entry'],ss['w1_limbs_per_entry_'+str(a['precision'])]);eq('work_closed',a['own_total']+a['shared_work']+a['verification_shared_work'],a['charged_by']);eq('work_stages_complete',a['stages_complete'],True);eq('own_unstaged',a['own_unstaged'],0);eq('shared_unstaged',a['shared_unstaged'],0)
        prefixes=[x for x in o if x['kind']=='prefix'];prefixcount+=len(prefixes);eq('actual_prefix_count',len(prefixes),3 if p['prefixes'] else 0)
        aa=[x for x in o if x['kind']=='attempt' and x['repeat']==4];segments=[];charge=0
        for j,a in enumerate(aa):
            charge+=a['shared_work']+a['own_total']-a['verification_work']-a['stop_rule_work'];segments.append(('solve_'+str(a['precision']),charge))
            if a['verification_work'] or a['verification_shared_work']:
                charge+=a['verification_work']+a['verification_shared_work'];segments.append(('verify_'+str(a['precision']),charge))
                if j and aa[j-1]['stop_rule_work']:charge+=aa[j-1]['stop_rule_work'];segments.append(('decide_'+str(aa[j-1]['precision']),charge))
        if prefixes:eq('prefix_actual_segment_limits',[(x['through_segment'],x['case_limit']) for x in prefixes],segments[:-1])
        for pre in prefixes:eq('prefix_class',pre['class'],'Unresolved');eq('prefix_reason',pre['reason'],'Budget(Case)')
        eq('publication_dump_scope',rid+'.rows' in raw,p['prefixes'])
        for ob in o:
            for field,value in ob.items():
                if 'heap' not in field or type(value)is not int:continue
                metric='moving' if 'move' in field else 'requested';bound=None;kind='outside_window'
                if field=='heap_cap_bytes':kind='configuration'
                elif ob['kind']=='stage':
                    stage=ob['stage'];phase='source' if stage=='w1_source' else 'solve' if stage=='w1_solve' else 'prefix';bound=w['all'][phase+'_'+metric];kind='matched'
                elif ob['kind']=='summary' and field.startswith('repeats_'):bound=max(w['all']['source_'+metric],w['all']['solve_'+metric]);kind='matched'
                elif ob['kind']=='summary' and field.startswith('prefix_'):bound=w['all']['prefix_'+metric];kind='matched'
                if bound is not None:ok('H_window_bound',value<=bound)
                metrics.append({'run_id':rid,'kind':ob['kind'],'stage':ob.get('stage'),'repeat':ob.get('repeat'),'field':field,'observed':value,'metric':metric,'bound':bound,'slack':None if bound is None else bound-value,'class':kind})
    else:
        for out in outs:eq('sparse_outcome',out['class'],'Passed')
        eq('sparse_dump',rid+'.u' in raw,True)
    estimate=c['estimate_adm_bytes_'+r['mode']];eq('journal_estimate',r['estimate_adm_bytes'],estimate)
    decision,pop=admit(r,estimate,journal[:i]);eq('chronological_admission',decision,r['admission']);admissions.append({'run_id':rid,'calibration_population':pop,'estimate':estimate,'decision':decision})
    outcomes.append({'run_id':rid,'mode':r['mode'],'completed_repeats':len(outs),'outcome':outs[0]['class'],'precision':outs[0].get('selected_precision')})

author=packet('METRIC_WINDOW_CHECKS.json');eq('metric_record_count',len(metrics),len(author))
for a,b in zip(metrics,author):eq('all_metric_values',{k:v for k,v in a.items() if k!='class'},{k:v for k,v in b.items() if k!='disposition'})
summary={'normal_rows':len(journal),'product_processes':len(product_records),'completed_normal_repeats':normalrepeats,'prefix_calls':prefixcount,'H_fields':dict(collections.Counter(m['class'] for m in metrics)),'minimum_H_slack':min(m['slack'] for m in metrics if m['slack'] is not None),'max_W1_requested':max(r['repeats_heap_peak'] for r in journal if r['mode']=='w1a'),'max_W1_moving':max(r['repeats_heap_peak_move'] for r in journal if r['mode']=='w1a'),'max_product_RSS':max(r['peak_rss_bytes'] for r in product_records.values()),'baseline_RSS':BR,'baseline_footprint':BF,'normal_outcomes':dict(collections.Counter(x['outcome'] for x in outcomes)),'all_T1_members_below100':all(r['members']<100 for r in journal),'failures':bad,'check_counts':dict(checks),'checked_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()}
save('INDEPENDENT_CHECKS.json',summary);save('INDEPENDENT_CONTEXT_BOUNDS.json',windows);save('INDEPENDENT_ADMISSIONS.json',admissions);save('INDEPENDENT_OUTCOMES.json',outcomes)
save('INDEPENDENT_METRIC_MINIMUMS.json',{'minimum':min((m for m in metrics if m['slack'] is not None),key=lambda m:m['slack']),'groups':{cl:{'count':sum(m['class']==cl for m in metrics),'minimum_slack':min((m['slack'] for m in metrics if m['class']==cl and m['slack'] is not None),default=None)} for cl in ['matched','outside_window','configuration']}})
print(json.dumps(summary,indent=2))
