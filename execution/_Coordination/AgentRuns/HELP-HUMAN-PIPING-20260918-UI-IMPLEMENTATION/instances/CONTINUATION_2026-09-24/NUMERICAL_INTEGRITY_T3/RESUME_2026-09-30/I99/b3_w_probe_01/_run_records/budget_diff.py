"""I99 B3-W item 4 (P-2's budget parity, a note): for each input and mode, the JSON paths whose
values differ between the captured ordinary run at 4,000,000 per case (permitted_run's
`SourceRecoveryBudget::default()`) and at 8,000,000 (`ordinary_dispatch`'s exact-route
`PHYSICS_SOURCE_WORK_LIMIT`), from the probe's budget_<limit>_<label>_<mode>.json files.
usage: budget_diff.py <out dir> <label>...   (values cut to 160 characters; at most 12 paths)"""
import hashlib, json, sys, pathlib
out = pathlib.Path(sys.argv[1])

def walk(a, b, path, diffs):
    if type(a) != type(b):
        diffs.append((path, a, b)); return
    if isinstance(a, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a or k not in b:
                diffs.append((f"{path}.{k}", a.get(k, "<absent>"), b.get(k, "<absent>")))
            else:
                walk(a[k], b[k], f"{path}.{k}", diffs)
    elif isinstance(a, list):
        if len(a) != len(b):
            diffs.append((f"{path}[len]", len(a), len(b)))
        for i, (x, y) in enumerate(zip(a, b)):
            walk(x, y, f"{path}[{i}]", diffs)
    elif a != b:
        diffs.append((path, a, b))

for label in sys.argv[2:]:
    for mode in ["sparse_interactive", "dense_scrutiny"]:
        A = (out / f"budget_4000000_{label}_{mode}.json").read_bytes()
        B = (out / f"budget_8000000_{label}_{mode}.json").read_bytes()
        a, b = json.loads(A), json.loads(B)
        diffs = []
        walk(a, b, "$", diffs)
        print(f"== {label} {mode}: 4M {hashlib.sha256(A).hexdigest()[:12]} ({len(A)} B; {a['producer']['semantic_contract_id'].rsplit('/',1)[1]}; {a['status']['mechanics']}; {len(a['results'])} results) vs 8M {hashlib.sha256(B).hexdigest()[:12]} ({len(B)} B; {b['producer']['semantic_contract_id'].rsplit('/',1)[1]}; {b['status']['mechanics']}; {len(b['results'])} results): {len(diffs)} differing paths")
        for p, x, y in diffs[:12]:
            print(f"   {p}: {json.dumps(x)[:160]} -> {json.dumps(y)[:160]}")
        if len(diffs) > 12:
            print(f"   ... {len(diffs) - 12} more")
