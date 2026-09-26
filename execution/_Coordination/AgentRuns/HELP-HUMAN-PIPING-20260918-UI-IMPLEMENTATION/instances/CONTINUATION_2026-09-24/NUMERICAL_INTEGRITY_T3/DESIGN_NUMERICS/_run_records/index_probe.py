"""Read-only crates.io sparse-index probe: latest non-yanked version of each
candidate arithmetic crate, its declared rust-version, its features, and the
transitive set of non-optional normal/build dependencies (latest compatible
approximated by latest non-yanked version satisfying a caret major match).
No crate is downloaded or built."""
import json, sys, urllib.request

def path(name):
    n = name.lower()
    if len(n) == 1: return f"1/{n}"
    if len(n) == 2: return f"2/{n}"
    if len(n) == 3: return f"3/{n[0]}/{n}"
    return f"{n[:2]}/{n[2:4]}/{n}"

cache = {}
def versions(name):
    if name not in cache:
        with urllib.request.urlopen(f"https://index.crates.io/{path(name)}", timeout=30) as r:
            cache[name] = [json.loads(l) for l in r.read().decode().splitlines() if l.strip()]
    return cache[name]

def parse(v):
    core = v.split('+')[0]
    pre = '-' in core
    nums = core.split('-')[0].split('.')
    return tuple(int(x) for x in nums), pre

def caret_ok(req, ver):
    (vn, pre) = parse(ver)
    if pre: return False
    req = req.strip()
    for part in req.split(','):
        part = part.strip()
        if part.startswith('^') or part[0].isdigit():
            base = part.lstrip('^')
            bn = tuple(int(x) for x in base.split('.') if x.isdigit())
            bn = bn + (0,)*(3-len(bn))
            if bn[0] > 0:
                if vn[0] != bn[0] or vn < bn: return False
            elif bn[1] > 0:
                if vn[0] != 0 or vn[1] != bn[1] or vn < bn: return False
            else:
                if vn[:3] != bn[:3] and len(base.split('.'))>=3: return False
                if vn[0]!=0 or vn[1]!=0: return False
        elif part.startswith('>='):
            bn = tuple(int(x) for x in part[2:].strip().split('.')); bn = bn+(0,)*(3-len(bn))
            if vn < bn: return False
        elif part.startswith('<'):
            bn = tuple(int(x) for x in part[1:].strip().split('.')); bn = bn+(0,)*(3-len(bn))
            if vn >= bn: return False
        elif part.startswith('~'):
            bn = tuple(int(x) for x in part[1:].strip().split('.')); bn=bn+(0,)*(3-len(bn))
            if vn[:2] != bn[:2] or vn < bn: return False
        elif part.startswith('='):
            if part[1:].strip() != ver: return False
    return True

def pick(name, req=None):
    vs = [v for v in versions(name) if not v.get('yanked')]
    if req: vs = [v for v in vs if caret_ok(req, v['vers'])]
    vs = [v for v in vs if not parse(v['vers'])[1]]
    return max(vs, key=lambda v: parse(v['vers'])[0]) if vs else None

def closure(name, req=None, default_features=True, features=(), seen=None, target_filter=True):
    seen = {} if seen is None else seen
    v = pick(name, req)
    if v is None: return seen
    key = v['name']
    if key in seen: return seen
    seen[key] = v['vers']
    feats = dict(v.get('features', {})); feats.update(v.get('features2', {}) or {})
    enabled = set(features) | ({'default'} if default_features else set())
    changed = True
    while changed:
        changed = False
        for f in list(enabled):
            for g in feats.get(f, []):
                g2 = g.replace('dep:', '')
                if '/' in g2 or '?' in g2: g2 = g2.split('/')[0].rstrip('?')
                if g2 not in enabled: enabled.add(g2); changed = True
    for d in v['deps']:
        if d['kind'] == 'dev': continue
        dname = d.get('package') or d['name']
        if d['optional'] and d['name'] not in enabled: continue
        if target_filter and d.get('target') and 'windows' in d['target'] and 'not' not in d['target']: continue
        if target_filter and d.get('target') and 'wasm' in d['target']: continue
        closure(dname, d['req'], d['default_features'], d['features'], seen)
    return seen

cands = [
    ("rug", ()), ("rug", ("float",)),
    ("gmp-mpfr-sys", ()),
    ("dashu-float", ()), ("dashu-base", ()), ("dashu-int", ()),
    ("astro-float", ()),
    ("malachite-float", ()),
    ("num-bigint", ()), ("num-rational", ()),
    ("twofloat", ()),
]
out = {}
for name, feats in cands:
    v = pick(name)
    df = True
    if name == "rug" and feats:
        df = False
    cl = closure(name, None, df, feats)
    out[f"{name}{'['+','.join(feats)+']' if feats else ''}"] = {
        "latest": v['vers'], "rust_version": v.get('rust_version'),
        "default_features": v.get('features', {}).get('default'),
        "links": v.get('links'),
        "transitive_nonoptional_crates": dict(sorted(cl.items())),
        "count_excluding_self": len(cl) - 1,
    }
print(json.dumps(out, indent=1))
