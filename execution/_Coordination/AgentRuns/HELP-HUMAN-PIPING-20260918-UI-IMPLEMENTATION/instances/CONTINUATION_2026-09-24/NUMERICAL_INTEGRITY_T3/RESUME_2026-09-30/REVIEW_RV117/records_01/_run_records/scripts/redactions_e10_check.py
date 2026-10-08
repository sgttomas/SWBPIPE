#!/usr/bin/env python3
"""RV117: E-10's originals are absent from the PR head.
For each row of R/I89/b1_sa_01/REDACTION_01.md's tables (old -> new sha256):
- the blob at the pre-redaction commit (if the path was committed there) hashes to the old sha256;
- H's blob hashes to the new sha256;
- the pre-redaction blob id is not in H's tree, nor in M's;
- no file changed in M..H hashes to any old sha256.
Usage: redactions_e10_check.py <repo> <M> <H> <pre-redaction rev>"""
import re, subprocess, sys, hashlib
repo, M, H, OLD = sys.argv[1:5]
def git(*a): return subprocess.run(["git", "-C", repo, *a], capture_output=True, check=True).stdout
T3 = "projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/"
D = T3 + "RESUME_2026-09-30/I89/b1_sa_01/"
doc = git("show", f"{H}:{D}REDACTION_01.md").decode()
rows = re.findall(r"^\| `([^`]+)` \| `([0-9a-f]{64})` \| `([0-9a-f]{64})` \|", doc, re.M)
print("rows in REDACTION_01 tables:", len(rows))
def blob(rev, p):
    r = subprocess.run(["git", "-C", repo, "rev-parse", f"{rev}:{p}"], capture_output=True)
    return r.stdout.decode().strip() if r.returncode == 0 else None
htree = set(l.split()[2] for l in git("ls-tree", "-r", H).decode().splitlines())
mtree = set(l.split()[2] for l in git("ls-tree", "-r", M).decode().splitlines())
old_ok = new_ok = at_old = 0; old_blobs = set(); olds = set()
for p, o, n in rows:
    olds.add(o)
    b0 = blob(OLD, D + p); bh = blob(H, D + p)
    if b0:
        at_old += 1; old_blobs.add(b0)
        if hashlib.sha256(git("cat-file", "blob", b0)).hexdigest() == o: old_ok += 1
        else: print("  OLD MISMATCH", p)
    else:
        print("  not committed at pre-redaction rev:", p)
    if bh and hashlib.sha256(git("cat-file", "blob", bh)).hexdigest() == n: new_ok += 1
    else: print("  NEW MISMATCH", p)
print(f"committed at {OLD}: {at_old}; old sha256 matches there: {old_ok}; new sha256 matches H: {new_ok}/{len(rows)}")
print("pre-redaction blob ids in H's tree:", len(old_blobs & htree), " in M's tree:", len(old_blobs & mtree))
ch = git("diff", "--no-renames", "--name-only", M, H).decode().split("\n")
ch = [c for c in ch if c]
hits = [c for c in ch if hashlib.sha256(git("show", f"{H}:{c}")).hexdigest() in olds]
print(f"changed files (of {len(ch)}) hashing to an old sha256:", len(hits))
