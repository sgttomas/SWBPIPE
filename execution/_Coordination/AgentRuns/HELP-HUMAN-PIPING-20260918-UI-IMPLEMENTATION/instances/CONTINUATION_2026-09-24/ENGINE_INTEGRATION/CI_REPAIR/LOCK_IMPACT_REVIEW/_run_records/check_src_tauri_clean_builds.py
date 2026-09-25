#!/usr/bin/env python3
"""Check 5 addendum: clean-build byte identity of the report_package unit (and
every unit it pulls in) inside the src-tauri resolve, with the leaf
report_package/Cargo.lock at 5b1c vs e650 bytes, plus a same-lock control.

The first attempt in check_src_tauri_resolve.py compared two *different*
target-dir paths with incremental compilation on; its byte difference is not
attributable to the leaf lock. Here every build uses the SAME target path
(removed between builds) and CARGO_INCREMENTAL=0.

Usage: python3 check_src_tauri_clean_builds.py <repo_root> <out_dir> <scratch_project_dir> <target_dir>
"""
import hashlib
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

root, out, sp, target = (Path(x) for x in sys.argv[1:5])
OLD, NEW = 'e65001ad50072d3399bf204da02055b0f360353a', '5b1ccd356c0d9656f7b9f42c6558f7a4c73f9295'
LEAF = 'core/reporting/report_package/Cargo.lock'
leaf = sp / LEAF
blob = {t: subprocess.check_output(['git', 'show', f'{r}:projects/chirality-piping/{LEAF}'], cwd=root) for t, r in (('e650', OLD), ('5b1c', NEW))}
env = os.environ.copy()
for k in ('CARGO_NET_OFFLINE', 'RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS'):
    env.pop(k, None)
env.update(CARGO_TERM_COLOR='never', CARGO_BUILD_JOBS='2', CARGO_INCREMENTAL='0', CARGO_TARGET_DIR=str(target))
cmd = ['cargo', '+1.97.1', 'build', '--locked', '--offline', '-j', '2', '-p', 'open_pipe_stress_report_package',
       '--manifest-path', 'apps/desktop/src-tauri/Cargo.toml']
runs = []
for i, tag in enumerate(('5b1c', 'e650', '5b1c-control')):
    if target.exists():
        shutil.rmtree(target)
    leaf.write_bytes(blob[tag.split('-')[0]])
    r = subprocess.run(cmd, cwd=sp, env=env, capture_output=True, text=True)
    (out / f'clean-build-{i}-{tag}.log').write_text(r.stdout + r.stderr)
    deps = target / 'debug' / 'deps'
    files = {p.name: hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(deps.iterdir())
             if p.suffix in ('.rlib', '.rmeta')} if deps.exists() else {}
    runs.append(dict(tag=tag, leaf_sha256=hashlib.sha256(leaf.read_bytes()).hexdigest(), exit_code=r.returncode,
                     artifact_count=len(files), artifacts=files))
res = dict(command=cmd, env=dict(CARGO_INCREMENTAL='0', CARGO_BUILD_JOBS='2', CARGO_TARGET_DIR=str(target)), runs=runs)
res['leaf_5b1c_vs_e650_identical'] = runs[0]['artifacts'] == runs[1]['artifacts'] and bool(runs[0]['artifacts'])
res['control_5b1c_vs_5b1c_identical'] = runs[0]['artifacts'] == runs[2]['artifacts']
res['differing_artifacts_5b1c_vs_e650'] = sorted(k for k in set(runs[0]['artifacts']) | set(runs[1]['artifacts'])
                                                 if runs[0]['artifacts'].get(k) != runs[1]['artifacts'].get(k))
leaf.write_bytes(blob['5b1c'])
(out / 'SRC_TAURI_CLEAN_BUILDS.json').write_text(json.dumps(res, indent=2) + '\n')
print(json.dumps({k: v for k, v in res.items() if k != 'runs'}, indent=2), [(r['tag'], r['exit_code'], r['artifact_count']) for r in runs])
