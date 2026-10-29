"""T4-U4b preview model checks (DEL-04-07 Design u4b-u5b-implementation.md §5.1), test-only: τ bit for bit with c_b
from the captured model's moduli and section for every member (the Poisson warning bound to the member's ν), and the
directional rows and records bound to the model's arcs with the pair. Kept apart
from ``u4b.py`` so that module (and the pressure-2 verifier that imports it) stays standard library only.
"""
from __future__ import annotations

import math
from typing import Any, Mapping

from .u4b import SHEAR_MAX, Rows, _case, _require, bits, directional_model_checks, poisson_exempt, preview_shear_factor, shear_maximum
from .units import to_canonical


def preview_model_checks(source: Mapping[str, Any], model: Mapping[str, Any]) -> None:
    """Every member of every case (review items 3 and 4): c_b from the moduli the solve uses (named point or base E
    and G) by ``preview_shear_factor``, admission by ν = RN(RN(E/RN(2G)) - 1). In the domain no Poisson warning names the member and each τ is bit for bit; outside it the
    warning names it. Interpolated moduli (modulus_basis_temperature) keep the model-free checks; no preview case
    carries analysis_state (the producer refuses it without the exact pressure contract)."""
    def _normalized(quantity: Mapping[str, Any], dimension: str) -> float:
        return to_canonical(quantity["value"], quantity["unit"], dimension)

    results = source.get("results", [])
    diagnostics = source.get("diagnostics", [])
    rows = Rows(results)
    materials = {m.get("id"): m for m in model.get("materials", [])}
    pipes = {p.get("id"): p for p in model.get("pipe_segments", [])}
    for preview_case in (source.get("contract_evidence") or {}).get("preview_cases") or []:
        cid = preview_case.get("load_case_id")
        case = next((c for c in model.get("load_cases", []) if c.get("id") == cid), None)
        if case is None:
            continue
        members = sorted({row["entity_ref"] for row in results if _case(row) == cid and row.get("kind") == "element_local_axial_force"})
        exempt = poisson_exempt(diagnostics, cid)
        _require(exempt <= set(members), "Poisson warning names a member outside the case")
        if case.get("modulus_basis_temperature") is not None:
            continue
        # No preview case carries analysis_state: the producer refuses it without the exact pressure contract.
        for pid in members:
            pipe = pipes.get(pid)
            if pipe is None:
                continue
            material = materials.get(pipe.get("material"))
            _require(material is not None, "member material")
            if case.get("modulus_basis_ref") is not None:
                point = next((p for p in material.get("temperature_points", []) if p.get("id") == case["modulus_basis_ref"]), None)
                _require(point is not None, "named point")
                e, g = _normalized(point["elastic_modulus"], "stress"), _normalized(point["shear_modulus"], "stress")
            else:
                e, g = _normalized(material["elastic_modulus"], "stress"), _normalized(material["shear_modulus"], "stress")
            section = pipe["section"]
            od = _normalized(section["outside_diameter"], "length")
            wall = _normalized(section["wall_thickness"], "length")
            t = wall if section.get("mill_tolerance") is None else wall - _normalized(section["mill_tolerance"], "length")
            inner = od - 2.0 * t
            inertia = math.pi * ((od * od) * (od * od) - (inner * inner) * (inner * inner)) / 64.0
            factor = preview_shear_factor(e, g, od / 2.0, inner / 2.0, inertia)
            _require((factor is None) == (pid in exempt), "Poisson warning does not match the member's nu")
            if factor is None:
                continue
            for row in results:
                if row.get("kind") == SHEAR_MAX and _case(row) == cid and row.get("entity_ref") == pid:
                    location = row["metadata"]["location"]
                    vy = rows.any(cid, pid, "element_local_shear_force_y", location)
                    vz = rows.any(cid, pid, "element_local_shear_force_z", location)
                    _require(vy is not None and vz is not None and bits(row["value"]) == bits(shear_maximum(vy, vz, factor)), "model-bound transverse shear value")
    # Review item 1: the directional rows and records bound to the model.
    directional_model_checks(source, model, False)
