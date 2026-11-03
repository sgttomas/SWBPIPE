"""RT-A3 increment 4 (DEL-04-07 Design ``route-unification-r4.md`` §2.4 and §3.5 S5; DEL-04-01
``kd5-nonlinear-active-set.md`` §0, §2.2, §2.4 and §3): test-only mirror of RE ``nonlinear_support_evidence.rs``, the
pressure-2 reader's binding of the nonlinear support results the exact route publishes through PF's shared active-set
loop.

``check`` reads the envelope alone and ``model_checks`` binds it to the model's nonlinear supports; both run the Rust
module's checks in its order and refuse by its codes (``SOURCE_PHYSICS_NONLINEAR_*``), so the two readers accept the
same envelopes. PF's support-magnitude rule is reused (``preview_physics_evidence._consistent_norm``), and the unit
catalog is the verifiers' own (``units.to_canonical``). Standard library and sibling verifiers only.
"""
from __future__ import annotations

from collections.abc import Mapping
import math
import struct
from typing import Any

from .preview_physics_evidence import _consistent_norm
from .units import to_canonical

STATE = "nonlinear_support_active_set_state_code"
DISPLACEMENT = "nonlinear_support_final_displacement"
RELATIVE = "nonlinear_support_final_relative_displacement"
REACTION = "nonlinear_support_final_reaction"
NORMAL_INPUT = "nonlinear_support_friction_normal_reaction_input"
NORMAL_DERIVED = "nonlinear_support_friction_normal_reaction_derived"
ITERATIONS = "nonlinear_support_active_set_iteration_count"
RESIDUAL = "nonlinear_support_active_set_final_residual_count"
CONVERGED = "nonlinear_support_active_set_converged_flag"
FORCE_RESIDUAL = "nonlinear_support_observed_free_dof_force_residual"
MOMENT_RESIDUAL = "nonlinear_support_observed_free_dof_moment_residual"
LOOP_ENTITY = "nonlinear_supports"
PER_SUPPORT = (STATE, DISPLACEMENT, RELATIVE, REACTION, NORMAL_INPUT, NORMAL_DERIVED)
COMPONENT = "support_reaction_component_v2"
FORCE_MAGNITUDE = "support_reaction_force_magnitude_v2"
MOMENT_MAGNITUDE = "support_reaction_moment_magnitude_v2"
STATES = ("inactive", "active", "sticking", "sliding")
CLASSES = ("friction", "gap", "lift_off", "one_way")
DOFS = ("ux", "uy", "uz", "rx", "ry", "rz")
COMPONENTS = ("Fx", "Fy", "Fz", "Mx", "My", "Mz")
NODAL = ("global_nodal_displacement_x", "global_nodal_displacement_y", "global_nodal_displacement_z",
         "global_nodal_rotation_x", "global_nodal_rotation_y", "global_nodal_rotation_z")
STATE_REVIEW = "NONLINEAR_SUPPORT_STATE_REVIEW"
INTEGRITY = "NUMERICAL_INTEGRITY_"
PASSED = "NUMERICAL_INTEGRITY_CHECKS_PASSED"
UNQUALIFIED = "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED"
CONFIRMED = "Converged support state: established at 128-bit precision by one formation-checked solve;"
RELATIVE_TOLERANCE = 1e-9
# product_physics ``assessed_numerical_quality``: each published integrity code's (structural_status, solve_quality,
# model_matrix_fidelity, accuracy_evidence); ``NUMERICAL_INTEGRITY_UNRESOLVED`` takes its default arm. Any other code
# is not the producer's.
STANDING_QUALITY = {
    PASSED: ("passive_model_basis", "checks_passed", "represented_equations_retained", "not_claimed"),
    "NUMERICAL_INTEGRITY_SENSITIVE": ("passive_model_basis", "sensitive", "represented_equations_retained", "not_claimed"),
    "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM": ("physical_mechanism_witnessed", "failed", "not_assessed", "not_claimed"),
    "NUMERICAL_INTEGRITY_NEGATIVE_ENERGY": ("negative_energy_witnessed", "failed", "not_assessed", "not_claimed"),
    UNQUALIFIED: ("numerically_unresolved", "unresolved", "not_assessed", "unresolved"),
    "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED": ("numerically_unresolved", "unresolved", "assembly_loss_detected", "unresolved"),
    "NUMERICAL_INTEGRITY_FAILED": ("numerically_unresolved", "failed", "assembly_uncertainty", "unresolved"),
    "NUMERICAL_INTEGRITY_UNRESOLVED": ("numerically_unresolved", "unresolved", "assembly_uncertainty", "unresolved"),
}
_TRANSLATIONAL_TOKENS = ("UX", "Ux", "ux", "UY", "Uy", "uy", "UZ", "Uz", "uz")
_DOF_TOKENS = _TRANSLATIONAL_TOKENS + ("RX", "Rx", "rx", "RY", "Ry", "ry", "RZ", "Rz", "rz")
_REACTION_PLUS = ("positive_reaction", "positive", "POSITIVE_REACTION", "POSITIVE")
_REACTION_MINUS = ("negative_reaction", "negative", "NEGATIVE_REACTION", "NEGATIVE")
_DISPLACEMENT_PLUS = ("positive_displacement", "positive", "POSITIVE_DISPLACEMENT", "POSITIVE")
_DISPLACEMENT_MINUS = ("negative_displacement", "negative", "NEGATIVE_DISPLACEMENT", "NEGATIVE")


def _require(condition: bool, code: str) -> None:
    if not condition:
        raise ValueError(f"SOURCE_PHYSICS_NONLINEAR_{code}")


def _items(value: Any) -> list:
    return value if isinstance(value, list) else []


def _get(value: Any, *keys: str) -> Any:
    for key in keys:
        value = value.get(key) if isinstance(value, Mapping) else None
    return value


def _text(value: Any) -> str | None:
    return value if isinstance(value, str) and value else None


def _number(value: Any) -> float:
    return float(value) if type(value) in (int, float) else math.nan


def _value(row: Any) -> float:
    return _number(_get(row, "value"))


def _kind(row: Any) -> str:
    kind = _get(row, "kind")
    return kind if isinstance(kind, str) else ""


def _case_of(row: Any) -> str | None:
    if _get(row, "basis_ref", "ref_type") != "load_case":
        return None
    ref = _get(row, "basis_ref", "ref_id")
    return ref if isinstance(ref, str) else None


def _basis(row: Any) -> str:
    basis = _get(row, "metadata", "basis")
    return basis if isinstance(basis, str) else ""


def _segment(basis: str, key: str) -> str | None:
    for part in basis.split("; "):
        if part.startswith(key + "="):
            return part[len(key) + 1:]
    return None


def _dof_index(dof: Any) -> int | None:
    return DOFS.index(dof) if isinstance(dof, str) and dof in DOFS else None


def _bits(value: float) -> bytes:
    return struct.pack(">d", value)


class _Support:
    def __init__(self, sid: str, dof: int, state: str, displacement: float, reaction: float,
                 normal: tuple[bool, float] | None, source: tuple[str, int] | None):
        self.id, self.dof, self.state = sid, dof, state
        # The deciding displacement in the row's unit (mm or rad).
        self.displacement, self.reaction = displacement, reaction
        self.normal, self.source = normal, source


class _Case:
    def __init__(self, cid: str, classes: list[str], supports: list[_Support], code: str):
        self.id, self.classes, self.supports, self.code = cid, classes, supports, code


def _case_ids(rows: list) -> list[str]:
    cases: list[str] = []
    for row in rows:
        # A combination's rows are the combination pass's (``exact_combinations``).
        if not _kind(row).startswith("nonlinear_support_") or _get(row, "basis_ref", "ref_type") == "combination":
            continue
        case = _case_of(row)
        _require(case is not None, "SCOPE")
        if case not in cases:
            cases.append(case)
    return cases


def _scales(source: Mapping, case: str, translational: bool) -> tuple[float, float]:
    components = COMPONENTS[:3] if translational else COMPONENTS[3:]
    nodal = NODAL[:3] if translational else NODAL[3:]
    force = motion = 0.0
    for row in _items(source.get("results")):
        if _case_of(row) != case:
            continue
        v = abs(_value(row))
        if _kind(row) == COMPONENT and _get(row, "metadata", "component") in components:
            force = max(force, v)
        if _kind(row) in nodal:
            motion = max(motion, v)
    return force, motion


def _read_case(source: Mapping, case: str) -> _Case:
    rows = _items(source.get("results"))
    in_case = [r for r in rows if _case_of(r) == case]
    nonlinear = [r for r in in_case if _kind(r).startswith("nonlinear_support_")]

    def one(kind: str) -> Any:
        found = [r for r in nonlinear if _kind(r) == kind]
        return found[0] if len(found) == 1 and _get(found[0], "entity_ref") == LOOP_ENTITY else None

    iterations, residual, converged = one(ITERATIONS), one(RESIDUAL), one(CONVERGED)
    _require(iterations is not None and residual is not None and converged is not None, "LOOP")
    count = _value(iterations)
    _require(count >= 1.0 and count == math.floor(count) and _value(residual) == 0.0 and _value(converged) == 1.0, "LOOP")
    segment = _segment(_basis(iterations), "support_classes")
    classes = segment.split(",") if segment is not None else []
    _require(bool(classes) and all(c in CLASSES for c in classes) and all(a < b for a, b in zip(classes, classes[1:])),
             "CLASSES")
    loop_basis = _basis(iterations)
    mode = loop_basis.split("_active_set_loop", 1)[0] if "_active_set_loop" in loop_basis else None
    _require(bool(mode) and " " not in mode, "LOOP")
    # No case moves a nonlinear support's ground on S5 (the Rust module docs).
    for row in nonlinear:
        _require(_kind(row) != RELATIVE, "RELATIVE_DISPLACEMENT")
        _require(_kind(row) in PER_SUPPORT or _get(row, "entity_ref") == LOOP_ENTITY, "ROWS")
    supports: list[_Support] = []
    for row in (r for r in nonlinear if _kind(r) == STATE):
        sid = _text(_get(row, "entity_ref"))
        _require(sid is not None, "ROWS")
        _require(not any(s.id == sid for s in supports), "STATE_DUPLICATE")
        code = _value(row)
        _require(code in (0.0, 1.0, 2.0, 3.0), "STATE_CODE")
        state = STATES[int(code)]
        dof = _dof_index(_get(row, "metadata", "location"))
        _require(dof is not None, "DOF")

        def mine(kind: str) -> list:
            return [r for r in nonlinear if _kind(r) == kind and _get(r, "entity_ref") == sid]

        displacement, reaction = mine(DISPLACEMENT), mine(REACTION)
        normals = mine(NORMAL_INPUT) + mine(NORMAL_DERIVED)
        _require(len(displacement) == 1 and len(reaction) == 1 and len(normals) <= 1, "ROWS")
        length, force = ("mm", "N") if dof < 3 else ("rad", "N*m")

        def stated(r: Any) -> bool:
            return _segment(_basis(r), "final_state") == state and _get(r, "metadata", "location") == DOFS[dof]

        _require(stated(row) and stated(displacement[0]) and stated(reaction[0]), "STATE_BASIS")
        _require(_get(row, "unit") == "state_code" and _get(displacement[0], "unit") == length
                 and _get(reaction[0], "unit") == force, "UNIT")
        # The producer restates a node's published row.
        _require(any(_kind(r) == NODAL[dof] and _get(r, "unit") == length and _bits(_value(r)) == _bits(_value(displacement[0]))
                     for r in in_case), "DISPLACEMENT")
        normal = source_ref = None
        if normals:
            n = normals[0]
            derived = _kind(n) == NORMAL_DERIVED
            _require(_get(n, "unit") == "N", "UNIT")
            if derived:
                named = _segment(_basis(n), "source_ref")
                at = _dof_index(_segment(_basis(n), "source_dof"))
                at = at if at is not None and at < 3 else None
                _require(named is not None and at is not None, "NORMAL")
                source_ref = (named, at)
            normal = (derived, _value(n))
        supports.append(_Support(sid, dof, state, _value(displacement[0]), _value(reaction[0]), normal, source_ref))
    for row in nonlinear:
        if _kind(row) in PER_SUPPORT:
            _require(any(_get(row, "entity_ref") == s.id for s in supports), "ROWS")
    _require(bool(supports), "ROWS")

    def action(entity: str, kind: str, component: str) -> float | None:
        found = [r for r in in_case if _kind(r) == kind and _get(r, "entity_ref") == entity
                 and _get(r, "metadata", "component") == component]
        return _value(found[0]) if len(found) == 1 else None

    for s in supports:
        slot = action(s.id, COMPONENT, COMPONENTS[s.dof])
        _require(slot is not None and _bits(slot) == _bits(s.reaction), "SLOT")
        c = [action(s.id, COMPONENT, component) for component in COMPONENTS]
        fm, mm = action(s.id, FORCE_MAGNITUDE, "force_magnitude"), action(s.id, MOMENT_MAGNITUDE, "moment_magnitude")

        def consistent(m: float | None, parts: list) -> bool:
            return m is not None and all(p is not None for p in parts) and m >= 0.0 and _consistent_norm(m, parts)

        _require(consistent(fm, c[:3]) and consistent(mm, c[3:]), "MAGNITUDE")
        if s.source is not None:
            published = action(s.source[0], COMPONENT, COMPONENTS[s.source[1]])
            _require(published is not None and _bits(abs(published)) == _bits(s.normal[1]), "NORMAL")
    diagnostics = _items(source.get("diagnostics"))

    def refs(d: Any) -> list:
        return _items(_get(d, "affected_refs"))

    reviews = [d for d in diagnostics if _get(d, "code") == STATE_REVIEW and len(refs(d)) > 1 and refs(d)[1] == case
               and isinstance(refs(d)[1], str)]
    _require(len(reviews) == len(supports), "STATE_REVIEW")
    for s in supports:
        message = f"nonlinear support {s.id} ended in {s.state} state for load case {case}; {mode} preview loop evidence only"
        _require(sum(1 for d in reviews if refs(d)[0] == s.id and len(refs(d)) == 2 and _get(d, "message") == message) == 1,
                 "STATE_REVIEW")
    friction = sum(1 for s in supports if s.normal is not None)
    _require((friction > 0) == ("friction" in classes)
             and (friction < len(supports)) == any(c != "friction" for c in classes), "CLASSES")
    for s in supports:
        if s.normal is None:
            _require(s.state in ("inactive", "active"), "STATE_KIND")
        else:
            derived, n = s.normal
            if s.state == "sliding":
                _require(s.displacement != 0.0 and s.reaction * s.displacement <= 0.0, "SLIP")
            if s.state == "inactive":
                _require(n == 0.0 if derived else n <= 0.0, "FRICTION_NORMAL")
    integrity = [d for d in diagnostics if isinstance(_get(d, "code"), str) and _get(d, "code").startswith(INTEGRITY)
                 and isinstance(_get(d, "affected_refs"), list) and _get(d, "affected_refs") == [case]]
    _require(len(integrity) == 1, "STANDING")
    code = _get(integrity[0], "code")
    message = _get(integrity[0], "message")
    message = message if isinstance(message, str) else ""
    quality = [q for q in _items(_get(source, "numerical_quality", "cases"))
               if _get(q, "basis_ref", "ref_type") == "load_case" and _get(q, "basis_ref", "ref_id") == case]
    _require(len(quality) == 1, "STANDING")
    solve_quality = _get(quality[0], "solve_quality")
    solve_quality = solve_quality if isinstance(solve_quality, str) else ""
    tokens = STANDING_QUALITY.get(code)
    _require(tokens is not None, "STANDING_CODE")
    fields = tuple(_get(quality[0], f) for f in ("structural_status", "solve_quality", "model_matrix_fidelity", "accuracy_evidence"))
    diagnostic_id = _get(integrity[0], "id")
    _require(all(type(f) is str and f == t for f, t in zip(fields, tokens)) and isinstance(diagnostic_id, str)
             and diagnostic_id in _items(_get(quality[0], "evidence_refs")), "STANDING")
    if "gap" in classes:
        _require(code == UNQUALIFIED and solve_quality == "unresolved", "GAP_STANDING")
    if code == PASSED:
        _require(CONFIRMED in message, "PASSED_EVIDENCE")
        for s in supports:
            _require(not (s.state == "inactive" and s.normal is not None and s.normal[0]), "FRICTION_NORMAL_STANDING")
            _require(not (s.normal is not None and s.state == "active"), "PASSED_STATE")
            if s.state == "inactive":
                _require(_bits(s.reaction) == _bits(0.0), "PASSED_ZERO")
    else:
        # A non-Passed inactive support: the loop's free-row residual, bound to the producer's own evaluation.
        for s in supports:
            if s.state == "inactive":
                _require(abs(s.reaction) <= _free_row_residual(nonlinear, s.dof < 3), "INACTIVE_REACTION")
    return _Case(case, classes, supports, code)


def _free_row_residual(nonlinear: list, translational: bool) -> float:
    """Rust ``free_row_residual``: the producer's original-equation evaluation of the final linearized solve's free
    rows in one family, its largest |residual| (N or N*m), published once on the loop with a compliant verdict."""
    wanted, unit = (FORCE_RESIDUAL, "N") if translational else (MOMENT_RESIDUAL, "N*m")
    found = [r for r in nonlinear if _kind(r) == wanted]
    _require(len(found) == 1, "RESIDUAL")
    row = found[0]
    evaluated, basis = _value(row), _basis(row)
    _require(_get(row, "entity_ref") == LOOP_ENTITY and _get(row, "unit") == unit and math.isfinite(evaluated)
             and evaluated >= 0.0 and _segment(basis, "residual_basis") == "free_dof_force_moment_equilibrium"
             and _segment(basis, "observed_compliance") == "true", "RESIDUAL")
    return evaluated


def _read(source: Mapping) -> list[_Case]:
    return [_read_case(source, case) for case in _case_ids(_items(source.get("results")))]


def check(source: Mapping[str, Any]) -> None:
    """Rust ``nonlinear_support_evidence::check``: the model-free checks, per load case with nonlinear rows in
    first-row order; an envelope without nonlinear rows passes unchanged."""
    if isinstance(source, Mapping):
        _read(source)


def _quantity(quantity: Any, dimension: str) -> float:
    value, unit = _get(quantity, "value"), _get(quantity, "unit")
    _require(type(value) in (int, float) and math.isfinite(value) and isinstance(unit, str) and bool(unit), "MODEL_SUPPORT")
    try:
        return to_canonical(value, unit, dimension)
    except (ValueError, OverflowError) as error:
        raise ValueError("SOURCE_PHYSICS_NONLINEAR_MODEL_SUPPORT") from error


class _Input:
    def __init__(self, sid: Any, node: str, dof: int):
        self.id, self.node, self.dof = sid, node, dof
        self.behavior, self.sense, self.clearance_mm, self.mu = "", 0.0, 0.0, 0.0
        self.explicit_normal: float | None = None
        self.source: tuple[Any, int] | None = None


def _model_input(support: Mapping) -> _Input:
    n = support.get("nonlinear")
    dof = _get(n, "dof")
    _require(isinstance(dof, str) and dof in _DOF_TOKENS, "MODEL_SUPPORT")
    node = _text(support.get("node"))
    _require(node is not None, "MODEL_SUPPORT")
    item = _Input(support.get("id"), node, DOFS.index(dof.lower()))

    def sense(token: Any, plus: tuple, minus: tuple) -> float:
        _require(isinstance(token, str) and (token in plus or token in minus), "MODEL_SUPPORT")
        return 1.0 if token in plus else -1.0

    behavior = _get(n, "behavior")
    if behavior in ("one_way", "one-way", "oneway"):
        item.behavior, item.sense = "one_way", sense(_get(n, "active_when"), _REACTION_PLUS, _REACTION_MINUS)
    elif behavior in ("lift_off", "lift-off", "liftoff"):
        item.behavior, item.sense = "lift_off", sense(_get(n, "contact_when"), _REACTION_PLUS, _REACTION_MINUS)
    elif behavior == "gap":
        item.behavior, item.sense = "gap", sense(_get(n, "closes_when"), _DISPLACEMENT_PLUS, _DISPLACEMENT_MINUS)
        _require(item.dof < 3, "MODEL_SUPPORT")
        item.clearance_mm = _quantity(_get(n, "gap"), "length") * 1000.0
    elif behavior == "friction":
        item.behavior = "friction"
        coefficient = _get(n, "friction_coefficient")
        if _get(coefficient, "unit") == "none":
            mu = _get(coefficient, "value")
            _require(type(mu) in (int, float) and math.isfinite(mu), "MODEL_SUPPORT")
            item.mu = float(mu)
        else:
            item.mu = _quantity(coefficient, "dimensionless")
        normal, source = _get(n, "normal_reaction"), _get(n, "normal_reaction_source")
        if normal is not None and source is None:
            item.explicit_normal = _quantity(normal, "force")
        elif normal is None and source is not None:
            at = _get(source, "dof")
            _require(isinstance(at, str) and at in _TRANSLATIONAL_TOKENS, "MODEL_SUPPORT")
            item.source = (_get(source, "support_ref"), DOFS.index(at.lower()))
        else:
            _require(False, "MODEL_SUPPORT")
    else:
        _require(False, "MODEL_SUPPORT")
    return item


def model_checks(source: Mapping[str, Any], model: Mapping[str, Any]) -> None:
    """Rust ``nonlinear_support_evidence::model_checks``: the rows bound to the model's nonlinear supports and the
    DEC-067 laws that need their inputs; a solved envelope only."""
    if _get(source, "status", "mechanics") != "MECHANICS_SOLVED" or _get(model, "supports") is None:
        return
    records = [s for s in _items(model.get("supports")) if isinstance(s, Mapping) and s.get("nonlinear") is not None]
    inputs = [_model_input(s) for s in records]
    cases = _read(source)
    solved = [_get(c, "load_case_id") for c in _items(_get(source, "contract_evidence", "exact_cases"))]
    for sid in solved:
        found = next((c for c in cases if isinstance(sid, str) and sid == c.id), None)
        if found is None:
            _require(not inputs, "MODEL_COVERAGE")
        else:
            _require(len(found.supports) == len(inputs)
                     and all(any(i.id == s.id and isinstance(i.id, str) for s in found.supports) for i in inputs),
                     "MODEL_COVERAGE")
    _require(all(any(isinstance(sid, str) and sid == c.id for sid in solved) for c in cases), "MODEL_COVERAGE")
    for case in cases:
        _require(case.classes == sorted({i.behavior for i in inputs}), "MODEL_CLASSES")
        for s in case.supports:
            item = next((i for i in inputs if isinstance(i.id, str) and i.id == s.id), None)
            _require(item is not None, "MODEL_COVERAGE")
            _require(item.dof == s.dof, "MODEL_DOF")
            # The final displacement is the support node's published row.
            length = "mm" if s.dof < 3 else "rad"
            nodal = [_value(r) for r in _items(source.get("results")) if _case_of(r) == case.id and _kind(r) == NODAL[s.dof]
                     and _get(r, "entity_ref") == item.node and _get(r, "unit") == length]
            _require(len(nodal) == 1 and _bits(nodal[0]) == _bits(s.displacement), "MODEL_DISPLACEMENT")
            if item.behavior == "friction":
                if s.normal is not None and not s.normal[0] and s.source is None:
                    bound = item.explicit_normal is not None and _bits(item.explicit_normal) == _bits(s.normal[1])
                elif s.normal is not None and s.normal[0] and s.source is not None:
                    bound = item.source is not None and isinstance(item.source[0], str) \
                        and item.source[0] == s.source[0] and item.source[1] == s.source[1]
                else:
                    bound = False
            else:
                bound = s.normal is None
            _require(bound, "MODEL_NORMAL")
            force_scale, motion_scale = _scales(source, case.id, s.dof < 3)
            r, delta = s.reaction, s.displacement
            if item.behavior == "friction":
                n = s.normal[1] if s.normal is not None else 0.0
                mu_n = item.mu * n
                tolerance = RELATIVE_TOLERANCE * max(force_scale, abs(mu_n))
                if s.state != "inactive":
                    _require(abs(r) <= mu_n + tolerance, "MODEL_FRICTION_CAPACITY")
                if s.state == "sticking":
                    _require(abs(delta) <= RELATIVE_TOLERANCE * motion_scale, "MODEL_STICK")
                elif s.state == "sliding":
                    _require(abs(abs(r) - mu_n) <= tolerance and (mu_n <= tolerance or r * delta < 0.0), "MODEL_SLIDE")
                    if case.code == PASSED and item.explicit_normal is not None:
                        exact = 0.0 if mu_n == 0.0 else -math.copysign(mu_n, delta)
                        _require(_bits(r) == _bits(exact), "MODEL_SLIDE_EXACT")
            else:
                if item.behavior == "gap":
                    separation, contact = item.clearance_mm - item.sense * delta, -item.sense * r
                else:
                    separation, contact = item.sense * delta, item.sense * r
                length_tolerance = RELATIVE_TOLERANCE * max(motion_scale, abs(item.clearance_mm))
                force_tolerance = RELATIVE_TOLERANCE * force_scale
                _require(separation >= -length_tolerance, "MODEL_COMPLEMENTARITY")
                # An active support is closed and pushes; an inactive one's force is the model-free rule.
                if s.state == "active":
                    _require(abs(separation) <= length_tolerance and contact >= -force_tolerance, "MODEL_COMPLEMENTARITY")
