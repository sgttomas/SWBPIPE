"""DEL-04-07, the cancellation-screen exception: the Python twin of the Rust reader
(``physics_evidence::cancellation_exception``, ``bore_group_value`` and ``assembled_is_group_sum``; its
mutations are also run on solved engine output in ``core/product_physics/tests/pressure_grouping_limits.rs``). A
screen within its limit stands alone; a failed screen stands only with the exception's record, whose family, node,
measured ratio, consistency bound and original screen the reader re-derives from the published groups and the
case's regions, after re-forming every bore group's value and the assembled vector from the groups.

The two assemblies below are the engine's (sparse), each one region transferring at both terminals without pressure
elongation: two collinear 3 m pipes whose outside diameters are one ulp apart (the interior thrust is 3.6e-12 N), and
two skew pipes whose interior node is 1/1000 of the 3 m span from a terminal and 50 epsilon of the span off the chord.
The original screen fails on each and each is admitted by the exception. This twin reads no model: the span and the
member lengths are the record's (the Rust verdict given the model re-forms them)."""
from __future__ import annotations

from copy import deepcopy
import math

import pytest

from tests.verifiers.pressure_3 import rhs_screen

CASE = "case:P"

ASSEMBLIES = {
    "one ulp": ({"node_order": ["node:0", "node:1", "node:2"], "assembled_pressure_rhs_global": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.637978807091713e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], "cancellation_screen": 61.35923151542565, "cancellation_screen_exception": {"basis": "straight_region_interior_cap_pairs_representation_guard_departure_thrust", "consistency_bound_ratio": 1.9895196601282805e-13, "member_lengths_m": [3.0, 3.0], "member_pipe_ids": ["pipe:0", "pipe:1"], "node_ref": "node:1", "region_span_m": 6.0, "thrust_to_bore_force_ratio": 2.316009240048811e-16}, "groups": [{"assembled_force_n": 0.0, "coefficient_sum": 0.0, "component": "Fx", "direction_component_magnitude": 1.0, "node_ref": "node:0", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": 1.0, "kind": "poisson_eigen", "pipe_id": "pipe:0", "region_id": "region:P"}, {"coefficient": -1.0, "kind": "terminal_cap", "pipe_id": "pipe:0", "region_id": "region:P"}]}, {"assembled_force_n": -15707.963267948964, "coefficient_sum": -1.0, "component": "Fx", "direction_component_magnitude": 1.0, "node_ref": "node:1", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": -1.0, "kind": "poisson_eigen", "pipe_id": "pipe:0", "region_id": "region:P"}]}, {"assembled_force_n": 15707.963267948968, "coefficient_sum": 1.0, "component": "Fx", "direction_component_magnitude": 1.0, "node_ref": "node:1", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa999999999999a", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": 1.0, "kind": "poisson_eigen", "pipe_id": "pipe:1", "region_id": "region:P"}]}, {"assembled_force_n": 0.0, "coefficient_sum": 0.0, "component": "Fx", "direction_component_magnitude": 1.0, "node_ref": "node:2", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa999999999999a", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": -1.0, "kind": "poisson_eigen", "pipe_id": "pipe:1", "region_id": "region:P"}, {"coefficient": 1.0, "kind": "terminal_cap", "pipe_id": "pipe:1", "region_id": "region:P"}]}]},
                {"region_id": "region:P", "load_case_id": "case:P", "member_pipe_ids": ["pipe:0", "pipe:1"], "geometry": [], "terminals": [{"node_ref": "node:0", "closure_transfer": "transfers_to_wall"}, {"node_ref": "node:2", "closure_transfer": "transfers_to_wall"}], "applied_loads": [{"pipe_id": "pipe:0", "local_x_global": [1.0, 0.0, 0.0]}, {"pipe_id": "pipe:1", "local_x_global": [1.0, 0.0, 0.0]}]}),
    "skew": ({"node_order": ["node:0", "node:1", "node:2"], "assembled_pressure_rhs_global": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.4524994185194373e-07, -9.683208190836012e-08, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], "cancellation_screen": 0.0012787147884321631, "cancellation_screen_exception": {"basis": "straight_region_interior_cap_pairs_representation_guard_departure_thrust", "consistency_bound_ratio": 3.384851335933629e-11, "member_lengths_m": [0.003, 2.9970000000000003], "member_pipe_ids": ["pipe:0", "pipe:1"], "node_ref": "node:1", "region_span_m": 3.0000000000000004, "thrust_to_bore_force_ratio": 1.111334606091418e-11}, "groups": [{"assembled_force_n": 0.0, "coefficient_sum": 0.0, "component": "Fx", "direction_component_magnitude": 0.5547001962159915, "node_ref": "node:0", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": 1.0, "kind": "poisson_eigen", "pipe_id": "pipe:0", "region_id": "region:P"}, {"coefficient": -1.0, "kind": "terminal_cap", "pipe_id": "pipe:0", "region_id": "region:P"}]}, {"assembled_force_n": 0.0, "coefficient_sum": 0.0, "component": "Fy", "direction_component_magnitude": 0.8320502943440021, "node_ref": "node:0", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": 1.0, "kind": "poisson_eigen", "pipe_id": "pipe:0", "region_id": "region:P"}, {"coefficient": -1.0, "kind": "terminal_cap", "pipe_id": "pipe:0", "region_id": "region:P"}]}, {"assembled_force_n": -8713.210306884877, "coefficient_sum": -1.0, "component": "Fx", "direction_component_magnitude": 0.5547001962159915, "node_ref": "node:1", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": -1.0, "kind": "poisson_eigen", "pipe_id": "pipe:0", "region_id": "region:P"}]}, {"assembled_force_n": 8713.210307030127, "coefficient_sum": 1.0, "component": "Fx", "direction_component_magnitude": 0.5547001962252384, "node_ref": "node:1", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": 1.0, "kind": "poisson_eigen", "pipe_id": "pipe:1", "region_id": "region:P"}]}, {"assembled_force_n": 13069.815460544876, "coefficient_sum": 1.0, "component": "Fy", "direction_component_magnitude": 0.8320502943378375, "node_ref": "node:1", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": 1.0, "kind": "poisson_eigen", "pipe_id": "pipe:1", "region_id": "region:P"}]}, {"assembled_force_n": -13069.815460641708, "coefficient_sum": -1.0, "component": "Fy", "direction_component_magnitude": 0.8320502943440021, "node_ref": "node:1", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": -1.0, "kind": "poisson_eigen", "pipe_id": "pipe:0", "region_id": "region:P"}]}, {"assembled_force_n": 0.0, "coefficient_sum": 0.0, "component": "Fx", "direction_component_magnitude": 0.5547001962252384, "node_ref": "node:2", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": -1.0, "kind": "poisson_eigen", "pipe_id": "pipe:1", "region_id": "region:P"}, {"coefficient": 1.0, "kind": "terminal_cap", "pipe_id": "pipe:1", "region_id": "region:P"}]}, {"assembled_force_n": 0.0, "coefficient_sum": 0.0, "component": "Fy", "direction_component_magnitude": 0.8320502943378375, "node_ref": "node:2", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": -1.0, "kind": "poisson_eigen", "pipe_id": "pipe:1", "region_id": "region:P"}, {"coefficient": 1.0, "kind": "terminal_cap", "pipe_id": "pipe:1", "region_id": "region:P"}]}]},
             {"region_id": "region:P", "load_case_id": "case:P", "member_pipe_ids": ["pipe:0", "pipe:1"], "geometry": [], "terminals": [{"node_ref": "node:0", "closure_transfer": "transfers_to_wall"}, {"node_ref": "node:2", "closure_transfer": "transfers_to_wall"}], "applied_loads": [{"pipe_id": "pipe:0", "local_x_global": [0.5547001962159915, 0.8320502943440021, 0.0]}, {"pipe_id": "pipe:1", "local_x_global": [0.5547001962252384, 0.8320502943378375, 0.0]}]}),
}


def _admitted(label: str) -> tuple[dict, list[dict]]:
    rhs, region = deepcopy(ASSEMBLIES[label])
    return rhs, [region]


def _up(value: float) -> float:
    return math.nextafter(value, math.inf)


def _refused(rhs: dict, regions: list[dict], code: str, elongation: bool | None = False) -> None:
    with pytest.raises(ValueError) as refusal:
        rhs_screen(rhs, CASE, elongation, regions)
    assert str(refusal.value).endswith(": " + code), str(refusal.value)


def _interior(rhs: dict) -> list[dict]:
    return [g for g in rhs["groups"] if g["node_ref"] == rhs["cancellation_screen_exception"]["node_ref"]]


def _node_dof(rhs: dict) -> int:
    return 6 * rhs["node_order"].index(rhs["cancellation_screen_exception"]["node_ref"])


@pytest.mark.parametrize("label", sorted(ASSEMBLIES))
def test_the_engine_s_admitted_assembly_is_read(label: str) -> None:
    rhs, regions = _admitted(label)
    rhs_screen(rhs, CASE, False, regions)


def _same_sign(rhs: dict, regions: list[dict]) -> None:
    for group in _interior(rhs):
        if group["assembled_force_n"] < 0:
            group["assembled_force_n"] *= -1
            group["coefficient_sum"] *= -1
            for term in group["terms"]:
                term["coefficient"] *= -1


def _zero_vector(rhs: dict, regions: list[dict]) -> None:
    rhs["assembled_pressure_rhs_global"] = [0.0] * len(rhs["assembled_pressure_rhs_global"])


def _one_group(rhs: dict, regions: list[dict]) -> None:
    group = _interior(rhs)[0]
    group["assembled_force_n"] = _up(group["assembled_force_n"])


def _vector_only(rhs: dict, regions: list[dict]) -> None:
    i = _node_dof(rhs)
    value = rhs["assembled_pressure_rhs_global"][i]
    rhs["assembled_pressure_rhs_global"][i] = _up(value) if value else 1e-300


def _coefficient_flip(rhs: dict, regions: list[dict]) -> None:
    _interior(rhs)[0]["terms"][0]["coefficient"] *= -1


def _coefficient_flip_carried(rhs: dict, regions: list[dict]) -> None:
    """The same flip carried through the group's sum, its value and the vector."""
    group = _interior(rhs)[0]
    group["terms"][0]["coefficient"] *= -1
    group["coefficient_sum"] *= -1
    group["assembled_force_n"] *= -1
    axis = ["Fx", "Fy", "Fz"].index(group["component"])
    rhs["assembled_pressure_rhs_global"][_node_dof(rhs) + axis] = math.fsum(
        g["assembled_force_n"] for g in rhs["groups"] if g["node_ref"] == group["node_ref"] and g["component"] == group["component"])


@pytest.mark.parametrize("label", sorted(ASSEMBLIES))
@pytest.mark.parametrize(("name", "change", "codes"), [
    ("no record beside a failed screen", lambda rhs, regions: rhs.pop("cancellation_screen_exception"), {"SOURCE_PHYSICS_RHS_SCREEN"}),
    ("the former basis", lambda rhs, regions: rhs["cancellation_screen_exception"].update(basis="straight_region_interior_cap_pairs_without_pressure_elongation"), {"SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION"}),
    ("the former record", lambda rhs, regions: rhs.update(cancellation_screen_exception={"basis": "straight_region_interior_cap_pairs_without_pressure_elongation", "residual_to_source_ratio": 0.0, "residual_to_source_ratio_limit": 1e-9}), {"SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION"}),
    ("the measured ratio one ulp off", lambda rhs, regions: rhs["cancellation_screen_exception"].update(thrust_to_bore_force_ratio=_up(rhs["cancellation_screen_exception"]["thrust_to_bore_force_ratio"])), {"SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_RATIO"}),
    ("the bound one ulp off", lambda rhs, regions: rhs["cancellation_screen_exception"].update(consistency_bound_ratio=_up(rhs["cancellation_screen_exception"]["consistency_bound_ratio"])), {"SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_BOUND"}),
    ("a span a millionth of the record's", lambda rhs, regions: rhs["cancellation_screen_exception"].update(region_span_m=rhs["cancellation_screen_exception"]["region_span_m"] * 1e-6), {"SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_BOUND"}),
    ("another node", lambda rhs, regions: rhs["cancellation_screen_exception"].update(node_ref=rhs["node_order"][0]), {"SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_NODE"}),
    ("the members swapped", lambda rhs, regions: rhs["cancellation_screen_exception"].update(member_pipe_ids=rhs["cancellation_screen_exception"]["member_pipe_ids"][::-1]), {"SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_NODE"}),
    ("same-sign interior groups", _same_sign, {"SOURCE_PHYSICS_RHS_ASSEMBLED_SUM"}),
    ("a zeroed assembled vector", _zero_vector, {"SOURCE_PHYSICS_RHS_ASSEMBLED_SUM"}),
    ("one group's force, the vector left alone", _one_group, {"SOURCE_PHYSICS_RHS_GROUP_VALUE"}),
    ("the vector, the groups left alone", _vector_only, {"SOURCE_PHYSICS_RHS_ASSEMBLED_SUM"}),
    ("a coefficient sign flipped", _coefficient_flip, {"SOURCE_PHYSICS_RHS_GROUP_COEFFICIENT"}),
    ("a coefficient sign flipped and carried through", _coefficient_flip_carried, {"SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_DIRECTION", "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_OPPOSITION"}),
    ("a direction component not the member's", lambda rhs, regions: regions[0]["applied_loads"][1].update(local_x_global=[_up(regions[0]["applied_loads"][1]["local_x_global"][0])] + regions[0]["applied_loads"][1]["local_x_global"][1:]), {"SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_DIRECTION"}),
    ("a record beside a passing screen", lambda rhs, regions: rhs.update(cancellation_screen=1e-12), {"SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_BESIDE_PASSING_SCREEN"}),
    ("another failed screen", lambda rhs, regions: rhs.update(cancellation_screen=_up(rhs["cancellation_screen"])), {"SOURCE_PHYSICS_RHS_SCREEN_RECOMPUTED"}),
    ("a terminal that does not transfer", lambda rhs, regions: regions[0]["terminals"][1].update(closure_transfer="separately_supported_or_compensated"), {"SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY"}),
    ("a kink remainder", lambda rhs, regions: _interior(rhs)[0]["terms"][0].update(kind="kink_remainder"), {"SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY"}),
    ("a second region at the interior node", lambda rhs, regions: _interior(rhs)[1]["terms"][0].update(region_id="region:other"), {"SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY"}),
    ("a junction of regions at a terminal", lambda rhs, regions: regions.append({"region_id": "region:other", "terminals": [{"node_ref": "node:2", "closure_transfer": "transfers_to_wall"}], "geometry": []}), {"SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY"}),
    ("a connector member", lambda rhs, regions: regions[0].update(member_connector_ids=["component:joint"]), {"SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY"}),
    ("a realized arc", lambda rhs, regions: regions[0]["geometry"].append({"pipe_id": "pipe:1", "member_kind": "realized_arc"}), {"SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY"}),
])
def test_a_record_that_the_groups_do_not_bear_out_is_refused_by_name(label: str, name: str, change, codes: set[str]) -> None:
    rhs, regions = _admitted(label)
    change(rhs, regions)
    with pytest.raises(ValueError) as refusal:
        rhs_screen(rhs, CASE, False, regions)
    assert str(refusal.value).split(": ")[-1] in codes, (name, str(refusal.value))


@pytest.mark.parametrize("label", sorted(ASSEMBLIES))
def test_a_case_that_includes_pressure_elongation_is_outside_the_family(label: str) -> None:
    rhs, regions = _admitted(label)
    _refused(rhs, regions, "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY", elongation=True)


def test_a_passing_screen_stands_alone_and_its_vector_is_still_the_sum_of_its_groups() -> None:
    rhs, regions = _admitted("one ulp")
    rhs.pop("cancellation_screen_exception")
    rhs["cancellation_screen"] = 0.0
    rhs_screen(deepcopy(rhs), CASE, False, regions)
    # Without the exception too, the vector must be the sum of its groups (the hole closed on both paths).
    changed = deepcopy(rhs)
    changed["assembled_pressure_rhs_global"][6] = 0.0
    _refused(changed, regions, "SOURCE_PHYSICS_RHS_ASSEMBLED_SUM")
    rhs["cancellation_screen"] = None
    _refused(rhs, regions, "SOURCE_PHYSICS_RHS_SCREEN")
