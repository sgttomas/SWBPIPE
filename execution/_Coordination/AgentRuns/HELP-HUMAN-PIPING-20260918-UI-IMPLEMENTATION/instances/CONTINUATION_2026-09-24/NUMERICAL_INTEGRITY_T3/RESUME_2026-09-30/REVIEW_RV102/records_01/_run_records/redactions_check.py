"""RV102: the 13 redacted originals are absent from the PR head.

Usage: python redactions_check.py <git_dir> <copy_root> <t3_rel> <orig_commit> <head> <main> <changed_name_status_tsv>

1. For each REDACTIONS.json entry, read the blob at <orig_commit> (#1084's first
   head, which carried the originals) and confirm its sha256 equals
   original_sha256; take its blob id.
2. Confirm the head's (and main's) blob at that path has sha256 = redacted_sha256.
3. Search the full recursive tree of <head> and of <main> for the 13 original blob ids.
4. Hash every changed file of the PR in the copy and compare with the 13 originals.
"""
import hashlib
import json
import subprocess
import sys
from pathlib import Path

git_dir, copy_root, t3rel, orig, head, main, listfile = sys.argv[1:8]


def git(*a):
    return subprocess.run(["git", "-C", git_dir, *a], capture_output=True, check=True).stdout


red = json.loads((Path(copy_root) / t3rel / "IMPLEMENTATION/HANDOFF_2026-10-05/_run_records/REDACTIONS.json").read_text())
entries = red["files"]
orig_blobs = {}
originals = set()
print(f"entries={len(entries)}")
for e in entries:
    path = e["path"].replace("T3/", t3rel + "/", 1)
    ob = git("rev-parse", f"{orig}:{path}").decode().strip()
    od = git("cat-file", "blob", ob)
    hb = git("rev-parse", f"{head}:{path}").decode().strip()
    hd = git("cat-file", "blob", hb)
    mb = git("rev-parse", f"{main}:{path}").decode().strip()
    originals.add(e["original_sha256"])
    orig_blobs[ob] = e["path"]
    print("\t".join([
        e["path"],
        "orig_sha_ok" if hashlib.sha256(od).hexdigest() == e["original_sha256"] else "orig_sha_MISMATCH",
        "head_is_redacted" if hashlib.sha256(hd).hexdigest() == e["redacted_sha256"] else "head_NOT_redacted",
        "main_eq_head" if mb == hb else "main_ne_head",
        ob[:12],
    ]))
for rev in (head, main):
    tree = git("ls-tree", "-r", rev).decode().splitlines()
    found = [l for l in tree if l.split()[2] in orig_blobs]
    print(f"original blob ids present in full tree of {rev[:10]}: {len(found)}/13 (tree entries scanned: {len(tree)})")
n = 0
bad = 0
for row in open(listfile).read().splitlines():
    status, name = row.split("\t", 1)
    n += 1
    if hashlib.sha256((Path(copy_root) / name).read_bytes()).hexdigest() in originals:
        bad += 1
        print("ORIGINAL CONTENT IN CHANGED FILE:", name)
print(f"changed files hashed: {n}; equal to an original_sha256: {bad}")
