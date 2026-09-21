#!/usr/bin/env python3
"""Build ROUTING_PATH_HINTS.json: per package, how often its deliverable records
name a path in each R1 area (input (b) to route_capabilities.py).

Reads each deliverable folder's scope and status records at the freeze commit,
finds repository paths, and assigns each to an area using the R1 partition
(briefs/R1-INV_areas.json). SHELL is the remainder of apps/desktop/.

Usage
  build_path_hints.py --run-dir <run> --areas <R1-INV_areas.json> --commit <sha> --out <json>
"""

from __future__ import annotations

import argparse
import collections
import csv
import json
import re
import subprocess
import sys

FILES = ["ScopeOfWork.md", "ArchitectureBasis.md", "_STATUS.md", "_CONTEXT.md",
         "Palette_Operation_Routing.md", "Palette_Organization_Contract.md"]
PATH = re.compile(r"(?:projects/chirality-piping/)?((?:core|apps|schemas|fixtures|tools|validation|docs|governance|examples|api|provenance)/[A-Za-z0-9_./\-]+)")


def main(argv):
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--areas", required=True)
    ap.add_argument("--commit", required=True)
    ap.add_argument("--out", required=True)
    a = ap.parse_args(argv)
    areas = {k: v for k, v in json.load(open(a.areas)).items() if not k.startswith("_")}

    def area_of(rel):
        for k, v in areas.items():
            if k == "SHELL":
                continue
            if any(rel == x.rstrip("/") or rel.startswith(x if x.endswith("/") else x + "/") for x in v):
                return k
        return "SHELL" if rel.startswith("apps/desktop/") else None

    counts = collections.defaultdict(collections.Counter)
    for d in csv.DictReader(open(f"{a.run_dir}/DELIVERABLE_INVENTORY.csv", newline="", encoding="utf-8")):
        for fn in FILES:
            t = subprocess.run(["git", "show", f"{a.commit}:{d['Folder']}{fn}"], capture_output=True, text=True).stdout
            for m in PATH.findall(t):
                ar = area_of(m.rstrip(".,)`"))
                if ar:
                    counts[d["PackageID"]][ar] += 1
    json.dump({p: dict(c) for p, c in counts.items()}, open(a.out, "w"))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
