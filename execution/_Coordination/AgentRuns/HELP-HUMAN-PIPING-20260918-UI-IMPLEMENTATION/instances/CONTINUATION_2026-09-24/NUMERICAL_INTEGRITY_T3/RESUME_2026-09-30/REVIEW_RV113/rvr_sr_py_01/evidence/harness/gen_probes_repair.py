"""RV113 (RV-R): three-reader probes for SR-PY's repair 01 items (b), (c), (d1), (d2) and (e), built here from the
shared corpus (not from I91's tests). 07j's statement = base `two_case_preparation_failure_synthetic` with the
must-pass entry `not_required_second_case_checks_passed`'s edits as a prefix (case 1 not_required, no source).
Every probe's `want` is {"observe": true}; the readers are compared with each other.
Usage: python gen_probes_repair.py <corpus.json> <out.json>"""
import copy
import json
import struct
import sys

corpus = json.load(open(sys.argv[1]))
B = ["retained_precision", "body"]
NR = next(m for m in corpus["must_pass"] if m["id"] == "not_required_second_case_checks_passed")
P = NR["base"]
O = "ordinary_prepared_synthetic"
mb0 = next(c for c in corpus["cases"] if c["id"] == P)["source"]["retained_precision"]["body"]["material_bases"][0]


def bits(x):
    return struct.pack(">d", x).hex()


def s(path, value):
    return {"path": path, "op": "set", "value": value}


def probe(pid, item, base, edits=(), inv=(), prefix=True, note=""):
    pre = copy.deepcopy(NR["edits"]) if prefix else []
    return {"id": pid, "item": item, "base": base, "edits": pre + list(edits), "invocation_edits": list(inv), "rehash": "all",
            "want": {"observe": True}, "want_i1": {"observe": True}, "note": note}


def basis(index, cases, **kw):
    b = copy.deepcopy(mb0)
    b.update(index=index, case_indices=cases, **kw)
    return b


point = {"id": "tp:rv113", "temperature": {"value": 400, "unit": "K"}, "elastic_modulus": {"value": 4.0e11, "unit": "Pa"},
         "shear_modulus": {"value": 1.54e11, "unit": "Pa"}, "thermal_expansion_coefficient": {"value": 1e-05, "unit": "1/K"}}
named_inv = [s(["request", "model", "materials", 0, "temperature_points"], [point]),
             s(["request", "model", "load_cases", 1, "modulus_basis_ref"], "tp:rv113")]


def named(**change):
    m = copy.deepcopy(mb0["materials"][0])
    m.update(elastic_modulus=bits(4.0e11), shear_modulus=bits(1.54e11), selection={"kind": "named_point", "point_id": "tp:rv113"})
    m.update(change)
    return {"index": 1, "selector": {"kind": "named", "id": "tp:rv113"}, "case_indices": [1], "materials": [m]}


def e_probe(pid, second, note=""):
    bases = [basis(0, [0])] + ([second] if second is not None else [])
    return probe(pid, "(e)" if second is not None else "(c) count", P,
                 [s(B + ["ordinary_attempts", 1, "material_basis_ref"], 1), s(B + ["material_bases"], bases)], named_inv, note=note)


R = [
    probe("r_control_07j", "control", P, note="07j's statement unedited"),
    probe("r_b_basis_ref_7", "(b)", P, [s(B + ["ordinary_attempts", 1, "material_basis_ref"], 7)]),
    probe("r_b_basis_ref_1_second_basis", "(b)", P, [s(B + ["ordinary_attempts", 1, "material_basis_ref"], 1),
                                                     s(B + ["material_bases"], [basis(0, [0]), basis(1, [1])])],
          note="a second base-selector basis lists case 1; one selector, two bases"),
    probe("r_c_basis_omits_case_1", "(c)", P, [s(B + ["material_bases", 0, "case_indices"], [0])]),
    probe("r_c_cases_out_of_order", "(c)", P, [s(B + ["material_bases", 0, "case_indices"], [1, 0])]),
    probe("r_c_extra_empty_basis", "(c)", P, [s(B + ["material_bases"], [basis(0, [0, 1]), basis(1, [])])]),
    probe("r_d1_extra_member", "(d1)", O, inv=[s(["extra"], 1)], prefix=False),
    probe("r_d2_mode_unknown", "(d2)", O, inv=[s(["solver_mode"], "foo")], prefix=False),
    probe("r_d2_mode_list", "(d2)", O, inv=[s(["solver_mode"], ["sparse_interactive"])], prefix=False),
    probe("r_d2_mode_removed", "(d2)", O, inv=[{"path": ["solver_mode"], "op": "remove"}], prefix=False),
    probe("r_d2_mode_null", "(d2)", O, inv=[s(["solver_mode"], None)], prefix=False),
    e_probe("r_e_control_named_basis", named(), "case 1 on its own named basis, materials correct"),
    e_probe("r_e_elastic_modulus_wrong", named(elastic_modulus=bits(3.0e11))),
    e_probe("r_e_selection_base", named(selection={"kind": "base"})),
    e_probe("r_e_material_id_other", named(id="material:other")),
    e_probe("r_e_no_material", {**named(), "materials": []}),
    e_probe("r_c_missing_sourceless_basis", None, "the named basis absent; case 1's attempt names basis 1"),
]
json.dump(R, open(sys.argv[2], "w"), indent=1)
print(len(R), "probes ->", sys.argv[2])
