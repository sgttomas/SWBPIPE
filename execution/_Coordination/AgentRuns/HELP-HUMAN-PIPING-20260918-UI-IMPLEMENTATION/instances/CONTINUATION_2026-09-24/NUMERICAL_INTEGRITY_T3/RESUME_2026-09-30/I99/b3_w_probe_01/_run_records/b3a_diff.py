"""I99 B3-W item 3 (B3a's check).
Part 1: the ordinary bytes of the milestone authored as 0.3.0 `legacy_pressure_v1` with zero
pressure (label b) against the 0.1.0 milestone's (label a), per mode: byte equality, then every
JSON path whose value differs, with both values.
Part 2: the W1 successor the private driver built for label b (refused at precommit; its
`source` is in the probe's precommit dump) against the 0.1.0 milestone's published successor.
usage: b3a_diff.py <out dir> <label a> <label b>
Reads plain_<label>_<mode>.json, successor_<a>_<mode>.json and precommit_refused_<mode>_*.json
(all written by the probe)."""
import glob, hashlib, json, sys, pathlib

out, la, lb = pathlib.Path(sys.argv[1]), sys.argv[2], sys.argv[3]

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

def show(diffs):
    print(f"   JSON paths that differ: {len(diffs)}")
    for p, x, y in diffs:
        print(f"   {p}:\n     {la}: {json.dumps(x)}\n     {lb}: {json.dumps(y)}")

print("Part 1: the ordinary route's bytes")
for mode in ["sparse_interactive", "dense_scrutiny"]:
    A = (out / f"plain_{la}_{mode}.json").read_bytes()
    B = (out / f"plain_{lb}_{mode}.json").read_bytes()
    print(f"== {mode}: {la} sha256 {hashlib.sha256(A).hexdigest()} ({len(A)} B); {lb} sha256 {hashlib.sha256(B).hexdigest()} ({len(B)} B); bytes equal: {A == B}")
    diffs = []
    walk(json.loads(A), json.loads(B), "$", diffs)
    show(diffs)
print()
print(f"Part 2: the W1 successor built for {lb} by the private driver (refused at precommit) against {la}'s published successor")
for mode in ["sparse_interactive", "dense_scrutiny"]:
    A = (out / f"successor_{la}_{mode}.json").read_bytes()
    dumps = sorted(glob.glob(str(out / f"precommit_refused_{mode}_*.json")))
    assert len(dumps) == 1, dumps
    dump = json.loads(pathlib.Path(dumps[0]).read_bytes())
    B = dump["source"]
    print(f"== {mode}: {la} successor sha256 {hashlib.sha256(A).hexdigest()}; {lb}'s from {pathlib.Path(dumps[0]).name}; invocation schema_version {dump['invocation']['request']['model']['schema_version']}, pressure_contract {json.dumps(dump['invocation']['request']['model'].get('pressure_contract'))}")
    diffs = []
    walk(json.loads(A), B, "$", diffs)
    show(diffs)
