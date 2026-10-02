from pathlib import Path
import importlib.util,json,io,contextlib,hashlib
out=Path(__file__).parent;a=Path(json.loads((out/'ARCHIVE.json').read_text())['archive']);h='projects/chirality-piping/core/solver/performance_harness/runner/k6_runner.py';v='projects/chirality-piping/validation/benchmarks/numerical_robustness/runner/vk_scale_runner.py'
def load(path,name):
 spec=importlib.util.spec_from_file_location(name,path);m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m);return m
old=load(out/'base'/h,'old_h');new=load(a/h,'new_h');oldv=load(out/'base'/v,'old_v');newv=load(a/v,'new_v')
counts={'H':0,'VR':0};mismatches=[]
for run in new.schedule():
 previous=new.previous_size(run['model']);samples=[[]]
 if previous:
  for status in [None,'not_run','ok','error','timed_out','heap_cap_abort']:
   samples.append([dict(run,model=previous,run_id='prior',members=new.members_of(previous),classification=status,estimate_adm_bytes=1000,peak_rss_bytes=1200,rss={'time_peak_footprint_bytes':1100})])
 for measured in samples:
  for estimate in [None,'missing',321,run['heap_cap_bytes']//2,run['heap_cap_bytes']//2+1,run['rss_cap_bytes']*2]:
   c={} if estimate is None else {run['model']:{} if estimate=='missing' else {new.estimate_key(run['mode']):estimate}}
   for ascent in [False,True]:
    x=old.admission(run,c,measured,50,require_ascent=ascent,baseline_footprint_bytes=40);y=new.admission(run,c,measured,50,require_ascent=ascent,baseline_footprint_bytes=40);counts['H']+=1
    if x!=y:mismatches.append({'run':run['run_id'],'old':x,'new':y})
for run in newv.schedule():
 previous=new.previous_size(run['model']);samples=[[]]+[[dict(run,model=previous,run_id='prior',members=new.members_of(previous),classification=status,estimate_adm_bytes=1000,peak_rss_bytes=1200,rss={'time_peak_footprint_bytes':1100})] for status in [None,'not_run','ok','error','timed_out','heap_cap_abort']]
 for measured in samples:
  for estimate in [None,'missing',321,run['heap_cap_bytes']//2,run['heap_cap_bytes']//2+1,run['rss_cap_bytes']*2]:
   c={} if estimate is None else {run['model']:{} if estimate=='missing' else {'estimate_max_bytes':estimate,new.estimate_key(run['mode']):estimate}}
   for approve in [False,True]:
    x=oldv.admission(run,c,measured,50,40,approve);y=newv.admission(run,c,measured,50,40,approve);counts['VR']+=1
    if x!=y:mismatches.append({'run':run['run_id'],'old':x,'new':y})
plans=[]
with contextlib.redirect_stdout(io.StringIO()):
 for path in [None,str(a/'projects/chirality-piping/core/solver/performance_harness/observations/k6b/counts.jsonl')]:
  x=old.plan(path);y=new.plan(path);plans.append({'runner':'H','counts':path,'equal':x==y,'rows':len(x)});assert x==y
 for approve in [False,True]:
  x=oldv.plan(None,None,approve);y=newv.plan(None,None,approve);plans.append({'runner':'VR','approve_10000':approve,'equal':x==y,'rows':len(x)});assert x==y
result={'pure_admission_comparisons':counts,'mismatches':mismatches,'plan_checks':plans,'method':'Independent bounded comparisons over all schedules, missing/absent/small/half-cap/over-cap estimates, require-ascent and approval settings, and absent/None/not_run/ok/error/timeout/abort prior attempts; no model-specific subprocess, product count/solver, or Cargo execution; schedule helpers retain their existing Python model metadata construction.'}
(out/'PURE_COMPARISON.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'counts':counts,'mismatches':len(mismatches),'plans':len(plans)}));assert not mismatches
