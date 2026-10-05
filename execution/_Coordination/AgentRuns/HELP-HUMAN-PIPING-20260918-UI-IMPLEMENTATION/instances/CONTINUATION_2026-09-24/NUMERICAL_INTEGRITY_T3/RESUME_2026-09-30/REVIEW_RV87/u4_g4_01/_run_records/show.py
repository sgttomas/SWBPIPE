import json, sys
for f in sys.argv[1:]:
    d=json.load(open(f)); print("==",f, "P",d["P"],"DENV",d["D_env"])
    print(" input check", d["input_eval_check"])
    print(" TAV", d["TAV"], "resum", d["TAV_resum_check"])
    print(" corrections", json.dumps(d["corrections_bytes"],indent=0))
    for md in ["sparse","dense"]:
        v=d["base"][md]; u=d["packet_scale_uncorrected_for_comparison"][md]
        print(md, "max", v["max_phase"], v["max"], round(v["max_fraction"],4), "margin0.9", v["margin_to_0.9M"], "toM", v["margin_to_M"], "| uncorrected", u["max"], round(u["max_fraction"],4))
        print("  comps", v["components"])
        for p,x in v["phases"].items(): print("   ",p, x["requested"], x["moving"], x["E_mov_plus_R"], x["fraction"], "| unc", u["phases"][p]["E_mov_plus_R"])
        print("  T16", v["T16_stages"]); print("  T17", v["T17_stages"])
    print(" +10%", d["stride_plus10"]); print(" -10%", d["stride_minus10"])
    print(" W3 split", d["W3_dense_stride_vs_bytes"], d["W3_dense_stride_total"])
