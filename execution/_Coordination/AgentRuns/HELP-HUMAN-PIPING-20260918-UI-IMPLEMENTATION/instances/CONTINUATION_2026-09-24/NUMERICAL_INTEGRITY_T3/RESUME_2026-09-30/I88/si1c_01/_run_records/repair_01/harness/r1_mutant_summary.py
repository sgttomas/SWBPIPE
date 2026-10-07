#!/usr/bin/env python3
"""I88 repair round 1: merge the mutant parts and summarize (scratch tool)."""
import json, sys
S = "WT/scratch/i88_si1c"
parts = [("part1", f"{S}/logs/r1/mutants_part1.log"), ("part2a", f"{S}/logs/r1/mutants_part2a.log"),
         ("part2b", f"{S}/logs/r1/mutants_part2b.log")]
merged, lines = {}, []
k = s = 0
for part, path in parts:
    for line in open(path):
        name, _, js = line.partition(" ")
        d = json.loads(js)
        key = name if name != "UNMUTATED" else f"UNMUTATED_{part}"
        merged[key] = d
        if d["killed"] is None:
            lines.append(f"{key}: baseline, compiled={d['compiled']}, {d['results']}")
        elif d["killed"]:
            k += 1
            lines.append(f"{name}: KILLED by {', '.join(d['failed_tests'])}")
        else:
            s += 1
            eq = d.get("equivalence_dumps")
            if isinstance(eq, dict):
                diff = sum(v["differ"] for v in eq.values()); tot = sum(v["lines"] for v in eq.values())
                lines.append(f"{name}: SURVIVED (equivalent): dumps differ {diff} of {tot} lines")
            else:
                lines.append(f"{name}: SURVIVED (equivalent): {eq}")
lines.append("")
lines.append(f"killed {k}, survived {s}, total {k + s}")
json.dump(merged, open(f"{S}/reports/r1_mutants.json", "w"), indent=1, sort_keys=True)
open(f"{S}/records_r1/mutant_logs_summary_r1.txt", "w").write(
    "# I88 T3-SI1c repair round 1 mutants at f5665f8862. Kill rule as in round 0. Part 1 ran with equivalence\n"
    "# dumps for survivors; part 2a is the runner mutants (RV111's N06, N09 first); part 2b re-ran SI1b's\n"
    "# remaining mutants without new dumps (EE non-test code identical to 7f233b2e01).\n\n" + "\n".join(lines) + "\n")
print("\n".join(lines[-12:]))
