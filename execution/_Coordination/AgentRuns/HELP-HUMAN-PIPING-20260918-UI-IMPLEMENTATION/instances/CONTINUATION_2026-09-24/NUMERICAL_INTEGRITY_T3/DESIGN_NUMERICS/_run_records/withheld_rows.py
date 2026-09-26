#!/usr/bin/env python3
"""D1 DESIGN rev 4: rows a W1-selected case would withhold, estimated on committed fixtures.

Usage: python3 withheld_rows.py <root> [<root> ...]    (standard library only)

For every committed raw envelope whose `source_block_recovery` selects a load case (the cases W1
would select after F2, since they are Sensitive on main), it classifies each published row of that
case with DESIGN revision 4's closed kind table and floor (R = 2^-34, S* from published rows per
section 4.1.6.1, propagation factors k per stress kind), using the committed selected values as a
stand-in for W1's values. Section terms come from the matching `.request.json` (OD, wall, E, G).
Reported per case: rows, relative_verified, absolute_verified (withheld), not_covered (withheld),
input_derived, non_quantity. Bodies: one per case (every committed selected model here is one
connected body; checked). Not a product run.
"""
import json
import math
import os
import sys
from collections import Counter, defaultdict

R = 2.0 ** -34
SQRT2 = math.sqrt(2.0)
UNIT = {"m": 1.0, "mm": 1e-3, "rad": 1.0, "N": 1.0, "kN": 1e3, "N*m": 1.0, "kN*m": 1e3, "Pa": 1.0, "MPa": 1e6}
KIND = {
    "global_nodal_displacement_x": "translation", "global_nodal_displacement_y": "translation",
    "global_nodal_displacement_z": "translation", "displacement_magnitude": "translation",
    "global_nodal_rotation_x": "rotation", "global_nodal_rotation_y": "rotation", "global_nodal_rotation_z": "rotation",
    "element_local_axial_force": "force", "element_local_shear_force_y": "force", "element_local_shear_force_z": "force",
    "pipe_wall_axial_force_v2": "force", "pipe_effective_axial_force_v2": "force",
    "support_reaction_force_magnitude_v2": "force", "reaction_resultant": "force",
    "element_local_torsional_moment": "moment", "element_local_bending_moment_y": "moment",
    "element_local_bending_moment_z": "moment", "support_reaction_moment_magnitude_v2": "moment",
    "support_reaction_component_v2": "by_unit", "pipe_wall_endpoint_action_v2": "by_unit",
    "element_local_axial_normal_stress": ("stress", 1.0), "element_local_bending_normal_stress_y": ("stress", 1.0),
    "element_local_bending_normal_stress_z": ("stress", 1.0), "element_local_torsional_shear_stress": ("stress", 1.0),
    "pipe_axial_membrane_stress_v2": ("stress", 1.0),
    "pipe_elastic_normal_stress_maximum_v2": ("stress", SQRT2),
    "component_equal_factor_intensified_bending_stress_v1": ("stress", "sqrt2_i"),
    "open_formula_stress_summary": ("stress", 2.0),
}
INPUT_DERIVED = {"pipe_lame_hoop_stress_v2", "pipe_lame_radial_stress_v2", "pipe_section_pressure_hoop_stress",
                 "pipe_section_pressure_longitudinal_stress", "constant_effort_support_applied_load",
                 "expansion_joint_pressure_thrust_load_review", "component_user_stress_multiplier_review",
                 "component_user_stiffness_macro_element_review", "constant_effort_user_input_review",
                 "spring_hanger_user_input_review"}
NON_QUANTITY = {"sparse_live_path_dense_parity_relative_delta", "linear_solver_mode_basis", "modulus_basis_record",
                "combination_modulus_basis_record"}
RESTRAINT = {"global_nodal_displacement_x": "UX", "global_nodal_displacement_y": "UY", "global_nodal_displacement_z": "UZ",
             "global_nodal_rotation_x": "RX", "global_nodal_rotation_y": "RY", "global_nodal_rotation_z": "RZ"}


def request_for(path):
    d, f = os.path.split(path)
    base = f.replace(".raw.json", "")
    for cand in (base + ".request.json", base.rsplit("-", 1)[0] + ".request.json"):
        p = os.path.join(d, cand)
        if os.path.exists(p):
            return json.load(open(p))
    return None


def classify_case(rows, model):
    m = model.get("model", model)
    nodes = {n["id"]: (n["position"]["x"], n["position"]["y"], n["position"]["z"]) for n in m["nodes"]}
    ext = [max(p[a] for p in nodes.values()) - min(p[a] for p in nodes.values()) for a in range(3)]
    Lb = math.sqrt(ext[0] ** 2 + ext[1] ** 2 + ext[2] ** 2)
    mats = {x["id"]: x for x in m["materials"]}
    sec = {}
    for p in m["pipe_segments"]:
        od = p["section"]["outside_diameter"]["value"]
        t = p["section"]["wall_thickness"]["value"]
        idd = od - 2 * t
        A = math.pi * (od * od - idd * idd) / 4.0
        I = math.pi * (od ** 4 - idd ** 4) / 64.0
        sec[p["id"]] = (A, I / (od / 2.0))
    restrained = defaultdict(set)
    for s in m["supports"]:
        if s.get("family") != "spring":
            for r in s.get("restraints", []):
                restrained[s["node"]].add(r)
    S = defaultdict(float)
    parsed = []
    for r in rows:
        k = r["kind"]
        cls = KIND.get(k)
        v = abs(r["value"]) * UNIT.get(r["unit"], float("nan")) if isinstance(r["value"], (int, float)) else None
        if cls == "by_unit":
            cls = "force" if r["unit"] in ("N", "kN") else "moment"
        parsed.append((r, cls, v))
        if isinstance(cls, str) and v is not None:
            S[cls] = max(S[cls], v)
    tr = max(S["translation"], Lb * S["rotation"])
    fo = max(S["force"], S["moment"] / Lb if Lb else 0.0)
    mo = max(S["moment"], Lb * S["force"])
    ro = max(S["rotation"], S["translation"] / Lb if Lb else 0.0)
    star = {"translation": tr, "rotation": ro, "force": fo, "moment": mo}
    count = Counter()
    for r, cls, v in parsed:
        k = r["kind"]
        if k in NON_QUANTITY:
            count["non_quantity"] += 1
            continue
        if k in INPUT_DERIVED:
            count["input_derived"] += 1
            continue
        if k in RESTRAINT and RESTRAINT[k] in restrained.get(r["entity_ref"], set()):
            count["input_derived_prescribed"] += 1
            continue
        if cls is None or v is None or v != v:
            count["not_covered"] += 1
            continue
        if isinstance(cls, tuple):
            A, Z = sec.get(r["entity_ref"], (None, None))
            if A is None:
                count["not_covered"] += 1
                continue
            kf = cls[1]
            if kf == "sqrt2_i":
                count["not_covered"] += 1  # no intensified rows in these fixtures
                continue
            s = fo / A + kf * (mo / Z)
        else:
            s = star[cls]
        if v < R * s:
            count["absolute_verified"] += 1
            if v == 0.0:
                count["absolute_verified_exact_zero"] += 1
        else:
            count["relative_verified"] += 1
    count["S_star"] = {k: float("%.4g" % v) for k, v in star.items()}
    return count


def main():
    out = {}
    for root in sys.argv[1:]:
        for dp, dn, fn in os.walk(root):
            for f in sorted(fn):
                if not f.endswith(".raw.json") and not f.endswith(".analysis_run.json"):
                    continue
                p = os.path.join(dp, f)
                try:
                    env = json.load(open(p))
                except Exception:
                    continue
                sbr = env.get("source_block_recovery")
                if not isinstance(sbr, dict):
                    continue
                model = request_for(p)
                if model is None:
                    continue
                selected = {c["basis_ref"]["ref_id"] for c in sbr.get("body", {}).get("cases", [])
                            if c.get("status") in (None, "selected", "qualified") and c.get("basis_ref")}
                for case in sorted(selected):
                    rows = [r for r in env.get("results", []) if (r.get("basis_ref") or {}).get("ref_id") == case]
                    if not rows:
                        continue
                    c = classify_case(rows, model)
                    c["rows"] = len(rows)
                    c["withheld"] = c.get("absolute_verified", 0) + c.get("not_covered", 0)
                    out[f"{os.path.relpath(p, root)} :: {case}"] = dict(c)
    print(json.dumps(out, indent=1, sort_keys=True))


if __name__ == "__main__":
    main()
