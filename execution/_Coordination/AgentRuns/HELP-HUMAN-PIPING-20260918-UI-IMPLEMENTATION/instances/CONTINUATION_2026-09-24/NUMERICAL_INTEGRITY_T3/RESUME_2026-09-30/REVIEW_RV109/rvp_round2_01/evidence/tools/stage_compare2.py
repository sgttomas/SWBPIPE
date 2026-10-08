#!/usr/bin/env python3
"""RV109 round 2: compare the c = 1 stage-count rows of I1 and SP's head, field by field.

Usage: stage_compare2.py <stage_i1.jsonl> <stage_head.jsonl>
Every field the probe records is compared: the adapter counts after the run, preparation, native,
the candidate (T-9), staging and serialization; each stage's outcome; the attempt's terminal
snapshot; the staged summary; the successor's sha256; and the accepted reader's verdict.
"""
import json
import sys


def load(path):
    rows = {}
    for line in open(path, encoding="utf-8"):
        r = json.loads(line)
        rows[(r["input"], r["mode"])] = r
    return rows


def main():
    a, b = load(sys.argv[1]), load(sys.argv[2])
    keys = sorted(set(a) | set(b))
    same, diff = 0, []
    fields = set()
    for k in keys:
        x, y = a.get(k), b.get(k)
        if x is None or y is None:
            diff.append((k, ["missing on one side"]))
            continue
        fields |= set(x) | set(y)
        d = [f for f in sorted(set(x) | set(y)) if f != "cases_seen" and x.get(f) != y.get(f)]
        if d:
            diff.append((k, d))
        else:
            same += 1
    print(f"c=1 rows: I1 {len(a)}, head {len(b)}; identical in every compared field: {same}; differing: {len(diff)}")
    print("fields compared: " + ", ".join(sorted(fields - {'input', 'mode'})))
    for k, d in diff:
        print(f"  DIFF {k[0]} [{k[1]}]: {d}")
        x, y = a.get(k), b.get(k)
        if x and y:
            for f in d:
                print(f"    {f}: I1={json.dumps(x.get(f))[:300]} head={json.dumps(y.get(f))[:300]}")
    reached = {}
    for k in keys:
        r = b.get(k) or {}
        stage = "successor" if r.get("serializer") == "ok" else (r.get("serializer") and "serializer") or (r.get("staging") and "staging") \
            or (r.get("candidate") and "candidate") or (r.get("native") and "native") or "preparation"
        reached[stage] = reached.get(stage, 0) + 1
    print("head rows by furthest stage reached: " + json.dumps(reached, sort_keys=True))


if __name__ == "__main__":
    main()
