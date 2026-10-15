#!/usr/bin/env python3
"""J7 RF census inputs (b7-plan, "The J7 RF census").

Reads the frozen T3 R1 references (sha256-pinned, never modified) and writes the
compact test-fixture input of the J7 census check
(`core/product_physics/src/source_recovery/j7_rf_census.rs`):

- `classification`: every one of the 216 frozen cases with its D1 source-family
  reasons (empty when in the family);
- `cases`: one genuine product request per source-family case: model 0.3.0, the v3
  contract `3.0.0/exact_pressure_v3`, explicitly empty `pressure_regions`. The Rust
  check derives each v2 twin by replacing only the contract.

Nothing here computes or copies an expected value; every expectation stays in
references.json and is judged by the Rust check at the unchanged criterion.
Authoring rules: every intended input is rounded once to binary64 (the
represented-basis case uses R1's RN64(OD), RN64(ID) pair); E with nu = E/(2G) - 1
(the exact contract derives G); OD with wall (OD - ID)/2; N*mm moments and
N*mm/rad springs, which the product cannot express, are translated by the exact
factor 1/1000 before the single rounding; the y reference follows the existing
VP-ROBUST adapter rule.

Usage:
    python3 tools/fixtures/j7_rf_census_inputs.py            # write the fixture
    python3 tools/fixtures/j7_rf_census_inputs.py --check    # compare, write nothing
"""
import hashlib
import importlib.util
import json
import sys
from fractions import Fraction
from pathlib import Path

sys.dont_write_bytecode = True
PROJECT = Path(__file__).resolve().parents[2]
REFS = PROJECT / "validation/references/t3_r1/REFERENCES"
FIXTURE = PROJECT / "fixtures/results/j7_rf_census_inputs.json"
PINS = {
    "references.json": "7b176dbbf2296be02d8bca19c698ee56d0d5753751150a9175e0d3ad4cf89cc9",
    "references.py": "80d473a7351e92a2a903d233b9e633aac794aeff07f3ac41e40d25a0d94fc8a8",
}
PROV = "invented_frozen_RF_J7_census_product_adapter_no_library_data"
DOFS = ["UX", "UY", "UZ", "RX", "RY", "RZ"]
DOF_LIMIT = 256  # D1: at most 256 DOFs (source_recovery::DENSE_SOURCE_DOF_LIMIT)
TEMPLATE = PROJECT / "fixtures/product_preview/pressure_source/physics_source/n05.request.json"


def _load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


# Existing standard-library helpers only: R1's exact input spelling, single RN64 and
# the VP-ROBUST y-reference rule.
_ADAPTER = _load("j7_census_adapter", PROJECT / "validation/benchmarks/numerical_robustness/cases/gen_vk_cases.py")
_STORAGE = _load("j7_census_storage", PROJECT / "tools/fixtures/retained_precision_corpus.py")
exact, fl = _ADAPTER.exact, _ADAPTER.fl


def node_count(model):
    if "nodes_m" in model:
        return len(model["nodes_m"])
    return int(model["generator"].split(" has ")[1].split(" nodes")[0])


def classify(ref):
    """D1 source-family reasons (empty list: in the family)."""
    model = ref["model"]
    reasons = []
    if ref["family"] == "RF-MECH" and "expected" not in ref:
        reasons.append("must_refuse_mechanism")
    if 6 * node_count(model) > DOF_LIMIT:
        reasons.append("dofs_exceed_256")
    if "member_uniform_loads_N_per_m_global" in model:
        reasons.append("non_nodal_load")
    directional = any(
        sum(1 for x in spring["direction"] if exact(x) != 0) != 1
        for sup in model["supports"].values() for spring in sup.get("springs", [])
    )
    assert directional == bool(ref.get("flags", {}).get("needs_directional_spring"))
    if directional:
        reasons.append("directional_spring_not_authorable")
    return reasons


def make(cid, ref, template):
    m = ref["model"]
    units = ref["units"]
    mm = units["length"] == "mm"
    basis = ref["basis"]
    translations = []
    request = json.loads(json.dumps(template))
    out = request["model"]
    out["schema_version"] = "0.3.0"
    # The template is the current 0.5.0 pressure-source request (B7b); the census inputs keep their committed
    # 0.3.0 spelling, with the contract member last as it was authored, so the fixture bytes are unchanged.
    out.pop("pressure_contract", None)
    out["pressure_contract"] = {"version": "3.0.0", "mode": "exact_pressure_v3"}
    out["project"]["id"] = "invented:j7-census:" + cid
    out["project"]["name"] = "Invented J7 RF census " + cid
    out["project"]["units"]["length"] = units["length"]
    out["nodes"] = [{"id": n, "position": dict(zip("xyz", [fl(exact(x)) for x in xyz])), "provenance": PROV}
                    for n, xyz in m["nodes_m"].items()]
    out["materials"] = []
    for sid, sec in m["sections"].items():
        e = exact(sec["E"])
        nu = exact(sec["nu"]) if "nu" in sec else e / (2 * exact(sec["G"])) - 1
        assert -1 < nu < Fraction(1, 2)
        if "G" in sec:
            assert e / (2 * (1 + nu)) == exact(sec["G"])
        out["materials"].append({
            "id": "mat:" + sid, "elastic_modulus": {"value": fl(e), "unit": units["modulus"]},
            "poisson_ratio": {"value": fl(nu), "unit": "1"},
            "constitutive_basis": "homogeneous_isotropic_E_nu_v1", "provenance": PROV,
        })
    out["pipe_segments"] = []
    for name, a, b, sid in m["members"]:
        sec = m["sections"][sid]
        d, inside = exact(sec["OD"]), exact(sec["ID"])
        if basis == "represented":
            d, inside = Fraction(fl(d)), Fraction(fl(inside))
        wall = (d - inside) / 2
        assert d - 2 * wall == inside
        if basis == "represented":
            assert Fraction(fl(wall)) == wall
        coords = [[fl(exact(x)) for x in m["nodes_m"][n]] for n in [a, b]]
        out["pipe_segments"].append({
            "id": name, "from": a, "to": b, "material": "mat:" + sid,
            "y_reference": dict(zip("xyz", _ADAPTER.y_reference(*coords))),
            "section": {"outside_diameter": {"value": fl(d), "unit": units["length"]},
                        "wall_thickness": {"value": fl(wall), "unit": units["length"]}},
            "provenance": PROV,
        })
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
                translations.append({"input": f"spring:{node}:{ordinal}", "source_unit": "N*mm/rad",
                                     "target_unit": "N*m/rad", "exact_positive_factor": "1/1000"})
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
            translations.append({"input": identifier, "source_unit": unit, "target_unit": "N*m",
                                 "exact_positive_factor": "1/1000"})
            unit = "N*m"
        loads.append({
            "id": identifier, "category": "concentrated_moment" if rot else "concentrated_force",
            "target": {"type": "node", "node": node}, "direction": dof if rot else "global_" + dof[-1].lower(),
            "magnitude": {"value": fl(value * factor), "unit": unit},
            "dimension": "moment" if rot else "force", "provenance": PROV,
        })

    for node, load in m["loads"].items():
        for key, offset in [("F", 0), ("M", 3)]:
            for axis, value in enumerate(load.get(key, ["0"] * 3)):
                if exact(value) != 0:
                    nodal(node, DOFS[offset + axis], exact(value), "l" + str(len(loads)))
    for ordinal, (node, dof, value) in enumerate(m.get("load_contributions_in_authored_order", [])):
        nodal(node, dof, exact(value), "c" + str(ordinal))
    out["load_cases"] = [{"id": "case", "label": cid, "kind": "primitive_user_load", "primitive_loads": loads,
                          "pressure_regions": [], "provenance": PROV}]
    out["combinations"] = []
    out["components"] = []
    request["materials"] = []
    entry = {"id": cid, "family": ref["family"], "basis": basis, "request": request}
    if translations:
        entry["input_translations"] = translations
    return entry


def build():
    """The exact fixture bytes (compact DAG storage of the logical document)."""
    for name, digest in PINS.items():
        actual = hashlib.sha256((REFS / name).read_bytes()).hexdigest()
        if actual != digest:
            raise SystemExit(f"frozen reference {name} changed: {actual}")
    refs = json.loads((REFS / "references.json").read_text())
    template = json.loads(TEMPLATE.read_text())
    classification, cases = [], []
    for cid, ref in refs["cases"].items():
        reasons = classify(ref)
        classification.append({"id": cid, "family": ref["family"], "reasons": reasons})
        if not reasons:
            cases.append(make(cid, ref, template))
    document = {"artifact": "j7-rf-census-inputs", "schema_version": "1.0.0",
                "generator": "tools/fixtures/j7_rf_census_inputs.py", "reference_pins": PINS,
                "classification": classification, "cases": cases}
    full = (json.dumps(document, ensure_ascii=True, separators=(",", ":"), allow_nan=False) + "\n").encode()
    definition = _STORAGE.compact(full)
    return (json.dumps(definition, ensure_ascii=True, separators=(",", ":"), allow_nan=False) + "\n").encode()


def main():
    output = build()
    if "--check" in sys.argv[1:]:
        if not FIXTURE.is_file() or FIXTURE.read_bytes() != output:
            raise SystemExit(f"{FIXTURE.relative_to(PROJECT)} is not current; run {Path(__file__).name}")
        print("current", len(output), hashlib.sha256(output).hexdigest())
        return
    FIXTURE.write_bytes(output)
    print("wrote", FIXTURE.relative_to(PROJECT), len(output), hashlib.sha256(output).hexdigest())


if __name__ == "__main__":
    main()
