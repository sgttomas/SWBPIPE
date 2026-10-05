"""RV89: each generated form at the chain's illustrative strides, against G4's l <= 128 record."""
import json, sys
G4, P2T, P2O, G4T25, P2T25 = sys.argv[1:6]
a = json.load(open(G4)); t = json.load(open(P2T)); b = json.load(open(P2O))
ta = json.load(open(G4T25)); tb = json.load(open(P2T25))
A = dict(t["assumed"]); A.update(t["text_atoms"])
def ev(f): return sum(v * (1 if k == "1" else A[k]) for k, v in t["forms"][f].items())
ca = a["modes"]["sparse"]["components"]; cd = a["modes"]["dense"]["components"]
g4 = {
 "O_base_sparse": ca["O_without_T25"], "O_base_dense": cd["O_without_T25"], "TAV_W": ca["TAV_W"], "TAV_X": ca["TAV_X"],
 "T11": ca["T11"], "STAGED": ca["STAGED"], "STATICS": ca["STATICS"], "SUCC": ca["SUCC"], "INVOC": ca["INVOC"], "T19": ca["T19"],
 "NOTICE": ca["T18_reserve"], "T16_P1": a["T16_stages"]["P1 env to_value + successor deltas + members + source identity hash"],
 "T16_P2": a["T16_stages"]["P2 body json! + hash(publication)"], "T16_P3": a["T16_stages"]["P3 hash(body)"],
 "T16_P4": a["T16_stages"]["P4 retained_precision insert (third body copy)"], "T16_moving": a["facts"]["ENV_S_text"],
 "T17_moving_publication": a["facts"]["ENV_S_text"], "T17_moving_invocation": a["facts"]["INVOC_text"],
 "T17_V1": a["T17_stages"]["V1 G1 receipt hash"], "T17_V3": a["T17_stages"]["V3 G1 source and preparation hashes"],
 "T17_V4": a["T17_stages"]["V4 G3-G6 working sets"], "T17_V5": a["T17_stages"]["V5 G7 project + preview evidence"],
 "T17_V6": a["T17_stages"]["V6 G8 projected + invocation hash + model rebuild"],
 "T25_moving": ta["publication_text_upper"] if "publication_text_upper" in ta else ta["facts"]["publication_text_upper"],
 "TXT_moving": 2599962, "HELPER_moving": 8388608,
}
for k, v in ta["stages"].items():
    key = {"I1": "T25_I1", "I2": "T25_I2", "I3": "T25_I3", "S1": "T25_S1", "S2": "T25_S2", "S3": "T25_S3", "S4": "T25_S4", "S5": "T25_S5"}.get(k.split()[0])
    if key: g4[key] = v
g4["T12_T15(sum)"] = ca["T12_T15"]
g4["T17(V2 hash + output)"] = a["T17_stages"]["V2 G1 public clone + hash(publication)"]
g4["T25(total)"] = ca["T25"]
g4["T16(max)"] = ca["T16"]
rows = []
for f in sorted(t["forms"]):
    rows.append((f, g4.get(f), ev(f)))
rows.append(("T12_T15(sum)", g4["T12_T15(sum)"], sum(ev(x) for x in ["T12", "T13", "T14", "T15"])))
rows.append(("T17(V2 hash + output)", g4["T17(V2 hash + output)"], ev("T17_V2_hash") + ev("T17_output")))
rows.append(("T16(max)", g4["T16(max)"], max(ev(x) for x in ["T16_P1", "T16_P2", "T16_P3", "T16_P4"])))
rows.append(("T25(total)", g4["T25(total)"], ev("T25_carried_case") + max(ev(x) for x in ["T25_S1","T25_S2","T25_S3","T25_S4","T25_S5","T25_I1","T25_I2","T25_I3"])))
for f, x, y in rows:
    print(f"{f:24s} {('—' if x is None else format(x, ',d')):>16} {y:>16,d} {('' if x is None else format(y - x, '+,d')):>14}")
