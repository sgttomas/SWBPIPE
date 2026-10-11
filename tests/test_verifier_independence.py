"""The Python verifiers under `tests/verifiers/` are test-only and independent of the producer.

- No module outside `tests/` imports them, by package or by path; the former product module
  `core.analysis_runs.retained_precision` is gone.
- They spawn no process: the retained verifier hashes through `tests/verifiers/canonical_json.py`
  and converts units through `tests/verifiers/units.py`, never through a bridge to a crate the
  producer links.
- Those two are checked against the crates' committed vectors, read from their files: the
  canonical JSON crate's `fixtures/canonical_hash/cases.json` and
  `fixtures/results/precision_transport_v0_3.json`, and the units crate's witness tables in
  `core/units/src/lib.rs`. The producer-solved successors' G8 replay checks the verifier's
  hashes and converted bits against the producer's own.
"""
import ast
from copy import deepcopy
import json
import math
from pathlib import Path
import re
import struct
import subprocess

import pytest

from tests.verifiers import canonical_json, units
from tests.verifiers import retained_precision as rp

ROOT = Path(__file__).resolve().parents[1]
SKIP_DIRS = {"node_modules", "target", ".git", "__pycache__", "dist", "build"}
SUCCESSORS = sorted(path.name for path in (ROOT / "fixtures/results").glob("retained_precision_*_successor_*.json")
                    if "derivative" not in path.name)


def _product_modules():
    for path in ROOT.rglob("*.py"):
        relative = path.relative_to(ROOT)
        if relative.parts[0] != "tests" and not SKIP_DIRS & set(relative.parts):
            yield relative, path.read_text(encoding="utf-8")


def _verifier_imports(tree):
    """Every import naming the verifiers package or one of its modules, whatever path put it on sys.path."""
    names = {"verifiers", "retained_precision", "canonical_json", "units"}
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            modules = [alias.name for alias in node.names]
        elif isinstance(node, ast.ImportFrom):
            modules = [node.module or ""] + [f"{node.module}.{alias.name}" for alias in node.names]
        else:
            continue
        for module in modules:
            parts = module.split(".")
            if parts[:2] == ["tests", "verifiers"] or parts[0] == "verifiers" or parts == ["retained_precision"] or (
                    parts[0] == "tests" and len(parts) > 1 and parts[1] in names) or module == "core.analysis_runs.retained_precision":
                yield module


def test_no_module_outside_tests_imports_a_verifier():
    found, count = [], 0
    for relative, text in _product_modules():
        count += 1
        tree = ast.parse(text, filename=str(relative))
        found += [(str(relative), module) for module in _verifier_imports(tree)]
        # By path: spec_from_file_location, runpy, sys.path or exec over the verifiers directory.
        if re.search(r"tests[/\\.]verifiers|verifiers[/\\]retained_precision", text):
            found.append((str(relative), "path"))
    assert count > 100 and found == []


def test_the_former_product_module_is_gone():
    import importlib.util
    assert importlib.util.find_spec("core.analysis_runs.retained_precision") is None
    assert not (ROOT / "core/analysis_runs/retained_precision.py").exists()


def test_canonical_json_matches_the_crates_committed_vectors():
    """`fixtures/canonical_hash/cases.json`, blessed by the native canonical JSON crate's unchecked text.
    Every case inside the checked profile is equal, text and digest. The checked profile refuses the three
    cases holding an integral value beyond 2^53 - 1, as the crate's checked text does; their numbers
    still render as the vectors state."""
    cases = json.loads((ROOT / "fixtures/canonical_hash/cases.json").read_text(encoding="utf-8"))["cases"]
    refused = []
    for case in cases:
        value = json.loads(case["input_json"])
        try:
            text = canonical_json.canonical_json(value)
        except ValueError:
            refused.append(case["case_id"])
            numbers = value if isinstance(value, list) else [value]
            if all(type(number) is float for number in numbers):
                rendered = ",".join(canonical_json.ecma_number(number) for number in numbers)
                assert (f"[{rendered}]" if isinstance(value, list) else rendered) == case["expected_canonical"]
            continue
        assert text == case["expected_canonical"], case["case_id"]
        assert canonical_json.canonical_sha256(value) == case["expected_sha256"], case["case_id"]
    assert len(cases) >= 20
    assert refused == ["number-large-exponent", "number-ecma-notation-boundaries", "number-beyond-2-53-integer-kept-exact"]


def test_canonical_json_keeps_every_transport_vectors_bits():
    """`fixtures/results/precision_transport_v0_3.json` (the crate's `precision_transport.rs`): each listed
    binary64 survives the checked text; 2^53 is outside the profile."""
    vectors = json.loads((ROOT / "fixtures/results/precision_transport_v0_3.json").read_text(encoding="utf-8"))["vectors"]
    for vector in vectors:
        value = float(vector["decimal"])
        assert struct.pack(">d", value).hex() == vector["bits_hex"]
        text = canonical_json.canonical_json({"value": value})
        back = json.loads(text)["value"]
        assert struct.pack(">d", float(back)).hex() == vector["bits_hex"], vector["id"]
    with pytest.raises(ValueError):
        canonical_json.canonical_json({"value": 9007199254740992})
    with pytest.raises(ValueError):
        canonical_json.canonical_json({"value": 1.7976931348623157e308})


def _rust_test(name):
    """The body of one test function in the units crate's committed test module."""
    text = (ROOT / "core/units/src/lib.rs").read_text(encoding="utf-8")
    start = text.index(f"fn {name}()")
    return text[start:text.index("\n    }\n", start)]


def _snake(name):
    return re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()


def test_units_match_the_crates_committed_witnesses():
    """`core/units/src/lib.rs`'s witness tables, read from the file, at the crate's own tolerance."""
    tolerance = float(re.search(r"const REL_TOL: f64 = ([0-9.e-]+);", (ROOT / "core/units/src/lib.rs").read_text()).group(1))
    def close(actual, expected):
        assert abs(actual - expected) <= tolerance * max(abs(expected), 1.0), (actual, expected)
    rows = re.findall(r'\("([^"]+)", Dimension::(\w+), UnitId::\w+, ([0-9_.]+)\)', _rust_test("practical_units_have_independent_numeric_conversion_witnesses"))
    assert len(rows) == 8
    for symbol, dimension, expected in rows:
        close(units.to_canonical(1.0, symbol, _snake(dimension)), float(expected.replace("_", "")))
    refused = re.findall(r'unit_by_symbol\("([^"]+)", Dimension::(\w+)\)\.is_err\(\)', _rust_test("practical_units_have_independent_numeric_conversion_witnesses"))
    assert len(refused) == 4
    for symbol, dimension in refused:
        with pytest.raises(ValueError, match="UNITS_CONVERSION_REFUSED"):
            units.to_canonical(1.0, symbol, _snake(dimension))
    affine = re.findall(r"convert\(([0-9.]+), UnitId::(DegreeCelsius|DegreeFahrenheit), UnitId::Kelvin\)\.unwrap\(\),\s*([0-9.]+),", _rust_test("absolute_temperature_uses_affine_conversion"))
    assert len(affine) == 2
    for value, unit, expected in affine:
        close(units.to_canonical(float(value), {"DegreeCelsius": "degC", "DegreeFahrenheit": "degF"}[unit], "temperature"), float(expected))
    close(units.to_canonical(0.0, "C", "temperature"), 273.15)
    close(units.to_canonical(10.0, "C", "temperature_interval"), 10.0)
    close(units.to_canonical(18.0, "degF", "temperature_interval"), 10.0)
    # Identity keeps every bit (`same_unit_identity_preserves_bits_without_canonical_round_trip`).
    literals = re.search(r"for value in \[([^\]]+)\]", _rust_test("same_unit_identity_preserves_bits_without_canonical_round_trip")).group(1)
    values = [float(item) if "from_bits" not in item else struct.unpack(">d", int(re.search(r"from_bits\((\w+)\)", item).group(1), 0).to_bytes(8, "big"))[0]
              for item in (part.strip() for part in literals.split(","))]
    assert len(values) == 4
    for value in values:
        for dimension, (symbol, _) in units.CANONICAL.items():
            assert struct.pack(">d", units.to_canonical(value, symbol, dimension)) == struct.pack(">d", value)
    # The bridge binary's own witnesses (`openpipestress_units.rs`): GPa to Pa and degF to K.
    assert units.convert_quantities_to_canonical([{"id": "E", "value": 200.0, "unit": "GPa", "dimension": "stress"}]) == [
        {"id": "E", "value": 2e11, "unit": "Pa", "dimension": "stress"}]
    assert units.convert_quantities_to_canonical([{"id": "T", "value": 68.0, "unit": "degF", "dimension": "temperature"}])[0]["unit"] == "K"
    for value in (True, math.inf, math.nan, 10**10000):
        with pytest.raises(ValueError, match="UNITS-REQUEST-NONFINITE"):
            units.convert_quantities_to_canonical([{"id": "bad", "value": value, "unit": "Pa", "dimension": "stress"}])
    with pytest.raises(ValueError, match="UNITS_CONVERSION_REFUSED"):
        units.convert_quantities_to_canonical([{"id": "wrong", "value": 1.0, "unit": "m", "dimension": "stress"}])


@pytest.fixture
def no_processes(monkeypatch):
    def forbidden(*args, **kwargs):
        raise AssertionError("the verifier spawned a process")
    for name in ("run", "Popen", "call", "check_call", "check_output"):
        monkeypatch.setattr(subprocess, name, forbidden)


@pytest.mark.parametrize("name", SUCCESSORS)
def test_the_verifier_replays_producer_successors_without_any_process(no_processes, name):
    """G0-G8 bound to the producer's own invocation: every preparation, identity, publication and invocation
    hash and every converted input bit at G8 equals what the producer wrote, with no process spawned."""
    fixture = json.loads((ROOT / "fixtures/results" / name).read_text(encoding="utf-8"))
    validation = rp.validate_retained_precision(deepcopy(fixture["source"]), deepcopy(fixture["invocation"]))
    assert validation["invocation_bound"] and validation["classifications"]
    assert rp.validate_retained_precision_transport(deepcopy(fixture["source"]))["numerical_eligible"] is False


@pytest.mark.parametrize("name", ["mixed_units", "n05_units"])
@pytest.mark.parametrize("mode", ["sparse_interactive", "dense_scrutiny"])
def test_the_verifiers_units_reproduce_the_producers_converted_materials_and_sections(no_processes, name, mode):
    """The retained successors' invocations are SI, so their G8 replay converts by identity. These
    producer-solved mixed-unit statements (GPa, MPa, degF, degC and K inputs) record the producer's
    converted moduli, temperature-interpolated moduli and section terms; the exact-route G8 check the
    verifier runs (`_actual_materials` over `_canonical_inputs`) matches them bit for bit with this
    module's conversions."""
    from core.analysis_runs.physics_source import _actual_materials, _canonical_inputs
    folder = ROOT / "fixtures/product_preview/physics_source"
    source = json.loads((folder / f"{name}-{mode}.raw.json").read_text(encoding="utf-8"))
    invocation = {"request": json.loads((folder / f"{name}.request.json").read_text(encoding="utf-8")), "solver_mode": mode}
    canonical = _canonical_inputs(invocation, units.convert_quantities_to_canonical)
    cases = source["contract_evidence"]["exact_cases"]
    assert cases
    for exact in cases:
        actual = next(c for c in invocation["request"]["model"]["load_cases"] if c["id"] == exact["load_case_id"])
        _actual_materials(invocation, actual, exact, canonical)
