"""E17's independent publication controls; no shared retained-corpus loader.

The rational premises below prove the independent single-rounding examples.
The test-only verifier then compares those publications with the published
components using the existing bound. Source-receipt exact value binding is E15.
"""
from fractions import Fraction
import json
from pathlib import Path
import struct

import pytest

from tests.verifiers.preview_physics_evidence import _consistent_norm, _number

ROOT = Path(__file__).resolve().parents[1]
CONTROLS = json.loads((ROOT / "fixtures/results/preview_magnitude_controls.json").read_text())["controls"]


def binary64(bits):
    return struct.unpack(">d", bytes.fromhex(bits))[0]


@pytest.mark.parametrize("control", CONTROLS, ids=lambda control: control["id"])
def test_e17_published_components_and_independent_magnitude_use_the_existing_guard(control):
    components = [binary64(bits) for bits in control["component_bits"]]
    magnitude = binary64(control["magnitude_bits"])
    assert _consistent_norm(magnitude, components) is control["admitted"]


def test_e17_independent_one_ulp_and_subnormal_premises_are_exact_rational_controls():
    x, y = Fraction(3) + Fraction(7, 2**55), Fraction(4) + Fraction(15, 2**55)
    assert (float(x), float(y)) == (3.0, 4.0)
    square_sum = x*x + y*y
    assert (Fraction(5) + Fraction(1, 2**51))**2 < square_sum < (Fraction(5) + Fraction(3, 2**51))**2
    s = Fraction(1, 2**1074)
    a = Fraction(49, 100) * s
    assert (float(a), float(a)) == (0.0, 0.0)
    assert (s/2)**2 < 2*a*a < (3*s/2)**2
    assert _consistent_norm(binary64("4014000000000001"), [3.0, 4.0, 0.0])
    assert _consistent_norm(float(s), [0.0, 0.0, 0.0])


def test_e17_signed_components_remain_finite_and_admitted():
    s = binary64("0000000000000001")
    assert _number(-s)
    assert _consistent_norm(s, [-s, 0.0, 0.0])
