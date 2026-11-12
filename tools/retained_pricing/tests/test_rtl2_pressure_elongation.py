"""RT-L2: the pressure-elongation owners' bindings (rtl2_owners_model) and the typed census's acceptance of the setting.

The model reads the list's bound from the caps module and fails closed on a changed owner statement or on a D1 that
starts reading model_settings; a snapshot without the setting adds nothing."""
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
sys.path.insert(0, str(PRICING / "base"))
MODEL = runpy.run_path(str(PRICING / "base/rtl2_owners_model.py"))
FILES = [MODEL["VAL"], MODEL["MS"], MODEL["EA"], MODEL["PE"], MODEL["RM"]]


def snapshot(tmp_path):
    for rel in FILES:
        (tmp_path / rel).parent.mkdir(parents=True, exist_ok=True)
        shutil.copy(SOURCE / rel, tmp_path / rel)
    return tmp_path


def test_the_bound_is_the_raw_census_caps():
    assert MODEL["inspect"](str(SOURCE)) == {"k": 16_384, "string_capacity": 131_072}


@pytest.mark.parametrize("rel,old", [
    ("core/product_physics/src/model_settings.rs", "let mut unknown = std::collections::HashSet::new();"),
    ("core/product_physics/src/validation.rs", "for (index, error) in refusals.into_iter().enumerate() {"),
    ("core/reporting/result_export/src/physics_evidence.rs", "let mut shape = vec!["),
])
def test_a_changed_owner_statement_stops(tmp_path, rel, old):
    snap = snapshot(tmp_path)
    path = snap / rel
    text = path.read_text(encoding="utf-8")
    assert old in text
    path.write_text(text.replace(old, old.replace(" ", "  ", 1), 1), encoding="utf-8")
    with pytest.raises(SystemExit, match="RTL2"):
        MODEL["inspect"](str(snap))


def test_a_d1_that_reads_model_settings_stops(tmp_path):
    snap = snapshot(tmp_path)
    path = snap / MODEL["RM"]
    text = path.read_text(encoding="utf-8")
    anchor = "        let units = &m.project.units;\n"
    assert text.count(anchor) == 1
    path.write_text(text.replace(anchor, "        let _ = &m.model_settings;\n" + anchor), encoding="utf-8")
    with pytest.raises(SystemExit, match="RTL2-K"):
        MODEL["inspect"](str(snap))


def test_a_snapshot_without_the_setting_adds_nothing(tmp_path):
    snap = snapshot(tmp_path)
    path = snap / MODEL["MS"]
    text = path.read_text(encoding="utf-8")
    path.write_text(text.replace("pub struct PressureElongationSetting {", "pub struct Renamed {"), encoding="utf-8")
    assert MODEL["inspect"](str(snap)) is None


def test_the_typed_census_accepts_the_setting_and_keeps_its_scalar_count():
    ts = runpy.run_path(str(PRICING / "base/typed_strings_model.py"))
    assert ts["inspect"](str(SOURCE)) == {"pipe": 13, "model": 4}
