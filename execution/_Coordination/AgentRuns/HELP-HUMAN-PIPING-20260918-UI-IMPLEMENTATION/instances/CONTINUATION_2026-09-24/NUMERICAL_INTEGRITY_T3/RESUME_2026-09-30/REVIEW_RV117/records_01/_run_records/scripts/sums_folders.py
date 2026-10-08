#!/usr/bin/env python3
"""RV117: for each sealed record folder, verify every sum file at the folder's top level
(whether added by this PR or already on main) from H's committed tree, and list the files
in the folder that none of its sum files covers. Blobs are read with git; nothing is checked out.
A sum file is a top-level file whose name contains SHA256SUMS.
Usage: sums_folders.py <repo> <M> <H> <T3-relative folder>..."""
import subprocess, sys, hashlib, posixpath
repo, M, H = sys.argv[1:4]
folders = sys.argv[4:]
def git(*a): return subprocess.run(["git", "-C", repo, *a], capture_output=True, check=True).stdout
T3 = "projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/"
tree = {}
for l in git("ls-tree", "-r", H, "--", T3).decode().splitlines():
    meta, p = l.split("\t", 1); tree[p] = (meta.split()[0], meta.split()[2])
mtree = set(git("ls-tree", "-r", "--name-only", M, "--", T3).decode().splitlines())
print("folder\tsumfile\ton_main\tentries\tok\tbad\tmissing")
GF = GE = GOK = 0
for f in folders:
    d = T3 + f.rstrip("/")
    files = sorted(p for p in tree if p.startswith(d + "/"))
    sums = sorted(p for p in files if posixpath.dirname(p) == d and "SHA256SUMS" in posixpath.basename(p))
    listed = set()
    for s in sums:
        n = ok = bad = miss = 0
        for line in git("cat-file", "blob", tree[s][1]).decode().splitlines():
            if not line.strip() or line.startswith("#"): continue
            h, name = line.split(None, 1); name = name.lstrip("*"); n += 1
            p = posixpath.normpath(posixpath.join(d, name)); listed.add(p)
            if p not in tree: miss += 1; print("  MISSING", p.replace(T3, "T3/")); continue
            if hashlib.sha256(git("cat-file", "blob", tree[p][1])).hexdigest() == h: ok += 1
            else: bad += 1; print("  BAD", p.replace(T3, "T3/"))
        GF += 1; GE += n; GOK += ok
        print(f"{f}\t{posixpath.basename(s)}\t{'main' if s in mtree else 'PR'}\t{n}\t{ok}\t{bad}\t{miss}")
    unc = [p for p in files if p not in listed and p not in sums]
    print(f"{f}\t(coverage: {len(sums)} sum files, {len(files)} files)\tuncovered={len(unc)}\t" + "; ".join(p[len(d) + 1:] for p in unc[:20]))
print(f"TOTAL sum files {GF}; entries {GE}; ok {GOK}")
