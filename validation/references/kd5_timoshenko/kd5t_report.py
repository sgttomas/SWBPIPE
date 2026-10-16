"""Aggregate kd5t_compare.py output into the M3 tables (after kd5-eval-final/analyze.py and b4-finish/analyze_b4.py).

Usage: python3 -I -B kd5t_report.py <eval.jsonl> [<eval.jsonl> ...] [--by family|theory|placement|mill] [--json out.json]
"""

from __future__ import annotations

import collections
import json
import sys


def load(paths):
    out = []
    for p in paths:
        for line in open(p):
            out.append(json.loads(line))
    return out


def table(recs):
    t = collections.Counter()
    worst = {}
    examples = collections.defaultdict(list)
    for r in recs:
        t["case-modes (runs)"] += 1
        t["entry:" + r["entry"]] += 1
        if "reference_error" in r:
            t["reference errors"] += 1
            examples["reference errors"].append((r["tag"], r["entry"], r["mode"], r["reference_error"][-300:]))
            continue
        t["cls:" + r["cls"]] += 1
        if r.get("error"):
            t["runner/DTO error"] += 1
            examples["errors"].append((r["tag"], r["entry"], r["mode"], r["error"][:200]))
        if r["blocking"]:
            t["refused (blocking)"] += 1
            t["refused:" + ",".join(sorted(set(r["blocking"])))] += 1
        if r["cls"] == "passed":
            t["Passed"] += 1
            t["Passed: judged groups"] += r.get("n_judged", 0)
            t["Passed: judged member/section/support/stress breaches (groups)"] += len(r.get("breaches", []))
            t["Passed: case-modes with such a breach"] += bool(r.get("breaches"))
            t["Passed: displacement/rotation breach groups (reported apart)"] += len(r.get("disp_breaches", []))
            t["Passed: extension (Lame) breach groups"] += len(r.get("ext_breaches", []))
            t["Passed: strict max_station_checks coverage failures (rows)"] += len(r.get("max_fail", []))
            t["Passed: case-modes with a coverage failure"] += bool(r.get("max_fail"))
            for g, w in (r.get("worst_by_group") or {}).items():
                if g not in worst or w["ratio"] > worst[g][0]:
                    worst[g] = (w["ratio"], r["tag"], r["entry"], r["mode"], w["entity"], w["loc"], w["basis"])
            for b in r.get("breaches", [])[:2]:
                examples["Passed breaches"].append((r["tag"], r["entry"], r["mode"], b["group"], b["entity"], b["loc"], b["basis"], b["ratio"]))
            for c in r.get("max_fail", [])[:1]:
                examples["coverage failures"].append((r["tag"], r["entry"], r["mode"], c))
            for cid, cs in (r.get("cases") or {}).items():
                if cs.get("headline_findings"):
                    t["Passed: headline findings (extension)"] += 1
                    examples["headline findings"].append((r["tag"], r["entry"], r["mode"], cs["headline_findings"][:2]))
                for c in cs["max_checks"]:
                    if not c.get("evidence"):
                        t["Passed: maxima without same-basis evidence"] += 1
                    x = c.get("ref_outside_enclosure_over_crit")
                    if x is not None:
                        t["Passed: maxima checked against the exact reference"] += 1
                        if x > 1:
                            t["Passed: maxima whose enclosure misses the exact maximum by > CRIT (extension)"] += 1
        if r["cls"].startswith("sensitive"):
            t["Sensitive"] += 1
        t["max rows checked"] += r.get("n_max", 0)
        t["unmapped product rows"] += r.get("n_unmapped", 0)
        t["reference rows the product did not publish"] += r.get("n_missing", 0)
        if r.get("n_unmapped"):
            for cid, cs in (r.get("cases") or {}).items():
                for u in cs["unmapped"][:1]:
                    examples["unmapped"].append((r["tag"], u))
        if r.get("n_missing"):
            for cid, cs in (r.get("cases") or {}).items():
                for u in cs["missing"][:1]:
                    examples["missing"].append((r["tag"], u))
        if r["entry"] == "captured" and r.get("typed_captured_rows_identical") is False:
            t["typed/captured published-row differences (captured runs)"] += 1
        if r["entry"] == "captured" and r.get("typed_captured_disagree"):
            t["typed/captured disagreements (integrity codes or breach counts)"] += 1
            examples["typed/captured disagreements"].append((r["tag"], r["mode"]))
        for c in r.get("captured_extra_codes") or []:
            t["captured-only diagnostic code: " + c] += 1
        for c in r.get("typed_extra_codes") or []:
            t["typed-only diagnostic code: " + c] += 1
        if r["cls"] == "passed":
            for b in r.get("disp_breaches", []):
                rb = b.get("ratio_body")
                if rb is not None:
                    key = "Passed: displacement/rotation breaches beyond 1e-9 of the body scale"
                    t[key] += rb > 1
                    if "disp_body_worst" not in worst or rb > worst["disp_body_worst"][0]:
                        worst["disp_body_worst"] = (rb, r["tag"], r["entry"], r["mode"], b["entity"], b["group"], None)
        if r.get("notices"):
            t["STRESS_FREE notices"] += r["notices"]
    return t, worst, examples


def phi_table(recs, set_path):
    """Passed case-modes with breaches, by the decade of the model's largest Timoshenko phi (sensitivity controls)."""
    import math
    phi = {}
    for line in open(set_path):
        x = json.loads(line)
        sec = x["ref"]["sections"]
        phi[x["tag"]] = max(float(s["phi_timoshenko"]) for s in sec.values())
    t = collections.defaultdict(collections.Counter)
    for r in recs:
        if r.get("cls") != "passed":
            continue
        dec = math.floor(math.log10(phi[r["tag"]]))
        row = t[dec]
        row["Passed"] += 1
        row["member/section/support/stress breach"] += bool(r.get("breaches"))
        row["displacement/rotation breach beyond the body scale"] += any((b.get("ratio_body") or 0) > 1 for b in r.get("disp_breaches", []))
        row["any breach"] += bool(r.get("breaches")) or any((b.get("ratio_body") or 0) > 1 for b in r.get("disp_breaches", []))
    print("  max phi decade | Passed | member/section/support/stress breach | disp/rot breach beyond body scale | any")
    for dec in sorted(t):
        row = t[dec]
        print(f"  1e{dec:+03d}          {row['Passed']:6d}   {row['member/section/support/stress breach']:6d}   {row['displacement/rotation breach beyond the body scale']:6d}   {row['any breach']:6d}")
    return {str(k): dict(v) for k, v in t.items()}


def main():
    args = sys.argv[1:]
    by = None
    jout = None
    phi_set = None
    if "--phi-table" in args:
        i = args.index("--phi-table")
        phi_set = args[i + 1]
        del args[i:i + 2]
    if "--by" in args:
        i = args.index("--by")
        by = args[i + 1]
        del args[i:i + 2]
    if "--json" in args:
        i = args.index("--json")
        jout = args[i + 1]
        del args[i:i + 2]
    recs = load(args)
    groups = {"all": recs}
    if by:
        key = {"family": "family", "theory": "theory", "placement": "placement", "mill": "mill_and_allowance"}[by]
        groups = collections.defaultdict(list)
        for r in recs:
            groups[str(r["meta"][key])].append(r)
        groups = dict(sorted(groups.items()))
        groups["all"] = recs
    report = {}
    for name, rs in groups.items():
        t, worst, ex = table(rs)
        print(f"===== {name}: {len(rs)} records ({len({r['tag'] for r in rs})} models)")
        for k in sorted(t):
            print(f"  {k:78s} {t[k]:>9}")
        if worst:
            print("  worst judged err/c on Passed, by group:")
            for g, w in sorted(worst.items()):
                print(f"    {g:30s} {w[0]:.4g}  {w[1:]}")
        for k, v in ex.items():
            print(f"  examples [{k}] ({len(v)}): {v[:4]}")
        report[name] = {"counts": dict(t), "worst": {g: list(w) for g, w in worst.items()}}
    if phi_set:
        report["phi_table"] = phi_table(recs, phi_set)
    if jout:
        json.dump(report, open(jout, "w"), indent=1, sort_keys=True)


if __name__ == "__main__":
    main()
