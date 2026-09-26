#!/usr/bin/env python3
"""D1 DESIGN revision 5 (D-15, option B): the mechanical exact-zero proof, and what it covers on the
committed exact-block-selected cases (standard library only; no product code built or run).

Usage: python3 b_proof.py <root> [<root> ...]

The proof (DESIGN revision 5 s8.1.2) uses only integer pattern data and exact rational tests:
  1. Frame pattern. For each straight member, from the binary64 end coordinates and y reference,
     the exact rational vectors d, y_c = y - ((y.d)/(d.d)) d and d x y_c give the zero pattern of
     the three local axes (normalisation does not change which entries are zero).
  2. Element pattern. Local stiffness couples only within four families: axial (local x
     translations), torsion (local x rotations), bending in the local x-y plane (y translations,
     z rotations) and in the x-z plane (z translations, y rotations). Each family's global DOF set is
     the union, over both ends, of the global components its local axes touch. The element couples
     every pair inside one family set (Boolean pattern of T^T K T, a superset of its nonzeros for
     every stiffness value).
  3. Seeds. Free DOFs that carry a load term; every free DOF of a member that carries any member
     load; free DOFs coupled to a nonzero prescribed DOF. Any load the script cannot place seeds
     every DOF (no proof for that case).
  4. Reach. Breadth-first search over the free-DOF coupling graph. For a positive definite K_ff the
     unreached block has a zero right-hand side and no coupling to the reached block, so its
     displacements are exactly zero for every stiffness value with this pattern.
  5. Rows. A row is proven zero when every DOF its formula reads is proven zero and no load term
     enters it: nodal rows by DOF; member end and station actions by family (no member load on the
     member); stresses by the families they read (no pressure on the model); support reaction
     components from the restrained DOF's element couplings (no load at that DOF), spring reactions
     from the spring DOF, unrestrained components structurally zero; magnitudes from all components.
A withheld row (absolute_verified under revision 5's table and floor) is then counted as
  proven_zero      published value +-0.0 and proven zero: exempt under B (bindable as exact 0);
  zero_unproven    published 0.0 but no proof (symmetry, cancellation or unplaced load): needs C;
  nonzero          published nonzero: needs C (a user may rely on it).
"""
import json
import math
import os
import sys
from collections import defaultdict
from fractions import Fraction as Fr

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import withheld_rows as W  # noqa: E402  (D1's own revision-4 classifier: tables and request lookup)

R = 2.0 ** -34
K_OVERRIDE = {"pipe_elastic_normal_stress_maximum_v2": 2.0 * math.sqrt(2.0), "open_formula_stress_summary": 4.0}
COMP = {"UX": 0, "UY": 1, "UZ": 2, "RX": 3, "RY": 4, "RZ": 5,
        "global_x": 0, "global_y": 1, "global_z": 2, "rotation_x": 3, "rotation_y": 4, "rotation_z": 5}
REACT = {"Fx": 0, "Fy": 1, "Fz": 2, "Mx": 3, "My": 4, "Mz": 5}
FAM = {"axial": [(0, 0)], "torsion": [(1, 0)], "xy": [(0, 1), (1, 2)], "xz": [(0, 2), (1, 1)]}
ROW_FAM = {
    "element_local_axial_force": ["axial"], "pipe_wall_axial_force_v2": ["axial"], "pipe_effective_axial_force_v2": ["axial"],
    "element_local_shear_force_y": ["xy"], "element_local_bending_moment_z": ["xy"],
    "element_local_shear_force_z": ["xz"], "element_local_bending_moment_y": ["xz"],
    "element_local_torsional_moment": ["torsion"], "element_local_torsional_shear_stress": ["torsion"],
    "element_local_axial_normal_stress": ["axial"], "pipe_axial_membrane_stress_v2": ["axial"],
    "element_local_bending_normal_stress_z": ["xy"], "element_local_bending_normal_stress_y": ["xz"],
    "pipe_elastic_normal_stress_maximum_v2": ["axial", "xy", "xz"], "open_formula_stress_summary": ["axial", "xy", "xz"],
    "component_equal_factor_intensified_bending_stress_v1": ["xy", "xz"],
    "pipe_wall_endpoint_action_v2": ["axial", "torsion", "xy", "xz"],
}


def axes_pattern(p, q, yref):
    d = [Fr(q[k]) - Fr(p[k]) for k in range(3)]
    y = [Fr(v) for v in yref]
    dd = sum(c * c for c in d)
    yd = sum(a * b for a, b in zip(y, d))
    yc = [y[k] - yd / dd * d[k] for k in range(3)]
    z = [d[1] * yc[2] - d[2] * yc[1], d[2] * yc[0] - d[0] * yc[2], d[0] * yc[1] - d[1] * yc[0]]
    return [[c != 0 for c in v] for v in (d, yc, z)]


def analyse(rows, req):
    m = req.get("model", req)
    nodes = {n["id"]: (n["position"]["x"], n["position"]["y"], n["position"]["z"]) for n in m["nodes"]}
    idx = {nid: i for i, nid in enumerate(nodes)}
    ndof = 6 * len(nodes)
    members = {}
    for s in m["pipe_segments"]:
        yr = s.get("y_reference") or {"x": 0, "y": 1, "z": 0}
        pat = axes_pattern(nodes[s["from"]], nodes[s["to"]], (yr["x"], yr["y"], yr["z"]))
        fams = {}
        for f, locs in FAM.items():
            g = set()
            for end in (s["from"], s["to"]):
                for (block, axis) in locs:
                    for comp in range(3):
                        if pat[axis][comp]:
                            g.add(6 * idx[end] + 3 * block + comp)
            fams[f] = g
        members[s["id"]] = fams
    restrained, springs, supports = set(), {}, {}
    for s in m["supports"]:
        n = idx[s["node"]]
        if s.get("family") == "spring":
            st = s.get("stiffness") or {}
            springs[s["id"]] = 6 * n + COMP[st.get("dof", s["restraints"][0])]
            supports[s["id"]] = ("spring", n, {springs[s["id"]]})
        else:
            dofs = {6 * n + COMP[r] for r in s.get("restraints", [])}
            restrained |= dofs
            supports[s["id"]] = ("rigid", n, dofs)
        if s.get("prescribed_displacements") or s.get("imposed_displacements"):
            return None, "prescribed motion present (not placed by this script)"
    adj = defaultdict(set)
    for fams in members.values():
        for g in fams.values():
            for a in g:
                adj[a] |= g
    seeds, loaded_members, loaded_dofs = set(), set(), set()
    pressure = False  # set below from this case's own pressure regions
    case_id = rows[0]["basis_ref"]["ref_id"]
    lcs = [lc for lc in m.get("load_cases", []) if lc["id"] == case_id]
    if not lcs:
        return None, "case not found as a primitive load case (combination or load state)"
    unplaced = []
    if m.get("components"):
        return None, "components present (joints, user stiffness or curved elements are outside the pattern)"
    extra = set(lcs[0]) - {"id", "kind", "label", "status", "provenance", "primitive_loads", "pressure_regions", "analysis_state",
                              "modulus_basis_ref", "modulus_basis_temperature"}
    if extra:
        return None, "load-case keys not placed: " + ",".join(sorted(extra))
    for pl in lcs[0].get("primitive_loads", []):
        cat, tgt = pl.get("category"), pl.get("target") or {}
        if cat in ("concentrated_moment", "concentrated_force") and tgt.get("type") == "node" and pl.get("direction") in COMP:
            loaded_dofs.add(6 * idx[tgt["node"]] + COMP[pl["direction"]])
        elif tgt.get("type") in ("pipe", "pipe_segment", "member", "element") and (tgt.get("pipe") or tgt.get("id")) in members:
            loaded_members.add(tgt.get("pipe") or tgt.get("id"))
        else:
            unplaced.append(cat)
    for region in lcs[0].get("pressure_regions") or []:
        pressure = True
        for mid in region.get("member_pipe_ids", []):
            if mid not in members:
                return None, "pressure region on an unknown member"
            loaded_members.add(mid)
    # 0.4.0 analysis state (T1): member eigen strains and support boundary motion are load terms too.
    moved, axial_members = set(), set()
    state = lcs[0].get("analysis_state")
    if state is not None:
        known = {"contract", "reference_configuration_ref", "element_states", "support_states", "provenance",
                 "load_sources", "history"}
        if (state.get("history") or {}).get("kind", "independent_equilibrium") != "independent_equilibrium":
            return None, "analysis_state history is not independent_equilibrium"
        if any(ls.get("source_ref") not in {pl.get("id") for pl in lcs[0].get("primitive_loads", [])}
               for ls in state.get("load_sources", [])):
            return None, "analysis_state load source not among the case's primitive loads"
        if set(state) - known:
            return None, "analysis_state keys not placed: " + ",".join(sorted(set(state) - known))
        for es in state.get("element_states", []):
            if set(es) - {"pipe_ref", "material_selection", "thermal_state"}:
                unplaced.append("element_state:" + ",".join(sorted(set(es) - {"pipe_ref", "material_selection", "thermal_state"})))
            ts = es.get("thermal_state") or {}
            if not ts or ts.get("kind") == "unchanged_reference":
                pass
            elif ts.get("kind") != "explicit_interval_strain":
                loaded_members.add(es["pipe_ref"])
            elif ts and (ts.get("strain") or {}).get("value", 1) != 0:
                axial_members.add(es["pipe_ref"])  # uniform axial eigen strain: axial family only
        for ss in state.get("support_states", []):
            sup = next((s for s in m["supports"] if s["id"] == ss["support_ref"]), None)
            for bm in ss.get("boundary_motion", []):
                if (bm.get("value") or {}).get("value", 1) != 0 and sup is not None:
                    moved.add(6 * idx[sup["node"]] + COMP[bm["dof"]])
        ref = next((rc for rc in m.get("reference_configurations", []) if rc["id"] == state.get("reference_configuration_ref")), None)
        if state.get("reference_configuration_ref") and ref is None:
            unplaced.append("reference_configuration")
        for mr in (ref or {}).get("member_references", []):
            fit = mr.get("fit") or {}
            if fit.get("kind") == "none":
                pass
            elif fit.get("kind") != "fit_strain":
                loaded_members.add(mr["pipe_ref"])
            elif (fit.get("strain") or {}).get("value", 1) != 0:
                axial_members.add(mr["pipe_ref"])  # uniform fit strain: axial family only
    if unplaced:
        return None, "unplaced load categories: " + ",".join(sorted(set(map(str, unplaced))))
    for d in loaded_dofs:
        if d not in restrained:
            seeds.add(d)
    for mid in loaded_members:
        for g in members[mid].values():
            seeds |= {d for d in g if d not in restrained}
    for mid in axial_members:
        seeds |= {d for d in members[mid]["axial"] if d not in restrained}
    for d in moved:
        seeds |= {b for b in adj[d] if b not in restrained}
    reach, stack = set(seeds), list(seeds)
    while stack:
        a = stack.pop()
        for b in adj[a]:
            if b not in restrained and b not in reach:
                reach.add(b)
                stack.append(b)
    zero = {d for d in range(ndof) if d not in reach and d not in moved}  # unreached free DOFs, zero-prescribed restrained DOFs

    def member_zero(mid, fams):
        if mid in loaded_members or (mid in axial_members and "axial" in fams):
            return False
        return all(members[mid][f] <= zero for f in fams)

    def reaction_zero(sid, comp):
        kind, n, dofs = supports[sid]
        d = 6 * n + comp
        if d not in dofs:
            return True  # no restraint or spring on this component: structurally zero
        if kind == "spring":
            return d in zero
        return d not in loaded_dofs and adj[d] <= zero and d in zero

    proven = {}
    for r in rows:
        k, e, md = r["kind"], r["entity_ref"], r.get("metadata") or {}
        ok = False
        if k in W.RESTRAINT and e in idx:
            ok = 6 * idx[e] + COMP[W.RESTRAINT[k]] in zero
        elif k == "displacement_magnitude" and e in idx:
            ok = all(6 * idx[e] + c in zero for c in range(3))
        elif k in ROW_FAM and e in members:
            ok = member_zero(e, ROW_FAM[k]) and not (pressure and k in (
                "pipe_wall_axial_force_v2", "pipe_effective_axial_force_v2", "pipe_axial_membrane_stress_v2",
                "pipe_elastic_normal_stress_maximum_v2", "open_formula_stress_summary"))
        elif k == "support_reaction_component_v2" and e in supports and md.get("component") in REACT:
            ok = reaction_zero(e, REACT[md["component"]])
        elif k == "support_reaction_force_magnitude_v2" and e in supports:
            ok = all(reaction_zero(e, c) for c in range(3))
        elif k == "support_reaction_moment_magnitude_v2" and e in supports:
            ok = all(reaction_zero(e, c) for c in range(3, 6))
        elif k == "reaction_resultant":
            sids = [e] if e in supports else list(supports)
            ok = all(reaction_zero(s, c) for s in sids for c in range(3))
        proven[r["id"]] = ok
    return proven, None


def classify(rows, req):
    """Per-row revision-5 class (withheld_rows' table and floor, with R4-1's span-statics k)."""
    m = req.get("model", req)
    nodes = {n["id"]: (n["position"]["x"], n["position"]["y"], n["position"]["z"]) for n in m["nodes"]}
    ext = [max(p[a] for p in nodes.values()) - min(p[a] for p in nodes.values()) for a in range(3)]
    Lb = math.sqrt(ext[0] ** 2 + ext[1] ** 2 + ext[2] ** 2)
    sec = {}
    for p in m["pipe_segments"]:
        od = p["section"]["outside_diameter"]["value"]
        t = p["section"]["wall_thickness"]["value"]
        idd = od - 2 * t
        sec[p["id"]] = (math.pi * (od * od - idd * idd) / 4.0, math.pi * (od ** 4 - idd ** 4) / 64.0 / (od / 2.0))
    restrained = defaultdict(set)
    for s in m["supports"]:
        if s.get("family") != "spring":
            for rr in s.get("restraints", []):
                restrained[s["node"]].add(rr)
    S = defaultdict(float)
    parsed = []
    for r in rows:
        cls = W.KIND.get(r["kind"])
        v = abs(r["value"]) * W.UNIT.get(r["unit"], float("nan")) if isinstance(r["value"], (int, float)) else None
        if cls == "by_unit":
            cls = "force" if r["unit"] in ("N", "kN") else "moment"
        parsed.append((r, cls, v))
        if isinstance(cls, str) and v is not None:
            S[cls] = max(S[cls], v)
    star = {"translation": max(S["translation"], Lb * S["rotation"]), "rotation": max(S["rotation"], S["translation"] / Lb if Lb else 0.0),
            "force": max(S["force"], S["moment"] / Lb if Lb else 0.0), "moment": max(S["moment"], Lb * S["force"])}
    out = {}
    for r, cls, v in parsed:
        k = r["kind"]
        if k in W.NON_QUANTITY:
            out[r["id"]] = "non_quantity"
        elif k in W.INPUT_DERIVED or (k in W.RESTRAINT and W.RESTRAINT[k] in restrained.get(r["entity_ref"], set())):
            out[r["id"]] = "input_derived"
        elif cls is None or v is None or v != v:
            out[r["id"]] = "not_covered"
        else:
            if isinstance(cls, tuple):
                A, Z = sec.get(r["entity_ref"], (None, None))
                kf = K_OVERRIDE.get(k, cls[1])
                if A is None or kf == "sqrt2_i":
                    out[r["id"]] = "not_covered"
                    continue
                s = star["force"] / A + kf * (star["moment"] / Z)
            else:
                s = star[cls]
            out[r["id"]] = "absolute_verified" if v < R * s else "relative_verified"
    return out


def main():
    out = {}
    for root in sys.argv[1:]:
        for dp, dn, fn in os.walk(root):
            for f in sorted(fn):
                if not f.endswith(".raw.json"):
                    continue
                p = os.path.join(dp, f)
                try:
                    env = json.load(open(p))
                except Exception:
                    continue
                sbr = env.get("source_block_recovery")
                if not isinstance(sbr, dict):
                    continue
                req = W.request_for(p)
                if req is None:
                    continue
                selected = {c["basis_ref"]["ref_id"] for c in sbr.get("body", {}).get("cases", [])
                            if c.get("status") in (None, "selected", "qualified") and c.get("basis_ref")}
                for case in sorted(selected):
                    rows = [r for r in env.get("results", []) if (r.get("basis_ref") or {}).get("ref_id") == case]
                    if not rows:
                        continue
                    cls = classify(rows, req)
                    proven, why = analyse(rows, req)
                    rec = {"rows": len(rows), "withheld": 0, "proven_zero": 0, "zero_unproven": 0, "nonzero": 0}
                    if why:
                        rec["no_proof"] = why
                    nz = []
                    for r in rows:
                        if cls[r["id"]] not in ("absolute_verified", "not_covered"):
                            continue
                        rec["withheld"] += 1
                        if r["value"] != 0:
                            rec["nonzero"] += 1
                            nz.append([r["id"], r["value"], r["unit"]])
                        elif proven and proven.get(r["id"]):
                            rec["proven_zero"] += 1
                        else:
                            rec["zero_unproven"] += 1
                    rec["nonzero_rows"] = nz
                    rec["withheld_after_B"] = rec["withheld"] - rec["proven_zero"]
                    out[f"{os.path.relpath(p, root)} :: {case}"] = rec
    print(json.dumps(out, indent=1, sort_keys=True))


if __name__ == "__main__":
    main()
