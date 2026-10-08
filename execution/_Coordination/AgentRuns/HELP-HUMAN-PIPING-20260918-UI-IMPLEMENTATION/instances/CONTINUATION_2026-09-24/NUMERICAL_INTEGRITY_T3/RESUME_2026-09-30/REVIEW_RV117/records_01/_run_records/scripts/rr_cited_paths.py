#!/usr/bin/env python3
"""RV117: backticked record paths (R/..., IMPLEMENTATION/..., BRIEFS/...) in RR's appended text and in
WG's T3 section exist at H (as a file or a folder). Usage: rr_cited_paths.py <repo> <H> <RR start line>"""
import re, subprocess, sys
repo, H, start = sys.argv[1], sys.argv[2], int(sys.argv[3])
def git(*a): return subprocess.run(["git", "-C", repo, *a], capture_output=True, check=True).stdout
T3 = "projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/"
R = T3 + "RESUME_2026-09-30/"
WGP = "projects/chirality-piping/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md"
rr = git("show", f"{H}:{T3}ROOT_RULINGS_V1.md").decode().split("\n")
wg = git("show", f"{H}:{WGP}").decode().split("\n")
s = next(i for i, l in enumerate(wg) if l.startswith("## T3 current route"))
pat = re.compile(r"`((?:R/|IMPLEMENTATION/|BRIEFS/)[^`*{}<>…]+?)/?`")
def exists(p):
    cands = [R + p[2:]] if p.startswith("R/") else [T3 + p, R + p]
    return any(subprocess.run(["git", "-C", repo, "cat-file", "-e", f"{H}:{c}"], capture_output=True).returncode == 0 for c in cands)
n = ok = 0
for tag, lines, off in (("RR", rr[start - 1:], start), ("WG", wg[s:], s + 1)):
    for i, l in enumerate(lines):
        for p in pat.findall(l):
            n += 1; e = exists(p); ok += e
            if not e: print(f"{tag}:{i + off}\tMISSING\t{p}")
print(f"cited record paths {n}; present {ok}")
