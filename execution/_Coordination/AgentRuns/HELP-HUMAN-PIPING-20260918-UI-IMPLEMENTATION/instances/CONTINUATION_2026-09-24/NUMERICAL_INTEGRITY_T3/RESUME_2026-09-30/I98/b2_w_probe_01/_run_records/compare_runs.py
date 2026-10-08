"""I98 B2-W: the determinism control (after I86's compare_runs.py).

Usage: python compare_runs.py <round-1 out dir> <round-2 out dir> <run1.log> <run2.log> [<run1.log> <run2.log>...]
1. Probe lines (I98_*) of each pair of logs, in order, with timing tokens (ms=, direct_ms=)
   and the wall-clock I98_MARK lines removed: every other line must be identical.
2. The output files both rounds wrote (successors, plain envelopes, precommit dumps):
   byte-identical by sha256.
"""
import hashlib
import os
import re
import sys

TIMING = re.compile(r" (?:ms|direct_ms|elapsed_ms)=\d+")


def probe_lines(path):
    out = []
    for line in open(path, encoding="utf-8", errors="replace"):
        line = line.rstrip("\n")
        if line.startswith("test "):
            if " I98_" not in line:
                continue
            line = line[line.index(" I98_") + 1:]
        if not line.startswith("I98_") or line.startswith("I98_MARK "):
            continue
        out.append(TIMING.sub("", line))
    return out


def sha(path):
    with open(path, "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()


def main():
    d1, d2, logs = sys.argv[1], sys.argv[2], sys.argv[3:]
    bad = 0
    for x, y in zip(logs[::2], logs[1::2]):
        a, b = probe_lines(x), probe_lines(y)
        same = a == b
        bad += not same
        print(f"{os.path.basename(x)} vs {os.path.basename(y)}: lines {len(a)} / {len(b)} identical={same}")
        if not same:
            for i, (p, q) in enumerate(zip(a, b)):
                if p != q:
                    print(f"  first difference at probe line {i}:\n    {p[:300]}\n    {q[:300]}")
                    break
    names = sorted(set(os.listdir(d1)) & set(os.listdir(d2)))
    # Only the files the probe rounds write (the builtin dumps were written once, before round 1).
    names = [n for n in names if n.endswith(".json") and n.startswith(("successor_", "plain_", "precommit_refused_"))]
    diff = [n for n in names if sha(os.path.join(d1, n)) != sha(os.path.join(d2, n))]
    bad += len(diff)
    print(f"output files written in both rounds: {len(names)}; byte-identical: {len(names) - len(diff)}; different: {diff}")
    only1 = sorted(n for n in set(os.listdir(d1)) - set(os.listdir(d2)) if n.endswith(".json"))
    print(f"round-1 files compared: {len(names)} of {len([n for n in os.listdir(d1) if n.startswith(('successor_', 'plain_', 'precommit_refused_'))])}")
    print(f"files only in round 1: {only1}")
    print(f"DETERMINISM differences={bad}")


if __name__ == "__main__":
    main()
