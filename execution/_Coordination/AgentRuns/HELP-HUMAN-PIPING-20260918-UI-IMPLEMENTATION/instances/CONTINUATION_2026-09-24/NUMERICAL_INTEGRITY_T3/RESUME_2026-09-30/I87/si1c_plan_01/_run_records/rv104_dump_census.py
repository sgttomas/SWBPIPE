"""I87 read-only census of RV104's candidate evaluator dump (not repository content).

Counts point evaluations whose point result is decided (a boolean, or a finite
quantity) while interval mode with every input bound as its exact point (RV104
variant i1, b = 0) reads indeterminate with the note non_finite_enclosure.
That over-counts SI1c's set: interval mode also steps one ulp outward after
each operation, so a point result at +-MAX reads U there with no point overflow.
Reads only: the dump file named on the command line.
"""
import gzip
import re
import sys
from collections import Counter

path = sys.argv[1]
point = {}
inter = {}
with gzip.open(path, "rt") as fh:
    for line in fh:
        parts = line.rstrip("\n").split("\t")
        if len(parts) < 3:
            continue
        label, kind, rest = parts[0], parts[1], parts[2]
        if kind == "P":
            point[label] = rest
        elif kind == "I" and label.endswith("#i1"):
            inter[label[: -len("#i1")]] = rest

FLOAT_INF = {"0x7ff0000000000000", "0xfff0000000000000"}
counts = Counter()
examples = {}
for label, p in point.items():
    head = p.split("|", 1)[0]
    i = inter.get(label)
    if i is None:
        continue
    ival, findings, notes, _ = i.split("|")
    if head in ("true", "false"):
        kind = "boolean_" + head
    elif head.startswith("0x"):
        bits = int(head, 16)
        exp = (bits >> 52) & 0x7FF
        kind = "quantity_nonfinite" if exp == 0x7FF else "quantity_finite"
    else:
        kind = "point_blocked_or_panic"
    u_nonfinite = ival in ("Indeterminate", "noenc") and "non_finite_enclosure" in notes
    counts[(kind, "i1_U_non_finite" if u_nonfinite else "i1_other")] += 1
    if u_nonfinite and kind.startswith(("boolean", "quantity_finite")):
        family = label.split("_", 1)[0]
        counts[("decided_vs_U_by_family", kind, family)] += 1
        examples.setdefault((kind, family), label)

for key in sorted(counts, key=str):
    print(key, counts[key])
print("examples:")
for key, label in sorted(examples.items()):
    print(key, label)
