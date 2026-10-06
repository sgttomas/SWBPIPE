"""RV102: verify SHA256SUMS-style files in the git-archive copy of the PR head.

Usage: python sums_verify.py <copy_root> <t3_rel> <sums_file_rel>...

For each sums file (path relative to T3): every entry is hashed from the copy
(OK / BAD / MISSING), and the folder's files are checked for coverage by the
union of the folder's SHA256SUMS* files (a sums file does not list itself).
Prints one TSV row per sums file, then the uncovered files per folder.
"""
import hashlib
import sys
from collections import defaultdict
from pathlib import Path

copy_root, t3rel = sys.argv[1:3]
T3 = Path(copy_root) / t3rel
rels = sys.argv[3:]
listed_by_folder = defaultdict(set)
sums_by_folder = defaultdict(set)
print("sums_file\tsha256_prefix\tentries\tok\tbad\tmissing")
for rel in rels:
    sf = T3 / rel
    folder = sf.parent
    sums_by_folder[folder].add(sf.name)
    ok = bad = missing = 0
    for line in sf.read_text().splitlines():
        if not line.strip():
            continue
        h, name = line.split(None, 1)
        name = name.lstrip("*").strip()
        p = folder / name
        listed_by_folder[folder].add(str(Path(name)))
        if not p.is_file():
            missing += 1
            print(f"#MISSING\t{rel}\t{name}")
            continue
        if hashlib.sha256(p.read_bytes()).hexdigest() == h.lower():
            ok += 1
        else:
            bad += 1
            print(f"#BAD\t{rel}\t{name}")
    n = ok + bad + missing
    print(f"{rel}\t{hashlib.sha256(sf.read_bytes()).hexdigest()[:12]}\t{n}\t{ok}\t{bad}\t{missing}")
print("# coverage (files in the folder not listed by any of its SHA256SUMS* files)")
for folder, listed in listed_by_folder.items():
    uncovered = []
    for p in sorted(folder.rglob("*")):
        if p.is_dir():
            continue
        r = str(p.relative_to(folder))
        if p.name.startswith("SHA256SUMS") and p.parent == folder:
            continue
        if r not in listed:
            uncovered.append(r)
    print(f"{folder.relative_to(T3)}\tuncovered={len(uncovered)}\t" + ";".join(uncovered[:10]))
