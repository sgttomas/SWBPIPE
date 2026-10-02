from inspect import *
for d in ['b','b3']:
 p=T3/f'IMPLEMENTATION/K6B/_run_records/{d}/records/records.jsonl'
 if p.exists():
  rows=[json.loads(x) for x in read(p).splitlines()];print(d,len(rows));print(json.dumps(rows[0],indent=1)[:10000])
for ds in ['VK','KF3']:
 p=T3/f'IMPLEMENTATION/{ds}/_run_records/b/runs/metadata.json'; print('metadata',ds,read(p)[:5000])
