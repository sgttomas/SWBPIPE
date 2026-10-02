from review_io import *
import collections,datetime,re,stat
counts=collections.Counter();fail=[]
def eq(name,a,b):
    counts[name]+=1
    if a!=b:fail.append({'check':name,'actual':a,'expected':b})
b=packet('INPUT_BINDING.json');source=js(R+'/I28/ordinary_artifacts_02/_run_records/SOURCE_AFTER.json');root=pathlib.Path(b['source_root'])
eq('source_exact_set',sorted(str(p.relative_to(root)) for p in root.rglob('*') if p.is_file()),sorted(x['relative'] for x in source))
eq('source_no_symlinks',sum(p.is_symlink() for p in root.rglob('*')),0)
for x in source:
    p=pathlib.Path(x['path']);bb=p.read_bytes();h=hashlib.sha256(bb).hexdigest()
    eq('source_current_sha',h,x['sha256']);eq('source_frozen_git_sha',hashlib.sha256(get(x['relative'],SOURCE)).hexdigest(),x['sha256']);eq('source_bytes',len(bb),x['bytes']);eq('source_mode',oct(stat.S_IMODE(p.stat().st_mode)),x['mode'])
for key in ['binary','seed']:
    item=b[key];data=pathlib.Path(item['path']).read_bytes();eq(key+'_current_hash',hashlib.sha256(data).hexdigest(),item['sha256'])
for x in b['preexisting_measurement_files']:eq('preexisting_hash',hashlib.sha256(pathlib.Path(x['path']).read_bytes()).hexdigest(),x['sha256'])
eq('no_preexisting_tier_journal',any('/records/' in x['path'] for x in b['preexisting_measurement_files']),False)
for x in b['references']:
    path=x['path']
    if '/source/projects/' in path:bb=get('projects/'+path.split('/source/projects/',1)[1],SOURCE)
    elif '/numerics/' in path:bb=get(path.split('/numerics/',1)[1],NUM)
    else:bb=get(path.split('/k6c/',1)[1])
    eq('reference_hash',hashlib.sha256(bb).hexdigest(),x['sha256'])

a=archive();seed={x['model']:x for x in map(json.loads,get(H+'/observations/k6b/counts.jsonl',SOURCE).splitlines())};journal=[json.loads(x) for x in a['records.jsonl'].splitlines()]
pubpairs=collections.defaultdict(list);poll=collections.Counter();parityitems=collections.Counter();minrss=None
for r in journal:
    rid=r['run_id'];oo=[json.loads(x) for x in a[rid+'.jsonl'].splitlines()];co=next(x for x in oo if x['kind']=='counts');out=[x for x in oo if x['kind']=='outcome'];ss=seed[r['model']]
    if r['mode']=='sparse':eq('sparse_admission_seed',r['estimate_adm_bytes'],ss['estimate_adm_bytes_sparse'])
    for x in oo:
        if x['kind']=='parity':
            parityitems[x['item']]+=1
            if x['item']=='w1_source_encoding_equals_evidence':eq('source_digest_independent_join',x['evidence_source_fnv64'],ss['w1_source_encoding_fnv64'])
            if x['item']=='rcm_count_equals_ordering':
                eq('sparse_rcm_profile_join',x['ordering_profile_entries'],ss['rcm_profile_entries']);eq('sparse_rcm_bandwidth_join',x['ordering_half_bandwidth'],ss['rcm_half_bandwidth'])
        if x['kind']=='attempt':
            own=sum(v for k,v in x.items() if k.startswith('own_') and k not in ['own_total','own_unstaged'])
            shared=sum(v for k,v in x.items() if k.startswith('shared_') and k not in ['shared_work','shared_built_here','shared_unstaged'])
            eq('own_stage_sum',own,x['own_total']);eq('shared_stage_sum',shared,x['shared_work']+x['verification_shared_work'])
    if r['mode']=='w1a':
        for repeat in range(5):
            att=[x for x in oo if x['kind']=='attempt' and x['repeat']==repeat];ot=out[repeat]
            for precision in [128,256]:
                aa=[x for x in att if x['precision']==precision];eq('precision_attempt_join',len(aa),ot['work_'+str(precision)+'_attempts']);eq('precision_own_join',sum(x['own_total'] for x in aa),ot['work_'+str(precision)+'_own']);eq('precision_shared_join',sum(x['shared_work']+x['verification_shared_work'] for x in aa),ot['work_'+str(precision)+'_shared'])
        if r['pass']==1:
            lines=a[rid+'.rows'].decode().splitlines();eq('publication_dump_header',lines[:2],['k6b-rows v1','model '+r['model']]);data=[x.split() for x in lines[2:]];eq('dump_rowcount',len(data),out[0]['rows']);eq('dump_unique_quantity_ids',len({x[0] for x in data}),len(data))
            for cl in ['relative_verified','absolute_verified','input_derived','unpublishable']:eq('dump_classes',sum(x[3]==cl for x in data),out[0]['rows_'+cl])
        pubpairs[r['model']].append({k:v for k,v in out[0].items() if k!='repeat'})
    else:
        vals=a[rid+'.u'].decode().splitlines();eq('sparse_solution_count',len(vals),ss['dofs']);eq('sparse_solution_bitwords',all(re.fullmatch('[0-9a-f]{16}',v) for v in vals),True)
    eq('quiet_host_levels',r['quiet_host']['memorystatus_level']>=30,True)
for mid,outs in pubpairs.items():eq('two_pass_W1_outcomes',outs[0],outs[1])
records=[json.loads(bb) for name,bb in a.items() if name.endswith('.record.json')]
for r in records:poll[(r['binary_pid_found'],r['watchdog_polls'])]+=1
timeline=packet('TIMELINE_AND_DEVIATION.json');eq('first_check_after_driver_last_write',timeline['first_model_observation_utc']>timeline['driver_log_last_write_utc'],True)
driver=a['tier_W1-T1.driver.log'].decode().splitlines();eq('final_stop_arrays',json.loads(driver[-1]),{'stop':[],'stop_after_tier':[]});eq('no_retry_or_already_recorded',any('already recorded' in l or 'STOP:' in l for l in driver),False)
save('PRESERVATION_AND_SEMANTICS.json',{'check_counts':dict(counts),'failures':fail,'parity_items':dict(parityitems),'watchdog_poll_distribution':[{'pid_found':k[0],'polls':k[1],'records':v} for k,v in sorted(poll.items())],'normal_success_survivor_array_limit':'SOURCE runner/k6_runner.py:1057 writes [] without invoking survivors_of on normal success. All91 successful records take this path. Wrapper wait4 reaping and the reported empty first/final owned-command snapshots are separate evidence. No fresh process scan was run by RV34.','monitoring_requirement_fulfilled':False,'timing_kind':'Preserved filesystem times, not independently sampled live product endpoints.','completed_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()})
print(json.dumps({'check_counts':dict(counts),'failures':fail,'monitoring_requirement_fulfilled':False},indent=2))
