"""T4-U4b (DEL-04-07 Design u4b-u5b-implementation.md §5.1): the test-only verifier's normative sequences
(``tests/verifiers/u4b.py``) reproduce every expected bit pattern of the stress_recovery vectors, read unmodified
from ``core/loads/stress_recovery/tests/u4b_section_bits/section_quantity_bits.json`` (exact-rational generator,
independent of both the Rust producer and this verifier)."""
import json
from pathlib import Path
import struct

import pytest

from tests.verifiers import u4b

PROJECT = Path(__file__).resolve().parents[1]
VECTORS = json.loads((PROJECT / "core/loads/stress_recovery/tests/u4b_section_bits/section_quantity_bits.json").read_text())


def f(text: str) -> float:
    return struct.unpack(">d", bytes.fromhex(text))[0]


def h(value: float) -> str:
    return struct.pack(">d", value).hex()


def evaluate(vector):
    i = {key: f(value) for key, value in vector["inputs"].items()}
    quantity = vector["quantity"]
    if quantity == "saint_venant_shear_factor":
        return {"c_b": u4b.shear_factor(i["nu"], i["outer_radius"], i["inner_radius"], i["second_moment"])}
    if quantity == "transverse_shear_maximum_mpa":
        return {"tau_v_max": u4b.shear_maximum(i["shear_y"], i["shear_z"], i["c_b"])}
    if quantity == "normal_stress_extremes":
        plus, minus = u4b.extremes(i["sigma_n"], i["sigma_y"], i["sigma_z"])
        return {"sigma_plus": plus, "sigma_minus": minus}
    if quantity == "normal_stress_extremes_membrane":
        sigma_n = i["membrane_pa"] / 1e6
        plus, minus = u4b.extremes(sigma_n, i["sigma_y"], i["sigma_z"])
        return {"sigma_n": sigma_n, "sigma_plus": plus, "sigma_minus": minus}
    if quantity == "directional_intensified_bending_pa":
        return {"intensified": u4b.directional(i["in_plane_factor"], i["out_of_plane_factor"], i["sigma_y"], i["sigma_z"])}
    if quantity == "toroidal_membrane_hoop":
        hoop = u4b.hoop(i["pressure"], i["inner_radius"], i["wall"], i["outer_radius"], i["bend_radius"])
        if hoop is None:
            return None
        return {"intrados": hoop[0], "crown": hoop[1], "extrados": hoop[2], "radial_inner": -i["pressure"], "radial_outer": 0.0}
    raise AssertionError(quantity)


def test_every_quantity_is_covered():
    assert {v["quantity"] for v in VECTORS["vectors"]} == {
        "saint_venant_shear_factor", "transverse_shear_maximum_mpa", "normal_stress_extremes",
        "normal_stress_extremes_membrane", "directional_intensified_bending_pa", "toroidal_membrane_hoop"}


@pytest.mark.parametrize("vector", VECTORS["vectors"], ids=[v["id"] for v in VECTORS["vectors"]])
def test_verifier_sequences_reproduce_the_expected_bits(vector):
    got = evaluate(vector)
    expected = vector["expected"]
    if expected.get("refused") or not expected or got is None:
        assert got is None or not expected, vector["id"]
        return
    for key, want in expected.items():
        assert h(got[key]) == want, (vector["id"], key)
