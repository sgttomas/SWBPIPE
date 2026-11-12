"""DEL-04-07, the cancellation-screen exception: the Python twin of the Rust reader
(``physics_evidence::cancellation_exception``; its mutations are run on solved engine output in
``core/product_physics/tests/pressure_grouping_limits.rs``). A screen within its limit stands alone; a
failed screen stands only with the exception's record, whose family, ratio and original screen the
reader re-derives from the published groups and the case's regions.

The assembly below is the engine's, for one region over two collinear pipes whose outside diameters are
one ulp apart, both terminals transferring and pressure elongation not included: the original screen
fails on the 3.6e-12 N residual at the interior node and the case is admitted by the exception."""
from __future__ import annotations

from copy import deepcopy
import struct

import pytest

from tests.verifiers.pressure_3 import CANCELLATION_EXCEPTION_BASIS, rhs_screen

CASE = "case:pressure-oracle"
REGION = "region:pressure-oracle"
PAI = 15707.963267948964


def _group(node: str, value: float, hi: str, *terms: tuple[float, str, str]) -> dict:
    return {"assembled_force_n": value, "coefficient_sum": sum(t[0] for t in terms), "component": "Fx", "direction_component_magnitude": 1.0,
            "node_ref": node, "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": hi, "source_inner_radius_lo_bits": "3c40000000000000",
            "terms": [{"coefficient": c, "kind": k, "pipe_id": p, "region_id": REGION} for c, k, p in terms]}


def _admitted() -> tuple[dict, list[dict]]:
    rhs = {
        "assembled_pressure_rhs_global": [0.0] * 6 + [3.637978807091713e-12] + [0.0] * 11,
        "cancellation_screen": 61.35923151542565,
        "cancellation_screen_exception": {"basis": CANCELLATION_EXCEPTION_BASIS, "residual_to_source_ratio": 1.1580046200244057e-16, "residual_to_source_ratio_limit": 1e-9},
        "groups": [
            _group("node:C0", 0.0, "3fa9999999999999", (1.0, "poisson_eigen", "pipe:C0"), (-1.0, "terminal_cap", "pipe:C0")),
            _group("node:C1", -PAI, "3fa9999999999999", (-1.0, "poisson_eigen", "pipe:C0")),
            _group("node:C1", 15707.963267948968, "3fa999999999999a", (1.0, "poisson_eigen", "pipe:C1")),
            _group("node:C2", 0.0, "3fa999999999999a", (-1.0, "poisson_eigen", "pipe:C1"), (1.0, "terminal_cap", "pipe:C1")),
        ],
        "node_order": ["node:C0", "node:C1", "node:C2"],
    }
    terminal = lambda node: {"node_ref": node, "closure_transfer": "transfers_to_wall"}
    regions = [{"region_id": REGION, "load_case_id": CASE, "member_pipe_ids": ["pipe:C0", "pipe:C1"], "geometry": [], "terminals": [terminal("node:C0"), terminal("node:C2")]}]
    return rhs, regions


def _up(value: float) -> float:
    return struct.unpack("<d", struct.pack("<q", struct.unpack("<q", struct.pack("<d", value))[0] + 1))[0]


def test_the_engine_s_admitted_assembly_is_read() -> None:
    rhs, regions = _admitted()
    rhs_screen(rhs, CASE, False, regions)


def _refused(rhs: dict, regions: list[dict], code: str, elongation: bool | None = False) -> None:
    with pytest.raises(ValueError) as refusal:
        rhs_screen(rhs, CASE, elongation, regions)
    assert str(refusal.value).endswith(": " + code)


@pytest.mark.parametrize(("label", "change", "code"), [
    ("no record beside a failed screen", lambda rhs, regions: rhs.pop("cancellation_screen_exception"), "SOURCE_PHYSICS_RHS_SCREEN"),
    ("another basis", lambda rhs, regions: rhs["cancellation_screen_exception"].update(basis="source_group_magnitudes"), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION"),
    ("another limit", lambda rhs, regions: rhs["cancellation_screen_exception"].update(residual_to_source_ratio_limit=1e-8), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION"),
    ("a ratio one ulp off", lambda rhs, regions: rhs["cancellation_screen_exception"].update(residual_to_source_ratio=_up(1.1580046200244057e-16)), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_RATIO"),
    ("a ratio over its limit", lambda rhs, regions: rhs["cancellation_screen_exception"].update(residual_to_source_ratio=2e-9), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_RATIO"),
    ("a record beside a passing screen", lambda rhs, regions: rhs.update(cancellation_screen=1e-12), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_BESIDE_PASSING_SCREEN"),
    ("another failed screen", lambda rhs, regions: rhs.update(cancellation_screen=_up(61.35923151542565)), "SOURCE_PHYSICS_RHS_SCREEN_RECOMPUTED"),
    ("a terminal that does not transfer", lambda rhs, regions: regions[0]["terminals"][1].update(closure_transfer="separately_supported_or_compensated"), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY"),
    ("a kink remainder", lambda rhs, regions: rhs["groups"][1]["terms"][0].update(kind="kink_remainder"), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY"),
    ("a second region at the interior node", lambda rhs, regions: rhs["groups"][2]["terms"][0].update(region_id="region:other"), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY"),
    ("a junction of regions at a terminal", lambda rhs, regions: regions.append({"region_id": "region:other", "terminals": [{"node_ref": "node:C2", "closure_transfer": "transfers_to_wall"}], "geometry": []}), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY"),
    ("a connector member", lambda rhs, regions: regions[0].update(member_connector_ids=["component:joint"]), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY"),
    ("a realized arc", lambda rhs, regions: regions[0]["geometry"].append({"pipe_id": "pipe:C1", "member_kind": "realized_arc"}), "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY"),
])
def test_a_record_that_the_groups_do_not_bear_out_is_refused_by_name(label: str, change, code: str) -> None:
    rhs, regions = _admitted()
    change(rhs, regions)
    _refused(rhs, regions, code)


def test_a_case_that_includes_pressure_elongation_is_outside_the_family() -> None:
    rhs, regions = _admitted()
    _refused(rhs, regions, "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY", elongation=True)


def test_a_passing_screen_stands_alone() -> None:
    rhs, regions = _admitted()
    rhs.pop("cancellation_screen_exception")
    rhs["cancellation_screen"] = 0.0
    rhs_screen(rhs, CASE, False, regions)
    rhs["cancellation_screen"] = None
    _refused(rhs, regions, "SOURCE_PHYSICS_RHS_SCREEN")
