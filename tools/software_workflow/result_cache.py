#!/usr/bin/env python3
"""Fingerprint successful Piping verification, never substitute a nearby cache key."""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import platform
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
# Names `hosted_ci.py` admits into the product job's Playwright command.
BROWSER_PROJECT = re.compile(r'[a-z0-9-]+')
BROWSER_SPEC = re.compile(r'[A-Za-z0-9_.-]+\.spec\.ts')


def browser_commands(desktop_mode, project, specs):
    """The browser verification's command identity: the build, the exact
    Chromium project and spec set the product job ran, and the desktop suite
    when it was selected. A pass without a spec cannot stand for a run that
    selects it."""
    if desktop_mode is None:
        raise SystemExit('Browser verification key needs the desktop selection')
    names = (specs or '').split()
    if not project or not BROWSER_PROJECT.fullmatch(project) or not names or not all(BROWSER_SPEC.fullmatch(n) for n in names):
        raise SystemExit('Browser verification key needs the selected Chromium project and specs')
    commands = ['production-build', project + ':' + ','.join(names)]
    if desktop_mode == 'full':
        commands.insert(0, 'desktop-test')
    return commands


def included(path):
    parts = Path(path).parts
    # Match the product jobs' sparse checkout: execution records are unavailable.
    if parts[0] == 'execution' or (len(parts) > 2 and parts[0] == 'projects' and parts[2] == 'execution'):
        return False
    # Conservative first version: every other tracked input participates,
    # including Python tests and fixtures. No inferred dependency exclusions.
    return True


def fingerprint(root, suite, commands, environment, selection=None):
    tree = subprocess.check_output(['git', '-C', str(root), 'ls-tree', '-r', '-z', 'HEAD'])
    entries = [entry.decode() for entry in tree.split(b'\0') if entry and included(entry.split(b'\t', 1)[1].decode())]
    payload = dict(version=2, suite=suite, tree=entries, commands=commands, environment=environment, selection=selection)
    return hashlib.sha256(json.dumps(payload, sort_keys=True).encode()).hexdigest()


def agreed_key(keys_dir, expected):
    """The one key every expected verifier published, or None.

    A record may stand only for verification that ran under its key. Verifiers
    on different runner images publish different keys; then nothing is saved.
    """
    keys = {}
    for name in expected:
        path = Path(keys_dir) / ('browser-key-' + name) / 'key'
        if not path.is_file():
            return None
        keys[name] = path.read_text().strip()
    values = set(keys.values())
    if len(values) != 1 or not next(iter(values)).startswith('verified-v2-browser-'):
        return None
    return values.pop()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=['key', 'save', 'reuse', 'agree'])
    parser.add_argument('--suite', choices=['numerical', 'browser'], required=True)
    parser.add_argument('--plan', type=Path)
    parser.add_argument('--desktop-mode', choices=['full', 'not-applicable'])
    parser.add_argument('--browser-project')
    parser.add_argument('--browser-specs', help='space-separated, in the order the product job runs them')
    parser.add_argument('--record', type=Path)
    parser.add_argument('--keys-dir', type=Path)
    parser.add_argument('--expect', action='append', default=[])
    args = parser.parse_args()
    if args.action == 'agree':
        if args.suite != 'browser' or args.keys_dir is None or not args.expect:
            raise SystemExit('agree needs --suite browser, --keys-dir and --expect')
        key = agreed_key(args.keys_dir, args.expect)
        with open(os.environ['GITHUB_OUTPUT'], 'a') as stream:
            stream.write('key=' + (key or '') + '\n')
        print(f'Verifier keys agree: {key}' if key else 'Verifier keys are missing or differ; no reuse record is saved.')
        return
    if args.record is None:
        raise SystemExit('--record is required')
    if args.action == 'key':
        selection = None
        if args.suite == 'browser':
            # A pass without the full desktop suite, or without a selected
            # spec, cannot stand for a run that needs it.
            commands = browser_commands(args.desktop_mode, args.browser_project, args.browser_specs)
        if args.suite == 'numerical':
            path = ROOT / 'tools/ci/numerical_ci.py'
            spec = importlib.util.spec_from_file_location('numerical_cache_plan', path)
            module = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(module)
            plan = json.loads(args.plan.read_text())
            module.validate_plan(ROOT, plan)
            selection = plan['piping_numerical_selection']
            _, commands = module.cargo_plan(ROOT / '.', selection=selection)
        environment = {key: os.environ.get(key, '') for key in ['ImageOS', 'ImageVersion', 'RUNNER_OS', 'RUNNER_ARCH']}
        # Missing runner identity cannot establish equivalent execution conditions.
        if not all(environment.values()):
            raise SystemExit('Runner image identity unavailable; cannot key verification reuse')
        environment['python'] = platform.python_version()
        if args.suite == 'browser':
            environment['node'] = subprocess.check_output(['node', '--version'], text=True).strip()
        key = 'verified-v2-' + args.suite + '-' + fingerprint(ROOT, args.suite, commands, environment, selection)
        with open(os.environ['GITHUB_OUTPUT'], 'a') as stream:
            stream.write('key=' + key + '\n')
        print(key)
        return
    key = os.environ['RESULT_KEY']
    if args.action == 'save':
        args.record.parent.mkdir(parents=True, exist_ok=True)
        record = dict(key=key, conclusion='success', head=subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
                      run=f"{os.environ['GITHUB_SERVER_URL']}/{os.environ['GITHUB_REPOSITORY']}/actions/runs/{os.environ['GITHUB_RUN_ID']}")
        args.record.write_text(json.dumps(record))
    else:
        record = json.loads(args.record.read_text())
        if record.get('key') != key or record.get('conclusion') != 'success' or not record.get('run'):
            raise SystemExit('Cached verification record does not match these inputs')
        message = f"Reused successful {args.suite} verification from {record['run']} (source head {record['head']}); exact input/environment key {key}. No tests executed in this job."
        print(message)
        with open(os.environ['GITHUB_STEP_SUMMARY'], 'a') as stream:
            stream.write(message + '\n')


if __name__ == '__main__':
    main()
