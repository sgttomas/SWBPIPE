"""I75 REPAIR_01: a vitest JSON report as a sorted per-test list (status, file from src/, full name)."""
import json, sys
d = json.load(open(sys.argv[1])); rows = []
for f in d["testResults"]:
    n = f["name"]; rel = n[n.index("src/"):]
    for t in f["assertionResults"]: rows.append((t["status"], rel, t["fullName"]))
with open(sys.argv[2], "w") as out:
    out.write("status\tfile\tfull_name\n")
    for r in sorted(rows, key=lambda r: (r[1], r[2])): out.write("\t".join(r) + "\n")
print(len(rows), d["numTotalTests"], d["numPassedTests"], d["numFailedTests"], len(d["testResults"]))
