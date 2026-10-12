"""The J7 RF census input fixture is the committed generator's exact output.

`tools/fixtures/j7_rf_census_inputs.py` reads the frozen, sha256-pinned T3 R1
references; the Rust census check (`core/product_physics/src/source_recovery/
j7_rf_census.rs`) compiles the fixture in. A stale fixture or a changed reference
fails here before any census verdict is read.
"""
import importlib.util
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
_SPEC = importlib.util.spec_from_file_location("j7_rf_census_inputs", ROOT / "tools/fixtures/j7_rf_census_inputs.py")
GENERATOR = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(GENERATOR)


def test_j7_rf_census_inputs_fixture_is_current():
    assert GENERATOR.FIXTURE.read_bytes() == GENERATOR.build()


def test_j7_rf_census_expected_pin_names_the_current_inputs():
    import hashlib
    pin = json.loads((ROOT / "fixtures/results/j7_rf_census_expected.json").read_bytes())
    assert pin["inputs_sha256"] == hashlib.sha256(GENERATOR.FIXTURE.read_bytes()).hexdigest()
    assert pin["references_sha256"] == GENERATOR.PINS["references.json"]
    assert len(pin["entries"]) == 322
