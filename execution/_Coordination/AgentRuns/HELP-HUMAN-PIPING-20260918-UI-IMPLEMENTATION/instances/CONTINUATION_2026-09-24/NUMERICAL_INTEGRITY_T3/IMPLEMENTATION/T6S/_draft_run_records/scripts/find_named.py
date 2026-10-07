#!/usr/bin/env python3
"""I80 scratch: list named (non-tool) references in the added lines of base..head (maintained paths)."""
import re, subprocess, sys, os, collections
repo, base, head = sys.argv[1:4]
env = dict(os.environ, GIT_OPTIONAL_LOCKS="0")
P = "projects/chirality-piping/"
S = [n for n in subprocess.run(["git","-C",repo,"diff","--name-only",base,head,"--",".",":!"+P+"execution"],capture_output=True,env=env,text=True).stdout.split("\n") if n]
diff = subprocess.run(["git","-C",repo,"diff","-U0",base,head,"--",*S],capture_output=True,env=env).stdout.decode()
PAT = [
 ("rr_decision", r"RR decisions? [0-9][0-9, and]*"),
 ("plan", r"(?:I74 )?PLAN [0-9][0-9.\-–]*(?: and [0-9.]+)?"),
 ("d2", r"\bD2 [0-9][0-9.]*(?:, [0-9.]+)*(?: and [0-9.]+)?"),
 ("decision_id", r"\bD-U[0-9]+-[0-9]+\b"),
 ("cq", r"\bCQ-[0-9]+\b"),
 ("review", r"\bRV[0-9]+ (?:SF|N|NT|S)-?[0-9]+\b|\bRV[0-9]+ SF-1\b"),
 ("repair", r"REPAIR_01"),
 ("finding", r"\bI[0-9]+'s F[0-9]+\b"),
 ("reading", r"\b(?:checkpoint )?reading R-[0-9]+\b|\bR-[0-9]\b"),
 ("checkpoint", r"CHECKPOINT_1(?: section [0-9]+)?"),
 ("commit", r"\b[0-9a-f]{10}\b"),
 ("instance", r"\bI7[5-6]\b|\bI67\b"),
 ("slice", r"\bU7 slice T\b|\b0?7[a-l]\b"),
 ("route", r"\bB8\b|\bPR-B1\b|\bS-I2\b|\bF-U[0-9a-z]+-[0-9]+\b|\bDD-[0-9]+\b"),
]
f=None; ln=0; out=collections.defaultdict(list)
for line in diff.split("\n"):
    if line.startswith("+++ "): f = line[6:].replace(P,"") if line.startswith("+++ b/") else None; continue
    m = re.match(r"@@ -\S+ \+(\d+)(?:,\d+)? @@", line)
    if m: ln = int(m.group(1)); continue
    if line.startswith("+") and f:
        for k, rx in PAT:
            for mm in re.finditer(rx, line[1:]): out[(k, mm.group(0))].append(f"{f}:{ln}")
        ln += 1
print("S =", len(S))
for (k,t),sites in sorted(out.items()):
    print(f"{k}\t{t}\t{len(sites)}\t{'; '.join(sites[:8])}{' …' if len(sites)>8 else ''}")
