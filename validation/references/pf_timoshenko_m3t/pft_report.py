"""PF M3-T report: kd5t_report's M3 tables (../kd5_timoshenko, read-only) over pft_compare.py records, with the
collection's provenance and inventory recomputed and the design-conformance verdict.

Usage: python3 -I -B pft_report.py <set.jsonl> <eval.jsonl> [<eval.jsonl> ...] [--ref own|twin]
                                   [--by family|theory|placement|section] [--phi-table] [--json out.json]

Nothing is taken from a chunk's summary (refuter round 2, P1). Over the whole collection of records the report
recomputes the inventory and every cross-record clause: the typed/captured pairing of each model and solver mode
(C4, C4b; pft_compare.pair_typed_captured), wherever each record was judged. Sidecars are cross-checks only.
Four verdicts, each printed; the exit status is 1 when any fails:
- PROVENANCE: every record carries the current stamp (pft_compare.JUDGING_FORMAT, the sha256 of pft_compare.py and
  kd5t_compare.py, the sha256 of <set.jsonl>, the --ref variant); every evaluation file has its sidecar, which
  names exactly that file's bytes and records and whose own inventory passed. An absent, stale or mismatched
  stamp or sidecar fails (a pre-repair evaluation has neither).
- INVENTORY: every model of <set.jsonl> x entry (typed, captured) x mode (SparseInteractive, DenseScrutiny) has
  exactly one record over all files, and no record is unexpected (3,624 runs for the full set).
- M3 VERDICT (the K-D5 M3 criterion and the soundness of its judging: C0-C4, C6, C8, C9): passes only when
  provenance and inventory pass and no record fails an acceptance clause after the recomputed pairing.
- DESIGN-CONFORMANCE VERDICT (admission, pf-timoshenko.md section 2: C10; separate from M3): passes only when
  provenance and inventory pass and every record carries `design_failures` and it is empty. An absent
  `design_failures` is a failure.
Information clauses (C3lit, C4b, C5, C6i, C7, C11, C11tau_b, C11ops) are counted beside them.
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


def report(set_path, eval_paths, variant="own", by=None, phi=False, quiet=False):
    """Print the tables and the four verdicts; return the report dict (with 'pass')."""
    say = (lambda *a: None) if quiet else print
    want = PC.stamp(set_path, variant)
    tags = PC.expected_tags(set_path)
    provenance = {p: PC.eval_problems(p, want) for p in eval_paths}
    for p in eval_paths:
        if not provenance[p] and not json.load(open(p + ".inventory.json")).get("pass"):
            provenance[p].append("its comparison's own inventory failed")
    recs = [json.loads(line) for p in eval_paths if os.path.exists(p) for line in open(p) if line.strip()]
    prov_pass = bool(eval_paths) and not any(provenance.values())
    inv = PC.inventory(tags, [tuple(i) for i in PC.record_identities(recs)])
    n_c4 = PC.pair_typed_captured(recs)
    judged = [r for r in recs if "duplicate_identity" not in r and "unexpected_tag" not in r]
    groups = {"all": judged}
    if by:
        key = {"family": "family", "theory": "theory", "placement": "placement", "section": "section_variant"}[by]
        groups = collections.defaultdict(list)
        for r in judged:
            groups[str((r.get("meta") or {}).get(key))].append(r)
        groups = dict(sorted(groups.items()))
        groups["all"] = judged
    out = {}
    for name, rs in groups.items():
        t, worst, ex = KR.table(rs)
        say(f"===== {name}: {len(rs)} records ({len({r['tag'] for r in rs})} models)")
        for k in sorted(t):
            say(f"  {k:78s} {t[k]:>9}")
        for g, w in sorted(worst.items()):
            say(f"    worst {g:30s} {w[0]:.4g}  {w[1:]}")
        for k, v in ex.items():
            say(f"  examples [{k}] ({len(v)}): {v[:4]}")
        out[name] = {"counts": dict(t), "worst": {g: list(w) for g, w in worst.items()}}
    m3_failing = sum(1 for r in recs if r.get("verdict_failures"))
    design_failing = [r for r in recs if "design_failures" not in r or r["design_failures"]]
    info = collections.Counter(c for r in recs for c in (r.get("info") or {}))
    m3_pass = prov_pass and inv["pass"] and m3_failing == 0
    design_pass = prov_pass and inv["pass"] and not design_failing
    say(f"PROVENANCE (stamp {want['format']}, comparator, reference, variant {variant}; sidecars as cross-checks): "
        f"{'PASS' if prov_pass else 'FAIL'} ({sum(1 for v in provenance.values() if v)} of {len(eval_paths)} evaluation files fail)")
    for p, v in provenance.items():
        if v:
            say(f"  {p}: {'; '.join(v)}")
    say(PC.inventory_text(inv))
    for k in ("missing", "duplicate", "unexpected"):
        if inv[k]:
            say(f"  {k} (first 5): {inv[k][:5]}")
    say(f"typed/captured pairing recomputed over the collection: {n_c4} C4 disagreements")
    say(f"M3 VERDICT (K-D5 M3 criterion and judging soundness: C0-C4, C6, C8, C9; requires provenance and inventory): "
        f"{'PASS' if m3_pass else 'FAIL'} ({m3_failing} of {len(recs)} records fail an acceptance clause)")
    say(f"DESIGN-CONFORMANCE VERDICT (admission, pf-timoshenko.md section 2, C10; separate from M3; requires provenance and inventory): "
        f"{'PASS' if design_pass else 'FAIL'} ({len(design_failing)} records fail or carry no design_failures; "
        f"first: {[(r.get('tag'), r.get('entry'), r.get('mode')) for r in design_failing[:3]]})")
    say(f"information clauses, records raising each: {dict(sorted(info.items()))}")
    if info.get("C11tau_b"):
        say("  C11tau_b is the known tau_v,max defect on main (RN(1 + nu_d) cancels as E/(2G) -> 0), ruled by Agent 0 a separate correction (option (b))")
    out.update({"provenance": provenance, "provenance_pass": prov_pass, "inventory": inv, "c4_recomputed": n_c4,
                "m3_pass": m3_pass, "m3_failing_records": m3_failing, "design_pass": design_pass,
                "design_failing_records": len(design_failing), "info_records": dict(info)})
    out["pass"] = prov_pass and inv["pass"] and m3_pass and design_pass
    if not quiet:
        out["standings_vs_twins"] = KR.standings_vs_twins(judged)
        if phi:
            out["phi_table"] = KR.phi_table(judged, set_path)
    return out


def main():
    args = sys.argv[1:]
    by = jout = None
    variant = "own"
    phi = False
    for flag in ("--by", "--json", "--ref"):
        if flag in args:
            i = args.index(flag)
            val = args[i + 1]
            del args[i:i + 2]
            if flag == "--by":
                by = val
            elif flag == "--json":
                jout = val
            else:
                variant = val
    if "--phi-table" in args:
        args.remove("--phi-table")
        phi = True
    if len(args) < 2 or variant not in ("own", "twin"):
        raise SystemExit(__doc__)
    out = report(args[0], args[1:], variant, by, phi)
    if jout:
        json.dump(out, open(jout, "w"), indent=1, sort_keys=True, default=list)
    if not out["pass"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
