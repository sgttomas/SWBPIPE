from review_io import *
from collections import Counter
import datetime
counts=Counter(); details=[]
for ds,suf in [('K6B','b3/records'),('VK','b/runs'),('KF3','b/runs')]:
    base=T3+'/IMPLEMENTATION/'+ds+'/_run_records/'+suf
    metadata=jget(base+'/metadata.json')
    for r in rows(base+'/records.jsonl'):
        if r.get('mode')!='w1a':continue
        obs=[json.loads(b) for b in get(base+'/'+r['run_id']+'.jsonl').splitlines()]
        summary=[x for x in obs if x.get('kind')=='summary']; assert len(summary)==1
        assert summary[0]==r['summary'];counts['summary_dictionary']+=1
        for metric in ['heap_peak','heap_peak_move']:
            assert summary[0]['repeats_'+metric]==r['repeats_'+metric];counts['runner_summary_peak']+=1
            if ds=='K6B':
                stages=[x for x in obs if x.get('kind')=='stage']
                inner=[x[metric] for x in stages if x['stage'] in ['w1_source','w1_solve']]
                assert max(inner)==r['repeats_'+metric];counts['inner_maximum']+=1
                for name,s in r['stages'].items():
                    actual=max(x[metric] for x in stages if x['stage']==name)
                    assert actual==s[metric];counts['stage_aggregate_maximum']+=1
                # Outer prefix edge includes stage-line/error owners; it need not
                # equal the inner observer prefix-stage maxima.
                ps=[x[metric] for x in stages if x['stage'].startswith('w1_prefix_')]
                if ps:
                    assert summary[0]['prefix_phase_'+metric]>=max(ps);counts['prefix_outer_contains_inner']+=1
            else:
                assert summary[0]['repeats_'+metric]>=max(x[metric] for x in obs if metric in x);counts['VR_global_contains_phases']+=1
        assert r['source_commit']==metadata['source_commit'];assert r['source_tree']==metadata['source_tree'];counts['dataset_source_identity']+=1
        details.append({'dataset':ds,'run_id':r['run_id'],'binary_sha256_from_dataset_metadata':metadata['binary_sha256'],'classification':r['classification'],'outcome':r.get('outcome'),'source_commit':r['source_commit']})
save('RAW_SUMMARY_CHECK.json',{'counts':dict(counts),'failures':[],'identity_rows':details,'completed_at_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()});print(json.dumps(dict(counts),indent=2))
