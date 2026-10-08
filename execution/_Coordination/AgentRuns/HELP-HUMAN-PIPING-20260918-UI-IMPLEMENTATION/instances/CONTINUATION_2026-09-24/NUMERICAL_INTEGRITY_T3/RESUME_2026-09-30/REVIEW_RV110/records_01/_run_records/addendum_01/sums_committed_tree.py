#!/usr/bin/env python3
"""RV110 addendum 01: every sum file added in M..H (or present at H in the listed folders) verifies
from H's committed tree (blobs read with git, paths resolved against the sum file's folder).
Usage: sums_committed_tree.py <repo> <M> <H>"""
import subprocess, sys, hashlib, posixpath
repo, M, H = sys.argv[1:4]
def git(*a): return subprocess.run(["git","-C",repo,*a],capture_output=True,check=True).stdout
T3="projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/"
tree={}
for l in git("ls-tree","-r",H).decode().splitlines():
    meta,p=l.split("\t",1); tree[p]=meta.split()[2]
added=[r.split("\t",1)[1] for r in git("diff","--no-renames","--name-status",M,H).decode().splitlines() if r.startswith("A\t")]
sums=sorted(p for p in added if "SHA256SUMS" in posixpath.basename(p))
F=E=OK=BAD=MISS=0
print("sumfile\tentries\tok\tbad\tmissing")
for s in sums:
    d=posixpath.dirname(s); n=ok=bad=miss=0
    for line in git("cat-file","blob",tree[s]).decode().splitlines():
        if not line.strip() or line.startswith("#"): continue
        h,name=line.split(None,1); name=name.lstrip("*"); n+=1
        p=posixpath.normpath(posixpath.join(d,name))
        if p not in tree: miss+=1; print("  MISSING", p.replace(T3,"T3/")); continue
        if hashlib.sha256(git("cat-file","blob",tree[p])).hexdigest()==h: ok+=1
        else: bad+=1; print("  BAD", p.replace(T3,"T3/"))
    F+=1; E+=n; OK+=ok; BAD+=bad; MISS+=miss
    print(f"{s.replace(T3,'T3/')}\t{n}\t{ok}\t{bad}\t{miss}")
print(f"TOTAL sum files {F}; entries {E}; ok {OK}; bad {BAD}; missing {MISS}")
