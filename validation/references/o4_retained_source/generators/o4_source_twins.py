#!/usr/bin/env python3
"""O4 source-control twins: Timoshenko and nonzero-allowance re-authorings of J2's twelve source requests
and RF's PIN-OFF-345 r1e-08 and r1e-12 cases, with both exact readouts.

Standard library only. It reads no product code and no product output. Its inputs are input-only files,
each pinned by SHA-256 (the generator refuses to run if one changes):
- the twelve J2 request inputs, fixtures/product_preview/pressure_source/{physics_source,
  load_reference_source}/<name>.request.json ({materials, model} only; no *.raw.json is opened);
- fixtures/results/j7_rf_census_inputs.json, the RF census inputs (no expected values), decoded with the
  `expand` function of tools/fixtures/retained_precision_corpus.py (pinned; nothing else is used).
Only RF-SKEW-T-PIN-OFF-345-r1e-08 and -r1e-12 are taken from the census inputs.

Each request is re-authored in memory into this unit's request-agnostic document form, then solved by
the unit's own pipeline (generators/o4_reference.py: the unchanged kd5t_engine.py for the annular
readout, the prepared-leaf admitted readout, the same rows, discriminators, cross-checks, format and
110/150-digit precision recheck). A request or load case outside that solver's family is reported with
its exact reasons and is not frozen; nothing is approximated. Standard claim fence applies (F-PIP-2;
claims taxonomy per DEC-081).

Usage (from the unit directory):
  python3 -I -B generators/o4_source_twins.py            check: rebuild, compare byte for byte
  python3 -I -B generators/o4_source_twins.py --write    intentional re-freeze only
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
import sys
from fractions import Fraction

sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
import o4_reference as R  # noqa: E402  (this unit's generator; it pins the engine)

UNIT = R.UNIT
PROJECT = os.path.normpath(os.path.join(UNIT, "..", "..", ".."))

J2_DIR = "fixtures/product_preview/pressure_source"
J2 = (("physics_source", "n05"), ("physics_source", "n06"), ("physics_source", "n05_unicode"),
      ("physics_source", "n05_units"), ("physics_source", "fields"), ("physics_source", "mixed"),
      ("physics_source", "mixed_units"), ("load_reference_source", "n05"), ("load_reference_source", "n06"),
      ("load_reference_source", "eigen_motion"), ("load_reference_source", "fields"),
      ("load_reference_source", "mixed"))
RF_INPUTS = "fixtures/results/j7_rf_census_inputs.json"
RF_DECODER = "tools/fixtures/retained_precision_corpus.py"
RF_CASES = ("RF-SKEW-T-PIN-OFF-345-r1e-08", "RF-SKEW-T-PIN-OFF-345-r1e-12")

PINS = {
    "fixtures/product_preview/pressure_source/physics_source/n05.request.json":
        "90c1edab9cbf5c7caa2d2072cea6484be1681eabac8c8f48d053342dcc7f4efb",
    "fixtures/product_preview/pressure_source/physics_source/n06.request.json":
        "d9e3ede6542ce6d73832d6205d45bed185986295ea192357b2d884048cc3a217",
    "fixtures/product_preview/pressure_source/physics_source/n05_unicode.request.json":
        "49d235584b3dcad4ab887320e44e470c78379aca8459a4ff3a0c84a013511b3d",
    "fixtures/product_preview/pressure_source/physics_source/n05_units.request.json":
        "e5f0564cb7d977a509db9b5bae3f51d5351c23e1199c3b7d31aac9e78992f23b",
    "fixtures/product_preview/pressure_source/physics_source/fields.request.json":
        "b0b8ce054f06382dc15916b436a6a8bad704135884878aa047120eb0071d8852",
    "fixtures/product_preview/pressure_source/physics_source/mixed.request.json":
        "0da7828c35c78d5f3fa25b5336d448c0c72ee83b469b908376e2bb09194f3349",
    "fixtures/product_preview/pressure_source/physics_source/mixed_units.request.json":
        "f8dc9ffafcf0d3854498ac3456b30264524589432dbe29af90a0ba3dc7c523f5",
    "fixtures/product_preview/pressure_source/load_reference_source/n05.request.json":
        "2d0223c1125e5eaefc7f87f8d5f5f2688ce223952a090b4b42fa92c90252a699",
    "fixtures/product_preview/pressure_source/load_reference_source/n06.request.json":
        "2a8226694ec44a4a07fa1e89b3bc4ff2e77e75098071bb5a0a118e5c7c48a55d",
    "fixtures/product_preview/pressure_source/load_reference_source/eigen_motion.request.json":
        "79820f837ed1be649af20304adea15402180d2422b983de16a673e53759e0382",
    "fixtures/product_preview/pressure_source/load_reference_source/fields.request.json":
        "5479d2422d6700418cf019e3629c16974b85fba7789a8ddb8f20e0ed5939ff82",
    "fixtures/product_preview/pressure_source/load_reference_source/mixed.request.json":
        "9209e39bbd5f8976e59b8989915d4baf67934111537db23cacff170dbf6755db",
    "fixtures/results/j7_rf_census_inputs.json":
        "736fe3b449e6fac94d1eff6833857b5ef114348f8dcc5ef527ddec6b441a686c",
    "tools/fixtures/retained_precision_corpus.py":
        "e69d6945d1b1a5abc30c120851ab06d79b6e986f7bf0e07ab14e61849305a7c7",
}

THEORY_VARIANTS = (("zero", "eb"), ("zero", "timo"), ("mill", "eb"), ("ca", "eb"), ("both", "eb"),
                   ("mill", "timo"), ("ca", "timo"), ("both", "timo"))

# exact unit factors to SI (Fractions); a converted value must be exactly a binary64 number
UNIT_FACTORS = {"m": 1, "mm": Fraction(1, 1000), "N": 1, "kN": 1000, "N*m": 1, "kN*m": 1000, "Pa": 1,
                "kPa": 1000, "MPa": 10 ** 6, "GPa": 10 ** 9, "1": 1, "N/m": 1, "N*m/rad": 1}
KINDS = {"length": ("m", "mm"), "force": ("N", "kN"), "moment": ("N*m", "kN*m"),
         "modulus": ("Pa", "kPa", "MPa", "GPa"), "ratio": ("1",), "stiffness_t": ("N/m",),
         "stiffness_r": ("N*m/rad",)}
LOAD_DIRECTIONS = {"global_x": ("force", "x"), "global_y": ("force", "y"), "global_z": ("force", "z"),
                   "UX": ("force", "x"), "UY": ("force", "y"), "UZ": ("force", "z"),
                   "rotation_x": ("moment", "x"), "rotation_y": ("moment", "y"), "rotation_z": ("moment", "z"),
                   "RX": ("moment", "x"), "RY": ("moment", "y"), "RZ": ("moment", "z")}


def sha(path):
    with open(os.path.join(PROJECT, path), "rb") as fh:
        return hashlib.sha256(fh.read()).hexdigest()


def input_files():
    return [f"{J2_DIR}/{fam}/{name}.request.json" for fam, name in J2] + [RF_INPUTS, RF_DECODER]


def check_pins():
    if set(PINS) != set(input_files()):
        raise SystemExit("pin table does not match the input list")
    for path in input_files():
        got = sha(path)
        if PINS[path] != got:
            raise SystemExit(f"input changed: {path} (sha256 {got}, pinned {PINS[path]})")


def rf_requests():
    """Decode the census inputs with the pinned `expand` only, and take the two PIN-OFF-345 cases."""
    spec = importlib.util.spec_from_file_location("retained_precision_corpus", os.path.join(PROJECT, RF_DECODER))
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    expand = mod.expand
    with open(os.path.join(PROJECT, RF_INPUTS)) as fh:
        doc = expand(json.load(fh))
    out = {}
    for c in doc["cases"]:
        if c["id"] in RF_CASES:
            if c["id"] in out:
                raise AssertionError("duplicate RF census case")
            out[c["id"]] = c["request"]
    assert set(out) == set(RF_CASES)
    return out


# ------------------------------------------------------------------ re-authoring
class OutOfScope(Exception):
    pass


def si(qty, kind, reasons, where):
    if not isinstance(qty, dict) or "value" not in qty or "unit" not in qty:
        reasons.append(f"{where}: not a quantity")
        return None
    u = qty["unit"]
    if u not in KINDS[kind]:
        reasons.append(f"{where}: unit {u!r} outside this solver's exact conversions")
        return None
    v = Fraction(qty["value"]) * UNIT_FACTORS[u]
    f = float(v)
    if Fraction(f) != v:
        reasons.append(f"{where}: {qty['value']} {u} is not exactly a binary64 number in SI")
        return None
    return f


def reauthor(model, case_id):
    """The request-agnostic Euler-Bernoulli zero-allowance document of one load case, or the reasons it is
    outside the solver's family (straight members, rigid restraints and grounded springs, nodal force and
    moment loads, explicitly empty pressure regions, base E and nu)."""
    reasons = []
    if model.get("components"):
        reasons.append("components present")
    if model.get("combinations"):
        reasons.append("combinations present")
    pc = model.get("pressure_contract") or {}
    if pc.get("version") != "3.0.0" or pc.get("mode") != "exact_pressure_v3":
        reasons.append("not a 3.0.0/exact_pressure_v3 contract")
    if pc.get("shear_model", "euler_bernoulli") != "euler_bernoulli":
        reasons.append("source shear model is not euler_bernoulli")
    mats = model.get("materials", [])
    if len(mats) != 1:
        reasons.append("not one common material")
    mat = mats[0] if mats else {}
    if mat.get("constitutive_basis") != "homogeneous_isotropic_E_nu_v1":
        reasons.append("material basis is not homogeneous_isotropic_E_nu_v1")
    E = si(mat.get("elastic_modulus"), "modulus", reasons, "material elastic_modulus")
    nu = si(mat.get("poisson_ratio"), "ratio", reasons, "material poisson_ratio")
    if "shear_modulus" in mat and E is not None and nu is not None:
        G = si(mat["shear_modulus"], "modulus", reasons, "material shear_modulus")
        if G is not None and Fraction(G) != Fraction(E) / (2 * (1 + Fraction(nu))):
            reasons.append("authored shear_modulus differs from E/(2(1+nu)); both readouts derive G from E and nu")
    nodes = [{"id": n["id"], "xyz_m": [float(n["position"][a]) for a in "xyz"]} for n in model["nodes"]]
    members = []
    for s in model["pipe_segments"]:
        if s.get("material") != mat.get("id"):
            reasons.append(f"member {s['id']}: other material")
        sec = s["section"]
        od = si(sec.get("outside_diameter"), "length", reasons, f"member {s['id']} outside_diameter")
        wall = si(sec.get("wall_thickness"), "length", reasons, f"member {s['id']} wall_thickness")
        for key in ("mill_tolerance", "corrosion_allowance"):
            if key in sec:
                v = si(sec[key], "length", reasons, f"member {s['id']} {key}")
                if v not in (None, 0.0):
                    reasons.append(f"member {s['id']}: source {key} is not zero")
        extra = sorted(set(s) - {"id", "from", "to", "material", "section", "y_reference", "label", "provenance"})
        if extra:
            reasons.append(f"member {s['id']}: fields {extra}")
        y = s["y_reference"]
        members.append({"id": s["id"], "from": s["from"], "to": s["to"], "outside_diameter_m": od,
                        "wall_thickness_m": wall, "mill_tolerance_m": 0.0, "corrosion_allowance_m": 0.0,
                        "y_reference": [float(y[a]) for a in "xyz"]})
    supports = []
    for s in model["supports"]:
        fam = s.get("family")
        if fam == "anchor":
            supports.append({"id": s["id"], "node": s["node"], "kind": "rigid", "restraints": list(s["restraints"])})
        elif fam == "spring":
            st = s["stiffness"]
            dof = st["dof"]
            if s.get("restraints") != [dof]:
                reasons.append(f"support {s['id']}: spring restraints {s.get('restraints')} differ from its DOF")
            k = si(st["value"], "stiffness_t" if dof[0] == "U" else "stiffness_r", reasons, f"support {s['id']} stiffness")
            supports.append({"id": s["id"], "node": s["node"], "kind": "spring", "dof": dof, "stiffness": k,
                             "unit": "N/m" if dof[0] == "U" else "N*m/rad"})
        else:
            reasons.append(f"support {s['id']}: family {fam!r} (only rigid anchors and grounded linear springs)")
    cases = [lc for lc in model["load_cases"] if lc["id"] == case_id]
    assert len(cases) == 1
    lc = cases[0]
    if lc.get("pressure_regions"):
        reasons.append("present pressure region")
    for key in sorted(set(lc) - {"id", "kind", "label", "pressure_regions", "primitive_loads", "provenance",
                                 "status", "analysis_state"}):
        reasons.append(f"load case field {key!r} (" + ("named-point or temperature modulus basis" if key.startswith("modulus_basis")
                                                       else "outside the solver") + ")")
    factors = None
    state = lc.get("analysis_state")
    if state is not None:
        factors = {}
        for src in state.get("load_sources", []):
            factors[src["source_ref"]] = Fraction(src["factor"])
        for es in state.get("element_states", []):
            ms = es.get("material_selection", {})
            if ms.get("kind") != "explicit_base_properties":
                reasons.append(f"element state {es.get('pipe_ref')}: material selection {ms.get('kind')!r}")
            ts = es.get("thermal_state", {})
            if ts.get("kind") != "unchanged_reference":
                reasons.append(f"element state {es.get('pipe_ref')}: thermal state {ts.get('kind')!r} (an eigenstrain load)")
        for ss in state.get("support_states", []):
            if ss.get("boundary_motion"):
                reasons.append(f"support state {ss['support_ref']}: prescribed boundary motion "
                               f"{[bm['dof'] for bm in ss['boundary_motion']]}")
            if ss.get("participation", {}).get("kind") != "active_model_device":
                reasons.append(f"support state {ss['support_ref']}: participation {ss.get('participation')}")
        if state.get("history", {}).get("kind") != "independent_equilibrium":
            reasons.append(f"history {state.get('history')}")
        ref_id = state.get("reference_configuration_ref")
        for rc in model.get("reference_configurations", []):
            if rc["id"] != ref_id:
                continue
            if rc.get("geometry_ref", {}).get("kind") != "authored_model_geometry":
                reasons.append(f"reference configuration geometry {rc.get('geometry_ref')}")
            for mr in rc.get("member_references", []):
                if mr.get("basis", {}).get("kind") != "direct_strain_reference":
                    reasons.append(f"reference basis {mr.get('basis')}")
                if mr.get("fit", {}).get("kind") != "none":
                    reasons.append(f"reference configuration fit {mr['fit'].get('kind')!r} on {mr['pipe_ref']} (an eigenstrain load)")
    loads = []
    for pl in lc.get("primitive_loads", []):
        cat = pl.get("category")
        if cat not in ("concentrated_force", "concentrated_moment") or pl.get("target", {}).get("type") != "node":
            reasons.append(f"load {pl.get('id')}: category {cat!r} (nodal force and moment loads only)")
            continue
        kind, axis = LOAD_DIRECTIONS[pl["direction"]]
        if (kind == "force") != (cat == "concentrated_force"):
            reasons.append(f"load {pl['id']}: direction {pl['direction']} does not match {cat}")
            continue
        v = si(pl["magnitude"], kind, reasons, f"load {pl['id']}")
        if v is None:
            continue
        if factors is not None:
            if pl["id"] not in factors:
                continue
            p = Fraction(v) * factors[pl["id"]]
            if Fraction(float(p)) != p:
                reasons.append(f"load {pl['id']}: factored magnitude is not exactly a binary64 number")
                continue
            v = float(p)
        loads.append({"id": pl["id"], "node": pl["target"]["node"], "kind": kind, "direction": axis,
                      "value": v, "unit": "N" if kind == "force" else "N*m"})
    if reasons:
        raise OutOfScope(reasons)
    return {"shear_model": R.EB, "material": {"id": mat["id"], "elastic_modulus_Pa": E, "poisson_ratio": nu},
            "nodes": nodes, "members": members, "supports": supports, "loads": loads, "pressure_regions": []}


def twin(doc, variant, theory):
    out = json.loads(json.dumps(doc))
    out["shear_model"] = R.THEORIES[theory]
    for m in out["members"]:
        cm, ca = R.allowance(variant, m["wall_thickness_m"])
        m["mill_tolerance_m"], m["corrosion_allowance_m"] = cm, ca
    return out


def sources():
    """(source id, file, sha, model, load-case id) for every source load case, in a fixed order."""
    out = []
    for fam, name in J2:
        path = f"{J2_DIR}/{fam}/{name}.request.json"
        with open(os.path.join(PROJECT, path), encoding="utf-8") as fh:
            req = json.load(fh)
        assert set(req) <= {"materials", "model"} and not req.get("materials")
        for lc in req["model"]["load_cases"]:
            out.append((f"J2-{fam}-{name}", path, req["model"], lc["id"]))
    rf = rf_requests()
    for cid in RF_CASES:
        assert not rf[cid].get("materials")
        for lc in rf[cid]["model"]["load_cases"]:
            out.append((cid, RF_INPUTS, rf[cid]["model"], lc["id"]))
    return out


def build_cases():
    cases, scope = [], []
    for sid, path, model, case_id in sources():
        entry = {"source": sid, "file": path, "file_sha256": sha(path), "load_case": case_id}
        try:
            doc = reauthor(model, case_id)
        except OutOfScope as e:
            entry["status"] = "out_of_scope"
            entry["reasons"] = e.args[0]
            scope.append(entry)
            continue
        R.validate_document(doc)
        entry["status"] = "frozen"
        entry["twins"] = []
        for variant, th in THEORY_VARIANTS:
            cid = f"{sid}|{case_id}|{variant}|{th}"
            entry["twins"].append(cid)
            cases.append({"id": cid, "design": f"{sid}|{case_id}", "family": "j2_source_control" if sid.startswith("J2")
                          else "rf_pin_off_345", "variant": variant, "theory": th,
                          "role": "euler_bernoulli_zero_allowance_harness_twin" if (variant, th) == ("zero", "eb") else "twin",
                          "tags": [], "source_control_style": False, "theory_independent_actions": True,
                          "mirror_reactions": [], "spring_ratios": {}, "note": "",
                          "source": {"file": path, "file_sha256": entry["file_sha256"], "load_case": case_id},
                          "document": twin(doc, variant, th)})
        scope.append(entry)
    return cases, scope


def summary(cases, contents):
    out = {}
    for mutant in R.MUTANTS:
        rec = {"discriminable_in": {}, "not_discriminable_in": {}, "not_formed_in": 0, "smallest_listed_tolerances": {}}
        for c in cases:
            d = contents[c["id"]]["discriminators"][mutant]
            if "not_formed" in d:
                rec["not_formed_in"] += 1
                continue
            for readout in ("annular", "admitted"):
                if readout not in d:
                    continue
                r = d[readout]
                if r["rows_moved_1e3"]:
                    rec["discriminable_in"][readout] = rec["discriminable_in"].get(readout, 0) + 1
                    s = r["listed"][-1]["tolerances"]
                    cur = rec["smallest_listed_tolerances"].get(readout)
                    if cur is None or float(s) < float(cur):
                        rec["smallest_listed_tolerances"][readout] = s
                else:
                    rec["not_discriminable_in"].setdefault(readout, []).append(c["id"])
        rec["not_discriminable_count"] = {k: len(v) for k, v in rec["not_discriminable_in"].items()}
        out[mutant] = rec
    return out


def build(workers):
    check_pins()
    R.set_precision(R.WORK_PREC)
    cases, scope = build_cases()
    ids = [c["id"] for c in cases]
    assert len(ids) == len(set(ids))
    work = R.run_all(cases, R.WORK_PREC, workers, True)
    hi = R.run_all(cases, R.CHECK_PREC, workers, False)
    R.set_precision(R.WORK_PREC)
    contents = {cid: work[cid][1] for cid in ids}
    compared = 0
    for cid in ids:
        if json.dumps(work[cid][1]) != json.dumps(hi[cid][1]):
            raise AssertionError(f"precision recheck differs in {cid}")
        compared += sum(1 for _ in R.flatten(work[cid][1]["readouts"]))
    max110 = max(R.D(work[cid][2]["max_snapped"]) for cid in ids)
    max150 = max(R.D(hi[cid][2]["max_snapped"]) for cid in ids)
    assert max150 < R.RECHECK_SNAP
    frac = [work[cid][3] for cid in ids]
    twins = R.twin_checks(cases, contents)
    cases_doc = {"schema": "o4_retained_source_twins_cases_v1", "status": "Frozen; not yet refuted.",
                 "claim_fence": "Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).",
                 "conventions": "SI units; binary64 inputs taken exactly; request-agnostic re-authorings (see README)",
                 "inputs": {p: PINS[p] for p in input_files()}, "scope": scope, "cases": cases}
    cases_bytes = R.dump_lines(cases_doc, "cases", None)
    by_class = {}
    for c in cases:
        k = f"{c['variant']}|{c['theory']}"
        by_class[k] = by_class.get(k, 0) + 1
    oracle = {
        "schema": "o4_retained_source_twins_oracle_v1",
        "status": "Frozen; not yet refuted.",
        "claim_fence": "Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).",
        "cases_sha256": hashlib.sha256(cases_bytes).hexdigest(),
        "engine": {"path": "../kd5_timoshenko/kd5t_engine.py", "sha256": R.ENGINE_SHA256, "used": "unchanged, by import"},
        "conventions": "as oracle.json (README, Rows and Value format)",
        "census": {"source_load_cases": len(scope), "frozen": sum(1 for s in scope if s["status"] == "frozen"),
                   "out_of_scope": sum(1 for s in scope if s["status"] == "out_of_scope"),
                   "documents": len(cases), "by_allowance_and_shear_model": dict(sorted(by_class.items()))},
        "discriminator_summary": summary(cases, contents),
        "cross_checks": {
            "precision": {"working_digits": R.WORK_PREC, "recheck_digits": R.CHECK_PREC, "values_compared": compared,
                          "identical": True, "largest_snapped_at_working": "%.1e" % max110,
                          "largest_snapped_at_recheck": "%.1e" % max150},
            "rational_resolve": {"documents_eligible": sum(1 for f in frac if f["eligible"]),
                                 "readouts_resolved": sum(len(f.get("readouts", [])) for f in frac),
                                 "values_compared": sum(f.get("values_compared", 0) for f in frac),
                                 "binary64_compared": sum(f.get("binary64_compared", 0) for f in frac),
                                 "all_identical_at_40_digits": True},
            "twins": twins,
            "per_solve": "as oracle.json"},
        "cases": contents,
    }
    return cases_bytes, R.dump_lines(oracle, "cases", None)


FROZEN = ("source_twins_cases.json", "source_twins_oracle.json")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--workers", type=int, default=min(12, os.cpu_count() or 1))
    a = ap.parse_args()
    cases_bytes, oracle_bytes = build(a.workers)
    if a.write:
        for name, data in zip(FROZEN, (cases_bytes, oracle_bytes)):
            with open(os.path.join(UNIT, name), "wb") as fh:
                fh.write(data)
        with open(os.path.join(UNIT, "SHA256SUMS"), "wb") as fh:
            fh.write(R.sha256sums())
        print("written")
        return
    bad = []
    for name, data in zip(FROZEN, (cases_bytes, oracle_bytes)):
        with open(os.path.join(UNIT, name), "rb") as fh:
            if fh.read() != data:
                bad.append(name)
    if bad:
        raise SystemExit(f"check FAILED: {bad}")
    print("check passed: source twins reproduced byte for byte at 110 digits, identical at 150, every cross-check holds")


if __name__ == "__main__":
    main()
