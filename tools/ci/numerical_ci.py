#!/usr/bin/env python3
"""Execute the release-readiness cargo profile with candidate-bound evidence."""
import argparse
import hashlib
import importlib.util
import json
import os
import re
from pathlib import Path
import subprocess
import sys
import tomllib
import time


def load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


HERE = Path(__file__).resolve().parent
readiness = load_module('numerical_release_readiness', HERE.parent / 'release/check_release_readiness.py')
PROJECT = 'projects/chirality-piping/'


def validate_plan(root, plan):
    if plan.get('schema') != 'chirality-hosted-ci/v1':
        raise ValueError('Unknown selection schema')
    head = subprocess.check_output(['git', '-C', str(root), 'rev-parse', 'HEAD'], text=True).strip()
    if plan.get('head') != head:
        raise ValueError('Selection does not name the checked-out candidate')
    if plan.get('modes', {}).get('piping-numerical') != 'full':
        raise ValueError('Numerical runner requires explicit selected coverage')


def source_read_dependencies(project, manifests):
    """Conservative edges for Rust source/fixture reads outside Cargo deps.

    Resolve relative path literals against both Rust source and manifest roots:
    include_str!/include_bytes! use the former, CARGO_MANIFEST_DIR joins the
    latter. Considering both can over-select but cannot drop either form.
    Also recognize project-relative core paths used after finding the root.
    """
    roots = {(project / m.parent).resolve(): (project / m).resolve() for m in manifests}
    edges = {m: set() for m in manifests}
    for manifest in manifests:
        crate = project / manifest.parent
        for directory, dirs, files in os.walk(crate):
            dirs[:] = [d for d in dirs if d not in ('target', '.git')]
            for filename in files:
                if not filename.endswith('.rs'):
                    continue
                source = Path(directory) / filename
                for literal in re.findall(r'"([^"\n]+)"', source.read_text()):
                    if literal.startswith('core/'):
                        candidates = [(project / literal).resolve()]
                    elif '../' in literal:
                        candidates = [(base / literal).resolve() for base in (source.parent, crate)]
                    else:
                        continue
                    for candidate in candidates:
                        for root, supplier in roots.items():
                            if candidate.is_relative_to(root) and supplier != (project / manifest).resolve():
                                edges[manifest].add(supplier)
    return edges


def affected_manifests(project, manifests, paths):
    """Select changed crates and transitive path-dependency consumers.

    Shared/unknown inputs conservatively retain the full set. A removed crate
    likewise cannot be resolved here and falls back to full verification.
    """
    if not paths:
        return manifests
    selected = set()
    roots = {m.parent: m for m in manifests}
    for path in paths:
        if not path.startswith(PROJECT):
            return manifests
        relative = Path(path[len(PROJECT):])
        # Python test edits select the Python lane, not every Rust crate.
        if relative.parts[0] == 'tests' and relative.suffix == '.py':
            continue
        owners = [m for directory, m in roots.items() if relative.is_relative_to(directory)]
        if not owners:
            return manifests
        selected.update(owners)
    dependencies = source_read_dependencies(project, manifests)
    for manifest in manifests:
        data = tomllib.loads((project / manifest).read_text())
        targets = set()
        def visit(value):
            if isinstance(value, dict):
                # Workspace inheritance needs resolution outside this manifest.
                if value.get('workspace') is True:
                    raise ValueError('workspace inheritance')
                if isinstance(value.get('path'), str):
                    target = (project / manifest.parent / value['path'] / 'Cargo.toml').resolve()
                    targets.add(target)
                for child in value.values():
                    visit(child)
            elif isinstance(value, list):
                for child in value:
                    visit(child)
        try:
            for key in ('dependencies', 'dev-dependencies', 'build-dependencies', 'target'):
                visit(data.get(key, {}))
        except ValueError:
            return manifests
        dependencies[manifest].update(targets)
    while True:
        selected_paths = {(project / m).resolve() for m in selected}
        consumers = {m for m, deps in dependencies.items() if deps & selected_paths}
        enlarged = selected | consumers
        if enlarged == selected:
            return [m for m in manifests if m in selected]
        selected = enlarged


def cargo_plan(project, paths=None):
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
    chosen = affected_manifests(project, manifests, paths) if paths is not None else manifests
    tests = []
    for manifest, step in zip(manifests, steps):
        if manifest not in chosen:
            continue
        argv = list(step.command)
        if argv[:2] != ['cargo', 'test'] or '--offline' not in argv or argv[argv.index('--manifest-path') + 1] != manifest.as_posix():
            raise ValueError('Unsupported release cargo command: ' + repr(argv))
        if '--locked' not in argv:
            argv.append('--locked')
        if manifest.as_posix() == 'core/solver/frame_kernel/Cargo.toml':
            # Exact-arithmetic unit vectors dominate debug runtime. Their full
            # unchanged oracle set passes optimized with safety checks retained.
            # Historical floating-point integration pins remain on the original
            # profile: powi-derived inputs are profile-sensitive.
            tests.append(argv + ['--lib', '--config', 'profile.test.opt-level=2',
                '--config', 'profile.test.debug-assertions=true',
                '--config', 'profile.test.overflow-checks=true'])
            tests.append(argv + ['--test', '*', '--bins', '--examples'])
            tests.append(argv + ['--doc'])
        else:
            tests.append(argv)
    fetches = [['cargo', 'fetch', '--locked', '--manifest-path', p.as_posix()] for p in chosen]
    return chosen, fetches + tests


def build_commands(commands):
    """Compile the selected test profiles without executing tests.

    rustdoc has no stable --no-run mode. The ordinary test-profile library
    compilation supplies its dependencies; omit the separate doctest execution.
    """
    return [command + ['--no-run'] if command[:2] == ['cargo', 'test'] else list(command)
            for command in commands if '--doc' not in command]


def cargo_timings(output):
    """Cargo compilation and harness test time, distinct from command wall time."""
    text = re.sub(r'\x1b\[[0-9;]*m', '', output)
    compile_seconds = 0.0
    for duration in re.findall(r'Finished .* target\(s\) in ([^\n]+)', text):
        compile_seconds += sum(float(number) * {'m': 60, 's': 1}[unit]
                               for number, unit in re.findall(r'([0-9.]+)([ms])', duration))
    test_seconds = sum(float(seconds) for seconds in
                       re.findall(r'test result: .* finished in ([0-9.]+)s', text))
    return dict(compile_seconds=round(compile_seconds, 3), test_seconds=round(test_seconds, 3))


def execute_commands(project, commands, evidence, evidence_dir, env=None):
    """Public bounded executor also supports a real temporary failing-crate control."""
    for index, argv in enumerate(commands):
        output = evidence_dir / f'command-{index:03d}.log'
        row = dict(argv=list(argv), cwd=str(project), output=output.name, exit_code=None)
        evidence['commands'].append(row)
        print('Running: ' + ' '.join(argv), flush=True)
        started = time.monotonic()
        with output.open('w') as log:
            result = subprocess.run(argv, cwd=project, env=env, stdout=log, stderr=subprocess.STDOUT, check=False)
        row['elapsed_seconds'] = round(time.monotonic() - started, 3)
        row['exit_code'] = result.returncode
        row.update(cargo_timings(output.read_text()))
        row['output_sha256'] = hashlib.sha256(output.read_bytes()).hexdigest()
        if result.returncode:
            return result.returncode if result.returncode > 0 else 128 - result.returncode
    return 0


def run(root, plan, evidence_dir, build_only=False):
    root, evidence_dir = Path(root).resolve(), Path(evidence_dir).resolve()
    evidence_dir.mkdir(parents=True, exist_ok=True)
    evidence = dict(status='failed', head=plan.get('head'), target_base=plan.get('target_base'),
                    plan=plan, commands=[], manifests=[], plan_validated=False,
                    execution='build-only' if build_only else 'tests')
    code = 1
    try:
        validate_plan(root, plan)
        evidence['plan_validated'] = True
        project = root / PROJECT
        manifests, commands = cargo_plan(project, plan.get('paths') if plan.get('event') == 'pull_request' else None)
        if build_only:
            commands = build_commands(commands)
            print('Build-only: compiling selected profiles; no tests execute.', flush=True)
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
        totals = {key: round(sum(row.get(key, 0) for row in evidence['commands']), 3)
                  for key in ('compile_seconds', 'test_seconds', 'elapsed_seconds')}
        evidence['timing'] = totals
        message = f"Numerical {evidence['execution']} timing (seconds): {json.dumps(totals)}; status={evidence['status']}"
        print(message, flush=True)
        if os.environ.get('GITHUB_STEP_SUMMARY'):
            with open(os.environ['GITHUB_STEP_SUMMARY'], 'a') as summary:
                summary.write(message + '\n')
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
    parser.add_argument('--build-only', action='store_true', help='Compile the selected test profiles without running tests')
    args = parser.parse_args()
    return run(HERE.parents[3], json.loads(Path(args.plan).read_text()), args.evidence_dir, build_only=args.build_only)


if __name__ == '__main__':
    raise SystemExit(main())
