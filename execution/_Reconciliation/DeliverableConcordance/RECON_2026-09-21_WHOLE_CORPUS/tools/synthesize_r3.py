#!/usr/bin/env python3
"""R3 deterministic synthesis for RECON_2026-09-21_WHOLE_CORPUS.

Reads only accepted R2 outputs and writes only under R3/:
- the current sealed forward and reverse ledgers, WAVES/W*/PKG-*/DEL-*/ (superseded_<n>/ excluded);
- the combined adopted resolutions, WAVES/CROSS_WAVE/ALL_WAVES_RESOLUTIONS_COMBINED.csv;
- DELIVERABLE_INVENTORY.csv, IMPLEMENTATION_SURFACES.csv and ROUTING_SAMPLE/SAMPLE_MANIFEST.csv.

Sealed ledgers are never edited. A resolution row that sets a Disposition replaces the five
value fields (Disposition, CauseTag, AuthorityTier, BaselineClass, DivergenceLayers) in the
effective view; every resolution row for a key is listed in ResolutionClasses, so contested,
field and observed rows stay visible.

Outputs (CSV, CRLF, `#END` sentinel carrying the body count):
- CORPUS_CLAIMS.csv        one row per claim key (all 102 deliverables), sealed and effective values
- PACKAGE_SUMMARY.csv      effective disposition counts per package and in total
- CLUSTER_MATRIX.csv       non-aligned effective rows by CauseTag x AuthorityTier x Disposition
- CAPABILITY_COVERAGE.csv  one row per inventory capability: routed answers, owners, coverage status
- REMAINING_CENSUS.csv     one row per deliverable: recorded Remaining items and their effective status
- SYNTHESIS_STATS.md       counts and input/output hashes (the reproduction record)

Usage: python3 tools/synthesize_r3.py --run-dir RUN [--check]
--check recomputes everything and fails if any committed output differs byte for byte.
"""
import argparse
import collections
import csv
import glob
import hashlib
import io
import os
import re
import sys

VALUE_FIELDS = ["Disposition", "CauseTag", "AuthorityTier", "BaselineClass", "DivergenceLayers"]
# Dispositions that assert no divergence, or carry no assessable claim.
NEUTRAL = {"ALIGNED", "NOT_ASSESSED", "COVERED_BY_CHILDREN"}
OWNERSHIP = {"CLAIMED_BY", "PARTIAL", "UNKEYED"}
RELATION = {"COVERS", "CONSTRAINS"}


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest()


def read_csv(path):
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


def ledgers(run, kind):
    out = sorted(glob.glob(f"{run}/WAVES/W*/PKG-*/DEL-*/DEL-*_{kind}.csv"))
    assert not any("superseded" in p for p in out)
    return out


def build(run):
    inv = {r["DeliverableID"]: r for r in read_csv(f"{run}/DELIVERABLE_INVENTORY.csv")}
    res_path = f"{run}/WAVES/CROSS_WAVE/ALL_WAVES_RESOLUTIONS_COMBINED.csv"
    res = collections.defaultdict(list)
    for r in read_csv(res_path):
        res[r["ClaimKey"]].append(r)

    fwd = ledgers(run, "forward")
    rev = ledgers(run, "reverse")
    assert len(fwd) == len(inv) == len(rev), (len(fwd), len(inv), len(rev))

    claims = []
    seen = set()
    for path in fwd:
        wave = path.split("/WAVES/")[1].split("/")[0]
        for r in read_csv(path):
            assert r["ClaimKey"] not in seen, r["ClaimKey"]
            seen.add(r["ClaimKey"])
            row = {"Wave": wave, "PackageID": inv[r["DeliverableID"]]["PackageID"]}
            row.update({k: r[k] for k in ["ClaimKey", "DeliverableID", "UnitKind", "ClaimType", "ClaimClass",
                                          "LifecycleState", "CanonicalSituation", "FindingGroup",
                                          "AuthorityNeeded", "Confidence"]})
            for k in VALUE_FIELDS:
                row["Sealed" + k] = r[k]
                row[k] = r[k]
            rr = res.get(r["ClaimKey"], [])
            setters = [x for x in rr if x["Disposition"]]
            assert len({tuple(x[k] for k in VALUE_FIELDS) for x in setters}) <= 1, r["ClaimKey"]
            if setters:
                for k in VALUE_FIELDS:
                    row[k] = setters[0][k]
            row["ResolutionClasses"] = ";".join(sorted({x["Class"] for x in rr}))
            # Only the five value fields are substituted; every other correction a resolution states
            # (AuthorityNeeded, CanonicalSituation, FindingGroup, RemainingWork, evidence, Notes...) is
            # carried verbatim so R3 readers apply it by judgment.
            row["OtherCorrections"] = " || ".join(f"[{x['Class']}] {x['OtherCorrections']}" for x in rr if x["OtherCorrections"])
            row["ProductCallerNone"] = "YES" if re.search(r"PRODUCT_CALLER:\s*NONE", r["Notes"]) else "NO"
            row["ValuesResolved"] = "YES" if setters and any(row[k] != row["Sealed" + k] for k in VALUE_FIELDS) else "NO"
            row["Divergent"] = "NO" if row["Disposition"] in NEUTRAL else "YES"
            row["Remaining"] = "YES" if "#remaining/" in r["ClaimKey"] or r["ClaimType"] == "REMAINING_WORK" else "NO"
            claims.append(row)
    unused = sorted(set(res) - seen)
    assert not unused, unused[:5]

    # package summary
    disps = sorted({c["Disposition"] for c in claims})
    per = collections.defaultdict(collections.Counter)
    for c in claims:
        per[c["PackageID"]][c["Disposition"]] += 1
        per["TOTAL"][c["Disposition"]] += 1
    pkg_rows = []
    for p in sorted(per, key=lambda x: (x == "TOTAL", x)):
        d = {"PackageID": p, "Rows": sum(per[p].values())}
        d.update({k: per[p][k] for k in disps})
        pkg_rows.append(d)

    # cluster matrix over divergent effective rows
    cm = collections.Counter((c["CauseTag"], c["AuthorityTier"], c["Disposition"]) for c in claims if c["Divergent"] == "YES")
    cm_pkgs = collections.defaultdict(set)
    for c in claims:
        if c["Divergent"] == "YES":
            cm_pkgs[(c["CauseTag"], c["AuthorityTier"], c["Disposition"])].add(c["PackageID"])
    cluster_rows = [{"CauseTag": k[0], "AuthorityTier": k[1], "Disposition": k[2], "Rows": n,
                     "Packages": ";".join(sorted(cm_pkgs[k]))}
                    for k, n in sorted(cm.items(), key=lambda kv: (-kv[1], kv[0]))]

    # capability coverage
    caps = {r["CapabilityID"]: r for r in read_csv(f"{run}/IMPLEMENTATION_SURFACES.csv")}
    manifest = {(r["PackageID"], r["RoutedID"]): r for r in read_csv(f"{run}/ROUTING_SAMPLE/SAMPLE_MANIFEST.csv")}
    answers = collections.defaultdict(list)
    for path in rev:
        dl = os.path.basename(path).split("_")[0]
        pkg = inv[dl]["PackageID"]
        for r in read_csv(path):
            m = manifest[(pkg, r["CapabilityID"])]
            answers[m["CapabilityID"]].append((dl, m["Routing"], r["Answer"], r["ClaimKey"]))
    cov_rows = []
    for cid in sorted(caps):
        a = answers.get(cid, [])
        own = [x for x in a if x[2] in OWNERSHIP]
        full = [x for x in a if x[2] == "CLAIMED_BY"]
        rel = [x for x in a if x[2] in RELATION]
        if not a:
            status = "NOT_ROUTED"
        elif not own:
            status = "UNMAPPED_RELATION_ONLY" if rel else "UNMAPPED"
        elif len(full) > 1:
            status = "DUPLICATE_OWNERSHIP"
        elif any(x[2] == "UNKEYED" for x in own) and not full and not any(x[2] == "PARTIAL" for x in own):
            status = "OWNED_UNKEYED"
        elif full:
            status = "OWNED_SHARED" if len(own) > 1 else "OWNED"
        else:
            status = "PARTIAL_ONLY"
        cov_rows.append({
            "CapabilityID": cid, "Area": caps[cid]["Area"], "Kind": caps[cid]["Kind"],
            "Status": status, "RoutedTo": len(a),
            "AreaRouted": sum(1 for x in a if x[1] == "AREA"), "SampleRouted": sum(1 for x in a if x[1] == "SAMPLE"),
            "Owners": ";".join(f"{x[0]}:{x[2]}" for x in sorted(own)),
            "Relations": ";".join(f"{x[0]}:{x[2]}" for x in sorted(rel)),
            "SampleRoutedOwnership": ";".join(x[0] for x in sorted(own) if x[1] == "SAMPLE"),
            "OwnerKeys": " | ".join(f"{x[0]}={x[3]}" for x in sorted(own) if x[3]),
            "Capability": caps[cid]["Capability"]})
    unrouted_answers = sorted(set(answers) - set(caps))
    assert not unrouted_answers, unrouted_answers[:5]

    # remaining census
    rem_rows = []
    by_del = collections.defaultdict(list)
    for c in claims:
        if c["Remaining"] == "YES":
            by_del[c["DeliverableID"]].append(c)
    for dl in sorted(inv):
        rs = by_del.get(dl, [])
        cnt = collections.Counter(c["Disposition"] for c in rs)
        rem_rows.append({"DeliverableID": dl, "PackageID": inv[dl]["PackageID"],
                         "Lifecycle": inv[dl]["Lifecycle"],
                         "InventoryRemainingItems": inv[dl]["RemainingItems"], "RemainingRows": len(rs),
                         "Aligned": cnt["ALIGNED"], "Divergent": sum(1 for c in rs if c["Divergent"] == "YES"),
                         "Dispositions": ";".join(f"{k}={v}" for k, v in sorted(cnt.items())) or "NONE",
                         "Keys": ";".join(c["ClaimKey"].split(":", 1)[1] for c in rs)})

    claim_hdr = ["ClaimKey", "DeliverableID", "PackageID", "Wave", "UnitKind", "ClaimType", "ClaimClass",
                 "LifecycleState", "CanonicalSituation", "FindingGroup", "AuthorityNeeded", "Confidence"] + \
                VALUE_FIELDS + ["Sealed" + k for k in VALUE_FIELDS] + \
                ["ValuesResolved", "ResolutionClasses", "OtherCorrections", "Divergent", "Remaining", "ProductCallerNone"]
    outs = {
        "CORPUS_CLAIMS.csv": to_csv(claim_hdr, claims),
        "PACKAGE_SUMMARY.csv": to_csv(["PackageID", "Rows"] + disps, pkg_rows),
        "CLUSTER_MATRIX.csv": to_csv(["CauseTag", "AuthorityTier", "Disposition", "Rows", "Packages"], cluster_rows),
        "CAPABILITY_COVERAGE.csv": to_csv(["CapabilityID", "Area", "Kind", "Status", "RoutedTo", "AreaRouted",
                                           "SampleRouted", "Owners", "Relations", "SampleRoutedOwnership",
                                           "OwnerKeys", "Capability"], cov_rows),
        "REMAINING_CENSUS.csv": to_csv(["DeliverableID", "PackageID", "Lifecycle", "InventoryRemainingItems",
                                        "RemainingRows", "Aligned", "Divergent", "Dispositions", "Keys"], rem_rows),
    }

    # stats
    st = collections.Counter(r["Status"] for r in cov_rows)
    inputs = [res_path, f"{run}/DELIVERABLE_INVENTORY.csv", f"{run}/IMPLEMENTATION_SURFACES.csv",
              f"{run}/ROUTING_SAMPLE/SAMPLE_MANIFEST.csv"] + fwd + rev
    lines = ["# R3 synthesis statistics (generated by tools/synthesize_r3.py; do not edit)", "",
             f"- Deliverables: {len(inv)}; forward ledgers: {len(fwd)}; reverse files: {len(rev)}",
             f"- Claim rows: {len(claims)}; divergent (effective): {sum(1 for c in claims if c['Divergent'] == 'YES')}; "
             f"values changed by resolutions: {sum(1 for c in claims if c['ValuesResolved'] == 'YES')}",
             f"- Resolution rows: {sum(len(v) for v in res.values())} over {len(res)} keys; "
             f"claim rows with other (non-value) corrections: {sum(1 for c in claims if c['OtherCorrections'])}",
             f"- Rows marked PRODUCT_CALLER: NONE: {sum(1 for c in claims if c['ProductCallerNone'] == 'YES')} "
             f"({sum(1 for c in claims if c['ProductCallerNone'] == 'YES' and c['Divergent'] == 'NO')} not divergent)",
             f"- Capabilities: {len(cov_rows)}; " + "; ".join(f"{k} {v}" for k, v in sorted(st.items())),
             f"- Remaining rows: {sum(r['RemainingRows'] for r in rem_rows)} across "
             f"{sum(1 for r in rem_rows if r['RemainingRows'])} deliverables; "
             f"{sum(1 for r in rem_rows if not r['RemainingRows'])} deliverables record NONE",
             "", "## Inputs (SHA-256)", ""]
    lines += [f"- `{os.path.relpath(p, run)}` {sha(p)}" for p in inputs[:4]]
    agg = hashlib.sha256("".join(sha(p) for p in inputs[4:]).encode()).hexdigest()
    lines += [f"- ledgers: {len(fwd)} forward + {len(rev)} reverse, concatenated-hash {agg}", "", "## Outputs (SHA-256)", ""]
    lines += [f"- `R3/{k}` {hashlib.sha256(v.encode()).hexdigest()}" for k, v in outs.items()]
    outs["SYNTHESIS_STATS.md"] = "\n".join(lines) + "\n"
    return outs


def main(argv):
    ap = argparse.ArgumentParser()
    ap.add_argument("--run-dir", required=True)
    ap.add_argument("--check", action="store_true")
    a = ap.parse_args(argv)
    outs = build(a.run_dir.rstrip("/"))
    od = f"{a.run_dir.rstrip('/')}/R3"
    if a.check:
        bad = [k for k, v in outs.items() if not os.path.exists(f"{od}/{k}") or open(f"{od}/{k}", encoding="utf-8", newline="").read() != v]
        print("CHECK " + ("FAIL " + ", ".join(bad) if bad else "PASS: all R3 synthesis outputs reproduce"))
        return 1 if bad else 0
    os.makedirs(od, exist_ok=True)
    for k, v in outs.items():
        with open(f"{od}/{k}", "w", encoding="utf-8", newline="") as fh:
            fh.write(v)
    print(outs["SYNTHESIS_STATS.md"])
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
