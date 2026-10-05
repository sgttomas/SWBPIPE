"""Compare RV87's packet-law reproduction with the packet's g4_caps outputs, phase by phase."""
import json, sys
RR = sys.argv[1]
out = {}
for mine, theirs in [("rv87_g4.l128.out.json", "g4_caps.caps.eps2.l128.out.json"), ("rv87_g4.l192.out.json", "g4_caps.caps.eps2.out.json")]:
    d = json.load(open(mine)); t = json.load(open(RR + "/" + theirs))
    rows = []
    for md in ["sparse", "dense"]:
        rp = d["packet_laws_reproduction"][md]["phases"]; tp = t["modes"][md]["phases"]
        for (pk, pv), (tk, tv) in zip(rp.items(), tp.items()):
            rows.append([md, pk, pv["E_mov_plus_R"], tv["E_mov_plus_R"], pv["E_mov_plus_R"] - tv["E_mov_plus_R"]])
        for k_ in ["T16", "T17", "STAGED", "SUCC", "INVOC", "STATICS", "T19", "O_without_T25", "T25", "TAV_X", "TAV_W", "T11", "T12_T15", "T18_reserve"]:
            mk = {"O_without_T25": "O_base", "T12_T15": "T12_15", "T18_reserve": "N1"}.get(k_, k_)
            rows.append([md, "component " + k_, d["packet_laws_reproduction"][md]["components"][mk], t["modes"][md]["components"][k_],
                         d["packet_laws_reproduction"][md]["components"][mk] - t["modes"][md]["components"][k_]])
    out[mine] = {"exact": all(r[4] == 0 for r in rows), "rows": rows}
print(json.dumps(out, indent=1))
