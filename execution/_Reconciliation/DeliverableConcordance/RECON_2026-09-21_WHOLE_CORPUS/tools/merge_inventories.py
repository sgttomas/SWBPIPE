#!/usr/bin/env python3
"""Merge the 12 R1 area inventories into IMPLEMENTATION_SURFACES.csv.

Rule: concatenate each inventory's body rows (its `#END` sentinel dropped) in
partition order, then write one `#END` sentinel whose last field is the total.

Usage
  merge_inventories.py --run-dir <run>
"""

from __future__ import annotations

import argparse
import csv
import sys

ORDER = ["VIEW", "WSUI", "FEATB", "FEATC", "SHELL", "SOLVER", "PHYS", "COREB", "COREC", "DATA", "CHECKS", "DOCS"]


def main(argv):
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-dir", required=True)
    a = ap.parse_args(argv)
    header, rows = None, []
    for ar in ORDER:
        with open(f"{a.run_dir}/R1_INVENTORY/INV_{ar}.csv", newline="", encoding="utf-8") as fh:
            rd = csv.reader(fh)
            h = next(rd)
            header = header or h
            if h != header:
                raise SystemExit(f"header mismatch in INV_{ar}.csv")
            rows += [r for r in rd if r and r[0] != "#END"]
    with open(f"{a.run_dir}/IMPLEMENTATION_SURFACES.csv", "w", newline="", encoding="utf-8") as fh:
        w = csv.writer(fh, lineterminator="\r\n")
        w.writerow(header)
        w.writerows(rows)
        w.writerow(["#END"] + [""] * (len(header) - 2) + [str(len(rows))])
    print(f"{len(rows)} capabilities")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
