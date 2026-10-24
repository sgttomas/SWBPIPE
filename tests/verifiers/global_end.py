"""W-7 (DEL-04-01 Design global-end-actions.md): the independent mirror of the Rust reader's checks of the member end
actions in global coordinates (`core/reporting/result_export/src/global_end_evidence.rs`), shared by the pressure-2 and
preview-physics-2 verifiers.

Per load-case row: kind, unit, component, frame `global`, location `end_i`/`end_j`, basis (the wall basis only where
the member's wall end action is published at that end), sign convention and no derivation references; a combination
row carries its combination's basis. Coverage: one row per member, end and component for every member that publishes
element-local end rows, none elsewhere (a document with no global end row at all is a pre-W-7 publication under the
same contract id and is read). Rotation invariance: the global vector's norm equals the published local
vector's (the wall action as axial where named) to NORM_TOLERANCE relative. Internal consistency only, no accuracy claim
(the norm does not establish direction).
"""
from __future__ import annotations

import math
from typing import Any, Mapping

FORCE = "element_global_end_force_v1"
MOMENT = "element_global_end_moment_v1"
KINDS = frozenset({FORCE, MOMENT})
BASIS = "rotated_from_published_element_local_end_actions_v1"
WALL_BASIS = BASIS + ";axial_is_pipe_wall_action"
SIGN = "global end action on the element at its node (node on element), F = Rᵀ f; the element acts on its node with −F"
COMBINATION_BASES = ("explicit_user_linear_combination", "explicit_user_result_state_subtraction", "explicit_user_range_envelope")
COMPONENTS = {FORCE: ("N", ("Fx", "Fy", "Fz")), MOMENT: ("N*m", ("Mx", "My", "Mz"))}
LOCAL_KINDS = ("element_local_axial_force", "element_local_shear_force_y", "element_local_shear_force_z",
               "element_local_torsional_moment", "element_local_bending_moment_y", "element_local_bending_moment_z")
WALL_KIND = "pipe_wall_endpoint_action_v2"
ENDS = ("end_i", "end_j")
EPSILON = 2.0 ** -52  # binary64 epsilon
MIN_POSITIVE = 2.0 ** -1022  # smallest positive normal binary64
# The frame kernel admits axes unit and mutually orthogonal to within 1e-12 each (AXIS_TOLERANCE), so
# R R^T = I + E with ||E||_2 <= sqrt(18) t < 5 t, and a rotated norm drifts by at most ||E|| |f|; plus 64 eps rounding.
ADMITTED_AXIS_TOLERANCE = 1.0e-12
NORM_TOLERANCE = 5 * ADMITTED_AXIS_TOLERANCE + 64 * EPSILON
METADATA_KEYS = {"component", "coordinate_system", "location", "basis", "sign_convention"}


def _require(condition: Any, code: str) -> None:
    if not condition:
        raise ValueError(f"SOURCE_GLOBAL_END_{code}")


def _case(row: Mapping[str, Any]) -> str | None:
    basis = row.get("basis_ref")
    return basis.get("ref_id") if isinstance(basis, Mapping) and basis.get("ref_type") == "load_case" else None


def row_semantics(row: Mapping[str, Any], wall_admitted: bool) -> None:
    """One global end row's declared semantics; `row["kind"]` is one of KINDS."""
    _require(row.get("kind") in KINDS, "KIND")
    unit, names = COMPONENTS[row["kind"]]
    _require(row.get("unit") == unit, "UNIT")
    value = row.get("value")
    _require(isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value), "VALUE")
    md = row.get("metadata")
    _require(isinstance(md, Mapping) and set(md) == METADATA_KEYS, "METADATA_SHAPE")
    _require(md["component"] in names, "COMPONENT")
    _require(md["coordinate_system"] == "global", "COORDINATE_SYSTEM")
    _require(md["location"] in ENDS, "LOCATION")
    if _case(row) is not None:
        _require(md["basis"] == BASIS or (wall_admitted and md["basis"] == WALL_BASIS), "BASIS")
        _require(md["sign_convention"] == SIGN, "SIGN_CONVENTION")
        _require(row.get("source_result_refs", []) == [], "DERIVATION")
    else:
        _require(md["basis"] in COMBINATION_BASES, "BASIS")


def coverage(rows: list[Mapping[str, Any]]) -> None:
    """Coverage, the wall basis and rotation invariance over every load-case row."""
    local: dict = {}
    wall: dict = {}
    glob: dict = {}
    bases: dict = {}
    for row in rows:
        case = _case(row)
        md = row.get("metadata") if isinstance(row.get("metadata"), Mapping) else {}
        if case is None or md.get("location") not in ENDS:
            continue
        key, end, kind = (case, row.get("entity_ref")), ENDS.index(md["location"]), row.get("kind")
        if kind in LOCAL_KINDS and md.get("coordinate_system") == "element_local":
            local.setdefault(key, [[None] * 6, [None] * 6])[end][LOCAL_KINDS.index(kind)] = row.get("value")
        elif kind == WALL_KIND:
            wall.setdefault(key, [None, None])[end] = row.get("value")
        elif kind in KINDS:
            names = COMPONENTS[kind][1]
            _require(md.get("component") in names, "COMPONENT")
            slot = names.index(md["component"]) + (0 if kind == FORCE else 3)
            cells = glob.setdefault(key, [[None] * 6, [None] * 6])[end]
            _require(cells[slot] is None, "SLOT_DUPLICATE")
            cells[slot] = row.get("value")
            bases.setdefault(key, [set(), set()])[end].add(md.get("basis") == WALL_BASIS)
    if not glob:
        return  # a publication made before W-7 under the same contract id carries none
    members = {key for key, ends in local.items() if all(e[1] is not None and e[2] is not None for e in ends)}
    for key in glob:
        _require(key in members, "MEMBER_UNRESOLVED")
    for key in sorted(members, key=repr):
        _require(key in glob, "COVERAGE")
        walls = wall.get(key, [None, None])
        for end in range(2):
            _require(all(v is not None for v in glob[key][end]), "COVERAGE")
            is_wall = walls[end] is not None
            _require(bases[key][end] == {is_wall}, "BASIS")
            f = list(local[key][end])
            if is_wall:
                _require(f[0] is None, "AXIAL_OPERAND")
                f[0] = walls[end]
            _require(all(v is not None for v in f), "LOCAL_OPERAND")
            g = glob[key][end]
            for block in (0, 3):
                a, b = math.hypot(*f[block:block + 3]), math.hypot(*g[block:block + 3])
                _require(abs(a - b) <= NORM_TOLERANCE * max(a, MIN_POSITIVE), "ROTATION_NORM")
