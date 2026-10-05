"""I65 U4 G4: the whole cap-priced chain re-evaluated at one cap vector (stdlib only).

For one G4_CAPS override and one escape factor, in a scratch work directory (never the system
temp directory): composite_text -> the four T08 text runs on the fixed repaired call graph and
lexicon (full, envelope-only, branch X, branch W), iterated to the diagnostics fixpoint D ->
t08_closure -> producer_caps, t07_repair -> t25_g4 -> ordinary_caps -> g4_caps. The call graph and
lexicon do not depend on the caps (they are structural), so they are reused.
Usage: python3 sens.py <work dir> <snapshot P dir> <edges json> <lexicon json> <eps> '<G4_CAPS json>'
Prints one JSON summary line.
"""
import json, os, shutil, subprocess, sys

H = os.path.dirname(os.path.abspath(__file__))
work, snap, edges, lexicon, eps, caps = sys.argv[1:7]
os.makedirs(work, exist_ok=True)
for f in os.listdir(H):
    if f.endswith(".py") or f in ("loop_bounds.g4.json", "text_args.g4.json", "callgraph_rules.g4.json",
                                  "template_inventory_head.out.json", "crate_dirs.txt"):
        shutil.copy(os.path.join(H, f), os.path.join(work, f))
env = dict(os.environ, G4_CAPS=caps)
def run(args, out, extra=None, cwd=None):
    e = dict(env, **(extra or {}))
    with open(os.path.join(work, out), "w") as fh:
        subprocess.run(["python3"] + args, cwd=cwd or work, env=e, stdout=fh, check=True)
for w in ("caps",):
    run([os.path.join(work, "composite_text.py"), snap, w], f"composite_text.{w}.json", cwd=snap)
ROOT = "run_linear_static_preview_value_with_retained_direct"
ZW = "source_receipt.rs:636:exact,source_receipt.rs:839:finalize,source_receipt.rs:853:finalize_composite,source_receipt.rs:867:finalize_for"
ENVZERO = ("core/product_physics/src/source_receipt,core/product_physics/src/retained_wire.rs,core/product_physics/src/retained_product.rs,"
           "core/product_physics/src/retained_receipt.rs,core/reporting/result_export")
def tb(variant, D):
    extra = {"TB_LEXICON": lexicon, "TB_COMPOSITE": os.path.join(work, "composite_text.caps.json"), "TB_D": str(D)}
    if variant == "_env": extra["TB_EXTRA_ZERO"] = ENVZERO
    if variant == "_X": extra["TB_FN_ZERO"] = "lib.rs:2950:retained_w1"
    if variant == "_W": extra["TB_FN_ZERO"] = ZW
    run([os.path.join(work, "text_budget.py"), snap, os.path.join(work, "template_inventory_head.out.json"), edges,
         os.path.join(work, "loop_bounds.g4.json"), os.path.join(work, "text_args.g4.json"), ROOT, "caps"],
        f"text_budget{variant}.caps.out.json", extra, cwd=snap)
    return json.load(open(os.path.join(work, f"text_budget{variant}.caps.out.json")))
D = 25_544
for _ in range(4):                                   # the diagnostics fixpoint (loops over a diagnostics vector use D)
    d_new = tb("", D)["D_diagnostics"]
    if d_new == D:
        break
    D = d_new
complete = True
for v in ("", "_env", "_X", "_W"):
    complete = complete and tb(v, D)["complete"]
run([os.path.join(work, "t08_closure.py"), "caps"], "t08_closure.caps.log")
run([os.path.join(work, "producer_caps.py"), "caps"], "producer_caps.caps.out.json")
run([os.path.join(work, "t07_repair.py"), "caps"], "t07_repair.caps.out.json")
run([os.path.join(work, "t25_g4.py"), "caps", eps], f"t25_g4.caps.eps{eps}.out.json")
shutil.copy(os.path.join(work, f"t25_g4.caps.eps{eps}.out.json"), os.path.join(work, "t25_caps.caps.out.json"))
run([os.path.join(work, "ordinary_caps.py"), "caps"], "ordinary_caps.caps.out.json")
run([os.path.join(work, "g4_caps.py"), "caps", eps, snap], f"g4_caps.caps.eps{eps}.out.json")
g = json.load(open(os.path.join(work, f"g4_caps.caps.eps{eps}.out.json")))
txt = json.load(open(os.path.join(work, "text_closure.caps.json")))["atoms"]
summary = {"caps": json.loads(caps), "eps": int(eps), "text_complete": complete, "D": D, "D_env": txt["D_env"],
           "TAV": txt["TAV_text_requested"]}
for mode in ("sparse", "dense"):
    md = g["modes"][mode]
    summary[mode] = {"admission_phase": md["admission_max"]["phase"].split(" ")[0],
                     "E_mov_plus_R": md["admission_max"]["E_mov_plus_R"], "fraction_of_M": md["admission_max"]["fraction_of_M"],
                     "X1": md["phases"]["X1 ordinary span with T25 (selected finalization)"]["fraction_of_M"],
                     "W4": md["phases"]["W4 precommit validation (T17) with the successor and the invocation Value"]["fraction_of_M"],
                     "margin_rule": md["margin_rule_0.9M"], "fits_M": md["fits_M"],
                     "TAV_X": md["components"]["TAV_X"], "TAV_W": md["components"]["TAV_W"], "T25": md["components"]["T25"],
                     "T16": md["components"]["T16"], "T17": md["components"]["T17"]}
print(json.dumps(summary))
