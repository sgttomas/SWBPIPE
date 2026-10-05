"""G6 repair (RV87 SF-3): enforcement controls. Each control is one change to a scratch copy of
text_args.g4.json; the whole-run text_budget.py (D = 14,734, the run's fixpoint) must then be
incomplete with the named finding, and the unmodified copy must be complete.
Usage: python3 audit_controls_g6r.py <sens work dir> <snapshot P dir> <edges json> <lexicon json> <out dir>"""
import json, os, subprocess, sys, copy
work, snap, edges, lexicon, out = sys.argv[1:6]
os.makedirs(out, exist_ok=True)
TA0 = json.load(open(os.path.join(work, "text_args.g4.json")))
def drop(site, k):
    def f(ta):
        del ta["id_audit"][site][k]
        if not ta["id_audit"][site]:
            del ta["id_audit"][site]
    return f
def add(fam, k, v):
    def f(ta):
        ta.setdefault(fam, {})[k] = v
    return f
def rekey(fam, old, new):
    def f(ta):
        ta[fam][new] = ta[fam].pop(old)
    return f
def bogus_entry(site):
    def f(ta):
        ta["id_audit"][site]["no_such_expression.id"] = {"bytes": 128, "source": "IN128"}
    return f
CONTROLS = [
    ("c0_unmodified", None, None),
    ("c1_rv87_c2_primitive_loads_299", drop("core/loads/primitive_loads/src/lib.rs:299", "actual.schema_ref()"), ("id-unaudited", "primitive_loads/src/lib.rs:299 actual.schema_ref()")),
    ("c2_stale_site_zero_key", add("site_zero", "product_physics/src/lib.rs:99999", {"why": "control: a key matching no row"}), ("stale-key", "site_zero product_physics/src/lib.rs:99999")),
    ("c3_rv87_c1_lib_5592", drop("core/product_physics/src/lib.rs:5592", "matched[0].id"), ("id-unaudited", "lib.rs:5592 matched[0].id")),
    ("c4_sf1_label_1708", drop("core/product_physics/src/lib.rs:1708", "integrity_dof_label(model,record.global_dof)"), ("id-unaudited", "lib.rs:1708 integrity_dof_label")),
    ("c5_sf2_copy_3125", drop("core/product_physics/src/retained_product.rs:3125", "s"), ("id-unaudited", "retained_product.rs:3125 s")),
    ("c6_sf2_lex_key_back_to_2963", rekey("lex_site_size", "retained_product.rs:3125", "retained_product.rs:2963"), ("stale-key", "lex_site_size retained_product.rs:2963")),
    ("c7_stale_audit_entry", bogus_entry("core/product_physics/src/lib.rs:5592"), ("stale-audit-entry", "lib.rs:5592 no_such_expression.id")),
    ("c8_static_class_entry_8795", drop("core/product_physics/src/lib.rs:8795", "label"), ("id-unaudited", "lib.rs:8795 label")),
]
res = []
for name, mod, expect in CONTROLS:
    ta = copy.deepcopy(TA0)
    if mod:
        mod(ta)
    tap = os.path.join(out, f"text_args.{name}.json")
    json.dump(ta, open(tap, "w"), indent=1)
    env = dict(os.environ, G4_CAPS='{"l": 128}', TB_LEXICON=lexicon, TB_COMPOSITE=os.path.join(work, "composite_text.caps.json"), TB_D="14734")
    op = os.path.join(out, f"{name}.out.json")
    with open(op, "w") as fh:
        subprocess.run(["python3", os.path.join(work, "text_budget.py"), snap, os.path.join(work, "template_inventory_head.out.json"), edges,
                        os.path.join(work, "loop_bounds.g4.json"), tap, "run_linear_static_preview_value_with_retained_direct", "caps"],
                       cwd=snap, env=env, stdout=fh, check=True)
    t = json.load(open(op))
    found = [u for u in t["unclassified_args"]]
    hit = expect is None or any(u[0][0] == expect[0] and all(p in u[0][1] for p in expect[1].split(" ")) for u in found)
    ok = (t["complete"] and not found) if expect is None else ((not t["complete"]) and hit and len(found) == 1)
    res.append({"control": name, "expect": expect, "complete": t["complete"], "TAV": t["total_text_requested_bytes"], "findings": found, "pass": ok})
    os.remove(op)   # keep the summary only (the whole-run output is ~MBs)
json.dump(res, open(os.path.join(out, "audit_controls_g6r.out.json"), "w"), indent=1)
for r in res:
    print(r["control"], "complete=", r["complete"], "TAV=", r["TAV"], "pass=", r["pass"], r["findings"][:2])
