import pathlib,json,hashlib,subprocess,os,sys
ROOT=pathlib.Path.cwd()
T3=pathlib.Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3')
R=T3/'RESUME_2026-09-30'; OUT=R/'I25/historical_replay_01'; REC=OUT/'_run_records'
H=pathlib.Path('projects/chirality-piping/core/solver/performance_harness'); VR=pathlib.Path('projects/chirality-piping/validation/benchmarks/numerical_robustness')
def read(p):
 p=pathlib.Path(p); b=p.read_bytes(); f=REC/'READS.jsonl'
 with f.open('a') as h:h.write(json.dumps({'path':str(p),'sha256':hashlib.sha256(b).hexdigest(),'bytes':len(b)})+'\n')
 return b.decode()
def js(p): return json.loads(read(p))
def save(n,x): (REC/n).write_text(json.dumps(x,indent=2)+'\n')
def show(p):
 x=js(p); print('\n',str(p)); print(type(x).__name__, list(x)[:30] if isinstance(x,dict) else len(x));
 if isinstance(x,dict):
  for k,v in x.items(): print(k,('len='+str(len(v))+' '+str(v[:1])[:1700]) if isinstance(v,list) else str(v)[:300])
 elif isinstance(x,list):print(str(x[:1])[:2200])
if __name__=='__main__':
 for p in ['AGENTS.md','agents/AGENT_TASK.md','projects/chirality-piping/AGENTS.md',R/'BRIEFS/COMMON.md',T3/'TASK_BRIEFS/I21_K6C_IMPLEMENTATION.md',R/'I21/k0_assembly_16/DESIGN_AND_PLAN.md']:read(p)
 for p in ['I21/k0_assembly_16/HISTORICAL_COMPARISON.json','I21/h_numeric_19/HISTORICAL_COMPARISON.json','I21/h_numeric_19/PHASES.json','metric_design_15_result5_join/RESULT5_CLI24.json','metric_design_14_reference_callers/CLI24_LAUNCHES.json','metric_design_14_reference_callers/CLI24_CALLERS.json']:show(R/p)
