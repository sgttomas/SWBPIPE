"""I95 B3-S: the multiplicities behind each rebound rule (stdlib only).

For I82's chain (mc_chain) and the exact-route-only variants (er, erc) at c = 1 and c = 3, per TEXT branch
(full graph, W, X): the function multiplicity M of each function a rebound rule governs, the TEXT bytes
(requested) and diagnostics of the files the rules open, and the totals (TAV, D).
Usage: python3 b3s_rules_mult.py <runs dir> <out json>
"""
import json, os, sys, collections

runs, out = sys.argv[1:3]
FUNCS = ["lib.rs:11705:append_exact_pressure_results",
         "pressure_runtime.rs:111:validate_profile", "pressure_runtime.rs:357:check_suffixes",
         "pressure_runtime.rs:427:build_pressure_case_with_members", "pressure_runtime.rs:836:finish_source_groups",
         "pressure_runtime.rs:85:problem",
         "pressure_material.rs:61:resolve_base", "pressure_material.rs:101:resolve_case", "pressure_material.rs:5:failure",
         "source_receipt/composite.rs:353:composite_support_norms", "source_receipt/composite.rs:165:composite_member_maximum",
         "source_receipt/composite.rs:721:composite_exact", "source_receipt/composite.rs:943:ordinary_physics",
         "source_receipt/composite.rs:994:validate_publication", "source_receipt/composite.rs:451:physical_source_built",
         "physics_evidence.rs:319:validate_physics_evidence", "physics_source.rs:409:validate_maximum",
         "preview_physics_evidence.rs:244:validate_preview_physics_evidence", "preview_physics.rs:528:render",
         "source_receipt.rs:636:exact", "source_receipt.rs:720:ordinary", "source_receipt.rs:839:finalize",
         "source_receipt.rs:853:finalize_composite", "source_receipt.rs:152:captured_load_state_case"]
FILES = ["pressure_runtime.rs", "pressure_material.rs", "source_receipt/composite.rs", "physics_evidence.rs",
         "physics_source.rs", "preview_physics_evidence.rs", "preview_physics.rs", "source_receipt.rs",
         "source_receipt/source.rs", "source_receipt/rows.rs", "source_recovery.rs", "lib.rs"]
res = {}
for ch in ("mc_chain", "er", "erc"):
    for c in (1, 3):
        for v, name in (("", "full"), ("_W", "W"), ("_X", "X")):
            p = os.path.join(runs, ch, f"c{c}", "work", f"text_budget{v}.caps.out.json")
            t = json.load(open(p))
            M = {k.split("/src/")[-1]: x for k, x in t["function_multiplicity"].items()}
            fb, fd = collections.Counter(), collections.Counter()
            for r in t["rows"]:
                f = r["file"].split("/src/")[-1]
                if f in FILES:
                    fb[f] += r["req"]
                    if r["kind"] in ("diag", "diag_literal"):
                        fd[f] += r["mult"]
            res[f"{ch}/c{c}/{name}"] = {"TAV": t["total_text_requested_bytes"], "D": t["D_diagnostics"],
                                        "M": {f: M.get(f, 0) for f in FUNCS},
                                        "file_text_bytes": {f: fb[f] for f in FILES}, "file_diagnostics": {f: fd[f] for f in FILES}}
json.dump(res, open(out, "w"), indent=1)
for k in ("mc_chain/c3/W", "erc/c3/W", "erc/c3/X", "er/c3/W", "er/c3/X", "erc/c1/W"):
    r = res[k]
    print(k, "TAV", r["TAV"], "D", r["D"])
    print("   M", {f.split(":")[-1]: x for f, x in r["M"].items() if x})
    print("   bytes", {f: x for f, x in r["file_text_bytes"].items() if x and f != "lib.rs"})
