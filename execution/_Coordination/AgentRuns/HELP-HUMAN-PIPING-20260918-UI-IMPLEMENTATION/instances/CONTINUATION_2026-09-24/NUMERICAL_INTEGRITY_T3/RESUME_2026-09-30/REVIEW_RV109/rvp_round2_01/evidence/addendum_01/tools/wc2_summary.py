#!/usr/bin/env python3
"""RV109 round 2: summarize the reviewer's W-C2 re-derivation lines (`RV109_WC2 {json}`) and the
one-case rows (`RV109_ONE {json}`) from a probe log.

Usage: wc2_summary.py <probe log>
"""
import json
import sys


def main():
    rows, ones = [], []
    for line in open(sys.argv[1], encoding="utf-8", errors="replace"):
        i = line.find("RV109_WC2 {")
        if i >= 0:
            rows.append(json.loads(line[i + len("RV109_WC2 "):]))
            continue
        i = line.find("RV109_ONE {")
        if i >= 0:
            ones.append(json.loads(line[i + len("RV109_ONE "):]))
    print(f"W-C2 rows: {len(rows)}")
    for r in rows:
        print(f"\n## {r['label']}  input {r['input_sha'][:12]}  plain {r['plain_sha'][:12]}")
        print(f"  verdicts {r['verdicts']}  A={r['attempted']}")
        if "custody" in r:
            print(f"  CUSTODY {r['custody']}")
            continue
        print(f"  attempts after T-7 {r['attempts_after_prep']}; ends after T-8 {r['ends_after_native']}; after T-9 {r['ends_after_freeze']}; selected bits {r['selected_bits']}")
        for run in r.get("runs", []):
            o = run.get("outcome") or {}
            print(f"  run: request {run['request']} id {run['run']} owner {run['owner']} terminal {o.get('terminal')} precision {o.get('precision')} attempts {len(o.get('attempts', []))}")
        print(f"  counts: run {r['after_run']}\n          prep {r['after_prep']}\n          native {r['after_native']}\n          freeze {r['after_freeze']}")
        print(f"  snapshots after T-8 {r['snapshots_after_native']}\n  snapshots after T-9 {r['snapshots_after_freeze']}")
        if "outcome" in r:
            print(f"  OUTCOME {r['outcome']}")
            continue
        if "serializer" in r:
            print(f"  SERIALIZER {r['serializer']}")
            continue
        print(f"  staging counts {r['after_staging']}; serialize counts {r['after_serialize']}; ordinary untouched {r['ordinary_untouched']}")
        print(f"  successor {r['successor_sha'][:16]} receipt {str(r['receipt_sha'])[:16]} reader: {r['reader']}  G7 on the projection: {r['g7_projected']}")
        print(f"  cases {json.dumps(r['cases_brief'])}")
        print(f"  calls {json.dumps([{k: c[k] for k in ('owner_refs', 'source_refs', 'run_refs', 'invocation_before', 'invocation_after')} for c in r['calls']])}")
        print(f"  execution_order {json.dumps(r['execution_order'])}  charged {r['charged']}")
        print(f"  groups {json.dumps(r['groups'])}")
        print(f"  builds {r['builds']}: {json.dumps(r['builds_detail'])[:400]}")
        print(f"  run origins {json.dumps(r['run_groups'])[:600]}")
        print(f"  pa counts {r['pa_counts']}; last frozen snapshot == counts after T-9: {r['last_frozen_snapshot_eq_after_freeze']}")
        for field, h in r["headlines"].items():
            print(f"  {field}: ordinary case {h['ordinary_case']} -> staged case {h['staged_case']}; ordinary {h['ordinary']} staged {h['staged']}")
            print(f"     G7 staged: {h['g7_staged']} | keep-ordinary: {h['g7_keep_ordinary']} | first-frozen alias: {h['g7_first_alias']} | selected-rows-only: {h['g7_selected_only']}")
        print(f"  checks: {r['checks_total']} run, failed: {r['checks_failed']}")
        for case, v in r["n16"].items():
            one = v["one_case"]
            run = one.get("run") or {}
            print(f"  N-16 {case}: one-case verdict {one.get('verdict')} end {one.get('end', one.get('custody'))} terminal {run.get('terminal')} attempts {len(run.get('attempts', []))}; batch Run equal: {v['batch_equal']}")
    if ones:
        print("\n## one-case rows")
        for o in ones:
            out = o["outcome"]
            run = out.get("run") or {}
            print(f"  {o['case']} {o['mode']}: verdict {out.get('verdict')} end {out.get('end', out.get('custody'))} terminal {run.get('terminal')} precision {run.get('precision')} attempts {len(run.get('attempts', []))}; retained_w1: {o['w1']}")


if __name__ == "__main__":
    main()
