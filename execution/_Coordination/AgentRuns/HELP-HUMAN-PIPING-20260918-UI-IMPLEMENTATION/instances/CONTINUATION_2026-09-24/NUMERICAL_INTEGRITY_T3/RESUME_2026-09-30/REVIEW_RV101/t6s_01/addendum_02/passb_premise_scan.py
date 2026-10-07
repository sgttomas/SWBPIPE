"""RV101 ADDENDUM_02: Pass B's premise at the PR head, by Git reads only (GIT_OPTIONAL_LOCKS=0).
Usage: passb_premise_scan.py <repo> <main> <head> <crate_dirs.txt>"""
import json, os, posixpath, re, subprocess, sys
repo, main, head, dirs_file = sys.argv[1:5]
env = dict(os.environ, GIT_OPTIONAL_LOCKS='0')
g = lambda *a: subprocess.run(['git', '-C', repo, *a], capture_output=True, text=True, env=env, check=True).stdout
P = 'projects/chirality-piping/'
changed = [l for l in g('diff', '--name-only', main, head).split('\n') if l]
status = [l for l in g('diff', '--name-status', main, head).split('\n') if l]
crate_src = [P + d for d in open(dirs_file).read().split()]
crate_roots = [d[:-len('/src')] for d in crate_src]
out = {'changed_paths': len(changed), 'statuses': sorted({s.split('\t')[0] for s in status})}
out['changed_in_crate_src'] = [c for c in changed if any(c.startswith(d + '/') for d in crate_src)]
out['changed_under_crate_roots'] = [c for c in changed if any(c.startswith(r + '/') for r in crate_roots)]
names = ('Cargo.toml', 'Cargo.lock', 'build.rs', 'package.json', 'package-lock.json', 'pyproject.toml', 'requirements-dev.txt', 'software-workflow.json', 'rust-toolchain.toml', 'tsconfig.json', 'vite.config.ts')
out['changed_manifests_locks_build'] = [c for c in changed if posixpath.basename(c) in names]
# embedded statics in the D1 crate sources and PP's build.rs
files = []
for d in crate_src:
    files += [l for l in g('ls-tree', '-r', '--name-only', head, '--', d).split('\n') if l.endswith('.rs')]
files.append(P + 'core/product_physics/build.rs')
pat = re.compile(r'include(?:_str|_bytes)?!\s*\(\s*"([^"]+)"\s*\)')
targets = {}
for f in files:
    try: text = g('show', f'{head}:{f}')
    except subprocess.CalledProcessError: continue
    for m in pat.finditer(text):
        t = posixpath.normpath(posixpath.join(posixpath.dirname(f), m.group(1)))
        targets.setdefault(t, 0); targets[t] += 1
out['include_literals'] = sum(targets.values()); out['include_targets'] = len(targets)
out['embedded_changed'] = sorted(t for t in targets if t in changed)
def blob(rev, p):
    try: return g('rev-parse', f'{rev}:{p}').strip()
    except subprocess.CalledProcessError: return None
out['embedded_blob_differs_main_vs_head'] = sorted(t for t in targets if blob(main, t) != blob(head, t))
# PP's reviewed inputs
bi = g('show', f'{head}:{P}core/product_physics/src/build_identity.rs')
m = re.search(r'REVIEWED_INPUTS[^=]*=\s*&?\[(.*?)\];', bi, re.S)
rev = re.findall(r'"([^"]+)"', m.group(1)) if m else []
out['reviewed_inputs_listed'] = len(rev)
out['reviewed_inputs_touched'] = [r for r in rev if any(c.endswith(r.lstrip('./')) or r.lstrip('./').endswith(c[len(P):]) for c in changed)]
# readers and carriers
readers = [P + 'core/reporting/result_export/src/' + n for n in ('retained_precision.rs', 'derivative.rs', 'semantic_contract.rs', 'source_blocks.rs')] + \
          [P + 'apps/desktop/src/features/results/' + n for n in ('retainedPrecision.ts', 'retainedPrecisionStanding.ts')]
out['reader_files_changed'] = [r for r in readers if r in changed]
out['python_readers_changed'] = [c for c in changed if c.startswith(P + 'core/analysis_runs/') or c.startswith(P + 'core/handoff/stress_neutral/')]
out['pp_changed'] = [c for c in changed if c.startswith(P + 'core/product_physics/')]
out['src_tauri_or_e2e_plan_changed'] = [c for c in changed if '/src-tauri/' in c or c.endswith('tools/ci/e2e_plan.py')]
out['schemas_changed'] = [c[len(P):] for c in changed if c.startswith(P + 'schemas/')]
# the dispatcher: everything but oneOf[2] and the description value-identical; $defs identical
a = json.loads(g('show', f'{main}:{P}schemas/results.schema.yaml')); b = json.loads(g('show', f'{head}:{P}schemas/results.schema.yaml'))
out['dispatcher_defs_identical'] = a['$defs'] == b['$defs']
out['dispatcher_other_keys_identical'] = all(a[k] == b[k] for k in a if k not in ('oneOf', 'description')) and set(a) == set(b)
out['dispatcher_oneOf_0_1_identical'] = a['oneOf'][:2] == b['oneOf'][:2]
out['dispatcher_oneOf_2_head'] = b['oneOf'][2]
out['version_file_blob_equal'] = blob(main, P + 'schemas/results.v0.3.schema.yaml') == blob(head, P + 'schemas/results.v0.3.schema.yaml')
print(json.dumps(out, indent=1))
