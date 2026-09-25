#!/usr/bin/env python3
"""Checks 2 and 4 of the lock-impact review, run against the 5b1c worktree.

stage fetch : `cargo +1.97.1 fetch --locked --manifest-path M` for every manifest
              the numerical CI discovers (core/**, validation/benchmarks/**),
              network allowed exactly as CI does (CARGO_NET_OFFLINE unset).
stage meta  : `cargo +1.97.1 metadata --locked --offline --format-version 1` and
              `cargo +1.97.1 tree --locked --offline` for the four changed crates.
stage test  : `cargo +1.97.1 test --locked --offline -j 2 --manifest-path M`
              for the four changed crates, one CARGO_TARGET_DIR per crate
              outside the repository.
Every stage records the SHA-256 of every discovered Cargo.lock before and
after, so any lock rewrite is detected.

Usage: python3 run_cargo_checks.py <repo_root> <out_dir> <target_base> <stage>...
"""
import hashlib
import json
import os
import subprocess
import sys
import time
from pathlib import Path

root, out, target_base = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
stages = sys.argv[4:]
project = root / 'projects/chirality-piping'
TOOL = '+1.97.1'
CHANGED = ['core/reporting/report_package/Cargo.toml', 'validation/benchmarks/mechanics/Cargo.toml',
           'validation/benchmarks/numerical_integrity/Cargo.toml', 'validation/benchmarks/stress/Cargo.toml']


def discover():
    found = []
    for base in ('core', 'validation/benchmarks'):
        for m in sorted((project / base).rglob('Cargo.toml')):
            rel = m.relative_to(project)
            if 'target' not in rel.parts:
                found.append(rel.as_posix())
    return found


def lock_hashes(manifests):
    return {m: hashlib.sha256((project / m).with_name('Cargo.lock').read_bytes()).hexdigest() for m in manifests}


def run(name, argv, env, log):
    t = time.monotonic()
    with open(log, 'wb') as fh:
        r = subprocess.run(argv, cwd=project, env=env, stdout=fh, stderr=subprocess.STDOUT)
    return dict(name=name, argv=argv, cwd='projects/chirality-piping', exit_code=r.returncode,
                seconds=round(time.monotonic() - t, 2), log=str(log.relative_to(out)),
                log_sha256=hashlib.sha256(log.read_bytes()).hexdigest())


manifests = discover()
base_env = os.environ.copy()
for k in ('CARGO_NET_OFFLINE', 'CARGO_TARGET_DIR', 'RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS'):
    base_env.pop(k, None)
base_env['CARGO_BUILD_JOBS'] = '2'
base_env['CARGO_TERM_COLOR'] = 'never'
for stage in stages:
    sdir = out / stage
    sdir.mkdir(parents=True, exist_ok=True)
    rec = dict(stage=stage, head=subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=root, text=True).strip(),
               tool=subprocess.check_output(['cargo', TOOL, '--version'], text=True).strip(),
               rustc=subprocess.check_output(['rustc', TOOL, '--version'], text=True).strip(),
               manifest_count=len(manifests), locks_before=lock_hashes(manifests), commands=[])
    env = dict(base_env, CARGO_TARGET_DIR=str(target_base / '_nobuild'))
    if stage in ('fetch', 'fetchcold'):
        if stage == 'fetchcold':
            # Hosted-runner analogue: an empty CARGO_HOME (no registry index or crate cache).
            cold = target_base / '_cold_cargo_home'
            assert not cold.exists(), cold
            env['CARGO_HOME'] = str(cold)
            rec['CARGO_HOME'] = str(cold)
        for i, m in enumerate(manifests):
            rec['commands'].append(run(m, ['cargo', TOOL, 'fetch', '--locked', '--manifest-path', m], env, sdir / f'{i:02}.log'))
    elif stage == 'meta':
        for m in CHANGED:
            crate = Path(m).parent.name
            meta_log = sdir / f'{crate}.metadata.json'
            rec['commands'].append(run(m + ' metadata', ['cargo', TOOL, 'metadata', '--locked', '--offline', '--format-version', '1', '--manifest-path', m], env, meta_log))
            rec['commands'].append(run(m + ' tree', ['cargo', TOOL, 'tree', '--locked', '--offline', '--target', 'all', '-e', 'normal,build,dev', '--prefix', 'depth', '--manifest-path', m], env, sdir / f'{crate}.tree.log'))
    elif stage == 'test':
        for m in CHANGED:
            crate = Path(m).parent.name
            tenv = dict(base_env, CARGO_TARGET_DIR=str(target_base / crate), RUST_TEST_THREADS='2')
            rec['commands'].append(run(m + ' test', ['cargo', TOOL, 'test', '--locked', '--offline', '-j', '2', '--manifest-path', m], tenv, sdir / f'{crate}.test.log'))
            rec['commands'][-1]['CARGO_TARGET_DIR'] = tenv['CARGO_TARGET_DIR']
    rec['locks_after'] = lock_hashes(manifests)
    rec['locks_unchanged'] = rec['locks_before'] == rec['locks_after']
    rec['failures'] = [c['name'] for c in rec['commands'] if c['exit_code']]
    rec['git_status_after'] = subprocess.check_output(['git', 'status', '--short', '--', 'projects/chirality-piping/core', 'projects/chirality-piping/validation', 'projects/chirality-piping/apps/desktop/src-tauri'], cwd=root, text=True)
    (sdir / 'RESULTS.json').write_text(json.dumps(rec, indent=2) + '\n')
    print(stage, 'commands', len(rec['commands']), 'failures', rec['failures'], 'locks_unchanged', rec['locks_unchanged'], flush=True)
