#!/usr/bin/env python3
"""Check 5 (Cargo semantics, empirical): does core/reporting/report_package/Cargo.lock
affect the src-tauri (native) dependency resolution or compilation?

A. In the real 5b1c worktree: `cargo +1.97.1 metadata --locked --offline
   --format-version 1 --manifest-path apps/desktop/src-tauri/Cargo.toml`.
B. In a scratch `git archive` export of 5b1c (outside the repository) the same
   command is run with report_package/Cargo.lock set to: its 5b1c bytes, its
   e650 bytes (== 8b982aa7 blob), absent, and deliberately invalid TOML.
   For each: the resolve (IDs normalised to a <ROOT> placeholder), the
   aarch64-apple-darwin `cargo tree` and the SHA-256 of src-tauri/Cargo.lock
   and report_package/Cargo.lock before/after are recorded.
C. Compile the report_package library unit inside the src-tauri resolve
   (`cargo build -p open_pipe_stress_report_package` with the src-tauri
   manifest) using the 5b1c leaf lock, then swap in the e650 leaf lock and
   rebuild verbosely: every unit must be reported Fresh. Then a clean build in
   a second target dir with the e650 leaf lock must produce byte-identical
   report_package .rlib/.rmeta outputs.

Usage: python3 check_src_tauri_resolve.py <repo_root> <out_dir> <scratch_dir> <target_base>
"""
import hashlib
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

root, out, scratch, tbase = (Path(x) for x in sys.argv[1:5])
OLD, NEW = 'e65001ad50072d3399bf204da02055b0f360353a', '5b1ccd356c0d9656f7b9f42c6558f7a4c73f9295'
P = 'projects/chirality-piping'
TAURI = 'apps/desktop/src-tauri/Cargo.toml'
LEAF = 'core/reporting/report_package/Cargo.lock'
TOOL = '+1.97.1'
out.mkdir(parents=True, exist_ok=True)
env = os.environ.copy()
for k in ('CARGO_NET_OFFLINE', 'RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'CARGO_TARGET_DIR'):
    env.pop(k, None)
env.update(CARGO_TERM_COLOR='never', CARGO_BUILD_JOBS='2')


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest() if p.exists() else None


def cargo(project, args, log, target):
    e = dict(env, CARGO_TARGET_DIR=str(target))
    r = subprocess.run(['cargo', TOOL, *args], cwd=project, env=e, capture_output=True, text=True)
    log.write_text(r.stdout + ('\n--- stderr ---\n' + r.stderr if r.stderr else ''))
    return r


def normalise(text, project_root):
    return text.replace(project_root.resolve().as_uri(), 'file://<ROOT>').replace(str(project_root.resolve()), '<ROOT>')


def resolve_of(project, tag):
    lock_tauri, lock_leaf = project / 'apps/desktop/src-tauri/Cargo.lock', project / LEAF
    before = dict(src_tauri=sha(lock_tauri), report_package=sha(lock_leaf))
    r = cargo(project, ['metadata', '--locked', '--offline', '--format-version', '1', '--manifest-path', TAURI],
              out / f'metadata-{tag}.log', tbase / '_nobuild')
    tree = cargo(project, ['tree', '--locked', '--offline', '--target', 'aarch64-apple-darwin', '-e', 'normal,build',
                           '--prefix', 'depth', '--manifest-path', TAURI], out / f'tree-macos-{tag}.log', tbase / '_nobuild')
    after = dict(src_tauri=sha(lock_tauri), report_package=sha(lock_leaf))
    row = dict(tag=tag, metadata_exit=r.returncode, tree_exit=tree.returncode, locks_before=before, locks_after=after)
    if r.returncode == 0:
        meta = json.loads(r.stdout)
        res = json.dumps(meta['resolve'], sort_keys=True)
        res_n = normalise(res, project / '..' / '..')
        (out / f'resolve-{tag}.normalised.json').write_text(res_n + '\n')
        row.update(resolve_nodes=len(meta['resolve']['nodes']), packages=len(meta['packages']),
                   resolve_sha256=hashlib.sha256(res_n.encode()).hexdigest(),
                   workspace_root=normalise(meta['workspace_root'], project / '..' / '..'),
                   report_package_in_resolve=any('open_pipe_stress_report_package' in n['id'] for n in meta['resolve']['nodes']))
    if tree.returncode == 0:
        t_n = normalise(tree.stdout, project / '..' / '..')
        (out / f'tree-macos-{tag}.normalised.log').write_text(t_n)
        row['tree_macos_sha256'] = hashlib.sha256(t_n.encode()).hexdigest()
        row['tree_macos_lines'] = len(t_n.splitlines())
    return row


results = {'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=root, text=True).strip()}
# A. real worktree
results['A_worktree_5b1c'] = resolve_of(root / P, 'worktree-5b1c')

# B. scratch export variants
if scratch.exists():
    shutil.rmtree(scratch)
scratch.mkdir(parents=True)
arch = subprocess.run(['git', 'archive', '--format=tar', NEW, f'{P}/core', f'{P}/apps/desktop/src-tauri', f'{P}/fixtures', f'{P}/schemas', f'{P}/validation'],
                      cwd=root, capture_output=True, check=True).stdout
subprocess.run(['tar', '-x', '-C', str(scratch)], input=arch, check=True)
sp = scratch / P
leaf = sp / LEAF
bytes_new = subprocess.check_output(['git', 'show', f'{NEW}:{P}/{LEAF}'], cwd=root)
bytes_old = subprocess.check_output(['git', 'show', f'{OLD}:{P}/{LEAF}'], cwd=root)
results['leaf_lock_sha256'] = dict(e650=hashlib.sha256(bytes_old).hexdigest(), b5b1c=hashlib.sha256(bytes_new).hexdigest())
variants = []
for tag, content in (('leaf-5b1c', bytes_new), ('leaf-e650', bytes_old), ('leaf-absent', None),
                     ('leaf-invalid-toml', b'this is [[[ not a lockfile\n')):
    if leaf.exists():
        leaf.unlink()
    if content is not None:
        leaf.write_bytes(content)
    row = resolve_of(sp, tag)
    row['leaf_bytes_after_equal_written'] = (leaf.read_bytes() == content) if content is not None else (not leaf.exists())
    variants.append(row)
results['B_scratch_variants'] = variants
hashes = {v['tag']: v.get('resolve_sha256') for v in variants}
hashes['worktree-5b1c'] = results['A_worktree_5b1c'].get('resolve_sha256')
trees = {v['tag']: v.get('tree_macos_sha256') for v in variants}
trees['worktree-5b1c'] = results['A_worktree_5b1c'].get('tree_macos_sha256')
results['resolve_identical_all'] = len(set(hashes.values())) == 1 and None not in hashes.values()
results['tree_macos_identical_all'] = len(set(trees.values())) == 1 and None not in trees.values()
results['resolve_hashes'] = hashes
results['tree_macos_hashes'] = trees

# C. compile report_package inside the src-tauri resolve
leaf.write_bytes(bytes_new)
t1, t2 = tbase / 'src-tauri-report_package-1', tbase / 'src-tauri-report_package-2'
for t in (t1, t2):
    if t.exists():
        shutil.rmtree(t)
build = ['build', '--locked', '--offline', '-j', '2', '-p', 'open_pipe_stress_report_package', '--manifest-path', TAURI]
c = {}
r1 = cargo(sp, build, out / 'build-1-leaf-5b1c.log', t1)
c['build1_leaf_5b1c_exit'] = r1.returncode
leaf.write_bytes(bytes_old)
os.utime(leaf, None)
r2 = cargo(sp, build + ['-v'], out / 'build-1-rebuild-after-swap-to-leaf-e650.log', t1)
c['rebuild_after_swap_exit'] = r2.returncode
lines = [l.strip() for l in r2.stderr.splitlines() if l.strip().startswith(('Fresh', 'Compiling', 'Dirty', 'Running'))]
c['rebuild_units_fresh'] = sum(l.startswith('Fresh') for l in lines)
c['rebuild_units_compiled_or_dirty'] = [l for l in lines if not l.startswith('Fresh')]
r3 = cargo(sp, build, out / 'build-2-clean-leaf-e650.log', t2)
c['build2_clean_leaf_e650_exit'] = r3.returncode


def outputs(t):
    d = t / 'debug' / 'deps'
    return {p.name: sha(p) for p in sorted(d.glob('libopen_pipe_stress_report_package-*')) if p.suffix in ('.rlib', '.rmeta')}


c['outputs_build1'] = outputs(t1)
c['outputs_build2'] = outputs(t2)
c['report_package_outputs_identical'] = bool(c['outputs_build1']) and c['outputs_build1'] == c['outputs_build2']
c['src_tauri_lock_sha256_after'] = sha(sp / 'apps/desktop/src-tauri/Cargo.lock')
c['src_tauri_lock_sha256_repo_5b1c'] = sha(root / P / 'apps/desktop/src-tauri/Cargo.lock')
results['C_compile'] = c
(out / 'SRC_TAURI_RESOLVE.json').write_text(json.dumps(results, indent=2) + '\n')
print(json.dumps({k: results[k] for k in ('resolve_identical_all', 'tree_macos_identical_all', 'resolve_hashes')}, indent=2))
print(json.dumps(c, indent=2))
