"""Aggregate kd5t_compare.py output into the M3 tables and verdict (after kd5-eval-final/analyze.py and b4-finish/analyze_b4.py).

Usage: python3 -I -B kd5t_report.py <eval.jsonl> [<eval.jsonl> ...] [--by family|theory|placement|mill|section]
                                    [--phi-table <set.jsonl>] [--json out.json]
The verdict passes when no run fails an acceptance clause of kd5t_compare.py (C0-C4, the K-D5 M3
criterion); the information clauses (C1L, C3lit, C4b, C5-C8) are counted and listed beside it.
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
            t["Passed: judged Lame breaches (C1L, information)"] += len(r.get("lame_breaches", []))
            t["Passed: displacement/rotation breach groups against their own q (information)"] += len(r.get("disp_breaches", []))
            for g, w in (r.get("worst_by_group") or {}).items():
                if g not in worst or w["ratio"] > worst[g][0]:
                    worst[g] = (w["ratio"], r["tag"], r["entry"], r["mode"], w["entity"], w["loc"], w["basis"])
            for b in r.get("breaches", [])[:2]:
                examples["Passed breaches"].append((r["tag"], r["entry"], r["mode"], b["group"], b["entity"], b["loc"], b["basis"], b["ratio"]))
            for cid, cs in (r.get("cases") or {}).items():
                for c in cs["max_checks"]:
                    x = c.get("ref_outside_enclosure_over_crit")
                    if x is not None:
                        t["Passed: maxima checked against the exact maximum"] += 1
                        if x > 1:
                            t["Passed: maxima whose enclosure misses the exact maximum by > CRIT"] += 1
                t["Passed: groups the reference judges left unjudged (C8)"] += cs.get("n_unjudged_by_ref", 0)
        t["strict max_station_checks coverage failures (rows, C2)"] += len(r.get("max_fail", []))
        for c in r.get("max_fail", [])[:1]:
            examples["coverage failures"].append((r["tag"], r["entry"], r["mode"], c))
        for cid, cs in (r.get("cases") or {}).items():
            if cs.get("headline_findings"):
                t["headline findings"] += 1
                examples["headline findings"].append((r["tag"], r["entry"], r["mode"], cs["headline_findings"][:2]))
            t["maxima without same-basis evidence"] += sum(1 for c in cs["max_checks"] if not c.get("evidence"))
            t["basis-free stress rows (U6 producer)"] += cs.get("n_basis_free", 0)
            t["basis-suffixed rows (pre-U6 producer)"] += cs.get("n_unexpected_basis", 0)
            t["missing maxima"] += len(cs.get("missing_maxima", []))
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
            t["Passed: displacement/rotation groups beyond 1e-9 of the body's coupled scale S*_kind (C3)"] += len(r.get("disp_beyond_kd5", []))
            t["Passed: displacement/rotation groups beyond 1e-9 of the separate published scale (C3lit, information)"] += len(r.get("disp_beyond_literal", []))
            for b in r.get("disp_beyond_kd5", [])[:2]:
                examples["C3 displacement/rotation beyond S*_kind"].append((r["tag"], r["entry"], r["mode"], b["group"], b["entity"], b["err"], b["ratio_kd5"]))
            for b in r.get("disp_beyond_literal", [])[:1]:
                examples["C3lit displacement/rotation beyond the separate scale"].append((r["tag"], r["entry"], r["mode"], b["group"], b["entity"], b["err"], b["ratio_literal"]))
            for key, val in (("disp_kd5_worst (C3, err/(c S*_kind))", r.get("worst_disp_kd5")), ("disp_literal_worst (C3lit, information)", r.get("worst_disp_literal"))):
                if val is not None and (key not in worst or val > worst[key][0]):
                    worst[key] = (val, r["tag"], r["entry"], r["mode"], None, None, None)
        if r.get("notices"):
            t["STRESS_FREE notices"] += r["notices"]
        vf = r.get("verdict_failures") or {}
        t["VERDICT runs failing (acceptance, C0-C4)"] += bool(vf)
        for c, n in vf.items():
            t[f"VERDICT {c} failures"] += n
            if len(examples["verdict " + c]) < 4:
                examples["verdict " + c].append((r["tag"], r["entry"], r["mode"], r["cls"]))
        inf = r.get("info") or {}
        t["INFO runs raising an information clause"] += bool(inf)
        for c, n in inf.items():
            t[f"INFO {c}"] += n
            t[f"INFO {c} runs"] += 1
            if len(examples["info " + c]) < 4:
                examples["info " + c].append((r["tag"], r["entry"], r["mode"], r["cls"]))
    return t, worst, examples


def standings_vs_twins(recs):
    """Per family: standings of each theory (the Timoshenko set against its Euler-Bernoulli twins)."""
    t = collections.defaultdict(collections.Counter)
    for r in recs:
        m = r.get("meta") or {}
        t[(m.get("family"), m.get("theory"))][r.get("cls", "error")] += 1
    print("  family / theory: standings")
    for k in sorted(t, key=lambda k: (str(k[0]), str(k[1]))):
        print(f"    {str(k[0]):16s} {str(k[1]):30s} {dict(sorted(t[k].items()))}")
    return {f"{k[0]}|{k[1]}": dict(v) for k, v in t.items()}


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
        row["displacement/rotation breach beyond S*_kind"] += bool(r.get("disp_beyond_kd5"))
        row["any breach"] += bool(r.get("breaches")) or bool(r.get("disp_beyond_kd5"))
        row["verdict fails"] += bool(r.get("verdict_failures"))
    print("  max phi decade | Passed | member/section/support/stress breach | disp/rot breach beyond S*_kind | any")
    for dec in sorted(t):
        row = t[dec]
        print(f"  1e{dec:+03d}          {row['Passed']:6d}   {row['member/section/support/stress breach']:6d}   {row['displacement/rotation breach beyond S*_kind']:6d}   {row['any breach']:6d}")
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
        key = {"family": "family", "theory": "theory", "placement": "placement", "mill": "mill_and_allowance", "section": "section_variant"}[by]
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
    failing = sum(1 for r in recs if r.get("verdict_failures"))
    flagged = collections.Counter(c for r in recs for c in (r.get("info") or {}))
    print(f"M3 VERDICT (K-D5 M3 criterion, C0-C4): {'PASS' if failing == 0 else 'FAIL'} ({failing} of {len(recs)} runs fail an acceptance clause)")
    print(f"information clauses (own, stricter), runs raising each: {dict(sorted(flagged.items()))}")
    report["verdict"] = {"pass": failing == 0, "failing_runs": failing, "runs": len(recs), "info_runs": dict(flagged)}
    report["standings_vs_twins"] = standings_vs_twins(recs)
    if phi_set:
        report["phi_table"] = phi_table(recs, phi_set)
    if jout:
        json.dump(report, open(jout, "w"), indent=1, sort_keys=True)


if __name__ == "__main__":
    main()
