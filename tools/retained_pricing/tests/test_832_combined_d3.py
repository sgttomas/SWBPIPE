"""§8.32: fail closed if the route, identity or decimal warrants change."""
from pathlib import Path
import runpy
import shutil
import pytest
PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]

def decisions(monkeypatch, source=SOURCE):
    monkeypatch.setenv("SQ2_SNAP", str(source))
    return runpy.run_path(str(PRICING / "rules/sq2_decisions_L.py"))

def test_timoshenko_cache_is_resident_despite_dispatch_zero(monkeypatch):
    d = decisions(monkeypatch)
    model = runpy.run_path(str(PRICING / "base/b7a_domain_model.py"))
    assert ("core/reporting/result_export/src/semantic_contract.rs:preview_physics_timoshenko_contract",
            "fixtures/results/semantic_contract_v0_3_preview_physics_timoshenko_1.json") in model["STATIC_OWNERS"]
    assert sum("TIM-RD:" in why for _, _, why in d["O10_EDGE_ZERO"]) == 2
    assert sum("TIM-PF:" in why for _, _, why in d["O10_EDGE_ZERO"]) == 4

@pytest.mark.parametrize("path,old,new", [
    ("core/product_physics/src/pf_timoshenko.rs", "if !selects(model) {", "if false {"),
    ("core/product_physics/src/model_settings.rs", "has_pressure_contract.then(|| ModelSettingError {", "true.then(|| ModelSettingError {"),
    ("core/reporting/result_export/src/reader_verdict.rs", "b.0 == route && b.1 == id", "b.0 == route"),
    ("core/reporting/result_export/src/preview_physics_evidence.rs", "preview_physics_identity_of(source)", "preview_physics_identity_of(&Value::Null)"),
    ("core/product_physics/src/case_state/temperature.rs", "canonical_decimal(quantity.value)", "canonical_decimal(0.0)"),
    ("core/product_physics/src/case_state/temperature/canonical_decimal.rs", "for n in 1..=17 {", "for n in 1..=16 {"),
    ("core/solver/frame_kernel/src/shear.rs", "{name} must be positive, got {value}", "{name} must be very positive, got {value}"),
])
def test_changed_premise_stops(tmp_path, monkeypatch, path, old, new):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    p = tmp_path / path
    text = p.read_text()
    assert old in text
    p.write_text(text.replace(old, new, 1))
    with pytest.raises((SystemExit, ValueError)):
        decisions(monkeypatch, tmp_path)
