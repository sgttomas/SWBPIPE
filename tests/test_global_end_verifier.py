"""W-7 (DEL-04-01 Design global-end-actions.md): the Python verifier's checks of the member end actions in global
coordinates (`tests/verifiers/global_end.py`), mirroring the Rust reader's unit controls
(`core/reporting/result_export/src/global_end_evidence.rs`). Invented rows only."""
from __future__ import annotations

from copy import deepcopy

import pytest

from tests.verifiers import global_end as ge

LOCAL = ge.LOCAL_KINDS


def _local(member: str, kind: str, end: str, value: float) -> dict:
    return {"id": f"c:{member}:{kind}:{end}", "kind": kind, "value": value, "unit": "N", "entity_ref": member,
            "basis_ref": {"ref_type": "load_case", "ref_id": "c"},
            "metadata": {"component": "c", "coordinate_system": "element_local", "location": end,
                         "basis": "recovered_from_local_element_stiffness", "sign_convention": "s"}}


def _global(member: str, end: str, name: str, value: float, basis: str) -> dict:
    force = name.startswith("F")
    return {"id": f"c:{member}:{end}:{name}", "kind": ge.FORCE if force else ge.MOMENT, "value": value,
            "unit": "N" if force else "N*m", "entity_ref": member, "basis_ref": {"ref_type": "load_case", "ref_id": "c"},
            "source_result_refs": [],
            "metadata": {"component": name, "coordinate_system": "global", "location": end, "basis": basis,
                         "sign_convention": ge.SIGN}}


def _member(member: str = "pipe:a", wall: bool = False) -> list[dict]:
    """A member along global y (local x = +y, y = -x, z = +z): f = (3, 4, 12), m = (1, 2, 2) at each end."""
    rows = []
    for end in ge.ENDS:
        for slot, kind in enumerate(LOCAL):
            if not (slot == 0 and wall):
                rows.append(_local(member, kind, end, [3.0, 4.0, 12.0, 1.0, 2.0, 2.0][slot]))
        if wall:
            rows.append(_local(member, ge.WALL_KIND, end, 3.0))
        for name, value in (("Fx", -4.0), ("Fy", 3.0), ("Fz", 12.0), ("Mx", -2.0), ("My", 1.0), ("Mz", 2.0)):
            rows.append(_global(member, end, name, value, ge.WALL_BASIS if wall else ge.BASIS))
    return rows


def _refused(rows: list[dict], code: str) -> None:
    with pytest.raises(ValueError, match=f"SOURCE_GLOBAL_END_{code}$"):
        ge.coverage(rows)


@pytest.mark.parametrize("wall", [False, True])
def test_well_formed_rows_pass(wall):
    rows = _member(wall=wall)
    ge.coverage(rows)
    for row in rows:
        if row["kind"] in ge.KINDS:
            ge.row_semantics(row, True)


def test_malformed_rows_are_refused_by_name():
    base = next(r for r in _member() if r["kind"] == ge.FORCE)
    for edit, code in [
        (lambda r: r.__setitem__("unit", "kN"), "UNIT"),
        (lambda r: r["metadata"].__setitem__("component", "Mx"), "COMPONENT"),
        (lambda r: r["metadata"].__setitem__("coordinate_system", "element_local"), "COORDINATE_SYSTEM"),
        (lambda r: r["metadata"].__setitem__("location", "midspan"), "LOCATION"),
        (lambda r: r["metadata"].__setitem__("basis", ge.WALL_BASIS), "BASIS"),
        (lambda r: r["metadata"].__setitem__("sign_convention", "node on element"), "SIGN_CONVENTION"),
        (lambda r: r["metadata"].__setitem__("extra", 1), "METADATA_SHAPE"),
        (lambda r: r.__setitem__("source_result_refs", ["result:x"]), "DERIVATION"),
        (lambda r: r.__setitem__("value", None), "VALUE"),
    ]:
        row = deepcopy(base)
        edit(row)
        with pytest.raises(ValueError, match=f"SOURCE_GLOBAL_END_{code}$"):
            ge.row_semantics(row, False)
    combined = deepcopy(base)
    combined["basis_ref"] = {"ref_type": "combination", "ref_id": "k"}
    with pytest.raises(ValueError, match="SOURCE_GLOBAL_END_BASIS$"):
        ge.row_semantics(combined, False)
    combined["metadata"]["basis"] = "explicit_user_linear_combination"
    ge.row_semantics(combined, False)


def test_coverage_and_rotation_are_refused_by_name():
    rows = _member()
    _refused([r for r in rows if not (r["kind"] == ge.MOMENT and r["metadata"]["location"] == "end_j" and r["metadata"]["component"] == "My")], "COVERAGE")
    # A document without any global row is a pre-W-7 publication and is read; a member
    # without its rows beside another member's is refused.
    ge.coverage([r for r in rows if r["kind"] not in ge.KINDS])
    _refused(rows + [r for r in _member("pipe:b") if r["kind"] not in ge.KINDS], "COVERAGE")
    _refused(rows + [deepcopy(next(r for r in rows if r["kind"] == ge.FORCE))], "SLOT_DUPLICATE")
    _refused(rows + [_global("pipe:b", "end_i", "Fx", 0.0, ge.BASIS)], "MEMBER_UNRESOLVED")
    scaled = deepcopy(rows)
    next(r for r in scaled if r["kind"] == ge.FORCE and r["metadata"]["component"] == "Fz")["value"] = 12.5
    _refused(scaled, "ROTATION_NORM")
    wrong = deepcopy(rows)
    for r in wrong:
        if r["kind"] in ge.KINDS:
            r["metadata"]["basis"] = ge.WALL_BASIS
    _refused(wrong, "BASIS")
    plain = _member(wall=True)
    for r in plain:
        if r["kind"] in ge.KINDS:
            r["metadata"]["basis"] = ge.BASIS
    _refused(plain, "BASIS")
    _refused(_member(wall=True) + [_local("pipe:a", LOCAL[0], "end_i", 3.0)], "AXIAL_OPERAND")
