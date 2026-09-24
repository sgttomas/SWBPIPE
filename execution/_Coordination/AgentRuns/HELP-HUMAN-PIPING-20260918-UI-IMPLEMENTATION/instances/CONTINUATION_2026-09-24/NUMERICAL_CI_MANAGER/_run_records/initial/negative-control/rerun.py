from pathlib import Path
import hashlib, importlib.util, json, os, shutil, subprocess, sys, tempfile
repo=Path('/private/tmp/piping-numerical-ci-20260924')
out=repo/'projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_CI_MANAGER/negative-control'
out.mkdir(parents=True,exist_ok=True)
fixture=Path(tempfile.mkdtemp(prefix='piping-numerical-expected-failure-'))
project=fixture/'projects/chirality-piping'
paths=['tools/ci/e2e_plan.py','tools/ci/numerical_ci.py','tools/release/check_release_readiness.py']
for path in paths:
 target=project/path;target.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(repo/'projects/chirality-piping'/path,target)
crate=project/'core/failure_control';(crate/'src').mkdir(parents=True)
inputs={'Cargo.toml':'[package]\nname = "piping_ci_failure_control"\nversion = "0.1.0"\nedition = "2021"\n','Cargo.lock':'version = 4\n\n[[package]]\nname = "piping_ci_failure_control"\nversion = "0.1.0"\n','src/lib.rs':'#[test]\nfn expected_negative_numerical_control() {\n    assert_eq!(2 + 2, 5, "intentional numerical failure control");\n}\n'}
for name,body in inputs.items(): (crate/name).write_text(body)
specfile=project/'apps/desktop/e2e/b3-accessibility.spec.ts';specfile.parent.mkdir(parents=True);specfile.write_text('// minimal inventory for runner identity control; no browser execution\n')
for args in (['init','-q'],['config','user.name','CI failure-control fixture'],['config','user.email','ci-fixture@example.invalid'],['add','.'],['commit','-qm','temporary expected failure fixture']): subprocess.run(['git','-C',str(fixture),*args],check=True)
spec=importlib.util.spec_from_file_location('failure_control_selector',project/'tools/ci/e2e_plan.py');selection=importlib.util.module_from_spec(spec);spec.loader.exec_module(selection)
plan=selection.make_plan(fixture,'workflow_dispatch');planpath=fixture/'plan.json';planpath.write_text(json.dumps(plan,indent=2)+'\n')
env=os.environ.copy()
for key in list(env):
 if key.startswith('GITHUB_'): env.pop(key)
env['RUSTUP_TOOLCHAIN']='1.97.1'
cmd=[sys.executable,str(project/'tools/ci/numerical_ci.py'),'--plan',str(planpath),'--evidence-dir',str(fixture/'evidence')]
result=subprocess.run(cmd,env=env,cwd=fixture,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
(out/'runner.stdout.log').write_text(result.stdout)
for p in (fixture/'evidence').rglob('*'):
 if p.is_file() and 'target' not in p.relative_to(fixture/'evidence').parts:
  target=out/'evidence'/p.relative_to(fixture/'evidence');target.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,target)
aggregate=[sys.executable,str(project/'tools/ci/e2e_plan.py'),'aggregate','--mode','full','--selection','success','--barrier','success','--remainder','success','--numerical-required','true','--numerical','failure']
gate=subprocess.run(aggregate,env=env,cwd=fixture,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
(out/'aggregate.stdout.log').write_text(gate.stdout)
summary={'kind':'expected-negative-control','not_product_or_hosted_execution':True,'fixture_root':str(fixture),'fixture_head':plan['head'],'candidate_base':'f7e8b467cb2db244f11fe49cede636140031b387','candidate_runner_sha256':{p:hashlib.sha256((repo/'projects/chirality-piping'/p).read_bytes()).hexdigest() for p in paths},'fixture_inputs':inputs,'runner_argv':cmd,'runner_exit':result.returncode,'aggregate_argv':aggregate,'aggregate_exit':gate.returncode,'aggregate_inputs_note':'Controlled job-status inputs isolate numerical failure; no browser run/pass is claimed.'}
(out/'control.json').write_text(json.dumps(summary,indent=2)+'\n'); print(json.dumps({'runner_exit':result.returncode,'aggregate_exit':gate.returncode,'evidence':str(out)},indent=2))
assert result.returncode == 101, 'real failing Cargo test must propagate through CLI'
assert gate.returncode == 1, 'aggregate must reject numerical failure'
evidence=json.loads((out/'evidence/numerical.json').read_text());assert evidence['status']=='failed' and evidence['plan_validated'] and evidence['exit_code']==101
assert evidence['commands'][-1]['exit_code']==101 and evidence['commands'][-1]['argv'][:2]==['cargo','test']
