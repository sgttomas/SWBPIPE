from inspect import *
for p in [T3/'IMPLEMENTATION/KF3/_run_records/b/runs/records.jsonl',T3/'IMPLEMENTATION/VK/_run_records/b/runs/records.jsonl']:
 rows=[json.loads(x) for x in read(p).splitlines()];print('\n',p,'LEN',len(rows));print(json.dumps(rows[:2],indent=1)[:19000])
for d in ['I21/h_numeric_19','I21/kernel_reference_22','metric_design_14_reference_callers','metric_design_15_result5_join']:
 for p in (R/d).glob('RETURN.md'):read(p)
