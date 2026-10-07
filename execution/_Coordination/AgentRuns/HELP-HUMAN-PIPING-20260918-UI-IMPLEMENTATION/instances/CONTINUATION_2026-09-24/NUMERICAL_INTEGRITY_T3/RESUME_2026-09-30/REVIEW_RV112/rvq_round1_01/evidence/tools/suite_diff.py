#!/usr/bin/env python3
"""RV112: test-by-test comparison of two cargo test logs.

Usage: suite_diff.py <base.log> <cand.log> [<target>]   (with <target>, only that test target)
Groups `test <name> ... <outcome>` lines by their `Running <target>` header (hash stripped) and
prints per-target counts and every test whose presence or outcome differs, then TOTAL_DIFFERENCES.
"""
import re, sys

RUN = re.compile(r"^\s+Running (?:unittests )?(\S+)")
DOC = re.compile(r"^\s+Doc-tests (\S+)")
TEST = re.compile(r"^test (\S+)(?: - .*?)? \.\.\. (ok|FAILED|ignored(?:, .*)?)")


def parse(path):
    out, target = {}, None
    for line in open(path, encoding="utf-8", errors="replace"):
        line = line.rstrip("\n")
        m = RUN.match(line)
        if m:
            target = re.sub(r"-[0-9a-f]{16}\)?$", "", m.group(1).split("/")[-1]).rstrip(")")
            continue
        m = DOC.match(line)
        if m:
            target = "doctests:" + m.group(1)
            continue
        m = TEST.match(line)
        if m and target is not None:
            out.setdefault(target, {})[m.group(1)] = "ignored" if m.group(2).startswith("ignored") else m.group(2)
    return out


def counts(d):
    return {k: sum(1 for v in d.values() if v == k) for k in ("ok", "FAILED", "ignored")}


def main():
    base, cand = parse(sys.argv[1]), parse(sys.argv[2])
    if len(sys.argv) > 3:
        base = {k: v for k, v in base.items() if k == sys.argv[3]}
        cand = {k: v for k, v in cand.items() if k == sys.argv[3]}
    total = 0
    tb, tc = {"ok": 0, "FAILED": 0, "ignored": 0}, {"ok": 0, "FAILED": 0, "ignored": 0}
    for t in sorted(set(base) | set(cand)):
        b, c = base.get(t, {}), cand.get(t, {})
        cb, cc = counts(b), counts(c)
        for k in tb:
            tb[k] += cb[k]
            tc[k] += cc[k]
        print(f"{t}: base {cb} cand {cc}")
        for name in sorted(set(b) | set(c)):
            if b.get(name) != c.get(name):
                total += 1
                print(f"  DIFF {name}: base={b.get(name)} cand={c.get(name)}")
    print(f"ALL TARGETS: base {tb} cand {tc}")
    print(f"TOTAL_DIFFERENCES {total}")


if __name__ == "__main__":
    main()
