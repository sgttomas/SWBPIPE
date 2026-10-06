"""I68 U8-0 probe: the L = 0 successor's receipt coverage for body 1, its body-1 rows, and body 0 against the milestone."""
import json, struct, sys
out = sys.argv[1]
def bits(x): return struct.pack(">d", x).hex()
for mode in ("sparse_interactive", "dense_scrutiny"):
    l0 = json.load(open(f"{out}/l0_isolated_node_{mode}.json"))["source"]
    ms = json.load(open(f"{out}/milestone_{mode}.json"))["source"]
    b = l0["retained_precision"]["body"]
    print(f"== {mode}")
    print("sources[].body_membership:", [s.get("body_membership") for s in b["sources"]])
    for ai, a in enumerate(b["product_attempts"]):
        cov = a["proof"]["summary_coverage"]
        print(f"product_attempts[{ai}].proof.summary_coverage:", json.dumps(cov))
    for ci, c in enumerate(b["cases"]):
        sel = c.get("selection")
        print(f"cases[{ci}] status={c['status']} selection keys={sorted(sel) if isinstance(sel, dict) else sel}")
        if isinstance(sel, dict):
            for k in ("bodies", "body_scales", "scales", "estimate", "stop_rule"):
                if k in sel: print(f"  selection.{k}:", json.dumps(sel[k])[:900])
    # rows touching N2 / rigid:N2
    ms_rows = {r["id"]: r for r in ms["results"]}
    body1 = [r for r in l0["results"] if "N2" in r["id"] or r.get("entity_ref") in ("N2", "rigid:N2", "node:N2", "support:rigid:N2")]
    print("body-1 rows:", len(body1))
    for r in body1:
        print(f"  {r['id']} kind={r['kind']} value={r['value']!r} bits={bits(r['value'])} entity={r.get('entity_ref')}")
    common = [r for r in l0["results"] if r["id"] in ms_rows]
    differ = [(r["id"], r["value"], ms_rows[r["id"]]["value"]) for r in common if bits(r["value"]) != bits(ms_rows[r["id"]]["value"])]
    only_l0 = [r["id"] for r in l0["results"] if r["id"] not in ms_rows]
    only_ms = [i for i in ms_rows if i not in {r["id"] for r in l0["results"]}]
    print(f"rows: l0={len(l0['results'])} milestone={len(ms['results'])} common={len(common)} common_bits_differ={len(differ)} only_l0={len(only_l0)} only_milestone={len(only_ms)}")
    for d in differ[:40]: print("  differ:", d)
    print("  only_l0:", only_l0)
    print("  only_milestone:", only_ms)
