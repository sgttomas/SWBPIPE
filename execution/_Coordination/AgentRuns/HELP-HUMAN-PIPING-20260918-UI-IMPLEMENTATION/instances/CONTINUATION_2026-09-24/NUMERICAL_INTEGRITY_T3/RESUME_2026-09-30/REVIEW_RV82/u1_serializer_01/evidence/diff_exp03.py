#!/usr/bin/env python3
"""RV82 item 3: structural diff of the serializer's milestone files against experiment 03's."""
import json, os, sys
OUT, EXP = os.environ["OUT"], os.environ["EXP"]
def walk(a, b, p, out):
    if type(a) != type(b): out.append((p, a, b)); return
    if isinstance(a, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a or k not in b: out.append((p + "/" + k, a.get(k, "<absent>"), b.get(k, "<absent>")))
            else: walk(a[k], b[k], p + "/" + k, out)
    elif isinstance(a, list):
        if len(a) != len(b): out.append((p + "#len", len(a), len(b)))
        for i, (x, y) in enumerate(zip(a, b)): walk(x, y, f"{p}[{i}]", out)
    elif a != b: out.append((p, a, b))
for m in ["sparse_interactive", "dense_scrutiny"]:
    e = json.load(open(f"{EXP}/milestone_{m}.json")); u = json.load(open(f"{OUT}/u1_milestone_{m}.json"))
    out = []; walk(e, u, "", out)
    print(f"== {m}: {len(out)} differing leaves (experiment 03 -> U1)")
    for p, a, b in out:
        print(f"  {p}\n     exp03: {json.dumps(a)[:240]}\n     u1:    {json.dumps(b)[:240]}")
