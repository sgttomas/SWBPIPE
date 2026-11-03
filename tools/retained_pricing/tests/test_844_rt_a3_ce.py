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


# §8.44 on the §8.45 lexer: CE lengthens only literals below the static class (S7, the D-2 load/reference-state
# limitation, the equivalent-static refusal text) and the pressure-2 `identity`, whose own-bound disposition
# (audited from source by U6-J4) carries. No new long literal, no disposition without its literal.
def _long_literals():
    import sys
    sys.path.insert(0, str(PRICING / "base"))
    import rust_literals as rl
    files = ["core/solver/frame_kernel/src/structural.rs", "core/solver/frame_kernel/src/lib.rs",
             "core/solver/frame_kernel/src/structural/exact_boundary.rs", "core/product_physics/src/lib.rs"]
    static = rl.measure(SOURCE, files)["max_bytes"]
    dirs = (PRICING / "base/crate_dirs.txt").read_text(encoding="utf-8").split()
    entries = runpy.run_path(str(PRICING / "base/static_literal_dispositions.py"))["ENTRIES"]
    return static, rl.long_literal_check(SOURCE, dirs, max(512, static), entries)


def test_the_constant_effort_literals_keep_their_dispositions():
    static, (matched, problems) = _long_literals()
    assert problems == []
    by_head = {m["head"]: m["bytes"] for m in matched}
    assert by_head["3.0.0/exact_pressure_v3 under pressure-2 semantics"] == 3007
    assert by_head["3.0.0/exact_pressure_v3 under pressure-source-1 semantics"] == 1262
    source = (SOURCE / "core/product_physics/src/exact_admission.rs").read_text(encoding="utf-8")
    for head in ("Signed support actions are attributed only for admitted linear restraints and springs and, as its",
                 "Global rigid support translations/rotations are prescribed absolute boundary values",
                 "equivalent_static generation is not provided on a load/reference-state document"):
        assert head in source and all(not m["head"].startswith(head[:30]) for m in matched), head
    # The pressure-2 identity's copy site is still the one `identity.to_string()` U6-J4 audits.
    assert source.count("identity.to_string()") == 1
    assert source.count("super::joint::CONNECTOR_LIMITATION.to_string()") == 1


def test_a_constant_effort_literal_over_the_class_stops(tmp_path, monkeypatch):
    """Negative control: S7 lengthened past the static class has no disposition, so the whole-scope check names it."""
    import sys
    sys.path.insert(0, str(PRICING / "base"))
    import rust_literals as rl
    snap = snapshot_with(tmp_path, "core/product_physics/src/exact_admission.rs",
                         "for a consumed constant-effort support; ambiguous",
                         "for a consumed constant-effort support" + " x" * 600 + "; ambiguous")
    shutil.copytree(SOURCE / "tools", tmp_path / "tools", ignore=shutil.ignore_patterns("__pycache__"))
    dirs = (PRICING / "base/crate_dirs.txt").read_text(encoding="utf-8").split()
    entries = runpy.run_path(str(PRICING / "base/static_literal_dispositions.py"))["ENTRIES"]
    _, problems = rl.long_literal_check(snap, dirs, 964, entries)
    assert any("exact_admission.rs" in p and "0 disposition entries" in p for p in problems), problems
