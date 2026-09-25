from pathlib import Path
import hashlib,json,os,re,subprocess,sys,time,tomllib
R=Path('/private/tmp/piping-numerical-integrity-20260924'); P=R/'projects/chirality-piping'
N=P/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/SOLVER_MANAGER/NUMERICAL_INTEGRITY'
D=N/'SOLVER_FIRST_CHECKS/_run_records';D.mkdir(parents=True,exist_ok=True)
def h(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def snapshot(manifest):
 todo=[manifest.resolve()];seen=set();files=set()
 while todo:
  m=todo.pop()
  if m in seen:continue
  seen.add(m);files.add(m)
  lock=m.parent/'Cargo.lock'
  if lock.exists():files.add(lock)
  for name in ['src','tests','examples','benches']:
   base=m.parent/name
   if base.exists():files.update(base.rglob('*.rs'))
  if (m.parent/'build.rs').exists():files.add(m.parent/'build.rs')
  data=tomllib.loads(m.read_text())
  groups=[data.get(k,{}) for k in ['dependencies','dev-dependencies','build-dependencies']]
  for t in data.get('target',{}).values():groups.extend(t.get(k,{}) for k in ['dependencies','dev-dependencies','build-dependencies'])
  for group in groups:
   for dep in group.values():
    if isinstance(dep,dict) and 'path' in dep:todo.append((m.parent/dep['path']/'Cargo.toml').resolve())
 for f in list(files):
  if f.suffix!='.rs':continue
  for rel in re.findall(r'include_(?:str|bytes)!\(\s*"([^"]+)"\s*,?\s*\)',f.read_text()):
   target=(f.parent/rel).resolve()
   if target.exists():files.add(target)
 return {'coverage':'superset local dependency manifests/locks/src/tests/examples/benches/build.rs plus literal include_str/include_bytes targets','manifests':[str(m.relative_to(R)) for m in sorted(seen)],'hashes':{str(f.relative_to(R)):h(f) for f in sorted(files)}}
STAGES={
 'frame_exact_boundary':('core/solver/frame_kernel','/private/tmp/piping-frame-kernel-hybrid-target',['--lib','exact_boundary','--','--test-threads=2','--nocapture']),
 'nonlinear_compile':('core/solver/nonlinear_integration','/private/tmp/piping-nonlinear-integration-exact-gap-target',['--lib','--no-run']),
 'nonlinear_ret01':('core/solver/nonlinear_integration','/private/tmp/piping-nonlinear-integration-exact-gap-target',['--lib','ret01_long_id','--','--test-threads=2','--nocapture']),
 'nonlinear_retained':('core/solver/nonlinear_integration','/private/tmp/piping-nonlinear-integration-exact-gap-target',['--lib','actual_adapter_retains_adjacent_response','--','--test-threads=2','--nocapture']),
 'nonlinear_strict':('core/solver/nonlinear_integration','/private/tmp/piping-nonlinear-integration-exact-gap-target',['--lib','strict_gap_','--','--test-threads=2','--nocapture']),
 'nonlinear_coupled':('core/solver/nonlinear_integration','/private/tmp/piping-nonlinear-integration-exact-gap-target',['--lib','audit_coupled_exact_contacts','--','--test-threads=2','--nocapture']),
 'nonlinear_decimal':('core/solver/nonlinear_integration','/private/tmp/piping-nonlinear-integration-exact-gap-target',['--lib','audit_gap_equilibrium','--','--test-threads=2','--nocapture']),
 'nonlinear_affine':('core/solver/nonlinear_integration','/private/tmp/piping-nonlinear-integration-exact-gap-target',['--lib','current_normal_affine_fixture','--','--test-threads=2','--nocapture']),
 'product_serialization':('core/product_physics','/private/tmp/piping-product-rotational-gap-target',['--lib','source_capture::tests','--','--test-threads=2','--nocapture']),
 'product_publication_baseline':('core/product_physics','/private/tmp/piping-product-rotational-gap-target',['--test','represented_gap_publication','--','--test-threads=2','--nocapture']),
}
for stage in sys.argv[1:]:
 rel,target,args=STAGES[stage];m=P/rel/'Cargo.toml';before=snapshot(m)
 out=D/stage;out.mkdir(exist_ok=False)
 cmd=['cargo','test','--offline','--locked','-j2','--manifest-path',str(m.relative_to(R)),*args]
 env=os.environ.copy();env['CARGO_BUILD_JOBS']='2';env['CARGO_TARGET_DIR']=target
 (out/'BEFORE.json').write_text(json.dumps({'snapshot':before,'command':cmd,'cwd':str(R),'target':target,'started_unix':time.time()},indent=2)+'\n')
 with (out/'cargo.log').open('w') as log:p=subprocess.run(cmd,cwd=R,env=env,stdout=log,stderr=subprocess.STDOUT)
 after=snapshot(m);text=(out/'cargo.log').read_text();summary=re.findall(r'test result:.*',text)
 (out/'AFTER.json').write_text(json.dumps({'exit_code':p.returncode,'unchanged':after==before,'after':after,'summary':summary,'log_sha256':h(out/'cargo.log'),'ended_unix':time.time()},indent=2)+'\n')
 print(json.dumps({'stage':stage,'exit_code':p.returncode,'unchanged':after==before,'summary':summary}),flush=True)
 if p.returncode or after!=before:
  print(text[-8000:],flush=True);sys.exit(p.returncode or 1)
