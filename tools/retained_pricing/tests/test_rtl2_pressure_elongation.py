"""RT-L2: the pressure-elongation owners' bindings (rtl2_owners_model) and the typed census's acceptance of the setting.

The model reads the list's bounds from the caps module (the raw census's at parse time; C after D1.4's elongation clause)
and fails closed on a changed owner statement, on a census that starts reading model_settings, and on a clause that is
removed, changed or no longer precedes validation; a snapshot without the setting adds nothing."""
from pathlib import Path
import runpy
import shutil
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
sys.path.insert(0, str(PRICING / "base"))
MODEL = runpy.run_path(str(PRICING / "base/rtl2_owners_model.py"))
FILES = [MODEL["VAL"], MODEL["MS"], MODEL["EA"], MODEL["PE"], MODEL["RM"], MODEL["PP"] + "lib.rs",
         MODEL["PP"] + "source_receipt.rs", MODEL["PP"] + "model_fields.rs"]


def snapshot(tmp_path):
    for rel in FILES:
        (tmp_path / rel).parent.mkdir(parents=True, exist_ok=True)
        shutil.copy(SOURCE / rel, tmp_path / rel)
    return tmp_path


def test_the_bounds_are_the_raw_census_caps_and_the_case_cap():
    assert MODEL["inspect"](str(SOURCE)) == {"k_census": 16_384, "string_capacity": 131_072, "k": 3}


def _edit(snap, rel, old, new):
    path = snap / rel
    text = path.read_text(encoding="utf-8")
    assert text.count(old) == 1, old
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


@pytest.mark.parametrize("rel,old,new", [
    # the clause removed
    ("core/product_physics/src/retained_memory.rs", MODEL["CLAUSE"], ""),
    # the clause weakened (a longer list admitted)
    ("core/product_physics/src/retained_memory.rs", "operating_load_cases.len() > m.load_cases.len()",
     "operating_load_cases.len() > m.load_cases.len() + 1"),
    # the domain no longer gates the permit
    ("core/product_physics/src/retained_memory.rs", "    domain?;\n    bound(index)", "    let _ = domain;\n    bound(index)"),
    # validation on the admission side
    ("core/product_physics/src/source_receipt.rs", "        let context = CompositeSourceContext::from_model(&parsed.model);\n",
     "        crate::validation::validate_model_inputs(&parsed.model, &[], &mut Vec::new());\n"
     "        let context = CompositeSourceContext::from_model(&parsed.model);\n"),
])
def test_a_clause_that_no_longer_precedes_validation_stops(tmp_path, rel, old, new):
    snap = snapshot(tmp_path)
    _edit(snap, rel, old, new)
    with pytest.raises(SystemExit, match="RTL2-K"):
        MODEL["inspect"](str(snap))


def test_a_domain_without_the_load_case_cap_stops(tmp_path):
    # k <= c gives k <= C only with the 1 <= c <= C check in the same domain.
    snap = snapshot(tmp_path)
    _edit(snap, MODEL["RM"], MODEL["CASES_CHECK"], "")
    with pytest.raises(SystemExit, match="RTL2-K"):
        MODEL["inspect"](str(snap))


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


def test_a_census_that_reads_model_settings_stops(tmp_path):
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
