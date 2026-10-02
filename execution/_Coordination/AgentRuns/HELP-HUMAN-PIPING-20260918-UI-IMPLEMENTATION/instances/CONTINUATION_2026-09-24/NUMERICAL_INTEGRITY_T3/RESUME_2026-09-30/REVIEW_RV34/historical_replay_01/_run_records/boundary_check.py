"""Pure metadata boundary checks against functions extracted from frozen Git bytes.
No runner/module import, subprocess launch, model construction, or solver call.
"""
from review_io import *
import ast,re,types,copy,math,datetime
s=get('projects/chirality-piping/core/solver/performance_harness/runner/k6_runner.py','9086964a1fb656a76cda6d1002d8594efa636fdc').decode()
names={'members_of','previous_size','run_entries','refusal_by_name','estimate_key','measured_ratio','peak_footprint','measured_footprint_ratio','rss_to_footprint','admission'}
const={'RF_LARGE_SIZES','Q5_GRID_SIDES','N2_MODES','N2_REFUSAL_MEMBERS','RHO_DEFAULT','RSS_TO_FOOTPRINT_DEFAULT','PROJECTED_RSS_FRACTION','RHO_MIN_MEMBERS_FOR_LARGE','LARGE_MEMBERS','P1_LINUX_PEAK_MIB'}
selected=[n for n in ast.parse(s).body if (isinstance(n,ast.FunctionDef) and n.name in names) or (isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id in const for t in n.targets))]
namespace={'re':re,'MIB':1024**2};exec(compile(ast.Module(body=selected,type_ignores=[]),'frozen pure K6 admission','exec'),namespace)
k6=types.SimpleNamespace(**namespace)
v=get('projects/chirality-piping/validation/benchmarks/numerical_robustness/runner/vk_scale_runner.py','9086964a1fb656a76cda6d1002d8594efa636fdc').decode();vns={'k6':k6}
exec(compile(ast.Module(body=[n for n in ast.parse(v).body if isinstance(n,ast.FunctionDef) and n.name=='admission'],type_ignores=[]),'frozen pure VR admission','exec'),vns)
results=[]
base={'model':'RF-LARGE-CHAIN-n01000-AX','mode':'w1a','members':1000,'family':'CHAIN','rss_cap_bytes':8589934592,'heap_cap_bytes':8053063680,'conditional':False}
prior={'model':'RF-LARGE-CHAIN-n00100-AX','mode':'w1a','members':100,'family':'CHAIN','classification':'ok','order':1,'run_id':'synthetic_pure_metadata','repeats_heap_peak_move':0}
def trial(name,estimate,footprint,rss,denominator,want,vr=False,records=None,baseline_rss=0,baseline_footprint=0):
    p=dict(prior,estimate_adm_bytes=denominator,peak_rss_bytes=rss,rss={'time_peak_footprint_bytes':footprint})
    ps=[p] if records is None else records
    counts={base['model']:{'estimate_adm_bytes_w1a':estimate,'estimate_max_bytes':estimate}}
    out=vns['admission'](base,counts,ps,baseline_rss,baseline_footprint,True) if vr else k6.admission(base,counts,ps,baseline_rss,baseline_footprint_bytes=baseline_footprint)
    assert out['decision']==want,(name,out)
    results.append({'name':name,'estimate':estimate,'prior_footprint':footprint,'prior_rss':rss,'prior_denominator':denominator,'decision':out,'expected_decision':want})
    return out

limit=int(.8*base['rss_cap_bytes']);half=base['rss_cap_bytes']//2;cap=base['heap_cap_bytes']//2
# At exactly the binary half cap the strict > predicate permits the row.
trial('H binary half exact',cap,1,1,4,'admitted')
trial('H binary half plus one',cap+1,1,1,4,'deferred')
o=trial('VR binary half plus one returns before rho/ascent',cap+1,1,1,4,'deferred',True,records=[])
assert set(o)=={'decision','reason'}
# Footprint floating product is checked before int conversion.
trial('footprint exact half',2,half,half,2,'admitted')
trial('footprint half plus one half',1,2*half+1,2*half+1,2,'deferred')
# Real evaluated RSS product sits above the integer limit by 0.5, but after
# the required conversion it equals the limit and is admitted.
o=trial('final RSS conversion changes boundary',3_000_000_000,6_000_000_000,2*limit+1,6_000_000_000,'admitted')
assert o['footprint_estimate_bytes']*o['rss_to_footprint']>limit and o['projected_rss_bytes']==limit
o=trial('RSS integer limit plus one',3_000_000_000,6_000_000_000,2*limit+2,6_000_000_000,'deferred')
assert o['projected_rss_bytes']==limit+1
# First conversion also truncates a positive fractional product.
o=trial('first footprint conversion',3,3,3,2,'admitted');assert o['footprint_estimate_bytes']==4
# When footprint is absent, the source intentionally passes current footprint
# baseline into its RSS fallback (not current RSS baseline).
o=trial('missing footprint baseline parameter preservation',10,None,100,100,'admitted',baseline_rss=20,baseline_footprint=30)
assert o['rho']==.7 and o['rho_rss']==.8
# Recorded ascent requires classification present/not not_run, unlike ratio eligibility.
ascent=dict(prior,classification='source_refused',estimate_adm_bytes=10,peak_rss_bytes=10,rss={'time_peak_footprint_bytes':10})
o=trial('ascent recorded non-ok does not calibrate',10,1,1,1,'admitted',records=[ascent]);assert o['rho']==2
# Ratios pool orientation but reject the below100 rows for a1000 target.
small=dict(prior,members=10,model='RF-LARGE-CHAIN-n00010-AX',estimate_adm_bytes=1,peak_rss_bytes=1_000_000,rss={'time_peak_footprint_bytes':1_000_000})
rot=dict(prior,model='RF-LARGE-CHAIN-n00100-ROT',estimate_adm_bytes=100,peak_rss_bytes=30,rss={'time_peak_footprint_bytes':20})
o=trial('orientation pooled and large floor100',10,1,1,1,'admitted',records=[ascent,small,rot]);assert o['rho']==.2 and o['rss_to_footprint']==1.5

table=packet('ACTUAL_H_REPLAY.json');real=[]
for x in table:
    if x['dataset']=='K6B' and x['members']==10000:
        n=x['new_as_recorded_history']; e=x['new_estimate_bytes'];real.append({'run_id':x['run_id'],'estimate':e,'calibrated_rho':n['rho'],'calibrated_product':e*n['rho'],'footprint_half_cap':n['half_cap_bytes'],'projected_rss':n['projected_rss_bytes'],'default_rho2_would_fail':e*2>n['half_cap_bytes'],'actual_decision':n['decision']})
assert len(real)==12 and all(x['default_rho2_would_fail'] and x['actual_decision']=='admitted' for x in real)
save('BOUNDARY_CHECK.json',{'pure_metadata_cases':results,'H10000_calibration_sensitivity':real,'complete_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'failure_count':0})
print(json.dumps({'pure_metadata_cases':len(results),'H10000_rows':len(real),'all_passed':True},indent=2))
