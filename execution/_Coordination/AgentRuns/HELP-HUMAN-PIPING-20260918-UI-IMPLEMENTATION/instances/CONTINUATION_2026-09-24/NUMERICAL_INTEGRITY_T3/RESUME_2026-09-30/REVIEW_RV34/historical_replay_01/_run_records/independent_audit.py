"""RV34 independent arithmetic/chronology checker; no author code is imported.
Reads frozen JSON/source bytes and runs ordinary integer/float arithmetic only.
"""
from review_io import *
import re, collections, copy, datetime, math

checks=collections.Counter(); failures=[]
def eq(label,a,b):
    checks[label]+=1
    if a!=b: failures.append({'check':label,'actual':a,'expected':b})
def ok(label,value): eq(label,bool(value),True)

histories={d:rows(T3+'/IMPLEMENTATION/'+d+'/_run_records/'+s+'/records.jsonl') for d,s in [('K6B','b3/records'),('VK','b/runs'),('KF3','b/runs')]}
H={x['model']:x for x in jget(R+'/I21/h_numeric_19/PHASES.json')['rows']}
V={x['id']:x for x in jget(R+'/metric_design_15_result5_join/RESULT5_CLI24.json')['rows']}
VC={x['id']:x for x in jget(R+'/metric_design_14_reference_callers/CLI24_CALLERS.json')['rows']}
VK={x['id']:x for x in jget(R+'/I21/kernel_reference_22/CLI24_EXACT_KERNEL.json')}
plan={x['run_id']:x for x in jget(R+'/I21/h_numeric_19/LAUNCH_BINDING.json')['plan']}

# Verify frozen source snapshots against the named Git objects, not current files.
for src in packet('SOURCE_ORIGINS.json'):
    b=get(src['source_path'],src['source_commit'])
    eq('source_snapshot_bytes',b,(PKT/src['local_snapshot']).read_bytes())
    eq('source_snapshot_hash',hashlib.sha256(b).hexdigest(),src['sha256'])

# Original full launch script supplies exact strings. Retained H parser fields
# exclude argv0 and consumed scalar/flag strings, as checked in source.
script=(PKT/'H_ORIGINAL_run_slot.sh.txt').read_text()
binary,counts_path,records_path=re.search(r'--binary (\S+) --counts (\S+) --records (\S+)',script).groups()
binding={x['run_id']:x for x in packet('H_ACTUAL_LAUNCH_BINDING.json')['rows']}
retained_flags={'--model','--model-file','--counts-file','--dump-solution','--dump-pattern','--dump-published'}
actual_args={}; deltas={}; envelopes={}; reference_envelopes={}
for run in histories['K6B']:
    if run.get('mode')!='w1a':continue
    rid=run['run_id']; h=H[run['model']]; ref=plan[rid]; argv=run['argv'][:]; argv[0]=binary
    eq('counts_token',argv[argv.index('--counts-file')+1],counts_path)
    if '--dump-published' in argv:
        pos=argv.index('--dump-published')+1;eq('dump_basename',argv[pos],rid+'.rows');argv[pos]=records_path+'/'+argv[pos]
    live=sum(len(argv[i+1].encode()) for i,a in enumerate(argv) if a in retained_flags)
    reference=sum(len(ref['argv'][i+1].encode()) for i,a in enumerate(ref['argv']) if a in retained_flags)
    delta=live-reference; deltas[rid]=delta; actual_args[rid]=live
    eq('launch_reconstruction',argv,binding[rid]['argv_reconstructed_from_original_script_and_record'])
    eq('H_path_delta',delta,binding[rid]['actual_minus_reference_bytes'])
    eq('H_path_delta_pass',delta,-145 if run['pass']==1 else -55)
    eq('H_reference_arg_bytes',reference,ref['persistent_arg_string_bytes'])
    eq('H_actual_arg_bytes',live,binding[rid]['retained_string_utf8_bytes'])
    eq('H_repeat',run['repeats'],5);eq('H_entry_repeat',run['entry_repeats'],5)
    eq('H_prefix',('--w1-prefixes' in argv),run['pass']==1)
    var=next(x for x in h['launch_variants'] if x['run_id']==rid)
    for argbytes,dest in [(reference,reference_envelopes),(live,envelopes)]:
        C=h['owners']['caller_model_runtime_without_args']+h['owners']['saved_attempts']+argbytes
        result={'fixed':h['owners']['BASE']+C+(387 if var['prefix'] else 0)}
        for schedname,s in h['schedules'].items():
            t={}
            for metric,err,emit,line in [('requested',393616,2756016,708),('moving',590424,3937177,1023)]:
                phase=max(p[metric+'_bytes'] for name,p in s['phases'].items() if name!='source_constructor')
                eq('H_kernel_max',phase,s['kernel_'+metric])
                source=C+max(s['phases']['source_constructor'][metric+'_bytes'],150 if metric=='requested' else 225)
                solve=C+max(phase,s['Kpad']+err)
                prefix=C+387+max(phase,s['Kpad']+err,s['Kpad']+emit,s['selected_return_upper']+line) if var['prefix'] else 0
                t.update({metric+'_bytes':max(source,solve,prefix),'source_'+metric:source,'solve_'+metric:solve,'prefix_'+metric:prefix})
            result[schedname]=t
            if argbytes==reference:eq('H_reference_total_recomposition',t,var['totals'][schedname])
        dest[rid]=result

# Recompose VR join maxima from accepted caller/kernel inputs, separately from H.
vr_bounds={}
for mid,v in V.items():
    c=VC[mid]; k=VK[mid]; out={}
    eq('VR_descriptor_identity',k['descriptor'],v['descriptor'])
    for metric in ['requested','moving']:
        cc=c[metric]; fields=v['metrics'][metric]['fields']; totals={}
        for sched,s in k['schedules'].items():
            phase=max(x[metric+'_bytes'] for x in s['phases'].values())
            eq('VR_kernel_max',phase,s['kernel_'+metric])
            stationary=max(s['outcome_union'][z] for z in ['selected','refused','unresolved'])
            terms=list(cc['caller_only_phases'].values())+[a+(phase if name=='solve_max' else stationary) for name,a in cc['kernel_phase_addends'].items()]
            totals[sched]=max(terms)
        model=cc['result5_components']['M']+cc['result5_components']['P_pub']
        fixed=k['owners']['B0']+cc['result5_components']['fixed_caller_retained_addend']
        decide=max(x[metric] for x in k['schedules']['all']['R7_by_precision'].values())
        eq('VR_five_fields',{'model':model,'fixed':fixed,'decide':decide,'max':totals['all'],'sel128':totals['selected128']},fields)
        out[metric]=totals['all'];out['fixed']=fixed;out['model']=model
        f=k['descriptor']['f'];factor=4*f*(f+1)
        eq('VR_sparse_subtotal',cc['details']['sparse']['factor_values'],factor)
        if c['members']==10000:ok('VR_path_independent_backstop',factor>4026531840)
    vr_bounds[mid]=out

def earlier_population(run,prior):
    return [p for p in prior if p.get('family')==run['family'] and p.get('mode')==run['mode'] and (100 if run['members']>=1000 else 0)<=p['members']<run['members'] and p.get('classification')=='ok']
def prev_model(run):
    if not run['model'].startswith('RF-LARGE-'):return None
    n=run['members']; sizes=[10,100,1000,10000]
    return None if n==10 else run['model'].replace('-n%05d-'%n,'-n%05d-'%sizes[sizes.index(n)-1])
def ratios(run,p,e):
    bf=run.get('baseline_footprint_bytes') or 0;br=run.get('baseline_rss_bytes') or 0
    ft=(p.get('rss') or {}).get('time_peak_footprint_bytes'); rss=p.get('peak_rss_bytes'); heap=p.get('repeats_heap_peak_move') or 0
    fp=max(0,(rss or 0)-bf) if ft is None else max(0,ft-bf)
    return {'run_id':p['run_id'],'order':p['order'],'members':p['members'],'estimate_adm_bytes':e,'footprint_ratio':max(fp,heap)/e,'rss_ratio':max(max(0,(rss or 0)-br),heap)/e,'rss_to_footprint':rss/ft if rss and ft else None,'net_footprint':max(0,(ft or 0)-bf),'heap_move':heap}
def admission(ds,run,e,prior,denominator):
    # These historical rows are all W1: the named dense/lane refusals and
    # P1 Linux sparse/dense exception cannot match any reviewed row.
    assert run['mode']=='w1a'
    cap=run['heap_cap_bytes']//2
    if ds!='K6B' and e>cap:
        return {'decision':'deferred','reason':"deferred:the binary's backstop would refuse (estimate %d B > half the heap cap %d B)"%(e,cap)}
    half=run['rss_cap_bytes']//2;limit=int(0.8*run['rss_cap_bytes'])
    out=dict(decision=None,reason=None,estimate_adm_bytes=e,rho=None,rho_rss=None,half_cap_bytes=half,footprint_estimate_bytes=None,rss_to_footprint=None,projected_rss_bytes=None,projected_rss_limit_bytes=limit)
    previous=prev_model(run)
    if previous and (ds=='K6B' or run['members']>100) and not any(p.get('model')==previous and p.get('mode')=='w1a' and p.get('classification') not in [None,'not_run'] for p in prior):
        out.update(decision='deferred',reason='deferred:ascent_previous_size_not_recorded (%s)'%previous);return out
    pop=[ratios(run,p,denominator(p)) for p in earlier_population(run,prior)]
    rho=max((p['footprint_ratio'] for p in pop),default=2.0)
    rr=max((p['rss_ratio'] for p in pop),default=2.0)
    rf=max((p['rss_to_footprint'] for p in pop if p['rss_to_footprint'] is not None),default=1.45)
    out.update(rho=rho,rho_rss=rr,rss_to_footprint=rf)
    if e*rho>half:
        out.update(decision='deferred',reason='deferred:estimate_fails_admission (%d B x rho %.3f > C/2)'%(e,rho));return out
    footprint=int(e*rho);projected=int(footprint*rf)
    out.update(footprint_estimate_bytes=footprint,projected_rss_bytes=projected)
    if e>cap:out.update(decision='deferred',reason='deferred:binary_backstop_refuses (estimate %d B > heap cap / 2 = %d B)'%(e,cap));return out
    if projected>limit:
        out.update(decision='deferred',reason='deferred:projected_rss_exceeds_0.8C (%d B x %.3f = %d B > %d B)'%(footprint,rf,projected,limit));return out
    out.update(decision='admitted',reason='estimate %d B x rho %.3f <= C/2; projected RSS %d B <= 0.8C'%(e,rho,projected));return out

all_replay=[]; all_peak=[]; raw_fields=[]; cf_rows=[]
for actual,prefix in [(False,''),(True,'ACTUAL_H_')]:
    table={(x['dataset'],x['run_id']):x for x in packet(prefix+'REPLAY.json')}
    counter={(x['dataset'],x['run_id']):x for x in packet(prefix+'COUNTERFACTUAL.json')}
    peak={(x['dataset'],x['run_id']):x for x in packet(prefix+'PEAK_COMPARISONS.json')}
    hb=envelopes if actual else reference_envelopes
    for ds,runs in histories.items():
        estimate=lambda r:hb[r['run_id']]['all']['moving_bytes'] if ds=='K6B' else vr_bounds[r['model']]['moving']
        available=[]
        for i,run in enumerate(runs):
            if run.get('mode')!='w1a':available.append(run);continue
            rid=run['run_id']; prior=runs[:i]; row=table[ds,rid]; p=peak[ds,rid]; est=estimate(run)
            ok('strict_chronology',all(q['order']<run['order'] for q in prior if 'order' in q))
            old=admission(ds,run,run['estimate_adm_bytes'],prior,lambda q:q['estimate_adm_bytes'])
            new=admission(ds,run,est,prior,estimate); cf=admission(ds,run,est,available,estimate)
            eq('old_original_dictionary',old,run['admission']);eq('old_packet_dictionary',old,row['old_reproduced'])
            eq('new_estimate',est,row['new_estimate_bytes']);eq('estimate_delta',est-run['estimate_adm_bytes'],row['estimate_delta_bytes'])
            eq('new_dictionary',new,row['new_as_recorded_history'])
            eq('calibration_evaluated',new.get('rho') is not None,row['calibration_evaluated_by_admission'])
            pop=[ratios(run,q,estimate(q)) for q in earlier_population(run,prior)]
            eq('all_prior_denominators_and_ratios',pop,row['calibration_population'])
            eq('ascent_witnesses',[q['run_id'] for q in prior if q.get('model')==prev_model(run) and q.get('mode')=='w1a' and q.get('classification') not in [None,'not_run']],row['prior_recorded_ascent_ids'])
            lost=[q['run_id'] for q in earlier_population(run,prior) if q['run_id'] not in {a['run_id'] for a in available}]
            eq('counterfactual_dictionary',cf,counter[ds,rid]['counterfactual_with_removed_nonadmitted_observations']);eq('counterfactual_lost',lost,counter[ds,rid]['lost_calibration_run_ids']);eq('counterfactual_decision_equal',cf,new)
            if cf['decision']=='admitted':available.append(run)
            if ds=='K6B':
                e=hb[rid]['all']; bounds={m:max(e['source_'+m],e['solve_'+m]) for m in ['requested','moving']}
                eq('H_fixed',hb[rid]['fixed'],p['fixed'])
            else:bounds=vr_bounds[run['model']];eq('VR_fixed',bounds['fixed'],p['fixed']);eq('VR_model',bounds['model'],p['model_component'])
            for m,field in [('requested','repeats_heap_peak'),('moving','repeats_heap_peak_move')]:
                eq('summary_bound',bounds[m],p[m+'_bound']);eq('summary_observed',run[field],p['historical_'+m]);eq('summary_slack',bounds[m]-run[field],p[m+'_slack']);ok('summary_nonnegative',bounds[m]>=run[field])
                if ds=='K6B':
                    obs=run['summary'].get('prefix_phase_heap_peak'+('_move' if m=='moving' else ''))
                    eq('prefix_observed',obs,p['prefix_'+m+'_observed'])
                    if obs is not None:eq('prefix_bound',e['prefix_'+m],p['prefix_'+m+'_bound']);eq('prefix_slack',e['prefix_'+m]-obs,p['prefix_'+m+'_slack']);ok('prefix_nonnegative',e['prefix_'+m]>=obs)
            if actual:
                all_replay.append({'dataset':ds,'run_id':rid,'members':run['members'],'estimate':est,'old':old,'new':new,'population_ids':[q['run_id'] for q in pop]})
                all_peak.append({'dataset':ds,'run_id':rid,'requested_slack':bounds['requested']-run['repeats_heap_peak'],'moving_slack':bounds['moving']-run['repeats_heap_peak_move']})
                suffix='b3/records' if ds=='K6B' else 'b/runs'
                raw=get(T3+'/IMPLEMENTATION/'+ds+'/_run_records/'+suffix+'/'+rid+'.jsonl')
                eq('stdout_hash',hashlib.sha256(raw).hexdigest(),run['stdout_sha256'])
                for obj in map(json.loads,raw.splitlines()):
                    for name,value in obj.items():
                        if 'heap' not in name or not isinstance(value,int):continue
                        metric='moving' if 'move' in name else 'requested'; bound=None
                        if name=='heap_cap_bytes':kind='configuration'
                        elif ds!='K6B':kind='matched';bound=bounds[metric]
                        elif obj.get('kind')=='stage':
                            kind='matched';stage=obj['stage'];bkey='source_' if stage=='w1_source' else 'solve_' if stage=='w1_solve' else 'prefix_';bound=e[bkey+metric]
                        elif obj.get('kind')=='summary' and name.startswith('repeats_'):kind='matched';bound=bounds[metric]
                        elif obj.get('kind')=='summary' and name.startswith('prefix_'):kind='matched';bound=e['prefix_'+metric]
                        else:kind='outside_H_window'
                        if bound is not None:ok('raw_nonnegative_slack',bound>=value)
                        raw_fields.append({'dataset':ds,'run_id':rid,'kind':obj.get('kind'),'stage':obj.get('stage'),'repeat':obj.get('repeat'),'field':name,'observed_bytes':value,'class':kind,'bound':bound,'slack':None if bound is None else bound-value})

# Verify all rows of the author's raw disposition, including every excluded field.
author=packet('RAW_METRIC_DISPOSITION.json');eq('raw_count',len(raw_fields),len(author))
for ours,theirs in zip(raw_fields,author):
    eq('raw_identity',{k:ours[k] for k in ['dataset','run_id','kind','stage','repeat','field','observed_bytes']},{k:theirs.get(k) for k in ['dataset','run_id','kind','stage','repeat','field','observed_bytes']})
    eq('raw_bound',ours['bound'],theirs['matching_bound_bytes'])
    if ours['bound'] is not None:eq('raw_slack',ours['slack'],theirs['slack_bytes'])

summary={'completed_at_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'check_counts':dict(checks),'failures':failures,'history_row_counts':{k:len(v) for k,v in histories.items()},'decisions':{ds:dict(collections.Counter(x['new']['decision'] for x in all_replay if x['dataset']==ds)) for ds in histories},'changes':sum(x['old']['decision']!=x['new']['decision'] for x in all_replay),'H_max_estimate':max(x['estimate'] for x in all_replay if x['dataset']=='K6B'),'minimum_summary_slack':{ds:{m:min(p[m+'_slack'] for p in all_peak if p['dataset']==ds) for m in ['requested','moving']} for ds in histories},'raw_field_classes':dict(collections.Counter(x['class'] for x in raw_fields))}
save('INDEPENDENT_AUDIT.json',summary);save('INDEPENDENT_DECISIONS.json',all_replay);save('INDEPENDENT_RAW_FIELDS.json',raw_fields)
print(json.dumps(summary,indent=2))
