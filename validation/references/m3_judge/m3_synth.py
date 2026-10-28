"""Synthetic raw runner records for the M3 judge's self-test: genuine in format (the runner's record shape, the
contract's units and declared bases, the fixtures' frames and evidence members, bound ids), with the reference
rows themselves as values (rounded to binary64). No product code runs."""

from __future__ import annotations

import copy
from decimal import Decimal

import m3_compare as MC
import m3_view as MV

SYNTH_RUNNER = "5" * 64          # a synthetic runner's sha256
SYNTH_HEAD = "0" * 40            # a placeholder head for runs judged in memory; collection tests use the checkout's HEAD
SOLVER_BASIS = "DEC-053 sparse_default_promotion; solver_mode=sparse_interactive; solution_basis=synthetic"

BASE_SOURCE = {**{k: "solved_from_global_linear_system" for k in MV.NODAL + ("displacement_magnitude",)},
               **{k: "recovered_from_local_element_stiffness" for k in MV.LOCAL_ACTIONS + MV.LOCAL_STRESSES + MV.WALL},
               **{k: "recovered_from_assembled_support_law" for k in MV.SUPPORT},
               **{k: "recovered_from_open_mechanics_stress_components" for k in MV.WALL_STRESSES + (MV.MAXIMUM,)},
               "element_local_normal_stress_extreme": "section_extremes_from_published_stress_rows_v1",
               "element_local_transverse_shear_stress_maximum": "saint_venant_annulus_flexure_v1",
               "element_global_end_force_v1": "rotated_from_published_element_local_end_actions_v1",
               "element_global_end_moment_v1": "rotated_from_published_element_local_end_actions_v1",
               MV.SOLVER: SOLVER_BASIS}


def source(route, kind, basis):
    s = BASE_SOURCE[kind]
    return s + MV.SECTION_MARKER + basis if (route == "exact" and basis) else s


def row(route, case, kind, ent, loc, comp, basis, value, rid):
    md = None if kind == "displacement_magnitude" else {"basis": source(route, kind, basis), "component": comp,
                                                        "coordinate_system": MV.FRAME[kind], "location": loc, "sign_convention": "synthetic"}
    return {"kind": kind, "value": value, "unit": MV.contract_unit(route, kind, comp), "entity_ref": ent, "case": case, "location": loc,
            "component": comp, "id": rid, "metadata": md}


def solver_row(case="case:1", entity=MV.SOLVER_ENTITY, rid="result:solver-mode"):
    return {"kind": MV.SOLVER, "value": 1.0, "unit": "mode_code", "entity_ref": entity, "case": case, "location": case, "component": "linear_solver_mode",
            "id": rid, "metadata": {"basis": SOLVER_BASIS, "component": "linear_solver_mode", "coordinate_system": "reduced_system", "location": case,
                                    "sign_convention": "mode_code 1=sparse_interactive"}}


def diag(code, severity, message="synthetic", case="case:1", n=0):
    d = {"code": code, "id": f"diagnostic:{code.lower()}:{case}:{n}", "message": message, "severity": severity, "source": "synthetic"}
    if case is not None:
        d["affected_refs"] = [case]
    return d


def extremum(route, pid, rid, basis, val, upper_scale):
    e = {"approximation": "piecewise_quadratic_straight_section_statics", "certified_gap_pa": 0.0, "coefficient_basis": "synthetic",
         "enclosure_scope": "synthetic", "global_upper_bound_pa": val * upper_scale, "local_fraction": 0.5, "pipe_id": pid, "result_id": rid,
         "span_index": 0, "station_fraction": 0.5, "subdivisions": 1, "value_lower_pa": val / upper_scale, "value_upper_pa": val * upper_scale}
    if route == "exact":
        e["section_basis"] = basis
    return e


def headline(rows, basis=None):
    mx = [r for r in rows if r["kind"] == MV.MAXIMUM and MV._section_basis(r["metadata"]["basis"]) == basis]
    if not mx:
        return None
    top = max(mx, key=lambda r: r["value"])
    return {"location_ref": top["entity_ref"], "result_ref": top["id"], "unit": "Pa", "value": top["value"]}


def refresh_headlines(r, route):
    """Re-point the summary headlines at the current maximum rows (after a mutation of the rows)."""
    if route == "pf":
        r["summary"]["max_open_formula_stress"] = headline(r["results"])
    else:
        for b in MV.SECTION_BASES:
            r["summary"]["max_open_formula_stress_" + b] = headline(r["results"], b)
    return r


def synthetic_run(route, gen, entry="typed", mode="SparseInteractive", head=SYNTH_HEAD, runner=SYNTH_RUNNER, upper_scale=1.0 + 1e-12, solver=False):
    """A raw runner record in the runner's format, Passed in every load case, publishing the reference itself."""
    ref, model = gen["ref"], gen["request"]["model"]
    cases = [c["id"] for c in model["load_cases"]]
    rows, cases_ev = [], []
    for cid in cases:
        c = ref["cases"][cid]
        if solver:
            rows.append(solver_row(cid, rid=f"result:solver:{cid}"))
        for kind, ent, loc, comp, basis, val in c["rows"]:
            rows.append(row(route, cid, kind, ent, loc, comp, basis, float(Decimal(val)), f"result:{cid}:{len(rows)}"))
        ev = []
        for pid, basis, val in c["maxima"]:
            rid = f"result:{cid}:{len(rows)}:maximum"
            v = float(Decimal(val))
            rows.append(row(route, cid, MV.MAXIMUM, pid, "governing_station", "maximum_absolute_normal_stress", basis, v, rid))
            ev.append(extremum(route, pid, rid, basis, v, upper_scale))
        if route == "pf":
            cases_ev.append({"intensified_measures": [], "load_case_id": cid, "pipe_stress_extrema": ev,
                             "stress_maximum_coverage": {"complete": True, "outside_domain_pipe_ids": [], "unavailable_pipe_ids": []},
                             "support_attribution": {"attributed_support_ids": [s["id"] for s in model["supports"]], "withheld": []}})
        else:
            cases_ev.append({"load_case_id": cid, "pipe_stress_extrema": ev, "profile_mode": "exact_pressure_v3",
                             "stress_maximum_coverage": [{"complete": True, "section_basis": b, "unavailable_pipe_ids": []} for b in MV.SECTION_BASES]})
    if route == "pf":
        # sigma+- and tau_v,max as the design prescribes them: binary64 operations on the published operand rows
        val = {(r["case"], r["kind"], r["entity_ref"], r["location"]): r["value"] for r in rows}
        sections = ref["sections"]
        for r in rows:
            k = (r["case"], r["entity_ref"], r["location"])
            if r["kind"] == "element_local_normal_stress_extreme":
                sp, sm = MC.sigma_ops(*(val[(k[0], kind, k[1], k[2])] for kind in MV.LOCAL_STRESSES[:3]))
                r["value"] = sp if r["component"] == "sigma_plus" else sm
            elif r["kind"] == "element_local_transverse_shear_stress_maximum":
                s = sections[r["entity_ref"]]
                t = MV.TauInputs(E=float(s["E"]), G=float(s["G"]), od=float(s["od"]), wall=float(s["wall"]), mill=float(s["mill"]),
                                 I=MV._fq(s["I"]), Z=MV._fq(s["Z"]), tau_c=MV._fq(s["tau_c"]))
                r["value"] = MC.tau_ops(t, val[(k[0], "element_local_shear_force_y", k[1], k[2])], val[(k[0], "element_local_shear_force_z", k[1], k[2])])
    summary = {"component_stress_modifier_count": 0, "curved_bend_macro_element_count": 0, "load_case_count": len(cases),
               "node_count": len(model["nodes"]), "segment_count": len(model["pipe_segments"]), "spring_hanger_user_input_count": 0,
               "support_count": len(model["supports"]), "max_displacement": None, "max_open_formula_stress": None}
    diags = [diag(MC.PASSED, "info", case=cid) for cid in cases]
    diags += [diag(w, "warning", case=None, n=i) for i, w in enumerate(ref.get("expected_warnings", []))]
    rec = {"tag": gen["tag"], "entry": entry, "mode": mode, "ns": 1, "diagnostics": diags, "results": rows, "results_count": len(rows),
           "numerical_quality": {}, "summary": summary, "contract_evidence": None, "publication": "ordinary",
           "request_sha256": MV.request_sha256(gen["tag"], gen["request"]), "runner_sha256": runner, "product_head": head}
    if route == "pf":
        rec["contract_evidence"] = {"combination_gates": [], "preview_cases": cases_ev}
    else:
        rec["contract_evidence"] = {"connector": [], "exact_cases": cases_ev, "pressure": []}
    refresh_headlines(rec, route)
    if route == "pf" and ref.get("expect") == "refused":
        rec.update(diagnostics=[diag(ref["refusal"]["code"], "blocking")], results=[], results_count=0, summary=None, contract_evidence=None)
    return rec


def refusal(r, code, case="case:1"):
    out = copy.deepcopy(r)
    out.update(diagnostics=[diag(code, "blocking", case=case)], results=[], results_count=0, summary=None, contract_evidence=None)
    return out


def set_standing(r, code, severity=None, message="synthetic"):
    """Every load case's standing replaced by `code`."""
    sev = severity or ("info" if code == MC.PASSED else "warning")
    keep = [d for d in r["diagnostics"] if not d["code"].startswith("NUMERICAL_INTEGRITY")]
    cases = sorted({d["affected_refs"][0] for d in r["diagnostics"] if d["code"].startswith("NUMERICAL_INTEGRITY")}) or ["case:1"]
    r["diagnostics"] = [diag(code, sev, message, case=c) for c in cases] + keep
    return r


def extrema_of(r, route):
    key = "preview_cases" if route == "pf" else "exact_cases"
    return [e for c in r["contract_evidence"][key] for e in c["pipe_stress_extrema"]]
