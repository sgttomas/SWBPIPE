"""T4-U3 (S14): the test-only pressure-2 verifier admits objective connector
records and rows over the shared connector corpus (the Rust reader's
``tests/connector_contract.rs`` reads the same bytes; the Rust reader is the
only product reader), refuses each broken binding by name, and the retained-source
route (pressure-source-1) refuses connector evidence. (The retired v2 readers'
refusal of connector evidence went with them in B7b.)
"""
from copy import deepcopy
from fractions import Fraction
import json
from pathlib import Path

import pytest
from core.analysis_runs import rust_reader
from tests.verifiers import physics_source
from tests.verifiers.source_blocks import domain_hash
from tests.verifiers.pressure_2 import (
    CONTRACT_ID as PRESSURE_ID, validate_exact_pressure_evidence, validate_pressure_evidence, verify_source,
)

PROJECT = Path(__file__).resolve().parents[1]
CORPUS = json.loads((PROJECT / "fixtures/results/pressure_v3_connector_reader_corpus.json").read_text())
CASES = [(case["label"], case["envelope"]) for case in CORPUS["cases"]]


def tied(envelope):
    return envelope["contract_evidence"]["connector"][0]["hardware"] == "explicit_axial_rods_v1"


# T4-U5b: the two untied entries keep the T4-U3 mutations; the five tied entries (t4_u5b AS-F and MO-F, and the three
# derived near-perpendicular or skewed rods of round 4) take the rod mutations below.
UNTIED_CASES = [(label, envelope) for label, envelope in CASES if not tied(envelope)]
TIED_CASES = [(label, envelope) for label, envelope in CASES if tied(envelope)]


def load_state(envelope):
    return "load_reference_states" in envelope["contract_evidence"]


def first_connector_row(envelope):
    return next(r for r in envelope["results"] if r["kind"].startswith("connector_"))


def u5a_remote_control():
    """Result-reader control only; no new model version is admitted here."""
    envelope = deepcopy(CASES[0][1])
    envelope["contract_evidence"]["connector"][0].update(pressure_model="constant_effective_area_reference_v1", effective_area_m2=0.004,
        pressure_basis="internal_differential_zero_external_v1", pressure_source_reference="invented U5a reader control")
    generalized = [row for row in envelope["results"] if row["kind"] == "connector_generalized_force_v1" and row["metadata"]["component"] == "gt_x"]
    for generalized_row in generalized:
        for kind, component, value in [("connector_pressure_force_v1", "pe_x", 0.0), ("connector_effective_axial_force_v1", "s_x", generalized_row["value"])]:
            row = deepcopy(generalized_row)
            row.update(id=f'{row["id"]}:{component}', kind=kind, value=value)
            row["metadata"]["component"] = component
            envelope["results"].append(row)
    return envelope


def test_u5a_remote_reader_control_and_zero_pressure_tamper():
    import math
    envelope = u5a_remote_control()
    validate_pressure_evidence(envelope)
    forged = deepcopy(envelope)
    next(row for row in forged["results"] if row["kind"] == "connector_pressure_force_v1")["value"] = math.ulp(0.0)
    with pytest.raises(ValueError, match="joint pressure row binding"):
        validate_pressure_evidence(forged)
    for field in ["effective_area_m2", "pressure_basis", "pressure_source_reference"]:
        missing = deepcopy(envelope)
        del missing["contract_evidence"]["connector"][0][field]
        with pytest.raises(ValueError):
            validate_pressure_evidence(missing)


def test_u5a_projection_guard_stays_finite_and_does_not_widen_direct_product():
    import math
    from tests.verifiers.pressure_2 import _joint_projection
    assert _joint_projection(0.0, 0.0, 1e308, 1e308, 0)
    assert not _joint_projection(1e300, 0.0, 1e308, 1e308, 0)
    tiny = math.ulp(0.0)
    assert _joint_projection(0.0, tiny, tiny, 0.0, 1)
    assert not _joint_projection(2 * tiny, 0.0, tiny, 0.0, 1)


def test_u5a_participating_producer_control_and_tampered_edges_and_groups():
    envelope = json.loads((PROJECT / "fixtures/results/pressure_v3_joint_reader_control.json").read_text())["envelope"]
    from tests.schema_validation import schema_for_definition, validate_instance, validate_schema_document
    schema = json.loads((PROJECT / "schemas/results_pressure_2.v0.3.schema.json").read_text())
    validate_schema_document(schema)
    validate_instance(schema_for_definition(schema, "PressureContractEvidence"), envelope["contract_evidence"])
    validate_pressure_evidence(envelope)
    for field, value in [("connector_id", "component:wrong"), ("face_i_pipe_id", "pipe:wrong"), ("effective_area_m2", 0.005), ("p_pa", 1.0), ("area_difference_force_n", [0.0, 0.0]), ("attachment_pair_global_n", [[0.0] * 3, [0.0] * 3])]:
        forged = deepcopy(envelope)
        forged["contract_evidence"]["pressure"][0]["joint_edges"][0][field] = value
        with pytest.raises(ValueError):
            validate_pressure_evidence(forged)
    for field in ["effective_area_bits", "pressure_bits", "direction_component_magnitude"]:
        forged = deepcopy(envelope)
        group = next(group for group in forged["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"] if "effective_area_bits" in group)
        group[field] = 0.5 if field == "direction_component_magnitude" else "0000000000000000"
        with pytest.raises(ValueError):
            validate_pressure_evidence(forged)
    forged = deepcopy(envelope)
    group = next(group for group in forged["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"] if "effective_area_bits" in group)
    group["terms"].append(deepcopy(group["terms"][0]))
    group["coefficient_sum"] *= 2
    group["assembled_force_n"] *= 2
    with pytest.raises(ValueError, match="duplicate joint RHS source"):
        validate_pressure_evidence(forged)


def record(envelope):
    return envelope["contract_evidence"]["connector"][0]


def _push_duplicate_row(envelope):
    row = deepcopy(first_connector_row(envelope))
    row["id"] = "result:connector:duplicate"
    envelope["results"].append(row)


def _push_replaced_span_row(envelope):
    row = deepcopy(next(r for r in envelope["results"] if r["kind"] == "element_local_axial_force"))
    row["id"], row["entity_ref"] = "result:replaced-span", "pipe:P-130"
    envelope["results"].append(row)


def _remove_connector_row(envelope):
    envelope["results"].remove(first_connector_row(envelope))


# (label, mutation, detail): the same mutations as the Rust reader's test.
MUTATIONS = [
    ("record removed", lambda e: e["contract_evidence"].__setitem__("connector", []), "unbound connector row"),
    ("record duplicated", lambda e: e["contract_evidence"]["connector"].append(deepcopy(record(e))), "duplicate connector record"),
    ("record extra key", lambda e: record(e).__setitem__("extra", 1), "connector record shape"),
    ("record hardware tied", lambda e: record(e).__setitem__("hardware", "tied"), "connector record law"),
    ("record q_ref short", lambda e: record(e).__setitem__("q_ref", [0.0, 0.0]), "connector record frame"),
    ("record rotation scale", lambda e: record(e)["work_matrix"].__setitem__("rotation_scale_rad", 2.0), "connector work matrix"),
    ("replaced span published", lambda e: record(e).__setitem__("replaced_pipe_id", "pipe:P-120"), "connector replaced span published"),
    ("row removed", _remove_connector_row, "connector row coverage"),
    ("row unbound", lambda e: first_connector_row(e).__setitem__("entity_ref", "component:C-999"), "unbound connector row"),
    ("row unknown kind", lambda e: first_connector_row(e).__setitem__("kind", "connector_energy_v1"), "unknown connector kind"),
    ("row unit", lambda e: first_connector_row(e).__setitem__("unit", "mm"), "connector row semantics"),
    ("row basis", lambda e: first_connector_row(e)["metadata"].__setitem__("basis", "objective_connector_v1;replaces_span=pipe:P-120;symmetric_midpoint_small_rotation_v1"), "connector row semantics"),
    ("row sign", lambda e: first_connector_row(e)["metadata"].__setitem__("sign_convention", "global end action on the connector at its node (node on element), f = B^T g; the connector acts on its node with -f"), "connector row semantics"),
    ("row duplicated", _push_duplicate_row, "duplicate connector row"),
    ("replaced span row", _push_replaced_span_row, "connector replaced span published"),
]


def test_corpus_shape():
    assert len(CASES) == 7 and len(UNTIED_CASES) == 2 and len(TIED_CASES) == 5
    assert [label for label, e in TIED_CASES if all(r["axial_share"] == 0.0 for r in record(e)["tie_rods"])] == [
        "T4-U5b MO-F single-rod control, Q = I, 1e-17 skew sparse_interactive"]
    assert {load_state(e) for _, e in UNTIED_CASES} == {True, False}
    assert all(len(e["contract_evidence"]["connector"]) == 1 for _, e in CASES)
    assert {record(e)["pressure_model"] for _, e in TIED_CASES} == {"unpressurized", "constant_effective_area_reference_v1"}


@pytest.mark.parametrize("label,envelope", CASES)
def test_verifier_and_dispatch_admit_the_connector_corpus(label, envelope):
    assert envelope["producer"]["semantic_contract_id"] == PRESSURE_ID
    validate_pressure_evidence(deepcopy(envelope))
    validate_exact_pressure_evidence(deepcopy(envelope))
    verify_source(deepcopy(envelope))
    verify_source(deepcopy(envelope), raw=False)


@pytest.mark.parametrize("label,envelope", UNTIED_CASES)
@pytest.mark.parametrize("what,mutate,detail", MUTATIONS, ids=[m[0] for m in MUTATIONS])
def test_broken_connector_bindings_are_refused_by_name(label, envelope, what, mutate, detail):
    broken = deepcopy(envelope)
    mutate(broken)
    with pytest.raises(ValueError) as refused:
        validate_pressure_evidence(broken)
    assert str(refused.value).endswith(f"SOURCE_PHYSICS_EVIDENCE_INVALID: {detail}"), str(refused.value)


def _rod_rows(envelope):
    return [r for r in envelope["results"] if r["kind"] == "connector_tie_rod_force_v1"]


def _row(envelope, kind):
    return next(r for r in envelope["results"] if r["kind"] == kind)


def _rows_without_record(envelope):
    record(envelope)["hardware"] = "untied"
    del record(envelope)["tie_rods"]


def _rename_rod_row(envelope):
    row = _rod_rows(envelope)[0]
    row["id"] += "x"


def _extra_rod_row(envelope):
    row = deepcopy(_rod_rows(envelope)[0])
    row["id"] = "result:connector:component-C-J:tie-rod-force:tie_rod:rod-extra:t_r"
    envelope["results"].append(row)


def _shift(row, delta):
    row["value"] += delta


def _binding_rod(e):
    """The rod whose force enters s_x most (largest |axial_share|): a rod with axial_share exactly 0 adds nothing to
    s_x, so s_x's binding cannot see its force value or its share being scaled; the value tampers target this rod."""
    rods = record(e)["tie_rods"]
    return max(range(len(rods)), key=lambda i: abs(rods[i]["axial_share"]))


def _scale_binding_share(e):
    rod = record(e)["tie_rods"][_binding_rod(e)]
    rod["axial_share"] = rod["axial_share"] * (1 - 2.0 ** -52)


def _shift_binding_force(e):
    rod_id = record(e)["tie_rods"][_binding_rod(e)]["id"].replace(":", "-")
    row = next(r for r in _rod_rows(e) if r["id"].endswith(f":tie_rod:{rod_id}:t_r"))
    _shift(row, 1.0)


# the same rod mutations as the Rust reader's test.
ROD_MUTATIONS = [
    ("rod record without rows", lambda e: e.__setitem__("results", [r for r in e["results"] if r["kind"] != "connector_tie_rod_force_v1"]), "connector tie-rod rows missing"),
    ("rod rows without a record", _rows_without_record, "connector tie-rod row unbound"),
    ("duplicate rod", lambda e: record(e)["tie_rods"].append(deepcopy(record(e)["tie_rods"][0])), "connector tie-rod duplicate"),
    ("untied record with tie_rods", lambda e: record(e).__setitem__("hardware", "untied"), "connector untied record with tie rods"),
    ("rod entry extra key", lambda e: record(e)["tie_rods"][0].__setitem__("extra", 1), "connector tie-rod record"),
    ("rod law", lambda e: record(e)["tie_rods"][0].__setitem__("law", "ideal_inextensible"), "connector tie-rod record"),
    ("rod stiffness zero", lambda e: record(e)["tie_rods"][0].__setitem__("stiffness_n_per_m", 0.0), "connector tie-rod record"),
    ("rod direction not unit", lambda e: record(e)["tie_rods"][0].__setitem__("direction_global", [0.5, 0.0, 0.0]), "connector tie-rod direction"),
    ("rod axial share", _scale_binding_share, "connector tie-rod axial share"),
    ("rod row renamed", _rename_rod_row, "connector tie-rod row unbound"),
    ("rod row extra", _extra_rod_row, "connector tie-rod row unbound"),
    ("rod row frame", lambda e: _rod_rows(e)[0]["metadata"].__setitem__("coordinate_system", "global"), "connector row semantics"),
    ("rod row basis", lambda e: _rod_rows(e)[0]["metadata"].__setitem__("basis", "objective_connector_v1;symmetric_midpoint_small_rotation_v1"), "connector row semantics"),
    ("rod force changed", _shift_binding_force, "joint effective row binding"),
    ("s_x changed", lambda e: _shift(_row(e, "connector_effective_axial_force_v1"), 1e-6), "joint effective row binding"),
    ("s_x removed", lambda e: e.__setitem__("results", [r for r in e["results"] if r["kind"] != "connector_effective_axial_force_v1"]), "connector row coverage"),
]


@pytest.mark.parametrize("label,envelope", TIED_CASES)
def test_tied_entries_match_the_result_schema_and_the_hardware_rows(label, envelope):
    from tests.schema_validation import schema_for_definition, validate_instance
    schema = json.loads((PROJECT / "schemas/results_pressure_2.v0.3.schema.json").read_text())
    validate_instance(schema_for_definition(schema, "PressureContractEvidence"), envelope["contract_evidence"])
    unpressurized = record(envelope)["pressure_model"] == "unpressurized"
    count = lambda kind: sum(1 for r in envelope["results"] if r["kind"] == kind)
    assert count("connector_pressure_force_v1") == (0 if unpressurized else 1)
    assert count("connector_effective_axial_force_v1") == 1
    assert count("connector_tie_rod_force_v1") == len(record(envelope)["tie_rods"])
    # Round 4: axial_share is RN(n.Q_x), which can exceed 1 by an ulp or two for a parallel rod in a rotated frame;
    # the schema bounds only its type (finite by JSON), never [-1, 1].
    beyond = deepcopy(envelope)
    record(beyond)["tie_rods"][0]["axial_share"] = 1.0000000000000004
    validate_instance(schema_for_definition(schema, "PressureContractEvidence"), beyond["contract_evidence"])
    untied = deepcopy(envelope)
    record(untied)["hardware"] = "untied"
    with pytest.raises(Exception):
        validate_instance(schema_for_definition(schema, "PressureContractEvidence"), untied["contract_evidence"])


@pytest.mark.parametrize("label,envelope", TIED_CASES)
@pytest.mark.parametrize("what,mutate,detail", ROD_MUTATIONS, ids=[m[0] for m in ROD_MUTATIONS])
def test_broken_tie_rod_bindings_are_refused_by_name(label, envelope, what, mutate, detail):
    broken = deepcopy(envelope)
    mutate(broken)
    if what in ("rod axial share", "rod force changed") and all(r["axial_share"] == 0.0 for r in record(envelope)["tie_rods"]):
        # Codex's single-rod control: its only rod's axial_share is exactly 0, so the rod adds nothing to s_x and s_x's
        # binding cannot see its force or share scaled. Its s_x operand is discriminated by product_physics
        # connector_readers.rs `s_x_formed_with_the_share_at_p_fails_the_binding`.
        assert "single-rod control" in label
        validate_pressure_evidence(broken)
        return
    with pytest.raises(ValueError) as refused:
        validate_pressure_evidence(broken)
    assert str(refused.value).endswith(f"SOURCE_PHYSICS_EVIDENCE_INVALID: {detail}"), str(refused.value)


def test_s_x_binding_sums_rods_exactly_with_its_coefficient_and_reserve():
    """§5.1's s_x rule (Rust `u5b_effective_axial_tests`): exact rational accumulation, (64 + n_r)eps, (n_r + 2) least
    subnormals; cancellation, subnormal edges and a 1,024-rod control."""
    import math
    from tests.verifiers.pressure_2 import _effective_axial_binding
    tiny = math.ulp(0.0)
    assert _effective_axial_binding(0.0, 553.25, 800.0, [(246.75, 1.0)])
    assert not _effective_axial_binding(1e-9, 553.25, 800.0, [(246.75, 1.0)])
    t, share = 144000.0 / 1661.0, 0.8
    g = 800.0 - t * share
    assert _effective_axial_binding(0.0, g, 800.0, [(t, share)])
    assert not _effective_axial_binding(g + t - 800.0, g, 800.0, [(t, share)])
    assert _effective_axial_binding(5 * tiny, tiny, 0.0, [(tiny, 1.0)])
    assert _effective_axial_binding(-tiny, tiny, 0.0, [(tiny, 1.0)])
    assert not _effective_axial_binding(6 * tiny, tiny, 0.0, [(tiny, 1.0)])
    assert not _effective_axial_binding(math.nan, 1.0, 0.0, [])
    rods = [(2.0 ** -54, 1.0)] * 1024
    exact = 1.0 + 2.0 ** -44
    assert _effective_axial_binding(exact, 1.0, 0.0, rods)
    tolerance = (64 + 1024) * 2.0 ** -52 * (1 + 1024 * 2.0 ** -54) + 1026 * tiny
    assert _effective_axial_binding(exact + 0.5 * tolerance, 1.0, 0.0, rods)
    assert not _effective_axial_binding(exact + 2 * tolerance, 1.0, 0.0, rods)


def test_the_axial_share_is_compared_by_bits_so_a_negative_zero_share_is_refused():
    """The Rust reader's verdicts (`u5b_effective_axial_tests::the_axial_share_is_compared_by_bits_so_a_negative_zero_
    share_is_refused`): the exact dot's binary64 rounding is compared with the share by bits, and an exactly zero dot
    rounds to +0.0, so a -0.0 share is refused. Python's `==` would admit it (the negative control)."""
    import struct
    from fractions import Fraction
    from tests.verifiers.pressure_2 import _tie_rods

    def rod(axis0, n, share):
        return _tie_rods({"connector_axes_global": [[axis0[0], 0.0, 0.0], [axis0[1], 1.0, 0.0], [axis0[2], 0.0, 1.0]],
                          "tie_rods": [{"id": "rod:a", "law": "bilateral_linear", "offset_global_m": [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
                                        "stiffness_n_per_m": 1.0e6, "reference_extension_m": 0.0, "length_m": 1.0, "direction_global": n,
                                        "axial_share": share, "source_reference": "invented signed-zero case"}]})

    def refused(axis0, n, share):
        with pytest.raises(ValueError, match="connector tie-rod axial share$"):
            rod(axis0, n, share)

    minus_one_ulp = struct.unpack("<d", struct.pack("<q", struct.unpack("<q", struct.pack("<d", 1.0))[0] - 1))[0]
    # A dot that is zero term by term: +0.0 admitted, -0.0 refused.
    assert len(rod([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 0.0)) == 1
    refused([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], -0.0)
    # Cancellation: 0.6*0.8 - 0.8*0.6 is exactly zero, rounded to +0.0.
    assert len(rod([0.8, -0.6, 0.0], [0.6, 0.8, 0.0], 0.0)) == 1
    refused([0.8, -0.6, 0.0], [0.6, 0.8, 0.0], -0.0)
    # Nonzero controls.
    assert len(rod([1.0, 0.0, 0.0], [1.0, 0.0, 0.0], 1.0)) == 1
    refused([1.0, 0.0, 0.0], [1.0, 0.0, 0.0], minus_one_ulp)
    assert len(rod([1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], -1.0)) == 1
    refused([1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], 1.0)
    # Negative control: the previous `==` comparison loses the distinction the reader now keeps.
    dot = float(sum(Fraction(a) * Fraction(b) for a, b in zip([0.6, 0.8, 0.0], [0.8, -0.6, 0.0])))
    assert dot == -0.0 and struct.pack("<d", dot) != struct.pack("<d", -0.0)


_MAX = float.fromhex("0x1.fffffffffffffp+1023")
_P = lambda k: 2.0 ** k  # noqa: E731 (exact powers of two in the normal range)
# Handcrafted full-reader fixtures (not producer output), the Rust reader's `AXIAL_SHARE_FIXTURES`
# (`connector_evidence.rs` `u5b_effective_axial_tests`; frame_kernel's `exact_sum` tests assert the producer's
# `exact_rounded_dot` gives the same bits): (label, Q_x column 0, n, the exact rational dot written out independently,
# the new reader's bits (None: outside binary64), the old Rust formula's bits (None: refused)).
AXIAL_SHARE_FIXTURES = [
    ("W2 above half an ulp", [1.0, 1.0, _P(-500)], [1.0, _P(-53), _P(-600)],
     1 + Fraction(1, 2 ** 53) + Fraction(1, 2 ** 1100), 0x3FF0000000000001, 0x3FF0000000000000),
    ("exactly half an ulp, to even", [1.0, 1.0, _P(-500)], [1.0, _P(-53), 0.0], 1 + Fraction(1, 2 ** 53), 0x3FF0000000000000, 0x3FF0000000000000),
    ("below half an ulp", [1.0, 1.0, _P(-500)], [1.0, _P(-53), -_P(-600)],
     1 + Fraction(1, 2 ** 53) - Fraction(1, 2 ** 1100), 0x3FF0000000000000, 0x3FF0000000000000),
    ("negative underflow (W1)", [1.0, 0.0, -_P(-500)], [0.0, 1.0, _P(-600)], -Fraction(1, 2 ** 1100), 0, 0),
    ("positive underflow", [1.0, 0.0, _P(-500)], [0.0, 1.0, _P(-600)], Fraction(1, 2 ** 1100), 0, 0),
    ("negative half the least subnormal, to even", [1.0, 0.0, _P(-500)], [0.0, 1.0, -_P(-575)], -Fraction(1, 2 ** 1075), 0, 0),
    ("negative, just beyond half the least subnormal", [-_P(-750), 0.0, _P(-500)], [_P(-750), 1.0, -_P(-575)],
     -Fraction(1, 2 ** 1500) - Fraction(1, 2 ** 1075), (1 << 63) | 1, 0),
    ("subnormal, exact", [1.0, 0.0, _P(-500)], [0.0, 1.0, 3 * _P(-574)], Fraction(3, 2 ** 1074), 3, None),
    ("subnormal tie 2.5*2^-1074, to even", [_P(-750), 0.0, _P(-500)], [0.0, 1.0, 5 * _P(-575)], Fraction(5, 2 ** 1075), 2, None),
    ("subnormal just above the tie", [_P(-750), 0.0, _P(-500)], [_P(-750), 1.0, 5 * _P(-575)],
     Fraction(5, 2 ** 1075) + Fraction(1, 2 ** 1500), 3, None),
    ("subnormal just below the tie", [_P(-750), 0.0, _P(-500)], [-_P(-750), 1.0, 5 * _P(-575)],
     Fraction(5, 2 ** 1075) - Fraction(1, 2 ** 1500), 2, None),
    ("negative subnormal tie, to even", [_P(-750), 0.0, _P(-500)], [0.0, 1.0, -5 * _P(-575)], -Fraction(5, 2 ** 1075), (1 << 63) | 2, None),
    ("cancelling product overflow", [_MAX, -_MAX, 0.0], [1 + _P(-52), _P(-26), 0.0],
     Fraction(_MAX) * (1 + Fraction(1, 2 ** 52) - Fraction(1, 2 ** 26)), 0x7FEFFFFFF8000001, None),
    ("unrepresentable total", [_MAX, 0.0, 0.0], [1 + _P(-52), 0.0, 0.0], Fraction(_MAX) * (1 + Fraction(1, 2 ** 52)), None, None),
    ("normal control", [0.8660254037844387, 0.5, 0.0], [0.6, 0.8, 0.0],
     Fraction(0.6) * Fraction(0.8660254037844387) + Fraction(0.8) / 2, 0x3FED6D7CF1CE9467, 0x3FED6D7CF1CE9467),
    ("exact cancellation", [0.8, -0.6, 0.0], [0.6, 0.8, 0.0], Fraction(0), 0, 0),
    ("zero term by term", [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], Fraction(0), 0, 0),
]


def _f64(bits):
    import struct
    return struct.unpack("<d", struct.pack("<Q", bits))[0]


def _rod_record(axis0, n, share, source="handcrafted exact-dot case"):
    return {"connector_axes_global": [[axis0[0], 0.0, 0.0], [axis0[1], 1.0, 0.0], [axis0[2], 0.0, 1.0]],
            "tie_rods": [{"id": "rod:a", "law": "bilateral_linear", "offset_global_m": [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
                          "stiffness_n_per_m": 1.0e6, "reference_extension_m": 0.0, "length_m": 1.0, "direction_global": n,
                          "axial_share": share, "source_reference": source}]}


@pytest.mark.parametrize("label,axis,n,exact,bits,old", AXIAL_SHARE_FIXTURES, ids=[f[0] for f in AXIAL_SHARE_FIXTURES])
def test_the_axial_share_is_the_exact_dot_rounded_once(label, axis, n, exact, bits, old):
    """The Rust reader's verdicts (`u5b_effective_axial_tests::the_axial_share_is_the_exact_dot_rounded_once`): the
    share is the exact rational dot of the binary64 operands, rounded once, with +0.0 for any result that rounds to
    zero (the producer's publication rounding), compared by bits. A total outside binary64 refuses every share."""
    import struct
    from tests.verifiers.pressure_2 import _tie_rods
    # The independent exact value: the Fraction dot of the operands is the rational written out above.
    assert sum(Fraction(a) * Fraction(b) for a, b in zip(n, axis)) == exact, label
    if bits is None:
        with pytest.raises(OverflowError):
            float(exact)
        shares = []
        refused = [_MAX, -_MAX, 0.0]
    else:
        rounded = float(exact)
        assert struct.pack("<d", rounded if rounded != 0.0 else 0.0) == struct.pack("<Q", bits), label
        shares = [_f64(bits)]
        refused = [_f64(b) for b in ([1 << 63, 1, (1 << 63) | 1] if bits == 0 else [bits - 1, bits + 1])]
        if old is not None and old != bits:
            refused.append(_f64(old))  # the old formula's share
    for share in shares:
        assert len(_tie_rods(_rod_record(axis, n, share))) == 1, label
    for share in refused:
        with pytest.raises(ValueError, match="connector tie-rod axial share$"):
            _tie_rods(_rod_record(axis, n, share))


def test_w2_and_the_unrepresentable_total_are_refused_as_the_share_by_both_full_readers():
    """W2 and the unrepresentable total on a tied corpus envelope (handcrafted mutation, not producer output): the
    verifier's `validate_pressure_evidence` and the Rust reader refuse the old formula's 1.0 for W2, and any share for a
    total outside binary64, as the axial share (the verifier no longer leaves the latter as an OverflowError)."""
    for axis0, n, share in [([1.0, 1.0, _P(-500)], [1.0, _P(-53), _P(-600)], 1.0),
                            ([_MAX, 0.0, 0.0], [1 + _P(-52), 0.0, 0.0], _MAX)]:
        envelope = deepcopy(TIED_CASES[0][1])
        connector = record(envelope)
        for k in range(3):
            connector["connector_axes_global"][k][0] = axis0[k]
        connector["tie_rods"][0].update(direction_global=n, axial_share=share)
        with pytest.raises(ValueError) as refused:
            validate_pressure_evidence(deepcopy(envelope))
        assert str(refused.value).endswith("SOURCE_PHYSICS_EVIDENCE_INVALID: connector tie-rod axial share"), str(refused.value)
        with pytest.raises(rust_reader.ReaderRefusal) as refused:
            rust_reader.validate("pressure", deepcopy(envelope))
        assert refused.value.code == "SOURCE_PHYSICS_CONNECTOR_TIE_ROD_AXIAL_SHARE", str(refused.value)


def test_the_old_reader_formula_loses_w2():
    """Negative control for W2: the old Rust formula (p = RN(a*b) plus the fma residual, after the 2^-969 guard) drops
    the product 2^-1100, which rounds to zero, and rounds the tie 1 + 2^-53 to the even 1.0; the exact dot is 1 + eps."""
    import math
    n, axis = [1.0, _P(-53), _P(-600)], [1.0, 1.0, _P(-500)]
    terms = []
    for a, b in zip(n, axis):
        p = a * b
        assert math.isfinite(p) and (p == 0.0 or abs(p) >= _P(-969))
        terms += [p, float(Fraction(a) * Fraction(b) - Fraction(p))]  # fma(a, b, -p): the residual rounded once
    assert float(sum(Fraction(t) for t in terms)) == 1.0
    assert float(sum(Fraction(a) * Fraction(b) for a, b in zip(n, axis))) == 1.0 + _P(-52)


def test_sequential_f64_sums_decide_the_direction_and_tolerance_boundaries():
    """The Rust reader's verdicts (`u5b_effective_axial_tests::sequential_f64_sums_decide_the_direction_and_tolerance_
    boundaries`, the same binary64 bits): |n|^2 and the s_x tolerance's rod terms are summed as Rust's
    `Iterator::sum::<f64>()`, sequentially from -0.0. A compensated sum (Python >= 3.12's `sum()`) decides each the
    other way."""
    import struct
    from tests.verifiers.pressure_2 import _effective_axial_binding, _tie_rods

    def f(bits):
        return struct.unpack(">d", bits.to_bytes(8, "big"))[0]

    def direction(n):
        return _tie_rods({"connector_axes_global": [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
                          "tie_rods": [{"id": "rod:a", "law": "bilateral_linear", "offset_global_m": [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
                                        "stiffness_n_per_m": 1.0e6, "reference_extension_m": 0.0, "length_m": 1.0, "direction_global": n,
                                        "axial_share": n[0], "source_reference": "invented boundary case"}]})

    # |n|^2 sequentially is 1 + 17eps, compensated 1 + 16eps: refused.
    with pytest.raises(ValueError, match="connector tie-rod direction$"):
        direction([f(0xbfee1566e2522cf2), f(0x3fcc6bcd304d6a58), f(0x3fd08d4e90b4b2ab)])
    # The other way: admitted.
    assert len(direction([f(0xbfeb5c74874966c6), f(0xbfc10ad15e97fa48), f(0x3fe009b5284e420d)])) == 1
    # Three rods: the sequential tolerance is one ulp below the compensated one, and s_x lies between: refused.
    rods = [(f(0x40829836406c80d9), f(0x3fcac156b94353c8)), (f(0x40e6dbb249249249), f(0xbfe51ce637c961ac)),
            (f(0x40d8322276276276), f(0x3fe97da8c79e5f96))]
    assert not _effective_axial_binding(f(0x3e15a9fe831de478), f(0xc0c6910000000000), f(0xc0d60cdcf3c47754), rods)
    # One ulp above: admitted.
    rods = [(f(0x4046456ed0af15bb), f(0xbfda1558c3f83d18)), (f(0x41193b1800000000), f(0xbfe607d49c6fc934)),
            (f(0x40e50d2276276276), f(0x3fa2457a67fbe500))]
    assert _effective_axial_binding(f(0x3e4227bf314ad803), f(0x40d8f69555555555), f(0xc10f6f444ec1d244), rods)
    # A JSON integer operand is binary64 before it is multiplied, as Rust's `as_f64`: 2^53 + 1 is 2^53, so Sigma = 0 and
    # s = 261 exceeds the tolerance 260: refused (an exact integer product would give Sigma = 1 and admit it).
    assert not _effective_axial_binding(261.0, 0.0, 9007199254740992.0, [(9007199254740993, 1)])
    # Integer operands that binary64 holds exactly, and an integer direction: admitted.
    assert _effective_axial_binding(0, 553, 800, [(247, 1)])
    assert len(direction([1, 0, 0])) == 1


@pytest.mark.parametrize("family,path,code", [
    # The ordinary family's closed physical shape (`connector: const []`) refuses the record before the evidence
    # check; the load/reference-state pre-pass names it first. Both readers agree (the Rust verdict below).
    ("0.3 family", "fixtures/product_preview/pressure_source/physics_source/n05-sparse_interactive.raw.json", "PHYSICS_SOURCE_PHYSICAL_SHAPE"),
    ("load/reference-state family", "fixtures/product_preview/pressure_source/load_reference_source/n05-sparse_interactive.raw.json", "CONNECTOR_UNSUPPORTED"),
])
def test_the_source_route_refuses_connector_evidence(family, path, code):
    # B7b (Rust `the_source_route_refuses_connector_evidence`): the retained-source route (pressure-source-1, both
    # source families) declines objective connectors (b7-plan J7/D7). A connector record grafted into a genuine
    # pressure-source-1 statement is refused by name, its receipt resealed so that the evidence check is reached.
    source = json.loads((PROJECT / path).read_text())
    physics_source.validate_physics_source(deepcopy(source))
    source["contract_evidence"]["connector"] = [deepcopy(CASES[0][1]["contract_evidence"]["connector"][0])]
    publication = {key: value for key, value in source.items() if key != "source_block_recovery"}
    recovery = source["source_block_recovery"]
    recovery["body"]["publication_sha256"] = domain_hash("source_blocks_publication_v1", publication)
    recovery["receipt_sha256"] = domain_hash("source_blocks_receipt_v1", recovery["body"])
    with pytest.raises(ValueError) as refused:
        physics_source.validate_physics_source(deepcopy(source))
    assert str(refused.value).endswith(code), f"{family}: {refused.value}"
    # The product path takes the Rust reader's verdict, refused by the same name.
    with pytest.raises(rust_reader.ReaderRefusal) as refused:
        rust_reader.validate("pressure-source", deepcopy(source))
    assert str(refused.value).endswith(code), f"{family}: {refused.value}"


def test_a_joint_axis_component_just_above_one_binds_exactly():
    """an authored connector axis may carry a
    component just above 1 (the frame kernel's orthonormality guard admits it); a joint-area group's magnitude binds
    exactly to that component and is not refused for exceeding 1, while a magnitude off the axis still is."""
    envelope = deepcopy(json.loads((PROJECT / "fixtures/results/pressure_v3_joint_reader_control.json").read_text())["envelope"])
    group = next(g for g in envelope["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"] if "effective_area_bits" in g)
    axis = ["Fx", "Fy", "Fz"].index(group["component"])
    record = next(r for r in envelope["contract_evidence"]["connector"] if r["component_id"] == group["terms"][0]["connector_id"])
    if abs(record["connector_axes_global"][axis][0]) != 1.0:
        pytest.skip("the control's joint group is not on a unit axis component")
    above = 1.0 + 2.0 ** -52
    record["connector_axes_global"][axis][0] = above if record["connector_axes_global"][axis][0] > 0 else -above
    # Every joint-area group of this connector on this axis (one per end node) takes the component.
    bound = [g for g in envelope["contract_evidence"]["exact_cases"][0]["pressure_rhs_assembly"]["groups"]
             if "effective_area_bits" in g and g["component"] == group["component"] and g["terms"][0]["connector_id"] == record["component_id"]]
    for g in bound:
        g["direction_component_magnitude"] = above
    validate_pressure_evidence(envelope)
    group["direction_component_magnitude"] = 1.0 + 2.0 ** -51
    with pytest.raises(ValueError):
        validate_pressure_evidence(envelope)


# RT-U4 (order): envelopes with two defects of different codes in one connector scan. Rust's first refusal follows
# the source's array order (`results`, then `exact_cases`); the verifier's follows the same order under every hash
# seed. (Rust's own determinism under fresh hash maps: result_export `tests/connector_order.rs`.)
COMBINATION_CORPUS = json.loads((PROJECT / "fixtures/results/pressure_v3_combination_reader_corpus.json").read_text())
_FIRST, _SECOND = "result:connector:component-C-J:", "result:loadcase:case-second:connector:component-C-J:"
_ROD, _END, _SX = "tie-rod-force:tie_rod:rod-yp:t_r", "connector-endpoint-force:end_i:fx", "connector-effective-axial-force:connector_local:s_x"
# Rust code -> the verifier's detail.
ORDER_DETAILS = {
    "SOURCE_PHYSICS_CONNECTOR_ROW_DUPLICATE": "duplicate connector row",
    "SOURCE_PHYSICS_CONNECTOR_ROW_UNBOUND": "unbound connector row",
    "SOURCE_PHYSICS_CONNECTOR_TIE_ROD_ROWS": "connector tie-rod rows missing",
    "SOURCE_PHYSICS_CONNECTOR_ROW_COVERAGE": "connector row coverage",
    "SOURCE_PHYSICS_CONNECTOR_UNPRESSURIZED_ROWS": "unpressurized joint pressure rows",
    "SOURCE_PHYSICS_CONNECTOR_EFFECTIVE_ROW_BINDING": "joint effective row binding",
}


def _rods_envelope():
    entry = next(c for c in COMBINATION_CORPUS["cases"] if c["label"] == "W16B MO-F: two tie rods withheld by name")
    return deepcopy(entry["envelope"])


def _drop(envelope, row_id):
    envelope["results"].remove(next(r for r in envelope["results"] if r["id"] == row_id))


def competing_envelopes():
    """(name, envelope, Rust's first code), mirroring `tests/connector_order.rs`."""
    out = []
    generalized = [r for r in CASES[0][1]["results"] if r["kind"] == "connector_generalized_force_v1"]
    duplicate = dict(deepcopy(generalized[0]), id="result:probe:duplicate")
    unbound = dict(deepcopy(generalized[1]), id="result:probe:unbound", entity_ref="component:probe-unknown")
    for name, extra, code in [("duplicate then unbound", [duplicate, unbound], "ROW_DUPLICATE"), ("unbound then duplicate", [unbound, duplicate], "ROW_UNBOUND")]:
        envelope = deepcopy(CASES[0][1])
        envelope["results"].extend(deepcopy(extra))
        out.append((name, envelope, "SOURCE_PHYSICS_CONNECTOR_" + code))
    for rod_case, end_case, code in [(_FIRST, _SECOND, "TIE_ROD_ROWS"), (_SECOND, _FIRST, "ROW_COVERAGE")]:
        envelope = _rods_envelope()
        _drop(envelope, rod_case + _ROD)
        _drop(envelope, end_case + _END)
        out.append((f"coverage {code}", envelope, "SOURCE_PHYSICS_CONNECTOR_" + code))
    for pe_case, s_case, code in [(_FIRST, _SECOND, "UNPRESSURIZED_ROWS"), (_SECOND, _FIRST, "EFFECTIVE_ROW_BINDING")]:
        envelope = _rods_envelope()
        _drop(envelope, pe_case + _END)
        pe = deepcopy(next(r for r in envelope["results"] if r["id"] == pe_case + _SX))
        pe.update(id=pe_case + "connector-pressure-force:connector_local:pe_x", kind="connector_pressure_force_v1", value=0.0)
        pe["metadata"]["component"] = "pe_x"
        envelope["results"].append(pe)
        s_row = next(r for r in envelope["results"] if r["id"] == s_case + _SX)
        s_row["value"] = s_row["value"] * 1.5 + 1000.0
        out.append((f"pressure {code}", envelope, "SOURCE_PHYSICS_CONNECTOR_" + code))
    return out


def competing_details():
    """The verifier's first refusal detail for each competing envelope, in this process."""
    details = {}
    for name, envelope, _ in competing_envelopes():
        with pytest.raises(ValueError) as refused:
            validate_pressure_evidence(envelope)
        details[name] = str(refused.value).rsplit("SOURCE_PHYSICS_EVIDENCE_INVALID: ", 1)[-1]
    return details


def test_connector_first_refusal_follows_source_order_in_rust_and_the_verifier():
    import os
    import subprocess
    import sys
    expected = {}
    for name, envelope, code in competing_envelopes():
        with pytest.raises(rust_reader.ReaderRefusal) as refused:
            rust_reader.validate("pressure", envelope)
        assert refused.value.code == code, name
        expected[name] = ORDER_DETAILS[code]
    assert competing_details() == expected
    # Each process has its own string hash seed; a set of case IDs would be walked in its order.
    script = "import json; from tests import test_pressure_connector_readers as t; print(json.dumps(t.competing_details()))"
    for seed in range(8):
        run = subprocess.run([sys.executable, "-c", script], cwd=PROJECT, capture_output=True, text=True, check=True,
                             env=dict(os.environ, PYTHONHASHSEED=str(seed)))
        assert json.loads(run.stdout.strip().splitlines()[-1]) == expected, seed
