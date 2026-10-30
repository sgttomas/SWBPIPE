"""The Python verifiers under `tests/verifiers/` are test-only and independent of the producer.

- No module outside `tests/` imports them, by package or by path; the former product modules
  (`core.analysis_runs.retained_precision` in R1; the base readers and the reader half of
  `compatibility` in R2) are gone.
- They import nothing from product code: each imports only the standard library and its
  siblings in `tests/verifiers/`.
- They spawn no process: the verifiers hash through `tests/verifiers/canonical_json.py`
  and convert units through `tests/verifiers/units.py`, never through a bridge to a crate the
  producer links (`core/serialization/canonical_json/adapter.py`, `core/units/adapter.py`).
- Those two are checked against the crates' committed vectors, read from their files: the
  canonical JSON crate's `fixtures/canonical_hash/cases.json` and
  `fixtures/canonical_hash/binary64_transport_vectors.json`, and the units crate's catalog (bit for bit)
  and witness tables in `core/units/src/lib.rs`. The producer-solved successors' G8 replay checks the verifier's
  hashes and converted bits against the producer's own.
"""
import ast
from copy import deepcopy
import json
import math
import operator
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


# R2: the base-reader verifiers moved from `core/analysis_runs/` (b7-plan R2).
MOVED = ("preview_physics_evidence", "physics_evidence", "physics_source", "load_reference_evidence", "load_reference_source", "source_blocks")
VERIFIERS = {path.stem for path in (ROOT / "tests/verifiers").glob("*.py")} - {"__init__"}


def _verifier_imports(tree):
    """Every import naming the verifiers package or one of its modules, whatever path put it on sys.path."""
    names = {"verifiers", *VERIFIERS}
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
                    parts[0] == "tests" and len(parts) > 1 and parts[1] in names) or module == "core.analysis_runs.retained_precision" or (
                    parts[-1] in MOVED):
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


def test_the_former_product_modules_are_gone():
    import importlib.util
    for name in ("retained_precision", *MOVED):
        assert importlib.util.find_spec(f"core.analysis_runs.{name}") is None, name
        assert not (ROOT / f"core/analysis_runs/{name}.py").exists(), name
        assert (ROOT / f"tests/verifiers/{name}.py").is_file(), name
    # The reader half of `compatibility` moved too: product keeps the record builder and takes the Rust verdict.
    from core.analysis_runs import compatibility
    for name in ("_standing_reason", "_retained_standing_from", "_classification_summary_from", "_class_binding_refusal",
                 "_retained_binding_refusal", "_not_required_cases_ordinarily_eligible"):
        assert not hasattr(compatibility, name), name
    import core.analysis_runs as package
    assert not hasattr(package, "validate_source_blocks")


def test_the_verifiers_import_nothing_from_product_code():
    """Each verifier imports only the standard library and its siblings (relative, or `tests.verifiers`)."""
    import sys
    stdlib = set(sys.stdlib_module_names)
    checked = 0
    for path in sorted((ROOT / "tests/verifiers").glob("*.py")):
        checked += 1
        for node in ast.walk(ast.parse(path.read_text(encoding="utf-8"), filename=str(path))):
            if isinstance(node, ast.Import):
                modules = [alias.name for alias in node.names]
            elif isinstance(node, ast.ImportFrom):
                if node.level:
                    assert node.module is None or node.module.split(".")[0] in VERIFIERS, (path.name, node.module)
                    continue
                modules = [node.module or ""]
            else:
                continue
            for module in modules:
                root = module.split(".")[0]
                assert module.startswith("tests.verifiers") or (root in stdlib and root != "subprocess"), (path.name, module)
    assert checked == len(VERIFIERS) + 1 and set(MOVED) | {"compatibility", "retained_precision"} <= VERIFIERS


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
    """`fixtures/canonical_hash/binary64_transport_vectors.json` (the crate's `binary64_transport.rs`): each listed
    binary64 survives the checked text; 2^53 is outside the profile."""
    vectors = json.loads((ROOT / "fixtures/canonical_hash/binary64_transport_vectors.json").read_text(encoding="utf-8"))["vectors"]
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


def _rust_fn(text, signature):
    """The body of one function in `core/units/src/lib.rs`, by its signature line."""
    start = text.index(signature)
    return text[start:text.index("\n    }\n", start) if signature.startswith("    ") else text.index("\n}\n", start)]


def _balanced(text, start):
    """The text inside the parentheses that open at `start`."""
    depth = 0
    for index in range(start, len(text)):
        depth += {"(": 1, ")": -1}.get(text[index], 0)
        if depth == 0:
            return text[start + 1:index]
    raise ValueError("unbalanced")


OPERATORS = {ast.Add: operator.add, ast.Sub: operator.sub, ast.Mult: operator.mul, ast.Div: operator.truediv}


def _evaluate(expression, constants):
    """A Rust f64 constant expression: literals, named constants, PI, + - * / and parentheses, in
    binary64 left to right, as Rust evaluates it."""
    tree = ast.parse(expression.replace("std::f64::consts::PI", "PI").replace("_", "").strip(), mode="eval")
    def value(node):
        if isinstance(node, ast.Expression):
            return value(node.body)
        if isinstance(node, ast.Constant) and type(node.value) in (int, float):
            return float(node.value)
        if isinstance(node, ast.Name):
            return math.pi if node.id == "PI" else constants[node.id]
        if isinstance(node, ast.UnaryOp) and isinstance(node.op, ast.USub):
            return -value(node.operand)
        if isinstance(node, ast.BinOp) and type(node.op) in OPERATORS:
            return OPERATORS[type(node.op)](value(node.left), value(node.right))
        raise ValueError(f"not a constant expression: {expression}")
    return value(tree)


def test_units_catalog_is_the_crates_bit_for_bit():
    """Every catalog entry (symbol, dimension, factor and offset bits, in the crate's CATALOG order) and every
    dimension's canonical unit, parsed from `core/units/src/lib.rs`'s `definition()`, `CATALOG`, `as_str()` and
    `canonical_unit()` and evaluated as Rust evaluates the constant expressions. A one-ulp drift fails here."""
    text = (ROOT / "core/units/src/lib.rs").read_text(encoding="utf-8")
    bits = lambda value: struct.pack(">d", value).hex()
    constants = {}
    # Names are keyed without "_", as _evaluate drops "_" (Rust's digit separator) from the whole expression.
    for name, expression in re.findall(r"^pub const ([A-Z0-9_]+): f64 = ([^;]+);", text, re.M):
        constants[name.replace("_", "")] = _evaluate(expression, constants)
    names = dict(re.findall(r'Self::(\w+) => "([^"]+)"', _rust_fn(text, "    pub fn as_str(self)")))
    body = _rust_fn(text, "    pub fn definition(self)")
    definitions = {}
    for arm in re.split(r"\n\s*Self::", body)[1:]:
        unit = arm.split()[0]
        symbol, dimension = re.search(r'def\(\s*self,\s*"([^"]+)",\s*Dimension::(\w+),', arm).groups()
        kind = re.search(r"Transform::(linear|affine)\(", arm)
        arguments = _balanced(arm, kind.end() - 1)
        parts = [part for part in re.split(r",(?![^(]*\))", arguments) if part.strip()]
        factor = _evaluate(parts[0], constants)
        offset = _evaluate(parts[1], constants) if kind.group(1) == "affine" else 0.0
        definitions[unit] = (symbol, names[dimension], bits(factor), bits(offset))
    order = re.findall(r"UnitId::(\w+)", text[text.index("const CATALOG"):text.index("];", text.index("const CATALOG"))])
    crate = [definitions[unit] for unit in order]
    ours = [(unit.symbol, unit.dimension, bits(unit.factor), bits(unit.offset)) for unit in units.CATALOG]
    assert len(order) == len(definitions) == 63 and ours == crate
    canonical = {}
    for dimensions, unit in re.findall(r"((?:Dimension::\w+\s*\|?\s*)+)=> Some\(UnitId::(\w+)\)", _rust_fn(text, "pub fn canonical_unit(")):
        for dimension in re.findall(r"Dimension::(\w+)", dimensions):
            canonical[names[dimension]] = definitions[unit][:2]
    assert len(canonical) == 29 and canonical == units.CANONICAL
    assert set(names.values()) == units.DIMENSIONS


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
    from tests.verifiers.physics_source import _actual_materials, _canonical_inputs
    folder = ROOT / "fixtures/product_preview/pressure_source/physics_source"
    source = json.loads((folder / f"{name}-{mode}.raw.json").read_text(encoding="utf-8"))
    invocation = {"request": json.loads((folder / f"{name}.request.json").read_text(encoding="utf-8")), "solver_mode": mode}
    canonical = _canonical_inputs(invocation, units.convert_quantities_to_canonical)
    cases = source["contract_evidence"]["exact_cases"]
    assert cases
    for exact in cases:
        actual = next(c for c in invocation["request"]["model"]["load_cases"] if c["id"] == exact["load_case_id"])
        _actual_materials(invocation, actual, exact, canonical)


@pytest.mark.parametrize("mode", ["sparse_interactive", "dense_scrutiny"])
def test_the_base_verifiers_read_raw_statements_without_any_process(no_processes, mode):
    """R2: the base verifiers' hashing (receipt, publication and invocation digests) and unit conversion
    run in process: pressure-source-1 statements bound to their producer's invocation, and a
    pressure-source-1 statement carrying the load/reference-state group (B7b: the twin of the retired
    joined load-reference-source-1), are admitted with no process spawned."""
    from tests.verifiers import compatibility
    from tests.verifiers.load_reference_source import validate_pressure_source_evidence
    from tests.verifiers.physics_source import validate_physics_source
    for folder, name, request, validate in (("pressure_source/physics_source", "n05", "n05", validate_physics_source),
                                            ("pressure_source/physics_source", "mixed_units", "mixed_units", validate_physics_source)):
        base = ROOT / "fixtures/product_preview" / folder
        source = json.loads((base / f"{name}-{mode}.raw.json").read_text(encoding="utf-8"))
        invocation = {"request": json.loads((base / f"{request}.request.json").read_text(encoding="utf-8")), "solver_mode": mode}
        assert validate(deepcopy(source), invocation) is True, (folder, name)
        refs = [case["basis_ref"] for case in source["source_block_recovery"]["body"]["cases"]]
        assert compatibility.numerical_use_standing(source, refs, invocation) == "numerically_eligible", (folder, name)
    joined = json.loads((ROOT / f"fixtures/product_preview/pressure_source/load_reference_source/n05-{mode}.raw.json").read_text(encoding="utf-8"))
    validate_pressure_source_evidence(deepcopy(joined), raw=True)
    # Admitted, never numerically eligible (LOAD_REFERENCE_SOURCE_NOT_NUMERICALLY_ELIGIBLE_IN_T1).
    assert validate_physics_source(deepcopy(joined)) is False
    assert compatibility._source_contract(joined)[0] == compatibility.PRESSURE_SOURCE_CONTRACT_ID
