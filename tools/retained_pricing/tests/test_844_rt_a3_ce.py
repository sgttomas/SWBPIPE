"""RT-A3 increment 3 (retained-pricing-rules.md §8.44): constant-effort supports on the exact route add no owner on D1.

RTA3CE-ID classifies the pressure-2 reader's two hanger-reference arguments in `applied_load_row` (IN128); its guard
binds them to the hanger record. D1.6 refuses every hanger and every support family outside the five it lists, so no
D1 model holds a constant-effort record. Each mutation below stops the run or breaks the premise. These are source
warrants, not runtime allocation observations.
"""
from pathlib import Path
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
RM = "core/product_physics/src/retained_memory.rs"
EV = "core/reporting/result_export/src/exact_combination_evidence.rs"


def decisions(monkeypatch, snapshot=SOURCE, route="E"):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(PRICING / f"rules/sq2_decisions_{route}.py"))


def snapshot_with(tmp_path, path, old, new):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    target = tmp_path / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1, (path, old)
    target.write_text(text.replace(old, new), encoding="utf-8")
    return tmp_path


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_two_hanger_references_are_classified_in128(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    audits = [value for key, value in d["O10_ID_AUDIT"]
              if str(key).split("/src/")[-1].startswith("exact_combination_evidence.rs:")
              and "self.manufacturer" in value]
    assert audits == [{"self.manufacturer": {"bytes": 128, "source": "IN128"},
                       "self.load_side_review": {"bytes": 128, "source": "IN128"}}]
    # The audit names the basis format's first line (the multi-line macro's row).
    key = next(str(k) for k, v in d["O10_ID_AUDIT"] if "self.manufacturer" in v)
    line = int(key.rsplit(":", 1)[1])
    text = (SOURCE / EV).read_text(encoding="utf-8").split("\n")
    assert '"basis": format!(' in text[line - 1]
    assert "load_side_review={};mechanics_consumption" in text[line]


@pytest.mark.parametrize("old, new", [
    # A hanger reference no longer read from the hanger record.
    ('manufacturer: reference(support, "manufacturer_reference", "manufacturer_reference_missing")?,',
     'manufacturer: support["id"].as_str()?,'),
    # Blank text no longer replaced by the static `..._missing` literal.
    ("Value::String(text) => Some(if text.trim().is_empty() { missing } else { text.as_str() }),",
     "Value::String(text) => Some(text.as_str()),"),
])
def test_a_changed_reference_source_stops(tmp_path, monkeypatch, old, new):
    snap = snapshot_with(tmp_path, EV, old, new)
    with pytest.raises(SystemExit, match="RTA3CE-ID"):
        decisions(monkeypatch, snapshot=snap)


def test_d1_refuses_every_hanger_and_every_other_support_family():
    """The premise behind §8.44's zero: no D1 model holds a constant-effort record (DEC-049 reads the hanger type,
    or the support family, `constant_effort_support`)."""
    text = (SOURCE / RM).read_text(encoding="utf-8")
    assert text.count("        if support.hanger.is_some() {\n            return refuse(C::Supports, F::Hanger);") == 1
    assert text.count('if !matches!(family.as_str(), "anchor" | "guide" | "line_stop" | "vertical_support" | "spring") {') == 1
    lib = (SOURCE / "core/product_physics/src/lib.rs").read_text(encoding="utf-8")
    assert lib.count('.or_else(|| support.family.as_deref().map(str::trim))') == 1
