"""T4-U4b τ_v,max on preview-physics-2 with G > E (DEL-04-07 Design u4b-u5b-implementation.md §2.4): the Python
reader against the product, read from the fixture the Rust test `u4b_preview_shear_cross_reader_fixture_is_current`
keeps current (`core/product_physics/tests/fixtures/u4b_preview_shear_moduli.json`).

- Every c_b of the 38-point family (E/(2G) from 1.4 down to 1e-300) is the product's bit for bit, and within 8.5u of
  the exact value, which is recomputed here in rational arithmetic from the Saint-Venant ν form.
- Both solved G > E documents pass the preview-physics reader with their models (τ bit for bit from the moduli).
"""
from __future__ import annotations

import json
import struct
from fractions import Fraction
from pathlib import Path

import pytest

from tests.verifiers import reader_verdict as rv
from tests.verifiers import u4b

PROJECT = Path(__file__).resolve().parents[1]
FIXTURE = json.loads((PROJECT / "core/product_physics/tests/fixtures/u4b_preview_shear_moduli.json").read_text())
U = Fraction(1, 2**53)


def _f(text: str) -> float:
    return struct.unpack(">d", int(text, 16).to_bytes(8, "big"))[0]


def _hex(value: float) -> str:
    return f"0x{u4b.bits(value):016x}"


def test_family_c_b_matches_the_product_and_its_bound():
    sections = [tuple(_f(v) for v in s) for s in FIXTURE["sections"]]
    refused = 0
    for item in FIXTURE["family"]:
        ro, ri, inertia = sections[item["section"]]
        e, g = _f(item["e"]), _f(item["g"])
        nu = Fraction(e) / (2 * Fraction(g)) - 1
        exact = ((3 + 2 * nu) * Fraction(ro) ** 2 + (1 + 2 * nu) * Fraction(ri) ** 2) / (4 * (1 + nu) * Fraction(inertia))
        assert Fraction(_f(item["exact_hi"])) + Fraction(_f(item["exact_lo"])) - exact <= U * U * exact, item
        factor = u4b.preview_shear_factor(e, g, ro, ri, inertia)
        assert (None if factor is None else _hex(factor)) == item["c_b"], (item, factor)
        if factor is None:
            assert e / (2.0 * g) - 1.0 == -1.0, item
            refused += 1
        else:
            assert abs(Fraction(factor) - exact) <= Fraction(17, 2) * U * exact, item
    assert refused == 6


@pytest.mark.parametrize("document", FIXTURE["documents"], ids=lambda d: d["name"])
def test_g_above_e_documents_pass_the_python_reader(document):
    outcome = rv.validate("preview-physics", document["envelope"], document["model"])
    assert outcome["ok"] is True, outcome
    assert any(r["kind"] == u4b.SHEAR_MAX and r["value"] > 0 for r in document["envelope"]["results"])
