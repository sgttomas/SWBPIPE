#!/usr/bin/env python3
"""J7 RF census inputs: classify every frozen T3 R1 case against D1's source family
and author product-v3 (0.3.0, exact_pressure_v3, explicitly empty pressure_regions)
requests for the source-family cases and for the authorable must-refuse mechanisms.

Test-only diagnostic input generator. It reads the pinned, unchanged references and
writes no expected value: every expectation stays in references.json. Adapted from the
uncommitted J7 RF pilot generator (same authoring rules, extended to every case and
with an explicit classification record per case).

Usage (from any directory):
  python3 -I j7_rf_census_generate.py <out.json>
"""
import hashlib
import importlib.util
import json
import sys
from fractions import Fraction
from pathlib import Path

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[4]
REFS = ROOT / "validation/references/t3_r1/REFERENCES"
PINS = {
    "references.json": "7b176dbbf2296be02d8bca19c698ee56d0d5753751150a9175e0d3ad4cf89cc9",
    "references.py": "80d473a7351e92a2a903d233b9e633aac794aeff07f3ac41e40d25a0d94fc8a8",
}
PROV = "invented_frozen_RF_J7_census_product_adapter_no_library_data"
DOFS = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
DOF_LIMIT = 256  # D1: at most 256 DOFs (source_recovery::DENSE_SOURCE_DOF_LIMIT)


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


# Existing standard-library adapter helpers (exact R1 spelling parser, single RN64,
# y_reference rule). Only these three pure helpers are used.
adapter = load("j7_census_adapter", ROOT / "validation/benchmarks/numerical_robustness/cases/gen_vk_cases.py")
exact, fl = adapter.exact, adapter.fl

for name, digest in PINS.items():
    actual = hashlib.sha256((REFS / name).read_bytes()).hexdigest()
    assert actual == digest, (name, actual)
refs = json.loads((REFS / "references.json").read_text())
template = json.loads((ROOT / "fixtures/product_preview/physics_source/n05.request.json").read_text())


def node_count(model):
    if "nodes_m" in model:
        return len(model["nodes_m"]), "references.json nodes_m"
    text = model["generator"]
    return int(text.split(" has ")[1].split(" nodes")[0]), "references.json abbreviated-model generator text: " + text


def spring_axes(model):
    out = []
    for node, sup in model["supports"].items():
        for ordinal, spring in enumerate(sup.get("springs", [])):
            nonzero = [i for i, x in enumerate(spring["direction"]) if exact(x) != 0]
            out.append((node, ordinal, spring, nonzero))
    return out


def classify(cid, ref):
    model = ref["model"]
    reasons = []
    evidence = {}
    nodes, how = node_count(model)
    dofs = 6 * nodes
    evidence["nodes"] = nodes
    evidence["global_dofs"] = dofs
    evidence["node_count_source"] = how
    if ref["family"] == "RF-MECH" and "expected" not in ref:
        reasons.append("must_refuse_mechanism")
        evidence["expected_outcome"] = ref.get("expected_outcome")
    if dofs > DOF_LIMIT:
        reasons.append("dofs_exceed_256")
    if "member_uniform_loads_N_per_m_global" in model:
        reasons.append("non_nodal_load_member_uniform_load")
        evidence["member_uniform_loads"] = model["member_uniform_loads_N_per_m_global"]
    directional = [
        {"node": n, "ordinal": o, "kind": s["kind"], "direction": s["direction"]}
        for n, o, s, nz in spring_axes(model)
        if len(nz) != 1
    ]
    if directional:
        reasons.append("directional_spring_not_authorable")
        evidence["directional_springs"] = directional
        evidence["product_spring_input"] = (
            "PreviewSupport.stiffness is SupportStiffnessInput{dof, value}: one global DOF per spring "
            "(core/product_physics/src/lib.rs); no direction vector exists in the product model"
        )
    evidence["needs_directional_spring_flag"] = ref.get("flags", {}).get("needs_directional_spring")
    assert bool(directional) == bool(evidence["needs_directional_spring_flag"]), cid
    return reasons, evidence


def make(cid, ref):
    m = ref["model"]
    units = ref.get("units") or {"length": "m", "force": "N", "moment": "N*m", "modulus": "Pa"}
    mm = units["length"] == "mm"
    basis = ref.get("basis", "intended")
    translations = []
    request = json.loads(json.dumps(template))
    out = request["model"]
    out["schema_version"] = "0.3.0"
    out["pressure_contract"] = {"version": "3.0.0", "mode": "exact_pressure_v3"}
    out["project"]["id"] = "invented:j7-census:" + cid
    out["project"]["name"] = "Invented J7 RF census " + cid
    out["project"]["units"]["length"] = units["length"]
    out["nodes"] = [
        {"id": n, "position": dict(zip("xyz", [fl(exact(x)) for x in xyz])), "provenance": PROV}
        for n, xyz in m["nodes_m"].items()
    ]
    out["materials"] = []
    eg = []
    for sid, sec in m["sections"].items():
        e = exact(sec["E"])
        nu = exact(sec["nu"]) if "nu" in sec else e / (2 * exact(sec["G"])) - 1
        assert -1 < nu < Fraction(1, 2)
        if "G" in sec:
            assert e / (2 * (1 + nu)) == exact(sec["G"])
        eg.append({"section": sid, "E": sec["E"], "G": sec.get("G"), "nu_exact": str(nu),
                   "nu_binary64_exact": Fraction(fl(nu)) == nu})
        out["materials"].append({
            "id": "mat:" + sid,
            "elastic_modulus": {"value": fl(e), "unit": units["modulus"]},
            "poisson_ratio": {"value": fl(nu), "unit": "1"},
            "constitutive_basis": "homogeneous_isotropic_E_nu_v1",
            "provenance": PROV,
        })
    out["pipe_segments"] = []
    sections = []
    for name, a, b, sid in m["members"]:
        sec = m["sections"][sid]
        d = exact(sec["OD"])
        inside = exact(sec["ID"])
        if basis == "represented":
            d, inside = Fraction(fl(d)), Fraction(fl(inside))
        wall = (d - inside) / 2
        assert d - 2 * wall == inside
        if basis == "represented":
            assert Fraction(fl(wall)) == wall
        coords = [[fl(exact(x)) for x in m["nodes_m"][n]] for n in [a, b]]
        out["pipe_segments"].append({
            "id": name, "from": a, "to": b, "material": "mat:" + sid,
            "y_reference": dict(zip("xyz", adapter.y_reference(*coords))),
            "section": {"outside_diameter": {"value": fl(d), "unit": units["length"]},
                        "wall_thickness": {"value": fl(wall), "unit": units["length"]}},
            "provenance": PROV,
        })
        sections.append({"member": name, "OD": sec["OD"], "ID": sec["ID"], "wall_exact": str(wall),
                         "wall_binary64_exact": Fraction(fl(wall)) == wall,
                         "parameterization": "represented pair RN64(OD), RN64(ID), exact half-difference" if basis == "represented" else "intended (OD-ID)/2 then single RN64"})
    out["supports"] = []
    for node, sup in m["supports"].items():
        if sup.get("rigid"):
            out["supports"].append({"id": "rigid:" + node, "node": node, "family": "anchor",
                                    "restraints": sup["rigid"], "provenance": PROV})
        for ordinal, spring in enumerate(sup.get("springs", [])):
            nonzero = [i for i, x in enumerate(spring["direction"]) if exact(x) != 0]
            assert len(nonzero) == 1, (cid, node, ordinal)
            rot = spring["kind"] == "rotation"
            dof = DOFS[nonzero[0] + (3 if rot else 0)]
            factor = Fraction(1, 1000) if rot and mm else Fraction(1)
            if factor != 1:
                translations.append({"input": f"spring:{node}:{ordinal}", "source_value": spring["k"],
                                     "source_unit": "N*mm/rad", "target_unit": "N*m/rad",
                                     "exact_positive_factor": "1/1000",
                                     "rounding": "single RN64 after exact translation (product has no N*mm/rad unit)"})
            out["supports"].append({
                "id": f"spring:{node}:{ordinal}", "node": node, "family": "spring", "restraints": [dof],
                "stiffness": {"dof": dof, "value": {"value": fl(exact(spring["k"]) * factor),
                                                    "unit": "N*m/rad" if rot else ("N/mm" if mm else "N/m")}},
                "provenance": PROV,
            })
    loads = []

    def nodal(node, dof, value, identifier):
        rot = dof.startswith("R")
        unit = units["moment"] if rot else units["force"]
        factor = Fraction(1, 1000) if rot and unit == "N*mm" else Fraction(1)
        if factor != 1:
            translations.append({"input": identifier, "source_value": str(value), "source_unit": unit,
                                 "target_unit": "N*m", "exact_positive_factor": "1/1000",
                                 "rounding": "single RN64 after exact translation (product has no N*mm unit)"})
            unit = "N*m"
        loads.append({
            "id": identifier, "category": "concentrated_moment" if rot else "concentrated_force",
            "target": {"type": "node", "node": node},
            "direction": dof if rot else "global_" + dof[-1].lower(),
            "magnitude": {"value": fl(value * factor), "unit": unit},
            "dimension": "moment" if rot else "force", "provenance": PROV,
        })

    for node, load in m["loads"].items():
        for key, offset in [("F", 0), ("M", 3)]:
            for axis, value in enumerate(load.get(key, ["0"] * 3)):
                value = exact(value)
                if value != 0:
                    nodal(node, DOFS[offset + axis], value, "l" + str(len(loads)))
    for ordinal, (node, dof, value) in enumerate(m.get("load_contributions_in_authored_order", [])):
        nodal(node, dof, exact(value), "c" + str(ordinal))
    out["load_cases"] = [{"id": "case", "label": cid, "kind": "primitive_user_load", "primitive_loads": loads,
                          "pressure_regions": [], "provenance": PROV}]
    out["combinations"] = []
    out["components"] = []
    request["materials"] = []
    entry = {"id": cid, "family": ref["family"], "basis": basis, "request": request,
             "eg_mapping": eg, "section_mapping": sections}
    if translations:
        entry["input_translations"] = translations
    return entry


def main():
    classification = []
    cases = []
    for cid, ref in refs["cases"].items():
        reasons, evidence = classify(cid, ref)
        in_family = not reasons
        record = {"id": cid, "family": ref["family"], "in_source_family": in_family,
                  "reasons": reasons, "evidence": evidence}
        authorable = in_family or (reasons == ["must_refuse_mechanism"])
        record["authored"] = authorable
        classification.append(record)
        if authorable:
            entry = make(cid, ref)
            entry["role"] = "source_family" if in_family else "must_refuse_mechanism"
            cases.append(entry)
    document = {"artifact": "J7 RF census product-v3 inputs", "reference_pins": PINS,
                "classification": classification, "cases": cases}
    text = json.dumps(document, ensure_ascii=True, separators=(",", ":"), allow_nan=False) + "\n"
    Path(sys.argv[1]).write_text(text)
    fam = sum(1 for c in classification if c["in_source_family"])
    print("cases", len(classification), "source_family", fam, "authored", len(cases),
          "bytes", len(text), "sha256", hashlib.sha256(text.encode()).hexdigest())


if __name__ == "__main__":
    main()
