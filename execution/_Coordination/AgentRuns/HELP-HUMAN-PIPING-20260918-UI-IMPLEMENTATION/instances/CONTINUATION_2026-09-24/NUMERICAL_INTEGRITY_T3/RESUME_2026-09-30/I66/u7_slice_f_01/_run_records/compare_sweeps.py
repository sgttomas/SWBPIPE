#!/usr/bin/env python3
"""I66 U6b: compare the per-test outcomes of the base and candidate sweeps.
Args: base.log cand.log out_prefix. Writes <out>.base.outcomes, <out>.cand.outcomes
and <out>.compare.txt (paths in the logs are sanitized to placeholders)."""
import re, sys
WT = "WT"


def outcomes(path):
    rows, total = {}, ""
    for line in open(path, encoding="utf-8", errors="replace"):
        m = re.match(r"^(PASSED|FAILED|ERROR|XFAIL|XPASS) (\S+)", line)
        if m:
            rows[m.group(2)] = m.group(1)
        if re.match(r"^=*\s*\d+ (passed|failed)", line.strip("= \n")) or re.search(r"\d+ passed.* in [\d.]+s", line):
            total = line.strip().strip("=").strip()
    return rows, total


base, btotal = outcomes(sys.argv[1])
cand, ctotal = outcomes(sys.argv[2])
out = sys.argv[3]
for name, rows in (("base", base), ("cand", cand)):
    with open(f"{out}.{name}.outcomes", "w") as fh:
        fh.writelines(f"{v} {k}\n" for k, v in sorted(rows.items()))
regress = sorted(k for k, v in base.items() if v == "PASSED" and cand.get(k) != "PASSED")
newfail = sorted(k for k, v in cand.items() if v != "PASSED" and base.get(k) == "PASSED")
fixed = sorted(k for k, v in base.items() if v != "PASSED" and cand.get(k) == "PASSED")
added = sorted(set(cand) - set(base))
removed = sorted(set(base) - set(cand))
lines = [f"base:      {btotal}", f"candidate: {ctotal}",
         f"passing at base, not passing in candidate: {len(regress)} {regress}",
         f"failing in candidate, passing at base: {len(newfail)} {newfail}",
         f"failing at base, passing in candidate: {len(fixed)} {fixed}",
         f"tests added in candidate: {len(added)} (files: {sorted({a.split('::')[0] for a in added})})",
         f"tests removed: {len(removed)} {removed}",
         f"candidate non-passing: {sorted(k for k, v in cand.items() if v != 'PASSED')}",
         f"base non-passing: {sorted(k for k, v in base.items() if v != 'PASSED')}"]
text = "\n".join(lines).replace(WT, "WT") + "\n"
open(f"{out}.compare.txt", "w").write(text)
print(text)
