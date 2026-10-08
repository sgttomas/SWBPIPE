"""I98 B2-W, W-CB3: does B_v change A's numerics? (read-only; VENV)

Usage: python cb3_rows.py <out dir> <variant>...
Compares the published successor rows of the combination proxy (`cb3_<v>_ab`) with the
selected operand alone (`cb3_a`, U8's L = 0 base), keyed by (kind, entity_ref, location,
component), per mode: which rows differ in value or recovery method, and by entity.
"""
import collections
import json
import os
import sys

MODES = ["sparse_interactive", "dense_scrutiny"]


def rows(path):
    with open(path, "rb") as f:
        doc = json.loads(f.read())
    out = {}
    for r in doc["results"]:
        m = r.get("metadata") or {}
        out[(r["kind"], r["entity_ref"], m.get("location"), m.get("component"))] = (r["value"], r.get("recovery_method"))
    return out, doc


def main():
    out_dir, variants = sys.argv[1], sys.argv[2:]
    for v in variants:
        for mode in MODES:
            a, _ = rows(os.path.join(out_dir, f"successor_cb3_a_{mode}.json"))
            ab, doc = rows(os.path.join(out_dir, f"successor_cb3_{v}_ab_{mode}.json"))
            same_keys = set(a) == set(ab)
            differ = sorted(k for k in a if k in ab and a[k] != ab[k])
            by_entity = collections.Counter(k[1] for k in differ)
            method = collections.Counter(x[1] for x in ab.values())
            print(f"CB3_ROWS {v} {mode} rows={len(ab)} same_row_keys={same_keys} rows_differing={len(differ)} by_entity={dict(by_entity)} "
                  f"kinds_differing={sorted({k[0] for k in differ})} recovery_methods={dict(method)}")
            for k in differ[:12]:
                print(f"  {k} A={a[k][0]!r} A+B={ab[k][0]!r}")


if __name__ == "__main__":
    main()
