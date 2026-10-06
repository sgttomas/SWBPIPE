"""I71: the wasm engines' path-dependency closure (Cargo.toml path deps, read with tomllib; no cargo run),
from P's two wasm crates (apps/desktop/scripts/build-wasm-engine.mjs `builds`). Run with cwd = a P tree."""
import os, tomllib, sys
root = os.getcwd()
def deps(d):
    t = tomllib.load(open(os.path.join(d, 'Cargo.toml'), 'rb'))
    secs = [t.get('dependencies', {}), t.get('build-dependencies', {})] + [v.get('dependencies', {}) for v in t.get('target', {}).values()]
    out = []
    for s in secs:
        for name, v in s.items():
            if isinstance(v, dict) and 'path' in v: out.append(os.path.normpath(os.path.join(d, v['path'])))
            elif isinstance(v, dict) and v.get('workspace'): sys.exit(f'workspace dependency {name} in {d}: not handled')
    return out
seen, stack = set(), [os.path.join(root, 'core/model_operations/operation_applier'), os.path.join(root, 'core/loads/self_weight_wasm')]
while stack:
    d = stack.pop()
    if d in seen: continue
    seen.add(d); stack.extend(deps(d))
for s in sorted(seen): print(os.path.relpath(s, root))
