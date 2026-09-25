#!/usr/bin/env python3
"""Checks 1 and 3 of the lock-impact review (read-only; uses git blobs only).

Check 1: each changed lockfile diff e650 -> 5b1c is additive: no existing
(name, version, source) key removed, no checksum/source changed, lockfile
format version unchanged, existing dependency lists only gain entries.
Check 3: every added registry package (name, version, source, checksum)
already appears with the identical checksum in a maintained lockfile at e650
(core/**, validation/benchmarks/**, apps/desktop/src-tauri); report any
version new to the repository and any checksum disagreement anywhere.

Usage: python3 check_additive_and_pins.py <repo_root> > check1_3.json
"""
import json
import subprocess
import sys
import tomllib
from collections import defaultdict

OLD = 'e65001ad50072d3399bf204da02055b0f360353a'
NEW = '5b1ccd356c0d9656f7b9f42c6558f7a4c73f9295'
P = 'projects/chirality-piping/'
CHANGED = [P + x for x in (
    'core/reporting/report_package/Cargo.lock',
    'validation/benchmarks/mechanics/Cargo.lock',
    'validation/benchmarks/numerical_integrity/Cargo.lock',
    'validation/benchmarks/stress/Cargo.lock')]
root = sys.argv[1]


def git(*a):
    return subprocess.check_output(['git', *a], cwd=root)


def lock(rev, path):
    return tomllib.loads(git('show', f'{rev}:{path}').decode())


def key(p):
    return (p['name'], p['version'], p.get('source'))


out = {'old': OLD, 'new': NEW, 'check1': [], 'check3': {}}
added_registry = []
for path in CHANGED:
    a, b = lock(OLD, path), lock(NEW, path)
    pa = {key(p): p for p in a['package']}
    pb = {key(p): p for p in b['package']}
    dup_a = len(pa) != len(a['package'])
    dup_b = len(pb) != len(b['package'])
    removed = sorted(map(list, pa.keys() - pb.keys()), key=str)
    added = sorted(map(list, pb.keys() - pa.keys()), key=str)
    checksum_changed = [list(k) for k in pa if k in pb and pa[k].get('checksum') != pb[k].get('checksum')]
    dep_changes = {}
    dep_removals = {}
    for k in pa:
        if k in pb:
            da, db = set(pa[k].get('dependencies', [])), set(pb[k].get('dependencies', []))
            if da != db:
                dep_changes['/'.join(filter(None, k[:2]))] = sorted(db - da)
                if da - db:
                    dep_removals['/'.join(filter(None, k[:2]))] = sorted(da - db)
            extra = set(pa[k]) ^ set(pb[k])
            if extra - {'dependencies'}:
                dep_removals.setdefault('field_set_changed', []).append([list(k), sorted(extra)])
    row = dict(path=path, lock_format_version=[a.get('version'), b.get('version')],
               packages_before=len(pa), packages_after=len(pb), duplicate_keys=[dup_a, dup_b],
               removed_keys=removed, checksum_changed=checksum_changed,
               added_keys=added, existing_dependency_additions=dep_changes,
               existing_dependency_removals_or_field_changes=dep_removals,
               additive_only=(not removed and not checksum_changed and not dep_removals
                              and a.get('version') == b.get('version') and not dup_a and not dup_b))
    out['check1'].append(row)
    for k in pb.keys() - pa.keys():
        if k[2] is not None:
            added_registry.append((path, pb[k]))
        else:
            row.setdefault('added_path_packages', []).append(list(k))

# Maintained lockfiles at e650 (the base the repair must agree with).
tracked = git('ls-tree', '-r', '--name-only', OLD).decode().splitlines()
maintained = [t for t in tracked if t.endswith('/Cargo.lock') and (
    t.startswith(P + 'core/') or t.startswith(P + 'validation/benchmarks/') or t == P + 'apps/desktop/src-tauri/Cargo.lock')]
all_locks = [t for t in tracked if t.endswith('Cargo.lock')]
index = defaultdict(list)          # (name,version,source) -> [(path, checksum)]
by_name = defaultdict(set)         # name -> versions in maintained locks
all_index = defaultdict(set)       # (name,version,source) -> checksums anywhere in repo
for t in all_locks:
    try:
        data = lock(OLD, t)
    except Exception as exc:  # noqa: BLE001 - record and continue
        out.setdefault('unparsed_locks', []).append([t, repr(exc)])
        continue
    for p in data.get('package', []):
        if p.get('source'):
            all_index[key(p)].add(p.get('checksum'))
            if t in maintained:
                index[key(p)].append((t, p.get('checksum')))
                by_name[p['name']].add(p['version'])
# Also include the post-repair state of the whole repository for disagreement checks.
tracked_new = git('ls-tree', '-r', '--name-only', NEW).decode().splitlines()
for t in tracked_new:
    if t.endswith('Cargo.lock'):
        try:
            data = lock(NEW, t)
        except Exception:
            continue
        for p in data.get('package', []):
            if p.get('source'):
                all_index[key(p)].add(p.get('checksum'))

out['check3']['maintained_lock_count_e650'] = len(maintained)
out['check3']['all_lock_count_e650'] = len(all_locks)
rows = []
for path, p in added_registry:
    k = key(p)
    hits = index.get(k, [])
    rows.append(dict(lock=path, name=p['name'], version=p['version'], source=p['source'],
                     checksum=p.get('checksum'),
                     maintained_e650_occurrences=len(hits),
                     maintained_e650_checksum_match=sorted({c for _, c in hits}) == [p.get('checksum')] if hits else False,
                     example_pins=sorted({h for h, _ in hits if h.endswith(('core/product_physics/Cargo.lock', 'src-tauri/Cargo.lock', 'result_export/Cargo.lock'))}),
                     new_to_repository=not hits,
                     checksums_seen_anywhere=sorted(c for c in all_index[k] if c),
                     other_versions_in_maintained_locks=sorted(by_name[p['name']] - {p['version']})))
out['check3']['added_registry_packages'] = rows
out['check3']['all_match'] = all(r['maintained_e650_checksum_match'] and len(r['checksums_seen_anywhere']) == 1 for r in rows)
out['check3']['new_third_party_versions'] = [f"{r['name']} {r['version']}" for r in rows if r['new_to_repository']]
# Repository-wide checksum disagreement for any registry package (informational).
out['check3']['repo_wide_checksum_conflicts'] = [list(k) + [sorted(v)] for k, v in all_index.items() if len({c for c in v if c}) > 1]
out['check1_all_additive'] = all(r['additive_only'] for r in out['check1'])
print(json.dumps(out, indent=2))
