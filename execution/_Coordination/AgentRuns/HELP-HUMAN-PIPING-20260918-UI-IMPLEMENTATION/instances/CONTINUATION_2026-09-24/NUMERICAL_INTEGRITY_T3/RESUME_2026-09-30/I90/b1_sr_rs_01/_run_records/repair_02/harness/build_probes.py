"""I90 SR-RS repair 2: rebuild probes_all.json, RV113's 103 SR-RS/SR-TS probes, its (f)/(g) table and its
repair-01 table, in that order, each entry tagged with its set. The inputs are RV113's committed records.
Usage: build_probes.py <R> <out.json>"""
import json, sys
R, out = sys.argv[1:]
E = f"{R}/REVIEW_RV113/rvr_sr_py_01/evidence"
allp = []
for name, f in [("v5", "probes/probes_v5.json"), ("fg", "fg/probes_fg.json"), ("rp", "repair_probes/probes_rp.json")]:
    for p in json.load(open(f"{E}/{f}")):
        p = dict(p); p["set"] = name; allp.append(p)
assert len({p["id"] for p in allp}) == len(allp) == 135
json.dump(allp, open(out, "w"))
