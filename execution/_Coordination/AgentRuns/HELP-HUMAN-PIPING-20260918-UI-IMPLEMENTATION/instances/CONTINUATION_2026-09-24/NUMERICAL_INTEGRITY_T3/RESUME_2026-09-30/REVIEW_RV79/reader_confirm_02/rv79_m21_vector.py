"""RV79: a negative (rehashed) vector that separates the G5a sanity factor 1+2^-40 from 1+2^-39 (M21).

Negative mutations need not be emittable: only their base must be. On the selected p128 base,
set E (selection and its verification record) to S/(1+1.5*2^-40) per kind, where S is the
reader's own coupled maximum for that body; then ê*(1+2^-40) < S <= ê*(1+2^-39).
"""
import json, math, struct, sys, types
from copy import deepcopy
sys.path.insert(0, ".")
from core.analysis_runs import retained_precision as rp
from tests.test_retained_precision_contract import apply_entry, corpus
C = corpus(); BASE = {f["id"]: f for f in C["cases"]}
O = "ordinary_prepared_synthetic"
b = BASE[O]["source"]["retained_precision"]["body"]; src = b["sources"][0]; s = b["cases"][0]["selection"]
rows = [r for r in BASE[O]["source"]["results"] if r["basis_ref"]["ref_id"] == b["cases"][0]["basis_ref"]["ref_id"]]
prescribed = {(x["node_id"], x["component"]) for x in s["input_derived_dofs"]}
names = ["translation", "rotation", "force", "moment"]
body0 = src["body_membership"][0]
coords = [[rp.from_bits(v) for v in src["id_maps"]["nodes"][int(i)]["coordinates"]] for i in body0["nodes"]]
L = rp._extent(coords); maxima = [0.0] * 4
for row in rows:
    kind = rp._row_kind(row); rb, _ = rp._row_body(row, src); n = rp._normalized(row)
    comp = ("U" + row["kind"][-1].upper()) if row["kind"].startswith("global_nodal_displacement_") else ("R" + row["kind"][-1].upper()) if row["kind"].startswith("global_nodal_rotation_") else None
    inp = comp is not None and (row["entity_ref"], comp) in prescribed
    if rb == 0 and kind in names and not inp: maxima[names.index(kind)] = max(maxima[names.index(kind)], abs(n))
S = rp._coupled(maxima, L)
f = 1 + 1.5 * 2.0 ** -40
e_fo = S[2] / f; e_mo = L * e_fo if L else S[3] / f
hat = rp._e_hat([e_fo, e_mo], L)
ok40 = [hat[0] * float.fromhex("0x1.0000000001000p+0") >= S[2], hat[1] * float.fromhex("0x1.0000000001000p+0") >= S[3]]
ok39 = [hat[0] * float.fromhex("0x1.0000000002000p+0") >= S[2], hat[1] * float.fromhex("0x1.0000000002000p+0") >= S[3]]
print("original E", s["resolution_scale"][0]["force"], rp.from_bits(s["resolution_scale"][0]["force"]), rp.from_bits(s["resolution_scale"][0]["moment"]))
print("L", L, "S", S, "E", [e_fo, e_mo], "passes 2^-40:", ok40, "passes 2^-39:", ok39)
res = [{"body": 0, "force": rp.bits(e_fo), "moment": rp.bits(e_mo)}] + deepcopy(s["resolution_scale"][1:])
B = ["retained_precision", "body"]
edits = [{"path": B + ["cases", 0, "selection", "resolution_scale"], "op": "set", "value": res},
         {"path": B + ["cases", 0, "run", "records", 1, "verification", "resolution"], "op": "set", "value": res}]
# The corpus bases bind the G5a lower test (stiffness x displacement) above S, so the separating
# vector also lowers the echoed member stiffnesses (source and selection) by 2^-40; that later
# defect only matters to a reader that passes the sanity check.
for side in (["sources", 0, "section_terms", 0], ["cases", 0, "selection", "section_terms", 0]):
    for k in ("axial_stiffness", "torsional_stiffness"):
        v = rp.from_bits((b["sources"][0]["section_terms"][0] if side[0] == "sources" else s["section_terms"][0])[k])
        edits.append({"path": B + side + [k], "op": "set", "value": rp.bits(v * 2.0 ** -40)})
entry = {"id": "g5a_sanity_margin_between_2m40_and_2m39", "base": O, "edits": edits, "rehash": "all"}
import traceback
def outcome(mod):
    src2, inv = apply_entry(BASE[O], entry)
    try: mod._validate_draft(src2, inv); return "PASS"
    except mod.RetainedPrecisionError as e: return f"{e.gate} {e.code} lines={[f.lineno for f in traceback.extract_tb(e.__traceback__) if f.filename.endswith('retained_precision.py')]}"
print("reviewed reader:", outcome(rp))
text = open(rp.__file__).read(); old = 'float.fromhex("0x1.0000000001000p+0")'; assert text.count(old) == 1
m21 = types.ModuleType("rv79_m21"); m21.__file__ = rp.__file__; m21.__package__ = "core.analysis_runs"
exec(compile(text.replace(old, 'float.fromhex("0x1.0000000002000p+0")'), rp.__file__, "exec"), m21.__dict__)
print("M21 reader:", outcome(m21))
print(json.dumps(entry)[:300])
