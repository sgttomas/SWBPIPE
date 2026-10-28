"""PF M3-T report: kd5t_report's M3 tables (../kd5_timoshenko, read-only) over pft_compare.py records, with the
collection inventory and the design-conformance verdict.

Usage: python3 -I -B pft_report.py <set.jsonl> <eval.jsonl> [<eval.jsonl> ...] [--expect <runner input.jsonl> ...]
                                   [--by family|theory|placement|section] [--phi-table] [--json out.json]

Three verdicts, each printed and failing on its own:
- INVENTORY: every expected model x entry x mode (typed and captured; SparseInteractive and DenseScrutiny) has exactly
  one record, and no record is unexpected. Expected models are the set's (3,624 runs for the full set), or those of
  the given runner inputs. An empty, partial or duplicated collection fails here (refuter P1).
- M3 VERDICT (the K-D5 M3 criterion and the soundness of its judging: C0-C4, C6, C8, C9): passes only when the
  inventory passes and no record fails an acceptance clause.
- DESIGN-CONFORMANCE VERDICT (admission, pf-timoshenko.md section 2: C10): separate from M3; fails on any record
  whose admission differs from the design's.
Information clauses (C3lit, C4b, C5, C6i, C7, C11, C11tau_b, C11ops) are counted beside them. The exit status is 1
when any verdict fails.
"""

from __future__ import annotations

import collections
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.dont_write_bytecode = True
sys.path.insert(0, HERE)
sys.path.insert(0, os.path.join(HERE, "..", "kd5_timoshenko"))
import kd5t_report as KR  # noqa: E402  (read-only reuse)
import pft_compare as PC  # noqa: E402


def report(set_path, eval_paths, expect_paths=None, by=None, phi=False, quiet=False):
    """Print the tables and the three verdicts; return the report dict (with 'pass')."""
    say = (lambda *a: None) if quiet else print
    tags = [json.loads(line)["tag"] for line in open(set_path) if line.strip()]
    if expect_paths:
        tags = [json.loads(line)["tag"] for p in expect_paths for line in open(p) if line.strip()]
    recs = [json.loads(line) for p in eval_paths for line in open(p) if line.strip()]
    # a record of a rejected duplicate identity stands for every copy received (pft_compare.work)
    inv = PC.inventory(tags, [(r.get("tag"), r.get("entry"), r.get("mode")) for r in recs for _ in range(r.get("duplicate_identity", 1))])
    # and each comparison's own inventory, where it wrote one, must have passed
    inv["comparison_inventories_failing"] = [p for p in eval_paths if os.path.exists(p + ".inventory.json")
                                             and not json.load(open(p + ".inventory.json"))["pass"]]
    if inv["comparison_inventories_failing"]:
        inv["pass"] = False
    groups = {"all": recs}
    if by:
        key = {"family": "family", "theory": "theory", "placement": "placement", "section": "section_variant"}[by]
        groups = collections.defaultdict(list)
        for r in recs:
            groups[str((r.get("meta") or {}).get(key))].append(r)
        groups = dict(sorted(groups.items()))
        groups["all"] = recs
    out = {}
    for name, rs in groups.items():
        t, worst, ex = KR.table([r for r in rs if "duplicate_identity" not in r and "unexpected_tag" not in r])
        say(f"===== {name}: {len(rs)} records ({len({r['tag'] for r in rs})} models)")
        for k in sorted(t):
            say(f"  {k:78s} {t[k]:>9}")
        for g, w in sorted(worst.items()):
            say(f"    worst {g:30s} {w[0]:.4g}  {w[1:]}")
        for k, v in ex.items():
            say(f"  examples [{k}] ({len(v)}): {v[:4]}")
        out[name] = {"counts": dict(t), "worst": {g: list(w) for g, w in worst.items()}}
    m3_failing = sum(1 for r in recs if r.get("verdict_failures"))
    design_failing = [r for r in recs if r.get("design_failures")]
    info = collections.Counter(c for r in recs for c in (r.get("info") or {}))
    m3_pass = inv["pass"] and m3_failing == 0
    say(PC.inventory_text(inv))
    for k in ("missing", "duplicate", "unexpected", "comparison_inventories_failing"):
        if inv[k]:
            say(f"  {k} (first 5): {inv[k][:5]}")
    say(f"M3 VERDICT (K-D5 M3 criterion and judging soundness: C0-C4, C6, C8, C9; requires the inventory): {'PASS' if m3_pass else 'FAIL'} "
        f"({m3_failing} of {len(recs)} records fail an acceptance clause{'' if inv['pass'] else '; inventory fails'})")
    say(f"DESIGN-CONFORMANCE VERDICT (admission, pf-timoshenko.md section 2, C10; separate from M3): {'PASS' if not design_failing else 'FAIL'} "
        f"({len(design_failing)} records; first: {[(r['tag'], r['entry'], r['mode']) for r in design_failing[:3]]})")
    say(f"information clauses, records raising each: {dict(sorted(info.items()))}")
    if info.get("C11tau_b"):
        say("  C11tau_b is the known tau_v,max defect on main (RN(1 + nu_d) cancels as E/(2G) -> 0), ruled by Agent 0 a separate correction (option (b))")
    out.update({"inventory": inv, "m3_pass": m3_pass, "m3_failing_records": m3_failing, "design_pass": not design_failing,
                "design_failing_records": len(design_failing), "info_records": dict(info)})
    out["pass"] = inv["pass"] and m3_pass and not design_failing
    if not quiet:
        out["standings_vs_twins"] = KR.standings_vs_twins(recs)
        if phi:
            out["phi_table"] = KR.phi_table([r for r in recs if "duplicate_identity" not in r and "unexpected_tag" not in r], set_path)
    return out


def main():
    args = sys.argv[1:]
    by = jout = None
    expect = []
    phi = False
    while "--expect" in args:
        i = args.index("--expect")
        expect.append(args[i + 1])
        del args[i:i + 2]
    for flag in ("--by", "--json"):
        if flag in args:
            i = args.index(flag)
            if flag == "--by":
                by = args[i + 1]
            else:
                jout = args[i + 1]
            del args[i:i + 2]
    if "--phi-table" in args:
        args.remove("--phi-table")
        phi = True
    if len(args) < 2:
        raise SystemExit(__doc__)
    out = report(args[0], args[1:], expect or None, by, phi)
    if jout:
        json.dump(out, open(jout, "w"), indent=1, sort_keys=True, default=list)
    if not out["pass"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
