"""Compare two cargo test logs test by test: (binary, test) -> status.

usage: compare_suites.py <base.log> <candidate.log> <out.tsv>
"""
import re
import sys

RUN = re.compile(r"^\s+Running (?:unittests )?(\S+)")
DOC = re.compile(r"^\s+Doc-tests (\S+)")
TEST = re.compile(r"^test (.+?) \.\.\. (ok|FAILED|ignored)$")
TOTAL = re.compile(r"^test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored")


def parse(path):
    out, totals, binary = {}, [], None
    for line in open(path, encoding="utf-8", errors="replace"):
        line = line.rstrip("\n")
        if m := RUN.match(line):
            binary = m.group(1)
        elif m := DOC.match(line):
            binary = "doctests:" + m.group(1)
        elif m := TEST.match(line):
            key = (binary, m.group(1))
            assert key not in out, key
            out[key] = m.group(2)
        elif m := TOTAL.match(line):
            totals.append((binary, int(m.group(2)), int(m.group(3)), int(m.group(4))))
    return out, totals


base, base_totals = parse(sys.argv[1])
cand, cand_totals = parse(sys.argv[2])
rows = []
for key in sorted(set(base) | set(cand), key=lambda k: (k[0] or "", k[1])):
    b, c = base.get(key, "absent"), cand.get(key, "absent")
    rows.append((key[0], key[1], b, c, "same" if b == c else "DIFF"))
with open(sys.argv[3], "w") as f:
    f.write("binary\ttest\tbase\tcandidate\tcompare\n")
    for r in rows:
        f.write("\t".join(r) + "\n")
print("base totals:", base_totals)
print("cand totals:", cand_totals)
print("base tests", len(base), "statuses", {s: list(base.values()).count(s) for s in set(base.values())})
print("cand tests", len(cand), "statuses", {s: list(cand.values()).count(s) for s in set(cand.values())})
diffs = [r for r in rows if r[4] == "DIFF"]
print("differences", len(diffs))
for r in diffs:
    print("  ", r)
