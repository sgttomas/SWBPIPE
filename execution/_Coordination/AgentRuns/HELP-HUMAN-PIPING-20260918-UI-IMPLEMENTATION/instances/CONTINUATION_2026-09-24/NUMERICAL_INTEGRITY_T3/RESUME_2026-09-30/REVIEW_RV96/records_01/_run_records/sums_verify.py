#!/usr/bin/env python3
"""RV96 read-only verification of every SHA256SUMS-style file in a list.
usage: sums_verify.py <archive_root> <sums_list_file> <out_tsv>
Lines: '<64hex>  <path>' or '<64hex> *<path>', path relative to the SUMS dir.
Symlinks are reported as SYMLINK (dangling or not) and not followed."""
import hashlib, os, re, sys
from pathlib import Path

root, lst, out = sys.argv[1:4]
LINE = re.compile(r"^([0-9a-f]{64}) [ *](.+)$")
rows = []
tot = {"OK": 0, "BAD": 0, "MISSING": 0, "SYMLINK": 0, "UNPARSED": 0}
for rel in Path(lst).read_text().splitlines():
    sums = Path(root) / rel
    d = sums.parent
    ok = bad = miss = sl = unp = 0
    details = []
    for ln in sums.read_text(encoding="utf-8", errors="replace").splitlines():
        if not ln.strip():
            continue
        m = LINE.match(ln)
        if not m:
            unp += 1; details.append(("UNPARSED", ln[:120])); continue
        h, p = m.groups()
        f = d / p
        if not (f.is_file() or os.path.islink(f)) and (d.parent / p).exists():
            f = d.parent / p
        if not (f.is_file() or os.path.islink(f)) and (d.parent.parent / p).exists():
            f = d.parent.parent / p
        if os.path.islink(f):
            sl += 1; details.append(("SYMLINK", p)); continue
        if not f.is_file():
            miss += 1; details.append(("MISSING", p)); continue
        dig = hashlib.sha256(f.read_bytes()).hexdigest()
        if dig == h:
            ok += 1
        else:
            bad += 1; details.append(("BAD", p))
    for k, v in (("OK", ok), ("BAD", bad), ("MISSING", miss), ("SYMLINK", sl), ("UNPARSED", unp)):
        tot[k] += v
    status = "PASS" if (bad == miss == sl == unp == 0 and ok > 0) else "CHECK"
    rows.append((status, rel, ok, bad, miss, sl, unp, "; ".join(f"{a}:{b}" for a, b in details[:6])))
with open(out, "w") as fh:
    for r in rows:
        fh.write("\t".join(map(str, r)) + "\n")
print("sums files", len(rows), "PASS", sum(r[0] == "PASS" for r in rows),
      "CHECK", sum(r[0] == "CHECK" for r in rows), tot)
