"""A minimal solved preview-physics-2 statement for transport and record tests.

B1 (b7-plan) retired precision-1, the identity these tests used as a readable
statement with invented rows. This one carries the current ordinary identity,
its profile limitations, an empty preview case per numerical case and no
headline, which the Rust preview reader admits. Invented values only; it makes
no solver, physics or accuracy claim.
"""
from __future__ import annotations

from copy import deepcopy
import json
from pathlib import Path
from typing import Any

PROJECT = Path(__file__).resolve().parents[1]
PREVIEW_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-2"
RETIRED_PRECISION_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/precision-1"
BASIS = {"ref_type": "load_case", "ref_id": "load:transport"}
# The preview table's signatures for the two row kinds these statements carry.
ROTATION_X_METADATA = {"component": "nodal_rotation_x", "coordinate_system": "global", "location": "node", "basis": "solved_from_global_linear_system", "sign_convention": "positive value follows the right-hand-rule rotation about the global cartesian X axis"}
DISPLACEMENT_X_METADATA = {"component": "nodal_displacement_x", "coordinate_system": "global", "location": "node", "basis": "solved_from_global_linear_system", "sign_convention": "positive value follows the global cartesian X axis displacement of the node"}
WORK_RESIDUAL_METADATA = {"component": "free_dof_work_residual", "coordinate_system": "solver_iteration", "location": "final_iteration", "basis": "invented transport fixture", "sign_convention": "nonnegative max absolute free-DOF residual work product in the final linearized solve"}
_LIMITATIONS = json.loads((PROJECT / "fixtures/results/semantic_contract_v0_3_preview_physics_2.json").read_text())["supported_profile_limitations"]


def source(value: float = 1.2345678901234567e-7) -> dict[str, Any]:
    return {"schema_version": "0.2.0", "run_id": "preview-transport", "model_ref": "preview-model",
        "producer": {"component_name": "open_pipe_stress_product_physics", "component_version": "0.2.0", "semantic_contract_id": PREVIEW_CONTRACT_ID},
        "numerical_quality": {"value_representation": "finite_binary64", "publication_quantization": "none", "integrity_policy": "M03-INTEGRITY-v1", "status": "not_assessed", "cases": []},
        "formulation_basis": {"profile_id": "product_preview_mechanics_v1", "limitations": deepcopy(_LIMITATIONS)},
        "contract_evidence": {"preview_cases": [], "combination_gates": []},
        "status": {"mechanics": "MECHANICS_SOLVED"}, "summary": {"max_displacement": None, "max_open_formula_stress": None},
        "diagnostics": [],
        "results": [{"id": "result:rotation", "kind": "global_nodal_rotation_x", "unit": "rad", "entity_ref": "node:transport", "value": value, "basis_ref": deepcopy(BASIS), "metadata": deepcopy(ROTATION_X_METADATA)}]}


def sync_preview_cases(raw: dict[str, Any]) -> dict[str, Any]:
    """One empty, complete preview case per numerical case, in its order (the preview reader's coverage)."""
    raw["contract_evidence"]["preview_cases"] = [
        {"load_case_id": case["basis_ref"]["ref_id"], "pipe_stress_extrema": [],
         "stress_maximum_coverage": {"complete": True, "unavailable_pipe_ids": [], "outside_domain_pipe_ids": []},
         "support_attribution": {"attributed_support_ids": [], "withheld": []}, "intensified_measures": []}
        for case in raw["numerical_quality"]["cases"]]
    return raw


def sync_headline(raw: dict[str, Any]) -> dict[str, Any]:
    """The displacement headline the preview reader recomputes from the rows (largest value; ties by case, then entity)."""
    rows = [row for row in raw["results"] if row.get("kind") == "displacement_magnitude"
            and isinstance(row.get("basis_ref"), dict) and row["basis_ref"].get("ref_type") == "load_case"]
    best = None
    for row in rows:
        key = (row["value"], row["basis_ref"]["ref_id"], row["entity_ref"])
        if best is None or key[0] > best[0][0] or (key[0] == best[0][0] and key[1:] < best[0][1:]):
            best = (key, row)
    raw["summary"]["max_displacement"] = None if best is None else {
        "value": best[1]["value"], "unit": best[1]["unit"], "location_ref": best[1]["entity_ref"], "result_ref": best[1]["id"]}
    return raw


def retired_precision(raw: dict[str, Any]) -> dict[str, Any]:
    """The same statement as a stored precision-1 result (no preview evidence): refused by name."""
    old = deepcopy(raw)
    old["producer"]["semantic_contract_id"] = RETIRED_PRECISION_CONTRACT_ID
    old.pop("contract_evidence", None)
    return old
