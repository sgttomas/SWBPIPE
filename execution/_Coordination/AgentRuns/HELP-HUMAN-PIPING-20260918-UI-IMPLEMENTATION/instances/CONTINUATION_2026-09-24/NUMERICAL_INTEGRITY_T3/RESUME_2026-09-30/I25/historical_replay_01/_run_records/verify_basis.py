from inspect import *
import datetime,re
packets=['I21/h_numeric_19','I21/kernel_reference_22','metric_design_14_reference_callers','metric_design_15_result5_join','source_review_RV30/h_numeric_23','source_review_RV30/kernel_reference_27','design_review_RV28/reference_callers_10','source_review_RV30/result5_join_28','I21/h_integration_25','I24/vr_integration_01']
seals=[]
for rel in packets:
 p=R/rel;manifest=read(p/'SHA256SUMS');checks=[]
 for line in manifest.splitlines():
  if not line.strip():continue
  wanted,name=line.split(None,1);name=name.lstrip('*');q=p/name
  if not q.exists():q=pathlib.Path(name)
  b=q.read_bytes();actual=hashlib.sha256(b).hexdigest();checks.append({'path':str(q),'sha256':actual,'expected_sha256':wanted,'ok':actual==wanted})
 assert all(x['ok']for x in checks),(rel,[x for x in checks if not x['ok']])
 seals.append({'packet':str(p),'seal':hashlib.sha256(manifest.encode()).hexdigest(),'payloads':len(checks),'checks':checks})
 for n in ['RETURN.md','REVIEW.md']:
  if (p/n).exists():read(p/n)
# Preserve all previously viewed key textual bases in the hash inventory.
for rel in ['I21/h_numeric_19/FORMULAS.md','I21/h_numeric_19/LAUNCH_BINDING.json','I21/k0_assembly_16/DESIGN_AND_PLAN.md','metric_design_14_reference_callers/METHOD_AND_JOIN.md','metric_design_15_result5_join/METHOD.md']:
 read(R/rel)
orig=js(R/'I21/k0_assembly_16/HISTORICAL_COMPARISON.json'); historical=[]
for x in orig['inputs']:
 b=read(x['path']).encode();h=hashlib.sha256(b).hexdigest();assert h==x['sha256'];historical.append({'path':x['path'],'sha256':h})
stdout=[];numeric_fields=[]
for dataset,dirname in [('K6B','K6B/_run_records/b3/records'),('VK','VK/_run_records/b/runs'),('KF3','KF3/_run_records/b/runs')]:
 base=T3/'IMPLEMENTATION'/dirname;rows=[json.loads(t)for t in read(base/'records.jsonl').splitlines()]
 for r in rows:
  if r.get('mode')!='w1a':continue
  b=read(base/(r['run_id']+'.jsonl')).encode();assert hashlib.sha256(b).hexdigest()==r['stdout_sha256']
  stdout.append({'dataset':dataset,'run_id':r['run_id'],'stdout_sha256':r['stdout_sha256']})
  obs=[json.loads(t)for t in b.decode().splitlines() if t.strip()]
  for o in obs:
   h={k:v for k,v in o.items()if 'heap' in k and isinstance(v,int)}
   if h:numeric_fields.append({'dataset':dataset,'run_id':r['run_id'],'kind':o.get('kind'),'stage':o.get('stage'),'repeat':o.get('repeat'),'heaps':h})
save('SEALED_BASIS_CHECK.json',seals);save('HISTORICAL_INPUT_HASH_CHECK.json',historical);save('ORIGINAL_STDOUT_HASH_CHECK.json',stdout);save('HISTORICAL_RAW_HEAP_FIELDS.json',numeric_fields)
print(json.dumps({'seals':len(seals),'payloads':sum(x['payloads']for x in seals),'historical_input_hashes':len(historical),'stdout_hashes':len(stdout),'raw_heap_objects':len(numeric_fields),'kinds':sorted(set(str(x['kind'])for x in numeric_fields)),'example':numeric_fields[:8]},indent=2))
