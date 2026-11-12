"""Seismic component-weight references (`validation/references/seismic_component_weight/`).

The generator rebuilds `references.json` and `SHA256SUMS` in memory and compares
them byte for byte with the committed files; while rebuilding it solves every case
by two exact methods and repeats the build on binary64 inputs. Here: that check
passes, the committed SHA256SUMS verifies, the generator is independent of the
product (standard-library imports only, no dynamic import or evaluation, no string
literal that names a product path), and the frozen parity and warning outcomes are
the ones the decided rule gives.
"""
import ast
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
REFERENCE = PROJECT / "validation" / "references" / "seismic_component_weight"
GENERATOR = REFERENCE / "generators" / "scw_reference.py"
PRODUCT_ROOTS = {"core", "apps", "tests", "tools", "fixtures", "schemas"}
FORBIDDEN_CALLS = {"__import__", "import_module", "exec", "eval", "spec_from_file_location"}
DECIDED_RULES = {"decided", "decided, at the weight case's gravity instead of the authored gravity",
                 "decided, in the limit of an applied weight tending to zero",
                 "decided: a weight of zero is not applied"}


def test_generator_check_rebuilds_the_committed_references():
    result = subprocess.run([sys.executable, "-I", str(GENERATOR)], cwd=GENERATOR.parent,
                            capture_output=True, text=True, timeout=600)
    assert result.returncode == 0, result.stdout + result.stderr


def test_sha256sums_verify():
    lines = (REFERENCE / "SHA256SUMS").read_text().splitlines()
    assert {line.split("  ", 1)[1] for line in lines} == {
        "README.md", "generators/scw_reference.py", "references.json"}
    for line in lines:
        digest, path = line.split("  ", 1)
        assert hashlib.sha256((REFERENCE / path).read_bytes()).hexdigest() == digest, path


def test_generator_is_independent_of_the_product():
    tree = ast.parse(GENERATOR.read_text())
    modules = set()
    for node in ast.walk(tree):
        if isinstance(node, ast.Import):
            modules.update(alias.name.split(".")[0] for alias in node.names)
        elif isinstance(node, ast.ImportFrom):
            assert node.level == 0, "relative import"
            modules.add(node.module.split(".")[0])
        elif isinstance(node, ast.Call):
            func = node.func
            name = func.id if isinstance(func, ast.Name) else func.attr if isinstance(func, ast.Attribute) else None
            assert name not in FORBIDDEN_CALLS, name
        elif isinstance(node, ast.Constant) and isinstance(node.value, (str, bytes)):
            text = node.value.decode("latin-1") if isinstance(node.value, bytes) else node.value
            named = PRODUCT_ROOTS & set(re.split(r"[\\/]+", text))
            assert not named, f"literal names a product path {sorted(named)}: {text[:80]!r}"
    assert modules <= set(sys.stdlib_module_names), modules - set(sys.stdlib_module_names)


def test_parity_warnings_and_cross_checks_are_frozen():
    doc = json.loads((REFERENCE / "references.json").read_text())
    assert all(c["count"] == c["agreeing"] for c in doc["crosscheck"])
    assert doc["binary64_input_check"]["sign_changes"] == 0
    assert doc["binary64_input_check"]["discrete_outcome_changes"] == 0
    for case in doc["cases"]:
        if "parity" not in case:
            continue
        par = case["parity"]
        same_gravity = case["gravity"]["exact"] == par["weight_gravity"]["exact"]
        assert par["holds"] is (case["rule"] in DECIDED_RULES and same_gravity), case["id"]
        if case["role"] in ("reference", "limit") and not same_gravity:
            # a document whose own weight case differs from the frozen comparison carries own_weight_case
            own = par.get("own_weight_case")
            if own is None:
                assert par["at_the_seismic_case_gravity"]["holds"] is True, case["id"]
                assert par["difference_force"] == par["pipes_share_force"], case["id"]
                assert par["difference_moment"] == par["pipes_share_moment"], case["id"]
            else:
                assert own["holds_at_the_seismic_case_gravity"] is True, case["id"]
                assert own["difference_force"] == own["pipes_share_force"], case["id"]
                assert own["difference_moment"] == own["pipes_share_moment"], case["id"]
        if case["role"] == "reference":
            warn = case["expected_model_check_warning"]
            differs = warn["seismic_gravity_authored"]["exact"] != warn["applied_self_weight_gravity"]["exact"]
            assert warn["expected"] is (warn["carries_applied_component_force"] and differs), case["id"]
        if case["role"] == "limit" and "expected_model_check_warning" in case:
            warn = case["expected_model_check_warning"]
            assert warn["expected"] is None and warn["applicable"] is False and warn["reason"], case["id"]
    warnings = {c["id"]: c["expected_model_check_warning"]["expected"]
                for c in doc["cases"] if "expected_model_check_warning" in c}
    for route in ("PF", "EX"):
        assert warnings[f"SCW-CANT-{route}-BND-BITS-SEISMIC-XYZ"] is False
        assert warnings[f"SCW-CANT-{route}-BND-NONE-SEISMIC-XYZ-G10"] is False
        assert warnings[f"SCW-CANT-{route}-BND-ULP-SEISMIC-XYZ"] is True
        assert warnings[f"SCW-CANT-{route}-SEISMIC-XYZ-G10"] is True
        # boundary 4: the limit carries no expectation; an entered zero weight is not applied, so no warning
        assert warnings[f"SCW-CANT-{route}-BND-ZERO-SEISMIC-XYZ-G10"] is None
        assert warnings[f"SCW-CANT-{route}-CTRL-ZERO-AS-NOT-APPLIED"] is False


def test_boundary_4_zero_weight_limit_and_document():
    doc = json.loads((REFERENCE / "references.json").read_text())
    cases = {c["id"]: c for c in doc["cases"]}
    for route in ("PF", "EX"):
        lim = cases[f"SCW-CANT-{route}-BND-ZERO-SEISMIC-XYZ-G10"]
        zdoc = cases[f"SCW-CANT-{route}-CTRL-ZERO-AS-NOT-APPLIED"]
        assert lim["role"] == "limit" and cases[f"SCW-CANT-{route}-BND-ZERO-WEIGHT-NZ"]["role"] == "limit"
        assert zdoc["role"] == "reference"
        lim_rows = {r["row"]: r for r in lim["rows"]}
        doc_rows = {r["row"]: r for r in zdoc["rows"]}
        # the limit: the member loses its distributed load; the component force and its couple are exactly zero
        assert not any(row.startswith("load/BC/") for row in lim_rows)
        assert [lim_rows[f"load/C/{a}"]["exact"] for a in ("Fx", "Fy", "Fz", "Mx", "My", "Mz")] == ["0"] * 6
        # the entered zero weight: not applied, the member keeps its load, no component force or couple
        assert all(f"load/BC/{q}" in doc_rows for q in ("qx", "qy", "qz"))
        assert not any(row.startswith("load/C/") for row in doc_rows)
        # it differs from the limit far above rounding
        assert zdoc["compared_with"] == lim["id"]
        for axis in ("Fx", "Fy", "Fz"):
            diff = zdoc["resultant_difference"][axis]["value"]
            assert abs(diff) > 1e9 * abs(lim_rows[f"resultant/{axis}"]["value"]) * 2.0 ** -52
        assert lim["parity"]["holds"] is False and lim["parity"]["at_the_seismic_case_gravity"]["holds"] is True
        assert zdoc["parity"]["holds"] is False
        assert zdoc["parity"]["own_weight_case"]["holds_at_the_seismic_case_gravity"] is True
        warn = zdoc["expected_model_check_warning"]
        assert warn["expected"] is False and warn["carries_applied_component_force"] is False and warn["reason"]
