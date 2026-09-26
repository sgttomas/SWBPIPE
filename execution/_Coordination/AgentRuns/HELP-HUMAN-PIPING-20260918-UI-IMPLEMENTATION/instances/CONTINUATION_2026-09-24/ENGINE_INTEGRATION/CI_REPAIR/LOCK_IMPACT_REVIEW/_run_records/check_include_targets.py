#!/usr/bin/env python3
"""Check 5 (closure part): list every include_str!/include_bytes!/include! literal
target in tracked Rust sources under core/ and apps/desktop/src-tauri at 5b1c,
resolve it relative to the including file, and report any target outside the
native recorder's roots (apps/desktop, core, schemas, fixtures) or any target
that changed between 8b982aa7 and 5b1c. Also report every Rust source that
names Cargo.lock, and every env!/option_env! use (compile-time env inputs).

Usage: python3 check_include_targets.py <repo_root>
"""
import json
import os
import re
import subprocess
import sys

root = sys.argv[1]
P = 'projects/chirality-piping/'
OLD, NEW = '8b982aa7ce64afe37e6067d1038d92608f4aaf3f', '5b1ccd356c0d9656f7b9f42c6558f7a4c73f9295'
ROOTS = tuple(P + x for x in ('apps/desktop/', 'core/', 'schemas/', 'fixtures/'))
MACRO = re.compile(r'include(?:_str|_bytes)?!\s*\(\s*(?:concat!\s*\(\s*env!\s*\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*)?"([^"]+)"', re.S)
ENV = re.compile(r'\b(option_env|env)!\s*\(\s*"([^"]+)"')


def git(*a):
    return subprocess.check_output(['git', *a], cwd=root, text=True)


files = [f for f in git('ls-tree', '-r', '--name-only', NEW, '--', P + 'core', P + 'apps/desktop/src-tauri').splitlines() if f.endswith('.rs')]
targets, outside, lock_refs, envs, changed = {}, [], [], {}, []
for f in files:
    text = git('show', f'{NEW}:{f}')
    for m in MACRO.finditer(text):
        lit = re.sub(r'\\\n\s*', '', m.group(1))  # Rust string-continuation escape
        base = os.path.dirname(f)
        if 'CARGO_MANIFEST_DIR' in text[m.start():m.end()]:
            # concat!(env!("CARGO_MANIFEST_DIR"), "/x"): relative to the package root
            pkg = base
            while pkg and git('ls-tree', '--name-only', NEW, pkg + '/Cargo.toml').strip() == '':
                pkg = os.path.dirname(pkg)
            resolved = os.path.normpath(pkg + '/' + lit.lstrip('/'))
        else:
            resolved = os.path.normpath(os.path.join(base, lit))
        targets.setdefault(resolved, []).append(f)
    if 'Cargo.lock' in text:
        lock_refs.append(f)
    for m in ENV.finditer(text):
        envs.setdefault(m.group(2), set()).add(f)
for t in sorted(targets):
    if not t.startswith(ROOTS):
        outside.append(t)
    a = subprocess.run(['git', 'rev-parse', '-q', '--verify', f'{OLD}:{t}'], cwd=root, capture_output=True, text=True).stdout.strip()
    b = subprocess.run(['git', 'rev-parse', '-q', '--verify', f'{NEW}:{t}'], cwd=root, capture_output=True, text=True).stdout.strip()
    if a != b or not b:
        changed.append([t, a or None, b or None])
print(json.dumps({'rust_files_scanned': len(files), 'distinct_include_targets': len(targets),
                  'targets_outside_recorder_roots': {t: sorted(set(targets[t])) for t in outside},
                  'targets_changed_8b98_to_5b1c_or_untracked': changed,
                  'rust_sources_naming_Cargo_lock': lock_refs,
                  'env_macro_names': {k: sorted(v) for k, v in sorted(envs.items())}}, indent=2))
