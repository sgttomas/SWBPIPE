from pathlib import Path
import hashlib,json,os,re,subprocess,time,runpy,sys
r=Path('/private/tmp/piping-numerical-integrity-20260924');n=r/'projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/SOLVER_MANAGER/NUMERICAL_INTEGRITY';a=n/'PRODUCT_FULL_DIAGNOSIS/REPAIR_02';d=a/'_run_records/SCOPED_DELTA_EXECUTION';d.mkdir(parents=True,exist_ok=False)
old=sys.argv;sys.argv=['snapshot'];snapshot=runpy.run_path('/private/tmp/solver_ready_checks.py')['snapshot'];sys.argv=old
freeze=json.loads((a/'PROBE_FREEZE.json').read_text());h=lambda b:hashlib.sha256(b).hexdigest();original={};probes={}
for name,raw in [('product','product.scoped_delta_probe_candidate.rs'),('nonlinear','nonlinear.scoped_delta_probe_basis.rs')]:
 f=r/freeze[name]['target'];original[f]=f.read_bytes();assert h(original[f])==freeze[name]['restore_five_repairs_sha256']
 probes[f]=(a/'_run_records'/raw).read_bytes();assert h(probes[f])==freeze[name].get('probe_candidate_sha256',freeze[name].get('probe_basis_sha256'))
m=r/'projects/chirality-piping/core/product_physics/Cargo.toml';before=snapshot(m)
cmd=['cargo','test','--offline','--locked','-j2','--manifest-path',str(m.relative_to(r)),'--lib',freeze['test'],'--','--exact','--nocapture','--test-threads=1'];env=os.environ.copy();env['CARGO_BUILD_JOBS']='2';env['CARGO_TARGET_DIR']='/private/tmp/piping-product-rotational-gap-target'
for f,b in probes.items():f.write_bytes(b)
(d/'BEFORE.json').write_text(json.dumps({'original':before,'instrumented':snapshot(m),'command':cmd,'cwd':str(r),'target':env['CARGO_TARGET_DIR'],'started_unix':time.time()},indent=2)+'\n');result=None
try:
 with (d/'cargo.log').open('w') as log:result=subprocess.run(cmd,cwd=r,env=env,stdout=log,stderr=subprocess.STDOUT)
finally:
 unchanged={str(f.relative_to(r)):h(f.read_bytes())==h(probes[f]) for f in probes}
 for f,b in original.items():
  if unchanged[str(f.relative_to(r))]:f.write_bytes(b)
 after=snapshot(m)
 (d/'AFTER.json').write_text(json.dumps({'exit_code':None if result is None else result.returncode,'probe_files_unchanged':unchanged,'restored_five_repairs_source_and_locks':after==before,'after':after,'log_sha256':h((d/'cargo.log').read_bytes()),'ended_unix':time.time()},indent=2)+'\n')
text=(d/'cargo.log').read_text();rows=[]
for b in text.split('SCOPED_DELTA INPUT ')[1:]:
 header=b.splitlines()[0].split(' {',1)[0]
 it=[]
 for body in b.split('DIAGPROBE ITERATION ')[1:]:
  residual=body.split('residual_observations=',1)[1].split('ordinary_structural_report=',1)[0]
  fields={k:float(v) for k,v in re.findall(r'(max_abs_[a-z_]+): Some\(\s*([-+0-9.eE]+),?\s*\)',residual)}
  it.append({'iteration':int(re.search(r'index=(\d+)',body).group(1)),'deltas':fields,'contact_admissible':'contact_branches_admissible=true' in body.split('active_set=',1)[0]})
 out=b.split('SCOPED_DELTA OUTPUT ',1)[-1]
 rows.append({'input':header,'converged':'converged=true' in b,'iterations':it,'output_mechanics':re.search(r'mechanics: "([^"]+)"',out).group(1) if 'mechanics:' in out else None})
summary={'exit_code':result.returncode,'restored':after==before,'observations':rows}
(d/'SUMMARY.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary,indent=2))
if result.returncode:print(text[-3500:])
sys.exit(result.returncode or (0 if before==after else 1))
