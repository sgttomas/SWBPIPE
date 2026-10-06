#!/usr/bin/env python3
"""I77: list the added lines of U8's diff (b1e2d7741e..bd6b4be2c3) that hold a named-reference token.
Read-only (git diff, GIT_OPTIONAL_LOCKS=0). Usage: python3 find_sites.py <U8 checkout>"""
import os, re, subprocess, sys
P = "projects/chirality-piping/"
d = subprocess.run(["git", "-C", sys.argv[1], "diff", "-U0", "b1e2d7741e", "bd6b4be2c3", "--", P], capture_output=True, text=True,
                   env=dict(os.environ, GIT_OPTIONAL_LOCKS="0"), check=True).stdout
pats = {"RV93": r"RV93", "D-U6-5": r"D-U6-5", "decision 6": r"[Dd]ecision 6", "W-C1": r"W-C1", "W-C2": r"W-C2", "U8-0": r"U8-0",
        "I68": r"\bI68\b", "I69": r"\bI69\b", "I70": r"\bI70", "W6": r"\bW6\b", "PHYS-R4": r"PHYS-R4", "witness_tests": r"retained_memory_witness_tests",
        "RV94 N-3": r"RV94 N-3", "D-U7-2": r"D-U7-2", "RV90": r"RV90", "C04": r"C04", "D11": r"\bD11\b", "D37": r"\bD37\b",
        "u8_head": r"u8_head", "U5": r"\bU5\b", "§": r"§", "PLAN": r"PLAN", "RR": r"\bRR\b", "R/": r"R/I", "probe verified": r"probe verified"}
f, ln = None, 0
for line in d.split("\n"):
    if line.startswith("+++ "): f = line[6:].replace(P, "") if line.startswith("+++ b/") else None; continue
    m = re.match(r"@@ -\S+ \+(\d+)(?:,\d+)? @@", line)
    if m: ln = int(m.group(1)); continue
    if line.startswith("+") and f:
        for k, p in pats.items():
            if re.search(p, line[1:]): print(f"{k}\t{f}:{ln}\t{line[1:].strip()[:150]}")
        ln += 1
