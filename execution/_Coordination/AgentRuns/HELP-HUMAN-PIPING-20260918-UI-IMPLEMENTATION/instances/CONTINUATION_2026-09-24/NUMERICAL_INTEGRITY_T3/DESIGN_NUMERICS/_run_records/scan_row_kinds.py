#!/usr/bin/env python3
"""D1 DESIGN rev 4 (R3B-2): result row kinds published in committed envelopes, by semantics id.

Usage: python3 scan_row_kinds.py <fixtures-root> [<more roots>...]
For every JSON object that has a `results` list of row objects with `kind`, records the kind,
its unit(s), whether its value is numeric, and the enclosing envelope's result-semantics id
(the first string value under a key containing 'semantics' found walking up, else 'unknown').
Standard library only.
"""
import json
import os
import sys
from collections import defaultdict


def sem_of(o):
    for k, v in o.items():
        if "semantic" in k and isinstance(v, str):
            return v
        if "semantic" in k and isinstance(v, dict):
            for kk, vv in v.items():
                if isinstance(vv, str) and "result_semantics" in vv:
                    return vv
    return None


def walk(o, sem, out):
    if isinstance(o, dict):
        s = sem_of(o) or sem
        rs = o.get("results")
        if isinstance(rs, list):
            for r in rs:
                if isinstance(r, dict) and "kind" in r:
                    v = r.get("value")
                    q = r.get("quantity") if isinstance(r.get("quantity"), dict) else {}
                    unit = r.get("unit") or q.get("unit")
                    out[(s or "unknown", r["kind"])].add((str(unit), type(v if v is not None else q.get("value")).__name__))
        for v in o.values():
            walk(v, s, out)
    elif isinstance(o, list):
        for v in o:
            walk(v, sem, out)


def main():
    out = defaultdict(set)
    for root in sys.argv[1:]:
        for dp, dn, fn in os.walk(root):
            for f in fn:
                if f.endswith(".json"):
                    try:
                        walk(json.load(open(os.path.join(dp, f))), None, out)
                    except Exception:
                        pass
    rows = sorted((s, k, sorted(v)) for (s, k), v in out.items())
    print(json.dumps(rows, indent=0))


if __name__ == "__main__":
    main()
