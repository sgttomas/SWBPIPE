#!/usr/bin/env python3
"""Execute the release-readiness cargo profile with candidate-bound evidence."""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys


def load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


HERE = Path(__file__).resolve().parent
readiness = load_module('numerical_release_readiness', HERE.parent / 'release/check_release_readiness.py')
selection = load_module('numerical_selection', HERE / 'e2e_plan.py')


def cargo_plan(project):
    """Use the real readiness discovery and commands, adding locked resolution."""
    manifests = readiness.discover_cargo_manifests(project)
    if not manifests:
        raise ValueError('No cargo manifests discovered')
    for manifest in manifests:
        for path in (manifest, manifest.with_name('Cargo.lock')):
            if not (project / path).is_file() or not (project / path).stat().st_size:
                raise ValueError('Missing or empty cargo input: ' + str(path))
    steps = readiness.build_plan('cargo', project)
    if len(steps) != len(manifests):
        raise ValueError('Cargo profile and discovery differ')
    tests = []
    for manifest, step in zip(manifests, steps):
        argv = list(step.command)
        if argv[:2] != ['cargo', 'test'] or '--offline' not in argv or argv[argv.index('--manifest-path') + 1] != manifest.as_posix():
            raise ValueError('Unsupported release cargo command: ' + repr(argv))
        if '--locked' not in argv:
            argv.append('--locked')
        tests.append(argv)
    fetches = [['cargo', 'fetch', '--locked', '--manifest-path', p.as_posix()] for p in manifests]
    return manifests, fetches + tests


def execute_commands(project, commands, evidence, evidence_dir, env=None):
    """Public bounded executor also supports a real temporary failing-crate control."""
    for index, argv in enumerate(commands):
        output = evidence_dir / f'command-{index:03d}.log'
        row = dict(argv=list(argv), cwd=str(project), output=output.name, exit_code=None)
        evidence['commands'].append(row)
        print('Running: ' + ' '.join(argv), flush=True)
        with output.open('w') as log:
            result = subprocess.run(argv, cwd=project, env=env, stdout=log, stderr=subprocess.STDOUT, check=False)
        row['exit_code'] = result.returncode
        row['output_sha256'] = hashlib.sha256(output.read_bytes()).hexdigest()
        if result.returncode:
            return result.returncode if result.returncode > 0 else 128 - result.returncode
    return 0


def run(root, plan, evidence_dir):
    root, evidence_dir = Path(root).resolve(), Path(evidence_dir).resolve()
    evidence_dir.mkdir(parents=True, exist_ok=True)
    evidence = dict(status='failed', head=plan.get('head'), target_base=plan.get('target_base'),
                    plan=plan, commands=[], manifests=[], plan_validated=False)
    code = 1
    try:
        selection.validate(root, plan)
        evidence['plan_validated'] = True
        if plan.get('numerical_required') is not True:
            raise ValueError('Numerical runner requires explicit numerical_required=true')
        project = root / selection.PROJECT
        manifests, commands = cargo_plan(project)
        evidence['manifests'] = [p.as_posix() for p in manifests]
        evidence['input_sha256'] = {p.as_posix(): hashlib.sha256((project / p).read_bytes()).hexdigest()
            for m in manifests for p in (m, m.with_name('Cargo.lock'))}
        evidence['planned_commands'] = commands
        env = os.environ.copy()
        env.pop('CARGO_NET_OFFLINE', None)  # Fetch first; every test explicitly uses --offline.
        env['CARGO_TARGET_DIR'] = str(evidence_dir / 'target')
        evidence['environment'] = {'CARGO_TARGET_DIR': env['CARGO_TARGET_DIR'], 'python': sys.version}
        code = execute_commands(project, [['rustc', '--version'], ['cargo', '--version']], evidence, evidence_dir, env)
        if code == 0:
            code = execute_cargo(project, commands, evidence, evidence_dir, env)
        evidence['status'] = 'success' if code == 0 else 'failed'
    except Exception as exc:
        code = 1
        evidence['error'] = f'{type(exc).__name__}: {exc}'
        print(evidence['error'], file=sys.stderr)
    finally:
        evidence['exit_code'] = code
        (evidence_dir / 'numerical.json').write_text(json.dumps(evidence, indent=2) + '\n')
    return code


def execute_cargo(project, commands, evidence, evidence_dir, env):
    # Separate filenames retain tool-version output without overwriting it.
    logs = evidence_dir / 'cargo-logs'
    logs.mkdir(exist_ok=True)
    start = len(evidence['commands'])
    code = execute_commands(project, commands, evidence, logs, env)
    for row in evidence['commands'][start:]:
        row['output'] = 'cargo-logs/' + row['output']
    return code


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--plan', required=True)
    parser.add_argument('--evidence-dir', required=True)
    args = parser.parse_args()
    return run(HERE.parents[3], json.loads(Path(args.plan).read_text()), args.evidence_dir)


if __name__ == '__main__':
    raise SystemExit(main())
