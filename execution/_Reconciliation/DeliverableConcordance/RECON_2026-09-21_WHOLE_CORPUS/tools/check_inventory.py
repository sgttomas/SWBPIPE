#!/usr/bin/env python3
"""Structural check for R1 inventory files (briefs/R1-INV_area_inventory_brief.md).

Checks header, CRLF, `#END` sentinel count, CapabilityID prefix and uniqueness,
Kind vocabulary, that every EntryPoints and Tests path exists at the frozen
commit, that Capability and Notes name no deliverable, package, scope,
objective or decision identifier, and (with --coverage) that every non-test
tracked file in the area's partition (git config files excepted) appears in an
EntryPoints cell (test-data fixtures may be listed under Tests). Directory
EntryPoints are allowed only in DATA, DOCS and CHECKS, and never for a whole
area prefix. The Area column must equal the area checked; the ownership screen
is case-insensitive.
Deterministic and read-only. Exit 0 = PASS, 1 = findings.

Usage
  check_inventory.py --repo-root . --run-dir <run> --area VIEW --partition <R1-INV_areas.json> [--coverage]
"""

from __future__ import annotations

import argparse
import csv
import io
import json
import re
import subprocess
import sys

FREEZE = "00115c71931bcae79909602d653740d3bb72dfa1"
P = "projects/chirality-piping/"
HEADER = ["CapabilityID", "Area", "Kind", "Capability", "EntryPoints", "Tests", "Notes"]
KINDS = {"ENGINE", "DATA_CONTRACT", "UI_OPERATION", "UI_SURFACE", "COMMAND", "DIAGNOSTIC", "INTEGRATION",
         "SECURITY_CONTROL", "DOCUMENT", "CHECK", "FIXTURE_SET", "TEST_ONLY"}
OWNER_TOKEN = re.compile(r"(?i)\b(DEL-\d\d-\d\d|PKG-\d\d|SOW-\d{3}|OBJ-\d{3}|DEC-\d{3}|D-\d{2,3}[a-z]?|SCA-\d{3})\b")
DIR_OK_AREAS = {"DATA", "DOCS", "CHECKS"}  # family-grain areas may cite directories
EXCLUDE = ("execution/", "plans/", "loop/", "init/", "tests/", "validation/evidence/")


def main(argv):
    ap = argparse.ArgumentParser()
    ap.add_argument("--repo-root", required=True)
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--area", required=True)
    ap.add_argument("--partition", required=True)
    ap.add_argument("--coverage", action="store_true")
    a = ap.parse_args(argv)
    f = []
    path = f"{a.run_dir}/R1_INVENTORY/INV_{a.area}.csv"
    raw = open(path, "rb").read()
    if not raw.split(b"\n", 1)[0].endswith(b"\r") or not raw.endswith(b"\r\n"):
        f.append("records must end with CRLF")
    rows = list(csv.reader(io.StringIO(raw.decode("utf-8"), newline="")))
    if not rows or rows[0] != HEADER:
        print("FAIL header mismatch")
        return 1
    body = rows[1:]
    if not body or body[-1][0] != "#END" or body[-1][-1] != str(len(body) - 1):
        f.append("missing or wrong #END sentinel")
    body = [r for r in body if r and r[0] != "#END"]
    tree = set(subprocess.run(["git", "-C", a.repo_root, "ls-tree", "-r", "-z", "--name-only", FREEZE],
                              capture_output=True, text=True).stdout.split("\0"))
    dirs = {"/".join(t.split("/")[:i]) for t in tree for i in range(1, t.count("/") + 1)}
    ids = set()
    covered = set()
    part = json.load(open(a.partition))
    prefixes = part[a.area] if a.area != "SHELL" else ["apps/desktop/"]
    for r in body:
        if len(r) != len(HEADER):
            f.append(f"record {r[:1]} has {len(r)} fields")
            continue
        cid, area, kind, cap, ep, tests, notes = r
        if area != a.area:
            f.append(f"{cid}: Area {area!r} must be {a.area}")
        if not re.match(rf"^CAP-{a.area}-\d{{3}}$", cid):
            f.append(f"{cid}: ID must be CAP-{a.area}-NNN")
        if cid in ids:
            f.append(f"{cid}: duplicate ID")
        ids.add(cid)
        if kind not in KINDS:
            f.append(f"{cid}: Kind {kind!r}")
        if len(cap) > 200:
            f.append(f"{cid}: Capability over 200 characters")
        m = OWNER_TOKEN.search(cap + " " + notes)
        if m:
            f.append(f"{cid}: ownership token {m.group(1)!r} in Capability/Notes")
        for col, val in (("EntryPoints", ep), ("Tests", tests)):
            for tok in filter(None, (t.strip() for t in val.split(";"))):
                if tok == "NONE_FOUND":
                    continue
                p = re.split(r"::|#L", tok, maxsplit=1)[0].rstrip("/")
                if p not in tree and p not in dirs:
                    f.append(f"{cid}: {col} path not at the freeze: {p}")
                    continue
                if col != "EntryPoints":
                    # test data (fixtures) listed under Tests counts toward coverage of test-data files only
                    if re.search(r"(fixture|/fixtures?/|\.snap$)", p, re.I):
                        covered |= {t for t in tree if t == p or t.startswith(p + "/")}
                    continue
                if p in dirs and p not in tree:
                    rel = p[len(P):] if p.startswith(P) else p
                    anc = [x for x in prefixes if x.rstrip("/").startswith(rel.rstrip("/") + "/") or x.rstrip("/") == rel.rstrip("/")]
                    if a.area not in DIR_OK_AREAS:
                        f.append(f"{cid}: directory EntryPoint {p} (only DATA, DOCS and CHECKS may cite directories)")
                        continue
                    if anc or not rel:
                        f.append(f"{cid}: EntryPoint {p} is (or contains) a whole area prefix; name the family")
                        continue
                covered |= {t for t in tree if t == p or t.startswith(p + "/")}
    if a.coverage:
        prefixes = part[a.area]
        mine = []
        for t in tree:
            if not t.startswith(P):
                continue
            rel = t[len(P):]
            if rel.startswith(EXCLUDE) or "/node_modules/" in rel or re.search(r"(\.test|\.spec)\.(ts|tsx|mjs|js)$|(^|/)test_[^/]+\.py$|/tests/[^/]+\.rs$|(^|/)\.git(ignore|attributes)$", rel):
                continue
            if a.area == "SHELL":
                if rel.startswith("apps/desktop/") and not rel.startswith("apps/desktop/src/features/"):
                    mine.append(t)
            elif any(rel == x.rstrip("/") or rel.startswith(x if x.endswith("/") else x + "/") for x in prefixes):
                mine.append(t)
        missing = sorted(set(mine) - covered)
        if missing:
            f.append(f"{len(missing)} of {len(mine)} area files not in any EntryPoints or Tests cell, e.g. {missing[:4]}")
    for x in f:
        print("FINDING", x)
    print(f"{'PASS' if not f else 'FAIL'} INV_{a.area}: {len(body)} capabilities, {len(f)} findings")
    return 1 if f else 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
