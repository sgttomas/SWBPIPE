"""DEL-04-07, the cancellation-screen exception: the Python twin of the Rust reader
(``physics_evidence::exception_consistency``, ``bore_group_value`` and ``assembled_is_group_sum``; its
mutations are also run on solved engine output in ``core/product_physics/tests/pressure_grouping_limits.rs``). A
screen within its limit stands alone; a failed screen stands only with the exception's record, whose family, node,
measured ratio, consistency bound and original screen the reader re-derives from the published groups and the
case's regions, after re-forming every bore group's value and the assembled vector from the groups.

The two assemblies below are the engine's (sparse), each one region transferring at both terminals without pressure
elongation: two collinear 3 m pipes whose outside diameters are one ulp apart (the interior remainder is 3.6e-12 N), and
two skew pipes whose interior node is 1/1000 of the 3 m span from a terminal and 50 epsilon of the span off the chord,
and the second review's chain (0,0,0), (2,0,0), (3, 63 epsilon * 6, 0), (6,0,0), outside diameters one ulp apart, whose
first interior node holds the second member's y term alone because the first member's y component is exactly zero.
The original screen fails on each and each is admitted by the exception. This twin reads no model and checks
consistency only, that the published evidence agrees with itself: the directions, span, member lengths, zero components
and member ends are the producer's as published (the Rust verdict given the model binds them to its coordinates)."""
from __future__ import annotations

from copy import deepcopy
import math

import pytest

from tests.verifiers.pressure_3 import CAP_REMAINDER_WARNING_MESSAGE, cap_remainder_warnings, rhs_screen

CASE = "case:P"

ASSEMBLIES = {
    "one ulp": ({"node_order": ["node:0", "node:1", "node:2"], "assembled_pressure_rhs_global": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.637978807091713e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], "cancellation_screen": 61.35923151542565, "cancellation_screen_exception": {"basis": "straight_region_interior_cap_pairs_cap_remainder_retained", "consistency_bound_ratio": 1.9895196601282805e-13, "member_lengths_m": [3.0, 3.0], "member_pipe_ids": ["pipe:0", "pipe:1"], "node_ref": "node:1", "region_span_m": 6.0, "remainder_to_bore_force_ratio": 2.316009240048811e-16}, "groups": [{"assembled_force_n": 0.0, "coefficient_sum": 0.0, "component": "Fx", "direction_component_magnitude": 1.0, "node_ref": "node:0", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": 1.0, "kind": "poisson_eigen", "pipe_id": "pipe:0", "region_id": "region:P"}, {"coefficient": -1.0, "kind": "terminal_cap", "pipe_id": "pipe:0", "region_id": "region:P"}]}, {"assembled_force_n": -15707.963267948964, "coefficient_sum": -1.0, "component": "Fx", "direction_component_magnitude": 1.0, "node_ref": "node:1", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": -1.0, "kind": "poisson_eigen", "pipe_id": "pipe:0", "region_id": "region:P"}]}, {"assembled_force_n": 15707.963267948968, "coefficient_sum": 1.0, "component": "Fx", "direction_component_magnitude": 1.0, "node_ref": "node:1", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa999999999999a", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": 1.0, "kind": "poisson_eigen", "pipe_id": "pipe:1", "region_id": "region:P"}]}, {"assembled_force_n": 0.0, "coefficient_sum": 0.0, "component": "Fx", "direction_component_magnitude": 1.0, "node_ref": "node:2", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa999999999999a", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": -1.0, "kind": "poisson_eigen", "pipe_id": "pipe:1", "region_id": "region:P"}, {"coefficient": 1.0, "kind": "terminal_cap", "pipe_id": "pipe:1", "region_id": "region:P"}]}]},
                {"region_id": "region:P", "load_case_id": "case:P", "member_pipe_ids": ["pipe:0", "pipe:1"], "geometry": [], "terminals": [{"node_ref": "node:0", "closure_transfer": "transfers_to_wall"}, {"node_ref": "node:2", "closure_transfer": "transfers_to_wall"}], "applied_loads": [{"pipe_id": "pipe:0", "local_x_global": [1.0, 0.0, 0.0]}, {"pipe_id": "pipe:1", "local_x_global": [1.0, 0.0, 0.0]}]}),
    "axis zero": ({"node_order": ["node:0", "node:1", "node:2", "node:3"], "assembled_pressure_rhs_global": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.637978807091713e-12, 1.318414292247263e-09, 0.0, 0.0, 0.0, 0.0, 0.0, -1.7578857229963507e-09, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], "cancellation_screen": 0.12698412698412698, "cancellation_screen_exception": {"basis": "straight_region_interior_cap_pairs_cap_remainder_retained", "consistency_bound_ratio": 3.339550858072471e-13, "member_lengths_m": [1.0, 3.0], "member_pipe_ids": ["pipe:1", "pipe:2"], "node_ref": "node:2", "region_span_m": 6.0, "remainder_to_bore_force_ratio": 1.1191048088221578e-13}, "groups": [{"assembled_force_n": 0.0, "coefficient_sum": 0.0, "component": "Fx", "direction_component_magnitude": 1.0, "node_ref": "node:0", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": 1.0, "kind": "poisson_eigen", "pipe_id": "pipe:0", "region_id": "region:P"}, {"coefficient": -1.0, "kind": "terminal_cap", "pipe_id": "pipe:0", "region_id": "region:P"}]}, {"assembled_force_n": -15707.963267948964, "coefficient_sum": -1.0, "component": "Fx", "direction_component_magnitude": 1.0, "node_ref": "node:1", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": -1.0, "kind": "poisson_eigen", "pipe_id": "pipe:0", "region_id": "region:P"}]}, {"assembled_force_n": 15707.963267948968, "coefficient_sum": 1.0, "component": "Fx", "direction_component_magnitude": 1.0, "node_ref": "node:1", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa999999999999a", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": 1.0, "kind": "poisson_eigen", "pipe_id": "pipe:1", "region_id": "region:P"}]}, {"assembled_force_n": 1.318414292247263e-09, "coefficient_sum": 1.0, "component": "Fy", "direction_component_magnitude": 8.393286066166183e-14, "node_ref": "node:1", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa999999999999a", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": 1.0, "kind": "poisson_eigen", "pipe_id": "pipe:1", "region_id": "region:P"}]}, {"assembled_force_n": 0.0, "coefficient_sum": 0.0, "component": "Fx", "direction_component_magnitude": 1.0, "node_ref": "node:2", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa999999999999a", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": -1.0, "kind": "poisson_eigen", "pipe_id": "pipe:1", "region_id": "region:P"}, {"coefficient": 1.0, "kind": "poisson_eigen", "pipe_id": "pipe:2", "region_id": "region:P"}]}, {"assembled_force_n": -4.3947143074908767e-10, "coefficient_sum": -1.0, "component": "Fy", "direction_component_magnitude": 2.7977620220553945e-14, "node_ref": "node:2", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa999999999999a", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": -1.0, "kind": "poisson_eigen", "pipe_id": "pipe:2", "region_id": "region:P"}]}, {"assembled_force_n": -1.318414292247263e-09, "coefficient_sum": -1.0, "component": "Fy", "direction_component_magnitude": 8.393286066166183e-14, "node_ref": "node:2", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa999999999999a", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": -1.0, "kind": "poisson_eigen", "pipe_id": "pipe:1", "region_id": "region:P"}]}, {"assembled_force_n": 0.0, "coefficient_sum": 0.0, "component": "Fx", "direction_component_magnitude": 1.0, "node_ref": "node:3", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa999999999999a", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": -1.0, "kind": "poisson_eigen", "pipe_id": "pipe:2", "region_id": "region:P"}, {"coefficient": 1.0, "kind": "terminal_cap", "pipe_id": "pipe:2", "region_id": "region:P"}]}, {"assembled_force_n": 0.0, "coefficient_sum": 0.0, "component": "Fy", "direction_component_magnitude": 2.7977620220553945e-14, "node_ref": "node:3", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa999999999999a", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": 1.0, "kind": "poisson_eigen", "pipe_id": "pipe:2", "region_id": "region:P"}, {"coefficient": -1.0, "kind": "terminal_cap", "pipe_id": "pipe:2", "region_id": "region:P"}]}]},
                {"region_id": "region:P", "load_case_id": "case:P", "member_pipe_ids": ["pipe:0", "pipe:1", "pipe:2"], "geometry": [], "terminals": [{"node_ref": "node:0", "closure_transfer": "transfers_to_wall"}, {"node_ref": "node:3", "closure_transfer": "transfers_to_wall"}], "applied_loads": [{"pipe_id": "pipe:0", "local_x_global": [1.0, 0.0, 0.0]}, {"pipe_id": "pipe:1", "local_x_global": [1.0, 8.393286066166183e-14, 0.0]}, {"pipe_id": "pipe:2", "local_x_global": [1.0, -2.7977620220553945e-14, 0.0]}]}),
    "skew": ({"node_order": ["node:0", "node:1", "node:2"], "assembled_pressure_rhs_global": [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.4524994185194373e-07, -9.683208190836012e-08, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], "cancellation_screen": 0.0012787147884321631, "cancellation_screen_exception": {"basis": "straight_region_interior_cap_pairs_cap_remainder_retained", "consistency_bound_ratio": 3.384851335933629e-11, "member_lengths_m": [0.003, 2.9970000000000003], "member_pipe_ids": ["pipe:0", "pipe:1"], "node_ref": "node:1", "region_span_m": 3.0000000000000004, "remainder_to_bore_force_ratio": 1.111334606091418e-11}, "groups": [{"assembled_force_n": 0.0, "coefficient_sum": 0.0, "component": "Fx", "direction_component_magnitude": 0.5547001962159915, "node_ref": "node:0", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": 1.0, "kind": "poisson_eigen", "pipe_id": "pipe:0", "region_id": "region:P"}, {"coefficient": -1.0, "kind": "terminal_cap", "pipe_id": "pipe:0", "region_id": "region:P"}]}, {"assembled_force_n": 0.0, "coefficient_sum": 0.0, "component": "Fy", "direction_component_magnitude": 0.8320502943440021, "node_ref": "node:0", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": 1.0, "kind": "poisson_eigen", "pipe_id": "pipe:0", "region_id": "region:P"}, {"coefficient": -1.0, "kind": "terminal_cap", "pipe_id": "pipe:0", "region_id": "region:P"}]}, {"assembled_force_n": -8713.210306884877, "coefficient_sum": -1.0, "component": "Fx", "direction_component_magnitude": 0.5547001962159915, "node_ref": "node:1", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": -1.0, "kind": "poisson_eigen", "pipe_id": "pipe:0", "region_id": "region:P"}]}, {"assembled_force_n": 8713.210307030127, "coefficient_sum": 1.0, "component": "Fx", "direction_component_magnitude": 0.5547001962252384, "node_ref": "node:1", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": 1.0, "kind": "poisson_eigen", "pipe_id": "pipe:1", "region_id": "region:P"}]}, {"assembled_force_n": 13069.815460544876, "coefficient_sum": 1.0, "component": "Fy", "direction_component_magnitude": 0.8320502943378375, "node_ref": "node:1", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": 1.0, "kind": "poisson_eigen", "pipe_id": "pipe:1", "region_id": "region:P"}]}, {"assembled_force_n": -13069.815460641708, "coefficient_sum": -1.0, "component": "Fy", "direction_component_magnitude": 0.8320502943440021, "node_ref": "node:1", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": -1.0, "kind": "poisson_eigen", "pipe_id": "pipe:0", "region_id": "region:P"}]}, {"assembled_force_n": 0.0, "coefficient_sum": 0.0, "component": "Fx", "direction_component_magnitude": 0.5547001962252384, "node_ref": "node:2", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": -1.0, "kind": "poisson_eigen", "pipe_id": "pipe:1", "region_id": "region:P"}, {"coefficient": 1.0, "kind": "terminal_cap", "pipe_id": "pipe:1", "region_id": "region:P"}]}, {"assembled_force_n": 0.0, "coefficient_sum": 0.0, "component": "Fy", "direction_component_magnitude": 0.8320502943378375, "node_ref": "node:2", "pressure_bits": "413e848000000000", "source_inner_radius_hi_bits": "3fa9999999999999", "source_inner_radius_lo_bits": "3c40000000000000", "terms": [{"coefficient": -1.0, "kind": "poisson_eigen", "pipe_id": "pipe:1", "region_id": "region:P"}, {"coefficient": 1.0, "kind": "terminal_cap", "pipe_id": "pipe:1", "region_id": "region:P"}]}]},
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
    """The same flip carried through the group's sum, its value and the vector, on a group of one term (where a group
    holds two members' terms, negating its sum is not that flip)."""
    group = next(g for g in _interior(rhs) if len(g["terms"]) == 1)
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
    ("the measured ratio one ulp off", lambda rhs, regions: rhs["cancellation_screen_exception"].update(remainder_to_bore_force_ratio=_up(rhs["cancellation_screen_exception"]["remainder_to_bore_force_ratio"])), {"SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_RATIO"}),
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


def test_a_passing_screen_stands_alone() -> None:
    rhs, regions = _admitted("one ulp")
    rhs.pop("cancellation_screen_exception")
    rhs["cancellation_screen"] = 0.0
    rhs_screen(deepcopy(rhs), CASE, False, regions)
    rhs["cancellation_screen"] = None
    _refused(rhs, regions, "SOURCE_PHYSICS_RHS_SCREEN")


def _document(label: str) -> dict:
    """A source holding one admitted case and its warning, as the producer publishes them (Rust
    ``pressure_runtime::cap_remainder_warning``): [case, region, the record's node]."""
    rhs, regions = _admitted(label)
    node = rhs["cancellation_screen_exception"]["node_ref"]
    warning = {"id": "diagnostic:pressure-runtime:invented:PRESSURE_REGION_CAP_REMAINDER_RETAINED", "code": "PRESSURE_REGION_CAP_REMAINDER_RETAINED",
               "severity": "warning", "message": CAP_REMAINDER_WARNING_MESSAGE, "source": "core/product_physics/src/pressure_runtime.rs",
               "affected_refs": [CASE, regions[0]["region_id"], node]}
    return {"contract_evidence": {"exact_cases": [{"load_case_id": CASE, "pressure_rhs_assembly": rhs}]}, "diagnostics": [warning]}


def _warning_refused(document: dict, code: str) -> None:
    with pytest.raises(ValueError) as refusal:
        cap_remainder_warnings(document)
    assert str(refusal.value).endswith(": " + code), str(refusal.value)


@pytest.mark.parametrize("label", sorted(ASSEMBLIES))
def test_the_warning_stands_exactly_with_the_record(label: str) -> None:
    cap_remainder_warnings(_document(label))
    # A record without its warning.
    document = _document(label)
    document["diagnostics"] = []
    _warning_refused(document, "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_WARNING_MISSING")
    # A warning without a record (the original screen admitted the case).
    document = _document(label)
    document["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"].pop("cancellation_screen_exception")
    _warning_refused(document, "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_WARNING_UNRECORDED")
    # A changed message.
    document = _document(label)
    document["diagnostics"][0]["message"] = CAP_REMAINDER_WARNING_MESSAGE.replace("not establish", "establish")
    _warning_refused(document, "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_WARNING")
    # A wrong node.
    document = _document(label)
    rhs = document["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]
    document["diagnostics"][0]["affected_refs"][2] = rhs["node_order"][0]
    _warning_refused(document, "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_WARNING")


def _scale(rhs: dict, regions: list[dict], factor: float, loads: bool, members: slice = slice(None)) -> None:
    """Published directions scaled by ``factor`` (the second review's forgery for 1/2), and, with ``loads``, every
    group's magnitude and value, the vector and the measured ratio with them."""
    for load in regions[0]["applied_loads"][members]:
        load["local_x_global"] = [x * factor for x in load["local_x_global"]]
    if loads:
        for group in rhs["groups"]:
            group["direction_component_magnitude"] *= factor
            group["assembled_force_n"] *= factor
        rhs["assembled_pressure_rhs_global"] = [x * factor for x in rhs["assembled_pressure_rhs_global"]]
        rhs["cancellation_screen_exception"]["remainder_to_bore_force_ratio"] *= factor


def _reverse(rhs: dict, regions: list[dict], loads: bool) -> None:
    """Published directions negated, and, with ``loads``, every coefficient, sum, value and the vector with them."""
    _scale(rhs, regions, -1.0, False)
    if loads:
        for group in rhs["groups"]:
            group["coefficient_sum"] *= -1
            group["assembled_force_n"] *= -1
            for term in group["terms"]:
                term["coefficient"] *= -1
        rhs["assembled_pressure_rhs_global"] = [-x for x in rhs["assembled_pressure_rhs_global"]]


NOT_UNIT = "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_DIRECTION_NOT_UNIT"


@pytest.mark.parametrize("label", sorted(ASSEMBLIES))
@pytest.mark.parametrize(("name", "change", "codes"), [
    ("halved with the loads (the second review's forgery)", lambda rhs, regions: _scale(rhs, regions, 0.5, True), {NOT_UNIT}),
    ("halved, the loads left alone", lambda rhs, regions: _scale(rhs, regions, 0.5, False), {NOT_UNIT}),
    ("doubled with the loads", lambda rhs, regions: _scale(rhs, regions, 2.0, True), {NOT_UNIT, "SOURCE_PHYSICS_RHS_DIRECTION"}),
    ("scaled by 1 - 16 epsilon", lambda rhs, regions: _scale(rhs, regions, 1 - 16 * 2.0 ** -52, False), {NOT_UNIT}),
    ("scaled by 0.999 with the loads", lambda rhs, regions: _scale(rhs, regions, 0.999, True), {NOT_UNIT, "SOURCE_PHYSICS_RHS_ASSEMBLED_SUM", "SOURCE_PHYSICS_RHS_GROUP_VALUE"}),
    ("one member only, halved", lambda rhs, regions: _scale(rhs, regions, 0.5, False, slice(1, 2)), {NOT_UNIT}),
    ("one component altered far from unit", lambda rhs, regions: regions[0]["applied_loads"][0]["local_x_global"].__setitem__(2, 0.25), {NOT_UNIT}),
])
def test_a_direction_that_is_not_unit_is_inconsistent(label: str, name: str, change, codes: set[str]) -> None:
    rhs, regions = _admitted(label)
    change(rhs, regions)
    with pytest.raises(ValueError) as refusal:
        rhs_screen(rhs, CASE, False, regions)
    assert str(refusal.value).split(": ")[-1] in codes, (name, str(refusal.value))


@pytest.mark.parametrize("label", sorted(ASSEMBLIES))
@pytest.mark.parametrize("loads", [True, False])
def test_a_reversal_is_consistent_without_the_model(label: str, loads: bool) -> None:
    """Every direction negated, with or without the loads, leaves the published evidence agreeing with itself: unit
    length and the published groups cannot tell a member's direction from its reverse. This twin claims consistency
    only; the Rust verdict given the model refuses both by name (SOURCE_PHYSICS_MODEL_EXCEPTION_DIRECTION,
    ``pressure_grouping_limits.rs``, forged_directions_...)."""
    rhs, regions = _admitted(label)
    _reverse(rhs, regions, loads)
    rhs_screen(rhs, CASE, False, regions)


@pytest.mark.parametrize("label", sorted(ASSEMBLIES))
def test_an_absent_term_needs_a_published_zero_component(label: str) -> None:
    """Every published member direction of these runs has a zero z component and no z term at the interior node.
    Publishing a nonzero (still unit) z component there leaves an absent term whose partner component is not zero."""
    rhs, regions = _admitted(label)
    regions[0]["applied_loads"][0]["local_x_global"][2] = 1e-20
    _refused(rhs, regions, "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY")


def test_the_axis_zero_chain_holds_a_lone_term_and_is_read() -> None:
    """At node:1 the first member's y component is published exactly zero and only the second member's y term
    stands; the reader admits it."""
    rhs, regions = _admitted("axis zero")
    lone = [g for g in rhs["groups"] if g["node_ref"] == "node:1" and g["component"] == "Fy"]
    assert len(lone) == 1 and [t["pipe_id"] for t in lone[0]["terms"]] == ["pipe:1"]
    assert regions[0]["applied_loads"][0]["local_x_global"][1] == 0.0
    rhs_screen(rhs, CASE, False, regions)


def test_an_omitted_nonzero_term_is_refused() -> None:
    """The second member's y group at node:1 removed and the vector made its sum: the absent term's member publishes
    a nonzero y component, so the evidence is inconsistent."""
    rhs, regions = _admitted("axis zero")
    rhs["groups"] = [g for g in rhs["groups"] if not (g["node_ref"] == "node:1" and g["component"] == "Fy")]
    rhs["assembled_pressure_rhs_global"][6 + 1] = 0.0
    _refused(rhs, regions, "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY")


def test_a_lone_term_whose_partner_publishes_a_nonzero_component_is_refused() -> None:
    rhs, regions = _admitted("axis zero")
    regions[0]["applied_loads"][0]["local_x_global"][1] = 1e-20
    _refused(rhs, regions, "SOURCE_PHYSICS_RHS_SCREEN_EXCEPTION_FAMILY")


@pytest.mark.parametrize(("name", "change"), [
    ("a term of a third member on the lone-term axis", lambda rhs: next(g for g in rhs["groups"] if g["node_ref"] == "node:1" and g["component"] == "Fy")["terms"].append({"coefficient": 0.0, "kind": "poisson_eigen", "pipe_id": "pipe:2", "region_id": "region:P"})),
    ("the lone term a kink remainder", lambda rhs: next(g for g in rhs["groups"] if g["node_ref"] == "node:1" and g["component"] == "Fy")["terms"][0].update(kind="kink_remainder")),
    ("the lone term a terminal cap", lambda rhs: next(g for g in rhs["groups"] if g["node_ref"] == "node:1" and g["component"] == "Fy")["terms"][0].update(kind="terminal_cap", coefficient=-1.0)),
    ("the lone term another region's", lambda rhs: next(g for g in rhs["groups"] if g["node_ref"] == "node:1" and g["component"] == "Fy")["terms"][0].update(region_id="region:other")),
])
def test_no_foreign_term_stands_alone_on_a_zero_axis(name: str, change) -> None:
    rhs, regions = _admitted("axis zero")
    change(rhs)
    with pytest.raises(ValueError) as refusal:
        rhs_screen(rhs, CASE, False, regions)
    assert str(refusal.value).split(": ")[-1].startswith("SOURCE_PHYSICS_"), (name, str(refusal.value))
