#!/usr/bin/env python3
"""Check 5 (input part): compare the 1,092 recorded native build inputs with
Git blobs at 8b982aa7 (sanity), e650 and 5b1c. Read-only; uses git blobs.

Reports every recorded path whose bytes differ at e650 or 5b1c, every
recorded path missing at a revision, and every tracked path that the
recorder's own path set (build.py `git ls-files` roots) would add at that
revision. Also reports the exact identity of the named lock/manifest inputs.

Usage: python3 check_native_inputs.py <repo_root> <inputs-pre.json> <inputs-post.json>
"""
import hashlib
import json
import subprocess
import sys

root, pre_path, post_path = sys.argv[1:4]
REVS = {'native_8b98': '8b982aa7ce64afe37e6067d1038d92608f4aaf3f',
        'pr905_e650': 'e65001ad50072d3399bf204da02055b0f360353a',
        'handoff_5b1c': '5b1ccd356c0d9656f7b9f42c6558f7a4c73f9295'}
P = 'projects/chirality-piping/'
# The recorder's roots (build.py): apps/desktop, core, schemas, fixtures, package.json, package-lock.json
ROOTS = [P + x for x in ('apps/desktop', 'core', 'schemas', 'fixtures', 'package.json', 'package-lock.json')]


def git(*a, text=True):
    return subprocess.check_output(['git', *a], cwd=root, text=text)


def blob_hashes(rev, paths):
    """sha256 of each path's blob at rev (None if absent), via one cat-file --batch."""
    spec = ''.join(f'{rev}:{p}\n' for p in paths).encode()
    proc = subprocess.run(['git', 'cat-file', '--batch'], cwd=root, input=spec, capture_output=True, check=True)
    data, out, i = proc.stdout, {}, 0
    for p in paths:
        nl = data.index(b'\n', i)
        header = data[i:nl].decode()
        if header.endswith(' missing'):
            out[p] = None
            i = nl + 1
            continue
        size = int(header.split()[2])
        body = data[nl + 1: nl + 1 + size]
        out[p] = hashlib.sha256(body).hexdigest()
        i = nl + 1 + size + 1
    return out


pre = json.load(open(pre_path))
post = json.load(open(post_path))
recorded = {r['path']: r['sha256'] for r in pre}
res = {'recorded_count': len(pre), 'recorded_unique_paths': len(recorded), 'pre_equals_post': pre == post}
lock_inputs = sorted(p for p in recorded if p.endswith('Cargo.lock'))
res['recorded_cargo_lock_inputs'] = lock_inputs
res['changed_leaf_locks_in_recorded_inputs'] = [p for p in lock_inputs if p.endswith((
    'core/reporting/report_package/Cargo.lock', 'validation/benchmarks/mechanics/Cargo.lock',
    'validation/benchmarks/numerical_integrity/Cargo.lock', 'validation/benchmarks/stress/Cargo.lock'))]
res['any_validation_benchmarks_path_in_inputs'] = any('/validation/' in p for p in recorded)
for name, rev in REVS.items():
    h = blob_hashes(rev, list(recorded))
    differ = sorted(p for p in recorded if h[p] is not None and h[p] != recorded[p])
    missing = sorted(p for p in recorded if h[p] is None)
    tracked_now = set(git('ls-tree', '-r', '--name-only', rev, '--', *ROOTS).splitlines())
    added = sorted(tracked_now - set(recorded))
    res[name] = {'rev': rev, 'differing_paths': differ, 'missing_paths': missing,
                 'tracked_paths_not_in_recorded_set': added,
                 'differing_detail': {p: {'recorded': recorded[p], 'at_rev': h[p]} for p in differ}}
named = [P + 'apps/desktop/src-tauri/Cargo.lock', P + 'apps/desktop/src-tauri/Cargo.toml',
         P + 'core/reporting/report_package/Cargo.lock', P + 'core/reporting/report_package/Cargo.toml',
         P + 'core/model_operations/operation_applier/Cargo.lock', P + 'core/model_operations/operation_applier/Cargo.toml',
         P + 'core/loads/self_weight_wasm/Cargo.lock', P + 'core/loads/self_weight_wasm/Cargo.toml',
         P + 'apps/desktop/src-tauri/tauri.conf.json', P + 'apps/desktop/package.json', P + 'package-lock.json',
         P + 'apps/desktop/scripts/build-wasm-engine.mjs']
res['named_inputs'] = {}
for p in named:
    row = {'recorded': recorded.get(p)}
    for name, rev in REVS.items():
        row[name] = blob_hashes(rev, [p])[p]
    row['blob_ids'] = {name: git('rev-parse', f'{rev}:{p}').strip() for name, rev in REVS.items()}
    res['named_inputs'][p] = row
print(json.dumps(res, indent=2))
