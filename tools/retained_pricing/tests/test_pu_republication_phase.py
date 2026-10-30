"""PU-X3 (retained-pricing-rules.md §8.14): the republication is its own phase only where the source republishes."""
from pathlib import Path
import os
import runpy
import shutil
import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = Path(os.environ.get("B7A_PHASE_TEST_SOURCE", str(PRICING.parents[1])))
MODEL = runpy.run_path(str(PRICING / "base/b7a_phase_model.py"))
LIB = "core/product_physics/src/lib.rs"


def _snapshot(tmp_path, text):
    path = tmp_path / LIB
    path.parent.mkdir(parents=True)
    path.write_text(text, encoding="utf-8")
    return tmp_path


def test_the_current_source_prices_the_republication_as_x3():
    names = MODEL["phase_names"](SOURCE)
    assert names[:7] == MODEL["PHASE_NAMES"] and names[7] == MODEL["X3_NAME"] and len(names) == 8


def test_an_earlier_source_keeps_seven_phases(tmp_path):
    text = (SOURCE / LIB).read_text(encoding="utf-8")
    earlier = text.replace("\nfn republication(\n", "\nfn retired_a(\n").replace("\nfn fail_closed_form(", "\nfn retired_b(")
    assert MODEL["phase_names"](_snapshot(tmp_path, earlier)) == MODEL["PHASE_NAMES"]


@pytest.mark.parametrize("marker", ["\nfn republication(\n", "\nfn fail_closed_form("])
def test_a_partial_republication_stops(tmp_path, marker):
    text = (SOURCE / LIB).read_text(encoding="utf-8")
    assert text.count(marker) == 1
    with pytest.raises(ValueError, match="PU-X3"):
        MODEL["republishes"](_snapshot(tmp_path, text.replace(marker, "\nfn renamed_marker(")))
