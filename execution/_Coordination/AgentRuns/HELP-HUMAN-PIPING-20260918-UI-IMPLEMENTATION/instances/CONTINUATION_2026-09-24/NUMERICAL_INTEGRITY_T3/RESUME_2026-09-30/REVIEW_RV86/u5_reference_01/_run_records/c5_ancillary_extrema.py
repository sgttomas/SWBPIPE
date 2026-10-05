"""RV86 check 5: (a) the oracle's omitted mode/parity value checks (oracle lines 120-129, value parts)
applied to the successors; (b) the extrema interval against the exact truth and the maximum row's bound."""
import json, math, struct, sys
from fractions import Fraction as F
from pathlib import Path
bits = lambda x: struct.pack('>d', float(x)).hex(); dec = lambda h: struct.unpack('>d', bytes.fromhex(h))[0]
out = {}
for p in sys.argv[1:]:
    d = json.loads(Path(p).read_text()); mode = d["invocation"]["solver_mode"]; rows = d["source"]["results"]
    mode_row = [r for r in rows if r["kind"] == "linear_solver_mode_basis"]; parity = [r for r in rows if r["kind"] == "sparse_live_path_dense_parity_relative_delta"]
    expected = 1.0 if mode == "sparse_interactive" else 2.0
    sel = d["source"]["retained_precision"]["body"]["cases"][0]["selection"]
    mx = next(r for r in rows if r["kind"] == "pipe_elastic_normal_stress_maximum_v2")
    b = F(dec(next(a["bound"] for a in sel["absolute_verified"] if a["result_id"] == mx["id"])))
    ex = d["source"]["contract_evidence"]["preview_cases"][0]["pipe_stress_extrema"][0]
    lo, hi, up = F(ex["value_lower_pa"]), F(ex["value_upper_pa"]), F(ex["global_upper_bound_pa"])
    truth = F(0)  # pure torsion: zero normal stress (oracle truth and RV86's independent derivation agree)
    out[mode] = {"mode_row_count": len(mode_row), "mode_row_bits_ok": bits(mode_row[0]["value"]) == bits(expected), "mode_basis_mentions_mode": f"solver_mode={mode}" in mode_row[0]["metadata"]["basis"],
                 "parity_count": len(parity), "parity_rule_ok": (mode == "dense_scrutiny") or not parity, "parity_finite_nonneg": all(math.isfinite(r["value"]) and r["value"] >= 0 for r in parity),
                 "extrema": {k: ex[k] for k in ["value_lower_pa", "value_upper_pa", "global_upper_bound_pa", "certified_gap_pa", "enclosure_scope"]},
                 "max_row_value": mx["value"], "max_row_bound": float(b),
                 "interval_encloses_exact_truth": lo <= truth <= hi, "global_upper_bound_ge_truth": up >= truth,
                 "interval_within_bound_of_truth": max(abs(lo - truth), abs(hi - truth), abs(up - truth)) <= b,
                 "midpoint_identity": F(mx["value"]) == F(ex["value_lower_pa"] + .5 * (ex["value_upper_pa"] - ex["value_lower_pa"]))}
print(json.dumps(out, indent=1))
