"""RV102 addendum 01: verify sums files from the committed tree (Git blobs),
not from a working folder.

Usage: python sums_verify_git.py <git_dir> <rev> <t3_rel> [<folder_rel>...]

Without folder arguments, every file under T3 whose name starts with
SHA256SUMS or ends with .SHA256SUMS is checked. Each entry is hashed from
the blob at <rev> (OK / BAD / MISSING). Coverage: files under the sums
file's folder (recursively) listed by none of that folder's sum files.
Prints one TSV row per sums file, then one coverage row per folder.
"""
import hashlib
import subprocess
import sys
from collections import defaultdict
from pathlib import PurePosixPath as P

git_dir, rev, t3rel = sys.argv[1:4]
only = sys.argv[4:]


def git(*a, inp=None):
    return subprocess.run(["git", "-C", git_dir, *a], capture_output=True, check=True, input=inp).stdout


tree = {}
for line in git("ls-tree", "-r", rev, "--", t3rel).decode().splitlines():
    meta, path = line.split("\t", 1)
    mode, typ, oid = meta.split()
    tree[path] = oid


def blob(path):
    return git("cat-file", "blob", tree[path])


def is_sums(name):
    return name.startswith("SHA256SUMS") or name.endswith(".SHA256SUMS")


sums_files = sorted(p for p in tree if is_sums(P(p).name))
if only:
    sums_files = [p for p in sums_files if any(p.startswith(f"{t3rel}/{o.rstrip('/')}/") for o in only)]
listed = defaultdict(set)
folders_sums = defaultdict(set)
print("sums_file\tsha256_prefix\tentries\tok\tbad\tmissing\tmissing_names")
for sf in sums_files:
    folder = str(P(sf).parent)
    folders_sums[folder].add(sf)
    data = blob(sf)
    ok = bad = 0
    missing = []
    for line in data.decode("utf-8", "replace").splitlines():
        if not line.strip():
            continue
        h, name = line.split(None, 1)
        name = name.lstrip("*").strip()
        if name.startswith("./"):
            name = name[2:]
        full = f"{folder}/{name}"
        listed[folder].add(full)
        if full not in tree:
            missing.append(name)
        elif hashlib.sha256(blob(full)).hexdigest() == h.lower():
            ok += 1
        else:
            bad += 1
    short = sf.replace(t3rel, "T3")
    print(f"{short}\t{hashlib.sha256(data).hexdigest()[:12]}\t{ok + bad + len(missing)}\t{ok}\t{bad}\t{len(missing)}\t{';'.join(missing[:6])}")
print("# coverage: files in the folder (recursive) listed by none of its sum files")
for folder in sorted(folders_sums):
    unc = [p for p in tree if p.startswith(folder + "/") and p not in listed[folder]
           and not (str(P(p).parent) == folder and is_sums(P(p).name))]
    print(f"{folder.replace(t3rel, 'T3')}\tuncovered={len(unc)}\t" + ";".join(u.replace(folder + '/', '') for u in unc[:6]))
