#!/usr/bin/env python3
"""RV117: every "`<path>` sha256 `<hex>…`" citation in RR's appended text (lines >= start) and in
WG's T3 section resolves to a file at H whose sha256 starts with the cited hex.
Paths: R/... = T3/RESUME_2026-09-30/...; IMPLEMENTATION/... and BRIEFS/... are T3- or R-relative.
Usage: rr_cited_hashes.py <repo> <H> <start line>"""
import re, subprocess, sys, hashlib
repo, H, start = sys.argv[1], sys.argv[2], int(sys.argv[3])
def git(*a): return subprocess.run(["git", "-C", repo, *a], capture_output=True, check=True).stdout
T3 = "projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/"
R = T3 + "RESUME_2026-09-30/"
rr = git("show", f"{H}:{T3}ROOT_RULINGS_V1.md").decode().split("\n")
cite = re.compile(r"`((?:R/|IMPLEMENTATION/|BRIEFS/)[^`]+)`[^`]{0,40}?sha256 `([0-9a-f]{8,64})")
def resolve(p):
    for cand in ([R + p[2:]] if p.startswith("R/") else [T3 + p, R + p]):
        r = subprocess.run(["git", "-C", repo, "rev-parse", f"{H}:{cand}"], capture_output=True)
        if r.returncode == 0: return cand, r.stdout.decode().strip()
    return None, None
n = ok = 0
for i in range(start - 1, len(rr)):
    for p, hx in cite.findall(rr[i]):
        n += 1; cand, b = resolve(p)
        if not b: print(f"RR:{i+1}\tUNRESOLVED\t{p}\t{hx}"); continue
        sha = hashlib.sha256(git("cat-file", "blob", b)).hexdigest()
        good = sha.startswith(hx); ok += good
        print(f"RR:{i+1}\t{'OK' if good else 'MISMATCH'}\t{p}\tcited {hx}\tactual {sha[:16]}")
print(f"citations {n}; ok {ok}")
