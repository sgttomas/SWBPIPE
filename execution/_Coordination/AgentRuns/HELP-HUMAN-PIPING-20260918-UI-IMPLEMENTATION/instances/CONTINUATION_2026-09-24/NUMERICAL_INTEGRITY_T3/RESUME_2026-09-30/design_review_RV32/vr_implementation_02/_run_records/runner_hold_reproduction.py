import sys,json
from pathlib import Path
from unittest.mock import patch
out=Path(__file__).resolve().parent
vr=out/'clean/projects/chirality-piping/validation/benchmarks/numerical_robustness'
sys.dont_write_bytecode=True
sys.path.insert(0,str(vr/'runner'))
import vk_scale_runner as v
results=[]
for tier,approved in [('V3',False),('V2',False)]:
    calls=[]; entries=[]; count_rows={}
    def launch(argv,**kw):
        calls.append({'argv':argv,'run_id':kw['run_id']})
        if '--case' in argv:
            count_rows[kw['run_id']]={'kind':'counts','case':argv[argv.index('--case')+1],'estimate_max_bytes':1000000}
        return {'classification':'ok','peak_rss_bytes':0,'rss':{}}
    def objects(directory,run_id):return [count_rows[run_id]]
    patches=[patch.object(v.k6,'launch',launch),patch.object(v.k6,'metadata',lambda *a:{}),patch.object(v.k6,'read_counts',lambda *a:{}),patch.object(v.k6,'read_records',lambda *a:[]),patch.object(v.k6,'append_record',lambda directory,row:entries.append(row)),patch.object(v,'objects_of',objects)]
    for p in patches:p.start()
    try:
        result=v.run_tier(tier,'never-executed-binary','models','counts',str(out/('stub_'+tier)),'candidate','tree',approve_10000=approved,log=lambda *a:None)
    finally:
        for p in reversed(patches):p.stop()
    bind=[c for c in calls if c['run_id'].endswith('.counts-binding')]
    results.append({'tier':tier,'approve_10000':approved,'all_launch_calls_stubbed':True,'counts_launches_before_defer':len(bind),'count_caps':[c['argv'][c['argv'].index('--heap-cap-bytes')+1] for c in bind],'decisions':[x['admission'] for x in entries],'normal_launches':len([c for c in calls if '--case' in c['argv'] and '--counts-only' not in c['argv']]),'result':result})
assert all(x['counts_launches_before_defer']==6 for x in results)
assert all(x['normal_launches']==0 for x in results)
assert all('10000_members_needs_root_approval' in x['reason'] for x in results[0]['decisions'])
assert all('ascent_previous_size_not_recorded' in x['reason'] for x in results[1]['decisions'])
print(json.dumps(results,indent=2))
