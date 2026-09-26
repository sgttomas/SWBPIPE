#!/usr/bin/env python3
"""D1 D-5: product condition estimates published in committed fixtures (standard library only).

Usage: python3 scan_rcond.py <fixtures-root> [<label>]   (reads files; prints JSON)
Collects, from every JSON file under the root, each rendered StructuralReport's
`reciprocal_condition_estimate` (main's Hager-Higham estimate on the radix-equilibrated
reduced matrix), `quality`, and the largest intended-action `guarded_ratio`, keyed by file and
by the load case named in the enclosing diagnostic message. It then evaluates the forward-error
figure fe = (1/rcond) * (max guarded_ratio + u) that P1 uses, against 1e-9.
"""
import json
import os
import re
import sys

U = 2.0 ** -53
PAT = re.compile(r"reciprocal_condition_estimate: ([-0-9.e+]+)")


def walk(o, path, out, fname):
    if isinstance(o, dict):
        for k, v in o.items():
            walk(v, path + [k], out, fname)
    elif isinstance(o, list):
        for i, v in enumerate(o):
            walk(v, path + [str(i)], out, fname)
    elif isinstance(o, str) and "reciprocal_condition_estimate" in o:
        for m in PAT.finditer(o):
            head = o[:m.start()]
            start = head.rfind("StructuralReport {")
            case = None
            for cm in re.finditer(r"load case (\S+?)(?::| |,)", head[max(0, start - 400):start + 1]):
                case = cm
            seg = o[m.start():m.start() + 200000]
            end = seg.find("symmetry_basis")
            seg = seg[: end if end > 0 else len(seg)]
            q = re.search(r"quality: (\w+)", o[start:m.start()]) if start >= 0 else None
            gr = [float(x) for x in re.findall(r"guarded_ratio: ([-0-9.e+]+)", seg)]
            rc = float(m.group(1))
            out.append({"file": fname, "case": case.group(1) if case else None,
                        "quality": q.group(1) if q else None, "rcond": rc,
                        "max_guarded_ratio": max(gr) if gr else None,
                        "fe": (1.0 / rc) * ((max(gr) if gr else 0.0) + U) if rc > 0 else None})


def main():
    root = sys.argv[1]
    out = []
    for dp, dn, fn in os.walk(root):
        for f in sorted(fn):
            if f.endswith(".json"):
                p = os.path.join(dp, f)
                try:
                    d = json.load(open(p))
                except Exception:
                    continue
                walk(d, [], out, os.path.relpath(p, root))
    print(json.dumps(out, indent=0))


if __name__ == "__main__":
    main()
