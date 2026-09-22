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
- NO_ACTION_ROWS.csv     every NO_ACTION row or item (classes, T8, T9, T11, T12) with its stated reason
- CAPABILITY_DISPOSITIONS.csv  the T1-T3 disposition of every non-OWNED capability
- T8_ROUTE_DISAGREEMENTS.csv   T8 rows whose route differs from the class route (or that are not divergent)
- CLASS_ROUTE_TOTALS.md  rows and classes per route and per owning authority, routing gaps, register counts

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
            hdr = None if out else hdr  # a table ends at the first non-table line after its rows
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
            named = list(dict.fromkeys(re.findall(r"DEL-\d\d-\d\d", r["ProposedOwner"])))
            for i, dl in enumerate(named):
                pkg = "PKG-" + dl[4:6]
                a = area[r["CapabilityID"]]
                gaps.append({"CapabilityID": r["CapabilityID"], "Area": a, "Source": f.split("_")[0],
                             "Classification": r["Classification"], "ProposedOwner": dl,
                             "Rank": "PRIMARY" if i == 0 else "CANDIDATE",
                             "OwnerPackageAreaRouted": "YES" if pkg in routed[a] else "NO",
                             "OwnerPackageRoutedCapability": "YES" if pkg in cap_routed[r["CapabilityID"]] else "NO"})
    gaps.sort(key=lambda g: (g["CapabilityID"], g["ProposedOwner"]))

    by_route = collections.Counter(a["Route"] for a in assign)
    cls_route = collections.Counter(i["Route"] for i in index)
    by_auth = collections.Counter(a["Authority"] for a in assign)
    prim = [g for g in gaps if g["Rank"] == "PRIMARY"]
    rg = [g for g in prim if g["Classification"] == "ROUTING_GAP"]
    lines = ["# R3 class routes and routing gaps (generated by tools/index_r3_classes.py; do not edit)", "",
             f"- Classes: {len(index)} across T4A-T7; divergent rows assigned: {len(assign)} (exactly once)", "",
             "| Route | Classes | Rows |", "|---|---:|---:|"]
    lines += [f"| {k} | {cls_route[k]} | {by_route[k]} |" for k in sorted(by_route)]
    lines += ["", "| Owning authority | Rows |", "|---|---:|"] + [f"| {k} | {v} |" for k, v in sorted(by_auth.items())]
    lines += ["", "## Routing gaps (T1-T3 proposed owners)", "",
              f"- Deliverables named in ProposedOwner: {len(gaps)} links ({len(prim)} primary = first named, "
              f"{len(gaps) - len(prim)} further candidates or alternatives)",
              f"- Primary owners whose package was never AREA-routed the capability's area: "
              f"{sum(1 for g in prim if g['OwnerPackageAreaRouted'] == 'NO')}; never routed the capability at all: "
              f"{sum(1 for g in prim if g['OwnerPackageRoutedCapability'] == 'NO')}",
              f"- ROUTING_GAP capabilities: {len(rg)}; primary owner's package never routed the capability: "
              f"{sum(1 for g in rg if g['OwnerPackageRoutedCapability'] == 'NO')} of {len(rg)}", "",
              "| Area | Packages AREA-routed |", "|---|---|"]
    lines += [f"| {a} | {', '.join(sorted(p))} |" for a, p in sorted(routed.items())]
    outs = {
        "CLASS_INDEX.csv": to_csv(["ClassID", "Task", "Name", "Rows", "Authority", "Route"], index),
        "CLASS_ASSIGNMENTS.csv": to_csv(["ClaimKey", "DeliverableID", "PackageID", "Disposition", "CauseTag",
                                         "AuthorityTier", "ClassID", "Route", "Authority"], assign),
        "ROUTING_GAPS.csv": to_csv(["CapabilityID", "Area", "Source", "Classification", "ProposedOwner", "Rank",
                                    "OwnerPackageAreaRouted", "OwnerPackageRoutedCapability"], gaps),
    }
    outs.update(extra_tables(run, assign, {i["ClassID"]: i for i in index}))
    na = [l for l in outs["NO_ACTION_ROWS.csv"].splitlines()[1:-1]]
    lines += ["", "## Registers", "",
              f"- NO_ACTION rows (explicit no-repair accounting): {len(na)} "
              f"({', '.join(f'{k} {v}' for k, v in sorted(collections.Counter(l.split(',')[0] for l in na).items()))})",
              f"- Capability dispositions (T1-T3): {len(outs['CAPABILITY_DISPOSITIONS.csv'].splitlines()) - 2}",
              f"- T8 rows whose route differs from the class route: {len(outs['T8_ROUTE_DISAGREEMENTS.csv'].splitlines()) - 2}"]
    lines += ["", "## Outputs (SHA-256)", ""] + [f"- `R3/{k}` {hashlib.sha256(v.encode()).hexdigest()}" for k, v in outs.items()]
    outs["CLASS_ROUTE_TOTALS.md"] = "\n".join(lines) + "\n"
    return outs


def extra_tables(run, assign, index):
    """NO_ACTION register (method R6: no-repair rows recorded explicitly), final capability dispositions,
    and the rows where T8's route differs from the class route."""
    t = f"{run}/R3/TASKS"
    na = [{"Source": "CLASS", "Key": a["ClaimKey"], "DeliverableID": a["DeliverableID"], "Group": a["ClassID"],
           "Reason": index[a["ClassID"]]["Name"]} for a in assign if a["Route"] == "NO_ACTION"]
    for r in read(f"{t}/T8_ROWS.csv"):
        if r["Route"] == "NO_ACTION":
            na.append({"Source": "T8", "Key": r["ClaimKey"], "DeliverableID": r["DeliverableID"], "Group": r["Cluster"],
                       "Reason": r["ProposedReading"]})
    for r in read(f"{t}/T9_LIFECYCLE.csv"):
        if r["Route"] == "NO_ACTION":
            na.append({"Source": "T9", "Key": r["Item"], "DeliverableID": r["DeliverableID"],
                       "Group": r["Finding"].split(";")[0], "Reason": r["Finding"]})
    for r in read(f"{t}/T11_METHOD.csv"):
        if r["Route"] == "NO_ACTION":
            na.append({"Source": "T11", "Key": r["Subject"], "DeliverableID": r["Deliverables"], "Group": r["Check"],
                       "Reason": r["Finding"]})
    for r in read(f"{t}/T12_UNREACHED.csv"):
        if r["Route"] == "NO_ACTION":
            na.append({"Source": "T12", "Key": r["ClaimKey"], "DeliverableID": r["DeliverableID"], "Group": r["ClusterID"],
                       "Reason": f"engine {r['Engine']} ({r['Area']}); see T12_UNREACHED.md {r['ClusterID']}"})
    caps = []
    for f in ["T1_UNMAPPED.csv", "T2_UNMAPPED.csv", "T3_OWNERSHIP.csv"]:
        for r in read(f"{t}/{f}"):
            caps.append({"CapabilityID": r["CapabilityID"], "Area": r["Area"], "Status": r["Status"],
                         "Source": f.split("_")[0], "Classification": r["Classification"],
                         "ProposedOwner": r["ProposedOwner"], "Confidence": r["Confidence"]})
    caps.sort(key=lambda c: c["CapabilityID"])
    cls = {a["ClaimKey"]: a for a in assign}
    dis = []
    for r in read(f"{t}/T8_ROWS.csv"):
        a = cls.get(r["ClaimKey"])
        croute = a["Route"] if a else "NOT_DIVERGENT"
        if croute != r["Route"]:
            dis.append({"ClaimKey": r["ClaimKey"], "DeliverableID": r["DeliverableID"], "Cluster": r["Cluster"],
                        "T8Route": r["Route"], "ClassRoute": croute, "ClassID": a["ClassID"] if a else ""})
    return {
        "NO_ACTION_ROWS.csv": to_csv(["Source", "Key", "DeliverableID", "Group", "Reason"], na),
        "CAPABILITY_DISPOSITIONS.csv": to_csv(["CapabilityID", "Area", "Status", "Source", "Classification",
                                               "ProposedOwner", "Confidence"], caps),
        "T8_ROUTE_DISAGREEMENTS.csv": to_csv(["ClaimKey", "DeliverableID", "Cluster", "T8Route", "ClassRoute", "ClassID"], dis),
    }


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
