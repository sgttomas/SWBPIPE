#!/usr/bin/env python3
"""R3 class index and routing-gap measure (deterministic, read-only on inputs).

Inputs (under RUN): R3/CORPUS_CLAIMS.csv, R3/CAPABILITY_COVERAGE.csv, ROUTING_SAMPLE/SAMPLE_MANIFEST.csv,
and the R3 task outputs R3/TASKS/T4A..T7_CLASSES.{csv,md} and R3/TASKS/T1_UNMAPPED.csv, T2_UNMAPPED.csv,
T3_OWNERSHIP.csv.

Outputs (under RUN/R3/):
- CLASS_INDEX.csv        one row per T4A-T7 class: task, name, rows, owning authority, route (from the
                         class summary table in each *_CLASSES.md; row counts are checked against the CSVs)
- CLASS_ASSIGNMENTS.csv  one row per divergent claim: effective values plus ClassID and the class route
- ROUTING_GAPS.csv       one row per T1-T3 capability whose proposed owner is a deliverable: whether that
                         deliverable's package was AREA-routed the capability's area, and whether the
                         owner was routed the capability at all
- CLASS_ROUTE_TOTALS.md  rows and classes per route and per owning authority, and the routing-gap summary

Usage: python3 tools/index_r3_classes.py --run-dir RUN [--check]
"""
import argparse
import collections
import csv
import hashlib
import io
import os
import re
import sys

TASKS = ["T4A", "T4B", "T5A", "T5B", "T6", "T7"]


def read(path):
    with open(path, newline="", encoding="utf-8") as fh:
        return [r for r in csv.DictReader(fh) if list(r.values())[0] != "#END"]


def to_csv(header, rows):
    buf = io.StringIO()
    w = csv.writer(buf, lineterminator="\r\n")
    w.writerow(header)
    for r in rows:
        w.writerow([r.get(h, "") for h in header])
    w.writerow(["#END"] + [""] * (len(header) - 2) + [str(len(rows))])
    return buf.getvalue()


def class_table(md_path, task):
    hdr, out = None, {}
    for line in open(md_path, encoding="utf-8").read().splitlines():
        if not line.startswith("|"):
            continue
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        low = [c.lower() for c in cells]
        if low and low[0] in ("id", "class", "classid") and any("route" in c for c in low):
            hdr = low
            continue
        if hdr and cells and re.search(r"C\d\d", cells[0]):
            cid = f"{task}-" + re.search(r"(C\d\d)", cells[0]).group(1)
            d = dict(zip(hdr, cells))
            auth = next((d[h] for h in hdr if "authority" in h), "")
            out[cid] = {"ClassID": cid, "Task": task, "Name": d.get("name", ""),
                        "Rows": re.sub(r"[^0-9]", "", d.get("rows", "")),
                        "Authority": re.sub(r"[`*]", "", auth), "Route": re.sub(r"[`*]", "", d.get("route", ""))}
    return out


def build(run):
    corp = {r["ClaimKey"]: r for r in read(f"{run}/R3/CORPUS_CLAIMS.csv")}
    index, assign = [], []
    for t in TASKS:
        tab = class_table(f"{run}/R3/TASKS/{t}_CLASSES.md", t)
        rows = read(f"{run}/R3/TASKS/{t}_CLASSES.csv")
        cnt = collections.Counter(r["ClassID"] for r in rows)
        assert set(cnt) == set(tab), (t, set(cnt) ^ set(tab))
        for cid, meta in sorted(tab.items()):
            assert meta["Rows"] == str(cnt[cid]), (cid, meta["Rows"], cnt[cid])
            index.append(meta)
        for r in rows:
            c = corp[r["ClaimKey"]]
            assign.append({"ClaimKey": r["ClaimKey"], "DeliverableID": c["DeliverableID"], "PackageID": c["PackageID"],
                           "Disposition": c["Disposition"], "CauseTag": c["CauseTag"], "AuthorityTier": c["AuthorityTier"],
                           "ClassID": r["ClassID"], "Route": tab[r["ClassID"]]["Route"],
                           "Authority": tab[r["ClassID"]]["Authority"]})
    div = {k for k, c in corp.items() if c["Divergent"] == "YES"}
    keys = [a["ClaimKey"] for a in assign]
    assert len(keys) == len(set(keys)) and set(keys) == div, "class assignments must cover divergent rows exactly once"
    assign.sort(key=lambda a: a["ClaimKey"])

    # routing gaps: which areas were AREA-routed to which packages
    routed = collections.defaultdict(set)      # area -> packages AREA-routed
    cap_routed = collections.defaultdict(set)  # capability -> packages routed (any)
    for r in read(f"{run}/ROUTING_SAMPLE/SAMPLE_MANIFEST.csv"):
        cap_routed[r["CapabilityID"]].add(r["PackageID"])
        if r["Routing"] == "AREA":
            routed[r["Area"]].add(r["PackageID"])
    area = {r["CapabilityID"]: r["Area"] for r in read(f"{run}/R3/CAPABILITY_COVERAGE.csv")}
    gaps = []
    for f in ["T1_UNMAPPED.csv", "T2_UNMAPPED.csv", "T3_OWNERSHIP.csv"]:
        for r in read(f"{run}/R3/TASKS/{f}"):
            for dl in sorted(set(re.findall(r"DEL-\d\d-\d\d", r["ProposedOwner"]))):
                pkg = "PKG-" + dl[4:6]
                a = area[r["CapabilityID"]]
                gaps.append({"CapabilityID": r["CapabilityID"], "Area": a, "Source": f.split("_")[0],
                             "Classification": r["Classification"], "ProposedOwner": dl,
                             "OwnerPackageAreaRouted": "YES" if pkg in routed[a] else "NO",
                             "OwnerPackageRoutedCapability": "YES" if pkg in cap_routed[r["CapabilityID"]] else "NO"})
    gaps.sort(key=lambda g: (g["CapabilityID"], g["ProposedOwner"]))

    by_route = collections.Counter(a["Route"] for a in assign)
    cls_route = collections.Counter(i["Route"] for i in index)
    by_auth = collections.Counter(a["Authority"] for a in assign)
    rg = [g for g in gaps if g["Classification"] == "ROUTING_GAP"]
    lines = ["# R3 class routes and routing gaps (generated by tools/index_r3_classes.py; do not edit)", "",
             f"- Classes: {len(index)} across T4A-T7; divergent rows assigned: {len(assign)} (exactly once)", "",
             "| Route | Classes | Rows |", "|---|---:|---:|"]
    lines += [f"| {k} | {cls_route[k]} | {by_route[k]} |" for k in sorted(by_route)]
    lines += ["", "| Owning authority | Rows |", "|---|---:|"] + [f"| {k} | {v} |" for k, v in sorted(by_auth.items())]
    lines += ["", "## Routing gaps (T1-T3 proposed owners)", "",
              f"- Proposed-owner links: {len(gaps)}; owner package never AREA-routed the capability's area: "
              f"{sum(1 for g in gaps if g['OwnerPackageAreaRouted'] == 'NO')}; owner package not routed the capability at all: "
              f"{sum(1 for g in gaps if g['OwnerPackageRoutedCapability'] == 'NO')}",
              f"- ROUTING_GAP capabilities: {len({g['CapabilityID'] for g in rg})}; of their owner links, "
              f"{sum(1 for g in rg if g['OwnerPackageRoutedCapability'] == 'NO')} of {len(rg)} were never routed to the owner's package", "",
              "| Area | Packages AREA-routed |", "|---|---|"]
    lines += [f"| {a} | {', '.join(sorted(p))} |" for a, p in sorted(routed.items())]
    outs = {
        "CLASS_INDEX.csv": to_csv(["ClassID", "Task", "Name", "Rows", "Authority", "Route"], index),
        "CLASS_ASSIGNMENTS.csv": to_csv(["ClaimKey", "DeliverableID", "PackageID", "Disposition", "CauseTag",
                                         "AuthorityTier", "ClassID", "Route", "Authority"], assign),
        "ROUTING_GAPS.csv": to_csv(["CapabilityID", "Area", "Source", "Classification", "ProposedOwner",
                                    "OwnerPackageAreaRouted", "OwnerPackageRoutedCapability"], gaps),
    }
    lines += ["", "## Outputs (SHA-256)", ""] + [f"- `R3/{k}` {hashlib.sha256(v.encode()).hexdigest()}" for k, v in outs.items()]
    outs["CLASS_ROUTE_TOTALS.md"] = "\n".join(lines) + "\n"
    return outs


def main(argv):
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--check", action="store_true")
    a = ap.parse_args(argv)
    run = a.run_dir.rstrip("/")
    outs = build(run)
    if a.check:
        bad = [k for k, v in outs.items() if not os.path.exists(f"{run}/R3/{k}")
               or open(f"{run}/R3/{k}", encoding="utf-8", newline="").read() != v]
        print("CHECK " + ("FAIL " + ", ".join(bad) if bad else "PASS: all R3 class-index outputs reproduce"))
        return 1 if bad else 0
    for k, v in outs.items():
        with open(f"{run}/R3/{k}", "w", encoding="utf-8", newline="") as fh:
            fh.write(v)
    print(outs["CLASS_ROUTE_TOTALS.md"])
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
