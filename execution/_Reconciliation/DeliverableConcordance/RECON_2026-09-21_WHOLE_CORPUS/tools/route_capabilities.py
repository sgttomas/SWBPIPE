#!/usr/bin/env python3
"""Route reverse-inventory capabilities to packages for the R2 reverse pass.

R0 ruling item 1 adopts R0_REVIEW.md §6: each worker answers every capability
in its package's areas, plus a random 10% cross-area sample. Routing decides
only which capabilities a worker is asked about. It never suggests an answer.

A package's areas are the union of:
  (a) AFFINITY below: Agent 0's table from the package's name and scope in
      SOFTWARE_DECOMP rev 0.12, written because declared paths in deliverable
      records name mostly documentation, not code;
  (b) every area its deliverable records name at least twice (path hints).
Every area goes to at least two packages. The 10% cross-area sample is
deterministic: a capability is sampled when int(sha256(package|capability ID))
is divisible by 10.

Writes ROUTING/<PKG>_capabilities.csv, each with the inventory header and an
`#END` sentinel. Worker files must not reveal which rows are sampled
(R0_REVIEW.md §6 uses the sample to catch anchored NOT_MINE answers), so they
carry no Area, Kind or Routing column, rows are ordered by
sha256(package|capability ID), and the CapabilityID column holds a
routing-local ID `RC-<nn>-<NNNN>` (the inventory ID encodes the area). The
mapping from routing-local ID to inventory ID, area and AREA/SAMPLE goes to
ROUTING_SAMPLE/SAMPLE_MANIFEST.csv, which only verifiers and Agent 0 read.

Usage
  route_capabilities.py --run-dir <run> --path-hints ROUTING_PATH_HINTS.json
"""

from __future__ import annotations

import argparse
import collections
import csv
import hashlib
import json
import os
import sys

AFFINITY = {
    "PKG-00": ["SHELL", "WSUI", "VIEW", "COREB", "CHECKS", "DATA", "DOCS"],
    "PKG-01": ["DOCS", "CHECKS"],
    "PKG-02": ["DATA", "COREB", "DOCS"],
    "PKG-03": ["DATA", "COREB", "SOLVER", "FEATB", "FEATC", "DOCS"],
    "PKG-04": ["SOLVER", "PHYS", "CHECKS", "FEATC", "DOCS"],
    "PKG-05": ["SOLVER", "PHYS", "FEATB", "FEATC", "DOCS"],
    "PKG-06": ["COREC", "FEATB", "DATA", "DOCS"],
    "PKG-07": ["VIEW", "WSUI", "FEATB", "FEATC", "SHELL", "DOCS"],
    "PKG-08": ["COREC", "FEATB", "FEATC", "DATA", "DOCS"],
    "PKG-09": ["CHECKS", "DATA", "SOLVER", "PHYS", "FEATB", "FEATC", "DOCS"],
    "PKG-10": ["CHECKS", "SHELL", "PHYS", "FEATC", "COREC", "DATA", "DOCS"],
    "PKG-11": ["DOCS", "DATA"],
    "PKG-12": ["COREC", "FEATB", "FEATC", "SHELL", "DOCS"],
    "PKG-13": ["COREB", "PHYS", "DATA", "FEATC", "DOCS"],
    "PKG-14": ["COREC", "PHYS", "FEATC", "DATA", "DOCS"],
    "PKG-15": ["COREC", "FEATC", "DATA", "DOCS"],
    "PKG-16": ["COREB", "WSUI", "VIEW", "FEATC", "DATA", "DOCS"],
    "PKG-17": ["COREC", "FEATC", "FEATB", "DATA", "DOCS"],
}
HEADER = ["CapabilityID", "Capability", "EntryPoints", "Tests", "Notes"]


def main(argv):
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--path-hints", required=True, help="JSON {PKG: {AREA: mentions}} from deliverable records")
    a = ap.parse_args(argv)
    caps = [r for r in csv.DictReader(open(f"{a.run_dir}/IMPLEMENTATION_SURFACES.csv", newline="", encoding="utf-8"))
            if r["CapabilityID"] != "#END"]
    hints = json.load(open(a.path_hints))
    os.makedirs(f"{a.run_dir}/ROUTING", exist_ok=True)
    os.makedirs(f"{a.run_dir}/ROUTING_SAMPLE", exist_ok=True)
    served = collections.Counter()
    summary = []
    manifest = []
    order = lambda pkg, r: hashlib.sha256(f"{pkg}|{r['CapabilityID']}".encode()).hexdigest()
    for pkg in sorted(AFFINITY):
        areas = set(AFFINITY[pkg]) | {ar for ar, n in hints.get(pkg, {}).items() if n >= 2}
        for ar in areas:
            served[ar] += 1
        chosen = []
        for r in caps:
            if r["Area"] in areas:
                chosen.append((r, "AREA"))
            elif int(hashlib.sha256(f"{pkg}|{r['CapabilityID']}".encode()).hexdigest(), 16) % 10 == 0:
                chosen.append((r, "SAMPLE"))
        chosen.sort(key=lambda x: order(pkg, x[0]))
        with open(f"{a.run_dir}/ROUTING/{pkg}_capabilities.csv", "w", newline="", encoding="utf-8") as fh:
            w = csv.writer(fh, lineterminator="\r\n")
            w.writerow(HEADER)
            for i, (r, why) in enumerate(chosen, 1):
                rid = f"RC-{pkg[4:]}-{i:04d}"
                w.writerow([rid] + [r[h] for h in HEADER[1:]])
                manifest.append([pkg, rid, r["CapabilityID"], r["Area"], why])
            w.writerow(["#END"] + [""] * (len(HEADER) - 2) + [str(len(chosen))])
        summary.append((pkg, sorted(areas), len(chosen), sum(1 for _, w in chosen if w == "SAMPLE")))
    with open(f"{a.run_dir}/ROUTING_SAMPLE/SAMPLE_MANIFEST.csv", "w", newline="", encoding="utf-8") as fh:
        w = csv.writer(fh, lineterminator="\r\n")
        w.writerow(["PackageID", "RoutedID", "CapabilityID", "Area", "Routing"])
        w.writerows(manifest)
        w.writerow(["#END", "", "", "", str(len(manifest))])
    for pkg, areas, n, s in summary:
        print(f"{pkg}: {n} capabilities ({s} sampled) from {','.join(areas)}")
    thin = [ar for ar in {r['Area'] for r in caps} if served[ar] < 2]
    print("areas served by fewer than two packages:", thin or "none")
    return 1 if thin else 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
