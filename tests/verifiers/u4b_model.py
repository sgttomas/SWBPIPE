"""T4-U4b preview model checks (DEL-04-07 Design u4b-u5b-implementation.md §5.1), test-only: τ bit for bit with c_b
from the captured model's moduli and section, and directional rows only on the model's arcs with the pair. Kept apart
from ``u4b.py`` so that module (and the pressure-2 verifier that imports it) stays standard library only.
"""
from __future__ import annotations

import math
from typing import Any, Mapping

from .u4b import SHEAR_MAX, Rows, _case, _require, bits, directional_model_checks, shear_factor, shear_maximum
from .units import to_canonical


def preview_model_checks(source: Mapping[str, Any], model: Mapping[str, Any]) -> None:
    """τ bit for bit with c_b from the captured model (named point or base E and G; interpolated or load-state cases
    keep the model-free checks), and directional rows only on the model's arcs with the pair."""
    def _normalized(quantity: Mapping[str, Any], dimension: str) -> float:
        return to_canonical(quantity["value"], quantity["unit"], dimension)
    results = source.get("results", [])
    rows = Rows(results)
    for row in results:
        if row.get("kind") != SHEAR_MAX or _case(row) is None:
            continue
        cid = _case(row)
        case = next((c for c in model.get("load_cases", []) if c.get("id") == cid), None)
        if case is None or case.get("modulus_basis_temperature") is not None or case.get("analysis_state") is not None:
            continue
        pipe = next((p for p in model.get("pipe_segments", []) if p.get("id") == row.get("entity_ref")), None)
        material = None if pipe is None else next((m for m in model.get("materials", []) if m.get("id") == pipe.get("material")), None)
        if material is None:
            continue
        if case.get("modulus_basis_ref") is not None:
            point = next((p for p in material.get("temperature_points", []) if p.get("id") == case["modulus_basis_ref"]), None)
            if point is None:
                continue
            e, g = _normalized(point["elastic_modulus"], "stress"), _normalized(point["shear_modulus"], "stress")
        else:
            e, g = _normalized(material["elastic_modulus"], "stress"), _normalized(material["shear_modulus"], "stress")
        section = pipe["section"]
        od = _normalized(section["outside_diameter"], "length")
        wall = _normalized(section["wall_thickness"], "length")
        t = wall if section.get("mill_tolerance") is None else wall - _normalized(section["mill_tolerance"], "length")
        inner = od - 2.0 * t
        inertia = math.pi * ((od * od) * (od * od) - (inner * inner) * (inner * inner)) / 64.0
        factor = shear_factor(e / (2.0 * g) - 1.0, od / 2.0, inner / 2.0, inertia)
        _require(factor is not None, "model nu outside the shear domain")
        location = row["metadata"]["location"]
        vy = rows.any(cid, row["entity_ref"], "element_local_shear_force_y", location)
        vz = rows.any(cid, row["entity_ref"], "element_local_shear_force_z", location)
        _require(vy is not None and vz is not None and bits(row["value"]) == bits(shear_maximum(vy, vz, factor)), "model-bound transverse shear value")
    # Review item 1: the directional rows and records bound to the model.
    directional_model_checks(source, model, False)
