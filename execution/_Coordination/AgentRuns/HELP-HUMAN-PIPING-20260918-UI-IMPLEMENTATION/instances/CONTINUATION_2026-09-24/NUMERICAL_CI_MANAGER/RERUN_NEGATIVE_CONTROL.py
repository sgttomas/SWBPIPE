#!/usr/bin/env python3
"""Portable recovery entry for the expected numerical failure control.

Full execution requires the coordinated Cargo CPU slot. --check-paths only
validates metadata; it never creates output/fixtures or launches Cargo.
Output must be a fresh _run_records/reruns/<name> directory inside the selected
repository. Git metadata and existing or initial historical records are excluded.
"""
from pathlib import Path
import argparse
import hashlib
import importlib.util
import json
import os
import shutil
import subprocess
import sys
import tempfile

PATHS = ['tools/ci/e2e_plan.py', 'tools/ci/numerical_ci.py',
         'tools/release/check_release_readiness.py']


def resolve_paths(args):
    selected = Path(args.repo_root).resolve() if args.repo_root else Path(
        subprocess.check_output(['git', 'rev-parse', '--show-toplevel'], text=True).strip()).resolve()
    actual = Path(subprocess.check_output(
        ['git', '-C', str(selected), 'rev-parse', '--show-toplevel'], text=True).strip()).resolve()
    if selected != actual:
        raise ValueError('--repo-root must identify the repository root')
    requested = Path(args.output_dir)
    output = (requested if requested.is_absolute() else selected / requested).resolve()
    if not output.is_relative_to(selected) or output == selected:
        raise ValueError('--output-dir must be contained inside --repo-root')
    parts = output.relative_to(selected).parts
    if '.git' in parts:
        raise ValueError('Output may not enter Git metadata')
    if len(parts) < 3 or parts[-3:-1] != ('_run_records', 'reruns') or parts.count('_run_records') != 1:
        raise ValueError('Output must be a fresh _run_records/reruns/<name> directory')
    if output.exists() or (requested if requested.is_absolute() else selected / requested).is_symlink():
        raise ValueError('--output-dir must be fresh and nonexistent')
    existing_parent = next(parent for parent in output.parents if parent.exists())
    if not existing_parent.is_dir():
        raise ValueError('Output ancestor must be a directory')
    hashes = {}
    for relative in PATHS:
        helper = selected / 'projects/chirality-piping' / relative
        if not helper.is_file() or not helper.resolve().is_relative_to(selected):
            raise ValueError('Missing or uncontained helper: ' + relative)
        hashes[relative] = hashlib.sha256(helper.read_bytes()).hexdigest()
    metadata = dict(repo_root=str(selected), output_dir=str(output),
                    candidate_head=subprocess.check_output(
                        ['git', '-C', str(selected), 'rev-parse', 'HEAD'], text=True).strip(),
                    helper_sha256=hashes)
    return selected, output, metadata


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--repo-root', help='Repository root; defaults to git rev-parse --show-toplevel')
    parser.add_argument('--output-dir', required=True, help='Fresh repository-contained _run_records/reruns/<name> directory (relative to repo root)')
    parser.add_argument('--check-paths', action='store_true', help='Check paths and print identity only; no Cargo or writes')
    args = parser.parse_args()
    repo, out, metadata = resolve_paths(args)
    if args.check_paths:
        print(json.dumps(metadata, indent=2))
        return 0
    out.mkdir(parents=True, exist_ok=False)
    paths = PATHS
    fixture=Path(tempfile.mkdtemp(prefix='piping-numerical-expected-failure-'))
    project=fixture/'projects/chirality-piping'
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
    env['CARGO_BUILD_JOBS']='1'
    cmd=[sys.executable,str(project/'tools/ci/numerical_ci.py'),'--plan',str(planpath),'--evidence-dir',str(fixture/'evidence')]
    result=subprocess.run(cmd,env=env,cwd=fixture,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
    (out/'runner.stdout.log').write_text(result.stdout)
    for p in (fixture/'evidence').rglob('*'):
     if p.is_file() and 'target' not in p.relative_to(fixture/'evidence').parts:
      target=out/'evidence'/p.relative_to(fixture/'evidence');target.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,target)
    aggregate=[sys.executable,str(project/'tools/ci/e2e_plan.py'),'aggregate','--mode','full','--selection','success','--barrier','success','--remainder','success','--numerical-required','true','--numerical','failure']
    gate=subprocess.run(aggregate,env=env,cwd=fixture,text=True,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
    (out/'aggregate.stdout.log').write_text(gate.stdout)
    summary={'kind':'expected-negative-control','not_product_or_hosted_execution':True,'fixture_root':str(fixture),'fixture_head':plan['head'],'candidate_head':metadata['candidate_head'],'candidate_runner_sha256':metadata['helper_sha256'],'fixture_inputs':inputs,'runner_argv':cmd,'runner_exit':result.returncode,'aggregate_argv':aggregate,'aggregate_exit':gate.returncode,'aggregate_inputs_note':'Controlled job-status inputs isolate numerical failure; no browser run/pass is claimed.'}
    (out/'control.json').write_text(json.dumps(summary,indent=2)+'\n'); print(json.dumps({'runner_exit':result.returncode,'aggregate_exit':gate.returncode,'evidence':str(out)},indent=2))
    assert result.returncode == 101, 'real failing Cargo test must propagate through CLI'
    assert gate.returncode == 1, 'aggregate must reject numerical failure'
    evidence=json.loads((out/'evidence/numerical.json').read_text());assert evidence['status']=='failed' and evidence['plan_validated'] and evidence['exit_code']==101
    assert evidence['commands'][-1]['exit_code']==101 and evidence['commands'][-1]['argv'][:2]==['cargo','test']
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
