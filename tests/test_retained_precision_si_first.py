"""Bounded current stress H controls through the independent G5 consumer."""
from copy import deepcopy
from fractions import Fraction
import pytest
from verifiers import retained_precision as rp

STRESS_KINDS = (
    "element_local_axial_normal_stress", "element_local_bending_normal_stress_y",
    "element_local_bending_normal_stress_z", "element_local_torsional_shear_stress",
)

def _consume(row, outcome):
    def require(ok):
        assert ok
    lanes = [{"law": law, "state": "completed", "error": None,
              "work": {"correction": {"calls": {"kind": "exact", "value": 1}},
                       "data_capacity": 0, "view": {"data_capacity": 0}}}
             for law in ("admitted_k", "annular_source")]
    attempt = {"stages": {"projection": "completed"}, "result": {"kind": "ready"},
               "proof": {"lanes": lanes, "projection_conversions": {"kind": "exact", "value": 1},
                         "projection_outcomes": [{"row_index": 0, "outcome": outcome}]}}
    rp._g5_proof_rows(attempt, [row], require, require)

@pytest.mark.parametrize("kind", STRESS_KINDS)
def test_si_first_stress_binds_exact_midpoint_and_publication(kind):
    lo, hi = map(rp.from_bits, ("40f8000000000001", "40f8000000000002"))
    midpoint = (Fraction(lo) + Fraction(hi)) / 2
    si = float(midpoint)
    assert rp.bits(si) == "40f8000000000002"
    # Exact rational reference distinguishes wide-before-conversion from SI-first.
    assert rp.bits(float(midpoint / 1_000_000)) == "3fb92a737110e455"
    published = si / 1_000_000.0
    assert rp.bits(published) == "3fb92a737110e456"
    row = {"kind": kind, "unit": "MPa", "value": published}
    outcome = {"kind": "normal", "value": rp.bits(si)}
    _consume(row, outcome)
    for field, value in (("value", rp.from_bits("3fb92a737110e455")),
                         ("kind", "element_local_axial_force"), ("unit", "Pa")):
        changed = deepcopy(row); changed[field] = value
        with pytest.raises(AssertionError):
            _consume(changed, outcome)
    with pytest.raises(AssertionError):
        _consume(row, {"kind": "normal", "value": "40f8000000000000"})

@pytest.mark.parametrize("kind,unit,word", (
    ("global_nodal_displacement_x", "mm", "4197700000000001"),
    ("element_local_torsional_moment", "N*m", "40f8000000000002"),
    ("pipe_elastic_normal_stress_maximum_v2", "Pa", "40f8000000000002"),
))
def test_nonstress_projection_keeps_raw_word(kind, unit, word):
    _consume({"kind": kind, "unit": unit, "value": rp.from_bits(word)},
             {"kind": "normal", "value": word})

def test_stress_zero_and_overflow_keep_truthful_si_class():
    row = {"kind": STRESS_KINDS[0], "unit": "MPa", "value": 0.0}
    for outcome in ({"kind": "underflow", "negative": True},
                    {"kind": "subnormal", "value": "0000000000000001", "relative_precision": "3ff0000000000000"}):
        _consume(row, outcome)
        negative_zero = deepcopy(row); negative_zero["value"] = -0.0
        with pytest.raises(AssertionError):
            _consume(negative_zero, outcome)
    with pytest.raises(AssertionError):
        _consume(row, {"kind": "overflow", "negative": False})
