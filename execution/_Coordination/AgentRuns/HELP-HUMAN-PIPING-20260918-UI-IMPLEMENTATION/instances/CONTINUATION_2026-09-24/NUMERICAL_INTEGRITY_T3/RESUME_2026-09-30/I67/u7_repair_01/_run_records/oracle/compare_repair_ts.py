#!/usr/bin/env python3
"""I67 U7 repair: the TS oracle-diff control. The TS dumper (slice F part 2's zzI67U7fOracle.test.ts)
runs in the base lane (8c84e7ae14) and the candidate lane on the same inputs: the candidate's 386,
which are I66's 381 (sha256 104fb714..., regenerated from the base lane) plus the shared N-3 probe
and the 4 D-U7-4 summary-form inputs. Checks: (1) no TS field changes on any input (TS's product
change is the docstring only); (2) on the 4 summary-form inputs TS reads its declared side, the
not-Current summary (withheld 97) with standing needs_recompute / NATIVE_CAPTURE_REQUIRED, while
its standing rule with the capture held true (`live`) reads Current (69), as Python and Rust do;
(3) the probe is refused at G7 with TS's own code.
Usage: compare_repair_ts.py TS_BASE TS_CAND INPUTS_BASE INPUTS_CAND OUT_TXT"""
import json, sys
tb, tc, ib, ic, out = sys.argv[1:6]
B, C = json.load(open(tb)), json.load(open(tc))
base_keys = {r["key"] for r in json.load(open(ib))}
cand_keys = [r["key"] for r in json.load(open(ic))]
assert B.keys() == C.keys() == set(cand_keys) and len(cand_keys) == 386 and base_keys <= set(cand_keys) and len(base_keys) == 381
changed = {k: sorted(f for f in set(B[k]) | set(C[k]) if B[k].get(f) != C[k].get(f)) for k in B}
changed = {k: v for k, v in changed.items() if v}
new = sorted(set(cand_keys) - base_keys)
SUM = [k for k in new if ":summary|" in k]
lines = [f"inputs: {len(cand_keys)} (I66's 381 plus {len(new)} new: {new})", f"(1) TS fields changed, base lane vs candidate lane: {len(changed)} {changed}"]
ok = not changed and len(SUM) == 4
for k in SUM:
    r = C[k]
    got = (r["ipc"]["token"], r["ipc"]["findings"], [c["withheld"] for c in r["ipc_summary"]], r["live"]["token"], [c["withheld"] for c in r["live_summary"]], r["numerical_eligible"])
    want = ("needs_recompute", ["RETAINED_PRECISION_NATIVE_CAPTURE_REQUIRED"], [97], "numerically_eligible", [69], True)
    lines.append(f"(2) {k}: ipc token, findings, ipc withheld, live token, live withheld, reader eligible = {got}; want {want}: {got == want}")
    ok = ok and got == want
probe = C["mutations|g7_not_required_quality_enum_invalid"]["reader"]
lines.append(f"(3) the N-3 probe, TS reader: {probe}; want G7:SOURCE_PRODUCER_CONTRACT_UNSUPPORTED: {probe == 'G7:SOURCE_PRODUCER_CONTRACT_UNSUPPORTED'}")
ok = ok and probe == "G7:SOURCE_PRODUCER_CONTRACT_UNSUPPORTED"
lines.append("RESULT: " + ("PASS" if ok else "FAIL"))
open(out, "w").write("\n".join(lines) + "\n"); print("\n".join(lines))
