#!/usr/bin/env python3
"""RV109: test-by-test comparison of two cargo test logs (base vs candidate).

Usage: suite_diff.py <base.log> <cand.log>
Groups `test <name> ... <outcome>` lines by the `Running <target>` header (hash stripped),
then prints per-target counts and every test whose presence or outcome differs.
"""
import re
import sys

RUN = re.compile(r"^\s+Running (?:unittests )?(\S+)")
DOC = re.compile(r"^\s+Doc-tests (\S+)")
TEST = re.compile(r"^test (\S+)(?: - .*?)? \.\.\. (ok|FAILED|ignored(?:, .*)?)$")


def parse(path):
    results = {}
    target = None
    with open(path, encoding="utf-8", errors="replace") as fh:
        for line in fh:
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
                outcome = m.group(2)
                outcome = "ignored" if outcome.startswith("ignored") else outcome
                results.setdefault(target, {})[m.group(1)] = outcome
    return results


def main():
    base, cand = parse(sys.argv[1]), parse(sys.argv[2])
    targets = sorted(set(base) | set(cand))
    total_diff = 0
    for t in targets:
        b, c = base.get(t, {}), cand.get(t, {})
        def counts(d):
            return {k: sum(1 for v in d.values() if v == k) for k in ("ok", "FAILED", "ignored")}
        print(f"{t}: base {counts(b)} cand {counts(c)}")
        for name in sorted(set(b) | set(c)):
            if b.get(name) != c.get(name):
                total_diff += 1
                print(f"  DIFF {name}: base={b.get(name)} cand={c.get(name)}")
    print(f"TOTAL_DIFFERENCES {total_diff}")


if __name__ == "__main__":
    main()
