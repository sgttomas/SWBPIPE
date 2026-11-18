"""Generated loads beside the rest of a load and reference-state document on the exact route
(`validation/references/exact_load_state_generated_interactions/`).

The generator rebuilds `references.json` and `SHA256SUMS` in memory, compares them byte for byte with the
committed files and fails if any of its cross-checks disagrees. The package stays independent: its generator
imports only the Python standard library, names no product path in any string literal and makes no dynamic
import or evaluation call. No product test reads the package yet.
"""
import ast
import hashlib
import json
import re
import subprocess
import sys
from fractions import Fraction
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
REFERENCE = PROJECT / "validation" / "references" / "exact_load_state_generated_interactions"
GENERATOR = REFERENCE / "generators" / "generated_interactions_reference.py"


def test_generator_check_rebuilds_the_committed_references():
    result = subprocess.run([sys.executable, "-I", "-B", str(GENERATOR)], cwd=GENERATOR.parent,
                            capture_output=True, text=True, timeout=900)
    assert result.returncode == 0, result.stdout + result.stderr


def test_sha256sums_verify():
    lines = (REFERENCE / "SHA256SUMS").read_text().splitlines()
    assert lines
    listed = set()
    for line in lines:
        digest, path = line.split("  ", 1)
        listed.add(path)
        assert hashlib.sha256((REFERENCE / path).read_bytes()).hexdigest() == digest, path
    assert listed == {"README.md", "references.json", "generators/generated_interactions_reference.py"}


PRODUCT_ROOTS = {"core", "apps", "tests", "tools", "fixtures", "schemas"}
DYNAMIC_CALLS = {"__import__", "import_module", "exec", "eval", "spec_from_file_location"}


def names_product_path(text):
    """A literal names a product path when it is non-empty, has no whitespace and a segment is a product root."""
    return bool(text) and not re.search(r"\s", text) and any(
        segment in PRODUCT_ROOTS for segment in re.split(r"[\\/]+", text))


def test_generator_is_independent_of_the_product():
    tree = ast.parse(GENERATOR.read_text())
    modules = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            modules |= {alias.name.split(".")[0] for alias in node.names}
        elif isinstance(node, ast.ImportFrom):
            assert node.level == 0, "relative import"
            modules.add(node.module.split(".")[0])
        elif isinstance(node, ast.Constant) and isinstance(node.value, str):
            assert not names_product_path(node.value), f"line {node.lineno}: {node.value!r}"
        elif isinstance(node, ast.Call):
            func = node.func
            name = func.id if isinstance(func, ast.Name) else func.attr if isinstance(func, ast.Attribute) else None
            assert name not in DYNAMIC_CALLS, f"line {node.lineno}: call to {name}"
    assert modules, "no imports found"
    assert modules <= set(sys.stdlib_module_names), sorted(modules - set(sys.stdlib_module_names))


def _doc():
    return json.loads((REFERENCE / "references.json").read_text())


def _rows(item):
    return {r["id"]: r for r in item["rows"]}


def test_every_cross_check_agrees():
    doc = _doc()
    assert doc["crosscheck"] and all(c["agrees"] for c in doc["crosscheck"])


def test_the_constant_effort_force_acts_in_every_case_and_cancels_in_every_term():
    doc = _doc()
    for case in doc["cases"]:
        for rid, row in _rows(case).items():
            if rid.startswith("constant_effort/"):
                assert row["expected"]["value"] > 0, (case["id"], rid)
    for term in doc["terms"]:
        ce = [r for rid, r in _rows(term).items() if rid.startswith("constant_effort/")]
        assert ce and all(r["expected"]["exact"] == "0" for r in ce), term["id"]


def test_srss_and_absolute_sum_are_formed_on_every_signed_row_of_the_three_terms():
    doc = _doc()
    terms = {t["id"]: _rows(t) for t in doc["terms"]}
    combos = {c["id"]: c for c in doc["combinations"]}
    srss, absum = _rows(combos["CEF-EQ-SRSS"]), _rows(combos["CEF-EQ-ABS"])
    operands = [terms[o] for o in combos["CEF-EQ-SRSS"]["operands"]]
    assert list(srss) == list(absum) == list(operands[0])
    for rid in srss:
        values = [t[rid]["expected"]["value"] for t in operands]
        assert srss[rid]["expected"]["value"] <= absum[rid]["expected"]["value"], rid
        assert abs(srss[rid]["expected"]["value"] - sum(v * v for v in values) ** 0.5) <= 1e-12 * (
            1 + abs(srss[rid]["expected"]["value"])), rid
        assert abs(absum[rid]["expected"]["value"] - sum(abs(v) for v in values)) <= 1e-12 * (
            1 + absum[rid]["expected"]["value"]), rid
    assert combos["CEF-EQ-SRSS-OF-CASES"]["rows"] == []


def test_each_term_reaction_resultant_balances_the_generated_resultant():
    for term in _doc()["terms"]:
        if "reaction_resultant" in term:
            assert term["reaction_resultant"]["equals_minus_generated_resultant"], term["id"]


def test_the_lift_off_support_lifts_only_above_the_threshold():
    doc = _doc()
    status = {c["id"]: c["expected_support_status"]["support:lo-b"] for c in doc["cases"] if c["model"] == "LO"}
    assert status == {"LO-SUS": "in contact (active)", "LO-EQZ-SEATED": "in contact (active)",
                      "LO-EQZ-LIFTED": "lifted (inactive)"}
    k_star = doc["lift_off_threshold"]["g_factor_z"]["value"]
    k = {c["id"]: float(Fraction(c["seismic"]["g_factors"][2])) for c in doc["cases"]
         if c["model"] == "LO" and c["seismic"]}
    assert k["LO-EQZ-SEATED"] < k_star < k["LO-EQZ-LIFTED"]


def test_the_replaced_span_carries_no_generated_load_and_a_wind_mark_on_it_is_refused():
    doc = _doc()
    for case in doc["cases"]:
        if case["model"] == "JR":
            assert all(g.get("member") != "pipe:b-c" for g in case["generated_loads"]), case["id"]
            assert not any("/pipe:b-c/" in rid for rid in _rows(case)), case["id"]
    refusal, = doc["refusals"]
    assert refusal["id"] == "JR-REFUSE-WIND-ON-REPLACED-SPAN" and refusal["code"] is None
