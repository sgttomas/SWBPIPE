"""Probe: a material basis used only by a case with no CaseSource (07j's not_required case given its own
named modulus basis). Is that basis's material list checked? Usage: probe_sourceless_basis.py <P root>"""
import json, sys
from copy import deepcopy
from pathlib import Path
P = Path(sys.argv[1]); sys.path.insert(0, str(P)); sys.path.insert(0, str(Path(__file__).parent))
from core.analysis_runs import retained_precision as rp
import census_lib as lib
data = json.loads((P / "fixtures/results/retained_precision_cases.json").read_text())
bases = {f["id"]: f for f in data["cases"]}
must = next(e for e in data["must_pass"] if e["id"] == "not_required_second_case_checks_passed")
B = ["retained_precision", "body"]
s = lambda path, v: {"path": path, "op": "set", "value": v}
point = {"id": "tp:b1", "temperature": {"value": 400, "unit": "K"}, "elastic_modulus": {"value": 400000000000.0, "unit": "Pa"},
         "shear_modulus": {"value": 154000000000.0, "unit": "Pa"}, "thermal_expansion_coefficient": {"value": 1e-05, "unit": "1/K"}}
mb0 = deepcopy(bases[must["base"]]["source"]["retained_precision"]["body"]["material_bases"][0])
mb1 = {"index": 1, "selector": {"kind": "named", "id": "tp:b1"}, "materials": [dict(deepcopy(mb0["materials"][0]),
       elastic_modulus=rp.bits(4e11), shear_modulus=rp.bits(1.54e11), selection={"kind": "named_point", "point_id": "tp:b1"})], "case_indices": [1]}
inv = [s(["request", "model", "materials", 0, "temperature_points"], [point]), s(["request", "model", "load_cases", 1, "modulus_basis_ref"], "tp:b1")]
def verdict(extra):
    edits = deepcopy(must["edits"]) + [s(B + ["ordinary_attempts", 1, "material_basis_ref"], 1),
                                       s(B + ["material_bases"], [dict(mb0, case_indices=[0]), extra])]
    src, invocation = lib.apply_entry(bases[must["base"]], {"edits": edits, "invocation_edits": inv, "rehash": "all"})
    try: r = rp.validate_retained_precision(src, invocation); return f"admitted standing={r['standing']}"
    except rp.RetainedPrecisionError as e: return f"{e.gate} {e.code}"
print("the not_required case on its own named basis, correct:", verdict(mb1))
m = deepcopy(mb1); m["materials"][0]["elastic_modulus"] = rp.bits(3e11)
print("  its basis's elastic modulus wrong:", verdict(m))
m = deepcopy(mb1); m["materials"][0]["selection"] = {"kind": "base"}
print("  its basis's material selection wrong:", verdict(m))
m = deepcopy(mb1); m["materials"] = []
print("  its basis lists no material:", verdict(m))
m = deepcopy(mb1); m["materials"][0]["id"] = "material:other"
print("  its basis names another material:", verdict(m))
