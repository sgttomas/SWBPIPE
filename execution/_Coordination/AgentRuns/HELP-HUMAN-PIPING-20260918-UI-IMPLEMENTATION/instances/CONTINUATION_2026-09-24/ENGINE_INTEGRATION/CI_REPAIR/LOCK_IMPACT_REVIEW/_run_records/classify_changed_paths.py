#!/usr/bin/env python3
"""Check 5 (classification): every path changed 8b982aa7 -> 5b1c, classified by
whether it is a recorded native input, lies inside a src-tauri path-package
directory (compiled closure candidate), inside a WASM crate graph, or inside
apps/desktop (dist candidate). Read-only.

Usage: python3 classify_changed_paths.py <repo_root> <metadata-worktree-5b1c.log.gz> <inputs-pre.json>
"""
import gzip, json, subprocess, sys
from collections import Counter
root, meta_gz, inputs = sys.argv[1:4]
OLD, NEW = '8b982aa7ce64afe37e6067d1038d92608f4aaf3f', '5b1ccd356c0d9656f7b9f42c6558f7a4c73f9295'
meta = json.loads(gzip.open(meta_gz).read())
rootdir = root.rstrip('/') + '/'
pkg_dirs = sorted({p['manifest_path'][len(rootdir):].rsplit('/', 1)[0] + '/' for p in meta['packages'] if p['source'] is None})
wasm_roots = []
for crate in ('core/model_operations/operation_applier', 'core/loads/self_weight_wasm'):
    m = json.loads(subprocess.check_output(['cargo', '+1.97.1', 'metadata', '--locked', '--offline', '--format-version', '1',
                                            '--manifest-path', f'projects/chirality-piping/{crate}/Cargo.toml'], cwd=root))
    wasm_roots += [p['manifest_path'][len(rootdir):].rsplit('/', 1)[0] + '/' for p in m['packages'] if p['source'] is None]
wasm_roots = sorted(set(wasm_roots))
recorded = {r['path'] for r in json.load(open(inputs))}
changed = subprocess.check_output(['git', 'diff', '--name-status', OLD, NEW], cwd=root, text=True).splitlines()
rows, cats = [], Counter()
for line in changed:
    status, path = line.split('\t', 1)
    in_pkg = [d for d in pkg_dirs if path.startswith(d)]
    in_wasm = [d for d in wasm_roots if path.startswith(d)]
    if '/execution/' in path:
        cat = 'execution evidence record'
    elif path in recorded:
        cat = 'recorded native input'
    else:
        cat = path.split('/')[2] + '/' + (path.split('/')[3] if len(path.split('/')) > 4 else '')
    cats[cat] += 1
    rows.append(dict(status=status, path=path, recorded_native_input=path in recorded,
                     inside_src_tauri_path_package=in_pkg, inside_wasm_crate_graph=in_wasm,
                     inside_apps_desktop=path.startswith('projects/chirality-piping/apps/desktop/'), category=cat))
non_evidence = [r for r in rows if r['category'] != 'execution evidence record']
print(json.dumps(dict(old=OLD, new=NEW, src_tauri_path_package_dirs=pkg_dirs, wasm_path_package_dirs=wasm_roots,
                      changed_count=len(rows), category_counts=cats, non_evidence_paths=non_evidence,
                      closure_candidates=[r for r in rows if r['inside_src_tauri_path_package'] or r['inside_wasm_crate_graph'] or r['inside_apps_desktop']]), indent=2))
